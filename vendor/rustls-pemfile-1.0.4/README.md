# rustls-pemfile
This is a basic parser for PEM-encodings commonly used for storing keys and certificates at rest.

It doesn't support reading encrypted keys: the cryptography standardised for this is typically very
poor and doing so doesn't address a meaningful threat model.

[![Build Status](https://github.com/rustls/pemfile/workflows/rustls-pemfile/badge.svg)](https://github.com/rustls/pemfile/actions)
[![Crate](https://img.shields.io/crates/v/rustls-pemfile.svg)](https://crates.io/crates/rustls-pemfile)
[![Documentation](https://docs.rs/rustls-pemfile/badge.svg)](https://docs.rs/rustls-pemfile/)

# Release history
- 1.0.4 (2023-11-09)
  * Enable parsing PEM files with items that have non-UNIX line endings.
- 1.0.3 (2023-06-28)
  * Add certificate revocation list (CRL) format support.
  * Add `crls` helper function.
- 1.0.2 (2023-01-10)
  * Add `ec_private_keys()` helper function.
  * Update base64 to the latest version.
- 1.0.1 (2022-08-02)
  * Enable parsing PEM files with non-UTF-8 content between items.
- 1.0.0 (2022-04-14)
  * Initial stable release. No API changes.
- 0.3.0 (2022-02-05)
  * Add SEC1 EC key format support (ie, "EC PRIVATE KEY" sections) thanks to @farcaller.
  * Make `Item` enum non-exhaustive.
- 0.2.1 (2021-04-17)
  * Performance improvements thanks to @zz85.
- 0.2.0 (2020-12-28)
  * Initial release.

# License
rustls-pemfile is distributed under the following three licenses:

- Apache License version 2.0.
- MIT license.
- ISC license.

These are included as LICENSE-APACHE, LICENSE-MIT and LICENSE-ISC
respectively.  You may use this software under the terms of any
of these licenses, at your option.
