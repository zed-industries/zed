#!/bin/sh

set -e

# wasm64 requires nightly, -Zbuild-std, and wasmtime with memory64 support.

CARGO_TARGET_WASM64_UNKNOWN_UNKNOWN_RUNNER="wasmtime --wasm memory64" \
RUSTFLAGS="-Ctarget-feature=+simd128" \
  cargo +nightly test \
  --target wasm64-unknown-unknown \
  --no-default-features --features libm \
  -Zbuild-std=std,panic_abort \
  -Zpanic-abort-tests

CARGO_TARGET_WASM64_UNKNOWN_UNKNOWN_RUNNER="wasmtime --wasm memory64" \
  cargo +nightly test \
  --target wasm64-unknown-unknown \
  --no-default-features --features libm \
  -Zbuild-std=std,panic_abort \
  -Zpanic-abort-tests
