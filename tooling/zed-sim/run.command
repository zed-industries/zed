#!/bin/sh
# Zed Sim — build the staff Zed (with state injection) if needed, then start
# the control panel. Works from any checkout; double-click in Finder or run:
#   ./tooling/zed-sim/run.command
#
# The first run does a full Zed build (can take 10-30 min). After that it's fast.
set -e
cd "$(dirname "$0")/../.."
echo "Building the staff Zed (this is slow the first time)…"
cargo build -p zed --features staff-sim
exec cargo run -p zed-sim
