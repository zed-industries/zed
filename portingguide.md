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

### E2E Test (`e2e-test/run_docker_e2e.sh`)

10-phase test that validates the full protocol. Run via:

```bash
cd crates/external_websocket_sync/e2e-test
ANTHROPIC_API_KEY=<key> ./run_docker_e2e.sh
```

Each phase queries UI state via `query_ui_state` to verify the agent panel displays the correct thread.

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

### `crates/agent_ui/src/conversation_view.rs`
> **Note:** Upstream renamed `crates/agent_ui/src/acp/thread_view.rs` → `crates/agent_ui/src/conversation_view.rs` as part of ACP consolidation (see ACP Consolidation section below).

- **`HeadlessConnection`**: No-op `AgentConnection` impl for WebSocket-created threads (cfg-gated)
- **`UserCreatedThread` event**: Sends when user creates a thread via UI (not via WebSocket)
- **`ThreadTitleChanged` event**: Forwards title changes to Helix
- **`from_existing_thread()` constructor**: Creates a `ConversationView` wrapping an existing `Entity<AcpThread>` with a `HeadlessConnection`. Uses `ConnectedServerState` with `active_id`, `threads` HashMap, `conversation` Entity, `history` (`Option<Entity<ThreadHistory>>`), and `_connection_entry_subscription`. Requires `connection_store` and `connection_key` parameters. Used when thread_service loads a thread and needs to display it
- **Thread registry integration**: Registers threads from `from_existing_thread` into `THREAD_REGISTRY`

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

**File:** `crates/agent_ui/src/conversation_view.rs` (was `crates/agent_ui/src/acp/thread_view.rs`)

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

## ACP Consolidation (Upstream Breaking Change)

Upstream Zed completed the ACP consolidation — making all agent functionality use the ACP protocol and retiring the legacy non-ACP native agent. This caused the following renames that affect Helix-specific code:

| Old name | New name |
|----------|----------|
| `crates/agent_ui/src/acp/thread_view.rs` | `crates/agent_ui/src/conversation_view.rs` |
| `AcpThreadHistory` | `ThreadHistory` (in `crates/agent_ui/src/thread_history.rs`) |
| `ExternalAgent` enum | `Agent` enum (in `crates/agent_ui/src/agent_ui.rs`) |
| `AcpServerView` struct | `ConversationView` struct |
| `crate::acp::AcpServerView::from_existing_thread` | `ConversationView::from_existing_thread` |

**Key impact on `from_existing_thread`:**
- Now takes `connection_store: Entity<AgentConnectionStore>` and `connection_key: Agent` as parameters
- `history` parameter type changed from `Entity<AcpThreadHistory>` to `Option<Entity<ThreadHistory>>`
- `EntryViewState::new` parameter list changed: removed `prompt_capabilities`/`available_commands`, replaced with `session_capabilities: SharedSessionCapabilities` and `agent_id: AgentId`
- `ThreadView::new` (was `AcpThreadView::new`) parameter list changed: removed `login` and `resume_thread_metadata`, added `agent_icon_from_external_svg`
- `ConnectedServerState` now has `history: Option<Entity<ThreadHistory>>` and `_connection_entry_subscription: Subscription` fields (use `Subscription::new(|| {})` for headless case)

**`set_session_list` cfg fix:**
`ThreadHistory::set_session_list()` in `thread_history.rs` is `#[cfg(any(test, feature = "test-support", feature = "external_websocket_sync"))]` — the `external_websocket_sync` feature was added to allow the WebSocket sync code to call it.

## Branch Naming

The internal git server only accepts pushes to branches matching the `feature/<task-id>-*` pattern. Always name the merge branch after the task ID, e.g. `feature/001617-merge-latest-zed`. Do not use date-based names like `merge-upstream-YYYY-MM-DD`.

## `.github/workflows/` — Always Revert Upstream Changes

The internal git server rejects pushes that modify `.github/workflows/` because it requires a GitHub token scope we don't have. **After every upstream merge, you must restore all workflow files to their pre-merge state.**

Steps:

```bash
# Find the last Helix commit before the merge (the merge's first parent)
PRE_MERGE=$(git log --merges --format="%P" -1 | awk '{print $1}')
# Restore all workflow files to that state
git checkout $PRE_MERGE -- .github/workflows/
# Delete any new workflow files upstream added
git ls-files --deleted -- .github/workflows/  # nothing to delete here (checkout restores)
# But upstream may have ADDED new files that didn't exist pre-merge:
git diff $PRE_MERGE HEAD --name-status -- .github/workflows/ | grep '^A' | awk '{print $2}' | xargs -r git rm
git commit -m "Revert .github/workflows to pre-merge state"
```

> **Never let upstream workflow changes through to the push.** If you see `.github/workflows/` in `git diff HEAD~N HEAD`, those changes must be reverted before pushing.

## Rebase Checklist

When rebasing/merging against upstream Zed:

0. **Revert `.github/workflows/`** — restore all files to pre-merge state (see section above); push will be rejected otherwise
1. **Preserve the `external_websocket_sync` crate** — it's self-contained and rarely conflicts
2. **Check `agent.rs` `load_session()`** — ensure the entity lifetime fix is present (Critical Fix #1)
3. **Check `conversation_view.rs` event handlers** — ensure no duplicate WebSocket sends (Critical Fix #2); file was `acp/thread_view.rs` before ACP consolidation
4. **Check `acp_thread.rs` `AssistantMessage`** — ensure `content_only()` exists (Critical Fix #3)
5. **Check `thread_service.rs` follow-up path** — ensure `notify_thread_display()` is called (Critical Fix #4)
6. **Check `agent_panel.rs` cfg-gated blocks** — callback setup, `from_existing_thread`, onboarding dismissal, `acp_history_store()`
7. **Check `conversation_view.rs` cfg-gated blocks** — `HeadlessConnection`, `UserCreatedThread`, `ThreadTitleChanged`, `from_existing_thread()`, THREAD_REGISTRY registration
8. **Check `from_existing_thread()` matches `ConnectedServerState` struct** — upstream may change fields (currently: `active_id`, `threads` HashMap, `conversation` Entity, `history: Option<Entity<ThreadHistory>>`, `_connection_entry_subscription`)
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
21. **Run E2E test** after merge to verify all 10 phases pass

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
| `adc592c` | Fix: remove final summary message_added causing duplicate responses |
| `3193353` | Fix: make thread event subscriptions persistent for Zed→Helix sync |
| `486a9d6` | Perf: throttle streaming EntryUpdated events (100ms) |
| `e779937` | Feat: Go-based E2E test server for WebSocket sync protocol |
| `818cf94` | Fix: adapt external_websocket_sync to upstream connect() API change |
| `276da28` | Fix: wire up auto_open_panel setting to AgentPanel starts_open() |
| `2f74e89` | Fix: disable migration banner in Helix builds |
| `01fbdfc` | Fix: prevent keyboard focus stealing when following agent |
| `87632d0` | Fix: thread entity brain split after container restart |
| `3e4d7d7` | Fix: wait for MCP tools before sending first WebSocket message |
| `d511c3e` | Fix: auto-follow mode and split-brain for external WebSocket sessions |
| `182cae0` | Fix: missing message_completed in follow-up subscription |
| `29f10aa` | Emit MessageCompleted from Stopped event for all turn sources |
| `91c281f` | Extract ensure_thread_subscription to fix missing event handlers |
| `c33ee0a` | Add entry_type field to MessageAdded sync event |
| `4053ccd` | Flush stale pending entries when different entry starts streaming |
| `4e204c4` | Handle ToolCall in NewEntry event (not just EntryUpdated) |
| `bfe84a2` | Send structured tool_name and tool_status in message_added events |
| `e38aad1` | Clear persistent subscription on unregister to fix E2E timeout |
| `8b033a4` | Test: add Stopped emission and mid-stream interrupt E2E tests |
| `d9fb0a0` | Fix: make thread event subscriptions persistent |
| `8881750` | Add diagnostic logs for auto-follow workspace.follow() calls |
| `95ce56d` | Fix: auto-follow mode split-brain for external WebSocket sessions |
| `fd97448` | Fix: thread hang from dropped GPUI Task in rapid cancel chain |
| `2fbc0966` | Fix: send user_created_thread for empty threads too |
| `1d5a75a` | Test: add Phase 10 E2E test for user_created_thread multi-thread sync |
| `846c74d` | Fix: remove hardcoded phase count from E2E test output |
| `28f9917` | Portingguide: document branch naming convention for upstream merges |
| `1f96cb8` | **Merge upstream/main into feature/001617-merge-latest-zed** |
| `57665e6` | **Fix Helix cfg-gated code for ACP consolidation renames** |
