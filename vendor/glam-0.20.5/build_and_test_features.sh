#!/bin/bash

set -e

# Set of features to build & test.
FEATURE_SETS=(
  # std
  "std"
  "std approx bytemuck mint rand serde debug-glam-assert transform-types"
  "std scalar-math approx bytemuck mint rand serde debug-glam-assert transform-types"
  "std cuda"
  "std scalar-math cuda"
  # no_std
  "libm"
  "libm scalar-math approx bytemuck mint rand serde debug-glam-assert transform-types"
)

rustc --version

for features in "${FEATURE_SETS[@]}"
do
   :
   cargo build --tests --no-default-features --features="$features"
   echo cargo test --no-default-features --features=\"$features\"
   cargo test --no-default-features --features="$features"
done

pushd test_no_std && cargo check
