# Helix Fork Porting Guide

This document describes all Helix-specific changes to the Zed codebase and the critical fixes needed when rebasing or updating the fork against upstream Zed. It serves as a checklist to ensure nothing is lost during future rebases.

## Overview

The Helix fork adds a WebSocket-based bidirectional sync layer between Zed and the Helix API server. This enables Helix to send chat messages to Zed's agent panel and receive streaming responses, thread lifecycle events, and UI state queries — all without modifying Zed's core agent/thread architecture.

**Design principle:** All Helix changes are behind `#[cfg(feature = "external_websocket_sync")]` feature gates where possible, minimizing merge conflicts with upstream.

## Architecture

```
Helix API Server
    ↕ WebSocket (bidirectional)
Zed (external_websocket_sync crate)
    ↕ GPUI entities + callbacks
Zed Agent Panel (agent_ui crate)
    ↕ AgentConnection trait
NativeAgent / ACP Agent (agent crate)
    ↕ LLM API
Claude / Qwen / etc.
```

## Helix-Specific Crates

### `crates/external_websocket_sync/`

The entire crate is Helix-specific. It provides:

| File | Purpose |
|------|---------|
| `external_websocket_sync.rs` | Crate root: global callback channels, init functions, public API |
| `websocket_sync.rs` | WebSocket client: connect, reconnect, send/receive messages |
| `thread_service.rs` | Thread lifecycle: create, follow-up, load, open threads via GPUI |
| `types.rs` | `SyncEvent` enum for all WebSocket event types |
| `sync_settings/` | Settings module: `ZED_HELIX_URL`, TLS config, etc. |
| `mock_helix_server.rs` | In-process mock server for unit tests |
| `protocol_test.rs` | Protocol-level integration tests |
| `server.rs` | WebSocket server utilities |
| `mcp.rs` | MCP integration helpers |
| `e2e-test/` | Docker-based E2E test with real LLM calls |

### E2E Test (`e2e-test/run_e2e.sh`)

Four-phase test that validates the full protocol:

1. **Phase 1**: New thread creation via `chat_message`
2. **Phase 2**: Follow-up message to existing thread
3. **Phase 3**: New thread creation (second thread)
4. **Phase 4**: Follow-up to non-visible thread (Thread A while Thread B is displayed)

Each phase also queries UI state via `query_ui_state` to verify the agent panel displays the correct thread.

```bash
# Run E2E test
cd ~/pm/zed
docker build -t zed-ws-e2e -f crates/external_websocket_sync/e2e-test/Dockerfile .
docker run --rm -e ANTHROPIC_API_KEY=sk-ant-... -e TEST_TIMEOUT=120 zed-ws-e2e
```

## Modified Upstream Files

These files contain Helix-specific changes that must be preserved during rebases:

### `Cargo.toml` (workspace root)
- Added `crates/external_websocket_sync` to workspace members
- Added `external_websocket_sync` workspace dependency

### `crates/zed/Cargo.toml`
- Added `external_websocket_sync` feature flag
- Added `external_websocket_sync` optional dependency

### `crates/zed/src/zed.rs`
- Initialization of WebSocket sync service on startup (cfg-gated)

### `crates/agent_ui/Cargo.toml`
- Added `external_websocket_sync` feature flag
- Added `external_websocket_sync_dep` optional dependency

### `crates/agent_ui/src/agent_panel.rs`
- **Thread display callback**: Receives `ThreadDisplayNotification` from thread_service, calls `from_existing_thread()` to display threads in the panel
- **UI state query callback**: Responds to `query_ui_state` with current active_view, thread_id, entry_count
- **Thread creation callback**: Wires up thread_service to create threads
- **Thread open callback**: Wires up thread_service to open existing threads
- **Onboarding dismissal**: Auto-dismisses `OnboardingUpsell` when WebSocket sync is active
- **`acp_history_store()`**: Accessor for `ThreadStore` entity, used by WebSocket integration setup (cfg-gated)
- **`NativeAgentSessionList` import**: Required for thread persistence in `ThreadDisplayNotification` handler

### `crates/agent_ui/src/acp/thread_view.rs`
- **`HeadlessConnection`**: No-op `AgentConnection` impl for WebSocket-created threads (cfg-gated)
- **`UserCreatedThread` event**: Sends when user creates a thread via UI (not via WebSocket)
- **`ThreadTitleChanged` event**: Forwards title changes to Helix
- **`from_existing_thread()` constructor**: Creates an `AcpServerView` wrapping an existing `Entity<AcpThread>` with a `HeadlessConnection`. Uses `ConnectedServerState` with `active_id`, `threads` HashMap, and `Conversation` entity. Used when thread_service loads a thread and needs to display it
- **Thread registry integration**: Registers threads from both `from_existing_thread` and `initial_state` into `THREAD_REGISTRY`
- **History refresh**: Calls `history.refresh()` on `Stopped` and `TitleUpdated` events

### `crates/extensions_ui/src/extensions_ui.rs`
- **Agent keyword removal**: Claude/Codex/Gemini keywords removed from search (enterprise — users should use corporate LLMs)
- **Agent upsell removal**: Claude/Codex/Gemini upsell banners removed from extensions UI

### `crates/recent_projects/src/dev_container_suggest.rs`
- **`suggest_dev_container` check**: Early return if `RemoteSettings::suggest_dev_container` is false

### `crates/feature_flags/src/flags.rs`
- **ACP beta feature flag override**: `AcpBetaFeatureFlag::enabled_for_all()` returns `true` to enable session list/load/resume in release builds

### `crates/acp_thread/src/acp_thread.rs`
- **`content_only()` method on `AssistantMessage`**: Returns content without the `## Assistant\n\n` heading. Used by thread_service.rs for WebSocket sync to avoid sending the heading to Helix.

### `crates/agent/src/agent.rs`
- **`load_session()` entity lifetime fix**: Clones `Entity<NativeAgent>` to keep it alive during async `open_thread` task (see Critical Fixes below)

### `crates/title_bar/`
- **Helix connection status indicator**: Shows WebSocket connection status in the title bar
- Depends on `external_websocket_sync` crate

### `crates/http_client_tls/src/http_client_tls.rs`
- **`NoCertVerifier`**: Skips TLS certificate verification when `ZED_HTTP_INSECURE_TLS=1`
- For enterprise deployments with internal CAs / self-signed certs

### `crates/reqwest_client/src/reqwest_client.rs`
- **Insecure TLS support**: Reads `ZED_HTTP_INSECURE_TLS=1` to disable cert verification

### `crates/agent_settings/src/agent_settings.rs`
- **`show_onboarding`**: Setting to control onboarding visibility
- **`auto_open_panel`**: Setting to control agent panel auto-open

### `.dockerignore`
- Simplified for Helix build context

## Critical Fixes (Must Be Preserved)

These fixes address subtle bugs that are easy to lose during rebases because they're small changes to upstream code. Each has been verified with E2E tests.

### 1. Keep NativeAgent Entity Alive During `load_session`

**File:** `crates/agent/src/agent.rs` — `NativeAgentConnection::load_session()`

**Bug:** When `load_session()` is called (e.g., after Zed restart to reload a thread), the `Rc<NativeAgentConnection>` is consumed. Inside `open_thread()`, the async task captures `this` as a `WeakEntity<NativeAgent>`. Once the `Rc` is dropped, the `WeakEntity` can't upgrade → "entity released" error.

**Fix:** Clone `Entity<NativeAgent>` before spawning the async task, keep it alive until the task completes:

```rust
fn load_session(self: Rc<Self>, session: AgentSessionInfo, ..., cx: &mut App)
    -> Task<Result<Entity<acp_thread::AcpThread>>>
{
    let agent = self.0.clone();  // Keep strong reference
    let task = self.0.update(cx, |a, cx| a.open_thread(session.session_id, cx));
    cx.spawn(async move |_cx| {
        let result = task.await;
        drop(agent);  // Release after task completes
        result
    })
}
```

**History:** Originally fixed in old fork commit `bc721cd`, lost during rebase, re-applied as `0a78bf8`.

**Symptom:** "Thread load failed: Failed to load thread: entity released" after Zed restart.

### 2. No Duplicate WebSocket Event Sends

**File:** `crates/agent_ui/src/acp/thread_view.rs`

**Bug:** Both `thread_service.rs` AND `thread_view.rs` subscribe to thread events (`NewEntry`, `EntryUpdated`, `Stopped`) and send `MessageAdded`/`MessageCompleted` WebSocket events, causing duplicate messages in the Helix chat.

**Fix:** `thread_service.rs` is the canonical source for WebSocket events. `thread_view.rs` must NOT send `MessageAdded`, `MessageCompleted`, or streaming `EntryUpdated` events. It should only send UI-specific events:
- `UserCreatedThread` (user created thread via UI)
- `ThreadTitleChanged` (title updated)

**History:** Commit `cc037db` moved event sending to thread_service.rs, but thread_view.rs events were not removed during the port. Fixed in `72e2952`.

**Symptom:** Every assistant message appears twice in the Helix Sessions chat.

### 3. Strip "## Assistant" Heading from Synced Messages

**File:** `crates/acp_thread/src/acp_thread.rs`, `crates/external_websocket_sync/src/thread_service.rs`

**Bug:** `AssistantMessage::to_markdown()` wraps content with `## Assistant\n\n...\n\n`. When synced to Helix, every response starts with a "## Assistant" heading.

**Fix:** Added `content_only()` method that returns just the chunks without the heading. All `msg.to_markdown(cx)` calls in `thread_service.rs` (for `AssistantMessage`) use `msg.content_only(cx)` instead.

**History:** Old fork had this fix, lost during rebase. Re-applied as `98ec442`.

**Symptom:** Every assistant response in Helix starts with "## Assistant" heading.

### 4. Follow-up to Non-Visible Thread Must Notify UI

**File:** `crates/external_websocket_sync/src/thread_service.rs`

**Bug:** When a `chat_message` targets a thread that exists in `THREAD_REGISTRY` but is not currently displayed (e.g., Thread A while Thread B is visible), the message is sent but the UI doesn't switch to show the response.

**Fix:** Before sending a follow-up message, call `notify_thread_display()` to tell the agent panel to switch to the target thread.

**History:** Added in `fb96f34`. Tested by E2E Phase 4.

**Symptom:** Follow-up message sent to hidden thread, but UI stays on the wrong thread.

## Environment Variables

| Variable | Purpose | Default |
|----------|---------|---------|
| `ZED_EXTERNAL_SYNC_ENABLED` | Enable WebSocket sync | `false` |
| `ZED_HELIX_URL` | Helix API server URL (host:port) | none |
| `ZED_HELIX_TOKEN` | Auth token for WebSocket | none |
| `ZED_HELIX_TLS` | Use TLS for WebSocket | `true` |
| `ZED_HELIX_SKIP_TLS_VERIFY` | Skip TLS cert verification | `false` |
| `ZED_HTTP_INSECURE_TLS` | Skip TLS for all HTTP (enterprise) | `0` |
| `ZED_WORK_DIR` | Working directory for sessions | auto-detected |
| `ZED_STATELESS` | Don't persist thread state | not set |

## Callback Architecture

The WebSocket sync layer communicates with the agent panel via global callback channels (using `tokio::sync::mpsc`). This avoids tight coupling:

```
WebSocket message received
    → websocket_sync.rs: dispatches by command type
    → thread_service.rs: processes via MPSC channel
    → external_websocket_sync.rs: calls global callback (e.g., notify_thread_display)
    → agent_panel.rs: callback handler updates UI
```

Global callbacks initialized during agent panel setup:
- `GLOBAL_THREAD_CREATION_CALLBACK` — create new thread or follow up
- `GLOBAL_THREAD_DISPLAY_CALLBACK` — display a thread in agent panel
- `GLOBAL_THREAD_OPEN_CALLBACK` — open existing thread from agent
- `GLOBAL_UI_STATE_QUERY_CALLBACK` — query current UI state

Pending request queues (`PENDING_*`) buffer requests that arrive before callbacks are registered.

## Rebase Checklist

When rebasing/merging against upstream Zed:

1. **Preserve the `external_websocket_sync` crate** — it's self-contained and rarely conflicts
2. **Check `agent.rs` `load_session()`** — ensure the entity lifetime fix is present (Critical Fix #1)
3. **Check `thread_view.rs` event handlers** — ensure no duplicate WebSocket sends (Critical Fix #2)
4. **Check `acp_thread.rs` `AssistantMessage`** — ensure `content_only()` exists (Critical Fix #3)
5. **Check `thread_service.rs` follow-up path** — ensure `notify_thread_display()` is called (Critical Fix #4)
6. **Check `agent_panel.rs` cfg-gated blocks** — callback setup, `from_existing_thread`, onboarding dismissal, `acp_history_store()`
7. **Check `thread_view.rs` cfg-gated blocks** — `HeadlessConnection`, `UserCreatedThread`, `ThreadTitleChanged`, `from_existing_thread()`, THREAD_REGISTRY registration in `initial_state`, history refresh on Stopped/TitleUpdated
8. **Check `from_existing_thread()` matches `ConnectedServerState` struct** — upstream may change fields (currently: `active_id`, `threads` HashMap, `conversation` Entity)
9. **Check `extensions_ui.rs`** — agent keyword/upsell removal preserved
10. **Check `dev_container_suggest.rs`** — `suggest_dev_container` early return preserved
13. **Check `feature_flags/flags.rs`** — `AcpBetaFeatureFlag::enabled_for_all()` returns `true`
14. **Check `http_client_tls.rs`** — `NoCertVerifier` and `ZED_HTTP_INSECURE_TLS` support
15. **Check `reqwest_client.rs`** — insecure TLS support
16. **Check `title_bar`** — connection status indicator + `external_websocket_sync` dependency
17. **Check `agent_settings`** — `show_onboarding`, `auto_open_panel` fields
18. **Check `.dockerignore`** — simplified for Helix builds
19. **Run `cargo check --package zed --features external_websocket_sync`** — must compile
20. **Run `cargo test -p external_websocket_sync`** — unit tests
21. **Run E2E test** after merge to verify all 4 phases pass

## Building

```bash
# Build with Helix features
cargo build --features external_websocket_sync -p zed

# Run unit tests
cargo test -p external_websocket_sync

# Run E2E test (requires ANTHROPIC_API_KEY)
docker build -t zed-ws-e2e -f crates/external_websocket_sync/e2e-test/Dockerfile .
docker run --rm -e ANTHROPIC_API_KEY=sk-ant-... -e TEST_TIMEOUT=120 zed-ws-e2e
```

## Commit History

Helix-specific commits on main (oldest first):

| Commit | Description |
|--------|-------------|
| `4cae6d9` | Port Helix fork changes to fresh upstream Zed |
| `54296a7` | Add WebSocket protocol spec, mock server, and test infrastructure |
| `b063ae0` | Add E2E test infrastructure with Docker container |
| `463b1cc` | Fix E2E test infrastructure: Docker caching, headless Zed startup |
| `bc52393` | Fix model configuration race and E2E test settings |
| `5fe75be` | Fix WebSocket event forwarding for thread_service-created threads |
| `746a9c4` | Add multi-thread E2E test: follow-ups and thread transitions |
| `7da861b` | Simplify .dockerignore for helix build context |
| `6fd8116` | Update Cargo.lock for agent_settings dependency |
| `cf72593` | Restore thread auto-open and disable restricted mode |
| `e0cc99f` | Implement from_existing_thread for AcpServerView |
| `a83ddc0` | Add query_ui_state command for E2E UI verification |
| `cc037db` | Send WebSocket events from thread_service instead of UI subscription |
| `55882e8` | Fix UI freeze and thread_id mismatch in from_existing_thread |
| `01c0c11` | Streaming WebSocket events, thread persistence, dismiss onboarding |
| `3ae2f1e` | Hide built-in agents (Claude Code, Codex, Gemini) in Helix builds |
| `4e87001` | Enable ACP beta features for session list and resume |
| `fb96f34` | Add Phase 4 E2E test + fix follow-up to non-visible thread |
| `0a78bf8` | **Fix: keep NativeAgent entity alive during load_session** |
| `98ec442` | **Fix: strip '## Assistant' heading from WebSocket-synced messages** |
| `72e2952` | **Fix: remove duplicate WebSocket event sends from thread_view.rs** |
