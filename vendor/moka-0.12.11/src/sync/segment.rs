use equivalent::Equivalent;

use super::{cache::Cache, CacheBuilder, OwnedKeyEntrySelector, RefKeyEntrySelector};
use crate::common::concurrent::Weigher;
use crate::common::time::Clock;
use crate::{
    common::{
        iter::{Iter, ScanningGet},
        HousekeeperConfig,
    },
    notification::EvictionListener,
    policy::{EvictionPolicy, ExpirationPolicy},
    Entry, Policy, PredicateError,
};

use std::{
    collections::hash_map::RandomState,
    fmt,
    hash::{BuildHasher, Hash, Hasher},
    sync::Arc,
};

/// A thread-safe concurrent in-memory cache, with multiple internal segments.
///
/// `SegmentedCache` has multiple internal [`Cache`][cache-struct] instances for
/// increased concurrent update performance. However, it has little overheads on
/// retrievals and updates for managing these segments.
///
/// For usage examples, see the document of the [`Cache`][cache-struct].
///
/// [cache-struct]: ./struct.Cache.html
///
pub struct SegmentedCache<K, V, S = RandomState> {
    inner: Arc<Inner<K, V, S>>,
}

unsafe impl<K, V, S> Send for SegmentedCache<K, V, S>
where
    K: Send + Sync,
    V: Send + Sync,
    S: Send,
{
}

unsafe impl<K, V, S> Sync for SegmentedCache<K, V, S>
where
    K: Send + Sync,
    V: Send + Sync,
    S: Sync,
{
}

impl<K, V, S> Clone for SegmentedCache<K, V, S> {
    /// Makes a clone of this shared cache.
    ///
    /// This operation is cheap as it only creates thread-safe reference counted
    /// pointers to the shared internal data structures.
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<K, V, S> fmt::Debug for SegmentedCache<K, V, S>
where
    K: fmt::Debug + Eq + Hash + Send + Sync + 'static,
    V: fmt::Debug + Clone + Send + Sync + 'static,
    // TODO: Remove these bounds from S.
    S: BuildHasher + Clone + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d_map = f.debug_map();

        for (k, v) in self {
            d_map.entry(&k, &v);
        }

        d_map.finish()
    }
}

impl<K, V> SegmentedCache<K, V, RandomState>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    /// Constructs a new `SegmentedCache<K, V>` that has multiple internal
    /// segments and will store up to the `max_capacity`.
    ///
    /// To adjust various configuration knobs such as `initial_capacity` or
    /// `time_to_live`, use the [`CacheBuilder`][builder-struct].
    ///
    /// [builder-struct]: ./struct.CacheBuilder.html
    ///
    /// # Panics
    ///
    /// Panics if `num_segments` is 0.
    pub fn new(max_capacity: u64, num_segments: usize) -> Self {
        let build_hasher = RandomState::default();
        Self::with_everything(
            None,
            Some(max_capacity),
            None,
            num_segments,
            build_hasher,
            None,
            EvictionPolicy::default(),
            None,
            ExpirationPolicy::default(),
            HousekeeperConfig::default(),
            false,
            Clock::default(),
        )
    }

    /// Returns a [`CacheBuilder`][builder-struct], which can builds a
    /// `SegmentedCache` with various configuration knobs.
    ///
    /// [builder-struct]: ./struct.CacheBuilder.html
    pub fn builder(num_segments: usize) -> CacheBuilder<K, V, SegmentedCache<K, V, RandomState>> {
        CacheBuilder::default().segments(num_segments)
    }
}

impl<K, V, S> SegmentedCache<K, V, S> {
    /// Returns cache’s name.
    pub fn name(&self) -> Option<&str> {
        self.inner.segments[0].name()
    }

    /// Returns a read-only cache policy of this cache.
    ///
    /// At this time, cache policy cannot be modified after cache creation.
    /// A future version may support to modify it.
    pub fn policy(&self) -> Policy {
        let mut policy = self.inner.segments[0].policy();
        policy.set_max_capacity(self.inner.desired_capacity);
        policy.set_num_segments(self.inner.segments.len());
        policy
    }

    /// Returns an approximate number of entries in this cache.
    ///
    /// The value returned is _an estimate_; the actual count may differ if there are
    /// concurrent insertions or removals, or if some entries are pending removal due
    /// to expiration. This inaccuracy can be mitigated by performing a
    /// `run_pending_tasks` first.
    ///
    /// # Example
    ///
    /// ```rust
    /// use moka::sync::SegmentedCache;
    ///
    /// let cache = SegmentedCache::new(10, 4);
    /// cache.insert('n', "Netherland Dwarf");
    /// cache.insert('l', "Lop Eared");
    /// cache.insert('d', "Dutch");
    ///
    /// // Ensure an entry exists.
    /// assert!(cache.contains_key(&'n'));
    ///
    /// // However, followings may print stale number zeros instead of threes.
    /// println!("{}", cache.entry_count());   // -> 0
    /// println!("{}", cache.weighted_size()); // -> 0
    ///
    /// // To mitigate the inaccuracy, call `run_pending_tasks` method to run
    /// // pending internal tasks.
    /// cache.run_pending_tasks();
    ///
    /// // Followings will print the actual numbers.
    /// println!("{}", cache.entry_count());   // -> 3
    /// println!("{}", cache.weighted_size()); // -> 3
    /// ```
    ///
    pub fn entry_count(&self) -> u64 {
        self.inner
            .segments
            .iter()
            .map(|seg| seg.entry_count())
            .sum()
    }

    /// Returns an approximate total weighted size of entries in this cache.
    ///
    /// The value returned is _an estimate_; the actual size may differ if there are
    /// concurrent insertions or removals, or if some entries are pending removal due
    /// to expiration. This inaccuracy can be mitigated by performing a
    /// `run_pending_tasks` first. See [`entry_count`](#method.entry_count) for a
    /// sample code.
    pub fn weighted_size(&self) -> u64 {
        self.inner
            .segments
            .iter()
            .map(|seg| seg.weighted_size())
            .sum()
    }
}

impl<K, V, S> SegmentedCache<K, V, S>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
    S: BuildHasher + Clone + Send + Sync + 'static,
{
    /// # Panics
    ///
    /// Panics if `num_segments` is 0.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn with_everything(
        name: Option<String>,
        max_capacity: Option<u64>,
        initial_capacity: Option<usize>,
        num_segments: usize,
        build_hasher: S,
        weigher: Option<Weigher<K, V>>,
        eviction_policy: EvictionPolicy,
        eviction_listener: Option<EvictionListener<K, V>>,
        expiration_policy: ExpirationPolicy<K, V>,
        housekeeper_config: HousekeeperConfig,
        invalidator_enabled: bool,
        clock: Clock,
    ) -> Self {
        Self {
            inner: Arc::new(Inner::new(
                name,
                max_capacity,
                initial_capacity,
                num_segments,
                build_hasher,
                weigher,
                eviction_policy,
                eviction_listener,
                expiration_policy,
                housekeeper_config,
                invalidator_enabled,
                clock,
            )),
        }
    }

    /// Returns `true` if the cache contains a value for the key.
    ///
    /// Unlike the `get` method, this method is not considered a cache read operation,
    /// so it does not update the historic popularity estimator or reset the idle
    /// timer for the key.
    ///
    /// The key may be any borrowed form of the cache's key type, but `Hash` and `Eq`
    /// on the borrowed form _must_ match those for the key type.
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        let hash = self.inner.hash(key);
        self.inner.select(hash).contains_key_with_hash(key, hash)
    }

    /// Returns a _clone_ of the value corresponding to the key.
    ///
    /// If you want to store values that will be expensive to clone, wrap them by
    /// `std::sync::Arc` before storing in a cache. [`Arc`][rustdoc-std-arc] is a
    /// thread-safe reference-counted pointer and its `clone()` method is cheap.
    ///
    /// The key may be any borrowed form of the cache's key type, but `Hash` and `Eq`
    /// on the borrowed form _must_ match those for the key type.
    ///
    /// [rustdoc-std-arc]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html
    pub fn get<Q>(&self, key: &Q) -> Option<V>
    where
        Q: Equivalent<K> + Hash + Eq + ?Sized,
    {
        let hash = self.inner.hash(key);
        self.inner
            .select(hash)
            .get_with_hash(key, hash, false)
            .map(Entry::into_value)
    }

    pub fn entry(&self, key: K) -> OwnedKeyEntrySelector<'_, K, V, S>
    where
        K: Hash + Eq,
    {
        let hash = self.inner.hash(&key);
        let cache = self.inner.select(hash);
        OwnedKeyEntrySelector::new(key, hash, cache)
    }

    pub fn entry_by_ref<'a, Q>(&'a self, key: &'a Q) -> RefKeyEntrySelector<'a, K, Q, V, S>
    where
        Q: Equivalent<K> + ToOwned<Owned = K> + Hash + ?Sized,
    {
        let hash = self.inner.hash(key);
        let cache = self.inner.select(hash);
        RefKeyEntrySelector::new(key, hash, cache)
    }

    /// TODO: Remove this in v0.13.0.
    /// Deprecated, replaced with [`get_with`](#method.get_with)
    #[deprecated(since = "0.8.0", note = "Replaced with `get_with`")]
    pub fn get_or_insert_with(&self, key: K, init: impl FnOnce() -> V) -> V {
        self.get_with(key, init)
    }

    /// TODO: Remove this in v0.13.0.
    /// Deprecated, replaced with [`try_get_with`](#method.try_get_with)
    #[deprecated(since = "0.8.0", note = "Replaced with `try_get_with`")]
    pub fn get_or_try_insert_with<F, E>(&self, key: K, init: F) -> Result<V, Arc<E>>
    where
        F: FnOnce() -> Result<V, E>,
        E: Send + Sync + 'static,
    {
        self.try_get_with(key, init)
    }

    /// Returns a _clone_ of the value corresponding to the key. If the value does
    /// not exist, evaluates the `init` closure and inserts the output.
    ///
    /// # Concurrent calls on the same key
    ///
    /// This method guarantees that concurrent calls on the same not-existing key are
    /// coalesced into one evaluation of the `init` closure. Only one of the calls
    /// evaluates its closure, and other calls wait for that closure to complete. See
    /// [`Cache::get_with`][get-with-method] for more details.
    ///
    /// [get-with-method]: ./struct.Cache.html#method.get_with
    pub fn get_with(&self, key: K, init: impl FnOnce() -> V) -> V {
        let hash = self.inner.hash(&key);
        let key = Arc::new(key);
        let replace_if = None as Option<fn(&V) -> bool>;
        self.inner
            .select(hash)
            .get_or_insert_with_hash_and_fun(key, hash, init, replace_if, false)
            .into_value()
    }

    /// Similar to [`get_with`](#method.get_with), but instead of passing an owned
    /// key, you can pass a reference to the key. If the key does not exist in the
    /// cache, the key will be cloned to create new entry in the cache.
    pub fn get_with_by_ref<Q>(&self, key: &Q, init: impl FnOnce() -> V) -> V
    where
        Q: Equivalent<K> + ToOwned<Owned = K> + Hash + ?Sized,
    {
        let hash = self.inner.hash(key);
        let replace_if = None as Option<fn(&V) -> bool>;
        self.inner
            .select(hash)
            .get_or_insert_with_hash_by_ref_and_fun(key, hash, init, replace_if, false)
            .into_value()
    }

    /// Works like [`get_with`](#method.get_with), but takes an additional
    /// `replace_if` closure.
    ///
    /// This method will evaluate the `init` closure and insert the output to the
    /// cache when:
    ///
    /// - The key does not exist.
    /// - Or, `replace_if` closure returns `true`.
    pub fn get_with_if(
        &self,
        key: K,
        init: impl FnOnce() -> V,
        replace_if: impl FnMut(&V) -> bool,
    ) -> V {
        let hash = self.inner.hash(&key);
        let key = Arc::new(key);
        self.inner
            .select(hash)
            .get_or_insert_with_hash_and_fun(key, hash, init, Some(replace_if), false)
            .into_value()
    }

    /// Returns a _clone_ of the value corresponding to the key. If the value does
    /// not exist, evaluates the `init` closure, and inserts the value if
    /// `Some(value)` was returned. If `None` was returned from the closure, this
    /// method does not insert a value and returns `None`.
    ///
    /// # Concurrent calls on the same key
    ///
    /// This method guarantees that concurrent calls on the same not-existing key are
    /// coalesced into one evaluation of the `init` closure. Only one of the calls
    /// evaluates its closure, and other calls wait for that closure to complete.
    /// See [`Cache::optionally_get_with`][opt-get-with-method] for more details.
    ///
    /// [opt-get-with-method]: ./struct.Cache.html#method.optionally_get_with
    pub fn optionally_get_with<F>(&self, key: K, init: F) -> Option<V>
    where
        F: FnOnce() -> Option<V>,
    {
        let hash = self.inner.hash(&key);
        let key = Arc::new(key);
        self.inner
            .select(hash)
            .get_or_optionally_insert_with_hash_and_fun(key, hash, init, false)
            .map(Entry::into_value)
    }

    /// Similar to [`optionally_get_with`](#method.optionally_get_with), but instead
    /// of passing an owned key, you can pass a reference to the key. If the key does
    /// not exist in the cache, the key will be cloned to create new entry in the
    /// cache.
    pub fn optionally_get_with_by_ref<F, Q>(&self, key: &Q, init: F) -> Option<V>
    where
        F: FnOnce() -> Option<V>,
        Q: Equivalent<K> + ToOwned<Owned = K> + Hash + ?Sized,
    {
        let hash = self.inner.hash(key);
        self.inner
            .select(hash)
            .get_or_optionally_insert_with_hash_by_ref_and_fun(key, hash, init, false)
            .map(Entry::into_value)
    }

    /// Returns a _clone_ of the value corresponding to the key. If the value does
    /// not exist, evaluates the `init` closure, and inserts the value if `Ok(value)`
    /// was returned. If `Err(_)` was returned from the closure, this method does not
    /// insert a value and returns the `Err` wrapped by [`std::sync::Arc`][std-arc].
    ///
    /// [std-arc]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html
    ///
    /// # Concurrent calls on the same key
    ///
    /// This method guarantees that concurrent calls on the same not-existing key are
    /// coalesced into one evaluation of the `init` closure (as long as these
    /// closures return the same error type). Only one of the calls evaluates its
    /// closure, and other calls wait for that closure to complete. See
    /// [`Cache::try_get_with`][try-get-with-method] for more details.
    ///
    /// [try-get-with-method]: ./struct.Cache.html#method.try_get_with
    pub fn try_get_with<F, E>(&self, key: K, init: F) -> Result<V, Arc<E>>
    where
        F: FnOnce() -> Result<V, E>,
        E: Send + Sync + 'static,
    {
        let hash = self.inner.hash(&key);
        let key = Arc::new(key);
        self.inner
            .select(hash)
            .get_or_try_insert_with_hash_and_fun(key, hash, init, false)
            .map(Entry::into_value)
    }

    /// Similar to [`try_get_with`](#method.try_get_with), but instead of passing an
    /// owned key, you can pass a reference to the key. If the key does not exist in
    /// the cache, the key will be cloned to create new entry in the cache.
    pub fn try_get_with_by_ref<F, E, Q>(&self, key: &Q, init: F) -> Result<V, Arc<E>>
    where
        F: FnOnce() -> Result<V, E>,
        E: Send + Sync + 'static,
        Q: Equivalent<K> + ToOwned<Owned = K> + Hash + ?Sized,
    {
        let hash = self.inner.hash(key);
        self.inner
            .select(hash)
            .get_or_try_insert_with_hash_by_ref_and_fun(key, hash, init, false)
            .map(Entry::into_value)
    }

    /// Inserts a key-value pair into the cache.
    ///
    /// If the cache has this key present, the value is updated.
    pub fn insert(&self, key: K, value: V) {
        let hash = self.inner.hash(&key);
        let key = Arc::new(key);
        self.inner.select(hash).insert_with_hash(key, hash, value);
    }

    /// Discards any cached value for the key.
    ///
    /// If you need to get a the value that has been discarded, use the
    /// [`remove`](#method.remove) method instead.
    ///
    /// The key may be any borrowed form of the cache's key type, but `Hash` and `Eq`
    /// on the borrowed form _must_ match those for the key type.
    pub fn invalidate<Q>(&self, key: &Q)
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        let hash = self.inner.hash(key);
        self.inner
            .select(hash)
            .invalidate_with_hash(key, hash, false);
    }

    /// Discards any cached value for the key and returns a clone of the value.
    ///
    /// If you do not need to get the value that has been discarded, use the
    /// [`invalidate`](#method.invalidate) method instead.
    ///
    /// The key may be any borrowed form of the cache's key type, but `Hash` and `Eq`
    /// on the borrowed form _must_ match those for the key type.
    pub fn remove<Q>(&self, key: &Q) -> Option<V>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        let hash = self.inner.hash(key);
        self.inner
            .select(hash)
            .invalidate_with_hash(key, hash, true)
    }

    /// Discards all cached values.
    ///
    /// This method returns immediately by just setting the current time as the
    /// invalidation time. `get` and other retrieval methods are guaranteed not to
    /// return the entries inserted before or at the invalidation time.
    ///
    /// The actual removal of the invalidated entries is done as a maintenance task
    /// driven by a user thread. For more details, see
    /// [the Maintenance Tasks section](../index.html#maintenance-tasks) in the crate
    /// level documentation.
    ///
    /// Like the `invalidate` method, this method does not clear the historic
    /// popularity estimator of keys so that it retains the client activities of
    /// trying to retrieve an item.
    pub fn invalidate_all(&self) {
        for segment in self.inner.segments.iter() {
            segment.invalidate_all();
        }
    }

    /// Discards cached values that satisfy a predicate.
    ///
    /// `invalidate_entries_if` takes a closure that returns `true` or `false`. The
    /// closure is called against each cached entry inserted before or at the time
    /// when this method was called. If the closure returns `true` that entry will be
    /// evicted from the cache.
    ///
    /// This method returns immediately by not actually removing the invalidated
    /// entries. Instead, it just sets the predicate to the cache with the time when
    /// this method was called. The actual removal of the invalidated entries is done
    /// as a maintenance task driven by a user thread. For more details, see
    /// [the Maintenance Tasks section](../index.html#maintenance-tasks) in the crate
    /// level documentation.
    ///
    /// Also the `get` and other retrieval methods will apply the closure to a cached
    /// entry to determine if it should have been invalidated. Therefore, it is
    /// guaranteed that these methods must not return invalidated values.
    ///
    /// Note that you must call
    /// [`CacheBuilder::support_invalidation_closures`][support-invalidation-closures]
    /// at the cache creation time as the cache needs to maintain additional internal
    /// data structures to support this method. Otherwise, calling this method will
    /// fail with a
    /// [`PredicateError::InvalidationClosuresDisabled`][invalidation-disabled-error].
    ///
    /// Like the `invalidate` method, this method does not clear the historic
    /// popularity estimator of keys so that it retains the client activities of
    /// trying to retrieve an item.
    ///
    /// [support-invalidation-closures]:
    ///     ./struct.CacheBuilder.html#method.support_invalidation_closures
    /// [invalidation-disabled-error]:
    ///     ../enum.PredicateError.html#variant.InvalidationClosuresDisabled
    pub fn invalidate_entries_if<F>(&self, predicate: F) -> Result<(), PredicateError>
    where
        F: Fn(&K, &V) -> bool + Send + Sync + 'static,
    {
        let pred = Arc::new(predicate);
        for segment in self.inner.segments.iter() {
            segment.invalidate_entries_with_arc_fun(Arc::clone(&pred))?;
        }
        Ok(())
    }

    /// Creates an iterator visiting all key-value pairs in arbitrary order. The
    /// iterator element type is `(Arc<K>, V)`, where `V` is a clone of a stored
    /// value.
    ///
    /// Iterators do not block concurrent reads and writes on the cache. An entry can
    /// be inserted to, invalidated or evicted from a cache while iterators are alive
    /// on the same cache.
    ///
    /// Unlike the `get` method, visiting entries via an iterator do not update the
    /// historic popularity estimator or reset idle timers for keys.
    ///
    /// # Guarantees
    ///
    /// In order to allow concurrent access to the cache, iterator's `next` method
    /// does _not_ guarantee the following:
    ///
    /// - It does not guarantee to return a key-value pair (an entry) if its key has
    ///   been inserted to the cache _after_ the iterator was created.
    ///   - Such an entry may or may not be returned depending on key's hash and
    ///     timing.
    ///
    /// and the `next` method guarantees the followings:
    ///
    /// - It guarantees not to return the same entry more than once.
    /// - It guarantees not to return an entry if it has been removed from the cache
    ///   after the iterator was created.
    ///     - Note: An entry can be removed by following reasons:
    ///         - Manually invalidated.
    ///         - Expired (e.g. time-to-live).
    ///         - Evicted as the cache capacity exceeded.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use moka::sync::SegmentedCache;
    ///
    /// let cache = SegmentedCache::new(100, 4);
    /// cache.insert("Julia", 14);
    ///
    /// let mut iter = cache.iter();
    /// let (k, v) = iter.next().unwrap(); // (Arc<K>, V)
    /// assert_eq!(*k, "Julia");
    /// assert_eq!(v, 14);
    ///
    /// assert!(iter.next().is_none());
    /// ```
    ///
    pub fn iter(&self) -> Iter<'_, K, V> {
        let num_cht_segments = self.inner.segments[0].num_cht_segments();
        let segments = self
            .inner
            .segments
            .iter()
            .map(|c| c as &dyn ScanningGet<_, _>)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Iter::with_multiple_cache_segments(segments, num_cht_segments)
    }

    /// Performs any pending maintenance operations needed by the cache.
    pub fn run_pending_tasks(&self) {
        for segment in self.inner.segments.iter() {
            segment.run_pending_tasks();
        }
    }
}

impl<'a, K, V, S> IntoIterator for &'a SegmentedCache<K, V, S>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
    S: BuildHasher + Clone + Send + Sync + 'static,
{
    type Item = (Arc<K>, V);

    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// For unit tests.
#[cfg(test)]
impl<K, V, S> SegmentedCache<K, V, S> {
    fn is_waiter_map_empty(&self) -> bool {
        self.inner.segments.iter().all(Cache::is_waiter_map_empty)
    }
}

#[cfg(test)]
impl<K, V, S> SegmentedCache<K, V, S>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
    S: BuildHasher + Clone + Send + Sync + 'static,
{
    fn invalidation_predicate_count(&self) -> usize {
        self.inner
            .segments
            .iter()
            .map(|seg| seg.invalidation_predicate_count())
            .sum()
    }

    fn reconfigure_for_testing(&mut self) {
        let inner = Arc::get_mut(&mut self.inner)
            .expect("There are other strong reference to self.inner Arc");

        for segment in inner.segments.iter_mut() {
            segment.reconfigure_for_testing();
        }
    }

    fn key_locks_map_is_empty(&self) -> bool {
        self.inner
            .segments
            .iter()
            .all(|seg| seg.key_locks_map_is_empty())
    }
}

struct Inner<K, V, S> {
    desired_capacity: Option<u64>,
    segments: Box<[Cache<K, V, S>]>,
    build_hasher: S,
    segment_shift: u32,
}

impl<K, V, S> Inner<K, V, S>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
    S: BuildHasher + Clone + Send + Sync + 'static,
{
    /// # Panics
    ///
    /// Panics if `num_segments` is 0.
    #[allow(clippy::too_many_arguments)]
    fn new(
        name: Option<String>,
        max_capacity: Option<u64>,
        initial_capacity: Option<usize>,
        num_segments: usize,
        build_hasher: S,
        weigher: Option<Weigher<K, V>>,
        eviction_policy: EvictionPolicy,
        eviction_listener: Option<EvictionListener<K, V>>,
        expiration_policy: ExpirationPolicy<K, V>,
        housekeeper_config: HousekeeperConfig,
        invalidator_enabled: bool,
        clock: Clock,
    ) -> Self {
        assert!(num_segments > 0);

        let actual_num_segments = num_segments.next_power_of_two();
        let segment_shift = 64 - actual_num_segments.trailing_zeros();
        let seg_max_capacity =
            max_capacity.map(|n| (n as f64 / actual_num_segments as f64).ceil() as u64);
        let seg_init_capacity =
            initial_capacity.map(|cap| (cap as f64 / actual_num_segments as f64).ceil() as usize);
        // NOTE: We cannot initialize the segments as `vec![cache; actual_num_segments]`
        // because Cache::clone() does not clone its inner but shares the same inner.
        let segments = (0..actual_num_segments)
            .map(|_| {
                Cache::with_everything(
                    name.clone(),
                    seg_max_capacity,
                    seg_init_capacity,
                    build_hasher.clone(),
                    weigher.clone(),
                    eviction_policy.clone(),
                    eviction_listener.clone(),
                    expiration_policy.clone(),
                    housekeeper_config.clone(),
                    invalidator_enabled,
                    clock.clone(),
                )
            })
            .collect::<Vec<_>>();

        Self {
            desired_capacity: max_capacity,
            segments: segments.into_boxed_slice(),
            build_hasher,
            segment_shift,
        }
    }

    #[inline]
    fn hash<Q>(&self, key: &Q) -> u64
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        let mut hasher = self.build_hasher.build_hasher();
        key.hash(&mut hasher);
        hasher.finish()
    }

    #[inline]
    fn select(&self, hash: u64) -> &Cache<K, V, S> {
        let index = self.segment_index_from_hash(hash);
        &self.segments[index]
    }

    #[inline]
    fn segment_index_from_hash(&self, hash: u64) -> usize {
        if self.segment_shift == 64 {
            0
        } else {
            (hash >> self.segment_shift) as usize
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SegmentedCache;
    use crate::notification::RemovalCause;
    use parking_lot::Mutex;
    use std::{error::Error, fmt::Display, sync::Arc, time::Duration};

    #[test]
    fn max_capacity_zero() {
        let mut cache = SegmentedCache::new(0, 1);
        cache.reconfigure_for_testing();

        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert(0, ());

        assert!(!cache.contains_key(&0));
        assert!(cache.get(&0).is_none());
        cache.run_pending_tasks();
        assert!(!cache.contains_key(&0));
        assert!(cache.get(&0).is_none());
        assert_eq!(cache.entry_count(), 0)
    }

    #[test]
    fn basic_single_thread() {
        // The following `Vec`s will hold actual and expected notifications.
        let actual = Arc::new(Mutex::new(Vec::new()));
        let mut expected = Vec::new();

        // Create an eviction listener.
        let a1 = Arc::clone(&actual);
        let listener = move |k, v, cause| a1.lock().push((k, v, cause));

        // Create a cache with the eviction listener.
        let mut cache = SegmentedCache::builder(1)
            .max_capacity(3)
            .eviction_listener(listener)
            .build();
        cache.reconfigure_for_testing();

        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert("a", "alice");
        cache.insert("b", "bob");
        assert_eq!(cache.get(&"a"), Some("alice"));
        assert!(cache.contains_key(&"a"));
        assert!(cache.contains_key(&"b"));
        assert_eq!(cache.get(&"b"), Some("bob"));
        cache.run_pending_tasks();
        // counts: a -> 1, b -> 1

        cache.insert("c", "cindy");
        assert_eq!(cache.get(&"c"), Some("cindy"));
        assert!(cache.contains_key(&"c"));
        // counts: a -> 1, b -> 1, c -> 1
        cache.run_pending_tasks();

        assert!(cache.contains_key(&"a"));
        assert_eq!(cache.get(&"a"), Some("alice"));
        assert_eq!(cache.get(&"b"), Some("bob"));
        assert!(cache.contains_key(&"b"));
        cache.run_pending_tasks();
        // counts: a -> 2, b -> 2, c -> 1

        // "d" should not be admitted because its frequency is too low.
        cache.insert("d", "david"); //   count: d -> 0
        expected.push((Arc::new("d"), "david", RemovalCause::Size));
        cache.run_pending_tasks();
        assert_eq!(cache.get(&"d"), None); //   d -> 1
        assert!(!cache.contains_key(&"d"));

        cache.insert("d", "david");
        expected.push((Arc::new("d"), "david", RemovalCause::Size));
        cache.run_pending_tasks();
        assert!(!cache.contains_key(&"d"));
        assert_eq!(cache.get(&"d"), None); //   d -> 2

        // "d" should be admitted and "c" should be evicted
        // because d's frequency is higher than c's.
        cache.insert("d", "dennis");
        expected.push((Arc::new("c"), "cindy", RemovalCause::Size));
        cache.run_pending_tasks();
        assert_eq!(cache.get(&"a"), Some("alice"));
        assert_eq!(cache.get(&"b"), Some("bob"));
        assert_eq!(cache.get(&"c"), None);
        assert_eq!(cache.get(&"d"), Some("dennis"));
        assert!(cache.contains_key(&"a"));
        assert!(cache.contains_key(&"b"));
        assert!(!cache.contains_key(&"c"));
        assert!(cache.contains_key(&"d"));

        cache.invalidate(&"b");
        expected.push((Arc::new("b"), "bob", RemovalCause::Explicit));
        cache.run_pending_tasks();
        assert_eq!(cache.get(&"b"), None);
        assert!(!cache.contains_key(&"b"));

        assert!(cache.remove(&"b").is_none());
        assert_eq!(cache.remove(&"d"), Some("dennis"));
        expected.push((Arc::new("d"), "dennis", RemovalCause::Explicit));
        cache.run_pending_tasks();
        assert_eq!(cache.get(&"d"), None);
        assert!(!cache.contains_key(&"d"));

        verify_notification_vec(&cache, actual, &expected);
        assert!(cache.key_locks_map_is_empty());
    }

    #[test]
    fn non_power_of_two_segments() {
        let mut cache = SegmentedCache::new(100, 5);
        cache.reconfigure_for_testing();

        // Make the cache exterior immutable.
        let cache = cache;

        assert_eq!(cache.iter().count(), 0);

        cache.insert("a", "alice");
        cache.insert("b", "bob");
        cache.insert("c", "cindy");

        assert_eq!(cache.iter().count(), 3);
        cache.run_pending_tasks();
        assert_eq!(cache.iter().count(), 3);
    }

    #[test]
    fn size_aware_eviction() {
        let weigher = |_k: &&str, v: &(&str, u32)| v.1;

        let alice = ("alice", 10);
        let bob = ("bob", 15);
        let bill = ("bill", 20);
        let cindy = ("cindy", 5);
        let david = ("david", 15);
        let dennis = ("dennis", 15);

        // The following `Vec`s will hold actual and expected notifications.
        let actual = Arc::new(Mutex::new(Vec::new()));
        let mut expected = Vec::new();

        // Create an eviction listener.
        let a1 = Arc::clone(&actual);
        let listener = move |k, v, cause| a1.lock().push((k, v, cause));

        // Create a cache with the eviction listener.
        let mut cache = SegmentedCache::builder(1)
            .max_capacity(31)
            .weigher(weigher)
            .eviction_listener(listener)
            .build();
        cache.reconfigure_for_testing();

        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert("a", alice);
        cache.insert("b", bob);
        assert_eq!(cache.get(&"a"), Some(alice));
        assert!(cache.contains_key(&"a"));
        assert!(cache.contains_key(&"b"));
        assert_eq!(cache.get(&"b"), Some(bob));
        cache.run_pending_tasks();
        // order (LRU -> MRU) and counts: a -> 1, b -> 1

        cache.insert("c", cindy);
        assert_eq!(cache.get(&"c"), Some(cindy));
        assert!(cache.contains_key(&"c"));
        // order and counts: a -> 1, b -> 1, c -> 1
        cache.run_pending_tasks();

        assert!(cache.contains_key(&"a"));
        assert_eq!(cache.get(&"a"), Some(alice));
        assert_eq!(cache.get(&"b"), Some(bob));
        assert!(cache.contains_key(&"b"));
        cache.run_pending_tasks();
        // order and counts: c -> 1, a -> 2, b -> 2

        // To enter "d" (weight: 15), it needs to evict "c" (w: 5) and "a" (w: 10).
        // "d" must have higher count than 3, which is the aggregated count
        // of "a" and "c".
        cache.insert("d", david); //   count: d -> 0
        expected.push((Arc::new("d"), david, RemovalCause::Size));
        cache.run_pending_tasks();
        assert_eq!(cache.get(&"d"), None); //   d -> 1
        assert!(!cache.contains_key(&"d"));

        cache.insert("d", david);
        expected.push((Arc::new("d"), david, RemovalCause::Size));
        cache.run_pending_tasks();
        assert!(!cache.contains_key(&"d"));
        assert_eq!(cache.get(&"d"), None); //   d -> 2

        cache.insert("d", david);
        expected.push((Arc::new("d"), david, RemovalCause::Size));
        cache.run_pending_tasks();
        assert_eq!(cache.get(&"d"), None); //   d -> 3
        assert!(!cache.contains_key(&"d"));

        cache.insert("d", david);
        expected.push((Arc::new("d"), david, RemovalCause::Size));
        cache.run_pending_tasks();
        assert!(!cache.contains_key(&"d"));
        assert_eq!(cache.get(&"d"), None); //   d -> 4

        // Finally "d" should be admitted by evicting "c" and "a".
        cache.insert("d", dennis);
        expected.push((Arc::new("c"), cindy, RemovalCause::Size));
        expected.push((Arc::new("a"), alice, RemovalCause::Size));
        cache.run_pending_tasks();
        assert_eq!(cache.get(&"a"), None);
        assert_eq!(cache.get(&"b"), Some(bob));
        assert_eq!(cache.get(&"c"), None);
        assert_eq!(cache.get(&"d"), Some(dennis));
        assert!(!cache.contains_key(&"a"));
        assert!(cache.contains_key(&"b"));
        assert!(!cache.contains_key(&"c"));
        assert!(cache.contains_key(&"d"));

        // Update "b" with "bill" (w: 15 -> 20). This should evict "d" (w: 15).
        cache.insert("b", bill);
        expected.push((Arc::new("b"), bob, RemovalCause::Replaced));
        expected.push((Arc::new("d"), dennis, RemovalCause::Size));
        cache.run_pending_tasks();
        assert_eq!(cache.get(&"b"), Some(bill));
        assert_eq!(cache.get(&"d"), None);
        assert!(cache.contains_key(&"b"));
        assert!(!cache.contains_key(&"d"));

        // Re-add "a" (w: 10) and update "b" with "bob" (w: 20 -> 15).
        cache.insert("a", alice);
        cache.insert("b", bob);
        expected.push((Arc::new("b"), bill, RemovalCause::Replaced));
        cache.run_pending_tasks();
        assert_eq!(cache.get(&"a"), Some(alice));
        assert_eq!(cache.get(&"b"), Some(bob));
        assert_eq!(cache.get(&"d"), None);
        assert!(cache.contains_key(&"a"));
        assert!(cache.contains_key(&"b"));
        assert!(!cache.contains_key(&"d"));

        // Verify the sizes.
        assert_eq!(cache.entry_count(), 2);
        assert_eq!(cache.weighted_size(), 25);

        verify_notification_vec(&cache, actual, &expected);
        assert!(cache.key_locks_map_is_empty());
    }

    #[test]
    fn basic_multi_threads() {
        let num_threads = 4;

        let mut cache = SegmentedCache::new(100, num_threads);
        cache.reconfigure_for_testing();

        // Make the cache exterior immutable.
        let cache = cache;

        // https://rust-lang.github.io/rust-clippy/master/index.html#needless_collect
        #[allow(clippy::needless_collect)]
        let handles = (0..num_threads)
            .map(|id| {
                let cache = cache.clone();
                std::thread::spawn(move || {
                    cache.insert(10, format!("{id}-100"));
                    cache.get(&10);
                    cache.run_pending_tasks();
                    cache.insert(20, format!("{id}-200"));
                    cache.invalidate(&10);
                })
            })
            .collect::<Vec<_>>();

        handles.into_iter().for_each(|h| h.join().expect("Failed"));

        cache.run_pending_tasks();

        assert!(cache.get(&10).is_none());
        assert!(cache.get(&20).is_some());
        assert!(!cache.contains_key(&10));
        assert!(cache.contains_key(&20));
    }

    #[test]
    fn invalidate_all() {
        use std::collections::HashMap;

        // The following `HashMap`s will hold actual and expected notifications.
        // Note: We use `HashMap` here as the order of invalidations is non-deterministic.
        let actual = Arc::new(Mutex::new(HashMap::new()));
        let mut expected = HashMap::new();

        // Create an eviction listener.
        let a1 = Arc::clone(&actual);
        let listener = move |k, v, cause| {
            a1.lock().insert(k, (v, cause));
        };

        // Create a cache with the eviction listener.
        let mut cache = SegmentedCache::builder(4)
            .max_capacity(100)
            .eviction_listener(listener)
            .build();
        cache.reconfigure_for_testing();

        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert("a", "alice");
        cache.insert("b", "bob");
        cache.insert("c", "cindy");
        assert_eq!(cache.get(&"a"), Some("alice"));
        assert_eq!(cache.get(&"b"), Some("bob"));
        assert_eq!(cache.get(&"c"), Some("cindy"));
        assert!(cache.contains_key(&"a"));
        assert!(cache.contains_key(&"b"));
        assert!(cache.contains_key(&"c"));

        // `cache.run_pending_tasks()` is no longer needed here before invalidating. The last
        // modified timestamp of the entries were updated when they were inserted.
        // https://github.com/moka-rs/moka/issues/155

        cache.invalidate_all();
        expected.insert(Arc::new("a"), ("alice", RemovalCause::Explicit));
        expected.insert(Arc::new("b"), ("bob", RemovalCause::Explicit));
        expected.insert(Arc::new("c"), ("cindy", RemovalCause::Explicit));
        cache.run_pending_tasks();

        cache.insert("d", "david");
        cache.run_pending_tasks();

        assert!(cache.get(&"a").is_none());
        assert!(cache.get(&"b").is_none());
        assert!(cache.get(&"c").is_none());
        assert_eq!(cache.get(&"d"), Some("david"));
        assert!(!cache.contains_key(&"a"));
        assert!(!cache.contains_key(&"b"));
        assert!(!cache.contains_key(&"c"));
        assert!(cache.contains_key(&"d"));

        verify_notification_map(&cache, actual, &expected);
    }

    #[test]
    fn invalidate_entries_if() -> Result<(), Box<dyn std::error::Error>> {
        use std::collections::{HashMap, HashSet};

        const SEGMENTS: usize = 4;

        // The following `HashMap`s will hold actual and expected notifications.
        // Note: We use `HashMap` here as the order of invalidations is non-deterministic.
        let actual = Arc::new(Mutex::new(HashMap::new()));
        let mut expected = HashMap::new();

        // Create an eviction listener.
        let a1 = Arc::clone(&actual);
        let listener = move |k, v, cause| {
            a1.lock().insert(k, (v, cause));
        };

        let (clock, mock) = crate::common::time::Clock::mock();

        // Create a cache with the eviction listener.
        let mut cache = SegmentedCache::builder(SEGMENTS)
            .max_capacity(100)
            .support_invalidation_closures()
            .eviction_listener(listener)
            .clock(clock)
            .build();
        cache.reconfigure_for_testing();

        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert(0, "alice");
        cache.insert(1, "bob");
        cache.insert(2, "alex");
        cache.run_pending_tasks();
        mock.increment(Duration::from_secs(5)); // 5 secs from the start.
        cache.run_pending_tasks();

        assert_eq!(cache.get(&0), Some("alice"));
        assert_eq!(cache.get(&1), Some("bob"));
        assert_eq!(cache.get(&2), Some("alex"));
        assert!(cache.contains_key(&0));
        assert!(cache.contains_key(&1));
        assert!(cache.contains_key(&2));

        let names = ["alice", "alex"].iter().cloned().collect::<HashSet<_>>();
        cache.invalidate_entries_if(move |_k, &v| names.contains(v))?;
        assert_eq!(cache.invalidation_predicate_count(), SEGMENTS);
        expected.insert(Arc::new(0), ("alice", RemovalCause::Explicit));
        expected.insert(Arc::new(2), ("alex", RemovalCause::Explicit));

        mock.increment(Duration::from_secs(5)); // 10 secs from the start.

        cache.insert(3, "alice");

        // Run the invalidation task and wait for it to finish. (TODO: Need a better way than sleeping)
        cache.run_pending_tasks(); // To submit the invalidation task.
        std::thread::sleep(Duration::from_millis(200));
        cache.run_pending_tasks(); // To process the task result.
        std::thread::sleep(Duration::from_millis(200));

        assert!(cache.get(&0).is_none());
        assert!(cache.get(&2).is_none());
        assert_eq!(cache.get(&1), Some("bob"));
        // This should survive as it was inserted after calling invalidate_entries_if.
        assert_eq!(cache.get(&3), Some("alice"));

        assert!(!cache.contains_key(&0));
        assert!(cache.contains_key(&1));
        assert!(!cache.contains_key(&2));
        assert!(cache.contains_key(&3));

        assert_eq!(cache.entry_count(), 2);
        assert_eq!(cache.invalidation_predicate_count(), 0);

        mock.increment(Duration::from_secs(5)); // 15 secs from the start.

        cache.invalidate_entries_if(|_k, &v| v == "alice")?;
        cache.invalidate_entries_if(|_k, &v| v == "bob")?;
        assert_eq!(cache.invalidation_predicate_count(), SEGMENTS * 2);
        expected.insert(Arc::new(1), ("bob", RemovalCause::Explicit));
        expected.insert(Arc::new(3), ("alice", RemovalCause::Explicit));

        // Run the invalidation task and wait for it to finish. (TODO: Need a better way than sleeping)
        cache.run_pending_tasks(); // To submit the invalidation task.
        std::thread::sleep(Duration::from_millis(200));
        cache.run_pending_tasks(); // To process the task result.
        std::thread::sleep(Duration::from_millis(200));

        assert!(cache.get(&1).is_none());
        assert!(cache.get(&3).is_none());

        assert!(!cache.contains_key(&1));
        assert!(!cache.contains_key(&3));

        assert_eq!(cache.entry_count(), 0);
        assert_eq!(cache.invalidation_predicate_count(), 0);

        verify_notification_map(&cache, actual, &expected);

        Ok(())
    }

    #[test]
    fn test_iter() {
        const NUM_KEYS: usize = 50;

        fn make_value(key: usize) -> String {
            format!("val: {key}")
        }

        // let cache = SegmentedCache::builder(5)
        let cache = SegmentedCache::builder(4)
            .max_capacity(100)
            .time_to_idle(Duration::from_secs(10))
            .build();

        for key in 0..NUM_KEYS {
            cache.insert(key, make_value(key));
        }

        let mut key_set = std::collections::HashSet::new();

        for (key, value) in &cache {
            assert_eq!(value, make_value(*key));

            key_set.insert(*key);
        }

        // Ensure there are no missing or duplicate keys in the iteration.
        assert_eq!(key_set.len(), NUM_KEYS);
    }

    /// Runs 16 threads at the same time and ensures no deadlock occurs.
    ///
    /// - Eight of the threads will update key-values in the cache.
    /// - Eight others will iterate the cache.
    ///
    #[test]
    fn test_iter_multi_threads() {
        use std::collections::HashSet;

        const NUM_KEYS: usize = 1024;
        const NUM_THREADS: usize = 16;

        fn make_value(key: usize) -> String {
            format!("val: {key}")
        }

        let cache = SegmentedCache::builder(4)
            .max_capacity(2048)
            .time_to_idle(Duration::from_secs(10))
            .build();

        // Initialize the cache.
        for key in 0..NUM_KEYS {
            cache.insert(key, make_value(key));
        }

        let rw_lock = Arc::new(std::sync::RwLock::<()>::default());
        let write_lock = rw_lock.write().unwrap();

        // https://rust-lang.github.io/rust-clippy/master/index.html#needless_collect
        #[allow(clippy::needless_collect)]
        let handles = (0..NUM_THREADS)
            .map(|n| {
                let cache = cache.clone();
                let rw_lock = Arc::clone(&rw_lock);

                if n % 2 == 0 {
                    // This thread will update the cache.
                    std::thread::spawn(move || {
                        let read_lock = rw_lock.read().unwrap();
                        for key in 0..NUM_KEYS {
                            // TODO: Update keys in a random order?
                            cache.insert(key, make_value(key));
                        }
                        std::mem::drop(read_lock);
                    })
                } else {
                    // This thread will iterate the cache.
                    std::thread::spawn(move || {
                        let read_lock = rw_lock.read().unwrap();
                        let mut key_set = HashSet::new();
                        for (key, value) in &cache {
                            assert_eq!(value, make_value(*key));
                            key_set.insert(*key);
                        }
                        // Ensure there are no missing or duplicate keys in the iteration.
                        assert_eq!(key_set.len(), NUM_KEYS);
                        std::mem::drop(read_lock);
                    })
                }
            })
            .collect::<Vec<_>>();

        // Let these threads to run by releasing the write lock.
        std::mem::drop(write_lock);

        handles.into_iter().for_each(|h| h.join().expect("Failed"));

        // Ensure there are no missing or duplicate keys in the iteration.
        let key_set = cache.iter().map(|(k, _v)| *k).collect::<HashSet<_>>();
        assert_eq!(key_set.len(), NUM_KEYS);
    }

    #[test]
    fn get_with() {
        use std::thread::{sleep, spawn};

        let cache = SegmentedCache::new(100, 4);
        const KEY: u32 = 0;

        // This test will run five threads:
        //
        // Thread1 will be the first thread to call `get_with` for a key, so its init
        // closure will be evaluated and then a &str value "thread1" will be inserted
        // to the cache.
        let thread1 = {
            let cache1 = cache.clone();
            spawn(move || {
                // Call `get_with` immediately.
                let v = cache1.get_with(KEY, || {
                    // Wait for 300 ms and return a &str value.
                    sleep(Duration::from_millis(300));
                    "thread1"
                });
                assert_eq!(v, "thread1");
            })
        };

        // Thread2 will be the second thread to call `get_with` for the same key, so
        // its init closure will not be evaluated. Once thread1's init closure
        // finishes, it will get the value inserted by thread1's init closure.
        let thread2 = {
            let cache2 = cache.clone();
            spawn(move || {
                // Wait for 100 ms before calling `get_with`.
                sleep(Duration::from_millis(100));
                let v = cache2.get_with(KEY, || unreachable!());
                assert_eq!(v, "thread1");
            })
        };

        // Thread3 will be the third thread to call `get_with` for the same key. By
        // the time it calls, thread1's init closure should have finished already and
        // the value should be already inserted to the cache. So its init closure
        // will not be evaluated and will get the value insert by thread1's init
        // closure immediately.
        let thread3 = {
            let cache3 = cache.clone();
            spawn(move || {
                // Wait for 400 ms before calling `get_with`.
                sleep(Duration::from_millis(400));
                let v = cache3.get_with(KEY, || unreachable!());
                assert_eq!(v, "thread1");
            })
        };

        // Thread4 will call `get` for the same key. It will call when thread1's init
        // closure is still running, so it will get none for the key.
        let thread4 = {
            let cache4 = cache.clone();
            spawn(move || {
                // Wait for 200 ms before calling `get`.
                sleep(Duration::from_millis(200));
                let maybe_v = cache4.get(&KEY);
                assert!(maybe_v.is_none());
            })
        };

        // Thread5 will call `get` for the same key. It will call after thread1's init
        // closure finished, so it will get the value insert by thread1's init closure.
        let thread5 = {
            let cache5 = cache.clone();
            spawn(move || {
                // Wait for 400 ms before calling `get`.
                sleep(Duration::from_millis(400));
                let maybe_v = cache5.get(&KEY);
                assert_eq!(maybe_v, Some("thread1"));
            })
        };

        for t in [thread1, thread2, thread3, thread4, thread5] {
            t.join().expect("Failed to join");
        }

        assert!(cache.is_waiter_map_empty());
    }

    #[test]
    fn get_with_if() {
        use std::thread::{sleep, spawn};

        let cache = SegmentedCache::new(100, 4);
        const KEY: u32 = 0;

        // This test will run seven threads:
        //
        // Thread1 will be the first thread to call `get_with_if` for a key, so its
        // init closure will be evaluated and then a &str value "thread1" will be
        // inserted to the cache.
        let thread1 = {
            let cache1 = cache.clone();
            spawn(move || {
                // Call `get_with` immediately.
                let v = cache1.get_with_if(
                    KEY,
                    || {
                        // Wait for 300 ms and return a &str value.
                        sleep(Duration::from_millis(300));
                        "thread1"
                    },
                    |_v| unreachable!(),
                );
                assert_eq!(v, "thread1");
            })
        };

        // Thread2 will be the second thread to call `get_with_if` for the same key,
        // so its init closure will not be evaluated. Once thread1's init closure
        // finishes, it will get the value inserted by thread1's init closure.
        let thread2 = {
            let cache2 = cache.clone();
            spawn(move || {
                // Wait for 100 ms before calling `get_with`.
                sleep(Duration::from_millis(100));
                let v = cache2.get_with_if(KEY, || unreachable!(), |_v| unreachable!());
                assert_eq!(v, "thread1");
            })
        };

        // Thread3 will be the third thread to call `get_with_if` for the same
        // key. By the time it calls, thread1's init closure should have finished
        // already and the value should be already inserted to the cache. Also
        // thread3's `replace_if` closure returns `false`. So its init closure will
        // not be evaluated and will get the value inserted by thread1's init closure
        // immediately.
        let thread3 = {
            let cache3 = cache.clone();
            spawn(move || {
                // Wait for 350 ms before calling `get_with_if`.
                sleep(Duration::from_millis(350));
                let v = cache3.get_with_if(
                    KEY,
                    || unreachable!(),
                    |v| {
                        assert_eq!(v, &"thread1");
                        false
                    },
                );
                assert_eq!(v, "thread1");
            })
        };

        // Thread4 will be the fourth thread to call `get_with_if` for the same
        // key. The value should have been already inserted to the cache by
        // thread1. However thread4's `replace_if` closure returns `true`. So its
        // init closure will be evaluated to replace the current value.
        let thread4 = {
            let cache4 = cache.clone();
            spawn(move || {
                // Wait for 400 ms before calling `get_with_if`.
                sleep(Duration::from_millis(400));
                let v = cache4.get_with_if(
                    KEY,
                    || "thread4",
                    |v| {
                        assert_eq!(v, &"thread1");
                        true
                    },
                );
                assert_eq!(v, "thread4");
            })
        };

        // Thread5 will call `get` for the same key. It will call when thread1's init
        // closure is still running, so it will get none for the key.
        let thread5 = {
            let cache5 = cache.clone();
            spawn(move || {
                // Wait for 200 ms before calling `get`.
                sleep(Duration::from_millis(200));
                let maybe_v = cache5.get(&KEY);
                assert!(maybe_v.is_none());
            })
        };

        // Thread6 will call `get` for the same key. It will call when thread1's init
        // closure is still running, so it will get none for the key.
        let thread6 = {
            let cache6 = cache.clone();
            spawn(move || {
                // Wait for 200 ms before calling `get`.
                sleep(Duration::from_millis(350));
                let maybe_v = cache6.get(&KEY);
                assert_eq!(maybe_v, Some("thread1"));
            })
        };

        // Thread7 will call `get` for the same key. It will call after thread1's init
        // closure finished, so it will get the value insert by thread1's init closure.
        let thread7 = {
            let cache7 = cache.clone();
            spawn(move || {
                // Wait for 400 ms before calling `get`.
                sleep(Duration::from_millis(450));
                let maybe_v = cache7.get(&KEY);
                assert_eq!(maybe_v, Some("thread4"));
            })
        };

        for t in [
            thread1, thread2, thread3, thread4, thread5, thread6, thread7,
        ] {
            t.join().expect("Failed to join");
        }

        assert!(cache.is_waiter_map_empty());
    }

    #[test]
    fn try_get_with() {
        use std::{
            sync::Arc,
            thread::{sleep, spawn},
        };

        #[derive(Debug)]
        pub struct MyError(String);

        impl Display for MyError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl Error for MyError {}

        type MyResult<T> = Result<T, Arc<MyError>>;

        let cache = SegmentedCache::new(100, 4);
        const KEY: u32 = 0;

        // This test will run eight threads:
        //
        // Thread1 will be the first thread to call `try_get_with` for a key, so its
        // init closure will be evaluated and then an error will be returned. Nothing
        // will be inserted to the cache.
        let thread1 = {
            let cache1 = cache.clone();
            spawn(move || {
                // Call `try_get_with` immediately.
                let v = cache1.try_get_with(KEY, || {
                    // Wait for 300 ms and return an error.
                    sleep(Duration::from_millis(300));
                    Err(MyError("thread1 error".into()))
                });
                assert!(v.is_err());
            })
        };

        // Thread2 will be the second thread to call `try_get_with` for the same key,
        // so its init closure will not be evaluated. Once thread1's init closure
        // finishes, it will get the same error value returned by thread1's init
        // closure.
        let thread2 = {
            let cache2 = cache.clone();
            spawn(move || {
                // Wait for 100 ms before calling `try_get_with`.
                sleep(Duration::from_millis(100));
                let v: MyResult<_> = cache2.try_get_with(KEY, || unreachable!());
                assert!(v.is_err());
            })
        };

        // Thread3 will be the third thread to call `get_with` for the same key. By
        // the time it calls, thread1's init closure should have finished already,
        // but the key still does not exist in the cache. So its init closure will be
        // evaluated and then an okay &str value will be returned. That value will be
        // inserted to the cache.
        let thread3 = {
            let cache3 = cache.clone();
            spawn(move || {
                // Wait for 400 ms before calling `try_get_with`.
                sleep(Duration::from_millis(400));
                let v: MyResult<_> = cache3.try_get_with(KEY, || {
                    // Wait for 300 ms and return an Ok(&str) value.
                    sleep(Duration::from_millis(300));
                    Ok("thread3")
                });
                assert_eq!(v.unwrap(), "thread3");
            })
        };

        // thread4 will be the fourth thread to call `try_get_with` for the same
        // key. So its init closure will not be evaluated. Once thread3's init
        // closure finishes, it will get the same okay &str value.
        let thread4 = {
            let cache4 = cache.clone();
            spawn(move || {
                // Wait for 500 ms before calling `try_get_with`.
                sleep(Duration::from_millis(500));
                let v: MyResult<_> = cache4.try_get_with(KEY, || unreachable!());
                assert_eq!(v.unwrap(), "thread3");
            })
        };

        // Thread5 will be the fifth thread to call `try_get_with` for the same
        // key. So its init closure will not be evaluated. By the time it calls,
        // thread3's init closure should have finished already, so its init closure
        // will not be evaluated and will get the value insert by thread3's init
        // closure immediately.
        let thread5 = {
            let cache5 = cache.clone();
            spawn(move || {
                // Wait for 800 ms before calling `try_get_with`.
                sleep(Duration::from_millis(800));
                let v: MyResult<_> = cache5.try_get_with(KEY, || unreachable!());
                assert_eq!(v.unwrap(), "thread3");
            })
        };

        // Thread6 will call `get` for the same key. It will call when thread1's init
        // closure is still running, so it will get none for the key.
        let thread6 = {
            let cache6 = cache.clone();
            spawn(move || {
                // Wait for 200 ms before calling `get`.
                sleep(Duration::from_millis(200));
                let maybe_v = cache6.get(&KEY);
                assert!(maybe_v.is_none());
            })
        };

        // Thread7 will call `get` for the same key. It will call after thread1's init
        // closure finished with an error. So it will get none for the key.
        let thread7 = {
            let cache7 = cache.clone();
            spawn(move || {
                // Wait for 400 ms before calling `get`.
                sleep(Duration::from_millis(400));
                let maybe_v = cache7.get(&KEY);
                assert!(maybe_v.is_none());
            })
        };

        // Thread8 will call `get` for the same key. It will call after thread3's init
        // closure finished, so it will get the value insert by thread3's init closure.
        let thread8 = {
            let cache8 = cache.clone();
            spawn(move || {
                // Wait for 800 ms before calling `get`.
                sleep(Duration::from_millis(800));
                let maybe_v = cache8.get(&KEY);
                assert_eq!(maybe_v, Some("thread3"));
            })
        };

        for t in [
            thread1, thread2, thread3, thread4, thread5, thread6, thread7, thread8,
        ] {
            t.join().expect("Failed to join");
        }

        assert!(cache.is_waiter_map_empty());
    }

    #[test]
    fn optionally_get_with() {
        use std::thread::{sleep, spawn};

        let cache = SegmentedCache::new(100, 4);
        const KEY: u32 = 0;

        // This test will run eight threads:
        //
        // Thread1 will be the first thread to call `optionally_get_with` for a key, so its
        // init closure will be evaluated and then an error will be returned. Nothing
        // will be inserted to the cache.
        let thread1 = {
            let cache1 = cache.clone();
            spawn(move || {
                // Call `optionally_get_with` immediately.
                let v = cache1.optionally_get_with(KEY, || {
                    // Wait for 300 ms and return an error.
                    sleep(Duration::from_millis(300));
                    None
                });
                assert!(v.is_none());
            })
        };

        // Thread2 will be the second thread to call `optionally_get_with` for the same key,
        // so its init closure will not be evaluated. Once thread1's init closure
        // finishes, it will get the same error value returned by thread1's init
        // closure.
        let thread2 = {
            let cache2 = cache.clone();
            spawn(move || {
                // Wait for 100 ms before calling `optionally_get_with`.
                sleep(Duration::from_millis(100));
                let v = cache2.optionally_get_with(KEY, || unreachable!());
                assert!(v.is_none());
            })
        };

        // Thread3 will be the third thread to call `get_with` for the same key. By
        // the time it calls, thread1's init closure should have finished already,
        // but the key still does not exist in the cache. So its init closure will be
        // evaluated and then an okay &str value will be returned. That value will be
        // inserted to the cache.
        let thread3 = {
            let cache3 = cache.clone();
            spawn(move || {
                // Wait for 400 ms before calling `optionally_get_with`.
                sleep(Duration::from_millis(400));
                let v = cache3.optionally_get_with(KEY, || {
                    // Wait for 300 ms and return an Ok(&str) value.
                    sleep(Duration::from_millis(300));
                    Some("thread3")
                });
                assert_eq!(v.unwrap(), "thread3");
            })
        };

        // thread4 will be the fourth thread to call `optionally_get_with` for the same
        // key. So its init closure will not be evaluated. Once thread3's init
        // closure finishes, it will get the same okay &str value.
        let thread4 = {
            let cache4 = cache.clone();
            spawn(move || {
                // Wait for 500 ms before calling `optionally_get_with`.
                sleep(Duration::from_millis(500));
                let v = cache4.optionally_get_with(KEY, || unreachable!());
                assert_eq!(v.unwrap(), "thread3");
            })
        };

        // Thread5 will be the fifth thread to call `optionally_get_with` for the same
        // key. So its init closure will not be evaluated. By the time it calls,
        // thread3's init closure should have finished already, so its init closure
        // will not be evaluated and will get the value insert by thread3's init
        // closure immediately.
        let thread5 = {
            let cache5 = cache.clone();
            spawn(move || {
                // Wait for 800 ms before calling `optionally_get_with`.
                sleep(Duration::from_millis(800));
                let v = cache5.optionally_get_with(KEY, || unreachable!());
                assert_eq!(v.unwrap(), "thread3");
            })
        };

        // Thread6 will call `get` for the same key. It will call when thread1's init
        // closure is still running, so it will get none for the key.
        let thread6 = {
            let cache6 = cache.clone();
            spawn(move || {
                // Wait for 200 ms before calling `get`.
                sleep(Duration::from_millis(200));
                let maybe_v = cache6.get(&KEY);
                assert!(maybe_v.is_none());
            })
        };

        // Thread7 will call `get` for the same key. It will call after thread1's init
        // closure finished with an error. So it will get none for the key.
        let thread7 = {
            let cache7 = cache.clone();
            spawn(move || {
                // Wait for 400 ms before calling `get`.
                sleep(Duration::from_millis(400));
                let maybe_v = cache7.get(&KEY);
                assert!(maybe_v.is_none());
            })
        };

        // Thread8 will call `get` for the same key. It will call after thread3's init
        // closure finished, so it will get the value insert by thread3's init closure.
        let thread8 = {
            let cache8 = cache.clone();
            spawn(move || {
                // Wait for 800 ms before calling `get`.
                sleep(Duration::from_millis(800));
                let maybe_v = cache8.get(&KEY);
                assert_eq!(maybe_v, Some("thread3"));
            })
        };

        for t in [
            thread1, thread2, thread3, thread4, thread5, thread6, thread7, thread8,
        ] {
            t.join().expect("Failed to join");
        }

        assert!(cache.is_waiter_map_empty());
    }

    // This test ensures that the `contains_key`, `get` and `invalidate` can use
    // borrowed form `&[u8]` for key with type `Vec<u8>`.
    // https://github.com/moka-rs/moka/issues/166
    #[test]
    fn borrowed_forms_of_key() {
        let cache: SegmentedCache<Vec<u8>, ()> = SegmentedCache::new(1, 2);

        let key = vec![1_u8];
        cache.insert(key.clone(), ());

        // key as &Vec<u8>
        let key_v: &Vec<u8> = &key;
        assert!(cache.contains_key(key_v));
        assert_eq!(cache.get(key_v), Some(()));
        cache.invalidate(key_v);

        cache.insert(key, ());

        // key as &[u8]
        let key_s: &[u8] = &[1_u8];
        assert!(cache.contains_key(key_s));
        assert_eq!(cache.get(key_s), Some(()));
        cache.invalidate(key_s);
    }

    // Ignored by default. This test becomes unstable when run in parallel with
    // other tests.
    #[test]
    #[ignore]
    fn drop_value_immediately_after_eviction() {
        use crate::common::test_utils::{Counters, Value};

        const NUM_SEGMENTS: usize = 1;
        const MAX_CAPACITY: u32 = 500;
        const KEYS: u32 = ((MAX_CAPACITY as f64) * 1.2) as u32;

        let counters = Arc::new(Counters::default());
        let counters1 = Arc::clone(&counters);

        let listener = move |_k, _v, cause| match cause {
            RemovalCause::Size => counters1.incl_evicted(),
            RemovalCause::Explicit => counters1.incl_invalidated(),
            _ => (),
        };

        let mut cache = SegmentedCache::builder(NUM_SEGMENTS)
            .max_capacity(MAX_CAPACITY as u64)
            .eviction_listener(listener)
            .build();
        cache.reconfigure_for_testing();

        // Make the cache exterior immutable.
        let cache = cache;

        for key in 0..KEYS {
            let value = Arc::new(Value::new(vec![0u8; 1024], &counters));
            cache.insert(key, value);
            counters.incl_inserted();
            cache.run_pending_tasks();
        }

        let eviction_count = KEYS - MAX_CAPACITY;

        cache.run_pending_tasks();
        assert_eq!(counters.inserted(), KEYS, "inserted");
        assert_eq!(counters.value_created(), KEYS, "value_created");
        assert_eq!(counters.evicted(), eviction_count, "evicted");
        assert_eq!(counters.invalidated(), 0, "invalidated");
        assert_eq!(counters.value_dropped(), eviction_count, "value_dropped");

        for key in 0..KEYS {
            cache.invalidate(&key);
            cache.run_pending_tasks();
        }

        cache.run_pending_tasks();
        assert_eq!(counters.inserted(), KEYS, "inserted");
        assert_eq!(counters.value_created(), KEYS, "value_created");
        assert_eq!(counters.evicted(), eviction_count, "evicted");
        assert_eq!(counters.invalidated(), MAX_CAPACITY, "invalidated");
        assert_eq!(counters.value_dropped(), KEYS, "value_dropped");

        std::mem::drop(cache);
        assert_eq!(counters.value_dropped(), KEYS, "value_dropped");
    }

    #[test]
    fn test_debug_format() {
        let cache = SegmentedCache::new(10, 4);
        cache.insert('a', "alice");
        cache.insert('b', "bob");
        cache.insert('c', "cindy");

        let debug_str = format!("{cache:?}");
        assert!(debug_str.starts_with('{'));
        assert!(debug_str.contains(r#"'a': "alice""#));
        assert!(debug_str.contains(r#"'b': "bob""#));
        assert!(debug_str.contains(r#"'c': "cindy""#));
        assert!(debug_str.ends_with('}'));
    }

    type NotificationPair<V> = (V, RemovalCause);
    type NotificationTriple<K, V> = (Arc<K>, V, RemovalCause);

    fn verify_notification_vec<K, V, S>(
        cache: &SegmentedCache<K, V, S>,
        actual: Arc<Mutex<Vec<NotificationTriple<K, V>>>>,
        expected: &[NotificationTriple<K, V>],
    ) where
        K: std::hash::Hash + Eq + std::fmt::Debug + Send + Sync + 'static,
        V: Eq + std::fmt::Debug + Clone + Send + Sync + 'static,
        S: std::hash::BuildHasher + Clone + Send + Sync + 'static,
    {
        // Retries will be needed when testing in a QEMU VM.
        const MAX_RETRIES: usize = 5;
        let mut retries = 0;
        loop {
            // Ensure all scheduled notifications have been processed.
            std::thread::sleep(Duration::from_millis(500));

            let actual = &*actual.lock();
            if actual.len() != expected.len() {
                if retries <= MAX_RETRIES {
                    retries += 1;
                    cache.run_pending_tasks();
                    continue;
                } else {
                    assert_eq!(actual.len(), expected.len(), "Retries exhausted");
                }
            }

            for (i, (actual, expected)) in actual.iter().zip(expected).enumerate() {
                assert_eq!(actual, expected, "expected[{i}]");
            }

            break;
        }
    }

    fn verify_notification_map<K, V, S>(
        cache: &SegmentedCache<K, V, S>,
        actual: Arc<Mutex<std::collections::HashMap<Arc<K>, NotificationPair<V>>>>,
        expected: &std::collections::HashMap<Arc<K>, NotificationPair<V>>,
    ) where
        K: std::hash::Hash + Eq + std::fmt::Display + Send + Sync + 'static,
        V: Eq + std::fmt::Debug + Clone + Send + Sync + 'static,
        S: std::hash::BuildHasher + Clone + Send + Sync + 'static,
    {
        // Retries will be needed when testing in a QEMU VM.
        const MAX_RETRIES: usize = 5;
        let mut retries = 0;
        loop {
            // Ensure all scheduled notifications have been processed.
            std::thread::sleep(Duration::from_millis(500));

            let actual = &*actual.lock();
            if actual.len() != expected.len() {
                if retries <= MAX_RETRIES {
                    retries += 1;
                    cache.run_pending_tasks();
                    continue;
                } else {
                    assert_eq!(actual.len(), expected.len(), "Retries exhausted");
                }
            }

            for actual_key in actual.keys() {
                assert_eq!(
                    actual.get(actual_key),
                    expected.get(actual_key),
                    "expected[{actual_key}]",
                );
            }

            break;
        }
    }
}
