//! [`HashIndex`] is a read-optimized asynchronous/concurrent hash map.

use std::collections::hash_map::RandomState;
use std::fmt::{self, Debug};
use std::hash::{BuildHasher, Hash};
use std::iter::FusedIterator;
use std::ops::{Deref, RangeInclusive};
use std::panic::UnwindSafe;
use std::ptr;
use std::sync::atomic::AtomicU8;
#[cfg(not(feature = "loom"))]
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Relaxed, Release};

#[cfg(feature = "loom")]
use loom::sync::atomic::AtomicUsize;
use sdd::{AtomicShared, Epoch, Guard, Shared, Tag};

use super::Equivalent;
use super::hash_table::bucket::{Bucket, EntryPtr, INDEX};
use super::hash_table::bucket_array::BucketArray;
use super::hash_table::{HashTable, LockedBucket};

/// Scalable asynchronous/concurrent hash index.
///
/// [`HashIndex`] is an asynchronous/concurrent hash map data structure optimized for parallel read
/// operations. The key characteristics of [`HashIndex`] are similar to those of
/// [`HashMap`](super::HashMap) except its read operations are lock-free.
///
/// ## The key differences between [`HashIndex`] and [`HashMap`](crate::HashMap).
///
/// * Lock-free read: read and scan operations are never blocked and do not modify shared data.
/// * Immutability: the data in the container is immutable until it becomes unreachable.
/// * Linearizability: linearizability of read operations relies on the CPU architecture.
///
/// ## The key statistics for [`HashIndex`]
///
/// * The expected size of metadata for a single key-value pair: 2 bytes.
/// * The expected number of atomic write operations required for an operation on a single key: 2.
/// * The expected number of atomic variables accessed during a single key operation: 2.
/// * The number of entries managed by a single bucket without a linked list: 32.
/// * The expected maximum linked list length when a resize is triggered: log(capacity) / 8.
///
/// ## Unwind safety
///
/// [`HashIndex`] is impervious to out-of-memory errors and panics in user-specified code under one
/// condition: `H::Hasher::hash`, `K::drop` and `V::drop` must not panic.
pub struct HashIndex<K, V, H = RandomState>
where
    H: BuildHasher,
{
    bucket_array: AtomicShared<BucketArray<K, V, (), INDEX>>,
    build_hasher: H,
    garbage_chain: AtomicShared<BucketArray<K, V, (), INDEX>>,
    garbage_epoch: AtomicU8,
    minimum_capacity: AtomicUsize,
}

/// [`Entry`] represents a single entry in a [`HashIndex`].
pub enum Entry<'h, K, V, H = RandomState>
where
    H: BuildHasher,
{
    /// An occupied entry.
    Occupied(OccupiedEntry<'h, K, V, H>),
    /// A vacant entry.
    Vacant(VacantEntry<'h, K, V, H>),
}

/// [`OccupiedEntry`] is a view into an occupied entry in a [`HashIndex`].
pub struct OccupiedEntry<'h, K, V, H = RandomState>
where
    H: BuildHasher,
{
    hashindex: &'h HashIndex<K, V, H>,
    locked_bucket: LockedBucket<K, V, (), INDEX>,
    entry_ptr: EntryPtr<K, V, INDEX>,
}

/// [`VacantEntry`] is a view into a vacant entry in a [`HashIndex`].
pub struct VacantEntry<'h, K, V, H = RandomState>
where
    H: BuildHasher,
{
    hashindex: &'h HashIndex<K, V, H>,
    key: K,
    hash: u64,
    locked_bucket: LockedBucket<K, V, (), INDEX>,
}

/// [`Reserve`] keeps the capacity of the associated [`HashIndex`] higher than a certain level.
///
/// The [`HashIndex`] does not shrink the capacity below the reserved capacity.
pub struct Reserve<'h, K, V, H = RandomState>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    hashindex: &'h HashIndex<K, V, H>,
    additional: usize,
}

/// An iterator over the entries of a [`HashIndex`].
///
/// An [`Iter`] iterates over all the entries that survive the [`Iter`].
pub struct Iter<'h, K, V, H = RandomState>
where
    H: BuildHasher,
{
    hashindex: &'h HashIndex<K, V, H>,
    bucket_array: Option<&'h BucketArray<K, V, (), INDEX>>,
    index: usize,
    bucket: Option<&'h Bucket<K, V, (), INDEX>>,
    entry_ptr: EntryPtr<K, V, INDEX>,
    guard: &'h Guard,
}

impl<K, V, H> HashIndex<K, V, H>
where
    H: BuildHasher,
{
    /// Creates an empty [`HashIndex`] with the given [`BuildHasher`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    /// use std::collections::hash_map::RandomState;
    ///
    /// let hashindex: HashIndex<u64, u32, RandomState> =
    ///     HashIndex::with_hasher(RandomState::new());
    /// ```
    #[cfg(not(feature = "loom"))]
    #[inline]
    pub const fn with_hasher(build_hasher: H) -> Self {
        Self {
            bucket_array: AtomicShared::null(),
            build_hasher,
            garbage_chain: AtomicShared::null(),
            garbage_epoch: AtomicU8::new(u8::MAX),
            minimum_capacity: AtomicUsize::new(0),
        }
    }

    /// Creates an empty [`HashIndex`] with the given [`BuildHasher`].
    #[cfg(feature = "loom")]
    #[inline]
    pub fn with_hasher(build_hasher: H) -> Self {
        Self {
            bucket_array: AtomicShared::null(),
            build_hasher,
            garbage_chain: AtomicShared::null(),
            garbage_epoch: AtomicU8::new(u8::MAX),
            minimum_capacity: AtomicUsize::new(0),
        }
    }

    /// Creates an empty [`HashIndex`] with the specified capacity and [`BuildHasher`].
    ///
    /// The actual capacity is equal to or greater than `capacity` unless it is greater than
    /// `1 << (usize::BITS - 1)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    /// use std::collections::hash_map::RandomState;
    ///
    /// let hashindex: HashIndex<u64, u32, RandomState> =
    ///     HashIndex::with_capacity_and_hasher(1000, RandomState::new());
    ///
    /// let result = hashindex.capacity();
    /// assert_eq!(result, 1024);
    /// ```
    #[inline]
    pub fn with_capacity_and_hasher(capacity: usize, build_hasher: H) -> Self {
        let (array, minimum_capacity) = if capacity == 0 {
            (AtomicShared::null(), AtomicUsize::new(0))
        } else {
            let array = unsafe {
                Shared::new_with_unchecked(|| {
                    BucketArray::<K, V, (), INDEX>::new(capacity, AtomicShared::null())
                })
            };
            let minimum_capacity = array.num_slots();
            (
                AtomicShared::from(array),
                AtomicUsize::new(minimum_capacity),
            )
        };
        Self {
            bucket_array: array,
            build_hasher,
            garbage_chain: AtomicShared::null(),
            garbage_epoch: AtomicU8::new(u8::MAX),
            minimum_capacity,
        }
    }
}

impl<K, V, H> HashIndex<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    /// Temporarily increases the minimum capacity of the [`HashIndex`].
    ///
    /// A [`Reserve`] is returned if the [`HashIndex`] could increase the minimum capacity while
    /// the increased capacity is not exclusively owned by the returned [`Reserve`], allowing
    /// others to benefit from it. The memory for the additional space may not be immediately
    /// allocated if the [`HashIndex`] is empty or currently being resized, however once the memory
    /// is reserved eventually, the capacity will not shrink below the additional capacity until
    /// the returned [`Reserve`] is dropped.
    ///
    /// # Errors
    ///
    /// Returns `None` if a too large number is given.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<usize, usize> = HashIndex::with_capacity(1000);
    /// assert_eq!(hashindex.capacity(), 1024);
    ///
    /// let reserved = hashindex.reserve(10000);
    /// assert!(reserved.is_some());
    /// assert_eq!(hashindex.capacity(), 16384);
    ///
    /// assert!(hashindex.reserve(usize::MAX).is_none());
    /// assert_eq!(hashindex.capacity(), 16384);
    ///
    /// for i in 0..16 {
    ///     assert!(hashindex.insert_sync(i, i).is_ok());
    /// }
    /// drop(reserved);
    ///
    /// assert_eq!(hashindex.capacity(), 1024);
    /// ```
    #[inline]
    pub fn reserve(&self, additional_capacity: usize) -> Option<Reserve<'_, K, V, H>> {
        let additional = self.reserve_capacity(additional_capacity);
        if additional == 0 {
            None
        } else {
            Some(Reserve {
                hashindex: self,
                additional,
            })
        }
    }

    /// Gets the entry associated with the given key in the map for in-place manipulation.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<char, u32> = HashIndex::default();
    ///
    /// let future_entry = hashindex.entry_async('b');
    /// ```
    #[inline]
    pub async fn entry_async(&self, key: K) -> Entry<'_, K, V, H> {
        self.reclaim_memory();
        let hash = self.hash(&key);
        let locked_bucket = self.writer_async(hash).await;
        let entry_ptr = locked_bucket.search(&key, hash);
        if entry_ptr.is_valid() {
            Entry::Occupied(OccupiedEntry {
                hashindex: self,
                locked_bucket,
                entry_ptr,
            })
        } else {
            let vacant_entry = VacantEntry {
                hashindex: self,
                key,
                hash,
                locked_bucket,
            };
            Entry::Vacant(vacant_entry)
        }
    }

    /// Gets the entry associated with the given key in the map for in-place manipulation.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<char, u32> = HashIndex::default();
    ///
    /// for ch in "a short treatise on fungi".chars() {
    ///     unsafe {
    ///         hashindex.entry_sync(ch).and_modify(|counter| *counter += 1).or_insert(1);
    ///     }
    /// }
    ///
    /// assert_eq!(hashindex.peek_with(&'s', |_, v| *v), Some(2));
    /// assert_eq!(hashindex.peek_with(&'t', |_, v| *v), Some(3));
    /// assert!(hashindex.peek_with(&'y', |_, v| *v).is_none());
    /// ```
    #[inline]
    pub fn entry_sync(&self, key: K) -> Entry<'_, K, V, H> {
        let hash = self.hash(&key);
        let locked_bucket = self.writer_sync(hash);
        let entry_ptr = locked_bucket.search(&key, hash);
        if entry_ptr.is_valid() {
            Entry::Occupied(OccupiedEntry {
                hashindex: self,
                locked_bucket,
                entry_ptr,
            })
        } else {
            let vacant_entry = VacantEntry {
                hashindex: self,
                key,
                hash,
                locked_bucket,
            };
            Entry::Vacant(vacant_entry)
        }
    }

    /// Tries to get the entry associated with the given key in the map for in-place manipulation.
    ///
    /// Returns `None` if the entry could not be locked.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<usize, usize> = HashIndex::default();
    ///
    /// assert!(hashindex.insert_sync(0, 1).is_ok());
    /// assert!(hashindex.try_entry(0).is_some());
    /// ```
    #[inline]
    pub fn try_entry(&self, key: K) -> Option<Entry<'_, K, V, H>> {
        self.reclaim_memory();
        let hash = self.hash(&key);
        let guard = Guard::new();
        let locked_bucket = self.try_reserve_bucket(hash, &guard)?;
        let entry_ptr = locked_bucket.search(&key, hash);
        if entry_ptr.is_valid() {
            Some(Entry::Occupied(OccupiedEntry {
                hashindex: self,
                locked_bucket,
                entry_ptr,
            }))
        } else {
            Some(Entry::Vacant(VacantEntry {
                hashindex: self,
                key,
                hash,
                locked_bucket,
            }))
        }
    }

    /// Begins iterating over entries by getting the first occupied entry.
    ///
    /// The returned [`OccupiedEntry`] in combination with [`OccupiedEntry::next_async`] can act as
    /// a mutable iterator over entries.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<char, u32> = HashIndex::default();
    ///
    /// let future_entry = hashindex.begin_async();
    /// ```
    #[inline]
    pub async fn begin_async(&self) -> Option<OccupiedEntry<'_, K, V, H>> {
        self.any_async(|_, _| true).await
    }

    /// Begins iterating over entries by getting the first occupied entry.
    ///
    /// The returned [`OccupiedEntry`] in combination with [`OccupiedEntry::next_sync`] can act as a
    /// mutable iterator over entries.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// assert!(hashindex.insert_sync(1, 0).is_ok());
    ///
    /// let mut first_entry = hashindex.begin_sync().unwrap();
    /// unsafe {
    ///     *first_entry.get_mut() = 2;
    /// }
    ///
    /// assert!(first_entry.next_sync().is_none());
    /// assert_eq!(hashindex.peek_with(&1, |_, v| *v), Some(2));
    /// ```
    #[inline]
    pub fn begin_sync(&self) -> Option<OccupiedEntry<'_, K, V, H>> {
        self.any_sync(|_, _| true)
    }

    /// Finds any entry satisfying the supplied predicate for in-place manipulation.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// let future_entry = hashindex.any_async(|k, _| *k == 2);
    /// ```
    #[inline]
    pub async fn any_async<P: FnMut(&K, &V) -> bool>(
        &self,
        mut pred: P,
    ) -> Option<OccupiedEntry<'_, K, V, H>> {
        self.reclaim_memory();
        let mut entry = None;
        self.for_each_writer_async(0, 0, |locked_bucket, _| {
            let mut entry_ptr = EntryPtr::null();
            while entry_ptr.find_next(&locked_bucket.writer) {
                let (k, v) = locked_bucket.entry(&entry_ptr);
                if pred(k, v) {
                    entry = Some(OccupiedEntry {
                        hashindex: self,
                        locked_bucket,
                        entry_ptr,
                    });
                    return false;
                }
            }
            true
        })
        .await;
        entry
    }

    /// Finds any entry satisfying the supplied predicate for in-place manipulation.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// assert!(hashindex.insert_sync(1, 0).is_ok());
    /// assert!(hashindex.insert_sync(2, 3).is_ok());
    ///
    /// let mut entry = hashindex.any_sync(|k, _| *k == 2).unwrap();
    /// assert_eq!(*entry.get(), 3);
    /// ```
    #[inline]
    pub fn any_sync<P: FnMut(&K, &V) -> bool>(
        &self,
        mut pred: P,
    ) -> Option<OccupiedEntry<'_, K, V, H>> {
        self.reclaim_memory();
        let mut entry = None;
        let guard = Guard::new();
        self.for_each_writer_sync(0, 0, &guard, |locked_bucket, _| {
            let mut entry_ptr = EntryPtr::null();
            while entry_ptr.find_next(&locked_bucket.writer) {
                let (k, v) = locked_bucket.entry(&entry_ptr);
                if pred(k, v) {
                    entry = Some(OccupiedEntry {
                        hashindex: self,
                        locked_bucket,
                        entry_ptr,
                    });
                    return false;
                }
            }
            true
        });
        entry
    }

    /// Inserts a key-value pair into the [`HashIndex`].
    ///
    /// # Note
    ///
    /// If the key exists, the value is *not* updated.
    ///
    /// # Errors
    ///
    /// Returns an error containing the supplied key-value pair if the key exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    /// let future_insert = hashindex.insert_async(11, 17);
    /// ```
    #[inline]
    pub async fn insert_async(&self, key: K, val: V) -> Result<(), (K, V)> {
        self.reclaim_memory();
        let hash = self.hash(&key);
        let locked_bucket = self.writer_async(hash).await;
        if locked_bucket.search(&key, hash).is_valid() {
            Err((key, val))
        } else {
            locked_bucket.insert(hash, (key, val));
            Ok(())
        }
    }

    /// Inserts a key-value pair into the [`HashIndex`].
    ///
    /// # Note
    ///
    /// If the key exists, the value is *not* updated.
    ///
    /// # Errors
    ///
    /// Returns an error along with the supplied key-value pair if the key exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// assert!(hashindex.insert_sync(1, 0).is_ok());
    /// assert_eq!(hashindex.insert_sync(1, 1).unwrap_err(), (1, 1));
    /// ```
    #[inline]
    pub fn insert_sync(&self, key: K, val: V) -> Result<(), (K, V)> {
        self.reclaim_memory();
        let hash = self.hash(&key);
        let locked_bucket = self.writer_sync(hash);
        if locked_bucket.search(&key, hash).is_valid() {
            Err((key, val))
        } else {
            locked_bucket.insert(hash, (key, val));
            Ok(())
        }
    }

    /// Removes a key-value pair if the key exists.
    ///
    /// Returns `false` if the key does not exist.
    ///
    /// Returns `true` if the key existed and the condition was met after marking the entry
    /// unreachable; the memory will be reclaimed later.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    /// let future_insert = hashindex.insert_async(11, 17);
    /// let future_remove = hashindex.remove_async(&11);
    /// ```
    #[inline]
    pub async fn remove_async<Q>(&self, key: &Q) -> bool
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.remove_if_async(key, |_| true).await
    }

    /// Removes a key-value pair if the key exists.
    ///
    /// Returns `false` if the key does not exist.
    ///
    /// Returns `true` if the key existed and the condition was met after marking the entry
    /// unreachable; the memory will be reclaimed later.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// assert!(!hashindex.remove_sync(&1));
    /// assert!(hashindex.insert_sync(1, 0).is_ok());
    /// assert!(hashindex.remove_sync(&1));
    /// ```
    #[inline]
    pub fn remove_sync<Q>(&self, key: &Q) -> bool
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.remove_if_sync(key, |_| true)
    }

    /// Removes a key-value pair if the key exists and the given condition is met.
    ///
    /// Returns `false` if the key does not exist or the condition was not met.
    ///
    /// Returns `true` if the key existed and the condition was met after marking the entry
    /// unreachable; the memory will be reclaimed later.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    /// let future_insert = hashindex.insert_async(11, 17);
    /// let future_remove = hashindex.remove_if_async(&11, |_| true);
    /// ```
    #[inline]
    pub async fn remove_if_async<Q, F: FnOnce(&V) -> bool>(&self, key: &Q, condition: F) -> bool
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.reclaim_memory();
        let hash = self.hash(key);
        let Some(mut locked_bucket) = self.optional_writer_async(hash).await else {
            return false;
        };
        let mut entry_ptr = locked_bucket.search(key, hash);
        if entry_ptr.is_valid() && condition(&mut locked_bucket.entry_mut(&mut entry_ptr).1) {
            locked_bucket.mark_removed(self, &mut entry_ptr);
            true
        } else {
            false
        }
    }

    /// Removes a key-value pair if the key exists and the given condition is met.
    ///
    /// Returns `false` if the key does not exist or the condition was not met.
    ///
    /// Returns `true` if the key existed and the condition was met after marking the entry
    /// unreachable; the memory will be reclaimed later.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// assert!(hashindex.insert_sync(1, 0).is_ok());
    /// assert!(!hashindex.remove_if_sync(&1, |v| *v == 1));
    /// assert!(hashindex.remove_if_sync(&1, |v| *v == 0));
    /// ```
    #[inline]
    pub fn remove_if_sync<Q, F: FnOnce(&V) -> bool>(&self, key: &Q, condition: F) -> bool
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.reclaim_memory();
        let hash = self.hash(key);
        let Some(mut locked_bucket) = self.optional_writer_sync(hash) else {
            return false;
        };
        let mut entry_ptr = locked_bucket.search(key, hash);
        if entry_ptr.is_valid() && condition(&mut locked_bucket.entry_mut(&mut entry_ptr).1) {
            locked_bucket.mark_removed(self, &mut entry_ptr);
            true
        } else {
            false
        }
    }

    /// Gets an [`OccupiedEntry`] corresponding to the key for in-place modification.
    ///
    /// [`OccupiedEntry`] exclusively owns the entry, preventing others from gaining access to it:
    /// use [`peek`](Self::peek) if read-only access is sufficient.
    ///
    /// Returns `None` if the key does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    /// let future_insert = hashindex.insert_async(11, 17);
    /// let future_get = hashindex.get_async(&11);
    /// ```
    #[inline]
    pub async fn get_async<Q>(&self, key: &Q) -> Option<OccupiedEntry<'_, K, V, H>>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.reclaim_memory();
        let hash = self.hash(key);
        let locked_bucket = self.optional_writer_async(hash).await?;
        let entry_ptr = locked_bucket.search(key, hash);
        if entry_ptr.is_valid() {
            return Some(OccupiedEntry {
                hashindex: self,
                locked_bucket,
                entry_ptr,
            });
        }
        None
    }

    /// Gets an [`OccupiedEntry`] corresponding to the key for in-place modification.
    ///
    /// [`OccupiedEntry`] exclusively owns the entry, preventing others from gaining access to it:
    /// use [`peek`](Self::peek) if read-only access is sufficient.
    ///
    /// Returns `None` if the key does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// assert!(hashindex.get_sync(&1).is_none());
    /// assert!(hashindex.insert_sync(1, 10).is_ok());
    /// assert_eq!(*hashindex.get_sync(&1).unwrap().get(), 10);
    /// assert_eq!(*hashindex.get_sync(&1).unwrap(), 10);
    /// ```
    #[inline]
    pub fn get_sync<Q>(&self, key: &Q) -> Option<OccupiedEntry<'_, K, V, H>>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.reclaim_memory();
        let hash = self.hash(key);
        let locked_bucket = self.optional_writer_sync(hash)?;
        let entry_ptr = locked_bucket.search(key, hash);
        if entry_ptr.is_valid() {
            return Some(OccupiedEntry {
                hashindex: self,
                locked_bucket,
                entry_ptr,
            });
        }
        None
    }

    /// Returns a guarded reference to the value for the specified key without acquiring locks.
    ///
    /// Returns `None` if the key does not exist.
    ///
    /// # Note
    ///
    /// The closure may see an old snapshot of the value if the entry has recently been relocated
    /// due to resizing. This means that the effects of interior mutability, e.g., `Mutex<T>` or
    /// `UnsafeCell<T>`, may not be observable in the closure.
    ///
    /// # Safety
    ///
    /// This method is safe to use if the value is not modified or if `V` is a pointer type such as
    /// `Box<T>` or `Arc<T>`. However, it is unsafe if `V` contains a pointer that can be modified
    /// through interior mutability.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// use sdd::Guard;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// assert!(hashindex.insert_sync(1, 10).is_ok());
    ///
    /// let guard = Guard::new();
    /// let value_ref = hashindex.peek(&1, &guard).unwrap();
    /// assert_eq!(*value_ref, 10);
    /// ```
    #[inline]
    pub fn peek<'h, Q>(&'h self, key: &Q, guard: &'h Guard) -> Option<&'h V>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.reclaim_memory();
        self.peek_entry(key, guard).map(|(_, v)| v)
    }

    /// Peeks a key-value pair without acquiring locks.
    ///
    /// Returns `None` if the key does not exist.
    ///
    /// # Note
    ///
    /// The closure may see an old snapshot of the value if the entry has recently been relocated
    /// due to resizing. This means that the effects of interior mutability, e.g., `Mutex<T>` or
    /// `UnsafeCell<T>`, may not be observable in the closure.
    ///
    /// # Safety
    ///
    /// This method is safe to use if the value is not modified or if `V` is a pointer type such as
    /// `Box<T>` or `Arc<T>`. However, it is unsafe if `V` contains a pointer that can be modified
    /// through interior mutability.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// assert!(hashindex.peek_with(&1, |_, v| *v).is_none());
    /// assert!(hashindex.insert_sync(1, 10).is_ok());
    /// assert_eq!(hashindex.peek_with(&1, |_, v| *v).unwrap(), 10);
    /// ```
    #[inline]
    pub fn peek_with<Q, R, F: FnOnce(&K, &V) -> R>(&self, key: &Q, reader: F) -> Option<R>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.reclaim_memory();
        let guard = Guard::new();
        self.peek_entry(key, &guard).map(|(k, v)| reader(k, v))
    }

    /// Returns `true` if the [`HashIndex`] contains a value for the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// assert!(!hashindex.contains(&1));
    /// assert!(hashindex.insert_sync(1, 0).is_ok());
    /// assert!(hashindex.contains(&1));
    /// ```
    #[inline]
    pub fn contains<Q>(&self, key: &Q) -> bool
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.peek_with(key, |_, _| ()).is_some()
    }

    /// Iterates over entries asynchronously for reading.
    ///
    /// Stops iterating when the closure returns `false`, and this method also returns `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u64> = HashIndex::default();
    ///
    /// assert!(hashindex.insert_sync(1, 0).is_ok());
    ///
    /// async {
    ///     let result = hashindex.iter_async(|k, v| {
    ///         false
    ///     }).await;
    ///     assert!(!result);
    /// };
    /// ```
    #[inline]
    pub async fn iter_async<F: FnMut(&K, &V) -> bool>(&self, mut f: F) -> bool {
        self.reclaim_memory();
        let mut result = true;
        self.for_each_reader_async(|reader, data_block| {
            let mut entry_ptr = EntryPtr::null();
            while entry_ptr.find_next(&reader) {
                let (k, v) = entry_ptr.get(data_block);
                if !f(k, v) {
                    result = false;
                    return false;
                }
            }
            true
        })
        .await;
        result
    }

    /// Iterates over entries synchronously for reading.
    ///
    /// Stops iterating when the closure returns `false`, and this method also returns `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u64> = HashIndex::default();
    ///
    /// assert!(hashindex.insert_sync(1, 0).is_ok());
    /// assert!(hashindex.insert_sync(2, 1).is_ok());
    ///
    /// let mut acc = 0_u64;
    /// let result = hashindex.iter_sync(|k, v| {
    ///     acc += *k;
    ///     acc += *v;
    ///     true
    /// });
    ///
    /// assert!(result);
    /// assert_eq!(acc, 4);
    /// ```
    #[inline]
    pub fn iter_sync<F: FnMut(&K, &V) -> bool>(&self, mut f: F) -> bool {
        self.reclaim_memory();
        let mut result = true;
        let guard = Guard::new();
        self.for_each_reader_sync(&guard, |reader, data_block| {
            let mut entry_ptr = EntryPtr::null();
            while entry_ptr.find_next(&reader) {
                let (k, v) = entry_ptr.get(data_block);
                if !f(k, v) {
                    result = false;
                    return false;
                }
            }
            true
        });
        result
    }

    /// Retains the entries specified by the predicate.
    ///
    /// Entries that have existed since the invocation of the method are guaranteed to be visited
    /// if they are not removed, however the same entry can be visited more than once if the
    /// [`HashIndex`] gets resized by another thread.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// let future_insert = hashindex.insert_async(1, 0);
    /// let future_retain = hashindex.retain_async(|k, v| *k == 1);
    /// ```
    #[inline]
    pub async fn retain_async<F: FnMut(&K, &V) -> bool>(&self, mut pred: F) {
        self.reclaim_memory();
        self.for_each_writer_async(0, 0, |mut locked_bucket, removed| {
            let mut entry_ptr = EntryPtr::null();
            while entry_ptr.find_next(&locked_bucket.writer) {
                let (k, v) = locked_bucket.entry_mut(&mut entry_ptr);
                if !pred(k, v) {
                    locked_bucket
                        .writer
                        .mark_removed(&mut entry_ptr, &Guard::new());
                    *removed = true;
                }
            }
            true
        })
        .await;
    }

    /// Retains the entries specified by the predicate.
    ///
    /// Entries that have existed since the invocation of the method are guaranteed to be visited
    /// if they are not removed, however the same entry can be visited more than once if the
    /// [`HashIndex`] gets resized by another thread.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// assert!(hashindex.insert_sync(1, 0).is_ok());
    /// assert!(hashindex.insert_sync(2, 1).is_ok());
    /// assert!(hashindex.insert_sync(3, 2).is_ok());
    ///
    /// hashindex.retain_sync(|k, v| *k == 1 && *v == 0);
    ///
    /// assert!(hashindex.contains(&1));
    /// assert!(!hashindex.contains(&2));
    /// assert!(!hashindex.contains(&3));
    /// ```
    #[inline]
    pub fn retain_sync<F: FnMut(&K, &V) -> bool>(&self, mut pred: F) {
        self.reclaim_memory();
        let guard = Guard::new();
        self.for_each_writer_sync(0, 0, &guard, |mut locked_bucket, removed| {
            let mut entry_ptr = EntryPtr::null();
            while entry_ptr.find_next(&locked_bucket.writer) {
                let (k, v) = locked_bucket.entry_mut(&mut entry_ptr);
                if !pred(k, v) {
                    locked_bucket.writer.mark_removed(&mut entry_ptr, &guard);
                    *removed = true;
                }
            }
            true
        });
    }

    /// Clears the [`HashIndex`] by removing all key-value pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// let future_insert = hashindex.insert_async(1, 0);
    /// let future_retain = hashindex.clear_async();
    /// ```
    pub async fn clear_async(&self) {
        self.retain_async(|_, _| false).await;
    }

    /// Clears the [`HashIndex`] by removing all key-value pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// assert!(hashindex.insert_sync(1, 0).is_ok());
    /// hashindex.clear_sync();
    ///
    /// assert!(!hashindex.contains(&1));
    /// ```
    pub fn clear_sync(&self) {
        self.retain_sync(|_, _| false);
    }

    /// Returns the number of entries in the [`HashIndex`].
    ///
    /// It reads the entire metadata area of the bucket array to calculate the number of valid
    /// entries, making its time complexity `O(N)`. Furthermore, it may overcount entries if an old
    /// bucket array has yet to be dropped.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// assert!(hashindex.insert_sync(1, 0).is_ok());
    /// assert_eq!(hashindex.len(), 1);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.num_entries(&Guard::new())
    }

    /// Returns `true` if the [`HashIndex`] is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// assert!(hashindex.is_empty());
    /// assert!(hashindex.insert_sync(1, 0).is_ok());
    /// assert!(!hashindex.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        !self.has_entry(&Guard::new())
    }

    /// Returns the capacity of the [`HashIndex`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex_default: HashIndex<u64, u32> = HashIndex::default();
    /// assert_eq!(hashindex_default.capacity(), 0);
    ///
    /// assert!(hashindex_default.insert_sync(1, 0).is_ok());
    /// assert_eq!(hashindex_default.capacity(), 64);
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::with_capacity(1000);
    /// assert_eq!(hashindex.capacity(), 1024);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.num_slots(&Guard::new())
    }

    /// Returns the current capacity range of the [`HashIndex`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// assert_eq!(hashindex.capacity_range(), 0..=(1_usize << (usize::BITS - 2)));
    ///
    /// let reserved = hashindex.reserve(1000);
    /// assert_eq!(hashindex.capacity_range(), 1000..=(1_usize << (usize::BITS - 2)));
    /// ```
    #[inline]
    pub fn capacity_range(&self) -> RangeInclusive<usize> {
        self.minimum_capacity.load(Relaxed)..=self.maximum_capacity()
    }

    /// Returns the index of the bucket that may contain the key.
    ///
    /// The method returns the index of the bucket associated with the key. The number of buckets
    /// can be calculated by dividing the capacity by `32`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::with_capacity(1024);
    ///
    /// let bucket_index = hashindex.bucket_index(&11);
    /// assert!(bucket_index < hashindex.capacity() / 32);
    /// ```
    #[inline]
    pub fn bucket_index<Q>(&self, key: &Q) -> usize
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        let hash = self.hash(key);
        self.calculate_bucket_index(hash)
    }

    /// Returns an [`Iter`].
    ///
    /// It is guaranteed to go through all the key-value pairs pertaining in the [`HashIndex`]
    /// at the moment, however the same key-value pair can be visited more than once if the
    /// [`HashIndex`] is being resized.
    ///
    /// It requires the user to supply a reference to a [`Guard`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// use sdd::Guard;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// assert!(hashindex.insert_sync(1, 0).is_ok());
    ///
    /// let guard = Guard::new();
    ///
    /// let mut iter = hashindex.iter(&guard);
    /// let entry_ref = iter.next().unwrap();
    /// assert_eq!(iter.next(), None);
    ///
    /// for iter in hashindex.iter(&guard) {
    ///     assert_eq!(iter, (&1, &0));
    /// }
    /// ```
    #[inline]
    pub fn iter<'h>(&'h self, guard: &'h Guard) -> Iter<'h, K, V, H> {
        Iter {
            hashindex: self,
            bucket_array: None,
            index: 0,
            bucket: None,
            entry_ptr: EntryPtr::null(),
            guard,
        }
    }

    /// Reclaims memory by dropping all garbage bucket arrays if they are unreachable.
    #[inline]
    fn reclaim_memory(&self) {
        if !self.garbage_chain.is_null(Acquire) {
            self.dealloc_garbage();
        }
    }

    /// Deallocates garbage bucket arrays if they are unreachable.
    fn dealloc_garbage(&self) {
        let guard = Guard::new();
        let head_ptr = self.garbage_chain.load(Acquire, &guard);
        if head_ptr.is_null() {
            return;
        }
        let garbage_epoch = self.garbage_epoch.load(Acquire);
        if Epoch::try_from(garbage_epoch).is_ok_and(|e| !e.in_same_generation(guard.epoch())) {
            if let Ok((mut garbage_head, _)) = self.garbage_chain.compare_exchange(
                head_ptr,
                (None, Tag::None),
                Acquire,
                Relaxed,
                &guard,
            ) {
                while let Some(garbage_bucket_array) = garbage_head {
                    garbage_head = garbage_bucket_array
                        .linked_array_var()
                        .swap((None, Tag::None), Acquire)
                        .0;
                    let dropped = unsafe { garbage_bucket_array.drop_in_place() };
                    debug_assert!(dropped);
                }
            }
        } else {
            guard.set_has_garbage();
        }
    }
}

impl<K, V, H> Clone for HashIndex<K, V, H>
where
    K: Clone + Eq + Hash,
    V: Clone,
    H: BuildHasher + Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        let self_clone = Self::with_capacity_and_hasher(self.capacity(), self.hasher().clone());
        for (k, v) in self.iter(&Guard::new()) {
            let _result = self_clone.insert_sync(k.clone(), v.clone());
        }
        self_clone
    }
}

impl<K, V, H> Debug for HashIndex<K, V, H>
where
    K: Debug + Eq + Hash,
    V: Debug,
    H: BuildHasher,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.reclaim_memory();
        let guard = Guard::new();
        f.debug_map().entries(self.iter(&guard)).finish()
    }
}

impl<K, V> HashIndex<K, V, RandomState>
where
    K: Eq + Hash,
{
    /// Creates an empty default [`HashIndex`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::new();
    ///
    /// let result = hashindex.capacity();
    /// assert_eq!(result, 0);
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty [`HashIndex`] with the specified capacity.
    ///
    /// The actual capacity is equal to or greater than `capacity` unless it is greater than
    /// `1 << (usize::BITS - 1)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::with_capacity(1000);
    ///
    /// let result = hashindex.capacity();
    /// assert_eq!(result, 1024);
    /// ```
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, RandomState::new())
    }
}

impl<K, V, H> Default for HashIndex<K, V, H>
where
    H: BuildHasher + Default,
{
    /// Creates an empty default [`HashIndex`].
    ///
    /// The default capacity is `64`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// let result = hashindex.capacity();
    /// assert_eq!(result, 0);
    /// ```
    #[inline]
    fn default() -> Self {
        Self::with_hasher(H::default())
    }
}

impl<K, V, H> Drop for HashIndex<K, V, H>
where
    H: BuildHasher,
{
    #[inline]
    fn drop(&mut self) {
        self.bucket_array
            .swap((None, Tag::None), Relaxed)
            .0
            .map(|a| unsafe {
                // The entire array does not need to wait for an epoch change as no references will
                // remain outside the lifetime of the `HashIndex`.
                a.drop_in_place()
            });
        let mut garbage_head = self.garbage_chain.swap((None, Tag::None), Acquire).0;
        while let Some(garbage_bucket_array) = garbage_head {
            garbage_head = garbage_bucket_array
                .linked_array_var()
                .swap((None, Tag::None), Acquire)
                .0;
            let dropped = unsafe { garbage_bucket_array.drop_in_place() };
            debug_assert!(dropped);
        }
    }
}

impl<K, V, H> FromIterator<(K, V)> for HashIndex<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher + Default,
{
    #[inline]
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let into_iter = iter.into_iter();
        let hashindex = Self::with_capacity_and_hasher(
            Self::capacity_from_size_hint(into_iter.size_hint()),
            H::default(),
        );
        into_iter.for_each(|e| {
            let _result = hashindex.insert_sync(e.0, e.1);
        });
        hashindex
    }
}

impl<K, V, H> HashTable<K, V, H, (), INDEX> for HashIndex<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    #[inline]
    fn hasher(&self) -> &H {
        &self.build_hasher
    }

    #[inline]
    fn defer_reclaim(&self, bucket_array: Shared<BucketArray<K, V, (), INDEX>>, guard: &Guard) {
        guard.accelerate();
        self.reclaim_memory();
        self.garbage_epoch.swap(u8::from(guard.epoch()), Release);
        let (Some(prev_head), _) = self
            .garbage_chain
            .swap((Some(bucket_array.clone()), Tag::None), AcqRel)
        else {
            return;
        };
        // The bucket array will be dropped when the epoch enters the next generation.
        bucket_array
            .linked_array_var()
            .swap((Some(prev_head), Tag::None), Release);
    }

    #[inline]
    fn bucket_array_var(&self) -> &AtomicShared<BucketArray<K, V, (), INDEX>> {
        &self.bucket_array
    }

    #[inline]
    fn minimum_capacity_var(&self) -> &AtomicUsize {
        &self.minimum_capacity
    }
}

impl<K, V, H> PartialEq for HashIndex<K, V, H>
where
    K: Eq + Hash,
    V: PartialEq,
    H: BuildHasher,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.reclaim_memory();
        let guard = Guard::new();
        if !self
            .iter(&guard)
            .any(|(k, v)| other.peek_with(k, |_, ov| v == ov) != Some(true))
        {
            return !other
                .iter(&guard)
                .any(|(k, v)| self.peek_with(k, |_, sv| v == sv) != Some(true));
        }
        false
    }
}

impl<'h, K, V, H> Entry<'h, K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    /// Ensures a value is in the entry by inserting the supplied instance if empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// hashindex.entry_sync(3).or_insert(7);
    /// assert_eq!(hashindex.peek_with(&3, |_, v| *v), Some(7));
    /// ```
    #[inline]
    pub fn or_insert(self, val: V) -> OccupiedEntry<'h, K, V, H> {
        self.or_insert_with(|| val)
    }

    /// Ensures a value is in the entry by inserting the result of the supplied closure if empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// hashindex.entry_sync(19).or_insert_with(|| 5);
    /// assert_eq!(hashindex.peek_with(&19, |_, v| *v), Some(5));
    /// ```
    #[inline]
    pub fn or_insert_with<F: FnOnce() -> V>(self, constructor: F) -> OccupiedEntry<'h, K, V, H> {
        self.or_insert_with_key(|_| constructor())
    }

    /// Ensures a value is in the entry by inserting the result of the supplied closure if empty.
    ///
    /// The reference to the moved key is provided, therefore cloning or copying the key is
    /// unnecessary.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// hashindex.entry_sync(11).or_insert_with_key(|k| if *k == 11 { 7 } else { 3 });
    /// assert_eq!(hashindex.peek_with(&11, |_, v| *v), Some(7));
    /// ```
    #[inline]
    pub fn or_insert_with_key<F: FnOnce(&K) -> V>(
        self,
        constructor: F,
    ) -> OccupiedEntry<'h, K, V, H> {
        match self {
            Self::Occupied(o) => o,
            Self::Vacant(v) => {
                let val = constructor(v.key());
                v.insert_entry(val)
            }
        }
    }

    /// Returns a reference to the key of this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    /// assert_eq!(hashindex.entry_sync(31).key(), &31);
    /// ```
    #[inline]
    pub fn key(&self) -> &K {
        match self {
            Self::Occupied(o) => o.key(),
            Self::Vacant(v) => v.key(),
        }
    }

    /// Provides in-place mutable access to an occupied entry.
    ///
    /// # Safety
    ///
    /// The caller has to make sure that there are no readers of the entry, e.g., a reader keeping
    /// a reference to the entry via [`HashIndex::iter`], [`HashIndex::peek`], or
    /// [`HashIndex::peek_with`], unless an instance of `V` can be safely read when there is a
    /// single writer, e.g., `V = [u8; 32]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// unsafe {
    ///     hashindex.entry_sync(37).and_modify(|v| { *v += 1 }).or_insert(47);
    /// }
    /// assert_eq!(hashindex.peek_with(&37, |_, v| *v), Some(47));
    ///
    /// unsafe {
    ///     hashindex.entry_sync(37).and_modify(|v| { *v += 1 }).or_insert(3);
    /// }
    /// assert_eq!(hashindex.peek_with(&37, |_, v| *v), Some(48));
    /// ```
    #[inline]
    #[must_use]
    pub unsafe fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        unsafe {
            match self {
                Self::Occupied(mut o) => {
                    f(o.get_mut());
                    Self::Occupied(o)
                }
                Self::Vacant(_) => self,
            }
        }
    }
}

impl<'h, K, V, H> Entry<'h, K, V, H>
where
    K: Eq + Hash,
    V: Default,
    H: BuildHasher,
{
    /// Ensures a value is in the entry by inserting the default value if empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    /// hashindex.entry_sync(11).or_default();
    /// assert_eq!(hashindex.peek_with(&11, |_, v| *v), Some(0));
    /// ```
    #[inline]
    pub fn or_default(self) -> OccupiedEntry<'h, K, V, H> {
        match self {
            Self::Occupied(o) => o,
            Self::Vacant(v) => v.insert_entry(Default::default()),
        }
    }
}

impl<K, V, H> Debug for Entry<'_, K, V, H>
where
    K: Debug + Eq + Hash,
    V: Debug,
    H: BuildHasher,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Vacant(v) => f.debug_tuple("Entry").field(v).finish(),
            Self::Occupied(o) => f.debug_tuple("Entry").field(o).finish(),
        }
    }
}

impl<'h, K, V, H> OccupiedEntry<'h, K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    /// Gets a reference to the key in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// assert_eq!(hashindex.entry_sync(29).or_default().key(), &29);
    /// ```
    #[inline]
    #[must_use]
    pub fn key(&self) -> &K {
        &self.locked_bucket.entry(&self.entry_ptr).0
    }

    /// Marks that the entry is removed from the [`HashIndex`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    /// use scc::hash_index::Entry;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// hashindex.entry_sync(11).or_insert(17);
    ///
    /// if let Entry::Occupied(o) = hashindex.entry_sync(11) {
    ///     o.remove_entry();
    /// };
    /// assert_eq!(hashindex.peek_with(&11, |_, v| *v), None);
    /// ```
    #[inline]
    pub fn remove_entry(mut self) {
        self.locked_bucket
            .mark_removed(self.hashindex, &mut self.entry_ptr);
    }

    /// Gets a reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    /// use scc::hash_index::Entry;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// hashindex.entry_sync(19).or_insert(11);
    ///
    /// if let Entry::Occupied(o) = hashindex.entry_sync(19) {
    ///     assert_eq!(o.get(), &11);
    /// };
    /// ```
    #[inline]
    #[must_use]
    pub fn get(&self) -> &V {
        &self.locked_bucket.entry(&self.entry_ptr).1
    }

    /// Gets a mutable reference to the value in the entry.
    ///
    /// # Safety
    ///
    /// The caller has to make sure that there are no readers of the entry, e.g., a reader keeping
    /// a reference to the entry via [`HashIndex::iter`], [`HashIndex::peek`], or
    /// [`HashIndex::peek_with`], unless an instance of `V` can be safely read when there is a
    /// single writer, e.g., `V = [u8; 32]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    /// use scc::hash_index::Entry;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// hashindex.entry_sync(37).or_insert(11);
    ///
    /// if let Entry::Occupied(mut o) = hashindex.entry_sync(37) {
    ///     // Safety: `u32` can be safely read while being modified.
    ///     unsafe { *o.get_mut() += 18; }
    ///     assert_eq!(*o.get(), 29);
    /// }
    ///
    /// assert_eq!(hashindex.peek_with(&37, |_, v| *v), Some(29));
    /// ```
    #[inline]
    pub unsafe fn get_mut(&mut self) -> &mut V {
        &mut self.locked_bucket.entry_mut(&mut self.entry_ptr).1
    }

    /// Gets the next closest occupied entry after removing the entry.
    ///
    /// [`HashIndex::begin_async`] and this method together enable the [`OccupiedEntry`] to
    /// effectively act as a mutable iterator over entries. The method never acquires more than one
    /// lock even when it searches other buckets for the next closest occupied entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    /// use scc::hash_index::Entry;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// assert!(hashindex.insert_sync(1, 0).is_ok());
    /// assert!(hashindex.insert_sync(2, 0).is_ok());
    ///
    /// let second_entry_future = hashindex.begin_sync().unwrap().remove_and_async();
    /// ```
    #[inline]
    pub async fn remove_and_async(self) -> Option<OccupiedEntry<'h, K, V, H>> {
        let hashindex = self.hashindex;
        let mut entry_ptr = self.entry_ptr.clone();
        let guard = Guard::new();
        self.locked_bucket
            .writer
            .mark_removed(&mut entry_ptr, &guard);
        self.locked_bucket.set_has_garbage(&guard);
        if let Some(locked_bucket) = self
            .locked_bucket
            .next_async(hashindex, &mut entry_ptr)
            .await
        {
            return Some(OccupiedEntry {
                hashindex,
                locked_bucket,
                entry_ptr,
            });
        }
        None
    }

    /// Gets the next closest occupied entry after removing the entry.
    ///
    /// [`HashIndex::begin_sync`] and this method together enable the [`OccupiedEntry`] to
    /// effectively act as a mutable iterator over entries. The method never acquires more than one
    /// lock even when it searches other buckets for the next closest occupied entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    /// use scc::hash_index::Entry;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// assert!(hashindex.insert_sync(1, 0).is_ok());
    /// assert!(hashindex.insert_sync(2, 0).is_ok());
    ///
    /// let first_entry = hashindex.begin_sync().unwrap();
    /// let first_key = *first_entry.key();
    /// let second_entry = first_entry.remove_and_sync().unwrap();
    /// assert_eq!(hashindex.len(),  1);
    ///
    /// let second_key = *second_entry.key();
    ///
    /// assert!(second_entry.remove_and_sync().is_none());
    /// assert_eq!(first_key + second_key, 3);
    /// assert_eq!(hashindex.len(),  0);
    /// ```
    #[inline]
    #[must_use]
    pub fn remove_and_sync(self) -> Option<Self> {
        let hashindex = self.hashindex;
        let mut entry_ptr = self.entry_ptr.clone();
        let guard = Guard::new();
        self.locked_bucket
            .writer
            .mark_removed(&mut entry_ptr, &guard);
        self.locked_bucket.set_has_garbage(&guard);
        if let Some(locked_bucket) = self.locked_bucket.next_sync(hashindex, &mut entry_ptr) {
            return Some(OccupiedEntry {
                hashindex,
                locked_bucket,
                entry_ptr,
            });
        }
        None
    }

    /// Gets the next closest occupied entry.
    ///
    /// [`HashIndex::begin_async`] and this method together enable the [`OccupiedEntry`] to
    /// effectively act as a mutable iterator over entries. The method never acquires more than one
    /// lock even when it searches other buckets for the next closest occupied entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    /// use scc::hash_index::Entry;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// assert!(hashindex.insert_sync(1, 0).is_ok());
    /// assert!(hashindex.insert_sync(2, 0).is_ok());
    ///
    /// let second_entry_future = hashindex.begin_sync().unwrap().next_async();
    /// ```
    #[inline]
    pub async fn next_async(self) -> Option<OccupiedEntry<'h, K, V, H>> {
        let hashindex = self.hashindex;
        let mut entry_ptr = self.entry_ptr.clone();
        if let Some(locked_bucket) = self
            .locked_bucket
            .next_async(hashindex, &mut entry_ptr)
            .await
        {
            return Some(OccupiedEntry {
                hashindex,
                locked_bucket,
                entry_ptr,
            });
        }
        None
    }

    /// Gets the next closest occupied entry.
    ///
    /// [`HashIndex::begin_sync`] and this method together enable the [`OccupiedEntry`] to
    /// effectively act as a mutable iterator over entries. The method never acquires more than one
    /// lock even when it searches other buckets for the next closest occupied entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    /// use scc::hash_index::Entry;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// assert!(hashindex.insert_sync(1, 0).is_ok());
    /// assert!(hashindex.insert_sync(2, 0).is_ok());
    ///
    /// let first_entry = hashindex.begin_sync().unwrap();
    /// let first_key = *first_entry.key();
    /// let second_entry = first_entry.next_sync().unwrap();
    /// let second_key = *second_entry.key();
    ///
    /// assert!(second_entry.next_sync().is_none());
    /// assert_eq!(first_key + second_key, 3);
    /// ```
    #[inline]
    #[must_use]
    pub fn next_sync(self) -> Option<Self> {
        let hashindex = self.hashindex;
        let mut entry_ptr = self.entry_ptr.clone();
        if let Some(locked_bucket) = self.locked_bucket.next_sync(hashindex, &mut entry_ptr) {
            return Some(OccupiedEntry {
                hashindex,
                locked_bucket,
                entry_ptr,
            });
        }
        None
    }
}

impl<K, V, H> OccupiedEntry<'_, K, V, H>
where
    K: Clone + Eq + Hash,
    H: BuildHasher,
{
    /// Updates the entry by inserting a new entry and marking the existing entry removed.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    /// use scc::hash_index::Entry;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// hashindex.entry_sync(37).or_insert(11);
    ///
    /// if let Entry::Occupied(mut o) = hashindex.entry_sync(37) {
    ///     o.update(29);
    ///     assert_eq!(o.get(), &29);
    /// }
    ///
    /// assert_eq!(hashindex.peek_with(&37, |_, v| *v), Some(29));
    /// ```
    #[inline]
    pub fn update(&mut self, val: V) {
        let key = self.key().clone();
        let partial_hash = self.entry_ptr.partial_hash(&self.locked_bucket.writer);
        let entry_ptr = self
            .locked_bucket
            .insert(u64::from(partial_hash), (key, val));
        let guard = Guard::new();
        self.locked_bucket
            .writer
            .mark_removed(&mut self.entry_ptr, &guard);
        self.locked_bucket.set_has_garbage(&guard);
        self.entry_ptr = entry_ptr;
    }
}

impl<K, V, H> Debug for OccupiedEntry<'_, K, V, H>
where
    K: Debug + Eq + Hash,
    V: Debug,
    H: BuildHasher,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OccupiedEntry")
            .field("key", self.key())
            .field("value", self.get())
            .finish_non_exhaustive()
    }
}

impl<K, V, H> Deref for OccupiedEntry<'_, K, V, H>
where
    K: Debug + Eq + Hash,
    V: Debug,
    H: BuildHasher,
{
    type Target = V;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'h, K, V, H> VacantEntry<'h, K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    /// Gets a reference to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    /// assert_eq!(hashindex.entry_sync(11).key(), &11);
    /// ```
    #[inline]
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Takes ownership of the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    /// use scc::hash_index::Entry;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// if let Entry::Vacant(v) = hashindex.entry_sync(17) {
    ///     assert_eq!(v.into_key(), 17);
    /// };
    /// ```
    #[inline]
    pub fn into_key(self) -> K {
        self.key
    }

    /// Sets the value of the entry with its key, and returns an [`OccupiedEntry`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashIndex;
    /// use scc::hash_index::Entry;
    ///
    /// let hashindex: HashIndex<u64, u32> = HashIndex::default();
    ///
    /// if let Entry::Vacant(o) = hashindex.entry_sync(19) {
    ///     o.insert_entry(29);
    /// }
    ///
    /// assert_eq!(hashindex.peek_with(&19, |_, v| *v), Some(29));
    /// ```
    #[inline]
    pub fn insert_entry(self, val: V) -> OccupiedEntry<'h, K, V, H> {
        let entry_ptr = self.locked_bucket.insert(self.hash, (self.key, val));
        OccupiedEntry {
            hashindex: self.hashindex,
            locked_bucket: self.locked_bucket,
            entry_ptr,
        }
    }
}

impl<K, V, H> Debug for VacantEntry<'_, K, V, H>
where
    K: Debug + Eq + Hash,
    V: Debug,
    H: BuildHasher,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("VacantEntry").field(self.key()).finish()
    }
}

impl<K, V, H> Reserve<'_, K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    /// Returns the number of reserved slots.
    #[inline]
    #[must_use]
    pub fn additional_capacity(&self) -> usize {
        self.additional
    }
}

impl<K, V, H> AsRef<HashIndex<K, V, H>> for Reserve<'_, K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    #[inline]
    fn as_ref(&self) -> &HashIndex<K, V, H> {
        self.hashindex
    }
}

impl<K, V, H> Debug for Reserve<'_, K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Reserve").field(&self.additional).finish()
    }
}

impl<K, V, H> Deref for Reserve<'_, K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    type Target = HashIndex<K, V, H>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.hashindex
    }
}

impl<K, V, H> Drop for Reserve<'_, K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    #[inline]
    fn drop(&mut self) {
        let result = self
            .hashindex
            .minimum_capacity
            .fetch_sub(self.additional, Relaxed);
        debug_assert!(result >= self.additional);

        let guard = Guard::new();
        if let Some(current_array) = self.hashindex.bucket_array(&guard) {
            self.try_shrink(current_array, 0, &guard);
        }
    }
}

impl<K, V, H> Clone for Iter<'_, K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            hashindex: self.hashindex,
            bucket_array: self.bucket_array,
            index: self.index,
            bucket: self.bucket,
            entry_ptr: self.entry_ptr.clone(),
            guard: self.guard,
        }
    }
}

impl<K, V, H> Debug for Iter<'_, K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Iter")
            .field("current_bucket_index", &self.index)
            .field("current_index_in_bucket", &self.entry_ptr.index())
            .finish()
    }
}

impl<'h, K, V, H> Iterator for Iter<'h, K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    type Item = (&'h K, &'h V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut array = if let Some(&array) = self.bucket_array.as_ref() {
            array
        } else {
            // Start scanning.
            let current_array = self.hashindex.bucket_array(self.guard)?;
            let array = if let Some(old_array) = current_array.linked_array(self.guard) {
                old_array
            } else {
                current_array
            };
            self.bucket_array.replace(array);
            self.bucket.replace(array.bucket(0));
            array
        };

        // Move to the next entry.
        loop {
            if let Some(bucket) = self.bucket.take() {
                // Move to the next entry in the bucket.
                if self.entry_ptr.find_next(bucket) {
                    let (k, v) = self.entry_ptr.get(array.data_block(self.index));
                    self.bucket.replace(bucket);
                    return Some((k, v));
                }
            }
            self.entry_ptr = EntryPtr::null();

            if self.index + 1 == array.len() {
                // Move to a newer bucket array.
                self.index = 0;
                let current_array = self.hashindex.bucket_array(self.guard)?;
                if self
                    .bucket_array
                    .as_ref()
                    .is_some_and(|&a| ptr::eq(a, current_array))
                {
                    // Finished scanning.
                    break;
                }

                array = if let Some(old_array) = current_array.linked_array(self.guard) {
                    if self
                        .bucket_array
                        .as_ref()
                        .is_some_and(|&a| ptr::eq(a, old_array))
                    {
                        // Start scanning the current array.
                        array = current_array;
                        self.bucket_array.replace(current_array);
                        self.bucket.replace(current_array.bucket(0));
                        continue;
                    }
                    old_array
                } else {
                    current_array
                };

                self.bucket_array.replace(array);
                self.bucket.replace(array.bucket(0));
            } else {
                self.index += 1;
                self.bucket.replace(array.bucket(self.index));
            }
        }
        None
    }
}

impl<K, V, H> FusedIterator for Iter<'_, K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
}

impl<K, V, H> UnwindSafe for Iter<'_, K, V, H>
where
    K: Eq + Hash + UnwindSafe,
    V: UnwindSafe,
    H: BuildHasher + UnwindSafe,
{
}
