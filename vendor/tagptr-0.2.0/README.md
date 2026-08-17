# tagptr

Strongly typed marked pointers for storing bit patterns (_tags_) alongside raw pointers
for concurrent programming with atomic operations.

[![Build Status](https://github.com/oliver-giersch/tagptr/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/oliver-giersch/tagptr/actions/workflows/rust.yml)
[![Latest version](https://img.shields.io/crates/v/tagptr.svg)](https://crates.io/crates/tagptr)
[![Documentation](https://docs.rs/tagptr/badge.svg)](https://docs.rs/tagptr)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/oliver-giersch/tagptr)

## Usage

Add the following to your `Cargo.toml`

```
[dependencies]
tagptr = "0.2.0"
```

## Motivation

Most atomic CPU instructions only work with register-sized memory words (e.g., 32-bit or 64-bit).
Many low-level concurrent algorithms thus need to store aditional data (_tags_) in the unused lower
bits of pointers to referenced data objects.
This crate provides thin and efficient abstractions for working with such pointers.

## License

`tagptr` is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
