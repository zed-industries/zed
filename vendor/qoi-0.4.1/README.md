# [qoi](https://crates.io/crates/qoi)

[![Build](https://github.com/aldanor/qoi-rust/workflows/CI/badge.svg)](https://github.com/aldanor/qoi-rust/actions?query=branch%3Amaster)
[![Latest Version](https://img.shields.io/crates/v/qoi.svg)](https://crates.io/crates/qoi)
[![Documentation](https://img.shields.io/docsrs/qoi)](https://docs.rs/qoi)
[![Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance)

Fast encoder/decoder for [QOI image format](https://qoiformat.org/), implemented in pure and safe Rust.

- One of the [fastest](#benchmarks) QOI encoders/decoders out there.
- Compliant with the [latest](https://qoiformat.org/qoi-specification.pdf) QOI format specification.
- Zero unsafe code.
- Supports decoding from / encoding to `std::io` streams directly.
- `no_std` support.
- Roundtrip-tested vs the reference C implementation; fuzz-tested.

### Examples

```rust
use qoi::{encode_to_vec, decode_to_vec};

let encoded = encode_to_vec(&pixels, width, height)?;
let (header, decoded) = decode_to_vec(&encoded)?;

assert_eq!(header.width, width);
assert_eq!(header.height, height);
assert_eq!(decoded, pixels);
```

### Benchmarks

```
             decode:Mp/s  encode:Mp/s  decode:MB/s  encode:MB/s
qoi.h              282.9        225.3        978.3        778.9
qoi-rust           427.4        290.0       1477.7       1002.9
```

- Reference C implementation:
  [phoboslab/qoi@00e34217](https://github.com/phoboslab/qoi/commit/00e34217).
- Benchmark timings were collected on an Apple M1 laptop.
- 2846 images from the suite provided upstream
  ([tarball](https://phoboslab.org/files/qoibench/qoi_benchmark_suite.tar)):
  all pngs except two with broken checksums.
- 1.32 GPixels in total with 4.46 GB of raw pixel data.

Benchmarks have also been run for all of the other Rust implementations
of QOI for comparison purposes and, at the time of writing this document,
this library proved to be the fastest one by a noticeable margin.

### Rust version

The minimum required Rust version for the latest crate version is 1.61.0.

### `no_std`

This crate supports `no_std` mode. By default, std is enabled via the `std`
feature. You can deactivate the `default-features` to target core instead.
In that case anything related to `std::io`, `std::error::Error` and heap
allocations is disabled. There is an additional `alloc` feature that can
be activated to bring back the support for heap allocations.

### License

This project is dual-licensed under MIT and Apache 2.0.
