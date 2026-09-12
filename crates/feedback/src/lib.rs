//! theligi-feedback: closed-loop pipeline (observe -> normalize -> persist -> query -> infer -> rank -> generate -> deploy -> measure -> adapt).

use thiserror::Error;
use uuid::Uuid;

pub mod adaptation;

/// Errors that can occur during feedback pipeline processing.
#[derive(Error, Debug)]
pub enum FeedbackError {
    #[error("pipeline stage '{0}' failed: {1}")]
    StageFailed(String, String),
    #[error("storage error: {0}")]
    Storage(String),
    #[error("inference error: {0}")]
    Inference(String),
}

pub type Result<T> = std::result::Result<T, FeedbackError>;

/// Represents a single feedback signal.
#[derive(Debug, Clone)]
pub struct FeedbackSignal {
    pub id: Uuid,
    pub source: String,
    pub payload: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Trait representing the closed-loop feedback pipeline.
#[async_trait::async_trait]
pub trait FeedbackPipeline {
    async fn observe(&self, signal: FeedbackSignal) -> Result<FeedbackSignal>;
    async fn normalize(&self, signal: FeedbackSignal) -> Result<FeedbackSignal>;
    async fn persist(&self, signal: FeedbackSignal) -> Result<Uuid>;
    async fn query(&self, since: chrono::DateTime<chrono::Utc>) -> Result<Vec<FeedbackSignal>>;
    async fn infer(&self, signals: Vec<FeedbackSignal>) -> Result<serde_json::Value>;
    async fn rank(&self, candidates: Vec<serde_json::Value>) -> Result<Vec<serde_json::Value>>;
    async fn generate(&self, ranked: Vec<serde_json::Value>) -> Result<String>;
    async fn deploy(&self, artifact: String) -> Result<Uuid>;
    async fn measure(&self, deployment_id: Uuid) -> Result<std::collections::HashMap<String, f64>>;
    async fn adapt(
        &self,
        deployment_id: Uuid,
        metrics: std::collections::HashMap<String, f64>,
    ) -> Result<()>;
}
