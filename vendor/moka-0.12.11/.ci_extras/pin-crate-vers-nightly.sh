#!/bin/sh

set -eux

# Downgrade reqwest v0.12.x in Cargo.toml to v0.11.11.
cargo remove --dev reqwest
cargo add --dev reqwest@0.11.11 --no-default-features --features rustls-tls

# Pin some dependencies to specific versions for the nightly toolchain.
# cargo update -p <crate> --precise <version>
# https://github.com/tkaitchuck/aHash/issues/200
cargo update -p ahash --precise 0.8.7
