<p align="center">
    <img src="https://raw.githubusercontent.com/rkyv/rkyv/master/media/logo_text_color.svg">
</p>
<p align="center">
    rkyv (<em>archive</em>) is a zero-copy deserialization framework for Rust
</p>
<p align="center">
    <a href="https://discord.gg/65F6MdnbQh">
        <img src="https://img.shields.io/discord/822925794249539645" alt="Discord">
    </a>
    <a href="https://docs.rs/rkyv">
        <img src="https://img.shields.io/docsrs/rkyv.svg">
    </a>
    <a href="https://crates.io/crates/rkyv">
        <img src="https://img.shields.io/crates/v/rkyv.svg">
    </a>
    <a href="https://github.com/rkyv/rkyv/blob/master/LICENSE">
        <img src="https://img.shields.io/badge/license-MIT-blue.svg">
    </a>
    <a href="https://blog.rust-lang.org/2021/05/06/Rust-1.52.0.html">
        <img src="https://img.shields.io/badge/rustc-1.52+-lightgray.svg">
    </a>
</p>

# Resources

## Learning Materials

- The [rkyv book](https://rkyv.github.io/rkyv) covers the motivation, architecture, and major
  features of rkyv
- The [rkyv discord](https://discord.gg/65F6MdnbQh) is a great place to get help with specific issues and meet
  other people using rkyv

## Documentation

- [rkyv](https://docs.rs/rkyv), the core library
- [rkyv_dyn](https://docs.rs/rkyv_dyn), which adds trait object support to rkyv
- [rkyv_typename](https://docs.rs/rkyv_typename), a type naming library

## Benchmarks

- The [rust serialization benchmark](https://github.com/djkoloski/rust_serialization_benchmark) is a
  shootout style benchmark comparing many rust serialization solutions. It includes special
  benchmarks for zero-copy serialization solutions like rkyv.

## Sister Crates

- [bytecheck](https://github.com/rkyv/bytecheck), which rkyv uses for validation
- [ptr_meta](https://github.com/rkyv/ptr_meta), which rkyv uses for pointer manipulation
- [rend](https://github.com/rkyv/rend), which rkyv uses for endian-agnostic features

# Example

```rust
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[archive(
    // This will generate a PartialEq impl between our unarchived and archived
    // types:
    compare(PartialEq),
    // bytecheck can be used to validate your data if you want. To use the safe
    // API, you have to derive CheckBytes for the archived type:
    check_bytes,
)]
// Derives can be passed through to the generated type:
#[archive_attr(derive(Debug))]
struct Test {
    int: u8,
    string: String,
    option: Option<Vec<i32>>,
}

let value = Test {
    int: 42,
    string: "hello world".to_string(),
    option: Some(vec![1, 2, 3, 4]),
};

// Serializing is as easy as a single function call
let bytes = rkyv::to_bytes::<_, 256>(&value).unwrap();

// Or you can customize your serialization for better performance
// and compatibility with #![no_std] environments
use rkyv::ser::{Serializer, serializers::AllocSerializer};

let mut serializer = AllocSerializer::<0>::default();
serializer.serialize_value(&value).unwrap();
let bytes = serializer.into_serializer().into_inner();

// You can use the safe API for fast zero-copy deserialization
let archived = rkyv::check_archived_root::<Test>(&bytes[..]).unwrap();
assert_eq!(archived, &value);

// Or you can use the unsafe API for maximum performance
let archived = unsafe { rkyv::archived_root::<Test>(&bytes[..]) };
assert_eq!(archived, &value);

// And you can always deserialize back to the original type
let deserialized: Test = archived.deserialize(&mut rkyv::Infallible).unwrap();
assert_eq!(deserialized, value);
```

_Note: the safe API requires the `validation` feature:_

```toml
rkyv = { version = "0.7", features = ["validation"] }
```

_Read more about [available features](https://docs.rs/rkyv/latest/rkyv/#features)._
