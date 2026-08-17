use super::{Cache, FutureExt};
use crate::{
    common::{builder_utils, concurrent::Weigher, time::Clock, HousekeeperConfig},
    notification::{AsyncEvictionListener, ListenerFuture, RemovalCause},
    policy::{EvictionPolicy, ExpirationPolicy},
    Expiry,
};

use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hash},
    marker::PhantomData,
    sync::Arc,
    time::Duration,
};

/// Builds a [`Cache`][cache-struct] with various configuration knobs.
///
/// [cache-struct]: ./struct.Cache.html
///
/// # Example: Expirations
///
/// ```rust
/// // Cargo.toml
/// //
/// // [dependencies]
/// // moka = { version = "0.12", features = ["future"] }
/// // tokio = { version = "1", features = ["rt-multi-thread", "macros" ] }
/// // futures = "0.3"
///
/// use moka::future::Cache;
/// use std::time::Duration;
///
/// #[tokio::main]
/// async fn main() {
///     let cache = Cache::builder()
///         // Max 10,000 entries
///         .max_capacity(10_000)
///         // Time to live (TTL): 30 minutes
///         .time_to_live(Duration::from_secs(30 * 60))
///         // Time to idle (TTI):  5 minutes
///         .time_to_idle(Duration::from_secs( 5 * 60))
///         // Create the cache.
///         .build();
///
///     // This entry will expire after 5 minutes (TTI) if there is no get().
///     cache.insert(0, "zero").await;
///
///     // This get() will extend the entry life for another 5 minutes.
///     cache.get(&0);
///
///     // Even though we keep calling get(), the entry will expire
///     // after 30 minutes (TTL) from the insert().
/// }
/// ```
///
#[must_use]
pub struct CacheBuilder<K, V, C> {
    name: Option<String>,
    max_capacity: Option<u64>,
    initial_capacity: Option<usize>,
    weigher: Option<Weigher<K, V>>,
    eviction_policy: EvictionPolicy,
    eviction_listener: Option<AsyncEvictionListener<K, V>>,
    expiration_policy: ExpirationPolicy<K, V>,
    housekeeper_config: HousekeeperConfig,
    invalidator_enabled: bool,
    clock: Clock,
    cache_type: PhantomData<C>,
}

impl<K, V> Default for CacheBuilder<K, V, Cache<K, V, RandomState>>
where
    K: Eq + Hash + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self {
            name: None,
            max_capacity: None,
            initial_capacity: None,
            weigher: None,
            eviction_policy: EvictionPolicy::default(),
            eviction_listener: None,
            expiration_policy: ExpirationPolicy::default(),
            housekeeper_config: HousekeeperConfig::default(),
            invalidator_enabled: false,
            clock: Clock::default(),
            cache_type: PhantomData,
        }
    }
}

impl<K, V> CacheBuilder<K, V, Cache<K, V, RandomState>>
where
    K: Eq + Hash + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    /// Construct a new `CacheBuilder` that will be used to build a `Cache` holding
    /// up to `max_capacity` entries.
    pub fn new(max_capacity: u64) -> Self {
        Self {
            max_capacity: Some(max_capacity),
            ..Self::default()
        }
    }

    /// Builds a `Cache<K, V>`.
    ///
    /// # Panics
    ///
    /// Panics if configured with either `time_to_live` or `time_to_idle` higher than
    /// 1000 years. This is done to protect against overflow when computing key
    /// expiration.
    pub fn build(self) -> Cache<K, V, RandomState> {
        let build_hasher = RandomState::default();
        let exp = &self.expiration_policy;
        builder_utils::ensure_expirations_or_panic(exp.time_to_live(), exp.time_to_idle());
        Cache::with_everything(
            self.name,
            self.max_capacity,
            self.initial_capacity,
            build_hasher,
            self.weigher,
            self.eviction_policy,
            self.eviction_listener,
            self.expiration_policy,
            self.housekeeper_config,
            self.invalidator_enabled,
            self.clock,
        )
    }

    /// Builds a `Cache<K, V, S>` with the given `hasher` of type `S`.
    ///
    /// # Examples
    ///
    /// This example uses AHash hasher from [AHash][ahash-crate] crate.
    ///
    /// [ahash-crate]: https://crates.io/crates/ahash
    ///
    /// ```rust
    /// // Cargo.toml
    /// // [dependencies]
    /// // ahash = "0.8"
    /// // moka = { version = ..., features = ["future"] }
    /// // tokio = { version = "1", features = ["rt-multi-thread", "macros" ] }
    ///
    /// use moka::future::Cache;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     // The type of this cache is: Cache<i32, String, ahash::RandomState>
    ///     let cache = Cache::builder()
    ///         .max_capacity(100)
    ///         .build_with_hasher(ahash::RandomState::default());
    ///     cache.insert(1, "one".to_string()).await;
    /// }
    /// ```
    ///
    /// Note: If you need to add a type annotation to your cache, you must use the
    /// form of `Cache<K, V, S>` instead of `Cache<K, V>`. That `S` is the type of
    /// the build hasher, and its default is the `RandomState` from
    /// `std::collections::hash_map` module . If you use a different build hasher,
    /// you must specify `S` explicitly.
    ///
    /// Here is a good example:
    ///
    /// ```rust
    /// # use moka::future::Cache;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let cache = Cache::builder()
    /// #     .build_with_hasher(ahash::RandomState::default());
    /// struct Good {
    ///     // Specifying the type in Cache<K, V, S> format.
    ///     cache: Cache<i32, String, ahash::RandomState>,
    /// }
    ///
    /// // Storing the cache from above example. This should compile.
    /// Good { cache };
    /// # }
    /// ```
    ///
    /// Here is a bad example. This struct cannot store the above cache because it
    /// does not specify `S`:
    ///
    /// ```compile_fail
    /// # use moka::future::Cache;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let cache = Cache::builder()
    /// #     .build_with_hasher(ahash::RandomState::default());
    /// struct Bad {
    ///     // Specifying the type in Cache<K, V> format.
    ///     cache: Cache<i32, String>,
    /// }
    ///
    /// // This should not compile.
    /// Bad { cache };
    /// // => error[E0308]: mismatched types
    /// //    expected struct `std::collections::hash_map::RandomState`,
    /// //       found struct `ahash::RandomState`
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if configured with either `time_to_live` or `time_to_idle` higher than
    /// 1000 years. This is done to protect against overflow when computing key
    /// expiration.
    pub fn build_with_hasher<S>(self, hasher: S) -> Cache<K, V, S>
    where
        S: BuildHasher + Clone + Send + Sync + 'static,
    {
        let exp = &self.expiration_policy;
        builder_utils::ensure_expirations_or_panic(exp.time_to_live(), exp.time_to_idle());
        Cache::with_everything(
            self.name,
            self.max_capacity,
            self.initial_capacity,
            hasher,
            self.weigher,
            self.eviction_policy,
            self.eviction_listener,
            self.expiration_policy,
            self.housekeeper_config,
            self.invalidator_enabled,
            self.clock,
        )
    }
}

impl<K, V, C> CacheBuilder<K, V, C> {
    /// Sets the name of the cache. Currently the name is used for identification
    /// only in logging messages.
    pub fn name(self, name: &str) -> Self {
        Self {
            name: Some(name.to_string()),
            ..self
        }
    }

    /// Sets the max capacity of the cache.
    pub fn max_capacity(self, max_capacity: u64) -> Self {
        Self {
            max_capacity: Some(max_capacity),
            ..self
        }
    }

    /// Sets the initial capacity (number of entries) of the cache.
    pub fn initial_capacity(self, number_of_entries: usize) -> Self {
        Self {
            initial_capacity: Some(number_of_entries),
            ..self
        }
    }

    /// Sets the eviction (and admission) policy of the cache.
    ///
    /// The default policy is TinyLFU. See [`EvictionPolicy`][eviction-policy] for
    /// more details.
    ///
    /// [eviction-policy]: ../policy/struct.EvictionPolicy.html
    pub fn eviction_policy(self, policy: EvictionPolicy) -> Self {
        Self {
            eviction_policy: policy,
            ..self
        }
    }

    /// Sets the weigher closure to the cache.
    ///
    /// The closure should take `&K` and `&V` as the arguments and returns a `u32`
    /// representing the relative size of the entry.
    pub fn weigher(self, weigher: impl Fn(&K, &V) -> u32 + Send + Sync + 'static) -> Self {
        Self {
            weigher: Some(Arc::new(weigher)),
            ..self
        }
    }

    /// Sets the eviction listener closure to the cache. The closure should take
    /// `Arc<K>`, `V` and [`RemovalCause`][removal-cause] as the arguments.
    ///
    /// See [this example][example] for a usage of eviction listener.
    ///
    /// # Sync or Async Eviction Listener
    ///
    /// The closure can be either synchronous or asynchronous, and `CacheBuilder`
    /// provides two methods for setting the eviction listener closure:
    ///
    /// - If you do not need to `.await` anything in the eviction listener, use this
    ///   `eviction_listener` method.
    /// - If you need to `.await` something in the eviction listener, use
    ///   [`async_eviction_listener`](#method.async_eviction_listener) method
    ///   instead.
    ///
    /// # Panics
    ///
    /// It is very important to make the listener closure not to panic. Otherwise,
    /// the cache will stop calling the listener after a panic. This is an intended
    /// behavior because the cache cannot know whether it is memory safe or not to
    /// call the panicked listener again.
    ///
    /// [removal-cause]: ../notification/enum.RemovalCause.html
    /// [example]: ./struct.Cache.html#per-entry-expiration-policy
    pub fn eviction_listener<F>(self, listener: F) -> Self
    where
        F: Fn(Arc<K>, V, RemovalCause) + Send + Sync + 'static,
    {
        let async_listener = move |k, v, c| {
            {
                listener(k, v, c);
                std::future::ready(())
            }
            .boxed()
        };

        self.async_eviction_listener(async_listener)
    }

    /// Sets the eviction listener closure to the cache. The closure should take
    /// `Arc<K>`, `V` and [`RemovalCause`][removal-cause] as the arguments, and
    /// return a [`ListenerFuture`][listener-future].
    ///
    /// See [this example][example] for a usage of asynchronous eviction listener.
    ///
    /// # Sync or Async Eviction Listener
    ///
    /// The closure can be either synchronous or asynchronous, and `CacheBuilder`
    /// provides two methods for setting the eviction listener closure:
    ///
    /// - If you do not need to `.await` anything in the eviction listener, use
    ///   [`eviction_listener`](#method.eviction_listener) method instead.
    /// - If you need to `.await` something in the eviction listener, use
    ///   this method.
    ///
    /// # Panics
    ///
    /// It is very important to make the listener closure not to panic. Otherwise,
    /// the cache will stop calling the listener after a panic. This is an intended
    /// behavior because the cache cannot know whether it is memory safe or not to
    /// call the panicked listener again.
    ///
    /// [removal-cause]: ../notification/enum.RemovalCause.html
    /// [listener-future]: ../notification/type.ListenerFuture.html
    /// [example]: ./struct.Cache.html#example-eviction-listener
    pub fn async_eviction_listener<F>(self, listener: F) -> Self
    where
        F: Fn(Arc<K>, V, RemovalCause) -> ListenerFuture + Send + Sync + 'static,
    {
        Self {
            eviction_listener: Some(Box::new(listener)),
            ..self
        }
    }

    /// Sets the time to live of the cache.
    ///
    /// A cached entry will be expired after the specified duration past from
    /// `insert`.
    ///
    /// # Panics
    ///
    /// `CacheBuilder::build*` methods will panic if the given `duration` is longer
    /// than 1000 years. This is done to protect against overflow when computing key
    /// expiration.
    pub fn time_to_live(self, duration: Duration) -> Self {
        let mut builder = self;
        builder.expiration_policy.set_time_to_live(duration);
        builder
    }

    /// Sets the time to idle of the cache.
    ///
    /// A cached entry will be expired after the specified duration past from `get`
    /// or `insert`.
    ///
    /// # Panics
    ///
    /// `CacheBuilder::build*` methods will panic if the given `duration` is longer
    /// than 1000 years. This is done to protect against overflow when computing key
    /// expiration.
    pub fn time_to_idle(self, duration: Duration) -> Self {
        let mut builder = self;
        builder.expiration_policy.set_time_to_idle(duration);
        builder
    }

    /// Sets the given `expiry` to the cache.
    ///
    /// See [the example][per-entry-expiration-example] for per-entry expiration
    /// policy in the `Cache` documentation.
    ///
    /// [per-entry-expiration-example]:
    ///     ./struct.Cache.html#per-entry-expiration-policy
    pub fn expire_after(self, expiry: impl Expiry<K, V> + Send + Sync + 'static) -> Self {
        let mut builder = self;
        builder.expiration_policy.set_expiry(Arc::new(expiry));
        builder
    }

    #[cfg(test)]
    pub(crate) fn housekeeper_config(self, conf: HousekeeperConfig) -> Self {
        Self {
            housekeeper_config: conf,
            ..self
        }
    }

    #[cfg(test)]
    pub(crate) fn clock(self, clock: Clock) -> Self {
        Self { clock, ..self }
    }

    /// Enables support for [`Cache::invalidate_entries_if`][cache-invalidate-if]
    /// method.
    ///
    /// The cache will maintain additional internal data structures to support
    /// `invalidate_entries_if` method.
    ///
    /// [cache-invalidate-if]: ./struct.Cache.html#method.invalidate_entries_if
    pub fn support_invalidation_closures(self) -> Self {
        Self {
            invalidator_enabled: true,
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CacheBuilder;

    use std::time::Duration;

    #[tokio::test]
    async fn build_cache() {
        // Cache<char, String>
        let cache = CacheBuilder::new(100).build();
        let policy = cache.policy();

        assert_eq!(policy.max_capacity(), Some(100));
        assert_eq!(policy.time_to_live(), None);
        assert_eq!(policy.time_to_idle(), None);
        assert_eq!(policy.num_segments(), 1);

        cache.insert('a', "Alice").await;
        assert_eq!(cache.get(&'a').await, Some("Alice"));

        let cache = CacheBuilder::new(100)
            .time_to_live(Duration::from_secs(45 * 60))
            .time_to_idle(Duration::from_secs(15 * 60))
            .build();
        let policy = cache.policy();

        assert_eq!(policy.max_capacity(), Some(100));
        assert_eq!(policy.time_to_live(), Some(Duration::from_secs(45 * 60)));
        assert_eq!(policy.time_to_idle(), Some(Duration::from_secs(15 * 60)));
        assert_eq!(policy.num_segments(), 1);

        cache.insert('a', "Alice").await;
        assert_eq!(cache.get(&'a').await, Some("Alice"));
    }

    #[tokio::test]
    #[should_panic(expected = "time_to_live is longer than 1000 years")]
    async fn build_cache_too_long_ttl() {
        let thousand_years_secs: u64 = 1000 * 365 * 24 * 3600;
        let builder: CacheBuilder<char, String, _> = CacheBuilder::new(100);
        let duration = Duration::from_secs(thousand_years_secs);
        builder
            .time_to_live(duration + Duration::from_secs(1))
            .build();
    }

    #[tokio::test]
    #[should_panic(expected = "time_to_idle is longer than 1000 years")]
    async fn build_cache_too_long_tti() {
        let thousand_years_secs: u64 = 1000 * 365 * 24 * 3600;
        let builder: CacheBuilder<char, String, _> = CacheBuilder::new(100);
        let duration = Duration::from_secs(thousand_years_secs);
        builder
            .time_to_idle(duration + Duration::from_secs(1))
            .build();
    }
}
