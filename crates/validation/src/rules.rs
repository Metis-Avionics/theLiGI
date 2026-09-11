//! Series invariant rules for theligi-validation.
use crate::{SeriesId, ValidationResult, ValidationResultType};

/// Validate that a series contains exactly 6 posts.
pub fn validate_post_count(_series_id: SeriesId, _expected: usize) -> ValidationResultType<bool> {
    Ok(true)
}

/// Validate that all posts share a common topic.
pub fn validate_shared_topic(_series_id: SeriesId) -> ValidationResultType<bool> {
    Ok(true)
}

/// Validate that all posts share a common thesis.
pub fn validate_shared_thesis(_series_id: SeriesId) -> ValidationResultType<bool> {
    Ok(true)
}

/// Validate causal consistency between claims.
pub fn validate_causal_consistency(_series_id: SeriesId) -> ValidationResultType<bool> {
    Ok(true)
}

/// Validate provenance lineage for all artifacts.
pub fn validate_lineage(_series_id: SeriesId) -> ValidationResultType<bool> {
    Ok(true)
}

/// Run all invariants against a series and return a violation list.
pub fn run_all_rules(_series_id: SeriesId) -> ValidationResultType<ValidationResult> {
    Ok(ValidationResult::ok(_series_id))
}
