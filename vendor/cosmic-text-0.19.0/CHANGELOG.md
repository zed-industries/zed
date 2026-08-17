# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.19.0] - 2026-04-22

### Fixed

- Empty lines use span metrics: https://github.com/pop-os/cosmic-text/pull/479
- Match variable fonts using wght axis: https://github.com/pop-os/cosmic-text/pull/486
- Used normalized\_coords for swash scaler: https://github.com/pop-os/cosmic-text/pull/488
- Fix highlight for editor: https://github.com/pop-os/cosmic-text/pull/491
- Optimize shape\_until\_scroll with large line count: https://github.com/pop-os/cosmic-text/pull/490
- Fix shape\_until\_scroll if a buffer\_line is modified: https://github.com/pop-os/cosmic-text/pull/494
- Remove 1px horizontal offset on pixel fonts: https://github.com/pop-os/cosmic-text/pull/495
- Make the swash feature compile with no\_std: https://github.com/pop-os/cosmic-text/pull/492
- Fallback to a default font in basic shaping mode: https://github.com/pop-os/cosmic-text/pull/498
- Clamp scroll.line to a valid range: https://github.com/pop-os/cosmic-text/pull/501

### Added

- Text decoration: https://github.com/pop-os/cosmic-text/pull/480
- Implement layout\_runs for BufferLine: https://github.com/pop-os/cosmic-text/pull/484
- Add cursor\_position and is\_rtl methods to buffer: https://github.com/pop-os/cosmic-text/pull/496

### Changed

- Buffer setter methods are now lazy: https://github.com/pop-os/cosmic-text/pull/483
- Update hashbrown: https://github.com/pop-os/cosmic-text/pull/502

## [0.18.2] - 2026-02-20

### Fixed

- Fixes for ellipsize

## [0.18.1] - 2026-02-20

### Fixed

- Fix aggressive ellipsizing

## [0.18.0] - 2026-02-19

### Added

- Support for ellipsizing at the start, middle, and end of a line

## [0.17.2] - 2026-02-18

### Fixed

- Motion::Home and Motion::End operate on unwrapped lines

## [0.17.1] - 2026-01-30

### Fixed

- Set correct rust-version to 1.89

### Changed

- Update all dependencies to latest versions

## [0.17.0] - 2026-01-29

### Fixed

- Fix variable font weight for SwashCache::get_outline_commands()
- Allow fallback to fonts with mismatched stretch or style
- Shape as fake italic if no matching italic font exists
- Prevent line break opportunities from splitting ligatures

### Changed

- Update all dependencies to latest versions

### Removed

- Attrs::matches was removed as it is not compatible with new fallback logic

## [0.16.0] - 2025-12-29

### Added

- Add `Renderer` trait for more flexible rendering of buffers and editors
- Make hinting configurable with `Hinting` enum

### Fixed

- Fix bench compilation
- Round x_advance to nearest monospace width when requested
- Do not use ASCII fast path when a word has incompatible spans

### Changed

- Update harfrust to 0.4.1
- Update skrifa to 0.39.0

## [0.15.0] - 2025-10-30

### Added

- Add DISABLE\_HINTING cache flag
- Variable font support
- Add pixel font flag
- Add ASCII fast path optimization to ShapeWord::build
- Optimize BidiParagraphs with ASCII fast path
- Add explicit lifetimes to borrowed return types
- Implement pixel-based scrolling for the Editor
- Add alignment paramater to set\_text

### Fixed

- Clip based on ascent and descent, not baseline
- Fix scroll when vertical offset is exactly layout\_height
- Do not ignore font family
- Transform outline if fake italic provided
- Fixed Tab indenting the line instead of adding Tab or spaces
- Update and fix cargo-deny
- Fix UDHR link
- If buffer is empty, do not set line ending
- Better handling of newlines in editor insert and delete
- Improve handling of non-existant files in load\_text
- Fix delete ranges removing interior newlines

### Changed

- fontdb updated to 0.23
- Replace rustybuzz with HarfRust
- Use linebender\_resource\_handle instead of peniko
- Upgrade skrifa to 0.37

## [0.14.2] - 2025-04-14

### Fixed

- Ensure MSRV of 1.75.0

## [0.14.1] - 2025-04-04

### Added

- Allow font to be stored as `peniko::Font` with `peniko` feature

## [0.14.0] - 2025-03-31

### Added

- Add configurable font fallback lists using `Fallback` trait
- Add letter spacing support in `Attrs`
- Add arbitrary variable font features in `Attrs`

## [0.13.2] - 2025-03-11

### Fixed

- Fix build for Android targets

## [0.13.1] - 2025-03-10

### Fixed

- Fix glyph start+end indices in `Basic` shaping

## [0.13.0] - 2025-03-10

### Added

- Add `Buffer::set_tab_width` function
- Add `AttrsList::spans_iter` and use it in `Buffer::append`
- Add alignment option to `Buffer::set_rich_text`
- Add `SwashCache::get_outline_commands_uncached`

### Fixed

- Fix typo in fallback font name `Noto Sans CJK JP`
- Fix the character index used for getting a glyph attribute in basic shaping
- Avoid debug assertion when handling `Motion::BufferEnd`
- Handle single wrapped line scrolling
- Reduce memory usage and loading time of FontSystem
- Resolve lints
- Use FreeMono as Braille script fallback
- Load fonts prior to setting defaults

### Changed

- Use `smol_str` for family name in `FamilyOwned`
- Optimize `Buffer::set_rich_text` for when the buffer is reconstructed
- Move `ShapeBuffer` to `FontSystem`
- Cache the monospace fallbacks buffer in `FontSystem`
- Apply fallback font to more Unix-like operating systems
- Use hinting for `swash_outline_commands`
- Update swash to `0.2.0` and hook up `std` feature
- Update minimum supported Rust version to `1.75`
- Update default fonts (for COSMIC, users should set their own as desired)

### Removed

- Drop `ShapePlanCache`

## [0.12.1] - 2024-06-31

### Changed

- Make collection of monospace fallback information optional

## [0.12.0] - 2024-06-18

### Added

- Cache codepoint support info for monospace fonts
- Store a sorted list of monospace font ids in font system
- Add line ending abstraction
- Horizontal scroll support in Buffer
- Concurrently load and parse fonts
- Add metrics to attributes
- Support expanding tabs
- Add an option to set selected text color
- Add Edit::cursor_position
- Allow layout to be calculated without specifying width
- Allow for undefined buffer width and/or height
- Add method to set syntax highlighting by file extension

### Fixed

- Fix no_std build
- Handle inverted Ranges in add_span
- Fix undo and redo updating editor modified status
- Ensure at least one line is in Buffer

### Changed

- Enable vi feature for docs.rs build
- Convert editor example to winit
- Refactor scrollbar width handling for editor example
- Convert rich-text example to winit
- Only try monospace fonts that support at least one requested script
- Skip trying monospace fallbacks if default font supports all codepoints
- Make vertical scroll by pixels instead of layout lines
- Upgrade dependencies and re-export ttf-parser

## [0.11.2] - 2024-02-08

### Fixed

- Fix glyph start and end when using `shape-run-cache`

## [0.11.1] - 2024-02-08

### Added

- Add `shape-run-cache` feature, that can significantly improve shaping performance

### Removed

- Remove editor-libcosmic, see cosmic-edit instead

## [0.11.0] - 2024-02-07

### Added

- Add function to set metrics and size simultaneously
- Cache `rustybuzz` shape plans
- Add capability to synthesize italic
- New wrapping option `WordOrGlyph` to allow word to glyph fallback

### Fixed

- `Buffer::set_rich_text`: Only add attrs if they do not match the defaults
- Do not use Emoji fonts as monospace fallback
- Refresh the attrs more often in basic shaping
- `Buffer`: fix max scroll going one line beyond end
- Improve reliability of `layout_cursor`
- Handle multiple BiDi paragraphs in `ShapeLine` gracefully
- Improved monospace font fallback
- Only commit a previous word range if we had an existing visual line

### Changed

- Update terminal example using `colored`
- Significant improvements for `Editor`, `SyntaxEditor`, and `ViEditor`
- Require default Attrs to be specified in `Buffer::set_rich_text`
- Bump `fontdb` to `0.16`
- Allow Clone of layout structs
- Move cursor motions to new `Motion` enum, move handling to `Buffer`
- Ensure that all shaping and layout uses scratch buffer
- `BufferLine`: user `layout_in_buffer` to implement layout
- `BufferLine`: remove wrap from struct, as wrap is passed to layout
- Refactor of scroll and shaping
- Move `color` and `x_opt` out of Cursor
- Add size limit to `font_matches_cache` and clear it when it is reached
- Update `swash` to `0.1.12`
- Set default buffer wrap to `WordOrGlyph`

## Removed
- Remove patch to load Redox system fonts, as fontdb does it now

## [0.10.0] - 2023-10-19

### Added

- Added `Buffer::set_rich_text` method
- Add `Align::End` for end-based alignment
- Add more `Debug` implementations
- Add feature to warn on missing glyphs
- Add easy conversions for tuples/arrays for `Color`
- Derive `Clone` for `AttrsList`
- Add feature to allow `fontdb` to get `fontconfig` information
- Add benchmarks to accurately gauge improvements
- Add image render tests
- Allow BSD-2-Clause and BSD-3-Clause licneses in cargo-deny

### Fixed

- Fix `no_std` build
- Fix `BufferLine::set_align` docs to not mention shape reset is performed
- Fix width computed during unconstrained layout and add test for it
- Set `cursor_moved` to true in `Editor::insert_string`
- Fix `NextWord` action in `Editor` when line ends with word boundaries
- Fix building `editor-libcosmic` with `vi` feature
- Respect `fontconfig` font aliases when enabled
- Fix rendering of RTL words

### Changed

- Unify `no_std` and `std` impls of `FontSystem`
- Move `hashbrown` behind `no_std` feature
- Require either `std` or `no_std` feature to be specified
- Use a scratch buffer to reduce allocations
- Enable `std` feature with `fontconfig` feature
- Enable `fontconfig` feature by default
- Refactor code in `ShapeLine::layout`
- Set MSRV to `1.65`
- Make `Edit::copy_selection` immutable
- Rewrite `PreviousWord` logic in `Editor` with iterators
- Use attributes at cursor position for insertions in `Editor`
- Update all dependencies
- Use `self_cell` for creating self-referential struct

## [0.9.0] - 2023-07-06

### Added

- Add `Shaping` enum to allow selecting the shaping strategy
- Add `Buffer::new_empty` to create `Buffer` without `FontSystem`
- Add `BidiParagraphs` iterator
- Allow setting `Cursor` color
- Allow setting `Editor` cursor
- Add `PhysicalGlyph` that allows computing `CacheKey` after layout
- Add light syntax highlighter to `libcosmic` example

### Fixed

- Fix WebAssembly support
- Fix alignment when not wrapping
- Fallback to monospaced font if Monospace family is not found
- Align glyphs in a `LayoutRun` to baseline

### Changed

- Update `fontdb` to 0.14.1
- Replace ouroboros with aliasable
- Use `BidiParagraphs` iterator instead of `str::Lines`
- Update `libcosmic` version

### Removed

- `LayoutGlyph` no longer has `x_int` and `y_int`, use `PhysicalGlyph` instead

## [0.8.0] - 2023-04-03

### Added

- `FontSystem::new_with_fonts` helper
- Alignment and justification
- `FontSystem::db_mut` provides mutable access to `fontdb` database
- `rustybuzz` is re-exported

### Fixed

- Fix some divide by zero panics
- Redox now uses `std` `FontSystem`
- Layout system improvements
- `BufferLinke::set_text` has been made more efficient
- Fix potential panic on window resize

### Changed

- Use `f32` instead of `i32` for lengths
- `FontSystem` no longer self-referencing
- `SwashCash` no longer keeps reference to `FontSystem`

### Removed

- `Attrs::monospaced` is removed, use `Family::Monospace` instead
