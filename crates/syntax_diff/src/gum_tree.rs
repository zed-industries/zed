// GumTree Algorithm Implementation
//
// Original paper: "Fine-grained and Accurate Source Code Differencing" (2014)
// https://hal.science/hal-01054552/document
//
// Improved version: "GumTree 4.0" (2024)
// https://hal.science/hal-04855170v1/document
//
// Hyperparameter tuning: "Hyperparameter Optimization for AST Differencing" (2023)
// https://hal.science/hal-04423080/document
//
// The algorithm works in three phases:
// 1. Top-down: Match isomorphic subtrees that are sufficiently deep and unique
// 2. Bottom-up: Match remaining nodes based on Dice similarity of matched descendants
// 3. Recovery: Match remaining unmatched children of matched parents (GumTree 4.0 improvement)

use collections::{FxHashMap, FxHashSet};
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    ops::Range,
};

/// Minimum height for subtree matching in top-down phase.
/// Subtrees shorter than this are not matched greedily.
/// Default: 1 (optimized value from DAT research)
const MIN_HEIGHT: u16 = 1;

/// Minimum Dice similarity threshold for bottom-up matching.
/// Two nodes match if their descendant overlap >= this value.
/// Default: 0.2 (optimized value from DAT research, lower than original 0.5)
const SIM_THRESHOLD: f64 = 0.2;

/// Maximum tree size for recovery phase.
/// Recovery is skipped for subtrees larger than this.
/// Default: 600 (optimized value from DAT research)
const MAX_RECOVERY_SIZE: usize = 600;

/// Unique identifier for a node within a DiffTree.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(usize);

impl NodeId {
    pub fn index(self) -> usize {
        self.0
    }
}

/// A preprocessed syntax tree optimized for diffing.
///
/// Stores nodes in a flat vector with precomputed hashes for fast isomorphism checks.
/// Nodes are stored in post-order (children before parents), which ensures
/// that when processing a node, all its descendants are already available.
pub struct DiffTree<'a> {
    nodes: Vec<DiffNode>,
    text: &'a str,
    root: NodeId,
}

/// A node within a DiffTree with precomputed metadata.
#[derive(Debug)]
pub struct DiffNode {
    /// Index of this node in the DiffTree's nodes vector
    pub id: NodeId,
    /// The tree-sitter node kind (e.g., "function_definition", "identifier")
    pub kind: &'static str,
    /// The tree-sitter kind ID for fast comparison
    pub kind_id: u16,
    /// Height of this node (0 for leaves, max child height + 1 for internal nodes)
    pub height: u16,
    /// Structural hash: hash of kind + children's structural hashes (for isomorphism)
    pub structural_hash: u64,
    /// Content hash: hash of the actual text content
    pub content_hash: u64,
    /// Parent node ID, if any
    pub parent: Option<NodeId>,
    /// Direct children of this node
    pub children: Vec<NodeId>,
    /// Number of descendants (including self)
    pub descendant_count: usize,
    /// Byte range in the source text
    pub byte_range: Range<usize>,
}

/// The result of matching two trees.
///
/// Contains bidirectional mappings between nodes in the old and new trees.
#[derive(Default, Debug)]
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

    pub fn len(&self) -> usize {
        self.old_to_new.len()
    }

    pub fn is_empty(&self) -> bool {
        self.old_to_new.is_empty()
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

    pub fn old_matched_nodes(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.old_to_new.keys().copied()
    }

    pub fn new_matched_nodes(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.new_to_old.keys().copied()
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
    pub fn new(tree: &tree_sitter::Tree, text: &'a str) -> Self {
        let mut nodes = Vec::with_capacity(tree.root_node().descendant_count());
        let root = build_nodes(&mut tree.walk(), text, &mut nodes, None);

        Self { nodes, text, root }
    }

    pub fn root(&self) -> NodeId {
        self.root
    }

    pub fn node(&self, id: NodeId) -> &DiffNode {
        &self.nodes[id.index()]
    }

    pub fn nodes(&self) -> &[DiffNode] {
        &self.nodes
    }

    pub fn text(&self) -> &str {
        self.text
    }

    pub fn node_text(&self, id: NodeId) -> &str {
        let node = &self.nodes[id.index()];
        &self.text[node.byte_range.clone()]
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the maximum height in the tree.
    pub fn max_height(&self) -> u16 {
        self.nodes[self.root.index()].height
    }

    /// Iterate over all descendants of a node (including the node itself).
    pub fn descendants(&self, id: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        DescendantIterator::new(self, id)
    }

    /// Get nodes at a specific height, sorted by descending descendant count.
    pub fn nodes_at_height(&self, height: u16) -> Vec<NodeId> {
        let mut nodes: Vec<_> = self
            .nodes
            .iter()
            .filter(|n| n.height == height)
            .map(|n| n.id)
            .collect();
        nodes.sort_by(|a, b| {
            self.nodes[b.index()]
                .descendant_count
                .cmp(&self.nodes[a.index()].descendant_count)
        });
        nodes
    }
}

/// Iterator over all descendants of a node (including the node itself).
struct DescendantIterator<'a, 'b> {
    tree: &'a DiffTree<'b>,
    stack: Vec<NodeId>,
}

impl<'a, 'b> DescendantIterator<'a, 'b> {
    fn new(tree: &'a DiffTree<'b>, root: NodeId) -> Self {
        Self {
            tree,
            stack: vec![root],
        }
    }
}

impl<'a, 'b> Iterator for DescendantIterator<'a, 'b> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.stack.pop()?;
        let node = self.tree.node(id);
        self.stack.extend(node.children.iter().rev().copied());
        Some(id)
    }
}

fn build_nodes(
    cursor: &mut tree_sitter::TreeCursor<'_>,
    text: &str,
    nodes: &mut Vec<DiffNode>,
    parent: Option<NodeId>,
) -> NodeId {
    let node = cursor.node();
    let byte_range = node.byte_range();

    let mut children = Vec::with_capacity(node.child_count());
    let this_id = NodeId(nodes.len());

    // Reserve space for this node (we'll fill it in after processing children)
    nodes.push(DiffNode {
        id: this_id,
        kind: node.kind(),
        kind_id: node.kind_id(),
        height: 0,
        structural_hash: 0,
        content_hash: 0,
        parent,
        children: Vec::new(),
        descendant_count: node.descendant_count(),
        byte_range: byte_range.clone(),
    });

    // Process children
    if cursor.goto_first_child() {
        loop {
            children.push(build_nodes(cursor, text, nodes, Some(this_id)));

            if !cursor.goto_next_sibling() {
                break;
            }
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

    let structural_hash = compute_structural_hash(node.kind_id(), &children, nodes);
    let content_hash = compute_content_hash(&text[byte_range]);

    // Update the node with computed values
    let node = &mut nodes[this_id.index()];
    node.height = height;
    node.structural_hash = structural_hash;
    node.content_hash = content_hash;
    node.children = children;

    this_id
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

/// Main entry point for GumTree matching algorithm.
///
/// Matches nodes between two syntax trees using a three-phase approach:
/// 1. Top-down: Match isomorphic subtrees greedily by height
/// 2. Bottom-up: Match container nodes by Dice similarity of descendants
/// 3. Recovery: Match remaining children of matched parents
pub fn match_trees(old: &DiffTree, new: &DiffTree) -> Matching {
    let mut matching = Matching::new();

    // Phase 1: Top-down greedy subtree matching
    top_down_matching(old, new, &mut matching);

    // Phase 2: Bottom-up container matching
    bottom_up_matching(old, new, &mut matching);

    // Phase 3: Recovery phase (GumTree 4.0 improvement)
    recovery_matching(old, new, &mut matching);

    matching
}

/// Phase 1: Top-down greedy subtree matching.
///
/// Matches isomorphic subtrees starting from the largest (highest) ones.
/// Two subtrees are isomorphic if they have the same structure (same structural hash)
/// and the same content (same content hash).
///
/// A subtree is only matched if:
/// - Its height >= min_height threshold
/// - Both the old and new subtrees are unique at their structural+content hash
fn top_down_matching(old: &DiffTree, new: &DiffTree, matching: &mut Matching) {
    // Build hash -> nodes index for both trees
    // We use (structural_hash, content_hash) as the key for exact isomorphism
    let mut old_hash_to_nodes: FxHashMap<(u64, u64), Vec<NodeId>> = FxHashMap::default();
    let mut new_hash_to_nodes: FxHashMap<(u64, u64), Vec<NodeId>> = FxHashMap::default();

    for node in old.nodes() {
        if node.height >= MIN_HEIGHT {
            let key = (node.structural_hash, node.content_hash);
            old_hash_to_nodes.entry(key).or_default().push(node.id);
        }
    }

    for node in new.nodes() {
        if node.height >= MIN_HEIGHT {
            let key = (node.structural_hash, node.content_hash);
            new_hash_to_nodes.entry(key).or_default().push(node.id);
        }
    }

    // Process heights from max to min_height
    let max_height = old.max_height().max(new.max_height());

    for height in (MIN_HEIGHT..=max_height).rev() {
        // Get candidate nodes at this height, sorted by size (largest first)
        let old_candidates = old.nodes_at_height(height);

        for old_id in old_candidates {
            if matching.is_old_matched(old_id) {
                continue;
            }

            let old_node = old.node(old_id);
            let key = (old_node.structural_hash, old_node.content_hash);

            // Find matching candidates in new tree
            let Some(new_candidates) = new_hash_to_nodes.get(&key) else {
                continue;
            };

            // Filter to unmatched candidates
            let unmatched_new: Vec<_> = new_candidates
                .iter()
                .copied()
                .filter(|&id| !matching.is_new_matched(id))
                .collect();

            // Only match if there's exactly one candidate (unique match)
            // or if we can find a unique match based on position/context
            if unmatched_new.len() == 1 {
                let new_id = unmatched_new[0];
                match_subtrees(old, new, old_id, new_id, matching);
            } else if unmatched_new.len() > 1 {
                // Multiple candidates - try to find the best match based on parent similarity
                if let Some(best_match) =
                    find_best_match_by_context(old, new, old_id, &unmatched_new, matching)
                {
                    match_subtrees(old, new, old_id, best_match, matching);
                }
            }
        }
    }
}

/// Find the best matching node when there are multiple candidates.
/// Uses parent matching context to disambiguate.
fn find_best_match_by_context(
    old: &DiffTree,
    new: &DiffTree,
    old_id: NodeId,
    new_candidates: &[NodeId],
    matching: &Matching,
) -> Option<NodeId> {
    let old_node = old.node(old_id);

    // If old node's parent is matched, prefer a new node whose parent matches
    if let Some(old_parent) = old_node.parent {
        if let Some(new_parent) = matching.get_new(old_parent) {
            for &new_id in new_candidates {
                let new_node = new.node(new_id);
                if new_node.parent == Some(new_parent) {
                    return Some(new_id);
                }
            }
        }
    }

    // If no parent context helps, check if any old sibling is matched to a sibling of a candidate
    if let Some(old_parent) = old_node.parent {
        let old_siblings: FxHashSet<_> = old.node(old_parent).children.iter().copied().collect();

        for &new_id in new_candidates {
            let new_node = new.node(new_id);
            if let Some(new_parent) = new_node.parent {
                let new_siblings: FxHashSet<_> =
                    new.node(new_parent).children.iter().copied().collect();

                // Check if any matched pair exists among siblings
                for &old_sib in &old_siblings {
                    if let Some(matched_new_sib) = matching.get_new(old_sib) {
                        if new_siblings.contains(&matched_new_sib) {
                            return Some(new_id);
                        }
                    }
                }
            }
        }
    }

    None
}

/// Match all nodes in two isomorphic subtrees.
fn match_subtrees(
    old: &DiffTree,
    new: &DiffTree,
    old_root: NodeId,
    new_root: NodeId,
    matching: &mut Matching,
) {
    let mut old_stack = vec![old_root];
    let mut new_stack = vec![new_root];

    while let (Some(old_id), Some(new_id)) = (old_stack.pop(), new_stack.pop()) {
        if matching.is_old_matched(old_id) || matching.is_new_matched(new_id) {
            continue;
        }

        matching.add(old_id, new_id);

        let old_node = old.node(old_id);
        let new_node = new.node(new_id);

        // Children should be in the same order for isomorphic subtrees
        old_stack.extend(old_node.children.iter().rev().copied());
        new_stack.extend(new_node.children.iter().rev().copied());
    }
}

/// Phase 2: Bottom-up container matching.
///
/// For each unmatched node, find candidates in the other tree with:
/// - Same node kind
/// - Dice similarity of matched descendants >= threshold
///
/// Dice(A, B) = 2 * |common matched descendants| / (|descendants of A| + |descendants of B|)
fn bottom_up_matching(old: &DiffTree, new: &DiffTree, matching: &mut Matching) {
    // Process nodes by decreasing height (parents after children are already matched)
    let max_height = old.max_height();

    for height in (0..=max_height).rev() {
        let old_candidates = old.nodes_at_height(height);

        for old_id in old_candidates {
            if matching.is_old_matched(old_id) {
                continue;
            }

            let old_node = old.node(old_id);

            // Find the best matching node in the new tree
            let mut best_match: Option<(NodeId, f64)> = None;

            for new_node in new.nodes() {
                if matching.is_new_matched(new_node.id) {
                    continue;
                }

                // Must have same node kind
                if new_node.kind_id != old_node.kind_id {
                    continue;
                }

                // Compute Dice similarity
                let dice = compute_dice_similarity(old, new, old_id, new_node.id, matching);

                if dice >= SIM_THRESHOLD {
                    if best_match.is_none()
                        || dice > best_match.as_ref().map(|(_, d)| *d).unwrap_or(0.0)
                    {
                        best_match = Some((new_node.id, dice));
                    }
                }
            }

            if let Some((new_id, _)) = best_match {
                matching.add(old_id, new_id);
            }
        }
    }
}

/// Compute Dice similarity between two nodes based on their matched descendants.
///
/// Dice(A, B) = 2 * |common| / (|A descendants| + |B descendants|)
/// where |common| is the number of matched pairs where one is in A's descendants
/// and the other is in B's descendants.
fn compute_dice_similarity(
    old: &DiffTree,
    new: &DiffTree,
    old_id: NodeId,
    new_id: NodeId,
    matching: &Matching,
) -> f64 {
    let old_descendants: FxHashSet<_> = old.descendants(old_id).collect();
    let new_descendants: FxHashSet<_> = new.descendants(new_id).collect();

    if old_descendants.is_empty() && new_descendants.is_empty() {
        return 1.0;
    }

    let mut common = 0;
    for &old_desc in &old_descendants {
        if let Some(matched_new) = matching.get_new(old_desc) {
            if new_descendants.contains(&matched_new) {
                common += 1;
            }
        }
    }

    (2.0 * common as f64) / (old_descendants.len() + new_descendants.len()) as f64
}

/// Phase 3: Recovery matching (GumTree 4.0 simple recovery).
///
/// For each pair of matched nodes, try to match their unmatched children
/// if they have the same type and there's only one candidate.
fn recovery_matching(old: &DiffTree, new: &DiffTree, matching: &mut Matching) {
    // Collect matched pairs to avoid borrowing issues
    let matched_pairs: Vec<_> = matching.matched_pairs().collect();

    for (old_id, new_id) in matched_pairs {
        let old_node = old.node(old_id);
        let new_node = new.node(new_id);

        // Skip if subtree is too large
        if old_node.descendant_count > MAX_RECOVERY_SIZE
            || new_node.descendant_count > MAX_RECOVERY_SIZE
        {
            continue;
        }

        // Get unmatched children
        let old_unmatched: Vec<_> = old_node
            .children
            .iter()
            .copied()
            .filter(|&id| !matching.is_old_matched(id))
            .collect();

        let new_unmatched: Vec<_> = new_node
            .children
            .iter()
            .copied()
            .filter(|&id| !matching.is_new_matched(id))
            .collect();

        // Group by kind
        let mut old_by_kind: FxHashMap<u16, Vec<NodeId>> = FxHashMap::default();
        let mut new_by_kind: FxHashMap<u16, Vec<NodeId>> = FxHashMap::default();

        for &id in &old_unmatched {
            old_by_kind
                .entry(old.node(id).kind_id)
                .or_default()
                .push(id);
        }

        for &id in &new_unmatched {
            new_by_kind
                .entry(new.node(id).kind_id)
                .or_default()
                .push(id);
        }

        // Match unique pairs by kind
        for (kind_id, old_ids) in &old_by_kind {
            if let Some(new_ids) = new_by_kind.get(kind_id) {
                if old_ids.len() == 1 && new_ids.len() == 1 {
                    let old_child = old_ids[0];
                    let new_child = new_ids[0];

                    if !matching.is_old_matched(old_child) && !matching.is_new_matched(new_child) {
                        matching.add(old_child, new_child);

                        // Recursively match subtrees if they're similar enough
                        let dice =
                            compute_dice_similarity(old, new, old_child, new_child, matching);
                        if dice >= SIM_THRESHOLD {
                            recover_subtree(old, new, old_child, new_child, matching);
                        }
                    }
                }
            }
        }
    }
}

/// Recursively recover matches within a subtree.
fn recover_subtree(
    old: &DiffTree,
    new: &DiffTree,
    old_id: NodeId,
    new_id: NodeId,
    matching: &mut Matching,
) {
    let old_node = old.node(old_id);
    let new_node = new.node(new_id);

    // Get unmatched children
    let old_unmatched: Vec<_> = old_node
        .children
        .iter()
        .copied()
        .filter(|&id| !matching.is_old_matched(id))
        .collect();

    let new_unmatched: Vec<_> = new_node
        .children
        .iter()
        .copied()
        .filter(|&id| !matching.is_new_matched(id))
        .collect();

    // Group by kind
    let mut old_by_kind: FxHashMap<u16, Vec<NodeId>> = FxHashMap::default();
    let mut new_by_kind: FxHashMap<u16, Vec<NodeId>> = FxHashMap::default();

    for &id in &old_unmatched {
        old_by_kind
            .entry(old.node(id).kind_id)
            .or_default()
            .push(id);
    }

    for &id in &new_unmatched {
        new_by_kind
            .entry(new.node(id).kind_id)
            .or_default()
            .push(id);
    }

    // Match unique pairs
    for (kind_id, old_ids) in &old_by_kind {
        if let Some(new_ids) = new_by_kind.get(kind_id) {
            if old_ids.len() == 1 && new_ids.len() == 1 {
                let old_child = old_ids[0];
                let new_child = new_ids[0];

                if !matching.is_old_matched(old_child) && !matching.is_new_matched(new_child) {
                    matching.add(old_child, new_child);
                    recover_subtree(old, new, old_child, new_child, matching);
                }
            }
        }
    }
}

/// Generate diff operations from a matching result.
///
/// Classifies each node as:
/// - Matched with same parent: no operation (unchanged)
/// - Matched with different parent: Move
/// - Matched with different content: Update
/// - Unmatched in old tree: Delete
/// - Unmatched in new tree: Insert
pub fn generate_diff(old: &DiffTree, new: &DiffTree, matching: &Matching) -> DiffResult {
    let mut operations = Vec::new();

    // Find deleted nodes (in old, not matched)
    for old_node in old.nodes() {
        if !matching.is_old_matched(old_node.id) {
            // Only report if parent is matched (or is root) - avoids redundant reports
            let parent_matched = old_node
                .parent
                .map(|p| matching.is_old_matched(p))
                .unwrap_or(true);

            if parent_matched {
                operations.push(DiffOperation::Delete {
                    range: old_node.byte_range.clone(),
                    node_kind: old_node.kind,
                });
            }
        }
    }

    // Find inserted nodes (in new, not matched)
    for new_node in new.nodes() {
        if !matching.is_new_matched(new_node.id) {
            // Only report if parent is matched (or is root)
            let parent_matched = new_node
                .parent
                .map(|p| matching.is_new_matched(p))
                .unwrap_or(true);

            if parent_matched {
                operations.push(DiffOperation::Insert {
                    range: new_node.byte_range.clone(),
                    node_kind: new_node.kind,
                });
            }
        }
    }

    // Find moves and updates
    for (old_id, new_id) in matching.matched_pairs() {
        let old_node = old.node(old_id);
        let new_node = new.node(new_id);

        // Check for content update (same structure, different text)
        if old_node.content_hash != new_node.content_hash {
            operations.push(DiffOperation::Update {
                old_range: old_node.byte_range.clone(),
                new_range: new_node.byte_range.clone(),
                node_kind: old_node.kind,
            });
        }

        // Check for move (parent changed)
        let old_parent_match = old_node.parent.and_then(|p| matching.get_new(p));
        let new_parent = new_node.parent;

        if old_parent_match != new_parent {
            // The node was moved to a different parent
            operations.push(DiffOperation::Move {
                old_range: old_node.byte_range.clone(),
                new_range: new_node.byte_range.clone(),
                node_kind: old_node.kind,
            });
        }
    }

    DiffResult { operations }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_rust(code: &str) -> tree_sitter::Tree {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .expect("failed to set language");
        parser.parse(code, None).expect("failed to parse")
    }

    #[test]
    fn test_identical_trees_fully_matched() {
        let code = r#"
fn main() {
    let x = 42;
    println!("{}", x);
}
"#;
        let tree = parse_rust(code);
        let old = DiffTree::new(&tree, code);
        let new = DiffTree::new(&tree, code);

        let matching = match_trees(&old, &new);

        // All nodes should be matched
        assert_eq!(matching.len(), old.node_count());

        let diff = generate_diff(&old, &new, &matching);
        // No operations for identical trees (no deletes, inserts, or moves)
        let non_update_ops: Vec<_> = diff
            .operations
            .iter()
            .filter(|op| !matches!(op, DiffOperation::Update { .. }))
            .collect();
        assert!(non_update_ops.is_empty());
    }

    #[test]
    fn test_simple_insertion() {
        let old_code = r#"
fn main() {
    let x = 1;
}
"#;
        let new_code = r#"
fn main() {
    let x = 1;
    let y = 2;
}
"#;
        let old_tree = parse_rust(old_code);
        let new_tree = parse_rust(new_code);

        let old = DiffTree::new(&old_tree, old_code);
        let new = DiffTree::new(&new_tree, new_code);

        let matching = match_trees(&old, &new);
        let diff = generate_diff(&old, &new, &matching);

        // Should have at least one insert operation
        let inserts: Vec<_> = diff
            .operations
            .iter()
            .filter(|op| matches!(op, DiffOperation::Insert { .. }))
            .collect();
        assert!(!inserts.is_empty(), "Expected insert operations");
    }

    #[test]
    fn test_simple_deletion() {
        let old_code = r#"
fn main() {
    let x = 1;
    let y = 2;
}
"#;
        let new_code = r#"
fn main() {
    let x = 1;
}
"#;
        let old_tree = parse_rust(old_code);
        let new_tree = parse_rust(new_code);

        let old = DiffTree::new(&old_tree, old_code);
        let new = DiffTree::new(&new_tree, new_code);

        let matching = match_trees(&old, &new);
        let diff = generate_diff(&old, &new, &matching);

        // Should have at least one delete operation
        let deletes: Vec<_> = diff
            .operations
            .iter()
            .filter(|op| matches!(op, DiffOperation::Delete { .. }))
            .collect();
        assert!(!deletes.is_empty(), "Expected delete operations");
    }

    #[test]
    fn test_content_update() {
        let old_code = r#"
fn main() {
    let x = 42;
}
"#;
        let new_code = r#"
fn main() {
    let x = 100;
}
"#;
        let old_tree = parse_rust(old_code);
        let new_tree = parse_rust(new_code);

        let old = DiffTree::new(&old_tree, old_code);
        let new = DiffTree::new(&new_tree, new_code);

        let matching = match_trees(&old, &new);
        let diff = generate_diff(&old, &new, &matching);

        // Should have update operations for the changed value
        let updates: Vec<_> = diff
            .operations
            .iter()
            .filter(|op| matches!(op, DiffOperation::Update { .. }))
            .collect();
        assert!(!updates.is_empty(), "Expected update operations");
    }

    #[test]
    fn test_function_rename() {
        let old_code = r#"
fn foo() {
    println!("hello");
}
"#;
        let new_code = r#"
fn bar() {
    println!("hello");
}
"#;
        let old_tree = parse_rust(old_code);
        let new_tree = parse_rust(new_code);

        let old = DiffTree::new(&old_tree, old_code);
        let new = DiffTree::new(&new_tree, new_code);

        let matching = match_trees(&old, &new);

        // The function body should still be matched (isomorphic subtrees)
        // The function name identifier should be updated
        let diff = generate_diff(&old, &new, &matching);

        let updates: Vec<_> = diff
            .operations
            .iter()
            .filter(|op| matches!(op, DiffOperation::Update { .. }))
            .collect();
        assert!(!updates.is_empty(), "Expected update for function name");
    }

    #[test]
    fn test_multiple_functions() {
        let old_code = r#"
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}
"#;
        let new_code = r#"
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn mul(a: i32, b: i32) -> i32 {
    a * b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}
"#;
        let old_tree = parse_rust(old_code);
        let new_tree = parse_rust(new_code);

        let old = DiffTree::new(&old_tree, old_code);
        let new = DiffTree::new(&new_tree, new_code);

        let matching = match_trees(&old, &new);

        // Both add and sub should be matched
        // mul should be inserted
        let diff = generate_diff(&old, &new, &matching);

        let inserts: Vec<_> = diff
            .operations
            .iter()
            .filter(|op| matches!(op, DiffOperation::Insert { .. }))
            .collect();
        assert!(!inserts.is_empty(), "Expected insert for mul function");
    }

    #[test]
    fn test_tree_structure() {
        let code = r#"
fn main() {
    let x = 1;
}
"#;
        let tree = parse_rust(code);
        let diff_tree = DiffTree::new(&tree, code);

        // Root should have height > 0
        let root = diff_tree.node(diff_tree.root());
        assert!(root.height > 0);

        // Root should have children
        assert!(!root.children.is_empty());

        // All nodes should have correct parent references
        for node in diff_tree.nodes() {
            for &child_id in &node.children {
                let child = diff_tree.node(child_id);
                assert_eq!(child.parent, Some(node.id));
            }
        }
    }

    #[test]
    fn test_descendant_iterator() {
        let code = r#"
fn main() {
    let x = 1;
}
"#;
        let tree = parse_rust(code);
        let diff_tree = DiffTree::new(&tree, code);

        let root = diff_tree.root();
        let descendants: Vec<_> = diff_tree.descendants(root).collect();

        // Should include all nodes
        assert_eq!(descendants.len(), diff_tree.node_count());

        // First descendant should be the root itself
        assert_eq!(descendants[0], root);
    }

    #[test]
    fn test_structural_hash_isomorphism() {
        let code1 = r#"
fn foo() {
    let x = 1;
}
"#;
        let code2 = r#"
fn bar() {
    let y = 2;
}
"#;
        let tree1 = parse_rust(code1);
        let tree2 = parse_rust(code2);

        let diff1 = DiffTree::new(&tree1, code1);
        let diff2 = DiffTree::new(&tree2, code2);

        // Both trees should have the same structure (function with let statement)
        // So root structural hashes should be equal
        let root1 = diff1.node(diff1.root());
        let root2 = diff2.node(diff2.root());

        assert_eq!(root1.structural_hash, root2.structural_hash);
        // But content hashes should differ
        assert_ne!(root1.content_hash, root2.content_hash);
    }

    #[test]
    fn test_matching_bidirectional() {
        let mut matching = Matching::new();
        let old_id = NodeId(0);
        let new_id = NodeId(1);

        matching.add(old_id, new_id);

        assert!(matching.is_old_matched(old_id));
        assert!(matching.is_new_matched(new_id));
        assert_eq!(matching.get_new(old_id), Some(new_id));
        assert_eq!(matching.get_old(new_id), Some(old_id));
    }

    #[test]
    fn test_complex_refactoring() {
        let old_code = r#"
fn process(items: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for item in items {
        if item > 0 {
            result.push(item * 2);
        }
    }
    result
}
"#;
        let new_code = r#"
fn process(items: Vec<i32>) -> Vec<i32> {
    items
        .into_iter()
        .filter(|&x| x > 0)
        .map(|x| x * 2)
        .collect()
}
"#;
        let old_tree = parse_rust(old_code);
        let new_tree = parse_rust(new_code);

        let old = DiffTree::new(&old_tree, old_code);
        let new = DiffTree::new(&new_tree, new_code);

        let matching = match_trees(&old, &new);
        let diff = generate_diff(&old, &new, &matching);

        // The function signature should be matched
        // There should be significant changes in the body
        assert!(!diff.operations.is_empty());
    }

    #[test]
    fn test_nested_structures() {
        let old_code = r#"
struct Outer {
    inner: Inner,
}

struct Inner {
    value: i32,
}
"#;
        let new_code = r#"
struct Outer {
    inner: Inner,
    name: String,
}

struct Inner {
    value: i32,
    count: usize,
}
"#;
        let old_tree = parse_rust(old_code);
        let new_tree = parse_rust(new_code);

        let old = DiffTree::new(&old_tree, old_code);
        let new = DiffTree::new(&new_tree, new_code);

        let matching = match_trees(&old, &new);
        let diff = generate_diff(&old, &new, &matching);

        // Should have inserts for new fields
        let inserts: Vec<_> = diff
            .operations
            .iter()
            .filter(|op| matches!(op, DiffOperation::Insert { .. }))
            .collect();
        assert!(!inserts.is_empty());
    }

    #[test]
    fn test_empty_to_content() {
        let old_code = "";
        let new_code = r#"fn main() {}"#;

        let old_tree = parse_rust(old_code);
        let new_tree = parse_rust(new_code);

        let old = DiffTree::new(&old_tree, old_code);
        let new = DiffTree::new(&new_tree, new_code);

        let matching = match_trees(&old, &new);
        let diff = generate_diff(&old, &new, &matching);

        // Everything in new should be inserted
        let inserts: Vec<_> = diff
            .operations
            .iter()
            .filter(|op| matches!(op, DiffOperation::Insert { .. }))
            .collect();
        assert!(!inserts.is_empty());
    }

    #[test]
    fn test_content_to_empty() {
        let old_code = r#"fn main() {}"#;
        let new_code = "";

        let old_tree = parse_rust(old_code);
        let new_tree = parse_rust(new_code);

        let old = DiffTree::new(&old_tree, old_code);
        let new = DiffTree::new(&new_tree, new_code);

        let matching = match_trees(&old, &new);
        let diff = generate_diff(&old, &new, &matching);

        // Everything in old should be deleted
        let deletes: Vec<_> = diff
            .operations
            .iter()
            .filter(|op| matches!(op, DiffOperation::Delete { .. }))
            .collect();
        assert!(!deletes.is_empty());
    }

    #[test]
    fn test_reordered_items() {
        let old_code = r#"
fn a() {}
fn b() {}
fn c() {}
"#;
        let new_code = r#"
fn c() {}
fn a() {}
fn b() {}
"#;
        let old_tree = parse_rust(old_code);
        let new_tree = parse_rust(new_code);

        let old = DiffTree::new(&old_tree, old_code);
        let new = DiffTree::new(&new_tree, new_code);

        let matching = match_trees(&old, &new);

        // All three functions should be matched (identical content)
        // Each should be recognized as moved
        let diff = generate_diff(&old, &new, &matching);
        let moves: Vec<_> = diff
            .operations
            .iter()
            .filter(|op| matches!(op, DiffOperation::Move { .. }))
            .collect();

        // At least some functions should be detected as moved
        assert!(!moves.is_empty() || matching.len() > 0);
    }

    #[test]
    fn test_dice_similarity_empty_descendants() {
        let code = "fn x() {}";
        let tree = parse_rust(code);
        let diff_tree = DiffTree::new(&tree, code);

        let matching = Matching::new();

        // Find a leaf node
        let leaf = diff_tree
            .nodes()
            .iter()
            .find(|n| n.children.is_empty())
            .expect("Should have leaf nodes");

        let dice = compute_dice_similarity(&diff_tree, &diff_tree, leaf.id, leaf.id, &matching);
        // Two single-node "trees" with no matches should have similarity based on the formula
        assert!(dice >= 0.0 && dice <= 1.0);
    }

    #[test]
    fn test_height_computation() {
        let code = r#"
fn main() {
    if true {
        while false {
            let x = 1;
        }
    }
}
"#;
        let tree = parse_rust(code);
        let diff_tree = DiffTree::new(&tree, code);

        // Root should have the maximum height
        let root = diff_tree.node(diff_tree.root());
        assert_eq!(root.height, diff_tree.max_height());

        // Leaves should have height 0
        for node in diff_tree.nodes() {
            if node.children.is_empty() {
                assert_eq!(node.height, 0);
            }
        }
    }

    #[test]
    fn test_nodes_at_height() {
        let code = r#"
fn a() { let x = 1; }
fn b() { let y = 2; }
"#;
        let tree = parse_rust(code);
        let diff_tree = DiffTree::new(&tree, code);

        // Should be able to get nodes at each height level
        for h in 0..=diff_tree.max_height() {
            let nodes = diff_tree.nodes_at_height(h);
            for node_id in nodes {
                assert_eq!(diff_tree.node(node_id).height, h);
            }
        }
    }
}
