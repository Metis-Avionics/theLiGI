//! # theligi-content
//!
//! Two complementary content graph models:
//!
//! * **Discrete** `ContentSeries` sub-graph — a DAG of `ContentSeries`,
//!   `ContentArtifact`, `Claim` and `Evidence` nodes joined by
//!   `HAS_POST`, `NEXT_IN_SERIES`, `SUPPORTS` and `CAUSES` edges.
//! * **Continuous** `Topic`-rooted temporal stream — chronological,
//!   related-to and influence edges that model how content evolves over
//!   time.

#![deny(warnings)]
#![warn(clippy::pedantic)]
#![warn(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::missing_errors_doc
)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Shared node / edge types
// ---------------------------------------------------------------------------

pub type ContentNodeId = Uuid;
pub type ContentEdgeId = Uuid;

/// Node kinds in the discrete content-series sub-graph.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum DiscreteNodeKind {
    ContentSeries,
    ContentArtifact,
    Claim,
    Evidence,
}

/// Edge kinds in the discrete content-series sub-graph.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum DiscreteEdgeKind {
    HasPost,
    NextInSeries,
    Supports,
    Causes,
    DerivesFrom,
}

/// Node kinds in the continuous topic-rooted temporal stream.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ContinuousNodeKind {
    ContentArtifact,
    Interaction,
    Signal,
    TelemetryObservation,
}

/// Edge kinds in the continuous temporal stream.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ContinuousEdgeKind {
    ChronologicalAdjacency,
    RelatedTo,
    InfluencedBy,
}

// ---------------------------------------------------------------------------
// Discrete model
// ---------------------------------------------------------------------------

/// A [`ContentSeries`] is a top-level series node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentSeries {
    pub id: ContentNodeId,
    pub title: String,
    pub thesis: String,
    pub topic_ids: Vec<Uuid>,
    pub post_ids: Vec<ContentNodeId>,
    pub properties: HashMap<String, serde_json::Value>,
}

impl ContentSeries {
    #[must_use]
    pub fn new(title: impl Into<String>, thesis: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.into(),
            thesis: thesis.into(),
            topic_ids: Vec::new(),
            post_ids: Vec::new(),
            properties: HashMap::new(),
        }
    }
}

/// A single post / artifact within a [`ContentSeries`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentArtifact {
    pub id: ContentNodeId,
    pub series_id: ContentNodeId,
    pub title: String,
    pub body: String,
    pub sequence: usize,
    pub properties: HashMap<String, serde_json::Value>,
}

impl ContentArtifact {
    #[must_use]
    pub fn new(series_id: ContentNodeId, title: impl Into<String>, sequence: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            series_id,
            title: title.into(),
            body: String::new(),
            sequence,
            properties: HashMap::new(),
        }
    }
}

/// A claim asserted in a [`ContentArtifact`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    pub id: ContentNodeId,
    pub artifact_id: ContentNodeId,
    pub text: String,
    pub confidence: f64,
    pub properties: HashMap<String, serde_json::Value>,
}

impl Claim {
    #[must_use]
    pub fn new(artifact_id: ContentNodeId, text: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            artifact_id,
            text: text.into(),
            confidence: 0.0,
            properties: HashMap::new(),
        }
    }
}

/// Evidence supporting or refuting a Claim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: ContentNodeId,
    pub claim_id: ContentNodeId,
    pub source_url: Option<String>,
    pub excerpt: String,
    pub supports: bool,
    pub properties: HashMap<String, serde_json::Value>,
}

impl Evidence {
    #[must_use]
    pub fn new(claim_id: ContentNodeId, excerpt: impl Into<String>, supports: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            claim_id,
            source_url: None,
            excerpt: excerpt.into(),
            supports,
            properties: HashMap::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Continuous model
// ---------------------------------------------------------------------------

/// A topic root node in the temporal stream.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicNode {
    pub id: ContentNodeId,
    pub label: String,
    pub embedding: Vec<f32>,
}

impl TopicNode {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            label: label.into(),
            embedding: Vec::new(),
        }
    }
}

/// Temporal edges connecting artifacts chronologically.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalEdge {
    pub id: ContentEdgeId,
    pub source: ContentNodeId,
    pub target: ContentNodeId,
    pub kind: ContinuousEdgeKind,
    pub timestamp_ms: i64,
    pub weight: Option<f64>,
}

impl TemporalEdge {
    #[must_use]
    pub fn new(
        source: ContentNodeId,
        target: ContentNodeId,
        kind: ContinuousEdgeKind,
        timestamp_ms: i64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            source,
            target,
            kind,
            timestamp_ms,
            weight: None,
        }
    }
}

/// Errors in the content model.
#[derive(Debug, thiserror::Error)]
pub enum ContentError {
    #[error("node not found: {0}")]
    NodeNotFound(ContentNodeId),

    #[error("series invariant violation: {0}")]
    SeriesInvariant(String),

    #[error("edge error: {0}")]
    Edge(String),
}

pub type ContentResult<T> = std::result::Result<T, ContentError>;

pub mod continuous;
pub mod discrete;
pub mod model;
pub mod state_machine;

pub use state_machine::{SeriesState, SeriesStateMachine, TopicStreamContract};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discrete_edge_types_disjoint_from_continuous() {
        let discrete: std::collections::HashSet<_> = [
            DiscreteEdgeKind::HasPost,
            DiscreteEdgeKind::NextInSeries,
            DiscreteEdgeKind::Supports,
            DiscreteEdgeKind::Causes,
            DiscreteEdgeKind::DerivesFrom,
        ]
        .iter()
        .map(|e| format!("{e:?}"))
        .collect();
        let continuous: std::collections::HashSet<_> = [
            ContinuousEdgeKind::ChronologicalAdjacency,
            ContinuousEdgeKind::RelatedTo,
            ContinuousEdgeKind::InfluencedBy,
        ]
        .iter()
        .map(|e| format!("{e:?}"))
        .collect();
        assert!(
            discrete.is_disjoint(&continuous),
            "edge types must be disjoint"
        );
    }

    #[test]
    fn content_series_creation() {
        let series = ContentSeries::new("AI Trends", "Exploring AI trends");
        assert_eq!(series.title, "AI Trends");
        assert_eq!(series.thesis, "Exploring AI trends");
        assert!(series.post_ids.is_empty());
    }

    #[test]
    fn discrete_build_series_skeleton() -> Result<(), ContentError> {
        let series = discrete::build_series_skeleton("Test", "Thesis", 3)?;
        assert_eq!(series.post_ids.len(), 3);
        Ok(())
    }

    #[test]
    fn topic_node_creation() {
        let topic = TopicNode::new("AI");
        assert_eq!(topic.label, "AI");
        assert!(topic.embedding.is_empty());
    }

    #[test]
    fn temporal_edge_timestamp() {
        let source = Uuid::new_v4();
        let target = Uuid::new_v4();
        let edge = TemporalEdge::new(source, target, ContinuousEdgeKind::RelatedTo, 1000);
        assert_eq!(edge.source, source);
        assert_eq!(edge.target, target);
        assert_eq!(edge.timestamp_ms, 1000);
    }
}
