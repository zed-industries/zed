#!/usr/bin/env bash

# Builds release remote_server binaries and stages them as app resources so
# the iOS app can install them on hosts over SSH.

set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
staging="$repository_root/crates/zed_ios/app/RemoteServers"

cargo build -p remote_server --release
gzip -c "$repository_root/target/release/remote_server" \
    > "$staging/zed-remote-server-macos-aarch64.gz"

# Requires the musl cross toolchain:
#   brew install messense/macos-cross-toolchains/x86_64-unknown-linux-musl
export CC_x86_64_unknown_linux_musl=x86_64-unknown-linux-musl-gcc
export CXX_x86_64_unknown_linux_musl=x86_64-unknown-linux-musl-g++
export AR_x86_64_unknown_linux_musl=x86_64-unknown-linux-musl-ar
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=x86_64-unknown-linux-musl-gcc
cargo build -p remote_server --release --target x86_64-unknown-linux-musl
stripped="$(mktemp)"
cp "$repository_root/target/x86_64-unknown-linux-musl/release/remote_server" "$stripped"
x86_64-unknown-linux-musl-strip "$stripped"
gzip -c "$stripped" > "$staging/zed-remote-server-linux-x86_64.gz"
rm "$stripped"

ls -lh "$staging"
