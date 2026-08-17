<div align="center">

  <h1><code>Arbitrary</code></h1>

  <p><strong>The trait for generating structured data from arbitrary, unstructured input.</strong></p>

  <img alt="GitHub Actions Status" src="https://github.com/rust-fuzz/rust_arbitrary/workflows/Rust/badge.svg"/>

</div>

## About

The `Arbitrary` crate lets you construct arbitrary instances of a type.

This crate is primarily intended to be combined with a fuzzer like [libFuzzer
and `cargo-fuzz`](https://github.com/rust-fuzz/cargo-fuzz) or
[AFL](https://github.com/rust-fuzz/afl.rs), and to help you turn the raw,
untyped byte buffers that they produce into well-typed, valid, structured
values. This allows you to combine structure-aware test case generation with
coverage-guided, mutation-based fuzzers.

## Documentation

[**Read the API documentation on `docs.rs`!**](https://docs.rs/arbitrary)

## Example

Say you're writing a color conversion library, and you have an `Rgb` struct to
represent RGB colors. You might want to implement `Arbitrary` for `Rgb` so that
you could take arbitrary `Rgb` instances in a test function that asserts some
property (for example, asserting that RGB converted to HSL and converted back to
RGB always ends up exactly where we started).

### Automatically Deriving `Arbitrary`

Automatically deriving the `Arbitrary` trait is the recommended way to implement
`Arbitrary` for your types.

Automatically deriving `Arbitrary` requires you to enable the `"derive"` cargo
feature:

```toml
# Cargo.toml

[dependencies]
arbitrary = { version = "1", features = ["derive"] }
```

And then you can simply add `#[derive(Arbitrary)]` annotations to your types:

```rust
// rgb.rs

use arbitrary::Arbitrary;

#[derive(Arbitrary)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
```

#### Customizing single fields

This can be particular handy if your structure uses a type that does not implement `Arbitrary` or you want to have more customization for particular fields.

```rust
#[derive(Arbitrary)]
pub struct Rgba {
    // set `r` to Default::default()
    #[arbitrary(default)]
    pub r: u8,

    // set `g` to 255
    #[arbitrary(value = 255)]
    pub g: u8,

    // Generate `b` with a custom function of type
    //
    //    fn(&mut Unstructured) -> arbitrary::Result<T>
    //
    // where `T` is the field's type.
    #[arbitrary(with = arbitrary_b)]
    pub b: u8,

    // Generate `a` with a custom closure (shortcut to avoid a custom function)
    #[arbitrary(with = |u: &mut Unstructured| u.int_in_range(0..=64))]
    pub a: u8,
}

fn arbitrary_b(u: &mut Unstructured) -> arbitrary::Result<u8> {
    u.int_in_range(64..=128)
}
```

### Implementing `Arbitrary` By Hand

Alternatively, you can write an `Arbitrary` implementation by hand:

```rust
// rgb.rs

use arbitrary::{Arbitrary, Result, Unstructured};

#[derive(Copy, Clone, Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl<'a> Arbitrary<'a> for Rgb {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let r = u8::arbitrary(u)?;
        let g = u8::arbitrary(u)?;
        let b = u8::arbitrary(u)?;
        Ok(Rgb { r, g, b })
    }
}
```

## Minimum Supported Rust Version (MSRV)

<!-- NB: Keep this number in sync with the `rust-version` in `Cargo.toml`. -->

This crate is guaranteed to compile on stable Rust **1.63.0** and up. It might
compile with older versions but that may change in any new patch release.

We reserve the right to increment the MSRV on minor releases, however we will
strive to only do it deliberately and for good reasons.

## License

Licensed under dual MIT or Apache-2.0 at your choice.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
