# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.12] - 2026-07-29

### Changed

- Bump `linktime-proc-macro` dependency to 0.2.1 (makes link-section names more stable).

## [1.0.11] - 2026-07-26

### Fixed

- Pointer-align the priority ctor anchor on Apple platforms (#502).

## [1.0.10] - 2026-07-21

### Changed

 - Faster atomics and #[cold] for `Static::deref` initialization (@yvt PR #501)

## [1.0.9] - 2026-07-15

### Fixed

- Ensure the single executing priority ctor is kept alive on Apple platforms (#496).

## [1.0.8] - 2026-07-06

### Changed

- Bump `link-section` dependency to 0.19.0.

## [1.0.7] - 2026-05-28

### Changed

- Bump `link-section` dependency to 0.18.0.
- Improve error messages for bad attribute values.

## [1.0.6] - 2026-05-16

### Changed

- Bump `link-section` dependency to 0.17.0.
- MSRV bumped to 1.85.0 (if `priority` feature is enabled), otherwise remains at
  1.60.0.
  - To restore MSRV to 1.60.0, use `ctor = { version = "1.0.6", default-features
    = false, features = ["proc_macro", "std"] }` in your Cargo.toml.

### Fixed

- `#[ctor]` requires significantly less macro recursion.

## [1.0.5] - 2026-05-11

### Changed

- Repair MSRV breakage for WASM targets.

## [1.0.4] - 2026-05-08

### Added

- Support for win7, uwp and other windows targets
- Support for UEFI targets (`.init_array` matching gnu-efi)
- Fallback to `.init_array` for unsupported targets (with a warning)

### Changed

- `target_vendor = "pc"` is now `target_os = "windows"`. This should be a
strictly increasing set of support for windows targets.

## [1.0.3] - 2026-05-07

### Fixed

- A `default` priority value is now supported and is the default. This is
  equivalent to a priority of 500 on most platforms. The previous `priority`
  default of 0 was unsafe for C library access on some platforms.
- The `early` priority value is now equivalent to a priority of 101 on most platforms.

## [1.0.2] - 2026-05-06

### Changed

- Bump `link-section` dependency to 0.15.0.
- Use `const` rather than `static` items for collected constructors on macOS.
- Allow various forms of `&'static` for `#[ctor]` statics which desugar to
  `&'static Static`.
- Static items delegate `Display` directly as well.
- Support for multiple `#[ctor]` items in a single `#[ctor]` block:
```rust
#[ctor]
static CTOR: &[fn()] = const {
  // ...
}
```

## [1.0.1] - 2026-05-04

### Changed

- `wasm32-unknown-unknown` would run ctor items on each call of an exported
  function.

## [1.0.0] - 2026-05-03

### Changed

- Stabilized (yay!). Identical to 0.13.1. Thanks to all the contributors who
  helped along the way! Please file any upgrade issues in
  <https://github.com/mmastrac/linktime/issues>.
- For those upgrading from earlier versions, the major changes for you to note
  are:
    - `dtor` was split into the `dtor` crate. You'll need to add it to your
      dependencies like so:
      ```toml
      [dependencies]
      dtor = "1.0.0" # or later
      ```
    - `#[ctor(unsafe)]` is now required for `#[ctor]` items. If you are building a
      binary, you can use `RUSTFLAGS="--cfg linktime_no_fail_on_missing_unsafe"`
      (or alternatively, specify this in your `config.toml` file) to bypass the
      error.
    - For those re-exporting `ctor` from their own crates: the
      `ctor::declarative::ctor!` macro should be preferred over
      `#[ctor(crate_path = ...)]`. The latter form will continue to work, but
      the declarative macro is far more stable for most use cases. See
      <https://docs.rs/ctor/latest/ctor/declarative/macro.ctor.html> for more
      details.

## [0.13.1] - 2026-05-02

### Changed

- Crate examples were reorganized.

### Fixed

- Documentation fixes (`--cfg` flags were incorrect).
- Incorrect crate feature in docs.

## [0.13.0] - 2026-05-02

### Changed

- `#[ctor(priority = naked)]` is now `#[ctor(naked)]`.
- `unsafe` is now required for `#[ctor]` items and the
  `no_warn_on_missing_unsafe` feature is gone.
  - `RUSTFLAGS="--cfg linktime_no_fail_on_missing_unsafe"` can bypass the error.
- `used_linker` feature moved to `--cfg linktime_used_linker` flag.

### Added

- Re-added link section option for body of `#[ctor]` items (supported for Linux/Android/FreeBSD/Apple).

## [0.12.0] - 2026-04-30

### Added

- Support for `#[ctor]` on `impl` items. To be valid, the `fn` must have no
  `self` parameter and must not access any generic parameters from the outer
  item.
- Added `life before main` documentation to all crates.
- `early` and `late` priority values are now supported on all platforms.

### Removed

- Deprecated `dtor` feature and crate dependency from `ctor` crate (use the
  `dtor` crate directly).

### Fixed

- AIX uses "standard" priority values from 0 to 999, early and late (mapped to
  80000000 to 80000999).

### Changed

- If the `priority` feature is enabled, `ctor` priority sorting is now stable
  and consistent across platforms: `early`/`0`/`unspecified`, then `1 <= N <
  1000`, then `late`.
- If a `link_section` or `export_name_prefix` is specified, a `priority` value
  must not be specified (now a compiler error).
- Migrated to using the `linktime-proc-macro` crate for proc-macro support.

## [0.11.1] - 2026-04-28

### Changed

- Deprecated `dtor` macros in favor of the `dtor` crate.
- Migrated to using the `linktime-proc-macro` crate for proc-macro support.

### Fixed

- Fixed some stray `dtor` references in ctor docs.

## [0.11.0] - 2026-04-28

### Added

- AIX support for `ctor`/`dtor` crates.

### Changed

- Significant rewrite to ctor/dtor macros and documentation.
- Macro attributes and crate features are auto-documented.
- Rewrote `statics` code in `ctor` to not require `std`.

## [0.10.1] - 2026-04-22

### Added

- Included licenses in all files.
- Bumped proc-macro dependency versions.

### Fixed

- Fix MSRV in ctor docs.
- Various hardening fixes under Miri.
- Adding priority to `ctor`s accidentally enabled the anonymous flag.

### Changed

- `ctor` exports all `dtor` macros from `dtor` crate rather than reimplementing them.
