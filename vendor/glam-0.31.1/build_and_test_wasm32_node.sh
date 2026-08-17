#!/bin/sh

set -e

# test with simd128
RUSTFLAGS="-Ctarget-feature=+simd128" wasm-pack test --node

# test without simd128
wasm-pack test --node
