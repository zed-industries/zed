# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [6.0.0]
### Changed
- `copy_buffers` is now unsafe.
- `get_display` is now unsafe.
- `get_platform_display` is now unsafe.
- `create_platform_window_surface` is now unsafe.
- `create_platform_pixmap_surface` is now unsafe.
### Fixed
- Fix `choose_config` & `get_config` undefined behavior when the input `configs`
  vector is empty. (Fixes [#21](https://github.com/timothee-haudebourg/khronos-egl/issues/21)).
- Fix Windows build (Fixes [#23](https://github.com/timothee-haudebourg/khronos-egl/pull/23)).

## [5.0.0]
### Changed
- Upgrade dependency `libloading`: ^0.7 -> ^0.8.

## [4.1.0]
### Changed
- `load_required` and `load` now trying to load `libEGL.so.1` or `libEGL.so`.

## [4.0.0]
### Added
- `no-pkg-config` feature.
### Changed
- Upgrade dependency `libloading`: ^0.6 -> ^0.7.
### Removed
- `nightly` feature hich is no longer needed since `const_fn` is stabilized.

## [3.0.2]
### Changed
- One Linux, use the `RTLD_NODELETE` when loading the EGL library in `load_required_from_filename` and `load_from_filename`.

## [3.0.1]
### Changed
- Load `libEGL.so.1` by default instead of `libEGL.so`.

## [3.0.0]
### Changed
- Impl `Debug` for `Static`, `Dynamic` and `Instance`.
- Add a `DynamicInstance` type alias for `Instance<Dynamic<libloading::Library>>` with helper functions.
- Precise version selection.
- Dynamic cast between versions with `Dynamic::load`, `Dynamic::load_required` and the `Upcast`/`Downcast` traits.
- `DynamicInstance::downcast` and `IDynamicInstance::upcast`.

## [3.0.0-beta]
### Changed
- Removed the `khronos` dependency.
- Dynamic linking: Add the `Api` trait and the `Instance` struct along with the `static` and `dynamic` features.
- The dependency to `pkg-config` is now optional, only required by the `static` feature.
- Add an optional dependency to `libloading`, only required by the `dynamic` feature.

## [2.2.0]
### Added
- Fix #9: new function `get_config_count` to get the number of available frame buffer configurations.

## [2.1.1]
### Changed
- Upgrade dependency `gl`: ^0.11 -> ^0.14
- Upgrade dependency `wayland-client`: ^0.23 -> ^0.25
- Upgrade dependency `wayland-protocols`: ^0.23 -> ^0.25
- Upgrade dependency `wayland-egl`: ^0.23 -> ^0.25

## [2.1.0]
### Changed
- Fix #3: accept `Option<Display>` instead of `Display` in `query_string`.
- More flexible dependencies versions.
