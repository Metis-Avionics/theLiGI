//! # theligi-validation
//!
//! Series invariants: every `ContentSeries` must contain exactly 6 posts,
//! share a common topic/thesis, maintain causal consistency between
//! claims, and have verifiable lineage for each artifact.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type NodeId = Uuid;
pub type SeriesId = Uuid;

/// Result of validating a single ContentSeries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub series_id: SeriesId,
    pub passed: bool,
    pub violations: Vec<Violation>,
}

impl ValidationResult {
    pub fn ok(series_id: SeriesId) -> Self {
        Self {
            series_id,
            passed: true,
            violations: Vec::new(),
        }
    }

    pub fn failed(series_id: SeriesId, violations: Vec<Violation>) -> Self {
        Self {
            series_id,
            passed: false,
            violations,
        }
    }
}

/// A single invariant violation detected during validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    pub rule: ViolationRule,
    pub message: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ViolationRule {
    PostCount,
    SharedTopic,
    SharedThesis,
    CausalConsistency,
    Lineage,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

/// Aggregate validation report over a collection of series.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidationReport {
    pub total_series: usize,
    pub passed: usize,
    pub failed: usize,
    pub results: Vec<ValidationResult>,
    pub all_violations: Vec<Violation>,
}

/// Errors produced during validation.
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("series not found: {0}")]
    SeriesNotFound(SeriesId),

    #[error("invariant violation: {0}")]
    Invariant(String),
}

pub type ValidationResultType<T> = std::result::Result<T, ValidationError>;

/// Trait for validating ContentSeries invariants.
#[async_trait::async_trait]
pub trait SeriesValidator: Send + Sync {
    async fn validate_series(&self, series_id: SeriesId) -> ValidationResultType<ValidationResult>;

    async fn validate_batch(
        &self,
        series_ids: &[SeriesId],
    ) -> ValidationResultType<ValidationReport>;

    async fn validate_post_count(
        &self,
        series_id: SeriesId,
        expected: usize,
    ) -> ValidationResultType<bool>;

    async fn validate_shared_topic(&self, series_id: SeriesId) -> ValidationResultType<bool>;

    async fn validate_shared_thesis(&self, series_id: SeriesId) -> ValidationResultType<bool>;

    async fn validate_causal_consistency(&self, series_id: SeriesId) -> ValidationResultType<bool>;

    async fn validate_lineage(&self, series_id: SeriesId) -> ValidationResultType<bool>;
}

pub mod report;
pub mod rules;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn violation_rule_display() {
        assert_eq!(format!("{:?}", ViolationRule::PostCount), "PostCount");
        assert_eq!(format!("{:?}", ViolationRule::SharedTopic), "SharedTopic");
    }

    #[test]
    fn severity_ordering() {
        assert!(Severity::Warning > Severity::Error);
        assert!(Severity::Info > Severity::Warning);
        assert!(Severity::Info > Severity::Error);
    }

    #[test]
    fn validation_result_failed() {
        let result = ValidationResult::failed(
            Uuid::new_v4(),
            vec![Violation {
                rule: ViolationRule::PostCount,
                message: "expected 6".into(),
                severity: Severity::Error,
            }],
        );
        assert!(!result.passed);
        assert_eq!(result.violations.len(), 1);
    }
}
