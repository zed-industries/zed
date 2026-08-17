#!/bin/bash

set -ex

echo Testing num-bigint on rustc ${TRAVIS_RUST_VERSION}

FEATURES="serde i128 u64_digit prime"

export RUST_BACKTRACE=1

# num-bigint should build and test everywhere.
cargo build --verbose
cargo test --verbose

# It should build with minimal features too.
cargo build --no-default-features --features="std"
cargo test --no-default-features --features="std"

# It should build in no_std
if [[ "$TRAVIS_RUST_VERSION" == "nightly" ]]; then
  rustup target add thumbv7m-none-eabi
  cargo build --no-default-features --target=thumbv7m-none-eabi

  # It should work in no_std on nightly.
  # Note: Doctest might show an error: https://github.com/rust-lang/rust/issues/54010
  # The "error" is wrong however, the doctests still run.
  cargo test --no-default-features
fi

# Each isolated feature should also work everywhere.
for feature in $FEATURES; do
  cargo build --verbose --no-default-features --features="std $feature"
  cargo test --verbose --no-default-features --features="std $feature"

  # Ensure that feature also works in nostd context on nightly.
  if [[ "$TRAVIS_RUST_VERSION" == "nightly" ]]; then
    cargo build --verbose --no-default-features --features="$feature"
    cargo test --verbose --no-default-features --features="$feature"
  fi
done

# test all supported features together
cargo build --features="std $FEATURES"
cargo test --features="std $FEATURES"

# make sure benchmarks can be built
if [[ "$TRAVIS_RUST_VERSION" == "nightly" ]]; then
  cd benchmark_crate
  cargo bench --all-features --no-run
fi
