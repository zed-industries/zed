# openssl-probe

A library for helping to find system-wide trust anchor ("root") certificate
locations based on paths typically used by `openssl`.

For most users, [`rustls-platform-verifier`][] or [`rustls-native-certs`][]
are more convenient and robust ways to have TLS connections respect system
configuration. This is a lower-level library.

[![Crates.io](https://img.shields.io/crates/v/openssl-probe.svg?maxAge=2592000)](https://crates.io/crates/openssl-probe)
[![docs.rs](https://docs.rs/openssl-probe/badge.svg)](https://docs.rs/openssl-probe/)

[`rustls-platform-verifier`]: https://crates.io/crates/rustls-platform-verifier
[`rustls-native-certs`]: https://crates.io/crates/rustls-native-certs

## Usage

```rust
fn main() {
    let result = openssl_probe::probe();
    if let Some(dir) = &result.cert_dir {
        //... your code
    }
    if let Some(file) = &result.cert_file {
        //... your code
    }
}
```

## License

`openssl-probe` is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0),
with portions covered by various BSD-like licenses.

See [LICENSE-APACHE](./LICENSE-APACHE), and [LICENSE-MIT](LICENSE-MIT) for details.
