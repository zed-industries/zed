#!/bin/sh

set -e

CARGO='rustup run 1.68.2 cargo'
$CARGO check --features "bytemuck mint rand serde debug-glam-assert" && \
$CARGO check --features "scalar-math bytemuck mint rand serde debug-glam-assert" && \
$CARGO check --no-default-features --features "libm scalar-math bytemuck mint rand serde debug-glam-assert"
