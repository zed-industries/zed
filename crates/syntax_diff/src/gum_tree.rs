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

use collections::FxHashMap;
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    ops::Range,
};

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
    pub parent: Option<NodeId>,
    /// Number of descendants (including self)
    pub descendant_count: u32,
}

/// The result of matching two trees.
///
/// Contains bidirectional mappings between nodes in the old and new trees.
#[derive(Default)]
pub struct Matching {
    /// Maps old tree node ID -> new tree node ID
    old_to_new: FxHashMap<NodeId, NodeId>,
    /// Maps new tree node ID -> old tree node ID
    new_to_old: FxHashMap<NodeId, NodeId>,
}

impl Matching {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, old_id: NodeId, new_id: NodeId) {
        self.old_to_new.insert(old_id, new_id);
        self.new_to_old.insert(new_id, old_id);
    }

    pub fn is_old_matched(&self, id: NodeId) -> bool {
        self.old_to_new.contains_key(&id)
    }

    pub fn is_new_matched(&self, id: NodeId) -> bool {
        self.new_to_old.contains_key(&id)
    }

    pub fn get_new(&self, old_id: NodeId) -> Option<NodeId> {
        self.old_to_new.get(&old_id).copied()
    }

    pub fn get_old(&self, new_id: NodeId) -> Option<NodeId> {
        self.new_to_old.get(&new_id).copied()
    }

    pub fn matched_pairs(&self) -> impl Iterator<Item = (NodeId, NodeId)> + '_ {
        self.old_to_new.iter().map(|(&old, &new)| (old, new))
    }
}

/// A single diff operation representing a change between trees.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffOperation {
    /// Node was deleted from the old tree
    Delete {
        range: Range<usize>,
        node_kind: &'static str,
    },
    /// Node was inserted into the new tree
    Insert {
        range: Range<usize>,
        node_kind: &'static str,
    },
    /// Node was moved to a different parent
    Move {
        old_range: Range<usize>,
        new_range: Range<usize>,
        node_kind: &'static str,
    },
    /// Node content was updated (same structure, different text)
    Update {
        old_range: Range<usize>,
        new_range: Range<usize>,
        node_kind: &'static str,
    },
}

/// The result of diffing two syntax trees.
#[derive(Debug, Default)]
pub struct DiffResult {
    pub operations: Vec<DiffOperation>,
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
        kind: node.kind(),
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
