use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

/// Cache tier levels from fastest/most local to slowest/most distributed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum CacheTier {
    L0 = 0,
    L1 = 1,
    L2 = 2,
    L3 = 3,
    L4 = 4,
    L5 = 5,
}

/// Canonical cache key.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CacheKey {
    pub namespace: String,
    pub key: String,
}

impl CacheKey {
    pub fn new(namespace: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            key: key.into(),
        }
    }
}

impl std::fmt::Display for CacheKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.key)
    }
}

/// Typed cache errors.
#[derive(Debug, Error)]
pub enum CacheError {
    #[error("key not found: {0}")]
    NotFound(CacheKey),

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("internal error: {0}")]
    Internal(String),
}

/// Cache entry with tier and expiry metadata.
#[derive(Debug, Clone)]
pub struct CacheEntry<V> {
    pub value: V,
    pub tier: CacheTier,
    pub ttl_seconds: Option<u64>,
}

/// Cache trait supporting L0-L5 tiers.
#[async_trait::async_trait]
pub trait Cache: Send + Sync + 'static {
    type Value: Send + Sync + 'static;

    async fn get(&self, key: &CacheKey, tier: CacheTier)
        -> Result<Option<Self::Value>, CacheError>;

    async fn set(
        &self,
        key: CacheKey,
        value: Self::Value,
        tier: CacheTier,
        ttl_seconds: Option<u64>,
    ) -> Result<(), CacheError>;

    async fn delete(&self, key: &CacheKey, tier: CacheTier) -> Result<(), CacheError>;

    async fn delete_prefix(&self, prefix: &str) -> Result<u64, CacheError>;

    async fn clear(&self, tier: Option<CacheTier>) -> Result<(), CacheError>;
}

/// In-memory cache implementation for testing.
#[derive(Debug, Default)]
pub struct InMemoryCache<V> {
    tiers: Arc<RwLock<HashMap<(CacheKey, CacheTier), CacheEntry<V>>>>,
}

impl<V> InMemoryCache<V> {
    pub fn new() -> Self {
        Self {
            tiers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl<V> Cache for InMemoryCache<V>
where
    V: Send + Sync + Clone + serde::Serialize + serde::de::DeserializeOwned + 'static,
{
    type Value = V;

    async fn get(
        &self,
        key: &CacheKey,
        tier: CacheTier,
    ) -> Result<Option<Self::Value>, CacheError> {
        let tiers = self.tiers.read().await;
        Ok(tiers
            .get(&(key.clone(), tier))
            .map(|entry| entry.value.clone()))
    }

    async fn set(
        &self,
        key: CacheKey,
        value: Self::Value,
        tier: CacheTier,
        ttl_seconds: Option<u64>,
    ) -> Result<(), CacheError> {
        self.tiers.write().await.insert(
            (key, tier),
            CacheEntry {
                value,
                tier,
                ttl_seconds,
            },
        );
        Ok(())
    }

    async fn delete(&self, key: &CacheKey, tier: CacheTier) -> Result<(), CacheError> {
        self.tiers.write().await.remove(&(key.clone(), tier));
        Ok(())
    }

    async fn delete_prefix(&self, prefix: &str) -> Result<u64, CacheError> {
        let mut tiers = self.tiers.write().await;
        let initial = tiers.len();
        tiers.retain(|key, _| {
            !key.0.namespace.starts_with(prefix) && !key.0.key.starts_with(prefix)
        });
        Ok((initial - tiers.len()) as u64)
    }

    async fn clear(&self, _tier: Option<CacheTier>) -> Result<(), CacheError> {
        self.tiers.write().await.clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn set_and_get() {
        let cache = InMemoryCache::new();
        let key = CacheKey::new("ns", "key");
        cache
            .set(key.clone(), "value".to_string(), CacheTier::L1, None)
            .await
            .unwrap();
        let result: Option<String> = cache.get(&key, CacheTier::L1).await.unwrap();
        assert_eq!(result, Some("value".to_string()));
    }

    #[tokio::test]
    async fn get_miss() {
        let cache = InMemoryCache::<String>::new();
        let key = CacheKey::new("ns", "missing");
        let result: Option<String> = cache.get(&key, CacheTier::L1).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn delete_entry() {
        let cache = InMemoryCache::new();
        let key = CacheKey::new("ns", "key");
        cache
            .set(key.clone(), "value".to_string(), CacheTier::L1, None)
            .await
            .unwrap();
        cache.delete(&key, CacheTier::L1).await.unwrap();
        let result: Option<String> = cache.get(&key, CacheTier::L1).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn delete_prefix() {
        let cache = InMemoryCache::new();
        cache
            .set(CacheKey::new("ns", "a:1"), 1i32, CacheTier::L1, None)
            .await
            .unwrap();
        cache
            .set(CacheKey::new("ns", "a:2"), 2i32, CacheTier::L1, None)
            .await
            .unwrap();
        cache
            .set(CacheKey::new("ns", "b:1"), 3i32, CacheTier::L1, None)
            .await
            .unwrap();
        let deleted = cache.delete_prefix("a:").await.unwrap();
        assert_eq!(deleted, 2);
    }
}
