# cargo_metadata

Structured access to the output of `cargo metadata`. Usually used from within a `cargo-*` executable.

Also supports serialization to aid in implementing `--message-format=json`-like
output generation in `cargo-*` subcommands, since some of the types in what
`cargo --message-format=json` emits are exactly the same as the ones from `cargo metadata`.

[![Build Status](https://github.com/oli-obk/cargo_metadata/workflows/CI/badge.svg?branch=main)](https://github.com/oli-obk/cargo_metadata/actions/workflows/main.yml?query=branch%3Amain)
[![crates.io](https://img.shields.io/crates/v/cargo_metadata.svg)](https://crates.io/crates/cargo_metadata)

[Documentation](https://docs.rs/cargo_metadata/)
