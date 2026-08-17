#!/bin/sh

set -eux

# Downgrade reqwest v0.12.x in Cargo.toml to v0.11.11.
cargo remove --dev reqwest
cargo add --dev reqwest@0.11.11 --no-default-features --features rustls-tls

# Pin some dependencies to specific versions for the MSRV.
cargo update -p url --precise 2.5.2
cargo update -p actix-rt --precise 2.10.0
cargo update -p tokio-rustls --precise 0.24.1
