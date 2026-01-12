use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::num::NonZeroUsize;
use std::ops::Range;

use language::LanguageScope;

/// A unique identifier for a node within a `SyntaxTree`.
///
/// This is an index into the preorder traversal of the tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SyntaxId(NonZeroUsize);

impl SyntaxId {
    fn new(index: usize) -> Self {
        Self(NonZeroUsize::new(index + 1).expect("index overflow"))
    }

    #[inline]
    pub fn index(self) -> usize {
        self.0.get() - 1
    }
}

/// Classification of special atom nodes for diff purposes.
///
/// Used to influence diff behavior for certain node types:
/// - Comments support replacement detection via Levenshtein similarity
/// - Punctuation is discouraged from matching over meaningful content
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntaxHint<'a> {
    /// A string literal (not yet detected, reserved for future use).
    String,
    /// A comment node. Stores the comment text for Levenshtein similarity
    /// computation when detecting replaced comments.
    Comment(&'a str),
    /// Punctuation tokens (`,`, `;`, `.`). These are discouraged from
    /// matching over more meaningful content in the diff algorithm.
    Punctuation,
}

/// A syntax tree stored as a Vec of nodes in preorder.
#[derive(Debug)]
pub struct SyntaxTree<'a> {
    nodes: Vec<SyntaxNode<'a>>,
}

// TODO: Can this just wrap tree sitter's node?
// TODO: Can we compute the infos without traversing the tree?
/// A single node in a syntax tree.
#[derive(Debug)]
pub struct SyntaxNode<'a> {
    pub id: SyntaxId,
    /// A hash of this node's structure (kind + children's hashes).
    /// Used for quickly detecting structurally identical subtrees.
    pub structural_hash: u64,
    /// The byte range this node spans in the source text.
    pub byte_range: Range<usize>,
    /// The classification of this atom node, if it has special diff behavior.
    ///
    /// `Some` for atoms that need special handling (comments, punctuation).
    /// `None` for regular atoms and list nodes.
    pub hint: Option<SyntaxHint<'a>>,
    /// Opening and closing delimiters for list nodes.
    ///
    /// `[0]` = opening delimiter (e.g., `{`, `(`, `[`)
    /// `[1]` = closing delimiter (e.g., `}`, `)`, `]`)
    ///
    /// Each entry contains the byte range and content of the delimiter.
    /// Delimiter children are excluded from the tree to reduce size.
    pub delimiters: [Option<(Range<usize>, &'a str)>; 2],
    /// Number of descendants (children + their descendants).
    pub descendant_count: usize,
    /// Depth (number of ancestors)
    pub depth: usize,
    /// Parent node, if any.
    parent: Option<SyntaxId>,
}

impl<'a> SyntaxNode<'a> {
    #[inline]
    pub fn open_delimiter(&self) -> Option<&str> {
        self.delimiters[0].as_ref().map(|d| d.1)
    }

    #[inline]
    pub fn close_delimiter(&self) -> Option<&str> {
        self.delimiters[1].as_ref().map(|d| d.1)
    }

    /// Returns the byte range of the opening delimiter (empty for atoms).
    #[inline]
    pub fn open_delimiter_range(&self) -> Option<Range<usize>> {
        self.delimiters[0].as_ref().map(|d| d.0.clone())
    }

    /// Returns the byte range of the closing delimiter (empty for atoms).
    #[inline]
    pub fn close_delimiter_range(&self) -> Option<Range<usize>> {
        self.delimiters[1].as_ref().map(|d| d.0.clone())
    }

    #[inline]
    pub fn has_delimiters(&self) -> bool {
        self.delimiters[0].is_some() && self.delimiters[1].is_some()
    }

    /// Returns true if this is a list node (has children).
    #[inline]
    pub fn is_list(&self) -> bool {
        self.descendant_count > 0
    }

    /// Returns true if this is an atom node (no children).
    #[inline]
    pub fn is_atom(&self) -> bool {
        self.descendant_count == 0
    }
}

impl<'a> SyntaxTree<'a> {
    /// Creates an empty syntax tree.
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Returns the root node's ID, if the tree is not empty.
    pub fn root(&self) -> Option<SyntaxId> {
        if self.nodes.is_empty() {
            None
        } else {
            Some(SyntaxId::new(0))
        }
    }

    /// Returns a reference to the node with the given ID.
    #[inline]
    pub fn get(&self, id: SyntaxId) -> &SyntaxNode<'a> {
        &self.nodes[id.index()]
    }

    /// Returns the number of nodes in the tree.
    #[inline]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns true if the tree has no nodes.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns the first child of the given node, if any.
    pub fn first_child(&self, id: SyntaxId) -> Option<SyntaxId> {
        let node = self.get(id);
        if node.descendant_count > 0 {
            Some(SyntaxId::new(id.index() + 1))
        } else {
            None
        }
    }

    /// Returns the next sibling of the given node, if any.
    pub fn next_sibling(&self, id: SyntaxId) -> Option<SyntaxId> {
        let node = self.get(id);
        let next_index = id.index() + 1 + node.descendant_count;

        let parent_id = node.parent?;
        let parent = self.get(parent_id);
        let parent_end = parent_id.index() + 1 + parent.descendant_count;

        if next_index < parent_end {
            Some(SyntaxId::new(next_index))
        } else {
            None
        }
    }

    /// Returns the parent of the given node, if any.
    #[inline]
    pub fn parent(&self, id: SyntaxId) -> Option<SyntaxId> {
        self.get(id).parent
    }

    /// Returns an iterator over all node IDs in preorder.
    pub fn preorder(&self) -> impl Iterator<Item = SyntaxId> + '_ {
        (0..self.nodes.len()).map(SyntaxId::new)
    }

    pub fn cursor(&self) -> SyntaxTreeCursor<'_> {
        SyntaxTreeCursor {
            tree: self,
            last: None,
            current: self.root(),
        }
    }

    pub fn cursor_at(&self, node: SyntaxId) -> SyntaxTreeCursor<'_> {
        SyntaxTreeCursor {
            tree: self,
            last: None,
            current: Some(node),
        }
    }
}

impl<'a> Default for SyntaxTree<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// A cursor for navigating a syntax tree.
///
/// Cursors are cheap to clone (just a reference and an index) and provide
/// convenient navigation methods. They're designed to be stored in graph
/// vertices for diff computation.
#[derive(Clone, Copy, Debug)]
pub struct SyntaxTreeCursor<'a> {
    tree: &'a SyntaxTree<'a>,
    last: Option<SyntaxId>,
    current: Option<SyntaxId>,
}

impl<'a> SyntaxTreeCursor<'a> {
    /// Returns the current node ID, if any.
    #[inline]
    pub fn id(&self) -> Option<SyntaxId> {
        self.current
    }

    /// Returns a reference to the current node, if any.
    #[inline]
    pub fn node(&self) -> Option<&'a SyntaxNode<'_>> {
        self.current.map(|id| self.tree.get(id))
    }

    /// Returns true if the cursor is at the end (no current node).
    #[inline]
    pub fn is_end(&self) -> bool {
        self.current.is_none()
    }

    /// Returns the underlying tree.
    #[inline]
    pub fn tree(&self) -> &'a SyntaxTree<'_> {
        self.tree
    }

    /// Moves to the first child of the current node.
    /// Returns true if successful.
    pub fn goto_first_child(&mut self) -> bool {
        if let Some(id) = self.current {
            if let Some(child) = self.tree.first_child(id) {
                self.last = Some(id);
                self.current = Some(child);
                return true;
            }
        }
        false
    }

    /// Moves to the next sibling of the current node.
    /// Returns true if successful.
    pub fn goto_next_sibling(&mut self) -> bool {
        if let Some(id) = self.current {
            if let Some(sibling) = self.tree.next_sibling(id) {
                self.last = Some(id);
                self.current = Some(sibling);
                return true;
            }
        }
        false
    }

    /// Moves to the parent of the current node.
    /// Returns true if successful.
    pub fn goto_parent(&mut self) -> bool {
        if let Some(id) = self.current {
            if let Some(parent) = self.tree.parent(id) {
                self.last = Some(id);
                self.current = Some(parent);
                return true;
            }
        }
        false
    }

    pub fn goto_last(&mut self) -> bool {
        if let Some(id) = self.last {
            self.last = self.current;
            self.current = Some(id);
            return true;
        }
        false
    }

    /// Returns a new cursor pointing to the first child.
    #[inline]
    pub fn first_child(&self) -> Self {
        Self {
            tree: self.tree,
            last: self.current.or(self.last),
            current: self.current.and_then(|id| self.tree.first_child(id)),
        }
    }

    /// Returns a new cursor pointing to the next sibling.
    #[inline]
    pub fn next_sibling(&self) -> Self {
        Self {
            tree: self.tree,
            last: self.current.or(self.last),
            current: self.current.and_then(|id| self.tree.next_sibling(id)),
        }
    }

    /// Returns a new cursor pointing to the last valid position.
    #[inline]
    pub fn last(&self) -> Self {
        Self {
            tree: self.tree,
            last: self.current,
            current: self.last,
        }
    }

    /// Returns a new cursor pointing to the parent.
    #[inline]
    pub fn parent(&self) -> Self {
        Self {
            tree: self.tree,
            last: self.current.or(self.last),
            current: self.current.and_then(|id| self.tree.parent(id)),
        }
    }

    /// Returns the depth (number of ancestors) of the current node.
    #[inline]
    pub fn depth(&self) -> usize {
        self.current.map(|id| self.tree.get(id).depth).unwrap_or(0)
    }
}

impl PartialEq for SyntaxTreeCursor<'_> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.tree, other.tree) && self.current == other.current
    }
}

impl Eq for SyntaxTreeCursor<'_> {}

impl Hash for SyntaxTreeCursor<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.current.hash(state);
    }
}

/// Builds a `SyntaxTree` from a tree-sitter parse tree and source text.
///
/// The source text is used to compute structural hashes that include
/// the actual content of leaf nodes, not just their types.
///
/// Node byte ranges are stored as absolute positions in the source text.
pub fn build_tree<'a>(
    mut cursor: tree_sitter::TreeCursor<'_>,
    source: &'a str,
    language_scope: &LanguageScope,
) -> SyntaxTree<'a> {
    let mut nodes = Vec::with_capacity(cursor.node().descendant_count());
    let brackets: HashSet<_> = language_scope
        .brackets()
        .filter(|(pair, _)| pair.surround)
        .flat_map(|(pair, _)| [pair.start.as_str(), pair.end.as_str()])
        .collect();

    if cursor.node().child_count() > 0 || !cursor.node().is_extra() {
        build_tree_recursive(&mut cursor, &mut nodes, None, source, &brackets);
    }

    SyntaxTree { nodes }
}

fn build_tree_recursive<'a>(
    cursor: &mut tree_sitter::TreeCursor<'_>,
    nodes: &mut Vec<SyntaxNode<'a>>,
    parent: Option<SyntaxId>,
    source: &'a str,
    brackets: &HashSet<&str>,
) -> SyntaxId {
    let mut ts_node = cursor.node();
    let this_id = SyntaxId::new(nodes.len());

    // If this node is just a wrapper with same byte range as its only child,
    // flatten by using the child instead
    let flattened = ts_node.child_count() == 1
        && cursor.goto_first_child()
        && ts_node.byte_range() == cursor.node().byte_range();

    if flattened {
        ts_node = cursor.node();
    } else if cursor.node() != ts_node {
        // We moved to child but didn't flatten, go back
        cursor.goto_parent();
    }

    nodes.push(SyntaxNode {
        id: this_id,
        structural_hash: 0,
        byte_range: ts_node.byte_range(),
        delimiters: [None, None],
        hint: None,
        descendant_count: 0,
        depth: parent
            .map(|parent| nodes[parent.index()].depth + 1)
            .unwrap_or(0),
        parent,
    });

    let mut hasher = std::hash::DefaultHasher::new();
    ts_node.kind_id().hash(&mut hasher);

    let mut remaining_children = ts_node.child_count();
    let mut delimiters = [None, None];
    let mut descendant_count = 0;
    let mut hint = None;

    // Detection and extraction of delimiters
    if remaining_children >= 2 {
        if let (Some(first_child), Some(last_child)) = (
            ts_node.child(0),
            ts_node.child((remaining_children - 1) as u32),
        ) {
            if first_child.start_byte() == ts_node.start_byte()
                && last_child.end_byte() == ts_node.end_byte()
            {
                if let Some((open, close)) =
                    detect_delimiters(first_child, last_child, source, brackets)
                {
                    open.hash(&mut hasher);
                    close.hash(&mut hasher);

                    delimiters[0] = Some((first_child.byte_range(), open));
                    delimiters[1] = Some((last_child.byte_range(), close));

                    remaining_children -= 2;
                }
            }
        }
    }

    if cursor.goto_first_child() {
        if delimiters[0].is_some() {
            cursor.goto_next_sibling();
        }

        loop {
            if remaining_children == 0 {
                break;
            }

            let child_id = build_tree_recursive(cursor, nodes, Some(this_id), source, brackets);
            let child_node = &nodes[child_id.index()];

            remaining_children -= 1;
            descendant_count += child_node.descendant_count + 1;
            child_node.structural_hash.hash(&mut hasher);

            if !cursor.goto_next_sibling() {
                break;
            }
        }

        cursor.goto_parent();
    } else {
        // Leaf node - include the actual text content in the hash
        if let Some(source) = source.get(ts_node.byte_range()) {
            source.hash(&mut hasher);

            // Does this node look like punctuation?
            //
            // This check is deliberately conservative, because it's hard to
            // accurately recognise punctuation in a language-agnostic way.
            // https://github.com/Wilfred/difftastic/blob/cba6cc5d5a0b47b36fdb028a87af03c89d1908b4/src/diff/graph.rs#L422
            if source == "," || source == ";" || source == "." {
                hint = Some(SyntaxHint::Punctuation);
            // TODO: we can use info provided by language scope here
            } else if ts_node.is_extra() {
                hint = Some(SyntaxHint::Comment(source));
            }
        }
    }

    // Restore cursor position if we flattened
    if flattened {
        cursor.goto_parent();
    }

    let node = &mut nodes[this_id.index()];
    node.structural_hash = hasher.finish();
    node.delimiters = delimiters;
    node.descendant_count = descendant_count;
    node.hint = hint;

    this_id
}

fn detect_delimiters<'a>(
    first_child: tree_sitter::Node<'_>,
    last_child: tree_sitter::Node<'_>,
    source: &'a str,
    delimiters: &HashSet<&str>,
) -> Option<(&'a str, &'a str)> {
    if first_child.child_count() != 0 || last_child.child_count() != 0 {
        return None;
    }

    let is_delimiter = |delimiter: &str| {
        !delimiter.is_empty() && delimiter.len() <= 2 && delimiters.contains(delimiter)
    };

    let open = source.get(first_child.byte_range())?;
    let close = source.get(last_child.byte_range())?;

    if !is_delimiter(open) || !is_delimiter(close) {
        return None;
    }

    Some((open, close))
}
