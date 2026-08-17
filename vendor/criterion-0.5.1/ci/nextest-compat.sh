#!/bin/bash
set -ex -o pipefail

CARGO=${CARGO:-cargo}

cd "$(git rev-parse --show-toplevel)"

echo "Checking benches/bench_main..."

$CARGO nextest list --benches
$CARGO nextest run --benches
