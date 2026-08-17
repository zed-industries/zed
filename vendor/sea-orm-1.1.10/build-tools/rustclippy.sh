#!/bin/bash
set -e
if [ -d ./build-tools ]; then
    targets=(
        "Cargo.toml"
        "sea-orm-cli/Cargo.toml"
        "sea-orm-codegen/Cargo.toml"
        "sea-orm-macros/Cargo.toml"
        "sea-orm-migration/Cargo.toml"
        "sea-orm-rocket/Cargo.toml"
    )

    for target in "${targets[@]}"; do
        echo "cargo clippy --manifest-path ${target} --fix --allow-dirty --allow-staged"
        cargo clippy --manifest-path "${target}" --fix --allow-dirty --allow-staged
    done

    examples=(`find examples -type f -name 'Cargo.toml'`)
    for example in "${examples[@]}"; do
        echo "cargo clippy --manifest-path ${example} --fix --allow-dirty --allow-staged"
        cargo clippy --manifest-path "${example}" --fix --allow-dirty --allow-staged
    done
else
    echo "Please execute this script from the repository root."
fi
