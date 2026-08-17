# Changelog

## [Unreleased]
## Added
## Changed
## Removed

## [0.56.0 / [cached_proc_macro[0.25.0]]]
## Added
## Changed
- *BREAKING* All timed/expiring caches now use std::time::Duration values instead of raw seconds/millis.
- Update `redis` to 0.32
- Update `hashbrown` to 0.15
## Removed

## [0.55.1 / [cached_proc_macro[0.24.0]]]
## Added
- Add `sync_writes = "by_key"` support to `#[cached]`
## Changed
- Update `redis` to 0.29.0
- Update `directories` to 6.0
- Update `thiserror` to 2.0
- With the `sync_writes = "by_key"` addition, the argument values changed from a boolean
  to strings. The equivalent of `sync_writes = true` is now `sync_writes = "default"`
## Removed

## [0.54.0]
## Added
- Add `Cached::cache_try_get_or_set_with` for parity with async trait
## Changed
- Remove unnecessary string clones in redis cache store
- Update cargo default features manifest key
## Removed

## [0.53.1 / [cached_proc_macro[0.23.0]]]
## Added
## Changed
- Replace `instant` with `web_time` in proc macro, update cached_proc_macro version
## Removed

## [0.53.0]
## Added
## Changed
- Replace unmaintained `instant` crate with `web_time`
## Removed

## [0.52.0 / [cached_proc_macro[0.22.0]] ]
## Added
## Changed
- Propagate function generics to generated inner cache function 
## Removed


## [0.51.4]
## Added
## Changed
- Update `DiskCache` to require `ToString` instead of `Display`
## Removed

## [0.51.3]
## Added
- `ExpiringSizedCache`: Allow specifying explicit TTL when inserting
## Changed
- Refactor `ExpiringSizedCache` internals to not require tombstones
- `ExpiringSizedCache` keys must impl `Ord`
- `ExpiringSizedCache` `remove` and `insert` updated to return only unexpired values
## Removed

## [0.51.2]
## Added
- Add `get_borrowed` methods to `ExpiringSizedCache` to support cache retrieval using `&str` / `&[T]`
  when the key types are `String` / `Vec<T>`. This is a workaround for issues implementing `Borrow`
  for a generic wrapper type.
## Changed
## Removed

## [0.51.1]
## Added
- Update documentation and add missing methods to `ExpiringSizedCache` (clear, configuration methods)
## Changed
- `ExpiringSizedCache`: When allocating using `with_capacity`, allocate enough space to account for
  the default max number of tombstone entries
## Removed

## [0.51.0]
## Added
- Add `ExpiringSizedCache` intended for high read scenarios. Currently incompatible with the cached trait and macros.
## Changed
## Removed

## [0.50.0 / [cached_proc_macro[0.21.0]] ]
## Added
- Add `DiskCacheBuilder::set_sync_to_disk_on_cache_change` to specify that the cache changes should be written to disk on every cache change.
- Add `sync_to_disk_on_cache_change` to `#[io_cached]` to allow setting `DiskCacheBuilder::set_sync_to_disk_on_cache_change` from the proc macro.
- Add `DiskCacheBuilder::set_connection_config` to give more control over the sled connection.
- Add `connection_config` to `#[io_cached]` to allow setting `DiskCacheBuilder::set_connection_config` from the proc macro.
- Add `DiskCache::connection()` and `DiskCache::connection_mut()` to give access to the underlying sled connection.
- Add `cache_unset_lifespan` to cached traits for un-setting expiration on types that support it
## Changed
- [Breaking] `type` attribute is now `ty`
- Upgrade to syn2 
- Corrected a typo in DiskCacheError (de)serialization variants
- Signature or `DiskCache::remove_expired_entries`: this now returns `Result<(), DiskCacheError>` instead of `()`, returning an `Err(sled::Error)` on removing and flushing from the connection.
## Removed

## [0.49.3]
## Added
## Changed
- Fix `DiskCache` expired value logic
## Removed

## [0.49.2]
## Added
## Changed
- While handling cache refreshes in `DiskCache::cache_get`, treat deserialization failures as non-existent values
## Removed

## [0.49.1]
## Added
## Changed
- Fix `DiskCache::remove_expired_entries` signature
## Removed

## [0.49.0 / [cached_proc_macro[0.20.0]] ]
## Added
- Add DiskCache store
- Add `disk=true` (and company) flags to `#[io_cached]`
## Changed
## Removed

## [0.48.1 / [cached_proc_macro[0.19.1]] / [cached_proc_macro_types[0.1.1]]]
## Added
- Include LICENSE file in `cached_proc_macro` and `cached_proc_macro_types`
## Changed
## Removed

## [0.48.0 / [cached_proc_macro[0.19.0]]]
## Added
- Add `CloneCached` trait with additional methods when the cache value type implements `Clone`
- Add `result_fallback` option to `cached` proc_macro to support re-using expired cache values
  when utilizing an expiring cache store and a fallible function.
## Changed
## Removed

## [0.47.0]
## Added
## Changed
- Update redis `0.23.0` -> `0.24.0`
## Removed

## [0.46.1 / [cached_proc_macro[0.18.1]]
## Added
## Changed
- Fix #once sync_writes bug causing a deadlock after ttl expiry, https://github.com/jaemk/cached/issues/174
## Removed

## [0.46.0]
## Added
- Add `ahash` feature to use the faster [ahash](https://github.com/tkaitchuck/aHash) algorithm.
- Set `ahash` as a default feature.
- Update hashbrown `0.13.0` -> `0.14.0`
## Changed
## Removed

## [0.45.1] / [cached_proc_macro[0.18.0]]
## Added
## Changed
- Release `*_no_cache` changes from `0.45.0`. The change is in the proc macro crate which
  I forgot to release a new version of.
## Removed

## [0.45.0]
## Added
- Generate `*_no_cache` function for every cached function to allow calling the original function
  without caching. **This is backwards incompatible if you have a function with the same name**.
## Changed
- `tokio` dependency has been removed from `proc_macro` feature (originally unecessarily included).
- `async` feature has been removed from the `default` feature. **This is a backwards incompatible change.**
  If you want to use `async` features, you need to enable `async` explicitly.
- remove accidental `#[doc(hidden)]` on the `stores` module
## Removed

## [0.44.0] / [cached_proc_macro[0.17.0]]
## Added
- Option to enable redis multiplex-connection manager on `AsyncRedisCache`
## Changed
- Show proc-macro documentation on docs.rs
- Document needed feature flags
- Hide implementation details in documentation
- Relax `Cached` trait's `cache_get`, `cache_get_mut` and `cache_remove` key parameter. Allow `K: Borrow<Q>`
  like `std::collections::HashMap` and friends. Avoids copies particularly on `Cached<String, _>` where now
  you can do `cache.cache_get("key")` and before you had to `cache.cache_get("key".to_string())`.

  Note: This is a minor breaking change for anyone manually implementing the `Cached` trait.
  The signatures of `cache_get`, `cache_get_mut`, and `cache_remove` must be updated to include the
  additional trait bound on the `key` type:
  ```rust
    fn cache_get<Q>(&mut self, key: &Q) -> Option<&V>
    where
        K: std::borrow::Borrow<Q>,
        Q: std::hash::Hash + Eq + ?Sized,
    {
  ```
## Removed
- Dependency to `lazy_static` and `async_once` are removed.

## [0.43.0]
## Added
## Changed
- Update redis `0.22.0` -> `0.23.0`
- Update serial_test `0.10.0` -> `2.0.0`
## Removed

## [0.42.0] / [cached_proc_macro[0.16.0]]
## Added
## Changed
- Better code generation for `#[cached]` when the `sync_writes` flag is true.
## Removed

## [0.41.0]
## Added
## Changed
- Fix "sized" cache types (`SizedCache`, `TimedSizedCache`) to check capacity and evict members after insertion.
- Fixes bug where continuously inserting a key present in the cache would incorrectly evict the oldest cache member
  even though the cache size was not increasing.
## Removed

## [0.40.0]
## Added
- Add optional feature flag `redis_ahash` to enable `redis`'s optional `ahash` feature
## Changed
- Update `redis` to `0.22.0`
- Move `tokio`'s `rt-multi-thread` feature from being a default to being optionally enabled by `async_tokio_rt_multi_thread`
- Fix makefile's doc target to match documentation, changed from `make sync` to `make docs`
## Removed

## [0.39.0]
## Added
- Add flush method to ExpiringValueCache
## Changed
## Removed

## [0.38.0] / [cached_proc_macro[0.15.0]]
## Added
## Changed
- Fix proc macro argument documentation
- Disable futures `default-features`
- Add cache-remove to redis example
## Removed

## [0.37.0] / [cached_proc_macro[0.14.0]]
## Added
## Changed
- Mark the auto-generated "priming" functions with `#[allow(dead_code)]`
- Fix documentation typos
- Replace dev/build scripts with a Makefile
## Removed

## [0.36.0] / [cached_proc_macro[0.13.0]]
## Added
- wasm support for non-io macros and stores
## Changed
- Use `instant` crate for wasm compatible time
## Removed

## [0.35.0]
## Added
- Added `ExpiringValueCache` for caching values that can themselves expire.
- Added COPYRIGHT file
## Changed
## Removed

## [0.34.1]
## Added
- Make sure `AsyncRedisCacheBuilder`, `RedisCacheBuilder`, and `RedisCacheBuildError` publicly visible
## Changed
## Removed

## [0.34.0] / [cached_proc_macro[0.12.0]]
## Added
## Changed
- Replace `async-mutex` and `async-rwlock` used by proc-macros with `tokio::sync` versions
- Add optional `version` field to `CachedRedisValue` struct
- Cleanup feature flags so async redis features include `redis_store` and `async` features automatically
## Removed

## [0.33.0]
## Added
- Allow specifying the namespace added to cache keys generated by redis stores
## Changed
- Bump hashbrown 0.11.2 -> 0.12: https://github.com/rust-lang/hashbrown/blob/master/CHANGELOG.md#v0120---2022-01-17
- Bump smartstring 0.2 -> 1: https://github.com/bodil/smartstring/blob/master/CHANGELOG.md#100---2022-02-24
## Removed

## [0.32.1]
## Added
## Changed
- Fix redis features so `redis/aio` is only included when async redis
  features (`redis_tokio` / `redis_async_std`) are enabled
## Removed

## [0.32.0] / [cached_proc_macro[0.11.0]]
## Added
- Fix how doc strings are handled by proc-macros. Capture all documentation on the
  cached function definitions and add them to the function definitions generated
  by the proc-macros. Add doc strings to generated static caches. Link to relevant static
  caches in generated function definitions. Add documentation to the generated
  cache-priming function.
## Changed
## Removed

## [0.31.0] / [cached_proc_macro[0.10.0]]
## Added
- `IOCached` and `IOCachedAsync` traits
- `RedisCache` and `AsyncRedisCache` store types
- Add `#[io_cached]` proc macro for defining cached functions backed
  by stores that implement `IOCached`/`IOCachedAsync`
## Changed
- Convert from travis-ci to github actions
- Update build status badge to link to github actions
## Removed

## [0.30.0]
## Added
- Add flush method to TimedSize and TimedSized caches
## Changed
- Fix timed/timed-sized cache-get/insert/remove to remove and not
  return expired values
## Removed

## [0.29.0] / [cached_proc_macro[0.9.0]]
## Added
- proc-macro: support arguments of the wrapped function being prefixed with `mut`
## Changed
## Removed

## [0.28.0]
## Added
- Add failable TimedSize and SizeCached constructors
## Changed
## Removed

## [0.27.0] / [cached_proc_macro[0.8.0]]
## Added
- Add `time_refresh` option to `#[cached]` to refresh TTLs on cache hits
- Generate `*_prime_cache` functions for every `#[cached]` and `#[once]` function
  to allow priming caches.
## Changed
## Removed

## [0.26.1] / [cached_proc_macro[0.7.1]]
## Added
- Add `sync_writes` option to `#[cached]` macro to synchronize
  concurrent function calls of duplicate arguments. For ex, if
  a long running `#[cached(sync_writes = true)]` function is called
  several times concurrently, the actual function is only executed
  once while all other calls block and return the newly cached value.
## Changed
## Removed

## [0.26.0] / [cached_proc_macro[0.7.0]]
## Added
- Add `#[once]` macro for create a `RwLock` cache wrapping a single value
- For all caches, add a function to get an immutable reference to their
  contents. This makes it possible to manually dump a cache, so its contents
  can be saved and restored later.
## Changed
## Removed

## [0.25.1]
## Added
## Changed
- Update deps hashbrown and darling, remove async-mutex from cached-proc-macro crate
## Removed

## [0.25.0]
## Added
- Add option to "timed" caches to refresh the ttl of entries on cache hits
## Changed
## Removed

## [0.24.1] / [cached_proc_macro[0.6.1]]
## Added
- Add docs strings to the items generated by the `#cached` proc macro
## Changed
## Removed

## [0.24.0]
## Added
- `cache_reset_metrics` trait method to reset hits/misses
## Changed
## Removed

## [0.23.0]
## Added
## Changed
- Refactor cache store types to separate modules
## Removed

## cached[0.22.0] / cached_proc_macro[0.6.0] / cached_proc_macro_types[0.1.0]
## Added
- Add support for returning a `cached::Return` wrapper type that
  indicates whether the result came from the function's cache.
## Changed
## Removed

## [0.21.1] / [0.5.0]
## Added
- Support mutual `size` & `time` args in the cached proc macro.
  Added when TimedSizedCache was added, but forgot to release
  the cached_proc_macro crate update.
## Changed
## Removed

## [0.21.0]
## Added
- Add a TimedSizedCache combining LRU and timed/ttl logic
## Changed
## Removed

## [0.20.0]
## Added
- Add new CachedAsync trait. Only present with async feature. Adds two async function in the entry API style of HashMap
## Changed
## Removed

## [0.19.0] / [0.4.0]
## Added
## Changed
- Add type hint `_result!` macros
- remove unnecessary transmute in cache reset
- remove unnecessary clones in proc macro
## Removed

## [0.18.0] / [0.3.0]
## Added
## Changed
- use `async-mutex` instead of full `async-std`
## Removed

## [0.17.0]
## Added
## Changed
- Store inner values when `result=true` or `option=true`. The `Error` type in the
`Result` now no longer needs to implement `Clone`.
## Removed

## [0.16.0]
## Added
- add `cache_set_lifespan` to change the cache lifespace, old value returned.
## Changed
## Removed

## [0.15.1]
## Added
## Changed
- fix proc macro when result=true, regression from changing `cache_set` to return the previous value
## Removed

## [0.15.0]
## Added
- add `Cached` implementation for std `HashMap`
## Changed
- trait `Cached` has a new method `cache_get_or_set_with`
- `cache_set` now returns the previous value if any
## Removed

## [0.14.0]
## Added
- add Clone, Debug trait derives on pub types

## Changed

## Removed

## [0.13.1]
## Added

## Changed
- fix proc macro documentation

## Removed

## [0.13.0]
## Added
- proc macro version
- async support when using the new proc macro version

## Changed

## Removed

## [0.12.0]
## Added
- Add `cache_get_mut` to `Cached` trait, to allow mutable access for values in the cache.
- Change the type of `hits` and `misses` to be `u64`.

## Changed

## Removed

## [0.11.0]
## Added
- Add `value_order` method to SizedCache, similar to `key_order`

## Changed

## Removed

## [0.10.0]
## Added
- add `cache_reset` trait method for resetting cache collections to
  their initial state

## Changed
- Update `once_cell` to 1.x

## Removed

## [0.9.0]
## Added

## Changed
- Replace SizedCache implementation to avoid O(n) lookup on cache-get
- Update to Rust-2018 edition
- cargo fmt everything

## Removed


## [0.8.1]
## Added

## Changed
- Replace inner cache when "clearing" unbounded cache

## Removed


## [0.8.0]
## Added

## Changed
- Switch to `once_cell`. Library users no longer need to import `lazy_static`

## Removed

## [0.7.0]
## Added
- Add `cache_clear` and `cache_result` to `Cached` trait
  - Allows for defeating cache entries if desired

## Changed

## Removed

## [0.6.2]
## Added

## Changed
- Update documentation
  - Note the in-memory nature of cache stores
  - Note the behavior of memoized functions under concurrent access

## Removed

## [0.6.1]
## Added

## Changed
- Fixed duplicate key eviction in `SizedCache::cache_set`. This would manifest when
  `cached` functions called with duplicate keys would race set an uncached key,
  or if `SizedCache` was used directly.

## Removed

## [0.6.0]
## Added
- Add `cached_result` and `cached_key_result` to allow the caching of success for a function that returns `Result`.
- Add `cached_control` macro to allow specifying functionality
  at key points of the macro

## [0.5.0]
## Added
- Add `cached_key` macro to allow defining the caching key

## Changed
- Tweak `cached` macro syntax
- Update readme

## Removed


## [0.4.4]
## Added

## Changed
- Update trait docs

## Removed


## [0.4.3]
## Added

## Changed
- Update readme
- Update examples
- Update crate documentation and examples

## Removed
