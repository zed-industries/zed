# Integration Tests

Some test cases depend on a LiveKit server and thus are not enabled by default.

## Basic E2E Tests

To run basic E2E tests, start a local LiveKit server in development mode, and enable the E2E test feature:

```sh
livekit-server --dev
cargo test --features default,__lk-e2e-test -- --nocapture
```

## Peer Connection Signaling Tests

The `peer_connection_signaling_test.rs` tests verify both V0 (dual peer connection) and V1 (single peer connection) signaling modes.

### V0 Tests (Dual Peer Connection)

V0 tests work on localhost with the default LiveKit development server:

```sh
# Start local server
livekit-server --dev

# Run V0 tests
cargo test -p livekit --features "__lk-e2e-test,native-tls" --test peer_connection_signaling_test v0_ -- --nocapture
```

Default localhost configuration:
- URL: `ws://localhost:7880`
- API Key: `devkey`
- API Secret: `secret`

### V1 Tests (Single Peer Connection)

**Important:** V1 (single peer connection) mode requires a LiveKit Cloud server or a server that supports the `/rtc/v1` endpoint.

⚠️ **V1 tests will fall back to V0 signaling on localhost**, so to truly test V1 functionality, you **must** set the cloud environment variables:

```sh
export LIVEKIT_URL="wss://your-project.livekit.cloud"
export LIVEKIT_API_KEY="your-api-key"
export LIVEKIT_API_SECRET="your-api-secret"

# Run V1 tests
cargo test -p livekit --features "__lk-e2e-test,native-tls" --test peer_connection_signaling_test v1_ -- --nocapture
```

### Running All Tests

```sh
# On localhost (V0 will work, V1 falls back to V0)
# Note, suggest running with --test-threads=1 to avoid flakiness
livekit-server --dev
cargo test -p livekit --features "__lk-e2e-test,native-tls" --test peer_connection_signaling_test -- --nocapture --test-threads=1

# On LiveKit Cloud (both V0 and V1 work correctly)
export LIVEKIT_URL="wss://your-project.livekit.cloud"
export LIVEKIT_API_KEY="your-api-key"
export LIVEKIT_API_SECRET="your-api-secret"
cargo test -p livekit --features "__lk-e2e-test,native-tls" --test peer_connection_signaling_test -- --nocapture --test-threads=1
```

## VS Code Integration

If you are using Rust Analyzer in Visual Studio Code, you can enable the E2E test feature to get code completion for these tests. Add the following setting to *.vscode/settings.json*:

```json
"rust-analyzer.cargo.features": ["default", "__lk-e2e-test"]
```
