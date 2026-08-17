//! [`HashSet`] is an asynchronous/concurrent hash set.

#![deny(unsafe_code)]

use std::collections::hash_map::RandomState;
use std::fmt::{self, Debug};
use std::hash::{BuildHasher, Hash};
use std::mem::swap;
use std::ops::{Deref, RangeInclusive};

use super::hash_map;
use super::hash_table::HashTable;
use super::{Equivalent, HashMap};

/// Scalable asynchronous/concurrent hash set.
///
/// [`HashSet`] is an asynchronous/concurrent hash set based on [`HashMap`].
pub struct HashSet<K, H = RandomState>
where
    H: BuildHasher,
{
    map: HashMap<K, (), H>,
}

/// [`ConsumableEntry`] is a view into an occupied entry in a [`HashSet`] when iterating over
/// entries in it.
pub struct ConsumableEntry<'w, K> {
    consumable: hash_map::ConsumableEntry<'w, K, ()>,
}

/// [`Reserve`] keeps the capacity of the associated [`HashSet`] higher than a certain level.
///
/// The [`HashSet`] does not shrink the capacity below the reserved capacity.
pub type Reserve<'h, K, H = RandomState> = super::hash_map::Reserve<'h, K, (), H>;

impl<K, H> HashSet<K, H>
where
    H: BuildHasher,
{
    /// Creates an empty [`HashSet`] with the given [`BuildHasher`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    /// use std::collections::hash_map::RandomState;
    ///
    /// let hashset: HashSet<u64, RandomState> = HashSet::with_hasher(RandomState::new());
    /// ```
    #[cfg(not(feature = "loom"))]
    #[inline]
    pub const fn with_hasher(build_hasher: H) -> Self {
        Self {
            map: HashMap::with_hasher(build_hasher),
        }
    }

    /// Creates an empty [`HashSet`] with the given [`BuildHasher`].
    #[cfg(feature = "loom")]
    #[inline]
    pub fn with_hasher(build_hasher: H) -> Self {
        Self {
            map: HashMap::with_hasher(build_hasher),
        }
    }

    /// Creates an empty [`HashSet`] with the specified capacity and [`BuildHasher`].
    ///
    /// The actual capacity is equal to or greater than `capacity` unless it is greater than
    /// `1 << (usize::BITS - 1)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    /// use std::collections::hash_map::RandomState;
    ///
    /// let hashset: HashSet<u64, RandomState> =
    ///     HashSet::with_capacity_and_hasher(1000, RandomState::new());
    ///
    /// let result = hashset.capacity();
    /// assert_eq!(result, 1024);
    /// ```
    #[inline]
    pub fn with_capacity_and_hasher(capacity: usize, build_hasher: H) -> Self {
        Self {
            map: HashMap::with_capacity_and_hasher(capacity, build_hasher),
        }
    }
}

impl<K, H> HashSet<K, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    /// Temporarily increases the minimum capacity of the [`HashSet`].
    ///
    /// A [`Reserve`] is returned if the [`HashSet`] could increase the minimum capacity while the
    /// increased capacity is not exclusively owned by the returned [`Reserve`], allowing others to
    /// benefit from it. The memory for the additional space may not be immediately allocated if
    /// the [`HashSet`] is empty or currently being resized, however once the memory is reserved
    /// eventually, the capacity will not shrink below the additional capacity until the returned
    /// [`Reserve`] is dropped.
    ///
    /// # Errors
    ///
    /// Returns `None` if a too large number is given.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<usize> = HashSet::with_capacity(1000);
    /// assert_eq!(hashset.capacity(), 1024);
    ///
    /// let reserved = hashset.reserve(10000);
    /// assert!(reserved.is_some());
    /// assert_eq!(hashset.capacity(), 16384);
    ///
    /// assert!(hashset.reserve(usize::MAX).is_none());
    /// assert_eq!(hashset.capacity(), 16384);
    ///
    /// for i in 0..16 {
    ///     assert!(hashset.insert_sync(i).is_ok());
    /// }
    /// drop(reserved);
    ///
    /// assert_eq!(hashset.capacity(), 1024);
    /// ```
    #[inline]
    pub fn reserve(&self, capacity: usize) -> Option<Reserve<'_, K, H>> {
        self.map.reserve(capacity)
    }

    /// Inserts a key into the [`HashSet`].
    ///
    /// # Errors
    ///
    /// Returns an error along with the supplied key if the key exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::default();
    /// let future_insert = hashset.insert_async(11);
    /// ```
    #[inline]
    pub async fn insert_async(&self, key: K) -> Result<(), K> {
        self.map.insert_async(key, ()).await.map_err(|(k, ())| k)
    }

    /// Inserts a key into the [`HashSet`].
    ///
    /// # Errors
    ///
    /// Returns an error along with the supplied key if the key exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::default();
    ///
    /// assert!(hashset.insert_sync(1).is_ok());
    /// assert_eq!(hashset.insert_sync(1).unwrap_err(), 1);
    /// ```
    #[inline]
    pub fn insert_sync(&self, key: K) -> Result<(), K> {
        if let Err((k, ())) = self.map.insert_sync(key, ()) {
            return Err(k);
        }
        Ok(())
    }

    /// Adds a key to the set, replacing the existing key, if any, that is equal to the given one.
    ///
    /// Returns the replaced key, if any.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::cmp::{Eq, PartialEq};
    /// use std::hash::{Hash, Hasher};
    ///
    /// use scc::HashSet;
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
    /// let hashset: HashSet<MaybeEqual> = HashSet::default();
    ///
    /// assert!(hashset.replace_sync(MaybeEqual(11, 7)).is_none());
    /// assert_eq!(hashset.replace_sync(MaybeEqual(11, 11)), Some(MaybeEqual(11, 7)));
    /// ```
    #[inline]
    pub async fn replace_async(&self, mut key: K) -> Option<K> {
        let hash = self.map.hash(&key);
        let mut locked_bucket = self.map.writer_async(hash).await;
        let mut entry_ptr = locked_bucket.search(&key, hash);
        if entry_ptr.is_valid() {
            let k = &mut locked_bucket.entry_mut(&mut entry_ptr).0;
            swap(k, &mut key);
            Some(key)
        } else {
            locked_bucket.insert(hash, (key, ()));
            None
        }
    }

    /// Adds a key to the set, replacing the existing key, if any, that is equal to the given one.
    ///
    /// Returns the replaced key, if any.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::cmp::{Eq, PartialEq};
    /// use std::hash::{Hash, Hasher};
    ///
    /// use scc::HashSet;
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
    /// let hashset: HashSet<MaybeEqual> = HashSet::default();
    ///
    /// async {
    ///     assert!(hashset.replace_async(MaybeEqual(11, 7)).await.is_none());
    ///     assert_eq!(hashset.replace_async(MaybeEqual(11, 11)).await, Some(MaybeEqual(11, 7)));
    /// };
    /// ```
    #[inline]
    pub fn replace_sync(&self, mut key: K) -> Option<K> {
        let hash = self.map.hash(&key);
        let mut locked_bucket = self.map.writer_sync(hash);
        let mut entry_ptr = locked_bucket.search(&key, hash);
        if entry_ptr.is_valid() {
            let k = &mut locked_bucket.entry_mut(&mut entry_ptr).0;
            swap(k, &mut key);
            Some(key)
        } else {
            locked_bucket.insert(hash, (key, ()));
            None
        }
    }

    /// Removes a key if the key exists.
    ///
    /// Returns `None` if the key does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::default();
    /// let future_insert = hashset.insert_async(11);
    /// let future_remove = hashset.remove_async(&11);
    /// ```
    #[inline]
    pub async fn remove_async<Q>(&self, key: &Q) -> Option<K>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.map
            .remove_if_async(key, |()| true)
            .await
            .map(|(k, ())| k)
    }

    /// Removes a key if the key exists.
    ///
    /// Returns `None` if the key does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::default();
    ///
    /// assert!(hashset.remove_sync(&1).is_none());
    /// assert!(hashset.insert_sync(1).is_ok());
    /// assert_eq!(hashset.remove_sync(&1).unwrap(), 1);
    /// ```
    #[inline]
    pub fn remove_sync<Q>(&self, key: &Q) -> Option<K>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.map.remove_sync(key).map(|(k, ())| k)
    }

    /// Removes a key if the key exists and the given condition is met.
    ///
    /// Returns `None` if the key does not exist or the condition was not met.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::default();
    /// let future_insert = hashset.insert_async(11);
    /// let future_remove = hashset.remove_if_async(&11, || true);
    /// ```
    #[inline]
    pub async fn remove_if_async<Q, F: FnOnce() -> bool>(&self, key: &Q, condition: F) -> Option<K>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.map
            .remove_if_async(key, |()| condition())
            .await
            .map(|(k, ())| k)
    }

    /// Removes a key if the key exists and the given condition is met.
    ///
    /// Returns `None` if the key does not exist or the condition was not met.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::default();
    ///
    /// assert!(hashset.insert_sync(1).is_ok());
    /// assert!(hashset.remove_if_sync(&1, || false).is_none());
    /// assert_eq!(hashset.remove_if_sync(&1, || true).unwrap(), 1);
    /// ```
    #[inline]
    pub fn remove_if_sync<Q, F: FnOnce() -> bool>(&self, key: &Q, condition: F) -> Option<K>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.map
            .remove_if_sync(key, |()| condition())
            .map(|(k, ())| k)
    }

    /// Reads a key.
    ///
    /// Returns `None` if the key does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::default();
    /// let future_insert = hashset.insert_async(11);
    /// let future_read = hashset.read_async(&11, |k| *k);
    /// ```
    #[inline]
    pub async fn read_async<Q, R, F: FnOnce(&K) -> R>(&self, key: &Q, reader: F) -> Option<R>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.map.read_async(key, |k, ()| reader(k)).await
    }

    /// Reads a key.
    ///
    /// Returns `None` if the key does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::default();
    ///
    /// assert!(hashset.read_sync(&1, |_| true).is_none());
    /// assert!(hashset.insert_sync(1).is_ok());
    /// assert!(hashset.read_sync(&1, |_| true).unwrap());
    /// ```
    #[inline]
    pub fn read_sync<Q, R, F: FnOnce(&K) -> R>(&self, key: &Q, reader: F) -> Option<R>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.map.read_sync(key, |k, ()| reader(k))
    }

    /// Returns `true` if the [`HashSet`] contains the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::default();
    ///
    /// let future_contains = hashset.contains_async(&1);
    /// ```
    #[inline]
    pub async fn contains_async<Q>(&self, key: &Q) -> bool
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.map.contains_async(key).await
    }

    /// Returns `true` if the [`HashSet`] contains the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::default();
    ///
    /// assert!(!hashset.contains_sync(&1));
    /// assert!(hashset.insert_sync(1).is_ok());
    /// assert!(hashset.contains_sync(&1));
    /// ```
    #[inline]
    pub fn contains_sync<Q>(&self, key: &Q) -> bool
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.read_sync(key, |_| ()).is_some()
    }

    /// Iterates over entries asynchronously for reading.
    ///
    /// Stops iterating when the closure returns `false`, and this method also returns `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::default();
    ///
    /// assert!(hashset.insert_sync(1).is_ok());
    ///
    /// async {
    ///     let result = hashset.iter_async(|k| {
    ///         true
    ///     }).await;
    ///     assert!(result);
    /// };
    /// ```
    #[inline]
    pub async fn iter_async<F: FnMut(&K) -> bool>(&self, mut f: F) -> bool {
        self.map.iter_async(|k, ()| f(k)).await
    }

    /// Iterates over entries synchronously for reading.
    ///
    /// Stops iterating when the closure returns `false`, and this method also returns `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::default();
    ///
    /// assert!(hashset.insert_sync(1).is_ok());
    /// assert!(hashset.insert_sync(2).is_ok());
    ///
    /// let mut acc = 0;
    /// let result = hashset.iter_sync(|k| {
    ///     acc += *k;
    ///     true
    /// });
    ///
    /// assert!(result);
    /// assert_eq!(acc, 3);
    /// ```
    #[inline]
    pub fn iter_sync<F: FnMut(&K) -> bool>(&self, mut f: F) -> bool {
        self.map.iter_sync(|k, ()| f(k))
    }

    /// Iterates over entries asynchronously for modification.
    ///
    /// This method stops iterating when the closure returns `false`, and also returns `false` in
    /// that case.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::default();
    ///
    /// assert!(hashset.insert_sync(1).is_ok());
    /// assert!(hashset.insert_sync(2).is_ok());
    ///
    /// async {
    ///     let result = hashset.iter_mut_async(|entry| {
    ///         if *entry == 1 {
    ///             entry.consume();
    ///             return false;
    ///         }
    ///         true
    ///     }).await;
    ///
    ///     assert!(!result);
    ///     assert_eq!(hashset.len(), 1);
    /// };
    /// ```
    #[inline]
    pub async fn iter_mut_async<F: FnMut(ConsumableEntry<'_, K>) -> bool>(&self, mut f: F) -> bool {
        self.map
            .iter_mut_async(|consumable| f(ConsumableEntry { consumable }))
            .await
    }

    /// Iterates over entries synchronously for modification.
    ///
    /// This method stops iterating when the closure returns `false`, and also returns `false` in
    /// that case.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::default();
    ///
    /// assert!(hashset.insert_sync(1).is_ok());
    /// assert!(hashset.insert_sync(2).is_ok());
    /// assert!(hashset.insert_sync(3).is_ok());
    ///
    /// let result = hashset.iter_mut_sync(|entry| {
    ///     if *entry == 1 {
    ///         entry.consume();
    ///         return false;
    ///     }
    ///     true
    /// });
    ///
    /// assert!(!result);
    /// assert!(!hashset.contains_sync(&1));
    /// assert_eq!(hashset.len(), 2);
    /// ```
    #[inline]
    pub fn iter_mut_sync<F: FnMut(ConsumableEntry<'_, K>) -> bool>(&self, mut f: F) -> bool {
        self.map
            .iter_mut_sync(|consumable| f(ConsumableEntry { consumable }))
    }

    /// Retains keys that satisfy the given predicate.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::default();
    ///
    /// let future_insert = hashset.insert_async(1);
    /// let future_retain = hashset.retain_async(|k| *k == 1);
    /// ```
    #[inline]
    pub async fn retain_async<F: FnMut(&K) -> bool>(&self, mut filter: F) {
        self.map.retain_async(|k, ()| filter(k)).await;
    }

    /// Retains keys that satisfy the given predicate.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::default();
    ///
    /// assert!(hashset.insert_sync(1).is_ok());
    /// assert!(hashset.insert_sync(2).is_ok());
    /// assert!(hashset.insert_sync(3).is_ok());
    ///
    /// hashset.retain_sync(|k| *k == 1);
    ///
    /// assert!(hashset.contains_sync(&1));
    /// assert!(!hashset.contains_sync(&2));
    /// assert!(!hashset.contains_sync(&3));
    /// ```
    #[inline]
    pub fn retain_sync<F: FnMut(&K) -> bool>(&self, mut pred: F) {
        self.iter_mut_sync(|e| {
            if !pred(&e) {
                drop(e.consume());
            }
            true
        });
    }

    /// Clears the [`HashSet`] by removing all keys.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::default();
    ///
    /// let future_insert = hashset.insert_async(1);
    /// let future_clear = hashset.clear_async();
    /// ```
    #[inline]
    pub async fn clear_async(&self) {
        self.map.clear_async().await;
    }

    /// Clears the [`HashSet`] by removing all keys.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::default();
    ///
    /// assert!(hashset.insert_sync(1).is_ok());
    /// hashset.clear_sync();
    ///
    /// assert!(!hashset.contains_sync(&1));
    /// ```
    #[inline]
    pub fn clear_sync(&self) {
        self.map.clear_sync();
    }

    /// Returns the number of entries in the [`HashSet`].
    ///
    /// It reads the entire metadata area of the bucket array to calculate the number of valid
    /// entries, making its time complexity `O(N)`. Furthermore, it may overcount entries if an old
    /// bucket array has yet to be dropped.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::default();
    ///
    /// assert!(hashset.insert_sync(1).is_ok());
    /// assert_eq!(hashset.len(), 1);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns `true` if the [`HashSet`] is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::default();
    ///
    /// assert!(hashset.is_empty());
    /// assert!(hashset.insert_sync(1).is_ok());
    /// assert!(!hashset.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Returns the capacity of the [`HashSet`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset_default: HashSet<u64> = HashSet::default();
    /// assert_eq!(hashset_default.capacity(), 0);
    ///
    /// assert!(hashset_default.insert_sync(1).is_ok());
    /// assert_eq!(hashset_default.capacity(), 64);
    ///
    /// let hashset: HashSet<u64> = HashSet::with_capacity(1000);
    /// assert_eq!(hashset.capacity(), 1024);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.map.capacity()
    }

    /// Returns the current capacity range of the [`HashSet`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::default();
    ///
    /// assert_eq!(hashset.capacity_range(), 0..=(1_usize << (usize::BITS - 2)));
    ///
    /// let reserved = hashset.reserve(1000);
    /// assert_eq!(hashset.capacity_range(), 1000..=(1_usize << (usize::BITS - 2)));
    /// ```
    #[inline]
    pub fn capacity_range(&self) -> RangeInclusive<usize> {
        self.map.capacity_range()
    }

    /// Returns the index of the bucket that may contain the key.
    ///
    /// The method returns the index of the bucket associated with the key. The number of buckets
    /// can be calculated by dividing the capacity by `32`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::with_capacity(1024);
    ///
    /// let bucket_index = hashset.bucket_index(&11);
    /// assert!(bucket_index < hashset.capacity() / 32);
    /// ```
    #[inline]
    pub fn bucket_index<Q>(&self, key: &Q) -> usize
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.map.bucket_index(key)
    }
}

impl<K, H> Clone for HashSet<K, H>
where
    K: Clone + Eq + Hash,
    H: BuildHasher + Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            map: self.map.clone(),
        }
    }
}

impl<K, H> Debug for HashSet<K, H>
where
    K: Debug + Eq + Hash,
    H: BuildHasher,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_set();
        self.iter_sync(|k| {
            d.entry(k);
            true
        });
        d.finish()
    }
}

impl<K: Eq + Hash> HashSet<K, RandomState> {
    /// Creates an empty default [`HashSet`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::new();
    ///
    /// let result = hashset.capacity();
    /// assert_eq!(result, 0);
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty [`HashSet`] with the specified capacity.
    ///
    /// The actual capacity is equal to or greater than `capacity` unless it is greater than
    /// `1 << (usize::BITS - 1)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::with_capacity(1000);
    ///
    /// let result = hashset.capacity();
    /// assert_eq!(result, 1024);
    /// ```
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
        }
    }
}

impl<K, H> Default for HashSet<K, H>
where
    H: BuildHasher + Default,
{
    /// Creates an empty default [`HashSet`].
    ///
    /// The default capacity is `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::default();
    ///
    /// let result = hashset.capacity();
    /// assert_eq!(result, 0);
    /// ```
    #[inline]
    fn default() -> Self {
        Self {
            map: HashMap::default(),
        }
    }
}

impl<K, H> FromIterator<K> for HashSet<K, H>
where
    K: Eq + Hash,
    H: BuildHasher + Default,
{
    #[inline]
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        let into_iter = iter.into_iter();
        let hashset = Self::with_capacity_and_hasher(
            HashMap::<K, (), H>::capacity_from_size_hint(into_iter.size_hint()),
            H::default(),
        );
        into_iter.for_each(|k| {
            let _result = hashset.insert_sync(k);
        });
        hashset
    }
}

impl<K, H> PartialEq for HashSet<K, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    /// Compares two [`HashSet`] instances.
    ///
    /// ### Locking behavior
    ///
    /// Shared locks on buckets are acquired when comparing two instances of [`HashSet`], therefore
    /// it may lead to a deadlock if the instances are being modified by another thread.
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.iter_sync(|k| other.contains_sync(k)) {
            return other.iter_sync(|k| self.contains_sync(k));
        }
        false
    }
}

impl<K> ConsumableEntry<'_, K> {
    /// Consumes the entry by moving out the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use scc::HashSet;
    ///
    /// let hashset: HashSet<u64> = HashSet::default();
    ///
    /// assert!(hashset.insert_sync(1).is_ok());
    /// assert!(hashset.insert_sync(2).is_ok());
    /// assert!(hashset.insert_sync(3).is_ok());
    ///
    /// let mut consumed = None;
    ///
    /// hashset.iter_mut_sync(|entry| {
    ///     if *entry == 1 {
    ///         consumed.replace(entry.consume());
    ///     }
    ///     true
    /// });
    ///
    /// assert!(!hashset.contains_sync(&1));
    /// assert_eq!(consumed, Some(1));
    /// ```
    #[inline]
    #[must_use]
    pub fn consume(self) -> K {
        self.consumable.consume().0
    }
}

impl<K> Deref for ConsumableEntry<'_, K> {
    type Target = K;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.consumable.0
    }
}
