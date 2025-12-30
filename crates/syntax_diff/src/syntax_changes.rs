//! Data types that track the change state for syntax nodes.

use collections::FxHashMap;

use crate::syntax_tree::SyntaxId;

/// The kind of change for a syntax node.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SyntaxChange {
    /// This node is unchanged. The associated ID is the corresponding
    /// node in the opposite tree.
    Unchanged(SyntaxId),
    /// This node was replaced with another node.
    Replaced(SyntaxId, SyntaxId),
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

    pub fn iter(&self) -> collections::hash_map::Iter<'_, SyntaxId, SyntaxChange> {
        self.0.iter()
    }
}
