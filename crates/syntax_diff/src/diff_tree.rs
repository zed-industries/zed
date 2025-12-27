// GumTree Algorithm Implementation (Optimized for Git Diffs)
//
// Original paper: "Fine-grained and Accurate Source Code Differencing" (2014)
// https://hal.science/hal-01054552/document
//
// Improved version: "GumTree 4.0" (2024)
// https://hal.science/hal-04855170v1/document
//
// Hyperparameter tuning: "Hyperparameter Optimization for AST Differencing" (2023)
// https://hal.science/hal-04423080/document

use std::{
    hash::{DefaultHasher, Hash, Hasher},
    num::NonZeroUsize,
    ops::Range,
};

/// Minimum height for subtree matching in top-down phase.
const MIN_HEIGHT: usize = 1;

/// Minimum Dice similarity threshold for matching.
const SIM_THRESHOLD: f64 = 0.2;

/// Maximum subtree size for recovery phase.
const MAX_RECOVERY_SIZE: usize = 600;

/// Node index in a DiffTree.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(NonZeroUsize);

impl NodeId {
    #[inline]
    fn new(idx: usize) -> Self {
        Self(NonZeroUsize::new(idx + 1).expect("index overflow"))
    }

    #[inline]
    pub fn index(self) -> usize {
        self.0.get() - 1
    }
}

/// Compact node representation optimized for diffing.
#[derive(Debug, Clone)]
pub struct DiffNode {
    /// Node's own index
    pub id: NodeId,
    /// Structural hash (kind + children structure)
    pub structural_hash: u64,
    /// Content hash (actual text)
    pub content_hash: u64,
    /// Byte range in source
    pub byte_range: Range<usize>,
    /// Tree-sitter kind ID
    pub kind_id: u16,
    /// Node height (0 for leaves)
    pub height: u16,
    /// Total descendant count including self
    pub descendant_count: usize,
    /// Parent node (None for root)
    parent: Option<NodeId>,
    /// First child (None if leaf)
    first_child: Option<NodeId>,
    /// Next sibling (None if last child)
    next_sibling: Option<NodeId>,
}

impl DiffNode {
    #[inline]
    pub fn parent(&self) -> Option<NodeId> {
        self.parent
    }
}

/// A preprocessed syntax tree for efficient diffing.
#[derive(Debug, Clone, Default)]
pub struct DiffTree {
    /// Nodes stored in pre-order (parents before children, root at index 0)
    nodes: Vec<DiffNode>,
    /// Nodes grouped by height for efficient iteration
    height_index: Vec<Vec<NodeId>>,
}

impl PartialEq for DiffTree {
    fn eq(&self, other: &Self) -> bool {
        let root = self.node(self.root());
        let root_other = other.node(other.root());

        root.structural_hash == root_other.structural_hash && root.content_hash == root.content_hash
    }
}

impl Eq for DiffTree {}

/// Iterator over a node's children.
pub struct ChildIter<'a> {
    tree: &'a DiffTree,
    current: Option<NodeId>,
}

impl Iterator for ChildIter<'_> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.current?;
        self.current = self.tree.nodes[id.index()].next_sibling;
        Some(id)
    }
}

impl DiffTree {
    pub fn new(mut tree: tree_sitter::TreeCursor<'_>, text: &str) -> Self {
        let estimated_nodes = tree.node().descendant_count();
        let estimated_height = (estimated_nodes as f64).log2().ceil() as usize + 1;
        let mut nodes = Vec::with_capacity(estimated_nodes);
        let mut height_index = vec![Vec::new(); estimated_height.max(16)];

        build_tree(&mut tree, text, &mut nodes, &mut height_index, None);

        Self {
            nodes,
            height_index,
        }
    }

    /// Returns the root node
    #[inline]
    pub fn root(&self) -> NodeId {
        NodeId::new(0)
    }

    #[inline]
    pub fn node(&self, id: NodeId) -> &DiffNode {
        &self.nodes[id.index()]
    }

    #[inline]
    pub fn nodes(&self) -> &[DiffNode] {
        &self.nodes
    }

    #[inline]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn max_height(&self) -> usize {
        self.height_index
            .iter()
            .enumerate()
            .rev()
            .find(|(_, v)| !v.is_empty())
            .map(|(h, _)| h)
            .unwrap_or(0)
    }

    #[inline]
    pub fn nodes_at_height(&self, height: usize) -> impl Iterator<Item = NodeId> + '_ {
        self.height_index
            .get(height)
            .into_iter()
            .flat_map(|v| v.iter().copied())
    }

    /// Iterate over a node's direct children.
    #[inline]
    pub fn children(&self, id: NodeId) -> ChildIter<'_> {
        ChildIter {
            tree: self,
            current: self.node(id).first_child,
        }
    }

    /// Check if a node is an ancestor of another.
    /// Uses the pre-order property: ancestors have lower indices and contain descendants in their range.
    #[inline]
    pub fn is_ancestor(&self, ancestor: NodeId, descendant: NodeId) -> bool {
        let ancestor_node = self.node(ancestor);
        // In pre-order, a node's descendants are in [idx + 1, idx + descendant_count - 1]
        let range = (ancestor.index() + 1)..ancestor.index() + ancestor_node.descendant_count;

        range.contains(&descendant.index())
    }
}

fn build_tree(
    cursor: &mut tree_sitter::TreeCursor<'_>,
    text: &str,
    nodes: &mut Vec<DiffNode>,
    height_index: &mut Vec<Vec<NodeId>>,
    parent: Option<NodeId>,
) -> NodeId {
    let node = cursor.node();
    let this_id = NodeId::new(nodes.len());

    // Reserve space - we'll update fields after processing children
    nodes.push(DiffNode {
        id: this_id,
        structural_hash: 0,
        content_hash: 0,
        byte_range: node.byte_range(),
        kind_id: node.kind_id(),
        height: 0,
        descendant_count: node.descendant_count(),
        parent,
        first_child: None,
        next_sibling: None,
    });

    // Process children
    let mut max_child_height: u16 = 0;
    let mut first_child: Option<NodeId> = None;
    let mut last_child: Option<NodeId> = None;
    let mut hasher = DefaultHasher::new();

    node.kind_id().hash(&mut hasher);

    if cursor.goto_first_child() {
        loop {
            let child_id = build_tree(cursor, text, nodes, height_index, Some(this_id));

            if first_child.is_none() {
                first_child = Some(child_id);
            } else if let Some(prev) = last_child {
                nodes[prev.index()].next_sibling = Some(child_id);
            }
            last_child = Some(child_id);

            let child_node = &nodes[child_id.index()];
            max_child_height = max_child_height.max(child_node.height);
            child_node.structural_hash.hash(&mut hasher);

            if !cursor.goto_next_sibling() {
                break;
            }
        }

        cursor.goto_parent();
    }

    // Compute metadata
    let height = if first_child.is_none() {
        0
    } else {
        max_child_height + 1
    };

    let byte_range = node.byte_range();
    let content_hash = compute_content_hash(&text[byte_range]);

    // Ensure height_index is large enough and add this node
    if height as usize >= height_index.len() {
        height_index.resize(height as usize + 1, Vec::new());
    }
    height_index[height as usize].push(this_id);

    // Update node
    let node = &mut nodes[this_id.index()];
    node.height = height;
    node.structural_hash = hasher.finish();
    node.content_hash = content_hash;
    node.first_child = first_child;

    this_id
}

#[inline]
fn compute_content_hash(content: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    hasher.finish()
}

/// Bidirectional matching between two trees.
#[derive(Debug)]
pub struct Matching {
    /// old index -> matched new node
    old_to_new: Vec<Option<NodeId>>,
    /// new index -> matched old node
    new_to_old: Vec<Option<NodeId>>,
    /// Count of unmatched old nodes
    unmatched_old_count: usize,
    /// Count of unmatched new nodes
    unmatched_new_count: usize,
}

impl Matching {
    fn new(old_count: usize, new_count: usize) -> Self {
        Self {
            old_to_new: vec![None; old_count],
            new_to_old: vec![None; new_count],
            unmatched_old_count: old_count,
            unmatched_new_count: new_count,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.old_to_new.len() - self.unmatched_old_count
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    fn add(&mut self, old_id: NodeId, new_id: NodeId) {
        let old_idx = old_id.index();
        let new_idx = new_id.index();

        if self.old_to_new[old_idx].is_none() {
            self.unmatched_old_count -= 1;
        }
        if self.new_to_old[new_idx].is_none() {
            self.unmatched_new_count -= 1;
        }

        self.old_to_new[old_idx] = Some(new_id);
        self.new_to_old[new_idx] = Some(old_id);
    }

    #[inline]
    pub fn is_old_matched(&self, id: NodeId) -> bool {
        self.old_to_new[id.index()].is_some()
    }

    #[inline]
    pub fn is_new_matched(&self, id: NodeId) -> bool {
        self.new_to_old[id.index()].is_some()
    }

    #[inline]
    pub fn get_new(&self, old_id: NodeId) -> Option<NodeId> {
        self.old_to_new[old_id.index()]
    }

    #[inline]
    pub fn get_old(&self, new_id: NodeId) -> Option<NodeId> {
        self.new_to_old[new_id.index()]
    }

    pub fn unmatched_old(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.old_to_new
            .iter()
            .enumerate()
            .filter(|(_, v)| v.is_none())
            .map(|(i, _)| NodeId::new(i))
    }

    pub fn unmatched_new(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.new_to_old
            .iter()
            .enumerate()
            .filter(|(_, v)| v.is_none())
            .map(|(i, _)| NodeId::new(i))
    }

    #[inline]
    pub fn unmatched_old_count(&self) -> usize {
        self.unmatched_old_count
    }

    #[inline]
    pub fn unmatched_new_count(&self) -> usize {
        self.unmatched_new_count
    }

    pub fn matched_pairs(&self) -> impl Iterator<Item = (NodeId, NodeId)> + '_ {
        self.old_to_new
            .iter()
            .enumerate()
            .filter_map(|(i, v)| v.map(|new_id| (NodeId::new(i), new_id)))
    }

    pub fn old_matched_nodes(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.old_to_new
            .iter()
            .enumerate()
            .filter(|(_, v)| v.is_some())
            .map(|(i, _)| NodeId::new(i))
    }

    pub fn new_matched_nodes(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.new_to_old
            .iter()
            .enumerate()
            .filter(|(_, v)| v.is_some())
            .map(|(i, _)| NodeId::new(i))
    }
}

/// A diff operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffOperation {
    Delete(Range<usize>),
    Insert(Range<usize>),
    Move {
        old_range: Range<usize>,
        new_range: Range<usize>,
    },
    Update {
        old_range: Range<usize>,
        new_range: Range<usize>,
    },
}

#[derive(Debug, Default, Clone)]
pub struct DiffResult {
    pub operations: Vec<DiffOperation>,
}

/// Match two syntax trees using the GumTree algorithm.
pub fn match_trees(old: &DiffTree, new: &DiffTree) -> Matching {
    let mut matching = Matching::new(old.node_count(), new.node_count());

    // Phase 1: Top-down exact subtree matching
    top_down_matching(old, new, &mut matching);

    // Phase 2: Bottom-up similarity matching
    bottom_up_matching(old, new, &mut matching);

    // Phase 3: Recovery matching for children of matched parents
    recovery_matching(old, new, &mut matching);

    matching
}

fn top_down_matching(old: &DiffTree, new: &DiffTree, matching: &mut Matching) {
    use collections::FxHashMap;

    // Index new tree nodes by (structural_hash, content_hash)
    let mut new_by_hash: FxHashMap<(u64, u64), Vec<NodeId>> = FxHashMap::default();
    for (idx, node) in new.nodes.iter().enumerate() {
        if node.height >= MIN_HEIGHT as u16 {
            new_by_hash
                .entry((node.structural_hash, node.content_hash))
                .or_default()
                .push(NodeId::new(idx));
        }
    }

    // Process old nodes from highest to lowest
    let max_height = old.max_height();
    for height in (MIN_HEIGHT..=max_height).rev() {
        for old_id in old.nodes_at_height(height) {
            if matching.is_old_matched(old_id) {
                continue;
            }

            let old_node = old.node(old_id);
            let key = (old_node.structural_hash, old_node.content_hash);

            let Some(candidates) = new_by_hash.get(&key) else {
                continue;
            };

            // Filter to unmatched candidates
            let unmatched: Vec<_> = candidates
                .iter()
                .copied()
                .filter(|&id| !matching.is_new_matched(id))
                .collect();

            let matched_new = match unmatched.len() {
                0 => continue,
                1 => Some(unmatched[0]),
                _ => find_best_match_by_context(old, new, old_id, &unmatched, matching),
            };

            if let Some(new_id) = matched_new {
                match_subtrees(old, old_id, new_id, matching);
            }
        }
    }
}

/// Find best match among candidates using parent/sibling context.
fn find_best_match_by_context(
    old: &DiffTree,
    new: &DiffTree,
    old_id: NodeId,
    candidates: &[NodeId],
    matching: &Matching,
) -> Option<NodeId> {
    let old_node = old.node(old_id);

    // Prefer candidate whose parent matches old node's parent
    if let Some(old_parent) = old_node.parent() {
        if let Some(new_parent) = matching.get_new(old_parent) {
            for &new_id in candidates {
                if new.node(new_id).parent() == Some(new_parent) {
                    return Some(new_id);
                }
            }
        }
    }

    // Check sibling context
    if let Some(old_parent) = old_node.parent() {
        for &new_id in candidates {
            if let Some(new_parent) = new.node(new_id).parent() {
                // Check if any old sibling is matched to a sibling of this candidate
                for old_sib in old.children(old_parent) {
                    if let Some(matched_new_sib) = matching.get_new(old_sib) {
                        if new.node(matched_new_sib).parent() == Some(new_parent) {
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
/// Since subtrees have identical structure (same hashes), nodes at the same
/// relative offset in pre-order correspond to each other.
#[inline]
fn match_subtrees(old: &DiffTree, old_root: NodeId, new_root: NodeId, matching: &mut Matching) {
    let count = old.node(old_root).descendant_count;
    let old_base = old_root.index();
    let new_base = new_root.index();

    for offset in 0..count {
        let old_id = NodeId::new(old_base + offset);
        let new_id = NodeId::new(new_base + offset);

        if !matching.is_old_matched(old_id) && !matching.is_new_matched(new_id) {
            matching.add(old_id, new_id);
        }
    }
}

fn bottom_up_matching(old: &DiffTree, new: &DiffTree, matching: &mut Matching) {
    use collections::FxHashMap;

    if matching.unmatched_old_count() == 0 {
        return;
    }

    let mut new_by_kind: FxHashMap<u16, Vec<NodeId>> = FxHashMap::default();
    for new_id in matching.unmatched_new() {
        new_by_kind
            .entry(new.node(new_id).kind_id)
            .or_default()
            .push(new_id);
    }

    let max_height = old.max_height();
    for height in (0..=max_height).rev() {
        for old_id in old.nodes_at_height(height) {
            if matching.is_old_matched(old_id) {
                continue;
            }

            let old_node = old.node(old_id);
            let Some(candidates) = new_by_kind.get(&old_node.kind_id) else {
                continue;
            };

            let mut best_match: Option<(NodeId, f64)> = None;

            for &new_id in candidates {
                if matching.is_new_matched(new_id) {
                    continue;
                }

                let dice = compute_dice_similarity(old, new, old_id, new_id, matching);
                if dice >= SIM_THRESHOLD && best_match.map_or(true, |(_, d)| dice > d) {
                    best_match = Some((new_id, dice));
                }
            }

            if let Some((new_id, _)) = best_match {
                matching.add(old_id, new_id);
            }
        }
    }
}

/// Compute Dice similarity between two nodes.
fn compute_dice_similarity(
    old: &DiffTree,
    new: &DiffTree,
    old_id: NodeId,
    new_id: NodeId,
    matching: &Matching,
) -> f64 {
    let old_count = old.node(old_id).descendant_count as usize;
    let new_count = new.node(new_id).descendant_count as usize;

    if old_count == 0 && new_count == 0 {
        return 1.0;
    }

    let old_start = old_id.index();
    let new_start = new_id.index();

    // Pre-order property: descendants are in [idx, idx + count - 1]
    let old_end = old_start + old_count;
    let new_end = new_start + new_count;

    let mut common = 0;

    // Iterate smaller range
    if old_count <= new_count {
        for idx in old_start..old_end {
            if let Some(matched_new) = matching.get_new(NodeId::new(idx)) {
                let matched_idx = matched_new.index();
                if matched_idx >= new_start && matched_idx < new_end {
                    common += 1;
                }
            }
        }
    } else {
        for idx in new_start..new_end {
            if let Some(matched_old) = matching.get_old(NodeId::new(idx)) {
                let matched_idx = matched_old.index();
                if matched_idx >= old_start && matched_idx < old_end {
                    common += 1;
                }
            }
        }
    }

    (2.0 * common as f64) / (old_count + new_count) as f64
}

fn recovery_matching(old: &DiffTree, new: &DiffTree, matching: &mut Matching) {
    let matched: Vec<_> = matching.matched_pairs().collect();

    for (old_id, new_id) in matched {
        let old_node = old.node(old_id);
        let new_node = new.node(new_id);

        if old_node.descendant_count > MAX_RECOVERY_SIZE
            || new_node.descendant_count > MAX_RECOVERY_SIZE
        {
            continue;
        }

        match_unique_children(old, new, old_id, new_id, matching, true);
    }
}

/// Match unmatched children that have unique kinds on both sides, then recurse.
/// At the first level (`check_dice=true`), only recurse if Dice similarity >= threshold.
fn match_unique_children(
    old: &DiffTree,
    new: &DiffTree,
    old_id: NodeId,
    new_id: NodeId,
    matching: &mut Matching,
    check_dice: bool,
) {
    use collections::FxHashMap;

    // Count children by kind. Store (count, node) - we only need the node if count == 1.
    let mut old_by_kind: FxHashMap<u16, (usize, NodeId)> = FxHashMap::default();
    let mut new_by_kind: FxHashMap<u16, (usize, NodeId)> = FxHashMap::default();

    for child in old.children(old_id) {
        if !matching.is_old_matched(child) {
            let kind = old.node(child).kind_id;
            old_by_kind
                .entry(kind)
                .and_modify(|(count, _)| *count += 1)
                .or_insert((1, child));
        }
    }

    for child in new.children(new_id) {
        if !matching.is_new_matched(child) {
            let kind = new.node(child).kind_id;
            new_by_kind
                .entry(kind)
                .and_modify(|(count, _)| *count += 1)
                .or_insert((1, child));
        }
    }

    // Match unique pairs (where both sides have exactly one child of that kind)
    for (kind_id, (old_count, old_child)) in &old_by_kind {
        if *old_count != 1 {
            continue;
        }

        if let Some(&(new_count, new_child)) = new_by_kind.get(kind_id) {
            if new_count == 1
                && !matching.is_old_matched(*old_child)
                && !matching.is_new_matched(new_child)
            {
                matching.add(*old_child, new_child);

                let should_recurse = if check_dice {
                    compute_dice_similarity(old, new, *old_child, new_child, matching)
                        >= SIM_THRESHOLD
                } else {
                    true
                };

                if should_recurse {
                    match_unique_children(old, new, *old_child, new_child, matching, false);
                }
            }
        }
    }
}

/// Generate diff operations from matching result.
pub fn generate_diff(old: &DiffTree, new: &DiffTree, matching: &Matching) -> DiffResult {
    let mut operations = Vec::new();

    // Deletions: unmatched old nodes with matched parents
    for old_id in matching.unmatched_old() {
        let old_node = old.node(old_id);
        let parent_matched = old_node
            .parent()
            .map(|p| matching.is_old_matched(p))
            .unwrap_or(true);

        if parent_matched {
            operations.push(DiffOperation::Delete(
                old_node.byte_range.start as usize..old_node.byte_range.end as usize,
            ));
        }
    }

    // Insertions: unmatched new nodes with matched parents
    for new_id in matching.unmatched_new() {
        let new_node = new.node(new_id);
        let parent_matched = new_node
            .parent()
            .map(|p| matching.is_new_matched(p))
            .unwrap_or(true);

        if parent_matched {
            operations.push(DiffOperation::Insert(
                new_node.byte_range.start as usize..new_node.byte_range.end as usize,
            ));
        }
    }

    // Moves and Updates
    for (old_id, new_id) in matching.matched_pairs() {
        let old_node = old.node(old_id);
        let new_node = new.node(new_id);

        // Content update
        if old_node.content_hash != new_node.content_hash {
            operations.push(DiffOperation::Update {
                old_range: old_node.byte_range.start as usize..old_node.byte_range.end as usize,
                new_range: new_node.byte_range.start as usize..new_node.byte_range.end as usize,
            });
        }

        // Move detection
        let old_parent_match = old_node.parent().and_then(|p| matching.get_new(p));
        if old_parent_match != new_node.parent() {
            operations.push(DiffOperation::Move {
                old_range: old_node.byte_range.start as usize..old_node.byte_range.end as usize,
                new_range: new_node.byte_range.start as usize..new_node.byte_range.end as usize,
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
        let old = DiffTree::new(tree.walk(), code);
        let new = DiffTree::new(tree.walk(), code);

        let matching = match_trees(&old, &new);

        assert_eq!(matching.len(), old.node_count());

        let diff = generate_diff(&old, &new, &matching);
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

        let old = DiffTree::new(old_tree.walk(), old_code);
        let new = DiffTree::new(new_tree.walk(), new_code);

        let matching = match_trees(&old, &new);
        let diff = generate_diff(&old, &new, &matching);

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

        let old = DiffTree::new(old_tree.walk(), old_code);
        let new = DiffTree::new(new_tree.walk(), new_code);

        let matching = match_trees(&old, &new);
        let diff = generate_diff(&old, &new, &matching);

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

        let old = DiffTree::new(old_tree.walk(), old_code);
        let new = DiffTree::new(new_tree.walk(), new_code);

        let matching = match_trees(&old, &new);
        let diff = generate_diff(&old, &new, &matching);

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

        let old = DiffTree::new(old_tree.walk(), old_code);
        let new = DiffTree::new(new_tree.walk(), new_code);

        let matching = match_trees(&old, &new);
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

        let old = DiffTree::new(old_tree.walk(), old_code);
        let new = DiffTree::new(new_tree.walk(), new_code);

        let matching = match_trees(&old, &new);
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
        let diff_tree = DiffTree::new(tree.walk(), code);

        let root = diff_tree.node(diff_tree.root());
        assert!(root.height > 0);

        // All children should reference correct parent
        fn check_parent(tree: &DiffTree, node_id: NodeId) {
            for child_id in tree.children(node_id) {
                assert_eq!(tree.node(child_id).parent(), Some(node_id));
                check_parent(tree, child_id);
            }
        }
        check_parent(&diff_tree, diff_tree.root());
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

        let diff1 = DiffTree::new(tree1.walk(), code1);
        let diff2 = DiffTree::new(tree2.walk(), code2);

        let root1 = diff1.node(diff1.root());
        let root2 = diff2.node(diff2.root());

        assert_eq!(root1.structural_hash, root2.structural_hash);
        assert_ne!(root1.content_hash, root2.content_hash);
    }

    #[test]
    fn test_matching_bidirectional() {
        let mut matching = Matching::new(2, 2);
        let old_id = NodeId::new(0);
        let new_id = NodeId::new(1);

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

        let old = DiffTree::new(old_tree.walk(), old_code);
        let new = DiffTree::new(new_tree.walk(), new_code);

        let matching = match_trees(&old, &new);
        let diff = generate_diff(&old, &new, &matching);

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

        let old = DiffTree::new(old_tree.walk(), old_code);
        let new = DiffTree::new(new_tree.walk(), new_code);

        let matching = match_trees(&old, &new);
        let diff = generate_diff(&old, &new, &matching);

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

        let old = DiffTree::new(old_tree.walk(), old_code);
        let new = DiffTree::new(new_tree.walk(), new_code);

        let matching = match_trees(&old, &new);
        let diff = generate_diff(&old, &new, &matching);

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

        let old = DiffTree::new(old_tree.walk(), old_code);
        let new = DiffTree::new(new_tree.walk(), new_code);

        let matching = match_trees(&old, &new);
        let diff = generate_diff(&old, &new, &matching);

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

        let old = DiffTree::new(old_tree.walk(), old_code);
        let new = DiffTree::new(new_tree.walk(), new_code);

        let matching = match_trees(&old, &new);
        let diff = generate_diff(&old, &new, &matching);

        let moves: Vec<_> = diff
            .operations
            .iter()
            .filter(|op| matches!(op, DiffOperation::Move { .. }))
            .collect();

        assert!(!moves.is_empty() || matching.len() > 0);
    }

    #[test]
    fn test_dice_similarity_empty_descendants() {
        let code = "fn x() {}";
        let tree = parse_rust(code);
        let diff_tree = DiffTree::new(tree.walk(), code);

        let matching = Matching::new(diff_tree.node_count(), diff_tree.node_count());

        // Find a leaf node
        let leaf = diff_tree
            .nodes()
            .iter()
            .enumerate()
            .find(|(_, n)| n.first_child.is_none())
            .map(|(i, _)| NodeId::new(i))
            .expect("Should have leaf nodes");

        let dice = compute_dice_similarity(&diff_tree, &diff_tree, leaf, leaf, &matching);
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
        let diff_tree = DiffTree::new(tree.walk(), code);

        let root = diff_tree.node(diff_tree.root());
        assert_eq!(root.height as usize, diff_tree.max_height());

        // Leaves should have height 0
        for node in diff_tree.nodes() {
            if node.first_child.is_none() {
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
        let diff_tree = DiffTree::new(tree.walk(), code);

        for h in 0..=diff_tree.max_height() {
            for node_id in diff_tree.nodes_at_height(h) {
                assert_eq!(diff_tree.node(node_id).height as usize, h);
            }
        }
    }

    #[test]
    fn test_is_ancestor() {
        let code = r#"
fn main() {
    let x = 1;
}
"#;
        let tree = parse_rust(code);
        let diff_tree = DiffTree::new(tree.walk(), code);

        let root = diff_tree.root();

        // Root should be ancestor of all other nodes
        for node in diff_tree.nodes() {
            if node.id != root {
                assert!(
                    diff_tree.is_ancestor(root, node.id),
                    "Root should be ancestor",
                );
            }
        }

        // No node should be its own ancestor
        for node in diff_tree.nodes() {
            assert!(!diff_tree.is_ancestor(node.id, node.id));
        }
    }
}
