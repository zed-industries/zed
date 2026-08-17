#!/bin/sh

set -e

# DEPENDENCIES="arbitrary approx bytemuck encase mint rand rkyv serde speedy zerocopy debug-glam-assert"
# remove optional dependencies that require a newer version of rust
DEPENDENCIES="arbitrary approx mint speedy debug-glam-assert"

CARGO='rustup run 1.68.2 cargo'
$CARGO check --features "$DEPENDENCIES" && \
$CARGO check --features "scalar-math $DEPENDENCIES" && \
$CARGO check --no-default-features --features "libm scalar-math $DEPENDENCIES"
