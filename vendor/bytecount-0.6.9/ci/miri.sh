#!/bin/bash
set -ex

# Setup
MIRI_NIGHTLY=nightly-$(curl -s https://rust-lang.github.io/rustup-components-history/x86_64-unknown-linux-gnu/miri)
echo "Installing latest nightly with Miri: $MIRI_NIGHTLY"
rustup default "$MIRI_NIGHTLY"
rustup component add miri

# Run tests
cargo miri test
cargo miri test --target=mips64-unknown-linux-gnuabi64 # big-endian architecture