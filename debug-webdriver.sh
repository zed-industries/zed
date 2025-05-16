export RUST_LOG=debug
cargo test -p browser_tools_server --test init_test -- --nocapture
