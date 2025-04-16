1. The `parse` and `parse_sync` functions must support both `Buffer` and `String` inputs for the `src` parameter, using the `Either` type from `napi` to avoid breaking existing string-based usage while adding buffer support.
2. A helper function `stringify` must handle conversion of `Either<Buffer, String>` to a unified `String` representation internally, ensuring consistent UTF-8 decoding for buffers and direct string passthrough.
3. The TypeScript binding declarations (`binding.d.ts`) must reflect the updated parameter types for `parse` and `parse_sync` to accept `Buffer | string`, ensuring compatibility with JavaScript/TypeScript callers.
4. Unit tests must validate both buffer and string input paths for asynchronous (`parse`) and synchronous (`parse_sync`) APIs, ensuring parity in functionality and output correctness.
5. The `filename` parameter must remain optional but use `FileName::Real` when provided and fall back to `FileName::Anon` if omitted, preserving existing file resolution logic.
6. No regressions in error handling, abort signal support, or serialization/deserialization of `ParseOptions` during the refactor.
