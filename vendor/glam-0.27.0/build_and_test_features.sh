#!/bin/bash

set -ex

# Set of features to build & test.
FEATURE_SETS=(
  # std
  "std"
  "std approx bytemuck mint rand serde debug-glam-assert"
  "std scalar-math approx bytemuck mint rand serde debug-glam-assert"
  "std cuda"
  "std scalar-math cuda"
  "std libm"
  "std scalar-math libm"
  # no_std
  "libm"
  "libm scalar-math approx bytemuck mint rand serde debug-glam-assert"
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
