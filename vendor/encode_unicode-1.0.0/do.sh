#!/usr/bin/env bash
set -e -o pipefail

MSRV=1.56.1
FUZZ_DURATION=60
FUZZ_PAUSE=2

if [[ ${1:0:1} == - || $1 == help ]] || (( $# > 1 )); then
    echo "A script to make it easy to check & lint & test everything." >&2
    echo "It assumes rustup is installed and that cargo +release works." >&2
    echo >&2
    echo "Usage: $0 ([setup|MSRV|check|test|ignored|clippy|miri|fuzz|bench|shellcheck|help])" >&2
    echo "If no argument is provided, all parts except ignored and help are run," >&2
    echo "but setup is only done if auto-detection fails." >&2
    exit 1
fi

# should have been a Makefile

# core check, Minimum supported Rust version
if [[ $1 == setup ]] || ! rustup show | grep --silent "$MSRV"; then
    rustup install "$MSRV" --no-self-update
fi
if [[ -z $1 || $1 == msrv ]]; then
    # FIXME modify Cargo.toml like on CI, and then restore it and Cargo.lock afterwards
    cargo "+$MSRV" build --all-features
fi

# check all feature combinations, stable
if [[ $1 == setup ]] || ! rustup show | grep --silent stable; then
    rustup install stable --no-self-update
fi
if [[ -z $1 || $1 == check ]]; then
    cargo +stable check --examples --tests --no-default-features
    cargo +stable check --examples --tests --no-default-features --features std
    cargo +stable check --examples --tests --no-default-features --features ascii
    cargo +stable check --examples --tests --all-features
fi

# tests, stable
if [[ -z $1 || $1 == test ]]; then
    cargo +stable test --all-features -- --quiet
elif [[ $1 == ignored ]]; then
    cargo +stable test --all-features -- --quiet --ignored
fi

# clippy, nightly
if [[ $1 == setup ]] || ! rustup show | grep --silent nightly; then
    rustup install nightly --no-self-update
fi
if [[ $1 == setup ]] || ! cargo +nightly help clippy >/dev/null 2>/dev/null; then
    rustup component add clippy --toolchain nightly
fi
if [[ -z $1 || $1 == clippy ]]; then
    cargo +nightly clippy --all-features --tests --benches --examples
fi

# miri, nightly
if [[ $1 == setup ]] || ! cargo +nightly help miri >/dev/null 2>/dev/null; then
    rustup component add miri --toolchain nightly
    cargo +nightly miri setup
fi
if [[ -z $1 || $1 == miri ]]; then
    cargo +nightly miri test --all-features -- --quiet
fi

# fuzzing tests, nightly
if [[ $1 == setup ]] || ! command -V cargo-fuzz >/dev/null 2>/dev/null; then
    cargo +nightly install cargo-fuzz
fi
if [[ -z $1 || $1 == fuzz ]]; then
    cargo +nightly fuzz build
    for fuzztest in $(cargo +nightly fuzz list); do
        sleep "$FUZZ_PAUSE"
        echo "Fuzzing $fuzztest"
        timeout "$FUZZ_DURATION" \
            cargo +nightly fuzz run "$fuzztest" \
            || true
        echo
    done
fi

# benchmarks, nightly
if [[ -z $1 || $1 == bench ]]; then
    cargo +nightly check --benches --no-default-features
    cargo +nightly check --benches --no-default-features --features std
    cargo +nightly check --benches --no-default-features --features ascii
    cargo +nightly check --benches --all-features
    # need nocapture to not hide error if setup fails
    cargo +nightly bench --all-features -- --nocapture
fi

if [[ $1 == shellcheck || $1 == selfcheck ]] \
|| ([[ -z $1 ]] && command -V shellcheck >/dev/null 2>/dev/null); then
    shellcheck "$0"
fi
