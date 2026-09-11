//! theligi-observability: structured_logs, metrics, tracing, health, correlation, causation, decision_trace.

use thiserror::Error;
use uuid::Uuid;

/// Errors that can occur in observability components.
#[derive(Error, Debug)]
pub enum ObservabilityError {
    #[error("tracing error: {0}")]
    Tracing(String),
    #[error("metrics error: {0}")]
    Metrics(String),
}

pub type Result<T> = std::result::Result<T, ObservabilityError>;

/// Correlation context propagated across service boundaries.
#[derive(Debug, Clone)]
pub struct CorrelationContext {
    pub trace_id: Uuid,
    pub span_id: Uuid,
    pub parent_span_id: Option<Uuid>,
}

impl CorrelationContext {
    pub fn new() -> Self {
        Self {
            trace_id: Uuid::new_v4(),
            span_id: Uuid::new_v4(),
            parent_span_id: None,
        }
    }
}

impl Default for CorrelationContext {
    fn default() -> Self {
        Self::new()
    }
}

/// A decision trace record for explainability and causality.
#[derive(Debug, Clone)]
pub struct DecisionTrace {
    pub id: Uuid,
    pub trace: CorrelationContext,
    pub decision: String,
    pub inputs: Vec<serde_json::Value>,
    pub outputs: Vec<serde_json::Value>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Structured log entry.
#[derive(Debug, Clone, serde::Serialize)]
pub struct LogEntry {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub level: String,
    pub message: String,
    pub trace_id: Option<Uuid>,
    #[serde(flatten)]
    pub fields: std::collections::HashMap<String, serde_json::Value>,
}
