//! Discrete content-series sub-model for theligi-content.
use crate::{Claim, ContentArtifact, ContentNodeId, ContentResult, ContentSeries, Evidence};

/// Build a series skeleton with N placeholder artifacts.
pub fn build_series_skeleton(
    title: impl Into<String>,
    thesis: impl Into<String>,
    post_count: usize,
) -> ContentResult<ContentSeries> {
    let mut series = ContentSeries::new(title, thesis);
    for i in 0..post_count {
        let artifact = ContentArtifact::new(series.id, format!("post-{}", i), i);
        series.post_ids.push(artifact.id);
    }
    Ok(series)
}

/// Attach a claim to a specific artifact in the series.
pub fn attach_claim(artifact_id: ContentNodeId, text: impl Into<String>) -> Claim {
    Claim::new(artifact_id, text)
}

/// Attach evidence to an existing claim.
pub fn attach_evidence(
    claim_id: ContentNodeId,
    excerpt: impl Into<String>,
    supports: bool,
) -> Evidence {
    Evidence::new(claim_id, excerpt, supports)
}
