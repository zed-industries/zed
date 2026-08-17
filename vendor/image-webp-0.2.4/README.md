# image-webp

[![crates.io](https://img.shields.io/crates/v/image-webp.svg)](https://crates.io/crates/image-webp)
[![Documentation](https://docs.rs/image-webp/badge.svg)](https://docs.rs/image-webp)
[![Build Status](https://github.com/image-rs/image-webp/workflows/Rust%20CI/badge.svg)](https://github.com/image-rs/image-webp/actions)

This crate is an independent implementation of the WebP image format, written so
that the `image` crate can have a pure-Rust WebP backend for both encoding and
decoding.

## Current Status

* **Decoder:** Supports all WebP format features including both lossless and
  lossy compression, alpha channel, and animation. Both the "simple" and
  "extended" formats are handled, and it exposes methods to extract ICC, EXIF,
  and XMP chunks. Decoding speed is generally in the range of **70-100%** of the
  speed of libwebp.

* **Encoder:** This crate only supports lossless encoding. The encoder
  implementation is relatively basic which makes it very fast, but it doesn't
  get as good compression ratios as libwebp can. Nonetheless, it often produces
  smaller files than PNG, even when compared against the slowest/highest
  compression options of PNG encoders.

## Future possibilities

* We continue to be interested in **optimizations** and **bug fixes** and hope
  the bring the decoder closer to parity with libwebp.

* Another potential area is **animation encoding**. Much of the groundwork is in
  place for this, but it will require some additional work to implement.

* We would like to add **lossy encoding** support, but this is a non-trivial
  task and would require a lot of work. If you are interested in helping with
  this, please get in touch!

## Unsafe code

Both this crate and all of its dependencies currently contain no unsafe code.

NOTE: This isn't a guarantee that unsafe code will never be added. It may prove
necessary in the future to improve performance, but we will always strive to
minimize the use of unsafe code and ensure that it is well-tested and
documented.

```
$ cargo geiger

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    🔒  = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ❓  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    ☢️   = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Dependency

0/0        0/0          0/0    0/0     0/0      🔒 image-webp 0.2.3
0/0        0/0          0/0    0/0     0/0      🔒 ├── byteorder-lite 0.1.0
0/0        0/0          0/0    0/0     0/0      ❓ └── quick-error 2.0.1

0/0        0/0          0/0    0/0     0/0
```
