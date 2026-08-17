# webpki-root-certs

This is a crate containing Mozilla's trusted root certificates in self-signed
X.509 certificate format.

**If you are using `webpki` or `rustls` you should prefer `webpki-roots` - it is
more space efficient and easier to use.**

This crate is inspired by [certifi.io](https://certifi.io/en/latest/) and uses the data provided by the
[Common CA Database (CCADB)](https://www.ccadb.org/).

# About

The `webpki` and `rustls` ecosystem represent trust anchors with the
`webpki::TrustAnchor` type, containing only the data used as inputs for the [RFC
5280] certificate path validation algorithm. In some instances (e.g. when
interacting with native platform certificate verifiers) it may be required to
provide trust anchors as full X.509 self-signed certificates.

Compared to `webpki-roots` this crate contains the full self-signed certificate
DER data for each trust anchor is included in `webpki_roots`.

[RFC 5280]: https://www.rfc-editor.org/rfc/rfc5280#section-6

# License

The underlying data is MPL-licensed, and `src/lib.rs` is therefore a derived work.

# Regenerating sources

Sources are generated in an integration test, in `tests/codegen.rs`. The test
will fail if the sources are out of date relative to upstream, and update
`src/lib.rs` if so. The code is generated in deterministic order so changes
to the source should only result from upstream changes.
