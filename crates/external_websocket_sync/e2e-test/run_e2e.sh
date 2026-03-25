#!/usr/bin/env bash
set -euo pipefail

# E2E test for Zed WebSocket sync
# Tests the full flow: Zed connects → sends agent_ready → mock server sends
# chat_message → Zed creates thread → streams response → sends message_completed
#
# The mock server is a Go binary (helix-ws-test-server) that imports the same
# wsprotocol package used by the production Helix API server — same message
# parsing, routing, and accumulation code runs in both tests and production.
#
# Environment variables:
#   ZED_BINARY              - Path to Zed binary (default: /usr/local/bin/zed)
#   TEST_TIMEOUT            - Timeout in seconds (default: 240)
#   HELIX_WS_TEST_SERVER    - Path to Go test server binary (default: /usr/local/bin/helix-ws-test-server)

echo "============================================"
echo "  Zed WebSocket Sync E2E Test"
echo "============================================"
echo ""

ZED_BINARY="${ZED_BINARY:-/usr/local/bin/zed}"
# Default timeout scales with number of agent rounds (each round takes ~120s)
AGENT_COUNT=$(echo "${E2E_AGENTS:-zed-agent}" | tr ',' '\n' | wc -l)
DEFAULT_TIMEOUT=$((240 * AGENT_COUNT))
TEST_TIMEOUT="${TEST_TIMEOUT:-$DEFAULT_TIMEOUT}"
MOCK_SERVER="${HELIX_WS_TEST_SERVER:-/usr/local/bin/helix-ws-test-server}"
PROJECT_DIR="/test/project"
MOCK_PORT_FILE="/tmp/mock_helix_port"

# Screenshot capture settings
SCREENSHOT_DIR="${SCREENSHOT_DIR:-/test/screenshots}"
SCREENSHOT_INTERVAL="${SCREENSHOT_INTERVAL:-3}"

# Cleanup function
cleanup() {
    echo "[cleanup] Shutting down..."
    [ -n "${SCREENSHOT_PID:-}" ] && kill "$SCREENSHOT_PID" 2>/dev/null || true
    [ -n "${ZED_PID:-}" ] && kill "$ZED_PID" 2>/dev/null || true
    [ -n "${MOCK_PID:-}" ] && kill "$MOCK_PID" 2>/dev/null || true
    [ -n "${XVFB_PID:-}" ] && kill "$XVFB_PID" 2>/dev/null || true
    rm -f "$MOCK_PORT_FILE"

    # Dump Zed errors/panics (full log available at ZED_LOG_FILE)
    if [ -f "${ZED_LOG_FILE:-}" ]; then
        ZED_ERRORS=$(grep -ciE "panic|error|fatal" "$ZED_LOG_FILE" 2>/dev/null || echo "0")
        if [ "$ZED_ERRORS" -gt 0 ]; then
            echo ""
            echo "=================================================="
            echo "  ZED PROCESS ERRORS ($ZED_ERRORS lines)"
            echo "=================================================="
            grep -iE "panic|error|fatal" "$ZED_LOG_FILE" | tail -50 || true
            echo "  (full log: $ZED_LOG_FILE)"
        fi
    fi

    # Report screenshots
    if [ -d "$SCREENSHOT_DIR" ]; then
        SHOT_COUNT=$(ls -1 "$SCREENSHOT_DIR"/*.png 2>/dev/null | wc -l)
        echo "[screenshots] Captured $SHOT_COUNT screenshots in $SCREENSHOT_DIR"
    fi
}
trap cleanup EXIT

# Start D-Bus session (required by Zed for GPU init / portal notifications)
if [ -z "${DBUS_SESSION_BUS_ADDRESS:-}" ]; then
    export DBUS_SESSION_BUS_ADDRESS=$(dbus-daemon --session --fork --print-address 2>/dev/null || true)
    echo "[setup] D-Bus session: ${DBUS_SESSION_BUS_ADDRESS:-none}"
fi

# Start virtual framebuffer if no display
if ! xdpyinfo -display "${DISPLAY:-}" >/dev/null 2>&1; then
    echo "[setup] Starting Xvfb on :99..."
    Xvfb :99 -screen 0 1280x720x24 -ac +extension GLX &
    XVFB_PID=$!
    export DISPLAY=:99
    sleep 1
    if ! kill -0 "$XVFB_PID" 2>/dev/null; then
        echo "[error] Xvfb failed to start"
        exit 1
    fi
    echo "[setup] Xvfb started (PID $XVFB_PID)"
fi

# Start background screenshot capture
mkdir -p "$SCREENSHOT_DIR"
(
    SHOT_NUM=0
    while true; do
        sleep "$SCREENSHOT_INTERVAL"
        SHOT_NUM=$((SHOT_NUM + 1))
        FILENAME=$(printf "%s/screenshot-%04d.png" "$SCREENSHOT_DIR" "$SHOT_NUM")
        import -window root "$FILENAME" 2>/dev/null || true
    done
) &
SCREENSHOT_PID=$!
echo "[screenshots] Background capture started (every ${SCREENSHOT_INTERVAL}s → $SCREENSHOT_DIR)"

# Verify binaries exist
if [ ! -f "$ZED_BINARY" ]; then
    echo "[error] Zed binary not found at $ZED_BINARY"
    exit 1
fi

if [ ! -f "$MOCK_SERVER" ]; then
    echo "[error] Go test server binary not found at $MOCK_SERVER"
    echo "[error] Build it with: cd helix-ws-test-server && CGO_ENABLED=0 go build -o $MOCK_SERVER ."
    exit 1
fi

echo "[setup] Zed binary: $ZED_BINARY"
echo "[setup]   md5: $(md5sum "$ZED_BINARY" 2>/dev/null | cut -c1-32)"
echo "[setup]   size: $(stat -c '%s' "$ZED_BINARY" 2>/dev/null) bytes"
echo "[setup]   mtime: $(stat -c '%y' "$ZED_BINARY" 2>/dev/null | cut -d. -f1)"
echo "[setup]   ThreadLoadError in source (expected 1): $(strings "$ZED_BINARY" | grep -c 'ThreadLoadError' 2>/dev/null || echo unknown)"
echo "[setup]   'creating new thread as fallback' present: $(strings "$ZED_BINARY" | grep -c 'creating new thread as fallback' 2>/dev/null || echo unknown)"
echo "[setup] Mock server: $MOCK_SERVER"
echo "[setup]   md5: $(md5sum "$MOCK_SERVER" 2>/dev/null | cut -c1-32)"

echo "[setup] Zed binary: $ZED_BINARY"
echo "[setup] Mock server: $MOCK_SERVER"
echo "[setup] Timeout: ${TEST_TIMEOUT}s"
echo ""

# ---- Start Go WebSocket Test Server ----
echo "[mock-server] Starting Go test server (shares wsprotocol with production Helix)..."

"$MOCK_SERVER" &
MOCK_PID=$!
sleep 2

if [ ! -f "$MOCK_PORT_FILE" ]; then
    echo "[error] Mock server failed to start"
    exit 1
fi
MOCK_PORT=$(cat "$MOCK_PORT_FILE")
echo "[mock-server] Running on port $MOCK_PORT"
echo ""

# ---- Configure Zed via environment variables ----
# ExternalSyncSettings reads from env vars, not settings.json
export ZED_EXTERNAL_SYNC_ENABLED=true
export ZED_WEBSOCKET_SYNC_ENABLED=true
export ZED_HELIX_URL="127.0.0.1:${MOCK_PORT}"
export ZED_HELIX_TOKEN="test-token"
export ZED_HELIX_TLS=false
export ZED_HELIX_SKIP_TLS_VERIFY=false
export HELIX_SESSION_ID="ses_e2e-test-session-001"

# ---- Determine which agents to test ----
# E2E_AGENTS controls which agent rounds to run. Default: zed-agent only.
# Set E2E_AGENTS="zed-agent,claude" to also test Claude Code.
export E2E_AGENTS="${E2E_AGENTS:-zed-agent}"
echo "[setup] E2E_AGENTS=$E2E_AGENTS"

# ---- Write Zed settings.json for LLM provider ----
ZED_CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/zed"
mkdir -p "$ZED_CONFIG_DIR"

# Build the agent_servers config for Claude Code if it's in E2E_AGENTS
AGENT_SERVERS_JSON=""
if echo "$E2E_AGENTS" | grep -q "claude"; then
    # Claude Code needs ANTHROPIC_API_KEY passed through settings (Zed clears env vars)
    CLAUDE_KEY="${ANTHROPIC_API_KEY:-}"
    if [ -z "$CLAUDE_KEY" ]; then
        echo "[error] ANTHROPIC_API_KEY is required when testing claude agent"
        exit 1
    fi
    # Use local claude-agent-acp build if mounted, otherwise let Zed auto-install from npm
    CLAUDE_PATH_JSON=""
    if [ -f "/opt/claude-agent-acp/dist/index.js" ]; then
        CLAUDE_PATH_JSON="\"path\": \"node\", \"args\": [\"/opt/claude-agent-acp/dist/index.js\"],"
        LOCAL_VERSION=$(node -e "console.log(require('/opt/claude-agent-acp/package.json').version)" 2>/dev/null || echo "unknown")
        echo "[setup] Using LOCAL claude-agent-acp v$LOCAL_VERSION from /opt/claude-agent-acp"
    else
        # Log which version npx will install so we can correlate failures
        # with claude-agent-acp upgrades. This is a quick check, not an install.
        CLAUDE_ACP_VERSION=$(npm view @anthropic-ai/claude-agent-acp version 2>/dev/null || echo "unknown")
        echo "[setup] Using npm-installed claude-agent-acp (auto-install, latest=$CLAUDE_ACP_VERSION)"
    fi
    AGENT_SERVERS_JSON=$(cat << AGENTEOF
  "agent_servers": {
    "claude": {
      ${CLAUDE_PATH_JSON}
      "env": {
        "ANTHROPIC_API_KEY": "${CLAUDE_KEY}"
      }
    }
  },
AGENTEOF
)
    echo "[setup] Claude Code agent configured with API key"
fi

cat > "$ZED_CONFIG_DIR/settings.json" << JSONEOF
{
${AGENT_SERVERS_JSON}
  "language_models": {
    "anthropic": {
      "api_url": "${ANTHROPIC_BASE_URL:-https://api.anthropic.com}"
    }
  },
  "agent": {
    "default_model": {
      "provider": "anthropic",
      "model": "claude-sonnet-4-5-latest"
    },
    "always_allow_tool_actions": true,
    "show_onboarding": false,
    "auto_open_panel": true
  },
  "context_servers": {
    "slow-mcp-test": {
      "enabled": true,
      "command": "/usr/local/bin/slow-mcp-server",
      "args": []
    }
  }
}
JSONEOF
echo "[zed] Wrote settings to $ZED_CONFIG_DIR/settings.json"

echo "[zed] Starting Zed with WebSocket sync..."
echo "[zed]   ZED_HELIX_URL=$ZED_HELIX_URL"
echo "[zed]   ZED_EXTERNAL_SYNC_ENABLED=$ZED_EXTERNAL_SYNC_ENABLED"
echo "[zed]   ZED_STATELESS=${ZED_STATELESS:-not set}"
echo "[zed]   ANTHROPIC_API_KEY=${ANTHROPIC_API_KEY:+set (${#ANTHROPIC_API_KEY} chars)}"
echo "[zed]   E2E_AGENTS=$E2E_AGENTS"
echo ""

# Start Zed (capture logs for debugging)
ZED_LOG_FILE="/tmp/zed-e2e.log"
"$ZED_BINARY" \
    --allow-multiple-instances \
    "$PROJECT_DIR" \
    > "$ZED_LOG_FILE" 2>&1 &
ZED_PID=$!
echo "[zed] Logs: $ZED_LOG_FILE"

echo "[zed] Started (PID $ZED_PID)"
echo ""
echo "[test] Waiting for protocol flow to complete (timeout: ${TEST_TIMEOUT}s)..."

# ---- Wait for test to complete ----
ELAPSED=0
while [ "$ELAPSED" -lt "$TEST_TIMEOUT" ]; do
    # Check if Zed crashed
    if ! kill -0 "$ZED_PID" 2>/dev/null; then
        wait "$ZED_PID" || ZED_EXIT=$?
        echo "[error] Zed exited early with code ${ZED_EXIT:-0}"
        # Still check if mock server got enough events
        if ! kill -0 "$MOCK_PID" 2>/dev/null; then
            wait "$MOCK_PID"
            MOCK_EXIT=$?
            if [ "$MOCK_EXIT" -eq 0 ]; then
                echo ""
                echo "============================================"
                echo "  E2E TEST PASSED (Zed exited but protocol completed)"
                echo "============================================"
                exit 0
            fi
        fi
        echo ""
        echo "============================================"
        echo "  E2E TEST FAILED (Zed crashed)"
        echo "============================================"
        exit 1
    fi

    # Check if mock server completed successfully
    if ! kill -0 "$MOCK_PID" 2>/dev/null; then
        wait "$MOCK_PID"
        MOCK_EXIT=$?
        if [ "$MOCK_EXIT" -eq 0 ]; then
            echo ""
            echo "============================================"
            echo "  E2E TEST PASSED"
            echo "============================================"
            exit 0
        else
            echo ""
            echo "============================================"
            echo "  E2E TEST FAILED (mock server exit: $MOCK_EXIT)"
            echo "============================================"
            exit 1
        fi
    fi

    sleep 2
    ELAPSED=$((ELAPSED + 2))
done

echo ""
echo "============================================"
echo "  E2E TEST TIMED OUT after ${TEST_TIMEOUT}s"
echo "============================================"
exit 1
