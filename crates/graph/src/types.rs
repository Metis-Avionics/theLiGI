//! Additional graph type definitions for theligi-graph.
use crate::{Edge, EdgeKind, Graph, NodeId};

/// Compute the in-degree of a node.
pub fn in_degree(graph: &Graph, node_id: &NodeId) -> usize {
    graph.edges.iter().filter(|e| e.target == *node_id).count()
}

/// Compute the out-degree of a node.
pub fn out_degree(graph: &Graph, node_id: &NodeId) -> usize {
    graph.adjacency.get(node_id).map(|v| v.len()).unwrap_or(0)
}

/// Filter edges by kind.
pub fn edges_by_kind(graph: &Graph, kind: EdgeKind) -> Vec<&Edge> {
    graph.edges.iter().filter(|e| e.kind == kind).collect()
}
