`crc-fast`
===========

[![Test status](https://github.com/awesomized/crc-fast-rust/workflows/Tests/badge.svg)](https://github.com/awesomized/crc-fast-rust/actions?query=workflow%3ATests)
[![Latest Version](https://img.shields.io/crates/v/crc-fast.svg)](https://crates.io/crates/crc-fast)
[![Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/crc-fast)

World's fastest generic CRC calculator for
[all known CRC-16, CRC-32, and CRC-64 variants](https://reveng.sourceforge.io/crc-catalogue/all.htm), as well as bring-your-own
custom parameters, using SIMD intrinsics,
which can exceed [100GiB/s](#performance) on modern systems.

Supports acceleration on `aarch64`, `x86_64`, and `x86` architectures, plus has a safe non-accelerated table-based
software fallback for others.

The [crc crate](https://crates.io/crates/crc) is ~0.5GiB/s by default, so this is
[up to >220X faster](#tldr-just-tell-me-how-to-turn-it-up-to-11-).

This is unique, not just because of the performance, but also because I couldn't find a single generic SIMD-accelerated
implementation (in any language) which worked for _all_ known variants, using the
[Rocksoft model](http://www.ross.net/crc/download/crc_v3.txt), especially the "non-reflected" variants.

So I wrote one. :)

## Other languages

Supplies a [C/C++ compatible library](#cc-compatible-library) for use with other non-`Rust` languages.

## Implementations

* [AWS SDK for Rust](https://awslabs.github.io/aws-sdk-rust/) via
  the [aws-smithy-checksums](https://crates.io/crates/aws-smithy-checksums) crate.
* [crc-fast-php-ext](https://github.com/awesomized/crc-fast-php-ext) `PHP` extension using this library.

## Changes

See [CHANGELOG](CHANGELOG.md).

## Build & Install

### Library

`cargo build --release` will obviously build the Rust library, including
the  [C/C++ compatible dynamic and static libraries](#cc-compatible-library).

### CLI tools

There are some command-line tools available:

- `checksum` calculates CRC checksums from the supplied string or file
- `get-custom-params` generates the custom CRC parameters for the supplied Rocksoft model values
- `arch-check` checks the current architecture's hardware acceleration features (primarily for debugging)

To build them, enable the `cli` feature: `cargo build --features cli --release`.

### Everything

To build the libraries and the CLI tools, use the `--all-features` flag:  `cargo build --all-features --release`. 

A _very_ basic [Makefile](Makefile) is supplied which supports `make install` to install the libraries, header file, and
CLI binaries to the local system. Specifying the `DESTDIR` environment variable will allow you to customize the install
location.

```
DESTDIR=/my/custom/path make install
```

## Features

The library supports various feature flags for different environments:

### Default Features
* `std` - Standard library support, includes `alloc`
* `ffi` - C/C++ FFI bindings for shared library (will become optional in v2.0)
* `panic-handler` - Provides panic handler for `no_std` environments (disable when building binaries)

### Optional Features
* `alloc` - Heap allocation support (enables `Digest` trait, custom CRC params, checksum combining)
* `cache` - Caches generated constants for custom CRC parameters (requires `alloc`)
* `cli` - Enables command-line tools (`checksum`, `arch-check`, `get-custom-params`)

### Building for `no_std`

For embedded targets without standard library:

```bash
# Minimal no_std (core CRC only, no heap)
cargo build --target thumbv7em-none-eabihf --no-default-features --lib

# With heap allocation (enables Digest, custom params)
cargo build --target thumbv7em-none-eabihf --no-default-features --features alloc --lib

# With caching (requires alloc)
cargo build --target thumbv7em-none-eabihf --no-default-features --features cache --lib
```

Tested on ARM Cortex-M (`thumbv7em-none-eabihf`, `thumbv8m.main-none-eabihf`) and RISC-V (
`riscv32imac-unknown-none-elf`).

### Building for `WASM`

For WebAssembly targets:

```bash
# Minimal WASM
cargo build --target wasm32-unknown-unknown --no-default-features --lib

# With heap allocation (typical use case)
cargo build --target wasm32-unknown-unknown --no-default-features --features alloc --lib

# Using wasm-pack for browser
wasm-pack build --target web --no-default-features --features alloc
```

Tested on `wasm32-unknown-unknown`, `wasm32-wasip1`, and `wasm32-wasip2` targets.

## Usage

Add `crc-fast = "1"` to your `Cargo.toml` dependencies, which will enable every available optimization for
the `stable` toolchain.

### Fast helper functions

For the [most common and popular](#important-crc-variants) CRC variants, there are specialized one-shot functions to make adoption easier and
performance faster, particularly for smaller input sizes, since it reduces some of the overhead of the generic 
`checksum` path.

#### CRC-32/ISCSI

Also commonly known as `crc32c` in many, but not all, implementations.

```rust
use crc_fast::crc32_iscsi;

let checksum = crc32_iscsi(b"123456789");

assert_eq!(checksum, 0xe3069283);
```

#### CRC-32/ISO-HDLC

Also commonly known as `crc32` in many, but not all, implementations.

```rust
use crc_fast::crc32_iso_hdlc;

let checksum = crc32_iso_hdlc(b"123456789");

assert_eq!(checksum, 0xcbf43926);
```

#### CRC-64/NVME

```rust
use crc_fast::crc64_nvme;

let checksum = crc64_nvme(b"123456789");

assert_eq!(checksum, 0xae8b14860a799888);
``` 

### Digest

Implements the [digest::DynDigest](https://docs.rs/digest/latest/digest/trait.DynDigest.html)
trait for easier integration with existing Rust code.

Creates a `Digest` which can be updated over time, for stream processing, intermittent workloads, etc, enabling
finalizing the checksum once processing is complete.

 ```rust
 use crc_fast::{Digest, CrcAlgorithm::Crc32IsoHdlc};

let mut digest = Digest::new(Crc32IsoHdlc);
digest.update(b"1234");
digest.update(b"56789");
let checksum = digest.finalize();

assert_eq!(checksum, 0xcbf43926);
 ```

### Digest Write

Implements the [std::io::Write](https://doc.rust-lang.org/std/io/trait.Write.html) trait for
easier integration with existing Rust code.

 ```rust
use std::env;
use std::fs::File;
use crc_fast::{Digest, CrcAlgorithm::Crc32IsoHdlc};

// for example/test purposes only, use your own file path
let binding = env::current_dir().expect("missing working dir").join("crc-check.txt");
let file_on_disk = binding.to_str().unwrap();

// actual usage
let mut digest = Digest::new(Crc32IsoHdlc);
let mut file = File::open(file_on_disk).unwrap();
std::io::copy( & mut file, & mut digest).unwrap();
let checksum = digest.finalize();

assert_eq!(checksum, 0xcbf43926);
 ```

### checksum

Checksums a string.

```rust
 use crc_fast::{checksum, CrcAlgorithm::Crc32IsoHdlc};

let checksum = checksum(Crc32IsoHdlc, b"123456789");

assert_eq!(checksum, 0xcbf43926);
 ```

### checksum_combine

Combines checksums from two different sources, which can be useful for distributed or multithreaded workloads, etc.

```rust
 use crc_fast::{checksum, checksum_combine, CrcAlgorithm::Crc32IsoHdlc};

let checksum_1 = checksum(Crc32IsoHdlc, b"1234");
let checksum_2 = checksum(Crc32IsoHdlc, b"56789");
let checksum = checksum_combine(Crc32IsoHdlc, checksum_1, checksum_2, 5);

assert_eq!(checksum, 0xcbf43926);
 ```

### checksum_file

Checksums a file, which will chunk through the file optimally, limiting RAM usage and maximizing throughput. Chunk size
is optional.

```rust
 use crc_fast::{checksum_file, CrcAlgorithm::Crc32IsoHdlc};

// for example/test purposes only, use your own file path
let binding = env::current_dir().expect("missing working dir").join("crc-check.txt");
let file_on_disk = binding.to_str().unwrap();

let checksum = checksum_file(Crc32IsoHdlc, file_on_disk, None);

assert_eq!(checksum.unwrap(), 0xcbf43926);
 ```

## Custom CRC Parameters

For cases where you need to use CRC variants not included in the predefined algorithms, you can define custom CRC
parameters and use the `*_with_params` functions.

### Digest with custom parameters

Creates a `Digest` with custom CRC parameters for stream processing.

```rust
use crc_fast::{Digest, CrcParams};

// Define custom CRC-32 parameters (equivalent to CRC-32/ISO-HDLC)
let custom_params = CrcParams::new(
    "CRC-32/CUSTOM",
    32,
    0x04c11db7,
    0xffffffff,
    true,
    0xffffffff,
    0xcbf43926,
);

let mut digest = Digest::new_with_params(custom_params);
digest.update(b"123456789");
let checksum = digest.finalize();

assert_eq!(checksum, 0xcbf43926);
```

### checksum_with_params

Checksums data using custom CRC parameters.

```rust
use crc_fast::{checksum_with_params, CrcParams};

// Define custom CRC-32 parameters (equivalent to CRC-32/ISO-HDLC)
let custom_params = CrcParams::new(
    "CRC-32/CUSTOM",
    32,
    0x04c11db7,
    0xffffffff,
    true,
    0xffffffff,
    0xcbf43926,
);

let checksum = checksum_with_params(custom_params, b"123456789");

assert_eq!(checksum, 0xcbf43926);
```

### checksum_combine_with_params

Combines checksums from two different sources using custom CRC parameters.

```rust
use crc_fast::{checksum_with_params, checksum_combine_with_params, CrcParams};

// Define custom CRC-32 parameters (equivalent to CRC-32/ISO-HDLC)
let custom_params = CrcParams::new(
    "CRC-32/CUSTOM",
    32,
    0x04c11db7,
    0xffffffff,
    true,
    0xffffffff,
    0xcbf43926,
);

let checksum_1 = checksum_with_params(custom_params, b"1234");
let checksum_2 = checksum_with_params(custom_params, b"56789");
let checksum = checksum_combine_with_params(custom_params, checksum_1, checksum_2, 5);

assert_eq!(checksum, 0xcbf43926);
```

### checksum_file_with_params

Checksums a file using custom CRC parameters, chunking through the file optimally.

```rust
use std::env;
use crc_fast::{checksum_file_with_params, CrcParams};

// for example/test purposes only, use your own file path
let binding = env::current_dir().expect("missing working dir").join("crc-check.txt");
let file_on_disk = binding.to_str().unwrap();

// Define custom CRC-32 parameters (equivalent to CRC-32/ISO-HDLC)
let custom_params = CrcParams::new(
    "CRC-32/CUSTOM",
    32,
    0x04c11db7,
    0xffffffff,
    true,
    0xffffffff,
    0xcbf43926,
);

let checksum = checksum_file_with_params(custom_params, file_on_disk, None);

assert_eq!(checksum.unwrap(), 0xcbf43926);
```

## C/C++ compatible library

`cargo build` will produce a shared library target (`.so` on Linux, `.dll` on Windows, `.dylib` on macOS, etc) and an
auto-generated [libcrc_fast.h](libcrc_fast.h) header file for use in non-Rust projects, such as through
[FFI](https://en.wikipedia.org/wiki/Foreign_function_interface). It will also produce a static library target (`.a` on
Linux and macOS, `.lib` on Windows, etc) for projects
which prefer statically linking.

There is a [crc-fast PHP extension](https://github.com/awesomized/crc-fast-php-ext) using it, for example.

## Background

This implementation is based on Intel's
[Fast CRC Computation for Generic Polynomials Using PCLMULQDQ Instruction](https://web.archive.org/web/20131224125630/https://www.intel.com/content/dam/www/public/us/en/documents/white-papers/fast-crc-computation-generic-polynomials-pclmulqdq-paper.pdf)
white paper, though it folds 8-at-a-time, like other modern implementations, rather than the 4-at-a-time as in Intel's
paper.

This library works on `aarch64`, `x86_64`, and `x86` architectures, and is hardware-accelerated and optimized for each
architecture.

Inspired by [`crc32fast`](https://crates.io/crates/crc32fast),
[`crc64fast`](https://crates.io/crates/crc64fast),
and [`crc64fast-nvme`](https://crates.io/crates/crc64fast-nvme), each of which only accelerates a single, different CRC
variant, and all of them were "reflected" variants.

In contrast, this library accelerates _every known variant_ (and should accelerate any future variants without changes),
including all the "non-reflected" variants.

## Important CRC variants

While there are [many variants](https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-iso-hdlc), three
stand out as being the most important and widely used (all of which are "reflected"):

### [CRC-32/ISCSI](https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-iscsi)

Many, but not all, implementations simply call this `crc32c` and it's probably the 2nd most popular and widely used,
after `CRC-32/ISO-HDLC`. It's used in `iSCSI`, `ext4`, `btrfs`, etc.

Both `x86_64` and `aarch64` have native hardware support for this CRC variant, so we can use
[fusion](https://www.corsix.org/content/fast-crc32c-4k) in many cases to accelerate it further by fusing SIMD CLMUL
instructions with the native CRC instructions.

### [CRC-32/ISO-HDLC](https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-32-iso-hdlc)

Many, but not all, implementations simply call this `crc32` and it may be the most popular and widely used. It's used in
`Ethernet`, `PKZIP`, `xz`, etc.

Only `aarch64` has native hardware support for this CRC variant, so we can use
[fusion](https://www.corsix.org/content/fast-crc32c-4k) on that platform, but not `x86_64`.

### [CRC-64/NVME](https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-64-nvme)

`CRC-64/NVME` comes from
the [NVM Express® NVM Command Set Specification](https://nvmexpress.org/wp-content/uploads/NVM-Express-NVM-Command-Set-Specification-1.0d-2023.12.28-Ratified.pdf)
(Revision 1.0d, December 2023),
is [AWS S3's recommended checksum option](https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html)
(as `CRC64-NVME`), and has also been implemented in the
[Linux kernel](https://github.com/torvalds/linux/blob/786c8248dbd33a5a7a07f7c6e55a7bfc68d2ca48/lib/crc64.c#L66-L73)
(where it's been called `CRC-64/Rocksoft` in the past).

Note that the `Check` value in the `NVMe` spec uses incorrect endianness (see `Section 5.2.1.3.4, Figure 120, page 83`)
but all known public & private implementations agree on the correct value, which this library produces.

# Acceleration targets

This library has baseline support for accelerating all known `CRC-16`, `CRC-32`, and `CRC-64` variants on `aarch64`,
`x86_64`, and
`x86` internally in pure `Rust`.

It uses the best available acceleration method for the detected CPU features at runtime, including:
* `aarch64`:
  * `neon-pmull-sha3` (preferred, if available)
  * `neon-pmull`
* `x86_64` and `x86`:
    * `avx512-vpclmulqdq` (preferred, if available)
    * `avx512-pclmulqdq`
    * `sse-pclmulqdq`
  
There is a safe table-based software fallback for other architectures, or if no acceleration features are detected.

### Checking your platform capabilities

There's an [arch-check](src/bin/arch-check.rs) binary which will explain the selected target architecture.

```
// test it works on your system (patches welcome!)
cargo test

// examine the chosen acceleration targets
cargo run arch-check

// build for release
cargo build --release
```

## Performance

Modern systems can exceed 100 GiB/s for calculating `CRC-32/ISCSI`, and nearly 90 GiB/s for `CRC-32/ISO-HDLC`,
`CRC-64/NVME`, and all other reflected variants. (Forward variants are slower, due to the extra shuffle-masking, but
are still extremely fast in this library).

This is a summary of the performance for the most important and popular CRC checksums.

### CRC-32/ISCSI (reflected)

AKA `crc32c` in many, but not all, implementations.

| Arch    | Brand | CPU             | System                    | Target            | 1KiB (GiB/s) | 1MiB (GiB/s) |
|:--------|:------|:----------------|:--------------------------|:------------------|-------------:|-------------:|
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-24xl        | avx512-vpclmulqdq |          ~61 |         ~111 |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | avx512-vpclmulqdq |          ~26 |          ~54 |
| aarch64 | AWS   | Graviton4       | EC2 c8g.metal-48xl        | neon-pmull-sha3   |          ~23 |          ~54 |
| aarch64 | AWS   | Graviton2       | EC2 c6g.metal             | neon-pmull        |          ~11 |          ~17 |
| aarch64 | Apple | M3 Ultra        | Mac Studio (32 core)      | neon-pmull-sha3   |          ~60 |          ~99 |
| aarch64 | Apple | M4 Max          | MacBook Pro 16" (16 core) | neon-pmull-sha3   |          ~56 |          ~94 |

### CRC-32/ISO-HDLC (reflected)

AKA `crc32` in many, but not all, implementations.

| Arch    | Brand | CPU             | System                    | Target            | 1KiB (GiB/s) | 1MiB (GiB/s) |
|:--------|:------|:----------------|:--------------------------|:------------------|-------------:|-------------:|
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-248xl       | avx512-vpclmulqdq |          ~28 |          ~88 |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | avx512-vpclmulqdq |          ~21 |          ~55 |
| aarch64 | AWS   | Graviton4       | EC2 c8g.metal-48xl        | neon-pmull-sha3   |          ~23 |          ~54 |
| aarch64 | AWS   | Graviton2       | EC2 c6g.metal             | neon-pmull        |          ~11 |          ~17 |
| aarch64 | Apple | M3 Ultra        | Mac Studio (32 core)      | neon-pmull-sha3   |          ~48 |          ~98 |
| aarch64 | Apple | M4 Max          | MacBook Pro 16" (16 core) | neon-pmull-sha3   |          ~56 |          ~94 |

### CRC-64/NVME (reflected)

[AWS S3's recommended checksum option](https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html)

| Arch    | Brand | CPU             | System                    | Target            | 1KiB (GiB/s) | 1MiB (GiB/s) |
|:--------|:------|:----------------|:--------------------------|:------------------|-------------:|-------------:|
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-24xl        | avx512-vpclmulqdq |          ~28 |          ~88 |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | avx512-vpclmulqdq |          ~22 |          ~55 |
| aarch64 | AWS   | Graviton4       | EC2 c8g.metal-48xl        | neon-pmull-sha3   |          ~28 |          ~41 |
| aarch64 | AWS   | Graviton2       | EC2 c6g.metal             | neon-pmull        |          ~11 |          ~16 |
| aarch64 | Apple | M3 Ultra        | Mac Studio (32 core)      | neon-pmull-sha3   |          ~58 |          ~72 |
| aarch64 | Apple | M4 Max          | MacBook Pro 16" (16 core) | neon-pmull-sha3   |          ~52 |          ~72 |

### CRC-32/BZIP2 (forward)

| Arch    | Brand | CPU             | System                    | Target            | 1KiB (GiB/s) | 1MiB (GiB/s) |
|:--------|:------|:----------------|:--------------------------|:------------------|-------------:|-------------:|
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-24xl        | avx512-vpclmulqdq |          ~20 |          ~56 |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | avx512-vpclmulqdq |          ~14 |          ~43 |
| aarch64 | AWS   | Graviton4       | EC2 c8g.metal-48xl        | neon-pmull-eor3   |          ~18 |          ~40 |
| aarch64 | AWS   | Graviton2       | EC2 c6g.metal             | neon-pmull        |           ~9 |          ~14 |
| aarch64 | Apple | M3 Ultra        | Mac Studio (32 core)      | neon-pmull-eor3   |          ~41 |          ~59 |
| aarch64 | Apple | M4 Max          | MacBook Pro 16" (16 core) | neon-pmull-eor3   |          ~47 |          ~64 |

### CRC-64/ECMA-182 (forward)

| Arch    | Brand | CPU             | System                    | Target            | 1KiB (GiB/s) | 1MiB (GiB/s) |
|:--------|:------|:----------------|:--------------------------|:------------------|-------------:|-------------:|
| x86_64  | Intel | Sapphire Rapids | EC2 c7i.metal-24xl        | avx512-vpclmulqdq |          ~21 |          ~56 |
| x86_64  | AMD   | Genoa           | EC2 c7a.metal-48xl        | avx512-vpclmulqdq |          ~14 |          ~43 |
| aarch64 | AWS   | Graviton4       | EC2 c8g.metal-48xl        | neon-pmull-eor3   |          ~19 |          ~40 |
| aarch64 | AWS   | Graviton2       | EC2 c6g.metal             | neon-pmull        |           ~9 |          ~14 |
| aarch64 | Apple | M3 Ultra        | Mac Studio (32 core)      | neon-pmull-eor3   |          ~40 |          ~59 |
| aarch64 | Apple | M4 Max          | MacBook Pro 16" (16 core) | neon-pmull-eor3   |          ~46 |          ~61 |

## Other CRC widths

There are [a lot of other known CRC widths and variants](https://reveng.sourceforge.io/crc-catalogue/all.htm), ranging
from `CRC-3/GSM` to `CRC-82/DARC`, and everything in between.

Since [Awesome](https://awesome.co) only uses  `CRC-32` or `CRC-64` widths in our products, this library began by supporting only those
widths, including all known variants plus support for custom [Rocksoft](http://www.ross.net/crc/download/crc_v3.txt)
parameters.

`CRC-16` has since been added, including all known variants plus support for custom parameters as well.

In theory, much of the "heavy lifting" has been done, so it should be possible to add other widths with minimal effort.

PRs welcome!

## Memory Safety

Given the heavy use of hardware intrinsics, this crate uses a decent amount of `unsafe` code.

To help ensure memory safety and stability, this crate is validated using [Miri](https://github.com/rust-lang/miri) on
`x86_64` as well as fuzz tested using [libFuzzer](https://github.com/rust-fuzz/libfuzzer) over millions of iterations.

## References

* [Catalogue of parametrised CRC algorithms](https://reveng.sourceforge.io/crc-catalogue/all.htm)
* [crc32-fast](https://crates.io/crates/crc32fast) Original `CRC-32/ISO-HDLC` (`crc32`) implementation in `Rust`.
* [crc64-fast](https://github.com/tikv/crc64fast) Original `CRC-64/XZ` implementation in `Rust`.
* [crc64fast-nvme](https://github.com/awesomized/crc64fast-nvme) Original `CRC-64/NVME` implementation in `Rust`.
* [Fast CRC Computation for Generic Polynomials Using PCLMULQDQ Instruction](https://web.archive.org/web/20131224125630/https://www.intel.com/content/dam/www/public/us/en/documents/white-papers/fast-crc-computation-generic-polynomials-pclmulqdq-paper.pdf)
  Intel's paper.
* [NVM Express® NVM Command Set Specification](https://nvmexpress.org/wp-content/uploads/NVM-Express-NVM-Command-Set-Specification-1.0d-2023.12.28-Ratified.pdf)
  The NVMe spec, including `CRC-64-NVME` (with incorrect endian `Check` value in
  `Section 5.2.1.3.4, Figure 120, page 83`).
* [CRC-64/NVME](https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-64-nvme) The `CRC-64/NVME` quick
  definition.
* [A PAINLESS GUIDE TO CRC ERROR DETECTION ALGORITHMS](http://www.ross.net/crc/download/crc_v3.txt) Best description of
  CRC I've seen to date (and the definition of the Rocksoft model).
* [Linux implementation](https://github.com/torvalds/linux/blob/786c8248dbd33a5a7a07f7c6e55a7bfc68d2ca48/lib/crc64.c)
  Linux implementation of `CRC-64/NVME`.
* [MASM/C++ artifacts implementation](https://github.com/jeffareid/crc/) - Reference MASM/C++ implementation for
  generating artifacts.
* [Intel isa-l GH issue #88](https://github.com/intel/isa-l/issues/88) - Additional insight into generating artifacts.
* [StackOverflow PCLMULQDQ CRC32 answer](https://stackoverflow.com/questions/71328336/fast-crc-with-pclmulqdq-not-reflected/71329114#71329114)
  Insightful answer to implementation details for CRC32.
* [StackOverflow PCLMULQDQ CRC32 question](https://stackoverflow.com/questions/21171733/calculating-constants-for-crc32-using-pclmulqdq)
  Insightful question & answer to CRC32 implementation details.
* [AWS S3 announcement about CRC64-NVME support](https://aws.amazon.com/blogs/aws/introducing-default-data-integrity-protections-for-new-objects-in-amazon-s3/)
* [AWS S3 docs on checking object integrity using CRC64-NVME](https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html)
* [Vector Carry-Less Multiplication of Quadwords (VPCLMULQDQ) details](https://en.wikichip.org/wiki/x86/vpclmulqdq)
* [Linux kernel updates by Eric Biggers to use VPCLMULQDQ, etc](https://lkml.org/lkml/2025/2/10/1367)
* [Faster CRC32-C on x86](https://www.corsix.org/content/fast-crc32c-4k)
* [Faster CRC32 on the Apple M1](https://dougallj.wordpress.com/2022/05/22/faster-crc32-on-the-apple-m1/)
* [An alternative exposition of crc32_4k_pclmulqdq](https://www.corsix.org/content/alternative-exposition-crc32_4k_pclmulqdq)
* [fast-crc32](https://github.com/corsix/fast-crc32) - implementations of fusion for two CRC-32 variants.

## License

`cfc-fast` is dual-licensed under

* Apache 2.0 license ([LICENSE-Apache](./LICENSE-Apache) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](./LICENSE-MIT) or <https://opensource.org/licenses/MIT>)