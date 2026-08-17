#!/usr/bin/env bash

# Setup LD_LIBRARY_PATH for ITs
# shellcheck source=/dev/null
source test_profile

# Run all tests with invocation feature (enables JavaVM ITs)
cargo test --features=invocation
