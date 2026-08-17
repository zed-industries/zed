# Changelog

Notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Unreleased - YYYY-MM-DD


## 0.5.1 - 2024-05-21

### Deprecated
* Deprecated the `apple` Cargo feature flag, it is assumed by default on Apple
  platforms.


## 0.5.0 - 2024-04-17

### Added
* **BREAKING**: Added `Block::copy` to convert blocks to `RcBlock`. This
  replaces `StackBlock::copy`, but since `StackBlock` implements `Deref`, this
  will likely work as before.
* Added `RcBlock::new(closure)` as a more efficient and flexible alternative
  to `StackBlock::new(closure).copy()`.
* Added `BlockFn` trait to describe valid `dyn Fn` types for blocks.

### Changed
* **BREAKING**: Changed how blocks specify their parameter and return types.
  We now use `dyn Fn` so that it is more clear what the parameter and return
  types are. This also allows us to support non-`'static` blocks.

  ```rust
  // Before
  let block: &Block<(), ()>;
  let block: &Block<(i32,), i32>;
  let block: &Block<(i32, u32), (i32, u32)>;

  // After
  let block: &Block<dyn Fn()>;
  let block: &Block<dyn Fn(i32) -> i32>;
  let block: &Block<dyn Fn(i32, u32) -> (i32, u32)>;
  // Now possible
  let block: &Block<dyn Fn() + '_>; // Non-'static block
  ```
* **BREAKING**: Make `Block::call` safe, and instead move the upholding of the
  safety invariant to the type itself.
* **BREAKING**: Renamed `RcBlock::new(ptr)` to `RcBlock::from_raw(ptr)`.
* **BREAKING**: Made `RcBlock` use the null-pointer optimization;
  `RcBlock::from_raw` and `RcBlock::copy` now return an `Option`.
* **BREAKING**: Only expose the actually public symbols `_Block_copy`,
  `_Block_release`, `_Block_object_assign`, `_Block_object_dispose`,
  `_NSConcreteGlobalBlock`, `_NSConcreteStackBlock` and `Class` in `ffi`
  module.
* **BREAKING**: Renamed `IntoConcreteBlock` to `IntoBlock`, moved
  associated type `Output` to be a generic parameter, and added lifetime
  parameter.`
* No longer use the `block-sys` crate for linking to the blocks runtime.
* Renamed `ConcreteBlock` to `StackBlock`, and added a lifetime parameter. The
  old name is deprecated.
* Added `Copy` implementation for `StackBlock`.

### Removed
* **BREAKING**: Removed `BlockArguments` in favour of `BlockFn`, which
  describes both the parameter types, as well as the return type.

### Fixed
* **BREAKING**: `StackBlock::new` now requires the closure to be `Clone`. If
  this is not desired, use `RcBlock::new` instead.
* Relaxed the `F: Debug` bound on `StackBlock`'s `Debug` implementation.
* **BREAKING**: Fixed `GlobalBlock` not having the correct variance. This may
  break if you were using lifetimes in your parameters, as those are now a bit
  too restrictive.


## 0.4.0 - 2023-12-03

### Changed
* **BREAKING**: Updated `objc2` dependency to `v0.5.0`.


## 0.3.0 - 2023-07-31

### Fixed
* Bumped version number to ensure that this crate can be compiled together
  with code that depends on pre-releases of `0.2.0`.


## 0.2.0 - 2023-06-20

### Changed
* **BREAKING**: Updated `objc2` dependency to `v0.4.0`.


## 0.2.0-alpha.8 - 2023-02-07

### Changed
* **BREAKING**: Use traits from `objc2` `v0.3.0-beta.5` instead of
  `objc2-encode`.
* Updated `ffi` module to `block-sys v0.2.0`.


## 0.2.0-alpha.7 - 2022-12-24

### Changed
* Improve efficiency when a block doesn't need to be destroyed.
* **BREAKING**: Updated `objc2-encode` to `v2.0.0-pre.3`.
* Updated `ffi` module to `block-sys v0.1.0-beta.2`.


## 0.2.0-alpha.6 - 2022-08-28

### Changed
* **BREAKING**: Updated `objc2-encode` to `v2.0.0-pre.2`.
* Updated `ffi` module to `block-sys v0.1.0-beta.1`.

### Fixed
* **BREAKING**: Cleaned up `BlockArguments` trait, it is now sealed and a
  subtrait of `EncodeArguments`.
* **BREAKING**: Cleaned up `IntoConcreteBlock` trait, it is now sealed and the
  associated output type has been renamed to `Output`.


## 0.2.0-alpha.5 - 2022-07-19

### Added
* Implemented `Debug` for `Block`, `ConcreteBlock`, `RcBlock` and
  `GlobalBlock`.

### Changed
* **BREAKING**: Updated `objc2-encode` to `v2.0.0-pre.1`.
* Updated `ffi` module to `block-sys v0.1.0-beta.0`.


## 0.2.0-alpha.4 - 2022-06-13

### Changed
* **BREAKING**: Updated `objc2-encode` to `v2.0.0-pre.0`.
* **BREAKING**: Updated `ffi` module to `block-sys v0.0.4`. This tweaks the
  types of a lot of fields and parameters, and makes the apple runtime always
  be the default.

### Removed
* **BREAKING**: Removed `DerefMut` implementation for `ConcreteBlock`.


## 0.2.0-alpha.3 - 2022-01-03

### Changed
* Changed `global_block!` macro to take an optional semicolon at the end.
* Improved documentation.
* **BREAKING**: Updated `ffi` module to `block-sys v0.0.3`.


## 0.2.0-alpha.2 - 2021-12-22

### Added
* `GlobalBlock` and corresponding `global_block!` macro, allowing statically
  creating blocks that don't reference their environment.

### Changed
* **BREAKING**: Updated `ffi` module to `block-sys v0.0.2`. This means that
  `Class` is now `!UnwindSafe`.


## 0.2.0-alpha.1 - 2021-11-22

### Added
* Proper GNUStep support using `block-sys`. See that crate for usage.
* Export `block-sys` as `ffi` module.

### Removed
* Dependency on `objc_test_utils`.

### Fixed
* `ConcreteBlock` no longer allocates block descriptors on the heap.
* Better unwind safety in `ConcreteBlock::copy`.


## 0.2.0-alpha.0 - 2021-10-28

### Added
* **BREAKING**: Blocks now require that parameter and return types implement
  `objc2_encode::Encode`. This is a safety measure to prevent creating blocks
  with invalid parameters.
* Blocks now implements `objc2_encode::RefEncode` (and as such can be used in
  Objective-C message sends).
* Update to 2018 edition.

### Changed
* **BREAKING**: Forked the project, so it is now available under the name
  `block2`.

### Fixed
* Soundness issues with using empty enums over FFI.


## [0.1.6] (`block` crate) - 2016-05-08

### Added
* Support for linking to `libBlocksRuntime`.


## [0.1.5] (`block` crate) - 2016-04-04

### Changed
* Minor code changes


## [0.1.4] (`block` crate) - 2015-11-12

### Removed
* `libc` dependency.


## [0.1.3] (`block` crate) - 2015-11-07

### Changed
* Updated `libc` dependency.


## [0.1.2] (`block` crate) - 2015-10-10

### Fixed
* `improper_ctypes` warning.


## [0.1.1] (`block` crate) - 2015-09-03

### Fixed
* Missing `Sized` bounds on traits.


## [0.1.0] (`block` crate) - 2015-05-18

### Added
* `Clone` implementation for `RcBlock`.
* Improved documentation.

### Changed
* **BREAKING**: Rename `IdBlock` to `RcBlock`.
* **BREAKING**: Make `Block::call` take self immutably and make it `unsafe`.
* **BREAKING**: Make `BlockArguments::call_block` `unsafe`.

### Removed
* **BREAKING**: `DerefMut` on `RcBlock`.
* `objc` dependency.
* `Foundation` dependency in tests.


## [0.0.2] (`block` crate) - 2015-05-02

### Changed
* Use `objc_id`.


## [0.0.1] (`block` crate) - 2015-04-17

Initial version.


[0.1.6]: https://github.com/madsmtm/objc2/compare/block-0.1.5...block-0.1.6
[0.1.5]: https://github.com/madsmtm/objc2/compare/block-0.1.4...block-0.1.5
[0.1.4]: https://github.com/madsmtm/objc2/compare/block-0.1.3...block-0.1.4
[0.1.3]: https://github.com/madsmtm/objc2/compare/block-0.1.2...block-0.1.3
[0.1.2]: https://github.com/madsmtm/objc2/compare/block-0.1.1...block-0.1.2
[0.1.1]: https://github.com/madsmtm/objc2/compare/block-0.1.0...block-0.1.1
[0.1.0]: https://github.com/madsmtm/objc2/compare/block-0.0.2...block-0.1.0
[0.0.2]: https://github.com/madsmtm/objc2/compare/block-0.0.1...block-0.0.2
[0.0.1]: https://github.com/madsmtm/objc2/releases/tag/block-0.0.1
