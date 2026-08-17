#!/bin/bash

set -ex

# Supported dependencies
DEPENDENCIES="arbitrary approx bytemuck encase mint rand rkyv serde speedy zerocopy debug-glam-assert"

# Set of features to build & test.
FEATURE_SETS=(
  # std
  "std"
  "std $DEPENDENCIES"
  "std $DEPENDENCIES bytecheck"
  "std scalar-math $DEPENDENCIES"
  "std cuda"
  "std scalar-math cuda"
  "std libm"
  "std scalar-math libm"
  # no_std
  "libm"
  "libm scalar-math $DEPENDENCIES"
)

rustc --version

for features in "${FEATURE_SETS[@]}"
do
  :
  cargo build --tests --no-default-features --features="$features"
  cargo test --no-default-features --features="$features"
done

RUSTFLAGS='-C target-feature=+fma' cargo check

cargo check -p glam-no_std
