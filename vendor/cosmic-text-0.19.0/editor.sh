# SPDX-License-Identifier: MIT OR Apache-2.0

RUST_LOG="cosmic_text=debug,editor=debug" cargo run --release --package editor -- "$@"
