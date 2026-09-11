//! # theligi-inference
//!
//! Graph neural network (GNN) inference layer.  Provides typed inference
//! functions for embedding, link prediction, topic ranking, affinity
//! scoring and performance prediction without exposing model internals.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type NodeId = Uuid;
pub type EdgeId = Uuid;

/// Dense embedding vector produced by the GNN encoder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeEmbedding {
    pub node_id: NodeId,
    pub vector: Vec<f32>,
    pub dimension: usize,
}

impl NodeEmbedding {
    pub fn new(node_id: NodeId, vector: Vec<f32>) -> Self {
        let dimension = vector.len();
        Self {
            node_id,
            vector,
            dimension,
        }
    }
}

/// Predicted link between two nodes with a confidence score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkPrediction {
    pub source: NodeId,
    pub target: NodeId,
    pub edge_kind: String,
    pub confidence: f64,
}

/// Ranked topic output from GNN inference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicRanking {
    pub topic_id: uuid::Uuid,
    pub rank: usize,
    pub score: f64,
}

/// Affinity scores from GNN inference for a given node.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AffinityScores {
    pub node_id: NodeId,
    pub audience: HashMap<String, f64>,
    pub platform: HashMap<String, f64>,
    pub content: HashMap<String, f64>,
}

use std::collections::HashMap;

/// Predicted performance signal for a content artifact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformancePrediction {
    pub artifact_id: Uuid,
    pub predicted_engagement: f64,
    pub predicted_reach: f64,
    pub confidence: f64,
}

/// Input bundle for a single inference call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceInput {
    pub node_embeddings: HashMap<NodeId, Vec<f32>>,
    pub edge_index: Vec<(NodeId, NodeId)>,
    pub edge_attr: Vec<Vec<f32>>,
    pub node_features: HashMap<NodeId, Vec<f32>>,
}

/// Errors produced during inference.
#[derive(Debug, thiserror::Error)]
pub enum InferenceError {
    #[error("model not loaded")]
    ModelNotLoaded,

    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("inference runtime error: {0}")]
    Runtime(String),
}

pub type InferenceResult<T> = std::result::Result<T, InferenceError>;

/// Trait covering all inference operations.
#[async_trait::async_trait]
pub trait InferenceEngine: Send + Sync {
    async fn node_embedding(&self, input: &InferenceInput) -> InferenceResult<Vec<NodeEmbedding>>;

    async fn link_prediction(&self, input: &InferenceInput)
        -> InferenceResult<Vec<LinkPrediction>>;

    async fn topic_ranking(&self, input: &InferenceInput) -> InferenceResult<Vec<TopicRanking>>;

    async fn audience_affinity(
        &self,
        input: &InferenceInput,
    ) -> InferenceResult<Vec<AffinityScores>>;

    async fn platform_affinity(
        &self,
        input: &InferenceInput,
    ) -> InferenceResult<Vec<AffinityScores>>;

    async fn content_affinity(
        &self,
        input: &InferenceInput,
    ) -> InferenceResult<Vec<AffinityScores>>;

    async fn performance_prediction(
        &self,
        input: &InferenceInput,
    ) -> InferenceResult<Vec<PerformancePrediction>>;
}

pub mod engine;
pub mod model;
