//! Data types that track the change state for syntax nodes.

use collections::FxHashMap;

use crate::syntax_tree::{SyntaxId, SyntaxTree};

/// The kind of change for a syntax node.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SyntaxChange {
    /// This node is unchanged. The associated ID is the corresponding
    /// node in the opposite tree.
    Unchanged(SyntaxId),
    /// This comment was replaced with a similar comment.
    ReplacedComment(SyntaxId, SyntaxId),
    /// This string was replaced with a similar string.
    ReplacedString(SyntaxId, SyntaxId),
    /// This node is novel (added or removed).
    Novel,
}

/// A map from syntax node IDs to their change status.
#[derive(Default)]
pub struct SyntaxChanges(FxHashMap<SyntaxId, SyntaxChange>);

impl SyntaxChanges {
    pub fn insert(&mut self, id: SyntaxId, kind: SyntaxChange) {
        self.0.insert(id, kind);
    }

    pub fn get(&self, id: SyntaxId) -> Option<SyntaxChange> {
        self.0.get(&id).copied()
    }

    pub fn contains(&self, id: SyntaxId) -> bool {
        self.0.contains_key(&id)
    }
}

/// Mark a node and all its descendants as unchanged.
pub fn insert_deep_unchanged(
    tree: &SyntaxTree,
    node_id: SyntaxId,
    opposite_tree: &SyntaxTree,
    opposite_id: SyntaxId,
    change_map: &mut SyntaxChanges,
) {
    change_map.insert(node_id, SyntaxChange::Unchanged(opposite_id));

    let node = tree.get(node_id);
    let opposite = opposite_tree.get(opposite_id);

    if node.is_list() && opposite.is_list() {
        let children: Vec<_> = tree.children(node_id).collect();
        let opposite_children: Vec<_> = opposite_tree.children(opposite_id).collect();

        for (child_id, opposite_child_id) in children.into_iter().zip(opposite_children) {
            insert_deep_unchanged(tree, child_id, opposite_tree, opposite_child_id, change_map);
        }
    }
}

/// Mark a node and all its descendants as novel.
pub fn insert_deep_novel(tree: &SyntaxTree, node_id: SyntaxId, change_map: &mut SyntaxChanges) {
    change_map.insert(node_id, SyntaxChange::Novel);

    for child_id in tree.children(node_id) {
        insert_deep_novel(tree, child_id, change_map);
    }
}
