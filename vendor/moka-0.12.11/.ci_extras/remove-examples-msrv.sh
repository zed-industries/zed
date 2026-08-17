#!/usr/bin/env bash

# Disable examples from the MSRV build.

set -eux

function disable_example() {
    local example_name="$1"

    mv ./examples/${example_name}.rs ./examples/${example_name}.rs.bak

    # Replace the main function of example $1.
    cat << EOF > ./examples/${example_name}.rs
fn main() {}
EOF

    echo "Disabled $example_name."
}

# `OnceLock` was introduced in 1.70.0.
# disable_example reinsert_expired_entries_sync
