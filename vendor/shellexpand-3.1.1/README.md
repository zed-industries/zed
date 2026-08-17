shellexpand, a library for shell-like expansion in strings
==========================================================

[![Build Status][actions]](https://gitlab.com/ijackson/rust-shellexpand/-/pipelines)
[![crates.io][crates]](https://crates.io/crates/shellexpand)
[![docs][docs]](https://docs.rs/shellexpand)

  [actions]: https://img.shields.io/gitlab/pipeline-status/ijackson/rust-shellexpand?branch=main&style=flat-square
  [crates]: https://img.shields.io/crates/v/shellexpand.svg?style=flat-square
  [docs]: https://img.shields.io/badge/docs-latest%20release-6495ed.svg?style=flat-square

[Documentation](https://docs.rs/shellexpand/)

shellexpand is a single dependency library which allows one to perform shell-like expansions in strings,
that is, to expand variables like `$A` or `${B}` into their values inside some context and to expand
`~` in the beginning of a string into the home directory (again, inside some context).

This crate provides generic functions which accept arbitrary contexts as well as default, system-based
functions which perform expansions using the system-wide context (represented by functions from `std::env`
module and [dirs](https://crates.io/crates/dirs) crate).

---

### Alternatives to this crate:

 * [`expanduser`](https://docs.rs/expanduser/latest/expanduser/):
   Tilde substitution only.
   Supports `~user` which this crate currently does not
   (although we hope to).
 * [`envsubst`](https://docs.rs/envsubst/latest/envsubst/):
   Does not do offer tildeexpansion.
   Only supports certain concrete types
   (eg `HashMap` for variable map).
 * [`expand_str`](https://crates.io/crates/expand_str):
   Uses `%..%` syntax.
   Does not offer tilde expansion.
   Variable lookups can only be infallible.
 * [`tilde_expand`](https://crates.io/crates/tilde-expand):
   Only does tilde expansion, on bytes (`[u8]`).

## Usage

Just add a dependency in your `Cargo.toml`:

```toml
[dependencies]
shellexpand = "3.0"
```

See the crate documentation (a link is present in the beginning of this readme) for more information
and examples.

### Cargo features

Functional features:

 * `tilde` (on by default): support for tilde expansion (home directory).
 * `path` (in `full`): support for operations on `Path`s.  (MSRV: 1.51)

Metafeatures:

 * `full`: all reasonable (non-experimental, non-hazardous) functionality.
   (Currently equivalent to `full-msrv-1.51`.)
 * `base-0` (on by default): basic functionality.
   You must enable this feature.
 * `full-msrv-1.51`: all reasonable functionality compatible with Rust 1.51.
   (currently: `base-0`, `tilde`, `paths`).
 * `full-msrv-1.31`: all reasonable functionality compatible with Rust 1.31.
   (currently: `base-0`, `tilde`).

At the time of writing there is no experimental or hazardous functionality;
if we introduce any it will be feature gated and not enabled by default nor part of `full*`.
Requiring `base-0` allows us to split existing functionality into a new optional feature,
without a semver break.
We will try to avoid MSRV increases for existing functionality;
increasing the MSRV for `full` will be minor version bump.

## Changelog

### Version 3.1.1 - 2025-04-14

Improved:

* MSRV (1.31.0) declared in Cargo.toml.

Added:

* Relaxed dependency on `dirs` to allow 6.x.

### Version 3.1.0 - 2023-03-24

Added:

* cargo features `full-msrv-1.31` and `full-msrv-1.51`

Fixed:

* Explicitly declared MSRV 1.51 for `paths` feature.
* Fixed build (without `paths` feature) with MSRV (1.31)

Improved:

* MSRV tested.
* Update to `dirs` 5.  (Allow use of dirs 4 too, since it's fine.)
* Update `Cargo.lock.exaple`

### Version 3.0.0 - 2022-12-01

Breaking changes:

* `tilde_with_context` and `full_with_context` now expect home directories as `str`, not `Path`.
  If you want to deal in Path, use the `path` feature and module.
* You must select at least one cargo feature.
  The `base-0` feature is equivalent to the features available in shellexpand 2.x.

Significant changes:

* Use Rust 2018, bumping MSRV to 1.31.0.
* New `path` module, and corresponding cargo feature.

### Version 2.1.2

Minor changes:

* "Un-forked": now released as `shellexpand` on crates.io.
* List alternatives to this crate.
* Switch back to dirs from dirs-next.
* Improve linking in docs and fix broken links and badges.
* Apply some proposals from `cargo fix`.

### Version 2.1.1

* Fix tilde expanding on Windows with Windows style (backslash) paths.
  Addresses <https://github.com/netvl/shellexpand/pull/13>.
* Forked as `shellexpand-fork` on crates.io.

### Version 2.1.0

* Switched to `dirs-next` instead of the obsolete `dirs` as the underlying dependency used to resolve the home directory
* Switched to GitHub Actions instead of Travis CI for building the project.

### Version 2.0.0

* Added support for default values in variable expansion (i.e. `${ANSWER:-42}`)
* Breaking changes (minimum Rust version is now 1.30.0):
  + Using `dyn` for trait objects to fix deprecation warning
  + Switched to using `source()` instead of `cause()` in the `Error` implementation, and
    therefore added a `'static` bound for the generic error parameter `E`

### Version 1.1.1

* Bump `dirs` dependency to 2.0.

### Version 1.1.0

* Changed use of deprecated `std::env::home_dir` to the [dirs](https://crates.io/crates/dirs)::home_dir function

### Version 1.0.0

* Fixed typos and minor incompletenesses in the documentation
* Changed `home_dir` argument type for tilde expansion functions to `FnOnce` instead `FnMut`
* Changed `LookupError::name` field name to `var_name`

### Version 0.1.0

* Initial release

## License

This program is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed 
as above, without any additional terms or conditions.
