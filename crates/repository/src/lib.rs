use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Content-addressable storage identifier.
pub type CasId = Uuid;

/// Generation number for optimistic concurrency control.
pub type Generation = u64;

/// Typed errors returned by repository operations.
#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("content not found: {0}")]
    NotFound(CasId),

    #[error("generation conflict: expected {expected}, got {actual}")]
    GenerationConflict {
        expected: Generation,
        actual: Generation,
    },

    #[error("cas mismatch: expected {expected}, got {actual}")]
    CasMismatch { expected: CasId, actual: CasId },

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("internal error: {0}")]
    Internal(String),
}

/// Stored content entry with generation tracking.
#[derive(Debug, Clone)]
pub struct StoredEntry<T> {
    pub id: CasId,
    pub generation: Generation,
    pub data: T,
    pub metadata: HashMap<String, String>,
}

/// Content-addressable repository trait.
#[async_trait::async_trait]
pub trait Repository: Send + Sync + 'static {
    type Data: Send + Sync + 'static;

    /// Create a new entry in the repository.
    async fn create(&self, data: Self::Data) -> Result<CasId, RepositoryError>;

    /// Retrieve an entry by its CAS id.
    async fn get(&self, id: CasId) -> Result<StoredEntry<Self::Data>, RepositoryError>;

    /// Save an existing entry with an updated generation.
    async fn save(&self, entry: StoredEntry<Self::Data>) -> Result<Generation, RepositoryError>;

    /// Delete an entry by its CAS id.
    async fn delete(&self, id: CasId) -> Result<(), RepositoryError>;

    /// Update an entry only if the provided generation matches.
    async fn try_update(
        &self,
        id: CasId,
        generation: Generation,
        data: Self::Data,
    ) -> Result<Generation, RepositoryError>;

    /// Delete an entry only if the provided generation matches.
    async fn try_delete(&self, id: CasId, generation: Generation) -> Result<(), RepositoryError>;
}

/// In-memory repository implementation for testing and prototyping.
#[derive(Debug, Default)]
pub struct InMemoryRepository<T> {
    entries: Arc<RwLock<HashMap<CasId, StoredEntry<T>>>>,
}

impl<T> InMemoryRepository<T> {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl<T> Repository for InMemoryRepository<T>
where
    T: Send + Sync + Clone + 'static,
{
    type Data = T;

    async fn create(&self, data: Self::Data) -> Result<CasId, RepositoryError> {
        let id = Uuid::new_v4();
        let entry = StoredEntry {
            id,
            generation: 0,
            data,
            metadata: HashMap::new(),
        };
        self.entries.write().await.insert(id, entry);
        Ok(id)
    }

    async fn get(&self, id: CasId) -> Result<StoredEntry<Self::Data>, RepositoryError> {
        let entries = self.entries.read().await;
        entries
            .get(&id)
            .cloned()
            .ok_or(RepositoryError::NotFound(id))
    }

    async fn save(&self, entry: StoredEntry<Self::Data>) -> Result<Generation, RepositoryError> {
        let mut entries = self.entries.write().await;
        let next_generation = entry.generation + 1;
        let mut updated = entry;
        updated.generation = next_generation;
        entries.insert(updated.id, updated);
        Ok(next_generation)
    }

    async fn delete(&self, id: CasId) -> Result<(), RepositoryError> {
        self.entries.write().await.remove(&id);
        Ok(())
    }

    async fn try_update(
        &self,
        id: CasId,
        generation: Generation,
        data: Self::Data,
    ) -> Result<Generation, RepositoryError> {
        let mut entries = self.entries.write().await;
        let entry = entries.get_mut(&id).ok_or(RepositoryError::NotFound(id))?;
        if entry.generation != generation {
            return Err(RepositoryError::GenerationConflict {
                expected: generation,
                actual: entry.generation,
            });
        }
        entry.generation += 1;
        entry.data = data;
        Ok(entry.generation)
    }

    async fn try_delete(&self, id: CasId, generation: Generation) -> Result<(), RepositoryError> {
        let entries = self.entries.read().await;
        let entry = entries.get(&id).ok_or(RepositoryError::NotFound(id))?;
        if entry.generation != generation {
            return Err(RepositoryError::GenerationConflict {
                expected: generation,
                actual: entry.generation,
            });
        }
        drop(entries);
        self.entries.write().await.remove(&id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_and_get() {
        let repo = InMemoryRepository::new();
        let id = repo.create("hello".to_string()).await.unwrap();
        let entry = repo.get(id).await.unwrap();
        assert_eq!(entry.data, "hello");
        assert_eq!(entry.generation, 0);
    }

    #[tokio::test]
    async fn get_not_found() {
        let repo = InMemoryRepository::<String>::new();
        let result = repo.get(Uuid::new_v4()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn save_increments_generation() {
        let repo = InMemoryRepository::new();
        let id = repo.create("v1".to_string()).await.unwrap();
        let entry = repo.get(id).await.unwrap();
        let new_gen = repo.save(entry).await.unwrap();
        assert_eq!(new_gen, 1);
    }

    #[tokio::test]
    async fn try_update_success() {
        let repo = InMemoryRepository::new();
        let id = repo.create("v1".to_string()).await.unwrap();
        let entry = repo.get(id).await.unwrap();
        let new_gen = repo.try_update(id, 0, "v2".to_string()).await.unwrap();
        assert_eq!(new_gen, 1);
        let updated = repo.get(id).await.unwrap();
        assert_eq!(updated.data, "v2");
    }

    #[tokio::test]
    async fn try_update_generation_conflict() {
        let repo = InMemoryRepository::new();
        let id = repo.create("v1".to_string()).await.unwrap();
        let result = repo.try_update(id, 99, "v2".to_string()).await;
        assert!(matches!(
            result,
            Err(RepositoryError::GenerationConflict { .. })
        ));
    }

    #[tokio::test]
    async fn try_delete_success() {
        let repo = InMemoryRepository::new();
        let id = repo.create("v1".to_string()).await.unwrap();
        let entry = repo.get(id).await.unwrap();
        repo.try_delete(id, entry.generation).await.unwrap();
        let result = repo.get(id).await;
        assert!(result.is_err());
    }
}
