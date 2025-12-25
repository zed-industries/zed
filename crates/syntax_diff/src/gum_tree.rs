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
pub struct NodeId(usize);

impl NodeId {
    pub fn index(self) -> usize {
        self.0
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
    /// The tree-sitter kind ID for fast comparison
    pub kind_id: u16,
    /// Height of this node (0 for leaves, max child height + 1 for internal nodes)
    pub height: u16,
    /// Structural hash: hash of kind + children's structural hashes
    pub structural_hash: u64,
    /// Content hash: hash of the actual text content
    pub content_hash: u64,
    /// Parent node ID, if any
    pub parent: Option<NodeId>,
    /// Number of descendants (including self)
    pub descendant_count: u32,
}

impl<'a> DiffTree<'a> {
    pub fn new(tree: &'a tree_sitter::Tree, text: &'a str) -> Self {
        let mut nodes = Vec::new();

        build_nodes(&mut tree.walk(), text, &mut nodes);

        Self { nodes, text }
    }
}

fn build_nodes(
    cursor: &mut tree_sitter::TreeCursor<'_>,
    text: &str,
    nodes: &mut Vec<DiffNode>,
) -> NodeId {
    let id = NodeId(nodes.len());
    let node = cursor.node();

    let mut children = Vec::new();
    if cursor.goto_first_child() {
        let mut has_next = true;
        while has_next {
            children.push(build_nodes(cursor, text, nodes));
            has_next = cursor.goto_next_sibling();
        }

        cursor.goto_parent();
    }

    // Compute height (max child height + 1, or 0 for leaves)
    let height = if children.is_empty() {
        0
    } else {
        children
            .iter()
            .map(|id| nodes[id.index()].height)
            .max()
            .unwrap_or(0)
            + 1
    };

    let descendant_count: u32 = 1 + children
        .iter()
        .map(|id| nodes[id.index()].descendant_count)
        .sum::<u32>();

    let structural_hash = compute_structural_hash(node.kind_id(), &children, nodes);
    let content_hash = compute_content_hash(&text[node.byte_range()]);

    let node = DiffNode {
        id,
        kind_id: node.kind_id(),
        structural_hash,
        content_hash,
        descendant_count,
        height,
        parent: None,
    };

    nodes.push(node);

    id
}

fn compute_structural_hash(kind_id: u16, children: &[NodeId], nodes: &[DiffNode]) -> u64 {
    let mut hasher = DefaultHasher::new();
    kind_id.hash(&mut hasher);

    for child_id in children {
        nodes[child_id.index()].structural_hash.hash(&mut hasher);
    }

    hasher.finish()
}

fn compute_content_hash(content: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    hasher.finish()
}
