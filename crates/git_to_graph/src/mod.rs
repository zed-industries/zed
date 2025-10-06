//! # git_to_graph
//!
//! A Rust port of the git2graph SourceTree algorithm for rendering git commit graphs.
//! This library provides functionality to convert git commit data into drawable graph structures.
//!
//! ## Attribution
//!
//! Ported from [git2graph](https://github.com/alaingilbert/git2graph)
//! Original Copyright (c) 2023 Alain Gilbert
//! Originally licensed under MIT License
//!
//! This port is licensed under GPL-3.0-or-later as part of the Zed project.

use crate::point::Point;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

// Module declarations
pub mod algorithm;
pub mod build;
pub mod color;
pub mod column;
pub mod finalize;
pub mod node;
pub mod output;
pub mod path;
pub mod point;
pub mod process_children;
pub mod process_parents;
pub mod types;

// Re-exports for public API
pub use self::build::build_tree;
pub use self::color::SimpleColorGen;
pub use self::node::Node;
pub use self::types::PointType;

/// Input data for a single commit
#[derive(Debug, Clone)]
pub struct CommitInput {
    pub oid: String,
    pub parents: Vec<String>,
}

/// Result of building a graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphResult {
    pub nodes: IndexMap<String, NodeInfo>,
    pub partial_paths: Vec<PartialPathInfo>,
}

/// Information about a partial path (paths that extend beyond visible range)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialPathInfo {
    pub points: Vec<(i32, i32, u8)>,
    pub color_idx: usize,
}

/// Information about a node in the graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub column: i32,
    pub color_idx: usize,
    pub parents_paths: IndexMap<String, PathInfo>,
}

/// Information about a path between nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathInfo {
    pub points: Vec<PointInfo>,
    pub color_idx: usize,
}

/// A point in a path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointInfo {
    pub x: i32,
    pub y: i32,
    pub point_type: PointType,
}

/// Build a git graph from commit inputs
///
/// This API wraps the underlying algorithm and provides:
/// - Node positioning (columns) for each commit
/// - Path information for drawing connections between commits
/// - Partial paths for commits that extend beyond the visible range (useful for culling)
/// - Color indices for branch coloring
///
/// Note: We don't currently expose pagination (`from`/`limit`) parameters since
/// the caller already controls which commits are passed in. Pagination would be
/// useful for virtualized scrolling where the algorithm needs to compute layout
/// for commits beyond the visible range.
pub fn build_graph(commits: Vec<CommitInput>) -> Result<GraphResult, String> {
    use serde_json::Value;

    // Convert CommitInput to Node format
    let input_nodes: Vec<Node> = commits
        .into_iter()
        .map(|commit| {
            let mut node = Node::new();
            node.insert("id".to_string(), Value::String(commit.oid));
            node.insert(
                "parents".to_string(),
                Value::Array(commit.parents.into_iter().map(Value::String).collect()),
            );
            node
        })
        .collect();

    // Use the algorithm directly (no pagination - caller controls commit list)
    let (nodes, partial_paths) = algorithm::set_columns(&input_nodes, "", -1);

    // Convert to GraphResult format
    let mut result_nodes = IndexMap::new();

    for internal_node in nodes {
        let borrowed = internal_node.borrow();
        let oid = borrowed.id.clone();

        let mut parents_paths = IndexMap::new();
        for (parent_id, path) in &borrowed.parents_paths {
            let points: Vec<PointInfo> = path
                .points
                .iter()
                .map(|point| PointInfo {
                    x: point.get_x(),
                    y: point.get_y(),
                    point_type: point.get_type(),
                })
                .collect();

            parents_paths.insert(
                parent_id.clone(),
                PathInfo {
                    points,
                    color_idx: path.color_idx as usize,
                },
            );
        }

        result_nodes.insert(
            oid,
            NodeInfo {
                column: borrowed.column,
                color_idx: borrowed.color_idx as usize,
                parents_paths,
            },
        );
    }

    // Convert partial paths for culling
    let result_partial_paths: Vec<PartialPathInfo> = partial_paths
        .iter()
        .map(|path| PartialPathInfo {
            points: path
                .points
                .iter()
                .map(|point| (point.get_x(), point.get_y(), point.get_type() as u8))
                .collect(),
            color_idx: path.color_idx as usize,
        })
        .collect();

    Ok(GraphResult {
        nodes: result_nodes,
        partial_paths: result_partial_paths,
    })
}
