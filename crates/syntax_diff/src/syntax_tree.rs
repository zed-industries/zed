use std::{
    hash::{DefaultHasher, Hash, Hasher},
    num::NonZeroUsize,
    ops::Range,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SyntaxId(NonZeroUsize);

impl SyntaxId {
    #[inline]
    fn new(idx: usize) -> Self {
        Self(NonZeroUsize::new(idx + 1).expect("index overflow"))
    }

    #[inline]
    pub fn index(self) -> usize {
        self.0.get() - 1
    }
}

pub struct SyntaxTree {
    nodes: Vec<SyntaxNode>,
}

pub struct SyntaxNode {
    pub id: SyntaxId,
    pub structural_hash: u64,
    pub byte_range: Range<usize>,
    /// For list nodes: range of content between delimiters (first_child.start..last_child.end)
    /// Open delimiter = byte_range.start..content_range.start
    /// Close delimiter = content_range.end..byte_range.end
    /// For atoms: equals byte_range
    pub content_range: Range<usize>,
    pub kind_id: u16,
    pub descendant_count: usize,
    pub parent: Option<SyntaxId>,
}

impl SyntaxNode {
    pub fn open_delimiter(&self) -> Range<usize> {
        self.byte_range.start..self.content_range.start
    }

    pub fn close_delimiter(&self) -> Range<usize> {
        self.content_range.end..self.byte_range.end
    }

    pub fn is_leaf(&self) -> bool {
        self.descendant_count == 0
    }
}

impl SyntaxTree {
    pub fn new(cursor: &mut tree_sitter::TreeCursor<'_>) -> Self {
        let mut nodes = Vec::new();

        build_tree(cursor, &mut nodes, None);

        Self { nodes }
    }

    pub fn root(&self) -> Option<SyntaxId> {
        if self.nodes.is_empty() {
            None
        } else {
            Some(SyntaxId::new(0))
        }
    }

    pub fn get(&self, id: SyntaxId) -> &SyntaxNode {
        &self.nodes[id.index()]
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn first_child(&self, id: SyntaxId) -> Option<SyntaxId> {
        let node = self.get(id);
        if node.descendant_count > 0 {
            Some(SyntaxId::new(id.index() + 1))
        } else {
            None
        }
    }

    pub fn next_sibling(&self, id: SyntaxId) -> Option<SyntaxId> {
        let node = self.get(id);
        let next_idx = id.index() + 1 + node.descendant_count;
        if let Some(parent_id) = node.parent {
            let parent = self.get(parent_id);
            let parent_end = parent_id.index() + 1 + parent.descendant_count;
            if next_idx < parent_end {
                return Some(SyntaxId::new(next_idx));
            }
        }
        None
    }

    pub fn parent(&self, id: SyntaxId) -> Option<SyntaxId> {
        self.get(id).parent
    }

    pub fn cursor(&self) -> SyntaxCursor<'_> {
        SyntaxCursor::at_root(self)
    }

    pub fn children(&self, id: SyntaxId) -> SyntaxChildrenIter<'_> {
        SyntaxChildrenIter {
            tree: self,
            current: self.first_child(id),
        }
    }

    pub fn preorder(&self) -> impl Iterator<Item = SyntaxId> + '_ {
        (0..self.nodes.len()).map(SyntaxId::new)
    }
}

pub struct SyntaxChildrenIter<'tree> {
    tree: &'tree SyntaxTree,
    current: Option<SyntaxId>,
}

impl<'tree> Iterator for SyntaxChildrenIter<'tree> {
    type Item = SyntaxId;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current?;
        self.current = self.tree.next_sibling(current);
        Some(current)
    }
}

/// A cursor for navigating a syntax tree during diff traversal.
///
/// The cursor tracks the current position and, when inside a list node,
/// remembers which node was entered so it can properly exit.
#[derive(Clone, Copy)]
pub struct SyntaxCursor<'a> {
    tree: &'a SyntaxTree,
    /// Current node to process, or None if exhausted at this level.
    current: Option<SyntaxId>,
    /// The list node we entered (for proper exit navigation).
    entered: Option<SyntaxId>,
}

impl std::fmt::Debug for SyntaxCursor<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyntaxCursor")
            .field("current", &self.current)
            .field("entered", &self.entered)
            .finish()
    }
}

impl PartialEq for SyntaxCursor<'_> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.tree, other.tree)
            && self.current == other.current
            && self.entered == other.entered
    }
}

impl Eq for SyntaxCursor<'_> {}

impl std::hash::Hash for SyntaxCursor<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.tree, state);
        self.current.hash(state);
        self.entered.hash(state);
    }
}

impl PartialOrd for SyntaxCursor<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SyntaxCursor<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.tree as *const SyntaxTree)
            .cmp(&(other.tree as *const SyntaxTree))
            .then_with(|| self.current.cmp(&other.current))
            .then_with(|| self.entered.cmp(&other.entered))
    }
}

impl<'a> SyntaxCursor<'a> {
    /// Create a cursor at the root of a tree.
    pub fn at_root(tree: &'a SyntaxTree) -> Self {
        Self {
            tree,
            current: tree.root(),
            entered: None,
        }
    }

    /// The current node id, if any.
    pub fn current(&self) -> Option<SyntaxId> {
        self.current
    }

    /// Get the node at the cursor's current position.
    pub fn node(&self) -> Option<&'a SyntaxNode> {
        self.current.map(|id| self.tree.get(id))
    }

    /// Returns true if cursor has no more nodes and no parent to exit to.
    pub fn is_done(&self) -> bool {
        self.current.is_none() && self.entered.is_none()
    }

    /// Returns true if cursor can exit (is inside an entered list).
    pub fn can_exit(&self) -> bool {
        self.entered.is_some()
    }

    /// Enter the current node's children (for list nodes).
    /// Returns a new cursor positioned at the first child.
    pub fn enter(&self) -> Self {
        let current = self.current.expect("cannot enter: no current node");
        Self {
            tree: self.tree,
            current: self.tree.first_child(current),
            entered: Some(current),
        }
    }

    /// Advance to the next sibling.
    /// Returns a new cursor positioned at the next sibling, or None if exhausted.
    pub fn advance(&self) -> Self {
        let current = self.current.expect("cannot advance: no current node");
        Self {
            tree: self.tree,
            current: self.tree.next_sibling(current),
            entered: self.entered,
        }
    }

    /// Exit the current list, moving to the entered node's next sibling.
    /// Returns a new cursor positioned after the list we exited.
    pub fn exit(&self) -> Self {
        let entered = self.entered.expect("cannot exit: not inside a list");
        let grandparent = self.tree.parent(entered);
        Self {
            tree: self.tree,
            current: self.tree.next_sibling(entered),
            entered: grandparent,
        }
    }
}

fn build_tree(
    cursor: &mut tree_sitter::TreeCursor<'_>,
    nodes: &mut Vec<SyntaxNode>,
    parent: Option<SyntaxId>,
) -> SyntaxId {
    let node = cursor.node();
    let this_id = SyntaxId::new(nodes.len());

    nodes.push(SyntaxNode {
        id: this_id,
        structural_hash: 0,
        byte_range: node.byte_range(),
        content_range: node.byte_range(),
        kind_id: node.kind_id(),
        descendant_count: node.descendant_count() - 1,
        parent,
    });

    let mut hasher = DefaultHasher::new();

    node.kind_id().hash(&mut hasher);

    if cursor.goto_first_child() {
        let first_child_start = cursor.node().start_byte();
        let mut last_child_end;

        loop {
            let child_id = build_tree(cursor, nodes, Some(this_id));
            let child_node = &nodes[child_id.index()];
            last_child_end = child_node.byte_range.end;
            child_node.structural_hash.hash(&mut hasher);

            if !cursor.goto_next_sibling() {
                break;
            }
        }

        cursor.goto_parent();
        nodes[this_id.index()].content_range = first_child_start..last_child_end;
    }

    let node = &mut nodes[this_id.index()];
    node.structural_hash = hasher.finish();

    this_id
}
