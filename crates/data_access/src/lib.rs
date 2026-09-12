use std::any::type_name;
use std::sync::Arc;
use theligi_algorithm::{Algorithm, AlgorithmError};
use theligi_authorization::{AuthorizationContext, AuthorizationError, AuthorizationProvider};
use theligi_cache::{CacheError, CacheKey};
use theligi_repository::{CasId, Repository, RepositoryError, StoredEntry};
use theligi_validation::SeriesValidator;
use thiserror::Error;

use daf_core::Cache as _;

impl From<daf_core::CacheError> for DataAccessError {
    fn from(e: daf_core::CacheError) -> Self {
        DataAccessError::Cache(e.into())
    }
}

/// Typed cache errors returned by the data access layer.
#[derive(Debug, Error)]
pub enum DataAccessError {
    #[error("validation error: {0}")]
    Validation(String),

    #[error("authorization error: {0}")]
    Authorization(#[from] AuthorizationError),

    #[error("cache error: {0}")]
    Cache(#[from] CacheError),

    #[error("repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("algorithm error: {0}")]
    Algorithm(#[from] AlgorithmError),

    #[error("not found")]
    NotFound,
}

impl From<theligi_validation::ValidationError> for DataAccessError {
    fn from(e: theligi_validation::ValidationError) -> Self {
        DataAccessError::Validation(e.to_string())
    }
}

/// Data access pipeline orchestrator.
#[async_trait::async_trait]
pub trait DataAccess: Send + Sync + 'static {
    type Request: Send + Sync + 'static;
    type Response: Send + Sync + 'static;

    async fn execute(
        &self,
        context: &AuthorizationContext,
        request: Self::Request,
    ) -> Result<Self::Response, DataAccessError>;
}

/// Concrete hierarchical data access implementation backed by a
/// `daf_cache::HierarchicalCache`, generic over the stored value type `V`.
pub struct HierarchicalDataAccess<V>
where
    V: Send + Sync + Clone + 'static,
{
    cache: Arc<daf_cache::HierarchicalCache>,
    repository: Arc<dyn Repository<Data = V>>,
    authorizer: Arc<dyn AuthorizationProvider>,
    algorithm: Option<Arc<dyn Algorithm<Input = V, Output = V>>>,
    validator: Option<Arc<dyn SeriesValidator>>,
}

impl<V> HierarchicalDataAccess<V>
where
    V: Send + Sync + Clone + 'static,
{
    #[must_use]
    pub fn new(
        cache: Arc<daf_cache::HierarchicalCache>,
        repository: Arc<dyn Repository<Data = V>>,
        authorizer: Arc<dyn AuthorizationProvider>,
        algorithm: Option<Arc<dyn Algorithm<Input = V, Output = V>>>,
        validator: Option<Arc<dyn SeriesValidator>>,
    ) -> Self {
        Self {
            cache,
            repository,
            authorizer,
            algorithm,
            validator,
        }
    }

    #[must_use]
    pub fn cache(&self) -> &Arc<daf_cache::HierarchicalCache> {
        &self.cache
    }

    #[must_use]
    pub fn repository(&self) -> &Arc<dyn Repository<Data = V>> {
        &self.repository
    }
}

#[async_trait::async_trait]
impl<V> DataAccess for HierarchicalDataAccess<V>
where
    V: Send + Sync + Clone + 'static,
{
    type Request = CasId;
    type Response = Result<V, DataAccessError>;

    async fn execute(
        &self,
        context: &AuthorizationContext,
        request: Self::Request,
    ) -> Result<Self::Response, DataAccessError> {
        let namespace = type_name::<V>().to_string();
        let cache_key = CacheKey::new(namespace, request.to_string());
        let key_str = format!("{}:{}", cache_key.namespace, cache_key.key);

        if let Some(validator) = &self.validator {
            let series_id: theligi_validation::SeriesId = request;
            let result = validator.validate_series(series_id).await?;
            if !result.passed {
                let violations = result
                    .violations
                    .into_iter()
                    .map(|v| v.message)
                    .collect::<Vec<_>>()
                    .join("; ");
                return Err(DataAccessError::Validation(format!(
                    "series {request} failed validation: {violations}"
                )));
            }
        }

        self.authorizer
            .authorize(context, "data_access", "execute")
            .await?;

        let cached = self.cache.get(&key_str).await?;
        if let Some(entry) = cached {
            let value = entry.value.downcast::<V>().map_err(|_| {
                CacheError::Internal(format!(
                    "type mismatch in cache; expected {}",
                    type_name::<V>()
                ))
            })?;
            return Ok(Ok((*value).clone()));
        }

        let stored: StoredEntry<V> = self.repository.get(request).await?;
        let mut value = stored.data;

        if let Some(algorithm) = &self.algorithm {
            value = algorithm.execute(value).await?;
        }

        let any: Arc<dyn std::any::Any + Send + Sync> = Arc::new(value.clone());
        self.cache.set(key_str, any).await?;

        Ok(Ok(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use theligi_authorization::AuthorizationContext;
    use theligi_repository::InMemoryRepository;

    #[derive(Clone, Default)]
    struct PassthroughAlgorithm<V> {
        _marker: std::marker::PhantomData<V>,
    }

    #[async_trait::async_trait]
    impl<V: Clone + Send + Sync + 'static> Algorithm for PassthroughAlgorithm<V> {
        type Input = V;
        type Output = V;

        fn kind(&self) -> &'static str {
            "passthrough"
        }

        async fn execute(&self, input: Self::Input) -> Result<Self::Output, AlgorithmError> {
            Ok(input)
        }
    }

    #[derive(Default)]
    struct FailingValidator;

    #[async_trait::async_trait]
    impl SeriesValidator for FailingValidator {
        async fn validate_series(
            &self,
            series_id: theligi_validation::SeriesId,
        ) -> theligi_validation::ValidationResultType<theligi_validation::ValidationResult>
        {
            Ok(theligi_validation::ValidationResult::failed(
                series_id,
                vec![theligi_validation::Violation {
                    rule: theligi_validation::ViolationRule::PostCount,
                    message: "expected 6 posts".into(),
                    severity: theligi_validation::Severity::Error,
                }],
            ))
        }

        async fn validate_batch(
            &self,
            _series_ids: &[theligi_validation::SeriesId],
        ) -> theligi_validation::ValidationResultType<theligi_validation::ValidationReport>
        {
            Ok(theligi_validation::ValidationReport::default())
        }

        async fn validate_post_count(
            &self,
            _series_id: theligi_validation::SeriesId,
            _expected: usize,
        ) -> theligi_validation::ValidationResultType<bool> {
            Ok(true)
        }

        async fn validate_shared_topic(
            &self,
            _series_id: theligi_validation::SeriesId,
        ) -> theligi_validation::ValidationResultType<bool> {
            Ok(true)
        }

        async fn validate_shared_thesis(
            &self,
            _series_id: theligi_validation::SeriesId,
        ) -> theligi_validation::ValidationResultType<bool> {
            Ok(true)
        }

        async fn validate_causal_consistency(
            &self,
            _series_id: theligi_validation::SeriesId,
        ) -> theligi_validation::ValidationResultType<bool> {
            Ok(true)
        }

        async fn validate_lineage(
            &self,
            _series_id: theligi_validation::SeriesId,
        ) -> theligi_validation::ValidationResultType<bool> {
            Ok(true)
        }

        async fn validate_state_machine(
            &self,
            _series_id: theligi_validation::SeriesId,
            _state_machine: &theligi_content::SeriesStateMachine,
        ) -> theligi_validation::ValidationResultType<bool> {
            Ok(true)
        }

        async fn validate_causal_dag(
            &self,
            _series_id: theligi_validation::SeriesId,
        ) -> theligi_validation::ValidationResultType<bool> {
            Ok(true)
        }

        async fn validate_topic_stream_append_only(
            &self,
            _series_id: theligi_validation::SeriesId,
            _contract: &theligi_content::TopicStreamContract,
        ) -> theligi_validation::ValidationResultType<bool> {
            Ok(true)
        }
    }

    fn isolated_context() -> AuthorizationContext {
        let tenant_id = uuid::Uuid::new_v4();
        let session_id = uuid::Uuid::new_v4();
        AuthorizationContext::new()
            .with_tenant(tenant_id)
            .with_session(session_id)
            .with_claim("sub", "test-subject")
    }

    fn make_cache() -> Arc<daf_cache::HierarchicalCache> {
        let l0 = Arc::new(daf_cache::MemoryCache::new(1024)) as Arc<dyn daf_core::Cache>;
        let l1 = Arc::new(daf_cache::MemoryCache::new(1024)) as Arc<dyn daf_core::Cache>;
        let l2 = Arc::new(daf_cache::MokaCache::new(1024)) as Arc<dyn daf_core::Cache>;
        let l3 = Arc::new(daf_cache::MemoryCache::new(1024)) as Arc<dyn daf_core::Cache>;
        let l4 = Arc::new(daf_cache::MemoryCache::new(1024)) as Arc<dyn daf_core::Cache>;
        let l5 = Arc::new(daf_cache::MemoryCache::new(1024)) as Arc<dyn daf_core::Cache>;
        Arc::new(daf_cache::HierarchicalCache::new(
            Some(l0),
            l1,
            l2,
            l3,
            l4,
            Some(l5),
        ))
    }

    #[tokio::test]
    async fn validation_enforced_before_authorization() {
        let repo = Arc::new(InMemoryRepository::<String>::new());
        let cache = make_cache();
        let authorizer = Arc::new(theligi_authorization::BetterAuthProvider);
        let validator = Arc::new(FailingValidator::default());
        let pipeline = HierarchicalDataAccess::new(cache, repo, authorizer, None, Some(validator));

        let context = isolated_context();
        let request = uuid::Uuid::new_v4();
        let result = pipeline.execute(&context, request).await;
        assert!(matches!(result, Err(DataAccessError::Validation(_))));
    }

    #[tokio::test]
    async fn authorization_enforced_before_cache_access() {
        let repo = Arc::new(InMemoryRepository::<String>::new());
        let cache = make_cache();
        let authorizer = Arc::new(theligi_authorization::BetterAuthProvider);
        let pipeline = HierarchicalDataAccess::<String>::new(cache, repo, authorizer, None, None);

        let context = AuthorizationContext::new();
        let request = uuid::Uuid::new_v4();
        let result = pipeline.execute(&context, request).await;
        assert!(matches!(
            result,
            Err(DataAccessError::Authorization(
                theligi_authorization::AuthorizationError::IsolationViolation
            ))
        ));
    }

    #[tokio::test]
    async fn full_pipeline_cache_miss_then_hit() {
        let repo = Arc::new(InMemoryRepository::new());
        let id = repo.create("hello".to_string()).await.unwrap();
        let cache = make_cache();
        let authorizer = Arc::new(theligi_authorization::BetterAuthProvider);
        let algorithm: Option<Arc<dyn Algorithm<Input = String, Output = String>>> =
            Some(Arc::new(PassthroughAlgorithm::default()));
        let pipeline =
            HierarchicalDataAccess::new(cache.clone(), repo.clone(), authorizer, algorithm, None);

        let context = isolated_context();

        let result = pipeline.execute(&context, id).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().unwrap(), "hello");

        let result = pipeline.execute(&context, id).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().unwrap(), "hello");
    }

    #[tokio::test]
    async fn cache_hit_short_circuits_repository() {
        let repo = Arc::new(InMemoryRepository::<String>::new());
        let id = repo.create("original".to_string()).await.unwrap();
        let cache = make_cache();
        let authorizer = Arc::new(theligi_authorization::BetterAuthProvider);

        {
            let pipeline = HierarchicalDataAccess::<String>::new(
                cache.clone(),
                repo.clone(),
                authorizer.clone(),
                None,
                None,
            );
            let context = isolated_context();
            let result = pipeline.execute(&context, id).await;
            assert!(result.is_ok());
            assert_eq!(result.unwrap().unwrap(), "original");
        }

        repo.delete(id).await.unwrap();

        let pipeline = HierarchicalDataAccess::<String>::new(cache, repo, authorizer, None, None);
        let context = isolated_context();
        let result = pipeline.execute(&context, id).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().unwrap(), "original");
    }

    #[tokio::test]
    async fn missing_l0_returns_error() {
        let l1 = Arc::new(daf_cache::MemoryCache::new(1024)) as Arc<dyn daf_core::Cache>;
        let l2 = Arc::new(daf_cache::MokaCache::new(1024)) as Arc<dyn daf_core::Cache>;
        let l3 = Arc::new(daf_cache::MemoryCache::new(1024)) as Arc<dyn daf_core::Cache>;
        let l4 = Arc::new(daf_cache::MemoryCache::new(1024)) as Arc<dyn daf_core::Cache>;
        let l5 = Arc::new(daf_cache::MemoryCache::new(1024)) as Arc<dyn daf_core::Cache>;
        let cache = Arc::new(daf_cache::HierarchicalCache::new(
            None,
            l1,
            l2,
            l3,
            l4,
            Some(l5),
        ));
        let repo = Arc::new(InMemoryRepository::<String>::new());
        let authorizer = Arc::new(theligi_authorization::BetterAuthProvider);
        let pipeline = HierarchicalDataAccess::new(cache, repo, authorizer, None, None);

        let context = isolated_context();
        let request = uuid::Uuid::new_v4();
        let result = pipeline.execute(&context, request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn missing_l5_returns_error() {
        let l0 = Arc::new(daf_cache::MemoryCache::new(1024)) as Arc<dyn daf_core::Cache>;
        let l1 = Arc::new(daf_cache::MemoryCache::new(1024)) as Arc<dyn daf_core::Cache>;
        let l2 = Arc::new(daf_cache::MokaCache::new(1024)) as Arc<dyn daf_core::Cache>;
        let l3 = Arc::new(daf_cache::MemoryCache::new(1024)) as Arc<dyn daf_core::Cache>;
        let l4 = Arc::new(daf_cache::MemoryCache::new(1024)) as Arc<dyn daf_core::Cache>;
        let cache = Arc::new(daf_cache::HierarchicalCache::new(
            Some(l0),
            l1,
            l2,
            l3,
            l4,
            None,
        ));
        let repo = Arc::new(InMemoryRepository::<String>::new());
        let authorizer = Arc::new(theligi_authorization::BetterAuthProvider);
        let pipeline = HierarchicalDataAccess::new(cache, repo, authorizer, None, None);

        let context = isolated_context();
        let request = uuid::Uuid::new_v4();
        let result = pipeline.execute(&context, request).await;
        assert!(result.is_err());
    }
}
