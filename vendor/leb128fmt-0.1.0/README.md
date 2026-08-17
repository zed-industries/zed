# Leb128fmt

Leb128fmt is a library to decode and encode [LEB128][leb128_wiki] formatted numbers.
LEB128 is a variable length integer compression format.

* [Latest API Documentation][api_docs]

The library does not allocate memory and can be used in `no_std` and
`no_std::no_alloc` environments.

Various functions are provided which encode and decode signed and unsigned
integers with the number of bits in the function name. There are generic
functions provided to read and write slices of encoded values as well.

There are encoding functions with the word `fixed` in the name which will
write out a value using the maximum number of bytes for a given bit size.
For instance, using `encode_fixed_u32` will always use 5 bytes to
write out the value. While always using the maximum number of bytes removes
the benefit of compression, in some scenarios, it is beneficial to have a
fixed encoding size.

Finally, there are macros provided which you can use to build your own
encoding and decoding functions for unusual variants like signed 33 bit
values.

## Installation

```sh
cargo add leb128fmt
```

## Examples

### Functions using Arrays

```rust
// Encode an unsigned 32 bit number:
let (output, written_len) = leb128fmt::encode_u32(43110).unwrap();
// The number of bytes written in the output array
assert_eq!(written_len, 3);
assert_eq!(&output[..written_len], &[0xE6, 0xD0, 0x02]);
// The entire output array. Note you should only use &output[..written_len] to copy
// into your output buffer
assert_eq!(output, [0xE6, 0xD0, 0x02, 0x00, 0x00]);

// Decode an unsigned 32 bit number:
let input = [0xE6, 0xD0, 0x02, 0x00, 0x00];
let (result, read_len) = leb128fmt::decode_u32(input).unwrap();
assert_eq!(result, 43110);
assert_eq!(read_len, 3);
```

#### Helper Functions

If you are reading from an input buffer, you can use `is_last` and
`max_len` to determine the bytes to copy into the array.

```rust
let buffer = vec![0xFE, 0xFE, 0xE6, 0xD0, 0x02, 0xFE, 0xFE, 0xFE];
let pos = 2;
let end = buffer.iter().skip(pos).copied().position(leb128fmt::is_last).map(|p| pos + p);
if let Some(end) = end {
    if end <= pos + leb128fmt::max_len::<32>() {
        let mut input = [0u8; leb128fmt::max_len::<32>()];
        input[..=end - pos].copy_from_slice(&buffer[pos..=end]);
        let (result, read_len) = leb128fmt::decode_u32(input).unwrap();
        assert_eq!(result, 43110);
        assert_eq!(read_len, 3);
    } else {
        // invalid LEB128 encoding
        panic!();
    }
} else {
  if buffer.len() - pos < leb128fmt::max_len::<32>() {
     // Need more bytes in the buffer
     panic!();
  } else {
     // invalid LEB128 encoding
     panic!();
  }
}

```

### Functions Using Slices

```rust
let mut buffer = vec![0xFE; 10];
let mut pos = 1;

// Encode an unsigned 64 bit number with a mutable slice:
let result = leb128fmt::encode_uint_slice::<u64, 64>(43110u64, &mut buffer, &mut pos);
// The number of bytes written in the output array
assert_eq!(result, Some(3));
assert_eq!(pos, 4);

assert_eq!(buffer, [0xFE, 0xE6, 0xD0, 0x02, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE, 0xFE]);

// Decode an unsigned 64 bit number with a slice:
pos = 1;
let result = leb128fmt::decode_uint_slice::<u64, 64>(&buffer, &mut pos);
assert_eq!(result, Ok(43110));
assert_eq!(pos, 4);
```

### Functions Using Fixed Sized Encoding

There may be several different ways to encode a value. For instance, `0` can
be encoded as 32 bits unsigned:

```rust
let mut pos = 0;
assert_eq!(leb128fmt::decode_uint_slice::<u32, 32>(&[0x00], &mut pos), Ok(0));
pos = 0;
assert_eq!(leb128fmt::decode_uint_slice::<u32, 32>(&[0x80, 0x00], &mut pos), Ok(0));
pos = 0;
assert_eq!(leb128fmt::decode_uint_slice::<u32, 32>(&[0x80, 0x80, 0x00], &mut pos), Ok(0));
pos = 0;
assert_eq!(leb128fmt::decode_uint_slice::<u32, 32>(&[0x80, 0x80, 0x80, 0x00], &mut pos), Ok(0));
pos = 0;
assert_eq!(leb128fmt::decode_uint_slice::<u32, 32>(&[0x80, 0x80, 0x80, 0x80, 0x00], &mut pos), Ok(0));
```

There are functions provided to encode a value using the maximum number of
bytes possible for a given bit size. Using the maximum number of bytes
removes the benefit of compression, but it may be useful in a few scenarios.

For instance, if a binary format needs to store the size or offset of some
data before the size of data is known, it can be beneficial to write a fixed
sized `0` placeholder value first. Then, once the real value is known, the
`0` placeholder can be overwritten without moving other bytes. The real
value is also written out using the fixed maximum number of bytes.

```rust
// Encode an unsigned 32 bit number with all 5 bytes:
let output  = leb128fmt::encode_fixed_u32(43110).unwrap();
assert_eq!(output, [0xE6, 0xD0, 0x82, 0x80, 0x00]);

// Decode an unsigned 32 bit number:
let input = output;
let (result, read_len) = leb128fmt::decode_u32(input).unwrap();
assert_eq!(result, 43110);

// Note that all 5 bytes are read
assert_eq!(read_len, 5);
```

## License

Licensed under either of [Apache License, Version 2.0][LICENSE_APACHE] or [MIT
License][LICENSE_MIT] at your option.

### Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[LICENSE_APACHE]: LICENSE-APACHE
[LICENSE_MIT]: LICENSE-MIT
[leb128_wiki]: https://en.wikipedia.org/wiki/LEB128
[api_docs]: https://docs.rs/leb128fmt/