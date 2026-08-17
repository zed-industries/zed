use std::cmp::Ordering::{Equal, Greater, Less};
use std::mem::forget;
use std::ops::{Bound, RangeBounds};
use std::sync::atomic::Ordering::{AcqRel, Acquire, Relaxed, Release};
use std::{fmt, ptr};

use saa::Lock;
use sdd::{AtomicShared, Guard, Ptr, Shared, Tag};

use super::Leaf;
use super::leaf::{Array, ArrayIter, InsertResult, Iter, RemoveResult, RevIter, range_contains};
use super::node::Node;
use crate::Comparable;
use crate::async_helper::LockPager;
use crate::exit_guard::ExitGuard;

/// [`LeafNode`] contains a list of instances of `K, V` [`Leaf`].
///
/// The layout of a leaf node: `|ptr(entry array)/max(child keys)|...|ptr(entry array)|`
pub struct LeafNode<K, V> {
    /// A child [`Leaf`] that has no upper key bound.
    ///
    /// It stores the maximum key in the node, and key-value pairs are first pushed to this
    /// [`Leaf`] until it splits.
    pub(super) unbounded_child: AtomicShared<Leaf<K, V>>,
    /// Children of the [`LeafNode`].
    pub(super) children: Array<K, AtomicShared<Leaf<K, V>>>,
    /// [`Lock`] to protect the [`LeafNode`].
    pub(super) lock: Lock,
}

/// [`Locker`] holds exclusive ownership of a [`LeafNode`].
pub(super) struct Locker<'n, K, V> {
    pub(super) leaf_node: &'n LeafNode<K, V>,
}

/// A state machine to keep track of the progress of a bulk removal operation.
#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) enum RemoveRangeState {
    /// The maximum key of the node is less than the start bound of the range.
    Below,
    /// The maximum key of the node is contained in the range, but it is not clear whether the
    /// minimum key of the node is contained in the range.
    MaybeBelow,
    /// The maximum key and the minimum key of the node are contained in the range.
    FullyContained,
    /// The maximum key of the node is not contained in the range, but it is not clear whether
    /// the minimum key of the node is contained in the range.
    MaybeAbove,
}

impl<K, V> LeafNode<K, V> {
    /// Creates a new empty [`LeafNode`].
    #[inline]
    pub(super) fn new() -> LeafNode<K, V> {
        LeafNode {
            unbounded_child: AtomicShared::null(),
            children: Array::new(),
            lock: Lock::default(),
        }
    }

    /// Returns `true` if the [`LeafNode`] has retired.
    #[inline]
    pub(super) fn is_retired(&self) -> bool {
        self.lock.is_poisoned(Acquire)
    }
}

impl<K, V> LeafNode<K, V>
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
                        // Data race with split.
                        //  - Writer: start to insert an intermediate low key leaf.
                        //  - Reader: read the metadata not including the intermediate low key leaf.
                        //  - Writer: insert the intermediate low key leaf.
                        //  - Writer: replace the high key leaf pointer.
                        //  - Reader: read the new high key leaf pointer
                        // Consequently, the reader may miss keys in the low key leaf.
                        //
                        // Resolution: metadata validation.
                        return child.search_entry(key);
                    }
                }

                // The child leaf must have been just removed.
                //
                // The `LeafNode` metadata is updated before a leaf is removed. This implies that
                // the new `metadata` will be different from the current `metadata`.
            } else {
                let unbounded_ptr = self.unbounded_child.load(Acquire, guard);
                if let Some(unbounded) = unbounded_ptr.as_ref() {
                    if self.children.validate(metadata) {
                        return unbounded.search_entry(key);
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
                        return child.search_val(key);
                    }
                }
                // Data race resolution - see `LeafNode::search_entry`.
            } else {
                let unbounded_ptr = self.unbounded_child.load(Acquire, guard);
                if let Some(unbounded) = unbounded_ptr.as_ref() {
                    if self.children.validate(metadata) {
                        return unbounded.search_val(key);
                    }
                } else {
                    return None;
                }
            }
        }
    }

    /// Returns an [`Iter`] pointing to the left-most leaf in the entire tree.
    #[inline]
    pub(super) fn min<'g>(&self, guard: &'g Guard) -> Option<Iter<'g, K, V>> {
        let mut min_leaf = None;
        for i in ArrayIter::new(&self.children) {
            let child_ptr = self.children.val(i).load(Acquire, guard);
            if let Some(child) = child_ptr.as_ref() {
                min_leaf.replace(child);
                break;
            }
        }
        if min_leaf.is_none() {
            let unbounded_ptr = self.unbounded_child.load(Acquire, guard);
            if let Some(unbounded) = unbounded_ptr.as_ref() {
                min_leaf.replace(unbounded);
            }
        }

        let Some(min_leaf) = min_leaf else {
            // `unbounded_child` being null means that the leaf was retired of empty.
            return None;
        };

        let mut rev_iter = RevIter::new(min_leaf);
        while rev_iter.jump(guard).is_some() {}
        rev_iter.rewind();
        Some(rev_iter.rev())
    }

    /// Returns a [`RevIter`] pointing to the right-most leaf in the entire tree.
    #[inline]
    pub(super) fn max<'g>(&self, guard: &'g Guard) -> Option<RevIter<'g, K, V>> {
        let unbounded_ptr = self.unbounded_child.load(Acquire, guard);
        if let Some(unbounded) = unbounded_ptr.as_ref() {
            let mut iter = Iter::new(unbounded);
            while iter.jump(guard).is_some() {}
            iter.rewind();
            return Some(iter.rev());
        }
        // `unbounded_child` being null means that the leaf was retired of empty.
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
        let leaf = loop {
            let (child, metadata) = self.children.min_greater_equal(key);
            if let Some(child) = child {
                if self.children.validate(metadata) {
                    if let Some(child) = child.load(Acquire, guard).as_ref() {
                        break child;
                    }
                }
                // It is not a hot loop - see `LeafNode::search_entry`.
                continue;
            }
            if let Some(unbounded) = self.unbounded_child.load(Acquire, guard).as_ref() {
                if self.children.validate(metadata) {
                    break unbounded;
                }
                continue;
            }
            // `unbounded_child` being null means that the leaf was retired of empty.
            return None;
        };

        // Tries to find "any" leaf that contains a reachable entry.
        let origin = Iter::new(leaf);
        let mut iter = origin.clone();
        if iter.next().is_none() && iter.jump(guard).is_none() {
            let mut rev_iter = origin.rev();
            if rev_iter.jump(guard).is_some() {
                iter = rev_iter.rev();
            } else {
                return None;
            }
        }
        iter.rewind();

        if LE {
            while let Some((k, _)) = iter.next() {
                if let Equal | Greater = key.compare(k) {
                    return Some(iter);
                }
                // Go to the prev leaf node that shall contain smaller keys.
                let mut rev_iter = iter.rev();
                rev_iter.jump(guard)?;
                iter = rev_iter.rev();
                // Rewind the iterator to point to the smallest key in the leaf.
                iter.rewind();
            }
        } else {
            let mut rev_iter = iter.rev();
            while let Some((k, _)) = rev_iter.next() {
                if let Less | Equal = key.compare(k) {
                    return Some(rev_iter.rev());
                }
                // Go to the next leaf node that shall contain larger keys.
                iter = rev_iter.rev();
                iter.jump(guard)?;
                rev_iter = iter.rev();
                // Rewind the iterator to point to the largest key in the leaf.
                rev_iter.rewind();
            }
        }

        // Reached the end of the linked list.
        None
    }

    /// Inserts a key-value pair.
    ///
    /// # Errors
    ///
    /// Returns an error if a retry is required.
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
                        let insert_result = child_ref.insert(key, val);
                        match insert_result {
                            InsertResult::Success | InsertResult::Duplicate(..) => {
                                return Ok(insert_result);
                            }
                            InsertResult::Full(k, v) => {
                                match self.split_leaf(child_ptr, child, pager, guard) {
                                    Ok(true) => {
                                        key = k;
                                        val = v;
                                        continue;
                                    }
                                    Ok(false) => return Ok(InsertResult::Full(k, v)),
                                    Err(()) => return Err((k, v)),
                                }
                            }
                            InsertResult::Frozen(k, v) => {
                                // The `Leaf` is being split: retry.
                                if pager.try_wait(&self.lock) && !self.children.validate(metadata) {
                                    // There was a change in the children, worth in-place retrying.
                                    key = k;
                                    val = v;
                                    continue;
                                }
                                return Err((k, v));
                            }
                        };
                    }
                }
                // It is not a hot loop - see `LeafNode::search_entry`.
                continue;
            }

            let mut unbounded_ptr = self.unbounded_child.load(Acquire, guard);
            if unbounded_ptr.is_null() {
                match self.unbounded_child.compare_exchange(
                    Ptr::null(),
                    (Some(Shared::new_with(Leaf::new)), Tag::None),
                    AcqRel,
                    Acquire,
                    guard,
                ) {
                    Ok((_, ptr)) => {
                        unbounded_ptr = ptr;
                    }
                    Err((_, actual)) => {
                        unbounded_ptr = actual;
                    }
                }
            }
            if let Some(unbounded) = unbounded_ptr.as_ref() {
                if !self.children.validate(metadata) {
                    continue;
                }
                let insert_result = unbounded.insert(key, val);
                match insert_result {
                    InsertResult::Success | InsertResult::Duplicate(..) => {
                        return Ok(insert_result);
                    }
                    InsertResult::Full(k, v) => {
                        match self.split_leaf(unbounded_ptr, &self.unbounded_child, pager, guard) {
                            Ok(true) => {
                                key = k;
                                val = v;
                                continue;
                            }
                            Ok(false) => return Ok(InsertResult::Full(k, v)),
                            Err(()) => return Err((k, v)),
                        }
                    }
                    InsertResult::Frozen(k, v) => {
                        if pager.try_wait(&self.lock) && !self.children.validate(metadata) {
                            // See above.
                            key = k;
                            val = v;
                            continue;
                        }
                        return Err((k, v));
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
    pub(super) fn remove_if<Q, F: FnMut(&V) -> bool, P: LockPager>(
        &self,
        key: &Q,
        condition: &mut F,
        pager: &mut P,
        guard: &Guard,
    ) -> Result<RemoveResult, ()>
    where
        Q: Comparable<K> + ?Sized,
    {
        loop {
            let (child, metadata) = self.children.min_greater_equal(key);
            if let Some(child) = child {
                let child_ptr = child.load(Acquire, guard);
                if let Some(child) = child_ptr.as_ref() {
                    if self.children.validate(metadata) {
                        // Data race resolution - see `LeafNode::search_entry`.
                        let result = child.remove_if(key, condition);
                        if result == RemoveResult::Frozen {
                            // Its entries may be being relocated.
                            if pager.try_wait(&self.lock) && !self.children.validate(metadata) {
                                continue;
                            }
                            return Err(());
                        } else if result == RemoveResult::Retired {
                            return Ok(self.post_remove(guard));
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
                let result = unbounded.remove_if(key, condition);
                if result == RemoveResult::Frozen {
                    if pager.try_wait(&self.lock) && !self.children.validate(metadata) {
                        continue;
                    }
                    return Err(());
                } else if result == RemoveResult::Retired {
                    return Ok(self.post_remove(guard));
                }
                return Ok(result);
            }
            return Ok(RemoveResult::Fail);
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
        debug_assert!(valid_lower_max_leaf.is_none() || start_unbounded);
        debug_assert!(valid_lower_max_leaf.is_none() || valid_upper_min_node.is_none());

        let _locker = if pager.try_acquire(&self.lock)? {
            Locker { leaf_node: self }
        } else {
            // The leaf node was retired: retry.
            return Err(());
        };

        let mut current_state = RemoveRangeState::Below;
        let mut num_leaves = 1;
        let mut min_max_leaf = None;

        for i in ArrayIter::new(&self.children) {
            current_state = current_state.next(self.children.key(i), range, start_unbounded);
            let child = self.children.val(i);
            match current_state {
                RemoveRangeState::Below | RemoveRangeState::MaybeBelow => {
                    if let Some(leaf) = child.load(Acquire, guard).as_ref() {
                        leaf.remove_range(range);
                    }
                    num_leaves += 1;
                    if min_max_leaf.is_none() {
                        min_max_leaf.replace(child);
                    }
                }
                RemoveRangeState::FullyContained => {
                    if let Some(leaf) = child.swap((None, Tag::None), AcqRel).0 {
                        leaf.unlink(guard);
                    }
                    // There can be another thread inserting keys into the leaf, and this may render
                    // those operations completely ineffective.
                    self.children.remove_unchecked(self.children.metadata(), i);
                }
                RemoveRangeState::MaybeAbove => {
                    if let Some(leaf) = child.load(Acquire, guard).as_ref() {
                        leaf.remove_range(range);
                    }
                    num_leaves += 1;
                    if min_max_leaf.is_none() {
                        min_max_leaf.replace(child);
                    }
                    break;
                }
            }
        }

        if let Some(unbounded) = self.unbounded_child.load(Acquire, guard).as_ref() {
            unbounded.remove_range(range);
        }

        if let Some(valid_lower_max_leaf) = valid_lower_max_leaf {
            // Splices the max min leaf with the min max leaf.
            let min_max = min_max_leaf
                .unwrap_or(&self.unbounded_child)
                .load(Acquire, guard);
            Leaf::<K, V>::splice_link(Some(valid_lower_max_leaf), min_max.as_ref(), guard);
        } else if let Some(valid_upper_min_node) = valid_upper_min_node {
            // Connect the unbounded child with the minimum valid leaf in the node.
            valid_upper_min_node.remove_range(
                range,
                true,
                self.unbounded_child.load(Acquire, guard).as_ref(),
                None,
                pager,
                guard,
            )?;
        } else if start_unbounded {
            // `min_max_leaf` becomes the first leaf in the entire tree.
            let min_max = min_max_leaf
                .unwrap_or(&self.unbounded_child)
                .load(Acquire, guard);
            Leaf::<K, V>::splice_link(None, min_max.as_ref(), guard);
        }

        Ok(num_leaves)
    }

    /// Splits a full leaf.
    ///
    /// Returns `false` if the parent node needs to be split.
    ///
    /// # Errors
    ///
    /// Returns an error if locking failed or the full leaf node was changed.
    #[allow(clippy::too_many_lines)]
    fn split_leaf<P: LockPager>(
        &self,
        full_leaf_ptr: Ptr<Leaf<K, V>>,
        full_leaf: &AtomicShared<Leaf<K, V>>,
        pager: &mut P,
        guard: &Guard,
    ) -> Result<bool, ()> {
        if self.is_retired() {
            // Let the parent node clean up this node.
            return Ok(false);
        }
        let Some(_locker) = Locker::try_lock(self) else {
            // Do not wait-and-acquire the lock as it is most likely that the leaf has already been
            // split by the time it acquires the lock.
            if pager.try_wait(&self.lock) {
                return Ok(true);
            }
            return Err(());
        };

        if full_leaf_ptr != full_leaf.load(Relaxed, guard) {
            // The leaf node was already split.
            return Err(());
        }

        let is_full = self.children.is_full();
        let target = full_leaf_ptr.as_ref().unwrap();

        // The metadata of `target` should be frozen for stable distribution of entries.
        let frozen = target.freeze();
        debug_assert!(frozen);
        let exit_guard = ExitGuard::new((), |()| {
            target.unfreeze();
        });

        let mut low_key_leaf = None;
        let mut high_key_leaf = None;
        let mut i = 0;
        if !target.distribute(
            |boundary, len| {
                // E.g., `boundary == 2, len == 2`, then `i` can be as large as `1`: `high_key_leaf`
                // is not needed.
                if (boundary as usize) < len && is_full {
                    // No space for new leaves.
                    return false;
                }
                true
            },
            |k, v, _, boundary| {
                // `v` is moved, not cloned; those new leaves do not own them until unfrozen.
                if i < boundary {
                    let low_key_leaf = low_key_leaf.get_or_insert_with(|| {
                        let new_leaf = Shared::new_with(Leaf::new);
                        new_leaf.freeze();
                        new_leaf
                    });
                    low_key_leaf.insert_unchecked(
                        unsafe { ptr::from_ref(k).read() },
                        unsafe { ptr::from_ref(v).read() },
                        i,
                    );
                } else {
                    let high_key_leaf = high_key_leaf.get_or_insert_with(|| {
                        let new_leaf = Shared::new_with(Leaf::new);
                        new_leaf.freeze();
                        new_leaf
                    });
                    high_key_leaf.insert_unchecked(
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

        let low_key_leaf = low_key_leaf.unwrap_or_else(|| {
            // `target` was empty.
            let new_leaf = Shared::new_with(Leaf::new);
            new_leaf.freeze();
            new_leaf
        });

        // Data race with iterators if the following code is executed without new leaves locked.
        // - T1 and T2 both observe, L1 -> L2.
        // - T2 splits L1 into L1_1 and L1_2: L1_1 <-> L1_2 (not reachable via tree) <-> L2.
        // - T1 splits L2 into L2_1 and L2_2: L1_1 <-> L1_2 <-> L2_1 <-> L2_2.
        // - T1 inserts entries into L2_1.
        // - T1 range queries get L1, instead of L1_2.
        // - T1 iterates over entries from L1 and L2, and cannot see entries in L2_1.
        //
        // The locking prevents T1 from splitting L2 until L1_2 becomes reachable via tree.
        low_key_leaf.lock.lock_sync();
        let low_key_leaf_lock = &low_key_leaf.get_guarded_ref(guard).lock;

        if let Some(high_key_leaf) = high_key_leaf {
            let low_key_max = low_key_leaf.max_key().unwrap().clone();

            // Unfreeze the leaves; those leaves now take ownership of the copied values.
            let unfrozen_low = low_key_leaf.unfreeze();
            let unfrozen_high = high_key_leaf.unfreeze();
            debug_assert!(unfrozen_low && unfrozen_high);

            low_key_leaf
                .next
                .store(high_key_leaf.as_ptr().cast_mut(), Relaxed);
            high_key_leaf
                .prev
                .store(low_key_leaf.as_ptr().cast_mut(), Relaxed);
            let high_key_leaf_lock = &high_key_leaf.get_guarded_ref(guard).lock;
            high_key_leaf_lock.lock_sync();

            target.replace_link(
                |prev_next, next_prev, prev_ptr, next_ptr| {
                    low_key_leaf.prev.store(prev_ptr.cast_mut(), Relaxed);
                    high_key_leaf.next.store(next_ptr.cast_mut(), Relaxed);
                    if let Some(prev_next) = prev_next {
                        prev_next.store(low_key_leaf.as_ptr().cast_mut(), Release);
                    }
                    if let Some(next_prev) = next_prev {
                        next_prev.store(high_key_leaf.as_ptr().cast_mut(), Release);
                    }
                    // From here, `Iter` can reach the new leaf.
                },
                guard,
            );

            // Take the max key value stored in the low key leaf as the leaf key.
            let result = self
                .children
                .insert(low_key_max, AtomicShared::from(low_key_leaf));
            debug_assert!(matches!(result, InsertResult::Success));
            let released = low_key_leaf_lock.release_lock();
            debug_assert!(released);

            full_leaf.swap((Some(high_key_leaf), Tag::None), Release);
            let released = high_key_leaf_lock.release_lock();
            debug_assert!(released);
        } else {
            // Unfreeze the leaf; it now takes ownership of the copied values.
            let unfrozen = low_key_leaf.unfreeze();
            debug_assert!(unfrozen);

            target.replace_link(
                |prev_next, next_prev, prev_ptr, next_ptr| {
                    low_key_leaf.prev.store(prev_ptr.cast_mut(), Relaxed);
                    low_key_leaf.next.store(next_ptr.cast_mut(), Relaxed);
                    if let Some(prev_next) = prev_next {
                        prev_next.store(low_key_leaf.as_ptr().cast_mut(), Release);
                    }
                    if let Some(next_prev) = next_prev {
                        next_prev.store(low_key_leaf.as_ptr().cast_mut(), Release);
                    }
                    // From here, `Iter` can reach the new leaf.
                },
                guard,
            );

            full_leaf.swap((Some(low_key_leaf), Tag::None), Release);
            let released = low_key_leaf_lock.release_lock();
            debug_assert!(released);
        }

        // The removed leaf stays frozen: ownership of the copied values is transferred.
        exit_guard.forget();

        Ok(true)
    }

    /// Tries to delete retired leaves after a successful removal of an entry.
    fn post_remove(&self, guard: &Guard) -> RemoveResult {
        let Some(lock) = Locker::try_lock(self) else {
            if self.is_retired() {
                return RemoveResult::Retired;
            }
            return RemoveResult::Success;
        };

        let mut prev_valid_leaf = None;
        for i in ArrayIter::new(&self.children) {
            let child = self.children.val(i);
            let leaf_ptr = child.load(Acquire, guard);
            let leaf = leaf_ptr.as_ref().unwrap();
            if leaf.is_retired() {
                leaf.unlink(guard);

                // As soon as the leaf is removed from the leaf node, the next leaf can store keys
                // that are smaller than those that were previously stored in the removed leaf node.
                //
                // Iterators cope with this by checking the prev/next pointers; right after
                // `unlink`, the prev/next leaves will not point to this leaf anymore.
                let result = self.children.remove_unchecked(self.children.metadata(), i);
                debug_assert_ne!(result, RemoveResult::Fail);

                // The pointer is set to null after the metadata of `self.children` is updated
                // to enable readers to retry when they find it being null.
                child.swap((None, Tag::None), Release);
            } else {
                prev_valid_leaf.replace(leaf);
            }
        }

        // The unbounded leaf becomes unreachable after all the other leaves are gone.
        let fully_empty = if prev_valid_leaf.is_some() {
            false
        } else {
            let unbounded_ptr = self.unbounded_child.load(Acquire, guard);
            if let Some(unbounded) = unbounded_ptr.as_ref() {
                if unbounded.is_retired() {
                    unbounded.unlink(guard);

                    // `Tag::First` prevents `insert` from allocating a new leaf.
                    self.unbounded_child.swap((None, Tag::First), Release);
                    true
                } else {
                    false
                }
            } else {
                true
            }
        };

        if fully_empty {
            lock.unlock_retire();
            RemoveResult::Retired
        } else {
            RemoveResult::Success
        }
    }
}

impl<K, V> fmt::Debug for LeafNode<K, V>
where
    K: 'static + Clone + fmt::Debug + Ord,
    V: 'static + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let guard = Guard::new();
        f.write_str("LeafNode { ")?;
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
    /// Acquires exclusive lock on the [`LeafNode`].
    #[inline]
    pub(super) fn try_lock(leaf_node: &'n LeafNode<K, V>) -> Option<Locker<'n, K, V>> {
        if leaf_node.lock.try_lock() {
            Some(Locker { leaf_node })
        } else {
            None
        }
    }

    /// Retires the leaf node by poisoning the lock.
    #[inline]
    pub(super) fn unlock_retire(self) {
        self.leaf_node.lock.poison_lock();
        forget(self);
    }
}

impl<K, V> Drop for Locker<'_, K, V> {
    #[inline]
    fn drop(&mut self) {
        self.leaf_node.lock.release_lock();
    }
}

impl RemoveRangeState {
    /// Returns the next state.
    pub(super) fn next<K, Q, R: RangeBounds<Q>>(
        self,
        key: &K,
        range: &R,
        start_unbounded: bool,
    ) -> Self
    where
        Q: Comparable<K> + ?Sized,
    {
        if range_contains(range, key) {
            match self {
                RemoveRangeState::Below => {
                    if start_unbounded {
                        RemoveRangeState::FullyContained
                    } else {
                        RemoveRangeState::MaybeBelow
                    }
                }
                RemoveRangeState::MaybeBelow | RemoveRangeState::FullyContained => {
                    RemoveRangeState::FullyContained
                }
                RemoveRangeState::MaybeAbove => unreachable!(),
            }
        } else {
            match self {
                RemoveRangeState::Below => match range.start_bound() {
                    Bound::Included(k) => match k.compare(key) {
                        Less | Equal => RemoveRangeState::MaybeAbove,
                        Greater => RemoveRangeState::Below,
                    },
                    Bound::Excluded(k) => match k.compare(key) {
                        Less => RemoveRangeState::MaybeAbove,
                        Greater | Equal => RemoveRangeState::Below,
                    },
                    Bound::Unbounded => RemoveRangeState::MaybeAbove,
                },
                RemoveRangeState::MaybeBelow | RemoveRangeState::FullyContained => {
                    RemoveRangeState::MaybeAbove
                }
                RemoveRangeState::MaybeAbove => unreachable!(),
            }
        }
    }
}

#[cfg(not(feature = "loom"))]
#[cfg(test)]
mod test {
    use super::*;
    use std::sync::atomic::AtomicBool;
    use tokio::sync::Barrier;

    #[test]
    fn basic() {
        let guard = Guard::new();
        let leaf_node: LeafNode<String, String> = LeafNode::new();
        assert!(matches!(
            leaf_node.insert(
                "MY GOODNESS!".to_owned(),
                "OH MY GOD!!".to_owned(),
                &mut (),
                &guard
            ),
            Ok(InsertResult::Success)
        ));
        assert!(matches!(
            leaf_node.insert(
                "GOOD DAY".to_owned(),
                "OH MY GOD!!".to_owned(),
                &mut (),
                &guard
            ),
            Ok(InsertResult::Success)
        ));
        assert_eq!(
            leaf_node.search_entry("MY GOODNESS!", &guard).unwrap().1,
            "OH MY GOD!!"
        );
        assert_eq!(
            leaf_node.search_entry("GOOD DAY", &guard).unwrap().1,
            "OH MY GOD!!"
        );
        assert!(matches!(
            leaf_node.remove_if::<_, _, _>("GOOD DAY", &mut |v| v == "OH MY", &mut (), &guard),
            Ok(RemoveResult::Fail)
        ));
        assert!(matches!(
            leaf_node.remove_if::<_, _, _>(
                "GOOD DAY",
                &mut |v| v == "OH MY GOD!!",
                &mut (),
                &guard
            ),
            Ok(RemoveResult::Success)
        ));
        assert!(matches!(
            leaf_node.remove_if::<_, _, _>("GOOD", &mut |v| v == "OH MY", &mut (), &guard),
            Ok(RemoveResult::Fail)
        ));
        assert!(matches!(
            leaf_node.remove_if::<_, _, _>("MY GOODNESS!", &mut |_| true, &mut (), &guard),
            Ok(RemoveResult::Retired)
        ));
        assert!(matches!(
            leaf_node.insert("HI".to_owned(), "HO".to_owned(), &mut (), &guard),
            Ok(InsertResult::Full(..))
        ));
    }

    #[test]
    fn bulk() {
        let guard = Guard::new();
        let leaf_node: LeafNode<usize, usize> = LeafNode::new();
        for k in 0..1024 {
            let mut result = leaf_node.insert(k, k, &mut (), &guard);
            if result.is_err() {
                result = leaf_node.insert(k, k, &mut (), &guard);
            }
            match result.unwrap() {
                InsertResult::Success => {
                    assert_eq!(leaf_node.search_entry(&k, &guard), Some((&k, &k)));
                }
                InsertResult::Duplicate(..) | InsertResult::Frozen(..) => unreachable!(),
                InsertResult::Full(_, _) => {
                    for r in 0..(k - 1) {
                        assert_eq!(leaf_node.search_entry(&r, &guard), Some((&r, &r)));
                        assert!(
                            leaf_node
                                .remove_if::<_, _, _>(&r, &mut |_| true, &mut (), &guard)
                                .is_ok()
                        );
                        assert_eq!(leaf_node.search_entry(&r, &guard), None);
                    }
                    assert_eq!(
                        leaf_node.search_entry(&(k - 1), &guard),
                        Some((&(k - 1), &(k - 1)))
                    );
                    assert_eq!(
                        leaf_node.remove_if::<_, _, _>(&(k - 1), &mut |_| true, &mut (), &guard),
                        Ok(RemoveResult::Retired)
                    );
                    assert_eq!(leaf_node.search_entry(&(k - 1), &guard), None);
                    break;
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
        for _ in 0..16 {
            let leaf_node = Shared::new(LeafNode::new());
            assert!(
                leaf_node
                    .insert(usize::MAX, usize::MAX, &mut (), &Guard::new())
                    .is_ok()
            );
            let mut task_handles = Vec::with_capacity(num_tasks);
            for task_id in 0..num_tasks {
                let barrier_clone = barrier.clone();
                let leaf_node_clone = leaf_node.clone();
                task_handles.push(tokio::task::spawn(async move {
                    barrier_clone.wait().await;
                    let guard = Guard::new();
                    let mut max_key = None;
                    let range = (task_id * workload_size)..((task_id + 1) * workload_size);
                    for id in range.clone() {
                        loop {
                            if let Ok(r) = leaf_node_clone.insert(id, id, &mut (), &guard) {
                                match r {
                                    InsertResult::Success => {
                                        match leaf_node_clone.insert(id, id, &mut (), &guard) {
                                            Ok(InsertResult::Duplicate(..)) | Err(_) => (),
                                            _ => unreachable!(),
                                        }
                                        break;
                                    }
                                    InsertResult::Full(..) => {
                                        max_key.replace(id);
                                        break;
                                    }
                                    InsertResult::Duplicate(..) | InsertResult::Frozen(..) => {
                                        unreachable!()
                                    }
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
                        assert_eq!(leaf_node_clone.search_entry(&id, &guard), Some((&id, &id)));
                    }
                    for id in range {
                        if max_key == Some(id) {
                            break;
                        }
                        loop {
                            if let Ok(r) = leaf_node_clone.remove_if::<_, _, _>(
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
                        assert!(
                            leaf_node_clone.search_entry(&id, &guard).is_none(),
                            "{}",
                            id
                        );
                        if let Ok(RemoveResult::Success) = leaf_node_clone.remove_if::<_, _, _>(
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
                leaf_node
                    .remove_if::<_, _, _>(&usize::MAX, &mut |_| true, &mut (), &Guard::new())
                    .is_ok()
            );
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test(flavor = "multi_thread", worker_threads = 16)]
    async fn durability() {
        let num_tasks = 8_usize;
        let workload_size = 64_usize;
        for _ in 0..16 {
            for k in 0..=workload_size {
                let barrier = Shared::new(Barrier::new(num_tasks));
                let leaf_node: Shared<LeafNode<usize, usize>> = Shared::new(LeafNode::new());
                let inserted: Shared<AtomicBool> = Shared::new(AtomicBool::new(false));
                let mut task_handles = Vec::with_capacity(num_tasks);
                for _ in 0..num_tasks {
                    let barrier_clone = barrier.clone();
                    let leaf_node_clone = leaf_node.clone();
                    let inserted_clone = inserted.clone();
                    task_handles.push(tokio::spawn(async move {
                        {
                            barrier_clone.wait().await;
                            let guard = Guard::new();
                            if let Ok(InsertResult::Success) =
                                leaf_node_clone.insert(k, k, &mut (), &guard)
                            {
                                assert!(!inserted_clone.swap(true, Relaxed));
                            }
                        }
                        {
                            barrier_clone.wait().await;
                            let guard = Guard::new();
                            for i in 0..workload_size {
                                if i != k {
                                    let result = leaf_node_clone.insert(i, i, &mut (), &guard);
                                    drop(result);
                                }
                                assert_eq!(
                                    leaf_node_clone.search_entry(&k, &guard).unwrap(),
                                    (&k, &k)
                                );
                            }
                            for i in 0..workload_size {
                                let max_iter =
                                    leaf_node_clone.approximate::<_, true>(&k, &guard).unwrap();
                                assert!(*max_iter.get().unwrap().0 <= k);
                                let mut min_iter = leaf_node_clone.min(&guard).unwrap();
                                if let Some((k_ref, v_ref)) = min_iter.next() {
                                    assert_eq!(*k_ref, *v_ref);
                                    assert!(*k_ref <= k);
                                } else {
                                    let (k_ref, v_ref) = min_iter.jump(&guard).unwrap();
                                    assert_eq!(*k_ref, *v_ref);
                                    assert!(*k_ref <= k);
                                }
                                let _result = leaf_node_clone.remove_if::<_, _, _>(
                                    &i,
                                    &mut |v| *v != k,
                                    &mut (),
                                    &guard,
                                );
                                assert_eq!(
                                    leaf_node_clone.search_entry(&k, &guard).unwrap(),
                                    (&k, &k)
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
