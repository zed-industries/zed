# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

See the [upgrading guide][] for more detailed information about
modifying code to account for new releases.

[upgrading guide]: https://docs.rs/snafu/*/snafu/guide/upgrading/index.html

## [0.8.9] - 2025-09-03

### Fixed

- Allow the `unstable-provider-api` feature to be used in a no_std
  environment, both with and without the `alloc` feature flag enabled.

[0.8.9]: https://github.com/shepmaster/snafu/releases/tag/0.8.9

## [0.8.8] - 2025-08-28

### Fixed

- Gate `TryFuture::whatever_context`,
  `TryFuture::with_whatever_context`, `TryStream::whatever_context`,
  and `TryStream::with_whatever_context` behind the `alloc` feature
  flag. This allows SNAFU's future and stream combinators to be used
  in a no_std environment.

- Only implement `GenerateImplicitData` and `AsBacktrace` for `Arc`
  wrappers when the target platform has access to atomics and `Arc`.

[0.8.8]: https://github.com/shepmaster/snafu/releases/tag/0.8.8

## [0.8.7] - 2025-08-20

### Added

- `GenerateImplicitData` and `AsBacktrace` are now implemented for
  `Box<T>`, `Rc<T>`, and `Arc<T>` when `T` implements the same
  trait. This allows you to use types like `Arc<Backtrace>` as a
  backtrace field.

[0.8.7]: https://github.com/shepmaster/snafu/releases/tag/0.8.7

## [0.8.6] - 2025-05-28

### Added

- A new `alloc` feature flag has been added as a subset of the
  existing `std` feature flag. This allows SNAFU to be used in
  environments that have access to an allocator but not the standard
  library.

- Context selectors may now be explicitly named. This allows
  flexibility in edge cases, but also is understood by rust-analyzer
  to help with renaming context selectors.

### Fixed

- Added `#[allow(dead_code)]` to the context selector's `fail`
  method. This works around an issue with the `dead_code` lint in rare
  usages involving declarative macros and explicit visibility
  qualifiers.

### Changed

- Add top-level documentation on reporting errors using
  `#[snafu::report]`.

- Show the `cfg` options that control features on docs.rs.

[0.8.6]: https://github.com/shepmaster/snafu/releases/tag/0.8.6

## [0.8.5] - 2024-09-23

### Added

- Implement the `core::error::Error` trait instead of
  `std::error::Error` when the `rust_1_81` feature flag is
  enabled. This supersedes the `unstable-core-error` feature flag.

[0.8.5]: https://github.com/shepmaster/snafu/releases/tag/0.8.5

## [0.8.4] - 2024-07-08

### Fixed

- Use the fully-qualified path to `Location` in the `location!()`
  macro.

[0.8.4]: https://github.com/shepmaster/snafu/releases/tag/0.8.4

## [0.8.3] - 2024-05-24

### Changed

- `Report` now prints an "Error: " prefix when returned directly from
  `main` or tests.

- Support heck version 0.5 in addition to 0.4.

[0.8.3]: https://github.com/shepmaster/snafu/releases/tag/0.8.3

## [0.8.2] - 2024-03-11

### Fixed

- Adding a custom Whatever variant to a generic error enum is now
  supported.

- The `#[report]` macro works better on functions that require more
  complicated type inference for their return types.

[0.8.2]: https://github.com/shepmaster/snafu/releases/tag/0.8.2

## [0.8.1] - 2024-02-26

### Added

- Error types that use constant generics are now supported.

[0.8.1]: https://github.com/shepmaster/snafu/releases/tag/0.8.1

## [0.8.0] - 2023-12-28

### Added

- `snafu(transparent)` allows creating compound error types that act
  as if they were not present, delegating thier `Display` and `Error`
  implementations to the source error. This is useful when aggregating
  smaller errors that already completely explain the failure.

- `ResultExt::boxed` and `ResultExt::boxed_local` are available to
  convert an error value into an owned trait object. This is useful
  when an error type is a generic controlled by the caller.

### Changed

- Rust 1.56 is now the *minimum* supported Rust version. This is a
  **breaking change**.

- Rust 1.65 is now the *default* supported Rust version. This is a
  **breaking change**.

- The item type of the `ChainCompat` iterator is now `&'a (dyn Error +
  'b)` to allow downcasting the error trait object to a concrete
  type. This is a **breaking change**.

- Error fields marked `location` are no longer automatically
  implicitly created. This is a **breaking change**.

- Adding `#[snafu]` attributes to the field of a tuple struct are now
  errors. This is a **breaking change**.

- The SNAFU copy of the `Error` trait now marks the `description` and
  `cause` methods as deprecated, following the standard library's
  example. This trait is only active when using SNAFU in a no_std
  environment and no functional difference is intended.

### Removed

- The default `Display` implementation no longer includes the error
  text of the source error. This is a **breaking change**.

- The `backtraces` and `unstable-backtraces-impl-std` feature flags
  have been removed. This is a **breaking change**. The `Backtrace`
  type is now the standard library's `Backtrace` type when it is
  available.

[0.8.0]: https://github.com/shepmaster/snafu/releases/tag/0.8.0

## [0.7.5] - 2023-07-09

### Added

- The `CleanedErrorText` iterator can be used to remove redundant
  parts from the messages that many errors duplicate from their
  underlying sources. This is the same behavior as `Report`, but can
  be used in contexts other than console output, such as when
  reporting an error in structured output like HTML or JSON.

### Fixed

- The documentation for the compatability feature flags now mentions
  `rust_1_39` and shows the correct default state of `rust_1_61`.

[0.7.5]: https://github.com/shepmaster/snafu/releases/tag/0.7.5

## [0.7.4] - 2022-12-19

### Changed

- `Report` and the `#[report]` macro now remove redundant parts from
  the messages that many errors duplicate from their underlying
  sources.

[0.7.4]: https://github.com/shepmaster/snafu/releases/tag/0.7.4

## [0.7.3] - 2022-10-20

### Fixed

- The macro no longer generates invalid code when implicitly-generated
  types (such as backtraces) were used in conjunction with
  `#[snafu(source(from))]` and the type before transformation does not
  implement `std::error::Error`.

[0.7.3]: https://github.com/shepmaster/snafu/releases/tag/0.7.3

## [0.7.2] - 2022-10-09

### Added

- `Report` can be returned from `main` or test functions to provide a
  user-friendly display of errors.

- A cheat sheet for the most common `#[snafu(...)]` attribute usages
  has been added to the `Snafu` macro's documentation.

- Optional support for using the standard library's
  `std::backtrace::Backtrace` type via the `backtraces-impl-std`
  feature flag.

- Optional support for implementing the Provider API using the
  `std::error::Error::provide` method via the `unstable-provider-api`
  feature flag.

- Optional support for implementing the `core::error::Error` trait
  instead of `std::error::Error` via the `unstable-core-error` feature
  flag.

- `GenerateImplicitData` has a new method `generate_with_source`.

### Changed

- `ErrorCompat::iter_chain` and `ChainCompat` are now available in
  no_std environments.

- `ChainCompat` now implements `Clone`.

- The `Debug` implementation for `Location` no longer shows some
  irrelevant internal details.

[0.7.2]: https://github.com/shepmaster/snafu/releases/tag/0.7.2

## [0.7.1] - 2022-05-02

### Added

- The macro `ensure_whatever` provides the functionality of the
  `ensure` macro for stringly-typed errors.

### Changed

- No longer require the `futures` feature flag to support the shim
  implementations of standard library errors that have existed since
  Rust 1.34.

- Documentation improved to demonstrate that custom Whatever errors
  can optionally be made `Send` and `Sync`.

[0.7.1]: https://github.com/shepmaster/snafu/releases/tag/0.7.1

## [0.7.0] - 2022-01-03

Many breaking changes in this release can be automatically addressed
with the [snafu-upgrade-assistant][].

[snafu-upgrade-assistant]: https://github.com/shepmaster/snafu-upgrade-assistant

### Added

- A crate prelude containing common macros and traits can be imported
  via `use snafu::prelude::*`.

- A ready-to-use error type `Whatever` is available to quickly start
  reporting errors with little hassle.

- "Stringly typed" error cases can be added to existing error types,
  allowing you to construct errors without defining them first.

- Formatting shorthand syntax for error type data fields is now supported:
  `#[snafu(display("Hello {name}"))]`.

- `#[snafu(module)]` can be specified on an error type. This will
  create a module for the error type and all associated context
  selectors will be placed in that module.

- `snafu::Location` can be added to an error type to provide
  lightweight tracking of the source location where the error was
  created.

- `#[snafu(implicit)]` can be specified on context selector data fields
  to automatically generate it via `snafu::GenerateImplicitData` when
  the error is created.

- `ErrorCompat::iter_chain` provides an iterator over the list of
  causal errors.

### Changed

- Generated context selectors now have the suffix `Snafu`. This is a
  **breaking change**.

- `ResultExt::with_context`, `TryFutureExt::with_context`, and
  `TryStreamExt::with_context` now pass the error into the
  closure. This is a **breaking change**.

- The `GenerateBacktrace` trait has been split into
  `GenerateImplicitData` and `AsBacktrace`. This is a **breaking
  change**.

- Rust 1.34 is now the minimum supported Rust version. This is a
  **breaking change**.

### Removed

- String attribute parsing (`#[snafu(foo = "...")]`) is no longer
  supported. This is a **breaking change**.

- The deprecated `eager_context` and `with_eager_context` methods have
  been removed. This is a **breaking change**.

[0.7.0]: https://github.com/shepmaster/snafu/releases/tag/0.7.0

## [0.6.10] - 2020-12-03

### Fixed

- `ensure!` now uses a fully-qualified path to avoid a name clash when
  the path `core` is ambiguous.

[0.6.10]: https://github.com/shepmaster/snafu/releases/tag/0.6.10

## [0.6.9] - 2020-09-21

### Added

- `#[derive(Snafu)]` is now supported on unit structs and structs with fields.
- `ensure!` now supports trailing commas.

### Fixed

- The error text for a misuse of `#[snafu(context)]` was corrected.
- More usages of `Option` in the generated code are now fully qualified.

[0.6.9]: https://github.com/shepmaster/snafu/releases/tag/0.6.9

## [0.6.8] - 2020-05-11

### Fixed

- The code generated by the `Snafu` macro no longer conflicts with a
  local module called `core` or `snafu`.

[0.6.8]: https://github.com/shepmaster/snafu/releases/tag/0.6.8

## [0.6.7] - 2020-05-03

### Added

- Demonstration error types are now present in the guide.
- The user's guide is now an optional feature flag. To preserve
  compatibility, it is enabled by default, but most users can disable
  it.
- It is now possible to import the `snafu` crate under a different
  name using `#[snafu(crate_root)]`.

[0.6.7]: https://github.com/shepmaster/snafu/releases/tag/0.6.7

## [0.6.6] - 2020-04-05

### Added

- Context selectors without an underlying cause now have a `build`
  method in addition to the existing `fail` method. `build` creates
  the error but does not wrap it in a `Result`.

[0.6.6]: https://github.com/shepmaster/snafu/releases/tag/0.6.6

## [0.6.5] - 2020-04-05

- This version was a failed publish; please use 0.6.6 instead.

[0.6.4]: https://github.com/shepmaster/snafu/releases/tag/0.6.4

## [0.6.4] - 2020-04-05

- This version was a failed publish; please use 0.6.6 instead.

[0.6.4]: https://github.com/shepmaster/snafu/releases/tag/0.6.4

## [0.6.3] - 2020-03-18

### Fixed

- License files are now included with the snafu-derive package.

[0.6.3]: https://github.com/shepmaster/snafu/releases/tag/0.6.3

## [0.6.2] - 2020-01-17

### Added

- Automatically-generated code no longer triggers the
  `single_use_lifetimes` lint.

[0.6.2]: https://github.com/shepmaster/snafu/releases/tag/0.6.2

## [0.6.1] - 2020-01-07

### Added

- It is now possible to create errors that have no context using
  `#[snafu(context(false))]`. This allows using the question mark
  operator without calling `.context(...)`.

### Fixed

- Reduced the possibility for a name collision when implementing
  `Display` when a formatted value was called `f`.

[0.6.1]: https://github.com/shepmaster/snafu/releases/tag/0.6.1

## [0.6.0] - 2019-11-07

### Added

- Optional support for using the unstable `std::backtrace::Backtrace`
  type and implementing `std::error::Error::backtrace` via the
  `unstable-backtraces-impl-std` feature flag.
- Error variants can now use `Option<Backtrace>` for the `backtrace`
  field. `Backtrace` will always have the backtrace collected, while
  `Option<Backtrace>` requires that an environment variable be set.
- Basic support for no-std environments.
- The `ensure!` macro now allows creating opaque errors.
- Context selectors have basic documentation generated. This allows
  using `#[deny(missing_docs)]`.

### Changed

- Rust 1.31 is now the minimum supported Rust version. This is a
  **breaking change**.
- The `Backtrace` type is now always available, but does nothing by
  default. It is recommended that the end application enables
  backtrace functionality. This is a **breaking change**.
- Support for `std::future::Future` has been stabilized, which means
  the feature flag has been renamed from `unstable-futures` to
  `futures`. This is a **breaking change**.
- The `backtrace-crate` feature flag has been renamed to
  `backtraces-impl-backtrace-crate`. Enabling this flag now *replaces*
  `snafu::Backtrace` with `backtrace::Backtrace`. The `AsRef`
  implementation has been removed. This is a **breaking change**.
- A new trait for constructing backtraces is used instead of `Default`
  so the `Backtrace` type no longer implements `Default` or has any
  inherent methods. This is a **breaking change**.

[0.6.0]: https://github.com/shepmaster/snafu/releases/tag/0.6.0

## [0.5.0] - 2019-08-26

### Added

- Compiler errors are generated when SNAFU attributes are used in
  incorrect locations. This is a **breaking change**.
- Compiler errors are generated when SNAFU attributes are
  duplicated. This is a **breaking change**.

### Changed

- `#[snafu(source(from))` implies `#[snafu(source)]` (which implies
  `#[snafu(source(true))]`); `#[snafu(source)]` and
  `#[snafu(source(true))]` can be removed in these cases.

### Fixed

- Multiple attributes can be specified inside of a single `#[snafu(...)]`.

### Removed

- `#[snafu(backtrace(delegate))]` on source fields is replaced by
  `#[snafu(backtrace)]`. This is a **breaking change**.

[0.5.0]: https://github.com/shepmaster/snafu/releases/tag/0.5.0

## [0.4.4] - 2019-08-07

### Fixed

- Ignore `#[doc]` attributes that do not correspond to documentation
  comments. This allows `#[doc(hidden)]` to be used again.

### Changed

- Implement `Future` and `Stream` instead of `TryFuture` and
  `TryStream` for the combinators for the standard library's
  futures. This allows the `Context` future combinator to be directly
  used with `.await` and for the `Context` stream combinator to be
  used without calling `.into_stream`.

[0.4.4]: https://github.com/shepmaster/snafu/releases/tag/0.4.4

## [0.4.3] - 2019-07-23

### Added

- Add optional conversion of `&snafu::Backtrace` into `&backtrace::Backtrace`.

### Fixed

- Support default generic parameters on error types.

[0.4.3]: https://github.com/shepmaster/snafu/releases/tag/0.4.3

## [0.4.2] - 2019-07-21

### Added

- Documentation comment summaries are used as the default `Display` text.

### Fixed

- Quieted warnings from usages of bare trait objects.
- The `From` trait is fully-qualified to avoid name clashes.

### Changed

- More errors are reported per compilation attempt.

[0.4.2]: https://github.com/shepmaster/snafu/releases/tag/0.4.2

## [0.4.1] - 2018-05-18

### Fixed

- A feature flag name was rejected by crates.io and needed to be
  updated; this release has no substantial changes beyond 0.4.0.

[0.4.1]: https://github.com/shepmaster/snafu/releases/tag/0.4.1

## [0.4.0] - 2018-05-18

### Added

- Context selectors now automatically implement `Debug`, `Copy`, and
  `Clone`. This is a **breaking change**.

- Support for futures 0.1 futures and streams is available using the
  `futures-01` feature flag.

- **Experimental** support for standard library futures and streams is
  available using the `unstable-futures` feature flag.

### Deprecated

- `eager_context` and `with_eager_context` have been deprecated.

### Removed

- The `Context` type is no longer needed. This is a **breaking
  change**.

- SNAFU types no longer implement `Borrow<std::error::Error>`. This is
  a **breaking change**.

[0.4.0]: https://github.com/shepmaster/snafu/releases/tag/0.4.0

## [0.3.1] - 2019-05-10

### Fixed

- Underlying error causes of `Box<dyn std::error::Error + Send +
  Sync>` are now supported.

### Deprecated

- `Borrow` is no longer required to be implemented for underlying
  error causes. In the next release containing breaking changes, the
  automatic implementation of `Borrow<dyn std::error::Error>` for
  SNAFU types will be removed.

[0.3.1]: https://github.com/shepmaster/snafu/releases/tag/0.3.1

## [0.3.0] - 2019-05-08

### Added

- `Borrow<std::error::Error>` is now automatically implemented for
  SNAFU types. This is a **breaking change** as it may conflict with
  an existing user implementation of the same trait. It is expected
  that the number of affected users is very small.

- `#[snafu(source)]` can be used to identify the field that
  corresponds to the underlying error if it is not called `source`. It
  can also be used to disable automatically using a field called
  `source` for the underlying error.

- `#[snafu(backtrace)]` can be used to identify the field that
  corresponds to the backtrace if it is not called `backtrace`. It can
  also be used to disable automatically using a field called
  `backtrace` for the backtrace.

- `#[snafu(source(from(...type..., ...expression...)))]` can be used
  to perform transformations on the underlying error before it is
  stored. This allows boxing of large errors to avoid bloated return
  types or recursive errors.

- The user guide has a basic comparison to Failure and migration paths
  for common Failure patterns.

### Changed

- The default `Display` implementation includes the underlying error
  message.

[0.3.0]: https://github.com/shepmaster/snafu/releases/tag/0.3.0

## [0.2.3] - 2019-04-24

### Fixed

- User-provided `where` clauses on error types are now copied to
  SNAFU-created `impl` blocks.
- User-provided inline trait bounds (`<T: SomeTrait>`) are no longer
  included in SNAFU-generated type names.

[0.2.3]: https://github.com/shepmaster/snafu/releases/tag/0.2.3

## [0.2.2] - 2019-04-19

### Fixed

- Error enums with variants named `Some` or `None` no longer cause
  name conflicts in the generated code.

[0.2.2]: https://github.com/shepmaster/snafu/releases/tag/0.2.2

## [0.2.1] - 2019-04-14

### Added

- Deriving `Snafu` on a newtype struct now creates an opaque error
  type, suitable for conservative public APIs.

[0.2.1]: https://github.com/shepmaster/snafu/releases/tag/0.2.1

## [0.2.0] - 2019-03-02

### Removed

- `snafu::display` and `snafu_display` have been replaced with `snafu(display)`
- `snafu_visibility` has been replaced with `snafu(visibility)`

### Added

- Backtraces can now be delegated to an underlying error via
  `#[snafu(backtrace(delegate))]`.

[0.2.0]: https://github.com/shepmaster/snafu/releases/tag/0.2.0

## [0.1.9] - 2019-03-02

### Added

- Error enums with generic lifetimes and types are now supported.

### Changed

- The trait bounds applied to the `fail` method have been moved from
  the implementation block to the function itself.

[0.1.9]: https://github.com/shepmaster/snafu/releases/tag/0.1.9

## [0.1.8] - 2019-02-27

### Fixed

- Visibility is now applied to context selector fields.

[0.1.8]: https://github.com/shepmaster/snafu/releases/tag/0.1.8

## [0.1.7] - 2019-02-27

### Added

- `#[snafu_visibility]` can be used to configure the visibility of
  context selectors.

[0.1.7]: https://github.com/shepmaster/snafu/releases/tag/0.1.7

## [0.1.6] - 2019-02-24

### Added

- The `OptionExt` extension trait is now available for converting
  `Option`s into `Result`s while adding context.

[0.1.6]: https://github.com/shepmaster/snafu/releases/tag/0.1.6

## [0.1.5] - 2019-02-05

### Changed

- Errors from the macro are more detailed and point to reasonable
  sections of code.

[0.1.5]: https://github.com/shepmaster/snafu/releases/tag/0.1.5

## [0.1.4] - 2019-02-05

### Added

- The `ensure` macro is now available.

[0.1.4]: https://github.com/shepmaster/snafu/releases/tag/0.1.4

## [0.1.3] - 2019-02-04

### Added

- Ability to automatically capture backtraces.

### Changed

- Version requirements for dependencies loosened to allow compiling
  with more crate versions.

[0.1.3]: https://github.com/shepmaster/snafu/releases/tag/0.1.3

## [0.1.2] - 2019-02-02

### Added

- Support for Rust 1.18

[0.1.2]: https://github.com/shepmaster/snafu/releases/tag/0.1.2

## [0.1.1] - 2019-02-01

### Added

- Context selectors without an underlying source now have a `fail`
  method.

- `ResultExt` now has the `eager_context` and `with_eager_context`
   methods to eagerly convert a source `Result` into a final `Result`
   type, skipping the intermediate `Result<_, Context<_>>` type.

[0.1.1]: https://github.com/shepmaster/snafu/releases/tag/0.1.1

## [0.1.0] - 2019-01-27

Initial version

[0.1.0]: https://github.com/shepmaster/snafu/releases/tag/0.1.0
