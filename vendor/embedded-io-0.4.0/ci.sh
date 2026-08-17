#!/bin/bash

set -euxo pipefail

export RUSTFLAGS=-Dwarnings
export RUSTDOCFLAGS="--cfg=docsrs -Dwarnings"

cargo fmt --check
cargo check
cargo check --features alloc
cargo check --features std
cargo check --features async
cargo check --features alloc,async
cargo check --features std,async
cargo check --features tokio
cargo check --features futures
cargo check --features tokio,futures
RUSTUP_TOOLCHAIN=stable cargo check
RUSTUP_TOOLCHAIN=stable cargo check --features alloc
RUSTUP_TOOLCHAIN=stable cargo check --features std
cargo rustdoc --features std,async,defmt,tokio,futures