# Change Log
All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [0.12.0] - 2022-11-28
- Produce error when `default` is used with `field(type = "...")` rather than silently ignoring `default` #269
- Add support for `crate = "..."` to support re-export scenarios #274

## [0.11.2] - 2022-04-20
- Allow restricted visibility using `vis = "..."` for builders, build methods, setters, and fields #247
- Allow specifying the type of a builder field using `#[builder(field(type = "..."))]` #246
- Allow specifying how a builder field is built using `#[builder(field(build = "..."))]` #246
- Update `darling`, `proc-macro2`, `syn`, and `quote` dependencies #250

## [0.11.1] - 2022-03-16
- Forward `allow` and `cfg` attributes from the deriving struct to the builder and its impl block #222
- Support passing attributes to the builder struct using `#[builder_struct_attr(...)]`
- Support passing attributes to the builder struct's inherent `impl` block using `#[builder_impl_attr(...)]`

## [0.11.0] - 2022-03-15
- Support shorthand and long-form collection setters; `#[builder(setter(each = "..."))]` and `#[builder(setter(each(name = "...")))]` #234
- Allow collection setters to be generic over `Into` using `#[builder(setter(each(name = "...", into)))] #234 and #214
- Allow specifying attributes for builder fields and setters using `#[builder_field_attr(...)]` and `#[builder_setter_attr(...)]` #237

## [0.10.2] - 2021-04-21
- Don't reference `derive_builder_core` from `derive_builder` #206

## [0.10.1] - 2021-04-20
- Don't reference `std` in no_std contexts #204

## [0.10.0] - 2021-03-31
- Requires Rust 1.40.0 or newer (was 1.37.0) #169
- Logging feature is removed #177
- Type parameters no longer have the `Default` bound #178
- Make most of `derive_builder_core` private #189
- Add `setter(each = "name")` for extension of collection-like fields #199

## [0.10.0-alpha] - 2021-01-13
- Requires Rust 1.40.0 or newer (was 1.37.0) #169
- Logging feature is removed #177
- Type parameters no longer have the `Default` bound #178
- Make most of `derive_builder_core` private #189

## [0.9.0] - 2019-11-07
- Add `setter(custom)` to allow implementing a custom setter #154

## [0.8.1] - 2019-10-30
- Increase `darling` dependency to 0.10.2 #153

## [0.8.0] - 2019-09-30
- Increase dependency versions to latest #148
- Requires Rust 1.37.0 or newer (was 1.18.0) #148
- Increase version of derive_builder_core crate to match crate's public interface

## [0.7.2] - 2019-05-22
- Add `strip_option` flag for setter #116

## [0.7.1] - 2019-02-05
- Updated `darling` to `0.8.5` and switched to better errors

## [0.7.0] - 2018-10-22

### Breaking Changes
- Updated all dependencies to latest versions #138

## [0.6.0] - 2018-09-04

### Breaking Changes
- Requires Rust 1.18.0 or newer (was 1.15.0) #120
- Updated to `syn` 0.13.10. #120
- Updated to `quote` 0.5.2 #120
- Removed support for deprecated attributes #120

### Changed
- `Clone` is no longer derived on a builder using the owned pattern unless it
  has a field override that uses the mutable/immutable pattern. #97
- Using `#[builder(private)]` at the struct level will now emit a private builder. #99

### Added
- Use `build_fn(private)` to generate a build method only accessible within the mod scope #89

### Internal Changes
- Rewrote options parser using `darling` 0.6.3 #120

## [0.5.2] - 2018-05-31

### Fixed
- Generated code for structs with type parameters and struct-level defaults now compiles #127

## [0.5.1] - 2017-12-16

### Changed
- The standard library `collections` crate was merged into `alloc`
  ([PR](https://github.com/rust-lang/rust/pull/42648)).
  Correspondingly when using this crate within a crate with `#![no_std]` you must
  use `#![feature(alloc)] extern crate alloc` in your crate,
  was `#![feature(collections)] extern crate collections`.

### Fixed
- `unused_mut` lint (variable does not need to be mutable) #104

## [0.5.0] - 2017-06-30

### Changed

- `#[builder(default)]` and `#[builder(default = "...")]` at the struct level
  change their behaviour and construct a default value for the struct,
  instead of all fields individually.
- builder fields are no longer public by default; Fields can be explicitly
  made public at the struct or field level using the new attribute:
  `#[builder(field(public))]`

### Removed
- removed previously deprecated syntax `#[builder(setter_prefix = "with")]`,
  please use `#[builder(setter(prefix = "with"))]` instead

## [0.4.7] - 2017-04-29

### Fixed
- for generic structs, apply the `T: Clone` type bound in builder impl
  instead of struct definition #91
- only emit the `T: Clone` type bound when it is actually needed, i.e.
  mutable/immutable pattern, but not owned pattern.

## [0.4.6] - 2017-04-26

### Added
- pre-build validation via `#[builder(build_fn(validate = "path::to::fn"))]`

## [0.4.5] - 2017-04-25

### Added
- customize setter names via `#[builder(setter(name = "..."))]`
- customize build_fn name via `#[builder(build_fn(name = "..."))]`
- suppress build method generation via `#[builder(build_fn(skip))]`
- derive additional traits via `#[builder(derive(Trait1, Trait2, ...))]`
- set field visibility separate from setter visibility via
  `#[builder(field(private))]` at the field or struct level

### Deprecated
- builder fields will no longer be public by default in 0.5.0; relying on this
  will now emit a deprecation warning. Fields can be explicitly made public at
  the struct or field level using the new `#[builder(field(public))]`
  attribute. To squelch this warning and opt-into the new behaviour, use the
  `private_fields` crate feature or explicitly set field visibility at the
  struct level.

## [0.4.4] - 2017-04-12

### Added
- try_setters, e.g. `#[builder(try_setter)]`. These setters are exposed
  alongside the normal field setters and allow callers to pass in values which
  have fallible conversions to the needed type through `TryInto`. This
  attribute can only be used on nightly when `#![feature(try_from)]` is
  declared in the consuming crate's root; this will change when Rust issue
  [#33417](https://github.com/rust-lang/rust/issues/33417) is resolved.

## [0.4.3] - 2017-04-11

### Fixed
- `setter(skip)` honors struct-inherited and explicit defaults #68

## [0.4.2] - 2017-04-10

### Fixed
- support generic references in structs #55
- support `#![no_std]` #63

## [0.4.1] - 2017-04-08

### Deprecated
- `#[builder(default)]` and `#[builder(default = "...")]` at the struct level will
  change their behaviour in 0.5.0 and construct a default value for the struct,
  instead of all fields individually. To opt into the new behaviour and squelch
  this deprecation warning you can add the `struct_default` feature flag.

## [0.4.0] - 2017-03-25

### Added
- skip setters, e.g. `#[builder(setter(skip))]`
- default values, e.g. `#[builder(default = "42")]` or just `#[builder(default)]`

### Changed
- deprecated syntax `#[builder(setter_prefix = "with")]`,
  please use `#[builder(setter(prefix = "with"))]` instead
- setter conversions are now off by default, you can opt-into via
  `#[builder(setter(into))]`
- logging is behind a feature flag. To activate it, please add
  `features = ["logging"]` to the dependency in `Cargo.toml`. Then you can use
  it like: `RUST_LOG=derive_builder=trace cargo test`.

### Fixed
- use full path for result #39
- support `#[deny(missing_docs)]` #37
- support `#![no_std]` via `#[builder(no_std)]` #41

## [0.3.0] - 2017-02-05

Requires Rust 1.15 or newer.

### Added
- different setter pattern, e.g. `#[builder(pattern = "immutable")]`
- private setters, e.g. `#[builder(private)]`
- additional debug info via env_logger, e.g.
  `RUST_LOG=derive_builder=trace cargo test`
- prefixes, e.g. `#[builder(setter_prefix = "with")]`
- field specific overrides
- customize builder name, e.g. `#[builder(name = "MyBuilder")]`

### Changed
- migration to macros 1.1
- migration to traditional builder pattern
  i.e. seperate `FooBuilder` struct to build `Foo`
=> please refer to the new docs

### Fixed
- missing lifetime support #21

## [0.2.1] - 2016-09-24

### Fixed
- preserve ordering of attributes #27

## [0.2.0] - 2016-08-22
### Added
- struct fields can be public
- struct fields can have attributes
- the following struct-attributes are copied to the setter-method
 - `/// ...`
 - `#[doc = ...]`
 - `#[cfg(...)]`
 - `#[allow(...)]`

### Changed
- setter-methods are non-consuming now -- breaking change
- setter-methods are public now

### Fixed
- automatic documentation does not work #16

## [0.1.0] - 2016-08-07
### Added
- first implementation
 - generate setter methods
 - support for generic structs

[Unreleased]:  https://github.com/colin-kiegel/rust-derive-builder/compare/v0.5.1...HEAD
[0.5.1]:  https://github.com/colin-kiegel/rust-derive-builder/compare/v0.5.0...v0.5.1
[0.5.0]:  https://github.com/colin-kiegel/rust-derive-builder/compare/v0.4.7...v0.5.0
[0.4.7]:  https://github.com/colin-kiegel/rust-derive-builder/compare/v0.4.6...v0.4.7
[0.4.6]:  https://github.com/colin-kiegel/rust-derive-builder/compare/v0.4.5...v0.4.6
[0.4.5]:  https://github.com/colin-kiegel/rust-derive-builder/compare/v0.4.4...v0.4.5
[0.4.4]:  https://github.com/colin-kiegel/rust-derive-builder/compare/v0.4.3...v0.4.4
[0.4.3]:  https://github.com/colin-kiegel/rust-derive-builder/compare/v0.4.2...v0.4.3
[0.4.2]:  https://github.com/colin-kiegel/rust-derive-builder/compare/v0.4.1...v0.4.2
[0.4.1]:  https://github.com/colin-kiegel/rust-derive-builder/compare/v0.4.0...v0.4.1
[0.4.0]:  https://github.com/colin-kiegel/rust-derive-builder/compare/v0.3.0...v0.4.0
[0.3.0]:  https://github.com/colin-kiegel/rust-derive-builder/compare/v0.2.1...v0.3.0
[0.2.1]:  https://github.com/colin-kiegel/rust-derive-builder/compare/v0.2.0...v0.2.1
[0.2.0]:  https://github.com/colin-kiegel/rust-derive-builder/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/colin-kiegel/rust-derive-builder/tree/v0.1.0
