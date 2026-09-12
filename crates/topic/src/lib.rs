//! # theligi-topic
//!
//! Topic lifecycle: discover topics from content, rank them by relevance
//! and saturation, deduplicate overlapping signals, and detect gaps in
//! topical coverage across the content graph.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type TopicId = Uuid;
pub type ArtifactId = Uuid;

/// A discovered topic with associated metadata and relevance signal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub id: TopicId,
    pub label: String,
    pub embedding: Vec<f32>,
    pub relevance_score: f64,
    pub saturation: f64,
    pub discovered_from: Vec<ArtifactId>,
}

impl Topic {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            label: label.into(),
            embedding: Vec::new(),
            relevance_score: 0.0,
            saturation: 0.0,
            discovered_from: Vec::new(),
        }
    }
}

/// A content artifact from which topics are extracted.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentArtifactRef {
    pub id: ArtifactId,
    pub title: String,
    pub embedding: Vec<f32>,
}

/// Configuration for topic discovery.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    /// Minimum cosine similarity threshold to form a topic cluster.
    pub similarity_threshold: f64,
    /// Maximum number of topics to return.
    pub max_topics: usize,
    /// Minimum relevance score for a topic to be retained.
    pub min_relevance: f64,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            similarity_threshold: 0.75,
            max_topics: 100,
            min_relevance: 0.1,
        }
    }
}

/// Result of a topic discovery run.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiscoveryResult {
    pub topics: Vec<Topic>,
    pub gaps: Vec<TopicGap>,
    pub duplicates_merged: usize,
}

/// A detected coverage gap — a topic signal with no corresponding content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicGap {
    pub label: String,
    pub score: f64,
    pub related_artifact_ids: Vec<ArtifactId>,
}

/// Errors produced during topic operations.
#[derive(Debug, thiserror::Error)]
pub enum TopicError {
    #[error("topic not found: {0}")]
    NotFound(TopicId),

    #[error("saturation error: {0}")]
    Saturation(String),

    #[error("deduplication error: {0}")]
    Deduplication(String),
}

pub type TopicResult<T> = std::result::Result<T, TopicError>;

/// Trait for topic discovery and ranking.
#[async_trait::async_trait]
pub trait TopicDiscovery: Send + Sync {
    async fn discover(
        &self,
        artifacts: &[ContentArtifactRef],
        cfg: &DiscoveryConfig,
    ) -> TopicResult<DiscoveryResult>;

    async fn rank(&self, topics: &[Topic]) -> TopicResult<Vec<Topic>>;

    async fn detect_saturation(&self, topics: &mut [Topic]) -> TopicResult<()>;

    async fn detect_gaps(
        &self,
        topics: &[Topic],
        artifacts: &[ContentArtifactRef],
    ) -> TopicResult<Vec<TopicGap>>;
}

pub mod deduplicate;
pub mod discover;
pub mod gap;
pub mod rank;
