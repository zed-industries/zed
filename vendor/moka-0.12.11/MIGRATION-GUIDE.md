# Moka Cache &mdash; Migration Guide

## Migrating to v0.12 from a prior version

v0.12.0 had major breaking changes on the API and internal behavior. This section
describes the code changes required to migrate to v0.12.0.

### Highlights v0.12

- **`sync` caches are no longer enabled by default**: Please use a crate feature
  `sync` to enable it.

- **No more background threads**: All cache types `future::Cache`, `sync::Cache`, and
  `sync::SegmentedCache` no longer spawn background threads.
  - The `scheduled-thread-pool` crate was removed from the dependency.
  - Because of this change, many private methods and some public methods under the
    `future` module were converted to `async` methods. You will need to add `.await`
    to your code for those methods.

- **Immediate notification delivery**: The `notification::DeliveryMode` enum for the
  eviction listener was removed. Now all cache types behave as if the `Immediate`
  delivery mode is specified.

  - `DeliveryMode` enum had two variants `Immediate` and `Queued`.
      - The former should be easier to use than other as it guarantees to preserve
        the order of events on a given cache key.
      - The latter did not use internal locks and would provide higher performance
        under heavy cache writes.
  - Now all cache types work as if the `Immediate` mode is specified.
      - **`future::Cache`**: In earlier versions of `future::Cache`, the queued mode
        was used. Now it behaves as if the immediate mode is specified.
      - **`sync` caches**: From earlier versions of `sync::Cache` and
        `sync::SegmentedCache`, the immediate mode is the default mode. So this
        change should only affects those of you who are explicitly using the queued
        mode.
  - The queued mode was implemented by using a background thread. The queued mode was
    removed because there is no thread pool available anymore.
  - If you need the queued mode back, please file a GitHub issue. We could provide
    a way to use a user supplied thread pool.

The following sections will describe about the changes you might need to make to your
code.

- [`sync::Cache` and `sync::SegmentedCache`](#synccache-and-syncsegmentedcache-v012)
- [`future::Cache`](#futurecache-v012)
- [The maintenance tasks](#the-maintenance-tasks)

### `sync::Cache` and `sync::SegmentedCache` v0.12

1. Please use a crate feature `sync` to enable `sync` caches.
2. Since the background threads were removed, the maintenance tasks such as removing
   expired entries are not executed periodically anymore.
   - The `thread_pool_enabled` method of the `sync::CacheBuilder` was removed. The
     thread pool is always disabled.
   - See the [maintenance tasks](#the-maintenance-tasks) section for more details.
3. The `sync` method of the `sync::ConcurrentCacheExt` trait was moved to
   `sync::Cache` and `sync::SegmentedCache` types. It is also renamed to
   `run_pending_tasks`.
4. Now `sync` caches always work as if the immediate delivery mode is specified
   for the eviction listener.
   - In older versions, the immediate mode was the default mode, and the queued
     mode could be optionally selected.

### `future::Cache` v0.12

#### API changes

1. The `get` method is now `async fn`, so you must `await` for the result.
2. The `blocking` method was removed.
   - Please use async runtime's blocking API instead.
   - See the [replacing the blocking API](#replacing-the-blocking-api) section for
     more details.
3. Now the `or_insert_with_if` method of the entry API requires `Send` bound for the
   `replace_if` closure.
4. The `eviction_listener_with_queued_delivery_mode` method of `future::CacheBuilder`
   was removed.
   - Please use one of the new methods instead:
     - `eviction_listener`
     - `async_eviction_listener`
   - See the [updating the eviction listener](#updating-the-eviction-listener)
     section for more details.
5. The `sync` method of the `future::ConcurrentCacheExt` trait was moved to
   `future::Cache` type and renamed to `run_pending_tasks`. It was also changed to
   `async fn`.

#### Behavior changes

1. Since the background threads were removed, the maintenance tasks such as removing
   expired entries are not executed periodically anymore.
   - See the [maintenance tasks](#the-maintenance-tasks) section for more details.
2. Now `future::Cache` always behaves as if the immediate delivery mode is specified
   for the eviction listener.
   - In older versions, the queued delivery mode was used.

#### Replacing the blocking API

The `blocking` method of `future::Cache` was removed. Please use async runtime's
blocking API instead.

**Tokio**

1. Call the `tokio::runtime::Handle::current()` method in async context to obtain a
   handle to the current Tokio runtime.
2. From outside async context, call cache's async function using `block_on` method of
   the runtime.

```rust
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Create a future cache.
    let cache = Arc::new(moka::future::Cache::new(100));

    // In async context, you can obtain a handle to the current Tokio runtime.
    let rt = tokio::runtime::Handle::current();

    // Spawn an OS thread. Pass the handle and cache.
    let thread = {
        let cache = Arc::clone(&cache);

        std::thread::spawn(move || {
            // Call async function using block_on method of Tokio runtime.
            rt.block_on(cache.insert(0, 'a'));
        })
    };

    // Wait for the threads to complete.
    thread.join().unwrap();

    // Check the result.
    assert_eq!(cache.get(&0).await, Some('a'));
}
```

**async-std**

- From outside async context, call cache's async function using the
  `async_std::task::block_on` method.

```rust
use std::sync::Arc;

#[async_std::main]
async fn main() {
    // Create a future cache.
    let cache = Arc::new(moka::future::Cache::new(100));

    // Spawn an OS thread. Pass the cache.
    let thread = {
        let cache = Arc::clone(&cache);

        std::thread::spawn(move || {
            use async_std::task::block_on;

            // Call async function using block_on method of async_std.
            block_on(cache.insert(0, 'a'));
        })
    };

    // Wait for the threads to complete.
    thread.join().unwrap();

    // Check the result.
    assert_eq!(cache.get(&0).await, Some('a'));
}
```

#### Updating the eviction listener

The `eviction_listener_with_queued_delivery_mode` method of `future::CacheBuilder`
was removed. Please use one of the new methods `eviction_listener` or
`async_eviction_listener` instead.

##### `eviction_listener` method

The `eviction_listener` method takes the same closure as the old method. If you do
not need to `.await` anything in the eviction listener, use this method.

This code snippet is borrowed from [an example][listener-ex1] in the document of
`future::Cache`:

```rust
let eviction_listener = |key, _value, cause| {
    println!("Evicted key {key}. Cause: {cause:?}");
};

let cache = Cache::builder()
    .max_capacity(100)
    .expire_after(expiry)
    .eviction_listener(eviction_listener)
    .build();
```

[listener-ex1]: https://docs.rs/moka/latest/moka/future/struct.Cache.html#per-entry-expiration-policy

##### `async_eviction_listener` method

The `async_eviction_listener` takes a closure that returns a `Future`. If you need to
`await` something in the eviction listener, use this method. The actual return type
of the closure is `notification::ListenerFuture`, which is a type alias of
`Pin<Box<dyn Future<Output = ()> + Send>>`. You can use the `boxed` method of
`future::FutureExt` trait to convert a regular `Future` into this type.

This code snippet is borrowed from [an example][listener-ex2] in the document of
`future::Cache`:

```rust
use moka::notification::ListenerFuture;
// FutureExt trait provides the boxed method.
use moka::future::FutureExt;

let eviction_listener = move |k, v: PathBuf, cause| -> ListenerFuture {
    println!("\n== An entry has been evicted. k: {k:?}, v: {v:?}, cause: {cause:?}");
    let file_mgr2 = Arc::clone(&file_mgr1);

    // Create a Future that removes the data file at the path `v`.
    async move {
        // Acquire the write lock of the DataFileManager.
        let mut mgr = file_mgr2.write().await;
        // Remove the data file. We must handle error cases here to
        // prevent the listener from panicking.
        if let Err(_e) = mgr.remove_data_file(v.as_path()).await {
            eprintln!("Failed to remove a data file at {v:?}");
        }
    }
    // Convert the regular Future into ListenerFuture. This method is
    // provided by moka::future::FutureExt trait.
    .boxed()
};

// Create the cache. Set time to live for two seconds and set the
// eviction listener.
let cache = Cache::builder()
    .max_capacity(100)
    .time_to_live(Duration::from_secs(2))
    .async_eviction_listener(eviction_listener)
    .build();
```

[listener-ex2]: https://docs.rs/moka/latest/moka/future/struct.Cache.html#example-eviction-listener

### The maintenance tasks

In older versions, the maintenance tasks needed by the cache were periodically
executed in background by a global thread pool managed by `moka`. Now all cache types
do not use the thread pool anymore, so those maintenance tasks are executed
_sometimes_ in foreground when certain cache methods (`get`, `get_with`, `insert`,
etc.) are called by user code.

![The lifecycle of cached entries](https://github.com/moka-rs/moka/wiki/images/benchmarks/moka-tiny-lfu.png)

Figure 1. The lifecycle of cached entries

These maintenance tasks include:

1. Determine whether to admit a "temporary admitted" entry or not.
2. Apply the recording of cache reads and writes to the internal data structures for
   the cache policies, such as the LFU filter, LRU queues, and hierarchical timer
   wheels.
3. When cache's max capacity is exceeded, remove least recently used (LRU) entries.
4. Remove expired entries.
5. Find and remove the entries that have been invalidated by the `invalidate_all` or
   `invalidate_entries_if` methods.
6. Deliver removal notifications to the eviction listener. (Call the eviction
   listener closure with the information about the evicted entry)

They will be executed in the following cache methods when one of the following
conditions is met:

Cache Methods:

- All cache write methods: `insert`, `get_with`, `invalidate`, etc., except for
  `invalidate_all` and `invalidate_entries_if`.
- Some of the cache read methods: `get`
- `run_pending_tasks` method, which executes the pending maintenance tasks
  explicitly.

Conditions:

- When one of the numbers of pending read and write recordings exceeds the threshold.
    - The threshold is currently hard-coded to 64 items.
- When the time since the last execution of the maintenance tasks exceeds the
  threshold.
    - The threshold is currently hard-coded to 300 milliseconds.

#### `run_pending_tasks` method

You can execute the pending maintenance tasks explicitly by calling the
`run_pending_tasks` method. This method is available for all cache types.

Note that cache read methods such as the `get`, `get_with` and `contains_key` never
return expired entries although they are not removed immediately from the cache when
they expire. You will not need to call `run_pending_tasks` method to remove expired
entries unless you want to remove them immediately (e.g. to free some resources).
