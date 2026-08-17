use equivalent::Equivalent;

use super::{
    base_cache::BaseCache,
    value_initializer::{InitResult, ValueInitializer},
    CacheBuilder, CancelGuard, Iter, OwnedKeyEntrySelector, PredicateId, RefKeyEntrySelector,
    WriteOp,
};
use crate::{
    common::{concurrent::Weigher, time::Clock, HousekeeperConfig},
    notification::AsyncEvictionListener,
    ops::compute::{self, CompResult},
    policy::{EvictionPolicy, ExpirationPolicy},
    Entry, Policy, PredicateError,
};

#[cfg(feature = "unstable-debug-counters")]
use crate::common::concurrent::debug_counters::CacheDebugStats;

use std::{
    collections::hash_map::RandomState,
    fmt,
    future::Future,
    hash::{BuildHasher, Hash},
    pin::Pin,
    sync::Arc,
};

#[cfg(test)]
use std::sync::atomic::{AtomicBool, Ordering};

/// A thread-safe, futures-aware concurrent in-memory cache.
///
/// `Cache` supports full concurrency of retrievals and a high expected concurrency
/// for updates. It utilizes a lock-free concurrent hash table as the central
/// key-value storage. It performs a best-effort bounding of the map using an entry
/// replacement algorithm to determine which entries to evict when the capacity is
/// exceeded.
///
/// To use this cache, enable a crate feature called "future".
///
/// # Table of Contents
///
/// - [Example: `insert`, `get` and `invalidate`](#example-insert-get-and-invalidate)
/// - [Avoiding to clone the value at `get`](#avoiding-to-clone-the-value-at-get)
/// - [Sharing a cache across asynchronous tasks](#sharing-a-cache-across-asynchronous-tasks)
///     - [No lock is needed](#no-lock-is-needed)
/// - [Hashing Algorithm](#hashing-algorithm)
/// - [Example: Size-based Eviction](#example-size-based-eviction)
/// - [Example: Time-based Expirations](#example-time-based-expirations)
///     - [Cache-level TTL and TTI policies](#cache-level-ttl-and-tti-policies)
///     - [Per-entry expiration policy](#per-entry-expiration-policy)
/// - [Example: Eviction Listener](#example-eviction-listener)
///     - [You should avoid eviction listener to panic](#you-should-avoid-eviction-listener-to-panic)
///
/// # Example: `insert`, `get` and `invalidate`
///
/// Cache entries are manually added using [`insert`](#method.insert) of
/// [`get_with`](#method.get_with) method, and are stored in the cache until either
/// evicted or manually invalidated:
///
/// Here's an example of reading and updating a cache by using multiple asynchronous
/// tasks with [Tokio][tokio-crate] runtime:
///
/// [tokio-crate]: https://crates.io/crates/tokio
///
///```rust
/// // Cargo.toml
/// //
/// // [dependencies]
/// // moka = { version = "0.12", features = ["future"] }
/// // tokio = { version = "1", features = ["rt-multi-thread", "macros" ] }
/// // futures-util = "0.3"
///
/// use moka::future::Cache;
///
/// #[tokio::main]
/// async fn main() {
///     const NUM_TASKS: usize = 16;
///     const NUM_KEYS_PER_TASK: usize = 64;
///
///     fn value(n: usize) -> String {
///         format!("value {n}")
///     }
///
///     // Create a cache that can store up to 10,000 entries.
///     let cache = Cache::new(10_000);
///
///     // Spawn async tasks and write to and read from the cache.
///     let tasks: Vec<_> = (0..NUM_TASKS)
///         .map(|i| {
///             // To share the same cache across the async tasks, clone it.
///             // This is a cheap operation.
///             let my_cache = cache.clone();
///             let start = i * NUM_KEYS_PER_TASK;
///             let end = (i + 1) * NUM_KEYS_PER_TASK;
///
///             tokio::spawn(async move {
///                 // Insert 64 entries. (NUM_KEYS_PER_TASK = 64)
///                 for key in start..end {
///                     my_cache.insert(key, value(key)).await;
///                     // get() returns Option<String>, a clone of the stored value.
///                     assert_eq!(my_cache.get(&key).await, Some(value(key)));
///                 }
///
///                 // Invalidate every 4 element of the inserted entries.
///                 for key in (start..end).step_by(4) {
///                     my_cache.invalidate(&key).await;
///                 }
///             })
///         })
///         .collect();
///
///     // Wait for all tasks to complete.
///     futures_util::future::join_all(tasks).await;
///
///     // Verify the result.
///     for key in 0..(NUM_TASKS * NUM_KEYS_PER_TASK) {
///         if key % 4 == 0 {
///             assert_eq!(cache.get(&key).await, None);
///         } else {
///             assert_eq!(cache.get(&key).await, Some(value(key)));
///         }
///     }
/// }
/// ```
///
/// If you want to atomically initialize and insert a value when the key is not
/// present, you might want to check other insertion methods
/// [`get_with`](#method.get_with) and [`try_get_with`](#method.try_get_with).
///
/// # Avoiding to clone the value at `get`
///
/// The return type of `get` method is `Option<V>` instead of `Option<&V>`. Every
/// time `get` is called for an existing key, it creates a clone of the stored value
/// `V` and returns it. This is because the `Cache` allows concurrent updates from
/// threads so a value stored in the cache can be dropped or replaced at any time by
/// any other thread. `get` cannot return a reference `&V` as it is impossible to
/// guarantee the value outlives the reference.
///
/// If you want to store values that will be expensive to clone, wrap them by
/// `std::sync::Arc` before storing in a cache. [`Arc`][rustdoc-std-arc] is a
/// thread-safe reference-counted pointer and its `clone()` method is cheap.
///
/// [rustdoc-std-arc]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html
///
/// # Sharing a cache across asynchronous tasks
///
/// To share a cache across async tasks (or OS threads), do one of the followings:
///
/// - Create a clone of the cache by calling its `clone` method and pass it to other
///   task.
/// - If you are using a web application framework such as Actix Web or Axum, you can
///   store a cache in Actix Web's [`web::Data`][actix-web-data] or Axum's
///   [shared state][axum-state-extractor], and access it from each request handler.
/// - Wrap the cache by a `sync::OnceCell` or `sync::Lazy` from
///   [once_cell][once-cell-crate] create, and set it to a `static` variable.
///
/// Cloning is a cheap operation for `Cache` as it only creates thread-safe
/// reference-counted pointers to the internal data structures.
///
/// [once-cell-crate]: https://crates.io/crates/once_cell
/// [actix-web-data]: https://docs.rs/actix-web/4.3.1/actix_web/web/struct.Data.html
/// [axum-state-extractor]: https://docs.rs/axum/latest/axum/#sharing-state-with-handlers
///
/// ## No lock is needed
///
/// Don't wrap a `Cache` by a lock such as `Mutex` or `RwLock`. All methods provided
/// by the `Cache` are considered thread-safe, and can be safely called by multiple
/// async tasks at the same time. No lock is needed.
///
/// [once-cell-crate]: https://crates.io/crates/once_cell
///
/// # Hashing Algorithm
///
/// By default, `Cache` uses a hashing algorithm selected to provide resistance
/// against HashDoS attacks. It will be the same one used by
/// `std::collections::HashMap`, which is currently SipHash 1-3.
///
/// While SipHash's performance is very competitive for medium sized keys, other
/// hashing algorithms will outperform it for small keys such as integers as well as
/// large keys such as long strings. However those algorithms will typically not
/// protect against attacks such as HashDoS.
///
/// The hashing algorithm can be replaced on a per-`Cache` basis using the
/// [`build_with_hasher`][build-with-hasher-method] method of the `CacheBuilder`.
/// Many alternative algorithms are available on crates.io, such as the
/// [AHash][ahash-crate] crate.
///
/// [build-with-hasher-method]: ./struct.CacheBuilder.html#method.build_with_hasher
/// [ahash-crate]: https://crates.io/crates/ahash
///
/// # Example: Size-based Eviction
///
/// ```rust
/// // Cargo.toml
/// //
/// // [dependencies]
/// // moka = { version = "0.12", features = ["future"] }
/// // tokio = { version = "1", features = ["rt-multi-thread", "macros" ] }
/// // futures-util = "0.3"
///
/// use moka::future::Cache;
///
/// #[tokio::main]
/// async fn main() {
///     // Evict based on the number of entries in the cache.
///     let cache = Cache::builder()
///         // Up to 10,000 entries.
///         .max_capacity(10_000)
///         // Create the cache.
///         .build();
///     cache.insert(1, "one".to_string()).await;
///
///     // Evict based on the byte length of strings in the cache.
///     let cache = Cache::builder()
///         // A weigher closure takes &K and &V and returns a u32
///         // representing the relative size of the entry.
///         .weigher(|_key, value: &String| -> u32 {
///             value.len().try_into().unwrap_or(u32::MAX)
///         })
///         // This cache will hold up to 32MiB of values.
///         .max_capacity(32 * 1024 * 1024)
///         .build();
///     cache.insert(2, "two".to_string()).await;
/// }
/// ```
///
/// If your cache should not grow beyond a certain size, use the `max_capacity`
/// method of the [`CacheBuilder`][builder-struct] to set the upper bound. The cache
/// will try to evict entries that have not been used recently or very often.
///
/// At the cache creation time, a weigher closure can be set by the `weigher` method
/// of the `CacheBuilder`. A weigher closure takes `&K` and `&V` as the arguments and
/// returns a `u32` representing the relative size of the entry:
///
/// - If the `weigher` is _not_ set, the cache will treat each entry has the same
///   size of `1`. This means the cache will be bounded by the number of entries.
/// - If the `weigher` is set, the cache will call the weigher to calculate the
///   weighted size (relative size) on an entry. This means the cache will be bounded
///   by the total weighted size of entries.
///
/// Note that weighted sizes are not used when making eviction selections.
///
/// [builder-struct]: ./struct.CacheBuilder.html
///
/// # Example: Time-based Expirations
///
/// ## Cache-level TTL and TTI policies
///
/// `Cache` supports the following cache-level expiration policies:
///
/// - **Time to live (TTL)**: A cached entry will be expired after the specified
///   duration past from `insert`.
/// - **Time to idle (TTI)**: A cached entry will be expired after the specified
///   duration past from `get` or `insert`.
///
/// They are a cache-level expiration policies; all entries in the cache will have
/// the same TTL and/or TTI durations. If you want to set different expiration
/// durations for different entries, see the next section.
///
/// ```rust
/// // Cargo.toml
/// //
/// // [dependencies]
/// // moka = { version = "0.12", features = ["future"] }
/// // tokio = { version = "1", features = ["rt-multi-thread", "macros" ] }
/// // futures-util = "0.3"
///
/// use moka::future::Cache;
/// use std::time::Duration;
///
/// #[tokio::main]
/// async fn main() {
///     let cache = Cache::builder()
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
/// ## Per-entry expiration policy
///
/// `Cache` supports per-entry expiration policy through the `Expiry` trait.
///
/// `Expiry` trait provides three callback methods:
/// [`expire_after_create`][exp-create], [`expire_after_read`][exp-read] and
/// [`expire_after_update`][exp-update]. When a cache entry is inserted, read or
/// updated, one of these methods is called. These methods return an
/// `Option<Duration>`, which is used as the expiration duration of the entry.
///
/// `Expiry` trait provides the default implementations of these methods, so you will
/// implement only the methods you want to customize.
///
/// [exp-create]: ../trait.Expiry.html#method.expire_after_create
/// [exp-read]: ../trait.Expiry.html#method.expire_after_read
/// [exp-update]: ../trait.Expiry.html#method.expire_after_update
///
/// ```rust
/// // Cargo.toml
/// //
/// // [dependencies]
/// // moka = { version = "0.12", features = ["future"] }
/// // tokio = { version = "1", features = ["rt-multi-thread", "macros", "time" ] }
///
/// use moka::{future::Cache, Expiry};
/// use std::time::{Duration, Instant};
///
/// // In this example, we will create a `future::Cache` with `u32` as the key, and
/// // `(Expiration, String)` as the value. `Expiration` is an enum to represent the
/// // expiration of the value, and `String` is the application data of the value.
///
/// /// An enum to represent the expiration of a value.
/// #[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// pub enum Expiration {
///     /// The value never expires.
///     Never,
///     /// The value expires after a short time. (5 seconds in this example)
///     AfterShortTime,
///     /// The value expires after a long time. (15 seconds in this example)
///     AfterLongTime,
/// }
///
/// impl Expiration {
///     /// Returns the duration of this expiration.
///     pub fn as_duration(&self) -> Option<Duration> {
///         match self {
///             Expiration::Never => None,
///             Expiration::AfterShortTime => Some(Duration::from_secs(5)),
///             Expiration::AfterLongTime => Some(Duration::from_secs(15)),
///         }
///     }
/// }
///
/// /// An expiry that implements `moka::Expiry` trait. `Expiry` trait provides the
/// /// default implementations of three callback methods `expire_after_create`,
/// /// `expire_after_read`, and `expire_after_update`.
/// ///
/// /// In this example, we only override the `expire_after_create` method.
/// pub struct MyExpiry;
///
/// impl Expiry<u32, (Expiration, String)> for MyExpiry {
///     /// Returns the duration of the expiration of the value that was just
///     /// created.
///     fn expire_after_create(
///         &self,
///         _key: &u32,
///         value: &(Expiration, String),
///         _current_time: Instant,
///     ) -> Option<Duration> {
///         let duration = value.0.as_duration();
///         println!("MyExpiry: expire_after_create called with key {_key} and value {value:?}. Returning {duration:?}.");
///         duration
///     }
/// }
///
/// #[tokio::main]
/// async fn main() {
///     // Create a `Cache<u32, (Expiration, String)>` with an expiry `MyExpiry` and
///     // eviction listener.
///     let expiry = MyExpiry;
///
///     let eviction_listener = |key, _value, cause| {
///         println!("Evicted key {key}. Cause: {cause:?}");
///     };
///
///     let cache = Cache::builder()
///         .max_capacity(100)
///         .expire_after(expiry)
///         .eviction_listener(eviction_listener)
///         .build();
///
///     // Insert some entries into the cache with different expirations.
///     cache
///         .get_with(0, async { (Expiration::AfterShortTime, "a".to_string()) })
///         .await;
///
///     cache
///         .get_with(1, async { (Expiration::AfterLongTime, "b".to_string()) })
///         .await;
///
///     cache
///         .get_with(2, async { (Expiration::Never, "c".to_string()) })
///         .await;
///
///     // Verify that all the inserted entries exist.
///     assert!(cache.contains_key(&0));
///     assert!(cache.contains_key(&1));
///     assert!(cache.contains_key(&2));
///
///     // Sleep for 6 seconds. Key 0 should expire.
///     println!("\nSleeping for 6 seconds...\n");
///     tokio::time::sleep(Duration::from_secs(6)).await;
///     cache.run_pending_tasks().await;
///     println!("Entry count: {}", cache.entry_count());
///
///     // Verify that key 0 has been evicted.
///     assert!(!cache.contains_key(&0));
///     assert!(cache.contains_key(&1));
///     assert!(cache.contains_key(&2));
///
///     // Sleep for 10 more seconds. Key 1 should expire.
///     println!("\nSleeping for 10 seconds...\n");
///     tokio::time::sleep(Duration::from_secs(10)).await;
///     cache.run_pending_tasks().await;
///     println!("Entry count: {}", cache.entry_count());
///
///     // Verify that key 1 has been evicted.
///     assert!(!cache.contains_key(&1));
///     assert!(cache.contains_key(&2));
///
///     // Manually invalidate key 2.
///     cache.invalidate(&2).await;
///     assert!(!cache.contains_key(&2));
///
///     println!("\nSleeping for a second...\n");
///     tokio::time::sleep(Duration::from_secs(1)).await;
///     cache.run_pending_tasks().await;
///     println!("Entry count: {}", cache.entry_count());
///
///     println!("\nDone!");
/// }
/// ```
///
/// # Example: Eviction Listener
///
/// A `Cache` can be configured with an eviction listener, a closure that is called
/// every time there is a cache eviction. The listener takes three parameters: the
/// key and value of the evicted entry, and the
/// [`RemovalCause`](../notification/enum.RemovalCause.html) to indicate why the
/// entry was evicted.
///
/// An eviction listener can be used to keep other data structures in sync with the
/// cache, for example.
///
/// The following example demonstrates how to use an eviction listener with
/// time-to-live expiration to manage the lifecycle of temporary files on a
/// filesystem. The cache stores the paths of the files, and when one of them has
/// expired, the eviction listener will be called with the path, so it can remove the
/// file from the filesystem.
///
/// ```rust
/// // Cargo.toml
/// //
/// // [dependencies]
/// // anyhow = "1.0"
/// // uuid = { version = "1.1", features = ["v4"] }
/// // tokio = { version = "1.18", features = ["fs", "macros", "rt-multi-thread", "sync", "time"] }
///
/// use moka::{future::Cache, notification::ListenerFuture};
/// // FutureExt trait provides the boxed method.
/// use moka::future::FutureExt;
///
/// use anyhow::{anyhow, Context};
/// use std::{
///     io,
///     path::{Path, PathBuf},
///     sync::Arc,
///     time::Duration,
/// };
/// use tokio::{fs, sync::RwLock};
/// use uuid::Uuid;
///
/// /// The DataFileManager writes, reads and removes data files.
/// struct DataFileManager {
///     base_dir: PathBuf,
///     file_count: usize,
/// }
///
/// impl DataFileManager {
///     fn new(base_dir: PathBuf) -> Self {
///         Self {
///             base_dir,
///             file_count: 0,
///         }
///     }
///
///     async fn write_data_file(
///         &mut self,
///         key: impl AsRef<str>,
///         contents: String
///     ) -> io::Result<PathBuf> {
///         // Use the key as a part of the filename.
///         let mut path = self.base_dir.to_path_buf();
///         path.push(key.as_ref());
///
///         assert!(!path.exists(), "Path already exists: {path:?}");
///
///         // create the file at the path and write the contents to the file.
///         fs::write(&path, contents).await?;
///         self.file_count += 1;
///         println!("Created a data file at {path:?} (file count: {})", self.file_count);
///         Ok(path)
///     }
///
///     async fn read_data_file(&self, path: impl AsRef<Path>) -> io::Result<String> {
///         // Reads the contents of the file at the path, and return the contents.
///         fs::read_to_string(path).await
///     }
///
///     async fn remove_data_file(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
///         // Remove the file at the path.
///         fs::remove_file(path.as_ref()).await?;
///         self.file_count -= 1;
///         println!(
///             "Removed a data file at {:?} (file count: {})",
///             path.as_ref(),
///             self.file_count
///         );
///
///         Ok(())
///     }
/// }
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     // Create an instance of the DataFileManager and wrap it with
///     // Arc<RwLock<_>> so it can be shared across threads.
///     let mut base_dir = std::env::temp_dir();
///     base_dir.push(Uuid::new_v4().as_hyphenated().to_string());
///     println!("base_dir: {base_dir:?}");
///     std::fs::create_dir(&base_dir)?;
///
///     let file_mgr = DataFileManager::new(base_dir);
///     let file_mgr = Arc::new(RwLock::new(file_mgr));
///
///     let file_mgr1 = Arc::clone(&file_mgr);
///     let rt = tokio::runtime::Handle::current();
///
///     // Create an eviction listener closure.
///     let eviction_listener = move |k, v: PathBuf, cause| -> ListenerFuture {
///         println!("\n== An entry has been evicted. k: {k:?}, v: {v:?}, cause: {cause:?}");
///         let file_mgr2 = Arc::clone(&file_mgr1);
///
///         // Create a Future that removes the data file at the path `v`.
///         async move {
///             // Acquire the write lock of the DataFileManager.
///             let mut mgr = file_mgr2.write().await;
///             // Remove the data file. We must handle error cases here to
///             // prevent the listener from panicking.
///             if let Err(_e) = mgr.remove_data_file(v.as_path()).await {
///                 eprintln!("Failed to remove a data file at {v:?}");
///             }
///         }
///         // Convert the regular Future into ListenerFuture. This method is
///         // provided by moka::future::FutureExt trait.
///         .boxed()
///     };
///
///     // Create the cache. Set time to live for two seconds and set the
///     // eviction listener.
///     let cache = Cache::builder()
///         .max_capacity(100)
///         .time_to_live(Duration::from_secs(2))
///         .async_eviction_listener(eviction_listener)
///         .build();
///
///     // Insert an entry to the cache.
///     // This will create and write a data file for the key "user1", store the
///     // path of the file to the cache, and return it.
///     println!("== try_get_with()");
///     let key = "user1";
///     let path = cache
///         .try_get_with(key, async {
///             let mut mgr = file_mgr.write().await;
///             let path = mgr
///                 .write_data_file(key, "user data".into())
///                 .await
///                 .with_context(|| format!("Failed to create a data file"))?;
///             Ok(path) as anyhow::Result<_>
///         })
///         .await
///         .map_err(|e| anyhow!("{e}"))?;
///
///     // Read the data file at the path and print the contents.
///     println!("\n== read_data_file()");
///     {
///         let mgr = file_mgr.read().await;
///         let contents = mgr
///             .read_data_file(path.as_path())
///             .await
///             .with_context(|| format!("Failed to read data from {path:?}"))?;
///         println!("contents: {contents}");
///     }
///
///     // Sleep for five seconds. While sleeping, the cache entry for key "user1"
///     // will be expired and evicted, so the eviction listener will be called to
///     // remove the file.
///     tokio::time::sleep(Duration::from_secs(5)).await;
///
///     cache.run_pending_tasks();
///
///     Ok(())
/// }
/// ```
///
/// ## You should avoid eviction listener to panic
///
/// It is very important to make an eviction listener closure not to panic.
/// Otherwise, the cache will stop calling the listener after a panic. This is an
/// intended behavior because the cache cannot know whether it is memory safe or not
/// to call the panicked listener again.
///
/// When a listener panics, the cache will swallow the panic and disable the
/// listener. If you want to know when a listener panics and the reason of the panic,
/// you can enable an optional `logging` feature of Moka and check error-level logs.
///
/// To enable the `logging`, do the followings:
///
/// 1. In `Cargo.toml`, add the crate feature `logging` for `moka`.
/// 2. Set the logging level for `moka` to `error` or any lower levels (`warn`,
///    `info`, ...):
///     - If you are using the `env_logger` crate, you can achieve this by setting
///       `RUST_LOG` environment variable to `moka=error`.
/// 3. If you have more than one caches, you may want to set a distinct name for each
///    cache by using cache builder's [`name`][builder-name-method] method. The name
///    will appear in the log.
///
/// [builder-name-method]: ./struct.CacheBuilder.html#method.name
///
pub struct Cache<K, V, S = RandomState> {
    pub(crate) base: BaseCache<K, V, S>,
    value_initializer: Arc<ValueInitializer<K, V, S>>,

    #[cfg(test)]
    schedule_write_op_should_block: AtomicBool,
}

unsafe impl<K, V, S> Send for Cache<K, V, S>
where
    K: Send + Sync,
    V: Send + Sync,
    S: Send,
{
}

unsafe impl<K, V, S> Sync for Cache<K, V, S>
where
    K: Send + Sync,
    V: Send + Sync,
    S: Sync,
{
}

// NOTE: We cannot do `#[derive(Clone)]` because it will add `Clone` bound to `K`.
impl<K, V, S> Clone for Cache<K, V, S> {
    /// Makes a clone of this shared cache.
    ///
    /// This operation is cheap as it only creates thread-safe reference counted
    /// pointers to the shared internal data structures.
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            value_initializer: Arc::clone(&self.value_initializer),

            #[cfg(test)]
            schedule_write_op_should_block: AtomicBool::new(
                self.schedule_write_op_should_block.load(Ordering::Acquire),
            ),
        }
    }
}

impl<K, V, S> fmt::Debug for Cache<K, V, S>
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

impl<K, V, S> Cache<K, V, S> {
    /// Returns cache’s name.
    pub fn name(&self) -> Option<&str> {
        self.base.name()
    }

    /// Returns a read-only cache policy of this cache.
    ///
    /// At this time, cache policy cannot be modified after cache creation.
    /// A future version may support to modify it.
    pub fn policy(&self) -> Policy {
        self.base.policy()
    }

    /// Returns an approximate number of entries in this cache.
    ///
    /// The value returned is _an estimate_; the actual count may differ if there are
    /// concurrent insertions or removals, or if some entries are pending removal due
    /// to expiration. This inaccuracy can be mitigated by calling
    /// `run_pending_tasks` first.
    ///
    /// # Example
    ///
    /// ```rust
    /// // Cargo.toml
    /// //
    /// // [dependencies]
    /// // moka = { version = "0.12", features = ["future"] }
    /// // tokio = { version = "1", features = ["rt-multi-thread", "macros" ] }
    /// use moka::future::Cache;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let cache = Cache::new(10);
    ///     cache.insert('n', "Netherland Dwarf").await;
    ///     cache.insert('l', "Lop Eared").await;
    ///     cache.insert('d', "Dutch").await;
    ///
    ///     // Ensure an entry exists.
    ///     assert!(cache.contains_key(&'n'));
    ///
    ///     // However, followings may print stale number zeros instead of threes.
    ///     println!("{}", cache.entry_count());   // -> 0
    ///     println!("{}", cache.weighted_size()); // -> 0
    ///
    ///     // To mitigate the inaccuracy, call `run_pending_tasks` to run pending
    ///     // internal tasks.
    ///     cache.run_pending_tasks().await;
    ///
    ///     // Followings will print the actual numbers.
    ///     println!("{}", cache.entry_count());   // -> 3
    ///     println!("{}", cache.weighted_size()); // -> 3
    /// }
    /// ```
    ///
    pub fn entry_count(&self) -> u64 {
        self.base.entry_count()
    }

    /// Returns an approximate total weighted size of entries in this cache.
    ///
    /// The value returned is _an estimate_; the actual size may differ if there are
    /// concurrent insertions or removals, or if some entries are pending removal due
    /// to expiration. This inaccuracy can be mitigated by calling
    /// `run_pending_tasks` first. See [`entry_count`](#method.entry_count) for a
    /// sample code.
    pub fn weighted_size(&self) -> u64 {
        self.base.weighted_size()
    }

    #[cfg(feature = "unstable-debug-counters")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable-debug-counters")))]
    pub async fn debug_stats(&self) -> CacheDebugStats {
        self.base.debug_stats().await
    }
}

impl<K, V> Cache<K, V, RandomState>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    /// Constructs a new `Cache<K, V>` that will store up to the `max_capacity`.
    ///
    /// To adjust various configuration knobs such as `initial_capacity` or
    /// `time_to_live`, use the [`CacheBuilder`][builder-struct].
    ///
    /// [builder-struct]: ./struct.CacheBuilder.html
    pub fn new(max_capacity: u64) -> Self {
        let build_hasher = RandomState::default();
        Self::with_everything(
            None,
            Some(max_capacity),
            None,
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

    /// Returns a [`CacheBuilder`][builder-struct], which can builds a `Cache` with
    /// various configuration knobs.
    ///
    /// [builder-struct]: ./struct.CacheBuilder.html
    pub fn builder() -> CacheBuilder<K, V, Cache<K, V, RandomState>> {
        CacheBuilder::default()
    }
}

impl<K, V, S> Cache<K, V, S>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
    S: BuildHasher + Clone + Send + Sync + 'static,
{
    // https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn with_everything(
        name: Option<String>,
        max_capacity: Option<u64>,
        initial_capacity: Option<usize>,
        build_hasher: S,
        weigher: Option<Weigher<K, V>>,
        eviction_policy: EvictionPolicy,
        eviction_listener: Option<AsyncEvictionListener<K, V>>,
        expiration_policy: ExpirationPolicy<K, V>,
        housekeeper_config: HousekeeperConfig,
        invalidator_enabled: bool,
        clock: Clock,
    ) -> Self {
        Self {
            base: BaseCache::new(
                name,
                max_capacity,
                initial_capacity,
                build_hasher.clone(),
                weigher,
                eviction_policy,
                eviction_listener,
                expiration_policy,
                housekeeper_config,
                invalidator_enabled,
                clock,
            ),
            value_initializer: Arc::new(ValueInitializer::with_hasher(build_hasher)),

            #[cfg(test)]
            schedule_write_op_should_block: Default::default(), // false
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
        self.base.contains_key_with_hash(key, self.base.hash(key))
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
    pub async fn get<Q>(&self, key: &Q) -> Option<V>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        let ignore_if = None as Option<&mut fn(&V) -> bool>;

        self.base
            .get_with_hash(key, self.base.hash(key), ignore_if, false, true)
            .await
            .map(Entry::into_value)
    }

    /// Takes a key `K` and returns an [`OwnedKeyEntrySelector`] that can be used to
    /// select or insert an entry.
    ///
    /// [`OwnedKeyEntrySelector`]: ./struct.OwnedKeyEntrySelector.html
    ///
    /// # Example
    ///
    /// ```rust
    /// // Cargo.toml
    /// //
    /// // [dependencies]
    /// // moka = { version = "0.12", features = ["future"] }
    /// // tokio = { version = "1", features = ["rt-multi-thread", "macros" ] }
    ///
    /// use moka::future::Cache;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let cache: Cache<String, u32> = Cache::new(100);
    ///     let key = "key1".to_string();
    ///
    ///     let entry = cache.entry(key.clone()).or_insert(3).await;
    ///     assert!(entry.is_fresh());
    ///     assert_eq!(entry.key(), &key);
    ///     assert_eq!(entry.into_value(), 3);
    ///
    ///     let entry = cache.entry(key).or_insert(6).await;
    ///     // Not fresh because the value was already in the cache.
    ///     assert!(!entry.is_fresh());
    ///     assert_eq!(entry.into_value(), 3);
    /// }
    /// ```
    pub fn entry(&self, key: K) -> OwnedKeyEntrySelector<'_, K, V, S>
    where
        K: Hash + Eq,
    {
        let hash = self.base.hash(&key);
        OwnedKeyEntrySelector::new(key, hash, self)
    }

    /// Takes a reference `&Q` of a key and returns an [`RefKeyEntrySelector`] that
    /// can be used to select or insert an entry.
    ///
    /// [`RefKeyEntrySelector`]: ./struct.RefKeyEntrySelector.html
    ///
    /// # Example
    ///
    /// ```rust
    /// // Cargo.toml
    /// //
    /// // [dependencies]
    /// // moka = { version = "0.12", features = ["future"] }
    /// // tokio = { version = "1", features = ["rt-multi-thread", "macros" ] }
    ///
    /// use moka::future::Cache;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let cache: Cache<String, u32> = Cache::new(100);
    ///     let key = "key1".to_string();
    ///
    ///     let entry = cache.entry_by_ref(&key).or_insert(3).await;
    ///     assert!(entry.is_fresh());
    ///     assert_eq!(entry.key(), &key);
    ///     assert_eq!(entry.into_value(), 3);
    ///
    ///     let entry = cache.entry_by_ref(&key).or_insert(6).await;
    ///     // Not fresh because the value was already in the cache.
    ///     assert!(!entry.is_fresh());
    ///     assert_eq!(entry.into_value(), 3);
    /// }
    /// ```
    pub fn entry_by_ref<'a, Q>(&'a self, key: &'a Q) -> RefKeyEntrySelector<'a, K, Q, V, S>
    where
        Q: Equivalent<K> + ToOwned<Owned = K> + Hash + ?Sized,
    {
        let hash = self.base.hash(key);
        RefKeyEntrySelector::new(key, hash, self)
    }

    /// Returns a _clone_ of the value corresponding to the key. If the value does
    /// not exist, resolve the `init` future and inserts the output.
    ///
    /// # Concurrent calls on the same key
    ///
    /// This method guarantees that concurrent calls on the same not-existing key are
    /// coalesced into one evaluation of the `init` future. Only one of the calls
    /// evaluates its future, and other calls wait for that future to resolve.
    ///
    /// The following code snippet demonstrates this behavior:
    ///
    /// ```rust
    /// // Cargo.toml
    /// //
    /// // [dependencies]
    /// // moka = { version = "0.12", features = ["future"] }
    /// // futures-util = "0.3"
    /// // tokio = { version = "1", features = ["rt-multi-thread", "macros" ] }
    /// use moka::future::Cache;
    /// use std::sync::Arc;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     const TEN_MIB: usize = 10 * 1024 * 1024; // 10MiB
    ///     let cache = Cache::new(100);
    ///
    ///     // Spawn four async tasks.
    ///     let tasks: Vec<_> = (0..4_u8)
    ///         .map(|task_id| {
    ///             let my_cache = cache.clone();
    ///             tokio::spawn(async move {
    ///                 println!("Task {task_id} started.");
    ///
    ///                 // Insert and get the value for key1. Although all four async
    ///                 // tasks will call `get_with` at the same time, the `init`
    ///                 // async block must be resolved only once.
    ///                 let value = my_cache
    ///                     .get_with("key1", async move {
    ///                         println!("Task {task_id} inserting a value.");
    ///                         Arc::new(vec![0u8; TEN_MIB])
    ///                     })
    ///                     .await;
    ///
    ///                 // Ensure the value exists now.
    ///                 assert_eq!(value.len(), TEN_MIB);
    ///                 assert!(my_cache.get(&"key1").await.is_some());
    ///
    ///                 println!("Task {task_id} got the value. (len: {})", value.len());
    ///             })
    ///         })
    ///         .collect();
    ///
    ///     // Run all tasks concurrently and wait for them to complete.
    ///     futures_util::future::join_all(tasks).await;
    /// }
    /// ```
    ///
    /// **A Sample Result**
    ///
    /// - The `init` future (async black) was resolved exactly once by task 3.
    /// - Other tasks were blocked until task 3 inserted the value.
    ///
    /// ```console
    /// Task 0 started.
    /// Task 3 started.
    /// Task 1 started.
    /// Task 2 started.
    /// Task 3 inserting a value.
    /// Task 3 got the value. (len: 10485760)
    /// Task 0 got the value. (len: 10485760)
    /// Task 1 got the value. (len: 10485760)
    /// Task 2 got the value. (len: 10485760)
    /// ```
    ///
    /// # Panics
    ///
    /// This method panics when the `init` future has panicked. When it happens, only
    /// the caller whose `init` future panicked will get the panic (e.g. only task 3
    /// in the above sample). If there are other calls in progress (e.g. task 0, 1
    /// and 2 above), this method will restart and resolve one of the remaining
    /// `init` futures.
    ///
    pub async fn get_with(&self, key: K, init: impl Future<Output = V>) -> V {
        futures_util::pin_mut!(init);
        let hash = self.base.hash(&key);
        let key = Arc::new(key);
        let replace_if = None as Option<fn(&V) -> bool>;
        self.get_or_insert_with_hash_and_fun(key, hash, init, replace_if, false)
            .await
            .into_value()
    }

    /// Similar to [`get_with`](#method.get_with), but instead of passing an owned
    /// key, you can pass a reference to the key. If the key does not exist in the
    /// cache, the key will be cloned to create new entry in the cache.
    pub async fn get_with_by_ref<Q>(&self, key: &Q, init: impl Future<Output = V>) -> V
    where
        Q: Equivalent<K> + ToOwned<Owned = K> + Hash + ?Sized,
    {
        futures_util::pin_mut!(init);
        let hash = self.base.hash(key);
        let replace_if = None as Option<fn(&V) -> bool>;
        self.get_or_insert_with_hash_by_ref_and_fun(key, hash, init, replace_if, false)
            .await
            .into_value()
    }

    /// TODO: Remove this in v0.13.0.
    /// Deprecated, replaced with
    /// [`entry()::or_insert_with_if()`](./struct.OwnedKeyEntrySelector.html#method.or_insert_with_if)
    #[deprecated(since = "0.10.0", note = "Replaced with `entry().or_insert_with_if()`")]
    pub async fn get_with_if(
        &self,
        key: K,
        init: impl Future<Output = V>,
        replace_if: impl FnMut(&V) -> bool + Send,
    ) -> V {
        futures_util::pin_mut!(init);
        let hash = self.base.hash(&key);
        let key = Arc::new(key);
        self.get_or_insert_with_hash_and_fun(key, hash, init, Some(replace_if), false)
            .await
            .into_value()
    }

    /// Returns a _clone_ of the value corresponding to the key. If the value does
    /// not exist, resolves the `init` future, and inserts the value if `Some(value)`
    /// was returned. If `None` was returned from the future, this method does not
    /// insert a value and returns `None`.
    ///
    /// # Concurrent calls on the same key
    ///
    /// This method guarantees that concurrent calls on the same not-existing key are
    /// coalesced into one evaluation of the `init` future. Only one of the calls
    /// evaluates its future, and other calls wait for that future to resolve.
    ///
    /// The following code snippet demonstrates this behavior:
    ///
    /// ```rust
    /// // Cargo.toml
    /// //
    /// // [dependencies]
    /// // moka = { version = "0.12", features = ["future"] }
    /// // futures-util = "0.3"
    /// // reqwest = "0.11"
    /// // tokio = { version = "1", features = ["rt-multi-thread", "macros" ] }
    /// use moka::future::Cache;
    ///
    /// // This async function tries to get HTML from the given URI.
    /// async fn get_html(task_id: u8, uri: &str) -> Option<String> {
    ///     println!("get_html() called by task {task_id}.");
    ///     reqwest::get(uri).await.ok()?.text().await.ok()
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let cache = Cache::new(100);
    ///
    ///     // Spawn four async tasks.
    ///     let tasks: Vec<_> = (0..4_u8)
    ///         .map(|task_id| {
    ///             let my_cache = cache.clone();
    ///             tokio::spawn(async move {
    ///                 println!("Task {task_id} started.");
    ///
    ///                 // Try to insert and get the value for key1. Although
    ///                 // all four async tasks will call `try_get_with`
    ///                 // at the same time, get_html() must be called only once.
    ///                 let value = my_cache
    ///                     .optionally_get_with(
    ///                         "key1",
    ///                         get_html(task_id, "https://www.rust-lang.org"),
    ///                     ).await;
    ///
    ///                 // Ensure the value exists now.
    ///                 assert!(value.is_some());
    ///                 assert!(my_cache.get(&"key1").await.is_some());
    ///
    ///                 println!(
    ///                     "Task {task_id} got the value. (len: {})",
    ///                     value.unwrap().len()
    ///                 );
    ///             })
    ///         })
    ///         .collect();
    ///
    ///     // Run all tasks concurrently and wait for them to complete.
    ///     futures_util::future::join_all(tasks).await;
    /// }
    /// ```
    ///
    /// **A Sample Result**
    ///
    /// - `get_html()` was called exactly once by task 2.
    /// - Other tasks were blocked until task 2 inserted the value.
    ///
    /// ```console
    /// Task 1 started.
    /// Task 0 started.
    /// Task 2 started.
    /// Task 3 started.
    /// get_html() called by task 2.
    /// Task 2 got the value. (len: 19419)
    /// Task 1 got the value. (len: 19419)
    /// Task 0 got the value. (len: 19419)
    /// Task 3 got the value. (len: 19419)
    /// ```
    ///
    /// # Panics
    ///
    /// This method panics when the `init` future has panicked. When it happens, only
    /// the caller whose `init` future panicked will get the panic (e.g. only task 2
    /// in the above sample). If there are other calls in progress (e.g. task 0, 1
    /// and 3 above), this method will restart and resolve one of the remaining
    /// `init` futures.
    ///
    pub async fn optionally_get_with<F>(&self, key: K, init: F) -> Option<V>
    where
        F: Future<Output = Option<V>>,
    {
        futures_util::pin_mut!(init);
        let hash = self.base.hash(&key);
        let key = Arc::new(key);
        self.get_or_optionally_insert_with_hash_and_fun(key, hash, init, false)
            .await
            .map(Entry::into_value)
    }

    /// Similar to [`optionally_get_with`](#method.optionally_get_with), but instead
    /// of passing an owned key, you can pass a reference to the key. If the key does
    /// not exist in the cache, the key will be cloned to create new entry in the
    /// cache.
    pub async fn optionally_get_with_by_ref<F, Q>(&self, key: &Q, init: F) -> Option<V>
    where
        F: Future<Output = Option<V>>,
        Q: Equivalent<K> + ToOwned<Owned = K> + Hash + ?Sized,
    {
        futures_util::pin_mut!(init);
        let hash = self.base.hash(key);
        self.get_or_optionally_insert_with_hash_by_ref_and_fun(key, hash, init, false)
            .await
            .map(Entry::into_value)
    }

    /// Returns a _clone_ of the value corresponding to the key. If the value does
    /// not exist, resolves the `init` future, and inserts the value if `Ok(value)`
    /// was returned. If `Err(_)` was returned from the future, this method does not
    /// insert a value and returns the `Err` wrapped by [`std::sync::Arc`][std-arc].
    ///
    /// [std-arc]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html
    ///
    /// # Concurrent calls on the same key
    ///
    /// This method guarantees that concurrent calls on the same not-existing key are
    /// coalesced into one evaluation of the `init` future (as long as these
    /// futures return the same error type). Only one of the calls evaluates its
    /// future, and other calls wait for that future to resolve.
    ///
    /// The following code snippet demonstrates this behavior:
    ///
    /// ```rust
    /// // Cargo.toml
    /// //
    /// // [dependencies]
    /// // moka = { version = "0.12", features = ["future"] }
    /// // futures-util = "0.3"
    /// // reqwest = "0.11"
    /// // tokio = { version = "1", features = ["rt-multi-thread", "macros" ] }
    /// use moka::future::Cache;
    ///
    /// // This async function tries to get HTML from the given URI.
    /// async fn get_html(task_id: u8, uri: &str) -> Result<String, reqwest::Error> {
    ///     println!("get_html() called by task {task_id}.");
    ///     reqwest::get(uri).await?.text().await
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let cache = Cache::new(100);
    ///
    ///     // Spawn four async tasks.
    ///     let tasks: Vec<_> = (0..4_u8)
    ///         .map(|task_id| {
    ///             let my_cache = cache.clone();
    ///             tokio::spawn(async move {
    ///                 println!("Task {task_id} started.");
    ///
    ///                 // Try to insert and get the value for key1. Although
    ///                 // all four async tasks will call `try_get_with`
    ///                 // at the same time, get_html() must be called only once.
    ///                 let value = my_cache
    ///                     .try_get_with(
    ///                         "key1",
    ///                         get_html(task_id, "https://www.rust-lang.org"),
    ///                     ).await;
    ///
    ///                 // Ensure the value exists now.
    ///                 assert!(value.is_ok());
    ///                 assert!(my_cache.get(&"key1").await.is_some());
    ///
    ///                 println!(
    ///                     "Task {task_id} got the value. (len: {})",
    ///                     value.unwrap().len()
    ///                 );
    ///             })
    ///         })
    ///         .collect();
    ///
    ///     // Run all tasks concurrently and wait for them to complete.
    ///     futures_util::future::join_all(tasks).await;
    /// }
    /// ```
    ///
    /// **A Sample Result**
    ///
    /// - `get_html()` was called exactly once by task 2.
    /// - Other tasks were blocked until task 2 inserted the value.
    ///
    /// ```console
    /// Task 1 started.
    /// Task 0 started.
    /// Task 2 started.
    /// Task 3 started.
    /// get_html() called by task 2.
    /// Task 2 got the value. (len: 19419)
    /// Task 1 got the value. (len: 19419)
    /// Task 0 got the value. (len: 19419)
    /// Task 3 got the value. (len: 19419)
    /// ```
    ///
    /// # Panics
    ///
    /// This method panics when the `init` future has panicked. When it happens, only
    /// the caller whose `init` future panicked will get the panic (e.g. only task 2
    /// in the above sample). If there are other calls in progress (e.g. task 0, 1
    /// and 3 above), this method will restart and resolve one of the remaining
    /// `init` futures.
    ///
    pub async fn try_get_with<F, E>(&self, key: K, init: F) -> Result<V, Arc<E>>
    where
        F: Future<Output = Result<V, E>>,
        E: Send + Sync + 'static,
    {
        futures_util::pin_mut!(init);
        let hash = self.base.hash(&key);
        let key = Arc::new(key);
        self.get_or_try_insert_with_hash_and_fun(key, hash, init, false)
            .await
            .map(Entry::into_value)
    }

    /// Similar to [`try_get_with`](#method.try_get_with), but instead of passing an
    /// owned key, you can pass a reference to the key. If the key does not exist in
    /// the cache, the key will be cloned to create new entry in the cache.
    pub async fn try_get_with_by_ref<F, E, Q>(&self, key: &Q, init: F) -> Result<V, Arc<E>>
    where
        F: Future<Output = Result<V, E>>,
        E: Send + Sync + 'static,
        Q: Equivalent<K> + ToOwned<Owned = K> + Hash + ?Sized,
    {
        futures_util::pin_mut!(init);
        let hash = self.base.hash(key);
        self.get_or_try_insert_with_hash_by_ref_and_fun(key, hash, init, false)
            .await
            .map(Entry::into_value)
    }

    /// Inserts a key-value pair into the cache.
    ///
    /// If the cache has this key present, the value is updated.
    pub async fn insert(&self, key: K, value: V) {
        let hash = self.base.hash(&key);
        let key = Arc::new(key);
        self.insert_with_hash(key, hash, value).await;
    }

    /// Discards any cached value for the key.
    ///
    /// If you need to get the value that has been discarded, use the
    /// [`remove`](#method.remove) method instead.
    ///
    /// The key may be any borrowed form of the cache's key type, but `Hash` and `Eq`
    /// on the borrowed form _must_ match those for the key type.
    pub async fn invalidate<Q>(&self, key: &Q)
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        let hash = self.base.hash(key);
        self.invalidate_with_hash(key, hash, false).await;
    }

    /// Discards any cached value for the key and returns a _clone_ of the value.
    ///
    /// If you do not need to get the value that has been discarded, use the
    /// [`invalidate`](#method.invalidate) method instead.
    ///
    /// The key may be any borrowed form of the cache's key type, but `Hash` and `Eq`
    /// on the borrowed form _must_ match those for the key type.
    pub async fn remove<Q>(&self, key: &Q) -> Option<V>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        let hash = self.base.hash(key);
        self.invalidate_with_hash(key, hash, true).await
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
        self.base.invalidate_all();
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
    pub fn invalidate_entries_if<F>(&self, predicate: F) -> Result<PredicateId, PredicateError>
    where
        F: Fn(&K, &V) -> bool + Send + Sync + 'static,
    {
        self.base.invalidate_entries_if(Arc::new(predicate))
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
    /// // Cargo.toml
    /// //
    /// // [dependencies]
    /// // moka = { version = "0.12", features = ["future"] }
    /// // tokio = { version = "1", features = ["rt-multi-thread", "macros" ] }
    /// use moka::future::Cache;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let cache = Cache::new(100);
    ///     cache.insert("Julia", 14).await;
    ///
    ///     let mut iter = cache.iter();
    ///     let (k, v) = iter.next().unwrap(); // (Arc<K>, V)
    ///     assert_eq!(*k, "Julia");
    ///     assert_eq!(v, 14);
    ///
    ///     assert!(iter.next().is_none());
    /// }
    /// ```
    ///
    pub fn iter(&self) -> Iter<'_, K, V> {
        use crate::common::iter::{Iter as InnerIter, ScanningGet};

        let inner = InnerIter::with_single_cache_segment(&self.base, self.base.num_cht_segments());
        Iter::new(inner)
    }

    /// Performs any pending maintenance operations needed by the cache.
    pub async fn run_pending_tasks(&self) {
        if let Some(hk) = &self.base.housekeeper {
            self.base.retry_interrupted_ops().await;
            hk.run_pending_tasks(Arc::clone(&self.base.inner)).await;
        }
    }
}

impl<'a, K, V, S> IntoIterator for &'a Cache<K, V, S>
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

//
// private methods
//
impl<K, V, S> Cache<K, V, S>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
    S: BuildHasher + Clone + Send + Sync + 'static,
{
    pub(crate) async fn get_or_insert_with_hash_and_fun(
        &self,
        key: Arc<K>,
        hash: u64,
        init: Pin<&mut impl Future<Output = V>>,
        mut replace_if: Option<impl FnMut(&V) -> bool + Send>,
        need_key: bool,
    ) -> Entry<K, V> {
        let maybe_entry = self
            .base
            .get_with_hash(&*key, hash, replace_if.as_mut(), need_key, true)
            .await;
        if let Some(entry) = maybe_entry {
            entry
        } else {
            self.insert_with_hash_and_fun(key, hash, init, replace_if, need_key)
                .await
        }
    }

    pub(crate) async fn get_or_insert_with_hash_by_ref_and_fun<Q>(
        &self,
        key: &Q,
        hash: u64,
        init: Pin<&mut impl Future<Output = V>>,
        mut replace_if: Option<impl FnMut(&V) -> bool + Send>,
        need_key: bool,
    ) -> Entry<K, V>
    where
        Q: Equivalent<K> + ToOwned<Owned = K> + Hash + ?Sized,
    {
        let maybe_entry = self
            .base
            .get_with_hash(key, hash, replace_if.as_mut(), need_key, true)
            .await;
        if let Some(entry) = maybe_entry {
            entry
        } else {
            let key = Arc::new(key.to_owned());
            self.insert_with_hash_and_fun(key, hash, init, replace_if, need_key)
                .await
        }
    }

    async fn insert_with_hash_and_fun(
        &self,
        key: Arc<K>,
        hash: u64,
        init: Pin<&mut impl Future<Output = V>>,
        replace_if: Option<impl FnMut(&V) -> bool + Send>,
        need_key: bool,
    ) -> Entry<K, V> {
        let k = if need_key {
            Some(Arc::clone(&key))
        } else {
            None
        };

        let type_id = ValueInitializer::<K, V, S>::type_id_for_get_with();
        let post_init = ValueInitializer::<K, V, S>::post_init_for_get_with;

        match self
            .value_initializer
            .try_init_or_read(&key, hash, type_id, self, replace_if, init, post_init)
            .await
        {
            InitResult::Initialized(v) => {
                crossbeam_epoch::pin().flush();
                Entry::new(k, v, true, false)
            }
            InitResult::ReadExisting(v) => Entry::new(k, v, false, false),
            InitResult::InitErr(_) => unreachable!(),
        }
    }

    pub(crate) async fn get_or_insert_with_hash(
        &self,
        key: Arc<K>,
        hash: u64,
        init: impl FnOnce() -> V,
    ) -> Entry<K, V> {
        match self
            .base
            .get_with_hash(&*key, hash, never_ignore(), true, true)
            .await
        {
            Some(entry) => entry,
            None => {
                let value = init();
                self.insert_with_hash(Arc::clone(&key), hash, value.clone())
                    .await;
                Entry::new(Some(key), value, true, false)
            }
        }
    }

    pub(crate) async fn get_or_insert_with_hash_by_ref<Q>(
        &self,
        key: &Q,
        hash: u64,
        init: impl FnOnce() -> V,
    ) -> Entry<K, V>
    where
        Q: Equivalent<K> + ToOwned<Owned = K> + Hash + ?Sized,
    {
        match self
            .base
            .get_with_hash(key, hash, never_ignore(), true, true)
            .await
        {
            Some(entry) => entry,
            None => {
                let key = Arc::new(key.to_owned());
                let value = init();
                self.insert_with_hash(Arc::clone(&key), hash, value.clone())
                    .await;
                Entry::new(Some(key), value, true, false)
            }
        }
    }

    pub(crate) async fn get_or_optionally_insert_with_hash_and_fun<F>(
        &self,
        key: Arc<K>,
        hash: u64,
        init: Pin<&mut F>,
        need_key: bool,
    ) -> Option<Entry<K, V>>
    where
        F: Future<Output = Option<V>>,
    {
        let entry = self
            .base
            .get_with_hash(&*key, hash, never_ignore(), need_key, true)
            .await;
        if entry.is_some() {
            return entry;
        }

        self.optionally_insert_with_hash_and_fun(key, hash, init, need_key)
            .await
    }

    pub(crate) async fn get_or_optionally_insert_with_hash_by_ref_and_fun<F, Q>(
        &self,
        key: &Q,
        hash: u64,
        init: Pin<&mut F>,
        need_key: bool,
    ) -> Option<Entry<K, V>>
    where
        F: Future<Output = Option<V>>,
        Q: Equivalent<K> + ToOwned<Owned = K> + Hash + ?Sized,
    {
        let entry = self
            .base
            .get_with_hash(key, hash, never_ignore(), need_key, true)
            .await;
        if entry.is_some() {
            return entry;
        }

        let key = Arc::new(key.to_owned());
        self.optionally_insert_with_hash_and_fun(key, hash, init, need_key)
            .await
    }

    async fn optionally_insert_with_hash_and_fun<F>(
        &self,
        key: Arc<K>,
        hash: u64,
        init: Pin<&mut F>,
        need_key: bool,
    ) -> Option<Entry<K, V>>
    where
        F: Future<Output = Option<V>>,
    {
        let k = if need_key {
            Some(Arc::clone(&key))
        } else {
            None
        };

        let type_id = ValueInitializer::<K, V, S>::type_id_for_optionally_get_with();
        let post_init = ValueInitializer::<K, V, S>::post_init_for_optionally_get_with;

        match self
            .value_initializer
            .try_init_or_read(&key, hash, type_id, self, never_ignore(), init, post_init)
            .await
        {
            InitResult::Initialized(v) => {
                crossbeam_epoch::pin().flush();
                Some(Entry::new(k, v, true, false))
            }
            InitResult::ReadExisting(v) => Some(Entry::new(k, v, false, false)),
            InitResult::InitErr(_) => None,
        }
    }

    pub(super) async fn get_or_try_insert_with_hash_and_fun<F, E>(
        &self,
        key: Arc<K>,
        hash: u64,
        init: Pin<&mut F>,
        need_key: bool,
    ) -> Result<Entry<K, V>, Arc<E>>
    where
        F: Future<Output = Result<V, E>>,
        E: Send + Sync + 'static,
    {
        if let Some(entry) = self
            .base
            .get_with_hash(&*key, hash, never_ignore(), need_key, true)
            .await
        {
            return Ok(entry);
        }

        self.try_insert_with_hash_and_fun(key, hash, init, need_key)
            .await
    }

    pub(super) async fn get_or_try_insert_with_hash_by_ref_and_fun<F, E, Q>(
        &self,
        key: &Q,
        hash: u64,
        init: Pin<&mut F>,
        need_key: bool,
    ) -> Result<Entry<K, V>, Arc<E>>
    where
        F: Future<Output = Result<V, E>>,
        E: Send + Sync + 'static,
        Q: Equivalent<K> + ToOwned<Owned = K> + Hash + ?Sized,
    {
        if let Some(entry) = self
            .base
            .get_with_hash(key, hash, never_ignore(), need_key, true)
            .await
        {
            return Ok(entry);
        }
        let key = Arc::new(key.to_owned());
        self.try_insert_with_hash_and_fun(key, hash, init, need_key)
            .await
    }

    async fn try_insert_with_hash_and_fun<F, E>(
        &self,
        key: Arc<K>,
        hash: u64,
        init: Pin<&mut F>,
        need_key: bool,
    ) -> Result<Entry<K, V>, Arc<E>>
    where
        F: Future<Output = Result<V, E>>,
        E: Send + Sync + 'static,
    {
        let k = if need_key {
            Some(Arc::clone(&key))
        } else {
            None
        };

        let type_id = ValueInitializer::<K, V, S>::type_id_for_try_get_with::<E>();
        let post_init = ValueInitializer::<K, V, S>::post_init_for_try_get_with;

        match self
            .value_initializer
            .try_init_or_read(&key, hash, type_id, self, never_ignore(), init, post_init)
            .await
        {
            InitResult::Initialized(v) => {
                crossbeam_epoch::pin().flush();
                Ok(Entry::new(k, v, true, false))
            }
            InitResult::ReadExisting(v) => Ok(Entry::new(k, v, false, false)),
            InitResult::InitErr(e) => {
                crossbeam_epoch::pin().flush();
                Err(e)
            }
        }
    }

    pub(crate) async fn insert_with_hash(&self, key: Arc<K>, hash: u64, value: V) {
        if self.base.is_map_disabled() {
            return;
        }

        let (op, ts) = self.base.do_insert_with_hash(key, hash, value).await;
        let mut cancel_guard = CancelGuard::new(&self.base.interrupted_op_ch_snd, ts);
        cancel_guard.set_op(op.clone());

        let should_block;
        #[cfg(not(test))]
        {
            should_block = false;
        }
        #[cfg(test)]
        {
            should_block = self.schedule_write_op_should_block.load(Ordering::Acquire);
        }

        let hk = self.base.housekeeper.as_ref();
        let event = self.base.write_op_ch_ready_event();

        BaseCache::<K, V, S>::schedule_write_op(
            &self.base.inner,
            &self.base.write_op_ch,
            event,
            op,
            ts,
            hk,
            should_block,
        )
        .await
        .expect("Failed to schedule write op for insert");
        cancel_guard.clear();
    }

    pub(crate) async fn compute_with_hash_and_fun<F, Fut>(
        &self,
        key: Arc<K>,
        hash: u64,
        f: F,
    ) -> compute::CompResult<K, V>
    where
        F: FnOnce(Option<Entry<K, V>>) -> Fut,
        Fut: Future<Output = compute::Op<V>>,
    {
        let post_init = ValueInitializer::<K, V, S>::post_init_for_compute_with;
        match self
            .value_initializer
            .try_compute(key, hash, self, f, post_init, true)
            .await
        {
            Ok(result) => result,
            Err(_) => unreachable!(),
        }
    }

    pub(crate) async fn try_compute_with_hash_and_fun<F, Fut, E>(
        &self,
        key: Arc<K>,
        hash: u64,
        f: F,
    ) -> Result<compute::CompResult<K, V>, E>
    where
        F: FnOnce(Option<Entry<K, V>>) -> Fut,
        Fut: Future<Output = Result<compute::Op<V>, E>>,
        E: Send + Sync + 'static,
    {
        let post_init = ValueInitializer::<K, V, S>::post_init_for_try_compute_with;
        self.value_initializer
            .try_compute(key, hash, self, f, post_init, true)
            .await
    }

    pub(crate) async fn try_compute_if_nobody_else_with_hash_and_fun<F, Fut, E>(
        &self,
        key: Arc<K>,
        hash: u64,
        f: F,
    ) -> Result<compute::CompResult<K, V>, E>
    where
        F: FnOnce(Option<Entry<K, V>>) -> Fut,
        Fut: Future<Output = Result<compute::Op<V>, E>>,
        E: Send + Sync + 'static,
    {
        let post_init = ValueInitializer::<K, V, S>::post_init_for_try_compute_with_if_nobody_else;
        self.value_initializer
            .try_compute_if_nobody_else(key, hash, self, f, post_init, true)
            .await
    }

    pub(crate) async fn upsert_with_hash_and_fun<F, Fut>(
        &self,
        key: Arc<K>,
        hash: u64,
        f: F,
    ) -> Entry<K, V>
    where
        F: FnOnce(Option<Entry<K, V>>) -> Fut,
        Fut: Future<Output = V>,
    {
        let post_init = ValueInitializer::<K, V, S>::post_init_for_upsert_with;
        match self
            .value_initializer
            .try_compute(key, hash, self, f, post_init, false)
            .await
        {
            Ok(CompResult::Inserted(entry) | CompResult::ReplacedWith(entry)) => entry,
            _ => unreachable!(),
        }
    }

    pub(crate) async fn invalidate_with_hash<Q>(
        &self,
        key: &Q,
        hash: u64,
        need_value: bool,
    ) -> Option<V>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        use futures_util::FutureExt;

        self.base.retry_interrupted_ops().await;

        // Lock the key for removal if blocking removal notification is enabled.
        let mut kl = None;
        let mut klg = None;
        if self.base.is_removal_notifier_enabled() {
            // To lock the key, we have to get Arc<K> for key (&Q).
            //
            // TODO: Enhance this if possible. This is rather hack now because
            // it cannot prevent race conditions like this:
            //
            // 1. We miss the key because it does not exist. So we do not lock
            //    the key.
            // 2. Somebody else (other thread) inserts the key.
            // 3. We remove the entry for the key, but without the key lock!
            //
            if let Some(arc_key) = self.base.get_key_with_hash(key, hash) {
                kl = self.base.maybe_key_lock(&arc_key);
                klg = if let Some(lock) = &kl {
                    Some(lock.lock().await)
                } else {
                    None
                };
            }
        }

        match self.base.remove_entry(key, hash) {
            None => None,
            Some(kv) => {
                let now = self.base.current_time();

                let maybe_v = if need_value {
                    Some(kv.entry.value.clone())
                } else {
                    None
                };

                let info = kv.entry.entry_info();
                let entry_gen = info.incr_entry_gen();

                let op: WriteOp<K, V> = WriteOp::Remove {
                    kv_entry: kv.clone(),
                    entry_gen,
                };

                // Async Cancellation Safety: To ensure the below future should be
                // executed even if our caller async task is cancelled, we create a
                // cancel guard for the future (and the op). If our caller is
                // cancelled while we are awaiting for the future, the cancel guard
                // will save the future and the op to the interrupted_op_ch channel,
                // so that we can resume/retry later.
                let mut cancel_guard = CancelGuard::new(&self.base.interrupted_op_ch_snd, now);

                if self.base.is_removal_notifier_enabled() {
                    let future = self
                        .base
                        .notify_invalidate(&kv.key, &kv.entry)
                        .boxed()
                        .shared();
                    cancel_guard.set_future_and_op(future.clone(), op.clone());
                    // Send notification to the eviction listener.
                    future.await;
                    cancel_guard.unset_future();
                } else {
                    cancel_guard.set_op(op.clone());
                }

                // Drop the locks before scheduling write op to avoid a potential
                // dead lock. (Scheduling write can do spin lock when the queue is
                // full, and queue will be drained by the housekeeping thread that
                // can lock the same key)
                std::mem::drop(klg);
                std::mem::drop(kl);

                let should_block;
                #[cfg(not(test))]
                {
                    should_block = false;
                }
                #[cfg(test)]
                {
                    should_block = self.schedule_write_op_should_block.load(Ordering::Acquire);
                }

                let event = self.base.write_op_ch_ready_event();
                let hk = self.base.housekeeper.as_ref();

                BaseCache::<K, V, S>::schedule_write_op(
                    &self.base.inner,
                    &self.base.write_op_ch,
                    event,
                    op,
                    now,
                    hk,
                    should_block,
                )
                .await
                .expect("Failed to schedule write op for remove");
                cancel_guard.clear();

                crossbeam_epoch::pin().flush();
                maybe_v
            }
        }
    }
}

// For unit tests.
// For unit tests.
#[cfg(test)]
impl<K, V, S> Cache<K, V, S> {
    pub(crate) fn is_table_empty(&self) -> bool {
        self.entry_count() == 0
    }

    pub(crate) fn is_waiter_map_empty(&self) -> bool {
        self.value_initializer.waiter_count() == 0
    }
}

#[cfg(test)]
impl<K, V, S> Cache<K, V, S>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
    S: BuildHasher + Clone + Send + Sync + 'static,
{
    fn invalidation_predicate_count(&self) -> usize {
        self.base.invalidation_predicate_count()
    }

    async fn reconfigure_for_testing(&mut self) {
        self.base.reconfigure_for_testing().await;
    }

    fn key_locks_map_is_empty(&self) -> bool {
        self.base.key_locks_map_is_empty()
    }

    fn run_pending_tasks_initiation_count(&self) -> usize {
        self.base
            .housekeeper
            .as_ref()
            .map(|hk| hk.start_count.load(Ordering::Acquire))
            .expect("housekeeper is not set")
    }

    fn run_pending_tasks_completion_count(&self) -> usize {
        self.base
            .housekeeper
            .as_ref()
            .map(|hk| hk.complete_count.load(Ordering::Acquire))
            .expect("housekeeper is not set")
    }
}

// AS of Rust 1.71, we cannot make this function into a `const fn` because mutable
// references are not allowed.
// See [#57349](https://github.com/rust-lang/rust/issues/57349).
#[inline]
fn never_ignore<'a, V>() -> Option<&'a mut fn(&V) -> bool> {
    None
}

// To see the debug prints, run test as `cargo test -- --nocapture`
#[cfg(test)]
mod tests {
    use super::Cache;
    use crate::{
        common::{time::Clock, HousekeeperConfig},
        future::FutureExt,
        notification::{ListenerFuture, RemovalCause},
        ops::compute,
        policy::{test_utils::ExpiryCallCounters, EvictionPolicy},
        Expiry,
    };

    use async_lock::{Barrier, Mutex};
    use std::{
        convert::Infallible,
        sync::{
            atomic::{AtomicU32, AtomicU8, Ordering},
            Arc,
        },
        time::{Duration, Instant as StdInstant},
        vec,
    };
    use tokio::time::sleep;

    #[test]
    fn futures_are_send() {
        let cache = Cache::new(0);

        fn is_send(_: impl Send) {}

        // pub fns
        is_send(cache.get(&()));
        is_send(cache.get_with((), async {}));
        is_send(cache.get_with_by_ref(&(), async {}));
        #[allow(deprecated)]
        is_send(cache.get_with_if((), async {}, |_| false));
        is_send(cache.insert((), ()));
        is_send(cache.invalidate(&()));
        is_send(cache.optionally_get_with((), async { None }));
        is_send(cache.optionally_get_with_by_ref(&(), async { None }));
        is_send(cache.remove(&()));
        is_send(cache.run_pending_tasks());
        is_send(cache.try_get_with((), async { Err(()) }));
        is_send(cache.try_get_with_by_ref(&(), async { Err(()) }));

        // entry fns
        is_send(
            cache
                .entry(())
                .and_compute_with(|_| async { compute::Op::Nop }),
        );
        is_send(
            cache
                .entry(())
                .and_try_compute_with(|_| async { Ok(compute::Op::Nop) as Result<_, Infallible> }),
        );
        is_send(cache.entry(()).and_upsert_with(|_| async {}));
        is_send(cache.entry(()).or_default());
        is_send(cache.entry(()).or_insert(()));
        is_send(cache.entry(()).or_insert_with(async {}));
        is_send(cache.entry(()).or_insert_with_if(async {}, |_| false));
        is_send(cache.entry(()).or_optionally_insert_with(async { None }));
        is_send(cache.entry(()).or_try_insert_with(async { Err(()) }));

        // entry_by_ref fns
        is_send(
            cache
                .entry_by_ref(&())
                .and_compute_with(|_| async { compute::Op::Nop }),
        );
        is_send(
            cache
                .entry_by_ref(&())
                .and_try_compute_with(|_| async { Ok(compute::Op::Nop) as Result<_, Infallible> }),
        );
        is_send(cache.entry_by_ref(&()).and_upsert_with(|_| async {}));
        is_send(cache.entry_by_ref(&()).or_default());
        is_send(cache.entry_by_ref(&()).or_insert(()));
        is_send(cache.entry_by_ref(&()).or_insert_with(async {}));
        is_send(
            cache
                .entry_by_ref(&())
                .or_insert_with_if(async {}, |_| false),
        );
        is_send(
            cache
                .entry_by_ref(&())
                .or_optionally_insert_with(async { None }),
        );
        is_send(
            cache
                .entry_by_ref(&())
                .or_try_insert_with(async { Err(()) }),
        );
    }

    #[tokio::test]
    async fn max_capacity_zero() {
        let mut cache = Cache::new(0);
        cache.reconfigure_for_testing().await;

        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert(0, ()).await;

        assert!(!cache.contains_key(&0));
        assert!(cache.get(&0).await.is_none());
        cache.run_pending_tasks().await;
        assert!(!cache.contains_key(&0));
        assert!(cache.get(&0).await.is_none());
        assert_eq!(cache.entry_count(), 0)
    }

    #[tokio::test]
    async fn basic_single_async_task() {
        // The following `Vec`s will hold actual and expected notifications.
        let actual = Arc::new(Mutex::new(Vec::new()));
        let mut expected = Vec::new();

        // Create an eviction listener.
        let a1 = Arc::clone(&actual);
        let listener = move |k, v, cause| -> ListenerFuture {
            let a2 = Arc::clone(&a1);
            async move {
                a2.lock().await.push((k, v, cause));
            }
            .boxed()
        };

        // Create a cache with the eviction listener.
        let mut cache = Cache::builder()
            .max_capacity(3)
            .async_eviction_listener(listener)
            .build();
        cache.reconfigure_for_testing().await;

        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert("a", "alice").await;
        cache.insert("b", "bob").await;
        assert_eq!(cache.get(&"a").await, Some("alice"));
        assert!(cache.contains_key(&"a"));
        assert!(cache.contains_key(&"b"));
        assert_eq!(cache.get(&"b").await, Some("bob"));
        cache.run_pending_tasks().await;
        // counts: a -> 1, b -> 1

        cache.insert("c", "cindy").await;
        assert_eq!(cache.get(&"c").await, Some("cindy"));
        assert!(cache.contains_key(&"c"));
        // counts: a -> 1, b -> 1, c -> 1
        cache.run_pending_tasks().await;

        assert!(cache.contains_key(&"a"));
        assert_eq!(cache.get(&"a").await, Some("alice"));
        assert_eq!(cache.get(&"b").await, Some("bob"));
        assert!(cache.contains_key(&"b"));
        cache.run_pending_tasks().await;
        // counts: a -> 2, b -> 2, c -> 1

        // "d" should not be admitted because its frequency is too low.
        cache.insert("d", "david").await; //   count: d -> 0
        expected.push((Arc::new("d"), "david", RemovalCause::Size));
        cache.run_pending_tasks().await;
        assert_eq!(cache.get(&"d").await, None); //   d -> 1
        assert!(!cache.contains_key(&"d"));

        cache.insert("d", "david").await;
        expected.push((Arc::new("d"), "david", RemovalCause::Size));
        cache.run_pending_tasks().await;
        assert!(!cache.contains_key(&"d"));
        assert_eq!(cache.get(&"d").await, None); //   d -> 2

        // "d" should be admitted and "c" should be evicted
        // because d's frequency is higher than c's.
        cache.insert("d", "dennis").await;
        expected.push((Arc::new("c"), "cindy", RemovalCause::Size));
        cache.run_pending_tasks().await;
        assert_eq!(cache.get(&"a").await, Some("alice"));
        assert_eq!(cache.get(&"b").await, Some("bob"));
        assert_eq!(cache.get(&"c").await, None);
        assert_eq!(cache.get(&"d").await, Some("dennis"));
        assert!(cache.contains_key(&"a"));
        assert!(cache.contains_key(&"b"));
        assert!(!cache.contains_key(&"c"));
        assert!(cache.contains_key(&"d"));

        cache.invalidate(&"b").await;
        expected.push((Arc::new("b"), "bob", RemovalCause::Explicit));
        cache.run_pending_tasks().await;
        assert_eq!(cache.get(&"b").await, None);
        assert!(!cache.contains_key(&"b"));

        assert!(cache.remove(&"b").await.is_none());
        assert_eq!(cache.remove(&"d").await, Some("dennis"));
        expected.push((Arc::new("d"), "dennis", RemovalCause::Explicit));
        cache.run_pending_tasks().await;
        assert_eq!(cache.get(&"d").await, None);
        assert!(!cache.contains_key(&"d"));

        verify_notification_vec(&cache, actual, &expected).await;
        assert!(cache.key_locks_map_is_empty());
    }

    #[tokio::test]
    async fn basic_lru_single_thread() {
        // The following `Vec`s will hold actual and expected notifications.
        let actual = Arc::new(Mutex::new(Vec::new()));
        let mut expected = Vec::new();

        // Create an eviction listener.
        let a1 = Arc::clone(&actual);
        let listener = move |k, v, cause| -> ListenerFuture {
            let a2 = Arc::clone(&a1);
            async move {
                a2.lock().await.push((k, v, cause));
            }
            .boxed()
        };

        // Create a cache with the eviction listener.
        let mut cache = Cache::builder()
            .max_capacity(3)
            .eviction_policy(EvictionPolicy::lru())
            .async_eviction_listener(listener)
            .build();
        cache.reconfigure_for_testing().await;

        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert("a", "alice").await;
        cache.insert("b", "bob").await;
        assert_eq!(cache.get(&"a").await, Some("alice"));
        assert!(cache.contains_key(&"a"));
        assert!(cache.contains_key(&"b"));
        assert_eq!(cache.get(&"b").await, Some("bob"));
        cache.run_pending_tasks().await;
        // a -> b

        cache.insert("c", "cindy").await;
        assert_eq!(cache.get(&"c").await, Some("cindy"));
        assert!(cache.contains_key(&"c"));
        cache.run_pending_tasks().await;
        // a -> b -> c

        assert!(cache.contains_key(&"a"));
        assert_eq!(cache.get(&"a").await, Some("alice"));
        assert_eq!(cache.get(&"b").await, Some("bob"));
        assert!(cache.contains_key(&"b"));
        cache.run_pending_tasks().await;
        // c -> a -> b

        // "d" should be admitted because the cache uses the LRU strategy.
        cache.insert("d", "david").await;
        // "c" is the LRU and should have be evicted.
        expected.push((Arc::new("c"), "cindy", RemovalCause::Size));
        cache.run_pending_tasks().await;

        assert_eq!(cache.get(&"a").await, Some("alice"));
        assert_eq!(cache.get(&"b").await, Some("bob"));
        assert_eq!(cache.get(&"c").await, None);
        assert_eq!(cache.get(&"d").await, Some("david"));
        assert!(cache.contains_key(&"a"));
        assert!(cache.contains_key(&"b"));
        assert!(!cache.contains_key(&"c"));
        assert!(cache.contains_key(&"d"));
        cache.run_pending_tasks().await;
        // a -> b -> d

        cache.invalidate(&"b").await;
        expected.push((Arc::new("b"), "bob", RemovalCause::Explicit));
        cache.run_pending_tasks().await;
        // a -> d
        assert_eq!(cache.get(&"b").await, None);
        assert!(!cache.contains_key(&"b"));

        assert!(cache.remove(&"b").await.is_none());
        assert_eq!(cache.remove(&"d").await, Some("david"));
        expected.push((Arc::new("d"), "david", RemovalCause::Explicit));
        cache.run_pending_tasks().await;
        // a
        assert_eq!(cache.get(&"d").await, None);
        assert!(!cache.contains_key(&"d"));

        cache.insert("e", "emily").await;
        cache.insert("f", "frank").await;
        // "a" should be evicted because it is the LRU.
        cache.insert("g", "gina").await;
        expected.push((Arc::new("a"), "alice", RemovalCause::Size));
        cache.run_pending_tasks().await;
        // e -> f -> g
        assert_eq!(cache.get(&"a").await, None);
        assert_eq!(cache.get(&"e").await, Some("emily"));
        assert_eq!(cache.get(&"f").await, Some("frank"));
        assert_eq!(cache.get(&"g").await, Some("gina"));
        assert!(!cache.contains_key(&"a"));
        assert!(cache.contains_key(&"e"));
        assert!(cache.contains_key(&"f"));
        assert!(cache.contains_key(&"g"));

        verify_notification_vec(&cache, actual, &expected).await;
        assert!(cache.key_locks_map_is_empty());
    }

    #[tokio::test]
    async fn size_aware_eviction() {
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
        let listener = move |k, v, cause| -> ListenerFuture {
            let a2 = Arc::clone(&a1);
            async move {
                a2.lock().await.push((k, v, cause));
            }
            .boxed()
        };

        // Create a cache with the eviction listener.
        let mut cache = Cache::builder()
            .max_capacity(31)
            .weigher(weigher)
            .async_eviction_listener(listener)
            .build();
        cache.reconfigure_for_testing().await;

        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert("a", alice).await;
        cache.insert("b", bob).await;
        assert_eq!(cache.get(&"a").await, Some(alice));
        assert!(cache.contains_key(&"a"));
        assert!(cache.contains_key(&"b"));
        assert_eq!(cache.get(&"b").await, Some(bob));
        cache.run_pending_tasks().await;
        // order (LRU -> MRU) and counts: a -> 1, b -> 1

        cache.insert("c", cindy).await;
        assert_eq!(cache.get(&"c").await, Some(cindy));
        assert!(cache.contains_key(&"c"));
        // order and counts: a -> 1, b -> 1, c -> 1
        cache.run_pending_tasks().await;

        assert!(cache.contains_key(&"a"));
        assert_eq!(cache.get(&"a").await, Some(alice));
        assert_eq!(cache.get(&"b").await, Some(bob));
        assert!(cache.contains_key(&"b"));
        cache.run_pending_tasks().await;
        // order and counts: c -> 1, a -> 2, b -> 2

        // To enter "d" (weight: 15), it needs to evict "c" (w: 5) and "a" (w: 10).
        // "d" must have higher count than 3, which is the aggregated count
        // of "a" and "c".
        cache.insert("d", david).await; //   count: d -> 0
        expected.push((Arc::new("d"), david, RemovalCause::Size));
        cache.run_pending_tasks().await;
        assert_eq!(cache.get(&"d").await, None); //   d -> 1
        assert!(!cache.contains_key(&"d"));

        cache.insert("d", david).await;
        expected.push((Arc::new("d"), david, RemovalCause::Size));
        cache.run_pending_tasks().await;
        assert!(!cache.contains_key(&"d"));
        assert_eq!(cache.get(&"d").await, None); //   d -> 2

        cache.insert("d", david).await;
        expected.push((Arc::new("d"), david, RemovalCause::Size));
        cache.run_pending_tasks().await;
        assert_eq!(cache.get(&"d").await, None); //   d -> 3
        assert!(!cache.contains_key(&"d"));

        cache.insert("d", david).await;
        expected.push((Arc::new("d"), david, RemovalCause::Size));
        cache.run_pending_tasks().await;
        assert!(!cache.contains_key(&"d"));
        assert_eq!(cache.get(&"d").await, None); //   d -> 4

        // Finally "d" should be admitted by evicting "c" and "a".
        cache.insert("d", dennis).await;
        expected.push((Arc::new("c"), cindy, RemovalCause::Size));
        expected.push((Arc::new("a"), alice, RemovalCause::Size));
        cache.run_pending_tasks().await;
        assert_eq!(cache.get(&"a").await, None);
        assert_eq!(cache.get(&"b").await, Some(bob));
        assert_eq!(cache.get(&"c").await, None);
        assert_eq!(cache.get(&"d").await, Some(dennis));
        assert!(!cache.contains_key(&"a"));
        assert!(cache.contains_key(&"b"));
        assert!(!cache.contains_key(&"c"));
        assert!(cache.contains_key(&"d"));

        // Update "b" with "bill" (w: 15 -> 20). This should evict "d" (w: 15).
        cache.insert("b", bill).await;
        expected.push((Arc::new("b"), bob, RemovalCause::Replaced));
        expected.push((Arc::new("d"), dennis, RemovalCause::Size));
        cache.run_pending_tasks().await;
        assert_eq!(cache.get(&"b").await, Some(bill));
        assert_eq!(cache.get(&"d").await, None);
        assert!(cache.contains_key(&"b"));
        assert!(!cache.contains_key(&"d"));

        // Re-add "a" (w: 10) and update "b" with "bob" (w: 20 -> 15).
        cache.insert("a", alice).await;
        cache.insert("b", bob).await;
        expected.push((Arc::new("b"), bill, RemovalCause::Replaced));
        cache.run_pending_tasks().await;
        assert_eq!(cache.get(&"a").await, Some(alice));
        assert_eq!(cache.get(&"b").await, Some(bob));
        assert_eq!(cache.get(&"d").await, None);
        assert!(cache.contains_key(&"a"));
        assert!(cache.contains_key(&"b"));
        assert!(!cache.contains_key(&"d"));

        // Verify the sizes.
        assert_eq!(cache.entry_count(), 2);
        assert_eq!(cache.weighted_size(), 25);

        verify_notification_vec(&cache, actual, &expected).await;
        assert!(cache.key_locks_map_is_empty());
    }

    #[tokio::test]
    async fn basic_multi_async_tasks() {
        let num_tasks = 2;
        let num_threads = 2;

        let cache = Cache::new(100);
        let barrier = Arc::new(Barrier::new(num_tasks + num_threads as usize));

        let tasks = (0..num_tasks)
            .map(|id| {
                let cache = cache.clone();
                let barrier = Arc::clone(&barrier);

                tokio::spawn(async move {
                    barrier.wait().await;

                    cache.insert(10, format!("{id}-100")).await;
                    cache.get(&10).await;
                    cache.insert(20, format!("{id}-200")).await;
                    cache.invalidate(&10).await;
                })
            })
            .collect::<Vec<_>>();

        let threads = (0..num_threads)
            .map(|id| {
                let cache = cache.clone();
                let barrier = Arc::clone(&barrier);
                let rt = tokio::runtime::Handle::current();

                std::thread::spawn(move || {
                    rt.block_on(barrier.wait());

                    rt.block_on(cache.insert(10, format!("{id}-100")));
                    rt.block_on(cache.get(&10));
                    rt.block_on(cache.insert(20, format!("{id}-200")));
                    rt.block_on(cache.invalidate(&10));
                })
            })
            .collect::<Vec<_>>();

        let _ = futures_util::future::join_all(tasks).await;
        threads.into_iter().for_each(|t| t.join().unwrap());

        assert!(cache.get(&10).await.is_none());
        assert!(cache.get(&20).await.is_some());
        assert!(!cache.contains_key(&10));
        assert!(cache.contains_key(&20));
    }

    #[tokio::test]
    async fn invalidate_all() {
        // The following `Vec`s will hold actual and expected notifications.
        let actual = Arc::new(Mutex::new(Vec::new()));
        let mut expected = Vec::new();

        // Create an eviction listener.
        let a1 = Arc::clone(&actual);
        let listener = move |k, v, cause| -> ListenerFuture {
            let a2 = Arc::clone(&a1);
            async move {
                a2.lock().await.push((k, v, cause));
            }
            .boxed()
        };

        // Create a cache with the eviction listener.
        let mut cache = Cache::builder()
            .max_capacity(100)
            .async_eviction_listener(listener)
            .build();
        cache.reconfigure_for_testing().await;

        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert("a", "alice").await;
        cache.insert("b", "bob").await;
        cache.insert("c", "cindy").await;
        assert_eq!(cache.get(&"a").await, Some("alice"));
        assert_eq!(cache.get(&"b").await, Some("bob"));
        assert_eq!(cache.get(&"c").await, Some("cindy"));
        assert!(cache.contains_key(&"a"));
        assert!(cache.contains_key(&"b"));
        assert!(cache.contains_key(&"c"));

        // `cache.run_pending_tasks().await` is no longer needed here before invalidating. The last
        // modified timestamp of the entries were updated when they were inserted.
        // https://github.com/moka-rs/moka/issues/155

        cache.invalidate_all();
        expected.push((Arc::new("a"), "alice", RemovalCause::Explicit));
        expected.push((Arc::new("b"), "bob", RemovalCause::Explicit));
        expected.push((Arc::new("c"), "cindy", RemovalCause::Explicit));
        cache.run_pending_tasks().await;

        cache.insert("d", "david").await;
        cache.run_pending_tasks().await;

        assert!(cache.get(&"a").await.is_none());
        assert!(cache.get(&"b").await.is_none());
        assert!(cache.get(&"c").await.is_none());
        assert_eq!(cache.get(&"d").await, Some("david"));
        assert!(!cache.contains_key(&"a"));
        assert!(!cache.contains_key(&"b"));
        assert!(!cache.contains_key(&"c"));
        assert!(cache.contains_key(&"d"));

        verify_notification_vec(&cache, actual, &expected).await;
    }

    // This test is for https://github.com/moka-rs/moka/issues/155
    #[tokio::test]
    async fn invalidate_all_without_running_pending_tasks() {
        let cache = Cache::new(1024);

        assert_eq!(cache.get(&0).await, None);
        cache.insert(0, 1).await;
        assert_eq!(cache.get(&0).await, Some(1));

        cache.invalidate_all();
        assert_eq!(cache.get(&0).await, None);
    }

    #[tokio::test]
    async fn invalidate_entries_if() -> Result<(), Box<dyn std::error::Error>> {
        use std::collections::HashSet;

        // The following `Vec`s will hold actual and expected notifications.
        let actual = Arc::new(Mutex::new(Vec::new()));
        let mut expected = Vec::new();

        // Create an eviction listener.
        let a1 = Arc::clone(&actual);
        let listener = move |k, v, cause| -> ListenerFuture {
            let a2 = Arc::clone(&a1);
            async move {
                a2.lock().await.push((k, v, cause));
            }
            .boxed()
        };

        let (clock, mock) = Clock::mock();

        // Create a cache with the eviction listener.
        let mut cache = Cache::builder()
            .max_capacity(100)
            .support_invalidation_closures()
            .async_eviction_listener(listener)
            .clock(clock)
            .build();
        cache.reconfigure_for_testing().await;

        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert(0, "alice").await;
        cache.insert(1, "bob").await;
        cache.insert(2, "alex").await;
        cache.run_pending_tasks().await;

        mock.increment(Duration::from_secs(5)); // 5 secs from the start.
        cache.run_pending_tasks().await;

        assert_eq!(cache.get(&0).await, Some("alice"));
        assert_eq!(cache.get(&1).await, Some("bob"));
        assert_eq!(cache.get(&2).await, Some("alex"));
        assert!(cache.contains_key(&0));
        assert!(cache.contains_key(&1));
        assert!(cache.contains_key(&2));

        let names = ["alice", "alex"].iter().cloned().collect::<HashSet<_>>();
        cache.invalidate_entries_if(move |_k, &v| names.contains(v))?;
        assert_eq!(cache.invalidation_predicate_count(), 1);
        expected.push((Arc::new(0), "alice", RemovalCause::Explicit));
        expected.push((Arc::new(2), "alex", RemovalCause::Explicit));

        mock.increment(Duration::from_secs(5)); // 10 secs from the start.

        cache.insert(3, "alice").await;

        // Run the invalidation task and wait for it to finish. (TODO: Need a better way than sleeping)
        cache.run_pending_tasks().await; // To submit the invalidation task.
        std::thread::sleep(Duration::from_millis(200));
        cache.run_pending_tasks().await; // To process the task result.
        std::thread::sleep(Duration::from_millis(200));

        assert!(cache.get(&0).await.is_none());
        assert!(cache.get(&2).await.is_none());
        assert_eq!(cache.get(&1).await, Some("bob"));
        // This should survive as it was inserted after calling invalidate_entries_if.
        assert_eq!(cache.get(&3).await, Some("alice"));

        assert!(!cache.contains_key(&0));
        assert!(cache.contains_key(&1));
        assert!(!cache.contains_key(&2));
        assert!(cache.contains_key(&3));

        assert_eq!(cache.entry_count(), 2);
        assert_eq!(cache.invalidation_predicate_count(), 0);

        mock.increment(Duration::from_secs(5)); // 15 secs from the start.

        cache.invalidate_entries_if(|_k, &v| v == "alice")?;
        cache.invalidate_entries_if(|_k, &v| v == "bob")?;
        assert_eq!(cache.invalidation_predicate_count(), 2);
        // key 1 was inserted before key 3.
        expected.push((Arc::new(1), "bob", RemovalCause::Explicit));
        expected.push((Arc::new(3), "alice", RemovalCause::Explicit));

        // Run the invalidation task and wait for it to finish. (TODO: Need a better way than sleeping)
        cache.run_pending_tasks().await; // To submit the invalidation task.
        std::thread::sleep(Duration::from_millis(200));
        cache.run_pending_tasks().await; // To process the task result.
        std::thread::sleep(Duration::from_millis(200));

        assert!(cache.get(&1).await.is_none());
        assert!(cache.get(&3).await.is_none());

        assert!(!cache.contains_key(&1));
        assert!(!cache.contains_key(&3));

        assert_eq!(cache.entry_count(), 0);
        assert_eq!(cache.invalidation_predicate_count(), 0);

        verify_notification_vec(&cache, actual, &expected).await;

        Ok(())
    }

    #[tokio::test]
    async fn time_to_live() {
        // The following `Vec`s will hold actual and expected notifications.
        let actual = Arc::new(Mutex::new(Vec::new()));
        let mut expected = Vec::new();

        // Create an eviction listener.
        let a1 = Arc::clone(&actual);
        let listener = move |k, v, cause| -> ListenerFuture {
            let a2 = Arc::clone(&a1);
            async move {
                a2.lock().await.push((k, v, cause));
            }
            .boxed()
        };

        let (clock, mock) = Clock::mock();

        // Create a cache with the eviction listener.
        let mut cache = Cache::builder()
            .max_capacity(100)
            .time_to_live(Duration::from_secs(10))
            .async_eviction_listener(listener)
            .clock(clock)
            .build();
        cache.reconfigure_for_testing().await;

        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert("a", "alice").await;
        cache.run_pending_tasks().await;

        mock.increment(Duration::from_secs(5)); // 5 secs from the start.
        cache.run_pending_tasks().await;

        assert_eq!(cache.get(&"a").await, Some("alice"));
        assert!(cache.contains_key(&"a"));

        mock.increment(Duration::from_secs(5)); // 10 secs.
        expected.push((Arc::new("a"), "alice", RemovalCause::Expired));
        assert_eq!(cache.get(&"a").await, None);
        assert!(!cache.contains_key(&"a"));

        assert_eq!(cache.iter().count(), 0);

        cache.run_pending_tasks().await;
        assert!(cache.is_table_empty());

        cache.insert("b", "bob").await;
        cache.run_pending_tasks().await;

        assert_eq!(cache.entry_count(), 1);

        mock.increment(Duration::from_secs(5)); // 15 secs.
        cache.run_pending_tasks().await;

        assert_eq!(cache.get(&"b").await, Some("bob"));
        assert!(cache.contains_key(&"b"));
        assert_eq!(cache.entry_count(), 1);

        cache.insert("b", "bill").await;
        expected.push((Arc::new("b"), "bob", RemovalCause::Replaced));
        cache.run_pending_tasks().await;

        mock.increment(Duration::from_secs(5)); // 20 secs
        cache.run_pending_tasks().await;

        assert_eq!(cache.get(&"b").await, Some("bill"));
        assert!(cache.contains_key(&"b"));
        assert_eq!(cache.entry_count(), 1);

        mock.increment(Duration::from_secs(5)); // 25 secs
        expected.push((Arc::new("b"), "bill", RemovalCause::Expired));

        assert_eq!(cache.get(&"a").await, None);
        assert_eq!(cache.get(&"b").await, None);
        assert!(!cache.contains_key(&"a"));
        assert!(!cache.contains_key(&"b"));

        assert_eq!(cache.iter().count(), 0);

        cache.run_pending_tasks().await;
        assert!(cache.is_table_empty());

        verify_notification_vec(&cache, actual, &expected).await;
    }

    #[tokio::test]
    async fn time_to_idle() {
        // The following `Vec`s will hold actual and expected notifications.
        let actual = Arc::new(Mutex::new(Vec::new()));
        let mut expected = Vec::new();

        // Create an eviction listener.
        let a1 = Arc::clone(&actual);
        let listener = move |k, v, cause| -> ListenerFuture {
            let a2 = Arc::clone(&a1);
            async move {
                a2.lock().await.push((k, v, cause));
            }
            .boxed()
        };

        let (clock, mock) = Clock::mock();

        // Create a cache with the eviction listener.
        let mut cache = Cache::builder()
            .max_capacity(100)
            .time_to_idle(Duration::from_secs(10))
            .async_eviction_listener(listener)
            .clock(clock)
            .build();
        cache.reconfigure_for_testing().await;

        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert("a", "alice").await;
        cache.run_pending_tasks().await;

        mock.increment(Duration::from_secs(5)); // 5 secs from the start.
        cache.run_pending_tasks().await;

        assert_eq!(cache.get(&"a").await, Some("alice"));

        mock.increment(Duration::from_secs(5)); // 10 secs.
        cache.run_pending_tasks().await;

        cache.insert("b", "bob").await;
        cache.run_pending_tasks().await;

        assert_eq!(cache.entry_count(), 2);

        mock.increment(Duration::from_secs(2)); // 12 secs.
        cache.run_pending_tasks().await;

        // contains_key does not reset the idle timer for the key.
        assert!(cache.contains_key(&"a"));
        assert!(cache.contains_key(&"b"));
        cache.run_pending_tasks().await;

        assert_eq!(cache.entry_count(), 2);

        mock.increment(Duration::from_secs(3)); // 15 secs.
        expected.push((Arc::new("a"), "alice", RemovalCause::Expired));

        assert_eq!(cache.get(&"a").await, None);
        assert_eq!(cache.get(&"b").await, Some("bob"));
        assert!(!cache.contains_key(&"a"));
        assert!(cache.contains_key(&"b"));

        assert_eq!(cache.iter().count(), 1);

        cache.run_pending_tasks().await;
        assert_eq!(cache.entry_count(), 1);

        mock.increment(Duration::from_secs(10)); // 25 secs
        expected.push((Arc::new("b"), "bob", RemovalCause::Expired));

        assert_eq!(cache.get(&"a").await, None);
        assert_eq!(cache.get(&"b").await, None);
        assert!(!cache.contains_key(&"a"));
        assert!(!cache.contains_key(&"b"));

        assert_eq!(cache.iter().count(), 0);

        cache.run_pending_tasks().await;
        assert!(cache.is_table_empty());

        verify_notification_vec(&cache, actual, &expected).await;
    }

    // https://github.com/moka-rs/moka/issues/359
    #[tokio::test]
    async fn ensure_access_time_is_updated_immediately_after_read() {
        let (clock, mock) = Clock::mock();
        let mut cache = Cache::builder()
            .max_capacity(10)
            .time_to_idle(Duration::from_secs(5))
            .clock(clock)
            .build();
        cache.reconfigure_for_testing().await;
        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert(1, 1).await;

        mock.increment(Duration::from_secs(4));
        assert_eq!(cache.get(&1).await, Some(1));

        mock.increment(Duration::from_secs(2));
        assert_eq!(cache.get(&1).await, Some(1));
        cache.run_pending_tasks().await;
        assert_eq!(cache.get(&1).await, Some(1));
    }

    #[tokio::test]
    async fn time_to_live_by_expiry_type() {
        // The following `Vec`s will hold actual and expected notifications.
        let actual = Arc::new(Mutex::new(Vec::new()));
        let mut expected = Vec::new();

        // Define an expiry type.
        struct MyExpiry {
            counters: Arc<ExpiryCallCounters>,
        }

        impl MyExpiry {
            fn new(counters: Arc<ExpiryCallCounters>) -> Self {
                Self { counters }
            }
        }

        impl Expiry<&str, &str> for MyExpiry {
            fn expire_after_create(
                &self,
                _key: &&str,
                _value: &&str,
                _current_time: StdInstant,
            ) -> Option<Duration> {
                self.counters.incl_actual_creations();
                Some(Duration::from_secs(10))
            }

            fn expire_after_update(
                &self,
                _key: &&str,
                _value: &&str,
                _current_time: StdInstant,
                _current_duration: Option<Duration>,
            ) -> Option<Duration> {
                self.counters.incl_actual_updates();
                Some(Duration::from_secs(10))
            }
        }

        // Create an eviction listener.
        let a1 = Arc::clone(&actual);
        let listener = move |k, v, cause| -> ListenerFuture {
            let a2 = Arc::clone(&a1);
            async move {
                a2.lock().await.push((k, v, cause));
            }
            .boxed()
        };

        // Create expiry counters and the expiry.
        let expiry_counters = Arc::new(ExpiryCallCounters::default());
        let expiry = MyExpiry::new(Arc::clone(&expiry_counters));

        let (clock, mock) = Clock::mock();

        // Create a cache with the expiry and eviction listener.
        let mut cache = Cache::builder()
            .max_capacity(100)
            .expire_after(expiry)
            .async_eviction_listener(listener)
            .clock(clock)
            .build();
        cache.reconfigure_for_testing().await;

        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert("a", "alice").await;
        expiry_counters.incl_expected_creations();
        cache.run_pending_tasks().await;

        mock.increment(Duration::from_secs(5)); // 5 secs from the start.
        cache.run_pending_tasks().await;

        assert_eq!(cache.get(&"a").await, Some("alice"));
        assert!(cache.contains_key(&"a"));

        mock.increment(Duration::from_secs(5)); // 10 secs.
        expected.push((Arc::new("a"), "alice", RemovalCause::Expired));
        assert_eq!(cache.get(&"a").await, None);
        assert!(!cache.contains_key(&"a"));

        assert_eq!(cache.iter().count(), 0);

        cache.run_pending_tasks().await;
        assert!(cache.is_table_empty());

        cache.insert("b", "bob").await;
        expiry_counters.incl_expected_creations();
        cache.run_pending_tasks().await;

        assert_eq!(cache.entry_count(), 1);

        mock.increment(Duration::from_secs(5)); // 15 secs.
        cache.run_pending_tasks().await;

        assert_eq!(cache.get(&"b").await, Some("bob"));
        assert!(cache.contains_key(&"b"));
        assert_eq!(cache.entry_count(), 1);

        cache.insert("b", "bill").await;
        expected.push((Arc::new("b"), "bob", RemovalCause::Replaced));
        expiry_counters.incl_expected_updates();
        cache.run_pending_tasks().await;

        mock.increment(Duration::from_secs(5)); // 20 secs
        cache.run_pending_tasks().await;

        assert_eq!(cache.get(&"b").await, Some("bill"));
        assert!(cache.contains_key(&"b"));
        assert_eq!(cache.entry_count(), 1);

        mock.increment(Duration::from_secs(5)); // 25 secs
        expected.push((Arc::new("b"), "bill", RemovalCause::Expired));

        assert_eq!(cache.get(&"a").await, None);
        assert_eq!(cache.get(&"b").await, None);
        assert!(!cache.contains_key(&"a"));
        assert!(!cache.contains_key(&"b"));

        assert_eq!(cache.iter().count(), 0);

        cache.run_pending_tasks().await;
        assert!(cache.is_table_empty());

        expiry_counters.verify();
        verify_notification_vec(&cache, actual, &expected).await;
    }

    #[tokio::test]
    async fn time_to_idle_by_expiry_type() {
        // The following `Vec`s will hold actual and expected notifications.
        let actual = Arc::new(Mutex::new(Vec::new()));
        let mut expected = Vec::new();

        // Define an expiry type.
        struct MyExpiry {
            counters: Arc<ExpiryCallCounters>,
        }

        impl MyExpiry {
            fn new(counters: Arc<ExpiryCallCounters>) -> Self {
                Self { counters }
            }
        }

        impl Expiry<&str, &str> for MyExpiry {
            fn expire_after_read(
                &self,
                _key: &&str,
                _value: &&str,
                _current_time: StdInstant,
                _current_duration: Option<Duration>,
                _last_modified_at: StdInstant,
            ) -> Option<Duration> {
                self.counters.incl_actual_reads();
                Some(Duration::from_secs(10))
            }
        }

        // Create an eviction listener.
        let a1 = Arc::clone(&actual);
        let listener = move |k, v, cause| -> ListenerFuture {
            let a2 = Arc::clone(&a1);
            async move {
                a2.lock().await.push((k, v, cause));
            }
            .boxed()
        };

        // Create expiry counters and the expiry.
        let expiry_counters = Arc::new(ExpiryCallCounters::default());
        let expiry = MyExpiry::new(Arc::clone(&expiry_counters));

        let (clock, mock) = Clock::mock();

        // Create a cache with the expiry and eviction listener.
        let mut cache = Cache::builder()
            .max_capacity(100)
            .expire_after(expiry)
            .async_eviction_listener(listener)
            .clock(clock)
            .build();
        cache.reconfigure_for_testing().await;

        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert("a", "alice").await;
        cache.run_pending_tasks().await;

        mock.increment(Duration::from_secs(5)); // 5 secs from the start.
        cache.run_pending_tasks().await;

        assert_eq!(cache.get(&"a").await, Some("alice"));
        expiry_counters.incl_expected_reads();

        mock.increment(Duration::from_secs(5)); // 10 secs.
        cache.run_pending_tasks().await;

        cache.insert("b", "bob").await;
        cache.run_pending_tasks().await;

        assert_eq!(cache.entry_count(), 2);

        mock.increment(Duration::from_secs(2)); // 12 secs.
        cache.run_pending_tasks().await;

        // contains_key does not reset the idle timer for the key.
        assert!(cache.contains_key(&"a"));
        assert!(cache.contains_key(&"b"));
        cache.run_pending_tasks().await;

        assert_eq!(cache.entry_count(), 2);

        mock.increment(Duration::from_secs(3)); // 15 secs.
        expected.push((Arc::new("a"), "alice", RemovalCause::Expired));

        assert_eq!(cache.get(&"a").await, None);
        assert_eq!(cache.get(&"b").await, Some("bob"));
        expiry_counters.incl_expected_reads();
        assert!(!cache.contains_key(&"a"));
        assert!(cache.contains_key(&"b"));

        assert_eq!(cache.iter().count(), 1);

        cache.run_pending_tasks().await;
        assert_eq!(cache.entry_count(), 1);

        mock.increment(Duration::from_secs(10)); // 25 secs
        expected.push((Arc::new("b"), "bob", RemovalCause::Expired));

        assert_eq!(cache.get(&"a").await, None);
        assert_eq!(cache.get(&"b").await, None);
        assert!(!cache.contains_key(&"a"));
        assert!(!cache.contains_key(&"b"));

        assert_eq!(cache.iter().count(), 0);

        cache.run_pending_tasks().await;
        assert!(cache.is_table_empty());

        expiry_counters.verify();
        verify_notification_vec(&cache, actual, &expected).await;
    }

    /// Verify that the `Expiry::expire_after_read()` method is called in `get_with`
    /// only when the key was already present in the cache.
    #[tokio::test]
    async fn test_expiry_using_get_with() {
        // Define an expiry type, which always return `None`.
        struct NoExpiry {
            counters: Arc<ExpiryCallCounters>,
        }

        impl NoExpiry {
            fn new(counters: Arc<ExpiryCallCounters>) -> Self {
                Self { counters }
            }
        }

        impl Expiry<&str, &str> for NoExpiry {
            fn expire_after_create(
                &self,
                _key: &&str,
                _value: &&str,
                _current_time: StdInstant,
            ) -> Option<Duration> {
                self.counters.incl_actual_creations();
                None
            }

            fn expire_after_read(
                &self,
                _key: &&str,
                _value: &&str,
                _current_time: StdInstant,
                _current_duration: Option<Duration>,
                _last_modified_at: StdInstant,
            ) -> Option<Duration> {
                self.counters.incl_actual_reads();
                None
            }

            fn expire_after_update(
                &self,
                _key: &&str,
                _value: &&str,
                _current_time: StdInstant,
                _current_duration: Option<Duration>,
            ) -> Option<Duration> {
                unreachable!("The `expire_after_update()` method should not be called.");
            }
        }

        // Create expiry counters and the expiry.
        let expiry_counters = Arc::new(ExpiryCallCounters::default());
        let expiry = NoExpiry::new(Arc::clone(&expiry_counters));

        // Create a cache with the expiry and eviction listener.
        let mut cache = Cache::builder()
            .max_capacity(100)
            .expire_after(expiry)
            .build();
        cache.reconfigure_for_testing().await;

        // Make the cache exterior immutable.
        let cache = cache;

        // The key is not present.
        cache.get_with("a", async { "alice" }).await;
        expiry_counters.incl_expected_creations();
        cache.run_pending_tasks().await;

        // The key is present.
        cache.get_with("a", async { "alex" }).await;
        expiry_counters.incl_expected_reads();
        cache.run_pending_tasks().await;

        // The key is not present.
        cache.invalidate("a").await;
        cache.get_with("a", async { "amanda" }).await;
        expiry_counters.incl_expected_creations();
        cache.run_pending_tasks().await;

        expiry_counters.verify();
    }

    // https://github.com/moka-rs/moka/issues/345
    #[tokio::test]
    async fn test_race_between_updating_entry_and_processing_its_write_ops() {
        let (clock, mock) = Clock::mock();
        let cache = Cache::builder()
            .max_capacity(2)
            .time_to_idle(Duration::from_secs(1))
            .clock(clock)
            .build();

        cache.insert("a", "alice").await;
        cache.insert("b", "bob").await;
        cache.insert("c", "cathy").await; // c1
        mock.increment(Duration::from_secs(2));

        // The following `insert` will do the followings:
        // 1. Replaces current "c" (c1) in the concurrent hash table (cht).
        // 2. Runs the pending tasks implicitly.
        //    (1) "a" will be admitted.
        //    (2) "b" will be admitted.
        //    (3) c1 will be evicted by size constraint.
        //    (4) "a" will be evicted due to expiration.
        //    (5) "b" will be evicted due to expiration.
        // 3. Send its `WriteOp` log to the channel.
        cache.insert("c", "cindy").await; // c2

        // Remove "c" (c2) from the cht.
        assert_eq!(cache.remove(&"c").await, Some("cindy")); // c-remove

        mock.increment(Duration::from_secs(2));

        // The following `run_pending_tasks` will do the followings:
        // 1. Admits "c" (c2) to the cache. (Create a node in the LRU deque)
        // 2. Because of c-remove, removes c2's node from the LRU deque.
        cache.run_pending_tasks().await;
        assert_eq!(cache.entry_count(), 0);
    }

    #[tokio::test]
    async fn test_race_between_recreating_entry_and_processing_its_write_ops() {
        let cache = Cache::builder().max_capacity(2).build();

        cache.insert('a', "a").await;
        cache.insert('b', "b").await;
        cache.run_pending_tasks().await;

        cache.insert('c', "c1").await; // (a) `EntryInfo` 1, gen: 1
        assert!(cache.remove(&'a').await.is_some()); // (b)
        assert!(cache.remove(&'b').await.is_some()); // (c)
        assert!(cache.remove(&'c').await.is_some()); // (d) `EntryInfo` 1, gen: 2
        cache.insert('c', "c2").await; // (e) `EntryInfo` 2, gen: 1

        // Now the `write_op_ch` channel contains the following `WriteOp`s:
        //
        // - 0: (a) insert "c1" (`EntryInfo` 1, gen: 1)
        // - 1: (b) remove "a"
        // - 2: (c) remove "b"
        // - 3: (d) remove "c1" (`EntryInfo` 1, gen: 2)
        // - 4: (e) insert "c2" (`EntryInfo` 2, gen: 1)
        //
        // 0 for "c1" is going to be rejected because the cache is full. Let's ensure
        // processing 0 must not remove "c2" from the concurrent hash table. (Their
        // gen are the same, but `EntryInfo`s are different)
        cache.run_pending_tasks().await;
        assert_eq!(cache.get(&'c').await, Some("c2"));
    }

    #[tokio::test]
    async fn test_iter() {
        const NUM_KEYS: usize = 50;

        fn make_value(key: usize) -> String {
            format!("val: {key}")
        }

        let cache = Cache::builder()
            .max_capacity(100)
            .time_to_idle(Duration::from_secs(10))
            .build();

        for key in 0..NUM_KEYS {
            cache.insert(key, make_value(key)).await;
        }

        let mut key_set = std::collections::HashSet::new();

        for (key, value) in &cache {
            assert_eq!(value, make_value(*key));

            key_set.insert(*key);
        }

        // Ensure there are no missing or duplicate keys in the iteration.
        assert_eq!(key_set.len(), NUM_KEYS);
    }

    /// Runs 16 async tasks at the same time and ensures no deadlock occurs.
    ///
    /// - Eight of the task will update key-values in the cache.
    /// - Eight others will iterate the cache.
    ///
    #[tokio::test]
    async fn test_iter_multi_async_tasks() {
        use std::collections::HashSet;

        const NUM_KEYS: usize = 1024;
        const NUM_TASKS: usize = 16;

        fn make_value(key: usize) -> String {
            format!("val: {key}")
        }

        let cache = Cache::builder()
            .max_capacity(2048)
            .time_to_idle(Duration::from_secs(10))
            .build();

        // Initialize the cache.
        for key in 0..NUM_KEYS {
            cache.insert(key, make_value(key)).await;
        }

        let rw_lock = Arc::new(tokio::sync::RwLock::<()>::default());
        let write_lock = rw_lock.write().await;

        let tasks = (0..NUM_TASKS)
            .map(|n| {
                let cache = cache.clone();
                let rw_lock = Arc::clone(&rw_lock);

                if n % 2 == 0 {
                    // This thread will update the cache.
                    tokio::spawn(async move {
                        let read_lock = rw_lock.read().await;
                        for key in 0..NUM_KEYS {
                            // TODO: Update keys in a random order?
                            cache.insert(key, make_value(key)).await;
                        }
                        std::mem::drop(read_lock);
                    })
                } else {
                    // This thread will iterate the cache.
                    tokio::spawn(async move {
                        let read_lock = rw_lock.read().await;
                        let mut key_set = HashSet::new();
                        // let mut key_count = 0usize;
                        for (key, value) in &cache {
                            assert_eq!(value, make_value(*key));
                            key_set.insert(*key);
                            // key_count += 1;
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

        let _ = futures_util::future::join_all(tasks).await;

        // Ensure there are no missing or duplicate keys in the iteration.
        let key_set = cache.iter().map(|(k, _v)| *k).collect::<HashSet<_>>();
        assert_eq!(key_set.len(), NUM_KEYS);
    }

    #[tokio::test]
    async fn get_with() {
        let cache = Cache::new(100);
        const KEY: u32 = 0;

        // This test will run five async tasks:
        //
        // Task1 will be the first task to call `get_with` for a key, so its async
        // block will be evaluated and then a &str value "task1" will be inserted to
        // the cache.
        let task1 = {
            let cache1 = cache.clone();
            async move {
                // Call `get_with` immediately.
                let v = cache1
                    .get_with(KEY, async {
                        // Wait for 300 ms and return a &str value.
                        sleep(Duration::from_millis(300)).await;
                        "task1"
                    })
                    .await;
                assert_eq!(v, "task1");
            }
        };

        // Task2 will be the second task to call `get_with` for the same key, so its
        // async block will not be evaluated. Once task1's async block finishes, it
        // will get the value inserted by task1's async block.
        let task2 = {
            let cache2 = cache.clone();
            async move {
                // Wait for 100 ms before calling `get_with`.
                sleep(Duration::from_millis(100)).await;
                let v = cache2.get_with(KEY, async { unreachable!() }).await;
                assert_eq!(v, "task1");
            }
        };

        // Task3 will be the third task to call `get_with` for the same key. By the
        // time it calls, task1's async block should have finished already and the
        // value should be already inserted to the cache. So its async block will not
        // be evaluated and will get the value inserted by task1's async block
        // immediately.
        let task3 = {
            let cache3 = cache.clone();
            async move {
                // Wait for 400 ms before calling `get_with`.
                sleep(Duration::from_millis(400)).await;
                let v = cache3.get_with(KEY, async { unreachable!() }).await;
                assert_eq!(v, "task1");
            }
        };

        // Task4 will call `get` for the same key. It will call when task1's async
        // block is still running, so it will get none for the key.
        let task4 = {
            let cache4 = cache.clone();
            async move {
                // Wait for 200 ms before calling `get`.
                sleep(Duration::from_millis(200)).await;
                let maybe_v = cache4.get(&KEY).await;
                assert!(maybe_v.is_none());
            }
        };

        // Task5 will call `get` for the same key. It will call after task1's async
        // block finished, so it will get the value insert by task1's async block.
        let task5 = {
            let cache5 = cache.clone();
            async move {
                // Wait for 400 ms before calling `get`.
                sleep(Duration::from_millis(400)).await;
                let maybe_v = cache5.get(&KEY).await;
                assert_eq!(maybe_v, Some("task1"));
            }
        };

        futures_util::join!(task1, task2, task3, task4, task5);

        assert!(cache.is_waiter_map_empty());
    }

    #[tokio::test]
    async fn get_with_by_ref() {
        let cache = Cache::new(100);
        const KEY: &u32 = &0;

        // This test will run five async tasks:
        //
        // Task1 will be the first task to call `get_with_by_ref` for a key, so its async
        // block will be evaluated and then a &str value "task1" will be inserted to
        // the cache.
        let task1 = {
            let cache1 = cache.clone();
            async move {
                // Call `get_with_by_ref` immediately.
                let v = cache1
                    .get_with_by_ref(KEY, async {
                        // Wait for 300 ms and return a &str value.
                        sleep(Duration::from_millis(300)).await;
                        "task1"
                    })
                    .await;
                assert_eq!(v, "task1");
            }
        };

        // Task2 will be the second task to call `get_with_by_ref` for the same key, so its
        // async block will not be evaluated. Once task1's async block finishes, it
        // will get the value inserted by task1's async block.
        let task2 = {
            let cache2 = cache.clone();
            async move {
                // Wait for 100 ms before calling `get_with_by_ref`.
                sleep(Duration::from_millis(100)).await;
                let v = cache2.get_with_by_ref(KEY, async { unreachable!() }).await;
                assert_eq!(v, "task1");
            }
        };

        // Task3 will be the third task to call `get_with_by_ref` for the same key. By the
        // time it calls, task1's async block should have finished already and the
        // value should be already inserted to the cache. So its async block will not
        // be evaluated and will get the value inserted by task1's async block
        // immediately.
        let task3 = {
            let cache3 = cache.clone();
            async move {
                // Wait for 400 ms before calling `get_with_by_ref`.
                sleep(Duration::from_millis(400)).await;
                let v = cache3.get_with_by_ref(KEY, async { unreachable!() }).await;
                assert_eq!(v, "task1");
            }
        };

        // Task4 will call `get` for the same key. It will call when task1's async
        // block is still running, so it will get none for the key.
        let task4 = {
            let cache4 = cache.clone();
            async move {
                // Wait for 200 ms before calling `get`.
                sleep(Duration::from_millis(200)).await;
                let maybe_v = cache4.get(KEY).await;
                assert!(maybe_v.is_none());
            }
        };

        // Task5 will call `get` for the same key. It will call after task1's async
        // block finished, so it will get the value insert by task1's async block.
        let task5 = {
            let cache5 = cache.clone();
            async move {
                // Wait for 400 ms before calling `get`.
                sleep(Duration::from_millis(400)).await;
                let maybe_v = cache5.get(KEY).await;
                assert_eq!(maybe_v, Some("task1"));
            }
        };

        futures_util::join!(task1, task2, task3, task4, task5);

        assert!(cache.is_waiter_map_empty());
    }

    #[tokio::test]
    async fn entry_or_insert_with_if() {
        let cache = Cache::new(100);
        const KEY: u32 = 0;

        // This test will run seven async tasks:
        //
        // Task1 will be the first task to call `or_insert_with_if` for a key, so its
        // async block will be evaluated and then a &str value "task1" will be
        // inserted to the cache.
        let task1 = {
            let cache1 = cache.clone();
            async move {
                // Call `or_insert_with_if` immediately.
                let entry = cache1
                    .entry(KEY)
                    .or_insert_with_if(
                        async {
                            // Wait for 300 ms and return a &str value.
                            sleep(Duration::from_millis(300)).await;
                            "task1"
                        },
                        |_v| unreachable!(),
                    )
                    .await;
                // Entry should be fresh because our async block should have been
                // evaluated.
                assert!(entry.is_fresh());
                assert_eq!(entry.into_value(), "task1");
            }
        };

        // Task2 will be the second task to call `or_insert_with_if` for the same
        // key, so its async block will not be evaluated. Once task1's async block
        // finishes, it will get the value inserted by task1's async block.
        let task2 = {
            let cache2 = cache.clone();
            async move {
                // Wait for 100 ms before calling `or_insert_with_if`.
                sleep(Duration::from_millis(100)).await;
                let entry = cache2
                    .entry(KEY)
                    .or_insert_with_if(async { unreachable!() }, |_v| unreachable!())
                    .await;
                // Entry should not be fresh because task1's async block should have
                // been evaluated instead of ours.
                assert!(!entry.is_fresh());
                assert_eq!(entry.into_value(), "task1");
            }
        };

        // Task3 will be the third task to call `or_insert_with_if` for the same key.
        // By the time it calls, task1's async block should have finished already and
        // the value should be already inserted to the cache. Also task3's
        // `replace_if` closure returns `false`. So its async block will not be
        // evaluated and will get the value inserted by task1's async block
        // immediately.
        let task3 = {
            let cache3 = cache.clone();
            async move {
                // Wait for 350 ms before calling `or_insert_with_if`.
                sleep(Duration::from_millis(350)).await;
                let entry = cache3
                    .entry(KEY)
                    .or_insert_with_if(async { unreachable!() }, |v| {
                        assert_eq!(v, &"task1");
                        false
                    })
                    .await;
                assert!(!entry.is_fresh());
                assert_eq!(entry.into_value(), "task1");
            }
        };

        // Task4 will be the fourth task to call `or_insert_with_if` for the same
        // key. The value should have been already inserted to the cache by task1.
        // However task4's `replace_if` closure returns `true`. So its async block
        // will be evaluated to replace the current value.
        let task4 = {
            let cache4 = cache.clone();
            async move {
                // Wait for 400 ms before calling `or_insert_with_if`.
                sleep(Duration::from_millis(400)).await;
                let entry = cache4
                    .entry(KEY)
                    .or_insert_with_if(async { "task4" }, |v| {
                        assert_eq!(v, &"task1");
                        true
                    })
                    .await;
                assert!(entry.is_fresh());
                assert_eq!(entry.into_value(), "task4");
            }
        };

        // Task5 will call `get` for the same key. It will call when task1's async
        // block is still running, so it will get none for the key.
        let task5 = {
            let cache5 = cache.clone();
            async move {
                // Wait for 200 ms before calling `get`.
                sleep(Duration::from_millis(200)).await;
                let maybe_v = cache5.get(&KEY).await;
                assert!(maybe_v.is_none());
            }
        };

        // Task6 will call `get` for the same key. It will call after task1's async
        // block finished, so it will get the value insert by task1's async block.
        let task6 = {
            let cache6 = cache.clone();
            async move {
                // Wait for 350 ms before calling `get`.
                sleep(Duration::from_millis(350)).await;
                let maybe_v = cache6.get(&KEY).await;
                assert_eq!(maybe_v, Some("task1"));
            }
        };

        // Task7 will call `get` for the same key. It will call after task4's async
        // block finished, so it will get the value insert by task4's async block.
        let task7 = {
            let cache7 = cache.clone();
            async move {
                // Wait for 450 ms before calling `get`.
                sleep(Duration::from_millis(450)).await;
                let maybe_v = cache7.get(&KEY).await;
                assert_eq!(maybe_v, Some("task4"));
            }
        };

        futures_util::join!(task1, task2, task3, task4, task5, task6, task7);

        assert!(cache.is_waiter_map_empty());
    }

    #[tokio::test]
    async fn entry_by_ref_or_insert_with_if() {
        let cache = Cache::new(100);
        const KEY: &u32 = &0;

        // This test will run seven async tasks:
        //
        // Task1 will be the first task to call `or_insert_with_if` for a key, so its
        // async block will be evaluated and then a &str value "task1" will be
        // inserted to the cache.
        let task1 = {
            let cache1 = cache.clone();
            async move {
                // Call `or_insert_with_if` immediately.
                let entry = cache1
                    .entry_by_ref(KEY)
                    .or_insert_with_if(
                        async {
                            // Wait for 300 ms and return a &str value.
                            sleep(Duration::from_millis(300)).await;
                            "task1"
                        },
                        |_v| unreachable!(),
                    )
                    .await;
                // Entry should be fresh because our async block should have been
                // evaluated.
                assert!(entry.is_fresh());
                assert_eq!(entry.into_value(), "task1");
            }
        };

        // Task2 will be the second task to call `or_insert_with_if` for the same
        // key, so its async block will not be evaluated. Once task1's async block
        // finishes, it will get the value inserted by task1's async block.
        let task2 = {
            let cache2 = cache.clone();
            async move {
                // Wait for 100 ms before calling `or_insert_with_if`.
                sleep(Duration::from_millis(100)).await;
                let entry = cache2
                    .entry_by_ref(KEY)
                    .or_insert_with_if(async { unreachable!() }, |_v| unreachable!())
                    .await;
                // Entry should not be fresh because task1's async block should have
                // been evaluated instead of ours.
                assert!(!entry.is_fresh());
                assert_eq!(entry.into_value(), "task1");
            }
        };

        // Task3 will be the third task to call `or_insert_with_if` for the same key.
        // By the time it calls, task1's async block should have finished already and
        // the value should be already inserted to the cache. Also task3's
        // `replace_if` closure returns `false`. So its async block will not be
        // evaluated and will get the value inserted by task1's async block
        // immediately.
        let task3 = {
            let cache3 = cache.clone();
            async move {
                // Wait for 350 ms before calling `or_insert_with_if`.
                sleep(Duration::from_millis(350)).await;
                let entry = cache3
                    .entry_by_ref(KEY)
                    .or_insert_with_if(async { unreachable!() }, |v| {
                        assert_eq!(v, &"task1");
                        false
                    })
                    .await;
                assert!(!entry.is_fresh());
                assert_eq!(entry.into_value(), "task1");
            }
        };

        // Task4 will be the fourth task to call `or_insert_with_if` for the same
        // key. The value should have been already inserted to the cache by task1.
        // However task4's `replace_if` closure returns `true`. So its async block
        // will be evaluated to replace the current value.
        let task4 = {
            let cache4 = cache.clone();
            async move {
                // Wait for 400 ms before calling `or_insert_with_if`.
                sleep(Duration::from_millis(400)).await;
                let entry = cache4
                    .entry_by_ref(KEY)
                    .or_insert_with_if(async { "task4" }, |v| {
                        assert_eq!(v, &"task1");
                        true
                    })
                    .await;
                assert!(entry.is_fresh());
                assert_eq!(entry.into_value(), "task4");
            }
        };

        // Task5 will call `get` for the same key. It will call when task1's async
        // block is still running, so it will get none for the key.
        let task5 = {
            let cache5 = cache.clone();
            async move {
                // Wait for 200 ms before calling `get`.
                sleep(Duration::from_millis(200)).await;
                let maybe_v = cache5.get(KEY).await;
                assert!(maybe_v.is_none());
            }
        };

        // Task6 will call `get` for the same key. It will call after task1's async
        // block finished, so it will get the value insert by task1's async block.
        let task6 = {
            let cache6 = cache.clone();
            async move {
                // Wait for 350 ms before calling `get`.
                sleep(Duration::from_millis(350)).await;
                let maybe_v = cache6.get(KEY).await;
                assert_eq!(maybe_v, Some("task1"));
            }
        };

        // Task7 will call `get` for the same key. It will call after task4's async
        // block finished, so it will get the value insert by task4's async block.
        let task7 = {
            let cache7 = cache.clone();
            async move {
                // Wait for 450 ms before calling `get`.
                sleep(Duration::from_millis(450)).await;
                let maybe_v = cache7.get(KEY).await;
                assert_eq!(maybe_v, Some("task4"));
            }
        };

        futures_util::join!(task1, task2, task3, task4, task5, task6, task7);

        assert!(cache.is_waiter_map_empty());
    }

    #[tokio::test]
    async fn try_get_with() {
        use std::sync::Arc;

        // Note that MyError does not implement std::error::Error trait
        // like anyhow::Error.
        #[derive(Debug)]
        pub struct MyError(#[allow(dead_code)] String);

        type MyResult<T> = Result<T, Arc<MyError>>;

        let cache = Cache::new(100);
        const KEY: u32 = 0;

        // This test will run eight async tasks:
        //
        // Task1 will be the first task to call `get_with` for a key, so its async
        // block will be evaluated and then an error will be returned. Nothing will
        // be inserted to the cache.
        let task1 = {
            let cache1 = cache.clone();
            async move {
                // Call `try_get_with` immediately.
                let v = cache1
                    .try_get_with(KEY, async {
                        // Wait for 300 ms and return an error.
                        sleep(Duration::from_millis(300)).await;
                        Err(MyError("task1 error".into()))
                    })
                    .await;
                assert!(v.is_err());
            }
        };

        // Task2 will be the second task to call `get_with` for the same key, so its
        // async block will not be evaluated. Once task1's async block finishes, it
        // will get the same error value returned by task1's async block.
        let task2 = {
            let cache2 = cache.clone();
            async move {
                // Wait for 100 ms before calling `try_get_with`.
                sleep(Duration::from_millis(100)).await;
                let v: MyResult<_> = cache2.try_get_with(KEY, async { unreachable!() }).await;
                assert!(v.is_err());
            }
        };

        // Task3 will be the third task to call `get_with` for the same key. By the
        // time it calls, task1's async block should have finished already, but the
        // key still does not exist in the cache. So its async block will be
        // evaluated and then an okay &str value will be returned. That value will be
        // inserted to the cache.
        let task3 = {
            let cache3 = cache.clone();
            async move {
                // Wait for 400 ms before calling `try_get_with`.
                sleep(Duration::from_millis(400)).await;
                let v: MyResult<_> = cache3
                    .try_get_with(KEY, async {
                        // Wait for 300 ms and return an Ok(&str) value.
                        sleep(Duration::from_millis(300)).await;
                        Ok("task3")
                    })
                    .await;
                assert_eq!(v.unwrap(), "task3");
            }
        };

        // Task4 will be the fourth task to call `get_with` for the same key. So its
        // async block will not be evaluated. Once task3's async block finishes, it
        // will get the same okay &str value.
        let task4 = {
            let cache4 = cache.clone();
            async move {
                // Wait for 500 ms before calling `try_get_with`.
                sleep(Duration::from_millis(500)).await;
                let v: MyResult<_> = cache4.try_get_with(KEY, async { unreachable!() }).await;
                assert_eq!(v.unwrap(), "task3");
            }
        };

        // Task5 will be the fifth task to call `get_with` for the same key. So its
        // async block will not be evaluated. By the time it calls, task3's async
        // block should have finished already, so its async block will not be
        // evaluated and will get the value insert by task3's async block
        // immediately.
        let task5 = {
            let cache5 = cache.clone();
            async move {
                // Wait for 800 ms before calling `try_get_with`.
                sleep(Duration::from_millis(800)).await;
                let v: MyResult<_> = cache5.try_get_with(KEY, async { unreachable!() }).await;
                assert_eq!(v.unwrap(), "task3");
            }
        };

        // Task6 will call `get` for the same key. It will call when task1's async
        // block is still running, so it will get none for the key.
        let task6 = {
            let cache6 = cache.clone();
            async move {
                // Wait for 200 ms before calling `get`.
                sleep(Duration::from_millis(200)).await;
                let maybe_v = cache6.get(&KEY).await;
                assert!(maybe_v.is_none());
            }
        };

        // Task7 will call `get` for the same key. It will call after task1's async
        // block finished with an error. So it will get none for the key.
        let task7 = {
            let cache7 = cache.clone();
            async move {
                // Wait for 400 ms before calling `get`.
                sleep(Duration::from_millis(400)).await;
                let maybe_v = cache7.get(&KEY).await;
                assert!(maybe_v.is_none());
            }
        };

        // Task8 will call `get` for the same key. It will call after task3's async
        // block finished, so it will get the value insert by task3's async block.
        let task8 = {
            let cache8 = cache.clone();
            async move {
                // Wait for 800 ms before calling `get`.
                sleep(Duration::from_millis(800)).await;
                let maybe_v = cache8.get(&KEY).await;
                assert_eq!(maybe_v, Some("task3"));
            }
        };

        futures_util::join!(task1, task2, task3, task4, task5, task6, task7, task8);

        assert!(cache.is_waiter_map_empty());
    }

    #[tokio::test]
    async fn try_get_with_by_ref() {
        use std::sync::Arc;

        // Note that MyError does not implement std::error::Error trait
        // like anyhow::Error.
        #[derive(Debug)]
        pub struct MyError(#[allow(dead_code)] String);

        type MyResult<T> = Result<T, Arc<MyError>>;

        let cache = Cache::new(100);
        const KEY: &u32 = &0;

        // This test will run eight async tasks:
        //
        // Task1 will be the first task to call `try_get_with_by_ref` for a key, so
        // its async block will be evaluated and then an error will be returned.
        // Nothing will be inserted to the cache.
        let task1 = {
            let cache1 = cache.clone();
            async move {
                // Call `try_get_with_by_ref` immediately.
                let v = cache1
                    .try_get_with_by_ref(KEY, async {
                        // Wait for 300 ms and return an error.
                        sleep(Duration::from_millis(300)).await;
                        Err(MyError("task1 error".into()))
                    })
                    .await;
                assert!(v.is_err());
            }
        };

        // Task2 will be the second task to call `get_with` for the same key, so its
        // async block will not be evaluated. Once task1's async block finishes, it
        // will get the same error value returned by task1's async block.
        let task2 = {
            let cache2 = cache.clone();
            async move {
                // Wait for 100 ms before calling `try_get_with_by_ref`.
                sleep(Duration::from_millis(100)).await;
                let v: MyResult<_> = cache2
                    .try_get_with_by_ref(KEY, async { unreachable!() })
                    .await;
                assert!(v.is_err());
            }
        };

        // Task3 will be the third task to call `get_with` for the same key. By the
        // time it calls, task1's async block should have finished already, but the
        // key still does not exist in the cache. So its async block will be
        // evaluated and then an okay &str value will be returned. That value will be
        // inserted to the cache.
        let task3 = {
            let cache3 = cache.clone();
            async move {
                // Wait for 400 ms before calling `try_get_with_by_ref`.
                sleep(Duration::from_millis(400)).await;
                let v: MyResult<_> = cache3
                    .try_get_with_by_ref(KEY, async {
                        // Wait for 300 ms and return an Ok(&str) value.
                        sleep(Duration::from_millis(300)).await;
                        Ok("task3")
                    })
                    .await;
                assert_eq!(v.unwrap(), "task3");
            }
        };

        // Task4 will be the fourth task to call `get_with` for the same key. So its
        // async block will not be evaluated. Once task3's async block finishes, it
        // will get the same okay &str value.
        let task4 = {
            let cache4 = cache.clone();
            async move {
                // Wait for 500 ms before calling `try_get_with_by_ref`.
                sleep(Duration::from_millis(500)).await;
                let v: MyResult<_> = cache4
                    .try_get_with_by_ref(KEY, async { unreachable!() })
                    .await;
                assert_eq!(v.unwrap(), "task3");
            }
        };

        // Task5 will be the fifth task to call `get_with` for the same key. So its
        // async block will not be evaluated. By the time it calls, task3's async
        // block should have finished already, so its async block will not be
        // evaluated and will get the value insert by task3's async block
        // immediately.
        let task5 = {
            let cache5 = cache.clone();
            async move {
                // Wait for 800 ms before calling `try_get_with_by_ref`.
                sleep(Duration::from_millis(800)).await;
                let v: MyResult<_> = cache5
                    .try_get_with_by_ref(KEY, async { unreachable!() })
                    .await;
                assert_eq!(v.unwrap(), "task3");
            }
        };

        // Task6 will call `get` for the same key. It will call when task1's async
        // block is still running, so it will get none for the key.
        let task6 = {
            let cache6 = cache.clone();
            async move {
                // Wait for 200 ms before calling `get`.
                sleep(Duration::from_millis(200)).await;
                let maybe_v = cache6.get(KEY).await;
                assert!(maybe_v.is_none());
            }
        };

        // Task7 will call `get` for the same key. It will call after task1's async
        // block finished with an error. So it will get none for the key.
        let task7 = {
            let cache7 = cache.clone();
            async move {
                // Wait for 400 ms before calling `get`.
                sleep(Duration::from_millis(400)).await;
                let maybe_v = cache7.get(KEY).await;
                assert!(maybe_v.is_none());
            }
        };

        // Task8 will call `get` for the same key. It will call after task3's async
        // block finished, so it will get the value insert by task3's async block.
        let task8 = {
            let cache8 = cache.clone();
            async move {
                // Wait for 800 ms before calling `get`.
                sleep(Duration::from_millis(800)).await;
                let maybe_v = cache8.get(KEY).await;
                assert_eq!(maybe_v, Some("task3"));
            }
        };

        futures_util::join!(task1, task2, task3, task4, task5, task6, task7, task8);

        assert!(cache.is_waiter_map_empty());
    }

    #[tokio::test]
    async fn optionally_get_with() {
        let cache = Cache::new(100);
        const KEY: u32 = 0;

        // This test will run eight async tasks:
        //
        // Task1 will be the first task to call `optionally_get_with` for a key,
        // so its async block will be evaluated and then an None will be
        // returned. Nothing will be inserted to the cache.
        let task1 = {
            let cache1 = cache.clone();
            async move {
                // Call `try_get_with` immediately.
                let v = cache1
                    .optionally_get_with(KEY, async {
                        // Wait for 300 ms and return an None.
                        sleep(Duration::from_millis(300)).await;
                        None
                    })
                    .await;
                assert!(v.is_none());
            }
        };

        // Task2 will be the second task to call `optionally_get_with` for the same
        // key, so its async block will not be evaluated. Once task1's async block
        // finishes, it will get the same error value returned by task1's async
        // block.
        let task2 = {
            let cache2 = cache.clone();
            async move {
                // Wait for 100 ms before calling `optionally_get_with`.
                sleep(Duration::from_millis(100)).await;
                let v = cache2
                    .optionally_get_with(KEY, async { unreachable!() })
                    .await;
                assert!(v.is_none());
            }
        };

        // Task3 will be the third task to call `optionally_get_with` for the
        // same key. By the time it calls, task1's async block should have
        // finished already, but the key still does not exist in the cache. So
        // its async block will be evaluated and then an okay &str value will be
        // returned. That value will be inserted to the cache.
        let task3 = {
            let cache3 = cache.clone();
            async move {
                // Wait for 400 ms before calling `optionally_get_with`.
                sleep(Duration::from_millis(400)).await;
                let v = cache3
                    .optionally_get_with(KEY, async {
                        // Wait for 300 ms and return an Some(&str) value.
                        sleep(Duration::from_millis(300)).await;
                        Some("task3")
                    })
                    .await;
                assert_eq!(v.unwrap(), "task3");
            }
        };

        // Task4 will be the fourth task to call `optionally_get_with` for the
        // same key. So its async block will not be evaluated. Once task3's
        // async block finishes, it will get the same okay &str value.
        let task4 = {
            let cache4 = cache.clone();
            async move {
                // Wait for 500 ms before calling `try_get_with`.
                sleep(Duration::from_millis(500)).await;
                let v = cache4
                    .optionally_get_with(KEY, async { unreachable!() })
                    .await;
                assert_eq!(v.unwrap(), "task3");
            }
        };

        // Task5 will be the fifth task to call `optionally_get_with` for the
        // same key. So its async block will not be evaluated. By the time it
        // calls, task3's async block should have finished already, so its async
        // block will not be evaluated and will get the value insert by task3's
        // async block immediately.
        let task5 = {
            let cache5 = cache.clone();
            async move {
                // Wait for 800 ms before calling `optionally_get_with`.
                sleep(Duration::from_millis(800)).await;
                let v = cache5
                    .optionally_get_with(KEY, async { unreachable!() })
                    .await;
                assert_eq!(v.unwrap(), "task3");
            }
        };

        // Task6 will call `get` for the same key. It will call when task1's async
        // block is still running, so it will get none for the key.
        let task6 = {
            let cache6 = cache.clone();
            async move {
                // Wait for 200 ms before calling `get`.
                sleep(Duration::from_millis(200)).await;
                let maybe_v = cache6.get(&KEY).await;
                assert!(maybe_v.is_none());
            }
        };

        // Task7 will call `get` for the same key. It will call after task1's async
        // block finished with an error. So it will get none for the key.
        let task7 = {
            let cache7 = cache.clone();
            async move {
                // Wait for 400 ms before calling `get`.
                sleep(Duration::from_millis(400)).await;
                let maybe_v = cache7.get(&KEY).await;
                assert!(maybe_v.is_none());
            }
        };

        // Task8 will call `get` for the same key. It will call after task3's async
        // block finished, so it will get the value insert by task3's async block.
        let task8 = {
            let cache8 = cache.clone();
            async move {
                // Wait for 800 ms before calling `get`.
                sleep(Duration::from_millis(800)).await;
                let maybe_v = cache8.get(&KEY).await;
                assert_eq!(maybe_v, Some("task3"));
            }
        };

        futures_util::join!(task1, task2, task3, task4, task5, task6, task7, task8);
    }

    #[tokio::test]
    async fn optionally_get_with_by_ref() {
        let cache = Cache::new(100);
        const KEY: &u32 = &0;

        // This test will run eight async tasks:
        //
        // Task1 will be the first task to call `optionally_get_with_by_ref` for a
        // key, so its async block will be evaluated and then an None will be
        // returned. Nothing will be inserted to the cache.
        let task1 = {
            let cache1 = cache.clone();
            async move {
                // Call `try_get_with` immediately.
                let v = cache1
                    .optionally_get_with_by_ref(KEY, async {
                        // Wait for 300 ms and return an None.
                        sleep(Duration::from_millis(300)).await;
                        None
                    })
                    .await;
                assert!(v.is_none());
            }
        };

        // Task2 will be the second task to call `optionally_get_with_by_ref` for the
        // same key, so its async block will not be evaluated. Once task1's async
        // block finishes, it will get the same error value returned by task1's async
        // block.
        let task2 = {
            let cache2 = cache.clone();
            async move {
                // Wait for 100 ms before calling `optionally_get_with_by_ref`.
                sleep(Duration::from_millis(100)).await;
                let v = cache2
                    .optionally_get_with_by_ref(KEY, async { unreachable!() })
                    .await;
                assert!(v.is_none());
            }
        };

        // Task3 will be the third task to call `optionally_get_with_by_ref` for the
        // same key. By the time it calls, task1's async block should have
        // finished already, but the key still does not exist in the cache. So
        // its async block will be evaluated and then an okay &str value will be
        // returned. That value will be inserted to the cache.
        let task3 = {
            let cache3 = cache.clone();
            async move {
                // Wait for 400 ms before calling `optionally_get_with_by_ref`.
                sleep(Duration::from_millis(400)).await;
                let v = cache3
                    .optionally_get_with_by_ref(KEY, async {
                        // Wait for 300 ms and return an Some(&str) value.
                        sleep(Duration::from_millis(300)).await;
                        Some("task3")
                    })
                    .await;
                assert_eq!(v.unwrap(), "task3");
            }
        };

        // Task4 will be the fourth task to call `optionally_get_with_by_ref` for the
        // same key. So its async block will not be evaluated. Once task3's
        // async block finishes, it will get the same okay &str value.
        let task4 = {
            let cache4 = cache.clone();
            async move {
                // Wait for 500 ms before calling `try_get_with`.
                sleep(Duration::from_millis(500)).await;
                let v = cache4
                    .optionally_get_with_by_ref(KEY, async { unreachable!() })
                    .await;
                assert_eq!(v.unwrap(), "task3");
            }
        };

        // Task5 will be the fifth task to call `optionally_get_with_by_ref` for the
        // same key. So its async block will not be evaluated. By the time it
        // calls, task3's async block should have finished already, so its async
        // block will not be evaluated and will get the value insert by task3's
        // async block immediately.
        let task5 = {
            let cache5 = cache.clone();
            async move {
                // Wait for 800 ms before calling `optionally_get_with_by_ref`.
                sleep(Duration::from_millis(800)).await;
                let v = cache5
                    .optionally_get_with_by_ref(KEY, async { unreachable!() })
                    .await;
                assert_eq!(v.unwrap(), "task3");
            }
        };

        // Task6 will call `get` for the same key. It will call when task1's async
        // block is still running, so it will get none for the key.
        let task6 = {
            let cache6 = cache.clone();
            async move {
                // Wait for 200 ms before calling `get`.
                sleep(Duration::from_millis(200)).await;
                let maybe_v = cache6.get(KEY).await;
                assert!(maybe_v.is_none());
            }
        };

        // Task7 will call `get` for the same key. It will call after task1's async
        // block finished with an error. So it will get none for the key.
        let task7 = {
            let cache7 = cache.clone();
            async move {
                // Wait for 400 ms before calling `get`.
                sleep(Duration::from_millis(400)).await;
                let maybe_v = cache7.get(KEY).await;
                assert!(maybe_v.is_none());
            }
        };

        // Task8 will call `get` for the same key. It will call after task3's async
        // block finished, so it will get the value insert by task3's async block.
        let task8 = {
            let cache8 = cache.clone();
            async move {
                // Wait for 800 ms before calling `get`.
                sleep(Duration::from_millis(800)).await;
                let maybe_v = cache8.get(KEY).await;
                assert_eq!(maybe_v, Some("task3"));
            }
        };

        futures_util::join!(task1, task2, task3, task4, task5, task6, task7, task8);

        assert!(cache.is_waiter_map_empty());
    }

    #[tokio::test]
    async fn upsert_with() {
        let cache = Cache::new(100);
        const KEY: u32 = 0;

        // Spawn three async tasks to call `and_upsert_with` for the same key and
        // each task increments the current value by 1. Ensure the key-level lock is
        // working by verifying the value is 3 after all tasks finish.
        //
        // |        |  task 1  |  task 2  |  task 3  |
        // |--------|----------|----------|----------|
        // |   0 ms | get none |          |          |
        // | 100 ms |          | blocked  |          |
        // | 200 ms | insert 1 |          |          |
        // |        |          | get 1    |          |
        // | 300 ms |          |          | blocked  |
        // | 400 ms |          | insert 2 |          |
        // |        |          |          | get 2    |
        // | 500 ms |          |          | insert 3 |

        let task1 = {
            let cache1 = cache.clone();
            async move {
                cache1
                    .entry(KEY)
                    .and_upsert_with(|maybe_entry| async move {
                        sleep(Duration::from_millis(200)).await;
                        assert!(maybe_entry.is_none());
                        1
                    })
                    .await
            }
        };

        let task2 = {
            let cache2 = cache.clone();
            async move {
                sleep(Duration::from_millis(100)).await;
                cache2
                    .entry_by_ref(&KEY)
                    .and_upsert_with(|maybe_entry| async move {
                        sleep(Duration::from_millis(200)).await;
                        let entry = maybe_entry.expect("The entry should exist");
                        entry.into_value() + 1
                    })
                    .await
            }
        };

        let task3 = {
            let cache3 = cache.clone();
            async move {
                sleep(Duration::from_millis(300)).await;
                cache3
                    .entry_by_ref(&KEY)
                    .and_upsert_with(|maybe_entry| async move {
                        sleep(Duration::from_millis(100)).await;
                        let entry = maybe_entry.expect("The entry should exist");
                        entry.into_value() + 1
                    })
                    .await
            }
        };

        let (ent1, ent2, ent3) = futures_util::join!(task1, task2, task3);
        assert_eq!(ent1.into_value(), 1);
        assert_eq!(ent2.into_value(), 2);
        assert_eq!(ent3.into_value(), 3);

        assert_eq!(cache.get(&KEY).await, Some(3));

        assert!(cache.is_waiter_map_empty());
    }

    #[tokio::test]
    async fn compute_with() {
        use crate::ops::compute;
        use tokio::sync::RwLock;

        let cache = Cache::new(100);
        const KEY: u32 = 0;

        // Spawn six async tasks to call `and_compute_with` for the same key. Ensure
        // the key-level lock is working by verifying the value after all tasks
        // finish.
        //
        // |         |   task 1   |    task 2     |   task 3   |  task 4  |   task 5   | task 6  |
        // |---------|------------|---------------|------------|----------|------------|---------|
        // |    0 ms | get none   |               |            |          |            |         |
        // |  100 ms |            | blocked       |            |          |            |         |
        // |  200 ms | insert [1] |               |            |          |            |         |
        // |         |            | get [1]       |            |          |            |         |
        // |  300 ms |            |               | blocked    |          |            |         |
        // |  400 ms |            | insert [1, 2] |            |          |            |         |
        // |         |            |               | get [1, 2] |          |            |         |
        // |  500 ms |            |               |            | blocked  |            |         |
        // |  600 ms |            |               | remove     |          |            |         |
        // |         |            |               |            | get none |            |         |
        // |  700 ms |            |               |            |          | blocked    |         |
        // |  800 ms |            |               |            | nop      |            |         |
        // |         |            |               |            |          | get none   |         |
        // |  900 ms |            |               |            |          |            | blocked |
        // | 1000 ms |            |               |            |          | insert [5] |         |
        // |         |            |               |            |          |            | get [5] |
        // | 1100 ms |            |               |            |          |            | nop     |

        let task1 = {
            let cache1 = cache.clone();
            async move {
                cache1
                    .entry(KEY)
                    .and_compute_with(|maybe_entry| async move {
                        sleep(Duration::from_millis(200)).await;
                        assert!(maybe_entry.is_none());
                        compute::Op::Put(Arc::new(RwLock::new(vec![1])))
                    })
                    .await
            }
        };

        let task2 = {
            let cache2 = cache.clone();
            async move {
                sleep(Duration::from_millis(100)).await;
                cache2
                    .entry_by_ref(&KEY)
                    .and_compute_with(|maybe_entry| async move {
                        let entry = maybe_entry.expect("The entry should exist");
                        let value = entry.into_value();
                        assert_eq!(*value.read().await, vec![1]);
                        sleep(Duration::from_millis(200)).await;
                        value.write().await.push(2);
                        compute::Op::Put(value)
                    })
                    .await
            }
        };

        let task3 = {
            let cache3 = cache.clone();
            async move {
                sleep(Duration::from_millis(300)).await;
                cache3
                    .entry(KEY)
                    .and_compute_with(|maybe_entry| async move {
                        let entry = maybe_entry.expect("The entry should exist");
                        let value = entry.into_value();
                        assert_eq!(*value.read().await, vec![1, 2]);
                        sleep(Duration::from_millis(200)).await;
                        compute::Op::Remove
                    })
                    .await
            }
        };

        let task4 = {
            let cache4 = cache.clone();
            async move {
                sleep(Duration::from_millis(500)).await;
                cache4
                    .entry(KEY)
                    .and_compute_with(|maybe_entry| async move {
                        assert!(maybe_entry.is_none());
                        sleep(Duration::from_millis(200)).await;
                        compute::Op::Nop
                    })
                    .await
            }
        };

        let task5 = {
            let cache5 = cache.clone();
            async move {
                sleep(Duration::from_millis(700)).await;
                cache5
                    .entry_by_ref(&KEY)
                    .and_compute_with(|maybe_entry| async move {
                        assert!(maybe_entry.is_none());
                        sleep(Duration::from_millis(200)).await;
                        compute::Op::Put(Arc::new(RwLock::new(vec![5])))
                    })
                    .await
            }
        };

        let task6 = {
            let cache6 = cache.clone();
            async move {
                sleep(Duration::from_millis(900)).await;
                cache6
                    .entry_by_ref(&KEY)
                    .and_compute_with(|maybe_entry| async move {
                        let entry = maybe_entry.expect("The entry should exist");
                        let value = entry.into_value();
                        assert_eq!(*value.read().await, vec![5]);
                        sleep(Duration::from_millis(100)).await;
                        compute::Op::Nop
                    })
                    .await
            }
        };

        let (res1, res2, res3, res4, res5, res6) =
            futures_util::join!(task1, task2, task3, task4, task5, task6);

        let compute::CompResult::Inserted(entry) = res1 else {
            panic!("Expected `Inserted`. Got {res1:?}")
        };
        assert_eq!(
            *entry.into_value().read().await,
            vec![1, 2] // The same Vec was modified by task2.
        );

        let compute::CompResult::ReplacedWith(entry) = res2 else {
            panic!("Expected `ReplacedWith`. Got {res2:?}")
        };
        assert_eq!(*entry.into_value().read().await, vec![1, 2]);

        let compute::CompResult::Removed(entry) = res3 else {
            panic!("Expected `Removed`. Got {res3:?}")
        };
        assert_eq!(*entry.into_value().read().await, vec![1, 2]);

        let compute::CompResult::StillNone(key) = res4 else {
            panic!("Expected `StillNone`. Got {res4:?}")
        };
        assert_eq!(*key, KEY);

        let compute::CompResult::Inserted(entry) = res5 else {
            panic!("Expected `Inserted`. Got {res5:?}")
        };
        assert_eq!(*entry.into_value().read().await, vec![5]);

        let compute::CompResult::Unchanged(entry) = res6 else {
            panic!("Expected `Unchanged`. Got {res6:?}")
        };
        assert_eq!(*entry.into_value().read().await, vec![5]);

        assert!(cache.is_waiter_map_empty());
    }

    #[tokio::test]
    async fn try_compute_with() {
        use crate::ops::compute;
        use tokio::sync::RwLock;

        let cache: Cache<u32, Arc<RwLock<Vec<i32>>>> = Cache::new(100);
        const KEY: u32 = 0;

        // Spawn four async tasks to call `and_try_compute_with` for the same key.
        // Ensure the key-level lock is working by verifying the value after all
        // tasks finish.
        //
        // |         |   task 1   |    task 2     |   task 3   |  task 4    |
        // |---------|------------|---------------|------------|------------|
        // |    0 ms | get none   |               |            |            |
        // |  100 ms |            | blocked       |            |            |
        // |  200 ms | insert [1] |               |            |            |
        // |         |            | get [1]       |            |            |
        // |  300 ms |            |               | blocked    |            |
        // |  400 ms |            | insert [1, 2] |            |            |
        // |         |            |               | get [1, 2] |            |
        // |  500 ms |            |               |            | blocked    |
        // |  600 ms |            |               | err        |            |
        // |         |            |               |            | get [1, 2] |
        // |  700 ms |            |               |            | remove     |
        //
        // This test is shorter than `compute_with` test because this one omits `Nop`
        // cases.

        let task1 = {
            let cache1 = cache.clone();
            async move {
                cache1
                    .entry(KEY)
                    .and_try_compute_with(|maybe_entry| async move {
                        sleep(Duration::from_millis(200)).await;
                        assert!(maybe_entry.is_none());
                        Ok(compute::Op::Put(Arc::new(RwLock::new(vec![1])))) as Result<_, ()>
                    })
                    .await
            }
        };

        let task2 = {
            let cache2 = cache.clone();
            async move {
                sleep(Duration::from_millis(100)).await;
                cache2
                    .entry_by_ref(&KEY)
                    .and_try_compute_with(|maybe_entry| async move {
                        let entry = maybe_entry.expect("The entry should exist");
                        let value = entry.into_value();
                        assert_eq!(*value.read().await, vec![1]);
                        sleep(Duration::from_millis(200)).await;
                        value.write().await.push(2);
                        Ok(compute::Op::Put(value)) as Result<_, ()>
                    })
                    .await
            }
        };

        let task3 = {
            let cache3 = cache.clone();
            async move {
                sleep(Duration::from_millis(300)).await;
                cache3
                    .entry(KEY)
                    .and_try_compute_with(|maybe_entry| async move {
                        let entry = maybe_entry.expect("The entry should exist");
                        let value = entry.into_value();
                        assert_eq!(*value.read().await, vec![1, 2]);
                        sleep(Duration::from_millis(200)).await;
                        Err(())
                    })
                    .await
            }
        };

        let task4 = {
            let cache4 = cache.clone();
            async move {
                sleep(Duration::from_millis(500)).await;
                cache4
                    .entry(KEY)
                    .and_try_compute_with(|maybe_entry| async move {
                        let entry = maybe_entry.expect("The entry should exist");
                        let value = entry.into_value();
                        assert_eq!(*value.read().await, vec![1, 2]);
                        sleep(Duration::from_millis(100)).await;
                        Ok(compute::Op::Remove) as Result<_, ()>
                    })
                    .await
            }
        };

        let (res1, res2, res3, res4) = futures_util::join!(task1, task2, task3, task4);

        let Ok(compute::CompResult::Inserted(entry)) = res1 else {
            panic!("Expected `Inserted`. Got {res1:?}")
        };
        assert_eq!(
            *entry.into_value().read().await,
            vec![1, 2] // The same Vec was modified by task2.
        );

        let Ok(compute::CompResult::ReplacedWith(entry)) = res2 else {
            panic!("Expected `ReplacedWith`. Got {res2:?}")
        };
        assert_eq!(*entry.into_value().read().await, vec![1, 2]);

        assert!(res3.is_err());

        let Ok(compute::CompResult::Removed(entry)) = res4 else {
            panic!("Expected `Removed`. Got {res4:?}")
        };
        assert_eq!(
            *entry.into_value().read().await,
            vec![1, 2] // Removed value.
        );

        assert!(cache.is_waiter_map_empty());
    }

    #[tokio::test]
    // https://github.com/moka-rs/moka/issues/43
    async fn handle_panic_in_get_with() {
        use tokio::time::{sleep, Duration};

        let cache = Cache::new(16);
        let semaphore = Arc::new(tokio::sync::Semaphore::new(0));
        {
            let cache_ref = cache.clone();
            let semaphore_ref = semaphore.clone();
            tokio::task::spawn(async move {
                let _ = cache_ref
                    .get_with(1, async move {
                        semaphore_ref.add_permits(1);
                        sleep(Duration::from_millis(50)).await;
                        panic!("Panic during try_get_with");
                    })
                    .await;
            });
        }
        let _ = semaphore.acquire().await.expect("semaphore acquire failed");
        assert_eq!(cache.get_with(1, async { 5 }).await, 5);
    }

    #[tokio::test]
    // https://github.com/moka-rs/moka/issues/43
    async fn handle_panic_in_try_get_with() {
        use tokio::time::{sleep, Duration};

        let cache = Cache::new(16);
        let semaphore = Arc::new(tokio::sync::Semaphore::new(0));
        {
            let cache_ref = cache.clone();
            let semaphore_ref = semaphore.clone();
            tokio::task::spawn(async move {
                let _ = cache_ref
                    .try_get_with(1, async move {
                        semaphore_ref.add_permits(1);
                        sleep(Duration::from_millis(50)).await;
                        panic!("Panic during try_get_with");
                    })
                    .await as Result<_, Arc<Infallible>>;
            });
        }
        let _ = semaphore.acquire().await.expect("semaphore acquire failed");
        assert_eq!(
            cache.try_get_with(1, async { Ok(5) }).await as Result<_, Arc<Infallible>>,
            Ok(5)
        );

        assert!(cache.is_waiter_map_empty());
    }

    #[tokio::test]
    // https://github.com/moka-rs/moka/issues/59
    async fn abort_get_with() {
        use tokio::time::{sleep, Duration};

        let cache = Cache::new(16);
        let semaphore = Arc::new(tokio::sync::Semaphore::new(0));

        let handle;
        {
            let cache_ref = cache.clone();
            let semaphore_ref = semaphore.clone();

            handle = tokio::task::spawn(async move {
                let _ = cache_ref
                    .get_with(1, async move {
                        semaphore_ref.add_permits(1);
                        sleep(Duration::from_millis(50)).await;
                        unreachable!();
                    })
                    .await;
            });
        }

        let _ = semaphore.acquire().await.expect("semaphore acquire failed");
        handle.abort();

        assert_eq!(cache.get_with(1, async { 5 }).await, 5);

        assert!(cache.is_waiter_map_empty());
    }

    #[tokio::test]
    // https://github.com/moka-rs/moka/issues/59
    async fn abort_try_get_with() {
        use tokio::time::{sleep, Duration};

        let cache = Cache::new(16);
        let semaphore = Arc::new(tokio::sync::Semaphore::new(0));

        let handle;
        {
            let cache_ref = cache.clone();
            let semaphore_ref = semaphore.clone();

            handle = tokio::task::spawn(async move {
                let _ = cache_ref
                    .try_get_with(1, async move {
                        semaphore_ref.add_permits(1);
                        sleep(Duration::from_millis(50)).await;
                        unreachable!();
                    })
                    .await as Result<_, Arc<Infallible>>;
            });
        }

        let _ = semaphore.acquire().await.expect("semaphore acquire failed");
        handle.abort();

        assert_eq!(
            cache.try_get_with(1, async { Ok(5) }).await as Result<_, Arc<Infallible>>,
            Ok(5)
        );

        assert!(cache.is_waiter_map_empty());
    }

    #[tokio::test]
    async fn test_removal_notifications() {
        // The following `Vec`s will hold actual and expected notifications.
        let actual = Arc::new(Mutex::new(Vec::new()));
        let mut expected = Vec::new();

        // Create an eviction listener.
        let a1 = Arc::clone(&actual);
        let listener = move |k, v, cause| -> ListenerFuture {
            let a2 = Arc::clone(&a1);
            async move {
                a2.lock().await.push((k, v, cause));
            }
            .boxed()
        };

        // Create a cache with the eviction listener.
        let mut cache = Cache::builder()
            .max_capacity(3)
            .async_eviction_listener(listener)
            .build();
        cache.reconfigure_for_testing().await;

        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert('a', "alice").await;
        cache.invalidate(&'a').await;
        expected.push((Arc::new('a'), "alice", RemovalCause::Explicit));

        cache.run_pending_tasks().await;
        assert_eq!(cache.entry_count(), 0);

        cache.insert('b', "bob").await;
        cache.insert('c', "cathy").await;
        cache.insert('d', "david").await;
        cache.run_pending_tasks().await;
        assert_eq!(cache.entry_count(), 3);

        // This will be rejected due to the size constraint.
        cache.insert('e', "emily").await;
        expected.push((Arc::new('e'), "emily", RemovalCause::Size));
        cache.run_pending_tasks().await;
        assert_eq!(cache.entry_count(), 3);

        // Raise the popularity of 'e' so it will be accepted next time.
        cache.get(&'e').await;
        cache.run_pending_tasks().await;

        // Retry.
        cache.insert('e', "eliza").await;
        // and the LRU entry will be evicted.
        expected.push((Arc::new('b'), "bob", RemovalCause::Size));
        cache.run_pending_tasks().await;
        assert_eq!(cache.entry_count(), 3);

        // Replace an existing entry.
        cache.insert('d', "dennis").await;
        expected.push((Arc::new('d'), "david", RemovalCause::Replaced));
        cache.run_pending_tasks().await;
        assert_eq!(cache.entry_count(), 3);

        verify_notification_vec(&cache, actual, &expected).await;
    }

    #[tokio::test]
    async fn test_removal_notifications_with_updates() {
        // The following `Vec`s will hold actual and expected notifications.
        let actual = Arc::new(Mutex::new(Vec::new()));
        let mut expected = Vec::new();

        // Create an eviction listener.
        let a1 = Arc::clone(&actual);
        let listener = move |k, v, cause| -> ListenerFuture {
            let a2 = Arc::clone(&a1);
            async move {
                a2.lock().await.push((k, v, cause));
            }
            .boxed()
        };

        let (clock, mock) = Clock::mock();

        // Create a cache with the eviction listener and also TTL and TTI.
        let mut cache = Cache::builder()
            .async_eviction_listener(listener)
            .time_to_live(Duration::from_secs(7))
            .time_to_idle(Duration::from_secs(5))
            .clock(clock)
            .build();
        cache.reconfigure_for_testing().await;

        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert("alice", "a0").await;
        cache.run_pending_tasks().await;

        // Now alice (a0) has been expired by the idle timeout (TTI).
        mock.increment(Duration::from_secs(6));
        expected.push((Arc::new("alice"), "a0", RemovalCause::Expired));
        assert_eq!(cache.get(&"alice").await, None);

        // We have not ran sync after the expiration of alice (a0), so it is
        // still in the cache.
        assert_eq!(cache.entry_count(), 1);

        // Re-insert alice with a different value. Since alice (a0) is still
        // in the cache, this is actually a replace operation rather than an
        // insert operation. We want to verify that the RemovalCause of a0 is
        // Expired, not Replaced.
        cache.insert("alice", "a1").await;
        cache.run_pending_tasks().await;

        mock.increment(Duration::from_secs(4));
        assert_eq!(cache.get(&"alice").await, Some("a1"));
        cache.run_pending_tasks().await;

        // Now alice has been expired by time-to-live (TTL).
        mock.increment(Duration::from_secs(4));
        expected.push((Arc::new("alice"), "a1", RemovalCause::Expired));
        assert_eq!(cache.get(&"alice").await, None);

        // But, again, it is still in the cache.
        assert_eq!(cache.entry_count(), 1);

        // Re-insert alice with a different value and verify that the
        // RemovalCause of a1 is Expired (not Replaced).
        cache.insert("alice", "a2").await;
        cache.run_pending_tasks().await;

        assert_eq!(cache.entry_count(), 1);

        // Now alice (a2) has been expired by the idle timeout.
        mock.increment(Duration::from_secs(6));
        expected.push((Arc::new("alice"), "a2", RemovalCause::Expired));
        assert_eq!(cache.get(&"alice").await, None);
        assert_eq!(cache.entry_count(), 1);

        // This invalidate will internally remove alice (a2).
        cache.invalidate(&"alice").await;
        cache.run_pending_tasks().await;
        assert_eq!(cache.entry_count(), 0);

        // Re-insert, and this time, make it expired by the TTL.
        cache.insert("alice", "a3").await;
        cache.run_pending_tasks().await;
        mock.increment(Duration::from_secs(4));
        assert_eq!(cache.get(&"alice").await, Some("a3"));
        cache.run_pending_tasks().await;
        mock.increment(Duration::from_secs(4));
        expected.push((Arc::new("alice"), "a3", RemovalCause::Expired));
        assert_eq!(cache.get(&"alice").await, None);
        assert_eq!(cache.entry_count(), 1);

        // This invalidate will internally remove alice (a2).
        cache.invalidate(&"alice").await;
        cache.run_pending_tasks().await;
        assert_eq!(cache.entry_count(), 0);

        verify_notification_vec(&cache, actual, &expected).await;
    }

    // When the eviction listener is not set, calling `run_pending_tasks` once should
    // evict all entries that can be removed.
    #[tokio::test]
    async fn no_batch_size_limit_on_eviction() {
        const MAX_CAPACITY: u64 = 20;

        const EVICTION_TIMEOUT: Duration = Duration::from_nanos(0);
        const MAX_LOG_SYNC_REPEATS: u32 = 1;
        const EVICTION_BATCH_SIZE: u32 = 1;

        let hk_conf = HousekeeperConfig::new(
            // Timeout should be ignored when the eviction listener is not provided.
            Some(EVICTION_TIMEOUT),
            Some(MAX_LOG_SYNC_REPEATS),
            Some(EVICTION_BATCH_SIZE),
        );

        // Create a cache with the LRU policy.
        let mut cache = Cache::builder()
            .max_capacity(MAX_CAPACITY)
            .eviction_policy(EvictionPolicy::lru())
            .housekeeper_config(hk_conf)
            .build();
        cache.reconfigure_for_testing().await;

        // Make the cache exterior immutable.
        let cache = cache;

        // Fill the cache.
        for i in 0..MAX_CAPACITY {
            let v = format!("v{i}");
            cache.insert(i, v).await
        }
        // The max capacity should not change because we have not called
        // `run_pending_tasks` yet.
        assert_eq!(cache.entry_count(), 0);

        cache.run_pending_tasks().await;
        assert_eq!(cache.entry_count(), MAX_CAPACITY);

        // Insert more items to the cache.
        for i in MAX_CAPACITY..(MAX_CAPACITY * 2) {
            let v = format!("v{i}");
            cache.insert(i, v).await
        }
        // The max capacity should not change because we have not called
        // `run_pending_tasks` yet.
        assert_eq!(cache.entry_count(), MAX_CAPACITY);
        // Both old and new keys should exist.
        assert!(cache.contains_key(&0)); // old
        assert!(cache.contains_key(&(MAX_CAPACITY - 1))); // old
        assert!(cache.contains_key(&(MAX_CAPACITY * 2 - 1))); // new

        // Process the remaining write op logs (there should be MAX_CAPACITY logs),
        // and evict the LRU entries.
        cache.run_pending_tasks().await;
        assert_eq!(cache.entry_count(), MAX_CAPACITY);

        // Now all the old keys should be gone.
        assert!(!cache.contains_key(&0));
        assert!(!cache.contains_key(&(MAX_CAPACITY - 1)));
        // And the new keys should exist.
        assert!(cache.contains_key(&(MAX_CAPACITY * 2 - 1)));
    }

    #[tokio::test]
    async fn slow_eviction_listener() {
        const MAX_CAPACITY: u64 = 20;

        const EVICTION_TIMEOUT: Duration = Duration::from_millis(30);
        const LISTENER_DELAY: Duration = Duration::from_millis(11);
        const MAX_LOG_SYNC_REPEATS: u32 = 1;
        const EVICTION_BATCH_SIZE: u32 = 1;

        let hk_conf = HousekeeperConfig::new(
            Some(EVICTION_TIMEOUT),
            Some(MAX_LOG_SYNC_REPEATS),
            Some(EVICTION_BATCH_SIZE),
        );

        let (clock, mock) = Clock::mock();
        let listener_call_count = Arc::new(AtomicU8::new(0));
        let lcc = Arc::clone(&listener_call_count);

        // A slow eviction listener that spend `LISTENER_DELAY` to process a removal
        // notification.
        let listener = move |_k, _v, _cause| {
            mock.increment(LISTENER_DELAY);
            lcc.fetch_add(1, Ordering::AcqRel);
        };

        // Create a cache with the LRU policy.
        let mut cache = Cache::builder()
            .max_capacity(MAX_CAPACITY)
            .eviction_policy(EvictionPolicy::lru())
            .eviction_listener(listener)
            .housekeeper_config(hk_conf)
            .clock(clock)
            .build();
        cache.reconfigure_for_testing().await;

        // Make the cache exterior immutable.
        let cache = cache;

        // Fill the cache.
        for i in 0..MAX_CAPACITY {
            let v = format!("v{i}");
            cache.insert(i, v).await
        }
        // The max capacity should not change because we have not called
        // `run_pending_tasks` yet.
        assert_eq!(cache.entry_count(), 0);

        cache.run_pending_tasks().await;
        assert_eq!(listener_call_count.load(Ordering::Acquire), 0);
        assert_eq!(cache.entry_count(), MAX_CAPACITY);

        // Insert more items to the cache.
        for i in MAX_CAPACITY..(MAX_CAPACITY * 2) {
            let v = format!("v{i}");
            cache.insert(i, v).await
        }
        assert_eq!(cache.entry_count(), MAX_CAPACITY);

        cache.run_pending_tasks().await;
        // Because of the slow listener, cache should get an over capacity.
        let mut expected_call_count = 3;
        assert_eq!(
            listener_call_count.load(Ordering::Acquire) as u64,
            expected_call_count
        );
        assert_eq!(cache.entry_count(), MAX_CAPACITY * 2 - expected_call_count);

        loop {
            cache.run_pending_tasks().await;

            expected_call_count += 3;
            if expected_call_count > MAX_CAPACITY {
                expected_call_count = MAX_CAPACITY;
            }

            let actual_count = listener_call_count.load(Ordering::Acquire) as u64;
            assert_eq!(actual_count, expected_call_count);
            let expected_entry_count = MAX_CAPACITY * 2 - expected_call_count;
            assert_eq!(cache.entry_count(), expected_entry_count);

            if expected_call_count >= MAX_CAPACITY {
                break;
            }
        }

        assert_eq!(cache.entry_count(), MAX_CAPACITY);
    }

    // NOTE: To enable the panic logging, run the following command:
    //
    // RUST_LOG=moka=info cargo test --features 'future, logging' -- \
    //   future::cache::tests::recover_from_panicking_eviction_listener --exact --nocapture
    //
    #[tokio::test]
    async fn recover_from_panicking_eviction_listener() {
        #[cfg(feature = "logging")]
        let _ = env_logger::builder().is_test(true).try_init();

        // The following `Vec`s will hold actual and expected notifications.
        let actual = Arc::new(Mutex::new(Vec::new()));
        let mut expected = Vec::new();

        // Create an eviction listener that panics when it see
        // a value "panic now!".
        let a1 = Arc::clone(&actual);
        let listener = move |k, v, cause| -> ListenerFuture {
            let a2 = Arc::clone(&a1);
            async move {
                if v == "panic now!" {
                    panic!("Panic now!");
                }
                a2.lock().await.push((k, v, cause));
            }
            .boxed()
        };

        // Create a cache with the eviction listener.
        let mut cache = Cache::builder()
            .name("My Future Cache")
            .async_eviction_listener(listener)
            .build();
        cache.reconfigure_for_testing().await;

        // Make the cache exterior immutable.
        let cache = cache;

        // Insert an okay value.
        cache.insert("alice", "a0").await;
        cache.run_pending_tasks().await;

        // Insert a value that will cause the eviction listener to panic.
        cache.insert("alice", "panic now!").await;
        expected.push((Arc::new("alice"), "a0", RemovalCause::Replaced));
        cache.run_pending_tasks().await;

        // Insert an okay value. This will replace the previous
        // value "panic now!" so the eviction listener will panic.
        cache.insert("alice", "a2").await;
        cache.run_pending_tasks().await;
        // No more removal notification should be sent.

        // Invalidate the okay value.
        cache.invalidate(&"alice").await;
        cache.run_pending_tasks().await;

        verify_notification_vec(&cache, actual, &expected).await;
    }

    #[tokio::test]
    async fn cancel_future_while_running_pending_tasks() {
        use crate::future::FutureExt;
        use futures_util::future::poll_immediate;
        use tokio::task::yield_now;

        let listener_initiation_count: Arc<AtomicU32> = Default::default();
        let listener_completion_count: Arc<AtomicU32> = Default::default();

        let listener = {
            // Variables to capture.
            let init_count = Arc::clone(&listener_initiation_count);
            let comp_count = Arc::clone(&listener_completion_count);

            // Our eviction listener closure.
            move |_k, _v, _r| {
                init_count.fetch_add(1, Ordering::AcqRel);
                let comp_count1 = Arc::clone(&comp_count);

                async move {
                    yield_now().await;
                    comp_count1.fetch_add(1, Ordering::AcqRel);
                }
                .boxed()
            }
        };

        let (clock, mock) = Clock::mock();

        let mut cache: Cache<u32, u32> = Cache::builder()
            .time_to_live(Duration::from_millis(10))
            .async_eviction_listener(listener)
            .clock(clock)
            .build();

        cache.reconfigure_for_testing().await;

        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert(1, 1).await;
        assert_eq!(cache.run_pending_tasks_initiation_count(), 0);
        assert_eq!(cache.run_pending_tasks_completion_count(), 0);

        // Key 1 is not yet expired.
        mock.increment(Duration::from_millis(7));

        cache.run_pending_tasks().await;
        assert_eq!(cache.run_pending_tasks_initiation_count(), 1);
        assert_eq!(cache.run_pending_tasks_completion_count(), 1);
        assert_eq!(listener_initiation_count.load(Ordering::Acquire), 0);
        assert_eq!(listener_completion_count.load(Ordering::Acquire), 0);

        // Now key 1 is expired, so the eviction listener should be called when we
        // call run_pending_tasks() and poll the returned future.
        mock.increment(Duration::from_millis(7));

        let fut = cache.run_pending_tasks();
        // Poll the fut only once, and drop it. The fut should not be completed (so
        // it is cancelled) because the eviction listener performed a yield_now().
        assert!(poll_immediate(fut).await.is_none());

        // The task is initiated but not completed.
        assert_eq!(cache.run_pending_tasks_initiation_count(), 2);
        assert_eq!(cache.run_pending_tasks_completion_count(), 1);
        // The listener is initiated but not completed.
        assert_eq!(listener_initiation_count.load(Ordering::Acquire), 1);
        assert_eq!(listener_completion_count.load(Ordering::Acquire), 0);

        // This will resume the task and the listener, and continue polling
        // until complete.
        cache.run_pending_tasks().await;
        // Now the task is completed.
        assert_eq!(cache.run_pending_tasks_initiation_count(), 2);
        assert_eq!(cache.run_pending_tasks_completion_count(), 2);
        // Now the listener is completed.
        assert_eq!(listener_initiation_count.load(Ordering::Acquire), 1);
        assert_eq!(listener_completion_count.load(Ordering::Acquire), 1);
    }

    #[tokio::test]
    async fn cancel_future_while_calling_eviction_listener() {
        use crate::future::FutureExt;
        use futures_util::future::poll_immediate;
        use tokio::task::yield_now;

        let listener_initiation_count: Arc<AtomicU32> = Default::default();
        let listener_completion_count: Arc<AtomicU32> = Default::default();

        let listener = {
            // Variables to capture.
            let init_count = Arc::clone(&listener_initiation_count);
            let comp_count = Arc::clone(&listener_completion_count);

            // Our eviction listener closure.
            move |_k, _v, _r| {
                init_count.fetch_add(1, Ordering::AcqRel);
                let comp_count1 = Arc::clone(&comp_count);

                async move {
                    yield_now().await;
                    comp_count1.fetch_add(1, Ordering::AcqRel);
                }
                .boxed()
            }
        };

        let mut cache: Cache<u32, u32> = Cache::builder()
            .time_to_live(Duration::from_millis(10))
            .async_eviction_listener(listener)
            .build();

        cache.reconfigure_for_testing().await;

        // Make the cache exterior immutable.
        let cache = cache;

        // ------------------------------------------------------------
        // Interrupt the eviction listener while calling `insert`
        // ------------------------------------------------------------

        cache.insert(1, 1).await;

        let fut = cache.insert(1, 2);
        // Poll the fut only once, and drop it. The fut should not be completed (so
        // it is cancelled) because the eviction listener performed a yield_now().
        assert!(poll_immediate(fut).await.is_none());

        // The listener is initiated but not completed.
        assert_eq!(listener_initiation_count.load(Ordering::Acquire), 1);
        assert_eq!(listener_completion_count.load(Ordering::Acquire), 0);

        // This will call retry_interrupted_ops() and resume the interrupted
        // listener.
        cache.run_pending_tasks().await;
        assert_eq!(listener_initiation_count.load(Ordering::Acquire), 1);
        assert_eq!(listener_completion_count.load(Ordering::Acquire), 1);

        // ------------------------------------------------------------
        // Interrupt the eviction listener while calling `invalidate`
        // ------------------------------------------------------------

        let fut = cache.invalidate(&1);
        // Cancel the fut after one poll.
        assert!(poll_immediate(fut).await.is_none());
        // The listener is initiated but not completed.
        assert_eq!(listener_initiation_count.load(Ordering::Acquire), 2);
        assert_eq!(listener_completion_count.load(Ordering::Acquire), 1);

        // This will call retry_interrupted_ops() and resume the interrupted
        // listener.
        cache.get(&99).await;
        assert_eq!(listener_initiation_count.load(Ordering::Acquire), 2);
        assert_eq!(listener_completion_count.load(Ordering::Acquire), 2);

        // ------------------------------------------------------------
        // Ensure retry_interrupted_ops() is called
        // ------------------------------------------------------------

        // Repeat the same test with `insert`, but this time, call different methods
        // to ensure retry_interrupted_ops() is called.
        let prepare = || async {
            cache.invalidate(&1).await;

            // Reset the counters.
            listener_initiation_count.store(0, Ordering::Release);
            listener_completion_count.store(0, Ordering::Release);

            cache.insert(1, 1).await;

            let fut = cache.insert(1, 2);
            // Poll the fut only once, and drop it. The fut should not be completed (so
            // it is cancelled) because the eviction listener performed a yield_now().
            assert!(poll_immediate(fut).await.is_none());

            // The listener is initiated but not completed.
            assert_eq!(listener_initiation_count.load(Ordering::Acquire), 1);
            assert_eq!(listener_completion_count.load(Ordering::Acquire), 0);
        };

        // Methods to test:
        //
        // - run_pending_tasks (Already tested in a previous test)
        // - get               (Already tested in a previous test)
        // - insert
        // - invalidate
        // - remove

        // insert
        prepare().await;
        cache.insert(99, 99).await;
        assert_eq!(listener_initiation_count.load(Ordering::Acquire), 1);
        assert_eq!(listener_completion_count.load(Ordering::Acquire), 1);

        // invalidate
        prepare().await;
        cache.invalidate(&88).await;
        assert_eq!(listener_initiation_count.load(Ordering::Acquire), 1);
        assert_eq!(listener_completion_count.load(Ordering::Acquire), 1);

        // remove
        prepare().await;
        cache.remove(&77).await;
        assert_eq!(listener_initiation_count.load(Ordering::Acquire), 1);
        assert_eq!(listener_completion_count.load(Ordering::Acquire), 1);
    }

    #[tokio::test]
    async fn cancel_future_while_scheduling_write_op() {
        use futures_util::future::poll_immediate;

        let mut cache: Cache<u32, u32> = Cache::builder().build();
        cache.reconfigure_for_testing().await;

        // Make the cache exterior immutable.
        let cache = cache;

        // --------------------------------------------------------------
        // Interrupt `insert` while blocking in `schedule_write_op`
        // --------------------------------------------------------------

        cache
            .schedule_write_op_should_block
            .store(true, Ordering::Release);
        let fut = cache.insert(1, 1);
        // Poll the fut only once, and drop it. The fut should not be completed (so
        // it is cancelled) because schedule_write_op should be awaiting for a lock.
        assert!(poll_immediate(fut).await.is_none());

        assert_eq!(cache.base.interrupted_op_ch_snd.len(), 1);
        assert_eq!(cache.base.write_op_ch.len(), 0);

        // This should retry the interrupted operation.
        cache
            .schedule_write_op_should_block
            .store(false, Ordering::Release);
        cache.get(&99).await;
        assert_eq!(cache.base.interrupted_op_ch_snd.len(), 0);
        assert_eq!(cache.base.write_op_ch.len(), 1);

        cache.run_pending_tasks().await;
        assert_eq!(cache.base.write_op_ch.len(), 0);

        // --------------------------------------------------------------
        // Interrupt `invalidate` while blocking in `schedule_write_op`
        // --------------------------------------------------------------

        cache
            .schedule_write_op_should_block
            .store(true, Ordering::Release);
        let fut = cache.invalidate(&1);
        // Poll the fut only once, and drop it. The fut should not be completed (so
        // it is cancelled) because schedule_write_op should be awaiting for a lock.
        assert!(poll_immediate(fut).await.is_none());

        assert_eq!(cache.base.interrupted_op_ch_snd.len(), 1);
        assert_eq!(cache.base.write_op_ch.len(), 0);

        // This should retry the interrupted operation.
        cache
            .schedule_write_op_should_block
            .store(false, Ordering::Release);
        cache.get(&99).await;
        assert_eq!(cache.base.interrupted_op_ch_snd.len(), 0);
        assert_eq!(cache.base.write_op_ch.len(), 1);

        cache.run_pending_tasks().await;
        assert_eq!(cache.base.write_op_ch.len(), 0);
    }

    // This test ensures that the `contains_key`, `get` and `invalidate` can use
    // borrowed form `&[u8]` for key with type `Vec<u8>`.
    // https://github.com/moka-rs/moka/issues/166
    #[tokio::test]
    async fn borrowed_forms_of_key() {
        let cache: Cache<Vec<u8>, ()> = Cache::new(1);

        let key = vec![1_u8];
        cache.insert(key.clone(), ()).await;

        // key as &Vec<u8>
        let key_v: &Vec<u8> = &key;
        assert!(cache.contains_key(key_v));
        assert_eq!(cache.get(key_v).await, Some(()));
        cache.invalidate(key_v).await;

        cache.insert(key, ()).await;

        // key as &[u8]
        let key_s: &[u8] = &[1_u8];
        assert!(cache.contains_key(key_s));
        assert_eq!(cache.get(key_s).await, Some(()));
        cache.invalidate(key_s).await;
    }

    #[tokio::test]
    async fn drop_value_immediately_after_eviction() {
        use crate::common::test_utils::{Counters, Value};

        const MAX_CAPACITY: u32 = 500;
        const KEYS: u32 = ((MAX_CAPACITY as f64) * 1.2) as u32;

        let counters = Arc::new(Counters::default());
        let counters1 = Arc::clone(&counters);

        let listener = move |_k, _v, cause| match cause {
            RemovalCause::Size => counters1.incl_evicted(),
            RemovalCause::Explicit => counters1.incl_invalidated(),
            _ => (),
        };

        let mut cache = Cache::builder()
            .max_capacity(MAX_CAPACITY as u64)
            .eviction_listener(listener)
            .build();
        cache.reconfigure_for_testing().await;

        // Make the cache exterior immutable.
        let cache = cache;

        for key in 0..KEYS {
            let value = Arc::new(Value::new(vec![0u8; 1024], &counters));
            cache.insert(key, value).await;
            counters.incl_inserted();
            cache.run_pending_tasks().await;
        }

        let eviction_count = KEYS - MAX_CAPACITY;

        // Retries will be needed when testing in a QEMU VM.
        const MAX_RETRIES: usize = 5;
        let mut retries = 0;
        loop {
            // Ensure all scheduled notifications have been processed.
            std::thread::sleep(Duration::from_millis(500));

            if counters.evicted() != eviction_count || counters.value_dropped() != eviction_count {
                if retries <= MAX_RETRIES {
                    retries += 1;
                    cache.run_pending_tasks().await;
                    continue;
                } else {
                    assert_eq!(counters.evicted(), eviction_count, "Retries exhausted");
                    assert_eq!(
                        counters.value_dropped(),
                        eviction_count,
                        "Retries exhausted"
                    );
                }
            }

            assert_eq!(counters.inserted(), KEYS, "inserted");
            assert_eq!(counters.value_created(), KEYS, "value_created");
            assert_eq!(counters.evicted(), eviction_count, "evicted");
            assert_eq!(counters.invalidated(), 0, "invalidated");
            assert_eq!(counters.value_dropped(), eviction_count, "value_dropped");

            break;
        }

        for key in 0..KEYS {
            cache.invalidate(&key).await;
            cache.run_pending_tasks().await;
        }

        let mut retries = 0;
        loop {
            // Ensure all scheduled notifications have been processed.
            std::thread::sleep(Duration::from_millis(500));

            if counters.invalidated() != MAX_CAPACITY || counters.value_dropped() != KEYS {
                if retries <= MAX_RETRIES {
                    retries += 1;
                    cache.run_pending_tasks().await;
                    continue;
                } else {
                    assert_eq!(counters.invalidated(), MAX_CAPACITY, "Retries exhausted");
                    assert_eq!(counters.value_dropped(), KEYS, "Retries exhausted");
                }
            }

            assert_eq!(counters.inserted(), KEYS, "inserted");
            assert_eq!(counters.value_created(), KEYS, "value_created");
            assert_eq!(counters.evicted(), eviction_count, "evicted");
            assert_eq!(counters.invalidated(), MAX_CAPACITY, "invalidated");
            assert_eq!(counters.value_dropped(), KEYS, "value_dropped");

            break;
        }

        std::mem::drop(cache);
        assert_eq!(counters.value_dropped(), KEYS, "value_dropped");
    }

    // https://github.com/moka-rs/moka/issues/383
    #[tokio::test]
    async fn ensure_gc_runs_when_dropping_cache() {
        let cache = Cache::builder().build();
        let val = Arc::new(0);
        cache
            .get_with(1, std::future::ready(Arc::clone(&val)))
            .await;
        drop(cache);
        assert_eq!(Arc::strong_count(&val), 1);
    }

    #[tokio::test]
    async fn test_debug_format() {
        let cache = Cache::new(10);
        cache.insert('a', "alice").await;
        cache.insert('b', "bob").await;
        cache.insert('c', "cindy").await;

        let debug_str = format!("{cache:?}");
        assert!(debug_str.starts_with('{'));
        assert!(debug_str.contains(r#"'a': "alice""#));
        assert!(debug_str.contains(r#"'b': "bob""#));
        assert!(debug_str.contains(r#"'c': "cindy""#));
        assert!(debug_str.ends_with('}'));
    }

    type NotificationTuple<K, V> = (Arc<K>, V, RemovalCause);

    async fn verify_notification_vec<K, V, S>(
        cache: &Cache<K, V, S>,
        actual: Arc<Mutex<Vec<NotificationTuple<K, V>>>>,
        expected: &[NotificationTuple<K, V>],
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

            let actual = &*actual.lock().await;
            if actual.len() != expected.len() {
                if retries <= MAX_RETRIES {
                    retries += 1;
                    cache.run_pending_tasks().await;
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
}
