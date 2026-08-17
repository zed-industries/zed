#!/usr/bin/env bash

set -eux
cd $(dirname $0)/..

export CARGO_TARGET_DIR=$(pwd)/target

cargo test --doc

pushd ./example
cargo fuzz build
cargo fuzz build  --dev
(! cargo fuzz run bananas -- -runs=100000)
popd

pushd ./example_arbitrary
cargo fuzz build
cargo fuzz build  --dev
(! cargo fuzz run rgb -- -runs=10000000)
RUST_LIBFUZZER_DEBUG_PATH=$(pwd)/debug_output \
    cargo fuzz run rgb \
    $(ls ./fuzz/artifacts/rgb/crash-* | head -n 1)
cat $(pwd)/debug_output
grep -q Rgb $(pwd)/debug_output
popd

pushd ./example_mutator
cargo fuzz build
cargo fuzz build  --dev
(! cargo fuzz run boom -- -runs=10000000)
popd

pushd ./example_crossover
cargo fuzz build
cargo fuzz build  --dev
(! cargo fuzz run --release boom -- -runs=10000000)
popd

pushd ./example_init
cargo fuzz build
cargo fuzz build  --dev
(! cargo fuzz run --release bigbang -- -runs=10000000)
popd

echo "All good!"
