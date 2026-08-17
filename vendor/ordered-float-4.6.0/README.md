# ordered-float

Provides several wrapper types for `Ord` and `Eq` implementations on f64 and friends.

* [API documentation](https://docs.rs/ordered-float)
* [Release notes](https://github.com/reem/rust-ordered-float/releases)

## no_std

To use `ordered_float` without requiring the Rust standard library, disable
the default `std` feature:

```toml
[dependencies]
ordered-float = { version = "4.0", default-features = false }
```

If you still want to have access to the `Pow` and `Float` traits, enable the `libm` feature.

## Optional features

The following optional features can be enabled in `Cargo.toml`:

* `libm`: When not using the `std` feature, enable the `libm` feature of `num-traits` to still access the `Pow` and `Float` traits.
* `arbitrary`: Implements the `arbitrary::Arbitrary` trait.
* `borsh`: Adds implementations for traits provided by the `borsh` crate.
* `bytemuck`: Adds implementations for traits provided by the `bytemuck` crate.
* `derive-visitor`: Adds implementations for traits provided by the `derive-visitor` crate.
* `num-cmp`: Implements the `num_cmp::NumCmp` trait.
* `proptest`: Implements the `proptest::Arbitrary` trait.
* `rand`: Adds implementations for various distribution types provided by the `rand` crate.
* `rkyv_16`: Implements `rkyv`'s `Archive`, `Serialize` and `Deserialize` traits with `size_16`.
* `rkyv_32`: Implements `rkyv`'s `Archive`, `Serialize` and `Deserialize` traits with `size_32`.
* `rkyv_64`: Implements `rkyv`'s `Archive`, `Serialize` and `Deserialize` traits with `size_64`.
* `rkyv_ck`: Implements the `bytecheck::CheckBytes` trait.
* `schemars`: Implements the `schemars::JsonSchema` trait.
* `serde`: Implements the `serde::Serialize` and `serde::Deserialize` traits.
* `speedy`: Implements `speedy`'s `Readable` and `Writable` traits.

## License

MIT
