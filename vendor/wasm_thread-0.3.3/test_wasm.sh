#!/bin/sh

RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' \
	wasm-pack test --headless --firefox -- -Z build-std=panic_abort,std
