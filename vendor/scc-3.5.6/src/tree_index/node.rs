use std::fmt;
use std::ops::RangeBounds;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Relaxed};

use sdd::{AtomicShared, Guard, Ptr, Shared, Tag};

use super::internal_node::InternalNode;
use super::internal_node::Locker as InternalNodeLocker;
use super::leaf::{InsertResult, Iter, Leaf, RemoveResult, RevIter};
use super::leaf_node::LeafNode;
use crate::Comparable;
use crate::async_helper::LockPager;

/// [`Node`] is either [`Self::Internal`] or [`Self::Leaf`].
pub enum Node<K, V> {
    /// Internal node.
    Internal(InternalNode<K, V>),
    /// Leaf node.
    Leaf(LeafNode<K, V>),
}

impl<K, V> Node<K, V> {
    /// Creates a new [`InternalNode`].
    #[inline]
    pub(super) fn new_internal_node() -> Self {
        Self::Internal(InternalNode::new())
    }

    /// Creates a new [`LeafNode`].
    #[inline]
    pub(super) fn new_leaf_node() -> Self {
        Self::Leaf(LeafNode::new())
    }

    /// Returns the depth of the node.
    #[inline]
    pub(super) fn depth(&self, depth: usize, guard: &Guard) -> usize {
        match &self {
            Self::Internal(internal_node) => internal_node.depth(depth, guard),
            Self::Leaf(_) => depth,
        }
    }

    /// Checks if the node has retired.
    #[inline]
    pub(super) fn is_retired(&self) -> bool {
        match &self {
            Self::Internal(internal_node) => internal_node.is_retired(),
            Self::Leaf(leaf_node) => leaf_node.is_retired(),
        }
    }

    /// Freezes children nodes or leaves.
    #[inline]
    pub(super) fn freeze(&self) -> bool {
        match &self {
            Self::Internal(internal_node) => internal_node.children.freeze(),
            Self::Leaf(leaf_node) => leaf_node.children.freeze(),
        }
    }
}

impl<K, V> Node<K, V>
where
    K: 'static + Clone + Ord,
    V: 'static,
{
    /// Searches for an entry containing the specified key.
    #[inline]
    pub(super) fn search_entry<'g, Q>(&self, key: &Q, guard: &'g Guard) -> Option<(&'g K, &'g V)>
    where
        K: 'g,
        Q: Comparable<K> + ?Sized,
    {
        match &self {
            Self::Internal(internal_node) => internal_node.search_entry(key, guard),
            Self::Leaf(leaf_node) => leaf_node.search_entry(key, guard),
        }
    }

    /// Searches for the value associated with the specified key.
    #[inline]
    pub(super) fn search_value<'g, Q>(&self, key: &Q, guard: &'g Guard) -> Option<&'g V>
    where
        K: 'g,
        Q: Comparable<K> + ?Sized,
    {
        match &self {
            Self::Internal(internal_node) => internal_node.search_value(key, guard),
            Self::Leaf(leaf_node) => leaf_node.search_value(key, guard),
        }
    }

    /// Returns the minimum key entry in the entire tree.
    #[inline]
    pub(super) fn min<'g>(&self, guard: &'g Guard) -> Option<Iter<'g, K, V>> {
        match &self {
            Self::Internal(internal_node) => internal_node.min(guard),
            Self::Leaf(leaf_node) => leaf_node.min(guard),
        }
    }

    /// Returns a [`RevIter`] pointing to the right-most leaf in the entire tree.
    #[inline]
    pub(super) fn max<'g>(&self, guard: &'g Guard) -> Option<RevIter<'g, K, V>> {
        match &self {
            Self::Internal(internal_node) => internal_node.max(guard),
            Self::Leaf(leaf_node) => leaf_node.max(guard),
        }
    }

    /// Returns a [`Iter`] pointing to an entry that is close enough to the specified key.
    ///
    /// If `LE == true`, the returned [`Iter`] does not contain any keys larger than the specified
    /// key. If not, the returned [`Iter`] does not contain any keys smaller than the specified key.
    #[inline]
    pub(super) fn approximate<'g, Q, const LE: bool>(
        &self,
        key: &Q,
        guard: &'g Guard,
    ) -> Option<Iter<'g, K, V>>
    where
        K: 'g,
        Q: Comparable<K> + ?Sized,
    {
        match &self {
            Self::Internal(internal_node) => internal_node.approximate::<_, LE>(key, guard),
            Self::Leaf(leaf_node) => leaf_node.approximate::<_, LE>(key, guard),
        }
    }

    /// Inserts a key-value pair.
    #[inline]
    pub(super) fn insert<P: LockPager>(
        &self,
        key: K,
        val: V,
        pager: &mut P,
        guard: &Guard,
    ) -> Result<InsertResult<K, V>, (K, V)> {
        match &self {
            Self::Internal(internal_node) => internal_node.insert(key, val, pager, guard),
            Self::Leaf(leaf_node) => leaf_node.insert(key, val, pager, guard),
        }
    }

    /// Removes an entry associated with the given key.
    #[inline]
    pub(super) fn remove_if<Q, F: FnMut(&V) -> bool, P>(
        &self,
        key: &Q,
        condition: &mut F,
        pager: &mut P,
        guard: &Guard,
    ) -> Result<RemoveResult, ()>
    where
        Q: Comparable<K> + ?Sized,
        P: LockPager,
    {
        match &self {
            Self::Internal(internal_node) => {
                internal_node.remove_if::<_, _, _>(key, condition, pager, guard)
            }
            Self::Leaf(leaf_node) => leaf_node.remove_if::<_, _, _>(key, condition, pager, guard),
        }
    }

    /// Removes a range of entries.
    ///
    /// Returns the number of remaining children.
    #[inline]
    pub(super) fn remove_range<'g, Q, R: RangeBounds<Q>, P: LockPager>(
        &self,
        range: &R,
        start_unbounded: bool,
        valid_lower_max_leaf: Option<&'g Leaf<K, V>>,
        valid_upper_min_node: Option<&'g Node<K, V>>,
        pager: &mut P,
        guard: &'g Guard,
    ) -> Result<usize, ()>
    where
        Q: Comparable<K> + ?Sized,
    {
        match &self {
            Self::Internal(internal_node) => internal_node.remove_range(
                range,
                start_unbounded,
                valid_lower_max_leaf,
                valid_upper_min_node,
                pager,
                guard,
            ),
            Self::Leaf(leaf_node) => leaf_node.remove_range(
                range,
                start_unbounded,
                valid_lower_max_leaf,
                valid_upper_min_node,
                pager,
                guard,
            ),
        }
    }

    /// Splits the current root node.
    #[inline]
    pub(super) fn split_root(
        root_ptr: Ptr<Node<K, V>>,
        root: &AtomicShared<Node<K, V>>,
        guard: &Guard,
    ) {
        if let Some(old_root) = root_ptr.get_shared() {
            let new_root = if old_root.is_retired() {
                Some(Shared::new_with(Node::new_leaf_node))
            } else {
                let internal_node = Shared::new_with(Node::new_internal_node);
                let Node::Internal(node) = internal_node.as_ref() else {
                    return;
                };
                node.unbounded_child
                    .swap((Some(old_root), Tag::None), Relaxed);
                Some(internal_node)
            };
            // Updates the pointer before unlocking the root.
            if let Err((Some(new_root), _)) =
                root.compare_exchange(root_ptr, (new_root, Tag::None), AcqRel, Acquire, guard)
            {
                let dropped = unsafe { new_root.drop_in_place() };
                debug_assert!(dropped);
            }
        }
    }

    /// Cleans up or removes the current root node.
    ///
    /// If the root is empty, the root is removed from the tree, or if the root has only a single
    /// child, the root is replaced with the child.
    ///
    /// Returns `false` if a conflict is detected.
    #[inline]
    pub(super) fn cleanup_root<P: LockPager>(
        root: &AtomicShared<Node<K, V>>,
        pager: &mut P,
        guard: &Guard,
    ) -> bool {
        let mut root_ptr = root.load(Acquire, guard);
        while let Some(root_ref) = root_ptr.as_ref() {
            if root_ref.is_retired() {
                if let Err((_, new_root_ptr)) =
                    root.compare_exchange(root_ptr, (None, Tag::None), AcqRel, Acquire, guard)
                {
                    root_ptr = new_root_ptr;
                    continue;
                }
                // The entire tree was truncated.
                break;
            }

            // Try to lower the tree.
            let Self::Internal(internal_node) = root_ref else {
                break;
            };

            if !internal_node.children.is_empty() {
                // Not empty.
                break;
            }

            let locker = match pager.try_acquire(&internal_node.lock) {
                Ok(true) => InternalNodeLocker { internal_node },
                Ok(false) => {
                    // The root was retired.
                    continue;
                }
                Err(()) => return false,
            };

            let new_root = if internal_node.children.is_empty() {
                // Replace the root with the unbounded child.
                internal_node.unbounded_child.get_shared(Acquire, guard)
            } else {
                // The internal node is not empty.
                break;
            };
            match root.compare_exchange(root_ptr, (new_root, Tag::None), AcqRel, Acquire, guard) {
                Ok((_, new_root_ptr)) => {
                    locker.unlock_retire();
                    root_ptr = new_root_ptr;
                }
                Err((_, new_root_ptr)) => {
                    // The root node has been changed.
                    root_ptr = new_root_ptr;
                }
            }
        }

        true
    }
}

impl<K, V> fmt::Debug for Node<K, V>
where
    K: 'static + Clone + fmt::Debug + Ord,
    V: 'static + fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Internal(internal_node) => internal_node.fmt(f),
            Self::Leaf(leaf_node) => leaf_node.fmt(f),
        }
    }
}
