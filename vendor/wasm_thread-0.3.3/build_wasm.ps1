$env:RUSTFLAGS="-C target-feature=+atomics,+bulk-memory,+mutable-globals"
cargo +nightly build --example simple --target wasm32-unknown-unknown --release -Z build-std=std,panic_abort

wasm-bindgen target/wasm32-unknown-unknown/release/examples/simple.wasm --out-dir ./examples/target/ --target no-modules
