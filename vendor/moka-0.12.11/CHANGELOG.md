# Moka Cache &mdash; Change Log

## Version 0.12.11

### Added

- Support `Equivalent` trait for the key type `K` of the caches.
  ([#492][gh-pull-0492])
- Added the `jittered_expiry_policy` example ([#489][gh-pull-0489]).

### Changed

- Adjusted license expression: some code is Apache-2.0 only ([#529][gh-pull-0529], by
  [@musicinmybrain][gh-musicinmybrain]).
    - The license expression in `Cargo.toml` was changed from
      `MIT OR Apache-2.0` to `(MIT OR Apache-2.0) AND Apache-2.0`.
    - See the [license section](README.md#license) of the README for details.
- Upgrading a crate in the dependencies:
    - Raised the minimum version of `crossbeam-channel` crate from `v0.5.5` to
      `v0.5.15` to avoid the following issue ([#514][gh-pull-0514],
      by [karankurbur][gh-karankurbur]).
        - [RUSTSEC-2025-0024] crossbeam-channel: double free on Drop
- Moving a crate from the dependencies to the dev-dependencies:
    - Switched `loom` crate to a dev-dependency
      ([#509][gh-pull-0509], by [thomaseizinger][gh-thomaseizinger]).
- Updating a crate in the dev-dependencies:
    - Upgraded `reqwest` crate in the dev-dependencies from `v0.11` to `v0.12`
      ([#531][gh-pull-0531], by [musicinmybrain][gh-musicinmybrain]).

### Removed

- Removing a crate from the dependencies:
    - Removed `thiserror` crate by manually implementing `std::error::Error` for
      `moka::PredicateError` ([#512][gh-pull-0512], by [@brownjohnf][gh-brownjohnf]).
- Removing crates from the dev-dependencies:
    - Removed unmaintained `paste` crate from the dev-dependencies ([#504][gh-pull-0504]).
        - [RUSTSEC-2024-0436] paste - no longer maintained
    - Removed discontinued `async-std` crate from the dev-dependencies ([#534][gh-pull-0534]).
        - [RUSTSEC-2025-0052] async-std has been discontinued
- Removed clippy ignore `non_send_fields_in_send_ty` that no longer applies
  ([#505][gh-pull-0505], by [@qti3e][gh-qti3e]).

### Fixed

- Remove redundant word in source code comment ([#532][gh-pull-0532],
  by [@quantpoet][gh-quantpoet]).


## Version 0.12.10

### Changed

- Disabled the `quanta` feature by default. ([#482][gh-pull-0482])
- Replaced most uses of `quanta::Instant` with `std::time::Instant` to increase the
  accuracy of time measurements ([#481][gh-pull-0481]):
    - When `quanta` feature is enabled, `quanta::Instant` is used for some
      performance critical parts in the cache, and `std::time::Instant` is used for
      the rest of the parts.
    - However, as of this version, enabling the `quanta` feature will not make any
      noticeable difference in the performance.
    - When `quanta` feature is disabled (default), `std::time::Instant` is used for
      all time measurements.
- Switched to `AtomicU64` of the `portable-atomic` crate, which provides fallback
  implementations for platforms where `std` `AtomicU64` is not available
  ([#480][gh-pull-0480]):
    - `moka`'s `atomic64` feature no longer has any effect on the build as
      `AtomicU64` is now always available on all platforms. But we keep the
      `atomic64` feature in `Cargo.toml` for backward compatibility.


## Version 0.12.9

Bumped the minimum supported Rust version (MSRV) to 1.70 (June 1, 2023)
([#474][gh-pull-0474]).

### Fixed

- Prevent an occasional panic in an internal `to_std_instant` method when per-entry
  expiration policy is used. ([#472][gh-issue-0472])
- Documentation: Removed leftover mentions of background threads.
  ([#464][gh-issue-0464])
    - Also added the implementation details chapter to the crate top-level
      documentation to explain some internal behavior of the cache.

### Added

- Added `and_try_compute_if_nobody_else` method to `future::Cache`'s `entry` API.
  ([#460][gh-pull-0460], by [@xuehaonan27][gh-xuehaonan27])

### Removed

- Removed `triomphe` crate from the dependency by adding our own internal `Arc` type.
  ([#456][gh-pull-0456])
    - Our `Arc` will be more memory efficient than `std::sync::Arc` or
      `triomphe::Arc` on 64-bit platforms as it uses a single `AtomicU32` counter.
- Removed needless traits along with `async-trait` usage. ([#445][gh-pull-0445], by
  [@Swatinem][gh-Swatinem])

### Changed

- Enable `atomic64` feature only when target supports `AtomicU64`.
  ([#466][gh-pull-0466], by [@zonyitoo][gh-zonyitoo])
- Made `once_cell` dependency optional ([#444][gh-pull-0444]).
- Stopped creating references unnecessarily to compare pointers by-address.
  ([#452][gh-pull-0452], by [@JoJoDeveloping][gh-JoJoDeveloping])


## Version 0.12.8

### Fixed

- Avoid to use recent versions (`v0.1.12` or newer) of `triomphe` crate to keep our
  MSRV (Minimum Supported Rust Version) at Rust 1.65
  ([#426][gh-pull-0426], by [@eaufavor][gh-eaufavor]).
    - `triomphe@v0.1.12` requires Rust 1.76 or newer, so it will not compile with our
      MSRV.
- docs: Fix per-entry expiration policy documentation ([#421][gh-pull-0421], by
  [@arcstur][gh-arcstur]).


## Version 0.12.7

### Changed

- Ensure a single call to `run_pending_tasks` to evict as many entries as possible
  from the cache ([#417][gh-pull-0417]).


## Version 0.12.6

### Fixed

- Fixed a bug in `future::Cache` that pending `run_pending_tasks` calls may cause
  infinite busy loop in an internal `schedule_write_op` method
  ([#412][gh-issue-0412]):
    - This bug was introduced in `v0.12.0` when the background threads were removed
      from `future::Cache`.
    - This bug can occur when `run_pending_task` method is called by user code while
      cache is receiving a very high number of concurrent cache write operations.
      (e.g. `insert`, `get_with`, `invalidate` etc.)
    - When it occurs, the `schedule_write_op` method will be spinning in a busy loop
      forever, causing high CPU usage and all other async tasks to be starved.

### Changed

- Upgraded `async-lock` crate used by `future::Cache` from `v2.4` to the latest
  `v3.3`.


## Version 0.12.5

### Added

- Added support for a plain LRU (Least Recently Used) eviction policy
    ([#390][gh-pull-0390]):
    - The LRU policy is enabled by calling the `eviction_policy` method of the cache
      builder with a policy obtained by `EvictionPolicy::lru` function.
    - The default eviction policy remains the TinyLFU (Tiny, Least Frequently Used)
      as it maintains better hit rate than LRU for most use cases. TinyLFU combines
      LRU eviction policy and popularity-based admission policy. A probabilistic data
      structure is used to estimate historical popularity of both hit and missed
      keys. (not only the keys currently in the cache.)
    - However, some use cases may prefer LRU policy over TinyLFU. An example is
      recency biased workload such as streaming data processing. LRU policy can be
      used for them to achieve better hit rate.
    - Note that we are planning to add an adaptive eviction/admission policy called
      Window-TinyLFU in the future. It will adjust the balance between recency and
      frequency based on the current workload.


## Version 0.12.4

### Fixed

- Ensure `crossbeam-epoch` to run GC when dropping a cache ([#384][gh-pull-0384]):
    - `crossbeam-epoch` crate provides an epoch-based memory reclamation scheme for
      concurrent data structures. It is used by Moka cache to safely drop cached
      entries while they are still being accessed by other threads.
    - `crossbeam-epoch` does its best to reclaim memory (drop the entries evicted
      from the cache) when the epoch is advanced. However, it does not guarantee that
      memory will be reclaimed immediately after the epoch is advanced. This means
      that entries can remain in the memory for a while after the cache is dropped.
    - This fix ensures that, when a cache is dropped, the epoch is advanced and
      `crossbeam-epoch`'s thread local buffers are flushed, helping to reclaim memory
      immediately.
    - Note that there are still chances that some entries remain in the memory for a
      while after a cache is dropped. We are looking for alternatives to
      `crossbeam-epoch` to improve this situation (e.g. [#385][gh-issue-0385]).

### Added

- Added an example for reinserting expired entries to the cache.
  ([#382][gh-pull-0382])


## Version 0.12.3

### Added

- Added the upsert and compute methods for modifying a cached entry
  ([#370][gh-pull-0370]):
    - Now the `entry` and `entry_by_ref` APIs have the following methods:
        - `and_upsert_with` method to insert or update the entry.
        - `and_compute_with` method to insert, update, remove or do nothing on the
          entry.
        - `and_try_compute_with` method, which is similar to above but returns
          `Result`.

### Fixed

- Raised the version requirement of the `quanta` from `>=0.11.0, <0.12.0` to
  `>=0.12.2, <0.13.0` to avoid under-measuring the elapsed time on Apple silicon
  Macs ([#376][gh-pull-0376]).
    - Due to this under-measurement, cached entries on macOS arm64 can expire sightly
      later than expected.


## Version 0.12.2

### Fixed

- Prevent timing issues in writes that cause inconsistencies between the cache's
  internal data structures ([#348][gh-pull-0348]):
    - One way to trigger the issue is that insert the same key twice quickly, once
      when the cache is full and a second time when there is a room in the cache.
      - When it occurs, the cache will not return the value inserted in the second
        call (which is wrong), and the `entry_count` method will keep returning a non
        zero value after calling the `invalidate_all` method (which is also wrong).
- Now the last access time of a cached entry is updated immediately after the entry
  is read ([#363][gh-pull-0363]):
    - When the time-to-idle of a cache is set, the last access time of a cached entry
      is used to determine if the entry has been expired.
    - Before this fix, the access time was updated (to the time when it was read)
      when pending tasks were processed. This delay caused issue that some entries
      become temporarily unavailable for reads even though they have been accessed
      recently. And then they will become available again after the pending tasks are
      processed.
    - Now the last access time is updated immediately after the entry is read. The
      entry will remain valid until the time-to-idle has elapsed.

Note that both of [#348][gh-pull-0348] and [#363][gh-pull-0363] were already present
in `v0.11.x` and older versions. However they were less likely to occur because they
had background threads to periodically process pending tasks. So there were much
shorter time windows for these issues to occur.

### Changed

- Updated the Rust edition from 2018 to 2021. ([#339][gh-pull-0339], by
  [@nyurik][gh-nyurik])
    - The MSRV remains at Rust 1.65.
- Changed to use inline format arguments throughout the code, including examples.
  ([#340][gh-pull-0340], by [@nyurik][gh-nyurik])

### Added

- Added an example for cascading drop triggered by eviction ([#350][gh-pull-0350], by
  [@peter-scholtens][gh-peter-scholtens])


## Version 0.12.1

### Fixed

- Fixed memory leak in `future::Cache` that occurred when `get_with()`,
  `entry().or_insert_with()`, and similar methods were used ([#329][gh-issue-0329]).
    - This bug was introduced in `v0.12.0`. Versions prior to `v0.12.0` do not
      have this bug.

### Changed

- (Performance)  Micro-optimize `ValueInitializer` ([#331][gh-pull-0331], by
  [@Swatinem][gh-Swatinem]).


## Version 0.12.0

> **Note**
> `v0.12.0` has major breaking changes on the API and internal behavior.

- **`sync` caches are no longer enabled by default**: Please use a crate feature
  `sync` to enable it.

- **No more background threads**: All cache types `future::Cache`, `sync::Cache`, and
  `sync::SegmentedCache` no longer spawn background threads.

  - The `scheduled-thread-pool` crate was removed from the dependency.
  - Because of this change, many private methods and some public methods under the
    `future` module were converted to `async` methods. You may need to add `.await`
    to your code for those methods.

- **Immediate notification delivery**: The `notification::DeliveryMode` enum for the
  eviction listener was removed. Now all cache types behave as if the `Immediate`
  delivery mode is specified.

Please read the [MIGRATION-GUIDE.md][migration-guide-v012] for more details.

[migration-guide-v012]: https://github.com/moka-rs/moka/blob/main/MIGRATION-GUIDE.md#migrating-to-v0120-from-a-prior-version

### Changed

- Removed the thread pool from `future` cache ([#294][gh-pull-0294]) and `sync`
  caches ([#316][gh-pull-0316]).
- Improved async cancellation safety of `future::Cache`. ([#309][gh-pull-0309])

### Fixed

- Fixed a bug that an internal `do_insert_with_hash` method gets the current
  `Instant` too early when eviction listener is enabled. ([#322][gh-issue-0322])


## Version 0.11.3

### Fixed

- Fixed a bug in `sync::Cache` and `sync::SegmentedCache` where memory usage kept
  increasing when the eviction listener was set with the `Immediate` delivery mode.
  ([#295][gh-pull-0295])


## Version 0.11.2

Bumped the minimum supported Rust version (MSRV) to 1.65 (Nov 3, 2022).
([#275][gh-pull-0275])

### Removed

- Removed `num_cpus` crate from the dependency. ([#277][gh-pull-0277])

### Changed

- Refactored internal methods of the concurrent hash table to reduce compile times.
  ([#265][gh-pull-0265], by [@Swatinem][gh-Swatinem])


## Version 0.11.1

### Fixed

-  Fixed occasional panic in internal `FrequencySketch` in debug build.
   ([#272][gh-pull-0272])

### Added

- Added some example programs to the `examples` directory. ([#268][gh-pull-0268], by
  [@peter-scholtens][gh-peter-scholtens])


## Version 0.11.0

### Added

- Added support for per-entry expiration ([#248][gh-pull-0248]):
    - In addition to the existing TTL and TTI (time-to-idle) expiration times that
      apply to all entries in the cache, the `sync` and `future` caches can now allow
      different expiration times for individual entries.
- Added the `remove` method to the `sync` and `future` caches
  ([#255](gh-issue-0255)):
    - Like the `invalidate` method, this method discards any cached value for the
      key, but returns a clone of the value.

### Fixed

- Fixed the caches mutating a deque node through a `NonNull` pointer derived from a
  shared reference. ([#259][gh-pull-0259])

### Removed

- Removed `unsync` cache that was marked as deprecated in [v0.10.0](#version-0100).


## Version 0.10.2

Bumped the minimum supported Rust version (MSRV) to 1.60 (Apr 7, 2022).
([#252][gh-issue-0252])

### Changed

- Upgraded `quanta` crate to v0.11.0. ([#251][gh-pull-0251])
    - This resolved "[RUSTSEC-2020-0168]: `mach` is unmaintained"
      ([#243][gh-issue-0243]) by replacing `mach` with `mach2`.
    - `quanta` v0.11.0's MSRV is 1.60, so we also bumped the MSRV of Moka to 1.60.


## Version 0.10.1

### Fixed

- Fixed a bug that `future` cache's `blocking().invalidate(key)` method does not
  trigger the eviction listener. ([#242][gh-issue-0242])

### Changed

- Now `sync` and `future` caches will not cache anything when the max capacity is set
  to zero ([#230][gh-issue-0230]):
    - Previously, they would cache some entries for short time (< 0.5 secs) even
      though the max capacity is zero.


## Version 0.10.0

### Breaking Changes

- The following caches have been moved to a separate crate called
  [Mini-Moka][mini-moka-crate]:
    - `moka::unsync::Cache` → `mini_moka::unsync::Cache`
    - `moka::dash::Cache` → `mini_moka::sync::Cache`
- The following methods have been removed from `sync` and `future` caches
  ([#199][gh-pull-0199]). They were deprecated in v0.8.0:
    - `get_or_insert_with` (Use `get_with` instead)
    - `get_or_try_insert_with` (Use `try_get_with` instead)
- The following methods of `sync` and `future` caches have been marked as deprecated
  ([#193][gh-pull-0193]):
    - `get_with_if` (Use `entry` API's `or_insert_with_if` instead)

### Added

- Add `entry` and `entry_by_ref` APIs to `sync` and `future` caches
  ([#193][gh-pull-0193]):
    - They allow users to perform more complex operations on a cache entry. At this
      point, the following operations (methods) are provided:
        - `or_default`
        - `or_insert`
        - `or_insert_with`
        - `or_insert_with_if`
        - `or_optionally_insert_with`
        - `or_try_insert_with`
    - The above methods return `Entry` type, which provides `is_fresh` method to
      check if the value was freshly computed or already existed in the cache.


## Version 0.9.7

### Fixed

- Fix an issue that `get_with` method of `future` cache inflates future size by ~7x,
  sometimes causing stack overflow ([#212][gh-issue-0212]):
    - This was caused by a known `rustc` optimization issue on async functions
      ([rust-lang/rust#62958][gh-rust-issue-62958]).
    - Added a workaround to our cache and now it will only inflate the size by ~2.5x.
- Fix a bug that setting the number of segments of `sync` cache will disable
  notifications. ([#207][gh-issue-0207])

### Added

- Add examples for `build_with_hasher` method of cache builders.
  ([#216][gh-pull-0216])


## Version 0.9.6

### Fixed

- Prevent race condition in `get_with` family methods to avoid evaluating `init`
  closure or future multiple times in concurrent calls. ([#195][gh-pull-0195])


## Version 0.9.5

### Added

- Add `optionally_get_with` method to `sync` and `future` caches
  ([#187][gh-pull-0187], by [@LMJW][gh-LMJW]):
    - It is similar to `try_get_with` but takes an init closure/future returning an
      `Option<V>` instead of `Result<V, E>`.
- Add `by_ref` version of API for `get_with`, `optionally_get_with`, and
  `try_get_with` of `sync` and `future` caches ([#190][gh-pull-0190], by
  [@LMJW][gh-LMJW]):
    - They are similar to the non-`by_ref` versions but take a reference of the key
      instead of an owned key. If the key does not exist in the cache, the key will
      be cloned to create new entry in the cache.

### Changed

- Change the CI to run Linux AArch64 tests on real hardware using Cirrus CI.
  ([#180][gh-pull-0180], by [@ClSlaid][gh-ClSlaid])

### Fixed

- Fix a typo in the documentation. ([#189][gh-pull-0189], by [@Swatinem][gh-Swatinem])


## Version 0.9.4

### Fixed

- Fix memory leak after dropping a `sync` or `future` cache ([#177][gh-pull-0177]):
    - This leaked the value part of cache entries.

### Added

- Add an experimental `js` feature to make `unsync` and `sync` caches to compile for
  `wasm32-unknown-unknown` target ([#173](gh-pull-0173), by [@aspect][gh-aspect]):
    - Note that we have not tested if these caches work correctly in wasm32
      environment.


## Version 0.9.3

### Added

- Add an option to the cache builder of the following caches not to start and use the
  global thread pools for housekeeping tasks ([#165][gh-pull-0165]):
    - `sync::Cache`
    - `sync::SegmentedCache`

### Fixed

- Ensure that the following caches will drop the value of evicted entries immediately
  after eviction ([#169][gh-pull-0169]):
    - `sync::Cache`
    - `sync::SegmentedCache`
    - `future::Cache`


## Version 0.9.2

### Fixed

- Fix segmentation faults in `sync` and `future` caches under heavy loads on
  many-core machine ([#34][gh-issue-0034]):
    - NOTE: Although this issue was found in our testing environment ten months ago
      (v0.5.1), no user reported that they had the same issue.
    - NOTE: In [v0.8.4](#version-084), we added a mitigation to reduce the chance of
      the segfaults occurring.

### Changed

- Upgrade crossbeam-epoch from v0.8.2 to v0.9.9 ([#157][gh-pull-0157]):
    - This will make GitHub Dependabot to stop alerting about a security advisory
      [CVE-2022-23639][ghsa-qc84-gqf4-9926] for crossbeam-utils versions < 0.8.7.
    - Moka v0.9.1 or older was _not_ vulnerable to the CVE:
        - Although the older crossbeam-epoch v0.8.2 depends on an affected version of
          crossbeam-utils, epoch v0.8.2 does not use the affected _functions_ of
          utils. ([#162][gh-issue-0162])


## Version 0.9.1

### Fixed

- Relax a too restrictive requirement `Arc<K>: Borrow<Q>` for the key `&Q` of the
  `contains_key`, `get` and `invalidate` methods in the following caches (with `K` as
  the key type) ([#167][gh-pull-0167]). The requirement is now `K: Borrow<Q>` so these
  methods will accept `&[u8]` for the key `&Q` when the stored key `K` is `Vec<u8>`.
    - `sync::Cache`
    - `sync::SegmentedCache`
    - `future::Cache`


## Version 0.9.0

### Added

- Add support for eviction listener to the following caches ([#145][gh-pull-0145]).
  Eviction listener is a callback function that will be called when an entry is
  removed from the cache:
    - `sync::Cache`
    - `sync::SegmentedCache`
    - `future::Cache`
- Add a crate feature `sync` for enabling and disabling `sync` caches.
  ([#141][gh-pull-0141] by [@Milo123459][gh-Milo123459], and [#143][gh-pull-0143])
    - This feature is enabled by default.
    - When using experimental `dash` cache, opting out of `sync` will reduce the
      number of dependencies.
- Add a crate feature `logging` to enable optional log crate dependency.
  ([#159][gh-pull-0159])
    - Currently log will be emitted only when an eviction listener has panicked.


## Version 0.8.6

### Fixed

- Fix a bug caused `invalidate_all` and `invalidate_entries_if` of the following
  caches will not invalidate entries inserted just before calling them
  ([#155][gh-issue-0155]):
    - `sync::Cache`
    - `sync::SegmentedCache`
    - `future::Cache`
    - Experimental `dash::Cache`


## Version 0.8.5

### Added

- Add basic stats (`entry_count` and `weighted_size`) methods to all caches.
  ([#137][gh-pull-0137])
- Add `Debug` impl to the following caches ([#138][gh-pull-0138]):
    - `sync::Cache`
    - `sync::SegmentedCache`
    - `future::Cache`
    - `unsync::Cache`

### Fixed

- Remove unnecessary `K: Clone` bound from the following caches when they are `Clone`
  ([#133][gh-pull-0133]):
    - `sync::Cache`
    - `future::Cache`
    - Experimental `dash::Cache`


## Version 0.8.4

### Fixed

- Fix the following issue by upgrading Quanta crate to v0.10.0 ([#126][gh-pull-0126]):
    - Quanta v0.9.3 or older may not work correctly on some x86_64 machines where
      the Time Stamp Counter (TSC) is not synched across the processor cores.
      ([#119][gh-issue-0119])
    - For more details about the issue, see [the relevant section][panic_in_quanta]
      of the README.

### Added

- Add `get_with_if` method to the following caches ([#123][gh-issue-0123]):
    - `sync::Cache`
    - `sync::SegmentedCache`
    - `future::Cache`

### Changed

The followings are internal changes to improve memory safety in unsafe Rust usages
in Moka:

- Remove pointer-to-integer transmute by converting `UnsafeWeakPointer` from `usize`
  to `*mut T`. ([#127][gh-pull-0127], by [saethlin][gh-saethlin])
- Increase the num segments of the waiters hash table from 16 to 64
  ([#129][gh-pull-0129]) to reduce the chance of the following issue occurring:
    - Segfaults under heavy workloads on a many-core machine. ([#34][gh-issue-0034])


## Version 0.8.3

### Changed

- Make [Quanta crate][quanta-crate] optional (but enabled by default)
  ([#121][gh-pull-0121])
    - Quanta v0.9.3 or older may not work correctly on some x86_64 machines where
      the Time Stamp Counter (TSC) is not synched across the processor cores.
      ([#119][gh-issue-0119])
    - This issue was fixed by Quanta v0.10.0. You can prevent the issue by upgrading
      Moka to v0.8.4 or newer.
    - For more details about the issue, see [the relevant section][panic_in_quanta]
      of the README.


## Version 0.8.2

### Added

- Add iterator to the following caches: ([#114][gh-pull-0114])
    - `sync::Cache`
    - `sync::SegmentedCache`
    - `future::Cache`
    - `unsync::Cache`
- Implement `IntoIterator` to the all caches (including experimental `dash::Cache`)
  ([#114][gh-pull-0114])

### Fixed

- Fix the `dash::Cache` iterator not to return expired entries.
  ([#116][gh-pull-0116])
- Prevent "index out of bounds" error when `sync::SegmentedCache` was created with
  a non-power-of-two segments. ([#117][gh-pull-0117])


## Version 0.8.1

### Added

- Add `contains_key` method to check if a key is present without resetting the idle
  timer or updating the historic popularity estimator. ([#107][gh-issue-0107])


## Version 0.8.0

As a part of stabilizing the cache API, the following cache methods have been renamed:

- `get_or_insert_with(K, F)` → `get_with(K, F)`
- `get_or_try_insert_with(K, F)` → `try_get_with(K, F)`

Old methods are still available but marked as deprecated. They will be removed in a
future version.

Also `policy` method was added to all caches and `blocking` method was added to
`future::Cache`. They return a `Policy` struct or `BlockingOp` struct
respectively. Some uncommon cache methods were moved to these structs, and old
methods were removed without deprecating.

Please see [#105][gh-pull-0105] for the complete list of the affected methods.

### Changed

- API stabilization. (Smaller core cache API, shorter names for common methods)
  ([#105][gh-pull-0105])
- Performance related:
    - Improve performance of `get_with` and `try_get_with`. ([#88][gh-pull-0088])
    - Avoid to calculate the same hash twice in `get`, `get_with`, `insert`,
      `invalidate`, etc. ([#90][gh-pull-0090])
- Update the minimum versions of dependencies:
    - crossbeam-channel to v0.5.4. ([#100][gh-pull-0100])
    - scheduled-thread-pool to v0.2.5. ([#103][gh-pull-0103], by
      [@Milo123459][gh-Milo123459])
    - (dev-dependency) skeptic to v0.13.5. ([#104][gh-pull-0104])

### Added

#### Experimental Additions

- Add a synchronous cache `moka::dash::Cache`, which uses `dashmap::DashMap` as the
  internal storage. ([#99][gh-pull-0099])
- Add iterator to `moka::dash::Cache`. ([#101][gh-pull-0101])

Please note that the above additions are highly experimental and their APIs will
be frequently changed in next few releases.


## Version 0.7.2

The minimum supported Rust version (MSRV) is now 1.51.0 (Mar 25, 2021).

### Fixed

- Addressed a memory utilization issue that will get worse when keys have hight
  cardinality ([#72][gh-issue-0072]):
    - Reduce memory overhead in the internal concurrent hash table (cht).
      ([#79][gh-pull-0079])
    - Fix a bug that can create oversized frequency sketch when weigher is set.
      ([#75][gh-pull-0075])
    - Change `EntryInfo` from `enum` to `struct` to reduce memory utilization.
      ([#76][gh-pull-0076])
    - Replace some `std::sync::Arc` usages with `triomphe::Arc` to reduce memory
      utilization. ([#80][gh-pull-0080])
    - Embed `CacheRegion` value into a 2-bit tag space of `TagNonNull` pointer.
      ([#84][gh-pull-0084])
- Fix a bug that will use wrong (oversized) initial capacity for the internal cht.
  ([#83][gh-pull-0083])

### Added

- Add `unstable-debug-counters` feature for testing purpose. ([#82][gh-pull-0082])

### Changed

- Import (include) cht source files for better integration. ([#77][gh-pull-0077],
  [#86](gh-pull-0086))
- Improve the CI coverage for Clippy lints and fix some Clippy warnings in unit
  tests. ([#73][gh-pull-0073], by [@06chaynes][gh-06chaynes])


## Version 0.7.1

- **Important Fix**: A memory leak issue (#65 below) was found in all previous
  versions (since v0.1.0) and fixed in this version. All users are encouraged to
  upgrade to this or newer version.

### Fixed

- Fix a memory leak that will happen when evicting/expiring an entry or manually
  invalidating an entry. ([#65][gh-pull-0065])

### Changed

- Update the minimum depending version of crossbeam-channel from v0.5.0 to v0.5.2.
  ([#67][gh-pull-0067])


## Version 0.7.0

- **Breaking change**: The type of the `max_capacity` has been changed from `usize`
  to `u64`. This was necessary to have the weight-based cache management consistent
  across different CPU architectures.

### Added

- Add support for weight-based (size aware) cache management.
  ([#24][gh-pull-0024])
- Add support for unbound cache. ([#24][gh-pull-0024])


## Version 0.6.3

### Fixed

- Fix a bug in `get_or_insert_with` and `get_or_try_insert_with` methods of
  `future::Cache`, which caused a panic if previously inserting task aborted.
  ([#59][gh-issue-0059])


## Version 0.6.2

### Removed

- Remove `Send` and `'static` bounds from `get_or_insert_with` and
  `get_or_try_insert_with` methods of `future::Cache`. ([#53][gh-pull-0053], by
  [@tinou98][gh-tinou98])

### Fixed

- Protect overflow when computing expiration. ([#56][gh-pull-0056], by
  [@barkanido][gh-barkanido])


## Version 0.6.1

### Changed

- Replace futures crate with futures-util. ([#47][gh-pull-0047], by
  [@messense][gh-messense]))


## Version 0.6.0

### Fixed

- Fix a bug in `get_or_insert_with` and `get_or_try_insert_with` methods of
  `future::Cache` and `sync::Cache`; a panic in the `init` future/closure
  causes subsequent calls on the same key to get "unreachable code" panics.
  ([#43][gh-issue-0043])

### Changed

- Change `get_or_try_insert_with` to return a concrete error type rather
  than a trait object. ([#23][gh-pull-0023], [#37][gh-pull-0037])


## Version 0.5.4

### Changed

-  Restore quanta dependency on some 32-bit platforms such as
   `armv5te-unknown-linux-musleabi` or `mips-unknown-linux-musl`.
   ([#42][gh-pull-0042], by [@messense][gh-messense])


## Version 0.5.3

### Added

- Add support for some 32-bit platforms where `std::sync::atomic::AtomicU64` is not
  provided. (e.g. `armv5te-unknown-linux-musleabi` or `mips-unknown-linux-musl`)
  ([#38][gh-issue-0038])
    - On these platforms, you will need to disable the default features of Moka.
      See [the relevant section][resolving-error-on-32bit] of the README.


## Version 0.5.2

### Fixed

- Fix a bug in `get_or_insert_with` and `get_or_try_insert_with` methods of
  `future::Cache` by adding missing bounds `Send` and `'static` to the `init`
  future. Without this fix, these methods will accept non-`Send` or
  non-`'static` future and may cause undefined behavior.
  ([#31][gh-issue-0031])
- Fix `usize` overflow on big cache capacity. ([#28][gh-pull-0028])

### Added

- Add examples for `get_or_insert_with` and `get_or_try_insert_with`
  methods to the docs. ([#30][gh-pull-0030])

### Changed

- Downgrade crossbeam-epoch used in moka-cht from v0.9.x to v0.8.x as a possible
  workaround for segmentation faults on many-core CPU machines.
  ([#33][gh-pull-0033])


## Version 0.5.1

### Changed

- Replace a dependency cht v0.4 with moka-cht v0.5. ([#22][gh-pull-0022])


## Version 0.5.0

### Added

- Add `get_or_insert_with` and `get_or_try_insert_with` methods to `sync` and
  `future` caches. ([#20][gh-pull-0020])


## Version 0.4.0

### Fixed

- **Breaking change**: Now `sync::{Cache, SegmentedCache}` and `future::Cache`
  require `Send`, `Sync` and `'static` for the generic parameters `K` (key),
  `V` (value) and `S` (hasher state). This is necessary to prevent potential
  undefined behaviors in applications using single-threaded async runtime such as
  Actix-rt. ([#19][gh-pull-0019])

### Added

- Add `invalidate_entries_if` method to `sync`, `future` and `unsync` caches.
  ([#12][gh-pull-0012])


## Version 0.3.1

### Changed

- Stop skeptic from having to be compiled by all downstream users.
  ([#16][gh-pull-0016], by [@paolobarbolini][gh-paolobarbolini])


## Version 0.3.0

### Added

- Add an unsync cache (`moka::unsync::Cache`) and its builder for single-thread
  applications. ([#9][gh-pull-0009])
- Add `invalidate_all` method to `sync`, `future` and `unsync` caches.
  ([#11][gh-pull-0011])

### Fixed

- Fix problems including segfault caused by race conditions between the sync/eviction
  thread and client writes. (Addressed as a part of [#11][gh-pull-0011]).


## Version 0.2.0

### Added

- Add an asynchronous, futures aware cache (`moka::future::Cache`) and its builder.
  ([#7][gh-pull-0007])


## Version 0.1.0

### Added

- Add thread-safe, highly concurrent in-memory cache implementations
  (`moka::sync::{Cache, SegmentedCache}`) with the following features:
    - Bounded by the maximum number of elements.
    - Maintains good hit rate by using entry replacement algorithms inspired by
      [Caffeine][caffeine-git]:
        - Admission to a cache is controlled by the Least Frequently Used (LFU) policy.
        - Eviction from a cache is controlled by the Least Recently Used (LRU) policy.
    - Expiration policies:
        - Time to live
        - Time to idle


<!-- Links -->

[caffeine-git]: https://github.com/ben-manes/caffeine
[mini-moka-crate]: https://crates.io/crates/mini-moka
[quanta-crate]: https://crates.io/crates/quanta

[panic_in_quanta]: https://github.com/moka-rs/moka#integer-overflow-in-quanta-crate-on-some-x86_64-machines
[resolving-error-on-32bit]: https://github.com/moka-rs/moka#compile-errors-on-some-32-bit-platforms

[ghsa-qc84-gqf4-9926]: https://github.com/advisories/GHSA-qc84-gqf4-9926
[gh-rust-issue-62958]: https://github.com/rust-lang/rust/issues/62958

[RUSTSEC-2025-0052]: https://rustsec.org/advisories/RUSTSEC-2025-0052.html
[RUSTSEC-2025-0024]: https://rustsec.org/advisories/RUSTSEC-2025-0024.html
[RUSTSEC-2024-0436]: https://rustsec.org/advisories/RUSTSEC-2024-0436.html
[RUSTSEC-2020-0168]: https://rustsec.org/advisories/RUSTSEC-2020-0168.html

[gh-06chaynes]: https://github.com/06chaynes
[gh-arcstur]: https://github.com/arcstur
[gh-aspect]: https://github.com/aspect
[gh-barkanido]: https://github.com/barkanido
[gh-brownjohnf]: https://github.com/brownjohnf
[gh-ClSlaid]: https://github.com/ClSlaid
[gh-eaufavor]: https://github.com/eaufavor
[gh-JoJoDeveloping]: https://github.com/JoJoDeveloping
[gh-karankurbur]: https://github.com/karankurbur
[gh-LMJW]: https://github.com/LMJW
[gh-messense]: https://github.com/messense
[gh-Milo123459]: https://github.com/Milo123459
[gh-musicinmybrain]: https://github.com/musicinmybrain
[gh-nyurik]: https://github.com/nyurik
[gh-paolobarbolini]: https://github.com/paolobarbolini
[gh-peter-scholtens]: https://github.com/peter-scholtens
[gh-qti3e]: https://github.com/qti3e
[gh-quantpoet]: https://github.com/quantpoet
[gh-saethlin]: https://github.com/saethlin
[gh-Swatinem]: https://github.com/Swatinem
[gh-thomaseizinger]: https://github.com/thomaseizinger
[gh-tinou98]: https://github.com/tinou98
[gh-xuehaonan27]: https://github.com/xuehaonan27
[gh-zonyitoo]: https://github.com/zonyitoo

[gh-issue-0472]: https://github.com/moka-rs/moka/issues/472/
[gh-issue-0464]: https://github.com/moka-rs/moka/issues/464/
[gh-issue-0412]: https://github.com/moka-rs/moka/issues/412/
[gh-issue-0385]: https://github.com/moka-rs/moka/issues/385/
[gh-issue-0329]: https://github.com/moka-rs/moka/issues/329/
[gh-issue-0322]: https://github.com/moka-rs/moka/issues/322/
[gh-issue-0255]: https://github.com/moka-rs/moka/issues/255/
[gh-issue-0252]: https://github.com/moka-rs/moka/issues/252/
[gh-issue-0243]: https://github.com/moka-rs/moka/issues/243/
[gh-issue-0242]: https://github.com/moka-rs/moka/issues/242/
[gh-issue-0230]: https://github.com/moka-rs/moka/issues/230/
[gh-issue-0212]: https://github.com/moka-rs/moka/issues/212/
[gh-issue-0207]: https://github.com/moka-rs/moka/issues/207/
[gh-issue-0162]: https://github.com/moka-rs/moka/issues/162/
[gh-issue-0155]: https://github.com/moka-rs/moka/issues/155/
[gh-issue-0123]: https://github.com/moka-rs/moka/issues/123/
[gh-issue-0119]: https://github.com/moka-rs/moka/issues/119/
[gh-issue-0107]: https://github.com/moka-rs/moka/issues/107/
[gh-issue-0072]: https://github.com/moka-rs/moka/issues/72/
[gh-issue-0059]: https://github.com/moka-rs/moka/issues/59/
[gh-issue-0043]: https://github.com/moka-rs/moka/issues/43/
[gh-issue-0038]: https://github.com/moka-rs/moka/issues/38/
[gh-issue-0034]: https://github.com/moka-rs/moka/issues/34/
[gh-issue-0031]: https://github.com/moka-rs/moka/issues/31/

[gh-pull-0534]: https://github.com/moka-rs/moka/pull/534/
[gh-pull-0532]: https://github.com/moka-rs/moka/pull/532/
[gh-pull-0531]: https://github.com/moka-rs/moka/pull/531/
[gh-pull-0529]: https://github.com/moka-rs/moka/pull/529/
[gh-pull-0514]: https://github.com/moka-rs/moka/pull/514/
[gh-pull-0512]: https://github.com/moka-rs/moka/pull/512/
[gh-pull-0509]: https://github.com/moka-rs/moka/pull/509/
[gh-pull-0505]: https://github.com/moka-rs/moka/pull/505/
[gh-pull-0504]: https://github.com/moka-rs/moka/pull/504/
[gh-pull-0503]: https://github.com/moka-rs/moka/pull/503/
[gh-pull-0492]: https://github.com/moka-rs/moka/pull/492/
[gh-pull-0489]: https://github.com/moka-rs/moka/pull/489/
[gh-pull-0482]: https://github.com/moka-rs/moka/pull/482/
[gh-pull-0481]: https://github.com/moka-rs/moka/pull/481/
[gh-pull-0480]: https://github.com/moka-rs/moka/pull/480/
[gh-pull-0474]: https://github.com/moka-rs/moka/pull/474/
[gh-pull-0466]: https://github.com/moka-rs/moka/pull/466/
[gh-pull-0460]: https://github.com/moka-rs/moka/pull/460/
[gh-pull-0456]: https://github.com/moka-rs/moka/pull/456/
[gh-pull-0452]: https://github.com/moka-rs/moka/pull/452/
[gh-pull-0445]: https://github.com/moka-rs/moka/pull/445/
[gh-pull-0444]: https://github.com/moka-rs/moka/pull/444/
[gh-pull-0426]: https://github.com/moka-rs/moka/pull/426/
[gh-pull-0421]: https://github.com/moka-rs/moka/pull/421/
[gh-pull-0417]: https://github.com/moka-rs/moka/pull/417/
[gh-pull-0390]: https://github.com/moka-rs/moka/pull/390/
[gh-pull-0384]: https://github.com/moka-rs/moka/pull/384/
[gh-pull-0382]: https://github.com/moka-rs/moka/pull/382/
[gh-pull-0376]: https://github.com/moka-rs/moka/pull/376/
[gh-pull-0370]: https://github.com/moka-rs/moka/pull/370/
[gh-pull-0363]: https://github.com/moka-rs/moka/pull/363/
[gh-pull-0350]: https://github.com/moka-rs/moka/pull/350/
[gh-pull-0348]: https://github.com/moka-rs/moka/pull/348/
[gh-pull-0340]: https://github.com/moka-rs/moka/pull/340/
[gh-pull-0339]: https://github.com/moka-rs/moka/pull/339/
[gh-pull-0331]: https://github.com/moka-rs/moka/pull/331/
[gh-pull-0316]: https://github.com/moka-rs/moka/pull/316/
[gh-pull-0309]: https://github.com/moka-rs/moka/pull/309/
[gh-pull-0295]: https://github.com/moka-rs/moka/pull/295/
[gh-pull-0294]: https://github.com/moka-rs/moka/pull/294/
[gh-pull-0277]: https://github.com/moka-rs/moka/pull/277/
[gh-pull-0275]: https://github.com/moka-rs/moka/pull/275/
[gh-pull-0272]: https://github.com/moka-rs/moka/pull/272/
[gh-pull-0268]: https://github.com/moka-rs/moka/pull/268/
[gh-pull-0265]: https://github.com/moka-rs/moka/pull/265/
[gh-pull-0259]: https://github.com/moka-rs/moka/pull/259/
[gh-pull-0251]: https://github.com/moka-rs/moka/pull/251/
[gh-pull-0248]: https://github.com/moka-rs/moka/pull/248/
[gh-pull-0216]: https://github.com/moka-rs/moka/pull/216/
[gh-pull-0199]: https://github.com/moka-rs/moka/pull/199/
[gh-pull-0195]: https://github.com/moka-rs/moka/pull/195/
[gh-pull-0193]: https://github.com/moka-rs/moka/pull/193/
[gh-pull-0190]: https://github.com/moka-rs/moka/pull/190/
[gh-pull-0189]: https://github.com/moka-rs/moka/pull/189/
[gh-pull-0187]: https://github.com/moka-rs/moka/pull/187/
[gh-pull-0180]: https://github.com/moka-rs/moka/pull/180/
[gh-pull-0177]: https://github.com/moka-rs/moka/pull/177/
[gh-pull-0173]: https://github.com/moka-rs/moka/pull/173/
[gh-pull-0169]: https://github.com/moka-rs/moka/pull/169/
[gh-pull-0167]: https://github.com/moka-rs/moka/pull/167/
[gh-pull-0165]: https://github.com/moka-rs/moka/pull/165/
[gh-pull-0159]: https://github.com/moka-rs/moka/pull/159/
[gh-pull-0157]: https://github.com/moka-rs/moka/pull/157/
[gh-pull-0145]: https://github.com/moka-rs/moka/pull/145/
[gh-pull-0143]: https://github.com/moka-rs/moka/pull/143/
[gh-pull-0141]: https://github.com/moka-rs/moka/pull/141/
[gh-pull-0138]: https://github.com/moka-rs/moka/pull/138/
[gh-pull-0137]: https://github.com/moka-rs/moka/pull/137/
[gh-pull-0133]: https://github.com/moka-rs/moka/pull/133/
[gh-pull-0129]: https://github.com/moka-rs/moka/pull/129/
[gh-pull-0127]: https://github.com/moka-rs/moka/pull/127/
[gh-pull-0126]: https://github.com/moka-rs/moka/pull/126/
[gh-pull-0121]: https://github.com/moka-rs/moka/pull/121/
[gh-pull-0117]: https://github.com/moka-rs/moka/pull/117/
[gh-pull-0116]: https://github.com/moka-rs/moka/pull/116/
[gh-pull-0114]: https://github.com/moka-rs/moka/pull/114/
[gh-pull-0105]: https://github.com/moka-rs/moka/pull/105/
[gh-pull-0104]: https://github.com/moka-rs/moka/pull/104/
[gh-pull-0103]: https://github.com/moka-rs/moka/pull/103/
[gh-pull-0101]: https://github.com/moka-rs/moka/pull/101/
[gh-pull-0100]: https://github.com/moka-rs/moka/pull/100/
[gh-pull-0099]: https://github.com/moka-rs/moka/pull/99/
[gh-pull-0090]: https://github.com/moka-rs/moka/pull/90/
[gh-pull-0088]: https://github.com/moka-rs/moka/pull/88/
[gh-pull-0086]: https://github.com/moka-rs/moka/pull/86/
[gh-pull-0084]: https://github.com/moka-rs/moka/pull/84/
[gh-pull-0083]: https://github.com/moka-rs/moka/pull/83/
[gh-pull-0082]: https://github.com/moka-rs/moka/pull/82/
[gh-pull-0080]: https://github.com/moka-rs/moka/pull/80/
[gh-pull-0079]: https://github.com/moka-rs/moka/pull/79/
[gh-pull-0077]: https://github.com/moka-rs/moka/pull/77/
[gh-pull-0076]: https://github.com/moka-rs/moka/pull/76/
[gh-pull-0075]: https://github.com/moka-rs/moka/pull/75/
[gh-pull-0073]: https://github.com/moka-rs/moka/pull/73/
[gh-pull-0067]: https://github.com/moka-rs/moka/pull/67/
[gh-pull-0065]: https://github.com/moka-rs/moka/pull/65/
[gh-pull-0056]: https://github.com/moka-rs/moka/pull/56/
[gh-pull-0053]: https://github.com/moka-rs/moka/pull/53/
[gh-pull-0047]: https://github.com/moka-rs/moka/pull/47/
[gh-pull-0042]: https://github.com/moka-rs/moka/pull/42/
[gh-pull-0037]: https://github.com/moka-rs/moka/pull/37/
[gh-pull-0033]: https://github.com/moka-rs/moka/pull/33/
[gh-pull-0030]: https://github.com/moka-rs/moka/pull/30/
[gh-pull-0028]: https://github.com/moka-rs/moka/pull/28/
[gh-pull-0024]: https://github.com/moka-rs/moka/pull/24/
[gh-pull-0023]: https://github.com/moka-rs/moka/pull/23/
[gh-pull-0022]: https://github.com/moka-rs/moka/pull/22/
[gh-pull-0020]: https://github.com/moka-rs/moka/pull/20/
[gh-pull-0019]: https://github.com/moka-rs/moka/pull/19/
[gh-pull-0016]: https://github.com/moka-rs/moka/pull/16/
[gh-pull-0012]: https://github.com/moka-rs/moka/pull/12/
[gh-pull-0011]: https://github.com/moka-rs/moka/pull/11/
[gh-pull-0009]: https://github.com/moka-rs/moka/pull/9/
[gh-pull-0007]: https://github.com/moka-rs/moka/pull/7/
