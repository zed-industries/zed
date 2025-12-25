// GumTree Algorithm Implementation
//
// Original paper: "Fine-grained and Accurate Source Code Differencing" (2014)
// https://hal.science/hal-01054552/document
//
// Improved version: "GumTree 4.0" (2024)
// https://hal.science/hal-04855170v1/document
//
// The algorithm works in two phases:
// 1. Top-down: Match isomorphic subtrees that are sufficiently deep and unique
// 2. Bottom-up: Match remaining nodes based on ancestor similarity

use std::hash::{DefaultHasher, Hash, Hasher};

/// Unique identifier for a node within a DiffTree.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(u32);

impl NodeId {
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

/// A preprocessed syntax tree optimized for diffing.
///
/// Stores nodes in a flat vector with precomputed hashes for fast isomorphism checks.
pub struct DiffTree<'a> {
    nodes: Vec<DiffNode>,
    text: &'a str,
}

/// A node within a DiffTree with precomputed metadata.
pub struct DiffNode {
    /// Index of this node in the DiffTree's nodes vector
    pub id: NodeId,
    /// The tree-sitter node kind (e.g., "function_definition", "identifier")
    pub kind: &'static str,
    /// The tree-sitter kind ID for fast comparison
    pub kind_id: u16,
    /// Height of this node (0 for leaves, max child height + 1 for internal nodes)
    pub height: u16,
    /// Structural hash: hash of kind + children's structural hashes
    pub structural_hash: u64,
    /// Content hash: hash of the actual text content
    pub content_hash: u64,
    /// Parent node ID, if any
    pub parnet: Option<NodeId>,
    /// Number of descendants (including self)
    pub descendant_count: u32,
}

fn compute_structural_hash(kind_id: u16, child_ids: &[NodeId], nodes: &[DiffNode]) -> u64 {
    let mut hasher = DefaultHasher::new();
    kind_id.hash(&mut hasher);

    for child_id in child_ids {
        nodes[child_id.index()].structural_hash.hash(&mut hasher);
    }

    hasher.finish()
}

fn compute_content_hash(content: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    hasher.finish()
}
