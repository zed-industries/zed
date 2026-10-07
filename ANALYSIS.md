# Crash Analysis: Tree-sitter reads inside a UTF-8 character

## Crash Summary

- **Linear issue:** FR-524
- **Sentry issue:** [ZED-C8E](https://zed-dev.sentry.io/issues/7772379476/)
- **Error:** `start byte index 69 is not a char boundary; it is inside 'и' (bytes 68..70 of string)`
- **Crash site:** `rope::Chunks::peek`, called by the input callback in `language::syntax_map::parse_text` while reparsing after an agent edit.

## Root Cause

Both language parser entry points sought a rope chunk to Tree-sitter's requested byte offset, sliced it as `str`, and only then converted it to bytes. A request inside a multibyte character therefore panicked before returning input to Tree-sitter.

Tree-sitter's input callback operates on byte offsets, not character boundaries. In the pinned Tree-sitter revision (`43623ec9bf0eaaf7113285c46e8a09018f181b18`), included-range validation checks byte ordering but not UTF-8 boundaries, and the lexer uses included-range start bytes directly when seeking and calling the input callback.

The crash report does not contain the edited document or grammar, so the exact sequence that caused the Windows WASM lexer to request byte 69 remains unknown. The regression test demonstrates the same first-party failure with a native Rust grammar and an included range starting inside `и`.

## Reproduction

`test_parse_text_inside_multibyte_character` parses text with included ranges starting inside two-, three-, and four-byte characters. It covers both zero and nonzero syntax-layer origins and compares the resulting syntax trees and byte ranges with Tree-sitter parsing contiguous bytes.

Before the fix, the minimal `// и\nfn main() {}` case with an included range starting at byte 4 aborted at `rope.rs:981`, with `start byte index 4 is not a char boundary; it is inside 'и' (bytes 3..5 of string)`.

Run the regression test:

```sh
cargo test -p language test_parse_text_inside_multibyte_character
```

## Fix

Add `Chunks::peek_bytes` and use it in both parser input callbacks. Slice the chunk as bytes, preserving the exact requested offset without allocating, rounding, or adding another rope lookup. Share the slice-range computation with the existing string and bitmap peeks; their UTF-8 boundary requirements remain unchanged.

`test_chunks_peek_bytes` checks every byte offset across multiple rope chunks in both seek directions, reversed chunks, bounded ranges, a range ending inside a character, and empty input.

## Verification

- The regression test reproduced the original panic before the fix and passed after it.
- `cargo test -p language`: 188 tests passed.
- `cargo test -p rope`: 25 tests passed.
- `./script/clippy -p language -p rope`: passed, including Cargo Shear and typo checks. The script skipped Buf because it was unavailable.
- The original Windows/WASM editing workflow was not reproduced.
