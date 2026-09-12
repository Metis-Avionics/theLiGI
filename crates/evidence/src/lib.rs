//! # theligi-evidence
//!
//! Evidence scoring and provenance tracking.  Assigns confidence scores
//! to evidence nodes, tracks their provenance chain, and aggregates
//! evidence weights for claims.

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

pub type EvidenceNodeId = Uuid;
pub type ClaimId = Uuid;
pub type ProvenanceId = Uuid;

/// A provenance record describing the origin of an evidence node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceRecord {
    pub id: ProvenanceId,
    pub source_url: Option<String>,
    pub source_type: ProvenanceSourceType,
    pub retrieved_at_ms: i64,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceSourceType {
    PrimarySource,
    SecondarySource,
    ExpertOpinion,
    AutomatedExtraction,
    UserGenerated,
    Unknown,
}

/// A scored evidence node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceNode {
    pub id: EvidenceNodeId,
    pub claim_id: ClaimId,
    pub excerpt: String,
    pub source_url: Option<String>,
    pub supports: bool,
    pub score: EvidenceScore,
    pub provenance: Vec<ProvenanceRecord>,
    pub properties: HashMap<String, serde_json::Value>,
}

impl EvidenceNode {
    pub fn new(claim_id: ClaimId, excerpt: impl Into<String>, supports: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            claim_id,
            excerpt: excerpt.into(),
            source_url: None,
            supports,
            score: EvidenceScore::default(),
            provenance: Vec::new(),
            properties: HashMap::new(),
        }
    }
}

/// Composite score for an evidence node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceScore {
    pub credibility: f64,
    pub relevance: f64,
    pub recency: f64,
    pub consensus: f64,
    pub aggregate: f64,
}

impl Default for EvidenceScore {
    fn default() -> Self {
        Self {
            credibility: 0.0,
            relevance: 0.0,
            recency: 0.0,
            consensus: 0.0,
            aggregate: 0.0,
        }
    }
}

impl EvidenceScore {
    #[must_use]
    pub fn compute(credibility: f64, relevance: f64, recency: f64, consensus: f64) -> Self {
        let aggregate =
            (credibility * 0.35) + (relevance * 0.30) + (recency * 0.20) + (consensus * 0.15);
        Self {
            credibility,
            relevance,
            recency,
            consensus,
            aggregate,
        }
    }
}

/// Aggregated evidence for a single claim.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClaimEvidenceSummary {
    pub claim_id: ClaimId,
    pub supporting_score: f64,
    pub refuting_score: f64,
    pub net_confidence: f64,
    pub evidence_count: usize,
    pub evidence_ids: Vec<EvidenceNodeId>,
}

/// Errors produced during evidence operations.
#[derive(Debug, thiserror::Error)]
pub enum EvidenceError {
    #[error("evidence node not found: {0}")]
    NotFound(EvidenceNodeId),

    #[error("claim not found: {0}")]
    ClaimNotFound(ClaimId),

    #[error("scoring error: {0}")]
    Scoring(String),

    #[error("provenance error: {0}")]
    Provenance(String),
}

pub type EvidenceResult<T> = std::result::Result<T, EvidenceError>;

/// Trait for evidence scoring and provenance tracking.
#[async_trait::async_trait]
pub trait EvidenceScorer: Send + Sync {
    async fn score_evidence(&self, node: &mut EvidenceNode) -> EvidenceResult<EvidenceScore>;

    async fn aggregate_for_claim(&self, claim_id: ClaimId) -> EvidenceResult<ClaimEvidenceSummary>;

    async fn add_provenance(
        &self,
        node: &mut EvidenceNode,
        record: ProvenanceRecord,
    ) -> EvidenceResult<()>;

    async fn verify_lineage(
        &self,
        node_id: EvidenceNodeId,
    ) -> EvidenceResult<Vec<ProvenanceRecord>>;
}

pub mod provenance;
pub mod scorer;
pub mod sufficiency;
