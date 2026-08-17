#!/bin/sh

cargo build --target wasm32-wasi
cp target/wasm32-wasi/debug/handlebars-cli.wasm wasm/
