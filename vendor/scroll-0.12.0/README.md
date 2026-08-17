[![Actions][actions-badge]][actions-url]
[![crates.io version][crates-scroll-badge]][crates-scroll]

<!-- Badges' links -->

[actions-badge]: https://github.com/m4b/scroll/workflows/CI/badge.svg?branch=master
[actions-url]: https://github.com/m4b/scroll/actions
[crates-scroll-badge]: https://img.shields.io/crates/v/scroll.svg
[crates-scroll]: https://crates.io/crates/scroll

## Scroll - cast some magic

```text
         _______________
    ()==(              (@==()
         '______________'|
           |             |
           |   ἀρετή     |
         __)_____________|
    ()==(               (@==()
         '--------------'

```

### Documentation

https://docs.rs/scroll

### Usage

Add to your `Cargo.toml`

```toml, no_test
[dependencies]
scroll = "0.11"
```

### Overview

Scroll implements several traits for read/writing generic containers (byte buffers are currently implemented by default). Most familiar will likely be the `Pread` trait, which at its basic takes an immutable reference to self, an immutable offset to read at, (and a parsing context, more on that later), and then returns the deserialized value.

Because self is immutable, _**all** reads can be performed in parallel_ and hence are trivially parallelizable.

A simple example demonstrates its flexibility:

```rust
use scroll::{ctx, Pread, LE};

fn main() -> Result<(), scroll::Error> {
    let bytes: [u8; 4] = [0xde, 0xad, 0xbe, 0xef];

    // reads a u32 out of `b` with the endianness of the host machine, at offset 0, turbofish-style
    let number: u32 = bytes.pread::<u32>(0)?;
    // ...or a byte, with type ascription on the binding.
    let byte: u8 = bytes.pread(0)?;

    //If the type is known another way by the compiler, say reading into a struct field, we can omit the turbofish, and type ascription altogether!

    // If we want, we can explicitly add a endianness to read with by calling `pread_with`.
    // The following reads a u32 out of `b` with Big Endian byte order, at offset 0
    let be_number: u32 = bytes.pread_with(0, scroll::BE)?;
    // or a u16 - specify the type either on the variable or with the beloved turbofish
    let be_number2 = bytes.pread_with::<u16>(2, scroll::BE)?;

    // Scroll has core friendly errors (no allocation). This will have the type `scroll::Error::BadOffset` because it tried to read beyond the bound
    let byte: scroll::Result<i64> = bytes.pread(0);

    // Scroll is extensible: as long as the type implements `TryWithCtx`, then you can read your type out of the byte array!

    // We can parse out custom datatypes, or types with lifetimes
    // if they implement the conversion trait `TryFromCtx`; here we parse a C-style \0 delimited &str (safely)
    let hello: &[u8] = b"hello_world\0more words";
    let hello_world: &str = hello.pread(0)?;
    assert_eq!("hello_world", hello_world);

    // ... and this parses the string if its space separated!
    use scroll::ctx::*;
    let spaces: &[u8] = b"hello world some junk";
    let world: &str = spaces.pread_with(6, StrCtx::Delimiter(SPACE))?;
    assert_eq!("world", world);
    Ok(())
}
```

### Deriving `Pread` and `Pwrite`

Scroll implements a custom derive that can provide `Pread` and `Pwrite` implementations for your structs.

```rust
use scroll::{Pread, Pwrite, BE};

#[derive(Pread, Pwrite)]
struct Data {
    one: u32,
    two: u16,
    three: u8,
}

fn main() -> Result<(), scroll::Error> {
    let bytes: [u8; 7] = [0xde, 0xad, 0xbe, 0xef, 0xfa, 0xce, 0xff];
    // Read a single `Data` at offset zero in big-endian byte order.
    let data: Data = bytes.pread_with(0, BE)?;
    assert_eq!(data.one, 0xdeadbeef);
    assert_eq!(data.two, 0xface);
    assert_eq!(data.three, 0xff);

    // Write it back to a buffer
    let mut out: [u8; 7] = [0; 7];
    out.pwrite_with(data, 0, BE)?;
    assert_eq!(bytes, out);
    Ok(())
}
```

This feature is **not** enabled by default, you must enable the `derive` feature in Cargo.toml to use it:

```toml, no_test
[dependencies]
scroll = { version = "0.10", features = ["derive"] }
```

# `std::io` API

Scroll can also read/write simple types from a `std::io::Read` or `std::io::Write` implementor. The  built-in numeric types are taken care of for you.  If you want to read a custom type, you need to implement the `FromCtx` (_how_ to parse) and `SizeWith` (_how_ big the parsed thing will be) traits.  You must compile with default features. For example:

```rust
use std::io::Cursor;
use scroll::IOread;

fn main() -> Result<(), scroll::Error> {
    let bytes_ = [0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00, 0xef,0xbe,0x00,0x00,];
    let mut bytes = Cursor::new(bytes_);

    // this will bump the cursor's Seek
    let foo = bytes.ioread::<usize>()?;
    // ..ditto
    let bar = bytes.ioread::<u32>()?;
    Ok(())
}
```

Similarly, we can write to anything that implements `std::io::Write` quite naturally:

```rust
use scroll::{IOwrite, LE, BE};
use std::io::{Write, Cursor};

fn main() -> Result<(), scroll::Error> {
    let mut bytes = [0x0u8; 10];
    let mut cursor = Cursor::new(&mut bytes[..]);
    cursor.write_all(b"hello")?;
    cursor.iowrite_with(0xdeadbeef as u32, BE)?;
    assert_eq!(cursor.into_inner(), [0x68, 0x65, 0x6c, 0x6c, 0x6f, 0xde, 0xad, 0xbe, 0xef, 0x0]);
    Ok(())
}
```

# Advanced Uses

Scroll is designed to be highly configurable - it allows you to implement various context (`Ctx`) sensitive traits, which then grants the implementor _automatic_ uses of the `Pread` and/or `Pwrite` traits.

For example, suppose we have a datatype and we want to specify how to parse or serialize this datatype out of some arbitrary
byte buffer. In order to do this, we need to provide a [TryFromCtx](trait.TryFromCtx.html) impl for our datatype.

In particular, if we do this for the `[u8]` target, using the convention `(usize, YourCtx)`, you will automatically get access to
calling `pread_with::<YourDatatype>` on arrays of bytes.

```rust
use scroll::{ctx, Pread, BE, Endian};

struct Data<'a> {
  name: &'a str,
  id: u32,
}

// note the lifetime specified here
impl<'a> ctx::TryFromCtx<'a, Endian> for Data<'a> {
  type Error = scroll::Error;
  // and the lifetime annotation on `&'a [u8]` here
  fn try_from_ctx (src: &'a [u8], endian: Endian)
    -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let name = src.gread::<&str>(offset)?;
    let id = src.gread_with(offset, endian)?;
    Ok((Data { name: name, id: id }, *offset))
  }
}

fn main() -> Result<(), scroll::Error> {
    let bytes = b"UserName\x00\x01\x02\x03\x04";
    let data = bytes.pread_with::<Data>(0, BE)?;
    assert_eq!(data.id, 0x01020304);
    assert_eq!(data.name.to_string(), "UserName".to_string());
    Ok(())
}
```

Please see the official documentation, or a simple [example](examples/data_ctx.rs) for more.

# Contributing

Any ideas, thoughts, or contributions are welcome!
