#!/usr/bin/env bash
# Builds the example plugin as a wasm32-wasip2 component.
# Requires: rustup target add wasm32-wasip2
set -euo pipefail
cd "$(dirname "$0")"
cargo build --manifest-path example_plugin/Cargo.toml --target wasm32-wasip2 --release
echo "Component: $(pwd)/example_plugin/target/wasm32-wasip2/release/example_plugin.wasm"
