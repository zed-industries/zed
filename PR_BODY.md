# edit_prediction: Fix UTF-8 char boundary panic in zeta1 model output cleaning

## Crash Summary

The `clean_zeta1_model_output` function in `zeta_prompt` panics when processing model output containing multi-byte UTF-8 characters (e.g., Russian Cyrillic text). The crash manifests as:

```
byte index <int> is not a char boundary; it is inside 'ч' (bytes <int>..<int>)
```

**Sentry Issue:** [ZED-4VT](https://sentry.io/organizations/zed-dev/issues/7243338399/) — 236 events between 2026-02-05 and 2026-02-19, primarily on Windows stable channel (v0.223.3).

## Root Cause

When the model output contains multi-byte UTF-8 characters and a cursor marker, the function calculates a cursor offset based on byte arithmetic from the original string. This offset is then used to slice the extracted content, but the offset may fall inside a multi-byte character boundary, causing a panic.

The problematic code path:
1. Find cursor marker position (`zeta1_cursor_pos`) as a byte offset
2. Calculate `offset_in_extracted` via byte arithmetic
3. Slice `extracted[..offset]` — **PANIC** when `offset` is not a char boundary

## Fix

Use `floor_char_boundary()` to ensure the cursor offset is a valid character boundary before slicing:

```rust
let safe_offset = extracted.floor_char_boundary(offset);
result.push_str(&extracted[..safe_offset]);
result.push_str(super::CURSOR_MARKER);
result.push_str(&extracted[safe_offset..]);
```

This rounds down to the nearest character boundary, preventing the panic while preserving cursor placement accuracy to the nearest character.

## Validation

- Added `test_clean_zeta1_model_output_multibyte_cursor` — tests cursor placement with multi-byte UTF-8 text (Russian)
- Added `test_clean_zeta1_model_output_multibyte_cursor_midchar` — tests edge case where cursor could land mid-character
- All 18 tests in `zeta_prompt` crate pass
- Clippy passes with `--deny warnings`

```
cargo test -p zeta_prompt
cargo clippy -p zeta_prompt --all-targets --all-features -- --deny warnings
```

## Potentially Related Issues

### High/Medium Confidence
- None found in `zed-industries/zed`

### Low Confidence  
- [#27164](https://github.com/zed-industries/zed/issues/27164) — Different crash in edit predictions (inline assistant trigger)
- [#46880](https://github.com/zed-industries/zed/issues/46880) — Edit predictions broken (different symptom)

## Reviewer Checklist

- [ ] Verify the fix correctly handles the crash scenario
- [ ] Confirm `floor_char_boundary` is appropriate (rounds down to nearest char boundary)
- [ ] Check that cursor placement accuracy is acceptable (places cursor at character boundary before intended position)
- [ ] Verify no regression in existing test cases

Release Notes:

- Fixed a crash when using edit predictions with text containing multi-byte UTF-8 characters (e.g., Russian, Chinese, Japanese)
