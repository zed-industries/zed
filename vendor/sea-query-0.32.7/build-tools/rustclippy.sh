#!/bin/bash
set -e
if [ -d ./build-tools ]; then
    targets=(
        "Cargo.toml --all-features"
        "sea-query-attr/Cargo.toml"
        "sea-query-binder/Cargo.toml"
        "sea-query-derive/Cargo.toml"
        "sea-query-postgres/Cargo.toml"
        "sea-query-rusqlite/Cargo.toml"
    )

    for target in "${targets[@]}"; do
        echo "cargo clippy --manifest-path ${target} --fix --allow-dirty --allow-staged"
        cargo clippy --manifest-path ${target} --fix --allow-dirty --allow-staged
    done

    examples=(`find examples -type f -name 'Cargo.toml'`)
    for example in "${examples[@]}"; do
        echo "cargo clippy --manifest-path ${example} --fix --allow-dirty --allow-staged"
        cargo clippy --manifest-path "${example}" --fix --allow-dirty --allow-staged
    done
else
    echo "Please execute this script from the repository root."
fi
