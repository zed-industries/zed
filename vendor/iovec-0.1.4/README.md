# IoVec

A specialized byte slice type for performing vectored I/O operations.

[![Crates.io](https://img.shields.io/crates/v/iovec.svg?maxAge=2592000)](https://crates.io/crates/iovec)
[![Build Status](https://travis-ci.org/carllerche/iovec.svg?branch=master)](https://travis-ci.org/carllerche/iovec)

[Documentation](https://docs.rs/iovec)

## Usage

To use `iovec`, first add this to your `Cargo.toml`:

```toml
[dependencies]
iovec = "0.1"
```

Next, add this to your crate:

```rust
extern crate iovec;

use iovec::IoVec;
```

For more detail, see [documentation](https://docs.rs/iovec).

# License

`iovec` is primarily distributed under the terms of both the MIT license and the
Apache License (Version 2.0), with portions covered by various BSD-like
licenses.

See LICENSE-APACHE, and LICENSE-MIT for details.
