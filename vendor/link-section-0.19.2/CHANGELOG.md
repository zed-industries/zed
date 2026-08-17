# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.19.2] - 2026-07-29

### Changed

- Bump `linktime-proc-macro` dependency to 0.2.1 (makes link-section names more stable).

## [0.19.1] - 2026-07-15

### Fixed

- Fix for semicolons accidentally inserted in WASM expression macro (generates a
  warning on nightly).

## [0.19.0] - 2026-07-06

### Fixed

- WASM: item counts no longer collapse under fat LTO. Items are gathered through
  address-significant intrusive list nodes that survive LTO. Fixes
  [#488](https://github.com/mmastrac/linktime/issues/488).

### Changed

- WASM: the `read_custom_section` host import is no longer required. Sections
  are gathered in-module via `.init_array.0` submissions and an `.init_array.1`
  finalizer; the `#[ctor]` priority contract is unchanged.
- WASM: each item now lives in a single intrusive `LinkCell<T>` static (value
  plus trailing list metadata) instead of a separate value static and cell.

### Removed

- `Ref::as_ptr` and `MovableRef::as_ptr` are now `pub(crate)`. They exposed raw
  pointer plumbing that was never meant to be public API; use `Deref` (or
  `offset_of` for section-membership checks) instead. This is a breaking change.

## [0.18.3] - 2026-06-23

### Fixed

- Windows/COFF: section start/end pointers are now opacified with `black_box`,
  fixing a miscompile where an optimizing (LTO) build could prove a gathered
  slice (e.g. `ScatteredSlice<&'static T>`) reads as all-zero and fold it to
  null values, crashing on use (`0xC0000005`). Fixes
  [#479](https://github.com/mmastrac/linktime/issues/479).

## [0.18.2] - 2026-06-06

### Changed

- Empty sections are now automatically supported on all platforms other than AIX.

## [0.18.1] - 2026-05-30

### Changed

- Fixed WASM import module for `read_custom_section` and `atexit`.

## [0.18.0] - 2026-05-28

### Changed

- `proc_macro` feature is now required for any non-`unsafe` section.
- Non-`unsafe` sections now generate a unique ID for the submission site which
allows multiple sections to share the same identifier.

## [0.17.2] - 2026-05-18

### Changed

- Fixed bug with submission of raw items to `aux` sections.

## [0.17.1] - 2026-05-17

### Added

- `MovableRef` and `Ref` implement `Debug` and `Display` if the referenced type does.

### Changed

- Deprecated `reference` module in favor of `Ref` from the crate root.

## [0.17.0] - 2026-05-16

### Added

- `reference` sections support access both as a slice and as a reference at the
  submission site on all platforms (including WASM).
- `movable` sections support reordering and back-reference updates during
  startup initialization (allowing a "reference" to an item that may move during
  initialization at the submission site).
- `typed` and `mutable` sections have been split: `typed` allows for `const` and
  `static` items, while `mutable` allows for `const` items and `as_mut_slice`
  access.

### Changed

- Bumped MSRV to 1.85.0.
- Significant rewrite to link-section's internal implementation.
- Sections require a type: `#[section(typed)]`, `#[section(untyped)]`,
  `#[section(mutable)]`, `#[section(movable)]`, or `#[section(reference)]`.
- When submitting a fn() with a body to a typed link section, the function's
body is not placed in any specific section. To restore the previous behavior,
manually split function pointers and bodies:

```rust
#[section(untyped, aux(main = FN_ARRAY))]
pub static FN_BODIES: link_section::Section;

#[section(typed)]
pub static FN_ARRAY: link_section::TypedSection<fn()>;

#[in_section(FN_ARRAY)]
const _: fn() = linked_function;

#[in_section(FN_BODIES)]
pub fn linked_function() {
    eprintln!("linked_function");
}
```

## [0.16.1] - 2026-05-11

### Changed

- Repair MSRV breakage for WASM targets.

## [0.16.0] - 2026-05-08

### Added

- Support for AIX targets. Requires `-C link-arg=-bdbg:namedsects:ss` (or a
recent Rust version that sets this automatically).

## [0.15.0] - 2026-05-06

### Added

- Support for `const` items in link sections.
- WASM now requires `const` items, and uses `ctor`-like initialization to copy
  data to a contiguous section. To access link-section slices in WASM in
  constructor functions, make sure to use `priority = 1`.
- Zero-sized types are no longer used in `extern`s. Windows now uses a
  non-zero-sized alignment marker to align the start and end of the section.
  Other LLVM/GCC platforms use a `u8`.
- `link-section` is now `no_std`-compatible.

## [0.14.0] - 2026-05-04

### Changed

- WASM targets now use an extern `read_custom_section` function to read custom
  sections.

## [0.13.1] - 2026-05-02

### Changed

- Documentation polish and typo fixes.

## [0.13.0] - 2026-05-02

### Changed

- `used_linker` feature moved to `--cfg linktime_used_linker` flag.
- On macOS, `fn` items are placed in a `__TEXT,__text,regular,pure_instructions`
  section (fixes a linker warning in nightly).

## [0.11.0] - 2026-04-28

### Changed

- Macro attributes and crate features are auto-documented.

## [0.2.1] - 2026-04-22

### Added

- Included licenses in all files.
- Bumped proc-macro dependency versions.

### Changed

- `link-section` crate no longer offers `const` section pointers.
