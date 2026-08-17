use std::cell::UnsafeCell;
use std::mem::{MaybeUninit, forget, needs_drop};
use std::ops::Deref;
use std::ptr::{self, NonNull, from_ref};
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::sync::atomic::{AtomicPtr, AtomicU32, AtomicUsize};

use saa::Lock;
use sdd::{AtomicShared, Epoch, Guard, Shared, Tag};

use crate::Equivalent;
use crate::async_helper::AsyncGuard;

/// [`Bucket`] is a lock-protected fixed-size entry array.
///
/// In case the fixed-size entry array overflows, additional entries can be stored in a linked list
/// of [`LinkedBucket`].
#[repr(align(64))]
pub struct Bucket<K, V, L: LruList, const TYPE: char> {
    /// Number of entries in the [`Bucket`].
    len: AtomicUsize,
    /// Reader-writer lock.
    rw_lock: Lock,
    /// [`Bucket`] metadata.
    metadata: Metadata<K, V, BUCKET_LEN>,
    /// The LRU list of the [`Bucket`].
    lru_list: L,
}

/// The type of [`Bucket`] that only allows sequential access to it.
pub const MAP: char = 'S';

/// The type of [`Bucket`] that allows lock-free read access.
pub const INDEX: char = 'O';

/// The type of [`Bucket`] that acts as an LRU cache.
pub const CACHE: char = 'C';

/// The size of the fixed-size entry array in a [`Bucket`].
pub const BUCKET_LEN: usize = u32::BITS as usize;

/// [`DataBlock`] is a type alias of a raw memory chunk of entries.
pub struct DataBlock<K, V, const LEN: usize>([UnsafeCell<MaybeUninit<(K, V)>>; LEN]);

/// [`Writer`] holds an exclusive lock on a [`Bucket`].
pub struct Writer<K, V, L: LruList, const TYPE: char> {
    bucket_ptr: NonNull<Bucket<K, V, L, TYPE>>,
}

/// [`Reader`] holds a shared lock on a [`Bucket`].
pub struct Reader<K, V, L: LruList, const TYPE: char> {
    bucket_ptr: NonNull<Bucket<K, V, L, TYPE>>,
}

/// [`EntryPtr`] points to an entry slot in a [`Bucket`].
pub struct EntryPtr<K, V, const TYPE: char> {
    /// Pointer to a [`LinkedBucket`].
    link_ptr: *const LinkedBucket<K, V>,
    /// Index of the entry.
    index: u32,
}

/// Doubly-linked list interfaces to efficiently manage least-recently-used entries.
pub trait LruList: 'static + Default {
    /// Evicts an entry.
    #[inline]
    fn evict(&self, _tail: u32) -> Option<(u8, u32)> {
        None
    }

    /// Removes an entry.
    #[inline]
    fn remove(&self, _tail: u32, _entry: u8) -> Option<u32> {
        None
    }

    /// Promotes the entry.
    #[inline]
    fn promote(&self, _tail: u32, _entry: u8) -> Option<u32> {
        None
    }
}

/// [`DoublyLinkedList`] is an array of `(u8, u8)` implementing [`LruList`].
#[derive(Default)]
pub struct DoublyLinkedList([UnsafeCell<(u8, u8)>; BUCKET_LEN]);

/// [`Metadata`] is a collection of metadata fields of [`Bucket`] and [`LinkedBucket`].
struct Metadata<K, V, const LEN: usize> {
    /// Occupied slot bitmap.
    occupied_bitmap: AtomicU32,
    /// Removed slot bitmap, or the 1-based index of the most recently used entry where `0`
    /// represents `nil` if `TYPE = CACHE`.
    removed_bitmap: AtomicU32,
    /// Partial hash array for fast hash lookup, or the epoch when the corresponding entry was
    /// removed if `TYPE = INDEX`.
    partial_hash_array: UnsafeCell<[u8; LEN]>,
    /// Linked list of entries.
    link: AtomicShared<LinkedBucket<K, V>>,
}

/// The size of the linked data block.
const LINKED_BUCKET_LEN: usize = BUCKET_LEN / 4;

/// [`LinkedBucket`] is a smaller [`Bucket`] that is attached to a [`Bucket`] as a linked list.
struct LinkedBucket<K, V> {
    /// [`LinkedBucket`] metadata.
    metadata: Metadata<K, V, LINKED_BUCKET_LEN>,
    /// Previous [`LinkedBucket`].
    prev_link: AtomicPtr<LinkedBucket<K, V>>,
    /// Own data block.
    data_block: DataBlock<K, V, LINKED_BUCKET_LEN>,
}

impl<K, V, L: LruList, const TYPE: char> Bucket<K, V, L, TYPE> {
    /// Creates a new [`Bucket`].
    #[cfg(any(test, feature = "loom"))]
    pub fn new() -> Self {
        Self {
            len: AtomicUsize::new(0),
            rw_lock: Lock::default(),
            metadata: Metadata {
                occupied_bitmap: AtomicU32::default(),
                removed_bitmap: AtomicU32::default(),
                partial_hash_array: UnsafeCell::new(Default::default()),
                link: AtomicShared::default(),
            },
            lru_list: L::default(),
        }
    }

    /// Returns the number of occupied and reachable slots in the [`Bucket`].
    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.len.load(Relaxed)
    }

    /// Reserves memory for insertion and then constructs the key-value pair in-place.
    #[inline]
    pub(crate) fn insert(
        &self,
        data_block: NonNull<DataBlock<K, V, BUCKET_LEN>>,
        hash: u64,
        entry: (K, V),
    ) -> EntryPtr<K, V, TYPE> {
        let partial_hash = Self::partial_hash(hash);
        let occupied_bitmap = self.metadata.occupied_bitmap.load(Relaxed);
        let occupied_bitmap = if TYPE == INDEX
            && partial_hash % 8 == 0
            && occupied_bitmap == u32::MAX
            && (self.metadata.removed_bitmap.load(Relaxed) != 0
                || !self.metadata.link.is_null(Relaxed))
        {
            self.clear_unreachable_entries(data_block_ref(data_block))
        } else {
            occupied_bitmap
        };

        let free_index = occupied_bitmap.trailing_ones();
        if free_index as usize == BUCKET_LEN {
            self.insert_overflow(partial_hash, entry)
        } else {
            self.insert_entry(
                &self.metadata,
                data_block_ref(data_block),
                free_index,
                occupied_bitmap,
                partial_hash,
                entry,
            );
            EntryPtr {
                link_ptr: ptr::null(),
                index: free_index,
            }
        }
    }

    /// Removes the entry pointed to by the supplied [`EntryPtr`].
    #[inline]
    pub(crate) fn remove(
        &self,
        data_block: NonNull<DataBlock<K, V, BUCKET_LEN>>,
        entry_ptr: &mut EntryPtr<K, V, TYPE>,
    ) -> (K, V) {
        debug_assert_ne!(TYPE, INDEX);
        debug_assert_ne!(entry_ptr.index, u32::MAX);
        debug_assert_ne!(entry_ptr.index as usize, BUCKET_LEN);

        self.len.store(self.len.load(Relaxed) - 1, Relaxed);

        if let Some(link) = link_ref(entry_ptr.link_ptr) {
            let mut occupied_bitmap = link.metadata.occupied_bitmap.load(Relaxed);
            debug_assert_ne!(occupied_bitmap & (1_u32 << entry_ptr.index), 0);

            occupied_bitmap &= !(1_u32 << entry_ptr.index);
            link.metadata
                .occupied_bitmap
                .store(occupied_bitmap, Relaxed);
            let removed = link.data_block.read_entry(entry_ptr.index);
            if occupied_bitmap == 0 && TYPE != INDEX {
                entry_ptr.unlink(&self.metadata.link);
            }
            removed
        } else {
            let occupied_bitmap = self.metadata.occupied_bitmap.load(Relaxed);
            debug_assert_ne!(occupied_bitmap & (1_u32 << entry_ptr.index), 0);

            if TYPE == CACHE {
                self.remove_from_lru_list(entry_ptr);
            }

            self.metadata
                .occupied_bitmap
                .store(occupied_bitmap & !(1_u32 << entry_ptr.index), Relaxed);
            data_block_ref(data_block).read_entry(entry_ptr.index)
        }
    }

    /// Marks the entry removed without dropping the entry.
    #[inline]
    pub(crate) fn mark_removed(&self, entry_ptr: &mut EntryPtr<K, V, TYPE>, guard: &Guard) {
        debug_assert_eq!(TYPE, INDEX);
        debug_assert_ne!(entry_ptr.index, u32::MAX);
        debug_assert_ne!(entry_ptr.index as usize, BUCKET_LEN);

        self.len.store(self.len.load(Relaxed) - 1, Relaxed);

        if let Some(link) = link_ref(entry_ptr.link_ptr) {
            link.metadata
                .update_partial_hash(entry_ptr.index, u8::from(guard.epoch()));
            let mut removed_bitmap = link.metadata.removed_bitmap.load(Relaxed);
            debug_assert_eq!(removed_bitmap & (1_u32 << entry_ptr.index), 0);

            removed_bitmap |= 1_u32 << entry_ptr.index;
            link.metadata.removed_bitmap.store(removed_bitmap, Release);
        } else {
            self.metadata
                .update_partial_hash(entry_ptr.index, u8::from(guard.epoch()));
            let mut removed_bitmap = self.metadata.removed_bitmap.load(Relaxed);
            debug_assert_eq!(removed_bitmap & (1_u32 << entry_ptr.index), 0);

            removed_bitmap |= 1_u32 << entry_ptr.index;
            self.metadata.removed_bitmap.store(removed_bitmap, Release);
        }
    }

    /// Evicts the least recently used entry if the [`Bucket`] is full.
    #[inline]
    pub(crate) fn evict_lru_head(
        &self,
        data_block: NonNull<DataBlock<K, V, BUCKET_LEN>>,
    ) -> Option<(K, V)> {
        debug_assert_eq!(TYPE, CACHE);

        let occupied_bitmap = self.metadata.occupied_bitmap.load(Relaxed);
        if occupied_bitmap == u32::MAX {
            self.len.store(self.len.load(Relaxed) - 1, Relaxed);

            let tail = self.metadata.removed_bitmap.load(Relaxed);
            let evicted = if let Some((evicted, new_tail)) = self.lru_list.evict(tail) {
                self.metadata.removed_bitmap.store(new_tail, Relaxed);
                u32::from(evicted)
            } else {
                // Evict the first occupied entry.
                0
            };
            debug_assert_ne!(occupied_bitmap & (1_u32 << evicted), 0);

            self.metadata
                .occupied_bitmap
                .store(occupied_bitmap & !(1_u32 << evicted), Relaxed);
            return Some(data_block_ref(data_block).read_entry(evicted));
        }

        None
    }

    /// Sets the entry as having been just accessed.
    #[inline]
    pub(crate) fn update_lru_tail(&self, entry_ptr: &EntryPtr<K, V, TYPE>) {
        debug_assert_eq!(TYPE, CACHE);
        debug_assert_ne!(entry_ptr.index, u32::MAX);
        debug_assert_ne!(entry_ptr.index as usize, BUCKET_LEN);

        if entry_ptr.link_ptr.is_null() {
            #[allow(clippy::cast_possible_truncation)]
            let entry = entry_ptr.index as u8;
            let tail = self.metadata.removed_bitmap.load(Relaxed);
            if let Some(new_tail) = self.lru_list.promote(tail, entry) {
                self.metadata.removed_bitmap.store(new_tail, Relaxed);
            }
        }
    }

    /// Reserves memory for additional entries.
    #[inline]
    pub(crate) fn reserve_slots(&self, additional: usize) {
        debug_assert!(self.rw_lock.is_locked(Relaxed));

        let required = additional + self.len();
        let mut capacity = BUCKET_LEN;
        if capacity >= required {
            return;
        }

        let mut link_ptr = self.metadata.load_link();
        while let Some(link) = link_ref(link_ptr) {
            capacity += LINKED_BUCKET_LEN;
            if capacity >= required {
                return;
            }
            let new_link_ptr = link.metadata.load_link();
            if new_link_ptr.is_null() {
                break;
            }
            link_ptr = new_link_ptr;
        }

        // Allocate additional overflow buckets.
        for _ in 0..(required - capacity).div_ceil(LINKED_BUCKET_LEN) {
            let new_link = LinkedBucket::new(None);
            let new_link_ptr = new_link.as_ptr();
            if let Some(link) = link_ref(link_ptr) {
                new_link.prev_link.store(link_ptr.cast_mut(), Relaxed);
                link.metadata
                    .link
                    .swap((Some(new_link), Tag::None), Release);
            } else {
                self.metadata
                    .link
                    .swap((Some(new_link), Tag::None), Release);
            }
            link_ptr = new_link_ptr;
        }
    }

    /// Extracts an entry from the given bucket and inserts the entry into itself.
    #[inline]
    pub(crate) fn extract_from(
        &self,
        data_block: NonNull<DataBlock<K, V, BUCKET_LEN>>,
        hash: u64,
        from_writer: &Writer<K, V, L, TYPE>,
        from_data_block: NonNull<DataBlock<K, V, BUCKET_LEN>>,
        from_entry_ptr: &mut EntryPtr<K, V, TYPE>,
    ) {
        debug_assert!(self.rw_lock.is_locked(Relaxed));

        let entry = if let Some(link) = link_ref(from_entry_ptr.link_ptr) {
            link.data_block.read_entry(from_entry_ptr.index)
        } else {
            data_block_ref(from_data_block).read_entry(from_entry_ptr.index)
        };
        self.insert(data_block, hash, entry);

        let mo = if TYPE == INDEX { Release } else { Relaxed };
        if let Some(link) = link_ref(from_entry_ptr.link_ptr) {
            let occupied_bitmap = link.metadata.occupied_bitmap.load(Relaxed);
            debug_assert_ne!(occupied_bitmap & (1_u32 << from_entry_ptr.index), 0);

            link.metadata
                .occupied_bitmap
                .store(occupied_bitmap & !(1_u32 << from_entry_ptr.index), mo);
        } else {
            let occupied_bitmap = from_writer.metadata.occupied_bitmap.load(Relaxed);
            debug_assert_ne!(occupied_bitmap & (1_u32 << from_entry_ptr.index), 0);

            from_writer
                .metadata
                .occupied_bitmap
                .store(occupied_bitmap & !(1_u32 << from_entry_ptr.index), mo);
        }

        let from_len = from_writer.len.load(Relaxed);
        from_writer.len.store(from_len - 1, Relaxed);
    }

    /// Drops entries in the [`DataBlock`] when the bucket array is being dropped.
    ///
    /// The [`Bucket`] and the [`DataBlock`] should never be used afterward.
    pub(super) fn drop_entries(&self, data_block: NonNull<DataBlock<K, V, BUCKET_LEN>>) {
        if !self.metadata.link.is_null(Relaxed) {
            let mut next = self.metadata.link.swap((None, Tag::None), Acquire);
            while let Some(current) = next.0 {
                next = current.metadata.link.swap((None, Tag::None), Acquire);
                let dropped = unsafe { current.drop_in_place() };
                debug_assert!(dropped);
            }
        }
        if needs_drop::<(K, V)>() {
            let mut occupied_bitmap = self.metadata.occupied_bitmap.load(Relaxed);
            while occupied_bitmap != 0 {
                let index = occupied_bitmap.trailing_zeros();
                data_block_ref(data_block).drop_entry(index);
                occupied_bitmap -= 1_u32 << index;
            }
        }
    }

    /// Returns the partial hash value of the given hash.
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    const fn partial_hash(hash: u64) -> u8 {
        hash as u8
    }

    /// Inserts an entry into an overflow bucket.
    fn insert_overflow(&self, partial_hash: u8, entry: (K, V)) -> EntryPtr<K, V, TYPE> {
        let mut link_ptr = self.metadata.load_link();
        while let Some(link) = link_ref(link_ptr) {
            let occupied_bitmap = link.metadata.occupied_bitmap.load(Relaxed);
            let free_index = occupied_bitmap.trailing_ones();
            if free_index as usize != LINKED_BUCKET_LEN {
                debug_assert!((free_index as usize) < LINKED_BUCKET_LEN);
                self.insert_entry(
                    &link.metadata,
                    &link.data_block,
                    free_index,
                    occupied_bitmap,
                    partial_hash,
                    entry,
                );
                return EntryPtr {
                    link_ptr,
                    index: free_index,
                };
            }
            link_ptr = link.metadata.load_link();
        }

        // Insert a new `LinkedBucket` at the linked list head.
        let head = self.metadata.link.get_shared(Relaxed, fake_ref(self));
        let link = LinkedBucket::new(head);
        self.insert_entry(&link.metadata, &link.data_block, 0, 1, partial_hash, entry);
        if let Some(head) = link_ref(link.metadata.load_link()) {
            head.prev_link.store(link.as_ptr().cast_mut(), Relaxed);
        }
        let link_ptr = link.as_ptr();
        self.metadata.link.swap((Some(link), Tag::None), Release);
        EntryPtr { link_ptr, index: 0 }
    }

    /// Inserts a key-value pair in the slot.
    #[inline]
    fn insert_entry<const LEN: usize>(
        &self,
        metadata: &Metadata<K, V, LEN>,
        data_block: &DataBlock<K, V, LEN>,
        index: u32,
        occupied_bitmap: u32,
        partial_hash: u8,
        entry: (K, V),
    ) {
        debug_assert!((index as usize) < LEN);
        debug_assert_eq!(metadata.occupied_bitmap.load(Relaxed) & (1_u32 << index), 0);

        unsafe {
            data_block.entry_ptr(index).cast_mut().write(entry);
        }
        metadata.update_partial_hash(index, partial_hash);
        metadata.occupied_bitmap.store(
            occupied_bitmap | (1_u32 << index),
            if TYPE == INDEX { Release } else { Relaxed },
        );
        self.len.store(self.len() + 1, Relaxed);
    }

    /// Clears unreachable entries.
    fn clear_unreachable_entries(&self, data_block: &DataBlock<K, V, BUCKET_LEN>) -> u32 {
        debug_assert_eq!(TYPE, INDEX);

        let guard = Guard::new();

        let mut link_ptr = self.metadata.load_link();
        while let Some(link) = link_ref(link_ptr) {
            let mut next_link_ptr = link.metadata.load_link();
            if next_link_ptr.is_null() {
                while let Some(link) = link_ref(link_ptr) {
                    let prev_link_ptr = link.prev_link.load(Acquire);
                    if Self::drop_unreachable_entries(&link.metadata, &link.data_block, &guard) == 0
                        && next_link_ptr.is_null()
                    {
                        debug_assert!(link.metadata.link.is_null(Relaxed));
                        let unlinked = if let Some(prev) = link_ref(prev_link_ptr) {
                            prev.metadata.link.swap((None, Tag::None), Acquire).0
                        } else {
                            self.metadata.link.swap((None, Tag::None), Acquire).0
                        };
                        debug_assert!(unlinked.is_some_and(Shared::release));
                    } else {
                        next_link_ptr = link_ptr;
                    }
                    link_ptr = prev_link_ptr;
                }
                break;
            }
            link_ptr = next_link_ptr;
        }

        Self::drop_unreachable_entries(&self.metadata, data_block, &guard)
    }

    /// Drops unreachable entries.
    fn drop_unreachable_entries<const LEN: usize>(
        metadata: &Metadata<K, V, LEN>,
        data_block: &DataBlock<K, V, LEN>,
        guard: &Guard,
    ) -> u32 {
        debug_assert_eq!(TYPE, INDEX);

        let mut dropped_bitmap = metadata.removed_bitmap.load(Relaxed);

        let current_epoch = guard.epoch();
        #[allow(clippy::cast_possible_truncation)]
        for i in 0..LEN as u32 {
            if Epoch::try_from(metadata.read_partial_hash(i))
                .is_ok_and(|e| e.in_same_generation(current_epoch))
            {
                dropped_bitmap &= !(1_u32 << i);
            }
        }

        // Store order: `occupied_bitmap` -> `release` -> `removed_bitmap`.
        let occupied_bitmap = metadata.occupied_bitmap.load(Relaxed) & !dropped_bitmap;
        metadata.occupied_bitmap.store(occupied_bitmap, Release);
        let removed_bitmap = metadata.removed_bitmap.load(Relaxed) & !dropped_bitmap;
        metadata.removed_bitmap.store(removed_bitmap, Release);
        if removed_bitmap != 0 {
            guard.set_has_garbage();
        }

        if needs_drop::<(K, V)>() {
            while dropped_bitmap != 0 {
                let index = dropped_bitmap.trailing_zeros();
                data_block.drop_entry(index);
                dropped_bitmap -= 1_u32 << index;
            }
        }

        occupied_bitmap
    }

    /// Removes the entry from the LRU linked list.
    #[inline]
    fn remove_from_lru_list(&self, entry_ptr: &EntryPtr<K, V, TYPE>) {
        debug_assert_eq!(TYPE, CACHE);
        debug_assert_ne!(entry_ptr.index, u32::MAX);
        debug_assert_ne!(entry_ptr.index as usize, BUCKET_LEN);

        if entry_ptr.link_ptr.is_null() {
            #[allow(clippy::cast_possible_truncation)]
            let entry = entry_ptr.index as u8;
            let tail = self.metadata.removed_bitmap.load(Relaxed);
            if let Some(new_tail) = self.lru_list.remove(tail, entry) {
                self.metadata.removed_bitmap.store(new_tail, Relaxed);
            }
        }
    }
}

impl<K: Eq, V, L: LruList, const TYPE: char> Bucket<K, V, L, TYPE> {
    /// Searches for an entry containing the key.
    ///
    /// Returns `None` if the key is not present.
    #[inline]
    pub(super) fn search_entry<'g, Q>(
        &self,
        data_block: NonNull<DataBlock<K, V, BUCKET_LEN>>,
        key: &Q,
        hash: u64,
    ) -> Option<&'g (K, V)>
    where
        Q: Equivalent<K> + ?Sized,
    {
        if self.len() != 0 {
            let partial_hash = Self::partial_hash(hash);
            if let Some((entry, _)) = Self::search_data_block(
                &self.metadata,
                data_block_ref(data_block),
                key,
                partial_hash,
            ) {
                return Some(entry);
            }

            let mut link_ptr = self.metadata.load_link();
            while let Some(link) = link_ref(link_ptr) {
                if let Some((entry, _)) =
                    Self::search_data_block(&link.metadata, &link.data_block, key, partial_hash)
                {
                    return Some(entry);
                }
                link_ptr = link.metadata.load_link();
            }
        }
        None
    }

    /// Gets an [`EntryPtr`] pointing to the slot containing the key.
    ///
    /// Returns an invalid [`EntryPtr`] if the key is not present.
    #[inline]
    pub(crate) fn get_entry_ptr<Q>(
        &self,
        data_block: NonNull<DataBlock<K, V, BUCKET_LEN>>,
        key: &Q,
        hash: u64,
    ) -> EntryPtr<K, V, TYPE>
    where
        Q: Equivalent<K> + ?Sized,
    {
        if self.len() != 0 {
            let partial_hash = Self::partial_hash(hash);
            if let Some((_, index)) = Self::search_data_block(
                &self.metadata,
                data_block_ref(data_block),
                key,
                partial_hash,
            ) {
                return EntryPtr {
                    link_ptr: ptr::null(),
                    index,
                };
            }

            let mut current_link_ptr = self.metadata.load_link();
            while let Some(link) = link_ref(current_link_ptr) {
                if let Some((_, index)) =
                    Self::search_data_block(&link.metadata, &link.data_block, key, partial_hash)
                {
                    return EntryPtr {
                        link_ptr: current_link_ptr,
                        index,
                    };
                }
                current_link_ptr = link.metadata.load_link();
            }
        }
        EntryPtr::null()
    }

    /// Searches the supplied data block for the entry containing the key.
    #[inline]
    fn search_data_block<'g, Q, const LEN: usize>(
        metadata: &Metadata<K, V, LEN>,
        data_block: &'g DataBlock<K, V, LEN>,
        key: &Q,
        partial_hash: u8,
    ) -> Option<(&'g (K, V), u32)>
    where
        Q: Equivalent<K> + ?Sized,
    {
        let mut bitmap = metadata.bitmap::<TYPE>();

        // Expect that the loop is vectorized by the compiler (https://godbolt.org/z/bcWYsaPbY).
        let partial_hash_array = unsafe { &*metadata.partial_hash_array.get() };
        (0..LEN).for_each(|i| {
            if partial_hash_array[i] != partial_hash {
                bitmap &= !(1_u32 << i);
            }
        });

        let mut index = bitmap.trailing_zeros();
        while index != u32::BITS {
            let entry = unsafe { &*data_block.entry_ptr(index) };
            if key.equivalent(&entry.0) {
                return Some((entry, index));
            }
            bitmap -= 1_u32 << index;
            index = bitmap.trailing_zeros();
        }

        None
    }
}

unsafe impl<K: Eq + Send, V: Send, L: LruList, const TYPE: char> Send for Bucket<K, V, L, TYPE> {}
unsafe impl<K: Eq + Send + Sync, V: Send + Sync, L: LruList, const TYPE: char> Sync
    for Bucket<K, V, L, TYPE>
{
}

impl<K, V, const LEN: usize> DataBlock<K, V, LEN> {
    /// Creates a new [`DataBlock`].
    #[inline]
    const fn new() -> DataBlock<K, V, LEN> {
        DataBlock([const { UnsafeCell::new(MaybeUninit::uninit()) }; LEN])
    }

    /// Returns a pointer to the entry at the given index.
    #[inline]
    const fn entry_ptr(&self, index: u32) -> *const (K, V) {
        self.0[index as usize].get().cast::<(K, V)>()
    }

    /// Reads the entry at the given index.
    #[inline]
    const fn read_entry(&self, index: u32) -> (K, V) {
        unsafe { self.entry_ptr(index).read() }
    }

    /// Drops the entry at the given index.
    #[inline]
    fn drop_entry(&self, index: u32) {
        unsafe {
            self.entry_ptr(index).cast_mut().drop_in_place();
        }
    }
}

unsafe impl<K: Send, V: Send, const LEN: usize> Send for DataBlock<K, V, LEN> {}
unsafe impl<K: Send + Sync, V: Send + Sync, const LEN: usize> Sync for DataBlock<K, V, LEN> {}

impl<K, V, L: LruList, const TYPE: char> Writer<K, V, L, TYPE> {
    /// Creates a new [`Writer`] from a [`Bucket`].
    #[inline]
    pub(crate) const fn from_bucket(bucket: &Bucket<K, V, L, TYPE>) -> Writer<K, V, L, TYPE> {
        Writer {
            bucket_ptr: bucket_ptr(bucket),
        }
    }

    /// Locks the [`Bucket`] asynchronously.
    #[inline]
    pub(crate) async fn lock_async<'g>(
        bucket: &'g Bucket<K, V, L, TYPE>,
        async_guard: &'g AsyncGuard,
    ) -> Option<Writer<K, V, L, TYPE>> {
        if bucket.rw_lock.lock_async_with(|| async_guard.reset()).await {
            // The `bucket` was not killed, and will not be killed until the `Writer` is dropped.
            // This guarantees that the `BucketArray` will survive as long as the `Writer` is alive.
            Some(Self::from_bucket(bucket))
        } else {
            None
        }
    }

    /// Locks the [`Bucket`] synchronously.
    #[inline]
    pub(crate) fn lock_sync(bucket: &Bucket<K, V, L, TYPE>) -> Option<Writer<K, V, L, TYPE>> {
        if bucket.rw_lock.lock_sync() {
            Some(Self::from_bucket(bucket))
        } else {
            None
        }
    }

    /// Tries to lock the [`Bucket`].
    #[inline]
    pub(crate) fn try_lock(
        bucket: &Bucket<K, V, L, TYPE>,
    ) -> Result<Option<Writer<K, V, L, TYPE>>, ()> {
        if bucket.rw_lock.try_lock() {
            Ok(Some(Self::from_bucket(bucket)))
        } else if bucket.rw_lock.is_poisoned(Relaxed) {
            Ok(None)
        } else {
            Err(())
        }
    }

    /// Marks the [`Bucket`] killed by poisoning the lock.
    #[inline]
    pub(super) fn kill(self) {
        debug_assert_eq!(self.len(), 0);
        debug_assert!(self.rw_lock.is_locked(Relaxed));
        debug_assert!(
            TYPE != INDEX
                || self.metadata.removed_bitmap.load(Relaxed)
                    == self.metadata.occupied_bitmap.load(Relaxed)
        );

        let poisoned = self.rw_lock.poison_lock();
        debug_assert!(poisoned);

        if (TYPE != INDEX || !needs_drop::<(K, V)>()) && !self.metadata.link.is_null(Relaxed) {
            // In case `TYPE == INDEX`, `(K, V)` that need `drop` should be dropped in
            // `drop_entries` to make sure that they are dropped before the container is dropped;
            // they should never be passed to the garbage collector.
            let mut link = self.metadata.link.swap((None, Tag::None), Acquire).0;
            while let Some(current) = link {
                link = current.metadata.link.swap((None, Tag::None), Acquire).0;
                let released = if TYPE == INDEX {
                    current.release()
                } else {
                    unsafe { current.drop_in_place() }
                };
                debug_assert!(released);
            }
        }

        forget(self);
    }
}

impl<K, V, L: LruList, const TYPE: char> Deref for Writer<K, V, L, TYPE> {
    type Target = Bucket<K, V, L, TYPE>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.bucket_ptr.as_ref() }
    }
}

impl<K, V, L: LruList, const TYPE: char> Drop for Writer<K, V, L, TYPE> {
    #[inline]
    fn drop(&mut self) {
        self.rw_lock.release_lock();
    }
}

unsafe impl<K: Send, V: Send, L: LruList, const TYPE: char> Send for Writer<K, V, L, TYPE> {}
unsafe impl<K: Send + Sync, V: Send + Sync, L: LruList, const TYPE: char> Sync
    for Writer<K, V, L, TYPE>
{
}

impl<'g, K, V, L: LruList, const TYPE: char> Reader<K, V, L, TYPE> {
    /// Locks the [`Bucket`] asynchronously.
    #[inline]
    pub(crate) async fn lock_async(
        bucket: &'g Bucket<K, V, L, TYPE>,
        async_guard: &AsyncGuard,
    ) -> Option<Reader<K, V, L, TYPE>> {
        if bucket
            .rw_lock
            .share_async_with(|| async_guard.reset())
            .await
        {
            // The `bucket` was not killed, and will not be killed until the `Reader` is dropped.
            // This guarantees that the `BucketArray` will survive as long as the `Reader` is alive.
            Some(Reader {
                bucket_ptr: bucket_ptr(bucket),
            })
        } else {
            None
        }
    }

    /// Locks the [`Bucket`] synchronously.
    ///
    /// Returns `None` if the [`Bucket`] has been killed or is empty.
    #[inline]
    pub(crate) fn lock_sync(bucket: &Bucket<K, V, L, TYPE>) -> Option<Reader<K, V, L, TYPE>> {
        if bucket.rw_lock.share_sync() {
            Some(Reader {
                bucket_ptr: bucket_ptr(bucket),
            })
        } else {
            None
        }
    }
}

impl<K, V, L: LruList, const TYPE: char> Deref for Reader<K, V, L, TYPE> {
    type Target = Bucket<K, V, L, TYPE>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.bucket_ptr.as_ref() }
    }
}

impl<K, V, L: LruList, const TYPE: char> Drop for Reader<K, V, L, TYPE> {
    #[inline]
    fn drop(&mut self) {
        self.rw_lock.release_share();
    }
}

unsafe impl<K: Send, V: Send, L: LruList, const TYPE: char> Send for Reader<K, V, L, TYPE> {}
unsafe impl<K: Send + Sync, V: Send + Sync, L: LruList, const TYPE: char> Sync
    for Reader<K, V, L, TYPE>
{
}

impl<K, V, const TYPE: char> EntryPtr<K, V, TYPE> {
    /// Creates a new invalid [`EntryPtr`].
    #[inline]
    pub(crate) const fn null() -> Self {
        Self {
            link_ptr: ptr::null(),
            index: u32::MAX,
        }
    }

    #[inline]
    pub(crate) const fn clone(&self) -> Self {
        Self {
            link_ptr: self.link_ptr,
            index: self.index,
        }
    }

    /// Returns `true` if the [`EntryPtr`] points to, or has pointed to, an occupied entry.
    #[inline]
    pub(crate) const fn is_valid(&self) -> bool {
        self.index != u32::MAX
    }

    /// Returns the current index.
    #[inline]
    pub(crate) const fn index(&self) -> u32 {
        self.index
    }

    /// Gets the partial hash value of the entry.
    ///
    /// The [`EntryPtr`] must point to a valid entry.
    #[inline]
    pub(crate) const fn partial_hash<L: LruList>(&self, bucket: &Bucket<K, V, L, TYPE>) -> u8 {
        if let Some(link) = link_ref(self.link_ptr) {
            link.metadata.read_partial_hash(self.index)
        } else {
            bucket.metadata.read_partial_hash(self.index)
        }
    }

    /// Gets a reference to the entry.
    ///
    /// The [`EntryPtr`] must point to a valid entry.
    #[inline]
    pub(crate) const fn get<'e>(
        &self,
        data_block: NonNull<DataBlock<K, V, BUCKET_LEN>>,
    ) -> &'e (K, V) {
        let entry_ptr = if let Some(link) = link_ref(self.link_ptr) {
            link.data_block.entry_ptr(self.index)
        } else {
            data_block_ref(data_block).entry_ptr(self.index)
        };
        unsafe { &(*entry_ptr) }
    }

    /// Gets a mutable reference to the entry.
    ///
    /// The associated [`Bucket`] must be locked, and the [`EntryPtr`] must point to a valid entry.
    #[inline]
    pub(crate) const fn get_mut<L: LruList>(
        &mut self,
        data_block: NonNull<DataBlock<K, V, BUCKET_LEN>>,
        _writer: &Writer<K, V, L, TYPE>,
    ) -> &mut (K, V) {
        let entry_ptr = if let Some(link) = link_ref(self.link_ptr) {
            link.data_block.entry_ptr(self.index)
        } else {
            data_block_ref(data_block).entry_ptr(self.index)
        };
        unsafe { &mut (*entry_ptr.cast_mut()) }
    }

    /// Moves the [`EntryPtr`] to point to the next occupied entry.
    ///
    /// Returns `true` if it successfully found the next occupied entry.
    #[inline]
    pub(crate) fn find_next<L: LruList>(&mut self, bucket: &Bucket<K, V, L, TYPE>) -> bool {
        if self.link_ptr.is_null() && self.next_entry::<L, BUCKET_LEN>(&bucket.metadata) {
            return true;
        }
        while let Some(link) = link_ref(self.link_ptr) {
            if self.next_entry::<L, LINKED_BUCKET_LEN>(&link.metadata) {
                return true;
            }
        }
        false
    }

    /// Unlinks the [`LinkedBucket`] currently pointed to by this [`EntryPtr`] from the linked list.
    ///
    /// The associated [`Bucket`] must be locked.
    fn unlink(&mut self, link_head: &AtomicShared<LinkedBucket<K, V>>) {
        debug_assert_ne!(TYPE, INDEX);

        let prev_link_ptr =
            link_ref(self.link_ptr).map_or(ptr::null_mut(), |link| link.prev_link.load(Relaxed));
        let next = link_ref(self.link_ptr)
            .and_then(|link| link.metadata.link.swap((None, Tag::None), Relaxed).0);
        if let Some(next) = next.as_ref() {
            // Move the pointer to the next `Link`.
            next.prev_link.store(prev_link_ptr, Relaxed);
            self.link_ptr = next.as_ptr();
            self.index = u32::MAX;
        } else {
            // Move the pointer to the previous `Link`.
            self.link_ptr = prev_link_ptr;
            self.index = u32::MAX - 1;
        }

        let unlinked = if let Some(prev) = link_ref(prev_link_ptr) {
            prev.metadata.link.swap((next, Tag::None), Acquire).0
        } else {
            link_head.swap((next, Tag::None), Acquire).0
        };
        if let Some(link) = unlinked {
            let dropped = unsafe { link.drop_in_place() };
            debug_assert!(dropped);
        }
    }

    /// Moves this [`EntryPtr`] to the next occupied entry in the [`Bucket`].
    ///
    /// Returns `false` if this currently points to the last entry.
    #[inline]
    fn next_entry<L: LruList, const LEN: usize>(&mut self, metadata: &Metadata<K, V, LEN>) -> bool {
        // Search for the next occupied entry.
        let current_index = if self.index == u32::MAX {
            0
        } else {
            self.index + 1
        };

        if (current_index as usize) < LEN {
            let bitmap = metadata.bitmap::<TYPE>() & (!((1_u32 << current_index) - 1));
            let next_index = bitmap.trailing_zeros();
            if (next_index as usize) < LEN {
                self.index = next_index;
                return true;
            }
        }

        self.link_ptr = metadata.load_link();
        self.index = u32::MAX;

        false
    }
}

unsafe impl<K: Send, V: Send, const TYPE: char> Send for EntryPtr<K, V, TYPE> {}
unsafe impl<K: Send + Sync, V: Send + Sync, const TYPE: char> Sync for EntryPtr<K, V, TYPE> {}

impl LruList for () {}

impl DoublyLinkedList {
    /// Reads the slot.
    #[inline]
    fn read(&self, index: u32) -> (u8, u8) {
        unsafe { *self.0.get_unchecked(index as usize).get() }
    }

    /// Writes the slot.
    #[inline]
    fn write<R, F: FnOnce(&mut (u8, u8)) -> R>(&self, index: u32, f: F) -> R {
        unsafe { f(&mut *self.0.get_unchecked(index as usize).get()) }
    }
}

impl LruList for DoublyLinkedList {
    #[inline]
    fn evict(&self, tail: u32) -> Option<(u8, u32)> {
        if tail == 0 {
            None
        } else {
            let lru = self.read(tail - 1).0;
            let new_tail = if tail - 1 == u32::from(lru) {
                // Reset the linked list.
                0
            } else {
                let new_lru = self.read(u32::from(lru)).0;
                {
                    #![allow(clippy::cast_possible_truncation)]
                    self.write(u32::from(new_lru), |v| {
                        v.1 = tail as u8 - 1;
                    });
                }
                self.write(tail - 1, |v| {
                    v.0 = new_lru;
                });
                tail
            };
            self.write(u32::from(lru), |v| {
                *v = (0, 0);
            });
            Some((lru, new_tail))
        }
    }

    #[inline]
    fn remove(&self, tail: u32, entry: u8) -> Option<u32> {
        if tail == 0
            || (self.read(u32::from(entry)) == (0, 0)
                && (self.read(0) != (entry, entry) || (tail != 1 && tail != u32::from(entry) + 1)))
        {
            // The linked list is empty, or the entry is not a part of the linked list.
            return None;
        }

        if self.read(u32::from(entry)).0 == entry {
            // It is the head and the only entry of the linked list.
            debug_assert_eq!(tail, u32::from(entry) + 1);
            self.write(u32::from(entry), |v| {
                *v = (0, 0);
            });
            return Some(0);
        }

        // Adjust `prev -> current`.
        let (prev, next) = self.read(u32::from(entry));
        debug_assert_eq!(self.read(u32::from(prev)).1, entry);
        self.write(u32::from(prev), |v| {
            v.1 = next;
        });

        // Adjust `next -> current`.
        debug_assert_eq!(self.read(u32::from(next)).0, entry);
        self.write(u32::from(next), |v| {
            v.0 = prev;
        });

        let new_tail = if tail == u32::from(entry) + 1 {
            // Update `head`.
            Some(u32::from(next) + 1)
        } else {
            None
        };
        self.write(u32::from(entry), |v| {
            *v = (0, 0);
        });

        new_tail
    }

    #[inline]
    fn promote(&self, tail: u32, entry: u8) -> Option<u32> {
        if tail == u32::from(entry) + 1 {
            // Nothing to do.
            return None;
        } else if tail == 0 {
            // The linked list is empty.
            self.write(u32::from(entry), |v| {
                *v = (entry, entry);
            });
            return Some(u32::from(entry) + 1);
        }

        // Remove the entry from the linked list only if it is a part of it.
        if self.read(u32::from(entry)) != (0, 0) || (self.read(0) == (entry, entry) && tail == 1) {
            // Adjust `prev -> current`.
            let (prev, next) = self.read(u32::from(entry));
            debug_assert_eq!(self.read(u32::from(prev)).1, entry);
            self.write(u32::from(prev), |v| {
                v.1 = next;
            });

            // Adjust `next -> current`.
            debug_assert_eq!(self.read(u32::from(next)).0, entry);
            self.write(u32::from(next), |v| {
                v.0 = prev;
            });
        }

        // Adjust `oldest -> head`.
        let oldest = self.read(tail - 1).0;
        debug_assert_eq!(u32::from(self.read(u32::from(oldest)).1) + 1, tail);
        self.write(u32::from(oldest), |v| {
            v.1 = entry;
        });
        self.write(u32::from(entry), |v| {
            v.0 = oldest;
        });

        // Adjust `head -> new head`
        self.write(tail - 1, |v| {
            v.0 = entry;
        });
        {
            #![allow(clippy::cast_possible_truncation)]
            self.write(u32::from(entry), |v| {
                v.1 = tail as u8 - 1;
            });
        }

        // Update `head`.
        Some(u32::from(entry) + 1)
    }
}

unsafe impl Send for DoublyLinkedList {}
unsafe impl Sync for DoublyLinkedList {}

impl<K, V, const LEN: usize> Metadata<K, V, LEN> {
    /// Returns the partial hash at the given index.
    #[inline]
    const fn read_partial_hash(&self, index: u32) -> u8 {
        unsafe {
            self.partial_hash_array
                .get()
                .cast::<u8>()
                .add(index as usize)
                .read()
        }
    }

    /// Updates the partial hash at the given index.
    #[inline]
    const fn update_partial_hash(&self, index: u32, partial_hash: u8) {
        unsafe {
            self.partial_hash_array
                .get()
                .cast::<u8>()
                .add(index as usize)
                .write(partial_hash);
        }
    }

    /// Returns a bitmap representing valid entries.
    #[inline]
    fn bitmap<const TYPE: char>(&self) -> u32 {
        if TYPE == INDEX {
            // Load order: `removed_bitmap` -> `acquire` -> `occupied_bitmap`.
            !self.removed_bitmap.load(Acquire) & self.occupied_bitmap.load(Acquire)
        } else {
            self.occupied_bitmap.load(Relaxed)
        }
    }

    /// Loads the linked bucket pointer.
    #[inline]
    fn load_link(&self) -> *const LinkedBucket<K, V> {
        unsafe { self.link.load(Acquire, fake_ref(&self)).as_ptr_unchecked() }
    }
}

unsafe impl<K: Send, V: Send, const LEN: usize> Send for Metadata<K, V, LEN> {}
unsafe impl<K: Send + Sync, V: Send + Sync, const LEN: usize> Sync for Metadata<K, V, LEN> {}

impl<K, V> LinkedBucket<K, V> {
    /// Creates an empty [`LinkedBucket`].
    #[inline]
    fn new(next: Option<Shared<LinkedBucket<K, V>>>) -> Shared<Self> {
        unsafe {
            Shared::new_with_unchecked(|| {
                let mut bucket = Self {
                    metadata: Metadata {
                        occupied_bitmap: AtomicU32::default(),
                        removed_bitmap: AtomicU32::default(),
                        partial_hash_array: UnsafeCell::new(Default::default()),
                        link: AtomicShared::default(),
                    },
                    prev_link: AtomicPtr::default(),
                    data_block: DataBlock::new(),
                };
                if let Some(next) = next {
                    bucket.metadata.link = AtomicShared::from(next);
                }
                bucket
            })
        }
    }
}

impl<K, V> Drop for LinkedBucket<K, V> {
    #[inline]
    fn drop(&mut self) {
        if needs_drop::<(K, V)>() {
            let mut occupied_bitmap = self.metadata.occupied_bitmap.load(Relaxed);
            while occupied_bitmap != 0 {
                let index = occupied_bitmap.trailing_zeros();
                self.data_block.drop_entry(index);
                occupied_bitmap -= 1_u32 << index;
            }
        }
    }
}

/// Returns a pointer to a bucket.
#[inline]
const fn bucket_ptr<K, V, L: LruList, const TYPE: char>(
    bucket: &Bucket<K, V, L, TYPE>,
) -> NonNull<Bucket<K, V, L, TYPE>> {
    unsafe { NonNull::new_unchecked(from_ref(bucket).cast_mut()) }
}

/// Returns a reference to the data block.
#[inline]
const fn data_block_ref<'l, K, V, const LEN: usize>(
    data_block_ptr: NonNull<DataBlock<K, V, LEN>>,
) -> &'l DataBlock<K, V, LEN> {
    unsafe { data_block_ptr.as_ref() }
}

/// Returns a reference to the linked bucket that the pointer might point to.
#[inline]
const fn link_ref<'l, K, V>(ptr: *const LinkedBucket<K, V>) -> Option<&'l LinkedBucket<K, V>> {
    unsafe { ptr.as_ref() }
}

/// Returns a fake reference for passing a reference to `U` when it is ensured that the returned
/// reference is never used.
#[inline]
const fn fake_ref<'l, T, U>(v: &T) -> &'l U {
    unsafe { &*ptr::from_ref(v).cast::<U>() }
}

#[cfg(not(feature = "loom"))]
#[cfg(test)]
mod test {
    use super::*;

    use std::sync::atomic::AtomicPtr;
    use std::sync::atomic::Ordering::Relaxed;

    use proptest::prelude::*;
    use sdd::Shared;
    use tokio::sync::Barrier;

    #[cfg(not(miri))]
    static_assertions::assert_eq_size!(Bucket<String, String, (), MAP>, [u8; BUCKET_LEN * 2]);
    #[cfg(not(miri))]
    static_assertions::assert_eq_size!(Bucket<String, String, DoublyLinkedList, CACHE>, [u8; BUCKET_LEN * 4]);

    proptest! {
        #[cfg_attr(miri, ignore)]
        #[test]
        fn evict_untracked(xs in 0..BUCKET_LEN * 2) {
            let data_block: DataBlock<usize, usize, BUCKET_LEN> = DataBlock::new();
            let data_block_ptr =
                unsafe { NonNull::new_unchecked(from_ref(&data_block).cast_mut()) };
            let bucket: Bucket<usize, usize, DoublyLinkedList, CACHE> = Bucket::new();
            for v in 0..xs {
                let writer = Writer::lock_sync(&bucket).unwrap();
                let evicted = writer.evict_lru_head(data_block_ptr);
                assert_eq!(v >= BUCKET_LEN, evicted.is_some());
                writer.insert(data_block_ptr, 0, (v, v));
                assert_eq!(writer.metadata.removed_bitmap.load(Relaxed), 0);
            }
        }

        #[cfg_attr(miri, ignore)]
        #[test]
        fn evict_overflowed(xs in 1..BUCKET_LEN * 2) {
            let data_block: DataBlock<usize, usize, BUCKET_LEN> = DataBlock::new();
            let data_block_ptr =
                unsafe { NonNull::new_unchecked(from_ref(&data_block).cast_mut()) };
            let bucket: Bucket<usize, usize, DoublyLinkedList, CACHE> = Bucket::new();
            let writer = Writer::lock_sync(&bucket).unwrap();
            for _ in 0..3 {
                for v in 0..xs {
                    let entry_ptr = writer.insert(data_block_ptr, 0, (v, v));
                    writer.update_lru_tail(&entry_ptr);
                    if v < BUCKET_LEN {
                        assert_eq!(
                            writer.metadata.removed_bitmap.load(Relaxed) as usize,
                            v + 1
                        );
                    }
                    assert_eq!(
                        writer.lru_list.read
                            (writer.metadata.removed_bitmap.load(Relaxed) - 1)
                            .0,
                        0
                    );
                }

                let mut evicted_key = None;
                if xs >= BUCKET_LEN {
                    let evicted = writer.evict_lru_head(data_block_ptr);
                    assert!(evicted.is_some());
                    evicted_key = evicted.map(|(k, _)| k);
                }
                assert_ne!(writer.metadata.removed_bitmap.load(Relaxed), 0);

                for v in 0..xs {
                    let mut entry_ptr = writer.get_entry_ptr(data_block_ptr, &v, 0);
                    if entry_ptr.is_valid() {
                        let _erased = writer.remove(data_block_ptr, &mut entry_ptr);
                    } else {
                        assert_eq!(v, evicted_key.unwrap());
                    }
                }
                assert_eq!(writer.metadata.removed_bitmap.load(Relaxed), 0);
            }
        }

        #[cfg_attr(miri, ignore)]
        #[test]
        fn evict_tracked(xs in 0..BUCKET_LEN * 2) {
            let data_block: DataBlock<usize, usize, BUCKET_LEN> = DataBlock::new();
            let data_block_ptr =
                unsafe { NonNull::new_unchecked(from_ref(&data_block).cast_mut()) };
            let bucket: Bucket<usize, usize, DoublyLinkedList, CACHE> = Bucket::new();
            for v in 0..xs {
                let writer = Writer::lock_sync(&bucket).unwrap();
                let evicted = writer.evict_lru_head(data_block_ptr);
                assert_eq!(v >= BUCKET_LEN, evicted.is_some());
                let mut entry_ptr = writer.insert(data_block_ptr, 0, (v, v));
                writer.update_lru_tail(&entry_ptr);
                assert_eq!(
                    writer.metadata.removed_bitmap.load(Relaxed),
                    entry_ptr.index + 1
                );
                if v >= BUCKET_LEN {
                    entry_ptr.index = u32::try_from(xs % BUCKET_LEN).unwrap_or(0);
                    writer.update_lru_tail(&entry_ptr);
                    assert_eq!(
                        writer.metadata.removed_bitmap.load(Relaxed),
                        entry_ptr.index + 1
                    );
                    let mut iterated = 1;
                    let mut i = u32::from(writer.lru_list.read(entry_ptr.index).1);
                    while i != entry_ptr.index {
                        iterated += 1;
                        i = u32::from(writer.lru_list.read(i).1);
                    }
                    assert_eq!(iterated, BUCKET_LEN);
                    iterated = 1;
                    i = u32::from(writer.lru_list.read(entry_ptr.index).0);
                    while i != entry_ptr.index {
                        iterated += 1;
                        i = u32::from(writer.lru_list.read(i).0);
                    }
                    assert_eq!(iterated, BUCKET_LEN);
                }
            }
        }

        #[cfg_attr(miri, ignore)]
        #[test]
        fn removed(xs in 0..BUCKET_LEN) {
            let data_block: DataBlock<usize, usize, BUCKET_LEN> = DataBlock::new();
            let data_block_ptr =
                unsafe { NonNull::new_unchecked(from_ref(&data_block).cast_mut()) };
            let bucket: Bucket<usize, usize, DoublyLinkedList, CACHE> = Bucket::new();
            for v in 0..xs {
                let writer = Writer::lock_sync(&bucket).unwrap();
                let entry_ptr = writer.insert(data_block_ptr, 0, (v, v));
                writer.update_lru_tail(&entry_ptr);
                let mut iterated = 1;
                let mut i = u32::from(writer.lru_list.read(entry_ptr.index).1);
                while i != entry_ptr.index {
                    iterated += 1;
                    i = u32::from(writer.lru_list.read(i).1);
                }
                assert_eq!(iterated, v + 1);
            }
            for v in 0..xs {
                let writer = Writer::lock_sync(&bucket).unwrap();
                let data_block_ptr =
                    unsafe { NonNull::new_unchecked(from_ref(&data_block).cast_mut()) };
                let entry_ptr = writer.get_entry_ptr(data_block_ptr, &v, 0);
                let mut iterated = 1;
                let mut i = u32::from(writer.lru_list.read(entry_ptr.index).1);
                while i != entry_ptr.index {
                    iterated += 1;
                    i = u32::from(writer.lru_list.read(i).1);
                }
                assert_eq!(iterated, xs - v);
                writer.remove_from_lru_list(&entry_ptr);
            }
            assert_eq!(bucket.metadata.removed_bitmap.load(Relaxed), 0);
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test(flavor = "multi_thread", worker_threads = 16)]
    async fn bucket_lock_sync() {
        let num_tasks = BUCKET_LEN + 2;
        let barrier = Shared::new(Barrier::new(num_tasks));
        let data_block: Shared<DataBlock<usize, usize, BUCKET_LEN>> = Shared::new(DataBlock::new());
        let mut bucket: Shared<Bucket<usize, usize, (), MAP>> = Shared::new(Bucket::new());
        let mut data: [u64; 128] = [0; 128];
        let mut task_handles = Vec::with_capacity(num_tasks);
        for task_id in 0..num_tasks {
            let barrier_clone = barrier.clone();
            let data_block_clone = data_block.clone();
            let bucket_clone = bucket.clone();
            let data_ptr = AtomicPtr::new(&raw mut data);
            task_handles.push(tokio::spawn(async move {
                barrier_clone.wait().await;
                let partial_hash = (task_id % BUCKET_LEN).try_into().unwrap();
                for i in 0..2048 {
                    let writer = Writer::lock_sync(&bucket_clone).unwrap();
                    let mut sum: u64 = 0;
                    for j in 0..128 {
                        unsafe {
                            sum += (*data_ptr.load(Relaxed))[j];
                            (*data_ptr.load(Relaxed))[j] = if i % 4 == 0 { 2 } else { 4 }
                        };
                    }
                    assert_eq!(sum % 256, 0);
                    let data_block_ptr =
                        unsafe { NonNull::new_unchecked(data_block_clone.as_ptr().cast_mut()) };
                    if i == 0 {
                        assert!(
                            writer
                                .insert(data_block_ptr, partial_hash, (task_id, 0))
                                .is_valid()
                        );
                    } else {
                        assert_eq!(
                            writer
                                .search_entry(data_block_ptr, &task_id, partial_hash)
                                .unwrap(),
                            &(task_id, 0_usize)
                        );
                    }
                    drop(writer);

                    let reader = Reader::lock_sync(&*bucket_clone).unwrap();
                    assert_eq!(
                        reader
                            .search_entry(data_block_ptr, &task_id, partial_hash)
                            .unwrap(),
                        &(task_id, 0_usize)
                    );
                }
            }));
        }
        for r in futures::future::join_all(task_handles).await {
            assert!(r.is_ok());
        }

        let sum: u64 = data.iter().sum();
        assert_eq!(sum % 256, 0);
        assert_eq!(bucket.len(), num_tasks);

        let data_block_ptr = unsafe { NonNull::new_unchecked(data_block.as_ptr().cast_mut()) };
        for task_id in 0..num_tasks {
            assert_eq!(
                bucket.search_entry(
                    data_block_ptr,
                    &task_id,
                    (task_id % BUCKET_LEN).try_into().unwrap(),
                ),
                Some(&(task_id, 0))
            );
        }

        let mut count = 0;
        let mut entry_ptr = EntryPtr::null();
        while entry_ptr.find_next(&bucket) {
            count += 1;
        }
        assert_eq!(bucket.len(), count);

        entry_ptr = EntryPtr::null();
        let writer = Writer::lock_sync(&bucket).unwrap();
        while entry_ptr.find_next(&writer) {
            writer.remove(
                unsafe { NonNull::new_unchecked(data_block.as_ptr().cast_mut()) },
                &mut entry_ptr,
            );
        }
        assert_eq!(writer.len(), 0);
        writer.kill();

        assert_eq!(bucket.len(), 0);
        assert!(Writer::lock_sync(unsafe { bucket.get_mut().unwrap() }).is_none());
    }
}
