//! [`HashCache`] is an asynchronous/concurrent 32-way associative cache backed by
//! [`HashMap`](super::HashMap).

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
use super::hash_table::MAXIMUM_CAPACITY_LIMIT;
use super::hash_table::bucket::{CACHE, DoublyLinkedList, EntryPtr};
use super::hash_table::bucket_array::BucketArray;
use super::hash_table::{HashTable, LockedBucket};

/// Scalable asynchronous/concurrent 32-way associative cache backed by [`HashMap`](super::HashMap).
///
/// [`HashCache`] is an asynchronous/concurrent 32-way associative cache based on the
/// [`HashMap`](super::HashMap) implementation. [`HashCache`] does not keep track of the least
/// recently used entry in the entire cache. Instead, each bucket maintains a doubly linked list of
/// occupied entries, updated on access to entries to keep track of the least recently used entries
/// within the bucket. Therefore, entries can be evicted before the cache is full.
///
/// [`HashCache`] and [`HashMap`](super::HashMap) share the same runtime characteristics, except
/// that each entry in a [`HashCache`] additionally uses 2-byte space for a doubly linked list and a
/// [`HashCache`] starts evicting least recently used entries if the bucket is full instead of
/// allocating a linked list of entries.
///
/// ## Unwind safety
///
/// [`HashCache`] is impervious to out-of-memory errors and panics in user-specified code under one
/// condition: `H::Hasher::hash`, `K::drop` and `V::drop` must not panic.
pub struct HashCache<K, V, H = RandomState>
where
    H: BuildHasher,
{
    bucket_array: AtomicShared<BucketArray<K, V, DoublyLinkedList, CACHE>>,
    minimum_capacity: AtomicUsize,
    maximum_capacity: usize,
    build_hasher: H,
}

/// The default maximum capacity of a [`HashCache`] is `256`.
pub const DEFAULT_MAXIMUM_CAPACITY: usize = 256;

/// [`EvictedEntry`] is a type alias for `Option<(K, V)>`.
pub type EvictedEntry<K, V> = Option<(K, V)>;

/// [`Entry`] represents a single cache entry in a [`HashCache`].
pub enum Entry<'h, K, V, H = RandomState>
where
    H: BuildHasher,
{
    /// An occupied entry.
    Occupied(OccupiedEntry<'h, K, V, H>),
    /// A vacant entry.
    Vacant(VacantEntry<'h, K, V, H>),
}

/// [`OccupiedEntry`] is a view into an occupied cache entry in a [`HashCache`].
pub struct OccupiedEntry<'h, K, V, H = RandomState>
where
    H: BuildHasher,
{
    hashcache: &'h HashCache<K, V, H>,
    locked_bucket: LockedBucket<K, V, DoublyLinkedList, CACHE>,
    entry_ptr: EntryPtr<K, V, CACHE>,
}

/// [`VacantEntry`] is a view into a vacant cache entry in a [`HashCache`].
pub struct VacantEntry<'h, K, V, H = RandomState>
where
    H: BuildHasher,
{
    hashcache: &'h HashCache<K, V, H>,
    key: K,
    hash: u64,
    locked_bucket: LockedBucket<K, V, DoublyLinkedList, CACHE>,
}

/// [`ConsumableEntry`] is a view into an occupied entry in a [`HashCache`] when iterating over
/// entries in it.
pub struct ConsumableEntry<'b, K, V> {
    /// Holds an exclusive lock on the entry bucket.
    locked_bucket: &'b mut LockedBucket<K, V, DoublyLinkedList, CACHE>,
    /// Pointer to the entry.
    entry_ptr: &'b mut EntryPtr<K, V, CACHE>,
    /// Probes removal.
    remove_probe: &'b mut bool,
}

/// [`ReplaceResult`] is the result type of the [`HashCache::replace_async`] and
/// [`HashCache::replace_sync`] methods.
pub enum ReplaceResult<'h, K, V, H = RandomState>
where
    H: BuildHasher,
{
    /// The key was replaced.
    Replaced(OccupiedEntry<'h, K, V, H>, K),
    /// The key did not exist in the [`HashCache`].
    ///
    /// An [`OccupiedEntry`] can be created from the [`VacantEntry`].
    NotReplaced(VacantEntry<'h, K, V, H>),
}

impl<K, V, H> HashCache<K, V, H>
where
    H: BuildHasher,
{
    /// Creates an empty [`HashCache`] with the given [`BuildHasher`].
    ///
    /// The maximum capacity is set to [`DEFAULT_MAXIMUM_CAPACITY`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    /// use std::collections::hash_map::RandomState;
    ///
    /// let hashcache: HashCache<u64, u32, RandomState> = HashCache::with_hasher(RandomState::new());
    /// ```
    #[cfg(not(feature = "loom"))]
    #[inline]
    pub const fn with_hasher(build_hasher: H) -> Self {
        HashCache {
            bucket_array: AtomicShared::null(),
            minimum_capacity: AtomicUsize::new(0),
            maximum_capacity: DEFAULT_MAXIMUM_CAPACITY,
            build_hasher,
        }
    }

    /// Creates an empty [`HashCache`] with the given [`BuildHasher`].
    #[cfg(feature = "loom")]
    #[inline]
    pub fn with_hasher(build_hasher: H) -> Self {
        Self {
            bucket_array: AtomicShared::null(),
            minimum_capacity: AtomicUsize::new(0),
            maximum_capacity: DEFAULT_MAXIMUM_CAPACITY,
            build_hasher,
        }
    }

    /// Creates an empty [`HashCache`] with the specified capacity and [`BuildHasher`].
    ///
    /// The actual capacity is equal to or greater than `minimum_capacity` unless it is greater than
    /// `1 << (usize::BITS - 1)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    /// use std::collections::hash_map::RandomState;
    ///
    /// let hashcache: HashCache<u64, u32, RandomState> =
    ///     HashCache::with_capacity_and_hasher(1000, 2000, RandomState::new());
    ///
    /// let result = hashcache.capacity();
    /// assert_eq!(result, 1024);
    /// ```
    #[inline]
    pub fn with_capacity_and_hasher(
        minimum_capacity: usize,
        maximum_capacity: usize,
        build_hasher: H,
    ) -> Self {
        let (array, minimum_capacity) = if minimum_capacity == 0 {
            (AtomicShared::null(), AtomicUsize::new(0))
        } else {
            let array = unsafe {
                Shared::new_with_unchecked(|| {
                    BucketArray::<K, V, DoublyLinkedList, CACHE>::new(
                        minimum_capacity,
                        AtomicShared::null(),
                    )
                })
            };
            let minimum_capacity = array.num_slots();
            (
                AtomicShared::from(array),
                AtomicUsize::new(minimum_capacity),
            )
        };
        let maximum_capacity = maximum_capacity
            .max(minimum_capacity.load(Relaxed))
            .max(BucketArray::<K, V, DoublyLinkedList, CACHE>::minimum_capacity())
            .min(MAXIMUM_CAPACITY_LIMIT)
            .next_power_of_two();
        HashCache {
            bucket_array: array,
            minimum_capacity,
            maximum_capacity,
            build_hasher,
        }
    }
}

impl<K, V, H> HashCache<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    /// Gets the entry associated with the given key in the map for in-place manipulation.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<char, u32> = HashCache::default();
    ///
    /// let future_entry = hashcache.entry_async('b');
    /// ```
    #[inline]
    pub async fn entry_async(&self, key: K) -> Entry<'_, K, V, H> {
        let hash = self.hash(&key);
        let locked_bucket = self.writer_async(hash).await;
        let entry_ptr = locked_bucket.search(&key, hash);
        if entry_ptr.is_valid() {
            Entry::Occupied(OccupiedEntry {
                hashcache: self,
                locked_bucket,
                entry_ptr,
            })
        } else {
            let vacant_entry = VacantEntry {
                hashcache: self,
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
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<char, u32> = HashCache::default();
    ///
    /// for ch in "a short treatise on fungi".chars() {
    ///     hashcache.entry_sync(ch).and_modify(|counter| *counter += 1).or_put(1);
    /// }
    ///
    /// assert_eq!(*hashcache.get_sync(&'s').unwrap().get(), 2);
    /// assert_eq!(*hashcache.get_sync(&'t').unwrap().get(), 3);
    /// assert!(hashcache.get_sync(&'y').is_none());
    /// ```
    #[inline]
    pub fn entry_sync(&self, key: K) -> Entry<'_, K, V, H> {
        let hash = self.hash(&key);
        let locked_bucket = self.writer_sync(hash);
        let entry_ptr = locked_bucket.search(&key, hash);
        if entry_ptr.is_valid() {
            Entry::Occupied(OccupiedEntry {
                hashcache: self,
                locked_bucket,
                entry_ptr,
            })
        } else {
            let vacant_entry = VacantEntry {
                hashcache: self,
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
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<usize, usize> = HashCache::default();
    ///
    /// assert!(hashcache.put_sync(0, 1).is_ok());
    /// assert!(hashcache.try_entry(0).is_some());
    /// ```
    #[inline]
    pub fn try_entry(&self, key: K) -> Option<Entry<'_, K, V, H>> {
        let hash = self.hash(&key);
        let guard = Guard::new();
        let locked_bucket = self.try_reserve_bucket(hash, &guard)?;
        let entry_ptr = locked_bucket.search(&key, hash);
        if entry_ptr.is_valid() {
            Some(Entry::Occupied(OccupiedEntry {
                hashcache: self,
                locked_bucket,
                entry_ptr,
            }))
        } else {
            Some(Entry::Vacant(VacantEntry {
                hashcache: self,
                key,
                hash,
                locked_bucket,
            }))
        }
    }

    /// Puts a key-value pair into the [`HashCache`].
    ///
    /// Returns `Some` if an entry was evicted for the new key-value pair.
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
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    /// let future_put = hashcache.put_async(11, 17);
    /// ```
    #[inline]
    pub async fn put_async(&self, key: K, val: V) -> Result<EvictedEntry<K, V>, (K, V)> {
        let hash = self.hash(&key);
        let locked_bucket = self.writer_async(hash).await;
        if locked_bucket.search(&key, hash).is_valid() {
            Err((key, val))
        } else {
            let evicted = locked_bucket.evict_lru_head(locked_bucket.data_block);
            let entry_ptr = locked_bucket.insert(hash, (key, val));
            locked_bucket.update_lru_tail(&entry_ptr);
            Ok(evicted)
        }
    }

    /// Puts a key-value pair into the [`HashCache`].
    ///
    /// Returns `Some` if an entry was evicted for the new key-value pair.
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
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// assert!(hashcache.put_sync(1, 0).is_ok());
    /// assert_eq!(hashcache.put_sync(1, 1).unwrap_err(), (1, 1));
    /// ```
    #[inline]
    pub fn put_sync(&self, key: K, val: V) -> Result<EvictedEntry<K, V>, (K, V)> {
        let hash = self.hash(&key);
        let locked_bucket = self.writer_sync(hash);
        let entry_ptr = locked_bucket.search(&key, hash);
        if entry_ptr.is_valid() {
            Err((key, val))
        } else {
            let evicted = locked_bucket
                .writer
                .evict_lru_head(locked_bucket.data_block);
            let entry_ptr = locked_bucket.insert(hash, (key, val));
            locked_bucket.writer.update_lru_tail(&entry_ptr);
            Ok(evicted)
        }
    }

    /// Adds a key to the [`HashCache`], replacing the existing key, if any, that is equal to the
    /// given one.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::cmp::{Eq, PartialEq};
    /// use std::hash::{Hash, Hasher};
    ///
    /// use scc::HashCache;
    /// use scc::hash_cache::ReplaceResult;
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
    /// let hashcache: HashCache<MaybeEqual, usize> = HashCache::default();
    ///
    /// async {
    ///     let ReplaceResult::NotReplaced(v) = hashcache.replace_async(MaybeEqual(11, 7)).await else {
    ///         unreachable!();
    ///     };
    ///     drop(v.put_entry(17));
    ///     let ReplaceResult::Replaced(_, k) = hashcache.replace_async(MaybeEqual(11, 11)).await else {
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
                    hashcache: self,
                    locked_bucket,
                    entry_ptr,
                },
                prev_key,
            )
        } else {
            ReplaceResult::NotReplaced(VacantEntry {
                hashcache: self,
                key,
                hash,
                locked_bucket,
            })
        }
    }

    /// Adds a key to the [`HashCache`], replacing the existing key, if any, that is equal to the
    /// given one.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::cmp::{Eq, PartialEq};
    /// use std::hash::{Hash, Hasher};
    ///
    /// use scc::HashCache;
    /// use scc::hash_cache::ReplaceResult;
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
    /// let hashcache: HashCache<MaybeEqual, usize> = HashCache::default();
    ///
    /// let ReplaceResult::NotReplaced(v) = hashcache.replace_sync(MaybeEqual(11, 7)) else {
    ///     unreachable!();
    /// };
    /// drop(v.put_entry(17));
    /// let ReplaceResult::Replaced(_, k) = hashcache.replace_sync(MaybeEqual(11, 11)) else {
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
                    hashcache: self,
                    locked_bucket,
                    entry_ptr,
                },
                prev_key,
            )
        } else {
            ReplaceResult::NotReplaced(VacantEntry {
                hashcache: self,
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
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    /// let future_put = hashcache.put_async(11, 17);
    /// let future_remove = hashcache.remove_async(&11);
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
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// assert!(hashcache.remove_sync(&1).is_none());
    /// assert!(hashcache.put_sync(1, 0).is_ok());
    /// assert_eq!(hashcache.remove_sync(&1).unwrap(), (1, 0));
    /// ```
    #[inline]
    pub fn remove_sync<Q>(&self, key: &Q) -> Option<(K, V)>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.remove_if_sync(key, |_| true)
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
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    /// let future_put = hashcache.put_async(11, 17);
    /// let future_get = hashcache.get_async(&11);
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
            locked_bucket.writer.update_lru_tail(&entry_ptr);
            return Some(OccupiedEntry {
                hashcache: self,
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
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// assert!(hashcache.get_sync(&1).is_none());
    /// assert!(hashcache.put_sync(1, 10).is_ok());
    /// assert_eq!(*hashcache.get_sync(&1).unwrap().get(), 10);
    ///
    /// *hashcache.get_sync(&1).unwrap() = 11;
    /// assert_eq!(*hashcache.get_sync(&1).unwrap(), 11);
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
            locked_bucket.writer.update_lru_tail(&entry_ptr);
            return Some(OccupiedEntry {
                hashcache: self,
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
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    /// let future_put = hashcache.put_async(11, 17);
    /// let future_read = hashcache.read_async(&11, |_, v| *v);
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
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// assert!(hashcache.read_sync(&1, |_, v| *v).is_none());
    /// assert!(hashcache.put_sync(1, 10).is_ok());
    /// assert_eq!(hashcache.read_sync(&1, |_, v| *v).unwrap(), 10);
    /// ```
    #[inline]
    pub fn read_sync<Q, R, F: FnOnce(&K, &V) -> R>(&self, key: &Q, reader: F) -> Option<R>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.reader_sync(key, reader)
    }

    /// Returns `true` if the [`HashCache`] contains a value for the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// let future_contains = hashcache.contains_async(&1);
    /// ```
    #[inline]
    pub async fn contains_async<Q>(&self, key: &Q) -> bool
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.reader_async(key, |_, _| ()).await.is_some()
    }

    /// Returns `true` if the [`HashCache`] contains a value for the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// assert!(!hashcache.contains_sync(&1));
    /// assert!(hashcache.put_sync(1, 0).is_ok());
    /// assert!(hashcache.contains_sync(&1));
    /// ```
    #[inline]
    pub fn contains_sync<Q>(&self, key: &Q) -> bool
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.read_sync(key, |_, _| ()).is_some()
    }

    /// Removes a key-value pair if the key exists and the given condition is met.
    ///
    /// Returns `None` if the key does not exist or the condition was not met.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    /// let future_put = hashcache.put_async(11, 17);
    /// let future_remove = hashcache.remove_if_async(&11, |_| true);
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
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// assert!(hashcache.put_sync(1, 0).is_ok());
    /// assert!(hashcache.remove_if_sync(&1, |v| { *v += 1; false }).is_none());
    /// assert_eq!(hashcache.remove_if_sync(&1, |v| *v == 1).unwrap(), (1, 1));
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

    /// Iterates over entries asynchronously for reading.
    ///
    /// Stops iterating when the closure returns `false`, and this method also returns `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u64> = HashCache::default();
    ///
    /// assert!(hashcache.put_sync(1, 0).is_ok());
    ///
    /// async {
    ///     let result = hashcache.iter_async(|k, v| {
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
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u64> = HashCache::default();
    ///
    /// assert!(hashcache.put_sync(1, 0).is_ok());
    /// assert!(hashcache.put_sync(2, 1).is_ok());
    ///
    /// let mut acc = 0_u64;
    /// let result = hashcache.iter_sync(|k, v| {
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
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// assert!(hashcache.put_sync(1, 0).is_ok());
    /// assert!(hashcache.put_sync(2, 1).is_ok());
    ///
    /// async {
    ///     let result = hashcache.iter_mut_async(|entry| {
    ///         if entry.0 == 1 {
    ///             entry.consume();
    ///             return false;
    ///         }
    ///         true
    ///     }).await;
    ///
    ///     assert!(!result);
    ///     assert_eq!(hashcache.len(), 1);
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
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// assert!(hashcache.put_sync(1, 0).is_ok());
    /// assert!(hashcache.put_sync(2, 1).is_ok());
    /// assert!(hashcache.put_sync(3, 2).is_ok());
    ///
    /// let result = hashcache.iter_mut_sync(|entry| {
    ///     if entry.0 == 1 {
    ///         entry.consume();
    ///         return false;
    ///     }
    ///     true
    /// });
    ///
    /// assert!(!result);
    /// assert!(!hashcache.contains_sync(&1));
    /// assert_eq!(hashcache.len(), 2);
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
    /// [`HashCache`] gets resized by another thread.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// let future_put = hashcache.put_async(1, 0);
    /// let future_retain = hashcache.retain_async(|k, v| *k == 1);
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
    /// [`HashCache`] gets resized by another thread.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// assert!(hashcache.put_sync(1, 0).is_ok());
    /// assert!(hashcache.put_sync(2, 1).is_ok());
    /// assert!(hashcache.put_sync(3, 2).is_ok());
    ///
    /// hashcache.retain_sync(|k, v| *k == 1 && *v == 0);
    ///
    /// assert!(hashcache.contains_sync(&1));
    /// assert!(!hashcache.contains_sync(&2));
    /// assert!(!hashcache.contains_sync(&3));
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

    /// Clears the [`HashCache`] by removing all key-value pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// let future_put = hashcache.put_async(1, 0);
    /// let future_clear = hashcache.clear_async();
    /// ```
    #[inline]
    pub async fn clear_async(&self) {
        self.retain_async(|_, _| false).await;
    }

    /// Clears the [`HashCache`] by removing all key-value pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// assert!(hashcache.put_sync(1, 0).is_ok());
    /// hashcache.clear_sync();
    ///
    /// assert!(!hashcache.contains_sync(&1));
    /// ```
    #[inline]
    pub fn clear_sync(&self) {
        self.retain_sync(|_, _| false);
    }

    /// Returns the number of entries in the [`HashCache`].
    ///
    /// It reads the entire metadata area of the bucket array to calculate the number of valid
    /// entries, making its time complexity `O(N)`. Furthermore, it may overcount entries if an old
    /// bucket array has yet to be dropped.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// assert!(hashcache.put_sync(1, 0).is_ok());
    /// assert_eq!(hashcache.len(), 1);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.num_entries(&Guard::new())
    }

    /// Returns `true` if the [`HashCache`] is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// assert!(hashcache.is_empty());
    /// assert!(hashcache.put_sync(1, 0).is_ok());
    /// assert!(!hashcache.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        !self.has_entry(&Guard::new())
    }

    /// Returns the capacity of the [`HashCache`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    ///
    /// let hashcache_default: HashCache<u64, u32> = HashCache::default();
    /// assert_eq!(hashcache_default.capacity(), 0);
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::with_capacity(1000, 2000);
    /// assert_eq!(hashcache.capacity(), 1024);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.num_slots(&Guard::new())
    }

    /// Returns the current capacity range of the [`HashCache`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// assert_eq!(hashcache.capacity_range(), 0..=256);
    /// ```
    #[inline]
    pub fn capacity_range(&self) -> RangeInclusive<usize> {
        self.minimum_capacity.load(Relaxed)..=self.maximum_capacity()
    }
}

impl<K, V> HashCache<K, V, RandomState> {
    /// Creates an empty default [`HashCache`].
    ///
    /// The maximum capacity is set to [`DEFAULT_MAXIMUM_CAPACITY`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::new();
    ///
    /// let result = hashcache.capacity();
    /// assert_eq!(result, 0);
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty [`HashCache`] with the specified capacity.
    ///
    /// The actual capacity is equal to or greater than `minimum_capacity` unless it is greater than
    /// `1 << (usize::BITS - 1)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::with_capacity(1000, 2000);
    ///
    /// let result = hashcache.capacity();
    /// assert_eq!(result, 1024);
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::with_capacity(0, 0);
    /// let result = hashcache.capacity_range();
    /// assert_eq!(result, 0..=64);
    /// ```
    #[inline]
    #[must_use]
    pub fn with_capacity(minimum_capacity: usize, maximum_capacity: usize) -> Self {
        Self::with_capacity_and_hasher(minimum_capacity, maximum_capacity, RandomState::new())
    }
}

impl<K, V, H> Default for HashCache<K, V, H>
where
    H: BuildHasher + Default,
{
    /// Creates an empty default [`HashCache`].
    ///
    /// The maximum capacity is set to [`DEFAULT_MAXIMUM_CAPACITY`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// let result = hashcache.capacity();
    /// assert_eq!(result, 0);
    /// ```
    #[inline]
    fn default() -> Self {
        Self::with_hasher(H::default())
    }
}

impl<K, V, H> Debug for HashCache<K, V, H>
where
    K: Debug + Eq + Hash,
    V: Debug,
    H: BuildHasher,
{
    /// Iterates over all the entries in the [`HashCache`] to print them.
    ///
    /// ## Locking behavior
    ///
    /// Shared locks on buckets are acquired during iteration, therefore any [`Entry`],
    /// [`OccupiedEntry`], or [`VacantEntry`] owned by the current thread will lead to a deadlock.
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

impl<K, V, H> Drop for HashCache<K, V, H>
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
                // remain outside the lifetime of the `HashCache`.
                a.drop_in_place()
            });
    }
}

impl<K, V, H> FromIterator<(K, V)> for HashCache<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher + Default,
{
    #[inline]
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let into_iter = iter.into_iter();
        let size_hint = into_iter.size_hint();
        let hashcache = Self::with_capacity_and_hasher(
            size_hint.0,
            Self::capacity_from_size_hint(size_hint),
            H::default(),
        );
        into_iter.for_each(|e| {
            let _result = hashcache.put_sync(e.0, e.1);
        });
        hashcache
    }
}

impl<K, V, H> HashTable<K, V, H, DoublyLinkedList, CACHE> for HashCache<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    #[inline]
    fn hasher(&self) -> &H {
        &self.build_hasher
    }

    #[inline]
    fn bucket_array_var(&self) -> &AtomicShared<BucketArray<K, V, DoublyLinkedList, CACHE>> {
        &self.bucket_array
    }

    #[inline]
    fn minimum_capacity_var(&self) -> &AtomicUsize {
        &self.minimum_capacity
    }

    #[inline]
    fn maximum_capacity(&self) -> usize {
        self.maximum_capacity
    }
}

impl<K, V, H> PartialEq for HashCache<K, V, H>
where
    K: Eq + Hash,
    V: PartialEq,
    H: BuildHasher,
{
    /// Compares two [`HashCache`] instances.
    ///
    /// ## Locking behavior
    ///
    /// Shared locks on buckets are acquired when comparing two instances of [`HashCache`], therefore
    /// this may lead to a deadlock if the instances are being modified by another thread.
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
    /// Ensures a value is in the entry by putting the supplied instance if empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// hashcache.entry_sync(3).or_put(7);
    /// assert_eq!(*hashcache.get_sync(&3).unwrap().get(), 7);
    /// ```
    #[inline]
    pub fn or_put(self, val: V) -> (EvictedEntry<K, V>, OccupiedEntry<'h, K, V, H>) {
        self.or_put_with(|| val)
    }

    /// Ensures a value is in the entry by putting the result of the supplied closure if empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// hashcache.entry_sync(19).or_put_with(|| 5);
    /// assert_eq!(*hashcache.get_sync(&19).unwrap().get(), 5);
    /// ```
    #[inline]
    pub fn or_put_with<F: FnOnce() -> V>(
        self,
        constructor: F,
    ) -> (EvictedEntry<K, V>, OccupiedEntry<'h, K, V, H>) {
        self.or_put_with_key(|_| constructor())
    }

    /// Ensures a value is in the entry by putting the result of the supplied closure if empty.
    ///
    /// The reference to the moved key is provided, therefore cloning or copying the key is
    /// unnecessary.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// hashcache.entry_sync(11).or_put_with_key(|k| if *k == 11 { 7 } else { 3 });
    /// assert_eq!(*hashcache.get_sync(&11).unwrap().get(), 7);
    /// ```
    #[inline]
    pub fn or_put_with_key<F: FnOnce(&K) -> V>(
        self,
        constructor: F,
    ) -> (EvictedEntry<K, V>, OccupiedEntry<'h, K, V, H>) {
        match self {
            Self::Occupied(o) => (None, o),
            Self::Vacant(v) => {
                let val = constructor(v.key());
                v.put_entry(val)
            }
        }
    }

    /// Returns a reference to the key of this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    /// assert_eq!(hashcache.entry_sync(31).key(), &31);
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
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// hashcache.entry_sync(37).and_modify(|v| { *v += 1 }).or_put(47);
    /// assert_eq!(*hashcache.get_sync(&37).unwrap().get(), 47);
    ///
    /// hashcache.entry_sync(37).and_modify(|v| { *v += 1 }).or_put(3);
    /// assert_eq!(*hashcache.get_sync(&37).unwrap().get(), 48);
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
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    /// let entry = hashcache.entry_sync(11).put_entry(17).1;
    /// assert_eq!(entry.key(), &11);
    /// ```
    #[inline]
    pub fn put_entry(self, val: V) -> (EvictedEntry<K, V>, OccupiedEntry<'h, K, V, H>) {
        match self {
            Self::Occupied(mut o) => {
                o.put(val);
                (None, o)
            }
            Self::Vacant(v) => v.put_entry(val),
        }
    }
}

impl<'h, K, V, H> Entry<'h, K, V, H>
where
    K: Eq + Hash,
    V: Default,
    H: BuildHasher,
{
    /// Ensures a value is in the entry by putting the default value if empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    /// hashcache.entry_sync(11).or_default();
    /// assert_eq!(*hashcache.get_sync(&11).unwrap().get(), 0);
    /// ```
    #[inline]
    pub fn or_default(self) -> (EvictedEntry<K, V>, OccupiedEntry<'h, K, V, H>) {
        match self {
            Self::Occupied(o) => (None, o),
            Self::Vacant(v) => v.put_entry(Default::default()),
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

impl<K, V, H> OccupiedEntry<'_, K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    /// Gets a reference to the key in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// assert_eq!(hashcache.entry_sync(29).or_default().1.key(), &29);
    /// ```
    #[inline]
    #[must_use]
    pub fn key(&self) -> &K {
        &self.locked_bucket.entry(&self.entry_ptr).0
    }

    /// Takes ownership of the key and value from the [`HashCache`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    /// use scc::hash_cache::Entry;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// hashcache.entry_sync(11).or_put(17);
    ///
    /// if let Entry::Occupied(o) = hashcache.entry_sync(11) {
    ///     assert_eq!(o.remove_entry(), (11, 17));
    /// };
    /// ```
    #[inline]
    #[must_use]
    pub fn remove_entry(mut self) -> (K, V) {
        self.locked_bucket
            .remove(self.hashcache, &mut self.entry_ptr)
    }

    /// Gets a reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    /// use scc::hash_cache::Entry;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// hashcache.entry_sync(19).or_put(11);
    ///
    /// if let Entry::Occupied(o) = hashcache.entry_sync(19) {
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
    /// use scc::HashCache;
    /// use scc::hash_cache::Entry;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// hashcache.entry_sync(37).or_put(11);
    ///
    /// if let Entry::Occupied(mut o) = hashcache.entry_sync(37) {
    ///     *o.get_mut() += 18;
    ///     assert_eq!(*o.get(), 29);
    /// }
    ///
    /// assert_eq!(*hashcache.get_sync(&37).unwrap().get(), 29);
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
    /// use scc::HashCache;
    /// use scc::hash_cache::Entry;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// hashcache.entry_sync(37).or_put(11);
    ///
    /// if let Entry::Occupied(mut o) = hashcache.entry_sync(37) {
    ///     assert_eq!(o.put(17), 11);
    /// }
    ///
    /// assert_eq!(*hashcache.get_sync(&37).unwrap().get(), 17);
    /// ```
    #[inline]
    pub fn put(&mut self, val: V) -> V {
        replace(self.get_mut(), val)
    }

    /// Takes the value out of the entry, and returns it.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    /// use scc::hash_cache::Entry;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// hashcache.entry_sync(11).or_put(17);
    ///
    /// if let Entry::Occupied(o) = hashcache.entry_sync(11) {
    ///     assert_eq!(o.remove(), 17);
    /// };
    /// ```
    #[inline]
    #[must_use]
    pub fn remove(self) -> V {
        self.remove_entry().1
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
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    /// assert_eq!(hashcache.entry_sync(11).key(), &11);
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
    /// use scc::HashCache;
    /// use scc::hash_cache::Entry;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// if let Entry::Vacant(v) = hashcache.entry_sync(17) {
    ///     assert_eq!(v.into_key(), 17);
    /// };
    /// ```
    #[inline]
    pub fn into_key(self) -> K {
        self.key
    }

    /// Sets the value of the entry with its key and returns an [`OccupiedEntry`].
    ///
    /// Returns a key-value pair if an entry was evicted for the new key-value pair.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashCache;
    /// use scc::hash_cache::Entry;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// if let Entry::Vacant(o) = hashcache.entry_sync(19) {
    ///     o.put_entry(29);
    /// }
    ///
    /// assert_eq!(*hashcache.get_sync(&19).unwrap().get(), 29);
    /// ```
    #[inline]
    pub fn put_entry(self, val: V) -> (EvictedEntry<K, V>, OccupiedEntry<'h, K, V, H>) {
        let evicted = self
            .locked_bucket
            .writer
            .evict_lru_head(self.locked_bucket.data_block);
        let entry_ptr = self.locked_bucket.insert(self.hash, (self.key, val));
        self.locked_bucket.writer.update_lru_tail(&entry_ptr);
        let occupied = OccupiedEntry {
            hashcache: self.hashcache,
            locked_bucket: self.locked_bucket,
            entry_ptr,
        };
        (evicted, occupied)
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
    /// use scc::HashCache;
    ///
    /// let hashcache: HashCache<u64, u32> = HashCache::default();
    ///
    /// assert!(hashcache.put_sync(1, 0).is_ok());
    /// assert!(hashcache.put_sync(2, 1).is_ok());
    /// assert!(hashcache.put_sync(3, 2).is_ok());
    ///
    /// let mut consumed = None;
    ///
    /// hashcache.iter_mut_sync(|entry| {
    ///     if entry.0 == 1 {
    ///         consumed.replace(entry.consume().1);
    ///     }
    ///     true
    /// });
    ///
    /// assert!(!hashcache.contains_sync(&1));
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
