## 1.10.0

### Breaking Changes

- The minimum supported Rust version has been increased to 1.84.0. ([\#612](https://github.com/proptest-rs/proptest/pull/612))

### New Features

- Added a memory-efficient strategy for sampling subsets from ranges without replacement. ([\#586](https://github.com/proptest-rs/proptest/pull/586))

### Other Notes

- Updated `convert_case` requirement from 0.6 to 0.11. ([\#623](https://github.com/proptest-rs/proptest/pull/623))
- Updated `trybuild` requirement from =1.0.113 to =1.0.115. ([\#624](https://github.com/proptest-rs/proptest/pull/624))

## 1.9.0

### Breaking Changes

- The minimum supported Rust version has been increased to 1.82.0. ([\#605](https://github.com/proptest-rs/proptest/pull/605))

### New Features

- Added `ProptestResultExt` trait with `prop_assume_ok` helper method for converting `Result` types to test outcomes. ([\#589](https://github.com/proptest-rs/proptest/pull/589))

### Other Notes

- Replaced `lazy_static` dependency with `std::sync::LazyLock`. ([\#546](https://github.com/proptest-rs/proptest/pull/546))
- Fixed doctest and cleaned up warnings. ([\#606](https://github.com/proptest-rs/proptest/pull/606))

## 1.8.0

### New Features

- Implement `Arbitrary` for `Saturating<T>`. ([\#585](https://github.com/proptest-rs/proptest/pull/585))
- Allow `prop_assert!` with trailing comma. ([\#581](https://github.com/proptest-rs/proptest/pull/581))

### Bug Fixes

- Fix arithmetic overflow on 32-bit processors. ([\#596](https://github.com/proptest-rs/proptest/pull/596))
- Sanitize user-facing macros to use fully qualified paths, preventing conflicts with user code. ([\#584](https://github.com/proptest-rs/proptest/pull/584))

## 1.7.0

### Other Notes

- Updated `rand` dependency from 0.8 to 0.9.
- Bump all dependencies to latest compatible with MSRV 1.66.

## 1.6.0

### New Features

- Added `handle-panics` feature which enables catching panics raised in tests
  and turning them into failures. ([\#525](https://github.com/proptest-rs/proptest/pull/525))
- Exit early if shrink disabled. ([\#520](https://github.com/proptest-rs/proptest/pull/520))
- Add `Config::with_failure_persistence`. A convenience constructor making use
  of a generic parameter over `FailurePersistence` impls and hiding the
  `Some(Box::new(...))`. ([\#508](https://github.com/proptest-rs/proptest/pull/508))
- Add From's for SizeRange and Probability. ([\#498]([https://github.com/proptest-rs/proptest/pull/498))
- When running persisted regressions, the most recently added regression is now
  run first. ([\#496](https://github.com/proptest-rs/proptest/pull/496]))

### Bug Fixes

- Fix WebAssembly support. Hides a few paths, that fail at runtime on
  wasm32-unknown-unknown, under conditional compilation. \([#519](https://github.com/proptest-rs/proptest/pull/519))
- Fix incorrectly reading environment configuration. Previously controlling
  proptest configuration via env vars was not properly applied. This caused
  vars like `PROPTEST_MAX_DEFAULT_SIZE_RANGE` to be not be properly applied,
  leading to unexpected behavior. ([\#457](https://github.com/proptest-rs/proptest/pull/457))
- Allow trailing comma in prop_assert_eq/ne like std. ([\#510](https://github.com/proptest-rs/proptest/pull/510))

### Other Notes

- Add `no_std` to `alloc` contexts. `no_std` must be used explicitly with
  `alloc`. Updated CI and documentation to reflect this. ([\#528](https://github.com/proptest-rs/proptest/pull/528))
- Make `libm` optional in a `std` environment. ([\#524](https://github.com/proptest-rs/proptest/pull/524))
- Update `bit-set` and `bit-vec` to `0.8.0`. ([\#501](https://github.com/proptest-rs/proptest/pull/501))
- Removed unused `frunk` feature. ([\#498](https://github.com/proptest-rs/proptest/pull/498))

## 1.5.0

### New Features

- Setting `PROPTEST_MAX_DEFAULT_SIZE_RANGE` now customizes the default `SizeRange`
  used by the default strategies for collections (like `Vec`). The default remains 100.
- Empty ranges panic during tree creation instead of during sampling.

### Documentation

- Reference the derive macro in Arbitrary's documentation
- Fix broken links in the book

### Bug Fixes

- Fixed issue where config contextualization would clobber existing failure persistence config

## 1.4.0

### Breaking Changes

- The minimum supported Rust version has been increased to 1.65.0.

### Other Notes

- `regex-syntax` updated from 0.7 to 0.8
- Fixed new clippies
- Fixed nightly build where Generator was renamed to Coroutine

## 1.3.1

### Other Notes

- `bit-set` updated from 0.5.0 to 0.5.2 to ensure minimum compatible version with bit-vec 0.6

## 1.3.0

### Breaking Changes

- The minimum supported Rust version has been increased to 1.64.0.

### New Features

- Adds Arbitrary impl for PathBuf
- Permit use of (?-u) in byte-regex strategies, allowing non-utf-8 bytes to be generated

### Book

- Various small fixes -- typos, formatting
- Removal of custom theme
- Add book page for Tips and Best Practices

### Other Notes

- `regex-syntax` version 0.7 is now used.
- Print a seed to stderr for a failed test even when a regressions file is already present.
- Fixed a performance issue with `VarBitSet::saturated` that can slow down `VecStrategy`
- Remove use of rust feature `core_intrinsics`
- Remove no longer needed "break-dead-code" feature
- Disable `clippy::arc_with_non_send_sync`
- Remove dependency on `byteorder`

## 1.2.0

### Breaking Changes

- `PROPTEST_` environment variables now take precedence over tests' non-default
  configuration.

### Bug Fixes

- Don't implement Arbitrary for NonZeroU128 and NonZeroI128 on wasm targets where
  u128 and i128 Arbitrary impls don't exist

### New Features

### Other Notes

- Minimal failing input is now printed using debug pretty-printing
- Made public `VarBitSet`, `SizeRange` read-only methods and num sampling
  functions in preparation for release of a `proptest-state-machine` crate.
- Removed dependency on `quick_error`
- Start publishing MSRV

## 1.1.0

### Bug Fixes

- Sampling from large ranges of floats such as `(0f32)..` no longer panics
  with newer versions of the `rand` crate
- [dependencies.x86] was bumped to latest current version. x86 crate does
  was on a very old version 0.33.0 which used a removed macro from rust.
- The calculation for the arbitrary impl of Layout was using a max_size that
  was too large and overflowing Layout. This has been fixed.
- Test for arbitrary AllocError was referring to AllocErr which
  does not exist, this was fixed.
- NoneError has been removed from rust so it was subsequently
  removed from proptest. It was blocking compilation. evidence:
  https://github.com/rust-lang/rust/issues/85614
- `try_reserve` is stable so removed from unstable features
- `try_trait` has been changed to `try_trait_v2` so that was adjusted
  in `Cargo.toml`.
- `prop_assert_ne!` now uses fully qualified `prop_assert_eq!`
- Persisted tests are not counted against the number of cases to run

### New Features

- Add `Arbitrary` impls for arrays of all sizes using const generics
- Add `Arbitrary` impls for `core::num::NonZero*`
- Adds ability to disable failure persistence via env var `PROPTEST_DISABLE_FAILURE_PERSISTENCE`

### Other Notes

- `proptest` no longer depends on the `quick-error` crate.

## 1.0.0

### Breaking Changes

- The minimum supported Rust version has been increased to 1.50.0.

- The version of the `rand` crate has been increased to 0.8.

- Due to changes in the `getrandom` crate, if you wish to use proptest on the
  `wasm32-unknown-unknown` target, you must manually add a dependency on that
  crate and enable a feature that will allow it to work. Refer to the
  `getrandom` crate documentation for more information.

### Bug Fixes

- `prop_shuffle()` can now produce all permutations.

### New Features

- Tuple strategies up to 12 elements are now supported, for parity with the
  blanket implementations that `std` provides.

## 0.10.1

### New Features

- Added `RngAlgorithm::Recorder` and supporting APIs which allow capturing
  random data generated as part of generating a value or running a test.

## 0.10.0

### Breaking Changes

- The version of the `rand` crate has been increased to 0.7.

- The `proptest!` macro no longer accepts function bodies which implicitly
  return a value (which would then be discarded).

- The `TupleUnion` implementation in `proptest` 0.9 has been removed and
  replaced with `LazyTupleUnion`. `prop_oneof!` is unaffected and continues
  to be the recommended way to build a union of strategies.

### New Features

- Enabling the `hardware-rng` optional dependency (disabled by default) allows
  obtaining non-deterministic random seeds even in `no_std` environments
  provided the architecture is x86 or AMD64.

- Added missing `?Sized` bound to `B` on the implementation of
  `Arbitrary` for `std::borrow::Cow<'_, B>`.

### Bug Fixes

- `prop_assert!` and `prop_assume!` should now be usable in `no_std`
  environments.

### Other Notes

- `rusty_fork` has been bumped to 0.3.0, which adds support for a number of
  [new test flags](https://github.com/AltSysrq/rusty-fork/blob/master/CHANGELOG.md#improvements)
  when running forked tests.

- The `PassThrough` RNG algorithm now returns 0 instead of panicking when it
  runs out of entropy.

## 0.9.6

### Bug Fixes

- Fixed [#186](https://github.com/proptest-rs/proptest/issues/186),
  a Rust future-compatibility issue.

## 0.9.5

### Bug Fixes

- Fixed a Rust future-compatibility issue (https://github.com/rust-lang/rust/pull/65819).

### New Additions

## 0.9.4

### Bug Fixes

- The `unstable` feature one again works against the latest nightly.

### Performance Improvements

- Unions and the `prop_oneof!` combinator now generate value trees
  lazily.

  In previous versions of `proptest`, if a value tree for a
  union variant was generated, so would value trees for earlier
  variants -- as a result, union value tree generation was linear in the
  number of variants.

  In `proptest` 0.9.4 and above, value trees are only generated for
  union variants that are picked. Union value tree generation is now
  independent of the number of variants.

### Deprecations

- `TupleUnion` has been deprecated, and its implementation will be
  replaced by `LazyTupleUnion`'s in 0.10.0.

### Other Notes

- The return type of `prop_oneof!` has changed from `TupleUnion` to
  `LazyTupleUnion`. `prop_oneof!`'s return type is documented to not be
  stable, and that continues to be the case.

- Shrinking is now limited to four times as many iterations as configured
  number of test cases by default.

- `prop_assert_eq!` and `prop_assert_ne!` produce output more similar to the
  `assert_eq!` and `assert_ne!` macros. This should also make it easier to
  visually parse out the source location in the resulting messages.

## 0.9.3

This is a minor release to correct some packaging errors. The license files are
now included in the files published to crates.io, and some unneeded files are
now excluded.

## 0.9.2

### New Additions

- Closures generated by `prop_compose!` are now `move`. This is not expected to
  cause any breakage since there is no way to successfully use a borrowing
  closure with that macro.

- There is now **highly experimental** support for building on Web Assembly.
  Refer to [the Proptest
  book](https://altsysrq.github.io/proptest-book/proptest/wasm.html) for build
  instructions.

### Other Notes

- Using proptest with the default `std` feature enabled, the `spin` crate is no
  longer brought in as a dependency.

- Using proptest with the `std` feature disabled, neither `spin` nor
  `lazy_static` are brought in as dependencies.

## 0.9.1

### New RNG Algorithm

Starting in this version, the default RNG algorithm has been changed from
XorShift to ChaCha since it produces higher-quality randomness. This may make
test case generation a bit slower but it avoids certain pathological cases that
the old generator had.

The old algorithm is still supported, and is used automatically when reading
old failure persistence files.

Note that this change also affects the internal representation of RNG seeds,
which affects the `FailurePersistence` trait which previously only supported
the seed representation for XorShift. This release maintains source
compatibility with 0.9.0 by providing defaults for the new methods which
delegate (when possible) to the old ones, but be aware that custom failure
persistence implementations using the old API will not function when using an
RNG other than XorShift.

To keep using the old algorithm, you can set the environment variable
`PROPTEST_RNG_ALGORITHM` to `xs` or set `Config.rng_algorithm` to
`RngAlgorithm::XorShift` in code.

Besides ChaCha, this version also adds a `PassThrough` RNG "algorithm" which
makes it possible to use an external source of entropy with Proptest.

### New Additions

- `TestRng` instances can be created with the `from_seed` function.

- `TestRunner` instances can be created with user-provided `TestRng`s.

- `TestRunner` now has a `deterministic()` constructor which uses the same RNG
  every time, to facilitate doing statistical tests on strategy outputs.

- There is now a work-around for a [compiler
  bug](https://github.com/rust-lang/rust/issues/52478) which prevents building
  with `-C link-dead-code`. Please see this issue for details:
  https://github.com/proptest-rs/proptest/issues/124

### Deprecations

- The `load_persisted_failures` and `save_persisted_failure` methods on the
  `FailurePersistence` trait have been deprecated and will be removed in
  0.10.0.

## 0.9.0

### Breaking Changes

- The minimum Rust version has been increased to 1.32.0.

- The version of the `rand` crate has been increased to 0.6.

- The `ValueFor` type alias (deprecated in 0.8.0) has been removed. Replace
  `ValueFor<S>` with `S::Value` or `<S as Strategy>::Value` as necessary.

- `From<SizeRange>` implementations converting a `SizeRange` back to various
  std types have been removed since they were of limited utility and had
  unclear or incorrect conversion properties.

- Many optional elements (such as trailing commas or function visibility
  modifiers) in certain macros could be specified more than once. The macros
  now accept at most one occurrence.

- Visibility modifiers inside `prop_compose` must no longer be enclosed in
  brackets. Unless other modifiers (e.g., `unsafe`) are also in use, simply
  removing the brackets is sufficient.

### New Additions

- Rust 2018 style macro imports are now supported.

- In a Rust 2018 crate, all the macros can be brought into scope with
  `import proptest::prelude::*;`.

- The proptest macros now accept trailing commas in more locations.

- Visibility modifiers can now be passed to `prop_compose!` without enclosing
  them in brackets. Unfortunately, the old way could not continue to be
  supported due to the way the `vis` macro matcher works.

### Nightly-only breakage

- The `nightly` feature, which was formerly required for using proptest with
  `#[no_std]`, has been removed. References to the feature can simply be
  deleted.

- When using the `unstable` feature and setting `default-features = false`, the
  `AtomicI64` and `AtomicU64` types are not supported unless the `atomic64bit`
  feature is enabled. This supports `no_std` usage on platforms which do not
  support atomic 64-bit operations.

### Other Notes

- Generated strings are now much more likely to contain right-to-left override
  characters.

- Most of the crate-level documentation has been relocated to the [Proptest
  Book](https://altsysrq.github.io/proptest-book/proptest/index.html).

## 0.8.7

### New Additions

- Add `max_shrink_iters` and `max_shrink_time` options to test configuration to
  allow capping the resources expended on shrinking test cases.

- Add `verbose` option to make proptest give details about what is happening as
  the test executes.

- When a failure is saved to the persistence file, the message now also
  includes the seed that was saved so that it can manually be added to the
  appropriate file should the test have run somewhere where the updated file is
  not accessible (for example, on a CI system).

### Bug Fixes

- `any::<SystemTime>()` now generates random values centred on the UNIX epoch
  rather than always producing the current time.

### Other Notes

- When using forking, proptest will now detect conditions which cause the child
  process to crash without running any tests, and will fail quickly instead of
  respawning child processes.

## 0.8.6

### New Additions

- `Vec<S> where S: Strategy` is now itself a `Strategy` for producing
  fixed-size `Vec`s whose values are derived from the respective strategies.

- It is now possible to configure the test runner to cache test results to
  avoid spending time running identical tests. See `Config.result_cache`.

- Add `sample::Index`, a type for generating indices into runtime-sized slices
  and vectors.

- Add `sample::Selector`, a type for picking items out of dynamically-created
  iterables.

### Bug Fixes

- Fix panic when using `sample::subsequence` with an empty vector.

- Fix panic when using `sample::subsequence` with a size equal to the size of
  the input vector.

- Fix sampled bitset strategies on integers not allowing to generate exactly
  the same number of bits as the integer is wide.

### Other Notes

- Passing empty size ranges to functions requiring a non-empty size range now
  panic with an explicit message immediately rather than causing an arithmetic
  error when generating input values.

- There were a few cases where proptest would accept a `SizeRange` with an
  inclusive maximum value of `usize::MAX`. Size ranges are now always clamped
  to `usize::MAX - 1`.

## 0.8.5

### Bug Fixes

- Fix build when nightly features are enabled.

## 0.8.4

### Bug Fixes

- Nightly and no_std support work against latest nightly once again.

### New Additions

- Added `bits::bool_vec` for generating `Vec<bool>` as a bit set.

### Nightly-only breakage

- `impl Arbitrary for CollectionAllocErr` is temporarily removed pending it
  being available outside the `alloc` crate again.

- `bits::bitset` is no longer available without the `bit-set` feature (enabled
  by default), which is [not compatible with `#[no_std]`
  environments](https://github.com/contain-rs/bit-vec/pull/51).

## 0.8.3

### Bug Fixes

- Fix that regex-based string generation could transpose the order of a literal
  and a non-literal component.

## 0.8.2

### New Additions

- Macros which previously accepted `pattern in strategy` syntax to specify
  arguments now also accept `pattern: type` syntax as shorthand for
  `pattern in any::<type>()`.

- Closure-style `proptest!` invocation no longer requires the body to use block
  syntax.

- Closure-style `proptest!` invocation now accepts custom configurations.

## 0.8.1

### New Additions

- `proptest!` now has form that accepts a closure. See the documentation for
  the macro for more details.

### Bug Fixes

- Fix spurious warning about corrupt regression files. The files were not
  corrupt but the parser was failing to handle the blank line at the end.

- The `multiplex_alloc!` and `multiplex_core!` macros which were
  unintentionally exported in 0.8.0 are no longer exported. This is not
  considered a breaking change since they were not supposed to be accessible,
  and in any case would not have expanded into valid code in most other crates.

## 0.8.0

### New Additions

- A combinator `.prop_filter_map` has been added to `Strategy`.
  It is similar to `.filter_map` for `Iterator` in that it is the
  combination of `.prop_filter` and `.prop_map`.

- `i128` and `u128` are now supported without any feature flags and on stable.

- More implementations of `Arbitrary` are supported for `alloc` + `no_std` users.

- `size_range` now accepts inclusive ranges of form `low..=high` and `..=high`.
  Thus, you can construct a `vec` strategy as: `vec(elt_strategy, low..=high)`
  and `vec(elt_strategy, ..=high)`. This also applies to other functions
  accepting `Into<SizeRange>`.

- `..= high` is now a valid strategy. Please note that `..= 1` will naturally
  include numbers lower than `0` for sized types.

- `low..=high` is also a valid strategy.

- `Arbitrary` is implemented for `RangeInclusive<Idx>`, `RangeToInclusive`,
  and `DecodeUtf16` on stable.

- Bitset strategies and `subsequence` now accept all range syntaxes.

### Bug Fixes

- Fix a race condition where a test failing due to running ever so slightly
  over the set timeout could cause the test harness to converge to the
  incorrect failing value, a non-failing value, or panic.

### Deprecations

- The type alias `ValueFor<S>` is now deprecated and will be removed in
  version 0.9. You should just use `S::Value` instead.

### Breaking changes

- A minimum version of 1.27 of Rust is now required.

- `regex-syntax` version 0.6 is now used.

- `rand` version 0.5 is now used.

- As a consequence, the `FailurePersistence` trait will now use `[u8; 16]` seeds
  instead of `[u32; 4]`. However, the stored failure persistence files using
  the default `FileFailurePersistence` will still use `[u32; 4]` so your old
  failure persistence files should still work.

- The RNG used by proptest has been changed to a PRNG `TestRng` which proptest
  exposes. This is currently a simple new-type wrapper around `XorShiftRng`.
  In the future, this will give us more freedom to make changes without breakage.

- The feature flag `i128_support` has been removed. The features it added are
  now always supported.

- The associated type `Value` of `Strategy` has been renamed to `Tree`.
  A new associated type `Value` has been added to `Strategy` which always refers
  to the same type as `<S::Tree as ValueTree>::Value` for some strategy `S`.
  This change allows you to write `-> impl Strategy<Value = T>` for functions
  returning a `Strategy` generating `T`s. This is more ergonomic to use than
  `-> impl Strategy<Value = impl ValueTree<Value = T>>`.

- The method `new_value` in `Strategy` has been renamed to `new_tree` to mirror
  the renaming of `Value` to `Tree`.

- As a consequence change, the associated type `ValueTree` has been removed from
  `Arbitrary`.

- The methods `run` and `run_one` on `TestRunner` now takes a function-under-test
  that accepts the generated type by value instead of by reference instead.
  This means that you don't need to write `ref value in my_strategy` and can
  write `value in my_strategy` instead even if `typeof(value)` doesn't implement
  `Copy`. This is also a step in the direction of allowing strategies to generate
  references when generic associated types (GATs) land.
  However, `ref value in my_strategy` will still be accepted, so not a lot of
  breakage should come of this if you've used `proptest! { .. }`.

- `prop_compose!` no longer applies `.boxed()` to the strategy produced.
  Therefore, `-> BoxedStrategy<T>` is no longer the correct type.
  The new return type is `-> impl Strategy<Value = T>`.
  If you want the old behaviour, you can use `.boxed()` yourself.

- `Arbitrary` for `SizeRange` changed its associated type to use `RangeInclusive`.
  Same applies for `CString`.

- Many APIs now use `impl Trait` in argument position, which could affect code
  using turbofishes to specify types explicitly.

- `char` APIs which formerly represented a range as `(start, end)` now require
  `start..=end`.

### Nightly-only breakage

- As `std::io::{Chars, CharsError}` have been deprecated on nightly,
  their `Arbitrary` implementations have been removed.

## 0.7.2

### Bug Fixes

- Fix that `bool` would not shrink correctly, leading to hangs when tests
  taking `bool` parameters would fail in certain circumstances.

## 0.7.1

### New Additions

- It is now possible to run test cases in sub-processes. This allows using
  proptest to test functions which may cause the test process to terminate
  abruptly, such as by calling `abort()` or even suffering a segmentation
  fault. This requires the "fork" feature, enabled by default.

- Added support for setting a timeout which applies on a per-test-case (i.e.,
  single input rather than the whole test) basis. This allows using proptest to
  find inputs which cause code to get stuck in infinite loops or exhibit other
  pathological performance behaviour. This requires the "timeout" feature (and
  transitively, the "fork" feature), enabled by default.

See also [the documentation](README.md#forking-and-timeouts) for these
features.

### Bug Fixes

- Fix that failure persistence file would be written to the incorrect location
  in projects using workspaces. See
  [#24](https://github.com/proptest-rs/proptest/issues/24) for more details and
  instructions on how to migrate any persistence files that had been written to
  the wrong location.

- Fix a case where `any::<ArgsOs>()` or `any::<VarsOs>()` could panic on
  Windows.

### Nightly-only breakage

- Support for the `hashmap_core` crate is removed pending
  https://github.com/Amanieu/hashmap_core/issues/3.

## 0.7.0

### Potential Breaking Changes

- The persistence system has been refactored to allow for
  non-file-system based persistence. `FailurePersistence`
  is now a trait, and the prior file-based enum which fulfilled
  that purpose is now called `FileFailurePersistence` and implements
  the generic trait. The default behavior has not changed.

- Reflecting the change to persistence, `Config.failure_persistence`
  is now of type `Option<Box<FailurePersistence>>`.

- The `source_file` used as an optional reference point to the location of the
  calling test is now tracked on the `Config` struct rather than the
  `TestRunner`.

### New Additions

- Experimental support on nightly for working in `#![no_std]` environments has
  been added. To use it, one must disable the default-features for proptest and
  use the new "alloc" and "nightly" features. Currently access to a heap
  allocator is still required.

## 0.6.0

### Potential Breaking Changes

- There is a small change of breakage if you've relied on `Recursive` using an
  `Arc<BoxedStrategy<T>>` as `Recursive` now internally uses `BoxedStrategy<T>`
  instead as well as expecting a `Fn(BoxedStrategy<T>) -> R` instead of
  `Fn(Arc<BoxedStrategy<T>>) -> R`. In addition, the type of recursive
  strategies has changed from `Recursive<BoxedStrategy<T>, F>` to just
  `Recursive<T, F>`.

### Minor changes

- Reduced indirections and heap allocations inside `Recursive<T, F>` somewhat.

- `BoxedStrategy<T>` and `SBoxedStrategy<T>` now use `Arc` internally instead of
  using `Box`. While this has marginal overhead, it also reduces the overhead
  in `Recursive<T, F>`. The upside to this change is also that you can very
  cheaply clone strategies.

- `Filter` is marginally faster.

### Bug Fixes

- Removed `impl Arbitrary for LocalKeyState` since `LocalKeyState` no longer
  exists in the nightly compiler.

- Unstable features compile on latest nightly again.

## 0.5.1

### New Additions

- `proptest::strategy::Union` and `proptest::strategy::TupleUnion` now work
  with weighted strategies even if the sum of the weights overflows a `u32`.

- Added `SIGNALING_NAN` strategy to generate signalling NaNs if supported by
  the platform. Note that this is _not_ included in `ANY`.

### Bug Fixes

- Fixed values produced via `prop_recursive()` not shrinking from the recursive
  to the non-recursive case.

- Fix that `QUIET_NAN` would generate signalling NaNs on most platforms on Rust
  1.24.0 and later.

## 0.5.0

### Potential Breaking Changes

- There is a small chance of breakage if you've relied on the constraints put
  on type inference by the closure in `leaf.prop_recursive(..)` having a fixed
  output type. The output type is now any strategy that generates the same type
  as `leaf`. This change is intended to make working with recursive types a bit
  easier as you no longer have to use `.boxed()` inside the closure you pass to
  `.prop_recursive(..)`.

- There is a small chance of breakage wrt. type inference due to the
  introduction of `SizeRange`.

- There is a small chance of breakage wrt. type inference due to the
  introduction of `Probability`.

- `BoxedStrategy` and `SBoxedStrategy` are now newtypes instead of being type
  aliases. You will only experience breaking changes if you've directly
  used `.boxed()` and not `(S)BoxedStrategy<T>` but rather
  `Box<Strategy<Value = Box<ValueTree<Value = T>>>>`. The probability of
  breakage is very small, but still possible. The benefit of this change
  is that calling `.boxed()` or `.sboxed()` twice only boxes once. This can
  happen in situations where you have functions `Strategy -> BoxedStrategy` or
  with code generation.

- `proptest::char::ANY` has been removed. Any remaining uses must be replaced
  by `proptest::char::any()`.

- `proptest::strategy::Singleton` has been removed. Any remaining uses must be
  replaced by `proptest::strategy::Just`.

### New Additions

- Proptest now has an `Arbitrary` trait in `proptest::arbitrary` and re-exported
  in the `proptest::prelude`. `Arbitrary` has also been `impl`emented for most
  of the standard library. The trait provides a mechanism to define a canonical
  `Strategy` for a given type just like `Arbitrary` in Haskell's QuickCheck.
  Deriving for this trait will also be provided soon in the crate
  `proptest_derive`. To use the canonical strategy for a certain type `T`,
  you can simply use `any::<T>()`. This is the major new addition of this release.

- The `any_with`, `arbitrary`, `arbitrary_with` free functions in
  the module `proptest::arbitrary`.

- The `ArbitraryF1` and `ArbitraryF2` traits in `proptest::arbitrary::functor`.
  These are "higher order" `Arbitrary` traits that correspond to the `Arbitrary1`
  and `Arbitrary2` type classes in Haskell's QuickCheck. They are mainly provided
  to support a common set of container-like types in custom deriving self-recursive
  types in `proptest_derive`. More on this later releases.

- The strategies in `proptest::option` and `proptest::result` now accept a type
  `Probability` which is a wrapper around `f64`. Conversions from types such as
  `f64` are provided to make the interface ergonomic to use. Users may also use
  the `proptest::option::prob` function to explicitly construct the type.

- The strategies in `proptest::collections` now accept a type `SizeRange`
  which is a wrapper around `Range<usize>`. Conversions from types
  such as `usize` and `Range<usize>` are provided to make the interface
  ergonomic to use. Users may also use the `proptest::collections::size_bounds`
  function to explicitly construct the type.

- A `.prop_map_into()` operation on all strategies that map
  using `Into<OutputType>`. This is a clearer and cheaper
  operation than using `.prop_map(OutputType::from)`.

- A nonshrinking `LazyJust` strategy that can be used instead of `Just` when you
  have non-`Clone` types.

- Anything that can be coerced to `fn() -> T` where `T: Debug` is a `Strategy`
  where `ValueFor<fn() -> T> == T`. This is intended to make it easier to reuse
  proptest for unit tests with manual input space partition where `fn() -> T`
  provides fixtures.

### Minor changes

- Relaxed the constraints of `btree_map` removing `'static`.

- Reduced the heap allocation inside `Recursive` somewhat.

## 0.4.2

### Bug Fixes

- The `unstable` feature now works again.

## 0.4.1

### New Additions

- The `proptest::num::f32` and `proptest::num::f64` modules now have additional
  constants (e.g., `POSITIVE`, `SUBNORMAL`, `INFINITE`) which can be used to
  generate subsets of the floating-point domain by class and sign.

### Bug Fixes

- `proptest::num::f32::ANY` and `proptest::num::f64::ANY` now actually produce
  arbitrary values. Previously, they had the same effect as `0.0..1.0`. While
  this fix is a very substantial change in behaviour, it was not considered a
  breaking change since (a) the new behaviour is consistent with the
  documentation and expectations, (b) it's quite unlikely anyone was depending
  on the old behaviour since anyone who wanted that range would have written it
  out, and (c) Proptest isn't generally a transitive dependency so the chance
  of this update happening "by surprise" is low.

## 0.4.0

### Deprecations and Potential Breaking Changes

- `proptest::char::ANY` replaced with `proptest::char::any()`.
  `proptest::char::ANY` is present but deprecated, and will be removed in
  proptest 0.5.0.

- Instead of returning `-> Result<Self::Value, String>`, strategies are
  expected to return `-> Result<Self::Value, Reason>` instead. `Reason` reduces
  the amount of heap allocations, especially for `.prop_filter(..)` where you
  may now also pass in `&'static str`. You will only experience breaks if
  you've written your own strategy types or if you've used
  `TestCaseError::Reject` or `TestCaseError::Fail` explicitly.

- Update of externally-visible crate `rand` to `0.4.2`.

### New Additions

- Added `proptest::test_runner::Reason` which allows you to avoid heap
  allocation in some places and may be used to make the API richer in the
  future without incurring more breaking changes.

- Added a type alias `proptest::strategy::NewTree<S>` where `S: Strategy`
  defined as: `type NewTree<S> = Result<<S as Strategy>::Value, Rejection>`.

## 0.3.4

### Bug Fixes

- Cases where `file!()` returns a relative path, such as on Windows, are now
  handled more reasonably. See
  [#24](https://github.com/proptest-rs/proptest/issues/24) for more details and
  instructions on how to migrate any persistence files that had been written to
  the wrong location.

## 0.3.3

Boxing Day Special

### New Additions

- Added support for `i128` and `u128`. Since this is an unstable feature in
  Rust, this is hidden behind the feature `unstable` which you have to
  explicitly opt into in your `Cargo.toml` file.

- Failing case persistence. By default, when a test fails, Proptest will now
  save the seed for the failing test to a file, and later runs will test the
  persisted failing cases before generating new ones.

- Added `UniformArrayStrategy` and helper functions to simplify generating
  homogeneous arrays with non-`Copy` inner strategies.

- Trait `rand::Rng` and struct `rand::XorShiftRng` are now included in
  `proptest::prelude`.

### Bug Fixes

- Fix a case where certain combinations of strategies, like two
  `prop_shuffle()`s in close proximity, could result in low-quality randomness.

## 0.3.2

### New Additions

- Added `SampledBitSetStrategy` to generate bit sets based on size
  distribution.

- Added `Strategy::sboxed()` and `SBoxedStrategy` to make `Send + Sync` boxed
  strategies.

- `RegexGeneratorStrategy` is now `Send` and `Sync`.

- Added a type alias `ValueFor<S>` where `S: Strategy`. This is a shorter way
  to refer to: `<<S as Strategy>::Value as ValueTree>::Value`.

- Added a type alias `type W<T> = (u32, T)` for a weighted strategy `T` in the
  context of union strategies.

- `TestRunner` now implements `Default`.

- Added `Config::with_cases(number_of_cases: u32) -> Config` for simpler
  construction of a `Config` that only differs by the number of test cases.

- All default fields of `Config` can now be overridden by setting environment
  variables. See the docs of that struct for more details.

- Bumped dependency `rand = "0.3.18"`.

- Added `proptest::sample::subsequence` which returns a strategy generating
  subsequences, of the source `Vec`, with a size within the given `Range`.

- Added `proptest::sample::select` which returns a strategy selecting exactly
  one value from another collection.

- Added `prop_perturb` strategy combinator.

- Added `strategy::check_strategy_sanity()` function to do sanity checks on the
  shrinking implementation of a strategy.

- Added `prop_shuffle` strategy combinator.

- Added `strategy::Fuse` adaptor.

### Bug Fixes

- Fix bug where `Vec`, array and tuple shrinking could corrupt the state of
  their inner values, for example leading to out-of-range integers.

- Fix bug where `Flatten` (a.k.a. the `prop_flat_map` combinator) could fail to
  converge to a failing test case during shrinking.

- Fix `TupleUnion` sometimes panicking during shrinking if there were more than
  two choices.

## 0.3.1

### New Additions

- Added `CharStrategy::new_borrowed`.

## 0.3.0

### New Additions

- `Union` now supports weighting via `Union::new_weighted`. Corresponding
  syntax to specify weights is also available in `prop_oneof!`.

- Added `TupleUnion`, which works like `Union` but permits doing static
  dispatch even with heterogeneous delegate strategies.

- `prop_oneof!` is smarter about how it combines the input strategies.

- Added `option` module to generate weighted or unweighted `Option` types.

- Added `result` module to generate weighted or unweighted `Result` types.

- All `bits` submodules now have a `masked` function to create a strategy for
  generating subsets of an arbitrary bitmask.

### Potential Breaking Changes

- `Union::new` now has a generic argument type which could impact type
  inference.

- The concrete types produced by `prop_oneof!` have changed.

- API functions which used to return `BoxedStrategy` now return a specific
  type.

- `BitSetStrategy<T>` is no longer `Copy` for non-`Copy` types `T` nor `Debug`
  for non-`Debug` types `T`.

- `BitSetLike::max` has been renamed to `BitSetLike::len`.

## 0.2.1

### New Additions

- Added `prop_assert!` macro family to assert without panicking, for quieter
  test failure modes.

- New `prelude` module for easier importing of important things.

- Renamed `Singleton` to `Just`. (The old name is still available.)

- Failure messages produced by `proptest!` are now much more readable.

- Added in-depth tutorial.

## 0.2.0

### Breaking Changes

- `Strategy` now requires `std::fmt::Debug`.

### New Additions

- `Strategy` now has a family of `prop_flat_map()` combinators for producing
  dynamic and higher-order strategies.

- `Strategy` has a `prop_recursive()` combinator which allows generating
  recursive structures.

- Added `proptest::bool::weighted()` to pull booleans from a weighted
  distribution.

- New `prop_oneof!` macro makes it easier to select from one of several
  strategies.

- New `prop_compose!` macro to simplify writing most types of custom
  strategies.

## 0.1.1

### New Additions

Add `strategy::NoShrink`, `Strategy::no_shrink()`.
