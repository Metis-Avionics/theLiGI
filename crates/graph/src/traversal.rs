//! Graph traversal algorithms for theligi-graph.
use crate::{Graph, GraphResult, NodeId};

/// Depth-first traversal, returning node IDs in visitation order.
pub fn dfs(graph: &Graph, start: &NodeId) -> GraphResult<Vec<NodeId>> {
    let mut visited = std::collections::HashSet::new();
    let mut result = Vec::new();
    let mut stack = vec![*start];

    while let Some(node_id) = stack.pop() {
        if visited.insert(node_id) {
            result.push(node_id);
            for neighbor in graph.neighbors(&node_id) {
                stack.push(neighbor.id);
            }
        }
    }

    Ok(result)
}

/// Breadth-first traversal.
pub fn bfs(graph: &Graph, start: &NodeId) -> GraphResult<Vec<NodeId>> {
    use std::collections::VecDeque;
    let mut visited = std::collections::HashSet::new();
    let mut result = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(*start);
    visited.insert(*start);

    while let Some(node_id) = queue.pop_front() {
        result.push(node_id);
        for neighbor in graph.neighbors(&node_id) {
            if visited.insert(neighbor.id) {
                queue.push_back(neighbor.id);
            }
        }
    }

    Ok(result)
}

/// Shortest path (by hop count) between two nodes.
pub fn shortest_path(
    graph: &Graph,
    from: &NodeId,
    to: &NodeId,
) -> GraphResult<Option<Vec<NodeId>>> {
    use std::collections::VecDeque;
    let mut visited = std::collections::HashSet::new();
    let mut queue = VecDeque::new();
    let mut parent: std::collections::HashMap<NodeId, NodeId> = std::collections::HashMap::new();

    queue.push_back(*from);
    visited.insert(*from);

    while let Some(current) = queue.pop_front() {
        if current == *to {
            let mut path = vec![*to];
            let mut node = *to;
            while let Some(&p) = parent.get(&node) {
                path.push(p);
                node = p;
            }
            path.reverse();
            return Ok(Some(path));
        }
        for neighbor in graph.neighbors(&current) {
            if visited.insert(neighbor.id) {
                parent.insert(neighbor.id, current);
                queue.push_back(neighbor.id);
            }
        }
    }

    Ok(None)
}
