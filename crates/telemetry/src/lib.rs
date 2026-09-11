//! theligi-telemetry: first-class, immutable, versioned telemetry observations.

use thiserror::Error;
use uuid::Uuid;

/// Errors that can occur while recording telemetry.
#[derive(Error, Debug)]
pub enum TelemetryError {
    #[error("observation error: {0}")]
    Observation(String),
}

pub type Result<T> = std::result::Result<T, TelemetryError>;

/// A versioned, immutable telemetry observation.
#[derive(Debug, Clone)]
pub struct Observation {
    pub id: Uuid,
    pub schema_version: u32,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub payload: serde_json::Value,
}

impl Observation {
    pub fn new(schema_version: u32, payload: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            schema_version,
            timestamp: chrono::Utc::now(),
            payload,
        }
    }
}

/// Trait for recording and querying telemetry observations.
#[async_trait::async_trait]
pub trait TelemetryStore {
    async fn record(&self, observation: Observation) -> Result<Observation>;
    async fn query(&self, since: chrono::DateTime<chrono::Utc>) -> Result<Vec<Observation>>;
}
