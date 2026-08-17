#!/bin/sh

set -e

RUSTFLAGS="-Ctarget-feature=+simd128" wasm-pack test --headless --firefox
wasm-pack test --headless --firefox
