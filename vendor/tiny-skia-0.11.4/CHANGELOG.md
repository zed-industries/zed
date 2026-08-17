# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [0.11.4] - 2024-02-04
### Fixed
- `Path::compute_tight_bounds` calculation.
  Thanks to [@qbcbyb](https://github.com/qbcbyb)

## [0.11.3] - 2023-12-03
### Added
- `Path::compute_tight_bounds`

## [0.11.2] - 2023-10-01
### Changed
- MSRV bumped to stable, because of the `flate2` crate.

### Fixed
- `Transform::is_valid` was treating 1/4096 as zero. We need a higher precision.
- Build failure on AVX2 with `--no-default-features`.
  Thanks to [@linkmauve](https://github.com/linkmauve)

## [0.11.1] - 2023-06-17
### Changed
- MSRV bumped to 1.60, because of the `log` crate.

### Fixed
- `LineJoin::MiterClip` handling with small `miter_limit`.
  Thanks to [@torokati44](https://github.com/torokati44)

## [0.11.0] - 2023-06-08
### Added
- `LineJoin::MiterClip`.
  Thanks to [@torokati44](https://github.com/torokati44)

### Changed
- `Rect::inset` and `Rect::outset` are no longer require a mutable `self`.
  Thanks to [@wezm](https://github.com/wezm)

## [0.10.0] - 2023-05-27
### Added
- `PathBuilder::push_path`
- `NonZeroRect`
- `Size`
- `Rect::transform`
- `Rect::bbox_transform`
- `Transform::from_bbox`
- `Transform::is_valid`
- `Transform::get_scale`
- `Transform::pre_rotate`
- `Transform::post_rotate`
- `Transform::pre_rotate_at`
- `Transform::post_rotate_at`
- `Transform::map_point`

### Changed
- `ColorU8` and `PremultipliedColorU8` are stored as `[u8; 4]` instead of `u32`.
  This fixes potential [alignment issues](https://github.com/RazrFalcon/tiny-skia/issues/70)
  and make the code easier to understand.
  Thanks to [@e00E](https://github.com/e00E)
- `PathBuilder::push_rect` accepts `Rect` and not `f32` numbers now.

### Removed
- `tiny_skia_path::ScreenIntRect`. It become private.
- `Path::is_empty`. `Path` cannot be empty by design.

### Removed
- `ColorU8::get` and `PremultipliedColorU8::get`. Use the getters instead.

## [0.9.1] - 2023-05-17
### Added
- Reexport `tiny_skia_path::PathStroker` in `tiny-skia`.

## [0.9.0] - 2023-04-23
### Added
- `Mask::from_vec`
- `Mask::from_pixmap` to convert `Pixmap` into `Mask` by extracting alpha or luminance.
- `Mask::width`
- `Mask::height`
- `Mask::data`
- `Mask::data_mut`
- `Mask::fill_path`
- `Mask::decode_png`
- `Mask::load_png`
- `Mask::encode_png`
- `Mask::save_png`
- `Mask::invert`
- `MaskType`
- `Pixmap::apply_mask`

### Changed
- Rename `ClipMask` into `Mask`.
- `Mask` is closer to a 8bit (A8) `Pixmap` now, rather than being its own thing.
- `Mask::new` requires width and height arguments now.
- Drawing on `Mask` using `Mask::fill_path` uses our SIMD pipeline now instead of a scalar code
  that should make it a bit faster.
- Painting API no longer returns `Option<()>`, but simply adds a warning to the log.
- `Paint::anti_alias` is set to `true` by default.

### Removed
- `Mask::set_path`. Use `Mask::fill_path` instead.
- `Mask::default()`. Mask cannot be empty anymore.

## [0.8.4] - 2023-04-22
### Added
- Implement `PartialEq` for `Paint` and subtypes. Thanks to [@hecrj](https://github.com/hecrj)

### Changed
- MSRV bumped to 1.57, mainly because of the `png` crate.

### Fixed
- `ClipMask`s larger than 8191x8191 pixels.
  Previously, the creation of a large mask via `ClipMask::set_path`
  would have created an empty mask.

## [0.8.3] - 2023-02-05
### Fixed
- Performance regression, probably due to LLVM update in Rust.
  Thanks to [@mostafa-khaled775](https://github.com/mostafa-khaled775)
- Big-endian targets support. Thanks to [@ids1024](https://github.com/ids1024)

## [0.8.2] - 2022-10-22
### Added
- `Pixmap::from_vec`.

### Fixed
- Increase Conic to Quad conversion precision. This allows us to produce nicer round caps.
  Previously, they were not as round as needed.

## [0.8.1] - 2022-08-29
### Fixed
- Conditional compilation of `FasterMinMax` on fallback platforms.
  Thanks to [@CryZe](https://github.com/CryZe)

## [0.8.0] - 2022-08-27
### Added
- AArch64 Neon SIMD support. Up to 3x faster on Apple M1.
  Thanks to [@CryZe](https://github.com/CryZe)

### Changed
- `FiniteF32`, `NormalizedF32` and `NonZeroPositiveF32` types have been moved
  to the `strict-num` crate.
- Rename `NormalizedF32::from_u8` into `NormalizedF32::new_u8`.
- Rename `NormalizedF32::new_bounded` into `NormalizedF32::new_clamped`.
- Use explicit SIMD intrinsic instead of relying on `safe_arch`.
- MSRV bumped to 1.51

## [0.7.0] - 2022-07-03
### Added
- `tiny-skia-path` dependency that can be used independently from `tiny-skia`.
  It contains the `tiny-skia` Bezier path implementation, including stroking and dashing.
  As well as all the geometry primitives (like `Point` and `Rect`).

### Changed
- When disabling the `std` feature, one have to enable `no-std-float` feature instead of `libm` now.

## [0.6.6] - 2022-06-23
### Fixed
- Panic in `Rect::round` and `Rect::round_out`.
  Thanks to [@Wardenfar](https://github.com/Wardenfar)

## [0.6.5] - 2022-06-10
### Fixed
- Minimum `arrayref` version.

## [0.6.4] - 2022-06-04
### Fixed
- Panic during non-aliased hairline stroking at the bottom edge of an image.

## [0.6.3] - 2022-02-01
### Fixed
- SourceOver blend mode must not be optimized to Source when ClipPath is present.

## [0.6.2] - 2021-12-30
### Fixed
- `ClipMask::intersect_path` alpha multiplying.

## [0.6.1] - 2021-08-28
### Added
- Support rendering on pixmaps larger than 8191x8191 pixels.
  From now, `Pixmap` is limited only by the amount of memory caller has.
- `Transform::map_points`
- `PathBuilder::push_oval`

## [0.6.0] - 2021-08-21
### Added
- WASM simd128 support. Thanks to [@CryZe](https://github.com/CryZe)

### Changed
- `Transform::post_scale` no longer requires `&mut self`.
- Update `png` crate.

## [0.5.1] - 2021-03-07
### Fixed
- Color memset optimizations should be ignored when clip mask is present.
- `ClipMask::intersect_path` logic.

## [0.5.0] - 2021-03-06
### Added
- `ClipMask::intersect_path`
- no_std support. Thanks to [@CryZe](https://github.com/CryZe)

### Changed
- Reduce `Transform` strictness. It's no longer guarantee to have only finite values,
  therefore we don't have to check each operation.

### Removed
- `Canvas`. Call `Pixmap`/`PixmapMut` drawing methods directly.

## [0.4.2] - 2021-01-23
### Fixed
- Panic during path filling with anti-aliasing because of incorrect edges processing.

## [0.4.1] - 2021-01-19
### Fixed
- Endless loop during stroke dashing.

## [0.4.0] - 2021-01-02
### Changed
- Remove almost all `unsafe`. No performance changes.

## [0.3.0] - 2020-12-20
### Added
- `PixmapRef` and `PixmapMut`, that can be created from `Pixmap` or from raw data.
- `Canvas::set_clip_mask`, `Canvas::get_clip_mask`, `Canvas::take_clip_mask`.

### Changed
- `Canvas` no longer owns a `Pixmap`.
- `Canvas::draw_pixmap` and `Pattern::new` accept `PixmapRef` instead of `&Pixmap` now.
- Improve clipping performance.
- The internal `ClipMask` type become public.

### Fixed
- Panic when path is drawn slightly past the `Pixmap` bounds.

### Removed
- `Canvas::new`

## 0.2.0 - 2020-11-16
### Changed
- Port to Rust.

## 0.1.0 - 2020-07-04
### Added
- Bindings to a stripped down Skia fork.

[Unreleased]: https://github.com/RazrFalcon/tiny-skia/compare/v0.11.4...HEAD
[0.11.4]: https://github.com/RazrFalcon/tiny-skia/compare/v0.11.3...v0.11.4
[0.11.3]: https://github.com/RazrFalcon/tiny-skia/compare/v0.11.2...v0.11.3
[0.11.2]: https://github.com/RazrFalcon/tiny-skia/compare/v0.11.1...v0.11.2
[0.11.1]: https://github.com/RazrFalcon/tiny-skia/compare/v0.11.0...v0.11.1
[0.11.0]: https://github.com/RazrFalcon/tiny-skia/compare/v0.10.0...v0.11.0
[0.10.0]: https://github.com/RazrFalcon/tiny-skia/compare/v0.9.1...v0.10.0
[0.9.1]: https://github.com/RazrFalcon/tiny-skia/compare/v0.9.0...v0.9.1
[0.9.0]: https://github.com/RazrFalcon/tiny-skia/compare/v0.8.4...v0.9.0
[0.8.4]: https://github.com/RazrFalcon/tiny-skia/compare/v0.8.3...v0.8.4
[0.8.3]: https://github.com/RazrFalcon/tiny-skia/compare/v0.8.2...v0.8.3
[0.8.2]: https://github.com/RazrFalcon/tiny-skia/compare/v0.8.1...v0.8.2
[0.8.1]: https://github.com/RazrFalcon/tiny-skia/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/RazrFalcon/tiny-skia/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/RazrFalcon/tiny-skia/compare/v0.6.6...v0.7.0
[0.6.6]: https://github.com/RazrFalcon/tiny-skia/compare/v0.6.5...v0.6.6
[0.6.5]: https://github.com/RazrFalcon/tiny-skia/compare/v0.6.4...v0.6.5
[0.6.4]: https://github.com/RazrFalcon/tiny-skia/compare/v0.6.3...v0.6.4
[0.6.3]: https://github.com/RazrFalcon/tiny-skia/compare/v0.6.2...v0.6.3
[0.6.2]: https://github.com/RazrFalcon/tiny-skia/compare/v0.6.1...v0.6.2
[0.6.1]: https://github.com/RazrFalcon/tiny-skia/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/RazrFalcon/tiny-skia/compare/v0.5.1...v0.6.0
[0.5.1]: https://github.com/RazrFalcon/tiny-skia/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/RazrFalcon/tiny-skia/compare/v0.4.2...v0.5.0
[0.4.2]: https://github.com/RazrFalcon/tiny-skia/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/RazrFalcon/tiny-skia/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/RazrFalcon/tiny-skia/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/RazrFalcon/tiny-skia/compare/v0.2.0...v0.3.0
