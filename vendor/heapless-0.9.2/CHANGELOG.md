# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.9.2] 2025-11-12

- Minor fixes to module docs.
- Make MSRV of 1.87.0 explicit.
- Implement `Default` for `CapacityError`.
- Implement `defmt::Format` for `CapacityError`.
- Implement `TryFrom` for `Deque` from array.
- Switch from `serde` to `serde_core` for enabling faster compilations.
- Implement `Zeroize` trait for all data structures with the `zeroize` feature to securely clear sensitive data from memory.
- Added `truncate` to `Deque`
- Added `retain` to `Deque`
- Added `retain_mut` to `Deque`
- Added `into_raw` to `Box`
- Added `from_raw` to `Box`
- Added `FusedIterator` to `vec::IntoIter`, `deque::IntoIter`, `index_map::IntoIter` and `linear_map::IntoIter`.
- Added `ExactSizeIterator` to `vec::IntoIter`, `deque::IntoIter`, `index_map::IntoIter` and `linear_map::IntoIter`.
- Added `DoubleEndedIterator` to `vec::IntoIter` and `deque::IntoIter`.
- Deprecate `mpmc` (see [#583](https://github.com/rust-embedded/heapless/issues/583#issuecomment-3469297720))
- Fixed initialization of the `ArcBlock<T>` and `BoxBlock<T>` types, fixing possible Undefined Behavior

## [v0.9.1] - 2025-08-19

### Added

- Added `String::insert` and `String::insert_str`.

### Added

- Added optional `embedded_io::Write` impl for `Vec`.

### Changed

- `bytes::BufMut` is now implemented on `VecInner`.
- Removed generic from `history_buf::OldestOrdered`.
- Made `LenType` opt-in.
- Minor fixes to `pool::boxed` docs.
- Add missing `Debug` derive to `vec::IntoIter`.
- Removed generic from `spsc::Consumer`, `spsc::Producer` and `spsc::Iter`.
- Added `LinearMap::entry()` API.
- Added `LinearMap::retain()`.

### Fixed

- CI now uses flags specified in `Cargo.toml` for `rustdoc` tests.
- Fixed clippy lints.
- Fixed the memory layout of the internal `UnionNode<T>` type, fixing possible Undefined Behaviour.

### Removed

- Removed invalid `bytes::Buf` implementation.
- Removed `DefaultLenType` struct.

## [v0.9.0] - 2025-04-28 [YANKED]

### Added

- Added `bytes::Buf` and `bytes::BufMut` implementations for `Vec`.
- Added `format` macro.
- Added `String::from_utf16`.
- Added `is_full`, `recent_index`, `oldest`, and `oldest_index` to `HistoryBuffer`
- Added `is_full` to `BinaryHeap`
- Added `is_full` to `IndexMap`
- Added `is_full` to `IndexSet`
- Added `is_full` to `LinearMap`
- Added infallible conversions from arrays to `Vec`.
- Added `Vec::spare_capacity_mut`.
- Added `Extend` impls for `Deque`.
- Added `Deque::make_contiguous`.
- Added `VecView`, the `!Sized` version of `Vec`.
- Added pool implementations for 64-bit architectures.
- Added `IntoIterator` implementation for `LinearMap`
- Added `Deque::{get, get_mut, get_unchecked, get_unchecked_mut}`.
- Added `serde::Serialize` and `serde::Deserialize` implementations to `HistoryBuffer`.
- Added `Vec::drain`.
- Added `String::drain`.
- Implemented `DoubleEndedIterator` for `OldestOrdered`.
- Added std `Entry` methods to indexmap `Entry`.
- Added `StringView`, the `!Sized` version of `String`.
- Added `BinaryHeapView`, the `!Sized` version of `BinaryHeap`.
- Added `MpMcQueueView`, the `!Sized` version of `MpMcQueue`.
- Added `LinearMapView`, the `!Sized` version of `LinearMap`.
- Added `HistoryBufferView`, the `!Sized` version of `HistoryBuffer`.
- Added `DequeView`, the `!Sized` version of `Deque`.
- Added `QueueView`, the `!Sized` version of `Queue`.
- Added `SortedLinkedListView`, the `!Sized` version of `SortedLinkedList`.
- Added implementation of `Borrow` and `BorrowMut` for `String` and `Vec`.
- Added `Deque::{swap, swap_unchecked, swap_remove_front, swap_remove_back}`.
- Make `String::from_utf8_unchecked` const.
- Implemented `PartialEq` and `Eq` for `Deque`.
- Added `alloc` feature to enable `alloc`-Vec interoperability.
- Added `TryFrom<alloc::vec::Vec>` impl for `Vec`.
- Added `TryFrom<Vec>` impl for `alloc::vec::Vec`.
- Added `truncate` to `IndexMap`.
- Added `get_index` and `get_index_mut` to `IndexMap`.
- Added `String::uDisplay`.
- Added `CString`.
- Added `LenT` generic to `Vec<T, N>` and `VecView<T>` to save memory when using a sane capacity value.
- Added the `index_set` module.
- Added the `index_map` module.
- Migrated `Idx` generic for `SortedLinkedList` to use the new `LenType` trait, allowing for `Idx` inference.
- Added similar `LenT` generic to `String`.
- Optimize size of zero capacity `Vec<T, 0>` to be 0 bytes

### Changed

- Updated defmt from 0.3 to 1.0.1
  - Changed the feature name from `defmt-03` to `defmt`.
- Changed the error type of these methods from `()` to `CapacityError`.
  - `String::push_str`
  - `String::push`
  - `Vec::extend_from_slice`
  - `Vec::from_slice`
  - `Vec::resize_default`
  - `Vec::resize`
- Renamed `FromUtf16Error::DecodeUtf16Error` to `FromUtf16Error::DecodeUtf16`.
- Changed `stable_deref_trait` to a platform-dependent dependency.
- Changed `SortedLinkedList::pop` return type from `Result<T, ()>` to `Option<T>` to match `std::vec::pop`.
- `Vec::capacity` is no longer a `const` function.
- Relaxed bounds on `PartialEq` for `IndexMap` from `V: Eq` to `V1: PartialEq<V2>`.
- Relaxed bounds on `PartialEq` for `LinearMap` from `V: PartialEq` to `V1: PartialEq<V2>`.
- The `FnvIndexSet` type is now inside the `index_set` module.
- The `IndexSetIter` type is now inside the `index_set` module and has been renamed to `Iter`.
- The `Bucket` type is now inside the `index_map` module.
- The `Entry` type is now inside the `index_map` module.
- The `FnvIndexMap` type is now inside the `index_map` module.
- The `IndexMapIter` type is now inside the `index_map` module and has been renamed to `Iter`.
- The `IndexMapIterMut` type is now inside the `index_map` module and has been renamed to `IterMut`.
- The `IndexMapKeys` type is now inside the `index_map` module and has been renamed to `Keys`.
- The `OccupiedEntry` type is now inside the `index_map` module.
- The `Pos` type is now inside the `index_map` module.
- The `VacantEntry` type is now inside the `index_map` module.
- The `VacantEntry` type is now inside the `index_map` module.
- The `IndexMapValues` type is now inside the `index_map` module and has been renamed to `Values`.
- The `IndexMapValuesMut` type is now inside the `index_map` module and has been renamed to `ValuesMut`.
- The `histbuf` module has been renamed to `history_buf`.
- The `HistoryBuffer` type has been renamed to `HistoryBuf`.
- The `HistoryBufferView` type has been renamed to `HistoryBufView`.
- The `OwnedHistBufStorage` type has been renamed to `OwnedHistoryBufStorage`.
- The `ViewHistBufStorage` type has been renamed to `ViewHistoryBufStorage`.
- The `MpMcQueue` type has been renamed to `Queue`.
- The `MpMcQueueView` type has been renamed to `QueueView`.
- The `MpMcQueueInner` type has been renamed to `QueueInner`.
- Remove `Q*` type aliases for `MpMcQueue`, and rename it to just `Queue`
- Changed `Queue::split` to be `const`.

### Fixed

- Fixed bug in `IndexMap::truncate` that left the map in an inconsistent state.
- Fixed compilation on `thumbv6m-none-eabi` without `portable-atomic` feature.
- Fixed clippy lints.
- Fixed `{arc,box,object}_pool!` emitting clippy lints.
- Fixed the list of implemented data structures in the crate docs, by adding `Deque`,
  `HistoryBuffer` and `SortedLinkedList` to the list.
- Fixed `MpMcQueue` with `mpmc_large` feature.
- Fix missing `Drop` for `MpMcQueue`

### Removed

- `Vec::storage_capacity` has been removed and `Vec::capacity` must be used instead.
- Removed `sorted_linked_list::Iter` and `sorted_linked_list::IterInner`.
- Removed `sorted_linked_list::FindMut` and `sorted_linked_list::FindMutInner`.
- The `Q2`, `Q4`, `Q8`, `Q16`, `Q32` and `Q64` aliases for `MpMcQueue` have been removed.
- `doc_auto_cfg` feature which was merged into `doc_cfg`. Presence of the feature led to doc
  build failures on nightly.

## [v0.8.0] - 2023-11-07

### Added

- Add `Clone` and `PartialEq` implementations to `HistoryBuffer`.
- Added an object pool API. see the `pool::object` module level doc for details
- Add `HistoryBuffer::as_slices()`
- Implemented `retain` for `IndexMap` and `IndexSet`.
- Recover `StableDeref` trait for `pool::object::Object` and `pool::boxed::Box`.
- Add polyfills for ESP32S2
- Added `String::from_utf8` and `String::from_utf8_unchecked`.

### Changed

- updated from edition 2018 to edition 2021
- [breaking-change] `IndexMap` and `IndexSet` now require that keys implement the `core::hash::Hash`
  trait instead of the `hash32::Hash` (v0.2.0) trait
- move `pool::singleton::Box` to the `pool::box` module
- renamed `pool::singleton::Pool` to `BoxPool` and moved it into the `pool::box` module
- move `pool::singleton::arc::Arc` to the `pool::arc` module
- renamed `pool::singleton::arc::Pool` to `ArcPool` and moved it into the `pool::arc` module
- [breaking-change] changed the target support of memory pool API to only support 32-bit x86 and a
  subset of ARM targets. See the module level documentation of the `pool` module for details
- relax trait requirements on `IndexMap` and `IndexSet`.
- export `IndexSet` and `IndexMap` iterator types.
- [breaking-change] export `IndexMapKeys`, `IndexMapValues` and
  `IndexMapValuesMut` iterator types.
- [breaking-change] this crate now uses `portable-atomic` v1.0 instead of `atomic-polyfill` for emulating
  CAS instructions on targets where they're not natively available.
- [breaking-change] `From<&str>` for `String` was replaced with `TryFrom<&str>` because the `From` trait must not fail.
- [breaking-change] Renamed Cargo features
  - `defmt-impl` is now `defmt-03`
  - `ufmt-impl` is now `ufmt`
  - `cas` is removed, atomic polyfilling is now opt-in via the `portable-atomic` feature.
- `Vec::as_mut_slice` is now a public method.
- `ufmt` functions are annotated with `inline`.

### Fixed

- Fixed a `dropping_references` warning in `LinearMap`.
- Fixed IndexMap entry API returning wrong slot after an insert on vacant entry. (#360)

### Removed

- [breaking-change] this crate no longer has a Minimum Supported Rust Version (MSRV) guarantee and
  should be used with the latest stable version of the Rust toolchain.

- [breaking-change] removed the `Init` and `Uninint` type states from `pool::singleton::Box`
- [breaking-change] removed the following `pool::singleton::Box` methods: `freeze`, `forget` and `init`
- [breaking-change] removed the `pool::singleton::arc::ArcInner` type
- [breaking-change] removed support for attributes from `pool!` and `arc_pool!`

## [v0.7.16] - 2022-08-09

### Added

- add more `PartialEq` implementations to `Vec` where `Vec` is the RHS

### Changed

### Fixed

- clarify in the docs that the capacity `heapless::String` is in bytes, not characters
- Fixed some broken links in the documentation.

## [v0.7.15] - 2022-07-05

### Added

- Added `Vec::insert(index, element)`
- Added `Vec::remove(index)`
- Added `Vec::retain(f)`
- Added `Vec::retain_mut(f)`

## [v0.7.14] - 2022-06-15

### Added

- Added support for AVR architecture.

### Fixed

- `IndexSet` and `IndexMap`'s `default` method now compile time checks that their capacity is a power of two.

## [v0.7.13] - 2022-05-16

### Added

- Added `into_vec` to `BinaryHeap`

## [v0.7.12] - 2022-05-12

### Added

- Added support for AVR architecture.
- Add `entry` API to `IndexMap`
- Implement `IntoIterator` trait for `Indexmap`
- Implement `FromIterator` for `String`
- Add `first` and `last` methods to `IndexMap` and `IndexSet`
- Add `pop_{front_back}_unchecked` methods to `Deque`

### Changed

- Optimize the codegen of `Vec::clone`
- `riscv32i` and `riscv32imc` targets unconditionally (e.g. `build --no-default-features`) depends on `atomic-polyfill`

### Fixed

- Inserting an item that replaces an already present item will no longer
  fail with an error

## [v0.7.11] - 2022-05-09

### Fixed

- Fixed `pool` example in docstring.
- Fixed undefined behavior in `Vec::truncate()`, `Vec::swap_remove_unchecked()`,
  and `Hole::move_to()` (internal to the binary heap implementation).
- Fixed `BinaryHeap` elements are being dropped twice

## [v0.7.10] - 2022-01-21

### Fixed

- `cargo test` can now run on non-`x86` hosts

### Added

- Added `OldestOrdered` iterator for `HistoryBuffer`

### Changed

- `atomic-polyfill` is now enabled and used for `cas` atomic emulation on `riscv` targets

## [v0.7.9] - 2021-12-16

### Fixed

- Fix `IndexMap` and `IndexSet` bounds
- Make `IndexSet::new()` a `const fn`

## [v0.7.8] - 2021-11-11

### Added

- A span of `defmt` versions is now supported (`0.2` and `0.3`)

## [v0.7.7] - 2021-09-22

### Fixed

- Fixed so `Pool` is `Sync` on ARMv6

## [v0.7.6] - 2021-09-21

### Added

- Added `ArcPool`
- Added `Debug` impl for `Deque`

### Fixed

- ZSTs in `Pool` now works correctly
- Some MIRI errors were resolved
- Allow `pool!` on thumbv6
- Fixed possible UB in `Pool` on x86

## [v0.7.5] - 2021-08-16

### Added

- Added `SortedLinkedList`
- Added `Vec::is_empty`, one does not need to go through a slice anymore

### Changed

- `Vec::pop_unchecked` is now public

## [v0.7.4] - 2021-08-06

### Added

- Implement `Default` for `MpMcQueue`, `Queue` and `HistoryBuffer`
- Implement `PartialOrd` and `Ord` for `Vec` and `String`

### Fixed

- Fixed comments in SPSC

## [v0.7.3] - 2021-07-01

### Added

- Added `Deque`

### Changed

- `Box::freeze` is deprecated due to possibility of undefined behavior.

## [v0.7.2] - 2021-06-30

### Added

- Added new `Vec::into_array` method
- Added const-asserts to all data structures

## [v0.7.1] - 2021-05-23

### Changed

- MPMC is now more generic

### Added

- `defmt` for `Vec` and `String`

## [v0.7.0] - 2021-04-23

### Changed

- [breaking-change] Converted all data structures to use the `const generics` MVP
- [breaking-change] `HistoryBuffer` is now working with const constructors and non-`Copy` data
- [breaking-change] `HistoryBuffer::as_slice` and others now only return initialized values
- Added missing `Deref`, `AsRef` and `Debug` for `HistoryBuffer`
- [breaking-change] `MultiCore`/`SingleCore` and `Uxx` is now removed from `spsc::Queue`
- [breaking-change] `spsc::Queue` is now `usize` only
- [breaking-change] `spsc::Queue` now sacrifices one element for correctness (see issue #207), i.e. it creates an `N - 1` sized queue instead of the old that generated an size `N` queue
- [breaking-change] `String` has had `utf8` related methods removed as this can be done via `str`
- [breaking-change] No data structures implement `AsSlice` traits any more, now using `AsRef` and `AsMut` as they work with any size of array now

### Fixed

- `Pool` and `MPMC` now works on `thumbv6m`
- `IndexMap::new()` is now a `const-fn`

## [v0.6.1] - 2021-03-02

### Fixed

- Security issue.

## [v0.6.0] - 2021-02-02

### Changed

- [breaking-change] The version of the `generic-array` dependency has been
  bumped to v0.14.2.

## [v0.5.6] - 2020-09-18

### Added

- Added `as_mut_vec` for `String`
- Added `set_len` for `Vec`
- Performance improvements in `histbuf`

### Fixed

- `Producer` was made `Send` for single core applications

## [v0.5.5] - 2020-05-04

### Added

- Added `HistoryBuffer`
- Added extra methods to `Vec`: `from_slice`, `starts_with`, `ends_with`
- Optional `ufmt` support for `String` and `Vec`
- Added `pool` support for bare-metal `armebv7r-` targets
- Added Sync to `pool` for `x86`

## [v0.5.4] - 2020-04-06

### Added

- Added `StableDeref` implementation for `pool::Box` and `pool::singleton::Box`.

## [v0.5.3] - 2020-01-27

### Added

- Extend the ARMv7-A `Pool` support to the bare-metal `armv7a-` targets.

## [v0.5.2] - 2020-01-15

### Fixed

- Fixed incorrect overflow behavior in computation of capacities
- Fixed edge case in `mpmc::Queue::dqueue` that led to an infinite loop
- IndexMap and LinerMap are now deserialized as maps, rather than as sequences
- Fixed compilation of this crates on built-in targets that don't have CAS instructions

### Changed

- `spsc::Queue` iterators now implement the double ended iterator trait

### Added

- opt-out `cas` feature to disable parts of the API that use CAS instructions.
  Useful if using a custom (i.e. not built-in) rustc target that does not have CAS
  instructions.

- singleton `Pool` support on ARMv7-A devices

## [v0.5.1] - 2019-08-29

### Added

- Added armv8 support
- Added `Queue::peek`
- Added `BinaryHeap::peek_mut`

## [v0.5.0] - 2019-07-12

### Added

- `Pool` now implements the `Sync` trait when targeting ARMv7-R.

- Most data structures can now be constructed in "const context" (e.g. `static
[mut]` variables) using a newtype in `heapless::i`.

- `Pool` has gained a `grow_exact` method to more efficiently use statically
  allocated memory.

- The `pool!` macro now accepts attributes.

- `mpmc::Q*` a family of fixed capacity multiple-producer multiple-consumer
  lock-free queues.

### Changed

- [breaking-change] `binary_heap::Kind` is now a sealed trait.

### Removed

- [breaking-change] The "smaller-atomics" feature has been removed. It is now
  always enabled.

- [breaking-change] The "min-const-fn" feature has been removed. It is now
  always enabled.

- [breaking-change] The MSRV has been bumped to Rust 1.36.0.

- [breaking-change] The version of the `generic-array` dependency has been
  bumped to v0.13.0.

## [v0.4.4] - 2019-05-02

### Added

- Implemented `PartialEq`, `PartialOrd`, `Eq`, `Ord` and `Hash` for `pool::Box`
  and `pool::singleton::Box`.

### Fixed

- Fixed UB in our internal, stable re-implementation of `core::mem::MaybeUninit`
  that occurred when using some of our data structures with types that implement
  `Drop`.

## [v0.4.3] - 2019-04-22

### Added

- Added a memory pool that's lock-free and interrupt-safe on the ARMv7-M
  architecture.

- `IndexMap` have gained `Eq` and `PartialEq` implementations.

## [v0.4.2] - 2019-02-12

### Added

- All containers now implement `Clone`

- `spsc::Queue` now implements `Debug`, `Hash`, `PartialEq` and `Eq`

- `LinearMap` now implements `Debug`, `FromIterator`, `IntoIter`, `PartialEq`,
  `Eq` and `Default`

- `BinaryHeap` now implements `Debug` and `Default`

- `String` now implements `FromStr`, `Hash`, `From<uxx>` and `Default`

- `Vec` now implements `Hash` and `Default`

- A "serde" Cargo feature that when enabled adds a `serde::Serialize` and
  `serde::Deserialize` implementations to each collection.

## [v0.4.1] - 2018-12-16

### Changed

- Add a new type parameter to `spsc::Queue` that indicates whether the queue is
  only single-core safe, or multi-core safe. By default the queue is multi-core
  safe; this preserves the current semantics. New `unsafe` constructors have
  been added to create the single-core variant.

## [v0.4.0] - 2018-10-19

### Changed

- [breaking-change] All Cargo features are disabled by default. This crate now
  compiles on stable by default.

- [breaking-change] RingBuffer has been renamed to spsc::Queue. The ring_buffer
  module has been renamed to spsc.

- [breaking-change] The bounds on spsc::Queue have changed.

### Removed

- [breaking-change] The sealed `Uxx` trait has been removed from the public API.

## [v0.3.7] - 2018-08-19

### Added

- Implemented `IntoIterator` and `FromIterator` for `Vec`
- `ready` methods to `ring_buffer::{Consumer,Producer}`
- An opt-out "const-fn" Cargo feature that turns `const` functions into normal functions when
  disabled.
- An opt-out "smaller-atomics" Cargo feature that removes the ability to shrink the size of
  `RingBuffer` when disabled.

### Changed

- This crate now compiles on stable when both the "const-fn" and "smaller-atomics" features are
  disabled.

### Fixed

- The `RingBuffer.len` function
- Compilation on recent nightlies

## [v0.3.6] - 2018-05-04

### Fixed

- The capacity of `RingBuffer`. It should be the requested capacity plus not twice that plus one.

## [v0.3.5] - 2018-05-03

### Added

- `RingBuffer.enqueue_unchecked` an unchecked version of `RingBuffer.enqueue`

## [v0.3.4] - 2018-04-28

### Added

- `BinaryHeap.pop_unchecked` an unchecked version of `BinaryHeap.pop`

## [v0.3.3] - 2018-04-28

### Added

- `BinaryHeap.push_unchecked` an unchecked version of `BinaryHeap.push`

## [v0.3.2] - 2018-04-27

### Added

- A re-export of `generic_array::ArrayLength`, for convenience.

## [v0.3.1] - 2018-04-23

### Added

- Fixed capacity implementations of `IndexMap` and `IndexSet`.
- A `Extend` implementation to `Vec`
- More `PartialEq` implementations to `Vec`

## [v0.3.0] - 2018-04-22

### Changed

- [breaking-change] The capacity of all data structures must now be specified using type level
  integers (cf. `typenum`). See documentation for details.

- [breaking-change] `BufferFullError` has been removed in favor of (a) returning ownership of the
  item that couldn't be added to the collection (cf. `Vec.push`), or (b) returning the unit type
  when the argument was not consumed (cf. `Vec.extend_from_slice`).

## [v0.2.7] - 2018-04-20

### Added

- Unchecked methods to dequeue and enqueue items into a `RingBuffer` via the `Consumer` and
  `Producer` end points.

### Changed

- `RingBuffer` now has a generic index type, which default to `usize` for backward compatibility.
  Changing the index type to `u8` or `u16` reduces the footprint of the `RingBuffer` but limits its
  maximum capacity (254 and 65534, respectively).

## [v0.2.6] - 2018-04-18

### Added

- A `BinaryHeap` implementation. `BinaryHeap` is a priority queue implemented with a binary heap.

## [v0.2.5] - 2018-04-13

### Fixed

- Dereferencing `heapless::Vec` no longer incurs in a bounds check.

## [v0.2.4] - 2018-03-12

### Fixed

- `LinerMap::new` is now a const fn

## [v0.2.3] - 2018-03-11

### Added

- A `swap_remove` method to `Vec`
- A `LinearMap` implementation. `LinearMap` is a map / dict backed by an array and that performs
  lookups via linear search.

## [v0.2.2] - 2018-03-01

### Added

- Fixed size version of `std::String`

## [v0.2.1] - 2017-12-21

### Added

- `Vec` now implements both `fmt::Debug`, `PartialEq` and `Eq`.

- `resize` and `resize_default` methods to `Vec`.

## [v0.2.0] - 2017-11-22

### Added

- A single producer single consumer mode to `RingBuffer`.

- A `truncate` method to `Vec`.

### Changed

- [breaking-change] Both `Vec::new` and `RingBuffer::new` no longer require an initial value. The
  signature of `new` is now `const fn() -> Self`.

- [breaking-change] The error type of all operations that may fail has changed from `()` to
  `BufferFullError`.

- Both `RingBuffer` and `Vec` now support arrays of _any_ size for their backup storage.

## [v0.1.0] - 2017-04-27

- Initial release

[Unreleased]: https://github.com/rust-embedded/heapless/compare/v0.9.2...HEAD
[v0.9.2]: https://github.com/rust-embedded/heapless/compare/v0.9.1...v0.9.2
[v0.9.1]: https://github.com/rust-embedded/heapless/compare/v0.9.0...v0.9.1
[v0.9.0]: https://github.com/rust-embedded/heapless/compare/v0.8.0...v0.9.0
[v0.8.0]: https://github.com/rust-embedded/heapless/compare/v0.7.16...v0.8.0
[v0.7.16]: https://github.com/rust-embedded/heapless/compare/v0.7.15...v0.7.16
[v0.7.15]: https://github.com/rust-embedded/heapless/compare/v0.7.14...v0.7.15
[v0.7.14]: https://github.com/rust-embedded/heapless/compare/v0.7.13...v0.7.14
[v0.7.13]: https://github.com/rust-embedded/heapless/compare/v0.7.12...v0.7.13
[v0.7.12]: https://github.com/rust-embedded/heapless/compare/v0.7.11...v0.7.12
[v0.7.11]: https://github.com/rust-embedded/heapless/compare/v0.7.10...v0.7.11
[v0.7.10]: https://github.com/rust-embedded/heapless/compare/v0.7.9...v0.7.10
[v0.7.9]: https://github.com/rust-embedded/heapless/compare/v0.7.8...v0.7.9
[v0.7.8]: https://github.com/rust-embedded/heapless/compare/v0.7.7...v0.7.8
[v0.7.7]: https://github.com/rust-embedded/heapless/compare/v0.7.6...v0.7.7
[v0.7.6]: https://github.com/rust-embedded/heapless/compare/v0.7.5...v0.7.6
[v0.7.5]: https://github.com/rust-embedded/heapless/compare/v0.7.4...v0.7.5
[v0.7.4]: https://github.com/rust-embedded/heapless/compare/v0.7.3...v0.7.4
[v0.7.3]: https://github.com/rust-embedded/heapless/compare/v0.7.2...v0.7.3
[v0.7.2]: https://github.com/rust-embedded/heapless/compare/v0.7.1...v0.7.2
[v0.7.1]: https://github.com/rust-embedded/heapless/compare/v0.7.0...v0.7.1
[v0.7.0]: https://github.com/rust-embedded/heapless/compare/v0.6.1...v0.7.0
[v0.6.1]: https://github.com/rust-embedded/heapless/compare/v0.6.0...v0.6.1
[v0.6.0]: https://github.com/rust-embedded/heapless/compare/v0.5.5...v0.6.0
[v0.5.5]: https://github.com/rust-embedded/heapless/compare/v0.5.4...v0.5.5
[v0.5.4]: https://github.com/rust-embedded/heapless/compare/v0.5.3...v0.5.4
[v0.5.3]: https://github.com/rust-embedded/heapless/compare/v0.5.2...v0.5.3
[v0.5.2]: https://github.com/rust-embedded/heapless/compare/v0.5.1...v0.5.2
[v0.5.1]: https://github.com/rust-embedded/heapless/compare/v0.5.0...v0.5.1
[v0.5.0]: https://github.com/rust-embedded/heapless/compare/v0.4.4...v0.5.0
[v0.4.4]: https://github.com/rust-embedded/heapless/compare/v0.4.3...v0.4.4
[v0.4.3]: https://github.com/rust-embedded/heapless/compare/v0.4.2...v0.4.3
[v0.4.2]: https://github.com/rust-embedded/heapless/compare/v0.4.1...v0.4.2
[v0.4.1]: https://github.com/rust-embedded/heapless/compare/v0.4.0...v0.4.1
[v0.4.0]: https://github.com/rust-embedded/heapless/compare/v0.3.7...v0.4.0
[v0.3.7]: https://github.com/rust-embedded/heapless/compare/v0.3.6...v0.3.7
[v0.3.6]: https://github.com/rust-embedded/heapless/compare/v0.3.5...v0.3.6
[v0.3.5]: https://github.com/rust-embedded/heapless/compare/v0.3.4...v0.3.5
[v0.3.4]: https://github.com/rust-embedded/heapless/compare/v0.3.3...v0.3.4
[v0.3.3]: https://github.com/rust-embedded/heapless/compare/v0.3.2...v0.3.3
[v0.3.2]: https://github.com/rust-embedded/heapless/compare/v0.3.1...v0.3.2
[v0.3.1]: https://github.com/rust-embedded/heapless/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/rust-embedded/heapless/compare/v0.2.7...v0.3.0
[v0.2.7]: https://github.com/rust-embedded/heapless/compare/v0.2.6...v0.2.7
[v0.2.6]: https://github.com/rust-embedded/heapless/compare/v0.2.5...v0.2.6
[v0.2.5]: https://github.com/rust-embedded/heapless/compare/v0.2.4...v0.2.5
[v0.2.4]: https://github.com/rust-embedded/heapless/compare/v0.2.3...v0.2.4
[v0.2.3]: https://github.com/rust-embedded/heapless/compare/v0.2.2...v0.2.3
[v0.2.2]: https://github.com/rust-embedded/heapless/compare/v0.2.1...v0.2.2
[v0.2.1]: https://github.com/rust-embedded/heapless/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/rust-embedded/heapless/compare/v0.1.0...v0.2.0
