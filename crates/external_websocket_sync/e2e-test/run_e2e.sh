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
TEST_TIMEOUT="${TEST_TIMEOUT:-240}"
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
export HELIX_SESSION_ID="e2e-test-session-001"

# ---- Write Zed settings.json for LLM provider ----
ZED_CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/zed"
mkdir -p "$ZED_CONFIG_DIR"
cat > "$ZED_CONFIG_DIR/settings.json" << JSONEOF
{
  "language_models": {
    "anthropic": {
      "api_url": "https://api.anthropic.com"
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
  }
}
JSONEOF
echo "[zed] Wrote settings to $ZED_CONFIG_DIR/settings.json"

echo "[zed] Starting Zed with WebSocket sync..."
echo "[zed]   ZED_HELIX_URL=$ZED_HELIX_URL"
echo "[zed]   ZED_EXTERNAL_SYNC_ENABLED=$ZED_EXTERNAL_SYNC_ENABLED"
echo "[zed]   ZED_STATELESS=${ZED_STATELESS:-not set}"
echo "[zed]   ANTHROPIC_API_KEY=${ANTHROPIC_API_KEY:+set (${#ANTHROPIC_API_KEY} chars)}"
echo ""

# Start Zed
"$ZED_BINARY" \
    --allow-multiple-instances \
    "$PROJECT_DIR" \
    &
ZED_PID=$!

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
