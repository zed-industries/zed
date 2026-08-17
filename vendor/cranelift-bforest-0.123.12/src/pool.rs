//! B+-tree node pool.

#[cfg(test)]
use super::Comparator;
use super::{Forest, Node, NodeData};
use crate::entity::PrimaryMap;
#[cfg(test)]
use core::fmt;
use core::ops::{Index, IndexMut};

/// A pool of nodes, including a free list.
pub(super) struct NodePool<F: Forest> {
    nodes: PrimaryMap<Node, NodeData<F>>,
    freelist: Option<Node>,
}

impl<F: Forest> NodePool<F> {
    /// Allocate a new empty pool of nodes.
    pub fn new() -> Self {
        Self {
            nodes: PrimaryMap::new(),
            freelist: None,
        }
    }

    /// Free all nodes.
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.freelist = None;
    }

    /// Allocate a new node containing `data`.
    pub fn alloc_node(&mut self, data: NodeData<F>) -> Node {
        debug_assert!(!data.is_free(), "can't allocate free node");
        match self.freelist {
            Some(node) => {
                // Remove this node from the free list.
                match self.nodes[node] {
                    NodeData::Free { next } => self.freelist = next,
                    _ => panic!("Invalid {} on free list", node),
                }
                self.nodes[node] = data;
                node
            }
            None => {
                // The free list is empty. Allocate a new node.
                self.nodes.push(data)
            }
        }
    }

    /// Free a node.
    pub fn free_node(&mut self, node: Node) {
        // Quick check for a double free.
        debug_assert!(!self.nodes[node].is_free(), "{node} is already free");
        self.nodes[node] = NodeData::Free {
            next: self.freelist,
        };
        self.freelist = Some(node);
    }

    /// Free the entire tree rooted at `node`.
    pub fn free_tree(&mut self, node: Node) {
        if let NodeData::Inner { size, tree, .. } = self[node] {
            // Note that we have to capture `tree` by value to avoid borrow checker trouble.
            for i in 0..usize::from(size + 1) {
                // Recursively free sub-trees. This recursion can never be deeper than `MAX_PATH`,
                // and since most trees have less than a handful of nodes, it is worthwhile to
                // avoid the heap allocation for an iterative tree traversal.
                self.free_tree(tree[i]);
            }
        }
        self.free_node(node);
    }
}

#[cfg(test)]
impl<F: Forest> NodePool<F> {
    /// Verify the consistency of the tree rooted at `node`.
    pub fn verify_tree<C: Comparator<F::Key>>(&self, node: Node, comp: &C)
    where
        NodeData<F>: fmt::Display,
        F::Key: fmt::Display,
    {
        use crate::entity::EntitySet;
        use alloc::vec::Vec;
        use core::borrow::Borrow;
        use core::cmp::Ordering;

        // The root node can't be an inner node with just a single sub-tree. It should have been
        // pruned.
        if let NodeData::Inner { size, .. } = self[node] {
            assert!(size > 0, "Root must have more than one sub-tree");
        }

        let mut done = match self[node] {
            NodeData::Inner { size, .. } | NodeData::Leaf { size, .. } => {
                EntitySet::with_capacity(size.into())
            }
            _ => EntitySet::new(),
        };

        let mut todo = Vec::new();

        // Todo-list entries are:
        // 1. Optional LHS key which must be <= all node entries.
        // 2. The node reference.
        // 3. Optional RHS key which must be > all node entries.
        todo.push((None, node, None));

        while let Some((lkey, node, rkey)) = todo.pop() {
            assert!(done.insert(node), "Node appears more than once in tree");
            let mut lower = lkey;

            match self[node] {
                NodeData::Inner { size, keys, tree } => {
                    let size = size as usize;
                    let capacity = tree.len();
                    let keys = &keys[0..size];

                    // Verify occupancy.
                    // Right-most nodes can be small, but others must be at least half full.
                    assert!(
                        rkey.is_none() || (size + 1) * 2 >= capacity,
                        "Only {}/{} entries in {}:{}, upper={}",
                        size + 1,
                        capacity,
                        node,
                        self[node],
                        rkey.unwrap()
                    );

                    // Queue up the sub-trees, checking for duplicates.
                    for i in 0..size + 1 {
                        // Get an upper bound for node[i].
                        let upper = keys.get(i).cloned().or(rkey);

                        // Check that keys are strictly monotonic.
                        if let (Some(a), Some(b)) = (lower, upper) {
                            assert_eq!(
                                comp.cmp(a, b),
                                Ordering::Less,
                                "Key order {} < {} failed in {}: {}",
                                a,
                                b,
                                node,
                                self[node]
                            );
                        }

                        // Queue up the sub-tree.
                        todo.push((lower, tree[i], upper));

                        // Set a lower bound for the next tree.
                        lower = upper;
                    }
                }
                NodeData::Leaf { size, keys, .. } => {
                    let size = size as usize;
                    let capacity = keys.borrow().len();
                    let keys = &keys.borrow()[0..size];

                    // Verify occupancy.
                    // Right-most nodes can be small, but others must be at least half full.
                    assert!(size > 0, "Leaf {node} is empty");
                    assert!(
                        rkey.is_none() || size * 2 >= capacity,
                        "Only {}/{} entries in {}:{}, upper={}",
                        size,
                        capacity,
                        node,
                        self[node],
                        rkey.unwrap()
                    );

                    for i in 0..size + 1 {
                        let upper = keys.get(i).cloned().or(rkey);

                        // Check that keys are strictly monotonic.
                        if let (Some(a), Some(b)) = (lower, upper) {
                            let wanted = if i == 0 {
                                Ordering::Equal
                            } else {
                                Ordering::Less
                            };
                            assert_eq!(
                                comp.cmp(a, b),
                                wanted,
                                "Key order for {} - {} failed in {}: {}",
                                a,
                                b,
                                node,
                                self[node]
                            );
                        }

                        // Set a lower bound for the next key.
                        lower = upper;
                    }
                }
                NodeData::Free { .. } => panic!("Free {} reached", node),
            }
        }
    }
}

impl<F: Forest> Index<Node> for NodePool<F> {
    type Output = NodeData<F>;

    fn index(&self, index: Node) -> &Self::Output {
        self.nodes.index(index)
    }
}

impl<F: Forest> IndexMut<Node> for NodePool<F> {
    fn index_mut(&mut self, index: Node) -> &mut Self::Output {
        self.nodes.index_mut(index)
    }
}
