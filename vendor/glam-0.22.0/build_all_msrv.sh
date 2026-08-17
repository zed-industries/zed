#!/bin/sh

set -e

CARGO='rustup run 1.58.1 cargo'
$CARGO test --features "bytemuck mint rand serde debug-glam-assert" && \
$CARGO test --features "scalar-math bytemuck mint rand serde debug-glam-assert" && \
$CARGO test --no-default-features --features "libm scalar-math bytemuck mint rand serde debug-glam-assert" && \
$CARGO bench --no-run
