#!/bin/sh

set -e

WASM_BINDGEN_USE_BROWSER=1 RUSTFLAGS="-Ctarget-feature=+simd128" wasm-pack test --headless --firefox
WASM_BINDGEN_USE_BROWSER=1 wasm-pack test --headless --firefox
