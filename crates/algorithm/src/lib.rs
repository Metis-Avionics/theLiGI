use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

/// Algorithm execution errors.
#[derive(Debug, Error)]
pub enum AlgorithmError {
    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("execution error: {0}")]
    Execution(String),

    #[error("not implemented")]
    NotImplemented,
}

/// Typed algorithm trait with named kind.
#[async_trait::async_trait]
pub trait Algorithm: Send + Sync + 'static {
    type Input: Send + Sync + 'static;
    type Output: Send + Sync + 'static;

    fn kind(&self) -> &'static str;
    async fn execute(&self, input: Self::Input) -> Result<Self::Output, AlgorithmError>;
}

/// Topic ranker input and output.
#[derive(Debug, Clone)]
pub struct TopicRankerInput {
    pub topics: Vec<String>,
    pub scores: HashMap<String, f64>,
}

pub type TopicRankerOutput = Vec<(String, f64)>;

#[derive(Debug, Default)]
pub struct TopicRanker;

#[async_trait::async_trait]
impl Algorithm for TopicRanker {
    type Input = TopicRankerInput;
    type Output = TopicRankerOutput;

    fn kind(&self) -> &'static str {
        "topic_ranker"
    }

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, AlgorithmError> {
        let mut ranked = input.scores.into_iter().collect::<Vec<_>>();
        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        Ok(ranked)
    }
}

/// Topic generator input and output.
#[derive(Debug, Clone)]
pub struct TopicGeneratorInput {
    pub seed: String,
    pub max_topics: usize,
}

pub type TopicGeneratorOutput = Vec<String>;

#[derive(Debug, Default)]
pub struct TopicGenerator;

#[async_trait::async_trait]
impl Algorithm for TopicGenerator {
    type Input = TopicGeneratorInput;
    type Output = TopicGeneratorOutput;

    fn kind(&self) -> &'static str {
        "topic_generator"
    }

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, AlgorithmError> {
        let mut topics = Vec::new();
        for i in 0..input.max_topics {
            topics.push(format!("{}-topic-{}", input.seed, i));
        }
        Ok(topics)
    }
}

/// Similarity detector input and output.
#[derive(Debug, Clone)]
pub struct SimilarityDetectorInput {
    pub source: Vec<f64>,
    pub target: Vec<f64>,
}

pub type SimilarityDetectorOutput = f64;

#[derive(Debug, Default)]
pub struct SimilarityDetector;

#[async_trait::async_trait]
impl Algorithm for SimilarityDetector {
    type Input = SimilarityDetectorInput;
    type Output = SimilarityDetectorOutput;

    fn kind(&self) -> &'static str {
        "similarity_detector"
    }

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, AlgorithmError> {
        if input.source.len() != input.target.len() {
            return Err(AlgorithmError::InvalidInput(
                "vector lengths must match".to_string(),
            ));
        }
        let dot: f64 = input
            .source
            .iter()
            .zip(&input.target)
            .map(|(a, b)| a * b)
            .sum();
        Ok(dot)
    }
}

/// Saturation detector input and output.
#[derive(Debug, Clone)]
pub struct SaturationDetectorInput {
    pub values: Vec<f64>,
    pub threshold: f64,
}

pub type SaturationDetectorOutput = bool;

#[derive(Debug, Default)]
pub struct SaturationDetector;

#[async_trait::async_trait]
impl Algorithm for SaturationDetector {
    type Input = SaturationDetectorInput;
    type Output = SaturationDetectorOutput;

    fn kind(&self) -> &'static str {
        "saturation_detector"
    }

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, AlgorithmError> {
        Ok(input.values.iter().any(|v| *v >= input.threshold))
    }
}

/// Audience affinity input and output.
#[derive(Debug, Clone)]
pub struct AudienceAffinityInput {
    pub audience_id: Uuid,
    pub features: Vec<f64>,
}

pub type AudienceAffinityOutput = f64;

#[derive(Debug, Default)]
pub struct AudienceAffinity;

#[async_trait::async_trait]
impl Algorithm for AudienceAffinity {
    type Input = AudienceAffinityInput;
    type Output = AudienceAffinityOutput;

    fn kind(&self) -> &'static str {
        "audience_affinity"
    }

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, AlgorithmError> {
        Ok(input.features.iter().sum())
    }
}

/// Platform affinity input and output.
#[derive(Debug, Clone)]
pub struct PlatformAffinityInput {
    pub platform: String,
    pub features: Vec<f64>,
}

pub type PlatformAffinityOutput = f64;

#[derive(Debug, Default)]
pub struct PlatformAffinity;

#[async_trait::async_trait]
impl Algorithm for PlatformAffinity {
    type Input = PlatformAffinityInput;
    type Output = PlatformAffinityOutput;

    fn kind(&self) -> &'static str {
        "platform_affinity"
    }

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, AlgorithmError> {
        let _ = input.platform;
        Ok(input.features.iter().sum())
    }
}

/// Evidence scorer input and output.
#[derive(Debug, Clone)]
pub struct EvidenceScorerInput {
    pub evidence_ids: Vec<Uuid>,
    pub weights: HashMap<Uuid, f64>,
}

pub type EvidenceScorerOutput = f64;

#[derive(Debug, Default)]
pub struct EvidenceScorer;

#[async_trait::async_trait]
impl Algorithm for EvidenceScorer {
    type Input = EvidenceScorerInput;
    type Output = EvidenceScorerOutput;

    fn kind(&self) -> &'static str {
        "evidence_scorer"
    }

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, AlgorithmError> {
        let score: f64 = input
            .evidence_ids
            .iter()
            .filter_map(|id| input.weights.get(id))
            .sum();
        Ok(score)
    }
}

/// GNN inference input and output.
#[derive(Debug, Clone)]
pub struct GnnInferenceInput {
    pub node_features: Vec<Vec<f64>>,
    pub edge_indices: Vec<(usize, usize)>,
}

pub type GnnInferenceOutput = Vec<f64>;

#[derive(Debug, Default)]
pub struct GnnInference;

#[async_trait::async_trait]
impl Algorithm for GnnInference {
    type Input = GnnInferenceInput;
    type Output = GnnInferenceOutput;

    fn kind(&self) -> &'static str {
        "gnn_inference"
    }

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, AlgorithmError> {
        let output = vec![0.0; input.node_features.len()];
        Ok(output)
    }
}
