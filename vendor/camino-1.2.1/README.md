# camino - UTF-8 paths

[![camino on crates.io](https://img.shields.io/crates/v/camino)](https://crates.io/crates/camino)
[![crates.io download count](https://img.shields.io/crates/d/camino)](https://crates.io/crates/camino)
[![Documentation (latest release)](https://img.shields.io/badge/docs-latest%20version-brightgreen.svg)](https://docs.rs/camino)
[![Documentation (main)](https://img.shields.io/badge/docs-main-purple.svg)](https://camino-rs.github.io/camino/rustdoc/camino/)
[![License](https://img.shields.io/badge/license-Apache-green.svg)](LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE-MIT)

This repository contains the source code for `camino`, an extension of the `std::path` module that adds new
[`Utf8PathBuf`] and [`Utf8Path`] types.

## What is camino?

`camino`'s [`Utf8PathBuf`] and [`Utf8Path`] types are like the standard library's [`PathBuf`] and [`Path`] types, except
they are guaranteed to only contain UTF-8 encoded data. Therefore, they expose the ability to get their
contents as strings, they implement `Display`, etc.

The `std::path` types are not guaranteed to be valid UTF-8. This is the right decision for the standard library,
since it must be as general as possible. However, on all platforms, non-Unicode paths are vanishingly uncommon for a
number of reasons:

- Unicode is now the dominant encoding for file names. There are still some legacy codebases that store paths in encodings like [Shift JIS], but most have been converted to Unicode at this point.
- Unicode is the common subset of supported paths across Windows and Unix platforms. (On Windows, Rust stores paths
  as [an extension to UTF-8](https://simonsapin.github.io/wtf-8/), and converts them to UTF-16 at Win32
  API boundaries.)
- There are already many systems, such as Cargo, that only support UTF-8 paths. If your own tool interacts with any such
  system, you can assume that paths are valid UTF-8 without creating any additional burdens on consumers.
- The ["makefile problem"](https://www.mercurial-scm.org/wiki/EncodingStrategy#The_.22makefile_problem.22) asks: given a
  Makefile or other metadata file (such as `Cargo.toml`) that lists the names of other files, how should the names in
  the Makefile be matched with the ones on disk? This has _no general, cross-platform solution_ in systems that support
  non-UTF-8 paths. However, restricting paths to UTF-8 eliminates this problem.

[Shift JIS]: https://en.wikipedia.org/wiki/Shift_JIS

Therefore, many programs that want to manipulate paths _do_ assume they contain UTF-8 data, and convert them to `str`s
as necessary. However, because this invariant is not encoded in the `Path` type, conversions such as
`path.to_str().unwrap()` need to be repeated again and again, creating a frustrating experience.

Instead, `camino` allows you to check that your paths are UTF-8 _once_, and then manipulate them
as valid UTF-8 from there on, avoiding repeated lossy and confusing conversions.

## Examples

The documentation for [`Utf8PathBuf`] and [`Utf8Path`] contains several examples.

For examples of how to use `camino` with other libraries like `serde` and `clap`, see the [`camino-examples`] directory.

## API design

`camino` is a very thin wrapper around `std::path`. [`Utf8Path`] and [`Utf8PathBuf`] are drop-in replacements
for [`Path`] and [`PathBuf`].

Most APIs are the same, but those at the boundary with `str` are different. Some examples:

- `Path::to_str() -> Option<&str>` has been renamed to `Utf8Path::as_str() -> &str`.
- [`Utf8Path`] implements `Display`, and `Path::display()` has been removed.
- Iterating over a [`Utf8Path`] returns `&str`, not `&OsStr`.

Every [`Utf8Path`] is a valid [`Path`], so [`Utf8Path`] implements `AsRef<Path>`. Any APIs that accept `impl AsRef<Path>`
will continue to work with [`Utf8Path`] instances.

## Should you use camino?

`camino` trades off some utility for a great deal of simplicity. Whether `camino` is appropriate for a project or not
is ultimately a case-by-case decision. Here are some general guidelines that may help.

_You should consider using camino if..._

- **You're building portable, cross-platform software.** While both Unix and Windows platforms support different kinds
  of non-Unicode paths, Unicode is the common subset that's supported across them.
- **Your system has files that contain the names of other files.** If you don't use UTF-8 paths, you will run into the
  makefile problem described above, which has no general, cross-platform solution.
- **You're interacting with existing systems that already assume UTF-8 paths.** In that case you won't be adding any new
  burdens on downstream consumers.
- **You're building something brand new and are willing to ask your users to rename their paths if necessary.** Projects
  that don't have to worry about legacy compatibility have more flexibility in choosing what paths they support.

In general, using camino is the right choice for most projects.

_You should **NOT** use camino, if..._

- **You're writing a core system utility.** If you're writing, say, an `mv` or `cat` replacement, you should
  **not** use camino. Instead, use [`std::path::Path`] and add extensive tests for non-UTF-8 paths.
- **You have legacy compatibility constraints.** For example, Git supports non-UTF-8 paths. If your tool needs to handle
  arbitrary Git repositories, it should use its own path type that's a wrapper around `Vec<u8>`.
  - [`std::path::Path`] supports arbitrary bytestrings [on Unix] but not on Windows.
- **There's some other reason you need to support non-UTF-8 paths.** Some tools like disk recovery utilities need to
  handle potentially corrupt filenames: only being able to handle UTF-8 paths would greatly diminish their utility.

[on Unix]: https://doc.rust-lang.org/std/os/unix/ffi/index.html

## Optional features

By default, `camino` has **no dependencies** other than `std`. There are some optional features that enable
dependencies:

- `serde1` adds serde [`Serialize`] and [`Deserialize`] impls for [`Utf8PathBuf`] and [`Utf8Path`]
  (zero-copy).
- `proptest1` adds [proptest](https://altsysrq.github.io/proptest-book/) [`Arbitrary`]
  implementations for [`Utf8PathBuf`] and `Box<Utf8Path>`.

## Rust version support

The minimum supported Rust version (MSRV) for `camino` with default features is **1.61**. This project is tested in CI
against the latest stable version of Rust and the MSRV.

- _Stable APIs_ added in later Rust versions are supported either through conditional compilation in `build.rs`, or through backfills that also work on older versions.
- _Deprecations_ are kept in sync with the version of Rust they're added in.
- _Unstable APIs_ are currently not supported. Please
  [file an issue on GitHub](https://github.com/camino-rs/camino/issues/new) if you need an unstable API.

`camino` is designed to be a core library and has a conservative MSRV policy. MSRV increases will only happen for
a compelling enough reason, and will involve at least a minor version bump.

Optional features may pull in dependencies that require a newer version of Rust.

## License

This project is available under the terms of either the [Apache 2.0 license](LICENSE-APACHE) or the [MIT
license](LICENSE-MIT).

This project's documentation is adapted from [The Rust Programming Language](https://github.com/rust-lang/rust/), which is
available under the terms of either the [Apache 2.0 license](https://github.com/rust-lang/rust/blob/master/LICENSE-APACHE)
or the [MIT license](https://github.com/rust-lang/rust/blob/master/LICENSE-MIT).

[`Utf8PathBuf`]: https://docs.rs/camino/*/camino/struct.Utf8PathBuf.html
[`Utf8Path`]: https://docs.rs/camino/*/camino/struct.Utf8Path.html
[`PathBuf`]: https://doc.rust-lang.org/std/path/struct.PathBuf.html
[`Path`]: https://doc.rust-lang.org/std/path/struct.Path.html
[`std::path::Path`]: https://doc.rust-lang.org/std/path/struct.Path.html
[`Serialize`]: https://docs.rs/serde/1/serde/trait.Serialize.html
[`Deserialize`]: https://docs.rs/serde/1/serde/trait.Deserialize.html
[`camino-examples`]: https://github.com/camino-rs/camino/tree/main/camino-examples
[`Arbitrary`]: https://docs.rs/proptest/1/proptest/arbitrary/trait.Arbitrary.html
