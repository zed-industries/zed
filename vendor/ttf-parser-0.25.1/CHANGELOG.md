# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [0.25.1] - 2024-11-29
### Changed
- Fix some typos and documentation related lints
- Set MSRV in manifest to actual current value (1.63.0)
- Update forge URL for ttf-parser and rustybuzz to harfbuzz org

## [0.25.0] - 2024-10-04
### Added
- `STAT` table parsing. Thanks to [inferiorhumanorgans](https://github.com/inferiorhumanorgans).
- `UnicodeRanges` internal field is public now.
- `cargo-c` metadata to C API. Thanks to [lu-zero](https://github.com/lu-zero).

### Changed
- `Face::is_italic` checks for italic angle as well.
- `Face::italic_angle` returns just a `f32` and not `Option<f32>` now.
- Bump MSRV to 1.59

### Fixed
- Only apply `avar` table to the variation axis being set.
  Thanks to [maxmelander](https://github.com/maxmelander).

## [0.24.1] - 2024-08-05
### Added
- (`glyf`) `glyf::Table::bbox`. Thanks to [LaurenzV](https://github.com/LaurenzV).

### Fixed
- (`kerx`) `kerx::SubtablesIter` wasn't updating the current subtable index.
- (`GPOS`) `gpos::AnchorMatrix` allows nullable/optional matrices now.
  Thanks to [LaurenzV](https://github.com/LaurenzV).

## [0.24.0] - 2024-07-02
### Changed
- Make `core_maths` dependency optional.
  When building for `no_std` one must enable `no-std-float` build feature now.

## [0.23.0] - 2024-07-02
### Changed
- Use `core_maths` instead of `libm`. Should simplify the build process.
  Thanks to [LaurenzV](https://github.com/LaurenzV).

### Removed
- `no-std-float` build flag. Should be handled automatically now.

## [0.22.0] - 2024-06-29
### Added
- `Face::glyph_phantom_points`
- `hvar::Table::right_side_bearing_offset`. Thanks to [LaurenzV](https://github.com/LaurenzV).
- `vvar::Table::advance_offset`. Thanks to [LaurenzV](https://github.com/LaurenzV).
- `vvar::Table::top_side_bearing_offset`. Thanks to [LaurenzV](https://github.com/LaurenzV).
- `vvar::Table::bottom_side_bearing_offset`. Thanks to [LaurenzV](https://github.com/LaurenzV).
- `vvar::Table::vertical_origin_offset`. Thanks to [LaurenzV](https://github.com/LaurenzV).
- `colr::Table::clip_box`. Thanks to [LaurenzV](https://github.com/LaurenzV).

### Changed
- `no_std` build of `ttf-parser` requires `--features=no-std-float` now.
  This is because we need trigonometry functions to flatten transforms in `COLR`.
  Thanks to [LaurenzV](https://github.com/LaurenzV).
- `colr::Painter` no longer has `push_translate`, `push_scale`, `push_rotate` and `push_skew`.
  Only `push_transform` left.
  Thanks to [LaurenzV](https://github.com/LaurenzV).
- Split `hvar::Table` into `hvar::Table` and `vvar::Table`.
  Previously, we treated both `HVAR` and `VVAR` tables as identical,
  but `VVAR` actually has additional fields.
  Thanks to [LaurenzV](https://github.com/LaurenzV).
- Rename `hvar::Table::side_bearing_offset` into `hvar::Table::left_side_bearing_offset`.
  Thanks to [LaurenzV](https://github.com/LaurenzV).

### Fixed
- `Face::glyph_hor_advance` and `Face::glyph_ver_advance` include `gvar`'s phantom points
  when `HVAR`/`VVAR` tables are missing. Affects only variable fonts.
- (`CFF`) Allow MoveTo with width commands in nested subroutines.
- `opentype_layout::LookupFlags::mark_attachment_type` parsing.
- (`CFF`) Allow empty charsets in `cff::parse_charset`.
  Thanks to [LaurenzV](https://github.com/LaurenzV).
- (`gvar`) Empty sub-glyphs/components is no longer an error.
  Thanks to [LaurenzV](https://github.com/LaurenzV).
- (`GSUB`/`GPOS`) Allow `NULL` offsets in `ChainedContextLookup` Format2 subtables.
  Thanks to [LaurenzV](https://github.com/LaurenzV).
- `Face::glyph_y_origin` properly handles variable fonts now.
  Thanks to [LaurenzV](https://github.com/LaurenzV).
- (`kerx`) Fix `AnchorPoints` parsing.
  Thanks to [LaurenzV](https://github.com/LaurenzV).

### Removed
- `push_translate`, `push_scale`, `push_rotate` and `push_skew` from `colr::Painter`.
  Use `colr::Painter::push_transform` instead.
  Thanks to [LaurenzV](https://github.com/LaurenzV).

## [0.21.1] - 2024-05-11
### Fixed
- Delta set length calculation in variable fonts.
  Thanks to [LaurenzV](https://github.com/LaurenzV).<br>
  Got broken in the previous version.

## [0.21.0] - 2024-05-10
### Added
- `COLR` / `CPAL` v1 support.
  Thanks to [LaurenzV](https://github.com/LaurenzV).

### Changed
- Replace `Face::is_bitmap_embedding_allowed` with `Face::is_outline_embedding_allowed`.
  The old one had a bool flag flipped.
  Thanks to [Fuzzyzilla](https://github.com/Fuzzyzilla).
- Increase lenience of embed permissions for older OS/2 versions.
  Thanks to [Fuzzyzilla](https://github.com/Fuzzyzilla).
- Bump MSRV to 1.51

## [0.20.0] - 2023-10-15
### Added
- `COLR` / `CPAL` v0 support.
  Thanks to [laurmaedje](https://github.com/laurmaedje).

### Changed
- `svg::SvgDocumentsList` returns `svg::SvgDocument` and not just `&[u8]` now.
  Thanks to [wjian23](https://github.com/wjian23).
- `Face::set_variation` allows duplicated axes now.

## [0.19.2] - 2023-09-13
### Added
- `cff::Table::glyph_cid`

## [0.19.1] - 2023-06-20
### Fixed
- `cff::Table::glyph_width` returns a correct width when subroutines are present.

## [0.19.0] - 2023-04-17
### Added
- `bdat`, `bloc`, `EBDT` and `EBLC` tables support.
  Thanks to [dzamkov](https://github.com/dzamkov).
- `BitmapMono`, `BitmapMonoPacked`, `BitmapGray2`, `BitmapGray2Packed`, `BitmapGray4`,
  `BitmapGray4Packed`, `BitmapGray8` and `BitmapPremulBgra32` variants to `RasterImageFormat`.

### Fixed
- `CBLC` table parsing.
  Thanks to [dzamkov](https://github.com/dzamkov).

## [0.18.1] - 2023-01-10
### Fixed
- (`MATH`) Handle NULL offsets.
  Thanks to [laurmaedje](https://github.com/laurmaedje).

## [0.18.0] - 2022-12-25
### Added
- `Face::permissions`
- `Face::is_subsetting_allowed`
- `Face::is_bitmap_embedding_allowed`
- `Face::unicode_ranges`
- `os2::Table::permissions`
- `os2::Table::is_subsetting_allowed`
- `os2::Table::is_bitmap_embedding_allowed`
- `os2::Table::unicode_ranges`
- `name::Name::language`
- `Language` enum with all Windows languages.

### Changed
- Using a non-zero index in `Face::parse` for a regular font will return
  `FaceParsingError::FaceIndexOutOfBounds` now. Thanks to [Pietrek14](https://github.com/Pietrek14).

## [0.17.0] - 2022-09-28
### Added
- `MATH` table support. Thanks to [ruifengx](https://github.com/ruifengx)
  and [laurmaedje](https://github.com/laurmaedje).

### Fixed
- (CFF) Fix large tables parsing.

## [0.16.0] - 2022-09-18
### Added
- CFF Encoding support.
- `cff::Table::glyph_index`
- `cff::Table::glyph_index_by_name`
- `cff::Table::glyph_width`
- `cff::Table::number_of_glyphs`
- `cff::Table::matrix`
- `post::Table::glyph_name`
- `post::Table::glyph_index_by_name`
- `post::Table::names`
- `Face::glyph_index_by_name`
- `RawFace` fields and `TableRecord` struct are public now.

### Changed
- `Face::from_slice` was replaced by `Face::parse`.
- `RawFace::from_slice` was replaced by `RawFace::parse`.
- `post::Table::names` is a method and not a field now.
- Use `post::Table::glyph_name` instead of `post::Table::names.get()`.

### Fixed
- (hmtx/vmtx) Allow missing additional side bearings.
- (loca) Allow incomplete table.
- Reduce strictness of some table length checks.
- (post) `post::Names::len` was returning a wrong value. Now this method is gone completely.
  You can use `post::Table::names().count()` instead.

## [0.15.2] - 2022-06-17
### Fixed
- Missing advance and side bearing offsets in `HVAR`/`VVAR` is not an error. Simply ignore them.

## [0.15.1] - 2022-06-04
### Fixed
- (cmap) `cmap::Subtable4::glyph_index` correctly handles malformed glyph offsets now.
- (cmap) `cmap::Subtable4::codepoints` no longer includes `0xFFFF` codepoint.
- (SVG) Fixed table parsing. Thanks to [Shubhamj280](https://github.com/Shubhamj280)

## [0.15.0] - 2022-02-20
### Added
- `apple-layout` build feature.
- `ankr`, `feat`, `kerx`, `morx` and `trak` tables.
- `kern` AAT subtable format 1.
- `RawFace`

### Changed
- The `parser` module is private now again.

## [0.14.0] - 2021-12-28
### Changed
- (cmap) `cmap::Subtable::glyph_index` and `cmap::Subtable::glyph_variation_index` accept
  `u32` instead of `char` now.
- (glyf) ~7% faster outline parsing.

## [0.13.4] - 2021-11-23
### Fixed
- (CFF) Panic during `seac` resolving.
- (CFF) Stack overflow during `seac` resolving.

## [0.13.3] - 2021-11-19
### Fixed
- (glyf) Endless loop during malformed file parsing.

## [0.13.2] - 2021-10-28
### Added
- `gvar-alloc` build feature that unlocks `gvar` table limits by using heap.
  Thanks to [OrionNebula](https://github.com/OrionNebula)

## [0.13.1] - 2021-10-27
### Fixed
- `Face::line_gap` logic.

## [0.13.0] - 2021-10-24
### Added
- Complete GSUB and GPOS tables support. Available under the `opentype-layout` feature.
- Public access to all supported TrueType tables. This allows a low-level, but still safe,
  access to internal data layout, which can be used for performance optimization, like caching.
- `Style` enum and `Face::style` method.
- `Face::glyph_name` can be disabled via the `glyph-names` feature to reduce binary size.

### Changed
- Improved ascender/descender/line_gap resolving logic.
- `Face` methods: `has_glyph_classes`, `glyph_class`, `glyph_mark_attachment_class`,
  `is_mark_glyph` and `glyph_variation_delta` are moved to `gdef::Table`.
- The `Names` struct is no longer an iterator, but a container.
  You have to call `into_iter()` manually.
- The `VariationAxes` struct is no longer an iterator, but a container.
  You have to call `into_iter()` manually.
- Most of the `Name` struct methods become public fields.
- `Face::units_per_em` no longer returns `Option`.
- (`cmap`) Improved subtable 12 performance. Thanks to [xnuk](https://github.com/xnuk)

### Removed
- (c-api) `ttfp_glyph_class`, `ttfp_get_glyph_class`, `ttfp_get_glyph_mark_attachment_class`,
  `ttfp_is_mark_glyph`, `ttfp_glyph_variation_delta` and `ttfp_has_table`.
- `TableName` enum and `Face::has_table`. Tables can be access directly now.
- `Face::character_mapping_subtables`. Use `Face::tables().cmap` instead.
- `Face::kerning_subtables`. Use `Face::tables().kern` instead.

### Fixed
- `Iterator::count` implementation for `cmap::Subtables`, `name::Names` and `LazyArrayIter32`.

## [0.12.3] - 2021-06-27
### Changed
- (`glyf`) Always use a calculated bbox.

## [0.12.2] - 2021-06-11
### Fixed
- `Face::glyph_bounding_box` for variable `glyf`.
- (`glyf`) Do not skip glyphs with zero-sized bbox.

## [0.12.1] - 2021-05-24
### Added
- Support Format 13 subtables in `cmap::Subtable::is_unicode`.
  Thanks to [csmulhern](https://github.com/csmulhern)
- Derive more traits by default. Thanks to [dhardy](https://github.com/dhardy)

## [0.12.0] - 2021-02-14
### Changed
- `Face::ascender` and `Face::descender` will use
  [usWinAscent](https://docs.microsoft.com/en-us/typography/opentype/spec/os2#uswinascent) and
  [usWinDescent](https://docs.microsoft.com/en-us/typography/opentype/spec/os2#uswindescent)
  when `USE_TYPO_METRICS` flag is not set in `OS/2` table.
  Previously, those values were ignored and
  [hhea::ascender](https://docs.microsoft.com/en-us/typography/opentype/spec/hhea#ascender) and
  [hhea::descender](https://docs.microsoft.com/en-us/typography/opentype/spec/hhea#descender)
  were used. Now `hhea` table values will be used only when `OS/2` table is not present.
- `Face::outline_glyph` and `Face::glyph_bounding_box` in case of a `glyf` table
  can fallback to a calculated bbox when the embedded bbox is malformed now.

## [0.11.0] - 2021-02-04
### Added
- `FaceTables`, which allowed to load `Face` not only from a single chunk of data,
  but also in a per-table way. Which is useful for WOFF parsing.
  No changes to the API.
  Thanks to [fschutt](https://github.com/fschutt)

## [0.10.1] - 2021-01-21
### Changed
- Update a font used for tests.

## [0.10.0] - 2021-01-16
### Added
- `variable-fonts` build feature. Enabled by default.
  By disabling it you can reduce `ttf-parser` binary size overhead almost twice.

### Changed
- (`gvar`) Increase the maximum number of variation tuples from 16 to 32.
  Increases stack usage and makes `gvar` parsing 10% slower now.

### Fixed
- (`CFF`) Fix `seac` processing. Thanks to [wezm](https://github.com/wezm)

## [0.9.0] - 2020-12-05
### Removed
- `kern` AAT subtable 1 aka `kern::state_machine`.
  Mainly because it's useless without a proper shaping.

## [0.8.3] - 2020-11-15
### Added
- `Face::glyph_variation_delta`

### Fixed
- `Iterator::nth` implementation for `cmap::Subtables` and `Names`.

## [0.8.2] - 2020-07-31
### Added
- `cmap::Subtable::codepoints`

### Fixed
- (cmap) Incorrectly returning glyph ID `0` instead of `None` for format 0
- (cmap) Possible invalid glyph mapping for format 2

## [0.8.1] - 2020-07-29
### Added
- `Face::is_monospaced`
- `Face::italic_angle`
- `Face::typographic_ascender`
- `Face::typographic_descender`
- `Face::typographic_line_gap`
- `Face::capital_height`

## [0.8.0] - 2020-07-21
### Added
- Allow `true` magic.
- `FaceParsingError`
- `NormalizedCoordinate`
- `Face::variation_coordinates`
- `Face::has_non_default_variation_coordinates`
- `Face::glyph_name` can lookup CFF names too.
- `Face::table_data`
- `Face::character_mapping_subtables`

### Changed
- (CFF,CFF2) 10% faster parsing.
- `Face::from_slice` returns `Result` now.
- `Name::platform_id` returns `PlatformId` instead of `Option<PlatformId>` now.
- The `cmap` module became public.

### Fixed
- `Face::width` parsing.
- Possible u32 overflow on 32-bit platforms during `Face::from_slice`.
- (cmap) `Face::glyph_variation_index` processing when the encoding table has only one glyph.

## [0.7.0] - 2020-07-16
### Added
- (CFF) CID fonts support.
- (CFF) `seac` support.
- `Font::global_bounding_box`

### Changed
- Rename `Font` to `Face`, because this is what it actually is.
- Rename `Font::from_data` to `Font::from_slice` to match serde and other libraries.
- Rename `Name::name_utf8` to `Name::to_string`.

### Removed
- `Font::family_name` and `Font::post_script_name`. They were a bit confusing.
  Prefer:
  ```
  face.names().find(|name| name.name_id() == name_id::FULL_NAME).and_then(|name| name.to_string())
  ```

## [0.6.2] - 2020-07-02
### Added
- `Name::is_unicode`
- `Font::family_name` will load names with Windows Symbol encoding now.

### Fixed
- `Font::glyph_bounding_box` will apply variation in case of `gvar` fonts.

## [0.6.1] - 2020-05-19
### Fixed
- (`kern`) Support fonts that ignore the subtable size limit.

## [0.6.0] - 2020-05-18
### Added
- `sbix`, `CBLC`, `CBDT` and `SVG` tables support.
- `Font::glyph_raster_image` and `Font::glyph_svg_image`.
- `Font::kerning_subtables` with subtable formats 0..3 support.

### Changed
- (c-api) The library doesn't allocate `ttfp_font` anymore. All allocations should be
  handled by the caller from now.

### Removed
- `Font::glyphs_kerning`. Use `Font::kerning_subtables` instead.
- (c-api) `ttfp_create_font` and `ttfp_destroy_font`.
  Use `ttfp_font_size_of` + `ttfp_font_init` instead.
  ```c
  ttfp_font *font = (ttfp_font*)alloca(ttfp_font_size_of());
  ttfp_font_init(font_data, font_data_size, 0, font);
  ```
- Logging support. We haven't used it anyway.

### Fixed
- (`gvar`) Integer overflow.
- (`cmap`) Integer overflow during subtable format 2 parsing.
- (`CFF`, `CFF2`) DICT number parsing.
- `Font::glyph_*_advance` will return `None` when glyph ID
  is larger than the number of metrics in the table.
- Ignore variation offset in `Font::glyph_*_advance` and `Font::glyph_*_side_bearing`
  when `HVAR`/`VVAR` tables are missing.
  Previously returned `None` which is incorrect.

## [0.5.0] - 2020-03-19
### Added
- Variable fonts support.
- C API.
- `gvar`, `CFF2`, `avar`, `fvar`, `HVAR`, `VVAR` and `MVAR` tables support.
- `Font::variation_axes`
- `Font::set_variation`
- `Font::is_variable`
- `Tag` type.

### Fixed
- Multiple issues due to arithmetic overflow.

## [0.4.0] - 2020-02-24

**A major rewrite.**

### Added
- `Font::glyph_bounding_box`
- `Font::glyph_name`
- `Font::has_glyph_classes`
- `Font::glyph_class`
- `Font::glyph_mark_attachment_class`
- `Font::is_mark_glyph`
- `Font::glyph_y_origin`
- `Font::vertical_ascender`
- `Font::vertical_descender`
- `Font::vertical_height`
- `Font::vertical_line_gap`
- Optional `log` dependency.

### Changed
- `Font::outline_glyph` now accepts `&mut dyn OutlineBuilder` and not `&mut impl OutlineBuilder`.
- `Font::ascender`, `Font::descender` and `Font::line_gap` will check `USE_TYPO_METRICS`
  flag in OS/2 table now.
- `glyph_hor_metrics` was split into `glyph_hor_advance` and `glyph_hor_side_bearing`.
- `glyph_ver_metrics` was split into `glyph_ver_advance` and `glyph_ver_side_bearing`.
- `CFFError` is no longer public.

### Removed
- `Error` enum. All methods will return `Option<T>` now.
- All `unsafe`.

### Fixed
- `glyph_hor_side_bearing` parsing when the number of metrics is less than the total number of glyphs.
- Multiple CFF parsing fixes. The parser is more strict now.

## [0.3.0] - 2019-09-26
### Added
- `no_std` compatibility.

### Changed
- The library has one `unsafe` block now.
- 35% faster `family_name()` method.
- 25% faster `from_data()` method for TrueType fonts.
- The `Name` struct has a new API. Public fields became public functions
  and data is parsed on demand and not beforehand.

## [0.2.2] - 2019-08-12
### Fixed
- Allow format 12 subtables with *Unicode full repertoire* in `cmap`.

## [0.2.1] - 2019-08-12
### Fixed
- Check that `cmap` subtable encoding is Unicode.

## [0.2.0] - 2019-07-10
### Added
- CFF support.
- Basic kerning support.
- All `cmap` subtable formats except Mixed Coverage (8) are supported.
- Vertical metrics querying from the `vmtx` table.
- OpenType fonts are allowed now.

### Changed
- A major rewrite. TrueType tables are no longer public.
- Use `GlyphId` instead of `u16`.

### Removed
- `GDEF` table parsing.

[Unreleased]: https://github.com/harfbuzz/ttf-parser/compare/v0.25.1...HEAD
[0.25.1]: https://github.com/harfbuzz/ttf-parser/compare/v0.25.0...v0.25.1
[0.25.0]: https://github.com/harfbuzz/ttf-parser/compare/v0.24.1...v0.25.0
[0.24.1]: https://github.com/harfbuzz/ttf-parser/compare/v0.24.0...v0.24.1
[0.24.0]: https://github.com/harfbuzz/ttf-parser/compare/v0.23.0...v0.24.0
[0.23.0]: https://github.com/harfbuzz/ttf-parser/compare/v0.22.0...v0.23.0
[0.22.0]: https://github.com/harfbuzz/ttf-parser/compare/v0.21.1...v0.22.0
[0.21.1]: https://github.com/harfbuzz/ttf-parser/compare/v0.21.0...v0.21.1
[0.21.0]: https://github.com/harfbuzz/ttf-parser/compare/v0.20.0...v0.21.0
[0.20.0]: https://github.com/harfbuzz/ttf-parser/compare/v0.19.2...v0.20.0
[0.19.2]: https://github.com/harfbuzz/ttf-parser/compare/v0.19.1...v0.19.2
[0.19.1]: https://github.com/harfbuzz/ttf-parser/compare/v0.19.0...v0.19.1
[0.19.0]: https://github.com/harfbuzz/ttf-parser/compare/v0.18.1...v0.19.0
[0.18.1]: https://github.com/harfbuzz/ttf-parser/compare/v0.18.0...v0.18.1
[0.18.0]: https://github.com/harfbuzz/ttf-parser/compare/v0.17.0...v0.18.0
[0.17.0]: https://github.com/harfbuzz/ttf-parser/compare/v0.16.0...v0.17.0
[0.16.0]: https://github.com/harfbuzz/ttf-parser/compare/v0.15.2...v0.16.0
[0.15.2]: https://github.com/harfbuzz/ttf-parser/compare/v0.15.1...v0.15.2
[0.15.1]: https://github.com/harfbuzz/ttf-parser/compare/v0.15.0...v0.15.1
[0.15.0]: https://github.com/harfbuzz/ttf-parser/compare/v0.14.0...v0.15.0
[0.14.0]: https://github.com/harfbuzz/ttf-parser/compare/v0.13.4...v0.14.0
[0.13.4]: https://github.com/harfbuzz/ttf-parser/compare/v0.13.3...v0.13.4
[0.13.3]: https://github.com/harfbuzz/ttf-parser/compare/v0.13.2...v0.13.3
[0.13.2]: https://github.com/harfbuzz/ttf-parser/compare/v0.13.1...v0.13.2
[0.13.1]: https://github.com/harfbuzz/ttf-parser/compare/v0.13.0...v0.13.1
[0.13.0]: https://github.com/harfbuzz/ttf-parser/compare/v0.12.3...v0.13.0
[0.12.3]: https://github.com/harfbuzz/ttf-parser/compare/v0.12.2...v0.12.3
[0.12.2]: https://github.com/harfbuzz/ttf-parser/compare/v0.12.1...v0.12.2
[0.12.1]: https://github.com/harfbuzz/ttf-parser/compare/v0.12.0...v0.12.1
[0.12.0]: https://github.com/harfbuzz/ttf-parser/compare/v0.11.0...v0.12.0
[0.11.0]: https://github.com/harfbuzz/ttf-parser/compare/v0.10.1...v0.11.0
[0.10.1]: https://github.com/harfbuzz/ttf-parser/compare/v0.10.0...v0.10.1
[0.10.0]: https://github.com/harfbuzz/ttf-parser/compare/v0.9.0...v0.10.0
[0.9.0]: https://github.com/harfbuzz/ttf-parser/compare/v0.8.3...v0.9.0
[0.8.3]: https://github.com/harfbuzz/ttf-parser/compare/v0.8.2...v0.8.3
[0.8.2]: https://github.com/harfbuzz/ttf-parser/compare/v0.8.1...v0.8.2
[0.8.1]: https://github.com/harfbuzz/ttf-parser/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/harfbuzz/ttf-parser/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/harfbuzz/ttf-parser/compare/v0.6.2...v0.7.0
[0.6.2]: https://github.com/harfbuzz/ttf-parser/compare/v0.6.1...v0.6.2
[0.6.1]: https://github.com/harfbuzz/ttf-parser/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/harfbuzz/ttf-parser/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/harfbuzz/ttf-parser/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/harfbuzz/ttf-parser/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/harfbuzz/ttf-parser/compare/v0.2.2...v0.3.0
[0.2.2]: https://github.com/harfbuzz/ttf-parser/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/harfbuzz/ttf-parser/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/harfbuzz/ttf-parser/compare/v0.1.0...v0.2.0
