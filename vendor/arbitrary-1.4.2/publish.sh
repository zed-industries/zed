#!/usr/bin/env bash

set -eux

cd $(dirname $0)/derive

cargo publish

cd ..

# Let the crates.io index figure out we've published `derive_arbitrary` already.
sleep 5

cargo publish
