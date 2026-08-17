use std::cell::UnsafeCell;
use std::cmp::Ordering;
use std::fmt;
use std::mem::{MaybeUninit, needs_drop};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::ops::{Deref, RangeBounds};
use std::ptr;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Relaxed, Release};
#[cfg(not(feature = "loom"))]
use std::sync::atomic::{AtomicPtr, AtomicUsize};

use saa::Lock;
use sdd::Guard;

use crate::Comparable;
#[cfg(feature = "loom")]
use loom::sync::atomic::{AtomicPtr, AtomicUsize};

/// [`Leaf`] is an ordered array of key-value pairs.
///
/// A constructed key-value pair entry is never dropped until the entire [`Leaf`] instance is
/// dropped.
pub struct Leaf<K, V> {
    /// Pointer to the previous [`Leaf`].
    pub(super) prev: AtomicPtr<Leaf<K, V>>,
    /// Pointer to the next [`Leaf`].
    pub(super) next: AtomicPtr<Leaf<K, V>>,
    /// [`Array`] containing the key-value pairs.
    array: Array<K, V>,
    /// Lock to protect the linked list.
    pub(super) lock: Lock,
}

pub struct Array<K, V> {
    /// Metadata for entry and array states.
    ///
    /// The state of each entry is as follows.
    /// * `0`: `uninit`.
    /// * `1 - array_size`: `rank`.
    /// * `max`: `removed`.
    ///
    /// The entry state transitions as follows.
    /// * `uninit -> removed -> rank -> removed`.
    metadata: AtomicUsize,
    /// Entry array.
    entry_array: UnsafeCell<EntryArray<K, V>>,
}

/// The number of entries and number of state bits per entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Dimension {
    /// Number of entries in an [`Array`].
    pub num_entries: u8,
    /// Number of bits required per entry metadata.
    pub num_bits_per_entry: u8,
}

/// Insertion result.
pub enum InsertResult<K, V> {
    /// Insertion succeeded.
    Success,
    /// Duplicate key found.
    Duplicate(K, V),
    /// No vacant slot for the key.
    Full(K, V),
    /// The [`Leaf`] is frozen.
    ///
    /// This is not a terminal state as a frozen [`Leaf`] can be unfrozen.
    Frozen(K, V),
}

/// Remove result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RemoveResult {
    /// Remove succeeded.
    Success,
    /// Remove succeeded and cleanup required.
    Retired,
    /// Remove failed.
    Fail,
    /// The [`Leaf`] is frozen.
    Frozen,
}

/// Each constructed entry in an `EntryArray` is never dropped until the [`Leaf`] is dropped.
pub type EntryArray<K, V> = (
    [MaybeUninit<K>; DIMENSION.num_entries as usize],
    [MaybeUninit<V>; DIMENSION.num_entries as usize],
);

/// Array entry iterator.
#[derive(Debug)]
pub struct ArrayIter {
    /// Snapshot of the metadata of [`Array`].
    metadata: usize,
    /// Rank to position mapping.
    pos: [u8; DIMENSION.num_entries as usize],
    /// Current rank.
    rank: u8,
}

/// Array entry iterator, reversed.
#[derive(Debug)]
pub struct ArrayRevIter {
    /// Snapshot of the metadata of [`Array`].
    metadata: usize,
    /// Rank to position mapping.
    pos: [u8; DIMENSION.num_entries as usize],
    /// Current rank.
    rev_rank: u8,
}

/// Leaf entry iterator.
pub struct Iter<'l, K, V> {
    /// Reference to the [`Leaf`] being iterated.
    leaf: &'l Leaf<K, V>,
    /// Array entry iterator.
    array_iter: ArrayIter,
}

/// Leaf entry iterator, reversed.
pub struct RevIter<'l, K, V> {
    /// Reference to the [`Leaf`] being iterated.
    leaf: &'l Leaf<K, V>,
    /// Array entry reverse iterator.
    array_rev_iter: ArrayRevIter,
}

/// Emulates `RangeBounds::contains`.
#[inline]
pub(crate) fn range_contains<K, Q, R: RangeBounds<Q>>(range: &R, key: &K) -> bool
where
    Q: Comparable<K> + ?Sized,
{
    (match range.start_bound() {
        Included(start) => start.compare(key).is_le(),
        Excluded(start) => start.compare(key).is_lt(),
        Unbounded => true,
    }) && (match range.end_bound() {
        Included(end) => end.compare(key).is_ge(),
        Excluded(end) => end.compare(key).is_gt(),
        Unbounded => true,
    })
}

impl<K, V> Leaf<K, V> {
    /// Creates a new empty [`Leaf`].
    #[inline]
    #[cfg(not(feature = "loom"))]
    pub(super) const fn new() -> Leaf<K, V> {
        Leaf {
            prev: AtomicPtr::new(ptr::null_mut()),
            next: AtomicPtr::new(ptr::null_mut()),
            array: Array::new(),
            lock: Lock::new(),
        }
    }

    /// Creates a new empty [`Leaf`].
    #[inline]
    #[cfg(feature = "loom")]
    pub(super) fn new() -> Leaf<K, V> {
        Leaf {
            prev: AtomicPtr::new(ptr::null_mut()),
            next: AtomicPtr::new(ptr::null_mut()),
            array: Array::new(),
            lock: Lock::new(),
        }
    }

    /// Replaces itself in the linked list with others as defined in the specified closure.
    #[inline]
    pub(super) fn replace_link<
        F: FnOnce(
            Option<&AtomicPtr<Self>>,
            Option<&AtomicPtr<Self>>,
            *const Leaf<K, V>,
            *const Leaf<K, V>,
        ),
    >(
        &self,
        f: F,
        _guard: &Guard,
    ) {
        let mut prev_ptr = self.prev.load(Acquire);
        loop {
            if let Some(prev) = unsafe { prev_ptr.as_ref() } {
                prev.lock.lock_sync();
            }
            self.lock.lock_sync();
            let prev_check = self.prev.load(Acquire);
            if prev_check == prev_ptr {
                break;
            }
            if let Some(prev) = unsafe { prev_ptr.as_ref() } {
                prev.lock.release_lock();
            }
            self.lock.release_lock();
            prev_ptr = prev_check;
        }
        let prev = unsafe { prev_ptr.as_ref() };
        let next_ptr = self.next.load(Acquire);
        let next = unsafe { next_ptr.as_ref() };
        if let Some(next_link) = next {
            next_link.lock.lock_sync();
        }

        // Check consistency before modifying the linked list, because this leaf may have been
        // deleted by `remove_range` operations and `prev` and `next` may have connected to other
        // leaves.
        if prev.is_none_or(|p| ptr::eq(p.next.load(Relaxed), self))
            && next.is_none_or(|n| ptr::eq(n.prev.load(Relaxed), self))
        {
            f(
                prev.map(|p| &p.next),
                next.map(|n| &n.prev),
                prev_ptr,
                next_ptr,
            );
        }

        if let Some(prev_link) = prev {
            let released = prev_link.lock.release_lock();
            debug_assert!(released);
        }
        let released = self.lock.release_lock();
        debug_assert!(released);
        if let Some(next_link) = next {
            let released = next_link.lock.release_lock();
            debug_assert!(released);
        }
    }

    /// Deletes itself from the linked list.
    #[inline]
    pub(super) fn unlink(&self, guard: &Guard) {
        self.replace_link(
            |prev_next, next_prev, prev_ptr, next_ptr| {
                // `self`, on the other hand, keeps its pointers.
                if let Some(prev_next) = prev_next {
                    prev_next.store(next_ptr.cast_mut(), Release);
                }
                if let Some(next_prev) = next_prev {
                    next_prev.store(prev_ptr.cast_mut(), Release);
                }
            },
            guard,
        );
    }

    /// Splices two leaves into the linked list.
    ///
    /// All the leaves between `left` and `right` are assumed to be `cleared` and bound to be
    /// unreachable.
    #[inline]
    pub(super) fn splice_link(
        left: Option<&Leaf<K, V>>,
        right: Option<&Leaf<K, V>>,
        _guard: &Guard,
    ) {
        let locked = left.is_none_or(|o| o.lock.lock_sync());
        debug_assert!(locked);
        let locked = right.is_none_or(|o| o.lock.lock_sync());
        debug_assert!(locked);

        if let Some(left) = left {
            let next = right.map_or(ptr::null(), ptr::from_ref).cast_mut();
            left.next.store(next, Release);
        }
        if let Some(right) = right {
            let prev = left.map_or(ptr::null(), ptr::from_ref).cast_mut();
            right.prev.store(prev, Release);
        }

        let released = left.is_none_or(|o| o.lock.release_lock());
        debug_assert!(released);
        let released = right.is_none_or(|o| o.lock.release_lock());
        debug_assert!(released);
    }
}

impl<K, V> fmt::Debug for Leaf<K, V>
where
    K: 'static + Clone + fmt::Debug + Ord,
    V: 'static + fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Leaf { ")?;
        let ptr: *const Self = ptr::addr_of!(*self);
        write!(f, "addr: {ptr:?}")?;
        write!(f, ", array: {:?}", &self.array)?;
        write!(f, ", prev: {:?}", self.prev.load(Relaxed))?;
        write!(f, ", next: {:?}", self.next.load(Relaxed))?;
        f.write_str(" }")
    }
}

impl<K, V> Deref for Leaf<K, V> {
    type Target = Array<K, V>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.array
    }
}

impl<K, V> Array<K, V> {
    #[cfg(not(feature = "loom"))]
    #[inline]
    pub(super) const fn new() -> Array<K, V> {
        Array {
            metadata: AtomicUsize::new(0),
            entry_array: UnsafeCell::new((
                [const { MaybeUninit::uninit() }; DIMENSION.num_entries as usize],
                [const { MaybeUninit::uninit() }; DIMENSION.num_entries as usize],
            )),
        }
    }

    #[cfg(feature = "loom")]
    #[inline]
    pub(super) fn new() -> Array<K, V> {
        Array {
            metadata: AtomicUsize::new(0),
            entry_array: UnsafeCell::new((
                [const { MaybeUninit::uninit() }; DIMENSION.num_entries as usize],
                [const { MaybeUninit::uninit() }; DIMENSION.num_entries as usize],
            )),
        }
    }

    /// Returns `true` if the [`Array`] has no reachable entry.
    #[inline]
    pub(super) fn is_empty(&self) -> bool {
        let mut mutable_metadata = self.metadata.load(Relaxed);
        let mut i = 0;
        while i != DIMENSION.num_entries {
            if mutable_metadata == 0 {
                break;
            }
            let rank = DIMENSION.rank_first(mutable_metadata);
            if rank != Dimension::uninit_rank() && rank != DIMENSION.removed_rank() {
                return false;
            }
            mutable_metadata >>= DIMENSION.num_bits_per_entry;
            i += 1;
        }
        true
    }

    /// Returns `true` if the [`Array`] is full or retired.
    ///
    /// This may return `true` even when there is a slot if there was an insertion failure.
    #[inline]
    pub(super) fn is_full(&self) -> bool {
        let metadata = self.metadata.load(Relaxed);
        let rank = DIMENSION.rank(metadata, DIMENSION.num_entries - 1);
        rank != Dimension::uninit_rank() || Dimension::is_retired(metadata)
    }

    /// Returns `true` if the [`Array`] has retired.
    #[inline]
    pub(super) fn is_retired(&self) -> bool {
        Dimension::is_retired(self.metadata.load(Acquire))
    }

    /// Returns the current metadata.
    #[inline]
    pub(super) fn metadata(&self) -> usize {
        self.metadata.load(Acquire)
    }

    /// Returns a reference to the key at the given position.
    #[inline]
    pub(super) const fn key(&self, pos: u8) -> &K {
        unsafe { &*(*self.entry_array.get()).0[pos as usize].as_ptr() }
    }

    /// Returns a reference to the key at the given position.
    #[inline]
    pub(super) const fn val(&self, pos: u8) -> &V {
        unsafe { &*(*self.entry_array.get()).1[pos as usize].as_ptr() }
    }

    /// Returns a reference to the max key.
    #[inline]
    pub(super) fn max_key(&self) -> Option<&K> {
        let mut mutable_metadata = self.metadata.load(Acquire) & (!Dimension::state_mask());
        let mut max_rank = 0;
        let mut max_pos = DIMENSION.num_entries;
        for pos in 0..DIMENSION.num_entries {
            if mutable_metadata == 0 {
                break;
            }
            let rank = DIMENSION.rank_first(mutable_metadata);
            if rank > max_rank && rank != DIMENSION.removed_rank() {
                max_rank = rank;
                max_pos = pos;
            }
            mutable_metadata >>= DIMENSION.num_bits_per_entry;
        }
        if max_pos != DIMENSION.num_entries {
            return Some(self.key(max_pos));
        }
        None
    }

    /// Inserts a key value pair at the specified position without checking the metadata when the
    /// leaf is frozen.
    ///
    /// `rank` is calculated as `pos + 1`.
    #[inline]
    pub(super) fn insert_unchecked(&self, key: K, val: V, pos: u8) {
        debug_assert!(pos < DIMENSION.num_entries);

        self.write(pos, key, val);

        let metadata = self.metadata.load(Relaxed);
        debug_assert!(Dimension::is_frozen(metadata));

        let new_metadata = DIMENSION.augment(metadata, pos, pos + 1);
        self.metadata.store(new_metadata, Release);
    }

    /// Removes the entry at the specified position without checking the metadata.
    #[inline]
    pub(super) fn remove_unchecked(&self, mut metadata: usize, pos: u8) -> RemoveResult {
        loop {
            let mut empty = true;
            let mut mutable_metadata = metadata;
            for j in 0..DIMENSION.num_entries {
                if mutable_metadata == 0 {
                    break;
                }
                if pos != j {
                    let rank = DIMENSION.rank_first(mutable_metadata);
                    if rank != Dimension::uninit_rank() && rank != DIMENSION.removed_rank() {
                        empty = false;
                        break;
                    }
                }
                mutable_metadata >>= DIMENSION.num_bits_per_entry;
            }

            let mut new_metadata = metadata | DIMENSION.rank_mask(pos);
            if empty {
                new_metadata = Dimension::retire(new_metadata);
            }
            match self
                .metadata
                .compare_exchange(metadata, new_metadata, AcqRel, Acquire)
            {
                Ok(_) => {
                    if empty {
                        return RemoveResult::Retired;
                    }
                    return RemoveResult::Success;
                }
                Err(actual) => {
                    if DIMENSION.rank(actual, pos) == DIMENSION.removed_rank() {
                        return RemoveResult::Fail;
                    }
                    if Dimension::is_frozen(actual) {
                        return RemoveResult::Frozen;
                    }
                    metadata = actual;
                }
            }
        }
    }

    /// Compares the given metadata value with the current one.
    #[inline]
    pub(super) fn validate(&self, metadata: usize) -> bool {
        // `Relaxed` is sufficient as long as the caller has load-acquired its contents.
        self.metadata.load(Relaxed) == metadata
    }

    /// Freezes the [`Array`] temporarily.
    #[inline]
    pub(super) fn freeze(&self) -> bool {
        self.metadata
            .fetch_update(AcqRel, Acquire, |p| {
                if Dimension::is_frozen(p) {
                    None
                } else {
                    Some(Dimension::freeze(p))
                }
            })
            .is_ok()
    }

    /// Unfreezes the [`Array`].
    #[inline]
    pub(super) fn unfreeze(&self) -> bool {
        self.metadata
            .fetch_update(Release, Relaxed, |p| {
                if Dimension::is_frozen(p) {
                    Some(Dimension::unfreeze(p))
                } else {
                    None
                }
            })
            .is_ok()
    }

    /// Returns the recommended number of entries that the left-side array should store when an
    /// [`Array`] is split, and the number of valid entries in the [`Array`].
    ///
    /// Returns a number in `[1, len)` that represents the recommended number of entries in
    /// the left-side node. The number is calculated as follows for each adjacent slot:
    /// - Initial `score = len`.
    /// - Rank increased: `score -= 1`.
    /// - Rank decreased: `score += 1`.
    /// - Clamp `score` in `[len / 2 + 1, len / 2 + len - 1)`.
    /// - Take `score - len / 2`.
    ///
    /// For instance, when the length of an [`Array`] is 7,
    /// - Returns 6 for `rank = [1, 2, 3, 4, 5, 6, 7]`.
    /// - Returns 1 for `rank = [7, 6, 5, 4, 3, 2, 1]`.
    #[inline]
    pub(super) fn optimal_boundary(mut mutable_metadata: usize) -> (u8, usize) {
        let mut boundary = DIMENSION.num_entries;
        let mut prev_rank = 0;
        let mut len = 0;
        for _ in 0..DIMENSION.num_entries {
            let rank = DIMENSION.rank_first(mutable_metadata);
            if rank != Dimension::uninit_rank() && rank != DIMENSION.removed_rank() {
                len += 1;
                if prev_rank >= rank {
                    boundary -= 1;
                } else if prev_rank != 0 {
                    boundary += 1;
                }
                prev_rank = rank;
            }
            mutable_metadata >>= DIMENSION.num_bits_per_entry;
        }
        (
            boundary.clamp(
                DIMENSION.num_entries / 2 + 1,
                DIMENSION.num_entries + DIMENSION.num_entries / 2 - 1,
            ) - DIMENSION.num_entries / 2,
            len,
        )
    }

    /// Distributes entries to other arrays.
    ///
    /// `dist` is a function to distribute entries to other containers where the first argument is
    /// the key, the second argument is the value, the third argument is the position, the fourth
    /// argument is the boundary, and the fifth argument is the length. Stops distribution if the
    /// function returns `false`, and this method returns `false`.
    #[inline]
    pub(super) fn distribute<P: FnOnce(u8, usize) -> bool, F: FnMut(&K, &V, u8, u8)>(
        &self,
        prepare: P,
        mut dist: F,
    ) -> bool {
        let metadata = self.metadata.load(Acquire);
        let (boundary, len) = Self::optimal_boundary(metadata);
        if !prepare(boundary, len) {
            // Do nothing if the preparation fails.
            return false;
        }
        for pos in ArrayIter::with_metadata(metadata) {
            dist(self.key(pos), self.val(pos), pos, boundary);
        }
        true
    }

    /// Writes the key and value at the given position.
    #[inline]
    const fn write(&self, pos: u8, key: K, val: V) {
        unsafe {
            (*self.entry_array.get()).0[pos as usize]
                .as_mut_ptr()
                .write(key);
            (*self.entry_array.get()).1[pos as usize]
                .as_mut_ptr()
                .write(val);
        }
    }

    /// Rolls back the insertion at the given position.
    fn rollback(&self, pos: u8) -> (K, V) {
        let (k, v) = unsafe {
            (
                (*self.entry_array.get()).0[pos as usize].as_ptr().read(),
                (*self.entry_array.get()).1[pos as usize].as_ptr().read(),
            )
        };
        self.metadata.fetch_and(!DIMENSION.rank_mask(pos), Release);
        (k, v)
    }

    /// Builds a rank to position map from metadata.
    #[inline]
    const fn build_index(metadata: usize) -> [u8; DIMENSION.num_entries as usize] {
        let mut index = [0; DIMENSION.num_entries as usize];
        let mut mutable_metadata = metadata & (!Dimension::state_mask());
        let mut i = 0;
        while i != DIMENSION.num_entries {
            if mutable_metadata == 0 {
                break;
            }
            i += 1;
            let rank = DIMENSION.rank_first(mutable_metadata);
            if rank != Dimension::uninit_rank() && rank != DIMENSION.removed_rank() {
                index[rank as usize - 1] = i;
            }
            mutable_metadata >>= DIMENSION.num_bits_per_entry;
        }
        index
    }
}

impl<K, V> Array<K, V>
where
    K: 'static + Ord,
    V: 'static,
{
    /// Inserts a key value pair.
    #[inline]
    pub(super) fn insert(&self, key: K, val: V) -> InsertResult<K, V> {
        let mut metadata = self.metadata.load(Acquire);
        'after_read_metadata: loop {
            if Dimension::is_retired(metadata) {
                return InsertResult::Full(key, val);
            } else if Dimension::is_frozen(metadata) {
                return InsertResult::Frozen(key, val);
            }

            let mut mutable_metadata = metadata;
            for pos in 0..DIMENSION.num_entries {
                let rank = DIMENSION.rank_first(mutable_metadata);
                if rank == Dimension::uninit_rank() {
                    let interim_metadata =
                        DIMENSION.augment(metadata, pos, DIMENSION.removed_rank());

                    // Reserve the slot.
                    //
                    // It doesn't have to be a release-store.
                    if let Err(actual) =
                        self.metadata
                            .compare_exchange(metadata, interim_metadata, Acquire, Acquire)
                    {
                        metadata = actual;
                        continue 'after_read_metadata;
                    }

                    self.write(pos, key, val);
                    return self.post_insert(pos, interim_metadata);
                }
                mutable_metadata >>= DIMENSION.num_bits_per_entry;
            }

            if self.search_slot(&key, metadata).is_some() {
                return InsertResult::Duplicate(key, val);
            }
            return InsertResult::Full(key, val);
        }
    }

    /// Removes the key if the condition is met.
    #[inline]
    pub(super) fn remove_if<Q, F: FnMut(&V) -> bool>(
        &self,
        key: &Q,
        condition: &mut F,
    ) -> RemoveResult
    where
        Q: Comparable<K> + ?Sized,
    {
        let metadata = self.metadata.load(Acquire);
        if Dimension::is_frozen(metadata) {
            return RemoveResult::Frozen;
        }
        let mut min_max_rank = DIMENSION.removed_rank();
        let mut max_min_rank = 0;
        let mut mutable_metadata = metadata;
        for pos in 0..DIMENSION.num_entries {
            if mutable_metadata == 0 {
                break;
            }
            let rank = DIMENSION.rank_first(mutable_metadata);
            if rank < min_max_rank && rank > max_min_rank {
                match self.compare(pos, key) {
                    Ordering::Less => {
                        if max_min_rank < rank {
                            max_min_rank = rank;
                        }
                    }
                    Ordering::Greater => {
                        if min_max_rank > rank {
                            min_max_rank = rank;
                        }
                    }
                    Ordering::Equal => {
                        // Found the key.
                        if !condition(self.val(pos)) {
                            // The given condition is not met.
                            return RemoveResult::Fail;
                        }
                        return self.remove_unchecked(metadata, pos);
                    }
                }
            }
            mutable_metadata >>= DIMENSION.num_bits_per_entry;
        }

        RemoveResult::Fail
    }

    /// Removes a range of entries.
    ///
    /// Returns the number of remaining entries.
    #[inline]
    pub(super) fn remove_range<Q, R: RangeBounds<Q>>(&self, range: &R)
    where
        Q: Comparable<K> + ?Sized,
    {
        let mut mutable_metadata = self.metadata.load(Acquire);
        for pos in 0..DIMENSION.num_entries {
            if mutable_metadata == 0 {
                break;
            }
            let rank = DIMENSION.rank_first(mutable_metadata);
            if rank != Dimension::uninit_rank() && rank != DIMENSION.removed_rank() {
                let k = self.key(pos);
                if range_contains(range, k) {
                    self.remove_if(k, &mut |_| true);
                }
            }
            mutable_metadata >>= DIMENSION.num_bits_per_entry;
        }
    }

    /// Returns an entry containing the specified key.
    #[inline]
    pub(super) fn search_entry<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        Q: Comparable<K> + ?Sized,
    {
        let metadata = self.metadata.load(Acquire);
        self.search_slot(key, metadata)
            .map(|i| (self.key(i), self.val(i)))
    }

    /// Returns the value associated with the specified key.
    #[inline]
    pub(super) fn search_val<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: Comparable<K> + ?Sized,
    {
        let metadata = self.metadata.load(Acquire);
        self.search_slot(key, metadata).map(|i| self.val(i))
    }

    /// Returns the minimum entry among those that are not `Ordering::Less` than the given key.
    ///
    /// It additionally returns the current version of its metadata so the caller can validate the
    /// correctness of the result.
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    pub(super) fn min_greater_equal<Q>(&self, key: &Q) -> (Option<&V>, usize)
    where
        Q: Comparable<K> + ?Sized,
    {
        let metadata = self.metadata.load(Acquire);
        let mut min_max_rank = DIMENSION.removed_rank();
        let mut max_min_rank = 0;
        let mut min_max_pos = DIMENSION.num_entries;
        let mut mutable_metadata = metadata;
        for pos in 0..DIMENSION.num_entries {
            if mutable_metadata == 0 {
                break;
            }
            let rank = DIMENSION.rank_first(mutable_metadata);
            if rank < min_max_rank && rank > max_min_rank {
                let k = self.key(pos);
                match key.compare(k) {
                    Ordering::Greater => {
                        if max_min_rank < rank {
                            max_min_rank = rank;
                        }
                    }
                    Ordering::Less => {
                        if min_max_rank > rank {
                            min_max_rank = rank;
                            min_max_pos = pos;
                        }
                    }
                    Ordering::Equal => {
                        return (Some(self.val(pos)), metadata);
                    }
                }
            }
            mutable_metadata >>= DIMENSION.num_bits_per_entry;
        }
        if min_max_pos != DIMENSION.num_entries {
            return (Some(self.val(min_max_pos)), metadata);
        }
        (None, metadata)
    }

    /// Iterates over initialized entries.
    pub(crate) fn for_each<E, F: FnMut(u8, u8, Option<(&K, &V)>, bool) -> Result<(), E>>(
        &self,
        mut f: F,
    ) -> Result<usize, E> {
        let metadata = self.metadata.load(Acquire);
        let mut mutable_metadata = metadata;
        for pos in 0..DIMENSION.num_entries {
            if mutable_metadata == 0 {
                break;
            }
            let rank = DIMENSION.rank_first(mutable_metadata);
            if rank == Dimension::uninit_rank() {
                f(pos, rank, None, false)?;
            } else {
                let entry = (self.key(pos), self.val(pos));
                if rank == DIMENSION.removed_rank() {
                    f(pos, rank, Some(entry), true)?;
                } else {
                    f(pos, rank, Some(entry), false)?;
                }
            }
            mutable_metadata >>= DIMENSION.num_bits_per_entry;
        }
        Ok(metadata)
    }

    /// Post-processing after reserving a free slot.
    fn post_insert(&self, free_slot_pos: u8, mut prev_metadata: usize) -> InsertResult<K, V> {
        let key = self.key(free_slot_pos);
        loop {
            let mut min_max_rank = DIMENSION.removed_rank();
            let mut max_min_rank = 0;
            let mut new_metadata = prev_metadata;
            let mut mutable_metadata = prev_metadata;
            for pos in 0..DIMENSION.num_entries {
                if mutable_metadata == 0 {
                    break;
                }
                let rank = DIMENSION.rank_first(mutable_metadata);
                if rank < min_max_rank && rank > max_min_rank {
                    match self.compare(pos, key) {
                        Ordering::Less => {
                            if max_min_rank < rank {
                                max_min_rank = rank;
                            }
                        }
                        Ordering::Greater => {
                            if min_max_rank > rank {
                                min_max_rank = rank;
                            }
                            new_metadata = DIMENSION.augment(new_metadata, pos, rank + 1);
                        }
                        Ordering::Equal => {
                            // Duplicate key.
                            let (k, v) = self.rollback(free_slot_pos);
                            return InsertResult::Duplicate(k, v);
                        }
                    }
                } else if rank != DIMENSION.removed_rank() && rank > min_max_rank {
                    new_metadata = DIMENSION.augment(new_metadata, pos, rank + 1);
                }
                mutable_metadata >>= DIMENSION.num_bits_per_entry;
            }

            // Make the newly inserted value reachable.
            let final_metadata = DIMENSION.augment(new_metadata, free_slot_pos, max_min_rank + 1);
            if let Err(actual) =
                self.metadata
                    .compare_exchange(prev_metadata, final_metadata, AcqRel, Acquire)
            {
                let frozen = Dimension::is_frozen(actual);
                let retired = Dimension::is_retired(actual);
                if frozen || retired {
                    let (k, v) = self.rollback(free_slot_pos);
                    if frozen {
                        return InsertResult::Frozen(k, v);
                    }
                    return InsertResult::Full(k, v);
                }
                prev_metadata = actual;
                continue;
            }

            return InsertResult::Success;
        }
    }

    /// Searches for a slot in which the key is stored.
    #[inline]
    fn search_slot<Q>(&self, key: &Q, mut mutable_metadata: usize) -> Option<u8>
    where
        Q: Comparable<K> + ?Sized,
    {
        let mut min_max_rank = DIMENSION.removed_rank();
        let mut max_min_rank = 0;
        for pos in 0..DIMENSION.num_entries {
            if mutable_metadata == 0 {
                break;
            }
            let rank = DIMENSION.rank_first(mutable_metadata);
            if rank < min_max_rank && rank > max_min_rank {
                match self.compare(pos, key) {
                    Ordering::Less => {
                        if max_min_rank < rank {
                            max_min_rank = rank;
                        }
                    }
                    Ordering::Greater => {
                        if min_max_rank > rank {
                            min_max_rank = rank;
                        }
                    }
                    Ordering::Equal => {
                        return Some(pos);
                    }
                }
            }
            mutable_metadata >>= DIMENSION.num_bits_per_entry;
        }
        None
    }

    #[inline]
    fn compare<Q>(&self, pos: u8, key: &Q) -> Ordering
    where
        Q: Comparable<K> + ?Sized,
    {
        key.compare(self.key(pos)).reverse()
    }
}

impl<K, V> fmt::Debug for Array<K, V>
where
    K: 'static + Clone + fmt::Debug + Ord,
    V: 'static + fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Array { ")?;
        let metadata = self.for_each(|i, rank, entry, removed| {
            if let Some(entry) = entry {
                write!(f, "{i}: ({rank}, removed: {removed}, {entry:?}), ")?;
            } else {
                write!(f, "{i}: (none), ")?;
            }
            Ok(())
        })?;
        write!(f, "frozen: {}, ", Dimension::is_frozen(metadata))?;
        write!(f, "retired: {}", Dimension::is_retired(metadata))?;
        f.write_str(" }")
    }
}

impl<K, V> Drop for Array<K, V> {
    #[inline]
    fn drop(&mut self) {
        if needs_drop::<K>() || needs_drop::<V>() {
            let metadata = self.metadata.load(Acquire);
            let is_frozen = Dimension::is_frozen(metadata);
            let mut mutable_metadata = metadata & (!Dimension::state_mask());
            for pos in 0..DIMENSION.num_entries {
                if mutable_metadata == 0 {
                    break;
                }
                let rank = DIMENSION.rank_first(mutable_metadata);
                if rank != Dimension::uninit_rank()
                    && (!is_frozen || rank == DIMENSION.removed_rank())
                {
                    if needs_drop::<K>() {
                        unsafe {
                            (*self.entry_array.get()).0[pos as usize]
                                .as_mut_ptr()
                                .drop_in_place();
                        }
                    }
                    if needs_drop::<V>() {
                        // `self` being frozen means that reachable values have copied to another
                        // array, and they should not be dropped here.
                        unsafe {
                            (*self.entry_array.get()).1[pos as usize]
                                .as_mut_ptr()
                                .drop_in_place();
                        }
                    }
                }
                mutable_metadata >>= DIMENSION.num_bits_per_entry;
            }
        }
    }
}

unsafe impl<K: Send, V: Send> Send for Array<K, V> {}
unsafe impl<K: Send + Sync, V: Send + Sync> Sync for Array<K, V> {}

impl Dimension {
    /// Flags indicating that the [`Array`] is frozen.
    const FROZEN: usize = 1_usize << (usize::BITS - 1);

    /// Flags indicating that the [`Array`] is retired.
    const RETIRED: usize = 1_usize << (usize::BITS - 2);

    /// Returns a bit mask for an array state.
    #[inline]
    const fn state_mask() -> usize {
        Self::RETIRED | Self::FROZEN
    }

    /// Returns `true` if the [`Array`] is frozen.
    #[inline]
    const fn is_frozen(metadata: usize) -> bool {
        metadata & Self::FROZEN != 0
    }

    /// Updates the metadata to represent a frozen state.
    #[inline]
    const fn freeze(metadata: usize) -> usize {
        metadata | Self::FROZEN
    }

    /// Updates the metadata to represent a non-frozen state.
    #[inline]
    const fn unfreeze(metadata: usize) -> usize {
        metadata & (!Self::FROZEN)
    }

    /// Returns `true` if the [`Leaf`] is retired.
    #[inline]
    const fn is_retired(metadata: usize) -> bool {
        metadata & Self::RETIRED != 0
    }

    /// Updates the metadata to represent a retired state.
    #[inline]
    const fn retire(metadata: usize) -> usize {
        metadata | Self::RETIRED
    }

    /// Returns a bit mask for an entry.
    #[inline]
    const fn rank_mask(self, pos: u8) -> usize {
        ((1_usize << self.num_bits_per_entry) - 1) << (pos * self.num_bits_per_entry)
    }

    /// Returns the rank of an entry.
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    const fn rank(self, metadata: usize, pos: u8) -> u8 {
        ((metadata >> (pos * self.num_bits_per_entry)) % (1_usize << self.num_bits_per_entry)) as u8
    }

    /// Returns the rank of the first entry.
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    const fn rank_first(self, metadata: usize) -> u8 {
        (metadata % (1_usize << self.num_bits_per_entry)) as u8
    }

    /// Returns the uninitialized rank value which is smaller than all the valid rank values.
    #[inline]
    const fn uninit_rank() -> u8 {
        0
    }

    /// Returns the removed rank value which is greater than all the valid rank values.
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    const fn removed_rank(self) -> u8 {
        ((1_usize << self.num_bits_per_entry) - 1) as u8
    }

    /// Augments the rank to the given metadata.
    #[inline]
    const fn augment(self, metadata: usize, pos: u8, rank: u8) -> usize {
        (metadata & (!self.rank_mask(pos))) | ((rank as usize) << (pos * self.num_bits_per_entry))
    }
}

/// The maximum number of entries and the number of metadata bits per entry in a [`Leaf`].
///
/// * `M`: The maximum number of entries.
/// * `B`: The minimum number of bits to express the state of an entry.
/// * `2`: The number of special states of an entry: uninitialized, removed.
/// * `3`: The number of special states of an [`Array`]: frozen, and retired.
/// * `U`: `usize::BITS`.
/// * `Eq1 = M + 2 <= 2^B`: `B` bits represent at least `M + 2` states.
/// * `Eq2 = B * M + 2 <= U`: `M entries + 2` special state.
/// * `Eq3 = Ceil(Log2(M + 2)) * M + 2 <= U`: derived from `Eq1` and `Eq2`.
///
/// Therefore, when `U = 64 => M = 14 / B = 4`, and `U = 32 => M = 7 / B = 4`.
pub const DIMENSION: Dimension = match usize::BITS / 8 {
    1 => Dimension {
        num_entries: 2,
        num_bits_per_entry: 2,
    },
    2 => Dimension {
        num_entries: 4,
        num_bits_per_entry: 3,
    },
    4 => Dimension {
        num_entries: 7,
        num_bits_per_entry: 4,
    },
    8 => Dimension {
        num_entries: 14,
        num_bits_per_entry: 4,
    },
    _ => Dimension {
        num_entries: 25,
        num_bits_per_entry: 5,
    },
};

impl ArrayIter {
    /// Creates a new [`ArrayIter`].
    #[inline]
    pub(super) fn new<K, V>(array: &Array<K, V>) -> ArrayIter {
        let metadata = array.metadata.load(Acquire);
        Self::with_metadata(metadata)
    }

    /// Clones the iterator.
    #[inline]
    pub(super) const fn clone(&self) -> ArrayIter {
        ArrayIter {
            pos: self.pos,
            metadata: self.metadata,
            rank: self.rank,
        }
    }

    /// Rewinds the iterator to the beginning.
    #[inline]
    pub(super) const fn rewind(&mut self) {
        self.rank = 0;
    }

    /// Converts itself into a [`ArrayRevIter`].
    #[inline]
    pub(super) const fn rev(self) -> ArrayRevIter {
        // `DIMENSION.num_entries - (self.rev_rank as usize) == (self.rank as usize) - 1`.
        #[allow(clippy::cast_possible_truncation)]
        let rev_rank = if self.rank == 0 {
            0
        } else {
            DIMENSION.num_entries + 1 - self.rank
        };
        ArrayRevIter {
            pos: self.pos,
            metadata: self.metadata,
            rev_rank,
        }
    }

    /// Returns the snapshot of leaf metadata that the [`Iter`] took.
    #[inline]
    pub(super) const fn metadata(&self) -> usize {
        self.metadata
    }

    /// Creates a new [`ArrayIter`] with the supplied metadata.
    #[inline]
    const fn with_metadata(metadata: usize) -> ArrayIter {
        let pos = Array::<(), ()>::build_index(metadata);
        ArrayIter {
            metadata,
            pos,
            rank: 0,
        }
    }
}

impl Iterator for ArrayIter {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.rank < DIMENSION.num_entries {
            self.rank += 1;
            let pos = self.pos[(self.rank as usize) - 1];
            if pos != 0 {
                return Some(pos - 1);
            }
        }
        self.rank = 0;
        None
    }
}

impl<'l, K, V> Iter<'l, K, V> {
    /// Creates a new [`Iter`].
    #[inline]
    pub(super) fn new(leaf: &'l Leaf<K, V>) -> Iter<'l, K, V> {
        Self {
            leaf,
            array_iter: ArrayIter::new(&leaf.array),
        }
    }

    /// Clones the iterator.
    #[inline]
    pub(super) const fn clone(&self) -> Iter<'l, K, V> {
        Iter {
            leaf: self.leaf,
            array_iter: self.array_iter.clone(),
        }
    }

    /// Rewinds the iterator to the beginning.
    #[inline]
    pub(super) const fn rewind(&mut self) {
        self.array_iter.rewind();
    }

    /// Converts itself into a [`RevIter`].
    #[inline]
    pub(super) const fn rev(self) -> RevIter<'l, K, V> {
        RevIter {
            leaf: self.leaf,
            array_rev_iter: self.array_iter.rev(),
        }
    }

    /// Returns a reference to the entry that the iterator is currently pointing to.
    #[inline]
    pub(super) const fn get(&self) -> Option<(&'l K, &'l V)> {
        if self.array_iter.rank == 0 {
            None
        } else {
            let pos = self.array_iter.pos[(self.array_iter.rank as usize) - 1] - 1;
            Some((self.leaf.array.key(pos), self.leaf.array.val(pos)))
        }
    }

    /// Returns a reference to the max key.
    #[inline]
    pub(super) fn max_key(&self) -> Option<&'l K> {
        for pos in self.array_iter.pos.iter().rev() {
            if *pos != 0 {
                return Some(self.leaf.key(*pos - 1));
            }
        }
        None
    }

    /// Jumps to the min entry of the next non-empty leaf.
    #[inline]
    pub(super) fn jump(&mut self, _guard: &'l Guard) -> Option<(&'l K, &'l V)>
    where
        K: Ord,
    {
        let max_key = self.max_key();
        let mut found_unlinked = false;
        loop {
            let next_leaf_ptr = self.leaf.next.load(Acquire);
            let Some(leaf) = (unsafe { next_leaf_ptr.as_ref() }) else {
                break;
            };
            let metadata = leaf.metadata.load(Acquire);
            found_unlinked |= !ptr::eq(leaf.prev.load(Relaxed), self.leaf);

            self.leaf = leaf;
            self.array_iter = ArrayIter::with_metadata(metadata);

            if found_unlinked {
                // Data race resolution:
                //  - T1:                remove(L1) -> range(L0) ->              traverse(L1)
                //  - T2: unlink(L0) ->                             delete(L0)
                //  - T3:                                                        insertSmall(L1)
                //
                // T1 must not see T3's insertion while it still needs to observe its own deletion.
                // Therefore, keys that are smaller than the max key in the current leaf should be
                // filtered out here.
                for (k, v) in self.by_ref() {
                    if max_key.is_none_or(|max| max < k) {
                        return Some((k, v));
                    }
                }
            } else if let Some((k, v)) = self.next() {
                return Some((k, v));
            }
        }
        None
    }
}

impl<K, V> fmt::Debug for Iter<'_, K, V> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Iter")
            .field("leaf", &ptr::addr_of!(*self.leaf))
            .field("prev", &self.leaf.prev.load(Relaxed))
            .field("next", &self.leaf.next.load(Relaxed))
            .field("array_iter", &self.array_iter)
            .finish()
    }
}

impl<'l, K, V> Iterator for Iter<'l, K, V> {
    type Item = (&'l K, &'l V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.array_iter
            .next()
            .map(|i| (self.leaf.key(i), self.leaf.val(i)))
    }
}

impl ArrayRevIter {
    /// Creates a new [`ArrayRevIter`].
    #[inline]
    pub(super) fn new<K, V>(array: &Array<K, V>) -> ArrayRevIter {
        let metadata = array.metadata.load(Acquire);
        Self::with_metadata(metadata)
    }

    /// Clones the iterator.
    #[inline]
    pub(super) const fn clone(&self) -> ArrayRevIter {
        ArrayRevIter {
            pos: self.pos,
            metadata: self.metadata,
            rev_rank: self.rev_rank,
        }
    }

    /// Rewinds the iterator to the beginning.
    #[inline]
    pub(super) const fn rewind(&mut self) {
        self.rev_rank = 0;
    }

    /// Converts itself into a [`ArrayIter`].
    #[inline]
    pub(super) const fn rev(self) -> ArrayIter {
        // `DIMENSION.num_entries - (self.rev_rank as usize) == (self.rank as usize) - 1`.
        #[allow(clippy::cast_possible_truncation)]
        let rank = if self.rev_rank == 0 {
            0
        } else {
            DIMENSION.num_entries + 1 - self.rev_rank
        };
        ArrayIter {
            pos: self.pos,
            metadata: self.metadata,
            rank,
        }
    }

    /// Returns the snapshot of leaf metadata that the [`ArrayRevIter`] took.
    #[inline]
    pub(super) const fn metadata(&self) -> usize {
        self.metadata
    }

    /// Creates a new [`ArrayRevIter`] with the supplied metadata.
    #[inline]
    const fn with_metadata(metadata: usize) -> ArrayRevIter {
        let pos = Array::<(), ()>::build_index(metadata);
        ArrayRevIter {
            metadata,
            pos,
            rev_rank: 0,
        }
    }
}

impl Iterator for ArrayRevIter {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.rev_rank < DIMENSION.num_entries {
            self.rev_rank += 1;
            let pos = self.pos[(DIMENSION.num_entries - self.rev_rank) as usize];
            if pos != 0 {
                return Some(pos - 1);
            }
        }
        self.rev_rank = 0;
        None
    }
}

impl<'l, K, V> RevIter<'l, K, V> {
    /// Creates a new [`RevIter`].
    #[inline]
    pub(super) fn new(leaf: &'l Leaf<K, V>) -> RevIter<'l, K, V> {
        Self {
            leaf,
            array_rev_iter: ArrayRevIter::new(&leaf.array),
        }
    }

    /// Clones the iterator.
    #[inline]
    pub(super) const fn clone(&self) -> RevIter<'l, K, V> {
        RevIter {
            leaf: self.leaf,
            array_rev_iter: self.array_rev_iter.clone(),
        }
    }

    /// Rewinds the iterator to the beginning.
    #[inline]
    pub(super) const fn rewind(&mut self) {
        self.array_rev_iter.rewind();
    }

    /// Converts itself into an [`Iter`].
    #[inline]
    pub(super) const fn rev(self) -> Iter<'l, K, V> {
        Iter {
            leaf: self.leaf,
            array_iter: self.array_rev_iter.rev(),
        }
    }

    /// Returns a reference to the entry that the iterator is currently pointing to.
    #[inline]
    pub(super) const fn get(&self) -> Option<(&'l K, &'l V)> {
        if self.array_rev_iter.rev_rank == 0 {
            None
        } else {
            let pos = self.array_rev_iter.pos
                [(DIMENSION.num_entries - self.array_rev_iter.rev_rank) as usize]
                - 1;
            Some((self.leaf.array.key(pos), self.leaf.array.val(pos)))
        }
    }

    /// Returns a reference to the min key entry.
    #[inline]
    pub(super) fn min_key(&self) -> Option<&'l K> {
        for pos in self.array_rev_iter.pos {
            if pos != 0 {
                return Some(self.leaf.key(pos - 1));
            }
        }
        None
    }

    /// Jumps to the max entry of the prev non-empty leaf.
    #[inline]
    pub(super) fn jump(&mut self, _guard: &'l Guard) -> Option<(&'l K, &'l V)>
    where
        K: Ord,
    {
        let min_key = self.min_key();
        let mut found_unlinked = false;
        loop {
            let prev_leaf_ptr = self.leaf.prev.load(Acquire);
            let Some(leaf) = (unsafe { prev_leaf_ptr.as_ref() }) else {
                break;
            };
            let metadata = leaf.metadata.load(Acquire);
            found_unlinked |= !ptr::eq(leaf.next.load(Relaxed), self.leaf);

            self.leaf = leaf;
            self.array_rev_iter = ArrayRevIter::with_metadata(metadata);

            if found_unlinked {
                // See `Iter::jump`.
                for (k, v) in self.by_ref() {
                    if min_key.is_none_or(|min| min > k) {
                        return Some((k, v));
                    }
                }
            } else if let Some((k, v)) = self.next() {
                return Some((k, v));
            }
        }
        None
    }
}

impl<K, V> fmt::Debug for RevIter<'_, K, V> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RevIter")
            .field("leaf", &ptr::addr_of!(*self.leaf))
            .field("prev", &self.leaf.prev.load(Relaxed))
            .field("next", &self.leaf.next.load(Relaxed))
            .field("array_rev_iter", &self.array_rev_iter)
            .finish()
    }
}

impl<'l, K, V> Iterator for RevIter<'l, K, V> {
    type Item = (&'l K, &'l V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.array_rev_iter
            .next()
            .map(|i| (self.leaf.key(i), self.leaf.val(i)))
    }
}

#[cfg(not(feature = "loom"))]
#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;
    use sdd::Shared;
    use std::sync::atomic::AtomicBool;
    use tokio::sync::Barrier;

    #[test]
    fn array() {
        let array: Array<String, String> = Array::new();
        assert!(matches!(
            array.insert("MY GOODNESS!".to_owned(), "OH MY GOD!!".to_owned()),
            InsertResult::Success
        ));
        assert!(matches!(
            array.insert("GOOD DAY".to_owned(), "OH MY GOD!!".to_owned()),
            InsertResult::Success
        ));
        assert_eq!(array.search_entry("MY GOODNESS!").unwrap().1, "OH MY GOD!!");
        assert_eq!(array.search_entry("GOOD DAY").unwrap().1, "OH MY GOD!!");

        for pos in 0..DIMENSION.num_entries {
            if let InsertResult::Full(k, v) = array.insert(pos.to_string(), pos.to_string()) {
                assert_eq!(pos + 2, DIMENSION.num_entries);
                assert_eq!(k, pos.to_string());
                assert_eq!(v, pos.to_string());
                break;
            }
            assert_eq!(
                array.search_entry(&pos.to_string()).unwrap(),
                (&pos.to_string(), &pos.to_string())
            );
        }

        for pos in 0..DIMENSION.num_entries {
            let result = array.remove_if(&pos.to_string(), &mut |_| pos >= 10);
            if pos >= 10 && pos + 2 < DIMENSION.num_entries {
                assert_eq!(result, RemoveResult::Success);
            } else {
                assert_eq!(result, RemoveResult::Fail);
            }
        }

        assert_eq!(
            array.remove_if("GOOD DAY", &mut |v| v == "OH MY"),
            RemoveResult::Fail
        );
        assert_eq!(
            array.remove_if("GOOD DAY", &mut |v| v == "OH MY GOD!!"),
            RemoveResult::Success
        );
        assert!(array.search_entry("GOOD DAY").is_none());
        assert_eq!(
            array.remove_if("MY GOODNESS!", &mut |_| true),
            RemoveResult::Success
        );
        assert!(array.search_entry("MY GOODNESS!").is_none());
        assert!(array.search_entry("1").is_some());
        assert!(matches!(
            array.insert("1".to_owned(), "1".to_owned()),
            InsertResult::Duplicate(..)
        ));
        assert!(matches!(
            array.insert("100".to_owned(), "100".to_owned()),
            InsertResult::Full(..)
        ));

        let mut iter = ArrayIter::new(&array);
        for pos in 0..DIMENSION.num_entries {
            if let Some(e) = iter.next() {
                assert_eq!(array.key(e), &pos.to_string());
                assert_eq!(array.val(e), &pos.to_string());
                assert_ne!(
                    array.remove_if(&pos.to_string(), &mut |_| true),
                    RemoveResult::Fail
                );
            } else {
                break;
            }
        }

        assert!(matches!(
            array.insert("200".to_owned(), "200".to_owned()),
            InsertResult::Full(..)
        ));
    }

    #[test]
    fn iter_rev_iter() {
        let leaf: Leaf<usize, usize> = Leaf::new();
        for pos in 0..DIMENSION.num_entries as usize {
            if pos % 2 == 0 {
                assert!(matches!(
                    leaf.insert(pos * 1024 + 1, pos),
                    InsertResult::Success
                ));
            } else {
                assert!(matches!(leaf.insert(pos * 2, pos), InsertResult::Success));
            }
        }
        assert!(matches!(
            leaf.remove_if(&6, &mut |_| true),
            RemoveResult::Success
        ));

        let mut iter = Iter::new(&leaf);
        assert_eq!(iter.next(), Some((&1, &0)));
        let rev_iter = iter.rev();
        assert_eq!(rev_iter.get(), Some((&1, &0)));
        iter = rev_iter.rev();
        assert_eq!(iter.get(), Some((&1, &0)));

        let mut prev_key = 0;
        let mut sum = 0;
        for (key, _) in Iter::new(&leaf) {
            assert_ne!(*key, 6);
            assert!(prev_key < *key);
            prev_key = *key;
            sum += *key;
        }
        prev_key = usize::MAX;

        for (key, _) in RevIter::new(&leaf) {
            assert_ne!(*key, 6);
            assert!(prev_key > *key);
            prev_key = *key;
            sum -= *key;
        }
        assert_eq!(sum, 0);
    }

    #[test]
    fn calculate_boundary() {
        let leaf: Leaf<usize, usize> = Leaf::new();
        for i in 0..DIMENSION.num_entries as usize {
            assert!(matches!(leaf.insert(i, i), InsertResult::Success));
        }
        assert_eq!(
            Array::<usize, usize>::optimal_boundary(leaf.metadata.load(Relaxed)),
            (DIMENSION.num_entries - 1, DIMENSION.num_entries as usize)
        );

        let leaf: Leaf<usize, usize> = Leaf::new();
        for i in (0..DIMENSION.num_entries as usize).rev() {
            assert!(matches!(leaf.insert(i, i), InsertResult::Success));
        }
        assert_eq!(
            Array::<usize, usize>::optimal_boundary(leaf.metadata.load(Relaxed)),
            (1, DIMENSION.num_entries as usize)
        );

        let leaf: Leaf<usize, usize> = Leaf::new();
        for i in 0..DIMENSION.num_entries as usize {
            if i < DIMENSION.num_entries as usize / 2 {
                assert!(matches!(
                    leaf.insert(usize::MAX - i, usize::MAX - i),
                    InsertResult::Success
                ));
            } else {
                assert!(matches!(leaf.insert(i, i), InsertResult::Success));
            }
        }
        if usize::BITS == 32 {
            assert_eq!(
                Array::<usize, usize>::optimal_boundary(leaf.metadata.load(Relaxed)),
                (4, DIMENSION.num_entries as usize)
            );
        } else {
            assert_eq!(
                Array::<usize, usize>::optimal_boundary(leaf.metadata.load(Relaxed)),
                (6, DIMENSION.num_entries as usize)
            );
        }
    }

    #[test]
    fn special() {
        let leaf: Leaf<usize, usize> = Leaf::new();
        assert!(matches!(leaf.insert(11, 17), InsertResult::Success));
        assert!(matches!(leaf.insert(17, 11), InsertResult::Success));

        let leaf1 = Leaf::new();
        leaf1.freeze();
        let leaf2 = Leaf::new();
        leaf2.freeze();
        assert!(leaf.freeze());
        let mut i = 0;
        leaf.distribute(
            |_, _| true,
            |k, v, _, b| {
                if i < b {
                    leaf1.insert_unchecked(*k, *v, i);
                } else {
                    leaf2.insert_unchecked(*k, *v, i - b);
                }
                i += 1;
            },
        );
        leaf1.unfreeze();
        leaf2.unfreeze();
        assert_eq!(leaf1.search_entry(&11), Some((&11, &17)));
        assert_eq!(leaf1.search_entry(&17), Some((&17, &11)));
        assert!(leaf2.is_empty());
        assert!(matches!(leaf.insert(1, 7), InsertResult::Frozen(..)));
        assert_eq!(leaf.remove_if(&17, &mut |_| true), RemoveResult::Frozen);
        assert!(matches!(leaf.insert(3, 5), InsertResult::Frozen(..)));

        assert!(leaf.unfreeze());
        assert!(matches!(leaf.insert(1, 7), InsertResult::Success));

        assert_eq!(leaf.remove_if(&1, &mut |_| true), RemoveResult::Success);
        assert_eq!(leaf.remove_if(&17, &mut |_| true), RemoveResult::Success);
        assert_eq!(leaf.remove_if(&11, &mut |_| true), RemoveResult::Retired);

        assert!(matches!(leaf.insert(5, 3), InsertResult::Full(..)));
    }

    proptest! {
        #[cfg_attr(miri, ignore)]
        #[test]
        fn general(insert in 0_usize..DIMENSION.num_entries as usize, remove in 0_usize..DIMENSION.num_entries as usize) {
            let array: Array<usize, usize> = Array::new();
            assert!(array.is_empty());
            for i in 0..insert {
                assert!(matches!(array.insert(i, i), InsertResult::Success));
            }
            if insert == 0 {
                assert_eq!(array.max_key(), None);
                assert!(array.is_empty());
            } else {
                assert_eq!(array.max_key(), Some(&(insert - 1)));
                assert!(!array.is_empty());
            }
            for i in 0..insert {
                assert!(matches!(array.insert(i, i), InsertResult::Duplicate(..)));
                assert!(!array.is_empty());
                let result = array.min_greater_equal(&i);
                assert_eq!(result.0, Some(&i));
            }
            for i in 0..insert {
                assert_eq!(array.search_entry(&i).unwrap(), (&i, &i));
            }
            if insert == DIMENSION.num_entries as usize {
                assert!(matches!(array.insert(usize::MAX, usize::MAX), InsertResult::Full(..)));
            }
            for i in 0..remove {
                if i < insert {
                    if i == insert - 1 {
                        assert!(matches!(array.remove_if(&i, &mut |_| true), RemoveResult::Retired));
                        for i in 0..insert {
                            assert!(matches!(array.insert(i, i), InsertResult::Full(..)));
                        }
                    } else {
                        assert!(matches!(array.remove_if(&i, &mut |_| true), RemoveResult::Success));
                    }
                } else {
                    assert!(matches!(array.remove_if(&i, &mut |_| true), RemoveResult::Fail));
                    assert!(array.is_empty());
                }
            }
        }

        #[cfg_attr(miri, ignore)]
        #[test]
        fn range(start in 0_usize..DIMENSION.num_entries as usize, end in 0_usize..DIMENSION.num_entries as usize) {
            let array: Array<usize, usize> = Array::new();
            for i in 1..DIMENSION.num_entries as usize - 1 {
                prop_assert!(matches!(array.insert(i, i), InsertResult::Success));
            }
            array.remove_range(&(start..end));
            for i in 1..DIMENSION.num_entries as usize - 1 {
                prop_assert!(array.search_entry(&i).is_none() == (start..end).contains(&i));
            }
            prop_assert!(array.search_entry(&0).is_none());
            prop_assert!(array.search_entry(&(DIMENSION.num_entries as usize - 1)).is_none());
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test(flavor = "multi_thread", worker_threads = 16)]
    async fn update() {
        let num_excess = 3;
        let num_tasks = DIMENSION.num_entries as usize + num_excess;
        for _ in 0..256 {
            let barrier = Shared::new(Barrier::new(num_tasks));
            let leaf: Shared<Leaf<usize, usize>> = Shared::new(Leaf::new());
            let full: Shared<AtomicUsize> = Shared::new(AtomicUsize::new(0));
            let retire: Shared<AtomicUsize> = Shared::new(AtomicUsize::new(0));
            let mut task_handles = Vec::with_capacity(num_tasks);
            for t in 1..=num_tasks {
                let barrier_clone = barrier.clone();
                let leaf_clone = leaf.clone();
                let full_clone = full.clone();
                let retire_clone = retire.clone();
                task_handles.push(tokio::spawn(async move {
                    barrier_clone.wait().await;
                    let inserted = match leaf_clone.insert(t, t) {
                        InsertResult::Success => {
                            assert_eq!(leaf_clone.search_entry(&t).unwrap(), (&t, &t));
                            true
                        }
                        InsertResult::Duplicate(_, _) | InsertResult::Frozen(_, _) => {
                            unreachable!();
                        }
                        InsertResult::Full(k, v) => {
                            assert_eq!(k, v);
                            assert_eq!(k, t);
                            full_clone.fetch_add(1, Relaxed);
                            false
                        }
                    };
                    {
                        let mut prev = 0;
                        let mut iter = Iter::new(&leaf_clone);
                        for e in iter.by_ref() {
                            assert_eq!(e.0, e.1);
                            assert!(*e.0 > prev);
                            prev = *e.0;
                        }
                    }

                    barrier_clone.wait().await;
                    assert_eq!((*full_clone).load(Relaxed), num_excess);
                    if inserted {
                        assert_eq!(leaf_clone.search_entry(&t).unwrap(), (&t, &t));
                    }
                    {
                        let iter = Iter::new(&leaf_clone);
                        assert_eq!(iter.count(), DIMENSION.num_entries as usize);
                    }

                    barrier_clone.wait().await;
                    match leaf_clone.remove_if(&t, &mut |_| true) {
                        RemoveResult::Success => assert!(inserted),
                        RemoveResult::Fail => assert!(!inserted),
                        RemoveResult::Frozen => unreachable!(),
                        RemoveResult::Retired => {
                            assert!(inserted);
                            assert_eq!(retire_clone.swap(1, Relaxed), 0);
                        }
                    }
                }));
            }
            for r in futures::future::join_all(task_handles).await {
                assert!(r.is_ok());
            }
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test(flavor = "multi_thread", worker_threads = 16)]
    async fn durability() {
        let num_tasks = 16_usize;
        let workload_size = 8_usize;
        for _ in 0..16 {
            for k in 0..=workload_size {
                let barrier = Shared::new(Barrier::new(num_tasks));
                let leaf: Shared<Leaf<usize, usize>> = Shared::new(Leaf::new());
                let inserted: Shared<AtomicBool> = Shared::new(AtomicBool::new(false));
                let mut task_handles = Vec::with_capacity(num_tasks);
                for _ in 0..num_tasks {
                    let barrier_clone = barrier.clone();
                    let leaf_clone = leaf.clone();
                    let inserted_clone = inserted.clone();
                    task_handles.push(tokio::spawn(async move {
                        {
                            barrier_clone.wait().await;
                            if let InsertResult::Success = leaf_clone.insert(k, k) {
                                assert!(!inserted_clone.swap(true, Relaxed));
                            }
                        }
                        {
                            barrier_clone.wait().await;
                            for i in 0..workload_size {
                                if i != k {
                                    let _result = leaf_clone.insert(i, i);
                                }
                                assert!(!leaf_clone.is_retired());
                                assert_eq!(leaf_clone.search_entry(&k).unwrap(), (&k, &k));
                            }
                            for i in 0..workload_size {
                                let _result = leaf_clone.remove_if(&i, &mut |v| *v != k);
                                assert_eq!(leaf_clone.search_entry(&k).unwrap(), (&k, &k));
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
