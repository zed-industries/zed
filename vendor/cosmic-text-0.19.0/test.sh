#!/usr/bin/env bash

set -ex

echo Run CI script
./ci.sh

echo Build documentation
cargo doc

echo Build all examples
cargo build --release --all

echo Run terminal example
target/release/terminal

echo Run editor-test example
env RUST_LOG=editor_test=info target/release/editor-test
