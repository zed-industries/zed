//! B+-tree nodes.

use super::{Forest, INNER_SIZE, Node, SetValue, slice_insert, slice_shift};
use core::borrow::{Borrow, BorrowMut};
use core::fmt;

/// B+-tree node.
///
/// A B+-tree has different node types for inner nodes and leaf nodes. Inner nodes contain M node
/// references and M-1 keys while leaf nodes contain N keys and values. Values for M and N are
/// chosen such that a node is exactly 64 bytes (a cache line) when keys and values are 32 bits
/// each.
///
/// An inner node contains at least M/2 node references unless it is the right-most node at its
/// level. A leaf node contains at least N/2 keys unless it is the right-most leaf.
pub(super) enum NodeData<F: Forest> {
    Inner {
        /// The number of keys in this node.
        /// The number of node references is always one more.
        size: u8,

        /// Keys discriminating sub-trees.
        ///
        /// The key in `keys[i]` is greater than all keys in `tree[i]` and less than or equal to
        /// all keys in `tree[i+1]`.
        keys: [F::Key; INNER_SIZE - 1],

        /// Sub-trees.
        tree: [Node; INNER_SIZE],
    },
    Leaf {
        /// Number of key-value pairs in this node.
        size: u8,

        // Key array.
        keys: F::LeafKeys,

        // Value array.
        vals: F::LeafValues,
    },
    /// An unused node on the free list.
    Free { next: Option<Node> },
}

// Implement `Clone` and `Copy` manually, because deriving them would also require `Forest` to
// implement `Clone`.
impl<F: Forest> Copy for NodeData<F> {}
impl<F: Forest> Clone for NodeData<F> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<F: Forest> NodeData<F> {
    /// Is this a free/unused node?
    pub fn is_free(&self) -> bool {
        match *self {
            Self::Free { .. } => true,
            _ => false,
        }
    }

    /// Get the number of entries in this node.
    ///
    /// This is the number of outgoing edges in an inner node, or the number of key-value pairs in
    /// a leaf node.
    pub fn entries(&self) -> usize {
        match *self {
            Self::Inner { size, .. } => usize::from(size) + 1,
            Self::Leaf { size, .. } => usize::from(size),
            Self::Free { .. } => panic!("freed node"),
        }
    }

    /// Create an inner node with a single key and two sub-trees.
    pub fn inner(left: Node, key: F::Key, right: Node) -> Self {
        // Splat the key and right node to the whole array.
        // Saves us from inventing a default/reserved value.
        let mut tree = [right; INNER_SIZE];
        tree[0] = left;
        Self::Inner {
            size: 1,
            keys: [key; INNER_SIZE - 1],
            tree,
        }
    }

    /// Create a leaf node with a single key-value pair.
    pub fn leaf(key: F::Key, value: F::Value) -> Self {
        Self::Leaf {
            size: 1,
            keys: F::splat_key(key),
            vals: F::splat_value(value),
        }
    }

    /// Unwrap an inner node into two slices (keys, trees).
    pub fn unwrap_inner(&self) -> (&[F::Key], &[Node]) {
        match *self {
            Self::Inner {
                size,
                ref keys,
                ref tree,
            } => {
                let size = usize::from(size);
                // TODO: We could probably use `get_unchecked()` here since `size` is always in
                // range.
                (&keys[0..size], &tree[0..=size])
            }
            _ => panic!("Expected inner node"),
        }
    }

    /// Unwrap a leaf node into two slices (keys, values) of the same length.
    pub fn unwrap_leaf(&self) -> (&[F::Key], &[F::Value]) {
        match *self {
            Self::Leaf {
                size,
                ref keys,
                ref vals,
            } => {
                let size = usize::from(size);
                let keys = keys.borrow();
                let vals = vals.borrow();
                // TODO: We could probably use `get_unchecked()` here since `size` is always in
                // range.
                (&keys[0..size], &vals[0..size])
            }
            _ => panic!("Expected leaf node"),
        }
    }

    /// Unwrap a mutable leaf node into two slices (keys, values) of the same length.
    pub fn unwrap_leaf_mut(&mut self) -> (&mut [F::Key], &mut [F::Value]) {
        match *self {
            Self::Leaf {
                size,
                ref mut keys,
                ref mut vals,
            } => {
                let size = usize::from(size);
                let keys = keys.borrow_mut();
                let vals = vals.borrow_mut();
                // TODO: We could probably use `get_unchecked_mut()` here since `size` is always in
                // range.
                (&mut keys[0..size], &mut vals[0..size])
            }
            _ => panic!("Expected leaf node"),
        }
    }

    /// Get the critical key for a leaf node.
    /// This is simply the first key.
    pub fn leaf_crit_key(&self) -> F::Key {
        match *self {
            Self::Leaf { size, ref keys, .. } => {
                debug_assert!(size > 0, "Empty leaf node");
                keys.borrow()[0]
            }
            _ => panic!("Expected leaf node"),
        }
    }

    /// Try to insert `(key, node)` at key-position `index` in an inner node.
    /// This means that `key` is inserted at `keys[i]` and `node` is inserted at `tree[i + 1]`.
    /// If the node is full, this leaves the node unchanged and returns false.
    pub fn try_inner_insert(&mut self, index: usize, key: F::Key, node: Node) -> bool {
        match *self {
            Self::Inner {
                ref mut size,
                ref mut keys,
                ref mut tree,
            } => {
                let sz = usize::from(*size);
                debug_assert!(sz <= keys.len());
                debug_assert!(index <= sz, "Can't insert at {index} with {sz} keys");

                if let Some(ks) = keys.get_mut(0..=sz) {
                    *size = (sz + 1) as u8;
                    slice_insert(ks, index, key);
                    slice_insert(&mut tree[1..=sz + 1], index, node);
                    true
                } else {
                    false
                }
            }
            _ => panic!("Expected inner node"),
        }
    }

    /// Try to insert `key, value` at `index` in a leaf node, but fail and return false if the node
    /// is full.
    pub fn try_leaf_insert(&mut self, index: usize, key: F::Key, value: F::Value) -> bool {
        match *self {
            Self::Leaf {
                ref mut size,
                ref mut keys,
                ref mut vals,
            } => {
                let sz = usize::from(*size);
                let keys = keys.borrow_mut();
                let vals = vals.borrow_mut();
                debug_assert!(sz <= keys.len());
                debug_assert!(index <= sz);

                if let Some(ks) = keys.get_mut(0..=sz) {
                    *size = (sz + 1) as u8;
                    slice_insert(ks, index, key);
                    slice_insert(&mut vals[0..=sz], index, value);
                    true
                } else {
                    false
                }
            }
            _ => panic!("Expected leaf node"),
        }
    }

    /// Split off the second half of this node.
    /// It is assumed that this a completely full inner or leaf node.
    ///
    /// The `insert_index` parameter is the position where an insertion was tried and failed. The
    /// node will be split in half with a bias towards an even split after the insertion is retried.
    pub fn split(&mut self, insert_index: usize) -> SplitOff<F> {
        match *self {
            Self::Inner {
                ref mut size,
                ref keys,
                ref tree,
            } => {
                debug_assert_eq!(usize::from(*size), keys.len(), "Node not full");

                // Number of tree entries in the lhs node.
                let l_ents = split_pos(tree.len(), insert_index + 1);
                let r_ents = tree.len() - l_ents;

                // With INNER_SIZE=8, we get l_ents=4 and:
                //
                // self: [ n0 k0 n1 k1 n2 k2 n3 k3 n4 k4 n5 k5 n6 k6 n7 ]
                // lhs:  [ n0 k0 n1 k1 n2 k2 n3 ]
                // crit_key = k3 (not present in either node)
                // rhs:  [ n4 k4 n5 k5 n6 k6 n7 ]

                // 1. Truncate the LHS.
                *size = (l_ents - 1) as u8;

                // 2. Copy second half to `rhs_data`.
                let mut r_keys = *keys;
                r_keys[0..r_ents - 1].copy_from_slice(&keys[l_ents..]);

                let mut r_tree = *tree;
                r_tree[0..r_ents].copy_from_slice(&tree[l_ents..]);

                SplitOff {
                    lhs_entries: l_ents,
                    rhs_entries: r_ents,
                    crit_key: keys[l_ents - 1],
                    rhs_data: Self::Inner {
                        size: (r_ents - 1) as u8,
                        keys: r_keys,
                        tree: r_tree,
                    },
                }
            }
            Self::Leaf {
                ref mut size,
                ref keys,
                ref vals,
            } => {
                let o_keys = keys.borrow();
                let o_vals = vals.borrow();
                debug_assert_eq!(usize::from(*size), o_keys.len(), "Node not full");

                let l_size = split_pos(o_keys.len(), insert_index);
                let r_size = o_keys.len() - l_size;

                // 1. Truncate the LHS node at `l_size`.
                *size = l_size as u8;

                // 2. Copy second half to `rhs_data`.
                let mut r_keys = *keys;
                r_keys.borrow_mut()[0..r_size].copy_from_slice(&o_keys[l_size..]);

                let mut r_vals = *vals;
                r_vals.borrow_mut()[0..r_size].copy_from_slice(&o_vals[l_size..]);

                SplitOff {
                    lhs_entries: l_size,
                    rhs_entries: r_size,
                    crit_key: o_keys[l_size],
                    rhs_data: Self::Leaf {
                        size: r_size as u8,
                        keys: r_keys,
                        vals: r_vals,
                    },
                }
            }
            _ => panic!("Expected leaf node"),
        }
    }

    /// Remove the sub-tree at `index` from this inner node.
    ///
    /// Note that `index` refers to a sub-tree entry and not a key entry as it does for
    /// `try_inner_insert()`. It is possible to remove the first sub-tree (which can't be inserted
    /// by `try_inner_insert()`).
    ///
    /// Return an indication of the node's health (i.e. below half capacity).
    pub fn inner_remove(&mut self, index: usize) -> Removed {
        match *self {
            Self::Inner {
                ref mut size,
                ref mut keys,
                ref mut tree,
            } => {
                let ents = usize::from(*size) + 1;
                debug_assert!(ents <= tree.len());
                debug_assert!(index < ents);
                // Leave an invalid 0xff size when node becomes empty.
                *size = ents.wrapping_sub(2) as u8;
                if ents > 1 {
                    slice_shift(&mut keys[index.saturating_sub(1)..ents - 1], 1);
                }
                slice_shift(&mut tree[index..ents], 1);
                Removed::new(index, ents - 1, tree.len())
            }
            _ => panic!("Expected inner node"),
        }
    }

    /// Remove the key-value pair at `index` from this leaf node.
    ///
    /// Return an indication of the node's health (i.e. below half capacity).
    pub fn leaf_remove(&mut self, index: usize) -> Removed {
        match *self {
            Self::Leaf {
                ref mut size,
                ref mut keys,
                ref mut vals,
            } => {
                let sz = usize::from(*size);
                let keys = keys.borrow_mut();
                let vals = vals.borrow_mut();
                *size -= 1;
                slice_shift(&mut keys[index..sz], 1);
                slice_shift(&mut vals[index..sz], 1);
                Removed::new(index, sz - 1, keys.len())
            }
            _ => panic!("Expected leaf node"),
        }
    }

    /// Balance this node with its right sibling.
    ///
    /// It is assumed that the current node has underflowed. Look at the right sibling node and do
    /// one of two things:
    ///
    /// 1. Move all entries to the right node, leaving this node empty, or
    /// 2. Distribute entries evenly between the two nodes.
    ///
    /// In the first case, `None` is returned. In the second case, the new critical key for the
    /// right sibling node is returned.
    pub fn balance(&mut self, crit_key: F::Key, rhs: &mut Self) -> Option<F::Key> {
        match (self, rhs) {
            (
                &mut Self::Inner {
                    size: ref mut l_size,
                    keys: ref mut l_keys,
                    tree: ref mut l_tree,
                },
                &mut Self::Inner {
                    size: ref mut r_size,
                    keys: ref mut r_keys,
                    tree: ref mut r_tree,
                },
            ) => {
                let l_ents = usize::from(*l_size) + 1;
                let r_ents = usize::from(*r_size) + 1;
                let ents = l_ents + r_ents;

                if ents <= r_tree.len() {
                    // All entries will fit in the RHS node.
                    // We'll leave the LHS node empty, but first use it as a scratch space.
                    *l_size = 0;
                    // Insert `crit_key` between the two nodes.
                    l_keys[l_ents - 1] = crit_key;
                    l_keys[l_ents..ents - 1].copy_from_slice(&r_keys[0..r_ents - 1]);
                    r_keys[0..ents - 1].copy_from_slice(&l_keys[0..ents - 1]);
                    l_tree[l_ents..ents].copy_from_slice(&r_tree[0..r_ents]);
                    r_tree[0..ents].copy_from_slice(&l_tree[0..ents]);
                    *r_size = (ents - 1) as u8;
                    None
                } else {
                    // The entries don't all fit in one node. Distribute some from RHS -> LHS.
                    // Split evenly with a bias to putting one entry in LHS.
                    let r_goal = ents / 2;
                    let l_goal = ents - r_goal;
                    debug_assert!(l_goal > l_ents, "Node must be underflowed");

                    l_keys[l_ents - 1] = crit_key;
                    l_keys[l_ents..l_goal - 1].copy_from_slice(&r_keys[0..l_goal - 1 - l_ents]);
                    l_tree[l_ents..l_goal].copy_from_slice(&r_tree[0..l_goal - l_ents]);
                    *l_size = (l_goal - 1) as u8;

                    let new_crit = r_keys[r_ents - r_goal - 1];
                    slice_shift(&mut r_keys[0..r_ents - 1], r_ents - r_goal);
                    slice_shift(&mut r_tree[0..r_ents], r_ents - r_goal);
                    *r_size = (r_goal - 1) as u8;

                    Some(new_crit)
                }
            }
            (
                &mut Self::Leaf {
                    size: ref mut l_size,
                    keys: ref mut l_keys,
                    vals: ref mut l_vals,
                },
                &mut Self::Leaf {
                    size: ref mut r_size,
                    keys: ref mut r_keys,
                    vals: ref mut r_vals,
                },
            ) => {
                let l_ents = usize::from(*l_size);
                let l_keys = l_keys.borrow_mut();
                let l_vals = l_vals.borrow_mut();
                let r_ents = usize::from(*r_size);
                let r_keys = r_keys.borrow_mut();
                let r_vals = r_vals.borrow_mut();
                let ents = l_ents + r_ents;

                if ents <= r_vals.len() {
                    // We can fit all entries in the RHS node.
                    // We'll leave the LHS node empty, but first use it as a scratch space.
                    *l_size = 0;
                    l_keys[l_ents..ents].copy_from_slice(&r_keys[0..r_ents]);
                    r_keys[0..ents].copy_from_slice(&l_keys[0..ents]);
                    l_vals[l_ents..ents].copy_from_slice(&r_vals[0..r_ents]);
                    r_vals[0..ents].copy_from_slice(&l_vals[0..ents]);
                    *r_size = ents as u8;
                    None
                } else {
                    // The entries don't all fit in one node. Distribute some from RHS -> LHS.
                    // Split evenly with a bias to putting one entry in LHS.
                    let r_goal = ents / 2;
                    let l_goal = ents - r_goal;
                    debug_assert!(l_goal > l_ents, "Node must be underflowed");

                    l_keys[l_ents..l_goal].copy_from_slice(&r_keys[0..l_goal - l_ents]);
                    l_vals[l_ents..l_goal].copy_from_slice(&r_vals[0..l_goal - l_ents]);
                    *l_size = l_goal as u8;

                    slice_shift(&mut r_keys[0..r_ents], r_ents - r_goal);
                    slice_shift(&mut r_vals[0..r_ents], r_ents - r_goal);
                    *r_size = r_goal as u8;

                    Some(r_keys[0])
                }
            }
            _ => panic!("Mismatched nodes"),
        }
    }
}

/// Find the right split position for halving a full node with `len` entries to recover from a
/// failed insertion at `ins`.
///
/// If `len` is even, we should split straight down the middle regardless of `len`.
///
/// If `len` is odd, we should split the node such that the two halves are the same size after the
/// insertion is retried.
fn split_pos(len: usize, ins: usize) -> usize {
    // Anticipate `len` being a compile time constant, so this all folds away when `len` is even.
    if ins <= len / 2 {
        len / 2
    } else {
        (len + 1) / 2
    }
}

/// The result of splitting off the second half of a node.
pub(super) struct SplitOff<F: Forest> {
    /// The number of entries left in the original node which becomes the left-hand-side of the
    /// pair. This is the number of outgoing node edges for an inner node, and the number of
    /// key-value pairs for a leaf node.
    pub lhs_entries: usize,

    /// The number of entries in the new RHS node.
    pub rhs_entries: usize,

    /// The critical key separating the LHS and RHS nodes. All keys in the LHS sub-tree are less
    /// than the critical key, and all entries in the RHS sub-tree are greater or equal to the
    /// critical key.
    pub crit_key: F::Key,

    /// The RHS node data containing the elements that were removed from the original node (now the
    /// LHS).
    pub rhs_data: NodeData<F>,
}

/// The result of removing an entry from a node.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Removed {
    /// An entry was removed, and the node is still in good shape.
    Healthy,

    /// The node is in good shape after removing the rightmost element.
    Rightmost,

    /// The node has too few entries now, and it should be balanced with a sibling node.
    Underflow,

    /// The last entry was removed. For an inner node, this means that the `keys` array is empty
    /// and there is just a single sub-tree left.
    Empty,
}

impl Removed {
    /// Create a `Removed` status from a size and capacity.
    fn new(removed: usize, new_size: usize, capacity: usize) -> Self {
        if 2 * new_size >= capacity {
            if removed == new_size {
                Self::Rightmost
            } else {
                Self::Healthy
            }
        } else if new_size > 0 {
            Self::Underflow
        } else {
            Self::Empty
        }
    }
}

// Display ": value" or nothing at all for `()`.
pub(super) trait ValDisp {
    fn valfmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl ValDisp for SetValue {
    fn valfmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl<T: fmt::Display> ValDisp for T {
    fn valfmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ":{self}")
    }
}

impl<F> fmt::Display for NodeData<F>
where
    F: Forest,
    F::Key: fmt::Display,
    F::Value: ValDisp,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Inner { size, keys, tree } => {
                write!(f, "[ {}", tree[0])?;
                for i in 0..usize::from(size) {
                    write!(f, " {} {}", keys[i], tree[i + 1])?;
                }
                write!(f, " ]")
            }
            Self::Leaf { size, keys, vals } => {
                let keys = keys.borrow();
                let vals = vals.borrow();
                write!(f, "[")?;
                for i in 0..usize::from(size) {
                    write!(f, " {}", keys[i])?;
                    vals[i].valfmt(f)?;
                }
                write!(f, " ]")
            }
            Self::Free { next: Some(n) } => write!(f, "[ free -> {n} ]"),
            Self::Free { next: None } => write!(f, "[ free ]"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use core::mem;

    // Forest impl for a set implementation.
    struct TF();

    impl Forest for TF {
        type Key = char;
        type Value = SetValue;
        type LeafKeys = [char; 15];
        type LeafValues = [SetValue; 15];

        fn splat_key(key: Self::Key) -> Self::LeafKeys {
            [key; 15]
        }

        fn splat_value(value: Self::Value) -> Self::LeafValues {
            [value; 15]
        }
    }

    #[test]
    fn inner() {
        let n1 = Node(1);
        let n2 = Node(2);
        let n3 = Node(3);
        let n4 = Node(4);
        let mut inner = NodeData::<TF>::inner(n1, 'c', n4);
        assert_eq!(mem::size_of_val(&inner), 64);
        assert_eq!(inner.to_string(), "[ node1 c node4 ]");

        assert!(inner.try_inner_insert(0, 'a', n2));
        assert_eq!(inner.to_string(), "[ node1 a node2 c node4 ]");

        assert!(inner.try_inner_insert(1, 'b', n3));
        assert_eq!(inner.to_string(), "[ node1 a node2 b node3 c node4 ]");

        for i in 3..7 {
            assert!(inner.try_inner_insert(
                usize::from(i),
                ('a' as u8 + i) as char,
                Node(i as u32 + 2),
            ));
        }
        assert_eq!(
            inner.to_string(),
            "[ node1 a node2 b node3 c node4 d node5 e node6 f node7 g node8 ]"
        );

        // Now the node is full and insertion should fail anywhere.
        assert!(!inner.try_inner_insert(0, 'x', n3));
        assert!(!inner.try_inner_insert(4, 'x', n3));
        assert!(!inner.try_inner_insert(7, 'x', n3));

        // Splitting should be independent of the hint because we have an even number of node
        // references.
        let saved = inner;
        let sp = inner.split(1);
        assert_eq!(sp.lhs_entries, 4);
        assert_eq!(sp.rhs_entries, 4);
        assert_eq!(sp.crit_key, 'd');
        // The critical key is not present in either of the resulting nodes.
        assert_eq!(inner.to_string(), "[ node1 a node2 b node3 c node4 ]");
        assert_eq!(sp.rhs_data.to_string(), "[ node5 e node6 f node7 g node8 ]");

        assert_eq!(inner.inner_remove(0), Removed::Underflow);
        assert_eq!(inner.to_string(), "[ node2 b node3 c node4 ]");

        assert_eq!(inner.inner_remove(1), Removed::Underflow);
        assert_eq!(inner.to_string(), "[ node2 c node4 ]");

        assert_eq!(inner.inner_remove(1), Removed::Underflow);
        assert_eq!(inner.to_string(), "[ node2 ]");

        assert_eq!(inner.inner_remove(0), Removed::Empty);

        inner = saved;
        let sp = inner.split(6);
        assert_eq!(sp.lhs_entries, 4);
        assert_eq!(sp.rhs_entries, 4);
        assert_eq!(sp.crit_key, 'd');
        assert_eq!(inner.to_string(), "[ node1 a node2 b node3 c node4 ]");
        assert_eq!(sp.rhs_data.to_string(), "[ node5 e node6 f node7 g node8 ]");
    }

    #[test]
    fn leaf() {
        let mut leaf = NodeData::<TF>::leaf('d', SetValue());
        assert_eq!(leaf.to_string(), "[ d ]");

        assert!(leaf.try_leaf_insert(0, 'a', SetValue()));
        assert_eq!(leaf.to_string(), "[ a d ]");
        assert!(leaf.try_leaf_insert(1, 'b', SetValue()));
        assert!(leaf.try_leaf_insert(2, 'c', SetValue()));
        assert_eq!(leaf.to_string(), "[ a b c d ]");
        for i in 4..15 {
            assert!(leaf.try_leaf_insert(usize::from(i), ('a' as u8 + i) as char, SetValue()));
        }
        assert_eq!(leaf.to_string(), "[ a b c d e f g h i j k l m n o ]");

        // Now the node is full and insertion should fail anywhere.
        assert!(!leaf.try_leaf_insert(0, 'x', SetValue()));
        assert!(!leaf.try_leaf_insert(8, 'x', SetValue()));
        assert!(!leaf.try_leaf_insert(15, 'x', SetValue()));

        // The index given to `split` is not the split position, it's a hint for balancing the node.
        let saved = leaf;
        let sp = leaf.split(12);
        assert_eq!(sp.lhs_entries, 8);
        assert_eq!(sp.rhs_entries, 7);
        assert_eq!(sp.crit_key, 'i');
        assert_eq!(leaf.to_string(), "[ a b c d e f g h ]");
        assert_eq!(sp.rhs_data.to_string(), "[ i j k l m n o ]");

        assert!(leaf.try_leaf_insert(8, 'i', SetValue()));
        assert_eq!(leaf.leaf_remove(2), Removed::Healthy);
        assert_eq!(leaf.to_string(), "[ a b d e f g h i ]");
        assert_eq!(leaf.leaf_remove(7), Removed::Underflow);
        assert_eq!(leaf.to_string(), "[ a b d e f g h ]");

        leaf = saved;
        let sp = leaf.split(7);
        assert_eq!(sp.lhs_entries, 7);
        assert_eq!(sp.rhs_entries, 8);
        assert_eq!(sp.crit_key, 'h');
        assert_eq!(leaf.to_string(), "[ a b c d e f g ]");
        assert_eq!(sp.rhs_data.to_string(), "[ h i j k l m n o ]");
    }

    #[test]
    fn optimal_split_pos() {
        // An even split is easy.
        assert_eq!(split_pos(8, 0), 4);
        assert_eq!(split_pos(8, 8), 4);

        // Easy cases for odd splits.
        assert_eq!(split_pos(7, 0), 3);
        assert_eq!(split_pos(7, 7), 4);

        // If the insertion point is the same as the split position, we
        // will append to the lhs node.
        assert_eq!(split_pos(7, 3), 3);
        assert_eq!(split_pos(7, 4), 4);
    }

    #[test]
    fn inner_balance() {
        let n1 = Node(1);
        let n2 = Node(2);
        let n3 = Node(3);
        let mut lhs = NodeData::<TF>::inner(n1, 'a', n2);
        assert!(lhs.try_inner_insert(1, 'b', n3));
        assert_eq!(lhs.to_string(), "[ node1 a node2 b node3 ]");

        let n11 = Node(11);
        let n12 = Node(12);
        let mut rhs = NodeData::<TF>::inner(n11, 'p', n12);

        for i in 1..4 {
            assert!(rhs.try_inner_insert(
                usize::from(i),
                ('p' as u8 + i) as char,
                Node(i as u32 + 12),
            ));
        }
        assert_eq!(
            rhs.to_string(),
            "[ node11 p node12 q node13 r node14 s node15 ]"
        );

        // 3+5 elements fit in RHS.
        assert_eq!(lhs.balance('o', &mut rhs), None);
        assert_eq!(
            rhs.to_string(),
            "[ node1 a node2 b node3 o node11 p node12 q node13 r node14 s node15 ]"
        );

        // 2+8 elements are redistributed.
        lhs = NodeData::<TF>::inner(Node(20), 'x', Node(21));
        assert_eq!(lhs.balance('y', &mut rhs), Some('o'));
        assert_eq!(
            lhs.to_string(),
            "[ node20 x node21 y node1 a node2 b node3 ]"
        );
        assert_eq!(
            rhs.to_string(),
            "[ node11 p node12 q node13 r node14 s node15 ]"
        );
    }

    #[test]
    fn leaf_balance() {
        let mut lhs = NodeData::<TF>::leaf('a', SetValue());
        for i in 1..6 {
            assert!(lhs.try_leaf_insert(usize::from(i), ('a' as u8 + i) as char, SetValue()));
        }
        assert_eq!(lhs.to_string(), "[ a b c d e f ]");

        let mut rhs = NodeData::<TF>::leaf('0', SetValue());
        for i in 1..8 {
            assert!(rhs.try_leaf_insert(usize::from(i), ('0' as u8 + i) as char, SetValue()));
        }
        assert_eq!(rhs.to_string(), "[ 0 1 2 3 4 5 6 7 ]");

        // 6+8 elements all fits in rhs.
        assert_eq!(lhs.balance('0', &mut rhs), None);
        assert_eq!(rhs.to_string(), "[ a b c d e f 0 1 2 3 4 5 6 7 ]");

        assert!(lhs.try_leaf_insert(0, 'x', SetValue()));
        assert!(lhs.try_leaf_insert(1, 'y', SetValue()));
        assert!(lhs.try_leaf_insert(2, 'z', SetValue()));
        assert_eq!(lhs.to_string(), "[ x y z ]");

        // 3+14 elements need redistribution.
        assert_eq!(lhs.balance('a', &mut rhs), Some('0'));
        assert_eq!(lhs.to_string(), "[ x y z a b c d e f ]");
        assert_eq!(rhs.to_string(), "[ 0 1 2 3 4 5 6 7 ]");
    }
}
