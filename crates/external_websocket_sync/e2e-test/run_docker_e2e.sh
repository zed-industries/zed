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

# Build Go binaries (unless --no-build)
if [ "${1:-}" != "--no-build" ]; then
    echo "=== Building Go test server ==="
    cd "$SCRIPT_DIR/helix-ws-test-server"
    go build -o helix-ws-test-server .
    echo "Built: $SCRIPT_DIR/helix-ws-test-server/helix-ws-test-server"
    echo ""

    echo "=== Building slow MCP server ==="
    cd "$SCRIPT_DIR/slow-mcp-server"
    CGO_ENABLED=0 go build -o slow-mcp-server .
    echo "Built: $SCRIPT_DIR/slow-mcp-server/slow-mcp-server"
    echo ""
fi

# Print binary versions so it's obvious what we're testing
echo "=== Binary versions ==="
echo "  zed-binary:          $(stat -c '%y' "$SCRIPT_DIR/zed-binary" 2>/dev/null | cut -d. -f1)  $(md5sum "$SCRIPT_DIR/zed-binary" 2>/dev/null | cut -c1-12)"
echo "  helix-ws-test-server: $(stat -c '%y' "$SCRIPT_DIR/helix-ws-test-server/helix-ws-test-server" 2>/dev/null | cut -d. -f1)  $(md5sum "$SCRIPT_DIR/helix-ws-test-server/helix-ws-test-server" 2>/dev/null | cut -c1-12)"
echo "  slow-mcp-server:     $(stat -c '%y' "$SCRIPT_DIR/slow-mcp-server/slow-mcp-server" 2>/dev/null | cut -d. -f1)  $(md5sum "$SCRIPT_DIR/slow-mcp-server/slow-mcp-server" 2>/dev/null | cut -c1-12)"
echo ""

# Build Docker image
echo "=== Building Docker image ==="
cd "$SCRIPT_DIR"
docker build -t zed-ws-e2e -f Dockerfile.runtime .
echo ""

# Prepare screenshots directory
SCREENSHOTS_DIR="$SCRIPT_DIR/screenshots"
mkdir -p "$SCREENSHOTS_DIR"

# Run E2E test
E2E_AGENTS="${E2E_AGENTS:-zed-agent}"
echo "=== Running E2E test (agents: $E2E_AGENTS) ==="

# Mount local claude-agent-acp if available (for testing local changes)
CLAUDE_ACP_MOUNT=""
CLAUDE_ACP_DIR="$ZED_DIR/../claude-agent-acp"
if [ -d "$CLAUDE_ACP_DIR/dist" ] && echo "$E2E_AGENTS" | grep -q "claude"; then
    CLAUDE_ACP_MOUNT="-v $(cd "$CLAUDE_ACP_DIR" && pwd):/opt/claude-agent-acp"
    echo "[setup] Mounting local claude-agent-acp from $CLAUDE_ACP_DIR"
fi

EXTRA_DOCKER_ARGS=""
if [ -n "${ANTHROPIC_BASE_URL:-}" ]; then
    # Resolve the API hostname on the host and inject it into the container via --add-host.
    # This ensures the container uses the host-side reverse proxy (with auth middleware),
    # not the internal container IP which bypasses auth.
    API_HOST=$(echo "${ANTHROPIC_BASE_URL}" | sed 's|https\?://||; s|:.*||')

    if [ "$API_HOST" = "host.docker.internal" ]; then
        # On Linux, host.docker.internal is not automatically resolvable inside containers.
        # Use the Docker bridge gateway IP (the host's IP on the default bridge network).
        BRIDGE_GW=$(docker network inspect bridge --format '{{range .IPAM.Config}}{{.Gateway}}{{end}}' 2>/dev/null)
        if [ -n "$BRIDGE_GW" ]; then
            EXTRA_DOCKER_ARGS="--add-host host.docker.internal:${BRIDGE_GW}"
            echo "[setup] Mapped host.docker.internal → ${BRIDGE_GW} (Docker bridge gateway)"
        fi
    else
        API_HOST_IP=$(getent hosts "$API_HOST" 2>/dev/null | awk '{print $1; exit}')
        if [ -n "$API_HOST_IP" ]; then
            EXTRA_DOCKER_ARGS="--add-host ${API_HOST}:${API_HOST_IP}"
            echo "[setup] Mapped ${API_HOST} → ${API_HOST_IP}"
        fi
    fi
fi

docker run --rm \
    -e ANTHROPIC_API_KEY="$ANTHROPIC_API_KEY" \
    ${ANTHROPIC_BASE_URL:+-e ANTHROPIC_BASE_URL="$ANTHROPIC_BASE_URL"} \
    -e E2E_AGENTS="$E2E_AGENTS" \
    -v "$SCREENSHOTS_DIR:/test/screenshots" \
    $CLAUDE_ACP_MOUNT \
    $EXTRA_DOCKER_ARGS \
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
