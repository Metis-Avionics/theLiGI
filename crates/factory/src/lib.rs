use std::sync::Arc;
use theligi_algorithm::Algorithm;
use theligi_authorization::AuthorizationProvider;
use theligi_cache::Cache;
use theligi_data_access::DataAccess;
use theligi_repository::Repository;

/// Dependency injection container composing repository, cache, algorithm, and authorizer.
#[derive(Default)]
pub struct Factory {
    repository: Option<Arc<dyn Repository<Data = ()>>>,
    cache: Option<Arc<dyn Cache<Value = ()>>>,
    algorithm: Option<Arc<dyn Algorithm<Input = (), Output = ()>>>,
    authorizer: Option<Arc<dyn AuthorizationProvider>>,
    data_access: Option<Arc<dyn DataAccess<Request = (), Response = ()>>>,
}

impl Factory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_repository(mut self, repository: Arc<dyn Repository<Data = ()>>) -> Self {
        self.repository = Some(repository);
        self
    }

    pub fn with_cache(mut self, cache: Arc<dyn Cache<Value = ()>>) -> Self {
        self.cache = Some(cache);
        self
    }

    pub fn with_algorithm(
        mut self,
        algorithm: Arc<dyn Algorithm<Input = (), Output = ()>>,
    ) -> Self {
        self.algorithm = Some(algorithm);
        self
    }

    pub fn with_authorizer(mut self, authorizer: Arc<dyn AuthorizationProvider>) -> Self {
        self.authorizer = Some(authorizer);
        self
    }

    pub fn with_data_access(
        mut self,
        data_access: Arc<dyn DataAccess<Request = (), Response = ()>>,
    ) -> Self {
        self.data_access = Some(data_access);
        self
    }

    pub fn repository(&self) -> Option<Arc<dyn Repository<Data = ()>>> {
        self.repository.clone()
    }

    pub fn cache(&self) -> Option<Arc<dyn Cache<Value = ()>>> {
        self.cache.clone()
    }

    pub fn algorithm(&self) -> Option<Arc<dyn Algorithm<Input = (), Output = ()>>> {
        self.algorithm.clone()
    }

    pub fn authorizer(&self) -> Option<Arc<dyn AuthorizationProvider>> {
        self.authorizer.clone()
    }

    pub fn data_access(&self) -> Option<Arc<dyn DataAccess<Request = (), Response = ()>>> {
        self.data_access.clone()
    }
}
