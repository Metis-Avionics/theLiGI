//! # theligi-graph
//!
//! Core graph types (nodes, edges, adjacency lists) and traversal
//! utilities shared across all graph-aware crates in the workspace.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub type NodeId = Uuid;
pub type EdgeId = Uuid;

/// A typed node in the knowledge graph.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Node {
    pub id: NodeId,
    pub kind: NodeKind,
    pub properties: HashMap<String, serde_json::Value>,
}

impl Node {
    pub fn new(kind: impl Into<NodeKind>) -> Self {
        Self {
            id: Uuid::new_v4(),
            kind: kind.into(),
            properties: HashMap::new(),
        }
    }
}

/// Discriminator for graph node types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum NodeKind {
    Topic,
    ContentSeries,
    ContentArtifact,
    Claim,
    Evidence,
    Platform,
    AudienceSegment,
    PerformanceSignal,
    Custom(String),
}

impl From<&str> for NodeKind {
    fn from(s: &str) -> Self {
        match s {
            "topic" => Self::Topic,
            "content_series" => Self::ContentSeries,
            "content_artifact" => Self::ContentArtifact,
            "claim" => Self::Claim,
            "evidence" => Self::Evidence,
            "platform" => Self::Platform,
            "audience_segment" => Self::AudienceSegment,
            "performance_signal" => Self::PerformanceSignal,
            other => Self::Custom(other.to_string()),
        }
    }
}

/// A typed, directed edge connecting two nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: EdgeId,
    pub source: NodeId,
    pub target: NodeId,
    pub kind: EdgeKind,
    pub weight: Option<f64>,
    pub properties: HashMap<String, serde_json::Value>,
}

impl Edge {
    pub fn new(source: NodeId, target: NodeId, kind: impl Into<EdgeKind>) -> Self {
        Self {
            id: Uuid::new_v4(),
            source,
            target,
            kind: kind.into(),
            weight: None,
            properties: HashMap::new(),
        }
    }
}

/// Discriminator for graph edge types.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum EdgeKind {
    HasPost,
    NextInSeries,
    Supports,
    Causes,
    ChronologicalAdjacency,
    RelatedTo,
    InfluencedBy,
    DerivedFrom,
    PlatformAffinity,
    AudienceAffinity,
    ContentAffinity,
    PerformanceEdge,
    Custom,
}

/// Adjacency-list representation of a graph snapshot.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Graph {
    pub nodes: HashMap<NodeId, Node>,
    pub edges: Vec<Edge>,
    pub adjacency: HashMap<NodeId, Vec<(EdgeId, NodeId)>>,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, node: Node) -> NodeId {
        let id = node.id;
        self.nodes.insert(id, node);
        id
    }

    pub fn add_edge(&mut self, edge: Edge) -> EdgeId {
        let id = edge.id;
        let src = edge.source;
        let tgt = edge.target;
        self.adjacency.entry(src).or_default().push((id, tgt));
        self.edges.push(edge);
        id
    }

    pub fn neighbors(&self, node_id: &NodeId) -> Vec<&Node> {
        self.adjacency
            .get(node_id)
            .map(|edges| {
                edges
                    .iter()
                    .filter_map(|(_, target)| self.nodes.get(target))
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// Result type for graph operations.
#[derive(Debug, thiserror::Error)]
pub enum GraphError {
    #[error("node not found: {0}")]
    NodeNotFound(NodeId),

    #[error("edge not found: {0}")]
    EdgeNotFound(EdgeId),

    #[error("cycle detected during traversal")]
    CycleDetected,
}

pub type GraphResult<T> = std::result::Result<T, GraphError>;

pub mod traversal;
pub mod types;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_node_and_neighbors() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(Node::new(NodeKind::Topic));
        let n2 = graph.add_node(Node::new(NodeKind::ContentSeries));
        graph.add_edge(Edge::new(n1, n2, EdgeKind::RelatedTo));
        let neighbors = graph.neighbors(&n1);
        assert_eq!(neighbors.len(), 1);
    }

    #[test]
    fn graph_empty_neighbors() {
        let graph = Graph::new();
        let id = Uuid::new_v4();
        assert!(graph.neighbors(&id).is_empty());
    }

    #[test]
    fn node_kind_equality() {
        assert_eq!(NodeKind::Topic, NodeKind::Topic);
        assert_ne!(NodeKind::Topic, NodeKind::ContentSeries);
    }
}
