#!/bin/sh

set -e

CARGO='rustup run 1.52.1 cargo'
$CARGO test --features "bytemuck mint rand serde debug-glam-assert transform-types" && \
$CARGO test --features "scalar-math bytemuck mint rand serde debug-glam-assert transform-types" && \
$CARGO test --no-default-features --features "libm scalar-math bytemuck mint rand serde debug-glam-assert transform-types" && \
$CARGO bench --no-run
