# Release rayon 1.11.0 / rayon-core 1.13.0 (2025-08-12)

- The minimum supported `rustc` is now 1.80.
- `iter::repeatn` has been renamed to `iter::repeat_n` to match the name
  stabilized in the standard library. The old name still exists as a deprecated
  function for compatibility.
- Fixed a bug in `in_place_scope` when the default global registry uses the
  current thread, like on WebAssembly without threading support.
- `binary_heap::Iter` no longer requires a temporary allocation.
- Relaxed trait bounds on many of the public structs.
- Implemented `IntoParallelIterator for Box<[T]>` and its references.
- Implemented `FromParallelIterator<_> for Box<str>` via `String`.

# Release rayon 1.10.0 (2024-03-23)

- The new methods `ParallelSlice::par_chunk_by` and
  `ParallelSliceMut::par_chunk_by_mut` work like the slice methods `chunk_by`
  and `chunk_by_mut` added in Rust 1.77.

# Release rayon 1.9.0 (2024-02-27)

- The new methods `IndexedParallelIterator::by_exponential_blocks` and
  `by_uniform_blocks` allow processing items in smaller groups at a time.
- The new `iter::walk_tree`, `walk_tree_prefix`, and `walk_tree_postfix`
  functions enable custom parallel iteration over tree-like structures.
- The new method `ParallelIterator::collect_vec_list` returns items as a linked
  list of vectors, which is an efficient mode of parallel collection used by
  many of the internal implementations of `collect`.
- The new methods `ParallelSliceMut::par_split_inclusive_mut`,
  `ParallelSlice::par_split_inclusive`, and
  `ParallelString::par_split_inclusive` all work like a normal split but
  keeping the separator as part of the left slice.
- The new `ParallelString::par_split_ascii_whitespace` splits only on ASCII
  whitespace, which is faster than including Unicode multi-byte whitespace.
- `OsString` now implements `FromParallelIterator<_>` and `ParallelExtend<_>`
  for a few item types similar to the standard `FromIterator` and `Extend`.
- The internal `Pattern` trait for string methods is now implemented for
  `[char; N]` and `&[char; N]`, matching any of the given characters.

# Release rayon 1.8.1 / rayon-core 1.12.1 (2024-01-17)

- The new `"web_spin_lock"` crate feature makes mutexes spin on the main
  browser thread in WebAssembly, rather than suffer an error about forbidden
  `atomics.wait` if they were to block in that context. Thanks @RReverser!

# Release rayon 1.8.0 / rayon-core 1.12.0 (2023-09-20)

- The minimum supported `rustc` is now 1.63.
- Added `ThreadPoolBuilder::use_current_thread` to use the builder thread as
  part of the new thread pool. That thread does not run the pool's main loop,
  but it may participate in work-stealing if it yields to rayon in some way.
- Implemented `FromParallelIterator<T>` for `Box<[T]>`, `Rc<[T]>`, and
  `Arc<[T]>`, as well as `FromParallelIterator<Box<str>>` and
  `ParallelExtend<Box<str>>` for `String`.
- `ThreadPoolBuilder::build_scoped` now uses `std::thread::scope`.
- The default number of threads is now determined using
  `std::thread::available_parallelism` instead of the `num_cpus` crate.
- The internal logging facility has been removed, reducing bloat for all users.
- Many smaller performance tweaks and documentation updates.

# Release rayon 1.7.0 / rayon-core 1.11.0 (2023-03-03)

- The minimum supported `rustc` is now 1.59.
- Added a fallback when threading is unsupported.
- The new `ParallelIterator::take_any` and `skip_any` methods work like
  unordered `IndexedParallelIterator::take` and `skip`, counting items in
  whatever order they are visited in parallel.
- The new `ParallelIterator::take_any_while` and `skip_any_while` methods work
  like unordered `Iterator::take_while` and `skip_while`, which previously had
  no parallel equivalent. The "while" condition may be satisfied from anywhere
  in the parallel iterator, affecting all future items regardless of position.
- The new `yield_now` and `yield_local` functions will cooperatively yield
  execution to Rayon, either trying to execute pending work from the entire
  pool or from just the local deques of the current thread, respectively.

# Release rayon-core 1.10.2 (2023-01-22)

- Fixed miri-reported UB for SharedReadOnly tags protected by a call.

# Release rayon 1.6.1 (2022-12-09)

- Simplified `par_bridge` to only pull one item at a time from the iterator,
  without batching. Threads that are waiting for iterator items will now block
  appropriately rather than spinning CPU. (Thanks @njaard!)
- Added protection against recursion in `par_bridge`, so iterators that also
  invoke rayon will not cause mutex recursion deadlocks.

# Release rayon-core 1.10.1 (2022-11-18)

- Fixed a race condition with threads going to sleep while a broadcast starts.

# Release rayon 1.6.0 / rayon-core 1.10.0 (2022-11-18)

- The minimum supported `rustc` is now 1.56.
- The new `IndexedParallelIterator::fold_chunks` and `fold_chunks_with` methods
  work like `ParallelIterator::fold` and `fold_with` with fixed-size chunks of
  items. This may be useful for predictable batching performance, without the
  allocation overhead of `IndexedParallelIterator::chunks`.
- New "broadcast" methods run a given function on all threads in the pool.
  These run at a sort of reduced priority after each thread has exhausted their
  local work queue, but before they attempt work-stealing from other threads.
  - The global `broadcast` function and `ThreadPool::broadcast` method will
    block until completion, returning a `Vec` of all return values.
  - The global `spawn_broadcast` function and methods on `ThreadPool`, `Scope`,
    and `ScopeFifo` will run detached, without blocking the current thread.
- Panicking methods now use `#[track_caller]` to report the caller's location.
- Fixed a truncated length in `vec::Drain` when given an empty range.

## Contributors

Thanks to all of the contributors for this release!

- @cuviper
- @idanmuze
- @JoeyBF
- @JustForFun88
- @kianmeng
- @kornelski
- @ritchie46
- @ryanrussell
- @steffahn
- @TheIronBorn
- @willcrozi

# Release rayon 1.5.3 (2022-05-13)

- The new `ParallelSliceMut::par_sort_by_cached_key` is a stable sort that caches
  the keys for each item -- a parallel version of `slice::sort_by_cached_key`.

# Release rayon-core 1.9.3 (2022-05-13)

- Fixed a use-after-free race in job notification.

# Release rayon 1.5.2 / rayon-core 1.9.2 (2022-04-13)

- The new `ParallelSlice::par_rchunks()` and `par_rchunks_exact()` iterate
  slice chunks in reverse, aligned the against the end of the slice if the
  length is not a perfect multiple of the chunk size. The new
  `ParallelSliceMut::par_rchunks_mut()` and `par_rchunks_exact_mut()` are the
  same for mutable slices.
- The `ParallelIterator::try_*` methods now support `std::ops::ControlFlow` and
  `std::task::Poll` items, mirroring the unstable `Try` implementations in the
  standard library.
- The `ParallelString` pattern-based methods now support `&[char]` patterns,
  which match when any character in that slice is found in the string.
- A soft limit is now enforced on the number of threads allowed in a single
  thread pool, respecting internal bit limits that already existed. The current
  maximum is publicly available from the new function `max_num_threads()`.
- Fixed several Stacked Borrow and provenance issues found by `cargo miri`.

## Contributors

Thanks to all of the contributors for this release!

- @atouchet
- @bluss
- @cuviper
- @fzyzcjy
- @nyanzebra
- @paolobarbolini
- @RReverser
- @saethlin

# Release rayon 1.5.1 / rayon-core 1.9.1 (2021-05-18)

- The new `in_place_scope` and `in_place_scope_fifo` are variations of `scope`
  and `scope_fifo`, running the initial non-`Send` callback directly on the
  current thread, rather than moving execution to the thread pool.
- With Rust 1.51 or later, arrays now implement `IntoParallelIterator`.
- New implementations of `FromParallelIterator` make it possible to `collect`
  complicated nestings of items.
  - `FromParallelIterator<(A, B)> for (FromA, FromB)` works like `unzip`.
  - `FromParallelIterator<Either<L, R>> for (A, B)` works like `partition_map`.
- Type inference now works better with parallel `Range` and `RangeInclusive`.
- The implementation of `FromParallelIterator` and `ParallelExtend` for
  `Vec<T>` now uses `MaybeUninit<T>` internally to avoid creating any
  references to uninitialized data.
- `ParallelBridge` fixed a bug with threads missing available work.

## Contributors

Thanks to all of the contributors for this release!

- @atouchet
- @cuviper
- @Hywan
- @iRaiko
- @Qwaz
- @rocallahan

# Release rayon 1.5.0 / rayon-core 1.9.0 (2020-10-21)

- Update crossbeam dependencies.
- The minimum supported `rustc` is now 1.36.

## Contributors

Thanks to all of the contributors for this release!

- @cuviper
- @mbrubeck
- @mrksu

# Release rayon 1.4.1 (2020-09-29)

- The new `flat_map_iter` and `flatten_iter` methods can be used to flatten
  sequential iterators, which may perform better in cases that don't need the
  nested parallelism of `flat_map` and `flatten`.
- The new `par_drain` method is a parallel version of the standard `drain` for
  collections, removing items while keeping the original capacity. Collections
  that implement this through `ParallelDrainRange` support draining items from
  arbitrary index ranges, while `ParallelDrainFull` always drains everything.
- The new `positions` method finds all items that match the given predicate and
  returns their indices in a new iterator.

# Release rayon-core 1.8.1 (2020-09-17)

- Fixed an overflow panic on high-contention workloads, for a counter that was
  meant to simply wrap. This panic only occurred with debug assertions enabled,
  and was much more likely on 32-bit targets.

# Release rayon 1.4.0 / rayon-core 1.8.0 (2020-08-24)

- Implemented a new thread scheduler, [RFC 5], which uses targeted wakeups for
  new work and for notifications of completed stolen work, reducing wasteful
  CPU usage in idle threads.
- Implemented `IntoParallelIterator for Range<char>` and `RangeInclusive<char>`
  with the same iteration semantics as Rust 1.45.
- Relaxed the lifetime requirements of the initial `scope` closure.

[RFC 5]: https://github.com/rayon-rs/rfcs/pull/5

## Contributors

Thanks to all of the contributors for this release!

- @CAD97
- @cuviper
- @kmaork
- @nikomatsakis
- @SuperFluffy


# Release rayon 1.3.1 / rayon-core 1.7.1 (2020-06-15)

- Fixed a use-after-free race in calls blocked between two rayon thread pools.
- Collecting to an indexed `Vec` now drops any partial writes while unwinding,
  rather than just leaking them. If dropping also panics, Rust will abort.
  - Note: the old leaking behavior is considered _safe_, just not ideal.
- The new `IndexedParallelIterator::step_by()` adapts an iterator to step
  through items by the given count, like `Iterator::step_by()`.
- The new `ParallelSlice::par_chunks_exact()` and mutable equivalent
  `ParallelSliceMut::par_chunks_exact_mut()` ensure that the chunks always have
  the exact length requested, leaving any remainder separate, like the slice
  methods `chunks_exact()` and `chunks_exact_mut()`.

## Contributors

Thanks to all of the contributors for this release!

- @adrian5
- @bluss
- @cuviper
- @FlyingCanoe
- @GuillaumeGomez
- @matthiasbeyer
- @picoHz
- @zesterer


# Release rayon 1.3.0 / rayon-core 1.7.0 (2019-12-21)

- Tuples up to length 12 now implement `IntoParallelIterator`, creating a
  `MultiZip` iterator that produces items as similarly-shaped tuples.
- The `--cfg=rayon_unstable` supporting code for `rayon-futures` is removed.
- The minimum supported `rustc` is now 1.31.

## Contributors

Thanks to all of the contributors for this release!

- @cuviper
- @c410-f3r
- @silwol


# Release rayon-futures 0.1.1 (2019-12-21)

- `Send` bounds have been added for the `Item` and `Error` associated types on
  all generic `F: Future` interfaces. While technically a breaking change, this
  is a soundness fix, so we are not increasing the semantic version for this.
- This crate is now deprecated, and the `--cfg=rayon_unstable` supporting code
  will be removed in `rayon-core 1.7.0`. This only supported the now-obsolete
  `Future` from `futures 0.1`, while support for `std::future::Future` is
  expected to come directly in `rayon-core` -- although that is not ready yet.

## Contributors

Thanks to all of the contributors for this release!

- @cuviper
- @kornelski
- @jClaireCodesStuff
- @jwass
- @seanchen1991


# Release rayon 1.2.1 / rayon-core 1.6.1 (2019-11-20)

- Update crossbeam dependencies.
- Add top-level doc links for the iterator traits.
- Document that the iterator traits are not object safe.

## Contributors

Thanks to all of the contributors for this release!

- @cuviper
- @dnaka91
- @matklad
- @nikomatsakis
- @Qqwy
- @vorner


# Release rayon 1.2.0 / rayon-core 1.6.0 (2019-08-30)

- The new `ParallelIterator::copied()` converts an iterator of references into
  copied values, like `Iterator::copied()`.
- `ParallelExtend` is now implemented for the unit `()`.
- Internal updates were made to improve test determinism, reduce closure type
  sizes, reduce task allocations, and update dependencies.
- The minimum supported `rustc` is now 1.28.

## Contributors

Thanks to all of the contributors for this release!

- @Aaron1011
- @cuviper
- @ralfbiedert


# Release rayon 1.1.0 / rayon-core 1.5.0 (2019-06-12)

- FIFO spawns are now supported using the new `spawn_fifo()` and `scope_fifo()`
  global functions, and their corresponding `ThreadPool` methods.
  - Normally when tasks are queued on a thread, the most recent is processed
    first (LIFO) while other threads will steal the oldest (FIFO). With FIFO
    spawns, those tasks are processed locally in FIFO order too.
  - Regular spawns and other tasks like `join` are not affected.
  - The `breadth_first` configuration flag, which globally approximated this
    effect, is now deprecated.
  - For more design details, please see [RFC 1].
- `ThreadPoolBuilder` can now take a custom `spawn_handler` to control how
  threads will be created in the pool.
  - `ThreadPoolBuilder::build_scoped()` uses this to create a scoped thread
    pool, where the threads are able to use non-static data.
  - This may also be used to support threading in exotic environments, like
    WebAssembly, which don't support the normal `std::thread`.
- `ParallelIterator` has 3 new methods: `find_map_any()`, `find_map_first()`,
  and `find_map_last()`, like `Iterator::find_map()` with ordering constraints.
- The new `ParallelIterator::panic_fuse()` makes a parallel iterator halt as soon
  as possible if any of its threads panic. Otherwise, the panic state is not
  usually noticed until the iterator joins its parallel tasks back together.
- `IntoParallelIterator` is now implemented for integral `RangeInclusive`.
- Several internal `Folder`s now have optimized `consume_iter` implementations.
- `rayon_core::current_thread_index()` is now re-exported in `rayon`.
- The minimum `rustc` is now 1.26, following the update policy defined in [RFC 3].

## Contributors

Thanks to all of the contributors for this release!

- @cuviper
- @didroe
- @GuillaumeGomez
- @huonw
- @janriemer
- @kornelski
- @nikomatsakis
- @seanchen1991
- @yegeun542

[RFC 1]: https://github.com/rayon-rs/rfcs/blob/main/accepted/rfc0001-scope-scheduling.md
[RFC 3]: https://github.com/rayon-rs/rfcs/blob/main/accepted/rfc0003-minimum-rustc.md


# Release rayon 1.0.3 (2018-11-02)

- `ParallelExtend` is now implemented for tuple pairs, enabling nested
  `unzip()` and `partition_map()` operations.  For instance, `(A, (B, C))`
  items can be unzipped into `(Vec<A>, (Vec<B>, Vec<C>))`.
  - `ParallelExtend<(A, B)>` works like `unzip()`.
  - `ParallelExtend<Either<A, B>>` works like `partition_map()`.
- `ParallelIterator` now has a method `map_init()` which calls an `init`
  function for a value to pair with items, like `map_with()` but dynamically
  constructed.  That value type has no constraints, not even `Send` or `Sync`.
  - The new `for_each_init()` is a variant of this for simple iteration.
  - The new `try_for_each_init()` is a variant for fallible iteration.

## Contributors

Thanks to all of the contributors for this release!

- @cuviper
- @dan-zheng
- @dholbert
- @ignatenkobrain
- @mdonoughe


# Release rayon 1.0.2 / rayon-core 1.4.1 (2018-07-17)

- The `ParallelBridge` trait with method `par_bridge()` makes it possible to
  use any `Send`able `Iterator` in parallel!
  - This trait has been added to `rayon::prelude`.
  - It automatically implements internal synchronization and queueing to
    spread the `Item`s across the thread pool.  Iteration order is not
    preserved by this adaptor.
  - "Native" Rayon iterators like `par_iter()` should still be preferred when
    possible for better efficiency.
- `ParallelString` now has additional methods for parity with `std` string
  iterators: `par_char_indices()`, `par_bytes()`, `par_encode_utf16()`,
  `par_matches()`, and `par_match_indices()`.
- `ParallelIterator` now has fallible methods `try_fold()`, `try_reduce()`,
  and `try_for_each`, plus `*_with()` variants of each, for automatically
  short-circuiting iterators on `None` or `Err` values.  These are inspired by
  `Iterator::try_fold()` and `try_for_each()` that were stabilized in Rust 1.27.
- `Range<i128>` and `Range<u128>` are now supported with Rust 1.26 and later.
- Small improvements have been made to the documentation.
- `rayon-core` now only depends on `rand` for testing.
- Rayon tests now work on stable Rust.

## Contributors

Thanks to all of the contributors for this release!

- @AndyGauge
- @cuviper
- @ignatenkobrain
- @LukasKalbertodt
- @MajorBreakfast
- @nikomatsakis
- @paulkernfeld
- @QuietMisdreavus


# Release rayon 1.0.1 (2018-03-16)

- Added more documentation for `rayon::iter::split()`.
- Corrected links and typos in documentation.

## Contributors

Thanks to all of the contributors for this release!

- @cuviper
- @HadrienG2
- @matthiasbeyer
- @nikomatsakis


# Release rayon 1.0.0 / rayon-core 1.4.0 (2018-02-15)

- `ParallelIterator` added the `update` method which applies a function to
  mutable references, inspired by `itertools`.
- `IndexedParallelIterator` added the `chunks` method which yields vectors of
  consecutive items from the base iterator, inspired by `itertools`.
- `String` now implements `FromParallelIterator<Cow<str>>` and
  `ParallelExtend<Cow<str>>`, inspired by `std`.
- `()` now implements `FromParallelIterator<()>`, inspired by `std`.
- The new `ThreadPoolBuilder` replaces and deprecates `Configuration`.
  - Errors from initialization now have the concrete `ThreadPoolBuildError`
    type, rather than `Box<Error>`, and this type implements `Send` and `Sync`.
  - `ThreadPool::new` is deprecated in favor of `ThreadPoolBuilder::build`.
  - `initialize` is deprecated in favor of `ThreadPoolBuilder::build_global`.
- Examples have been added to most of the parallel iterator methods.
- A lot of the documentation has been reorganized and extended.

## Breaking changes

- Rayon now requires rustc 1.13 or greater.
- `IndexedParallelIterator::len` and `ParallelIterator::opt_len` now operate on
  `&self` instead of `&mut self`.
- `IndexedParallelIterator::collect_into` is now `collect_into_vec`.
- `IndexedParallelIterator::unzip_into` is now `unzip_into_vecs`.
- Rayon no longer exports the deprecated `Configuration` and `initialize` from
  rayon-core.

## Contributors

Thanks to all of the contributors for this release!

- @Bilkow
- @cuviper
- @Enet4
- @ignatenkobrain
- @iwillspeak
- @jeehoonkang
- @jwass
- @Kerollmops
- @KodrAus
- @kornelski
- @MaloJaffre
- @nikomatsakis
- @obv-mikhail
- @oddg
- @phimuemue
- @stjepang
- @tmccombs
- bors[bot]


# Release rayon 0.9.0 / rayon-core 1.3.0 / rayon-futures 0.1.0 (2017-11-09)

- `Configuration` now has a `build` method.
- `ParallelIterator` added `flatten` and `intersperse`, both inspired by
  itertools.
- `IndexedParallelIterator` added `interleave`, `interleave_shortest`, and
  `zip_eq`, all inspired by itertools.
- The new functions `iter::empty` and `once` create parallel iterators of
  exactly zero or one item, like their `std` counterparts.
- The new functions `iter::repeat` and `repeatn` create parallel iterators
  repeating an item indefinitely or `n` times, respectively.
- The new function `join_context` works like `join`, with an added `FnContext`
  parameter that indicates whether the job was stolen.
- `Either` (used by `ParallelIterator::partition_map`) is now re-exported from
  the `either` crate, instead of defining our own type.
  - `Either` also now implements `ParallelIterator`, `IndexedParallelIterator`,
    and `ParallelExtend` when both of its `Left` and `Right` types do.
- All public types now implement `Debug`.
- Many of the parallel iterators now implement `Clone` where possible.
- Much of the documentation has been extended. (but still could use more help!)
- All rayon crates have improved metadata.
- Rayon was evaluated in the Libz Blitz, leading to many of these improvements.
- Rayon pull requests are now guarded by bors-ng.

## Futures

The `spawn_future()` method has been refactored into its own `rayon-futures`
crate, now through a `ScopeFutureExt` trait for `ThreadPool` and `Scope`.  The
supporting `rayon-core` APIs are still gated by `--cfg rayon_unstable`.

## Breaking changes

- Two breaking changes have been made to `rayon-core`, but since they're fixing
  soundness bugs, we are considering these _minor_ changes for semver.
  - `Scope::spawn` now requires `Send` for the closure.
  - `ThreadPool::install` now requires `Send` for the return value.
- The `iter::internal` module has been renamed to `iter::plumbing`, to hopefully
  indicate that while these are low-level details, they're not really internal
  or private to rayon.  The contents of that module are needed for third-parties
  to implement new parallel iterators, and we'll treat them with normal semver
  stability guarantees.
- The function `rayon::iter::split` is no longer re-exported as `rayon::split`.

## Contributors

Thanks to all of the contributors for this release!

- @AndyGauge
- @ChristopherDavenport
- @chrisvittal
- @cuviper
- @dns2utf8
- @dtolnay
- @frewsxcv
- @gsquire
- @Hittherhod
- @jdr023
- @laumann
- @leodasvacas
- @lvillani
- @MajorBreakfast
- @mamuleanu
- @marmistrz
- @mbrubeck
- @mgattozzi
- @nikomatsakis
- @smt923
- @stjepang
- @tmccombs
- @vishalsodani
- bors[bot]


# Release rayon 0.8.2 (2017-06-28)

- `ParallelSliceMut` now has six parallel sorting methods with the same
  variations as the standard library.
  - `par_sort`, `par_sort_by`, and `par_sort_by_key` perform stable sorts in
    parallel, using the default order, a custom comparator, or a key extraction
    function, respectively.
  - `par_sort_unstable`, `par_sort_unstable_by`, and `par_sort_unstable_by_key`
    perform unstable sorts with the same comparison options.
  - Thanks to @stjepang!


# Release rayon 0.8.1 / rayon-core 1.2.0 (2017-06-14)

- The following core APIs are being stabilized:
  - `rayon::spawn()` -- spawns a task into the Rayon thread pool; as it
    is contained in the global scope (rather than a user-created
    scope), the task cannot capture anything from the current stack
    frame.
  - `ThreadPool::join()`, `ThreadPool::spawn()`, `ThreadPool::scope()`
    -- convenience APIs for launching new work within a thread pool.
- The various iterator adapters are now tagged with `#[must_use]`
- Parallel iterators now offer a `for_each_with` adapter, similar to
  `map_with`.
- We are adopting a new approach to handling the remaining unstable
  APIs (which primarily pertain to futures integration). As awlays,
  unstable APIs are intended for experimentation, but do not come with
  any promise of compatibility (in other words, we might change them
  in arbitrary ways in any release). Previously, we designated such
  APIs using a Cargo feature "unstable". Now, we are using a regular
  `#[cfg]` flag. This means that to see the unstable APIs, you must do
  `RUSTFLAGS='--cfg rayon_unstable' cargo build`. This is
  intentionally inconvenient; in particular, if you are a library,
  then your clients must also modify their environment, signaling
  their agreement to instability.


# Release rayon 0.8.0 / rayon-core 1.1.0 (2017-06-13)

## Rayon 0.8.0

- Added the `map_with` and `fold_with` combinators, which help for
  passing along state (like channels) that cannot be shared between
  threads but which can be cloned on each thread split.
- Added the `while_some` combinator, which helps for writing short-circuiting iterators.
- Added support for "short-circuiting" collection: e.g., collecting
  from an iterator producing `Option<T>` or `Result<T, E>` into a
  `Option<Collection<T>>` or `Result<Collection<T>, E>`.
- Support `FromParallelIterator` for `Cow`.
- Removed the deprecated weight APIs.
- Simplified the parallel iterator trait hierarchy by removing the
  `BoundedParallelIterator` and `ExactParallelIterator` traits,
  which were not serving much purpose.
- Improved documentation.
- Added some missing `Send` impls.
- Fixed some small bugs.

## Rayon-core 1.1.0

- We now have more documentation.
- Renamed the (unstable) methods `spawn_async` and
  `spawn_future_async` -- which spawn tasks that cannot hold
  references -- to simply `spawn` and `spawn_future`, respectively.
- We are now using the coco library for our deque.
- Individual thread pools can now be configured in "breadth-first"
  mode, which causes them to execute spawned tasks in the reverse
  order that they used to.  In some specific scenarios, this can be a
  win (though it is not generally the right choice).
- Added top-level functions:
  - `current_thread_index`, for querying the index of the current worker thread within
    its thread pool (previously available as `thread_pool.current_thread_index()`);
  - `current_thread_has_pending_tasks`, for querying whether the
    current worker that has an empty task deque or not. This can be
    useful when deciding whether to spawn a task.
- The environment variables for controlling Rayon are now
  `RAYON_NUM_THREADS` and `RAYON_LOG`. The older variables (e.g.,
  `RAYON_RS_NUM_CPUS` are still supported but deprecated).

## Rayon-demo

- Added a new game-of-life benchmark.

## Contributors

Thanks to the following contributors:

- @ChristopherDavenport
- @SuperFluffy
- @antoinewdg
- @crazymykl
- @cuviper
- @glandium
- @julian-seward1
- @leodasvacas
- @leshow
- @lilianmoraru
- @mschmo
- @nikomatsakis
- @stjepang


# Release rayon 0.7.1 / rayon-core 1.0.2 (2017-05-30)

This release is a targeted performance fix for #343, an issue where
rayon threads could sometimes enter into a spin loop where they would
be unable to make progress until they are pre-empted.


# Release rayon 0.7 / rayon-core 1.0 (2017-04-06)

This release marks the first step towards Rayon 1.0. **For best
performance, it is important that all Rayon users update to at least
Rayon 0.7.** This is because, as of Rayon 0.7, we have taken steps to
ensure that, no matter how many versions of rayon are actively in use,
there will only be a single global scheduler. This is achieved via the
`rayon-core` crate, which is being released at version 1.0, and which
encapsulates the core schedule APIs like `join()`. (Note: the
`rayon-core` crate is, to some degree, an implementation detail, and
not intended to be imported directly; it's entire API surface is
mirrored through the rayon crate.)

We have also done a lot of work reorganizing the API for Rayon 0.7 in
preparation for 1.0. The names of iterator types have been changed and
reorganized (but few users are expected to be naming those types
explicitly anyhow). In addition, a number of parallel iterator methods
have been adjusted to match those in the standard iterator traits more
closely. See the "Breaking Changes" section below for
details.

Finally, Rayon 0.7 includes a number of new features and new parallel
iterator methods. **As of this release, Rayon's parallel iterators
have officially reached parity with sequential iterators** -- that is,
every sequential iterator method that makes any sense in parallel is
supported in some capacity.

### New features and methods

- The internal `Producer` trait now features `fold_with`, which enables
  better performance for some parallel iterators.
- Strings now support `par_split()` and `par_split_whitespace()`.
- The `Configuration` API is expanded and simplified:
    - `num_threads(0)` no longer triggers an error
    - you can now supply a closure to name the Rayon threads that get created
      by using `Configuration::thread_name`.
    - you can now inject code when Rayon threads start up and finish
    - you can now set a custom panic handler to handle panics in various odd situations
- Threadpools are now able to more gracefully put threads to sleep when not needed.
- Parallel iterators now support `find_first()`, `find_last()`, `position_first()`,
  and `position_last()`.
- Parallel iterators now support `rev()`, which primarily affects subsequent calls
  to `enumerate()`.
- The `scope()` API is now considered stable (and part of `rayon-core`).
- There is now a useful `rayon::split` function for creating custom
  Rayon parallel iterators.
- Parallel iterators now allow you to customize the min/max number of
  items to be processed in a given thread. This mechanism replaces the
  older `weight` mechanism, which is deprecated.
- `sum()` and friends now use the standard `Sum` traits

### Breaking changes

In the move towards 1.0, there have been a number of minor breaking changes:

- Configuration setters like `Configuration::set_num_threads()` lost the `set_` prefix,
  and hence become something like `Configuration::num_threads()`.
- `Configuration` getters are removed
- Iterator types have been shuffled around and exposed more consistently:
    - combinator types live in `rayon::iter`, e.g. `rayon::iter::Filter`
    - iterators over various types live in a module named after their type,
      e.g. `rayon::slice::Windows`
- When doing a `sum()` or `product()`, type annotations are needed for the result
  since it is now possible to have the resulting sum be of a type other than the value
  you are iterating over (this mirrors sequential iterators).

### Experimental features

Experimental features require the use of the `unstable` feature. Their
APIs may change or disappear entirely in future releases (even minor
releases) and hence they should be avoided for production code.

- We now have (unstable) support for futures integration. You can use
  `Scope::spawn_future` or `rayon::spawn_future_async()`.
- There is now a `rayon::spawn_async()` function for using the Rayon
  thread pool to run tasks that do not have references to the stack.

### Contributors

Thanks to the following people for their contributions to this release:

- @Aaronepower
- @ChristopherDavenport
- @bluss
- @cuviper
- @froydnj
- @gaurikholkar
- @hniksic
- @leodasvacas
- @leshow
- @martinhath
- @mbrubeck
- @nikomatsakis
- @pegomes
- @schuster
- @torkleyy


# Release 0.6 (2016-12-21)

This release includes a lot of progress towards the goal of parity
with the sequential iterator API, though there are still a few methods
that are not yet complete. If you'd like to help with that effort,
[check out the milestone](https://github.com/rayon-rs/rayon/issues?q=is%3Aopen+is%3Aissue+milestone%3A%22Parity+with+the+%60Iterator%60+trait%22)
to see the remaining issues.

**Announcement:** @cuviper has been added as a collaborator to the
Rayon repository for all of his outstanding work on Rayon, which
includes both internal refactoring and helping to shape the public
API. Thanks @cuviper! Keep it up.

- We now support `collect()` and not just `collect_with()`.
  You can use `collect()` to build a number of collections,
  including vectors, maps, and sets. Moreover, when building a vector
  with `collect()`, you are no longer limited to exact parallel iterators.
  Thanks @nikomatsakis, @cuviper!
- We now support `skip()` and `take()` on parallel iterators.
  Thanks @martinhath!
- **Breaking change:** We now match the sequential APIs for `min()` and `max()`.
  We also support `min_by_key()` and `max_by_key()`. Thanks @tapeinosyne!
- **Breaking change:** The `mul()` method is now renamed to `product()`,
  to match sequential iterators. Thanks @jonathandturner!
- We now support parallel iterator over ranges on `u64` values. Thanks @cuviper!
- We now offer a `par_chars()` method on strings for iterating over characters
  in parallel. Thanks @cuviper!
- We now have new demos: a traveling salesman problem solver as well as matrix
  multiplication. Thanks @nikomatsakis, @edre!
- We are now documenting our minimum rustc requirement (currently
  v1.12.0).  We will attempt to maintain compatibility with rustc
  stable v1.12.0 as long as it remains convenient, but if new features
  are stabilized or added that would be helpful to Rayon, or there are
  bug fixes that we need, we will bump to the most recent rustc. Thanks @cuviper!
- The `reduce()` functionality now has better inlining.
  Thanks @bluss!
- The `join()` function now has some documentation. Thanks @gsquire!
- The project source has now been fully run through rustfmt.
  Thanks @ChristopherDavenport!
- Exposed helper methods for accessing the current thread index.
  Thanks @bholley!


# Release 0.5 (2016-11-04)

- **Breaking change:** The `reduce` method has been vastly
  simplified, and `reduce_with_identity` has been deprecated.
- **Breaking change:** The `fold` method has been changed. It used to
  always reduce the values, but now instead it is a combinator that
  returns a parallel iterator which can itself be reduced. See the
  docs for more information.
- The following parallel iterator combinators are now available (thanks @cuviper!):
  - `find_any()`: similar to `find` on a sequential iterator,
    but doesn't necessarily return the *first* matching item
  - `position_any()`: similar to `position` on a sequential iterator,
    but doesn't necessarily return the index of *first* matching item
  - `any()`, `all()`: just like their sequential counterparts
- The `count()` combinator is now available for parallel iterators.
- We now build with older versions of rustc again (thanks @durango!),
  as we removed a stray semicolon from `thread_local!`.
- Various improvements to the (unstable) `scope()` API implementation.


# Release 0.4.3 (2016-10-25)

- Parallel iterators now offer an adaptive weight scheme,
  which means that explicit weights should no longer
  be necessary in most cases! Thanks @cuviper!
  - We are considering removing weights or changing the weight mechanism
    before 1.0. Examples of scenarios where you still need weights even
    with this adaptive mechanism would be great. Join the discussion
    at <https://github.com/rayon-rs/rayon/issues/111>.
- New (unstable) scoped threads API, see `rayon::scope` for details.
  - You will need to supply the [cargo feature] `unstable`.
- The various demos and benchmarks have been consolidated into one
  program, `rayon-demo`.
- Optimizations in Rayon's inner workings. Thanks @emilio!
- Update `num_cpus` to 1.0. Thanks @jamwt!
- Various internal cleanup in the implementation and typo fixes.
  Thanks @cuviper, @Eh2406, and @spacejam!

[cargo feature]: https://doc.rust-lang.org/cargo/reference/features.html#the-features-section


# Release 0.4.2 (2016-09-15)

- Updated crates.io metadata.


# Release 0.4.1 (2016-09-14)

- New `chain` combinator for parallel iterators.
- `Option`, `Result`, as well as many more collection types now have
  parallel iterators.
- New mergesort demo.
- Misc fixes.

Thanks to @cuviper, @edre, @jdanford, @frewsxcv for their contributions!


# Release 0.4 (2016-05-16)

- Make use of latest versions of catch-panic and various fixes to panic propagation.
- Add new prime sieve demo.
- Add `cloned()` and `inspect()` combinators.
- Misc fixes for Rust RFC 1214.

Thanks to @areilb1, @Amanieu, @SharplEr, and @cuviper for their contributions!


# Release 0.3 (2016-02-23)

- Expanded `par_iter` APIs now available:
  - `into_par_iter` is now supported on vectors (taking ownership of the elements)
- Panic handling is much improved:
  - if you use the Nightly feature, experimental panic recovery is available
  - otherwise, panics propagate out and poision the workpool
- New `Configuration` object to control number of threads and other details
- New demos and benchmarks
  - try `cargo run --release -- visualize` in `demo/nbody` :)
    - Note: a nightly compiler is required for this demo due to the
      use of the `+=` syntax

Thanks to @bjz, @cuviper, @Amanieu, and @willi-kappler for their contributions!


# Release 0.2 and earlier

No release notes were being kept at this time.
