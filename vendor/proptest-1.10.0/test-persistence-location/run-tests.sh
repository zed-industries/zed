#! /bin/sh

find . -name \*.txt -o -name proptest-regressions -depth -exec rm -rf {} \; || \
    exit $?

(
    cd single-crate
    cargo test >cargo-out.txt 2>&1 # Ignore expected failure
    cargo clean >/dev/null
    if ! test -f proptest-regressions/submodule/code.txt; then
        echo >&2 "Persistence file not written to the correct location. FS:"
        find . >&2
        echo >&2 "Cargo output:"
        cat >&2 cargo-out.txt
        exit 1
    fi
) && (
    cd workspace
    cargo test --all >cargo-out.txt 2>&1 # Ignore expected failure
    cargo clean >/dev/null
    if ! test -f member/proptest-regressions/submodule/code.txt; then
        echo >&2 "Persistence file not written to the correct location. FS:"
        find . >&2
        echo >&2 "Cargo output:"
        cat >&2 cargo-out.txt
        exit 1
    fi
)
