## The Rust MessagePack Library

RMP is a pure Rust [MessagePack](http://msgpack.org) implementation of an efficient binary
serialization format. This crate provides low-level core functionality, writers and readers for
primitive values with direct mapping between binary MessagePack format.

[Looking for Serde support](https://lib.rs/crates/rmp-serde)?

This crate represents the very basic functionality needed to work with MessagePack format.
Ideologically it is developed as a basis for building high-level abstractions.

### Usage

To use `rmp`, first add this to your `Cargo.toml`:

```toml
[dependencies.rmp]
rmp = "0.8"
```

### Features

- **Low-level API**

  RMP is designed to be lightweight and straightforward. There are low-level APIs, which give you
  full control over the encoding/decoding process. `no-std` environments are supported.

- **Zero-copy value decoding**

  RMP allows to decode bytes from a buffer in a zero-copy manner, without any heap allocations.
  easily and blazingly fast. Rust static checks guarantee that the data will be valid until buffer lives.

- **Clear error handling**

  RMP's error system guarantees that you never receive an error enum with unreachable variant.

- **Robust and tested**

  This project is developed using TDD and CI, so any found bugs will be fixed without breaking
  existing functionality.

### Detailed

Currently there are two large modules: encode and decode. More detail you can find in the
corresponding sections.

Formally every MessagePack message consists of some marker encapsulating a data type and the
data itself. Sometimes there are no separate data chunk, for example for booleans. In these
cases a marker contains the value. For example, the `true` value is encoded as `0xc3`.

```rust
let mut buf = Vec::new();
rmp::encode::write_bool(&mut buf, true).unwrap();

assert_eq!([0xc3], buf[..]);
```

Sometimes a single value can be encoded in multiple ways. For example a value of `42` can be
represented as: `[0x2a], [0xcc, 0x2a], [0xcd, 0x00, 0x2a]` and so on, and all of them are
considered as valid representations. To allow fine-grained control over encoding such values
the library provides direct mapping functions.

```rust
let mut bufs = vec![vec![]; 5];

rmp::encode::write_pfix(&mut bufs[0], 42).unwrap();
rmp::encode::write_u8(&mut bufs[1], 42).unwrap();
rmp::encode::write_u16(&mut bufs[2], 42).unwrap();
rmp::encode::write_u32(&mut bufs[3], 42).unwrap();
rmp::encode::write_u64(&mut bufs[4], 42).unwrap();

assert_eq!([0x2a], bufs[0][..]);
assert_eq!([0xcc, 0x2a], bufs[1][..]);
assert_eq!([0xcd, 0x00, 0x2a], bufs[2][..]);
assert_eq!([0xce, 0x00, 0x00, 0x00, 0x2a], bufs[3][..]);
assert_eq!([0xcf, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2a], bufs[4][..]);
```

But they aren't planned to be widely used. Instead we often need to encode bytes compactly to
save space. In these cases RMP provides functions that guarantee that for encoding the most
compact representation will be chosen.

```rust
let mut buf = Vec::new();

rmp::encode::write_sint(&mut buf, 300).unwrap();

assert_eq!([0xcd, 0x1, 0x2c], buf[..]);
```

On the other hand for deserialization it is not matter in which representation the value is
encoded - RMP deals with all of them.

Sometimes you know the exact type representation and want to enforce the deserialization process
to make it strongly type safe.

```rust
let buf = [0xcd, 0x1, 0x2c];

assert_eq!(300, rmp::decode::read_u16(&mut &buf[..]).unwrap());
```

However if you try to decode such bytearray as other integer type, for example `u32`, there will
be type mismatch error.

```rust
let buf = [0xcd, 0x1, 0x2c];
rmp::decode::read_u32(&mut &buf[..]).err().unwrap();
```

But sometimes all you want is just to encode an integer that *must* fit in the specified type
no matter how it was encoded. RMP provides [`such`][read_int] function to ease integration with
other MessagePack libraries.

```rust
let buf = [0xcd, 0x1, 0x2c];

assert_eq!(300i16, rmp::decode::read_int(&mut &buf[..]).unwrap());
assert_eq!(300i32, rmp::decode::read_int(&mut &buf[..]).unwrap());
assert_eq!(300i64, rmp::decode::read_int(&mut &buf[..]).unwrap());
assert_eq!(300u16, rmp::decode::read_int(&mut &buf[..]).unwrap());
assert_eq!(300u32, rmp::decode::read_int(&mut &buf[..]).unwrap());
assert_eq!(300u64, rmp::decode::read_int(&mut &buf[..]).unwrap());
```

### API

Almost all API are represented as pure functions, which accepts a generic `Write` or `Read` and
the value to be encoded/decoded. For example let's do a round trip for π number.

```rust
let pi = std::f64::consts::PI;
let mut buf = Vec::new();
rmp::encode::write_f64(&mut buf, pi).unwrap();

assert_eq!([0xcb, 0x40, 0x9, 0x21, 0xfb, 0x54, 0x44, 0x2d, 0x18], buf[..]);
assert_eq!(pi, rmp::decode::read_f64(&mut &buf[..]).unwrap());
```

License: MIT

[read_int]: https://docs.rs/rmp/latest/rmp/decode/fn.read_int.html
