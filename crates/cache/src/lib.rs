use dashmap::DashMap;
use std::sync::Arc;
use thiserror::Error;

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

impl std::fmt::Display for CacheTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheTier::L0 => write!(f, "L0"),
            CacheTier::L1 => write!(f, "L1"),
            CacheTier::L2 => write!(f, "L2"),
            CacheTier::L3 => write!(f, "L3"),
            CacheTier::L4 => write!(f, "L4"),
            CacheTier::L5 => write!(f, "L5"),
        }
    }
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

    #[error("cache tier unavailable: {0}")]
    Unavailable(CacheTier),

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
    tiers: Arc<DashMap<(CacheKey, CacheTier), CacheEntry<V>>>,
}

impl<V> InMemoryCache<V> {
    pub fn new() -> Self {
        Self {
            tiers: Arc::new(DashMap::new()),
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
        Ok(self
            .tiers
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
        self.tiers.insert(
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
        self.tiers.remove(&(key.clone(), tier));
        Ok(())
    }

    async fn delete_prefix(&self, prefix: &str) -> Result<u64, CacheError> {
        let keys_to_remove: Vec<(CacheKey, CacheTier)> = self
            .tiers
            .iter()
            .filter(|entry| {
                entry.key().0.namespace.starts_with(prefix) || entry.key().0.key.starts_with(prefix)
            })
            .map(|entry| entry.key().clone())
            .collect();
        for key in &keys_to_remove {
            self.tiers.remove(key);
        }
        Ok(keys_to_remove.len() as u64)
    }

    async fn clear(&self, _tier: Option<CacheTier>) -> Result<(), CacheError> {
        self.tiers.clear();
        Ok(())
    }
}

use async_trait::async_trait;
use daf_cache::HierarchicalCache;
use daf_core::Cache as DafCache;

/// Adapter that wraps a `daf_cache::HierarchicalCache` and implements
/// `theligi_cache::Cache`, routing tier-aware operations to the appropriate
/// underlying tier.
#[derive(Debug, Clone)]
pub struct HierarchicalCacheWrapper<V> {
    inner: HierarchicalCache,
    _marker: std::marker::PhantomData<V>,
}

impl<V> HierarchicalCacheWrapper<V> {
    #[must_use]
    pub fn new(
        l0: Option<Arc<dyn DafCache>>,
        l1: Arc<dyn DafCache>,
        l2: Arc<dyn DafCache>,
        l3: Arc<dyn DafCache>,
        l4: Arc<dyn DafCache>,
        l5: Option<Arc<dyn DafCache>>,
    ) -> Self {
        Self {
            inner: HierarchicalCache::new(l0, l1, l2, l3, l4, l5),
            _marker: std::marker::PhantomData,
        }
    }

    #[must_use]
    pub fn inner(&self) -> &HierarchicalCache {
        &self.inner
    }
}

#[async_trait]
impl<V> Cache for HierarchicalCacheWrapper<V>
where
    V: std::any::Any + Send + Sync + Clone + 'static,
{
    type Value = V;

    async fn get(
        &self,
        key: &CacheKey,
        tier: CacheTier,
    ) -> Result<Option<Self::Value>, CacheError> {
        let key_str = format!("{}:{}", key.namespace, key.key);
        let entry = match tier {
            CacheTier::L0 => {
                let l0 = self
                    .inner
                    .l0()
                    .ok_or_else(|| CacheError::Unavailable(CacheTier::L0))?;
                l0.get(&key_str).await?
            }
            CacheTier::L1 => self.inner.l1().get(&key_str).await?,
            CacheTier::L2 => self.inner.l2().get(&key_str).await?,
            CacheTier::L3 => self.inner.l3().get(&key_str).await?,
            CacheTier::L4 => self.inner.l4().get(&key_str).await?,
            CacheTier::L5 => {
                let l5 = self
                    .inner
                    .l5()
                    .ok_or_else(|| CacheError::Unavailable(CacheTier::L5))?;
                l5.get(&key_str).await?
            }
        };
        match entry {
            Some(e) => {
                let arc = e.value.downcast::<V>().map_err(|_| {
                    CacheError::Internal(format!(
                        "type mismatch in cache; expected {}",
                        std::any::type_name::<V>()
                    ))
                })?;
                Ok(Some((*arc).clone()))
            }
            None => Ok(None),
        }
    }

    async fn set(
        &self,
        key: CacheKey,
        value: Self::Value,
        tier: CacheTier,
        ttl_seconds: Option<u64>,
    ) -> Result<(), CacheError> {
        let key_str = format!("{}:{}", key.namespace, key.key);
        let any: Arc<dyn std::any::Any + Send + Sync> = Arc::new(value);
        match tier {
            CacheTier::L0 => {
                let l0 = self
                    .inner
                    .l0()
                    .ok_or_else(|| CacheError::Unavailable(CacheTier::L0))?;
                l0.set(key_str, any).await?;
            }
            CacheTier::L1 => self.inner.l1().set(key_str, any).await?,
            CacheTier::L2 => self.inner.l2().set(key_str, any).await?,
            CacheTier::L3 => self.inner.l3().set(key_str, any).await?,
            CacheTier::L4 => self.inner.l4().set(key_str, any).await?,
            CacheTier::L5 => {
                let l5 = self
                    .inner
                    .l5()
                    .ok_or_else(|| CacheError::Unavailable(CacheTier::L5))?;
                l5.set(key_str, any).await?;
            }
        }
        if ttl_seconds.is_some() {
            tracing::debug!(
                tier = ?tier,
                "ttl_seconds discarded; daf-cache tiers do not support TTL"
            );
        }
        Ok(())
    }

    async fn delete(&self, key: &CacheKey, tier: CacheTier) -> Result<(), CacheError> {
        let key_str = format!("{}:{}", key.namespace, key.key);
        match tier {
            CacheTier::L0 => {
                let l0 = self
                    .inner
                    .l0()
                    .ok_or_else(|| CacheError::Unavailable(CacheTier::L0))?;
                l0.delete(&key_str).await?;
            }
            CacheTier::L1 => self.inner.l1().delete(&key_str).await?,
            CacheTier::L2 => self.inner.l2().delete(&key_str).await?,
            CacheTier::L3 => self.inner.l3().delete(&key_str).await?,
            CacheTier::L4 => self.inner.l4().delete(&key_str).await?,
            CacheTier::L5 => {
                let l5 = self
                    .inner
                    .l5()
                    .ok_or_else(|| CacheError::Unavailable(CacheTier::L5))?;
                l5.delete(&key_str).await?;
            }
        }
        Ok(())
    }

    async fn delete_prefix(&self, prefix: &str) -> Result<u64, CacheError> {
        Ok(DafCache::delete_prefix(&self.inner, prefix).await?)
    }

    async fn clear(&self, _tier: Option<CacheTier>) -> Result<(), CacheError> {
        DafCache::clear(&self.inner).await?;
        Ok(())
    }
}

impl From<daf_core::CacheError> for CacheError {
    fn from(e: daf_core::CacheError) -> Self {
        CacheError::Internal(e.to_string())
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

    #[tokio::test]
    async fn hierarchical_wrapper_set_get_round_trip() {
        let l0 = Arc::new(daf_cache::MemoryCache::new(1024)) as Arc<dyn DafCache>;
        let l1 = Arc::new(daf_cache::MemoryCache::new(1024)) as Arc<dyn DafCache>;
        let l2 = Arc::new(daf_cache::MokaCache::new(1024)) as Arc<dyn DafCache>;
        let l5 = Arc::new(daf_cache::MemoryCache::new(1024)) as Arc<dyn DafCache>;
        let cache = HierarchicalCacheWrapper::<String>::new(
            Some(l0.clone()),
            l1.clone(),
            l2,
            l1.clone(),
            l1.clone(),
            Some(l5),
        );
        let key = CacheKey::new("ns", "key");
        cache
            .set(key.clone(), "value".to_string(), CacheTier::L1, None)
            .await
            .unwrap();
        let result: Option<String> = cache.get(&key, CacheTier::L1).await.unwrap();
        assert_eq!(result, Some("value".to_string()));
    }

    #[tokio::test]
    async fn hierarchical_wrapper_l0_unavailable() {
        let l1 = Arc::new(daf_cache::MemoryCache::new(1024)) as Arc<dyn DafCache>;
        let cache = HierarchicalCacheWrapper::<String>::new(
            None,
            l1,
            Arc::new(daf_cache::MemoryCache::new(1024)) as Arc<dyn DafCache>,
            Arc::new(daf_cache::MemoryCache::new(1024)) as Arc<dyn DafCache>,
            Arc::new(daf_cache::MemoryCache::new(1024)) as Arc<dyn DafCache>,
            None,
        );
        let key = CacheKey::new("ns", "key");
        let result = cache.get(&key, CacheTier::L0).await;
        assert!(matches!(
            result,
            Err(CacheError::Unavailable(CacheTier::L0))
        ));
    }

    #[tokio::test]
    async fn hierarchical_wrapper_l5_unavailable() {
        let l1 = Arc::new(daf_cache::MemoryCache::new(1024)) as Arc<dyn DafCache>;
        let cache = HierarchicalCacheWrapper::<String>::new(
            Some(Arc::new(daf_cache::MemoryCache::new(1024)) as Arc<dyn DafCache>),
            l1,
            Arc::new(daf_cache::MemoryCache::new(1024)) as Arc<dyn DafCache>,
            Arc::new(daf_cache::MemoryCache::new(1024)) as Arc<dyn DafCache>,
            Arc::new(daf_cache::MemoryCache::new(1024)) as Arc<dyn DafCache>,
            None,
        );
        let key = CacheKey::new("ns", "key");
        let result = cache.get(&key, CacheTier::L5).await;
        assert!(matches!(
            result,
            Err(CacheError::Unavailable(CacheTier::L5))
        ));
    }
}
