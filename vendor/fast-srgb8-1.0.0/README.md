# `fast-srgb8`
[![Build Status](https://github.com/thomcc/fast-srgb8/workflows/CI/badge.svg)](https://github.com/thomcc/fast-srgb8/actions)
[![Docs](https://docs.rs/fast-srgb8/badge.svg)](https://docs.rs/fast-srgb8)
[![Latest Version](https://img.shields.io/crates/v/fast-srgb8.svg)](https://crates.io/crates/fast-srgb8)
![Minimum Rust Version](https://img.shields.io/badge/MSRV%201.46-blue.svg)

Small crate implementing fast conversion between linear float and 8-bit sRGB. Includes API for performing 4 simultaneous conversions, which are SIMD accelerated using SSE2 if available. Supports no_std (doesn't need `libm` either).

## Features
- `f32_to_srgb8`: converting a linear `f32` to sRGB `u8`. Compliant with [the most relevent public spec](https://microsoft.github.io/DirectX-Specs/d3d/archive/D3D11_3_FunctionalSpec.htm#FLOATtoSRGB) for this conversion (correct to ULP of 0.6, monotonic over range, etc)
- `f32x4_to_srgb8`: Produces results identical to calling `f32_to_srgb8` 4 times in a row, but uses SSE2 to SIMD accelerate on `x86` and `x86_64` where SSE2 is known to be present. Otherwise, it just returns the results of calling `f32_to_srgb8` (the scalar equivalent) 4 times.
- `srgb8_to_f32`: Inverse operation of `f32_to_srgb8`. Uses the standard technique of a 256-item lookup table.

## Benefits
- Huge performance improvments over the naive implementation — ~5x for conversion to f32->srgb8, ~20x for srgb8->f32.
- Supports `no_std` — normally this is tricky, as these operations require `powf` naively, which is not available to libcore.
- No dependencies.
- SIMD support for conversion to sRGB (conversion from sRGB is already ~20x faster than naive impl, and would probably be slower in SIMD, so for now it's not implemented).
- Consistent and correct (according to at least one relevant spec) handling of edge cases, such as NaN/Inf/etc.
- Exhaustive checking of all inputs for correctness (in tests).

### Benchmarks
```
# Measures `fast_srgb8::f32_to_srgb8` vs ref impl
test tests::bench::fast_scalar       ... bench:         144 ns/iter (+/- 11)
test tests::bench::naive_scalar      ... bench:         971 ns/iter (+/- 48)

# Measures `fast_srgb8::f32x4_to_srgb8` vs calling reference impl 4 times
test tests::bench::fast_f32x4        ... bench:         440 ns/iter (+/- 29)
test tests::bench::naive_f32x4       ... bench:       3,625 ns/iter (+/- 282)
test tests::bench::fast_f32x4_nosimd ... bench:         482 ns/iter (+/- 27)

# Measures `fast_srgb8::srgb8_to_f32` vs ref impl
test tests::bench::fast_from_srgb8   ... bench:          81 ns/iter (+/- 6)
test tests::bench::naive_from_srgb8  ... bench:       4,026 ns/iter (+/- 282)
```
(Note that the `ns/iter` time is not for a single invocation of these function, it's for several)

## License
Public domain, as explained [here](https://creativecommons.org/publicdomain/zero/1.0/legalcode). If that's unacceptable, it's also available under either the Apache-2.0 or MIT licenses, at your option.

The float->srgb code is originally¹ based on public domain routines by [Fabien "ryg" Giesen](https://fgiesen.wordpress.com), although I'm no longer sure where these are available.

¹ (Well, specifically: The Rust code in this crate is ported from code in a C++ game engine of mine, which in turn, was based on the code from ryg. This doesn't make a difference, but increases the likelihood that any errors are solely my responsibility).
