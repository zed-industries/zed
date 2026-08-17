# rustls-pemfile
This is a basic parser for PEM-encodings commonly used for storing keys and certificates at rest.

It doesn't support reading encrypted keys: the cryptography standardised for this is typically very
poor and doing so doesn't address a meaningful threat model.

[![Build Status](https://github.com/rustls/pemfile/workflows/rustls-pemfile/badge.svg)](https://github.com/rustls/pemfile/actions)
[![Crate](https://img.shields.io/crates/v/rustls-pemfile.svg)](https://crates.io/crates/rustls-pemfile)
[![Documentation](https://docs.rs/rustls-pemfile/badge.svg)](https://docs.rs/rustls-pemfile/)

# See also: rustls-pki-types

The main function of this crate has been incorporated into
[rustls-pki-types](https://crates.io/crates/rustls-pki-types). 2.2.0 maintains the
existing public API for this crate, on top of this new implementation. This drops
the dependency on the `base64` crate, and allows for constant-time decoding of private keys.

This crate will continue to exist in its current form, but it is somewhat unlikely that the
API will be extended from its current state.

Should you wish to migrate to using the new [`rustls-pki-types` PEM APIs](https://docs.rs/rustls-pki-types/latest/rustls_pki_types/pem/trait.PemObject.html)
directly, here is a rough cheat-sheet:

| *Use case* | *Replace* |
|---|---|
| File stream to `CertificateDer` iterator |`rustls_pemfile::certs(io::BufRead)` <br>  ➡️ <br> `CertificateDer::pem_reader_iter(io::Read)` |
| File stream to one `PrivateKeyDer` | `rustls_pemfile::private_key(io::BufRead)` <br> ➡️  <br> `PrivateKeyDer::from_pem_reader(io::Read)` |
| File stream to one `CertificateSigningRequestDer` | `rustls_pemfile::csr(io::BufRead)` <br> ➡️  <br> `CertificateSigningRequestDer::from_pem_reader(io::Read)` |
| File stream to `CertificateRevocationListDer` iterator |`rustls_pemfile::crls(io::BufRead)` <br>  ➡️ <br> `CertificateRevocationListDer::pem_reader_iter(io::Read)` |
| File stream to `PrivatePkcs1KeyDer` iterator |`rustls_pemfile::rsa_private_keys(io::BufRead)` <br>  ➡️ <br> `PrivatePkcs1KeyDer::pem_reader_iter(io::Read)` |
| File stream to `PrivatePkcs8KeyDer` iterator |`rustls_pemfile::pkcs8_private_keys(io::BufRead)` <br>  ➡️ <br> `PrivatePkcs8KeyDer::pem_reader_iter(io::Read)` |
| File stream to `PrivateSec1KeyDer` iterator |`rustls_pemfile::ec_private_keys(io::BufRead)` <br>  ➡️ <br> `PrivateSec1KeyDer::pem_reader_iter(io::Read)` |
| File stream to `SubjectPublicKeyInfoDer` iterator |`rustls_pemfile::public_keys(io::BufRead)` <br>  ➡️ <br> `SubjectPublicKeyInfoDer::pem_reader_iter(io::Read)` |

# Changelog

The detailed list of changes in each release can be found at
https://github.com/rustls/pemfile/releases.

# License
rustls-pemfile is distributed under the following three licenses:

- Apache License version 2.0.
- MIT license.
- ISC license.

These are included as LICENSE-APACHE, LICENSE-MIT and LICENSE-ISC
respectively.  You may use this software under the terms of any
of these licenses, at your option.
