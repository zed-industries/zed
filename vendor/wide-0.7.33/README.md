[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.61-green.svg)
[![crates.io](https://img.shields.io/crates/v/wide.svg)](https://crates.io/crates/wide)
[![docs.rs](https://docs.rs/wide/badge.svg)](https://docs.rs/wide/)

# wide

A crate to help you go wide.

Specifically, this has portable "wide" data types that do their best to be SIMD when possible.

On `x86`, `x86_64`, `wasm32` and `aarch64 neon` this is done with explicit
intrinsic usage (via [safe_arch](https://docs.rs/safe_arch)), and on other
architectures this is done by carefully writing functions so that LLVM hopefully
does the right thing. When Rust stabilizes more explicit intrinsics then they
can go into `safe_arch` and then they can get used here.

