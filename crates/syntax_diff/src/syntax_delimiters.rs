//! Persistent stack for tracking delimiter entries during diff graph traversal.
//!
//! When computing a syntax diff, we traverse both LHS and RHS trees simultaneously.
//! As we enter delimiters (brackets, parens, braces), we need to track them so we
//! know where to return when exiting. This module provides an efficient persistent
//! stack implementation for this purpose.
//!
//! The key insight is that during Dijkstra's algorithm, many graph vertices share
//! common delimiter history. Instead of cloning the entire stack for each vertex,
//! we store all entries in a shared tree structure where each node points to its
//! parent. A "stack" is then just a pointer to a node in this tree.

use std::cell::RefCell;

use crate::SyntaxId;

/// Represents how we entered a delimiter during diff traversal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxDelimiterEntry {
    /// Entered matching delimiters on both sides together (must exit together).
    Both { lhs: SyntaxId, rhs: SyntaxId },
    /// Entered a delimiter only on the LHS (can exit independently).
    Lhs(SyntaxId),
    /// Entered a delimiter only on the RHS (can exit independently).
    Rhs(SyntaxId),
}

/// Shared storage for delimiter stack nodes.
///
/// Nodes are stored in a flat vector, with each node containing its entry and
/// a parent index. This forms a tree where paths from any node to the root
/// represent complete stacks. Multiple cursors can share prefixes efficiently.
pub struct SyntaxDelimiterTree {
    nodes: RefCell<Vec<(SyntaxDelimiterEntry, Option<usize>)>>,
}

impl SyntaxDelimiterTree {
    pub fn new() -> Self {
        Self {
            nodes: RefCell::new(Vec::new()),
        }
    }

    /// Creates an empty cursor pointing to the root (empty stack).
    pub fn cursor(&self) -> SyntaxDelimiterCursor<'_> {
        SyntaxDelimiterCursor {
            tree: self,
            head: None,
        }
    }
}

/// A cursor into the delimiter tree that behaves like a stack.
///
/// The cursor is `Copy` - it's just a reference to the tree plus an index.
/// Push operations return a new cursor; the original remains valid.
/// This enables structural sharing: branching paths share common prefixes.
#[derive(Clone, Copy)]
pub struct SyntaxDelimiterCursor<'a> {
    tree: &'a SyntaxDelimiterTree,
    head: Option<usize>,
}

impl<'a> SyntaxDelimiterCursor<'a> {
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Pushes a `Both` entry, returning a new cursor. O(1).
    pub fn push_both(&self, lhs: SyntaxId, rhs: SyntaxId) -> Self {
        self.push(SyntaxDelimiterEntry::Both { lhs, rhs })
    }

    /// Pushes an `Lhs` entry, returning a new cursor. O(1).
    pub fn push_lhs(&self, id: SyntaxId) -> Self {
        self.push(SyntaxDelimiterEntry::Lhs(id))
    }

    /// Pushes an `Rhs` entry, returning a new cursor. O(1).
    pub fn push_rhs(&self, id: SyntaxId) -> Self {
        self.push(SyntaxDelimiterEntry::Rhs(id))
    }

    fn push(&self, entry: SyntaxDelimiterEntry) -> Self {
        let mut nodes = self.tree.nodes.borrow_mut();
        let idx = nodes.len();
        nodes.push((entry, self.head));
        Self {
            tree: self.tree,
            head: Some(idx),
        }
    }

    /// Returns the top entry without modifying the cursor.
    pub fn last(&self) -> Option<SyntaxDelimiterEntry> {
        self.head.map(|idx| self.tree.nodes.borrow()[idx].0)
    }

    /// Pops the top entry, returning it and a cursor to the parent. O(1).
    pub fn pop(&self) -> Option<(SyntaxDelimiterEntry, Self)> {
        self.head.map(|idx| {
            let nodes = self.tree.nodes.borrow();
            let (entry, parent) = nodes[idx];
            (
                entry,
                Self {
                    tree: self.tree,
                    head: parent,
                },
            )
        })
    }

    /// Returns true if the top entry is `Lhs` or `Rhs` (can pop independently).
    pub fn can_pop_either(&self) -> bool {
        matches!(
            self.last(),
            Some(SyntaxDelimiterEntry::Lhs(_) | SyntaxDelimiterEntry::Rhs(_))
        )
    }
}

impl PartialEq for SyntaxDelimiterCursor<'_> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.tree, other.tree) && self.head == other.head
    }
}

impl Eq for SyntaxDelimiterCursor<'_> {}

impl std::hash::Hash for SyntaxDelimiterCursor<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.head.hash(state);
    }
}
