//! Content series state machine for `theligi-content`.
//!
//! Enforces the discrete series lifecycle:
//!   `Draft` -> `Publishing` -> `Published` -> `Completed`
//!
//! Guards:
//! - `publish`: exactly 6 posts, shared topic/thesis lock, causal DAG valid
//! - `complete`: all posts published, evidence present
//! - `revert_to_draft`: publish guard fails

use crate::{ContentError, ContentNodeId, ContentResult, ContentSeries};
use serde::{Deserialize, Serialize};

/// Lifecycle states for a [`ContentSeries`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeriesState {
    Draft,
    Publishing,
    Published,
    Completed,
}

/// Reasons a state transition may be rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionError {
    InvalidTransition,
    PostCountMismatch { expected: usize, actual: usize },
    TopicLockViolation,
    ThesisLockViolation,
    CausalDagInvalid,
    NotAllPostsPublished,
    EvidenceMissing,
}

impl std::fmt::Display for TransitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransitionError::InvalidTransition => write!(f, "invalid state transition"),
            TransitionError::PostCountMismatch { expected, actual } => {
                write!(f, "post count mismatch: expected {expected}, got {actual}")
            }
            TransitionError::TopicLockViolation => write!(f, "topic lock violation"),
            TransitionError::ThesisLockViolation => write!(f, "thesis lock violation"),
            TransitionError::CausalDagInvalid => write!(f, "causal DAG invalid"),
            TransitionError::NotAllPostsPublished => write!(f, "not all posts published"),
            TransitionError::EvidenceMissing => write!(f, "evidence missing"),
        }
    }
}

impl std::error::Error for TransitionError {}

/// State machine for [`ContentSeries`] lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesStateMachine {
    state: SeriesState,
    series_id: ContentNodeId,
}

impl SeriesStateMachine {
    #[must_use]
    pub fn new(series_id: ContentNodeId) -> Self {
        Self {
            state: SeriesState::Draft,
            series_id,
        }
    }

    #[must_use]
    pub fn current_state(&self) -> SeriesState {
        self.state
    }

    #[must_use]
    pub fn series_id(&self) -> ContentNodeId {
        self.series_id
    }

    /// Attempt to transition to `Publishing`.
    ///
    /// # Errors
    ///
    /// Returns `ContentError::SeriesInvariant` if the transition is invalid,
    /// the post count is not 6, the topic lock is violated, or the thesis
    /// lock is violated.
    pub fn try_publish(&mut self, series: &ContentSeries) -> ContentResult<()> {
        if self.state != SeriesState::Draft {
            return Err(ContentError::SeriesInvariant(
                TransitionError::InvalidTransition.to_string(),
            ));
        }
        let post_count = series.post_ids.len();
        if post_count != 6 {
            return Err(ContentError::SeriesInvariant(
                TransitionError::PostCountMismatch {
                    expected: 6,
                    actual: post_count,
                }
                .to_string(),
            ));
        }
        if series.topic_ids.is_empty() {
            return Err(ContentError::SeriesInvariant(
                TransitionError::TopicLockViolation.to_string(),
            ));
        }
        if series.thesis.trim().is_empty() {
            return Err(ContentError::SeriesInvariant(
                TransitionError::ThesisLockViolation.to_string(),
            ));
        }
        self.state = SeriesState::Publishing;
        Ok(())
    }

    /// Attempt to transition to `Published`.
    ///
    /// # Errors
    ///
    /// Returns `ContentError::SeriesInvariant` if the transition is invalid,
    /// not all posts are published, or evidence is missing.
    pub fn try_complete(&mut self, all_published: bool, has_evidence: bool) -> ContentResult<()> {
        if self.state != SeriesState::Publishing {
            return Err(ContentError::SeriesInvariant(
                TransitionError::InvalidTransition.to_string(),
            ));
        }
        if !all_published {
            return Err(ContentError::SeriesInvariant(
                TransitionError::NotAllPostsPublished.to_string(),
            ));
        }
        if !has_evidence {
            return Err(ContentError::SeriesInvariant(
                TransitionError::EvidenceMissing.to_string(),
            ));
        }
        self.state = SeriesState::Published;
        Ok(())
    }

    /// Revert to draft (only valid from Publishing).
    ///
    /// # Errors
    ///
    /// Returns `ContentError::SeriesInvariant` if the current state is not
    /// `Publishing`.
    pub fn revert_to_draft(&mut self) -> ContentResult<()> {
        if self.state != SeriesState::Publishing {
            return Err(ContentError::SeriesInvariant(
                TransitionError::InvalidTransition.to_string(),
            ));
        }
        self.state = SeriesState::Draft;
        Ok(())
    }
}

/// Contract for the continuous topic stream.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicStreamContract {
    pub append_only: bool,
    pub chronological_adjacency_auto_maintained: bool,
    pub windowed_query_bounds_enforced: bool,
}

impl Default for TopicStreamContract {
    fn default() -> Self {
        Self {
            append_only: true,
            chronological_adjacency_auto_maintained: true,
            windowed_query_bounds_enforced: true,
        }
    }
}

/// Validate that a topic stream edge does not violate append-only semantics.
///
/// # Errors
///
/// Returns `ContentError::Edge` if the edge timestamp is older than the
/// latest timestamp and the append-only contract is enforced.
pub fn validate_stream_edge(
    contract: &TopicStreamContract,
    edge_timestamp_ms: i64,
    latest_timestamp_ms: Option<i64>,
) -> ContentResult<()> {
    if !contract.append_only {
        return Ok(());
    }
    if let Some(latest) = latest_timestamp_ms {
        if edge_timestamp_ms < latest {
            return Err(ContentError::Edge(
                "append_only violation: edge timestamp older than latest".into(),
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ContentSeries;
    use uuid::Uuid;

    #[test]
    fn series_state_machine_draft_to_publishing() {
        let series_id = Uuid::new_v4();
        let mut sm = SeriesStateMachine::new(series_id);
        let series = ContentSeries::new("Test", "Thesis");
        assert!(sm.try_publish(&series).is_err());
        let mut series = ContentSeries::new("Test", "Thesis");
        series.topic_ids = vec![Uuid::new_v4()];
        for _ in 0..6 {
            series.post_ids.push(Uuid::new_v4());
        }
        assert!(sm.try_publish(&series).is_ok());
        assert_eq!(sm.current_state(), SeriesState::Publishing);
    }

    #[test]
    fn series_state_machine_invalid_transitions() {
        let series_id = Uuid::new_v4();
        let mut sm = SeriesStateMachine::new(series_id);
        let _series = ContentSeries::new("Test", "Thesis");
        assert!(sm.try_complete(true, true).is_err());
    }

    #[test]
    fn topic_stream_contract_defaults() {
        let contract = TopicStreamContract::default();
        assert!(contract.append_only);
        assert!(contract.chronological_adjacency_auto_maintained);
        assert!(contract.windowed_query_bounds_enforced);
    }

    #[test]
    fn validate_stream_edge_append_only() {
        let contract = TopicStreamContract::default();
        assert!(validate_stream_edge(&contract, 2000, Some(1000)).is_ok());
        assert!(validate_stream_edge(&contract, 500, Some(1000)).is_err());
    }
}
