# Moka Examples

This directory contains examples of how to use Moka cache. Each example is a
standalone binary that can be run with the following command:

```console
$ cargo run --example <example_name> -F sync,future
```

Each example has a suffix `_async` or `_sync`:

- `_async` indicates that the example uses the `moka::future::Cache`, which is a
  `Future`-aware, concurrent cache.
- `_sync` indicates that the example uses the `moka::sync::Cache`, which is a
  multi-thread safe, concurrent cache.

## Basics of the Cache API

- [basics_async](./basics_async.rs) and [basics_sync](./basics_sync.rs)
    - Shares a cache between async tasks or OS threads.
        - Do not wrap a `Cache` with `Arc<Mutex<_>>`! Just clone the `Cache` and you
          are all set.
    - Uses `insert`, `get` and `invalidate` methods.

- [size_aware_eviction_sync](./size_aware_eviction_sync.rs)
    - Configures the max capacity of the cache based on the total size of the cached
      entries.

## The `Entry` API

Atomically inserts, updates and removes an entry from the cache depending on the
existence of the entry.

- [counter_async](./counter_async.rs) and [counter_sync](./counter_sync.rs)
    - Atomically increments a cached `u64` by 1. If the entry does not exist, inserts
      a new entry with the value 1.
    - Uses `and_upsert_with` method.
- [bounded_counter_async](./bounded_counter_async.rs) and
  [bounded_counter_sync](./bounded_counter_sync.rs)
    - Same as above except removing the entry when the value is 2.
    - `and_compute_with` method.
- [append_value_async](./append_value_async.rs) and
  [append_value_sync](./append_value_sync.rs)
    - Atomically appends an `i32` to a cached `Arc<RwLock<Vec<i32>>>`. If the entry
      does not exist, inserts a new entry.
    - Uses `and_upsert_with` method.
- [try_append_value_async](./try_append_value_async.rs) and
  [try_append_value_sync](./try_append_value_sync.rs)
    - Atomically reads an `char` from a reader and appends it to a cached `Arc<RwLock<String>>`,
      but reading may fail by an early EOF.
    - Uses `and_try_compute_with` method.

## Expiration and Eviction Listener

- [eviction_listener_sync](./eviction_listener_sync.rs)
    - Configures the `time_to_live` expiration policy.
    - Registers a listener (closure) to be notified when an entry is evicted from the
      cache.
    - Uses `insert`, `invalidate`, `invalidate_all` and `run_pending_tasks` methods.
    - Demonstrates when the expired entries will be actually evicted from the cache,
      and why the `run_pending_tasks` method could be important in some cases.

- [jittered_expiry_policy_sync](./jittered_expiry_policy_sync.rs)
    - Implements a jittered expiry policy for a cache.
    - The `JitteredExpiry` struct is a custom expiry policy that adds jitter to the
      base expiry duration.
        - It implements the `moka::Expiry` trait and calculates the expiry duration
          after a write or read operation.
        - The jitter is randomly generated and added to or subtracted from the base
          expiry duration.
    - This example uses the `moka::sync::Cache` type, but The same expiry policy can
      be used with the `moka::future::Cache`.

- [cascading_drop_async](./cascading_drop_async.rs)
    - Controls the lifetime of the objects in a separate `BTreeMap` collection from
      the cache using an eviction listener.
    - Beside the cache APIs, uses `BTreeMap`, `Arc` and mpsc channel (multi-producer,
      single consumer channel).

- [reinsert_expired_entries_sync](./reinsert_expired_enties_sync.rs)
    - Reinserts the expired entries into the cache using eviction listener and
      worker threads.
    - Spawns two worker threads; one for reinserting entries, and the other for
      calling `run_pending_tasks`.
    - Uses a mpsc channel (multi-producer, single consumer channel) to send commands
      from the eviction listener to the first worker thread.

## Check out the API Documentation too!

The examples are not meant to be exhaustive. Please check the
[API documentation][api-doc] for more examples and details.

[api-doc]: https://docs.rs/moka
