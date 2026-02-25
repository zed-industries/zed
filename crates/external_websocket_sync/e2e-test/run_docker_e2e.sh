#!/usr/bin/env bash
set -euo pipefail

# Build and run the Zed WebSocket sync E2E test in Docker.
#
# Prerequisites:
#   - Pre-built Zed binary at ./zed-binary (from: ./stack build-zed release, then cp zed-build/zed here)
#   - ANTHROPIC_API_KEY set in environment (or in ~/.env.usercreds)
#
# Usage:
#   ./run_docker_e2e.sh              # build Go test server + Docker image + run
#   ./run_docker_e2e.sh --no-build   # skip Go build, use existing binary

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
# e2e-test → external_websocket_sync → crates → zed
ZED_DIR="$(cd "$SCRIPT_DIR/../../.." && pwd)"
# Helix repo is expected as a sibling of the zed repo (../helix relative to zed root)
HELIX_DIR="$(cd "$ZED_DIR/../helix" 2>/dev/null && pwd || echo "")"

# If ANTHROPIC_API_KEY not set, try sourcing from helix repo
if [ -z "${ANTHROPIC_API_KEY:-}" ] && [ -n "$HELIX_DIR" ]; then
    for envfile in "$HELIX_DIR/.env" "$HELIX_DIR/.env.usercreds"; do
        if [ -f "$envfile" ] && grep -q ANTHROPIC_API_KEY "$envfile"; then
            set -a
            source "$envfile"
            set +a
            break
        fi
    done
fi

if [ -z "${ANTHROPIC_API_KEY:-}" ]; then
    echo "ERROR: ANTHROPIC_API_KEY not set."
    echo "Either: export ANTHROPIC_API_KEY=sk-..."
    echo "Or:     add it to ${HELIX_DIR:-(helix repo)}/.env.usercreds"
    exit 1
fi

# Check for Zed binary
if [ ! -f "$SCRIPT_DIR/zed-binary" ]; then
    echo "ERROR: No zed-binary found at $SCRIPT_DIR/zed-binary"
    echo "Build it: cd $HELIX_DIR && ./stack build-zed release && cp zed-build/zed $SCRIPT_DIR/zed-binary"
    exit 1
fi

# Build Go test server (unless --no-build)
if [ "${1:-}" != "--no-build" ]; then
    echo "=== Building Go test server ==="
    cd "$SCRIPT_DIR/helix-ws-test-server"
    go build -o helix-ws-test-server .
    echo "Built: $SCRIPT_DIR/helix-ws-test-server/helix-ws-test-server"
    echo ""
fi

# Build Docker image
echo "=== Building Docker image ==="
cd "$SCRIPT_DIR"
docker build -t zed-ws-e2e -f Dockerfile.runtime .
echo ""

# Prepare screenshots directory
SCREENSHOTS_DIR="$SCRIPT_DIR/screenshots"
mkdir -p "$SCREENSHOTS_DIR"

# Run E2E test
echo "=== Running E2E test ==="
docker run --rm \
    -e ANTHROPIC_API_KEY="$ANTHROPIC_API_KEY" \
    -v "$SCREENSHOTS_DIR:/test/screenshots" \
    zed-ws-e2e

# Report screenshots
SHOT_COUNT=$(ls -1 "$SCREENSHOTS_DIR"/*.png 2>/dev/null | wc -l || echo 0)
if [ "$SHOT_COUNT" -gt 0 ]; then
    echo ""
    echo "=== Screenshots ==="
    echo "Captured $SHOT_COUNT screenshots in: $SCREENSHOTS_DIR"
    ls -lh "$SCREENSHOTS_DIR"/*.png | tail -5
    echo "(showing last 5)"
fi
