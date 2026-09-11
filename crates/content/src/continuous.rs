//! Continuous topic-rooted temporal stream for `theligi-content`.
use crate::{ContentNodeId, ContinuousEdgeKind, TemporalEdge, TopicNode};

/// Link two nodes chronologically.
#[must_use]
pub fn link_chronological(
    source: ContentNodeId,
    target: ContentNodeId,
    timestamp_ms: i64,
) -> TemporalEdge {
    TemporalEdge::new(
        source,
        target,
        ContinuousEdgeKind::ChronologicalAdjacency,
        timestamp_ms,
    )
}

/// Create a new topic root node.
pub fn create_topic(label: impl Into<String>) -> TopicNode {
    TopicNode::new(label)
}
