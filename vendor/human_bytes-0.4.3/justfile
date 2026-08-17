all: test lint build-binary

test:
	cargo test
	cargo test --features fast
	cargo test --no-default-features
	cargo test --no-default-features --features fast

lint:
	cargo clippy
	cargo clippy --features fast
	cargo clippy --bin hb --features "build-binary"

build-binary:
	cargo build --release --features 'build-binary fast' --bin hb
