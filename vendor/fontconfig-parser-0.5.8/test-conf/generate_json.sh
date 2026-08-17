#!/bin/bash

# Usage: ./test-conf/generate_json.sh

cargo build --all-features --release --example parse_dump_json

for f in ./test-conf/**/*.conf; do
    echo "Processing $f file..."

    ./target/release/examples/parse_dump_json $f | jq > $f.json
done
