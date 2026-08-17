# strict-num
![Build Status](https://github.com/RazrFalcon/strict-num/workflows/Build/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/strict-num.svg)](https://crates.io/crates/strict-num)
[![Documentation](https://docs.rs/strict-num/badge.svg)](https://docs.rs/strict-num)
[![Rust 1.35+](https://img.shields.io/badge/rust-1.35+-orange.svg)](https://www.rust-lang.org)

A collection of bounded numeric types.

Includes:

- `FiniteF32`
- `FiniteF64`
- `NonZeroPositiveF32`
- `NonZeroPositiveF64`
- `PositiveF32`
- `PositiveF64`
- `NormalizedF32`
- `NormalizedF64`

Unlike `f32`/`f64`, all float types implement `Ord`, `PartialOrd` and `Hash`,
since it's guaranteed that they all are finite.

## License

MIT
