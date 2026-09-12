use std::sync::Arc;
use theligi_algorithm::{Algorithm, AlgorithmError};
use theligi_authorization::{AuthorizationContext, AuthorizationError, AuthorizationProvider};
use theligi_cache::{Cache, CacheError};
use theligi_repository::{Repository, RepositoryError};
use thiserror::Error;

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

/// Orchestration pipeline implementation.
pub struct OrchestrationPipeline<R, C, A, Algo>
where
    R: Repository + 'static,
    C: Cache + 'static,
    A: AuthorizationProvider + 'static,
    Algo: Algorithm + 'static,
{
    _repository: Arc<R>,
    _cache: Arc<C>,
    _authorizer: Arc<A>,
    _algorithm: Arc<Algo>,
}

impl<R, C, A, Algo> OrchestrationPipeline<R, C, A, Algo>
where
    R: Repository + 'static,
    C: Cache + 'static,
    A: AuthorizationProvider + 'static,
    Algo: Algorithm + 'static,
{
    pub fn new(
        repository: Arc<R>,
        cache: Arc<C>,
        authorizer: Arc<A>,
        algorithm: Arc<Algo>,
    ) -> Self {
        Self {
            _repository: repository,
            _cache: cache,
            _authorizer: authorizer,
            _algorithm: algorithm,
        }
    }
}

#[async_trait::async_trait]
impl<R, C, A, Algo> DataAccess for OrchestrationPipeline<R, C, A, Algo>
where
    R: Repository + 'static,
    C: Cache + 'static,
    A: AuthorizationProvider + 'static,
    Algo: Algorithm + 'static,
{
    type Request = ();
    type Response = ();

    async fn execute(
        &self,
        context: &AuthorizationContext,
        _request: Self::Request,
    ) -> Result<Self::Response, DataAccessError> {
        self._authorizer
            .authorize(context, "data_access", "execute")
            .await?;

        tracing::trace!(
            "pipeline: validate, cache_lookup, repository_lookup, algorithm, cache_population"
        );

        Ok(())
    }
}

/// Concrete data access implementation backed by a `daf_cache::HierarchicalCache`.
pub struct HierarchicalDataAccess {
    cache: Arc<daf_cache::HierarchicalCache>,
}

impl HierarchicalDataAccess {
    #[must_use]
    pub fn new(cache: Arc<daf_cache::HierarchicalCache>) -> Self {
        Self { cache }
    }

    #[must_use]
    pub fn cache(&self) -> &Arc<daf_cache::HierarchicalCache> {
        &self.cache
    }
}

#[async_trait::async_trait]
impl DataAccess for HierarchicalDataAccess {
    type Request = ();
    type Response = ();

    async fn execute(
        &self,
        context: &AuthorizationContext,
        _request: Self::Request,
    ) -> Result<Self::Response, DataAccessError> {
        tracing::trace!(
            "hierarchical pipeline: validate, cache_lookup, repository_lookup, algorithm, cache_population"
        );
        let _ = context;
        Ok(())
    }
}
