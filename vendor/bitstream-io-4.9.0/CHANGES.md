# 4.9.0

- Further extend VBR integer support

# 4.8.0

- Add io::Seek support to ByteReader

# 4.7.0

- Add support for LLVM-style variable bit rate integers

# 4.6.0

- Add no alloc support

# 4.5.0

- Implement new `CheckedSignedFixed` type alias
- Implement `FixedSignedBitCount` for use by `CheckedSignedFixed`

# 4.4.0

- Implement new `CheckedUnsignedFixed` type alias
- Implement `FixedBitCount` for use by `CheckedUnsignedFixed`

# 4.3.0

- Add `bytes` method to ToByteStream traits with default implementations

# 4.2.0

- Add `Checked` API for avoiding redundant validation during writes
- Add `BitCount`-related helper methods to optimize unary writes

# 4.1.0

- Implement `FromBitStreamUsing` convenience trait
- Implement `FromByteStreamUsing` convenience trait
- Implement `ToBitStreamUsing` convenience trait
- Implement `ToByteStreamUsing` convenience trait

# 4.0.0

- Implement `SignedBitCount` for signed integer types.
- Remove endianness requirement for `BitCounter`
- Optimize `BitRecorder` implementation to be much faster
- Optimize read/write_bytes to be much faster for un-aligned bytes
- Seal `Endianness` traits from further downstream implementation

# 3.2.0

- Implement `Integer` for `bool` and arrays of `Integer` types.
- Add a `read_const` method to `BitRead` for consisency with `BitWrite::write_const`.
- Widen unknown `BitCount` values to handle more edge cases.

# 3.1.0

- Implement `Integer` for the unsigned `NonZero` types.
- Add byte alignment convenience methods.
- Fix `BitCount.checked_sub` for consistency with `checked_add`.

