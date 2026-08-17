# Changelog


## Version 3

3.5.6

* Add support for `MIRIFLAGS="-Zmiri-strict-provenance"`.
* Fine-tune the `Hash*` resizing strategy.
* Reduce lock contention during `TreeIndex` node split operations.

3.5.5

* Better `Hash*` resizing strategy: minimize any effect of mutable iterators on sampling.
* Reduce the number of `memcpy` operations during `TreeIndex::insert*`.

3.5.4

* Optimize the `Hash*` resizing strategy: reduce performance fluctuations by more accurately estimating the number of entries.

3.5.3

* Implement `Clone` for `HashIndex::Iter` and `TreeIndex::{Iter, Range}`.

3.5.2

* Add `TreeIndex::{Iter, Range}::{get, get_back, flip}`.

3.5.1

* Implement `TreeIndex::locate` to locate the nearest key if the key is absent: [#203](https://codeberg.org/wvwwvwwv/scalable-concurrent-containers/issues/203).

3.5.0

* Stabilize `DoubleEndedIterator` for `TreeIndex::{Iter, Range}`: [#217](https://codeberg.org/wvwwvwwv/scalable-concurrent-containers/issues/217).
* `TreeIndex::{Iter, Range}` performance improvements.

3.4.16

* Optimize linked list modification during `TreeIndex::remove_range*`.
* Fix data race issues in the `TreeIndex` range iterator.

3.4.15 (yanked)

* Implement `DoubleEndedIterator` for `tree_index::Range`: [#217](https://codeberg.org/wvwwvwwv/scalable-concurrent-containers/issues/217).
 
3.4.14

* Implement `DoubleEndedIterator` for `tree_index::Iter`: [#217](https://codeberg.org/wvwwvwwv/scalable-concurrent-containers/issues/217).

3.4.13

* Remove the `V: Clone` bound from `TreeIndex`.

3.4.12

* Optimize `TreeIndex` entry iteration.

3.4.11 - 3.4.12

* Code cleanup.

3.4.10

* Minor `Hash*` performance improvements.

3.4.9

* Migrate to [`codeberg`](https://codeberg.org/wvwwvwwv/scalable-concurrent-containers).
* Better `Hash*` OOM handling.

3.4.8

* Minor updates for `codeberg` migration.

3.4.7

* Minor `Future` size improvement for `Hash*` containers.

3.4.6

* Improve `HashIndex` entry slot recycling strategies.

3.4.5

* Minor `Hash*` performance improvements.

3.4.4

* Minor `Future` size improvement.
* Simplify `HashIndex` entry removal policies.
* Optimize `HashIndex` for `!needs_drop::<(K, V)>()`.

3.4.3

* Maximum capacity limit is adjusted to `2^(usize::BITS - 2)`.
* Minor `Future` size improvement.

3.4.2

* Minor optimization.

3.4.1

* Migrate `Bag`, `LinkedList`, `Queue`, and `Stack` to `sdd`.

3.3.7 - 3.3.8

* Improve sampling accuracy in `Hash*` containers.

3.3.6

* Simplify `Hash*` capacity management.
* Fix the `loom` feature to pass the proper loom option to `saa`.

3.3.5

* Enhance `Hash*` capacity management.

3.3.4

* Minor asynchronous code optimization.

3.3.3

* API update: add `{HashMap, HashCache}::{replace_async, replace_sync}`.

3.3.2

* Remove the `'static` bound from `HashIndex`: [#200](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/200).
* Ensure that `HashIndex` entries are deallocated when the `HashIndex` is dropped: [#198](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/198).

3.3.1

* More conservative grow/shrink policies in `Hash*` containers to avoid wrong load factor estimation.
* Deallocate `HashIndex` entries as early as possible: [#198](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/198).

3.3.0

* API update: `HashIndex::{peek, iter}` no longer allow entry references to outlive the `HashIndex`: [#198](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/198).
* More memory usage optimization: [#194](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/194).

3.2.0

* API update: `hash_index::OccupiedEntry::update` does not consume itself.
* API update: add `HashSet::replace_{async|sync}`.
* Minor memory usage optimization: [#194](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/194).
* Fix a data race between `TreeIndex::clear` and `TreeIndex::insert*`.

3.1.2

* Minor memory usage optimization.

3.1.1

* Remove the 128B alignment requirement for `Hash*` asynchronous methods.

3.1.0

* Optimize stack memory usage of asynchronous `Hash*` methods: [#194](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/194).

3.0.7

* Fix a rare data race in `Hash*` iterator methods.

3.0.5 - 3.0.6

* Fix a potential duplicate key issue with `Hash*` containers if `HashMap::insert_*` fails to allocate memory.

3.0.3 - 3.0.4

* Fix potential data races in asynchronous operations of `Hash*` containers.

3.0.1 - 3.0.2

* Dependencies updated to the latest versions.

3.0.0

* New API.

### API update

* See [`#186`](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/186).


## Version 2

2.4.0

* Add `Hash*::try_entry`.

2.3.4

* Limit the maximum initial capacity of serialized containers to `1 << 24`.

2.3.3

* Minor performance optimization for `HashIndex`.

2.3.1

* **Yanked `[2.0.0, 2.3.0]` due to a use-after-free problem: [#176](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/176).**
* Fix a problem with `HashCache::read` and `HashMap::read` where the read lock is dropped too early: [#176](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/176).
* Fix `HashCache` documentation: [#175](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/175).
* Implement `FromIterator` for all collection types: [#173](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/173).

2.3.0

* Fix incorrect Sync trait bounds of `Bag` and `Hash*`.

2.2.6

* Fix linting errors.

2.2.5

* `Hash*::with_hasher` is now a const function: by [Daniel-Aaron-Bloom](https://github.com/Daniel-Aaron-Bloom).
* Fix the `HashMap::upsert` document: by [dfaust](https://github.com/dfaust).

2.2.4

* Minor `Hash*` performance optimization.

2.2.3

* Minor `Hash*` memory optimization.

2.2.2

* Fix a data race between `TreeIndex::clear` and `TreeIndex::insert`.
* Update internal doc.

2.2.1

* Fix a minor Miri specific assertion failure.

2.2.0

* Add `Comparable` and `Equivalent` traits: [#162](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/162) by [qthree](https://github.com/qthree).

2.1.18

* Add `TreeIndex::peek_entry`: [#157](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/157) by [qthree](https://github.com/qthree).
* `tree_index::Range` accepts borrowed key ranges: [#158](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/158) by [qthree](https://github.com/qthree).

2.1.17

* Optimize `TreeIndex::{clear, drop}`: [#156](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/156).

2.1.16

* Fix potential data races in `HashMap` and `TreeIndex`.

2.1.15

* Add `upsert` to `HashMap`.

2.1.14

* Fix theoretical data race issues in `TreeIndex`: replace dependent loads with load-acquires.

2.1.13

* Fix a data race in `TreeIndex::insert*`.

2.1.12

* Bump `SDD` to `3.0`.
* Update doc to clearly state that `Hash*::get*` owns the entry for modification, and recommend `Hash*::read*` for read-only access.

2.1.11

* Activate Miri tests: [#88](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/88).

2.1.10

* Add `loom` support.
* Fix data races in `TreeIndex`: [#153](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/153).

2.1.9

* Fix a correctness issue with `Stack`, `Queue`, and `Bag::pop`: [#153](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/153).

2.1.8

* Fix a correctness issue with `TreeIndex::remove_range`: [#153](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/153).
* Add `TreeIndex::remove_range_async`: [#123](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/123).

2.1.7

* Fix a correctness issue with `HashMap` on a 32-bit CPU: [#153](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/153).

2.1.6

* Minor optimization: bump `SDD` to `2.1.0`.

2.1.5

* Optimize `LinkedList` deleted node reclamation.
* Optimize `Bag` by not allowing pushed instances to be dropped without being used.

2.1.4

* Implement more aggressive entry removal in `HashIndex` ([#150](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/150)).

2.1.3

* Update [`sdd`](https://crates.io/crates/sdd).

2.1.2

* Implement `any_entry` and `any_entry_async` to `HashMap` and `HashIndex` ([#148](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/148)).

2.1.1

* Implement `Deref` and `DerefMut` for `OccupiedEntry`: [#140](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/140) by [gituser-rs](https://github.com/gituser-rs)

2.1.0

* Use [`sdd`](https://crates.io/crates/sdd) as the memory reclaimer.

2.0.20

* Relax trait bounds of `TreeIndex`.

2.0.19

* Remove unnecessary trait bounds from type definitions of `HashCache`, `HashIndex`, `HashMap`, and `HashSet`.
* Fix [#135](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/135).

2.0.18

* API update: add `Entry::take_inner`.
* Much faster `Queue` and `Stack` entry garbage collection.

2.0.17

* Faster `Queue` and `Stack` entry garbage collection.

2.0.16

* Fix an issue with `HashCache` where an evicted entry is dropped without notifying it when `HashCache` shrinks.

2.0.15

* Fix [#122](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/122).

2.0.14

* Fix [#120](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/120).
* More aggressive `TreeIndex` root node cleanup.

2.0.13

* Add `iter` to `Queue` and `Stack`.
* Add `len` to `Bag`, `Queue`, and `Stack`.

2.0.12

* Fix an issue with `tree_index::Range` where it intermittently fails to find the minimum key in a `TreeIndex` if the `TreeIndex` is being updated by other threads.

2.0.10 - 2.0.11

* Fix [#121](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/121).

2.0.9

* Add `TreeIndex::contains` and `TreeIndex::remove_range`; `TreeIndex::remove_range` is experimental and will be stabilized later ([#120](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/120)).

2.0.8

* Add support for old Rust versions >= 1.65.0.

2.0.7

* Add `bucket_index` to `HashIndex`, `HashMap`, and `HashSet`.

2.0.6

* Fix [#118](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/118).

2.0.5

* Add support for 32-bit binaries.
* Fix [#116](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/116).

2.0.4

* Fix [#115](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/115).

2.0.3

* Add `IntoIter` to `Bag`.

2.0.2

* Enhance the wait queue implementation.

2.0.1

* Minor code cleanup.

2.0.0

* New API.

### API update

* See [`#108`](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/108).


## Version 1

1.9.1

* API update: add the `hash_index::Entry` API.

1.9.0

* API update: add `ebr::{AtomicOwned, Owned}` for non-reference-counted instances.

1.8.3

* API update: add `ebr::AtomicArc::compare_exchange_weak`.

1.8.2

* API update: [#107](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/107) add `HashMap::{prune, prune_async}`.

1.8.1

* API update: add `HashCache::{contains, contains_async}`.

1.8.0

* API update: overhaul `hash_cache::Entry` API; values can be evicted through `hash_cache::Entry` API.

1.7.3

* Add `Bag::pop_all` and `Stack::pop_all`.

1.7.2

* Add `HashCache::{any, any_async, for_each, for_each_async, read, read_async}`.

1.7.1

* Add `Serde` support to `HashCache`.

1.7.0

* Optimize `Hash*::update*` and `HashIndex::modify*`.
* API update 1: [#94](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/94) a _WORK IN PROGRESS_ `HashCache` minimal implementation.
* API update 2: add `HashMap::{get, get_async}` returning an `OccupiedEntry`.
* API update 3: add `Hash*::capacity_range`.

1.6.3

* API update 1: [#96](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/96) - add `HashIndex::{modify, modify_async}`, special thanks to [novacrazy](https://github.com/novacrazy).
* API update 2: `Hash*::default()` for any `H: BuildHasher + Default`, by [novacrazy](https://github.com/novacrazy).

1.6.2

* API update: add `HashIndex::{retain, retain_async}`.

1.6.1

* API update: add a mutable `Bag` iterator.
* Replace `compare_exchange` with `compare_exchange_weak` where spurious failures do not cost much.
* Fix an issue with `hash_map::Reserve::fmt` which printed superfluous information.

1.6.0

* API update 1: remove `ebr::Barrier::defer_incremental_execute` in favor of unwind-safety.
* API update 2: all the data structures except for `hash_map::OccupiedEntry` and `hash_map::VacantEntry` are now `UnwindSafe`.
* API update 3: export `linked_list::Entry` as `LinkedEntry`.

1.5.0

* API update: `HashMap::remove_if*` passes `&mut V` to the supplied predicate.

1.4.4

* Major `Hash*` performance boost: vectorize `Hash*` bucket lookup operations.

1.4.3

* Add `const ARRAY_LEN: usize` type parameter to `Bag`.
* Minor optimization.

1.4.2

* Optimize `TreeIndex::is_empty`.
* Update documentation.

1.4.1

* Add `hash_index::Reserve` and `HashIndex::reserve`.
* Add missing `H = RandomState` to several types.
* Add `const` to several trivial functions.

1.4.0

* **Fix a correctness issue with LLVM 16 (Rust 1.70.0)**.
* API update: `{Stack, Queue}::{peek*}` receive `FnOnce(Option<&Entry<T>>) -> R`.
* `RandomState` is now the default type parameter for `hash_*` structures.
* Remove explicit `Sync` requirements.
* Remove `'static` lifetime constraints from `Bag`, `LinkedList`, `Queue`, and `Stack`.
* Minor `Hash*` optimization.

1.3.0

* Add `HashMap::first_occupied_entry*` for more flexible mutable iteration over entries.
* Add `ebr::Arc::get_ref_with`.
* Implement `Send` for `hash_map::Entry` if `(K, V): Send`.
* `Hash*::remove*` methods may deallocate the entire hash table when they find the container empty.

1.2.0

* API update 1: `AtomicArc::update_tag_if` now receives `fetch_order`, and the closure can access the pointer value.
* API update 2: rename `hash_map::Ticket` `hash_map::Reserve`.
* `Hash*` do not allocate bucket arrays until the first write access.
* `Hash*::remove*` more aggressively shrinks the bucket array.

1.1.4 - 1.1.5

* Optimize `Hash*::is_empty`.
* Remove unnecessary lifetime constraints on `BuildHasher`.

1.1.3

* Fix [#86](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/86) completely.
* EBR garbage instances and the garbage collector instance of a thread is now deallocated immediately when the thread exits if certain conditions are met.

1.1.2

* Fix [#86](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/86).

1.1.1

* Fix a rare problem with `HashMap` and `HashSet` violating lifetime contracts on drop.

1.1.0

* Remove `'static` bounds from `HashMap`, `HashSet`, and `ebr::{Arc, AtomicArc}`.

1.0.9

* Add `HashMap::{entry, entry_async}`.

1.0.8

* More robust panic handling.
* Doc update.

1.0.7

* Minor performance optimization.
* Identified a piece of blocking code in `HashIndex::read`, and make it non-blocking.

1.0.6

* Optimize `TreeIndex` for low-entropy input patterns.

1.0.5

* Add `{HashMap, HashSet}::{any, any_async}` to emulate `Iterator::any`.
* Implement `PartialEq` for `{HashMap, HashSet, HashIndex, TreeIndex}`.
* Add `serde` support to `{HashMap, HashSet, HashIndex, TreeIndex}`.
* Remove the unnecessary `Send` bound from `TreeIndex`.

1.0.4

* Minor `Hash*` optimization.

1.0.3

* Major `TreeIndex` performance improvement.
* Add `From<ebr::Tag> for u8`.

1.0.2

* Optimize `TreeIndex`.

1.0.1

* Add `Stack`.
* API update 1: remove `Bag::clone`.
* API update 2: replace `Queue::Entry` with `<linked_list::Entry as LinkedList>`.
* Optimize `Bag`.
* Fix memory ordering in `Bag::drop`.

1.0.0

* Implement `Bag`.

## Version 0

0.12.4

* Remove `scopeguard`.

0.12.3

* Minor `ebr` optimization.

0.12.2

* `Hash*::remove*` accept `FnOnce`.

0.12.1

* `HashMap::read`, `HashIndex::read`, and `HashIndex::read_with` accept `FnOnce`.
* Proper optimization for `T: Copy` and `!needs_drop::<T>()`.

0.12.0

* More aggressive EBR garbage collection.

0.11.5

* Fix [#84](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/84) completely.
* Micro-optimization.

0.11.4

* Optimize performance for `T: Copy`.

0.11.3

* Fix [#84](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/84).
* 0.11.2 and any older versions have a serious correctness problem with Rust 1.65.0 and newer.

0.11.2

* `HashIndex` and `HashMap` cleanup entries immediately when the instance is dropped.

0.11.1

* Adjust `HashIndex` parameters to suppress latency spikes.

0.11.0

* Replace `ebr::Barrier::reclaim` with `ebr::Arc::release`.
* Rename `ebr::Arc::drop_in_place` `ebr::Arc::release_drop_in_place`.
* Implement `ebr::Barrier::defer`.
* Make `ebr::Collectible` public.

0.10.2

* Fix [#82](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/82).
* Implement `ebr::Barrier::defer_incremental_execute`.

0.10.1

* Significant `HashMap`, `HashSet`, and `HashIndex` insert performance improvement by segregating zero and non-zero memory regions.

0.9.1

* `HashMap`, `HashSet`, and `HashIndex` performance improvement.

0.9.0

* API update: `HashMap::new`, `HashIndex::new`, and `HashSet::new`.
* Add `unsafe HashIndex::update` for linearizability.

0.8.4

* Implement `ebr::Barrier::defer_execute` for deferred closure execution.
* Fix [#78](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/78).

0.8.3

* Fix `ebr::AtomicArc::{clone, get_arc}` to never return a null pointer if the `ebr::AtomicArc` is always non-null.

0.8.2

* Fix [#77](https://github.com/wvwwvwwv/scalable-concurrent-containers/issues/77).

0.8.1

* Implement `Debug` for container types.

0.8.0

* Add `ebr::suspend` which enables garbage instances in a dormant thread to be reclaimed by other threads.
* Minor `Queue` API update.
* Reduce `HashMap` memory usage.
