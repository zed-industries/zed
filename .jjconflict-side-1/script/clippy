#!/usr/bin/env bash

set -euo pipefail

if [[ ! " $* " == *" -p "* && ! " $* " == *" --package "* ]]; then
    set -- "$@" --workspace
fi

set -x
"${CARGO:-cargo}" clippy "$@" --release --all-targets --all-features -- --deny warnings

# If local, run other checks if we have the tools installed.
if [[ -z "${GITHUB_ACTIONS+x}" ]]; then
    which cargo-machete >/dev/null 2>&1 || exit 0
    cargo machete

    which typos >/dev/null 2>&1 || exit 0
    typos --config typos.toml
fi
