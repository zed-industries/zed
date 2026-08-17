use std::mem::forget;
use std::ops::RangeBounds;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Relaxed, Release};
use std::{fmt, ptr};

use saa::Lock;
use sdd::{AtomicShared, Guard, Ptr, Shared, Tag};

use super::leaf::{
    Array, ArrayIter, ArrayRevIter, InsertResult, Iter, Leaf, RemoveResult, RevIter,
};
use super::leaf_node::Locker as LeafNodeLocker;
use super::leaf_node::RemoveRangeState;
use super::node::Node;
use crate::Comparable;
use crate::async_helper::LockPager;

/// Internal node.
///
/// The layout of an internal node: `|ptr(children)/max(child keys)|...|ptr(children)|`.
pub struct InternalNode<K, V> {
    /// A child [`Node`] that has no upper key bound.
    ///
    /// It stores the maximum key in the node, and key-value pairs are first pushed to this [`Node`]
    /// until it splits.
    pub(super) unbounded_child: AtomicShared<Node<K, V>>,
    /// Children of the [`InternalNode`].
    pub(super) children: Array<K, AtomicShared<Node<K, V>>>,
    /// [`Lock`] to protect the [`InternalNode`].
    pub(super) lock: Lock,
}

/// [`Locker`] holds exclusive ownership of an [`InternalNode`].
pub(super) struct Locker<'n, K, V> {
    pub(super) internal_node: &'n InternalNode<K, V>,
}

impl<K, V> InternalNode<K, V> {
    /// Creates a new empty internal node.
    #[inline]
    pub(super) fn new() -> InternalNode<K, V> {
        InternalNode {
            unbounded_child: AtomicShared::null(),
            children: Array::new(),
            lock: Lock::default(),
        }
    }

    /// Returns the depth of the node.
    #[inline]
    pub(super) fn depth(&self, depth: usize, guard: &Guard) -> usize {
        let unbounded_ptr = self.unbounded_child.load(Relaxed, guard);
        if let Some(unbounded_ref) = unbounded_ptr.as_ref() {
            return unbounded_ref.depth(depth + 1, guard);
        }
        depth
    }

    /// Returns `true` if the [`InternalNode`] has retired.
    #[inline]
    pub(super) fn is_retired(&self) -> bool {
        self.lock.is_poisoned(Acquire)
    }
}

impl<K, V> InternalNode<K, V>
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
        loop {
            let (child, metadata) = self.children.min_greater_equal(key);
            if let Some(child) = child {
                if let Some(child) = child.load(Acquire, guard).as_ref() {
                    if self.children.validate(metadata) {
                        // Data race resolution - see `LeafNode::search_entry`.
                        return child.search_entry(key, guard);
                    }
                }
            } else {
                let unbounded_ptr = self.unbounded_child.load(Acquire, guard);
                if let Some(unbounded) = unbounded_ptr.as_ref() {
                    if self.children.validate(metadata) {
                        return unbounded.search_entry(key, guard);
                    }
                } else {
                    return None;
                }
            }
        }
    }

    /// Searches for the value associated with the specified key.
    #[inline]
    pub(super) fn search_value<'g, Q>(&self, key: &Q, guard: &'g Guard) -> Option<&'g V>
    where
        K: 'g,
        Q: Comparable<K> + ?Sized,
    {
        loop {
            let (child, metadata) = self.children.min_greater_equal(key);
            if let Some(child) = child {
                if let Some(child) = child.load(Acquire, guard).as_ref() {
                    if self.children.validate(metadata) {
                        // Data race resolution - see `LeafNode::search_entry`.
                        return child.search_value(key, guard);
                    }
                }
            } else {
                let unbounded_ptr = self.unbounded_child.load(Acquire, guard);
                if let Some(unbounded) = unbounded_ptr.as_ref() {
                    if self.children.validate(metadata) {
                        return unbounded.search_value(key, guard);
                    }
                } else {
                    return None;
                }
            }
        }
    }

    /// Returns the minimum key entry in the entire tree.
    #[inline]
    pub(super) fn min<'g>(&self, guard: &'g Guard) -> Option<Iter<'g, K, V>> {
        let mut unbounded_ptr = self.unbounded_child.load(Acquire, guard);
        while let Some(unbounded) = unbounded_ptr.as_ref() {
            let mut iter = ArrayIter::new(&self.children);
            for i in iter.by_ref() {
                let child_ptr = self.children.val(i).load(Acquire, guard);
                if let Some(child) = child_ptr.as_ref() {
                    if let Some(iter) = child.min(guard) {
                        return Some(iter);
                    }
                }
            }
            if let Some(iter) = unbounded.min(guard) {
                return Some(iter);
            }
            // `post_remove` may be replacing the retired unbounded child with an existing child.
            let new_ptr = self.unbounded_child.load(Acquire, guard);
            if unbounded_ptr == new_ptr && self.children.validate(iter.metadata()) {
                // All the children are empty or retired.
                break;
            }
            unbounded_ptr = new_ptr;
        }

        None
    }

    /// Returns a [`RevIter`] pointing to the right-most leaf in the entire tree.
    #[inline]
    pub(super) fn max<'g>(&self, guard: &'g Guard) -> Option<RevIter<'g, K, V>> {
        let mut unbounded_ptr = self.unbounded_child.load(Acquire, guard);
        while let Some(unbounded) = unbounded_ptr.as_ref() {
            let mut rev_iter = ArrayRevIter::new(&self.children);
            if let Some(iter) = unbounded.max(guard) {
                return Some(iter);
            }
            // `post_remove` may be replacing the retired unbounded child with an existing child.
            for i in rev_iter.by_ref() {
                let child_ptr = self.children.val(i).load(Acquire, guard);
                if let Some(child) = child_ptr.as_ref() {
                    if let Some(iter) = child.max(guard) {
                        return Some(iter);
                    }
                }
            }
            let new_ptr = self.unbounded_child.load(Acquire, guard);
            if unbounded_ptr == new_ptr && self.children.validate(rev_iter.metadata()) {
                // All the children are empty or retired.
                break;
            }
            unbounded_ptr = new_ptr;
        }

        None
    }

    /// Returns a [`Iter`] pointing to an entry that is close enough to the specified key.
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
        loop {
            // Firstly, try to find a key in the optimal child.
            let (child, metadata) = self.children.min_greater_equal(key);
            if let Some(child) = child {
                if let Some(child) = child.load(Acquire, guard).as_ref() {
                    if self.children.validate(metadata) {
                        if let Some(iter) = child.approximate::<_, LE>(key, guard) {
                            return Some(iter);
                        }
                    } else {
                        // It is not a hot loop - see `LeafNode::search_entry`.
                        continue;
                    }
                } else {
                    // It is not a hot loop - see `LeafNode::search_entry`.
                    continue;
                }
            }

            // Secondly, check the unbounded child.
            let unbounded_ptr = self.unbounded_child.load(Acquire, guard);
            if let Some(unbounded) = unbounded_ptr.as_ref() {
                if self.children.validate(metadata) {
                    if let Some(iter) = unbounded.approximate::<_, LE>(key, guard) {
                        return Some(iter);
                    }
                } else {
                    continue;
                }
            } else {
                // Retired.
                return None;
            }

            // Lastly, try to find a key in any child.
            for i in ArrayIter::new(&self.children) {
                let child_ptr = self.children.val(i).load(Acquire, guard);
                if let Some(child) = child_ptr.as_ref() {
                    if let Some(iter) = child.approximate::<_, LE>(key, guard) {
                        return Some(iter);
                    }
                }
            }

            if unbounded_ptr == self.unbounded_child.load(Acquire, guard)
                && self.children.validate(metadata)
            {
                // All the children are empty or retired.
                return None;
            }
        }
    }

    /// Inserts a key-value pair.
    #[inline]
    pub(super) fn insert<P: LockPager>(
        &self,
        mut key: K,
        mut val: V,
        pager: &mut P,
        guard: &Guard,
    ) -> Result<InsertResult<K, V>, (K, V)> {
        loop {
            let (child, metadata) = self.children.min_greater_equal(&key);
            if let Some(child) = child {
                let child_ptr = child.load(Acquire, guard);
                if let Some(child_ref) = child_ptr.as_ref() {
                    if self.children.validate(metadata) {
                        // Data race resolution - see `LeafNode::search_entry`.
                        let insert_result = child_ref.insert(key, val, pager, guard)?;
                        match insert_result {
                            InsertResult::Success
                            | InsertResult::Duplicate(..)
                            | InsertResult::Frozen(..) => return Ok(insert_result),
                            InsertResult::Full(k, v) => {
                                match self.split_node(child_ptr, child, pager, guard) {
                                    Ok(true) => {
                                        key = k;
                                        val = v;
                                        continue;
                                    }
                                    Ok(false) => return Ok(InsertResult::Full(k, v)),
                                    Err(()) => return Err((k, v)),
                                }
                            }
                        };
                    }
                }
                // It is not a hot loop - see `LeafNode::search_entry`.
                continue;
            }

            let unbounded_ptr = self.unbounded_child.load(Acquire, guard);
            if let Some(unbounded) = unbounded_ptr.as_ref() {
                if !self.children.validate(metadata) {
                    continue;
                }
                let insert_result = unbounded.insert(key, val, pager, guard)?;
                match insert_result {
                    InsertResult::Success
                    | InsertResult::Duplicate(..)
                    | InsertResult::Frozen(..) => return Ok(insert_result),
                    InsertResult::Full(k, v) => {
                        match self.split_node(unbounded_ptr, &self.unbounded_child, pager, guard) {
                            Ok(true) => {
                                key = k;
                                val = v;
                                continue;
                            }
                            Ok(false) => return Ok(InsertResult::Full(k, v)),
                            Err(()) => return Err((k, v)),
                        }
                    }
                };
            }
            return Ok(InsertResult::Full(key, val));
        }
    }

    /// Removes an entry associated with the given key.
    ///
    /// # Errors
    ///
    /// Returns an error if a retry is required.
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
        loop {
            let (child, metadata) = self.children.min_greater_equal(key);
            if let Some(child) = child {
                let child_ptr = child.load(Acquire, guard);
                if let Some(child) = child_ptr.as_ref() {
                    if self.children.validate(metadata) {
                        // Data race resolution - see `LeafNode::search_entry`.
                        let result = child.remove_if::<_, _, _>(key, condition, pager, guard)?;
                        if result == RemoveResult::Retired {
                            return Ok(self.post_remove(None, guard).1);
                        }
                        return Ok(result);
                    }
                }
                // It is not a hot loop - see `LeafNode::search_entry`.
                continue;
            }
            let unbounded_ptr = self.unbounded_child.load(Acquire, guard);
            if let Some(unbounded) = unbounded_ptr.as_ref() {
                if !self.children.validate(metadata) {
                    // Data race resolution - see `LeafNode::search_entry`.
                    continue;
                }
                let result = unbounded.remove_if::<_, _, _>(key, condition, pager, guard)?;
                if result == RemoveResult::Retired {
                    return Ok(self.post_remove(None, guard).1);
                }
                return Ok(result);
            }
            return Ok(RemoveResult::Fail);
        }
    }

    /// Removes a range of entries.
    ///
    /// Returns the number of remaining children.
    #[allow(clippy::too_many_lines)]
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
        debug_assert!(valid_lower_max_leaf.is_none() || start_unbounded);
        debug_assert!(valid_lower_max_leaf.is_none() || valid_upper_min_node.is_none());

        let locker = if pager.try_acquire(&self.lock)? {
            Locker {
                internal_node: self,
            }
        } else {
            // The internal node was retired: retry.
            return Err(());
        };

        let (Some(_locker), _) = self.post_remove(Some(locker), guard) else {
            // The locker was consumed, meaning that the node was just retired.
            return Err(());
        };

        let mut current_state = RemoveRangeState::Below;
        let mut num_children = 1;
        let mut lower_border = None;
        let mut upper_border = None;

        for pos in ArrayIter::new(&self.children) {
            let key = self.children.key(pos);
            let node = self.children.val(pos);
            current_state = current_state.next(key, range, start_unbounded);
            match current_state {
                RemoveRangeState::Below => {
                    num_children += 1;
                }
                RemoveRangeState::MaybeBelow => {
                    debug_assert!(!start_unbounded);
                    num_children += 1;
                    lower_border.replace((Some(pos), node));
                }
                RemoveRangeState::FullyContained => {
                    // There can be another thread inserting keys into the node, and this may
                    // render those concurrent operations completely ineffective.
                    self.children
                        .remove_unchecked(self.children.metadata(), pos);
                    node.swap((None, Tag::None), AcqRel);
                }
                RemoveRangeState::MaybeAbove => {
                    if valid_upper_min_node.is_some() {
                        // `valid_upper_min_node` is not in this sub-tree.
                        self.children
                            .remove_unchecked(self.children.metadata(), pos);
                        node.swap((None, Tag::None), AcqRel);
                    } else {
                        num_children += 1;
                        upper_border.replace(node);
                    }
                    break;
                }
            }
        }

        // Now, examine the unbounded child.
        match current_state {
            RemoveRangeState::Below => {
                // The unbounded child is the only child, or all the children are below the range.
                debug_assert!(lower_border.is_none() && upper_border.is_none());
                if valid_upper_min_node.is_some() {
                    lower_border.replace((None, &self.unbounded_child));
                } else {
                    upper_border.replace(&self.unbounded_child);
                }
            }
            RemoveRangeState::MaybeBelow => {
                debug_assert!(!start_unbounded);
                debug_assert!(lower_border.is_some() && upper_border.is_none());
                upper_border.replace(&self.unbounded_child);
            }
            RemoveRangeState::FullyContained => {
                debug_assert!(upper_border.is_none());
                upper_border.replace(&self.unbounded_child);
            }
            RemoveRangeState::MaybeAbove => {
                debug_assert!(upper_border.is_some());
            }
        }

        if let Some(lower_leaf) = valid_lower_max_leaf {
            // It is currently in the middle of a recursive call: pass `lower_leaf` to connect leaves.
            debug_assert!(start_unbounded && lower_border.is_none() && upper_border.is_some());
            if let Some(upper_node) = upper_border.and_then(|n| n.load(Acquire, guard).as_ref()) {
                upper_node.remove_range(range, true, Some(lower_leaf), None, pager, guard)?;
            }
        } else if let Some(upper_node) = valid_upper_min_node {
            // Pass `upper_node` to the lower leaf to connect leaves, so that this method can be
            // recursively invoked on `upper_node`.
            debug_assert!(lower_border.is_some());
            if let Some((Some(pos), lower_node)) = lower_border {
                self.children
                    .remove_unchecked(self.children.metadata(), pos);
                self.unbounded_child
                    .swap((lower_node.get_shared(Acquire, guard), Tag::None), AcqRel);
                lower_node.swap((None, Tag::None), Release);
            }
            if let Some(lower_node) = self.unbounded_child.load(Acquire, guard).as_ref() {
                lower_node.remove_range(
                    range,
                    start_unbounded,
                    None,
                    Some(upper_node),
                    pager,
                    guard,
                )?;
            }
        } else {
            let lower_node = lower_border.and_then(|n| n.1.load(Acquire, guard).as_ref());
            let upper_node = upper_border.and_then(|n| n.load(Acquire, guard).as_ref());
            match (lower_node, upper_node) {
                (_, None) => (),
                (None, Some(upper_node)) => {
                    upper_node.remove_range(range, start_unbounded, None, None, pager, guard)?;
                }
                (Some(lower_node), Some(upper_node)) => {
                    debug_assert!(!ptr::eq(lower_node, upper_node));
                    lower_node.remove_range(
                        range,
                        start_unbounded,
                        None,
                        Some(upper_node),
                        pager,
                        guard,
                    )?;
                }
            }
        }

        Ok(num_children)
    }

    /// Splits a full node.
    ///
    /// Returns `false` if the parent node needs to be split.
    ///
    /// # Errors
    ///
    /// Returns an error if locking failed or the full internal node was changed.
    #[allow(clippy::too_many_lines)]
    pub(super) fn split_node<P: LockPager>(
        &self,
        full_node_ptr: Ptr<Node<K, V>>,
        full_node: &AtomicShared<Node<K, V>>,
        pager: &mut P,
        guard: &Guard,
    ) -> Result<bool, ()> {
        if self.is_retired() {
            // Let the parent node clean up this node.
            return Ok(false);
        }
        let Some(locker) = Locker::try_lock(self) else {
            // Do not wait-and-acquire the lock as it is most likely that the node has already been
            // split by the time it acquires the lock.
            if pager.try_wait(&self.lock) {
                return Ok(true);
            }
            return Err(());
        };

        if full_node_ptr != full_node.load(Relaxed, guard) {
            return Err(());
        }

        let target = full_node_ptr.as_ref().unwrap();
        let is_full = self.children.is_full();
        match target {
            Node::Internal(target) => {
                let target_locker = if pager.try_acquire(&target.lock)? {
                    Locker {
                        internal_node: target,
                    }
                } else {
                    // The target node was retired.
                    if self.post_remove(Some(locker), guard).1 == RemoveResult::Success {
                        return Ok(true);
                    }
                    return Err(());
                };

                let mut low_key_node = None;
                let mut high_key_node = None;
                let mut boundary_key = None;
                let mut i = 0;
                if !target.children.distribute(
                    |boundary, len| {
                        // E.g., `boundary == 3, len == 3 (+ unbounded)`, then `low_i` can be as
                        // large as `2`, and the unbounded child goes to `high_key_node`.
                        if (boundary as usize) < (len + 1) && is_full {
                            return false;
                        }
                        true
                    },
                    |k, v, pos, boundary| {
                        if v.load(Acquire, guard)
                            .as_ref()
                            .is_some_and(Node::is_retired)
                        {
                            // See `post_remove` for this operation ordering.
                            let result = target
                                .children
                                .remove_unchecked(target.children.metadata(), pos);
                            debug_assert_ne!(result, RemoveResult::Fail);
                            v.swap((None, Tag::None), Release);
                            return;
                        } else if i < boundary {
                            let low_key_internal_node = low_key_node.get_or_insert_with(|| {
                                let node = Shared::new_with(Node::new_internal_node);
                                node.freeze();
                                node
                            });
                            let Node::Internal(low_key_internal_node) =
                                low_key_internal_node.as_ref()
                            else {
                                return;
                            };
                            if i == boundary - 1 {
                                // Need to adjust the reference count of  `v` before `target` is frozen.
                                debug_assert!(!is_full);
                                low_key_internal_node
                                    .unbounded_child
                                    .swap((v.get_shared(Acquire, guard), Tag::None), Relaxed);
                                boundary_key.replace(k.clone());
                            } else {
                                low_key_internal_node.children.insert_unchecked(
                                    unsafe { ptr::from_ref(k).read() },
                                    unsafe { ptr::from_ref(v).read() },
                                    i,
                                );
                            }
                        } else {
                            debug_assert!(!is_full);
                            let high_key_node = high_key_node.get_or_insert_with(|| {
                                let node = Shared::new_with(Node::new_internal_node);
                                node.freeze();
                                node
                            });
                            let Node::Internal(high_key_internal_node) = high_key_node.as_ref()
                            else {
                                return;
                            };
                            high_key_internal_node.children.insert_unchecked(
                                unsafe { ptr::from_ref(k).read() },
                                unsafe { ptr::from_ref(v).read() },
                                i - boundary,
                            );
                        }
                        i += 1;
                    },
                ) {
                    return Ok(false);
                }

                let low_key_node = low_key_node.unwrap_or_else(|| {
                    let node = Shared::new_with(Node::new_internal_node);
                    node.freeze();
                    node
                });
                let Node::Internal(low_key_internal_node) = low_key_node.as_ref() else {
                    // Technically unreachable.
                    return Err(());
                };

                if low_key_internal_node.unbounded_child.is_null(Relaxed) {
                    // `low_key_internal_node` now owns the children.
                    let unfrozen = low_key_internal_node.children.unfreeze();
                    debug_assert!(unfrozen);
                    debug_assert!(high_key_node.is_none());

                    // The target will be replaced with `low_key_node`.
                    low_key_internal_node.unbounded_child.swap(
                        (target.unbounded_child.get_shared(Acquire, guard), Tag::None),
                        Relaxed,
                    );
                    full_node.swap((Some(low_key_node), Tag::None), AcqRel);
                } else {
                    let high_key_node = high_key_node.unwrap_or_else(|| {
                        let node = Shared::new_with(Node::new_internal_node);
                        node.freeze();
                        node
                    });
                    let (Node::Internal(high_key_internal_node), Some(key)) =
                        (high_key_node.as_ref(), boundary_key)
                    else {
                        // Technically unreachable.
                        return Err(());
                    };

                    // New nodes now own the children.
                    let unfrozen_low = low_key_internal_node.children.unfreeze();
                    let unfrozen_high = high_key_internal_node.children.unfreeze();
                    debug_assert!(unfrozen_low && unfrozen_high);

                    high_key_internal_node.unbounded_child.swap(
                        (target.unbounded_child.get_shared(Acquire, guard), Tag::None),
                        Relaxed,
                    );

                    // Adjust the reference count of `low_key_internal_node.unbounded_child`.
                    let unbounded_child =
                        unsafe { ptr::from_ref(&low_key_internal_node.unbounded_child).read() };
                    drop(unbounded_child);

                    let result = self.children.insert(key, AtomicShared::from(low_key_node));
                    debug_assert!(matches!(result, InsertResult::Success));
                    full_node.swap((Some(high_key_node), Tag::None), Release);
                }

                // Ownership of entries has been transferred to the new internal nodes.
                let frozen = target.children.freeze();
                debug_assert!(frozen);

                target_locker.unlock_retire();
            }
            Node::Leaf(target) => {
                let target_locker = if pager.try_acquire(&target.lock)? {
                    LeafNodeLocker { leaf_node: target }
                } else {
                    // The target node was retired.
                    if self.post_remove(Some(locker), guard).1 == RemoveResult::Success {
                        return Ok(true);
                    }
                    return Err(());
                };

                let mut low_key_node = None;
                let mut high_key_node = None;
                let mut boundary_key = None;
                let mut i = 0;
                if !target.children.distribute(
                    |boundary, len| {
                        // E.g., `boundary == 3, len == 3 (+ unbounded)`, then `low_i` can be as
                        // large as `2`, and the unbounded child goes to `high_key_node`.
                        if (boundary as usize) < (len + 1) && is_full {
                            return false;
                        }
                        true
                    },
                    |k, v, _, boundary| {
                        if i < boundary {
                            let low_key_node = low_key_node.get_or_insert_with(|| {
                                let node = Shared::new_with(Node::new_leaf_node);
                                node.freeze();
                                node
                            });
                            let Node::Leaf(low_key_leaf_node) = low_key_node.as_ref() else {
                                return;
                            };
                            if i == boundary - 1 {
                                // Need to adjust the reference count of  `v` before `target` is frozen.
                                debug_assert!(!is_full);
                                low_key_leaf_node
                                    .unbounded_child
                                    .swap((v.get_shared(Acquire, guard), Tag::None), Relaxed);
                                boundary_key.replace(k.clone());
                            } else {
                                low_key_leaf_node.children.insert_unchecked(
                                    unsafe { ptr::from_ref(k).read() },
                                    unsafe { ptr::from_ref(v).read() },
                                    i,
                                );
                            }
                        } else {
                            debug_assert!(!is_full);
                            let high_key_node = high_key_node.get_or_insert_with(|| {
                                let node = Shared::new_with(Node::new_leaf_node);
                                node.freeze();
                                node
                            });
                            let Node::Leaf(high_key_leaf_node) = high_key_node.as_ref() else {
                                return;
                            };
                            high_key_leaf_node.children.insert_unchecked(
                                unsafe { ptr::from_ref(k).read() },
                                unsafe { ptr::from_ref(v).read() },
                                i - boundary,
                            );
                        }
                        i += 1;
                    },
                ) {
                    return Ok(false);
                }

                let low_key_node = low_key_node.unwrap_or_else(|| {
                    let node = Shared::new_with(Node::new_leaf_node);
                    node.freeze();
                    node
                });
                let Node::Leaf(low_key_leaf_node) = low_key_node.as_ref() else {
                    // Technically unreachable.
                    return Err(());
                };

                if low_key_leaf_node.unbounded_child.is_null(Relaxed) {
                    // `low_key_leaf_node` now owns the children.
                    let unfrozen = low_key_leaf_node.children.unfreeze();
                    debug_assert!(unfrozen);
                    debug_assert!(high_key_node.is_none());

                    // The target will be replaced with `low_key_node`.
                    low_key_leaf_node.unbounded_child.swap(
                        (target.unbounded_child.get_shared(Acquire, guard), Tag::None),
                        Relaxed,
                    );
                    full_node.swap((Some(low_key_node), Tag::None), AcqRel);
                } else {
                    let high_key_node = high_key_node.unwrap_or_else(|| {
                        let node = Shared::new_with(Node::new_leaf_node);
                        node.freeze();
                        node
                    });
                    let (Node::Leaf(high_key_leaf_node), Some(key)) =
                        (high_key_node.as_ref(), boundary_key)
                    else {
                        // Technically unreachable.
                        return Err(());
                    };

                    // New nodes now own the children.
                    let unfrozen_low = low_key_leaf_node.children.unfreeze();
                    let unfrozen_high = high_key_leaf_node.children.unfreeze();
                    debug_assert!(unfrozen_low && unfrozen_high);

                    high_key_leaf_node.unbounded_child.swap(
                        (target.unbounded_child.get_shared(Acquire, guard), Tag::None),
                        Relaxed,
                    );

                    // Adjust the reference count of `low_key_leaf_node.unbounded_child`.
                    let unbounded_child =
                        unsafe { ptr::from_ref(&low_key_leaf_node.unbounded_child).read() };
                    drop(unbounded_child);

                    let result = self.children.insert(key, AtomicShared::from(low_key_node));
                    debug_assert!(matches!(result, InsertResult::Success));
                    full_node.swap((Some(high_key_node), Tag::None), Release);
                }

                // Ownership of entries has been transferred to the new leaf nodes.
                let frozen = target.children.freeze();
                debug_assert!(frozen);

                target_locker.unlock_retire();
            }
        }

        Ok(true)
    }

    /// Tries to delete retired nodes after a successful removal of an entry.
    fn post_remove<'n>(
        &'n self,
        locker: Option<Locker<'n, K, V>>,
        guard: &Guard,
    ) -> (Option<Locker<'n, K, V>>, RemoveResult) {
        let Some(lock) = locker.or_else(|| Locker::try_lock(self)) else {
            if self.is_retired() {
                return (None, RemoveResult::Retired);
            }
            return (None, RemoveResult::Success);
        };

        let mut max_key_entry = None;
        for pos in ArrayIter::new(&self.children) {
            let node = self.children.val(pos);
            let node_ptr = node.load(Acquire, guard);
            let node_ref = node_ptr.as_ref().unwrap();
            if node_ref.is_retired() {
                let result = self
                    .children
                    .remove_unchecked(self.children.metadata(), pos);
                debug_assert_ne!(result, RemoveResult::Fail);

                // Once the key is removed, it is safe to deallocate the node as the validation
                // loop ensures the absence of readers.
                node.swap((None, Tag::None), Release);
            } else {
                max_key_entry.replace((node, pos));
            }
        }

        // The unbounded node is replaced with the maximum key node if retired.
        let unbounded_ptr = self.unbounded_child.load(Acquire, guard);
        let fully_empty = if let Some(unbounded) = unbounded_ptr.as_ref() {
            if unbounded.is_retired() {
                if let Some((max_key_child, pos)) = max_key_entry {
                    self.unbounded_child.swap(
                        (max_key_child.get_shared(Relaxed, guard), Tag::None),
                        Release,
                    );
                    self.children
                        .remove_unchecked(self.children.metadata(), pos);
                    max_key_child.swap((None, Tag::None), Release);
                    false
                } else {
                    // `Tag::First` prevents `insert` from allocating a new node.
                    if let Some(obsolete_node) =
                        self.unbounded_child.swap((None, Tag::First), Release).0
                    {
                        debug_assert!(obsolete_node.is_retired());
                        let _: bool = obsolete_node.release();
                    }
                    true
                }
            } else {
                false
            }
        } else {
            true
        };

        if fully_empty {
            lock.unlock_retire();
            (None, RemoveResult::Retired)
        } else {
            (Some(lock), RemoveResult::Success)
        }
    }
}

impl<K, V> fmt::Debug for InternalNode<K, V>
where
    K: 'static + Clone + fmt::Debug + Ord,
    V: 'static + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let guard = Guard::new();
        f.write_str("InternalNode { ")?;
        write!(f, "retired: {}, ", self.is_retired())?;
        self.children.for_each(|i, rank, entry, removed| {
            if let Some((k, l)) = entry {
                if let Some(l) = l.load(Acquire, &guard).as_ref() {
                    write!(f, "{i}: ({k:?}, {rank}, removed: {removed}, {l:?}), ")?;
                } else {
                    write!(f, "{i}: ({k:?}, {rank}, removed: {removed}, null), ")?;
                }
            }
            Ok(())
        })?;
        if let Some(unbounded) = self.unbounded_child.load(Acquire, &guard).as_ref() {
            write!(f, "unbounded: {unbounded:?}")?;
        } else {
            write!(f, "unbounded: null")?;
        }
        f.write_str(" }")
    }
}

impl<'n, K, V> Locker<'n, K, V> {
    /// Acquires exclusive lock on the [`InternalNode`].
    #[inline]
    pub(super) fn try_lock(internal_node: &'n InternalNode<K, V>) -> Option<Locker<'n, K, V>> {
        if internal_node.lock.try_lock() {
            Some(Locker { internal_node })
        } else {
            None
        }
    }

    /// Retires the leaf node by poisoning the lock.
    #[inline]
    pub(super) fn unlock_retire(self) {
        self.internal_node.lock.poison_lock();
        forget(self);
    }
}

impl<K, V> Drop for Locker<'_, K, V> {
    #[inline]
    fn drop(&mut self) {
        self.internal_node.lock.release_lock();
    }
}

#[cfg(not(feature = "loom"))]
#[cfg(test)]
mod test {
    use super::*;
    use std::sync::atomic::AtomicBool;
    use tokio::sync::Barrier;

    fn new_level_3_node() -> InternalNode<usize, usize> {
        InternalNode {
            unbounded_child: AtomicShared::new(Node::Internal(InternalNode {
                unbounded_child: AtomicShared::new(Node::new_leaf_node()),
                children: Array::new(),
                lock: Lock::default(),
            })),
            children: Array::new(),
            lock: Lock::default(),
        }
    }

    #[test]
    fn bulk() {
        let internal_node = new_level_3_node();
        let guard = Guard::new();
        assert_eq!(internal_node.depth(1, &guard), 3);

        let data_size = if cfg!(miri) { 256 } else { 8192 };
        for k in 0..data_size {
            match internal_node.insert(k, k, &mut (), &guard) {
                Ok(result) => match result {
                    InsertResult::Success => {
                        assert_eq!(internal_node.search_entry(&k, &guard), Some((&k, &k)));
                    }
                    InsertResult::Duplicate(..) | InsertResult::Frozen(..) => unreachable!(),
                    InsertResult::Full(_, _) => {
                        for j in 0..k {
                            assert_eq!(internal_node.search_entry(&j, &guard), Some((&j, &j)));
                            if j == k - 1 {
                                assert!(matches!(
                                    internal_node.remove_if::<_, _, _>(
                                        &j,
                                        &mut |_| true,
                                        &mut (),
                                        &guard
                                    ),
                                    Ok(RemoveResult::Retired)
                                ));
                            } else {
                                assert!(
                                    internal_node
                                        .remove_if::<_, _, _>(&j, &mut |_| true, &mut (), &guard)
                                        .is_ok(),
                                );
                            }
                            assert_eq!(internal_node.search_entry(&j, &guard), None);
                        }
                        break;
                    }
                },
                Err((k, v)) => {
                    let result = internal_node.insert(k, v, &mut (), &guard);
                    assert!(result.is_ok());
                    assert_eq!(internal_node.search_entry(&k, &guard), Some((&k, &k)));
                }
            }
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test(flavor = "multi_thread", worker_threads = 16)]
    async fn parallel() {
        let num_tasks = 8;
        let workload_size = 64;
        let barrier = Shared::new(Barrier::new(num_tasks));
        for _ in 0..64 {
            let internal_node = Shared::new(new_level_3_node());
            assert!(
                internal_node
                    .insert(usize::MAX, usize::MAX, &mut (), &Guard::new())
                    .is_ok()
            );
            let mut task_handles = Vec::with_capacity(num_tasks);
            for task_id in 0..num_tasks {
                let barrier_clone = barrier.clone();
                let internal_node_clone = internal_node.clone();
                task_handles.push(tokio::task::spawn(async move {
                    barrier_clone.wait().await;
                    let guard = Guard::new();
                    let mut max_key = None;
                    let range = (task_id * workload_size)..((task_id + 1) * workload_size);
                    for id in range.clone() {
                        loop {
                            if let Ok(r) = internal_node_clone.insert(id, id, &mut (), &guard) {
                                match r {
                                    InsertResult::Success => {
                                        match internal_node_clone.insert(id, id, &mut (), &guard) {
                                            Ok(InsertResult::Duplicate(..)) | Err(_) => (),
                                            _ => unreachable!(),
                                        }
                                        break;
                                    }
                                    InsertResult::Full(..) => {
                                        max_key.replace(id);
                                        break;
                                    }
                                    InsertResult::Frozen(..) => (),
                                    InsertResult::Duplicate(..) => unreachable!(),
                                }
                            }
                        }
                        if max_key.is_some() {
                            break;
                        }
                    }
                    for id in range.clone() {
                        if max_key == Some(id) {
                            break;
                        }
                        assert_eq!(
                            internal_node_clone.search_entry(&id, &guard),
                            Some((&id, &id))
                        );
                    }
                    for id in range {
                        if max_key == Some(id) {
                            break;
                        }
                        loop {
                            if let Ok(r) = internal_node_clone.remove_if::<_, _, _>(
                                &id,
                                &mut |_| true,
                                &mut (),
                                &guard,
                            ) {
                                match r {
                                    RemoveResult::Success | RemoveResult::Fail => break,
                                    RemoveResult::Frozen | RemoveResult::Retired => unreachable!(),
                                }
                            }
                        }
                        assert!(internal_node_clone.search_entry(&id, &guard).is_none());
                        if let Ok(RemoveResult::Success) = internal_node_clone.remove_if::<_, _, _>(
                            &id,
                            &mut |_| true,
                            &mut (),
                            &guard,
                        ) {
                            unreachable!()
                        }
                    }
                }));
            }

            for r in futures::future::join_all(task_handles).await {
                assert!(r.is_ok());
            }
            assert!(
                internal_node
                    .remove_if::<_, _, _>(&usize::MAX, &mut |_| true, &mut (), &Guard::new())
                    .is_ok()
            );
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test(flavor = "multi_thread", worker_threads = 16)]
    async fn durability() {
        let num_tasks = 8_usize;
        let num_iterations = 64;
        let workload_size = 64_usize;
        for k in 0..64 {
            let fixed_point = k * 16;
            for _ in 0..=num_iterations {
                let barrier = Shared::new(Barrier::new(num_tasks));
                let internal_node = Shared::new(new_level_3_node());
                let inserted: Shared<AtomicBool> = Shared::new(AtomicBool::new(false));
                let mut task_handles = Vec::with_capacity(num_tasks);
                for _ in 0..num_tasks {
                    let barrier_clone = barrier.clone();
                    let internal_node_clone = internal_node.clone();
                    let inserted_clone = inserted.clone();
                    task_handles.push(tokio::spawn(async move {
                        {
                            barrier_clone.wait().await;
                            let guard = Guard::new();
                            if let Ok(InsertResult::Success) = internal_node_clone.insert(
                                fixed_point,
                                fixed_point,
                                &mut (),
                                &guard,
                            ) {
                                assert!(!inserted_clone.swap(true, Relaxed));
                            }
                            assert_eq!(
                                internal_node_clone
                                    .search_entry(&fixed_point, &guard)
                                    .unwrap(),
                                (&fixed_point, &fixed_point)
                            );
                        }
                        {
                            barrier_clone.wait().await;
                            let guard = Guard::new();
                            for i in 0..workload_size {
                                if i != fixed_point {
                                    let result = internal_node_clone.insert(i, i, &mut (), &guard);
                                    drop(result);
                                }
                                assert_eq!(
                                    internal_node_clone
                                        .search_entry(&fixed_point, &guard)
                                        .unwrap(),
                                    (&fixed_point, &fixed_point)
                                );
                            }
                            for i in 0..workload_size {
                                let max_iter = internal_node_clone
                                    .approximate::<_, true>(&fixed_point, &guard)
                                    .unwrap();
                                assert!(*max_iter.get().unwrap().0 <= fixed_point);
                                let mut min_iter = internal_node_clone.min(&guard).unwrap();
                                if let Some((f, v)) = min_iter.next() {
                                    assert_eq!(*f, *v);
                                    assert!(*f <= fixed_point);
                                } else {
                                    let (f, v) = min_iter.jump(&guard).unwrap();
                                    assert_eq!(*f, *v);
                                    assert!(*f <= fixed_point);
                                }
                                let _result = internal_node_clone.remove_if::<_, _, _>(
                                    &i,
                                    &mut |v| *v != fixed_point,
                                    &mut (),
                                    &guard,
                                );
                                assert_eq!(
                                    internal_node_clone
                                        .search_entry(&fixed_point, &guard)
                                        .unwrap(),
                                    (&fixed_point, &fixed_point)
                                );
                            }
                        }
                    }));
                }
                for r in futures::future::join_all(task_handles).await {
                    assert!(r.is_ok());
                }
                assert!((*inserted).load(Relaxed));
            }
        }
    }
}
