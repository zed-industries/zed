#!/bin/bash

set -e  # Exit immediately if any command exits with a non-zero status

# This is a Temporary Fix for CI fail as CI fails on nightly toolchain due to some formatting issues which I am unable
# to reproduce locally.
#
# For more info, check https://github.com/AS1100K/pastey/pull/15
if [[ "$1" == "nightly" ]]; then
    cp pastey-test-suite/tests/ui/case-warning.nightly.stderr pastey-test-suite/tests/ui/case-warning.stderr
    cp pastey-test-suite/tests/ui/raw-mode-wrong-position.nightly.stderr pastey-test-suite/tests/ui/raw-mode-wrong-position.stderr
else
    cp pastey-test-suite/tests/ui/case-warning.stable.stderr pastey-test-suite/tests/ui/case-warning.stderr
    cp pastey-test-suite/tests/ui/raw-mode-wrong-position.stable.stderr pastey-test-suite/tests/ui/raw-mode-wrong-position.stderr
fi

echo "========================================"
echo "Running tests with pastey crate..."
echo "========================================"
cargo test

echo "========================================"
echo "Running tests with pastey-test-suite crate..."
echo "========================================"
cd pastey-test-suite
cargo test
rm ./tests/ui/case-warning.stderr
rm ./tests/ui/raw-mode-wrong-position.stderr
cd ../

cd paste-compat
./test.sh
cd ../
