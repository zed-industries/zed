//! Branch-preserving undo tree data structure.
//!
//! Tracks the full tree of committed transactions, preserving branches that
//! would otherwise be lost when the redo stack is cleared after a new edit.
//!
//! # Data model
//!
//! ```text
//!   (root)              Implicit initial state (current = None)
//!     │
//!     ├── [0] A         Nodes stored in a flat Vec, indices = creation order.
//!     │   ├── [1] B     Each node has: parent index, children vec, HistoryEntry.
//!     │   │   └── [2] C
//!     │   └── [3] D     Branching: undo to A, then edit → D becomes child of A.
//!     │                 Children ordered newest-first, so D is A's primary child.
//!     └── [4] E         Multiple root children are possible too.
//!
//!   current ──► [3] D   Pointer to the "you are here" node (or None for root).
//!   index: {A→0, B→1, C→2, D→3, E→4}   Fast TransactionId → node index lookup.
//! ```
//!
//! **Navigation:** `undo()` moves current to parent, `redo_primary()` to first
//! child, `goto()` jumps to any node. Chronological g-/g+ scans the flat Vec
//! by index since creation order = chronological order.
//!
//! **Liveness:** Merged/removed nodes stay in the Vec (indices are stable) but
//! are removed from `index`, so `is_live()` filters them out.

use crate::{HistoryEntry, TransactionId};
use collections::HashMap;

/// A node in the undo tree.
#[derive(Clone, Debug)]
pub struct UndoTreeNode {
    /// The full history entry (transaction + timestamps).
    pub entry: HistoryEntry,
    /// Index of the parent node, or `None` if this is a child of the root state.
    pub parent: Option<usize>,
    /// Indices of child nodes, ordered newest-first.
    pub children: Vec<usize>,
}

/// Tracks the full tree structure of undo history.
///
/// Nodes are stored in a flat Vec in chronological creation order, so the
/// index doubles as the sequence number. This makes g-/g+ trivial.
#[derive(Clone, Debug)]
pub struct UndoTree {
    nodes: Vec<UndoTreeNode>,
    index: HashMap<TransactionId, usize>,
    root_children: Vec<usize>,
    current: Option<usize>,
}

impl UndoTree {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            index: HashMap::default(),
            root_children: Vec::new(),
            current: None,
        }
    }

    /// Record a committed transaction as a child of the current position.
    pub fn push(&mut self, entry: HistoryEntry) {
        let id = entry.transaction_id();
        let node_index = self.nodes.len();
        let parent = self.current;

        if let Some(parent_idx) = parent {
            self.nodes[parent_idx].children.insert(0, node_index);
        } else {
            self.root_children.insert(0, node_index);
        }

        self.nodes.push(UndoTreeNode {
            entry,
            parent,
            children: Vec::new(),
        });
        self.index.insert(id, node_index);
        self.current = Some(node_index);
    }

    /// Merge the current node into its parent (for transaction grouping).
    pub fn merge_current_into_parent(&mut self) -> bool {
        let Some(current_idx) = self.current else { return false };
        let Some(parent_idx) = self.nodes[current_idx].parent else { return false };

        let edit_ids = self.nodes[current_idx].entry.transaction().edit_ids.clone();
        let orphaned_children: Vec<usize> = self.nodes[current_idx].children.clone();

        for &child_idx in &orphaned_children {
            self.nodes[child_idx].parent = Some(parent_idx);
            self.nodes[parent_idx].children.insert(0, child_idx);
        }

        self.nodes[parent_idx].children.retain(|&idx| idx != current_idx);

        let parent_entry = &mut self.nodes[parent_idx].entry;
        for eid in edit_ids {
            parent_entry.transaction_mut().edit_ids.push(eid);
        }

        let current_last = self.nodes[current_idx].entry.last_edit_at();
        self.nodes[parent_idx].entry.set_last_edit_at(current_last);

        let removed_id = self.nodes[current_idx].entry.transaction_id();
        self.index.remove(&removed_id);

        self.current = Some(parent_idx);
        true
    }

    /// Move current to parent (undo direction).
    pub fn undo(&mut self) -> Option<usize> {
        let current_idx = self.current?;
        let parent = self.nodes[current_idx].parent;
        self.current = parent;
        parent
    }

    /// Move current to a specific child by transaction ID.
    pub fn redo_to(&mut self, transaction_id: TransactionId) -> bool {
        if let Some(&idx) = self.index.get(&transaction_id) {
            let is_child = if let Some(current_idx) = self.current {
                self.nodes[current_idx].children.contains(&idx)
            } else {
                self.root_children.contains(&idx)
            };
            if is_child {
                self.current = Some(idx);
                return true;
            }
        }
        false
    }

    /// Move current to the first (primary) child.
    pub fn redo_primary(&mut self) -> Option<usize> {
        let children = if let Some(current_idx) = self.current {
            &self.nodes[current_idx].children
        } else {
            &self.root_children
        };
        if let Some(&first) = children.iter().find(|&&idx| self.is_live(idx)) {
            self.current = Some(first);
            Some(first)
        } else {
            None
        }
    }

    /// Set the current pointer to a specific node index.
    pub fn goto(&mut self, node_index: usize) -> bool {
        if node_index < self.nodes.len() && self.is_live(node_index) {
            self.current = Some(node_index);
            true
        } else {
            false
        }
    }

    /// Set current to root (initial state).
    pub fn goto_root(&mut self) {
        self.current = None;
    }

    // ── Queries ───────────────────────────────────────────────────────

    pub fn current(&self) -> Option<usize> { self.current }

    pub fn current_transaction_id(&self) -> Option<TransactionId> {
        self.current.map(|idx| self.nodes[idx].entry.transaction_id())
    }

    pub fn len(&self) -> usize { self.nodes.len() }

    pub fn live_count(&self) -> usize { self.index.len() }

    pub fn is_empty(&self) -> bool { self.index.is_empty() }

    pub fn node(&self, index: usize) -> Option<&UndoTreeNode> {
        self.nodes.get(index).filter(|_| self.is_live(index))
    }

    pub fn index_for_transaction(&self, id: TransactionId) -> Option<usize> {
        self.index.get(&id).copied()
    }

    pub fn all_nodes(&self) -> &[UndoTreeNode] { &self.nodes }

    pub fn root_children(&self) -> &[usize] { &self.root_children }

    pub fn is_live(&self, index: usize) -> bool {
        if index >= self.nodes.len() { return false; }
        let id = self.nodes[index].entry.transaction_id();
        self.index.get(&id) == Some(&index)
    }

    // ── Path computation ──────────────────────────────────────────────

    pub fn path_to(&self, target: usize) -> Vec<usize> {
        if target >= self.nodes.len() || !self.is_live(target) {
            return Vec::new();
        }
        let mut path = Vec::new();
        let mut current = Some(target);
        while let Some(idx) = current {
            path.push(idx);
            current = self.nodes[idx].parent;
        }
        path.reverse();
        path
    }

    pub fn active_path(&self) -> Vec<usize> {
        match self.current {
            Some(idx) => self.path_to(idx),
            None => Vec::new(),
        }
    }

    pub fn transaction_ids_on_path(&self, target: usize) -> Vec<TransactionId> {
        self.path_to(target)
            .into_iter()
            .map(|idx| self.nodes[idx].entry.transaction_id())
            .collect()
    }

    // ── Chronological navigation (g-/g+) ─────────────────────────────

    pub fn chrono_prev_index(&self) -> Option<usize> {
        let current_idx = self.current?;
        let mut idx = current_idx;
        while idx > 0 {
            idx -= 1;
            if self.is_live(idx) { return Some(idx); }
        }
        None
    }

    pub fn chrono_next_index(&self) -> Option<usize> {
        let start = match self.current {
            Some(idx) => idx + 1,
            None => 0,
        };
        for idx in start..self.nodes.len() {
            if self.is_live(idx) { return Some(idx); }
        }
        None
    }

    pub fn chrono_position(&self) -> (usize, usize) {
        let total = self.live_count();
        let position = match self.current {
            Some(idx) => (0..=idx).filter(|&i| self.is_live(i)).count(),
            None => 0,
        };
        (position, total)
    }

    // ── Stack rebuilding ──────────────────────────────────────────────

    pub fn entries_on_path(&self, target: usize) -> Vec<HistoryEntry> {
        self.path_to(target)
            .into_iter()
            .map(|idx| self.nodes[idx].entry.clone())
            .collect()
    }

    pub fn primary_branch_forward(&self, from: usize) -> Vec<HistoryEntry> {
        let mut entries = Vec::new();
        let mut current = from;
        loop {
            let live_child = self.nodes[current].children.iter()
                .find(|&&idx| self.is_live(idx))
                .copied();
            match live_child {
                Some(child_idx) => {
                    entries.push(self.nodes[child_idx].entry.clone());
                    current = child_idx;
                }
                None => break,
            }
        }
        entries.reverse();
        entries
    }

    pub fn primary_branch_forward_from_root(&self) -> Vec<HistoryEntry> {
        let first = match self.root_children.iter().find(|&&idx| self.is_live(idx)) {
            Some(&idx) => idx,
            None => return Vec::new(),
        };
        let mut entries = vec![self.nodes[first].entry.clone()];
        let mut current = first;
        loop {
            let live_child = self.nodes[current].children.iter()
                .find(|&&idx| self.is_live(idx))
                .copied();
            match live_child {
                Some(child_idx) => {
                    entries.push(self.nodes[child_idx].entry.clone());
                    current = child_idx;
                }
                None => break,
            }
        }
        entries.reverse();
        entries
    }

    /// Remove a node from the tree, re-parenting its children to the node's parent.
    pub fn remove_node(&mut self, index: usize) -> bool {
        if index >= self.nodes.len() || !self.is_live(index) {
            return false;
        }

        let parent = self.nodes[index].parent;
        let children: Vec<usize> = self.nodes[index].children.clone();

        for &child_idx in &children {
            self.nodes[child_idx].parent = parent;
            if let Some(parent_idx) = parent {
                self.nodes[parent_idx].children.push(child_idx);
            } else {
                self.root_children.push(child_idx);
            }
        }

        if let Some(parent_idx) = parent {
            self.nodes[parent_idx].children.retain(|&idx| idx != index);
        } else {
            self.root_children.retain(|&idx| idx != index);
        }

        let id = self.nodes[index].entry.transaction_id();
        self.index.remove(&id);

        if self.current == Some(index) {
            self.current = parent;
        }

        true
    }

    /// Mutable access to a live node.
    pub fn node_mut(&mut self, index: usize) -> Option<&mut UndoTreeNode> {
        if index < self.nodes.len() && self.is_live(index) {
            Some(&mut self.nodes[index])
        } else {
            None
        }
    }

    /// Returns transaction IDs of all live nodes in chronological (creation) order.
    pub fn all_live_transaction_ids(&self) -> Vec<TransactionId> {
        self.nodes
            .iter()
            .enumerate()
            .filter(|(idx, _)| self.is_live(*idx))
            .map(|(_, node)| node.entry.transaction_id())
            .collect()
    }
}

impl Default for UndoTree {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Transaction;
    use clock;
    use std::time::Instant;

    fn make_entry(id_value: u32, replica_id: u16) -> HistoryEntry {
        let id = clock::Lamport {
            replica_id: clock::ReplicaId::new(replica_id),
            value: id_value,
        };
        let now = Instant::now();
        HistoryEntry::new(
            Transaction {
                id,
                edit_ids: vec![id],
                start: clock::Global::new(),
            },
            now,
        )
    }

    #[test]
    fn test_push_and_basic_structure() {
        let mut tree = UndoTree::new();
        tree.push(make_entry(1, 0));
        assert_eq!(tree.len(), 1);
        assert_eq!(tree.current(), Some(0));
        assert_eq!(tree.root_children(), &[0]);

        tree.push(make_entry(2, 0));
        assert_eq!(tree.len(), 2);
        assert_eq!(tree.current(), Some(1));
        assert_eq!(tree.nodes[0].children, vec![1]);

        tree.push(make_entry(3, 0));
        assert_eq!(tree.len(), 3);
        assert_eq!(tree.current(), Some(2));
    }

    #[test]
    fn test_undo_redo() {
        let mut tree = UndoTree::new();
        tree.push(make_entry(1, 0));
        tree.push(make_entry(2, 0));
        tree.push(make_entry(3, 0));

        assert_eq!(tree.undo(), Some(1));
        assert_eq!(tree.undo(), Some(0));
        assert_eq!(tree.undo(), None);
        assert_eq!(tree.current(), None);
        assert_eq!(tree.undo(), None);
        assert_eq!(tree.redo_primary(), Some(0));
    }

    #[test]
    fn test_branching() {
        let mut tree = UndoTree::new();
        tree.push(make_entry(1, 0)); // A=0
        tree.push(make_entry(2, 0)); // B=1
        tree.push(make_entry(3, 0)); // C=2

        tree.undo(); // at B
        tree.undo(); // at A
        tree.push(make_entry(4, 0)); // D=3

        assert_eq!(tree.nodes[0].children, vec![3, 1]);
        tree.undo(); // back to A
        assert_eq!(tree.redo_primary(), Some(3));
    }

    #[test]
    fn test_chrono_navigation() {
        let mut tree = UndoTree::new();
        tree.push(make_entry(1, 0)); // A=0
        tree.push(make_entry(2, 0)); // B=1
        tree.undo(); // at A
        tree.push(make_entry(3, 0)); // D=2

        assert_eq!(tree.chrono_prev_index(), Some(1)); // B
        assert_eq!(tree.chrono_next_index(), None);

        tree.goto(1);
        assert_eq!(tree.chrono_prev_index(), Some(0));
        assert_eq!(tree.chrono_next_index(), Some(2));

        tree.goto_root();
        assert_eq!(tree.chrono_prev_index(), None);
        assert_eq!(tree.chrono_next_index(), Some(0));
    }

    #[test]
    fn test_path_to() {
        let mut tree = UndoTree::new();
        tree.push(make_entry(1, 0));
        tree.push(make_entry(2, 0));
        tree.push(make_entry(3, 0));
        assert_eq!(tree.path_to(2), vec![0, 1, 2]);
        assert_eq!(tree.path_to(0), vec![0]);
        assert_eq!(tree.active_path(), vec![0, 1, 2]);
    }

    #[test]
    fn test_chrono_position() {
        let mut tree = UndoTree::new();
        tree.push(make_entry(1, 0));
        tree.push(make_entry(2, 0));
        tree.push(make_entry(3, 0));
        assert_eq!(tree.chrono_position(), (3, 3));
        tree.undo();
        assert_eq!(tree.chrono_position(), (2, 3));
        tree.goto_root();
        assert_eq!(tree.chrono_position(), (0, 3));
    }

    #[test]
    fn test_remove_node_leaf() {
        let mut tree = UndoTree::new();
        tree.push(make_entry(1, 0)); // A=0
        tree.push(make_entry(2, 0)); // B=1
        tree.push(make_entry(3, 0)); // C=2

        // Remove leaf C
        assert!(tree.remove_node(2));
        assert_eq!(tree.live_count(), 2);
        assert!(!tree.is_live(2));
        // Current should move to parent B
        assert_eq!(tree.current(), Some(1));
        // B should have no children
        assert!(tree.nodes[1].children.is_empty());
    }

    #[test]
    fn test_remove_node_middle() {
        let mut tree = UndoTree::new();
        tree.push(make_entry(1, 0)); // A=0
        tree.push(make_entry(2, 0)); // B=1
        tree.push(make_entry(3, 0)); // C=2

        // Remove middle node B (not current)
        tree.goto(2); // stay at C
        assert!(tree.remove_node(1));
        assert_eq!(tree.live_count(), 2);
        // C should now be re-parented to A
        assert_eq!(tree.nodes[2].parent, Some(0));
        assert!(tree.nodes[0].children.contains(&2));
        // Current stays at C
        assert_eq!(tree.current(), Some(2));
    }

    #[test]
    fn test_remove_node_root_child() {
        let mut tree = UndoTree::new();
        tree.push(make_entry(1, 0)); // A=0
        tree.push(make_entry(2, 0)); // B=1

        // Remove A (root child), B should become root child
        tree.goto(1);
        assert!(tree.remove_node(0));
        assert!(tree.root_children().contains(&1));
        assert_eq!(tree.nodes[1].parent, None);
    }

    #[test]
    fn test_all_live_transaction_ids() {
        let mut tree = UndoTree::new();
        let e1 = make_entry(1, 0);
        let e2 = make_entry(2, 0);
        let e3 = make_entry(3, 0);
        let id1 = e1.transaction_id();
        let id2 = e2.transaction_id();
        let id3 = e3.transaction_id();
        tree.push(e1);
        tree.push(e2);
        tree.push(e3);

        let ids = tree.all_live_transaction_ids();
        assert_eq!(ids, vec![id1, id2, id3]);

        // Remove middle node
        tree.remove_node(1);
        let ids = tree.all_live_transaction_ids();
        assert_eq!(ids, vec![id1, id3]);
    }
}