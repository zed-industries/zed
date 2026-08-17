//! [`HashMap`] is an asynchronous/concurrent hash map.

use std::collections::hash_map::RandomState;
use std::fmt::{self, Debug};
use std::hash::{BuildHasher, Hash};
use std::mem::replace;
use std::ops::{Deref, DerefMut, RangeInclusive};
#[cfg(not(feature = "loom"))]
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;

#[cfg(feature = "loom")]
use loom::sync::atomic::AtomicUsize;
use sdd::{AtomicShared, Guard, Shared, Tag};

use super::Equivalent;
use super::hash_table::bucket::{EntryPtr, MAP};
use super::hash_table::bucket_array::BucketArray;
use super::hash_table::{HashTable, LockedBucket};

/// Scalable asynchronous/concurrent hash map.
///
/// [`HashMap`] is an asynchronous/concurrent hash map data structure optimized for highly
/// concurrent workloads. [`HashMap`] has a dynamically sized array of buckets where a bucket is a
/// fixed-size hash table with linear probing that can be expanded by allocating a linked list of
/// smaller buckets when it is full.
///
/// ## The key features of [`HashMap`]
///
/// * Non-sharded: data is stored in a single array of entry buckets.
/// * Non-blocking resizing: resizing does not block other threads or tasks.
/// * Automatic resizing: grows or shrinks as needed.
/// * Incremental resizing: entries in the old bucket array are incrementally relocated.
/// * No busy waiting: no spin locks or hot loops to wait for desired resources.
/// * Linearizability: [`HashMap`] manipulation methods are linearizable.
///
/// ## The key statistics for [`HashMap`]
///
/// * The expected size of metadata for a single entry: 2 bytes.
/// * The expected number of atomic write operations required for an operation on a single key: 2.
/// * The expected number of atomic variables accessed during a single key operation: 2.
/// * The number of entries managed by a single bucket without a linked list: 32.
/// * The expected maximum linked list length when a resize is triggered: log(capacity) / 8.
///
/// ## Locking behavior
///
/// ### Bucket access
///
/// Bucket arrays are protected by [`sdd`], thus enabling lock-free access to them.
///
/// ### Entry access
///
/// Each read/write access to an entry is serialized by the read-write lock in the bucket containing
/// the entry. There are no container-level locks, therefore, the larger the [`HashMap`] gets, the
/// lower the chance that the bucket-level lock will be contended.
///
/// ### Resize
///
/// Resizing of the [`HashMap`] is non-blocking and lock-free; resizing does not block any other
/// read/write access to the [`HashMap`] or resizing attempts. Resizing is analogous to pushing a
/// new bucket array into a lock-free stack. Each entry in the old bucket array will be
/// incrementally relocated to the new bucket array upon future access to the [`HashMap`], and the old
/// bucket array is dropped when it becomes empty and unreachable.
///
/// ### Synchronous methods in an asynchronous code block
///
/// It is generally not recommended to use blocking methods, such as [`HashMap::insert_sync`], in an
/// asynchronous code block or [`poll`](std::future::Future::poll), since it may lead to deadlocks
/// or performance degradation.
///
/// ## Unwind safety
///
/// [`HashMap`] is impervious to out-of-memory errors and panics in user-specified code under one
/// condition: `H::Hasher::hash`, `K::drop` and `V::drop` must not panic.
pub struct HashMap<K, V, H = RandomState>
where
    H: BuildHasher,
{
    bucket_array: AtomicShared<BucketArray<K, V, (), MAP>>,
    minimum_capacity: AtomicUsize,
    build_hasher: H,
}

/// [`Entry`] represents a single entry in a [`HashMap`].
pub enum Entry<'h, K, V, H = RandomState>
where
    H: BuildHasher,
{
    /// An occupied entry.
    Occupied(OccupiedEntry<'h, K, V, H>),
    /// A vacant entry.
    Vacant(VacantEntry<'h, K, V, H>),
}

/// [`OccupiedEntry`] is a view into an occupied entry in a [`HashMap`].
pub struct OccupiedEntry<'h, K, V, H = RandomState>
where
    H: BuildHasher,
{
    hashmap: &'h HashMap<K, V, H>,
    locked_bucket: LockedBucket<K, V, (), MAP>,
    entry_ptr: EntryPtr<K, V, MAP>,
}

/// [`VacantEntry`] is a view into a vacant entry in a [`HashMap`].
pub struct VacantEntry<'h, K, V, H = RandomState>
where
    H: BuildHasher,
{
    hashmap: &'h HashMap<K, V, H>,
    key: K,
    hash: u64,
    locked_bucket: LockedBucket<K, V, (), MAP>,
}

/// [`ConsumableEntry`] is a view into an occupied entry in a [`HashMap`] when iterating over
/// entries in it.
pub struct ConsumableEntry<'b, K, V> {
    /// Holds an exclusive lock on the entry bucket.
    locked_bucket: &'b mut LockedBucket<K, V, (), MAP>,
    /// Pointer to the entry.
    entry_ptr: &'b mut EntryPtr<K, V, MAP>,
    /// Probes removal.
    remove_probe: &'b mut bool,
}

/// [`ReplaceResult`] is the result type of the [`HashMap::replace_async`] and
/// [`HashMap::replace_sync`] methods.
pub enum ReplaceResult<'h, K, V, H = RandomState>
where
    H: BuildHasher,
{
    /// The key was replaced.
    Replaced(OccupiedEntry<'h, K, V, H>, K),
    /// The key did not exist in the [`HashMap`].
    ///
    /// An [`OccupiedEntry`] can be created from the [`VacantEntry`].
    NotReplaced(VacantEntry<'h, K, V, H>),
}

/// [`Reserve`] keeps the capacity of the associated [`HashMap`] higher than a certain level.
///
/// The [`HashMap`] does not shrink the capacity below the reserved capacity.
pub struct Reserve<'h, K, V, H = RandomState>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    hashmap: &'h HashMap<K, V, H>,
    additional: usize,
}

impl<K, V, H> HashMap<K, V, H>
where
    H: BuildHasher,
{
    /// Creates an empty [`HashMap`] with the given [`BuildHasher`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    /// use std::collections::hash_map::RandomState;
    ///
    /// let hashmap: HashMap<u64, u32, RandomState> = HashMap::with_hasher(RandomState::new());
    /// ```
    #[cfg(not(feature = "loom"))]
    #[inline]
    pub const fn with_hasher(build_hasher: H) -> Self {
        Self {
            bucket_array: AtomicShared::null(),
            minimum_capacity: AtomicUsize::new(0),
            build_hasher,
        }
    }

    /// Creates an empty [`HashMap`] with the given [`BuildHasher`].
    #[cfg(feature = "loom")]
    #[inline]
    pub fn with_hasher(build_hasher: H) -> Self {
        Self {
            bucket_array: AtomicShared::null(),
            minimum_capacity: AtomicUsize::new(0),
            build_hasher,
        }
    }

    /// Creates an empty [`HashMap`] with the specified capacity and [`BuildHasher`].
    ///
    /// The actual capacity is equal to or greater than `capacity` unless it is greater than
    /// `1 << (usize::BITS - 1)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    /// use std::collections::hash_map::RandomState;
    ///
    /// let hashmap: HashMap<u64, u32, RandomState> =
    ///     HashMap::with_capacity_and_hasher(1000, RandomState::new());
    ///
    /// let result = hashmap.capacity();
    /// assert_eq!(result, 1024);
    /// ```
    #[inline]
    pub fn with_capacity_and_hasher(capacity: usize, build_hasher: H) -> Self {
        let (array, minimum_capacity) = if capacity == 0 {
            (AtomicShared::null(), AtomicUsize::new(0))
        } else {
            let array = unsafe {
                Shared::new_with_unchecked(|| {
                    BucketArray::<K, V, (), MAP>::new(capacity, AtomicShared::null())
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
            minimum_capacity,
            build_hasher,
        }
    }
}

impl<K, V, H> HashMap<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    /// Temporarily increases the minimum capacity of the [`HashMap`] to prevent shrinking.
    ///
    /// A [`Reserve`] is returned if the [`HashMap`] can increase the minimum capacity. The
    /// increased capacity is not exclusively owned by the returned [`Reserve`], allowing others to
    /// benefit from it. The memory for the additional space may not be immediately allocated if
    /// the [`HashMap`] is empty or currently being resized; however, once the memory is eventually reserved,
    /// the capacity will not shrink below the additional capacity until the returned
    /// [`Reserve`] is dropped.
    ///
    /// # Errors
    ///
    /// Returns `None` if a too large number is given.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<usize, usize> = HashMap::with_capacity(1000);
    /// assert_eq!(hashmap.capacity(), 1024);
    ///
    /// let reserved = hashmap.reserve(10000);
    /// assert!(reserved.is_some());
    /// assert_eq!(hashmap.capacity(), 16384);
    ///
    /// assert!(hashmap.reserve(usize::MAX).is_none());
    /// assert_eq!(hashmap.capacity(), 16384);
    ///
    /// for i in 0..16 {
    ///     assert!(hashmap.insert_sync(i, i).is_ok());
    /// }
    /// drop(reserved);
    ///
    /// assert_eq!(hashmap.capacity(), 1024);
    /// ```
    #[inline]
    pub fn reserve(&self, additional_capacity: usize) -> Option<Reserve<'_, K, V, H>> {
        let additional = self.reserve_capacity(additional_capacity);
        if additional == 0 {
            None
        } else {
            Some(Reserve {
                hashmap: self,
                additional,
            })
        }
    }

    /// Gets the entry associated with the given key in the map for in-place manipulation.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<char, u32> = HashMap::default();
    ///
    /// let future_entry = hashmap.entry_async('b');
    /// ```
    #[inline]
    pub async fn entry_async(&self, key: K) -> Entry<'_, K, V, H> {
        let hash = self.hash(&key);
        let locked_bucket = self.writer_async(hash).await;
        let entry_ptr = locked_bucket.search(&key, hash);
        if entry_ptr.is_valid() {
            Entry::Occupied(OccupiedEntry {
                hashmap: self,
                locked_bucket,
                entry_ptr,
            })
        } else {
            let vacant_entry = VacantEntry {
                hashmap: self,
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
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<char, u32> = HashMap::default();
    ///
    /// for ch in "a short treatise on fungi".chars() {
    ///     hashmap.entry_sync(ch).and_modify(|counter| *counter += 1).or_insert(1);
    /// }
    ///
    /// assert_eq!(hashmap.read_sync(&'s', |_, v| *v), Some(2));
    /// assert_eq!(hashmap.read_sync(&'t', |_, v| *v), Some(3));
    /// assert!(hashmap.read_sync(&'y', |_, v| *v).is_none());
    /// ```
    #[inline]
    pub fn entry_sync(&self, key: K) -> Entry<'_, K, V, H> {
        let hash = self.hash(&key);
        let locked_bucket = self.writer_sync(hash);
        let entry_ptr = locked_bucket.search(&key, hash);
        if entry_ptr.is_valid() {
            Entry::Occupied(OccupiedEntry {
                hashmap: self,
                locked_bucket,
                entry_ptr,
            })
        } else {
            let vacant_entry = VacantEntry {
                hashmap: self,
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
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<usize, usize> = HashMap::default();
    ///
    /// assert!(hashmap.insert_sync(0, 1).is_ok());
    /// assert!(hashmap.try_entry(0).is_some());
    /// ```
    #[inline]
    pub fn try_entry(&self, key: K) -> Option<Entry<'_, K, V, H>> {
        let hash = self.hash(&key);
        let guard = Guard::new();
        let locked_bucket = self.try_reserve_bucket(hash, &guard)?;
        let entry_ptr = locked_bucket.search(&key, hash);
        if entry_ptr.is_valid() {
            Some(Entry::Occupied(OccupiedEntry {
                hashmap: self,
                locked_bucket,
                entry_ptr,
            }))
        } else {
            Some(Entry::Vacant(VacantEntry {
                hashmap: self,
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
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<char, u32> = HashMap::default();
    ///
    /// let future_entry = hashmap.begin_async();
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
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// assert!(hashmap.insert_sync(1, 0).is_ok());
    ///
    /// let mut first_entry = hashmap.begin_sync().unwrap();
    /// *first_entry.get_mut() = 2;
    ///
    /// assert!(first_entry.next_sync().is_none());
    /// assert_eq!(hashmap.read_sync(&1, |_, v| *v), Some(2));
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
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// let future_entry = hashmap.any_async(|k, _| *k == 2);
    /// ```
    #[inline]
    pub async fn any_async<P: FnMut(&K, &V) -> bool>(
        &self,
        mut pred: P,
    ) -> Option<OccupiedEntry<'_, K, V, H>> {
        let mut entry = None;
        self.for_each_writer_async(0, 0, |locked_bucket, _| {
            let mut entry_ptr = EntryPtr::null();
            while entry_ptr.find_next(&locked_bucket.writer) {
                let (k, v) = locked_bucket.entry(&entry_ptr);
                if pred(k, v) {
                    entry = Some(OccupiedEntry {
                        hashmap: self,
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
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// assert!(hashmap.insert_sync(1, 0).is_ok());
    /// assert!(hashmap.insert_sync(2, 3).is_ok());
    ///
    /// let mut entry = hashmap.any_sync(|k, _| *k == 2).unwrap();
    /// assert_eq!(*entry.get(), 3);
    /// ```
    #[inline]
    pub fn any_sync<P: FnMut(&K, &V) -> bool>(
        &self,
        mut pred: P,
    ) -> Option<OccupiedEntry<'_, K, V, H>> {
        let mut entry = None;
        let guard = Guard::new();
        self.for_each_writer_sync(0, 0, &guard, |locked_bucket, _| {
            let mut entry_ptr = EntryPtr::null();
            while entry_ptr.find_next(&locked_bucket.writer) {
                let (k, v) = locked_bucket.entry(&entry_ptr);
                if pred(k, v) {
                    entry = Some(OccupiedEntry {
                        hashmap: self,
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

    /// Inserts a key-value pair into the [`HashMap`].
    ///
    /// # Note
    ///
    /// If the key exists, the value is *not* updated. [`upsert_async`](Self::upsert_async)
    /// provides a way to update the value if the key exists.
    ///
    /// # Errors
    ///
    /// Returns an error containing the supplied key-value pair if the key exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    /// let future_insert = hashmap.insert_async(11, 17);
    /// ```
    #[inline]
    pub async fn insert_async(&self, key: K, val: V) -> Result<(), (K, V)> {
        let hash = self.hash(&key);
        let locked_bucket = self.writer_async(hash).await;
        if locked_bucket.search(&key, hash).is_valid() {
            Err((key, val))
        } else {
            locked_bucket.insert(hash, (key, val));
            Ok(())
        }
    }

    /// Inserts a key-value pair into the [`HashMap`].
    ///
    /// # Note
    ///
    /// If the key exists, the value is *not* updated. [`upsert_sync`](Self::upsert_sync)
    /// provides a way to update the value if the key exists.
    ///
    /// # Errors
    ///
    /// Returns an error containing the supplied key-value pair if the key exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// assert!(hashmap.insert_sync(1, 0).is_ok());
    /// assert_eq!(hashmap.insert_sync(1, 1).unwrap_err(), (1, 1));
    /// ```
    #[inline]
    pub fn insert_sync(&self, key: K, val: V) -> Result<(), (K, V)> {
        let hash = self.hash(&key);
        let locked_bucket = self.writer_sync(hash);
        if locked_bucket.search(&key, hash).is_valid() {
            Err((key, val))
        } else {
            locked_bucket.insert(hash, (key, val));
            Ok(())
        }
    }

    /// Upserts a key-value pair into the [`HashMap`].
    ///
    /// Returns the old value if the [`HashMap`] has this key present, or returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    /// let future_upsert = hashmap.upsert_async(11, 17);
    /// ```
    #[inline]
    pub async fn upsert_async(&self, key: K, val: V) -> Option<V> {
        match self.entry_async(key).await {
            Entry::Occupied(mut o) => Some(replace(o.get_mut(), val)),
            Entry::Vacant(v) => {
                v.insert_entry(val);
                None
            }
        }
    }

    /// Upserts a key-value pair into the [`HashMap`].
    ///
    /// Returns the old value if the [`HashMap`] has this key present, or returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// assert!(hashmap.upsert_sync(1, 0).is_none());
    /// assert_eq!(hashmap.upsert_sync(1, 1).unwrap(), 0);
    /// assert_eq!(hashmap.read_sync(&1, |_, v| *v).unwrap(), 1);
    /// ```
    #[inline]
    pub fn upsert_sync(&self, key: K, val: V) -> Option<V> {
        match self.entry_sync(key) {
            Entry::Occupied(mut o) => Some(replace(o.get_mut(), val)),
            Entry::Vacant(v) => {
                v.insert_entry(val);
                None
            }
        }
    }

    /// Updates an existing key-value pair in-place.
    ///
    /// Returns `None` if the key does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// assert!(hashmap.insert_sync(1, 0).is_ok());
    /// let future_update = hashmap.update_async(&1, |_, v| { *v = 2; *v });
    /// ```
    #[inline]
    pub async fn update_async<Q, U, R>(&self, key: &Q, updater: U) -> Option<R>
    where
        Q: Equivalent<K> + Hash + ?Sized,
        U: FnOnce(&K, &mut V) -> R,
    {
        let hash = self.hash(key);
        let mut locked_bucket = self.optional_writer_async(hash).await?;
        let mut entry_ptr = locked_bucket.search(key, hash);
        if entry_ptr.is_valid() {
            let (k, v) = locked_bucket.entry_mut(&mut entry_ptr);
            Some(updater(k, v))
        } else {
            None
        }
    }

    /// Updates an existing key-value pair in-place.
    ///
    /// Returns `None` if the key does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// assert!(hashmap.update_sync(&1, |_, _| true).is_none());
    /// assert!(hashmap.insert_sync(1, 0).is_ok());
    /// assert_eq!(hashmap.update_sync(&1, |_, v| { *v = 2; *v }).unwrap(), 2);
    /// assert_eq!(hashmap.read_sync(&1, |_, v| *v).unwrap(), 2);
    /// ```
    #[inline]
    pub fn update_sync<Q, U, R>(&self, key: &Q, updater: U) -> Option<R>
    where
        Q: Equivalent<K> + Hash + ?Sized,
        U: FnOnce(&K, &mut V) -> R,
    {
        let hash = self.hash(key);
        let mut locked_bucket = self.optional_writer_sync(hash)?;
        let mut entry_ptr = locked_bucket.search(key, hash);
        if entry_ptr.is_valid() {
            let (k, v) = locked_bucket.entry_mut(&mut entry_ptr);
            Some(updater(k, v))
        } else {
            None
        }
    }

    /// Adds a key to the [`HashMap`], replacing the existing key, if any, that is equal to the
    /// given one.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::cmp::{Eq, PartialEq};
    /// use std::hash::{Hash, Hasher};
    ///
    /// use scc::HashMap;
    /// use scc::hash_map::ReplaceResult;
    ///
    /// #[derive(Debug)]
    /// struct MaybeEqual(u64, u64);
    ///
    /// impl Eq for MaybeEqual {}
    ///
    /// impl Hash for MaybeEqual {
    ///     fn hash<H: Hasher>(&self, state: &mut H) {
    ///         // Do not read `self.1`.
    ///         self.0.hash(state);
    ///     }
    /// }
    ///
    /// impl PartialEq for MaybeEqual {
    ///     fn eq(&self, other: &Self) -> bool {
    ///         // Do not compare `self.1`.
    ///         self.0 == other.0
    ///     }
    /// }
    ///
    /// let hashmap: HashMap<MaybeEqual, usize> = HashMap::default();
    ///
    /// async {
    ///     let ReplaceResult::NotReplaced(v) = hashmap.replace_async(MaybeEqual(11, 7)).await else {
    ///         unreachable!();
    ///     };
    ///     drop(v.insert_entry(17));
    ///     let ReplaceResult::Replaced(_, k) = hashmap.replace_async(MaybeEqual(11, 11)).await else {
    ///         unreachable!();
    ///     };
    ///     assert_eq!(k.1, 7);
    /// };
    /// ```
    #[inline]
    pub async fn replace_async(&self, key: K) -> ReplaceResult<'_, K, V, H> {
        let hash = self.hash(&key);
        let locked_bucket = self.writer_async(hash).await;
        let mut entry_ptr = locked_bucket.search(&key, hash);
        if entry_ptr.is_valid() {
            let prev_key = replace(
                &mut entry_ptr
                    .get_mut(locked_bucket.data_block, &locked_bucket.writer)
                    .0,
                key,
            );
            ReplaceResult::Replaced(
                OccupiedEntry {
                    hashmap: self,
                    locked_bucket,
                    entry_ptr,
                },
                prev_key,
            )
        } else {
            ReplaceResult::NotReplaced(VacantEntry {
                hashmap: self,
                key,
                hash,
                locked_bucket,
            })
        }
    }

    /// Adds a key to the [`HashMap`], replacing the existing key, if any, that is equal to the
    /// given one.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::cmp::{Eq, PartialEq};
    /// use std::hash::{Hash, Hasher};
    ///
    /// use scc::HashMap;
    /// use scc::hash_map::ReplaceResult;
    ///
    /// #[derive(Debug)]
    /// struct MaybeEqual(u64, u64);
    ///
    /// impl Eq for MaybeEqual {}
    ///
    /// impl Hash for MaybeEqual {
    ///     fn hash<H: Hasher>(&self, state: &mut H) {
    ///         // Do not read `self.1`.
    ///         self.0.hash(state);
    ///     }
    /// }
    ///
    /// impl PartialEq for MaybeEqual {
    ///     fn eq(&self, other: &Self) -> bool {
    ///         // Do not compare `self.1`.
    ///         self.0 == other.0
    ///     }
    /// }
    ///
    /// let hashmap: HashMap<MaybeEqual, usize> = HashMap::default();
    ///
    /// let ReplaceResult::NotReplaced(v) = hashmap.replace_sync(MaybeEqual(11, 7)) else {
    ///     unreachable!();
    /// };
    /// drop(v.insert_entry(17));
    /// let ReplaceResult::Replaced(_, k) = hashmap.replace_sync(MaybeEqual(11, 11)) else {
    ///     unreachable!();
    /// };
    /// assert_eq!(k.1, 7);
    /// ```
    #[inline]
    pub fn replace_sync(&self, key: K) -> ReplaceResult<'_, K, V, H> {
        let hash = self.hash(&key);
        let locked_bucket = self.writer_sync(hash);
        let mut entry_ptr = locked_bucket.search(&key, hash);
        if entry_ptr.is_valid() {
            let prev_key = replace(
                &mut entry_ptr
                    .get_mut(locked_bucket.data_block, &locked_bucket.writer)
                    .0,
                key,
            );
            ReplaceResult::Replaced(
                OccupiedEntry {
                    hashmap: self,
                    locked_bucket,
                    entry_ptr,
                },
                prev_key,
            )
        } else {
            ReplaceResult::NotReplaced(VacantEntry {
                hashmap: self,
                key,
                hash,
                locked_bucket,
            })
        }
    }

    /// Removes a key-value pair if the key exists.
    ///
    /// Returns `None` if the key does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    /// let future_insert = hashmap.insert_async(11, 17);
    /// let future_remove = hashmap.remove_async(&11);
    /// ```
    #[inline]
    pub async fn remove_async<Q>(&self, key: &Q) -> Option<(K, V)>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.remove_if_async(key, |_| true).await
    }

    /// Removes a key-value pair if the key exists.
    ///
    /// Returns `None` if the key does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// assert!(hashmap.remove_sync(&1).is_none());
    /// assert!(hashmap.insert_sync(1, 0).is_ok());
    /// assert_eq!(hashmap.remove_sync(&1).unwrap(), (1, 0));
    /// ```
    #[inline]
    pub fn remove_sync<Q>(&self, key: &Q) -> Option<(K, V)>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.remove_if_sync(key, |_| true)
    }

    /// Removes a key-value pair if the key exists and the given condition is met.
    ///
    /// Returns `None` if the key does not exist or the condition was not met.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    /// let future_insert = hashmap.insert_async(11, 17);
    /// let future_remove = hashmap.remove_if_async(&11, |_| true);
    /// ```
    #[inline]
    pub async fn remove_if_async<Q, F: FnOnce(&mut V) -> bool>(
        &self,
        key: &Q,
        condition: F,
    ) -> Option<(K, V)>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        let hash = self.hash(key);
        let mut locked_bucket = self.optional_writer_async(hash).await?;
        let mut entry_ptr = locked_bucket.search(key, hash);
        if entry_ptr.is_valid() && condition(&mut locked_bucket.entry_mut(&mut entry_ptr).1) {
            Some(locked_bucket.remove(self, &mut entry_ptr))
        } else {
            None
        }
    }

    /// Removes a key-value pair if the key exists and the given condition is met.
    ///
    /// Returns `None` if the key does not exist or the condition was not met.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// assert!(hashmap.insert_sync(1, 0).is_ok());
    /// assert!(hashmap.remove_if_sync(&1, |v| { *v += 1; false }).is_none());
    /// assert_eq!(hashmap.remove_if_sync(&1, |v| *v == 1).unwrap(), (1, 1));
    /// ```
    #[inline]
    pub fn remove_if_sync<Q, F: FnOnce(&mut V) -> bool>(
        &self,
        key: &Q,
        condition: F,
    ) -> Option<(K, V)>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        let hash = self.hash(key);
        let mut locked_bucket = self.optional_writer_sync(hash)?;
        let mut entry_ptr = locked_bucket.search(key, hash);
        if entry_ptr.is_valid() && condition(&mut locked_bucket.entry_mut(&mut entry_ptr).1) {
            Some(locked_bucket.remove(self, &mut entry_ptr))
        } else {
            None
        }
    }

    /// Gets an [`OccupiedEntry`] corresponding to the key for in-place modification.
    ///
    /// [`OccupiedEntry`] exclusively owns the entry, preventing others from gaining access to it:
    /// use [`read_async`](Self::read_async) if read-only access is sufficient.
    ///
    /// Returns `None` if the key does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    /// let future_insert = hashmap.insert_async(11, 17);
    /// let future_get = hashmap.get_async(&11);
    /// ```
    #[inline]
    pub async fn get_async<Q>(&self, key: &Q) -> Option<OccupiedEntry<'_, K, V, H>>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        let hash = self.hash(key);
        let locked_bucket = self.optional_writer_async(hash).await?;
        let entry_ptr = locked_bucket.search(key, hash);
        if entry_ptr.is_valid() {
            return Some(OccupiedEntry {
                hashmap: self,
                locked_bucket,
                entry_ptr,
            });
        }
        None
    }

    /// Gets an [`OccupiedEntry`] corresponding to the key for in-place modification.
    ///
    /// [`OccupiedEntry`] exclusively owns the entry, preventing others from gaining access to it:
    /// use [`read_sync`](Self::read_sync) if read-only access is sufficient.
    ///
    /// Returns `None` if the key does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// assert!(hashmap.get_sync(&1).is_none());
    /// assert!(hashmap.insert_sync(1, 10).is_ok());
    /// assert_eq!(*hashmap.get_sync(&1).unwrap().get(), 10);
    ///
    /// *hashmap.get_sync(&1).unwrap() = 11;
    /// assert_eq!(*hashmap.get_sync(&1).unwrap(), 11);
    /// ```
    #[inline]
    pub fn get_sync<Q>(&self, key: &Q) -> Option<OccupiedEntry<'_, K, V, H>>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        let hash = self.hash(key);
        let locked_bucket = self.optional_writer_sync(hash)?;
        let entry_ptr = locked_bucket.search(key, hash);
        if entry_ptr.is_valid() {
            return Some(OccupiedEntry {
                hashmap: self,
                locked_bucket,
                entry_ptr,
            });
        }
        None
    }

    /// Reads a key-value pair.
    ///
    /// Returns `None` if the key does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    /// let future_insert = hashmap.insert_async(11, 17);
    /// let future_read = hashmap.read_async(&11, |_, v| *v);
    /// ```
    #[inline]
    pub async fn read_async<Q, R, F: FnOnce(&K, &V) -> R>(&self, key: &Q, reader: F) -> Option<R>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.reader_async(key, reader).await
    }

    /// Reads a key-value pair.
    ///
    /// Returns `None` if the key does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// assert!(hashmap.read_sync(&1, |_, v| *v).is_none());
    /// assert!(hashmap.insert_sync(1, 10).is_ok());
    /// assert_eq!(hashmap.read_sync(&1, |_, v| *v).unwrap(), 10);
    /// ```
    #[inline]
    pub fn read_sync<Q, R, F: FnOnce(&K, &V) -> R>(&self, key: &Q, reader: F) -> Option<R>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.reader_sync(key, reader)
    }

    /// Returns `true` if the [`HashMap`] contains a value for the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// let future_contains = hashmap.contains_async(&1);
    /// ```
    #[inline]
    pub async fn contains_async<Q>(&self, key: &Q) -> bool
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.reader_async(key, |_, _| ()).await.is_some()
    }

    /// Returns `true` if the [`HashMap`] contains a value for the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// assert!(!hashmap.contains_sync(&1));
    /// assert!(hashmap.insert_sync(1, 0).is_ok());
    /// assert!(hashmap.contains_sync(&1));
    /// ```
    #[inline]
    pub fn contains_sync<Q>(&self, key: &Q) -> bool
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.read_sync(key, |_, _| ()).is_some()
    }

    /// Iterates over entries asynchronously for reading.
    ///
    /// Stops iterating when the closure returns `false`, and this method also returns `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u64> = HashMap::default();
    ///
    /// assert!(hashmap.insert_sync(1, 0).is_ok());
    ///
    /// async {
    ///     let result = hashmap.iter_async(|k, v| {
    ///         false
    ///     }).await;
    ///     assert!(!result);
    /// };
    /// ```
    #[inline]
    pub async fn iter_async<F: FnMut(&K, &V) -> bool>(&self, mut f: F) -> bool {
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
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u64> = HashMap::default();
    ///
    /// assert!(hashmap.insert_sync(1, 0).is_ok());
    /// assert!(hashmap.insert_sync(2, 1).is_ok());
    ///
    /// let mut acc = 0_u64;
    /// let result = hashmap.iter_sync(|k, v| {
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

    /// Iterates over entries asynchronously for modification.
    ///
    /// Stops iterating when the closure returns `false`, and this method also returns `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// assert!(hashmap.insert_sync(1, 0).is_ok());
    /// assert!(hashmap.insert_sync(2, 1).is_ok());
    ///
    /// async {
    ///     let result = hashmap.iter_mut_async(|entry| {
    ///         if entry.0 == 1 {
    ///             entry.consume();
    ///             return false;
    ///         }
    ///         true
    ///     }).await;
    ///
    ///     assert!(!result);
    ///     assert_eq!(hashmap.len(), 1);
    /// };
    /// ```
    #[inline]
    pub async fn iter_mut_async<F: FnMut(ConsumableEntry<'_, K, V>) -> bool>(
        &self,
        mut f: F,
    ) -> bool {
        let mut result = true;
        self.for_each_writer_async(0, 0, |mut locked_bucket, removed| {
            let mut entry_ptr = EntryPtr::null();
            while entry_ptr.find_next(&locked_bucket.writer) {
                let consumable_entry = ConsumableEntry {
                    locked_bucket: &mut locked_bucket,
                    entry_ptr: &mut entry_ptr,
                    remove_probe: removed,
                };
                if !f(consumable_entry) {
                    result = false;
                    return false;
                }
            }
            true
        })
        .await;
        result
    }

    /// Iterates over entries synchronously for modification.
    ///
    /// Stops iterating when the closure returns `false`, and this method also returns `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// assert!(hashmap.insert_sync(1, 0).is_ok());
    /// assert!(hashmap.insert_sync(2, 1).is_ok());
    /// assert!(hashmap.insert_sync(3, 2).is_ok());
    ///
    /// let result = hashmap.iter_mut_sync(|entry| {
    ///     if entry.0 == 1 {
    ///         entry.consume();
    ///         return false;
    ///     }
    ///     true
    /// });
    ///
    /// assert!(!result);
    /// assert!(!hashmap.contains_sync(&1));
    /// assert_eq!(hashmap.len(), 2);
    /// ```
    #[inline]
    pub fn iter_mut_sync<F: FnMut(ConsumableEntry<'_, K, V>) -> bool>(&self, mut f: F) -> bool {
        let mut result = true;
        let guard = Guard::new();
        self.for_each_writer_sync(0, 0, &guard, |mut locked_bucket, removed| {
            let mut entry_ptr = EntryPtr::null();
            while entry_ptr.find_next(&locked_bucket.writer) {
                let consumable_entry = ConsumableEntry {
                    locked_bucket: &mut locked_bucket,
                    entry_ptr: &mut entry_ptr,
                    remove_probe: removed,
                };
                if !f(consumable_entry) {
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
    /// This method allows the predicate closure to modify the value field.
    ///
    /// Entries that have existed since the invocation of the method are guaranteed to be visited
    /// if they are not removed, however the same entry can be visited more than once if the
    /// [`HashMap`] gets resized by another thread.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// let future_insert = hashmap.insert_async(1, 0);
    /// let future_retain = hashmap.retain_async(|k, v| *k == 1);
    /// ```
    #[inline]
    pub async fn retain_async<F: FnMut(&K, &mut V) -> bool>(&self, mut pred: F) {
        self.iter_mut_async(|mut e| {
            let (k, v) = &mut *e;
            if !pred(k, v) {
                drop(e.consume());
            }
            true
        })
        .await;
    }

    /// Retains the entries specified by the predicate.
    ///
    /// This method allows the predicate closure to modify the value field.
    ///
    /// Entries that have existed since the invocation of the method are guaranteed to be visited
    /// if they are not removed, however the same entry can be visited more than once if the
    /// [`HashMap`] gets resized by another thread.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// assert!(hashmap.insert_sync(1, 0).is_ok());
    /// assert!(hashmap.insert_sync(2, 1).is_ok());
    /// assert!(hashmap.insert_sync(3, 2).is_ok());
    ///
    /// hashmap.retain_sync(|k, v| *k == 1 && *v == 0);
    ///
    /// assert!(hashmap.contains_sync(&1));
    /// assert!(!hashmap.contains_sync(&2));
    /// assert!(!hashmap.contains_sync(&3));
    /// ```
    #[inline]
    pub fn retain_sync<F: FnMut(&K, &mut V) -> bool>(&self, mut pred: F) {
        self.iter_mut_sync(|mut e| {
            let (k, v) = &mut *e;
            if !pred(k, v) {
                drop(e.consume());
            }
            true
        });
    }

    /// Clears the [`HashMap`] by removing all key-value pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// let future_insert = hashmap.insert_async(1, 0);
    /// let future_clear = hashmap.clear_async();
    /// ```
    #[inline]
    pub async fn clear_async(&self) {
        self.retain_async(|_, _| false).await;
    }

    /// Clears the [`HashMap`] by removing all key-value pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// assert!(hashmap.insert_sync(1, 0).is_ok());
    /// hashmap.clear_sync();
    ///
    /// assert!(!hashmap.contains_sync(&1));
    /// ```
    #[inline]
    pub fn clear_sync(&self) {
        self.retain_sync(|_, _| false);
    }

    /// Returns the number of entries in the [`HashMap`].
    ///
    /// It reads the entire metadata area of the bucket array to calculate the number of valid
    /// entries, making its time complexity `O(N)`. Furthermore, it may overcount entries if an old
    /// bucket array has not yet been dropped.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// assert!(hashmap.insert_sync(1, 0).is_ok());
    /// assert_eq!(hashmap.len(), 1);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.num_entries(&Guard::new())
    }

    /// Returns `true` if the [`HashMap`] is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// assert!(hashmap.is_empty());
    /// assert!(hashmap.insert_sync(1, 0).is_ok());
    /// assert!(!hashmap.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        !self.has_entry(&Guard::new())
    }

    /// Returns the capacity of the [`HashMap`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap_default: HashMap<u64, u32> = HashMap::default();
    /// assert_eq!(hashmap_default.capacity(), 0);
    ///
    /// assert!(hashmap_default.insert_sync(1, 0).is_ok());
    /// assert_eq!(hashmap_default.capacity(), 64);
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::with_capacity(1000);
    /// assert_eq!(hashmap.capacity(), 1024);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.num_slots(&Guard::new())
    }

    /// Returns the current capacity range of the [`HashMap`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// assert_eq!(hashmap.capacity_range(), 0..=(1_usize << (usize::BITS - 2)));
    ///
    /// let reserved = hashmap.reserve(1000);
    /// assert_eq!(hashmap.capacity_range(), 1000..=(1_usize << (usize::BITS - 2)));
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
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::with_capacity(1024);
    ///
    /// let bucket_index = hashmap.bucket_index(&11);
    /// assert!(bucket_index < hashmap.capacity() / 32);
    /// ```
    #[inline]
    pub fn bucket_index<Q>(&self, key: &Q) -> usize
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        let hash = self.hash(key);
        self.calculate_bucket_index(hash)
    }
}

impl<K, V> HashMap<K, V, RandomState> {
    /// Creates an empty default [`HashMap`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::new();
    ///
    /// let result = hashmap.capacity();
    /// assert_eq!(result, 0);
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty [`HashMap`] with the specified capacity.
    ///
    /// The actual capacity is equal to or greater than `capacity` unless it is greater than
    /// `1 << (usize::BITS - 1)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::with_capacity(1000);
    ///
    /// let result = hashmap.capacity();
    /// assert_eq!(result, 1024);
    /// ```
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, RandomState::new())
    }
}

impl<K, V, H> Clone for HashMap<K, V, H>
where
    K: Clone + Eq + Hash,
    V: Clone,
    H: BuildHasher + Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        let self_clone = Self::with_capacity_and_hasher(self.capacity(), self.hasher().clone());
        self.iter_sync(|k, v| {
            let _result = self_clone.insert_sync(k.clone(), v.clone());
            true
        });
        self_clone
    }
}

impl<K, V, H> Debug for HashMap<K, V, H>
where
    K: Debug + Eq + Hash,
    V: Debug,
    H: BuildHasher,
{
    /// Iterates over all the entries in the [`HashMap`] to print them.
    ///
    /// # Locking behavior
    ///
    /// Shared locks on buckets are acquired during iteration, therefore any [`Entry`],
    /// [`OccupiedEntry`], or [`VacantEntry`] owned by the current thread will lead to deadlocks.
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_map();
        self.iter_sync(|k, v| {
            d.entry(k, v);
            true
        });
        d.finish()
    }
}

impl<K, V, H> Default for HashMap<K, V, H>
where
    H: BuildHasher + Default,
{
    /// Creates an empty default [`HashMap`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// let result = hashmap.capacity();
    /// assert_eq!(result, 0);
    /// ```
    #[inline]
    fn default() -> Self {
        Self::with_hasher(H::default())
    }
}

impl<K, V, H> Drop for HashMap<K, V, H>
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
                // remain outside the lifetime of the `HashMap`.
                a.drop_in_place()
            });
    }
}

impl<K, V, H> FromIterator<(K, V)> for HashMap<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher + Default,
{
    #[inline]
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let into_iter = iter.into_iter();
        let hashmap = Self::with_capacity_and_hasher(
            Self::capacity_from_size_hint(into_iter.size_hint()),
            H::default(),
        );
        into_iter.for_each(|e| {
            hashmap.upsert_sync(e.0, e.1);
        });
        hashmap
    }
}

impl<K, V, H> HashTable<K, V, H, (), MAP> for HashMap<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    #[inline]
    fn hasher(&self) -> &H {
        &self.build_hasher
    }

    #[inline]
    fn bucket_array_var(&self) -> &AtomicShared<BucketArray<K, V, (), MAP>> {
        &self.bucket_array
    }

    #[inline]
    fn minimum_capacity_var(&self) -> &AtomicUsize {
        &self.minimum_capacity
    }
}

impl<K, V, H> PartialEq for HashMap<K, V, H>
where
    K: Eq + Hash,
    V: PartialEq,
    H: BuildHasher,
{
    /// Compares two [`HashMap`] instances.
    ///
    /// # Locking behavior
    ///
    /// Shared locks on buckets are acquired when comparing two instances of [`HashMap`], therefore
    /// this may lead to deadlocks if the instances are being modified by another thread.
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.iter_sync(|k, v| other.read_sync(k, |_, ov| v == ov) == Some(true)) {
            return other.iter_sync(|k, v| self.read_sync(k, |_, sv| v == sv) == Some(true));
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
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// hashmap.entry_sync(3).or_insert(7);
    /// assert_eq!(hashmap.read_sync(&3, |_, v| *v), Some(7));
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
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// hashmap.entry_sync(19).or_insert_with(|| 5);
    /// assert_eq!(hashmap.read_sync(&19, |_, v| *v), Some(5));
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
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// hashmap.entry_sync(11).or_insert_with_key(|k| if *k == 11 { 7 } else { 3 });
    /// assert_eq!(hashmap.read_sync(&11, |_, v| *v), Some(7));
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
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    /// assert_eq!(hashmap.entry_sync(31).key(), &31);
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
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// hashmap.entry_sync(37).and_modify(|v| { *v += 1 }).or_insert(47);
    /// assert_eq!(hashmap.read_sync(&37, |_, v| *v), Some(47));
    ///
    /// hashmap.entry_sync(37).and_modify(|v| { *v += 1 }).or_insert(3);
    /// assert_eq!(hashmap.read_sync(&37, |_, v| *v), Some(48));
    /// ```
    #[inline]
    #[must_use]
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        match self {
            Self::Occupied(mut o) => {
                f(o.get_mut());
                Self::Occupied(o)
            }
            Self::Vacant(_) => self,
        }
    }

    /// Sets the value of the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    /// let entry = hashmap.entry_sync(11).insert_entry(17);
    /// assert_eq!(entry.key(), &11);
    /// ```
    #[inline]
    pub fn insert_entry(self, val: V) -> OccupiedEntry<'h, K, V, H> {
        match self {
            Self::Occupied(mut o) => {
                o.insert(val);
                o
            }
            Self::Vacant(v) => v.insert_entry(val),
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
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    /// hashmap.entry_sync(11).or_default();
    /// assert_eq!(hashmap.read_sync(&11, |_, v| *v), Some(0));
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
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// assert_eq!(hashmap.entry_sync(29).or_default().key(), &29);
    /// ```
    #[inline]
    #[must_use]
    pub fn key(&self) -> &K {
        &self.locked_bucket.entry(&self.entry_ptr).0
    }

    /// Takes ownership of the key and value from the [`HashMap`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    /// use scc::hash_map::Entry;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// hashmap.entry_sync(11).or_insert(17);
    ///
    /// if let Entry::Occupied(o) = hashmap.entry_sync(11) {
    ///     assert_eq!(o.remove_entry(), (11, 17));
    /// };
    /// ```
    #[inline]
    #[must_use]
    pub fn remove_entry(mut self) -> (K, V) {
        self.locked_bucket.remove(self.hashmap, &mut self.entry_ptr)
    }

    /// Gets a reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    /// use scc::hash_map::Entry;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// hashmap.entry_sync(19).or_insert(11);
    ///
    /// if let Entry::Occupied(o) = hashmap.entry_sync(19) {
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
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    /// use scc::hash_map::Entry;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// hashmap.entry_sync(37).or_insert(11);
    ///
    /// if let Entry::Occupied(mut o) = hashmap.entry_sync(37) {
    ///     *o.get_mut() += 18;
    ///     assert_eq!(*o.get(), 29);
    /// }
    ///
    /// assert_eq!(hashmap.read_sync(&37, |_, v| *v), Some(29));
    /// ```
    #[inline]
    pub fn get_mut(&mut self) -> &mut V {
        &mut self.locked_bucket.entry_mut(&mut self.entry_ptr).1
    }

    /// Sets the value of the entry, and returns the old value.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    /// use scc::hash_map::Entry;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// hashmap.entry_sync(37).or_insert(11);
    ///
    /// if let Entry::Occupied(mut o) = hashmap.entry_sync(37) {
    ///     assert_eq!(o.insert(17), 11);
    /// }
    ///
    /// assert_eq!(hashmap.read_sync(&37, |_, v| *v), Some(17));
    /// ```
    #[inline]
    pub fn insert(&mut self, val: V) -> V {
        replace(self.get_mut(), val)
    }

    /// Takes the value out of the entry, and returns it.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    /// use scc::hash_map::Entry;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// hashmap.entry_sync(11).or_insert(17);
    ///
    /// if let Entry::Occupied(o) = hashmap.entry_sync(11) {
    ///     assert_eq!(o.remove(), 17);
    /// };
    /// ```
    #[inline]
    #[must_use]
    pub fn remove(self) -> V {
        self.remove_entry().1
    }

    /// Removes the entry and gets the next closest occupied entry.
    ///
    /// [`HashMap::begin_async`] and this method together allow the [`OccupiedEntry`] to
    /// effectively act as a mutable iterator over entries. This method never acquires more than one
    /// lock, even when it searches other buckets for the next closest occupied entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    /// use scc::hash_map::Entry;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// assert!(hashmap.insert_sync(1, 0).is_ok());
    /// assert!(hashmap.insert_sync(2, 0).is_ok());
    ///
    /// let second_entry_future = hashmap.begin_sync().unwrap().remove_and_async();
    /// ```
    #[inline]
    pub async fn remove_and_async(self) -> ((K, V), Option<OccupiedEntry<'h, K, V, H>>) {
        let hashmap = self.hashmap;
        let mut entry_ptr = self.entry_ptr.clone();
        let entry = self
            .locked_bucket
            .writer
            .remove(self.locked_bucket.data_block, &mut entry_ptr);
        if let Some(locked_bucket) = self.locked_bucket.next_async(hashmap, &mut entry_ptr).await {
            return (
                entry,
                Some(OccupiedEntry {
                    hashmap,
                    locked_bucket,
                    entry_ptr,
                }),
            );
        }
        (entry, None)
    }

    /// Removes the entry and gets the next closest occupied entry.
    ///
    /// [`HashMap::begin_sync`] and this method together allow the [`OccupiedEntry`] to effectively
    /// act as a mutable iterator over entries. This method never acquires more than one lock, even
    /// when it searches other buckets for the next closest occupied entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    /// use scc::hash_map::Entry;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// assert!(hashmap.insert_sync(1, 0).is_ok());
    /// assert!(hashmap.insert_sync(2, 0).is_ok());
    ///
    /// let first_entry = hashmap.begin_sync().unwrap();
    /// let first_key = *first_entry.key();
    /// let (removed, second_entry) = first_entry.remove_and_sync();
    /// assert_eq!(removed.1, 0);
    /// assert_eq!(hashmap.len(), 1);
    ///
    /// let second_entry = second_entry.unwrap();
    /// let second_key = *second_entry.key();
    ///
    /// assert!(second_entry.remove_and_sync().1.is_none());
    /// assert_eq!(first_key + second_key, 3);
    /// ```
    #[inline]
    #[must_use]
    pub fn remove_and_sync(self) -> ((K, V), Option<Self>) {
        let hashmap = self.hashmap;
        let mut entry_ptr = self.entry_ptr.clone();
        let entry = self
            .locked_bucket
            .writer
            .remove(self.locked_bucket.data_block, &mut entry_ptr);
        if let Some(locked_bucket) = self.locked_bucket.next_sync(hashmap, &mut entry_ptr) {
            return (
                entry,
                Some(OccupiedEntry {
                    hashmap,
                    locked_bucket,
                    entry_ptr,
                }),
            );
        }
        (entry, None)
    }

    /// Gets the next closest occupied entry.
    ///
    /// [`HashMap::begin_async`] and this method together allow the [`OccupiedEntry`] to
    /// effectively act as a mutable iterator over entries. This method never acquires more than one
    /// lock, even when it searches other buckets for the next closest occupied entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    /// use scc::hash_map::Entry;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// assert!(hashmap.insert_sync(1, 0).is_ok());
    /// assert!(hashmap.insert_sync(2, 0).is_ok());
    ///
    /// let second_entry_future = hashmap.begin_sync().unwrap().next_async();
    /// ```
    #[inline]
    pub async fn next_async(self) -> Option<OccupiedEntry<'h, K, V, H>> {
        let hashmap = self.hashmap;
        let mut entry_ptr = self.entry_ptr.clone();
        if let Some(locked_bucket) = self.locked_bucket.next_async(hashmap, &mut entry_ptr).await {
            return Some(OccupiedEntry {
                hashmap,
                locked_bucket,
                entry_ptr,
            });
        }
        None
    }

    /// Gets the next closest occupied entry.
    ///
    /// [`HashMap::begin_sync`] and this method together allow the [`OccupiedEntry`] to effectively
    /// act as a mutable iterator over entries. This method never acquires more than one lock, even
    /// when it searches other buckets for the next closest occupied entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    /// use scc::hash_map::Entry;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// assert!(hashmap.insert_sync(1, 0).is_ok());
    /// assert!(hashmap.insert_sync(2, 0).is_ok());
    ///
    /// let first_entry = hashmap.begin_sync().unwrap();
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
        let hashmap = self.hashmap;
        let mut entry_ptr = self.entry_ptr.clone();
        if let Some(locked_bucket) = self.locked_bucket.next_sync(hashmap, &mut entry_ptr) {
            return Some(OccupiedEntry {
                hashmap,
                locked_bucket,
                entry_ptr,
            });
        }
        None
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
    K: Eq + Hash,
    H: BuildHasher,
{
    type Target = V;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<K, V, H> DerefMut for OccupiedEntry<'_, K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
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
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    /// assert_eq!(hashmap.entry_sync(11).key(), &11);
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
    /// use scc::HashMap;
    /// use scc::hash_map::Entry;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// if let Entry::Vacant(v) = hashmap.entry_sync(17) {
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
    /// use scc::HashMap;
    /// use scc::hash_map::Entry;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// if let Entry::Vacant(o) = hashmap.entry_sync(19) {
    ///     o.insert_entry(29);
    /// }
    ///
    /// assert_eq!(hashmap.read_sync(&19, |_, v| *v), Some(29));
    /// ```
    #[inline]
    pub fn insert_entry(self, val: V) -> OccupiedEntry<'h, K, V, H> {
        let entry_ptr = self.locked_bucket.insert(self.hash, (self.key, val));
        OccupiedEntry {
            hashmap: self.hashmap,
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

impl<K, V> ConsumableEntry<'_, K, V> {
    /// Consumes the entry by moving out the key and value.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashMap;
    ///
    /// let hashmap: HashMap<u64, u32> = HashMap::default();
    ///
    /// assert!(hashmap.insert_sync(1, 0).is_ok());
    /// assert!(hashmap.insert_sync(2, 1).is_ok());
    /// assert!(hashmap.insert_sync(3, 2).is_ok());
    ///
    /// let mut consumed = None;
    ///
    /// hashmap.iter_mut_sync(|entry| {
    ///     if entry.0 == 1 {
    ///         consumed.replace(entry.consume().1);
    ///     }
    ///     true
    /// });
    ///
    /// assert!(!hashmap.contains_sync(&1));
    /// assert_eq!(consumed, Some(0));
    /// ```
    #[inline]
    #[must_use]
    pub fn consume(self) -> (K, V) {
        *self.remove_probe |= true;
        self.locked_bucket
            .writer
            .remove(self.locked_bucket.data_block, self.entry_ptr)
    }
}

impl<K, V> Deref for ConsumableEntry<'_, K, V> {
    type Target = (K, V);

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.locked_bucket.entry(self.entry_ptr)
    }
}

impl<K, V> DerefMut for ConsumableEntry<'_, K, V> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.locked_bucket.entry_mut(self.entry_ptr)
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

impl<K, V, H> AsRef<HashMap<K, V, H>> for Reserve<'_, K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    #[inline]
    fn as_ref(&self) -> &HashMap<K, V, H> {
        self.hashmap
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
    type Target = HashMap<K, V, H>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.hashmap
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
            .hashmap
            .minimum_capacity
            .fetch_sub(self.additional, Relaxed);
        debug_assert!(result >= self.additional);

        let guard = Guard::new();
        if let Some(current_array) = self.hashmap.bucket_array(&guard) {
            self.try_shrink(current_array, 0, &guard);
        }
    }
}
