use std::hash::{Hash, Hasher};
use std::num::NonZeroUsize;
use std::ops::Range;

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

/// The kind of an atom node for syntax highlighting purposes.
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum AtomKind {
    /// A normal token like a variable name or numeric literal.
    Normal,
    /// A string literal.
    String,
    /// A type name.
    Type,
    /// A comment.
    Comment,
    /// A language keyword.
    Keyword,
}

/// A syntax tree stored as a Vec of nodes in preorder.
///
/// This representation allows efficient tree traversal while avoiding
/// arena allocation and pointer-based node references.
pub struct SyntaxTree {
    nodes: Vec<SyntaxNode>,
}

/// A single node in a syntax tree.
pub struct SyntaxNode {
    id: SyntaxId,
    /// A hash of this node's structure (kind + children's hashes).
    /// Used for quickly detecting structurally identical subtrees.
    structural_hash: u64,
    /// The byte range this node spans in the source text.
    byte_range: Range<usize>,
    /// For list nodes: the range of content between delimiters.
    /// For atoms: equals byte_range.
    ///
    /// Open delimiter = byte_range.start..content_range.start
    /// Close delimiter = content_range.end..byte_range.end
    content_range: Range<usize>,
    /// Number of descendants (children + their descendants).
    descendant_count: usize,
    /// Parent node, if any.
    parent: Option<SyntaxId>,
    /// For atoms, the kind of atom. None for list nodes.
    kind: Option<AtomKind>,
}

impl SyntaxNode {
    #[inline]
    pub fn id(&self) -> SyntaxId {
        self.id
    }

    #[inline]
    pub fn structural_hash(&self) -> u64 {
        self.structural_hash
    }

    #[inline]
    pub fn byte_range(&self) -> Range<usize> {
        self.byte_range.clone()
    }

    #[inline]
    pub fn content_range(&self) -> Range<usize> {
        self.content_range.clone()
    }

    /// Returns the byte range of the opening delimiter (empty for atoms).
    #[inline]
    pub fn open_delimiter(&self) -> Range<usize> {
        self.byte_range.start..self.content_range.start
    }

    /// Returns the byte range of the closing delimiter (empty for atoms).
    #[inline]
    pub fn close_delimiter(&self) -> Range<usize> {
        self.content_range.end..self.byte_range.end
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

    /// Returns the atom kind, if this is an atom node.
    #[inline]
    pub fn atom_kind(&self) -> Option<AtomKind> {
        self.kind
    }

    /// Returns the number of descendants (not including self).
    #[inline]
    pub fn descendant_count(&self) -> usize {
        self.descendant_count
    }
}

impl SyntaxTree {
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
    pub fn get(&self, id: SyntaxId) -> &SyntaxNode {
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

    /// Returns an iterator over the children of the given node.
    pub fn children(&self, id: SyntaxId) -> ChildrenIter<'_> {
        ChildrenIter {
            tree: self,
            current: self.first_child(id),
        }
    }

    /// Returns an iterator over the ancestors of the given node (parent, grandparent, etc.).
    pub fn ancestors(&self, id: SyntaxId) -> AncestorsIter<'_> {
        AncestorsIter {
            tree: self,
            current: self.parent(id),
        }
    }

    /// Returns an iterator over all descendants of the given node in preorder.
    pub fn descendants(&self, id: SyntaxId) -> DescendantsIter<'_> {
        let node = self.get(id);
        let start = id.index() + 1;
        let end = start + node.descendant_count;
        DescendantsIter {
            current: start,
            end,
        }
    }
}

impl Default for SyntaxTree {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator over the children of a node.
pub struct ChildrenIter<'a> {
    tree: &'a SyntaxTree,
    current: Option<SyntaxId>,
}

impl Iterator for ChildrenIter<'_> {
    type Item = SyntaxId;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.current?;
        self.current = self.tree.next_sibling(id);
        Some(id)
    }
}

/// Iterator over the ancestors of a node.
pub struct AncestorsIter<'a> {
    tree: &'a SyntaxTree,
    current: Option<SyntaxId>,
}

impl Iterator for AncestorsIter<'_> {
    type Item = SyntaxId;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.current?;
        self.current = self.tree.parent(id);
        Some(id)
    }
}

/// Iterator over descendants in preorder.
pub struct DescendantsIter {
    current: usize,
    end: usize,
}

impl Iterator for DescendantsIter {
    type Item = SyntaxId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let id = SyntaxId::new(self.current);
            self.current += 1;
            Some(id)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.end - self.current;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for DescendantsIter {}

/// Builds a `SyntaxTree` from a tree-sitter parse tree.
pub fn build_tree(tree: &tree_sitter::Tree) -> SyntaxTree {
    let mut nodes = Vec::new();
    let mut cursor = tree.walk();

    if cursor.node().child_count() > 0 || !cursor.node().is_extra() {
        build_tree_recursive(&mut cursor, &mut nodes, None);
    }

    SyntaxTree { nodes }
}

fn build_tree_recursive(
    cursor: &mut tree_sitter::TreeCursor<'_>,
    nodes: &mut Vec<SyntaxNode>,
    parent: Option<SyntaxId>,
) -> SyntaxId {
    let ts_node = cursor.node();
    let this_id = SyntaxId::new(nodes.len());

    nodes.push(SyntaxNode {
        id: this_id,
        structural_hash: 0,
        byte_range: ts_node.byte_range(),
        content_range: ts_node.byte_range(),
        kind: None,
        descendant_count: 0,
        parent,
    });

    let mut hasher = std::hash::DefaultHasher::new();
    ts_node.kind_id().hash(&mut hasher);

    let mut descendant_count = 0;
    let mut first_child_start = None;
    let mut last_child_end = ts_node.end_byte();

    if cursor.goto_first_child() {
        first_child_start = Some(cursor.node().start_byte());

        loop {
            let child_id = build_tree_recursive(cursor, nodes, Some(this_id));
            let child_node = &nodes[child_id.index()];

            last_child_end = child_node.byte_range.end;
            child_node.structural_hash.hash(&mut hasher);
            descendant_count += 1 + child_node.descendant_count;

            if !cursor.goto_next_sibling() {
                break;
            }
        }

        cursor.goto_parent();
    }

    let node = &mut nodes[this_id.index()];
    node.structural_hash = hasher.finish();
    node.descendant_count = descendant_count;

    if let Some(first_start) = first_child_start {
        node.content_range = first_start..last_child_end;
    }

    this_id
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_json(source: &str) -> tree_sitter::Tree {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_json::LANGUAGE.into())
            .expect("Error loading JSON grammar");
        parser.parse(source, None).expect("Failed to parse")
    }

    #[test]
    fn test_empty_tree() {
        let tree = SyntaxTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
        assert!(tree.root().is_none());
    }

    #[test]
    fn test_build_simple_tree() {
        let source = r#"{"key": "value"}"#;
        let ts_tree = parse_json(source);
        let tree = build_tree(&ts_tree);

        assert!(!tree.is_empty());
        assert!(tree.root().is_some());

        let root = tree.root().unwrap();
        let root_node = tree.get(root);
        assert_eq!(root_node.byte_range(), 0..source.len());
    }

    #[test]
    fn test_preorder_traversal() {
        let source = r#"[1, 2]"#;
        let ts_tree = parse_json(source);
        let tree = build_tree(&ts_tree);

        let ids: Vec<_> = tree.preorder().collect();
        assert_eq!(ids.len(), tree.len());

        for (i, id) in ids.iter().enumerate() {
            assert_eq!(id.index(), i);
        }
    }

    #[test]
    fn test_children_iterator() {
        let source = r#"[1, 2, 3]"#;
        let ts_tree = parse_json(source);
        let tree = build_tree(&ts_tree);

        let root = tree.root().unwrap();
        let children: Vec<_> = tree.children(root).collect();

        assert!(!children.is_empty());
    }

    #[test]
    fn test_ancestors_iterator() {
        let source = r#"{"nested": {"deep": 1}}"#;
        let ts_tree = parse_json(source);
        let tree = build_tree(&ts_tree);

        let last_id = SyntaxId::new(tree.len() - 1);
        let ancestors: Vec<_> = tree.ancestors(last_id).collect();

        assert!(!ancestors.is_empty());

        let root = tree.root().unwrap();
        assert!(ancestors.contains(&root) || last_id == root);
    }

    #[test]
    fn test_descendants_iterator() {
        let source = r#"[1, 2]"#;
        let ts_tree = parse_json(source);
        let tree = build_tree(&ts_tree);

        let root = tree.root().unwrap();
        let root_node = tree.get(root);

        let descendants: Vec<_> = tree.descendants(root).collect();
        assert_eq!(descendants.len(), root_node.descendant_count());
    }

    #[test]
    fn test_structural_hash_differs_for_different_content() {
        let source1 = r#"{"a": 1}"#;
        let source2 = r#"{"b": 2}"#;

        let tree1 = build_tree(&parse_json(source1));
        let tree2 = build_tree(&parse_json(source2));

        let root1 = tree1.get(tree1.root().unwrap());
        let root2 = tree2.get(tree2.root().unwrap());

        assert_ne!(root1.structural_hash(), root2.structural_hash());
    }

    #[test]
    fn test_structural_hash_same_for_same_structure() {
        let source = r#"[1, 2]"#;

        let tree1 = build_tree(&parse_json(source));
        let tree2 = build_tree(&parse_json(source));

        let root1 = tree1.get(tree1.root().unwrap());
        let root2 = tree2.get(tree2.root().unwrap());

        assert_eq!(root1.structural_hash(), root2.structural_hash());
    }
}
