#!/usr/bin/env bash

# Builds release remote_server binaries and stages them as app resources so
# the iOS app can install them on hosts over SSH.

set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
staging="$repository_root/crates/zed_ios/app/RemoteServers"

cargo build -p remote_server --release
gzip -c "$repository_root/target/release/remote_server" \
    > "$staging/zed-remote-server-macos-aarch64.gz"

cargo build -p remote_server --release --target x86_64-unknown-linux-musl
gzip -c "$repository_root/target/x86_64-unknown-linux-musl/release/remote_server" \
    > "$staging/zed-remote-server-linux-x86_64.gz"

ls -lh "$staging"
