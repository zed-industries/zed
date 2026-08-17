# fs2

Extended utilities for working with files and filesystems in Rust. `fs2`
requires Rust stable 1.8 or greater.

[![Build Status](https://travis-ci.org/danburkert/fs2-rs.svg?branch=master)](https://travis-ci.org/danburkert/fs2-rs)
[![Windows Build status](https://ci.appveyor.com/api/projects/status/iuvjv1aaaml0rntt/branch/master?svg=true)](https://ci.appveyor.com/project/danburkert/fs2-rs/branch/master)
[![Documentation](https://docs.rs/fs2/badge.svg)](https://docs.rs/memmap)
[![Crate](https://img.shields.io/crates/v/fs2.svg)](https://crates.io/crates/memmap)

## Features

- [x] file descriptor duplication.
- [x] file locks.
- [x] file (pre)allocation.
- [x] file allocation information.
- [x] filesystem space usage information.

## Platforms

`fs2` should work on any platform supported by
[`libc`](https://github.com/rust-lang-nursery/libc#platforms-and-documentation).

`fs2` is continuously tested on:
  * `x86_64-unknown-linux-gnu` (Linux)
  * `i686-unknown-linux-gnu`
  * `x86_64-apple-darwin` (OSX)
  * `i686-apple-darwin`
  * `x86_64-pc-windows-msvc` (Windows)
  * `i686-pc-windows-msvc`
  * `x86_64-pc-windows-gnu`
  * `i686-pc-windows-gnu`

## Benchmarks

Simple benchmarks are provided for the methods provided. Many of these
benchmarks use files in a temporary directory. On many modern Linux distros the
default temporary directory, `/tmp`, is mounted on a tempfs filesystem, which
will have different performance characteristics than a disk-backed filesystem.
The temporary directory is configurable at runtime through the environment (see
[`env::temp_dir`](https://doc.rust-lang.org/stable/std/env/fn.temp_dir.html)).

## License

`fs2` is primarily distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT) for details.

Copyright (c) 2015 Dan Burkert.
