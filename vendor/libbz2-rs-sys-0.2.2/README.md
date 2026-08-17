![checks](https://github.com/trifectatechfoundation/libbzip2-rs/actions/workflows/checks.yaml/badge.svg?branch=main)
[![codecov](https://codecov.io/gh/trifectatechfoundation/libbzip2-rs/graph/badge.svg?token=Lqtmehzxm0)](https://codecov.io/gh/trifectatechfoundation/libbzip2-rs)
[![Crates.io](https://img.shields.io/crates/v/libbz2-rs-sys.svg)](https://crates.io/crates/libbz2-rs-sys)


# libbzip2-rs: a safer libbzip

This repository contains a Rust implementation of the bzip2 file format that is compatible with the libbzip2 API.

This repository contains the following public crate:

* [libbz2-rs-sys](https://crates.io/crates/libbz2-rs-sys/), a libbzip2-compatible C API.

## How to use libbzip2-rs in your project

libbzip2-rs can be used in both Rust and C projects.

### Rust projects

By far the easiest way to use libbzip2-rs is through the [bzip2](https://crates.io/crates/bzip2) crate, by simply enabling the `libbz2-rs-sys` feature gate. This will enable the `libbz2-rs-sys` backend.

You can also directly use the C api exported by the `libbz2-rs-sys` crate.

## C projects

libbzip2-rs can be built as a shared object file for usage by C programs that dynamically link to libbzip2. Please see the example in [libbz2-rs-sys-cdylib](https://github.com/trifectatechfoundation/libbzip2-rs/tree/main/libbz2-rs-sys-cdylib).

## Acknowledgment

This project is based on a [c2rust](https://github.com/immunant/c2rust) translation of the original [libbzip2](https://sourceware.org/bzip2/).

## About

libbzip2-rs is part of Trifecta Tech Foundation's [Data compression initiative](https://trifectatech.org/initiatives/data-compression/).

## Funding

This project is funded through [NGI Zero Core](https://nlnet.nl/core), a fund established by [NLnet](https://nlnet.nl) with financial support from the European Commission's [Next Generation Internet](https://ngi.eu) program. Learn more at the [NLnet project page](https://nlnet.nl/project/ZipLinting).

[<img src="https://nlnet.nl/logo/banner.png" alt="NLnet foundation logo" width="20%" />](https://nlnet.nl)  
[<img src="https://nlnet.nl/image/logos/NGI0_tag.svg" alt="NGI Zero Logo" width="20%" />](https://nlnet.nl/core)
