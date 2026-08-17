<!-- markdownlint-disable no-inline-html first-line-heading -->

<div align="center">

# `🆔 spdx`

**Helper crate for [SPDX](https://spdx.org/about) [license expressions](https://spdx.org/spdx-specification-21-web-version#h.jxpfx0ykyb60)**

[![Embark](https://img.shields.io/badge/embark-open%20source-blueviolet.svg)](https://embark.dev)
[![Embark](https://img.shields.io/badge/discord-ark-%237289da.svg?logo=discord)](https://discord.gg/dAuKfZS)
[![Crates.io](https://img.shields.io/crates/v/spdx.svg)](https://crates.io/crates/spdx)
[![Docs](https://docs.rs/spdx/badge.svg)](https://docs.rs/spdx)
[![Minimum Stable Rust Version](https://img.shields.io/badge/Rust-1.65.0-blue?color=fc8d62&logo=rust)](https://blog.rust-lang.org/2022/11/03/Rust-1.65.0.html)
[![SPDX Version](https://img.shields.io/badge/SPDX%20Version-3.27.0-blue.svg)](https://spdx.org/licenses/)
[![dependency status](https://deps.rs/repo/github/EmbarkStudios/spdx/status.svg)](https://deps.rs/repo/github/EmbarkStudios/spdx)
[![Build Status](https://github.com/EmbarkStudios/spdx/workflows/CI/badge.svg)](https://github.com/EmbarkStudios/spdx/actions?workflow=CI)

</div>

## Usage

```rust
use spdx::Expression;

fn main() {
    let this_is_fine = Expression::parse("MIT OR Apache-2.0").unwrap();

    assert!(this_is_fine.evaluate(|req| {
        if let spdx::LicenseItem::Spdx { id, .. } = req.license {
            // Both MIT and Apache-2.0 are OSI approved, so this expression
            // evaluates to true
            return id.is_osi_approved();
        }

        false
    }));

    assert!(!this_is_fine.evaluate(|req| {
        if let spdx::LicenseItem::Spdx { id, .. } = req.license {
            // This is saying we don't accept any licenses that are OSI approved
            // so the expression will evaluate to false as both sides of the OR
            // are now rejected
            return !id.is_osi_approved();
        }

        false
    }));

    // `NOPE` is not a valid SPDX license identifier, so this expression
    // will fail to parse
    let _this_is_not = Expression::parse("MIT OR NOPE").unwrap_err();
}
```

## Updating SPDX list

You can update the list of SPDX identifiers for licenses and exceptions by running the update program `cargo run --manifest-path=update/Cargo.toml -- v3.6` where `v3.6` is the tag in the [SPDX data repo](https://github.com/spdx/license-list-data).

## Contributing

[![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4-ff69b4.svg)](CODE_OF_CONDUCT.md)

We welcome community contributions to this project.

Please read our [Contributor Guide](CONTRIBUTING.md) for more information on how to get started.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
