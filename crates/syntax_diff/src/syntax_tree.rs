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

    pub fn children(&self, id: SyntaxId) -> ChildrenIter<'_> {
        ChildrenIter {
            tree: self,
            current: self.first_child(id),
        }
    }

    pub fn preorder(&self) -> impl Iterator<Item = SyntaxId> + '_ {
        (0..self.nodes.len()).map(SyntaxId::new)
    }
}

pub struct ChildrenIter<'tree> {
    tree: &'tree SyntaxTree,
    current: Option<SyntaxId>,
}

impl<'tree> Iterator for ChildrenIter<'tree> {
    type Item = SyntaxId;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current?;
        self.current = self.tree.next_sibling(current);
        Some(current)
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
