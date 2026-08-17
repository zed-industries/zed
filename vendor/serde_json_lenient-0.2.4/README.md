# serde_json_lenient &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] [![Rustc Version 1.36+]][rustc]

[Build Status]: https://img.shields.io/github/actions/workflow/status/google/serde_json_lenient/ci.yml?branch=master
[actions]: https://github.com/google/serde_json_lenient/actions?query=branch%3Amaster
[Latest Version]: https://img.shields.io/crates/v/serde_json_lenient.svg
[crates.io]: https://crates.io/crates/serde\_json\_lenient
[Rustc Version 1.36+]: https://img.shields.io/badge/rustc-1.36+-lightgray.svg
[rustc]: https://blog.rust-lang.org/2019/07/04/Rust-1.36.0.html

This is a lenient JSON parser forked from the
[serde_json](https://crates.io/crates/serde_json) crate
that is that is designed to parse JSON written by humans
(e.g., JSON config files). This means that it supports:

- `/*` and `//` style comments.
- Trailing commas for object and array literals.
- `\v` and `\xDD` literal escapes (for vertical tab and
  two-digit hexadecimal characters)
- [planned] Unquoted object keys (precise spec TBD).

Each such feature is switchable.

Earlier work to make `serde_json` more lenient was performed
by Michael Bolin as the crate [serde_jsonrc](https://docs.rs/serde_jsonrc/latest/serde_jsonrc/).
This crate builds on his work and updates to more recent [serde_json].

### Why not make `serde_json` more lenient?

[The maintainer wanted to keep the
scope of `serde_json` limited to strict JSON](https://github.com/dtolnay/request-for-implementation/issues/24),
so we respectfully agreed that forking was the way to go.

## License

Because serde_json_lenient is a fork of serde_json, it maintains the original licence,
which means it is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in serde_json_lenient by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
