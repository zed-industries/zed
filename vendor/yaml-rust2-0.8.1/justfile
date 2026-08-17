before_commit:
  cargo clippy --release --all-targets -- -D warnings
  cargo clippy --all-targets -- -D warnings
  cargo build --release --all-targets
  cargo build --all-targets
  cargo test
  cargo test --release
  cargo test --doc
  cargo build --profile=release-lto --package gen_large_yaml --bin gen_large_yaml --manifest-path tools/gen_large_yaml/Cargo.toml
  RUSTDOCFLAGS="-D warnings" cargo doc --all-features

ethi_bench:
  cargo build --release --all-targets
  cd ../Yaml-rust && cargo build --release --all-targets
  cd ../serde-yaml/ && cargo build --release --all-targets
  cd ../libfyaml/build && ninja
  cargo bench_compare run_bench
