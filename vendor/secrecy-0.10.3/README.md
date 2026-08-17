# secrecy.rs 🤐 <a href="https://www.iqlusion.io"><img src="https://storage.googleapis.com/iqlusion-production-web/img/logo/iqlusion-rings-sm.png" alt="iqlusion" width="24" height="24"></a>

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Apache 2.0/MIT Licensed][license-image]
![MSRV][rustc-image]
[![Safety Dance][safety-image]][safety-link]
[![Build Status][build-image]][build-link]

A simple secret-keeping library for Rust.

[Documentation][docs-link]

## About

**secrecy** is a *simple*, safe (i.e. `forbid(unsafe_code)` library which
provides wrapper types and traits for secret management in Rust, namely the
`Secret<T>` type for wrapping another value in a "secret cell" which attempts
to limit exposure (only available through a special `ExposeSecret` trait).

This helps to ensure secrets aren't accidentally copied, logged, or otherwise
exposed (as much as possible), and also ensures secrets are securely wiped
from memory when dropped.

## Minimum Supported Rust Version

Rust **1.60** or newer.

In the future, we reserve the right to change MSRV (i.e. MSRV is out-of-scope
for this crate's SemVer guarantees), however when we do it will be accompanied by
a minor version bump.

## serde support

Optional `serde` support for parsing owned secret values is available, gated
under the `serde` cargo feature.

It uses the `Deserialize` and `DeserializeOwned` traits to implement
deserializing secret types which also impl these traits.

This doesn't guarantee `serde` (or code providing input to `serde`) won't
accidentally make additional copies of the secret, but does the best it can
with what it is given and tries to minimize risk of exposure as much as
possible.

## License

Copyright © 2019-2024 iqlusion

**secrecy** is distributed under the terms of either the MIT license
or the Apache License (Version 2.0), at your option.

See [LICENSE] (Apache License, Version 2.0) file in the `iqlusioninc/crates`
toplevel directory of this repository or [LICENSE-MIT] for details.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you shall be dual licensed as above,
without any additional terms or conditions.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/secrecy.svg?logo=rust
[crate-link]: https://crates.io/crates/secrecy
[docs-image]: https://docs.rs/secrecy/badge.svg
[docs-link]: https://docs.rs/secrecy/
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.60+-blue.svg
[safety-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[safety-link]: https://github.com/rust-secure-code/safety-dance/
[build-image]: https://github.com/iqlusioninc/crates/actions/workflows/secrecy.yml/badge.svg
[build-link]: https://github.com/iqlusioninc/crates/actions/workflows/secrecy.yml

[//]: # (general links)

[LICENSE]: https://github.com/iqlusioninc/crates/blob/main/LICENSE
[LICENSE-MIT]: https://github.com/iqlusioninc/crates/blob/main/secrecy/LICENSE-MIT
