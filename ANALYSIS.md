# Crash Analysis: UTF-8 char boundary panic in zeta1 model output cleaning

## Crash Summary
- **Sentry Issue:** ZED-4VT (https://sentry.io/organizations/zed-dev/issues/7243338399/)
- **Error:** `byte index <int> is not a char boundary; it is inside 'ч' (bytes <int>..<int>)`
- **Crash Site:** `zeta_prompt::zeta1::clean_zeta1_model_output` in `zeta_prompt.rs`
- **Events:** 236 occurrences between 2026-02-05 and 2026-02-19
- **Platform:** Primarily Windows, affects stable channel (v0.223.3)

## Root Cause

The `clean_zeta1_model_output` function panics when processing model output that contains multi-byte UTF-8 characters (like Russian Cyrillic text). The bug occurs when calculating the cursor position to insert in the cleaned output.

The problematic code flow:

1. The function receives model output containing text with multi-byte UTF-8 characters
2. It finds the cursor marker position (`zeta1_cursor_pos`) as a byte offset in the original string
3. It calculates `offset_in_extracted` based on byte arithmetic from the original string
4. When the function tries to slice `extracted[..offset]` and `extracted[offset..]`, the `offset` may fall inside a multi-byte UTF-8 character boundary, causing the panic

The specific crash happens at lines 1076-1078 in `zeta_prompt.rs`:
```rust
if let Some(offset) = cursor_offset {
    result.push_str(&extracted[..offset]);   // PANIC when offset is not a char boundary
    result.push_str(super::CURSOR_MARKER);
    result.push_str(&extracted[offset..]);
}
```

The calculation of `offset_in_extracted` doesn't account for the fact that:
- Byte positions in UTF-8 strings may land inside multi-byte characters
- The relationship between byte positions in the original output and the extracted content changes when content is removed

## Reproduction

The bug can be triggered when:
1. A user edits code containing multi-byte UTF-8 characters (e.g., Russian, Chinese, Japanese text in strings/comments)
2. The edit prediction system processes the content using the zeta1 model
3. The model output contains these multi-byte characters along with cursor markers
4. The calculated cursor offset lands inside a multi-byte character

Test command:
```
cargo test -p zeta_prompt test_clean_zeta1_model_output_multibyte_cursor
```

## Suggested Fix

Use `floor_char_boundary()` to ensure the cursor offset is a valid character boundary before slicing:

```rust
let mut result = String::with_capacity(extracted.len() + super::CURSOR_MARKER.len());
if let Some(offset) = cursor_offset {
    // Ensure offset is at a valid char boundary
    let safe_offset = extracted.floor_char_boundary(offset.min(extracted.len()));
    result.push_str(&extracted[..safe_offset]);
    result.push_str(super::CURSOR_MARKER);
    result.push_str(&extracted[safe_offset..]);
} else {
    result.push_str(extracted);
}
```

This is a minimal fix that:
1. Ensures the offset is within bounds (`.min(extracted.len())`)
2. Rounds down to the nearest char boundary using `floor_char_boundary()`
3. Prevents panics while preserving cursor placement accuracy to the nearest character

The `floor_char_boundary` function is available on `str` in Rust's standard library and is already used elsewhere in this codebase for similar UTF-8 safety concerns.
