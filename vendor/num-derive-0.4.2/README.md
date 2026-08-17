# num-derive

[![crate](https://img.shields.io/crates/v/num-derive.svg)](https://crates.io/crates/num-derive)
[![documentation](https://docs.rs/num-derive/badge.svg)](https://docs.rs/num-derive)
[![minimum rustc 1.56](https://img.shields.io/badge/rustc-1.56+-red.svg)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)
[![build status](https://github.com/rust-num/num-derive/workflows/master/badge.svg)](https://github.com/rust-num/num-derive/actions)

Procedural macros to derive numeric traits in Rust.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
num-traits = "0.2"
num-derive = "0.4"
```

and this to your crate root:

```rust
#[macro_use]
extern crate num_derive;
```

Then you can derive traits on your own types:

```rust
#[derive(FromPrimitive, ToPrimitive)]
enum Color {
    Red,
    Blue,
    Green,
}
```

## Optional features

- **`full-syntax`** — Enables `num-derive` to handle enum discriminants
  represented by complex expressions. Usually can be avoided by
  [utilizing constants], so only use this feature if namespace pollution is
  undesired and [compile time doubling] is acceptable.

[utilizing constants]: https://github.com/rust-num/num-derive/pull/3#issuecomment-359044704
[compile time doubling]: https://github.com/rust-num/num-derive/pull/3#issuecomment-359172588

## Releases

Release notes are available in [RELEASES.md](RELEASES.md).

## Compatibility

The `num-derive` crate is tested for rustc 1.56 and greater.

## License

Licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
