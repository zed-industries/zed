# aligned-vec

This crate provides the `AVec<T>` and `ABox<T>` types, which are intended to have a similar API
to `Vec<T>` and `Box<T>`, but align the data they contain to a runtime alignment value.

This is useful for situations where the alignment of the data matters, such as when working with
numerical data that can get performance benefits from being aligned to a SIMD-compatible memory address.

# Features

- `std` (default feature): Links this crate to the `std-crate` instead of the `core-crate`.
- `serde`: Implements serialization and deserialization features for `ABox` and `AVec`.
