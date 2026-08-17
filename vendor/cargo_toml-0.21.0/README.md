# Deserialize `Cargo.toml`

<img src="the-real-tom.jpeg" align="left" alt="tom replacement" title="due to a milkshake duck situation, the preferred Tom for this format has been replaced">

This is a definition of fields in `Cargo.toml` files for [serde](https://serde.rs). It allows reading of `Cargo.toml` data, and serializing it using TOML or other formats. It's used by [the lib.rs site](https://lib.rs) to extract information about crates.

This crate is more than just schema definition. It supports post-processing of the data to emulate Cargo's workspace inheritance and `autobins` features. It supports files on disk as well as other non-disk data sources.

To get started, see [`Manifest::from_slice`][docs]. If you need to get information about Cargo projects local to devs' machines, also consider [cargo_metadata](lib.rs/crates/cargo_metadata).

[docs]: https://docs.rs/cargo_toml/latest/cargo_toml/struct.Manifest.html#method.from_slice

## Features

 * Allows parsing `Cargo.toml` independently of Cargo. It can read manifests that use nightly features, without requiring a nightly Cargo version. Unlike `cargo metadata`, this is a standalone self-contained implementation, and it doesn't run any external commands.

 * It is safe to use with untrusted code. It is just a parser. It won't run any build commands nor apply any `.cargo/config.toml` files.

 * It supports Cargo workspaces and inheritance of fields.

 * It supports abstracting the file system, so parsing of `Cargo.toml` can auto-detect files [parsed from `.crate` tarballs](https://lib.rs/crates/crate_untar), bare git repositories, and other data sources, without having to extract the files to disk first.

 * It has optional helper functions for interpreting the `[features]` section.

## There will be updates

Cargo regularly adds new features to `Cargo.toml`. Keep this crate up-to-date to correctly parse them all — **use [dependabot][db] or [renovate][ren]**.

[db]: https://docs.github.com/en/code-security/dependabot/dependabot-version-updates/configuring-dependabot-version-updates
[ren]: https://docs.renovatebot.com/rust/
