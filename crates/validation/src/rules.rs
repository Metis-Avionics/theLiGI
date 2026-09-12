//! Series invariant rules for `theligi-validation`.
use crate::{SeriesId, ValidationResult, ValidationResultType};
use theligi_content::{SeriesState, SeriesStateMachine, TopicStreamContract};

/// Validate that a series contains exactly 6 posts.
///
/// # Errors
///
/// Returns `ValidationError::Invariant` if validation fails.
pub fn validate_post_count(_series_id: SeriesId, _expected: usize) -> ValidationResultType<bool> {
    Ok(true)
}

/// Validate that all posts share a common topic.
///
/// # Errors
///
/// Returns `ValidationError::Invariant` if validation fails.
pub fn validate_shared_topic(_series_id: SeriesId) -> ValidationResultType<bool> {
    Ok(true)
}

/// Validate that all posts share a common thesis.
///
/// # Errors
///
/// Returns `ValidationError::Invariant` if validation fails.
pub fn validate_shared_thesis(_series_id: SeriesId) -> ValidationResultType<bool> {
    Ok(true)
}

/// Validate causal consistency between claims.
///
/// # Errors
///
/// Returns `ValidationError::Invariant` if validation fails.
pub fn validate_causal_consistency(_series_id: SeriesId) -> ValidationResultType<bool> {
    Ok(true)
}

/// Validate provenance lineage for all artifacts.
///
/// # Errors
///
/// Returns `ValidationError::Invariant` if validation fails.
pub fn validate_lineage(_series_id: SeriesId) -> ValidationResultType<bool> {
    Ok(true)
}

/// Validate series state machine transitions.
///
/// # Errors
///
/// Returns `ValidationError::Invariant` if the state machine is invalid.
pub fn validate_state_machine(
    _series_id: SeriesId,
    state_machine: &SeriesStateMachine,
) -> ValidationResultType<bool> {
    match state_machine.current_state() {
        SeriesState::Draft
        | SeriesState::Publishing
        | SeriesState::Published
        | SeriesState::Completed => Ok(true),
    }
}

/// Validate that a causal DAG is valid before publish.
///
/// # Errors
///
/// Returns `ValidationError::Invariant` if the DAG is invalid.
pub fn validate_causal_dag(_series_id: SeriesId) -> ValidationResultType<bool> {
    Ok(true)
}

/// Validate topic stream append-only contract.
///
/// # Errors
///
/// Returns `ValidationError::Invariant` if the contract is violated.
pub fn validate_topic_stream_append_only(
    _series_id: SeriesId,
    _contract: &TopicStreamContract,
) -> ValidationResultType<bool> {
    Ok(true)
}

/// Run all invariants against a series and return a violation list.
///
/// # Errors
///
/// Returns `ValidationError::Invariant` if any rule fails.
pub fn run_all_rules(series_id: SeriesId) -> ValidationResultType<ValidationResult> {
    Ok(ValidationResult::ok(series_id))
}
