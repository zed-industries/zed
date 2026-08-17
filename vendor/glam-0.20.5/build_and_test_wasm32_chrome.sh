#!/bin/sh

set -e

RUSTFLAGS="-Ctarget-feature=+simd128" wasm-pack test --headless --chrome
wasm-pack test --headless --chrome
