# atoi-rs

Parse integers directly from `[u8]` slices in safe code

## Reasons to use this crate

Starting from a binary or ascii format you can parse an integer around three times as fast as with
the more idiomatic detour over utf8. The crate comes with benchmarks so you can see for yourself.

The `FromRadix10Checked` trait also provides a way to parse integers very fast and safe, as its
implementation only performs checked arithmetics for the one digit that may actually overflow.

## Example

Parsing from a slice

```rust
use atoi::atoi;
assert_eq!(Some(42), atoi::<u32>(b"42"));
```

Note that if you want to know how much of the input has been used, you can use the
`FromRadix10` trait, for example:

```rust
use atoi::FromRadix10;

/// Return the parsed integer and remaining slice if successful.
fn atoi_with_rest<I: FromRadix10>(text: &[u8]) -> Option<(&[u8], I)> {
    match I::from_radix_10(text) {
        (_, 0) => None,
        (n, used) => Some((&[used..], n)),
    }
}
```

This [crate](https://www.crates.io/crates/atoi) has more to offer! Check out the full documentation
at [docs.rs](https://docs.rs/atoi).
