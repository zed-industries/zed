use equivalent::Equivalent;

use crate::{ops::compute, Entry};

use super::Cache;

use std::{
    hash::{BuildHasher, Hash},
    sync::Arc,
};

/// Provides advanced methods to select or insert an entry of the cache.
///
/// Many methods here return an [`Entry`], a snapshot of a single key-value pair in
/// the cache, carrying additional information like `is_fresh`.
///
/// `OwnedKeyEntrySelector` is constructed from the [`entry`][entry-method] method on
/// the cache.
///
/// [`Entry`]: ../struct.Entry.html
/// [entry-method]: ./struct.Cache.html#method.entry
pub struct OwnedKeyEntrySelector<'a, K, V, S> {
    owned_key: K,
    hash: u64,
    cache: &'a Cache<K, V, S>,
}

impl<'a, K, V, S> OwnedKeyEntrySelector<'a, K, V, S>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
    S: BuildHasher + Clone + Send + Sync + 'static,
{
    pub(crate) fn new(owned_key: K, hash: u64, cache: &'a Cache<K, V, S>) -> Self {
        Self {
            owned_key,
            hash,
            cache,
        }
    }

    /// Performs a compute operation on a cached entry by using the given closure
    /// `f`. A compute operation is either put, remove or no-operation (nop).
    ///
    /// The closure `f` should take the current entry of `Option<Entry<K, V>>` for
    /// the key, and return an `ops::compute::Op<V>` enum.
    ///
    /// This method works as the followings:
    ///
    /// 1. Apply the closure `f` to the current cached `Entry`, and get an
    ///    `ops::compute::Op<V>`.
    /// 2. Execute the op on the cache:
    ///    - `Op::Put(V)`: Put the new value `V` to the cache.
    ///    - `Op::Remove`: Remove the current cached entry.
    ///    - `Op::Nop`: Do nothing.
    /// 3. Return an `ops::compute::CompResult<K, V>` as the followings:
    ///
    /// | [`Op<V>`] | [`Entry<K, V>`] already exists? | [`CompResult<K, V>`] | Notes |
    /// |:--------- |:--- |:--------------------------- |:------------------------------- |
    /// | `Put(V)`  | no  | `Inserted(Entry<K, V>)`     | The new entry is returned.      |
    /// | `Put(V)`  | yes | `ReplacedWith(Entry<K, V>)` | The new entry is returned.      |
    /// | `Remove`  | no  | `StillNone(Arc<K>)`         |                                 |
    /// | `Remove`  | yes | `Removed(Entry<K, V>)`      | The removed entry is returned.  |
    /// | `Nop`     | no  | `StillNone(Arc<K>)`         |                                 |
    /// | `Nop`     | yes | `Unchanged(Entry<K, V>)`    | The existing entry is returned. |
    ///
    /// # See Also
    ///
    /// - If you want the `Future` resolve to `Result<Op<V>>` instead of `Op<V>`, and
    ///   modify entry only when resolved to `Ok(V)`, use the
    ///   [`and_try_compute_with`] method.
    /// - If you only want to update or insert, use the [`and_upsert_with`] method.
    ///
    /// [`Entry<K, V>`]: ../struct.Entry.html
    /// [`Op<V>`]: ../ops/compute/enum.Op.html
    /// [`CompResult<K, V>`]: ../ops/compute/enum.CompResult.html
    /// [`and_upsert_with`]: #method.and_upsert_with
    /// [`and_try_compute_with`]: #method.and_try_compute_with
    ///
    /// # Example
    ///
    /// ```rust
    /// use moka::{
    ///     sync::Cache,
    ///     ops::compute::{CompResult, Op},
    /// };
    ///
    /// let cache: Cache<String, u64> = Cache::new(100);
    /// let key = "key1".to_string();
    ///
    /// /// Increment a cached `u64` counter. If the counter is greater than or
    /// /// equal to 2, remove it.
    /// fn inclement_or_remove_counter(
    ///     cache: &Cache<String, u64>,
    ///     key: &str,
    /// ) -> CompResult<String, u64> {
    ///     cache
    ///         .entry(key.to_string())
    ///         .and_compute_with(|maybe_entry| {
    ///             if let Some(entry) = maybe_entry {
    ///                 let counter = entry.into_value();
    ///                 if counter < 2 {
    ///                     Op::Put(counter.saturating_add(1)) // Update
    ///                 } else {
    ///                     Op::Remove
    ///                 }
    ///             } else {
    ///                   Op::Put(1) // Insert
    ///             }
    ///         })
    /// }
    ///
    /// // This should insert a new counter value 1 to the cache, and return the
    /// // value with the kind of the operation performed.
    /// let result = inclement_or_remove_counter(&cache, &key);
    /// let CompResult::Inserted(entry) = result else {
    ///     panic!("`Inserted` should be returned: {result:?}");
    /// };
    /// assert_eq!(entry.into_value(), 1);
    ///
    /// // This should increment the cached counter value by 1.
    /// let result = inclement_or_remove_counter(&cache, &key);
    /// let CompResult::ReplacedWith(entry) = result else {
    ///     panic!("`ReplacedWith` should be returned: {result:?}");
    /// };
    /// assert_eq!(entry.into_value(), 2);
    ///
    /// // This should remove the cached counter from the cache, and returns the
    /// // _removed_ value.
    /// let result = inclement_or_remove_counter(&cache, &key);
    /// let CompResult::Removed(entry) = result else {
    ///     panic!("`Removed` should be returned: {result:?}");
    /// };
    /// assert_eq!(entry.into_value(), 2);
    ///
    /// // The key should no longer exist.
    /// assert!(!cache.contains_key(&key));
    ///
    /// // This should start over; insert a new counter value 1 to the cache.
    /// let result = inclement_or_remove_counter(&cache, &key);
    /// let CompResult::Inserted(entry) = result else {
    ///     panic!("`Inserted` should be returned: {result:?}");
    /// };
    /// assert_eq!(entry.into_value(), 1);
    /// ```
    ///
    /// # Concurrent calls on the same key
    ///
    /// This method guarantees that concurrent calls on the same key are executed
    /// serially. That is, `and_compute_with` calls on the same key never run
    /// concurrently. The calls are serialized by the order of their invocation. It
    /// uses a key-level lock to achieve this.
    pub fn and_compute_with<F>(self, f: F) -> compute::CompResult<K, V>
    where
        F: FnOnce(Option<Entry<K, V>>) -> compute::Op<V>,
    {
        let key = Arc::new(self.owned_key);
        self.cache.compute_with_hash_and_fun(key, self.hash, f)
    }

    /// Performs a compute operation on a cached entry by using the given closure
    /// `f`. A compute operation is either put, remove or no-operation (nop).
    ///
    /// The closure `f` should take the current entry of `Option<Entry<K, V>>` for
    /// the key, and return a `Result<ops::compute::Op<V>, E>`.
    ///
    /// This method works as the followings:
    ///
    /// 1. Apply the closure `f` to the current cached `Entry`, and get a
    ///    `Result<ops::compute::Op<V>, E>`.
    /// 2. If resolved to `Err(E)`, return it.
    /// 3. Else, execute the op on the cache:
    ///    - `Ok(Op::Put(V))`: Put the new value `V` to the cache.
    ///    - `Ok(Op::Remove)`: Remove the current cached entry.
    ///    - `Ok(Op::Nop)`: Do nothing.
    /// 4. Return an `Ok(ops::compute::CompResult<K, V>)` as the followings:
    ///
    /// | [`Op<V>`] | [`Entry<K, V>`] already exists? | [`CompResult<K, V>`] | Notes |
    /// |:--------- |:--- |:--------------------------- |:------------------------------- |
    /// | `Put(V)`  | no  | `Inserted(Entry<K, V>)`     | The new entry is returned.      |
    /// | `Put(V)`  | yes | `ReplacedWith(Entry<K, V>)` | The new entry is returned.      |
    /// | `Remove`  | no  | `StillNone(Arc<K>)`         |                                 |
    /// | `Remove`  | yes | `Removed(Entry<K, V>)`      | The removed entry is returned.  |
    /// | `Nop`     | no  | `StillNone(Arc<K>)`         |                                 |
    /// | `Nop`     | yes | `Unchanged(Entry<K, V>)`    | The existing entry is returned. |
    ///
    /// # See Also
    ///
    /// - If you want the `Future` resolve to `Op<V>` instead of `Result<Op<V>>`, use
    ///   the [`and_compute_with`] method.
    /// - If you only want to put, use the [`and_upsert_with`] method.
    ///
    /// [`Entry<K, V>`]: ../struct.Entry.html
    /// [`Op<V>`]: ../ops/compute/enum.Op.html
    /// [`CompResult<K, V>`]: ../ops/compute/enum.CompResult.html
    /// [`and_upsert_with`]: #method.and_upsert_with
    /// [`and_compute_with`]: #method.and_compute_with
    ///
    /// # Example
    ///
    /// See [`try_append_value_async.rs`] in the `examples` directory.
    ///
    /// [`try_append_value_sync.rs`]:
    ///     https://github.com/moka-rs/moka/tree/main/examples/try_append_value_sync.rs
    ///
    /// # Concurrent calls on the same key
    ///
    /// This method guarantees that concurrent calls on the same key are executed
    /// serially. That is, `and_try_compute_with` calls on the same key never run
    /// concurrently. The calls are serialized by the order of their invocation. It
    /// uses a key-level lock to achieve this.
    pub fn and_try_compute_with<F, E>(self, f: F) -> Result<compute::CompResult<K, V>, E>
    where
        F: FnOnce(Option<Entry<K, V>>) -> Result<compute::Op<V>, E>,
        E: Send + Sync + 'static,
    {
        let key = Arc::new(self.owned_key);
        self.cache.try_compute_with_hash_and_fun(key, self.hash, f)
    }

    /// Performs an upsert of an [`Entry`] by using the given closure `f`. The word
    /// "upsert" here means "update" or "insert".
    ///
    /// The closure `f` should take the current entry of `Option<Entry<K, V>>` for
    /// the key, and return a new value `V`.
    ///
    /// This method works as the followings:
    ///
    /// 1. Apply the closure `f` to the current cached `Entry`, and get a new value
    ///    `V`.
    /// 2. Upsert the new value to the cache.
    /// 3. Return the `Entry` having the upserted value.
    ///
    /// # See Also
    ///
    /// - If you want to optionally upsert, that is to upsert only when certain
    ///   conditions meet, use the [`and_compute_with`] method.
    /// - If you try to upsert, that is to make the `Future` resolve to `Result<V>`
    ///   instead of `V`, and upsert only when resolved to `Ok(V)`, use the
    ///   [`and_try_compute_with`] method.
    ///
    /// [`Entry`]: ../struct.Entry.html
    /// [`and_compute_with`]: #method.and_compute_with
    /// [`and_try_compute_with`]: #method.and_try_compute_with
    ///
    /// # Example
    ///
    /// ```rust
    /// use moka::sync::Cache;
    ///
    /// let cache: Cache<String, u64> = Cache::new(100);
    /// let key = "key1".to_string();
    ///
    /// let entry = cache
    ///     .entry(key.clone())
    ///     .and_upsert_with(|maybe_entry| {
    ///         if let Some(entry) = maybe_entry {
    ///             entry.into_value().saturating_add(1) // Update
    ///         } else {
    ///             1 // Insert
    ///         }
    ///     });
    /// // It was not an update.
    /// assert!(!entry.is_old_value_replaced());
    /// assert_eq!(entry.key(), &key);
    /// assert_eq!(entry.into_value(), 1);
    ///
    /// let entry = cache
    ///     .entry(key.clone())
    ///     .and_upsert_with(|maybe_entry| {
    ///         if let Some(entry) = maybe_entry {
    ///             entry.into_value().saturating_add(1)
    ///         } else {
    ///             1
    ///         }
    ///     });
    /// // It was an update.
    /// assert!(entry.is_old_value_replaced());
    /// assert_eq!(entry.key(), &key);
    /// assert_eq!(entry.into_value(), 2);
    /// ```
    ///
    /// # Concurrent calls on the same key
    ///
    /// This method guarantees that concurrent calls on the same key are executed
    /// serially. That is, `and_upsert_with` calls on the same key never run
    /// concurrently. The calls are serialized by the order of their invocation. It
    /// uses a key-level lock to achieve this.
    pub fn and_upsert_with<F>(self, f: F) -> Entry<K, V>
    where
        F: FnOnce(Option<Entry<K, V>>) -> V,
    {
        let key = Arc::new(self.owned_key);
        self.cache.upsert_with_hash_and_fun(key, self.hash, f)
    }

    /// Returns the corresponding [`Entry`] for the key given when this entry
    /// selector was constructed. If the entry does not exist, inserts one by calling
    /// the [`default`][std-default-function] function of the value type `V`.
    ///
    /// [`Entry`]: ../struct.Entry.html
    /// [std-default-function]: https://doc.rust-lang.org/stable/std/default/trait.Default.html#tymethod.default
    ///
    /// # Example
    ///
    /// ```rust
    /// use moka::sync::Cache;
    ///
    /// let cache: Cache<String, Option<u32>> = Cache::new(100);
    /// let key = "key1".to_string();
    ///
    /// let entry = cache.entry(key.clone()).or_default();
    /// assert!(entry.is_fresh());
    /// assert_eq!(entry.key(), &key);
    /// assert_eq!(entry.into_value(), None);
    ///
    /// let entry = cache.entry(key).or_default();
    /// // Not fresh because the value was already in the cache.
    /// assert!(!entry.is_fresh());
    /// ```
    pub fn or_default(self) -> Entry<K, V>
    where
        V: Default,
    {
        let key = Arc::new(self.owned_key);
        self.cache
            .get_or_insert_with_hash(key, self.hash, Default::default)
    }

    /// Returns the corresponding [`Entry`] for the key given when this entry
    /// selector was constructed. If the entry does not exist, inserts one by using
    /// the the given `default` value for `V`.
    ///
    /// [`Entry`]: ../struct.Entry.html
    ///
    /// # Example
    ///
    /// ```rust
    /// use moka::sync::Cache;
    ///
    /// let cache: Cache<String, u32> = Cache::new(100);
    /// let key = "key1".to_string();
    ///
    /// let entry = cache.entry(key.clone()).or_insert(3);
    /// assert!(entry.is_fresh());
    /// assert_eq!(entry.key(), &key);
    /// assert_eq!(entry.into_value(), 3);
    ///
    /// let entry = cache.entry(key).or_insert(6);
    /// // Not fresh because the value was already in the cache.
    /// assert!(!entry.is_fresh());
    /// assert_eq!(entry.into_value(), 3);
    /// ```
    pub fn or_insert(self, default: V) -> Entry<K, V> {
        let key = Arc::new(self.owned_key);
        let init = || default;
        self.cache.get_or_insert_with_hash(key, self.hash, init)
    }

    /// Returns the corresponding [`Entry`] for the key given when this entry
    /// selector was constructed. If the entry does not exist, evaluates the `init`
    /// closure and inserts the output.
    ///
    /// [`Entry`]: ../struct.Entry.html
    ///
    /// # Example
    ///
    /// ```rust
    /// use moka::sync::Cache;
    ///
    /// let cache: Cache<String, String> = Cache::new(100);
    /// let key = "key1".to_string();
    ///
    /// let entry = cache
    ///     .entry(key.clone())
    ///     .or_insert_with(|| "value1".to_string());
    /// assert!(entry.is_fresh());
    /// assert_eq!(entry.key(), &key);
    /// assert_eq!(entry.into_value(), "value1");
    ///
    /// let entry = cache
    ///     .entry(key)
    ///     .or_insert_with(|| "value2".to_string());
    /// // Not fresh because the value was already in the cache.
    /// assert!(!entry.is_fresh());
    /// assert_eq!(entry.into_value(), "value1");
    /// ```
    ///
    /// # Concurrent calls on the same key
    ///
    /// This method guarantees that concurrent calls on the same not-existing entry
    /// are coalesced into one evaluation of the `init` closure. Only one of the
    /// calls evaluates its closure (thus returned entry's `is_fresh` method returns
    /// `true`), and other calls wait for that closure to complete (and their
    /// `is_fresh` return `false`).
    ///
    /// For more detail about the coalescing behavior, see
    /// [`Cache::get_with`][get-with-method].
    ///
    /// [get-with-method]: ./struct.Cache.html#method.get_with
    pub fn or_insert_with(self, init: impl FnOnce() -> V) -> Entry<K, V> {
        let key = Arc::new(self.owned_key);
        let replace_if = None as Option<fn(&V) -> bool>;
        self.cache
            .get_or_insert_with_hash_and_fun(key, self.hash, init, replace_if, true)
    }

    /// Works like [`or_insert_with`](#method.or_insert_with), but takes an additional
    /// `replace_if` closure.
    ///
    /// This method will evaluate the `init` closure and insert the output to the
    /// cache when:
    ///
    /// - The key does not exist.
    /// - Or, `replace_if` closure returns `true`.
    pub fn or_insert_with_if(
        self,
        init: impl FnOnce() -> V,
        replace_if: impl FnMut(&V) -> bool,
    ) -> Entry<K, V> {
        let key = Arc::new(self.owned_key);
        self.cache
            .get_or_insert_with_hash_and_fun(key, self.hash, init, Some(replace_if), true)
    }

    /// Returns the corresponding [`Entry`] for the key given when this entry
    /// selector was constructed. If the entry does not exist, evaluates the `init`
    /// closure, and inserts an entry if `Some(value)` was returned. If `None` was
    /// returned from the closure, this method does not insert an entry and returns
    /// `None`.
    ///
    /// [`Entry`]: ../struct.Entry.html
    ///
    /// # Example
    ///
    /// ```rust
    /// use moka::sync::Cache;
    ///
    /// let cache: Cache<String, u32> = Cache::new(100);
    /// let key = "key1".to_string();
    ///
    /// let none_entry = cache
    ///     .entry(key.clone())
    ///     .or_optionally_insert_with(|| None);
    /// assert!(none_entry.is_none());
    ///
    /// let some_entry = cache
    ///     .entry(key.clone())
    ///     .or_optionally_insert_with(|| Some(3));
    /// assert!(some_entry.is_some());
    /// let entry = some_entry.unwrap();
    /// assert!(entry.is_fresh());
    /// assert_eq!(entry.key(), &key);
    /// assert_eq!(entry.into_value(), 3);
    ///
    /// let some_entry = cache
    ///     .entry(key)
    ///     .or_optionally_insert_with(|| Some(6));
    /// let entry = some_entry.unwrap();
    /// // Not fresh because the value was already in the cache.
    /// assert!(!entry.is_fresh());
    /// assert_eq!(entry.into_value(), 3);
    /// ```
    ///
    /// # Concurrent calls on the same key
    ///
    /// This method guarantees that concurrent calls on the same not-existing entry
    /// are coalesced into one evaluation of the `init` closure. Only one of the
    /// calls evaluates its closure (thus returned entry's `is_fresh` method returns
    /// `true`), and other calls wait for that closure to complete (and their
    /// `is_fresh` return `false`).
    ///
    /// For more detail about the coalescing behavior, see
    /// [`Cache::optionally_get_with`][opt-get-with-method].
    ///
    /// [opt-get-with-method]: ./struct.Cache.html#method.optionally_get_with
    pub fn or_optionally_insert_with(
        self,
        init: impl FnOnce() -> Option<V>,
    ) -> Option<Entry<K, V>> {
        let key = Arc::new(self.owned_key);
        self.cache
            .get_or_optionally_insert_with_hash_and_fun(key, self.hash, init, true)
    }

    /// Returns the corresponding [`Entry`] for the key given when this entry
    /// selector was constructed. If the entry does not exist, evaluates the `init`
    /// closure, and inserts an entry if `Ok(value)` was returned. If `Err(_)` was
    /// returned from the closure, this method does not insert an entry and returns
    /// the `Err` wrapped by [`std::sync::Arc`][std-arc].
    ///
    /// [`Entry`]: ../struct.Entry.html
    /// [std-arc]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html
    ///
    /// # Example
    ///
    /// ```rust
    /// use moka::sync::Cache;
    ///
    /// let cache: Cache<String, u32> = Cache::new(100);
    /// let key = "key1".to_string();
    ///
    /// let error_entry = cache
    ///     .entry(key.clone())
    ///     .or_try_insert_with(|| Err("error"));
    /// assert!(error_entry.is_err());
    ///
    /// let ok_entry = cache
    ///     .entry(key.clone())
    ///     .or_try_insert_with(|| Ok::<u32, &str>(3));
    /// assert!(ok_entry.is_ok());
    /// let entry = ok_entry.unwrap();
    /// assert!(entry.is_fresh());
    /// assert_eq!(entry.key(), &key);
    /// assert_eq!(entry.into_value(), 3);
    ///
    /// let ok_entry = cache
    ///     .entry(key)
    ///     .or_try_insert_with(|| Ok::<u32, &str>(6));
    /// let entry = ok_entry.unwrap();
    /// // Not fresh because the value was already in the cache.
    /// assert!(!entry.is_fresh());
    /// assert_eq!(entry.into_value(), 3);
    /// ```
    ///
    /// # Concurrent calls on the same key
    ///
    /// This method guarantees that concurrent calls on the same not-existing entry
    /// are coalesced into one evaluation of the `init` closure (as long as these
    /// closures return the same error type). Only one of the calls evaluates its
    /// closure (thus returned entry's `is_fresh` method returns `true`), and other
    /// calls wait for that closure to complete (and their `is_fresh` return
    /// `false`).
    ///
    /// For more detail about the coalescing behavior, see
    /// [`Cache::try_get_with`][try-get-with-method].
    ///
    /// [try-get-with-method]: ./struct.Cache.html#method.try_get_with
    pub fn or_try_insert_with<F, E>(self, init: F) -> Result<Entry<K, V>, Arc<E>>
    where
        F: FnOnce() -> Result<V, E>,
        E: Send + Sync + 'static,
    {
        let key = Arc::new(self.owned_key);
        self.cache
            .get_or_try_insert_with_hash_and_fun(key, self.hash, init, true)
    }
}

/// Provides advanced methods to select or insert an entry of the cache.
///
/// Many methods here return an [`Entry`], a snapshot of a single key-value pair in
/// the cache, carrying additional information like `is_fresh`.
///
/// `RefKeyEntrySelector` is constructed from the
/// [`entry_by_ref`][entry-by-ref-method] method on the cache.
///
/// [`Entry`]: ../struct.Entry.html
/// [entry-by-ref-method]: ./struct.Cache.html#method.entry_by_ref
pub struct RefKeyEntrySelector<'a, K, Q, V, S>
where
    Q: ?Sized,
{
    ref_key: &'a Q,
    hash: u64,
    cache: &'a Cache<K, V, S>,
}

impl<'a, K, Q, V, S> RefKeyEntrySelector<'a, K, Q, V, S>
where
    K: Hash + Eq + Send + Sync + 'static,
    Q: Equivalent<K> + ToOwned<Owned = K> + Hash + ?Sized,
    V: Clone + Send + Sync + 'static,
    S: BuildHasher + Clone + Send + Sync + 'static,
{
    pub(crate) fn new(ref_key: &'a Q, hash: u64, cache: &'a Cache<K, V, S>) -> Self {
        Self {
            ref_key,
            hash,
            cache,
        }
    }

    /// Performs a compute operation on a cached entry by using the given closure
    /// `f`. A compute operation is either put, remove or no-operation (nop).
    ///
    /// The closure `f` should take the current entry of `Option<Entry<K, V>>` for
    /// the key, and return an `ops::compute::Op<V>` enum.
    ///
    /// This method works as the followings:
    ///
    /// 1. Apply the closure `f` to the current cached `Entry`, and get an
    ///    `ops::compute::Op<V>`.
    /// 2. Execute the op on the cache:
    ///    - `Op::Put(V)`: Put the new value `V` to the cache.
    ///    - `Op::Remove`: Remove the current cached entry.
    ///    - `Op::Nop`: Do nothing.
    /// 3. Return an `ops::compute::CompResult<K, V>` as the followings:
    ///
    /// | [`Op<V>`] | [`Entry<K, V>`] already exists? | [`CompResult<K, V>`] | Notes |
    /// |:--------- |:--- |:--------------------------- |:------------------------------- |
    /// | `Put(V)`  | no  | `Inserted(Entry<K, V>)`     | The new entry is returned.      |
    /// | `Put(V)`  | yes | `ReplacedWith(Entry<K, V>)` | The new entry is returned.      |
    /// | `Remove`  | no  | `StillNone(Arc<K>)`         |                                 |
    /// | `Remove`  | yes | `Removed(Entry<K, V>)`      | The removed entry is returned.  |
    /// | `Nop`     | no  | `StillNone(Arc<K>)`         |                                 |
    /// | `Nop`     | yes | `Unchanged(Entry<K, V>)`    | The existing entry is returned. |
    ///
    /// # See Also
    ///
    /// - If you want the `Future` resolve to `Result<Op<V>>` instead of `Op<V>`, and
    ///   modify entry only when resolved to `Ok(V)`, use the
    ///   [`and_try_compute_with`] method.
    /// - If you only want to update or insert, use the [`and_upsert_with`] method.
    ///
    /// [`Entry<K, V>`]: ../struct.Entry.html
    /// [`Op<V>`]: ../ops/compute/enum.Op.html
    /// [`CompResult<K, V>`]: ../ops/compute/enum.CompResult.html
    /// [`and_upsert_with`]: #method.and_upsert_with
    /// [`and_try_compute_with`]: #method.and_try_compute_with
    ///
    /// # Example
    ///
    /// ```rust
    /// use moka::{
    ///     sync::Cache,
    ///     ops::compute::{CompResult, Op},
    /// };
    ///
    /// let cache: Cache<String, u64> = Cache::new(100);
    /// let key = "key1".to_string();
    ///
    /// /// Increment a cached `u64` counter. If the counter is greater than or
    /// /// equal to 2, remove it.
    /// fn inclement_or_remove_counter(
    ///     cache: &Cache<String, u64>,
    ///     key: &str,
    /// ) -> CompResult<String, u64> {
    ///     cache
    ///         .entry_by_ref(key)
    ///         .and_compute_with(|maybe_entry| {
    ///             if let Some(entry) = maybe_entry {
    ///                 let counter = entry.into_value();
    ///                 if counter < 2 {
    ///                     Op::Put(counter.saturating_add(1)) // Update
    ///                 } else {
    ///                     Op::Remove
    ///                 }
    ///             } else {
    ///                   Op::Put(1) // Insert
    ///             }
    ///         })
    /// }
    ///
    /// // This should insert a now counter value 1 to the cache, and return the
    /// // value with the kind of the operation performed.
    /// let result = inclement_or_remove_counter(&cache, &key);
    /// let CompResult::Inserted(entry) = result else {
    ///     panic!("`Inserted` should be returned: {result:?}");
    /// };
    /// assert_eq!(entry.into_value(), 1);
    ///
    /// // This should increment the cached counter value by 1.
    /// let result = inclement_or_remove_counter(&cache, &key);
    /// let CompResult::ReplacedWith(entry) = result else {
    ///     panic!("`ReplacedWith` should be returned: {result:?}");
    /// };
    /// assert_eq!(entry.into_value(), 2);
    ///
    /// // This should remove the cached counter from the cache, and returns the
    /// // _removed_ value.
    /// let result = inclement_or_remove_counter(&cache, &key);
    /// let CompResult::Removed(entry) = result else {
    ///     panic!("`Removed` should be returned: {result:?}");
    /// };
    /// assert_eq!(entry.into_value(), 2);
    ///
    /// // The key should no longer exist.
    /// assert!(!cache.contains_key(&key));
    ///
    /// // This should start over; insert a new counter value 1 to the cache.
    /// let result = inclement_or_remove_counter(&cache, &key);
    /// let CompResult::Inserted(entry) = result else {
    ///     panic!("`Inserted` should be returned: {result:?}");
    /// };
    /// assert_eq!(entry.into_value(), 1);
    /// ```
    ///
    /// # Concurrent calls on the same key
    ///
    /// This method guarantees that concurrent calls on the same key are executed
    /// serially. That is, `and_compute_with` calls on the same key never run
    /// concurrently. The calls are serialized by the order of their invocation. It
    /// uses a key-level lock to achieve this.
    pub fn and_compute_with<F>(self, f: F) -> compute::CompResult<K, V>
    where
        F: FnOnce(Option<Entry<K, V>>) -> compute::Op<V>,
    {
        let key = Arc::new(self.ref_key.to_owned());
        self.cache.compute_with_hash_and_fun(key, self.hash, f)
    }

    /// Performs a compute operation on a cached entry by using the given closure
    /// `f`. A compute operation is either put, remove or no-operation (nop).
    ///
    /// The closure `f` should take the current entry of `Option<Entry<K, V>>` for
    /// the key, and return a `Result<ops::compute::Op<V>, E>`.
    ///
    /// This method works as the followings:
    ///
    /// 1. Apply the closure `f` to the current cached `Entry`, and get a
    ///    `Result<ops::compute::Op<V>, E>`.
    /// 2. If resolved to `Err(E)`, return it.
    /// 3. Else, execute the op on the cache:
    ///    - `Ok(Op::Put(V))`: Put the new value `V` to the cache.
    ///    - `Ok(Op::Remove)`: Remove the current cached entry.
    ///    - `Ok(Op::Nop)`: Do nothing.
    /// 4. Return an `Ok(ops::compute::CompResult<K, V>)` as the followings:
    ///
    /// | [`Op<V>`] | [`Entry<K, V>`] already exists? | [`CompResult<K, V>`] | Notes |
    /// |:--------- |:--- |:--------------------------- |:------------------------------- |
    /// | `Put(V)`  | no  | `Inserted(Entry<K, V>)`     | The new entry is returned.      |
    /// | `Put(V)`  | yes | `ReplacedWith(Entry<K, V>)` | The new entry is returned.      |
    /// | `Remove`  | no  | `StillNone(Arc<K>)`         |                                 |
    /// | `Remove`  | yes | `Removed(Entry<K, V>)`      | The removed entry is returned.  |
    /// | `Nop`     | no  | `StillNone(Arc<K>)`         |                                 |
    /// | `Nop`     | yes | `Unchanged(Entry<K, V>)`    | The existing entry is returned. |
    ///
    /// # Similar Methods
    ///
    /// - If you want the `Future` resolve to `Op<V>` instead of `Result<Op<V>>`, use
    ///   the [`and_compute_with`] method.
    /// - If you only want to update or insert, use the [`and_upsert_with`] method.
    ///
    /// [`Entry<K, V>`]: ../struct.Entry.html
    /// [`Op<V>`]: ../ops/compute/enum.Op.html
    /// [`CompResult<K, V>`]: ../ops/compute/enum.CompResult.html
    /// [`and_upsert_with`]: #method.and_upsert_with
    /// [`and_compute_with`]: #method.and_compute_with
    ///
    /// # Example
    ///
    /// See [`try_append_value_async.rs`] in the `examples` directory.
    ///
    /// [`try_append_value_sync.rs`]:
    ///     https://github.com/moka-rs/moka/tree/main/examples/try_append_value_sync.rs
    ///
    /// # Concurrent calls on the same key
    ///
    /// This method guarantees that concurrent calls on the same key are executed
    /// serially. That is, `and_try_compute_with` calls on the same key never run
    /// concurrently. The calls are serialized by the order of their invocation. It
    /// uses a key-level lock to achieve this.
    pub fn and_try_compute_with<F, E>(self, f: F) -> Result<compute::CompResult<K, V>, E>
    where
        F: FnOnce(Option<Entry<K, V>>) -> Result<compute::Op<V>, E>,
        E: Send + Sync + 'static,
    {
        let key = Arc::new(self.ref_key.to_owned());
        self.cache.try_compute_with_hash_and_fun(key, self.hash, f)
    }

    /// Performs an upsert of an [`Entry`] by using the given closure `f`. The word
    /// "upsert" here means "update" or "insert".
    ///
    /// The closure `f` should take the current entry of `Option<Entry<K, V>>` for
    /// the key, and return a new value `V`.
    ///
    /// This method works as the followings:
    ///
    /// 1. Apply the closure `f` to the current cached `Entry`, and get a new value
    ///    `V`.
    /// 2. Upsert the new value to the cache.
    /// 3. Return the `Entry` having the upserted value.
    ///
    /// # Similar Methods
    ///
    /// - If you want to optionally upsert, that is to upsert only when certain
    ///   conditions meet, use the [`and_compute_with`] method.
    /// - If you try to upsert, that is to make the `Future` resolve to `Result<V>`
    ///   instead of `V`, and upsert only when resolved to `Ok(V)`, use the
    ///   [`and_try_compute_with`] method.
    ///
    /// [`Entry`]: ../struct.Entry.html
    /// [`and_compute_with`]: #method.and_compute_with
    /// [`and_try_compute_with`]: #method.and_try_compute_with
    ///
    /// # Example
    ///
    /// ```rust
    /// use moka::sync::Cache;
    ///
    /// let cache: Cache<String, u64> = Cache::new(100);
    /// let key = "key1".to_string();
    ///
    /// let entry = cache
    ///     .entry_by_ref(&key)
    ///     .and_upsert_with(|maybe_entry| {
    ///         if let Some(entry) = maybe_entry {
    ///             entry.into_value().saturating_add(1) // Update
    ///         } else {
    ///             1 // Insert
    ///         }
    ///     });
    /// // It was not an update.
    /// assert!(!entry.is_old_value_replaced());
    /// assert_eq!(entry.key(), &key);
    /// assert_eq!(entry.into_value(), 1);
    ///
    /// let entry = cache
    ///     .entry_by_ref(&key)
    ///     .and_upsert_with(|maybe_entry| {
    ///         if let Some(entry) = maybe_entry {
    ///             entry.into_value().saturating_add(1)
    ///         } else {
    ///             1
    ///         }
    ///     });
    /// // It was an update.
    /// assert!(entry.is_old_value_replaced());
    /// assert_eq!(entry.key(), &key);
    /// assert_eq!(entry.into_value(), 2);
    /// ```
    ///
    /// # Concurrent calls on the same key
    ///
    /// This method guarantees that concurrent calls on the same key are executed
    /// serially. That is, `and_upsert_with` calls on the same key never run
    /// concurrently. The calls are serialized by the order of their invocation. It
    /// uses a key-level lock to achieve this.
    pub fn and_upsert_with<F>(self, f: F) -> Entry<K, V>
    where
        F: FnOnce(Option<Entry<K, V>>) -> V,
    {
        let key = Arc::new(self.ref_key.to_owned());
        self.cache.upsert_with_hash_and_fun(key, self.hash, f)
    }

    /// Returns the corresponding [`Entry`] for the reference of the key given when
    /// this entry selector was constructed. If the entry does not exist, inserts one
    /// by cloning the key and calling the [`default`][std-default-function] function
    /// of the value type `V`.
    ///
    /// [`Entry`]: ../struct.Entry.html
    /// [std-default-function]: https://doc.rust-lang.org/stable/std/default/trait.Default.html#tymethod.default
    ///
    /// # Example
    ///
    /// ```rust
    /// use moka::sync::Cache;
    ///
    /// let cache: Cache<String, Option<u32>> = Cache::new(100);
    /// let key = "key1".to_string();
    ///
    /// let entry = cache.entry_by_ref(&key).or_default();
    /// assert!(entry.is_fresh());
    /// assert_eq!(entry.key(), &key);
    /// assert_eq!(entry.into_value(), None);
    ///
    /// let entry = cache.entry_by_ref(&key).or_default();
    /// // Not fresh because the value was already in the cache.
    /// assert!(!entry.is_fresh());
    /// ```
    pub fn or_default(self) -> Entry<K, V>
    where
        V: Default,
    {
        self.cache
            .get_or_insert_with_hash_by_ref(self.ref_key, self.hash, Default::default)
    }

    /// Returns the corresponding [`Entry`] for the reference of the key given when
    /// this entry selector was constructed. If the entry does not exist, inserts one
    /// by cloning the key and using the given `default` value for `V`.
    ///
    /// [`Entry`]: ../struct.Entry.html
    ///
    /// # Example
    ///
    /// ```rust
    /// use moka::sync::Cache;
    ///
    /// let cache: Cache<String, u32> = Cache::new(100);
    /// let key = "key1".to_string();
    ///
    /// let entry = cache.entry_by_ref(&key).or_insert(3);
    /// assert!(entry.is_fresh());
    /// assert_eq!(entry.key(), &key);
    /// assert_eq!(entry.into_value(), 3);
    ///
    /// let entry = cache.entry_by_ref(&key).or_insert(6);
    /// // Not fresh because the value was already in the cache.
    /// assert!(!entry.is_fresh());
    /// assert_eq!(entry.into_value(), 3);
    /// ```
    pub fn or_insert(self, default: V) -> Entry<K, V> {
        let init = || default;
        self.cache
            .get_or_insert_with_hash_by_ref(self.ref_key, self.hash, init)
    }

    /// Returns the corresponding [`Entry`] for the reference of the key given when
    /// this entry selector was constructed. If the entry does not exist, inserts one
    /// by cloning the key and evaluating the `init` closure for the value.
    ///
    /// [`Entry`]: ../struct.Entry.html
    ///
    /// # Example
    ///
    /// ```rust
    /// use moka::sync::Cache;
    ///
    /// let cache: Cache<String, String> = Cache::new(100);
    /// let key = "key1".to_string();
    ///
    /// let entry = cache
    ///     .entry_by_ref(&key)
    ///     .or_insert_with(|| "value1".to_string());
    /// assert!(entry.is_fresh());
    /// assert_eq!(entry.key(), &key);
    /// assert_eq!(entry.into_value(), "value1");
    ///
    /// let entry = cache
    ///     .entry_by_ref(&key)
    ///     .or_insert_with(|| "value2".to_string());
    /// // Not fresh because the value was already in the cache.
    /// assert!(!entry.is_fresh());
    /// assert_eq!(entry.into_value(), "value1");
    /// ```
    ///
    /// # Concurrent calls on the same key
    ///
    /// This method guarantees that concurrent calls on the same not-existing entry
    /// are coalesced into one evaluation of the `init` closure. Only one of the
    /// calls evaluates its closure (thus returned entry's `is_fresh` method returns
    /// `true`), and other calls wait for that closure to complete (and their
    /// `is_fresh` return `false`).
    ///
    /// For more detail about the coalescing behavior, see
    /// [`Cache::get_with`][get-with-method].
    ///
    /// [get-with-method]: ./struct.Cache.html#method.get_with
    pub fn or_insert_with(self, init: impl FnOnce() -> V) -> Entry<K, V> {
        let replace_if = None as Option<fn(&V) -> bool>;
        self.cache.get_or_insert_with_hash_by_ref_and_fun(
            self.ref_key,
            self.hash,
            init,
            replace_if,
            true,
        )
    }

    /// Works like [`or_insert_with`](#method.or_insert_with), but takes an additional
    /// `replace_if` closure.
    ///
    /// This method will evaluate the `init` closure and insert the output to the
    /// cache when:
    ///
    /// - The key does not exist.
    /// - Or, `replace_if` closure returns `true`.
    pub fn or_insert_with_if(
        self,
        init: impl FnOnce() -> V,
        replace_if: impl FnMut(&V) -> bool,
    ) -> Entry<K, V> {
        self.cache.get_or_insert_with_hash_by_ref_and_fun(
            self.ref_key,
            self.hash,
            init,
            Some(replace_if),
            true,
        )
    }

    /// Returns the corresponding [`Entry`] for the reference of the key given when
    /// this entry selector was constructed. If the entry does not exist, clones the
    /// key and evaluates the `init` closure. If `Some(value)` was returned by the
    /// closure, inserts an entry with the value . If `None` was returned, this
    /// method does not insert an entry and returns `None`.
    ///
    /// [`Entry`]: ../struct.Entry.html
    ///
    /// # Example
    ///
    /// ```rust
    /// use moka::sync::Cache;
    ///
    /// let cache: Cache<String, u32> = Cache::new(100);
    /// let key = "key1".to_string();
    ///
    /// let none_entry = cache
    ///     .entry_by_ref(&key)
    ///     .or_optionally_insert_with(|| None);
    /// assert!(none_entry.is_none());
    ///
    /// let some_entry = cache
    ///     .entry_by_ref(&key)
    ///     .or_optionally_insert_with(|| Some(3));
    /// assert!(some_entry.is_some());
    /// let entry = some_entry.unwrap();
    /// assert!(entry.is_fresh());
    /// assert_eq!(entry.key(), &key);
    /// assert_eq!(entry.into_value(), 3);
    ///
    /// let some_entry = cache
    ///     .entry_by_ref(&key)
    ///     .or_optionally_insert_with(|| Some(6));
    /// let entry = some_entry.unwrap();
    /// // Not fresh because the value was already in the cache.
    /// assert!(!entry.is_fresh());
    /// assert_eq!(entry.into_value(), 3);
    /// ```
    ///
    /// # Concurrent calls on the same key
    ///
    /// This method guarantees that concurrent calls on the same not-existing entry
    /// are coalesced into one evaluation of the `init` closure. Only one of the
    /// calls evaluates its closure (thus returned entry's `is_fresh` method returns
    /// `true`), and other calls wait for that closure to complete (and their
    /// `is_fresh` return `false`).
    ///
    /// For more detail about the coalescing behavior, see
    /// [`Cache::optionally_get_with`][opt-get-with-method].
    ///
    /// [opt-get-with-method]: ./struct.Cache.html#method.optionally_get_with
    pub fn or_optionally_insert_with(
        self,
        init: impl FnOnce() -> Option<V>,
    ) -> Option<Entry<K, V>> {
        self.cache
            .get_or_optionally_insert_with_hash_by_ref_and_fun(self.ref_key, self.hash, init, true)
    }

    /// Returns the corresponding [`Entry`] for the reference of the key given when
    /// this entry selector was constructed. If the entry does not exist, clones the
    /// key and evaluates the `init` closure. If `Ok(value)` was returned from the
    /// closure, inserts an entry with the value. If `Err(_)` was returned, this
    /// method does not insert an entry and returns the `Err` wrapped by
    /// [`std::sync::Arc`][std-arc].
    ///
    /// [`Entry`]: ../struct.Entry.html
    /// [std-arc]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html
    ///
    /// # Example
    ///
    /// ```rust
    /// use moka::sync::Cache;
    ///
    /// let cache: Cache<String, u32> = Cache::new(100);
    /// let key = "key1".to_string();
    ///
    /// let error_entry = cache
    ///     .entry_by_ref(&key)
    ///     .or_try_insert_with(|| Err("error"));
    /// assert!(error_entry.is_err());
    ///
    /// let ok_entry = cache
    ///     .entry_by_ref(&key)
    ///     .or_try_insert_with(|| Ok::<u32, &str>(3));
    /// assert!(ok_entry.is_ok());
    /// let entry = ok_entry.unwrap();
    /// assert!(entry.is_fresh());
    /// assert_eq!(entry.key(), &key);
    /// assert_eq!(entry.into_value(), 3);
    ///
    /// let ok_entry = cache
    ///     .entry_by_ref(&key)
    ///     .or_try_insert_with(|| Ok::<u32, &str>(6));
    /// let entry = ok_entry.unwrap();
    /// // Not fresh because the value was already in the cache.
    /// assert!(!entry.is_fresh());
    /// assert_eq!(entry.into_value(), 3);
    /// ```
    ///
    /// # Concurrent calls on the same key
    ///
    /// This method guarantees that concurrent calls on the same not-existing entry
    /// are coalesced into one evaluation of the `init` closure (as long as these
    /// closures return the same error type). Only one of the calls evaluates its
    /// closure (thus returned entry's `is_fresh` method returns `true`), and other
    /// calls wait for that closure to complete (and their `is_fresh` return
    /// `false`).
    ///
    /// For more detail about the coalescing behavior, see
    /// [`Cache::try_get_with`][try-get-with-method].
    ///
    /// [try-get-with-method]: ./struct.Cache.html#method.try_get_with
    pub fn or_try_insert_with<F, E>(self, init: F) -> Result<Entry<K, V>, Arc<E>>
    where
        F: FnOnce() -> Result<V, E>,
        E: Send + Sync + 'static,
    {
        self.cache
            .get_or_try_insert_with_hash_by_ref_and_fun(self.ref_key, self.hash, init, true)
    }
}
