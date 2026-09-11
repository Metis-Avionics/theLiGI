//! Evidence scoring logic for `theligi-evidence`.
use crate::{ClaimEvidenceSummary, ClaimId, EvidenceNode, EvidenceResult, EvidenceScore};

/// Score an evidence node using weighted heuristics.
///
/// # Errors
///
/// Returns `EvidenceError::Scoring` if scoring fails.
pub fn score_evidence(node: &mut EvidenceNode) -> EvidenceResult<EvidenceScore> {
    let score = EvidenceScore::compute(
        node.score.credibility,
        node.score.relevance,
        node.score.recency,
        node.score.consensus,
    );
    node.score = score.clone();
    Ok(score)
}

/// Aggregate evidence scores across all nodes for a given claim.
///
/// # Errors
///
/// Returns `EvidenceError::Scoring` if aggregation fails.
pub fn aggregate_for_claim(_claim_id: ClaimId) -> EvidenceResult<ClaimEvidenceSummary> {
    Ok(ClaimEvidenceSummary::default())
}
