set -ex

export CARGO_INCREMENTAL=0

FEATURES="async_smol async_tokio async_std async_futures"

if [ "$CLIPPY" = "yes" ]; then
      cargo clippy --all -- -D warnings
elif [ "$DOCS" = "yes" ]; then
    cargo clean
    cargo doc --features "$FEATURES" --all --no-deps
    cd book
    mdbook build
    cd ..
    cp -r book/book/html/ target/doc/book/
    travis-cargo doc-upload || true
elif [ "$RUSTFMT" = "yes" ]; then
    cargo fmt --all -- --check
elif [ "$MINIMAL_VERSIONS" = "yes" ]; then
    rm Cargo.lock || true
    cargo build -Z minimal-versions
else
    export RUSTFLAGS="-D warnings"

    cargo build --features "$FEATURES" $BUILD_ARGS

    cargo test --features "$FEATURES" --all
    cargo test --features "$FEATURES" --benches
    
    cd bencher_compat
    export CARGO_TARGET_DIR="../target"
    cargo test --benches
    cd ..

    if [ "$TRAVIS_RUST_VERSION" = "nightly" ]; then
        cd macro
        export CARGO_TARGET_DIR="../target"
        cargo test --benches
        cd ..
    fi
fi
