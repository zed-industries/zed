#!/bin/bash -x
set -ex

cargo "$@" build
cargo "$@" test --all-features
cargo "$@" test --all-features --examples

cargo "$@" test --bench json --bench http -- --test
cargo "$@" check --bench mp4 --features mp4

cargo "$@" build --no-default-features --features alloc
cargo "$@" test --no-default-features --features alloc --examples

cargo "$@" build --no-default-features
cargo "$@" test --no-default-features --examples

cargo "$@" check --no-default-features --features tokio-02
cargo "$@" check --no-default-features --features tokio-03
