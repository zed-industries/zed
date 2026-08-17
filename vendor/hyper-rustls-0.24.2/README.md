# hyper-rustls
This is an integration between the [rustls TLS stack](https://github.com/rustls/rustls)
and the [hyper HTTP library](https://github.com/hyperium/hyper).

[![Build Status](https://github.com/rustls/hyper-rustls/workflows/hyper-rustls/badge.svg)](https://github.com/rustls/hyper-rustls/actions)
[![Crate](https://img.shields.io/crates/v/hyper-rustls.svg)](https://crates.io/crates/hyper-rustls)
[![Documentation](https://docs.rs/hyper-rustls/badge.svg)](https://docs.rs/hyper-rustls/)

# Release history

Release history can be found [on GitHub](https://github.com/rustls/hyper-rustls/releases).

# License
hyper-rustls is distributed under the following three licenses:

- Apache License version 2.0.
- MIT license.
- ISC license.

These are included as LICENSE-APACHE, LICENSE-MIT and LICENSE-ISC
respectively.  You may use this software under the terms of any
of these licenses, at your option.

## Running examples

### server

```bash
cargo run --example server
```

### client

```bash
cargo run --example client "https://docs.rs/hyper-rustls/latest/hyper_rustls/"
```
