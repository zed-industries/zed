#!/bin/bash
set -e
if [ -d ./build-tools ]; then
    targets=(
        "Cargo.toml"
        "sea-query-attr/Cargo.toml"
        "sea-query-binder/Cargo.toml"
        "sea-query-derive/Cargo.toml"
        "sea-query-postgres/Cargo.toml"
        "sea-query-rusqlite/Cargo.toml"
    )

    for target in "${targets[@]}"; do
        echo "cargo +nightly fmt --manifest-path ${target} --all"
        cargo +nightly fmt --manifest-path "${target}" --all
    done

    examples=(`find examples -type f -name 'Cargo.toml'`)
    for example in "${examples[@]}"; do
        echo "cargo +nightly fmt --manifest-path ${example} --all"
        cargo +nightly fmt --manifest-path "${example}" --all
    done
else
    echo "Please execute this script from the repository root."
fi
