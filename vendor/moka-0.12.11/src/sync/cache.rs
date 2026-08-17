use super::{
    base_cache::{BaseCache, HouseKeeperArc},
    value_initializer::{InitResult, ValueInitializer},
    CacheBuilder, OwnedKeyEntrySelector, RefKeyEntrySelector,
};
use crate::{
    common::{
        concurrent::{
            constants::WRITE_RETRY_INTERVAL_MICROS, housekeeper::InnerSync, Weigher, WriteOp,
        },
        iter::ScanningGet,
        time::{Clock, Instant},
        HousekeeperConfig,
    },
    notification::EvictionListener,
    ops::compute::{self, CompResult},
    policy::{EvictionPolicy, ExpirationPolicy},
    sync::{Iter, PredicateId},
    Entry, Policy, PredicateError,
};

use crossbeam_channel::{Sender, TrySendError};
use equivalent::Equivalent;
use std::{
    collections::hash_map::RandomState,
    fmt,
    hash::{BuildHasher, Hash},
    sync::Arc,
    time::Duration,
};

/// A thread-safe concurrent synchronous in-memory cache.
///
/// `Cache` supports full concurrency of retrievals and a high expected concurrency
/// for updates.
///
/// `Cache` utilizes a lock-free concurrent hash table as the central key-value
/// storage. `Cache` performs a best-effort bounding of the map using an entry
/// replacement algorithm to determine which entries to evict when the capacity is
/// exceeded.
///
/// # Table of Contents
///
/// - [Example: `insert`, `get` and `invalidate`](#example-insert-get-and-invalidate)
/// - [Avoiding to clone the value at `get`](#avoiding-to-clone-the-value-at-get)
/// - [Sharing a cache across threads](#sharing-a-cache-across-threads)
///     - [No lock is needed](#no-lock-is-needed)
/// - [Hashing Algorithm](#hashing-algorithm)
/// - [Example: Size-based Eviction](#example-size-based-eviction)
/// - [Example: Time-based Expirations](#example-time-based-expirations)
///     - [Cache-level TTL and TTI policies](#cache-level-ttl-and-tti-policies)
///     - [Per-entry expiration policy](#per-entry-expiration-policy)
/// - [Example: Eviction Listener](#example-eviction-listener)
///     - [You should avoid eviction listener to
///       panic](#you-should-avoid-eviction-listener-to-panic)
///
/// # Example: `insert`, `get` and `invalidate`
///
/// Cache entries are manually added using [`insert`](#method.insert) or
/// [`get_with`](#method.get_with) methods, and are stored in the cache until either
/// evicted or manually invalidated.
///
/// Here's an example of reading and updating a cache by using multiple threads:
///
/// ```rust
/// use moka::sync::Cache;
///
/// use std::thread;
///
/// fn value(n: usize) -> String {
///     format!("value {n}")
/// }
///
/// const NUM_THREADS: usize = 16;
/// const NUM_KEYS_PER_THREAD: usize = 64;
///
/// // Create a cache that can store up to 10,000 entries.
/// let cache = Cache::new(10_000);
///
/// // Spawn threads and read and update the cache simultaneously.
/// let threads: Vec<_> = (0..NUM_THREADS)
///     .map(|i| {
///         // To share the same cache across the threads, clone it.
///         // This is a cheap operation.
///         let my_cache = cache.clone();
///         let start = i * NUM_KEYS_PER_THREAD;
///         let end = (i + 1) * NUM_KEYS_PER_THREAD;
///
///         thread::spawn(move || {
///             // Insert 64 entries. (NUM_KEYS_PER_THREAD = 64)
///             for key in start..end {
///                 my_cache.insert(key, value(key));
///                 // get() returns Option<String>, a clone of the stored value.
///                 assert_eq!(my_cache.get(&key), Some(value(key)));
///             }
///
///             // Invalidate every 4 element of the inserted entries.
///             for key in (start..end).step_by(4) {
///                 my_cache.invalidate(&key);
///             }
///         })
///     })
///     .collect();
///
/// // Wait for all threads to complete.
/// threads.into_iter().for_each(|t| t.join().expect("Failed"));
///
/// // Verify the result.
/// for key in 0..(NUM_THREADS * NUM_KEYS_PER_THREAD) {
///     if key % 4 == 0 {
///         assert_eq!(cache.get(&key), None);
///     } else {
///         assert_eq!(cache.get(&key), Some(value(key)));
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
/// # Sharing a cache across threads
///
/// To share a cache across threads, do one of the followings:
///
/// - Create a clone of the cache by calling its `clone` method and pass it to other
///   thread.
/// - Wrap the cache by a `sync::OnceCell` or `sync::Lazy` from
///   [once_cell][once-cell-crate] create, and set it to a `static` variable.
///
/// Cloning is a cheap operation for `Cache` as it only creates thread-safe
/// reference-counted pointers to the internal data structures.
///
/// ## No lock is needed
///
/// Don't wrap a `Cache` by a lock such as `Mutex` or `RwLock`. All methods provided
/// by the `Cache` are considered thread-safe, and can be safely called by multiple
/// threads at the same time. No lock is needed.
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
/// use moka::sync::Cache;
///
/// // Evict based on the number of entries in the cache.
/// let cache = Cache::builder()
///     // Up to 10,000 entries.
///     .max_capacity(10_000)
///     // Create the cache.
///     .build();
/// cache.insert(1, "one".to_string());
///
/// // Evict based on the byte length of strings in the cache.
/// let cache = Cache::builder()
///     // A weigher closure takes &K and &V and returns a u32
///     // representing the relative size of the entry.
///     .weigher(|_key, value: &String| -> u32 {
///         value.len().try_into().unwrap_or(u32::MAX)
///     })
///     // This cache will hold up to 32MiB of values.
///     .max_capacity(32 * 1024 * 1024)
///     .build();
/// cache.insert(2, "two".to_string());
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
/// use moka::sync::Cache;
/// use std::time::Duration;
///
/// let cache = Cache::builder()
///     // Time to live (TTL): 30 minutes
///     .time_to_live(Duration::from_secs(30 * 60))
///     // Time to idle (TTI):  5 minutes
///     .time_to_idle(Duration::from_secs( 5 * 60))
///     // Create the cache.
///     .build();
///
/// // This entry will expire after 5 minutes (TTI) if there is no get().
/// cache.insert(0, "zero");
///
/// // This get() will extend the entry life for another 5 minutes.
/// cache.get(&0);
///
/// // Even though we keep calling get(), the entry will expire
/// // after 30 minutes (TTL) from the insert().
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
/// use moka::{sync::Cache, Expiry};
/// use std::time::{Duration, Instant};
///
/// // In this example, we will create a `sync::Cache` with `u32` as the key, and
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
/// // Create a `Cache<u32, (Expiration, String)>` with an expiry `MyExpiry` and
/// // eviction listener.
/// let expiry = MyExpiry;
///
/// let eviction_listener = |key, _value, cause| {
///     println!("Evicted key {key}. Cause: {cause:?}");
/// };
///
/// let cache = Cache::builder()
///     .max_capacity(100)
///     .expire_after(expiry)
///     .eviction_listener(eviction_listener)
///     .build();
///
/// // Insert some entries into the cache with different expirations.
/// cache.get_with(0, || (Expiration::AfterShortTime, "a".to_string()));
/// cache.get_with(1, || (Expiration::AfterLongTime, "b".to_string()));
/// cache.get_with(2, || (Expiration::Never, "c".to_string()));
///
/// // Verify that all the inserted entries exist.
/// assert!(cache.contains_key(&0));
/// assert!(cache.contains_key(&1));
/// assert!(cache.contains_key(&2));
///
/// // Sleep for 6 seconds. Key 0 should expire.
/// println!("\nSleeping for 6 seconds...\n");
/// std::thread::sleep(Duration::from_secs(6));
/// println!("Entry count: {}", cache.entry_count());
///
/// // Verify that key 0 has been evicted.
/// assert!(!cache.contains_key(&0));
/// assert!(cache.contains_key(&1));
/// assert!(cache.contains_key(&2));
///
/// // Sleep for 10 more seconds. Key 1 should expire.
/// println!("\nSleeping for 10 seconds...\n");
/// std::thread::sleep(Duration::from_secs(10));
/// println!("Entry count: {}", cache.entry_count());
///
/// // Verify that key 1 has been evicted.
/// assert!(!cache.contains_key(&1));
/// assert!(cache.contains_key(&2));
///
/// // Manually invalidate key 2.
/// cache.invalidate(&2);
/// assert!(!cache.contains_key(&2));
///
/// println!("\nSleeping for a second...\n");
/// std::thread::sleep(Duration::from_secs(1));
/// println!("Entry count: {}", cache.entry_count());
///
/// println!("\nDone!");
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
///
/// use moka::{sync::Cache, notification};
///
/// use anyhow::{anyhow, Context};
/// use std::{
///     fs, io,
///     path::{Path, PathBuf},
///     sync::{Arc, RwLock},
///     time::Duration,
/// };
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
///     fn write_data_file(
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
///         fs::write(&path, contents)?;
///         self.file_count += 1;
///         println!("Created a data file at {path:?} (file count: {})", self.file_count);
///         Ok(path)
///     }
///
///     fn read_data_file(&self, path: impl AsRef<Path>) -> io::Result<String> {
///         // Reads the contents of the file at the path, and return the contents.
///         fs::read_to_string(path)
///     }
///
///     fn remove_data_file(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
///         // Remove the file at the path.
///         fs::remove_file(path.as_ref())?;
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
/// fn main() -> anyhow::Result<()> {
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
///
///     // Create an eviction listener closure.
///     let eviction_listener = move |k, v: PathBuf, cause| {
///         // Try to remove the data file at the path `v`.
///         println!("\n== An entry has been evicted. k: {k:?}, v: {v:?}, cause: {cause:?}");
///
///         // Acquire the write lock of the DataFileManager. We must handle
///         // error cases here to prevent the listener from panicking.
///         match file_mgr1.write() {
///             Err(_e) => {
///                 eprintln!("The lock has been poisoned");
///             }
///             Ok(mut mgr) => {
///                 // Remove the data file using the DataFileManager.
///                 if let Err(_e) = mgr.remove_data_file(v.as_path()) {
///                     eprintln!("Failed to remove a data file at {v:?}");
///                 }
///             }
///         }
///     };
///
///     // Create the cache. Set time to live for two seconds and set the
///     // eviction listener.
///     let cache = Cache::builder()
///         .max_capacity(100)
///         .time_to_live(Duration::from_secs(2))
///         .eviction_listener(eviction_listener)
///         .build();
///
///     // Insert an entry to the cache.
///     // This will create and write a data file for the key "user1", store the
///     // path of the file to the cache, and return it.
///     println!("== try_get_with()");
///     let key = "user1";
///     let path = cache
///         .try_get_with(key, || -> anyhow::Result<_> {
///             let mut mgr = file_mgr
///                 .write()
///                 .map_err(|_e| anyhow::anyhow!("The lock has been poisoned"))?;
///             let path = mgr
///                 .write_data_file(key, "user data".into())
///                 .with_context(|| format!("Failed to create a data file"))?;
///             Ok(path)
///         })
///         .map_err(|e| anyhow!("{e}"))?;
///
///     // Read the data file at the path and print the contents.
///     println!("\n== read_data_file()");
///     {
///         let mgr = file_mgr
///             .read()
///             .map_err(|_e| anyhow::anyhow!("The lock has been poisoned"))?;
///         let contents = mgr
///             .read_data_file(path.as_path())
///             .with_context(|| format!("Failed to read data from {path:?}"))?;
///         println!("contents: {contents}");
///     }
///
///     // Sleep for five seconds. While sleeping, the cache entry for key "user1"
///     // will be expired and evicted, so the eviction listener will be called to
///     // remove the file.
///     std::thread::sleep(Duration::from_secs(5));
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
    /// to expiration. This inaccuracy can be mitigated by performing a
    /// `run_pending_tasks` first.
    ///
    /// # Example
    ///
    /// ```rust
    /// use moka::sync::Cache;
    ///
    /// let cache = Cache::new(10);
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
    /// // To mitigate the inaccuracy, Call `run_pending_tasks` method to run
    /// // pending internal tasks.
    /// cache.run_pending_tasks();
    ///
    /// // Followings will print the actual numbers.
    /// println!("{}", cache.entry_count());   // -> 3
    /// println!("{}", cache.weighted_size()); // -> 3
    /// ```
    ///
    pub fn entry_count(&self) -> u64 {
        self.base.entry_count()
    }

    /// Returns an approximate total weighted size of entries in this cache.
    ///
    /// The value returned is _an estimate_; the actual size may differ if there are
    /// concurrent insertions or removals, or if some entries are pending removal due
    /// to expiration. This inaccuracy can be mitigated by performing a
    /// `run_pending_tasks` first. See [`entry_count`](#method.entry_count) for a
    /// sample code.
    pub fn weighted_size(&self) -> u64 {
        self.base.weighted_size()
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

    /// Returns a [`CacheBuilder`][builder-struct], which can builds a `Cache` or
    /// `SegmentedCache` with various configuration knobs.
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
        eviction_listener: Option<EvictionListener<K, V>>,
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

    pub(crate) fn contains_key_with_hash<Q>(&self, key: &Q, hash: u64) -> bool
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.base.contains_key_with_hash(key, hash)
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
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.base
            .get_with_hash(key, self.base.hash(key), false)
            .map(Entry::into_value)
    }

    pub(crate) fn get_with_hash<Q>(&self, key: &Q, hash: u64, need_key: bool) -> Option<Entry<K, V>>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.base.get_with_hash(key, hash, need_key)
    }

    /// Takes a key `K` and returns an [`OwnedKeyEntrySelector`] that can be used to
    /// select or insert an entry.
    ///
    /// [`OwnedKeyEntrySelector`]: ./struct.OwnedKeyEntrySelector.html
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
    pub fn entry_by_ref<'a, Q>(&'a self, key: &'a Q) -> RefKeyEntrySelector<'a, K, Q, V, S>
    where
        Q: Equivalent<K> + ToOwned<Owned = K> + Hash + ?Sized,
    {
        let hash = self.base.hash(key);
        RefKeyEntrySelector::new(key, hash, self)
    }

    /// Returns a _clone_ of the value corresponding to the key. If the value does
    /// not exist, evaluates the `init` closure and inserts the output.
    ///
    /// # Concurrent calls on the same key
    ///
    /// This method guarantees that concurrent calls on the same not-existing key are
    /// coalesced into one evaluation of the `init` closure. Only one of the calls
    /// evaluates its closure, and other calls wait for that closure to complete.
    ///
    /// The following code snippet demonstrates this behavior:
    ///
    /// ```rust
    /// use moka::sync::Cache;
    /// use std::{sync::Arc, thread};
    ///
    /// const TEN_MIB: usize = 10 * 1024 * 1024; // 10MiB
    /// let cache = Cache::new(100);
    ///
    /// // Spawn four threads.
    /// let threads: Vec<_> = (0..4_u8)
    ///     .map(|task_id| {
    ///         let my_cache = cache.clone();
    ///         thread::spawn(move || {
    ///             println!("Thread {task_id} started.");
    ///
    ///             // Try to insert and get the value for key1. Although all four
    ///             // threads will call `get_with` at the same time, the `init` closure
    ///             // must be evaluated only once.
    ///             let value = my_cache.get_with("key1", || {
    ///                 println!("Thread {task_id} inserting a value.");
    ///                 Arc::new(vec![0u8; TEN_MIB])
    ///             });
    ///
    ///             // Ensure the value exists now.
    ///             assert_eq!(value.len(), TEN_MIB);
    ///             assert!(my_cache.get(&"key1").is_some());
    ///
    ///             println!("Thread {task_id} got the value. (len: {})", value.len());
    ///         })
    ///     })
    ///     .collect();
    ///
    /// // Wait all threads to complete.
    /// threads
    ///     .into_iter()
    ///     .for_each(|t| t.join().expect("Thread failed"));
    /// ```
    ///
    /// **Result**
    ///
    /// - The `init` closure was called exactly once by thread 1.
    /// - Other threads were blocked until thread 1 inserted the value.
    ///
    /// ```console
    /// Thread 1 started.
    /// Thread 0 started.
    /// Thread 3 started.
    /// Thread 2 started.
    /// Thread 1 inserting a value.
    /// Thread 2 got the value. (len: 10485760)
    /// Thread 1 got the value. (len: 10485760)
    /// Thread 0 got the value. (len: 10485760)
    /// Thread 3 got the value. (len: 10485760)
    /// ```
    ///
    /// # Panics
    ///
    /// This method panics when the `init` closure has panicked. When it happens,
    /// only the caller whose `init` closure panicked will get the panic (e.g. only
    /// thread 1 in the above sample). If there are other calls in progress (e.g.
    /// thread 0, 2 and 3 above), this method will restart and resolve one of the
    /// remaining `init` closure.
    ///
    pub fn get_with(&self, key: K, init: impl FnOnce() -> V) -> V {
        let hash = self.base.hash(&key);
        let key = Arc::new(key);
        let replace_if = None as Option<fn(&V) -> bool>;
        self.get_or_insert_with_hash_and_fun(key, hash, init, replace_if, false)
            .into_value()
    }

    /// Similar to [`get_with`](#method.get_with), but instead of passing an owned
    /// key, you can pass a reference to the key. If the key does not exist in the
    /// cache, the key will be cloned to create new entry in the cache.
    pub fn get_with_by_ref<Q>(&self, key: &Q, init: impl FnOnce() -> V) -> V
    where
        Q: Equivalent<K> + ToOwned<Owned = K> + Hash + ?Sized,
    {
        let hash = self.base.hash(key);
        let replace_if = None as Option<fn(&V) -> bool>;

        self.get_or_insert_with_hash_by_ref_and_fun(key, hash, init, replace_if, false)
            .into_value()
    }

    /// TODO: Remove this in v0.13.0.
    /// Deprecated, replaced with
    /// [`entry()::or_insert_with_if()`](./struct.OwnedKeyEntrySelector.html#method.or_insert_with_if)
    #[deprecated(since = "0.10.0", note = "Replaced with `entry().or_insert_with_if()`")]
    pub fn get_with_if(
        &self,
        key: K,
        init: impl FnOnce() -> V,
        replace_if: impl FnMut(&V) -> bool,
    ) -> V {
        let hash = self.base.hash(&key);
        let key = Arc::new(key);
        self.get_or_insert_with_hash_and_fun(key, hash, init, Some(replace_if), false)
            .into_value()
    }

    pub(crate) fn get_or_insert_with_hash_and_fun(
        &self,
        key: Arc<K>,
        hash: u64,
        init: impl FnOnce() -> V,
        mut replace_if: Option<impl FnMut(&V) -> bool>,
        need_key: bool,
    ) -> Entry<K, V> {
        self.base
            .get_with_hash_and_ignore_if(&*key, hash, replace_if.as_mut(), need_key)
            .unwrap_or_else(|| self.insert_with_hash_and_fun(key, hash, init, replace_if, need_key))
    }

    // Need to create new function instead of using the existing
    // `get_or_insert_with_hash_and_fun`. The reason is `by_ref` function will
    // require key reference to have `ToOwned` trait. If we modify the existing
    // `get_or_insert_with_hash_and_fun` function, it will require all the existing
    // apis that depends on it to make the `K` to have `ToOwned` trait.
    pub(crate) fn get_or_insert_with_hash_by_ref_and_fun<Q>(
        &self,
        key: &Q,
        hash: u64,
        init: impl FnOnce() -> V,
        mut replace_if: Option<impl FnMut(&V) -> bool>,
        need_key: bool,
    ) -> Entry<K, V>
    where
        Q: Equivalent<K> + ToOwned<Owned = K> + Hash + ?Sized,
    {
        self.base
            .get_with_hash_and_ignore_if(key, hash, replace_if.as_mut(), need_key)
            .unwrap_or_else(|| {
                let key = Arc::new(key.to_owned());
                self.insert_with_hash_and_fun(key, hash, init, replace_if, need_key)
            })
    }

    pub(crate) fn insert_with_hash_and_fun(
        &self,
        key: Arc<K>,
        hash: u64,
        init: impl FnOnce() -> V,
        mut replace_if: Option<impl FnMut(&V) -> bool>,
        need_key: bool,
    ) -> Entry<K, V> {
        let get = || {
            self.base
                .get_with_hash_without_recording(&*key, hash, replace_if.as_mut())
        };
        let insert = |v| self.insert_with_hash(key.clone(), hash, v);

        let k = if need_key {
            Some(Arc::clone(&key))
        } else {
            None
        };

        let type_id = ValueInitializer::<K, V, S>::type_id_for_get_with();
        let post_init = ValueInitializer::<K, V, S>::post_init_for_get_with;

        match self
            .value_initializer
            .try_init_or_read(&key, type_id, get, init, insert, post_init)
        {
            InitResult::Initialized(v) => {
                crossbeam_epoch::pin().flush();
                Entry::new(k, v, true, false)
            }
            InitResult::ReadExisting(v) => Entry::new(k, v, false, false),
            InitResult::InitErr(_) => unreachable!(),
        }
    }

    pub(crate) fn get_or_insert_with_hash(
        &self,
        key: Arc<K>,
        hash: u64,
        init: impl FnOnce() -> V,
    ) -> Entry<K, V> {
        match self.base.get_with_hash(&*key, hash, true) {
            Some(entry) => entry,
            None => {
                let value = init();
                self.insert_with_hash(Arc::clone(&key), hash, value.clone());
                Entry::new(Some(key), value, true, false)
            }
        }
    }

    pub(crate) fn get_or_insert_with_hash_by_ref<Q>(
        &self,
        key: &Q,
        hash: u64,
        init: impl FnOnce() -> V,
    ) -> Entry<K, V>
    where
        Q: Equivalent<K> + ToOwned<Owned = K> + Hash + ?Sized,
    {
        match self.base.get_with_hash(key, hash, true) {
            Some(entry) => entry,
            None => {
                let key = Arc::new(key.to_owned());
                let value = init();
                self.insert_with_hash(Arc::clone(&key), hash, value.clone());
                Entry::new(Some(key), value, true, false)
            }
        }
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
    ///
    /// The following code snippet demonstrates this behavior:
    ///
    /// ```rust
    /// use moka::sync::Cache;
    /// use std::{path::Path, thread};
    ///
    /// /// This function tries to get the file size in bytes.
    /// fn get_file_size(thread_id: u8, path: impl AsRef<Path>) -> Option<u64> {
    ///     println!("get_file_size() called by thread {thread_id}.");
    ///     std::fs::metadata(path).ok().map(|m| m.len())
    /// }
    ///
    /// let cache = Cache::new(100);
    ///
    /// // Spawn four threads.
    /// let threads: Vec<_> = (0..4_u8)
    ///     .map(|thread_id| {
    ///         let my_cache = cache.clone();
    ///         thread::spawn(move || {
    ///             println!("Thread {thread_id} started.");
    ///
    ///             // Try to insert and get the value for key1. Although all four
    ///             // threads will call `optionally_get_with` at the same time,
    ///             // get_file_size() must be called only once.
    ///             let value = my_cache.optionally_get_with(
    ///                 "key1",
    ///                 || get_file_size(thread_id, "./Cargo.toml"),
    ///             );
    ///
    ///             // Ensure the value exists now.
    ///             assert!(value.is_some());
    ///             assert!(my_cache.get(&"key1").is_some());
    ///
    ///             println!(
    ///                 "Thread {thread_id} got the value. (len: {})",
    ///                 value.unwrap()
    ///             );
    ///         })
    ///     })
    ///     .collect();
    ///
    /// // Wait all threads to complete.
    /// threads
    ///     .into_iter()
    ///     .for_each(|t| t.join().expect("Thread failed"));
    /// ```
    ///
    /// **Result**
    ///
    /// - `get_file_size()` was called exactly once by thread 0.
    /// - Other threads were blocked until thread 0 inserted the value.
    ///
    /// ```console
    /// Thread 0 started.
    /// Thread 1 started.
    /// Thread 2 started.
    /// get_file_size() called by thread 0.
    /// Thread 3 started.
    /// Thread 2 got the value. (len: 1466)
    /// Thread 0 got the value. (len: 1466)
    /// Thread 1 got the value. (len: 1466)
    /// Thread 3 got the value. (len: 1466)
    /// ```
    ///
    /// # Panics
    ///
    /// This method panics when the `init` closure has panicked. When it happens,
    /// only the caller whose `init` closure panicked will get the panic (e.g. only
    /// thread 1 in the above sample). If there are other calls in progress (e.g.
    /// thread 0, 2 and 3 above), this method will restart and resolve one of the
    /// remaining `init` closure.
    ///
    pub fn optionally_get_with<F>(&self, key: K, init: F) -> Option<V>
    where
        F: FnOnce() -> Option<V>,
    {
        let hash = self.base.hash(&key);
        let key = Arc::new(key);

        self.get_or_optionally_insert_with_hash_and_fun(key, hash, init, false)
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
        let hash = self.base.hash(key);
        self.get_or_optionally_insert_with_hash_by_ref_and_fun(key, hash, init, false)
            .map(Entry::into_value)
    }

    pub(super) fn get_or_optionally_insert_with_hash_and_fun<F>(
        &self,
        key: Arc<K>,
        hash: u64,
        init: F,
        need_key: bool,
    ) -> Option<Entry<K, V>>
    where
        F: FnOnce() -> Option<V>,
    {
        let entry = self.get_with_hash(&*key, hash, need_key);
        if entry.is_some() {
            return entry;
        }

        self.optionally_insert_with_hash_and_fun(key, hash, init, need_key)
    }

    pub(super) fn get_or_optionally_insert_with_hash_by_ref_and_fun<F, Q>(
        &self,
        key: &Q,
        hash: u64,
        init: F,
        need_key: bool,
    ) -> Option<Entry<K, V>>
    where
        F: FnOnce() -> Option<V>,
        Q: Equivalent<K> + ToOwned<Owned = K> + Hash + ?Sized,
    {
        let entry = self.get_with_hash(key, hash, need_key);
        if entry.is_some() {
            return entry;
        }

        let key = Arc::new(key.to_owned());
        self.optionally_insert_with_hash_and_fun(key, hash, init, need_key)
    }

    pub(super) fn optionally_insert_with_hash_and_fun<F>(
        &self,
        key: Arc<K>,
        hash: u64,
        init: F,
        need_key: bool,
    ) -> Option<Entry<K, V>>
    where
        F: FnOnce() -> Option<V>,
    {
        let get = || {
            let ignore_if = None as Option<&mut fn(&V) -> bool>;
            self.base
                .get_with_hash_without_recording(&*key, hash, ignore_if)
        };
        let insert = |v| self.insert_with_hash(key.clone(), hash, v);

        let k = if need_key {
            Some(Arc::clone(&key))
        } else {
            None
        };

        let type_id = ValueInitializer::<K, V, S>::type_id_for_optionally_get_with();
        let post_init = ValueInitializer::<K, V, S>::post_init_for_optionally_get_with;

        match self
            .value_initializer
            .try_init_or_read(&key, type_id, get, init, insert, post_init)
        {
            InitResult::Initialized(v) => {
                crossbeam_epoch::pin().flush();
                Some(Entry::new(k, v, true, false))
            }
            InitResult::ReadExisting(v) => Some(Entry::new(k, v, false, false)),
            InitResult::InitErr(_) => {
                crossbeam_epoch::pin().flush();
                None
            }
        }
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
    /// closure, and other calls wait for that closure to complete.
    ///
    /// The following code snippet demonstrates this behavior:
    ///
    /// ```rust
    /// use moka::sync::Cache;
    /// use std::{path::Path, thread};
    ///
    /// /// This function tries to get the file size in bytes.
    /// fn get_file_size(thread_id: u8, path: impl AsRef<Path>) -> Result<u64, std::io::Error> {
    ///     println!("get_file_size() called by thread {thread_id}.");
    ///     Ok(std::fs::metadata(path)?.len())
    /// }
    ///
    /// let cache = Cache::new(100);
    ///
    /// // Spawn four threads.
    /// let threads: Vec<_> = (0..4_u8)
    ///     .map(|thread_id| {
    ///         let my_cache = cache.clone();
    ///         thread::spawn(move || {
    ///             println!("Thread {thread_id} started.");
    ///
    ///             // Try to insert and get the value for key1. Although all four
    ///             // threads will call `try_get_with` at the same time,
    ///             // get_file_size() must be called only once.
    ///             let value = my_cache.try_get_with(
    ///                 "key1",
    ///                 || get_file_size(thread_id, "./Cargo.toml"),
    ///             );
    ///
    ///             // Ensure the value exists now.
    ///             assert!(value.is_ok());
    ///             assert!(my_cache.get(&"key1").is_some());
    ///
    ///             println!(
    ///                 "Thread {thread_id} got the value. (len: {})",
    ///                 value.unwrap()
    ///             );
    ///         })
    ///     })
    ///     .collect();
    ///
    /// // Wait all threads to complete.
    /// threads
    ///     .into_iter()
    ///     .for_each(|t| t.join().expect("Thread failed"));
    /// ```
    ///
    /// **Result**
    ///
    /// - `get_file_size()` was called exactly once by thread 1.
    /// - Other threads were blocked until thread 1 inserted the value.
    ///
    /// ```console
    /// Thread 1 started.
    /// Thread 2 started.
    /// get_file_size() called by thread 1.
    /// Thread 3 started.
    /// Thread 0 started.
    /// Thread 2 got the value. (len: 1466)
    /// Thread 0 got the value. (len: 1466)
    /// Thread 1 got the value. (len: 1466)
    /// Thread 3 got the value. (len: 1466)
    /// ```
    ///
    /// # Panics
    ///
    /// This method panics when the `init` closure has panicked. When it happens,
    /// only the caller whose `init` closure panicked will get the panic (e.g. only
    /// thread 1 in the above sample). If there are other calls in progress (e.g.
    /// thread 0, 2 and 3 above), this method will restart and resolve one of the
    /// remaining `init` closure.
    ///
    pub fn try_get_with<F, E>(&self, key: K, init: F) -> Result<V, Arc<E>>
    where
        F: FnOnce() -> Result<V, E>,
        E: Send + Sync + 'static,
    {
        let hash = self.base.hash(&key);
        let key = Arc::new(key);
        self.get_or_try_insert_with_hash_and_fun(key, hash, init, false)
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
        let hash = self.base.hash(key);
        self.get_or_try_insert_with_hash_by_ref_and_fun(key, hash, init, false)
            .map(Entry::into_value)
    }

    pub(crate) fn get_or_try_insert_with_hash_and_fun<F, E>(
        &self,
        key: Arc<K>,
        hash: u64,
        init: F,
        need_key: bool,
    ) -> Result<Entry<K, V>, Arc<E>>
    where
        F: FnOnce() -> Result<V, E>,
        E: Send + Sync + 'static,
    {
        if let Some(entry) = self.get_with_hash(&*key, hash, need_key) {
            return Ok(entry);
        }

        self.try_insert_with_hash_and_fun(key, hash, init, need_key)
    }

    pub(crate) fn get_or_try_insert_with_hash_by_ref_and_fun<F, Q, E>(
        &self,
        key: &Q,
        hash: u64,
        init: F,
        need_key: bool,
    ) -> Result<Entry<K, V>, Arc<E>>
    where
        F: FnOnce() -> Result<V, E>,
        E: Send + Sync + 'static,
        Q: Equivalent<K> + ToOwned<Owned = K> + Hash + ?Sized,
    {
        if let Some(entry) = self.get_with_hash(key, hash, false) {
            return Ok(entry);
        }

        let key = Arc::new(key.to_owned());
        self.try_insert_with_hash_and_fun(key, hash, init, need_key)
    }

    pub(crate) fn try_insert_with_hash_and_fun<F, E>(
        &self,
        key: Arc<K>,
        hash: u64,
        init: F,
        need_key: bool,
    ) -> Result<Entry<K, V>, Arc<E>>
    where
        F: FnOnce() -> Result<V, E>,
        E: Send + Sync + 'static,
    {
        let get = || {
            let ignore_if = None as Option<&mut fn(&V) -> bool>;
            self.base
                .get_with_hash_without_recording(&*key, hash, ignore_if)
        };
        let insert = |v| self.insert_with_hash(key.clone(), hash, v);

        let k = if need_key {
            Some(Arc::clone(&key))
        } else {
            None
        };

        let type_id = ValueInitializer::<K, V, S>::type_id_for_try_get_with::<E>();
        let post_init = ValueInitializer::<K, V, S>::post_init_for_try_get_with;

        match self
            .value_initializer
            .try_init_or_read(&key, type_id, get, init, insert, post_init)
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

    /// Inserts a key-value pair into the cache.
    ///
    /// If the cache has this key present, the value is updated.
    pub fn insert(&self, key: K, value: V) {
        let hash = self.base.hash(&key);
        let key = Arc::new(key);
        self.insert_with_hash(key, hash, value);
    }

    pub(crate) fn insert_with_hash(&self, key: Arc<K>, hash: u64, value: V) {
        if self.base.is_map_disabled() {
            return;
        }

        let (op, now) = self.base.do_insert_with_hash(key, hash, value);
        let hk = self.base.housekeeper.as_ref();
        Self::schedule_write_op(
            self.base.inner.as_ref(),
            &self.base.write_op_ch,
            op,
            now,
            hk,
        )
        .expect("Failed to insert");
    }

    pub(crate) fn compute_with_hash_and_fun<F>(
        &self,
        key: Arc<K>,
        hash: u64,
        f: F,
    ) -> compute::CompResult<K, V>
    where
        F: FnOnce(Option<Entry<K, V>>) -> compute::Op<V>,
    {
        let post_init = ValueInitializer::<K, V, S>::post_init_for_compute_with;
        match self
            .value_initializer
            .try_compute(key, hash, self, f, post_init, true)
        {
            Ok(result) => result,
            Err(_) => unreachable!(),
        }
    }

    pub(crate) fn try_compute_with_hash_and_fun<F, E>(
        &self,
        key: Arc<K>,
        hash: u64,
        f: F,
    ) -> Result<compute::CompResult<K, V>, E>
    where
        F: FnOnce(Option<Entry<K, V>>) -> Result<compute::Op<V>, E>,
        E: Send + Sync + 'static,
    {
        let post_init = ValueInitializer::<K, V, S>::post_init_for_try_compute_with;
        self.value_initializer
            .try_compute(key, hash, self, f, post_init, true)
    }

    pub(crate) fn upsert_with_hash_and_fun<F>(&self, key: Arc<K>, hash: u64, f: F) -> Entry<K, V>
    where
        F: FnOnce(Option<Entry<K, V>>) -> V,
    {
        let post_init = ValueInitializer::<K, V, S>::post_init_for_upsert_with;
        match self
            .value_initializer
            .try_compute(key, hash, self, f, post_init, false)
        {
            Ok(CompResult::Inserted(entry) | CompResult::ReplacedWith(entry)) => entry,
            _ => unreachable!(),
        }
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
        let hash = self.base.hash(key);
        self.invalidate_with_hash(key, hash, false);
    }

    /// Discards any cached value for the key and returns a _clone_ of the value.
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
        let hash = self.base.hash(key);
        self.invalidate_with_hash(key, hash, true)
    }

    pub(crate) fn invalidate_with_hash<Q>(&self, key: &Q, hash: u64, need_value: bool) -> Option<V>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
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
                klg = kl.as_ref().map(|kl| kl.lock());
            }
        }

        match self.base.remove_entry(key, hash) {
            None => None,
            Some(kv) => {
                let now = self.base.current_time();

                let info = kv.entry.entry_info();
                let entry_gen = info.incr_entry_gen();

                if self.base.is_removal_notifier_enabled() {
                    self.base.notify_invalidate(&kv.key, &kv.entry);
                }
                // Drop the locks before scheduling write op to avoid a potential
                // dead lock. (Scheduling write can do spin lock when the queue is
                // full, and queue will be drained by the housekeeping thread that
                // can lock the same key)
                std::mem::drop(klg);
                std::mem::drop(kl);

                let maybe_v = if need_value {
                    Some(kv.entry.value.clone())
                } else {
                    None
                };

                let op = WriteOp::Remove {
                    kv_entry: kv,
                    entry_gen,
                };
                let hk = self.base.housekeeper.as_ref();
                Self::schedule_write_op(
                    self.base.inner.as_ref(),
                    &self.base.write_op_ch,
                    op,
                    now,
                    hk,
                )
                .expect("Failed to remove");
                crossbeam_epoch::pin().flush();
                maybe_v
            }
        }
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

    pub(crate) fn invalidate_entries_with_arc_fun<F>(
        &self,
        predicate: Arc<F>,
    ) -> Result<PredicateId, PredicateError>
    where
        F: Fn(&K, &V) -> bool + Send + Sync + 'static,
    {
        self.base.invalidate_entries_if(predicate)
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
    /// use moka::sync::Cache;
    ///
    /// let cache = Cache::new(100);
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
        Iter::with_single_cache_segment(&self.base, self.num_cht_segments())
    }

    /// Performs any pending maintenance operations needed by the cache.
    pub fn run_pending_tasks(&self) {
        if let Some(hk) = &self.base.housekeeper {
            hk.run_pending_tasks(&*self.base.inner);
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
// Iterator support
//
impl<K, V, S> ScanningGet<K, V> for Cache<K, V, S>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
    S: BuildHasher + Clone + Send + Sync + 'static,
{
    fn num_cht_segments(&self) -> usize {
        self.base.num_cht_segments()
    }

    fn scanning_get(&self, key: &Arc<K>) -> Option<V> {
        self.base.scanning_get(key)
    }

    fn keys(&self, cht_segment: usize) -> Option<Vec<Arc<K>>> {
        self.base.keys(cht_segment)
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
    // TODO: Like future::Cache, move this method to BaseCache.
    #[inline]
    fn schedule_write_op(
        inner: &impl InnerSync,
        ch: &Sender<WriteOp<K, V>>,
        op: WriteOp<K, V>,
        now: Instant,
        housekeeper: Option<&HouseKeeperArc>,
    ) -> Result<(), TrySendError<WriteOp<K, V>>> {
        let mut op = op;

        // NOTES:
        // - This will block when the channel is full.
        // - We are doing a busy-loop here. We were originally calling `ch.send(op)?`,
        //   but we got a notable performance degradation.
        loop {
            BaseCache::<K, V, S>::apply_reads_writes_if_needed(inner, ch, now, housekeeper);
            match ch.try_send(op) {
                Ok(()) => break,
                Err(TrySendError::Full(op1)) => {
                    op = op1;
                    std::thread::sleep(Duration::from_micros(WRITE_RETRY_INTERVAL_MICROS));
                }
                Err(e @ TrySendError::Disconnected(_)) => return Err(e),
            }
        }
        Ok(())
    }
}

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
    pub(crate) fn invalidation_predicate_count(&self) -> usize {
        self.base.invalidation_predicate_count()
    }

    pub(crate) fn reconfigure_for_testing(&mut self) {
        self.base.reconfigure_for_testing();
    }

    pub(crate) fn key_locks_map_is_empty(&self) -> bool {
        self.base.key_locks_map_is_empty()
    }
}

// To see the debug prints, run test as `cargo test -- --nocapture`
#[cfg(test)]
mod tests {
    use super::Cache;
    use crate::{
        common::{time::Clock, HousekeeperConfig},
        notification::RemovalCause,
        policy::{test_utils::ExpiryCallCounters, EvictionPolicy},
        Expiry,
    };

    use parking_lot::Mutex;
    use std::{
        convert::Infallible,
        sync::{
            atomic::{AtomicU8, Ordering},
            Arc,
        },
        time::{Duration, Instant as StdInstant},
    };

    #[test]
    fn max_capacity_zero() {
        let mut cache = Cache::new(0);
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
        let mut cache = Cache::builder()
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
    fn basic_lru_single_thread() {
        // The following `Vec`s will hold actual and expected notifications.
        let actual = Arc::new(Mutex::new(Vec::new()));
        let mut expected = Vec::new();

        // Create an eviction listener.
        let a1 = Arc::clone(&actual);
        let listener = move |k, v, cause| a1.lock().push((k, v, cause));

        // Create a cache with the eviction listener.
        let mut cache = Cache::builder()
            .max_capacity(3)
            .eviction_policy(EvictionPolicy::lru())
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
        // a -> b

        cache.insert("c", "cindy");
        assert_eq!(cache.get(&"c"), Some("cindy"));
        assert!(cache.contains_key(&"c"));
        cache.run_pending_tasks();
        // a -> b -> c

        assert!(cache.contains_key(&"a"));
        assert_eq!(cache.get(&"a"), Some("alice"));
        assert_eq!(cache.get(&"b"), Some("bob"));
        assert!(cache.contains_key(&"b"));
        cache.run_pending_tasks();
        // c -> a -> b

        // "d" should be admitted because the cache uses the LRU strategy.
        cache.insert("d", "david");
        // "c" is the LRU and should have be evicted.
        expected.push((Arc::new("c"), "cindy", RemovalCause::Size));
        cache.run_pending_tasks();

        assert_eq!(cache.get(&"a"), Some("alice"));
        assert_eq!(cache.get(&"b"), Some("bob"));
        assert_eq!(cache.get(&"c"), None);
        assert_eq!(cache.get(&"d"), Some("david"));
        assert!(cache.contains_key(&"a"));
        assert!(cache.contains_key(&"b"));
        assert!(!cache.contains_key(&"c"));
        assert!(cache.contains_key(&"d"));
        cache.run_pending_tasks();
        // a -> b -> d

        cache.invalidate(&"b");
        expected.push((Arc::new("b"), "bob", RemovalCause::Explicit));
        cache.run_pending_tasks();
        // a -> d
        assert_eq!(cache.get(&"b"), None);
        assert!(!cache.contains_key(&"b"));

        assert!(cache.remove(&"b").is_none());
        assert_eq!(cache.remove(&"d"), Some("david"));
        expected.push((Arc::new("d"), "david", RemovalCause::Explicit));
        cache.run_pending_tasks();
        // a
        assert_eq!(cache.get(&"d"), None);
        assert!(!cache.contains_key(&"d"));

        cache.insert("e", "emily");
        cache.insert("f", "frank");
        // "a" should be evicted because it is the LRU.
        cache.insert("g", "gina");
        expected.push((Arc::new("a"), "alice", RemovalCause::Size));
        cache.run_pending_tasks();
        // e -> f -> g
        assert_eq!(cache.get(&"a"), None);
        assert_eq!(cache.get(&"e"), Some("emily"));
        assert_eq!(cache.get(&"f"), Some("frank"));
        assert_eq!(cache.get(&"g"), Some("gina"));
        assert!(!cache.contains_key(&"a"));
        assert!(cache.contains_key(&"e"));
        assert!(cache.contains_key(&"f"));
        assert!(cache.contains_key(&"g"));

        verify_notification_vec(&cache, actual, &expected);
        assert!(cache.key_locks_map_is_empty());
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
        let mut cache = Cache::builder()
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
        let cache = Cache::new(100);

        // https://rust-lang.github.io/rust-clippy/master/index.html#needless_collect
        #[allow(clippy::needless_collect)]
        let handles = (0..num_threads)
            .map(|id| {
                let cache = cache.clone();
                std::thread::spawn(move || {
                    cache.insert(10, format!("{id}-100"));
                    cache.get(&10);
                    cache.insert(20, format!("{id}-200"));
                    cache.invalidate(&10);
                })
            })
            .collect::<Vec<_>>();

        handles.into_iter().for_each(|h| h.join().expect("Failed"));

        assert!(cache.get(&10).is_none());
        assert!(cache.get(&20).is_some());
        assert!(!cache.contains_key(&10));
        assert!(cache.contains_key(&20));
    }

    #[test]
    fn invalidate_all() {
        // The following `Vec`s will hold actual and expected notifications.
        let actual = Arc::new(Mutex::new(Vec::new()));
        let mut expected = Vec::new();

        // Create an eviction listener.
        let a1 = Arc::clone(&actual);
        let listener = move |k, v, cause| a1.lock().push((k, v, cause));

        // Create a cache with the eviction listener.
        let mut cache = Cache::builder()
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
        expected.push((Arc::new("a"), "alice", RemovalCause::Explicit));
        expected.push((Arc::new("b"), "bob", RemovalCause::Explicit));
        expected.push((Arc::new("c"), "cindy", RemovalCause::Explicit));
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

        verify_notification_vec(&cache, actual, &expected);
    }

    #[test]
    fn invalidate_entries_if() -> Result<(), Box<dyn std::error::Error>> {
        use std::collections::HashSet;

        // The following `Vec`s will hold actual and expected notifications.
        let actual = Arc::new(Mutex::new(Vec::new()));
        let mut expected = Vec::new();

        // Create an eviction listener.
        let a1 = Arc::clone(&actual);
        let listener = move |k, v, cause| a1.lock().push((k, v, cause));

        let (clock, mock) = Clock::mock();

        // Create a cache with the eviction listener.
        let mut cache = Cache::builder()
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
        assert_eq!(cache.base.invalidation_predicate_count(), 1);
        expected.push((Arc::new(0), "alice", RemovalCause::Explicit));
        expected.push((Arc::new(2), "alex", RemovalCause::Explicit));

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
        assert_eq!(cache.invalidation_predicate_count(), 2);
        // key 1 was inserted before key 3.
        expected.push((Arc::new(1), "bob", RemovalCause::Explicit));
        expected.push((Arc::new(3), "alice", RemovalCause::Explicit));

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

        verify_notification_vec(&cache, actual, &expected);

        Ok(())
    }

    #[test]
    fn time_to_live() {
        // The following `Vec`s will hold actual and expected notifications.
        let actual = Arc::new(Mutex::new(Vec::new()));
        let mut expected = Vec::new();

        // Create an eviction listener.
        let a1 = Arc::clone(&actual);
        let listener = move |k, v, cause| a1.lock().push((k, v, cause));

        let (clock, mock) = Clock::mock();

        // Create a cache with the eviction listener.
        let mut cache = Cache::builder()
            .max_capacity(100)
            .time_to_live(Duration::from_secs(10))
            .eviction_listener(listener)
            .clock(clock)
            .build();
        cache.reconfigure_for_testing();

        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert("a", "alice");
        cache.run_pending_tasks();

        mock.increment(Duration::from_secs(5)); // 5 secs from the start.
        cache.run_pending_tasks();

        assert_eq!(cache.get(&"a"), Some("alice"));
        assert!(cache.contains_key(&"a"));

        mock.increment(Duration::from_secs(5)); // 10 secs.
        expected.push((Arc::new("a"), "alice", RemovalCause::Expired));
        assert_eq!(cache.get(&"a"), None);
        assert!(!cache.contains_key(&"a"));

        assert_eq!(cache.iter().count(), 0);

        cache.run_pending_tasks();
        assert!(cache.is_table_empty());

        cache.insert("b", "bob");
        cache.run_pending_tasks();

        assert_eq!(cache.entry_count(), 1);

        mock.increment(Duration::from_secs(5)); // 15 secs.
        cache.run_pending_tasks();

        assert_eq!(cache.get(&"b"), Some("bob"));
        assert!(cache.contains_key(&"b"));
        assert_eq!(cache.entry_count(), 1);

        cache.insert("b", "bill");
        expected.push((Arc::new("b"), "bob", RemovalCause::Replaced));
        cache.run_pending_tasks();

        mock.increment(Duration::from_secs(5)); // 20 secs
        cache.run_pending_tasks();

        assert_eq!(cache.get(&"b"), Some("bill"));
        assert!(cache.contains_key(&"b"));
        assert_eq!(cache.entry_count(), 1);

        mock.increment(Duration::from_secs(5)); // 25 secs
        expected.push((Arc::new("b"), "bill", RemovalCause::Expired));

        assert_eq!(cache.get(&"a"), None);
        assert_eq!(cache.get(&"b"), None);
        assert!(!cache.contains_key(&"a"));
        assert!(!cache.contains_key(&"b"));

        assert_eq!(cache.iter().count(), 0);

        cache.run_pending_tasks();
        assert!(cache.is_table_empty());

        verify_notification_vec(&cache, actual, &expected);
    }

    #[test]
    fn time_to_idle() {
        // The following `Vec`s will hold actual and expected notifications.
        let actual = Arc::new(Mutex::new(Vec::new()));
        let mut expected = Vec::new();

        // Create an eviction listener.
        let a1 = Arc::clone(&actual);
        let listener = move |k, v, cause| a1.lock().push((k, v, cause));

        let (clock, mock) = Clock::mock();

        // Create a cache with the eviction listener.
        let mut cache = Cache::builder()
            .max_capacity(100)
            .time_to_idle(Duration::from_secs(10))
            .eviction_listener(listener)
            .clock(clock)
            .build();
        cache.reconfigure_for_testing();

        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert("a", "alice");
        cache.run_pending_tasks();

        mock.increment(Duration::from_secs(5)); // 5 secs from the start.
        cache.run_pending_tasks();

        assert_eq!(cache.get(&"a"), Some("alice"));

        mock.increment(Duration::from_secs(5)); // 10 secs.
        cache.run_pending_tasks();

        cache.insert("b", "bob");
        cache.run_pending_tasks();

        assert_eq!(cache.entry_count(), 2);

        mock.increment(Duration::from_secs(2)); // 12 secs.
        cache.run_pending_tasks();

        // contains_key does not reset the idle timer for the key.
        assert!(cache.contains_key(&"a"));
        assert!(cache.contains_key(&"b"));
        cache.run_pending_tasks();

        assert_eq!(cache.entry_count(), 2);

        mock.increment(Duration::from_secs(3)); // 15 secs.
        expected.push((Arc::new("a"), "alice", RemovalCause::Expired));

        assert_eq!(cache.get(&"a"), None);
        assert_eq!(cache.get(&"b"), Some("bob"));
        assert!(!cache.contains_key(&"a"));
        assert!(cache.contains_key(&"b"));

        assert_eq!(cache.iter().count(), 1);

        cache.run_pending_tasks();
        assert_eq!(cache.entry_count(), 1);

        mock.increment(Duration::from_secs(10)); // 25 secs
        expected.push((Arc::new("b"), "bob", RemovalCause::Expired));

        assert_eq!(cache.get(&"a"), None);
        assert_eq!(cache.get(&"b"), None);
        assert!(!cache.contains_key(&"a"));
        assert!(!cache.contains_key(&"b"));

        assert_eq!(cache.iter().count(), 0);

        cache.run_pending_tasks();
        assert!(cache.is_table_empty());

        verify_notification_vec(&cache, actual, &expected);
    }

    // https://github.com/moka-rs/moka/issues/359
    #[test]
    fn ensure_access_time_is_updated_immediately_after_read() {
        let (clock, mock) = Clock::mock();
        let mut cache = Cache::builder()
            .max_capacity(10)
            .time_to_idle(Duration::from_secs(5))
            .clock(clock)
            .build();
        cache.reconfigure_for_testing();

        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert(1, 1);

        mock.increment(Duration::from_secs(4));
        assert_eq!(cache.get(&1), Some(1));

        mock.increment(Duration::from_secs(2));
        assert_eq!(cache.get(&1), Some(1));
        cache.run_pending_tasks();
        assert_eq!(cache.get(&1), Some(1));
    }

    #[test]
    fn time_to_live_by_expiry_type() {
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

        // The following `Vec`s will hold actual and expected notifications.
        let actual = Arc::new(Mutex::new(Vec::new()));
        let mut expected = Vec::new();

        // Create expiry counters and the expiry.
        let expiry_counters = Arc::new(ExpiryCallCounters::default());
        let expiry = MyExpiry::new(Arc::clone(&expiry_counters));

        // Create an eviction listener.
        let a1 = Arc::clone(&actual);
        let listener = move |k, v, cause| a1.lock().push((k, v, cause));

        let (clock, mock) = Clock::mock();

        // Create a cache with the eviction listener.
        let mut cache = Cache::builder()
            .max_capacity(100)
            .expire_after(expiry)
            .eviction_listener(listener)
            .clock(clock)
            .build();
        cache.reconfigure_for_testing();

        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert("a", "alice");
        expiry_counters.incl_expected_creations();
        cache.run_pending_tasks();

        mock.increment(Duration::from_secs(5)); // 5 secs from the start.
        cache.run_pending_tasks();

        assert_eq!(cache.get(&"a"), Some("alice"));
        assert!(cache.contains_key(&"a"));

        mock.increment(Duration::from_secs(5)); // 10 secs.
        expected.push((Arc::new("a"), "alice", RemovalCause::Expired));
        assert_eq!(cache.get(&"a"), None);
        assert!(!cache.contains_key(&"a"));

        assert_eq!(cache.iter().count(), 0);

        cache.run_pending_tasks();
        assert!(cache.is_table_empty());

        cache.insert("b", "bob");
        expiry_counters.incl_expected_creations();
        cache.run_pending_tasks();

        assert_eq!(cache.entry_count(), 1);

        mock.increment(Duration::from_secs(5)); // 15 secs.
        cache.run_pending_tasks();

        assert_eq!(cache.get(&"b"), Some("bob"));
        assert!(cache.contains_key(&"b"));
        assert_eq!(cache.entry_count(), 1);

        cache.insert("b", "bill");
        expected.push((Arc::new("b"), "bob", RemovalCause::Replaced));
        expiry_counters.incl_expected_updates();
        cache.run_pending_tasks();

        mock.increment(Duration::from_secs(5)); // 20 secs
        cache.run_pending_tasks();

        assert_eq!(cache.get(&"b"), Some("bill"));
        assert!(cache.contains_key(&"b"));
        assert_eq!(cache.entry_count(), 1);

        mock.increment(Duration::from_secs(5)); // 25 secs
        expected.push((Arc::new("b"), "bill", RemovalCause::Expired));

        assert_eq!(cache.get(&"a"), None);
        assert_eq!(cache.get(&"b"), None);
        assert!(!cache.contains_key(&"a"));
        assert!(!cache.contains_key(&"b"));

        assert_eq!(cache.iter().count(), 0);

        cache.run_pending_tasks();
        assert!(cache.is_table_empty());

        expiry_counters.verify();
        verify_notification_vec(&cache, actual, &expected);
    }

    #[test]
    fn time_to_idle_by_expiry_type() {
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

        // The following `Vec`s will hold actual and expected notifications.
        let actual = Arc::new(Mutex::new(Vec::new()));
        let mut expected = Vec::new();

        // Create expiry counters and the expiry.
        let expiry_counters = Arc::new(ExpiryCallCounters::default());
        let expiry = MyExpiry::new(Arc::clone(&expiry_counters));

        // Create an eviction listener.
        let a1 = Arc::clone(&actual);
        let listener = move |k, v, cause| a1.lock().push((k, v, cause));

        let (clock, mock) = Clock::mock();

        // Create a cache with the eviction listener.
        let mut cache = Cache::builder()
            .max_capacity(100)
            .expire_after(expiry)
            .eviction_listener(listener)
            .clock(clock)
            .build();
        cache.reconfigure_for_testing();

        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert("a", "alice");
        cache.run_pending_tasks();

        mock.increment(Duration::from_secs(5)); // 5 secs from the start.
        cache.run_pending_tasks();

        assert_eq!(cache.get(&"a"), Some("alice"));
        expiry_counters.incl_expected_reads();

        mock.increment(Duration::from_secs(5)); // 10 secs.
        cache.run_pending_tasks();

        cache.insert("b", "bob");
        cache.run_pending_tasks();

        assert_eq!(cache.entry_count(), 2);

        mock.increment(Duration::from_secs(2)); // 12 secs.
        cache.run_pending_tasks();

        // contains_key does not reset the idle timer for the key.
        assert!(cache.contains_key(&"a"));
        assert!(cache.contains_key(&"b"));
        cache.run_pending_tasks();

        assert_eq!(cache.entry_count(), 2);

        mock.increment(Duration::from_secs(3)); // 15 secs.
        expected.push((Arc::new("a"), "alice", RemovalCause::Expired));

        assert_eq!(cache.get(&"a"), None);
        assert_eq!(cache.get(&"b"), Some("bob"));
        expiry_counters.incl_expected_reads();
        assert!(!cache.contains_key(&"a"));
        assert!(cache.contains_key(&"b"));

        assert_eq!(cache.iter().count(), 1);

        cache.run_pending_tasks();
        assert_eq!(cache.entry_count(), 1);

        mock.increment(Duration::from_secs(10)); // 25 secs
        expected.push((Arc::new("b"), "bob", RemovalCause::Expired));

        assert_eq!(cache.get(&"a"), None);
        assert_eq!(cache.get(&"b"), None);
        assert!(!cache.contains_key(&"a"));
        assert!(!cache.contains_key(&"b"));

        assert_eq!(cache.iter().count(), 0);

        cache.run_pending_tasks();
        assert!(cache.is_table_empty());

        expiry_counters.verify();
        verify_notification_vec(&cache, actual, &expected);
    }

    /// Verify that the `Expiry::expire_after_read()` method is called in `get_with`
    /// only when the key was already present in the cache.
    #[test]
    fn test_expiry_using_get_with() {
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
        cache.reconfigure_for_testing();

        // Make the cache exterior immutable.
        let cache = cache;

        // The key is not present.
        cache.get_with("a", || "alice");
        expiry_counters.incl_expected_creations();
        cache.run_pending_tasks();

        // The key is present.
        cache.get_with("a", || "alex");
        expiry_counters.incl_expected_reads();
        cache.run_pending_tasks();

        // The key is not present.
        cache.invalidate("a");
        cache.get_with("a", || "amanda");
        expiry_counters.incl_expected_creations();
        cache.run_pending_tasks();

        expiry_counters.verify();
    }

    // https://github.com/moka-rs/moka/issues/345
    #[test]
    fn test_race_between_updating_entry_and_processing_its_write_ops() {
        let (clock, mock) = Clock::mock();
        let cache = Cache::builder()
            .max_capacity(2)
            .time_to_idle(Duration::from_secs(1))
            .clock(clock)
            .build();

        cache.insert("a", "alice");
        cache.insert("b", "bob");
        cache.insert("c", "cathy"); // c1
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
        cache.insert("c", "cindy"); // c2

        // Remove "c" (c2) from the cht.
        assert_eq!(cache.remove(&"c"), Some("cindy")); // c-remove

        mock.increment(Duration::from_secs(2));

        // The following `run_pending_tasks` will do the followings:
        // 1. Admits "c" (c2) to the cache. (Create a node in the LRU deque)
        // 2. Because of c-remove, removes c2's node from the LRU deque.
        cache.run_pending_tasks();
        assert_eq!(cache.entry_count(), 0);
    }

    #[test]
    fn test_race_between_recreating_entry_and_processing_its_write_ops() {
        let cache = Cache::builder().max_capacity(2).build();

        cache.insert('a', "a");
        cache.insert('b', "b");
        cache.run_pending_tasks();

        cache.insert('c', "c1"); // (a) `EntryInfo` 1, gen: 1
        assert!(cache.remove(&'a').is_some()); // (b)
        assert!(cache.remove(&'b').is_some()); // (c)
        assert!(cache.remove(&'c').is_some()); // (d) `EntryInfo` 1, gen: 2
        cache.insert('c', "c2"); // (e) `EntryInfo` 2, gen: 1

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
        cache.run_pending_tasks();
        assert_eq!(cache.get(&'c'), Some("c2"));
    }

    #[test]
    fn test_iter() {
        const NUM_KEYS: usize = 50;

        fn make_value(key: usize) -> String {
            format!("val: {key}")
        }

        let cache = Cache::builder()
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

        let cache = Cache::builder()
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

        let cache = Cache::new(100);
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
    fn get_with_by_ref() {
        use std::thread::{sleep, spawn};

        let cache = Cache::new(100);
        const KEY: &u32 = &0;

        // This test will run five threads:
        //
        // Thread1 will be the first thread to call `get_with_by_ref` for a key, so
        // its init closure will be evaluated and then a &str value "thread1" will be
        // inserted to the cache.
        let thread1 = {
            let cache1 = cache.clone();
            spawn(move || {
                // Call `get_with_by_ref` immediately.
                let v = cache1.get_with_by_ref(KEY, || {
                    // Wait for 300 ms and return a &str value.
                    sleep(Duration::from_millis(300));
                    "thread1"
                });
                assert_eq!(v, "thread1");
            })
        };

        // Thread2 will be the second thread to call `get_with_by_ref` for the same
        // key, so its init closure will not be evaluated. Once thread1's init
        // closure finishes, it will get the value inserted by thread1's init
        // closure.
        let thread2 = {
            let cache2 = cache.clone();
            spawn(move || {
                // Wait for 100 ms before calling `get_with_by_ref`.
                sleep(Duration::from_millis(100));
                let v = cache2.get_with_by_ref(KEY, || unreachable!());
                assert_eq!(v, "thread1");
            })
        };

        // Thread3 will be the third thread to call `get_with_by_ref` for the same
        // key. By the time it calls, thread1's init closure should have finished
        // already and the value should be already inserted to the cache. So its init
        // closure will not be evaluated and will get the value insert by thread1's
        // init closure immediately.
        let thread3 = {
            let cache3 = cache.clone();
            spawn(move || {
                // Wait for 400 ms before calling `get_with_by_ref`.
                sleep(Duration::from_millis(400));
                let v = cache3.get_with_by_ref(KEY, || unreachable!());
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
                let maybe_v = cache4.get(KEY);
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
                let maybe_v = cache5.get(KEY);
                assert_eq!(maybe_v, Some("thread1"));
            })
        };

        for t in [thread1, thread2, thread3, thread4, thread5] {
            t.join().expect("Failed to join");
        }

        assert!(cache.is_waiter_map_empty());
    }

    #[test]
    fn entry_or_insert_with_if() {
        use std::thread::{sleep, spawn};

        let cache = Cache::new(100);
        const KEY: u32 = 0;

        // This test will run seven threads:
        //
        // Thread1 will be the first thread to call `or_insert_with_if` for a key, so
        // its init closure will be evaluated and then a &str value "thread1" will be
        // inserted to the cache.
        let thread1 = {
            let cache1 = cache.clone();
            spawn(move || {
                // Call `get_with` immediately.
                let entry = cache1.entry(KEY).or_insert_with_if(
                    || {
                        // Wait for 300 ms and return a &str value.
                        sleep(Duration::from_millis(300));
                        "thread1"
                    },
                    |_v| unreachable!(),
                );
                // Entry should be fresh because our async block should have been
                // evaluated.
                assert!(entry.is_fresh());
                assert_eq!(entry.into_value(), "thread1");
            })
        };

        // Thread2 will be the second thread to call `or_insert_with_if` for the same
        // key, so its init closure will not be evaluated. Once thread1's init
        // closure finishes, it will get the value inserted by thread1's init
        // closure.
        let thread2 = {
            let cache2 = cache.clone();
            spawn(move || {
                // Wait for 100 ms before calling `get_with`.
                sleep(Duration::from_millis(100));
                let entry = cache2
                    .entry(KEY)
                    .or_insert_with_if(|| unreachable!(), |_v| unreachable!());
                // Entry should not be fresh because thread1's async block should have
                // been evaluated instead of ours.
                assert!(!entry.is_fresh());
                assert_eq!(entry.into_value(), "thread1");
            })
        };

        // Thread3 will be the third thread to call `or_insert_with_if` for the same
        // key. By the time it calls, thread1's init closure should have finished
        // already and the value should be already inserted to the cache. Also
        // thread3's `replace_if` closure returns `false`. So its init closure will
        // not be evaluated and will get the value inserted by thread1's init closure
        // immediately.
        let thread3 = {
            let cache3 = cache.clone();
            spawn(move || {
                // Wait for 350 ms before calling `or_insert_with_if`.
                sleep(Duration::from_millis(350));
                let entry = cache3.entry(KEY).or_insert_with_if(
                    || unreachable!(),
                    |v| {
                        assert_eq!(v, &"thread1");
                        false
                    },
                );
                assert!(!entry.is_fresh());
                assert_eq!(entry.into_value(), "thread1");
            })
        };

        // Thread4 will be the fourth thread to call `or_insert_with_if` for the same
        // key. The value should have been already inserted to the cache by thread1.
        // However thread4's `replace_if` closure returns `true`. So its init closure
        // will be evaluated to replace the current value.
        let thread4 = {
            let cache4 = cache.clone();
            spawn(move || {
                // Wait for 400 ms before calling `or_insert_with_if`.
                sleep(Duration::from_millis(400));
                let entry = cache4.entry(KEY).or_insert_with_if(
                    || "thread4",
                    |v| {
                        assert_eq!(v, &"thread1");
                        true
                    },
                );
                assert!(entry.is_fresh());
                assert_eq!(entry.into_value(), "thread4");
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
                // Wait for 350 ms before calling `get`.
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
                // Wait for 450 ms before calling `get`.
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
    fn entry_by_ref_or_insert_with_if() {
        use std::thread::{sleep, spawn};

        let cache: Cache<u32, &str> = Cache::new(100);
        const KEY: &u32 = &0;

        // This test will run seven threads:
        //
        // Thread1 will be the first thread to call `or_insert_with_if` for a key, so
        // its init closure will be evaluated and then a &str value "thread1" will be
        // inserted to the cache.
        let thread1 = {
            let cache1 = cache.clone();
            spawn(move || {
                // Call `get_with` immediately.
                let v = cache1
                    .entry_by_ref(KEY)
                    .or_insert_with_if(
                        || {
                            // Wait for 300 ms and return a &str value.
                            sleep(Duration::from_millis(300));
                            "thread1"
                        },
                        |_v| unreachable!(),
                    )
                    .into_value();
                assert_eq!(v, "thread1");
            })
        };

        // Thread2 will be the second thread to call `or_insert_with_if` for the same
        // key, so its init closure will not be evaluated. Once thread1's init
        // closure finishes, it will get the value inserted by thread1's init
        // closure.
        let thread2 = {
            let cache2 = cache.clone();
            spawn(move || {
                // Wait for 100 ms before calling `get_with`.
                sleep(Duration::from_millis(100));
                let v = cache2
                    .entry_by_ref(KEY)
                    .or_insert_with_if(|| unreachable!(), |_v| unreachable!())
                    .into_value();
                assert_eq!(v, "thread1");
            })
        };

        // Thread3 will be the third thread to call `or_insert_with_if` for the same
        // key. By the time it calls, thread1's init closure should have finished
        // already and the value should be already inserted to the cache. Also
        // thread3's `replace_if` closure returns `false`. So its init closure will
        // not be evaluated and will get the value inserted by thread1's init closure
        // immediately.
        let thread3 = {
            let cache3 = cache.clone();
            spawn(move || {
                // Wait for 350 ms before calling `or_insert_with_if`.
                sleep(Duration::from_millis(350));
                let v = cache3
                    .entry_by_ref(KEY)
                    .or_insert_with_if(
                        || unreachable!(),
                        |v| {
                            assert_eq!(v, &"thread1");
                            false
                        },
                    )
                    .into_value();
                assert_eq!(v, "thread1");
            })
        };

        // Thread4 will be the fourth thread to call `or_insert_with_if` for the same
        // key. The value should have been already inserted to the cache by
        // thread1. However thread4's `replace_if` closure returns `true`. So its
        // init closure will be evaluated to replace the current value.
        let thread4 = {
            let cache4 = cache.clone();
            spawn(move || {
                // Wait for 400 ms before calling `or_insert_with_if`.
                sleep(Duration::from_millis(400));
                let v = cache4
                    .entry_by_ref(KEY)
                    .or_insert_with_if(
                        || "thread4",
                        |v| {
                            assert_eq!(v, &"thread1");
                            true
                        },
                    )
                    .into_value();
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
                let maybe_v = cache5.get(KEY);
                assert!(maybe_v.is_none());
            })
        };

        // Thread6 will call `get` for the same key. It will call when thread1's init
        // closure is still running, so it will get none for the key.
        let thread6 = {
            let cache6 = cache.clone();
            spawn(move || {
                // Wait for 350 ms before calling `get`.
                sleep(Duration::from_millis(350));
                let maybe_v = cache6.get(KEY);
                assert_eq!(maybe_v, Some("thread1"));
            })
        };

        // Thread7 will call `get` for the same key. It will call after thread1's init
        // closure finished, so it will get the value insert by thread1's init closure.
        let thread7 = {
            let cache7 = cache.clone();
            spawn(move || {
                // Wait for 450 ms before calling `get`.
                sleep(Duration::from_millis(450));
                let maybe_v = cache7.get(KEY);
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

        // Note that MyError does not implement std::error::Error trait like
        // anyhow::Error.
        #[derive(Debug)]
        pub struct MyError(#[allow(dead_code)] String);

        type MyResult<T> = Result<T, Arc<MyError>>;

        let cache = Cache::new(100);
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
    fn try_get_with_by_ref() {
        use std::{
            sync::Arc,
            thread::{sleep, spawn},
        };

        // Note that MyError does not implement std::error::Error trait like
        // anyhow::Error.
        #[derive(Debug)]
        pub struct MyError(#[allow(dead_code)] String);

        type MyResult<T> = Result<T, Arc<MyError>>;

        let cache = Cache::new(100);
        const KEY: &u32 = &0;

        // This test will run eight threads:
        //
        // Thread1 will be the first thread to call `try_get_with_by_ref` for a key,
        // so its init closure will be evaluated and then an error will be returned.
        // Nothing will be inserted to the cache.
        let thread1 = {
            let cache1 = cache.clone();
            spawn(move || {
                // Call `try_get_with_by_ref` immediately.
                let v = cache1.try_get_with_by_ref(KEY, || {
                    // Wait for 300 ms and return an error.
                    sleep(Duration::from_millis(300));
                    Err(MyError("thread1 error".into()))
                });
                assert!(v.is_err());
            })
        };

        // Thread2 will be the second thread to call `try_get_with_by_ref` for the
        // same key, so its init closure will not be evaluated. Once thread1's init
        // closure finishes, it will get the same error value returned by thread1's
        // init closure.
        let thread2 = {
            let cache2 = cache.clone();
            spawn(move || {
                // Wait for 100 ms before calling `try_get_with_by_ref`.
                sleep(Duration::from_millis(100));
                let v: MyResult<_> = cache2.try_get_with_by_ref(KEY, || unreachable!());
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
                // Wait for 400 ms before calling `try_get_with_by_ref`.
                sleep(Duration::from_millis(400));
                let v: MyResult<_> = cache3.try_get_with_by_ref(KEY, || {
                    // Wait for 300 ms and return an Ok(&str) value.
                    sleep(Duration::from_millis(300));
                    Ok("thread3")
                });
                assert_eq!(v.unwrap(), "thread3");
            })
        };

        // thread4 will be the fourth thread to call `try_get_with_by_ref` for the
        // same key. So its init closure will not be evaluated. Once thread3's init
        // closure finishes, it will get the same okay &str value.
        let thread4 = {
            let cache4 = cache.clone();
            spawn(move || {
                // Wait for 500 ms before calling `try_get_with_by_ref`.
                sleep(Duration::from_millis(500));
                let v: MyResult<_> = cache4.try_get_with_by_ref(KEY, || unreachable!());
                assert_eq!(v.unwrap(), "thread3");
            })
        };

        // Thread5 will be the fifth thread to call `try_get_with_by_ref` for the
        // same key. So its init closure will not be evaluated. By the time it calls,
        // thread3's init closure should have finished already, so its init closure
        // will not be evaluated and will get the value insert by thread3's init
        // closure immediately.
        let thread5 = {
            let cache5 = cache.clone();
            spawn(move || {
                // Wait for 800 ms before calling `try_get_with_by_ref`.
                sleep(Duration::from_millis(800));
                let v: MyResult<_> = cache5.try_get_with_by_ref(KEY, || unreachable!());
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
                let maybe_v = cache6.get(KEY);
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
                let maybe_v = cache7.get(KEY);
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
                let maybe_v = cache8.get(KEY);
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

        let cache = Cache::new(100);
        const KEY: u32 = 0;

        // This test will run eight threads:
        //
        // Thread1 will be the first thread to call `optionally_get_with` for a key,
        // so its init closure will be evaluated and then an error will be returned.
        // Nothing will be inserted to the cache.
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

        // Thread2 will be the second thread to call `optionally_get_with` for the
        // same key, so its init closure will not be evaluated. Once thread1's init
        // closure finishes, it will get the same error value returned by thread1's
        // init closure.
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

        // thread4 will be the fourth thread to call `optionally_get_with` for the
        // same key. So its init closure will not be evaluated. Once thread3's init
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

        // Thread5 will be the fifth thread to call `optionally_get_with` for the
        // same key. So its init closure will not be evaluated. By the time it calls,
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

    #[test]
    fn optionally_get_with_by_ref() {
        use std::thread::{sleep, spawn};

        let cache = Cache::new(100);
        const KEY: &u32 = &0;

        // This test will run eight threads:
        //
        // Thread1 will be the first thread to call `optionally_get_with_by_ref` for
        // a key, so its init closure will be evaluated and then an error will be
        // returned. Nothing will be inserted to the cache.
        let thread1 = {
            let cache1 = cache.clone();
            spawn(move || {
                // Call `optionally_get_with_by_ref` immediately.
                let v = cache1.optionally_get_with_by_ref(KEY, || {
                    // Wait for 300 ms and return an error.
                    sleep(Duration::from_millis(300));
                    None
                });
                assert!(v.is_none());
            })
        };

        // Thread2 will be the second thread to call `optionally_get_with_by_ref` for
        // the same key, so its init closure will not be evaluated. Once thread1's
        // init closure finishes, it will get the same error value returned by
        // thread1's init closure.
        let thread2 = {
            let cache2 = cache.clone();
            spawn(move || {
                // Wait for 100 ms before calling `optionally_get_with_by_ref`.
                sleep(Duration::from_millis(100));
                let v = cache2.optionally_get_with_by_ref(KEY, || unreachable!());
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
                // Wait for 400 ms before calling `optionally_get_with_by_ref`.
                sleep(Duration::from_millis(400));
                let v = cache3.optionally_get_with_by_ref(KEY, || {
                    // Wait for 300 ms and return an Ok(&str) value.
                    sleep(Duration::from_millis(300));
                    Some("thread3")
                });
                assert_eq!(v.unwrap(), "thread3");
            })
        };

        // thread4 will be the fourth thread to call `optionally_get_with_by_ref` for
        // the same key. So its init closure will not be evaluated. Once thread3's
        // init closure finishes, it will get the same okay &str value.
        let thread4 = {
            let cache4 = cache.clone();
            spawn(move || {
                // Wait for 500 ms before calling `optionally_get_with_by_ref`.
                sleep(Duration::from_millis(500));
                let v = cache4.optionally_get_with_by_ref(KEY, || unreachable!());
                assert_eq!(v.unwrap(), "thread3");
            })
        };

        // Thread5 will be the fifth thread to call `optionally_get_with_by_ref` for
        // the same key. So its init closure will not be evaluated. By the time it
        // calls, thread3's init closure should have finished already, so its init
        // closure will not be evaluated and will get the value insert by thread3's
        // init closure immediately.
        let thread5 = {
            let cache5 = cache.clone();
            spawn(move || {
                // Wait for 800 ms before calling `optionally_get_with_by_ref`.
                sleep(Duration::from_millis(800));
                let v = cache5.optionally_get_with_by_ref(KEY, || unreachable!());
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
                let maybe_v = cache6.get(KEY);
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
                let maybe_v = cache7.get(KEY);
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
                let maybe_v = cache8.get(KEY);
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
    fn upsert_with() {
        use std::thread::{sleep, spawn};

        let cache = Cache::new(100);
        const KEY: u32 = 0;

        // Spawn three threads to call `and_upsert_with` for the same key and each
        // task increments the current value by 1. Ensure the key-level lock is
        // working by verifying the value is 3 after all threads finish.
        //
        // |        | thread 1 | thread 2 | thread 3 |
        // |--------|----------|----------|----------|
        // |   0 ms | get none |          |          |
        // | 100 ms |          | blocked  |          |
        // | 200 ms | insert 1 |          |          |
        // |        |          | get 1    |          |
        // | 300 ms |          |          | blocked  |
        // | 400 ms |          | insert 2 |          |
        // |        |          |          | get 2    |
        // | 500 ms |          |          | insert 3 |

        let thread1 = {
            let cache1 = cache.clone();
            spawn(move || {
                cache1.entry(KEY).and_upsert_with(|maybe_entry| {
                    sleep(Duration::from_millis(200));
                    assert!(maybe_entry.is_none());
                    1
                })
            })
        };

        let thread2 = {
            let cache2 = cache.clone();
            spawn(move || {
                sleep(Duration::from_millis(100));
                cache2.entry_by_ref(&KEY).and_upsert_with(|maybe_entry| {
                    sleep(Duration::from_millis(200));
                    let entry = maybe_entry.expect("The entry should exist");
                    entry.into_value() + 1
                })
            })
        };

        let thread3 = {
            let cache3 = cache.clone();
            spawn(move || {
                sleep(Duration::from_millis(300));
                cache3.entry_by_ref(&KEY).and_upsert_with(|maybe_entry| {
                    sleep(Duration::from_millis(100));
                    let entry = maybe_entry.expect("The entry should exist");
                    entry.into_value() + 1
                })
            })
        };

        let ent1 = thread1.join().expect("Thread 1 should finish");
        let ent2 = thread2.join().expect("Thread 2 should finish");
        let ent3 = thread3.join().expect("Thread 3 should finish");
        assert_eq!(ent1.into_value(), 1);
        assert_eq!(ent2.into_value(), 2);
        assert_eq!(ent3.into_value(), 3);

        assert_eq!(cache.get(&KEY), Some(3));

        assert!(cache.is_waiter_map_empty());
    }

    #[test]
    fn compute_with() {
        use crate::ops::compute;
        use std::{
            sync::RwLock,
            thread::{sleep, spawn},
        };

        let cache = Cache::new(100);
        const KEY: u32 = 0;

        // Spawn six threads to call `and_compute_with` for the same key. Ensure the
        // key-level lock is working by verifying the value after all threads finish.
        //
        // |         |  thread 1  |   thread 2    |  thread 3  | thread 4 |  thread 5  | thread 6 |
        // |---------|------------|---------------|------------|----------|------------|----------|
        // |    0 ms | get none   |               |            |          |            |          |
        // |  100 ms |            | blocked       |            |          |            |          |
        // |  200 ms | insert [1] |               |            |          |            |          |
        // |         |            | get [1]       |            |          |            |          |
        // |  300 ms |            |               | blocked    |          |            |          |
        // |  400 ms |            | insert [1, 2] |            |          |            |          |
        // |         |            |               | get [1, 2] |          |            |          |
        // |  500 ms |            |               |            | blocked  |            |          |
        // |  600 ms |            |               | remove     |          |            |          |
        // |         |            |               |            | get none |            |          |
        // |  700 ms |            |               |            |          | blocked    |          |
        // |  800 ms |            |               |            | nop      |            |          |
        // |         |            |               |            |          | get none   |          |
        // |  900 ms |            |               |            |          |            | blocked  |
        // | 1000 ms |            |               |            |          | insert [5] |          |
        // |         |            |               |            |          |            | get [5]  |
        // | 1100 ms |            |               |            |          |            | nop      |

        let thread1 = {
            let cache1 = cache.clone();
            spawn(move || {
                cache1.entry(KEY).and_compute_with(|maybe_entry| {
                    sleep(Duration::from_millis(200));
                    assert!(maybe_entry.is_none());
                    compute::Op::Put(Arc::new(RwLock::new(vec![1])))
                })
            })
        };

        let thread2 = {
            let cache2 = cache.clone();
            spawn(move || {
                sleep(Duration::from_millis(100));
                cache2.entry_by_ref(&KEY).and_compute_with(|maybe_entry| {
                    let entry = maybe_entry.expect("The entry should exist");
                    let value = entry.into_value();
                    assert_eq!(*value.read().unwrap(), vec![1]);
                    sleep(Duration::from_millis(200));
                    value.write().unwrap().push(2);
                    compute::Op::Put(value)
                })
            })
        };

        let thread3 = {
            let cache3 = cache.clone();
            spawn(move || {
                sleep(Duration::from_millis(300));
                cache3.entry(KEY).and_compute_with(|maybe_entry| {
                    let entry = maybe_entry.expect("The entry should exist");
                    let value = entry.into_value();
                    assert_eq!(*value.read().unwrap(), vec![1, 2]);
                    sleep(Duration::from_millis(200));
                    compute::Op::Remove
                })
            })
        };

        let thread4 = {
            let cache4 = cache.clone();
            spawn(move || {
                sleep(Duration::from_millis(500));
                cache4.entry(KEY).and_compute_with(|maybe_entry| {
                    assert!(maybe_entry.is_none());
                    sleep(Duration::from_millis(200));
                    compute::Op::Nop
                })
            })
        };

        let thread5 = {
            let cache5 = cache.clone();
            spawn(move || {
                sleep(Duration::from_millis(700));
                cache5.entry_by_ref(&KEY).and_compute_with(|maybe_entry| {
                    assert!(maybe_entry.is_none());
                    sleep(Duration::from_millis(200));
                    compute::Op::Put(Arc::new(RwLock::new(vec![5])))
                })
            })
        };

        let thread6 = {
            let cache6 = cache.clone();
            spawn(move || {
                sleep(Duration::from_millis(900));
                cache6.entry_by_ref(&KEY).and_compute_with(|maybe_entry| {
                    let entry = maybe_entry.expect("The entry should exist");
                    let value = entry.into_value();
                    assert_eq!(*value.read().unwrap(), vec![5]);
                    sleep(Duration::from_millis(100));
                    compute::Op::Nop
                })
            })
        };

        let res1 = thread1.join().expect("Thread 1 should finish");
        let res2 = thread2.join().expect("Thread 2 should finish");
        let res3 = thread3.join().expect("Thread 3 should finish");
        let res4 = thread4.join().expect("Thread 4 should finish");
        let res5 = thread5.join().expect("Thread 5 should finish");
        let res6 = thread6.join().expect("Thread 6 should finish");

        let compute::CompResult::Inserted(entry) = res1 else {
            panic!("Expected `Inserted`. Got {res1:?}")
        };
        assert_eq!(
            *entry.into_value().read().unwrap(),
            vec![1, 2] // The same Vec was modified by task2.
        );

        let compute::CompResult::ReplacedWith(entry) = res2 else {
            panic!("Expected `ReplacedWith`. Got {res2:?}")
        };
        assert_eq!(*entry.into_value().read().unwrap(), vec![1, 2]);

        let compute::CompResult::Removed(entry) = res3 else {
            panic!("Expected `Removed`. Got {res3:?}")
        };
        assert_eq!(*entry.into_value().read().unwrap(), vec![1, 2]);

        let compute::CompResult::StillNone(key) = res4 else {
            panic!("Expected `StillNone`. Got {res4:?}")
        };
        assert_eq!(*key, KEY);

        let compute::CompResult::Inserted(entry) = res5 else {
            panic!("Expected `Inserted`. Got {res5:?}")
        };
        assert_eq!(*entry.into_value().read().unwrap(), vec![5]);

        let compute::CompResult::Unchanged(entry) = res6 else {
            panic!("Expected `Unchanged`. Got {res6:?}")
        };
        assert_eq!(*entry.into_value().read().unwrap(), vec![5]);

        assert!(cache.is_waiter_map_empty());
    }

    #[test]
    fn try_compute_with() {
        use crate::ops::compute;
        use std::{
            sync::RwLock,
            thread::{sleep, spawn},
        };

        let cache: Cache<u32, Arc<RwLock<Vec<i32>>>> = Cache::new(100);
        const KEY: u32 = 0;

        // Spawn four threads to call `and_try_compute_with` for the same key. Ensure
        // the key-level lock is working by verifying the value after all threads
        // finish.
        //
        // |         |  thread 1  |   thread 2    |  thread 3  | thread 4   |
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

        let thread1 = {
            let cache1 = cache.clone();
            spawn(move || {
                cache1.entry(KEY).and_try_compute_with(|maybe_entry| {
                    sleep(Duration::from_millis(200));
                    assert!(maybe_entry.is_none());
                    Ok(compute::Op::Put(Arc::new(RwLock::new(vec![1])))) as Result<_, ()>
                })
            })
        };

        let thread2 = {
            let cache2 = cache.clone();
            spawn(move || {
                sleep(Duration::from_millis(100));
                cache2
                    .entry_by_ref(&KEY)
                    .and_try_compute_with(|maybe_entry| {
                        let entry = maybe_entry.expect("The entry should exist");
                        let value = entry.into_value();
                        assert_eq!(*value.read().unwrap(), vec![1]);
                        sleep(Duration::from_millis(200));
                        value.write().unwrap().push(2);
                        Ok(compute::Op::Put(value)) as Result<_, ()>
                    })
            })
        };

        let thread3 = {
            let cache3 = cache.clone();
            spawn(move || {
                sleep(Duration::from_millis(300));
                cache3.entry(KEY).and_try_compute_with(|maybe_entry| {
                    let entry = maybe_entry.expect("The entry should exist");
                    let value = entry.into_value();
                    assert_eq!(*value.read().unwrap(), vec![1, 2]);
                    sleep(Duration::from_millis(200));
                    Err(())
                })
            })
        };

        let thread4 = {
            let cache4 = cache.clone();
            spawn(move || {
                sleep(Duration::from_millis(500));
                cache4.entry(KEY).and_try_compute_with(|maybe_entry| {
                    let entry = maybe_entry.expect("The entry should exist");
                    let value = entry.into_value();
                    assert_eq!(*value.read().unwrap(), vec![1, 2]);
                    sleep(Duration::from_millis(100));
                    Ok(compute::Op::Remove) as Result<_, ()>
                })
            })
        };

        let res1 = thread1.join().expect("Thread 1 should finish");
        let res2 = thread2.join().expect("Thread 2 should finish");
        let res3 = thread3.join().expect("Thread 3 should finish");
        let res4 = thread4.join().expect("Thread 4 should finish");

        let Ok(compute::CompResult::Inserted(entry)) = res1 else {
            panic!("Expected `Inserted`. Got {res1:?}")
        };
        assert_eq!(
            *entry.into_value().read().unwrap(),
            vec![1, 2] // The same Vec was modified by task2.
        );

        let Ok(compute::CompResult::ReplacedWith(entry)) = res2 else {
            panic!("Expected `ReplacedWith`. Got {res2:?}")
        };
        assert_eq!(*entry.into_value().read().unwrap(), vec![1, 2]);

        assert!(res3.is_err());

        let Ok(compute::CompResult::Removed(entry)) = res4 else {
            panic!("Expected `Removed`. Got {res4:?}")
        };
        assert_eq!(
            *entry.into_value().read().unwrap(),
            vec![1, 2] // Removed value.
        );

        assert!(cache.is_waiter_map_empty());
    }

    #[test]
    // https://github.com/moka-rs/moka/issues/43
    fn handle_panic_in_get_with() {
        use std::{sync::Barrier, thread};

        let cache = Cache::new(16);
        let barrier = Arc::new(Barrier::new(2));
        {
            let cache_ref = cache.clone();
            let barrier_ref = barrier.clone();
            thread::spawn(move || {
                let _ = cache_ref.get_with(1, || {
                    barrier_ref.wait();
                    thread::sleep(Duration::from_millis(50));
                    panic!("Panic during get_with");
                });
            });
        }

        barrier.wait();
        assert_eq!(cache.get_with(1, || 5), 5);

        assert!(cache.is_waiter_map_empty());
    }

    #[test]
    // https://github.com/moka-rs/moka/issues/43
    fn handle_panic_in_try_get_with() {
        use std::{sync::Barrier, thread};

        let cache = Cache::new(16);
        let barrier = Arc::new(Barrier::new(2));
        {
            let cache_ref = cache.clone();
            let barrier_ref = barrier.clone();
            thread::spawn(move || {
                let _ = cache_ref.try_get_with(1, || {
                    barrier_ref.wait();
                    thread::sleep(Duration::from_millis(50));
                    panic!("Panic during try_get_with");
                }) as Result<_, Arc<Infallible>>;
            });
        }

        barrier.wait();
        assert_eq!(
            cache.try_get_with(1, || Ok(5)) as Result<_, Arc<Infallible>>,
            Ok(5)
        );

        assert!(cache.is_waiter_map_empty());
    }

    #[test]
    fn test_removal_notifications() {
        // The following `Vec`s will hold actual and expected notifications.
        let actual = Arc::new(Mutex::new(Vec::new()));
        let mut expected = Vec::new();

        // Create an eviction listener.
        let a1 = Arc::clone(&actual);
        let listener = move |k, v, cause| a1.lock().push((k, v, cause));

        // Create a cache with the eviction listener.
        let mut cache = Cache::builder()
            .max_capacity(3)
            .eviction_listener(listener)
            .build();
        cache.reconfigure_for_testing();

        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert('a', "alice");
        cache.invalidate(&'a');
        expected.push((Arc::new('a'), "alice", RemovalCause::Explicit));

        cache.run_pending_tasks();
        assert_eq!(cache.entry_count(), 0);

        cache.insert('b', "bob");
        cache.insert('c', "cathy");
        cache.insert('d', "david");
        cache.run_pending_tasks();
        assert_eq!(cache.entry_count(), 3);

        // This will be rejected due to the size constraint.
        cache.insert('e', "emily");
        expected.push((Arc::new('e'), "emily", RemovalCause::Size));
        cache.run_pending_tasks();
        assert_eq!(cache.entry_count(), 3);

        // Raise the popularity of 'e' so it will be accepted next time.
        cache.get(&'e');
        cache.run_pending_tasks();

        // Retry.
        cache.insert('e', "eliza");
        // and the LRU entry will be evicted.
        expected.push((Arc::new('b'), "bob", RemovalCause::Size));
        cache.run_pending_tasks();
        assert_eq!(cache.entry_count(), 3);

        // Replace an existing entry.
        cache.insert('d', "dennis");
        expected.push((Arc::new('d'), "david", RemovalCause::Replaced));
        cache.run_pending_tasks();
        assert_eq!(cache.entry_count(), 3);

        verify_notification_vec(&cache, actual, &expected);
    }

    #[test]
    fn test_immediate_removal_notifications_with_updates() {
        // The following `Vec` will hold actual notifications.
        let actual = Arc::new(Mutex::new(Vec::new()));

        // Create an eviction listener.
        let a1 = Arc::clone(&actual);
        let listener = move |k, v, cause| a1.lock().push((k, v, cause));

        let (clock, mock) = Clock::mock();

        // Create a cache with the eviction listener and also TTL and TTI.
        let mut cache = Cache::builder()
            .eviction_listener(listener)
            .time_to_live(Duration::from_secs(7))
            .time_to_idle(Duration::from_secs(5))
            .clock(clock)
            .build();
        cache.reconfigure_for_testing();

        // Make the cache exterior immutable.
        let cache = cache;

        cache.insert("alice", "a0");
        cache.run_pending_tasks();

        // Now alice (a0) has been expired by the idle timeout (TTI).
        mock.increment(Duration::from_secs(6));
        assert_eq!(cache.get(&"alice"), None);

        // We have not ran sync after the expiration of alice (a0), so it is
        // still in the cache.
        assert_eq!(cache.entry_count(), 1);

        // Re-insert alice with a different value. Since alice (a0) is still
        // in the cache, this is actually a replace operation rather than an
        // insert operation. We want to verify that the RemovalCause of a0 is
        // Expired, not Replaced.
        cache.insert("alice", "a1");
        {
            let mut a = actual.lock();
            assert_eq!(a.len(), 1);
            assert_eq!(a[0], (Arc::new("alice"), "a0", RemovalCause::Expired));
            a.clear();
        }

        cache.run_pending_tasks();

        mock.increment(Duration::from_secs(4));
        assert_eq!(cache.get(&"alice"), Some("a1"));
        cache.run_pending_tasks();

        // Now alice has been expired by time-to-live (TTL).
        mock.increment(Duration::from_secs(4));
        assert_eq!(cache.get(&"alice"), None);

        // But, again, it is still in the cache.
        assert_eq!(cache.entry_count(), 1);

        // Re-insert alice with a different value and verify that the
        // RemovalCause of a1 is Expired (not Replaced).
        cache.insert("alice", "a2");
        {
            let mut a = actual.lock();
            assert_eq!(a.len(), 1);
            assert_eq!(a[0], (Arc::new("alice"), "a1", RemovalCause::Expired));
            a.clear();
        }

        cache.run_pending_tasks();

        assert_eq!(cache.entry_count(), 1);

        // Now alice (a2) has been expired by the idle timeout.
        mock.increment(Duration::from_secs(6));
        assert_eq!(cache.get(&"alice"), None);
        assert_eq!(cache.entry_count(), 1);

        // This invalidate will internally remove alice (a2).
        cache.invalidate(&"alice");
        cache.run_pending_tasks();
        assert_eq!(cache.entry_count(), 0);

        {
            let mut a = actual.lock();
            assert_eq!(a.len(), 1);
            assert_eq!(a[0], (Arc::new("alice"), "a2", RemovalCause::Expired));
            a.clear();
        }

        // Re-insert, and this time, make it expired by the TTL.
        cache.insert("alice", "a3");
        cache.run_pending_tasks();
        mock.increment(Duration::from_secs(4));
        assert_eq!(cache.get(&"alice"), Some("a3"));
        cache.run_pending_tasks();
        mock.increment(Duration::from_secs(4));
        assert_eq!(cache.get(&"alice"), None);
        assert_eq!(cache.entry_count(), 1);

        // This invalidate will internally remove alice (a2).
        cache.invalidate(&"alice");
        cache.run_pending_tasks();

        assert_eq!(cache.entry_count(), 0);

        {
            let mut a = actual.lock();
            assert_eq!(a.len(), 1);
            assert_eq!(a[0], (Arc::new("alice"), "a3", RemovalCause::Expired));
            a.clear();
        }

        assert!(cache.key_locks_map_is_empty());
    }

    // This test ensures the key-level lock for the immediate notification
    // delivery mode is working so that the notifications for a given key
    // should always be ordered. This is true even if multiple client threads
    // try to modify the entries for the key at the same time. (This test will
    // run three client threads)
    //
    // This test is ignored by default. It becomes unstable when run in parallel
    // with other tests.
    #[test]
    #[ignore]
    fn test_key_lock_used_by_immediate_removal_notifications() {
        use std::thread::{sleep, spawn};

        const KEY: &str = "alice";

        type Val = &'static str;

        #[derive(PartialEq, Eq, Debug)]
        enum Event {
            Insert(Val),
            Invalidate(Val),
            BeginNotify(Val, RemovalCause),
            EndNotify(Val, RemovalCause),
        }

        // The following `Vec will hold actual notifications.
        let actual = Arc::new(Mutex::new(Vec::new()));

        // Create an eviction listener.
        // Note that this listener is slow and will take 300 ms to complete.
        let a0 = Arc::clone(&actual);
        let listener = move |_k, v, cause| {
            a0.lock().push(Event::BeginNotify(v, cause));
            sleep(Duration::from_millis(300));
            a0.lock().push(Event::EndNotify(v, cause));
        };

        // Create a cache with the eviction listener and also TTL 500 ms.
        let mut cache = Cache::builder()
            .eviction_listener(listener)
            .time_to_live(Duration::from_millis(500))
            .build();
        cache.reconfigure_for_testing();

        // Make the cache exterior immutable.
        let cache = cache;

        // - Notifications for the same key must not overlap.

        // Time  Event
        // ----- -------------------------------------
        // 0000: Insert value a0
        // 0500: a0 expired
        // 0600: Insert value a1 -> expired a0 (N-A0)
        // 0800: Insert value a2 (waiting) (A-A2)
        // 0900: N-A0 processed
        //       A-A2 finished waiting -> replace a1 (N-A1)
        // 1100: Invalidate (waiting) (R-A2)
        // 1200: N-A1 processed
        //       R-A2 finished waiting -> explicit a2 (N-A2)
        // 1500: N-A2 processed

        let expected = vec![
            Event::Insert("a0"),
            Event::Insert("a1"),
            Event::BeginNotify("a0", RemovalCause::Expired),
            Event::Insert("a2"),
            Event::EndNotify("a0", RemovalCause::Expired),
            Event::BeginNotify("a1", RemovalCause::Replaced),
            Event::Invalidate("a2"),
            Event::EndNotify("a1", RemovalCause::Replaced),
            Event::BeginNotify("a2", RemovalCause::Explicit),
            Event::EndNotify("a2", RemovalCause::Explicit),
        ];

        // 0000: Insert value a0
        actual.lock().push(Event::Insert("a0"));
        cache.insert(KEY, "a0");
        // Call `sync` to set the last modified for the KEY immediately so that
        // this entry should expire in 1000 ms from now.
        cache.run_pending_tasks();

        // 0500: Insert value a1 -> expired a0 (N-A0)
        let thread1 = {
            let a1 = Arc::clone(&actual);
            let c1 = cache.clone();
            spawn(move || {
                sleep(Duration::from_millis(600));
                a1.lock().push(Event::Insert("a1"));
                c1.insert(KEY, "a1");
            })
        };

        // 0800: Insert value a2 (waiting) (A-A2)
        let thread2 = {
            let a2 = Arc::clone(&actual);
            let c2 = cache.clone();
            spawn(move || {
                sleep(Duration::from_millis(800));
                a2.lock().push(Event::Insert("a2"));
                c2.insert(KEY, "a2");
            })
        };

        // 1100: Invalidate (waiting) (R-A2)
        let thread3 = {
            let a3 = Arc::clone(&actual);
            let c3 = cache.clone();
            spawn(move || {
                sleep(Duration::from_millis(1100));
                a3.lock().push(Event::Invalidate("a2"));
                c3.invalidate(&KEY);
            })
        };

        for t in [thread1, thread2, thread3] {
            t.join().expect("Failed to join");
        }

        let actual = actual.lock();
        assert_eq!(actual.len(), expected.len());

        for (i, (actual, expected)) in actual.iter().zip(&expected).enumerate() {
            assert_eq!(actual, expected, "expected[{i}]");
        }

        assert!(cache.key_locks_map_is_empty());
    }

    // When the eviction listener is not set, calling `run_pending_tasks` once should
    // evict all entries that can be removed.
    #[test]
    fn no_batch_size_limit_on_eviction() {
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
        cache.reconfigure_for_testing();

        // Make the cache exterior immutable.
        let cache = cache;

        // Fill the cache.
        for i in 0..MAX_CAPACITY {
            let v = format!("v{i}");
            cache.insert(i, v)
        }
        // The max capacity should not change because we have not called
        // `run_pending_tasks` yet.
        assert_eq!(cache.entry_count(), 0);

        cache.run_pending_tasks();
        assert_eq!(cache.entry_count(), MAX_CAPACITY);

        // Insert more items to the cache.
        for i in MAX_CAPACITY..(MAX_CAPACITY * 2) {
            let v = format!("v{i}");
            cache.insert(i, v)
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
        cache.run_pending_tasks();
        assert_eq!(cache.entry_count(), MAX_CAPACITY);

        // Now all the old keys should be gone.
        assert!(!cache.contains_key(&0));
        assert!(!cache.contains_key(&(MAX_CAPACITY - 1)));
        // And the new keys should exist.
        assert!(cache.contains_key(&(MAX_CAPACITY * 2 - 1)));
    }

    #[test]
    fn slow_eviction_listener() {
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
        cache.reconfigure_for_testing();

        // Make the cache exterior immutable.
        let cache = cache;

        // Fill the cache.
        for i in 0..MAX_CAPACITY {
            let v = format!("v{i}");
            cache.insert(i, v)
        }
        // The max capacity should not change because we have not called
        // `run_pending_tasks` yet.
        assert_eq!(cache.entry_count(), 0);

        cache.run_pending_tasks();
        assert_eq!(listener_call_count.load(Ordering::Acquire), 0);
        assert_eq!(cache.entry_count(), MAX_CAPACITY);

        // Insert more items to the cache.
        for i in MAX_CAPACITY..(MAX_CAPACITY * 2) {
            let v = format!("v{i}");
            cache.insert(i, v);
        }
        assert_eq!(cache.entry_count(), MAX_CAPACITY);

        cache.run_pending_tasks();
        // Because of the slow listener, cache should get an over capacity.
        let mut expected_call_count = 3;
        assert_eq!(
            listener_call_count.load(Ordering::Acquire) as u64,
            expected_call_count
        );
        assert_eq!(cache.entry_count(), MAX_CAPACITY * 2 - expected_call_count);

        loop {
            cache.run_pending_tasks();

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
    // RUST_LOG=moka=info cargo test --features 'logging' -- \
    //   sync::cache::tests::recover_from_panicking_eviction_listener --exact --nocapture
    //
    #[test]
    fn recover_from_panicking_eviction_listener() {
        #[cfg(feature = "logging")]
        let _ = env_logger::builder().is_test(true).try_init();

        // The following `Vec`s will hold actual and expected notifications.
        let actual = Arc::new(Mutex::new(Vec::new()));
        let mut expected = Vec::new();

        // Create an eviction listener that panics when it see
        // a value "panic now!".
        let a1 = Arc::clone(&actual);
        let listener = move |k, v, cause| {
            if v == "panic now!" {
                panic!("Panic now!");
            }
            a1.lock().push((k, v, cause))
        };

        // Create a cache with the eviction listener.
        let mut cache = Cache::builder()
            .name("My Sync Cache")
            .eviction_listener(listener)
            .build();
        cache.reconfigure_for_testing();

        // Make the cache exterior immutable.
        let cache = cache;

        // Insert an okay value.
        cache.insert("alice", "a0");
        cache.run_pending_tasks();

        // Insert a value that will cause the eviction listener to panic.
        cache.insert("alice", "panic now!");
        expected.push((Arc::new("alice"), "a0", RemovalCause::Replaced));
        cache.run_pending_tasks();

        // Insert an okay value. This will replace the previous
        // value "panic now!" so the eviction listener will panic.
        cache.insert("alice", "a2");
        cache.run_pending_tasks();
        // No more removal notification should be sent.

        // Invalidate the okay value.
        cache.invalidate(&"alice");
        cache.run_pending_tasks();

        verify_notification_vec(&cache, actual, &expected);
    }

    // This test ensures that the `contains_key`, `get` and `invalidate` can use
    // borrowed form `&[u8]` for key with type `Vec<u8>`.
    // https://github.com/moka-rs/moka/issues/166
    #[test]
    fn borrowed_forms_of_key() {
        let cache: Cache<Vec<u8>, ()> = Cache::new(1);

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

    // For testing the issue reported by: https://github.com/moka-rs/moka/issues/383
    //
    // Ignored by default. This test becomes unstable when run in parallel with
    // other tests.
    #[test]
    #[ignore]
    fn ensure_gc_runs_when_dropping_cache() {
        let cache = Cache::builder().build();
        let val = Arc::new(0);
        {
            let val = Arc::clone(&val);
            cache.get_with(1, move || val);
        }
        drop(cache);
        assert_eq!(Arc::strong_count(&val), 1);
    }

    #[test]
    fn test_debug_format() {
        let cache = Cache::new(10);
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

    type NotificationTuple<K, V> = (Arc<K>, V, RemovalCause);

    fn verify_notification_vec<K, V, S>(
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
            cache.run_pending_tasks();
            std::thread::sleep(Duration::from_millis(500));

            let actual = &*actual.lock();
            if actual.len() != expected.len() {
                if retries <= MAX_RETRIES {
                    retries += 1;
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
