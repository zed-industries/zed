# Crash Analysis: cosmic_text BiDi assertion failure during text shaping on Linux

## Crash Summary
- **Sentry Issue:** ZED-3VR (https://sentry.io/organizations/zed-dev/issues/7083984873/)
- **Error:** `assertion 'left == right' failed` with `left: true, right: false` in `cosmic_text::shape::ShapeLine::build`
- **Crash Site:** `cosmic_text::shape::ShapeLine::build` at shape.rs line 1052
- **First Seen:** 2025-11-20
- **Event Count:** 120
- **Platform:** Linux (Wayland, AMD Radeon GPU)
- **Version:** 0.224.11+stable

## Root Cause

The crash occurs in the `cosmic_text` text shaping library (version 0.17.0) during bidirectional (BiDi) text processing. The assertion `assert_eq!(line_rtl, rtl)` at shape.rs:1052 fails when:

1. The text line's overall BiDi direction (`rtl`) is determined to be LTR (`false`)
2. But individual BiDi paragraphs within the line have RTL direction (`line_rtl = true`)

This happens when text contains certain sequences of characters that create a BiDi paragraph with different directionality than the overall line. The `unicode_bidi` crate used by `cosmic_text` can produce this state when:
- Text contains isolated RTL characters (Arabic, Hebrew) embedded in LTR text
- Text contains BiDi control characters that override or isolate directionality
- Text contains certain Unicode sequences that create separate BiDi paragraphs

The call path is:
```
editor::display_map::DisplaySnapshot::layout_row
  → gpui::text_system::WindowTextSystem::layout_line
  → gpui::text_system::line_layout::LineLayoutCache::layout_line
  → gpui::platform::linux::text_system::CosmicTextSystem::layout_line
  → cosmic_text::shape::ShapeLine::new
  → cosmic_text::shape::ShapeLine::build  ← PANIC HERE
```

The crash occurs during layout of editor lines when displaying or measuring text containing mixed-direction content.

## Analysis Details

The crash is in a third-party library (`cosmic_text`) that Zed cannot directly modify. The options for fixing this are:

1. **Upstream fix**: Wait for `cosmic_text` to fix the assertion in their BiDi handling
2. **Catch the panic**: Use `std::panic::catch_unwind` to catch the panic and return a fallback layout
3. **Sanitize input**: Pre-process text to remove problematic BiDi control characters before passing to `cosmic_text`
4. **Upgrade cosmic_text**: Check if a newer version fixes this issue

Looking at the cosmic_text repository (issue #442), this is a known problem related to how shaping runs are handled with BiDi text. The upstream fix would require changes to how `cosmic_text` handles BiDi paragraphs within a single line.

## Reproduction

The crash can be triggered by laying out text that contains:
- RTL characters (Arabic/Hebrew) mixed with LTR characters in a way that creates paragraph-level BiDi conflicts
- BiDi override or isolate control characters

Example text patterns that may trigger this:
- Arabic text embedded in English: `"Hello مرحبا World"`
- Hebrew text with numbers: `"שלום 123"`
- Text with BiDi control characters: U+202A (LRE), U+202B (RLE), U+202C (PDF), etc.

**Note**: A unit test could not be added because the panic occurs deep in the `cosmic_text` library during text shaping, which requires a full font system setup. The fix is defensive (catch_unwind) and is verified by:
1. Code compiles correctly
2. Clippy passes (no new warnings from the change)
3. The fallback layout function provides a reasonable degrade path

## Suggested Fix

Since the crash is in a third-party library, the safest fix is to catch the panic in Zed's `layout_line` function and return a fallback layout:

1. **Location**: `crates/gpui_linux/src/linux/text_system.rs` in `CosmicTextSystemState::layout_line`

2. **Approach**: Wrap the `ShapeLine::new()` call in `std::panic::catch_unwind`. If it panics, fall back to a simple layout that treats each grapheme as a single glyph without proper shaping.

3. **Tradeoffs**:
   - Pro: Prevents crash for users
   - Pro: Text still displays (though possibly with incorrect BiDi ordering)
   - Con: BiDi text may not render correctly when the panic is caught
   - Con: Using `catch_unwind` has some overhead

This is a defensive fix that prevents the crash while maintaining app stability. Proper BiDi support would require an upstream fix in `cosmic_text`.
