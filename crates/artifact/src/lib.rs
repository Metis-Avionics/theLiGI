//! theligi-artifact: versioned, hashed (blake3), schema_versioned, lineage_required, rollback_supported artifacts.

use thiserror::Error;
use uuid::Uuid;

/// Errors that can occur while managing artifacts.
#[derive(Error, Debug)]
pub enum ArtifactError {
    #[error("versioning error: {0}")]
    Versioning(String),
    #[error("hash mismatch: expected={expected}, actual={actual}")]
    HashMismatch { expected: String, actual: String },
    #[error("lineage error: {0}")]
    Lineage(String),
    #[error("rollback error: {0}")]
    Rollback(String),
}

pub type Result<T> = std::result::Result<T, ArtifactError>;

/// An immutable, versioned artifact.
#[derive(Debug, Clone)]
pub struct Artifact {
    pub id: Uuid,
    pub version: String,
    pub schema_version: u32,
    pub blake3_hash: String,
    pub lineage: Vec<Uuid>,
    pub content: Vec<u8>,
}

impl Artifact {
    pub fn new(version: impl Into<String>, schema_version: u32, content: Vec<u8>) -> Result<Self> {
        let blake3_hash = blake3::hash(&content).to_string();
        Ok(Self {
            id: Uuid::new_v4(),
            version: version.into(),
            schema_version,
            blake3_hash,
            lineage: Vec::new(),
            content,
        })
    }

    pub fn with_lineage(mut self, lineage: Vec<Uuid>) -> Self {
        self.lineage = lineage;
        self
    }
}

/// Trait for managing artifacts.
#[async_trait::async_trait]
pub trait ArtifactStore {
    async fn store(&self, artifact: Artifact) -> Result<Artifact>;
    async fn load(&self, id: Uuid, version: &str) -> Result<Artifact>;
    async fn rollback(&self, id: Uuid, to_version: &str) -> Result<Artifact>;
}
