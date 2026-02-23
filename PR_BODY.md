# Fix crash in cosmic_text BiDi text shaping on Linux

## Crash Summary

**Sentry Issue:** [ZED-3VR](https://sentry.io/organizations/zed-dev/issues/7083984873/) (120 events since 2025-11-20)

The crash occurs on Linux when laying out text containing certain mixed bidirectional (RTL/LTR) character sequences. The panic happens in the `cosmic_text` library with an assertion failure:

```
assertion `left == right` failed
  left: true
 right: false
```

at `cosmic_text::shape::ShapeLine::build` (shape.rs:1052)

## Root Cause

The crash is in the `cosmic_text` text shaping library (version 0.17.0). The assertion `assert_eq!(line_rtl, rtl)` fails when:
1. The text line's overall BiDi direction is determined to be LTR (`rtl = false`)
2. But individual BiDi paragraphs within the line have RTL direction (`line_rtl = true`)

This happens with certain sequences of Unicode characters that create BiDi paragraphs with different directionality than the overall line. The `unicode_bidi` crate used by `cosmic_text` can produce this state when text contains isolated RTL characters, BiDi control characters, or certain Unicode sequences.

This is a known upstream issue: [cosmic-text#442](https://github.com/pop-os/cosmic-text/issues/442)

## Fix

Since we cannot modify the third-party `cosmic_text` library, this PR adds defensive handling:

1. **Panic catching**: Wraps `ShapeLine::new()` in `std::panic::catch_unwind` to catch the assertion panic
2. **Fallback layout**: When a panic is caught, returns a fallback layout that treats each character as a single glyph without proper shaping

The fallback is intentionally simple - it allows the text to be displayed (preventing a crash) even though BiDi ordering may not be correct. This is a graceful degradation that keeps the application running.

## Validation

- [x] Code compiles with `cargo check -p gpui_linux --features wayland`
- [x] Clippy passes (only pre-existing warning in platform.rs)
- [x] Fallback layout provides ascent/descent from font metrics for proper vertical positioning

## Potentially Related Issues

### Medium Confidence
- [#39385](https://github.com/zed-industries/zed/pull/39385) — Fix displaying of RTL text (CLOSED)
- [#35613](https://github.com/zed-industries/zed/pull/35613) — Implement Bidirectionality (CLOSED)

### External References
- [cosmic-text#442](https://github.com/pop-os/cosmic-text/issues/442) — Allow breaking up shaping runs (upstream issue)
- [cosmic-text#252](https://github.com/pop-os/cosmic-text/issues/252) — Bidirectional text overflows buffer instead of wrapping

## Reviewer Checklist

- [ ] Verify the panic catch approach is acceptable for this use case
- [ ] Consider if the fallback layout quality is sufficient
- [ ] Check if `cosmic_text` has a newer version that might fix this upstream

Release Notes:

- Fixed a crash on Linux when editing files containing certain bidirectional text sequences (mixed RTL/LTR characters)
