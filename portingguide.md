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

### E2E Test (`e2e-test/`)

Seven-phase test that validates the full protocol. Runs in Docker against a real LLM (Anthropic API). Two Dockerfiles:
- `Dockerfile.runtime` — for local dev runs (`run_docker_e2e.sh`)
- `Dockerfile.ci` — for CI (takes pre-built Zed binary + Helix Go source as build context)

Phases:
1. **Phase 1**: New thread creation via `chat_message`
2. **Phase 2**: Follow-up message to existing thread
3. **Phase 3**: New thread creation (second thread)
4. **Phase 4**: Follow-up to non-visible thread (Thread A while Thread B is displayed)
5. **Phase 5**: `message_completed` emitted after `Stopped` for all turn sources
6. **Phase 6**: Mid-stream interrupt (second `send()` displaces active turn, both emit `Stopped`)
7. **Phase 7**: MCP tool call events appear with correct `entry_type`/`tool_name`/`tool_status`

Each phase also queries UI state via `query_ui_state` to verify the agent panel state.

A `slow-mcp-server` test helper (in `e2e-test/slow-mcp-server/`) simulates an MCP server with delayed tool responses, used by phases 6 and 7 to test the `wait_for_tools_ready` path.

```bash
# Run E2E test (local)
cd crates/external_websocket_sync/e2e-test
cp ../../zed-build/zed zed-binary
./run_docker_e2e.sh  # builds Go test server + Docker image + runs test
# Screenshots saved to ./screenshots/
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
- **Thread display callback**: Receives `ThreadDisplayNotification` from thread_service, calls `from_existing_thread()` to display threads in the panel. Passes `this.connection_store.clone()` and `crate::Agent::NativeAgent` to the constructor (required since the 2026-03-22 upstream merge added these fields to `ConversationView`).
- **UI state query callback**: Responds to `query_ui_state` with current active_view, thread_id, entry_count, `mcp_servers` map, and `active_model` string. Matches `ActiveView::AgentThread { conversation_view }` (not `server_view` — field was renamed in upstream 2026-03-22 merge).
- **Thread creation callback**: Wires up thread_service to create threads
- **Thread open callback**: Wires up thread_service to open existing threads
- **Onboarding dismissal**: Auto-dismisses `OnboardingUpsell` when WebSocket sync is active
- **`acp_history_store()`**: Accessor for `ThreadStore` entity, used by WebSocket integration setup (cfg-gated)
- **Entity-level split-brain detection**: In `ThreadDisplayNotification` handler, compares `Entity` references (not just session IDs) to detect container-restart split-brain where the same thread ID has a new entity. Match on `conversation_view` (not `server_view`) in `ActiveView::AgentThread`.
- **Auto-follow activation**: After `set_active_view`, calls `workspace.follow(CollaboratorId::Agent)` if `should_be_following` is true — both for new threads and follow-up messages via the "same entity" path
- **History from connection_store**: `ThreadDisplayNotification` reads history via `this.connection_store.read(cx).entry(&Agent::NativeAgent).and_then(|e| e.read(cx).history().cloned())` — backed by `AcpSessionList`, not `NativeAgentSessionList`.

### `crates/agent_ui/src/conversation_view.rs`

> **Note:** This was previously `crates/agent_ui/src/acp/thread_view.rs`. The upstream 2026-03-22 merge renamed the `acp` module to `conversation_view`. All Helix changes moved with it.

- **`HeadlessConnection`**: No-op `AgentConnection` impl for WebSocket-created threads (cfg-gated). Must implement `agent_id()` and `new_session()` — their signatures must track the `AgentConnection` trait. Default impls handle `wait_for_tools_ready()`.
- **`from_existing_thread()` constructor**: Creates a `ConversationView` wrapping an existing `Entity<AcpThread>` with a `HeadlessConnection`. Uses `ConnectedServerState` with `connection`, `auth_state`, `active_id`, `threads` HashMap, `conversation` Entity, `history`, and `_connection_entry_subscription` (use `Subscription::new(|| {})`). Takes `connection_store` and `connection_key` parameters.
- **Thread registry integration**: Registers threads from both `from_existing_thread` and the connected state into `THREAD_REGISTRY`
- **History refresh**: Calls `self.history().update(cx, |h, cx| h.refresh(cx))` on `Stopped` events — note `history` is now a method (`history()`) not a field, and must guard with `if let Some(history) = self.history()`.
- **Thread unregistration on reset/drop**: Calls `external_websocket_sync::unregister_thread()` when the view resets or the entity changes
- **`is_resume` flag**: Uses `load_session_id.is_some()` (not the removed `resume_thread` variable) to determine whether a thread is being resumed vs created new, for the `UserCreatedThread` WebSocket event gate

### `crates/agent_ui/src/config_options.rs`

> **Note:** Previously `crates/agent_ui/src/acp/config_options.rs`.

- **`current_model_value()` method**: Returns the current model ID string from the `SessionConfigOptionCategory::Model` config option. Used by `thread_view.rs` `current_model_id()` fallback path

### `crates/agent_ui/src/conversation_view/thread_view.rs`

> **Note:** Previously `crates/agent_ui/src/acp/thread_view/active_thread.rs`.

- **`current_model_id()` fallback chain**: Now tries (1) model_selector, (2) config_options_view via `current_model_value()`, (3) global `LanguageModelRegistry::read_global()` default. This ensures headless/external threads report a model ID in UI state queries

### `crates/extensions_ui/src/extensions_ui.rs`
- **Agent keyword removal**: Claude/Codex/Gemini keywords removed from search (enterprise — users should use corporate LLMs)
- **Agent upsell removal**: Claude/Codex/Gemini upsell banners removed from extensions UI

### `crates/recent_projects/src/dev_container_suggest.rs`
- **`suggest_dev_container` check**: Early return if `RemoteSettings::suggest_dev_container` is false

### `crates/feature_flags/src/flags.rs`
- **ACP beta feature flag override**: `AcpBetaFeatureFlag::enabled_for_all()` returns `true` to enable session list/load/resume in release builds

### `crates/acp_thread/src/acp_thread.rs`
- **`content_only()` method on `AssistantMessage`**: Returns content without the `## Assistant\n\n` heading. Used by thread_service.rs for WebSocket sync to avoid sending the heading to Helix.
- **`AcpThreadEvent::Stopped` is a tuple variant**: As of the 2026-03-22 upstream merge, `Stopped` takes a `StopReason` argument: `Stopped(acp::StopReason)`. Pattern matches must use `Stopped(_)` and emission must pass a reason, e.g. `cx.emit(AcpThreadEvent::Stopped(acp::StopReason::Cancelled))`.

### `crates/acp_thread/src/connection.rs`
- **`wait_for_tools_ready()` on `AgentConnection` trait**: New method added to `AgentConnection`. Default impl returns `Task::ready(())`. `HeadlessConnection` relies on the default. `NativeAgentConnection` implementation in `context_server_registry.rs` waits for all pending MCP tool loads. **When upstream adds methods to `AgentConnection`, `HeadlessConnection` must be updated** — it won't compile otherwise.
- **`new_session()` takes `PathList` not `&Path`**: As of 2026-03-22, signature is `new_session(self: Rc<Self>, project: Entity<Project>, work_dirs: PathList, cx: &mut App)`. Use `PathList::new(&[cwd.clone()])` to construct from a `PathBuf`.
- **`load_session()` signature changed**: Now `load_session(self: Rc<Self>, session_id: acp::SessionId, project: Entity<Project>, work_dirs: PathList, title: Option<SharedString>, cx: &mut App)`. The old `AgentSessionInfo` wrapper is gone — pass `acp::SessionId::new(id)` directly.

### `crates/agent_servers/`
- **`AgentServerDelegate::new` takes 2 args**: As of 2026-03-22, signature is `new(store: Entity<AgentServerStore>, new_version_tx: Option<watch::Sender<Option<String>>>)`. The `project` and `status_tx` parameters were removed.
- **`AgentServer::connect` takes 3 args and returns `Task<Result<Rc<dyn AgentConnection>>>`**: Signature is `connect(delegate, project: Entity<Project>, cx)`. No longer returns a tuple — just `Rc<dyn AgentConnection>`.
- **`Gemini` and `ClaudeCode` structs removed**: Use `CustomAgentServer::new(AgentId("gemini-cli".into()))` and `CustomAgentServer::new(AgentId("claude".into()))` respectively.
- **`CustomAgentServer::new` takes `AgentId`**: Not `SharedString`. Use `AgentId(name.clone())`.

### `crates/agent/src/agent.rs`
- **`load_session()` entity lifetime fix**: Clones `Entity<NativeAgent>` to keep it alive during async `open_thread` task (see Critical Fixes below)
- **Multi-project `NativeAgent`**: Upstream restructured `NativeAgent` to support multiple projects: `projects: HashMap<EntityId, ProjectState>` where each `ProjectState` has `context_server_registry` and `project` fields. The old flat `agent.project` and `agent.context_server_registry()` no longer exist. `wait_for_tools_ready` uses `agent.projects.values().next()` to get the first `ProjectState`.
- **`wait_for_tools_ready` accesses `ProjectState`**: Use `project_state.context_server_registry.read(cx)` and `project_state.project.read(cx).context_server_store()` when implementing tools-ready logic.

### `crates/agent/src/agent.rs`
- **`load_session()` entity lifetime fix**: Clones `Entity<NativeAgent>` to keep it alive during async `open_thread` task (see Critical Fixes below)

### `crates/agent/src/tools/grep_tool.rs`
- **Line truncation**: `truncate_long_lines()` helper caps grep output at 500 chars per line with `[truncated, N chars total]` indicator. Prevents context window blowups when grepping minified files.

### `crates/agent/src/tools/context_server_registry.rs`
- **MCP tools-ready tracking**: Added `pending_tool_loads: usize`, `pending_server_starts: HashSet<ContextServerId>`, and `tools_ready_tx: watch::Sender<usize>` to track when all MCP servers have finished loading tools. Implements `wait_for_tools_ready()` for `NativeAgentConnection` by watching for `pending_tool_loads` to reach zero.

### `crates/workspace/src/workspace.rs`
- **Agent follow doesn't steal keyboard focus**: In `follow()` and `update_follower_items()`, added `!matches!(leader_id, CollaboratorId::Agent)` guard before `window.focus(...)` calls. When following the agent, Zed tracks the agent's active file visually without stealing keyboard focus from the user's current input. **Critical: upstream will modify `follow()` frequently — this guard must be re-checked after every merge.**

### `crates/zed/src/zed/migrate.rs`
- **Migration banner hidden in Helix builds**: Early return `ToolbarItemLocation::Hidden` when `cfg!(feature = "external_websocket_sync")`. In Helix, settings are managed by the settings-sync-daemon and the migration prompt is irrelevant.

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

### 5. Flush Stale Pending Entries When a Different Entry Starts Streaming

**File:** `crates/external_websocket_sync/src/thread_service.rs`

**Bug:** When two entries stream concurrently (e.g., a tool call overlaps with a text entry), the throttle buffer can hold a stale pending message for the old entry while a new entry starts. The stale message is then sent out of order or dropped.

**Fix:** At the start of each streaming update, check whether the incoming `message_id` differs from the buffered pending message. If so, flush all stale pending entries for other `message_id`s before processing the new entry. This preserves ordering and ensures every entry's content reaches Helix.

**History:** Added in `6e4967240a`. Required by multi-tool-call E2E test scenarios.

**Symptom:** Tool call results appear out of order or are missing from the Helix session view.

### 6. AcpThread::Stopped Must Be Emitted for Every Turn

**File:** `crates/acp_thread/src/acp_thread.rs`

**Invariant:** Every call to `AcpThread::send()` must eventually emit exactly one `AcpThreadEvent::Stopped`, even if a second `send()` displaces the first turn mid-stream. Helix uses `message_completed` (triggered by `Stopped`) to pop its FIFO queue and route the next response. Missing a `Stopped` stalls the queue permanently.

**Context:** This is an upstream invariant that must hold across merges. If upstream changes `AcpThread::send()` to cancel in-progress turns without emitting `Stopped`, all subsequent Helix messages will stall.

**Test:** `test_second_send_during_active_turn_emits_stopped_for_both_turns` in `acp_thread.rs` verifies this invariant. Run it after every upstream merge: `cargo test -p acp_thread test_second_send`.

**History:** Documented in `8b033a4451`.

**Symptom:** Follow-up messages from Helix queue up but never get responses — Zed appears to process only the first message then goes silent.

### 7. THREAD_REGISTRY Must Be Unregistered on Entity Replacement

**File:** `crates/agent_ui/src/acp/thread_view.rs`

**Bug:** After a container restart, `load_thread_from_agent()` creates a **new** `Entity<AcpThread>` for the same session ID. If the old entity is still registered in `THREAD_REGISTRY`, thread_service will send follow-up messages to the stale entity, which no longer receives live events. The agent panel observes the new entity (live), but Helix receives updates from the dead entity — causing "brain split" where Zed is working but Helix sees nothing.

**Fix:** When `thread_view.rs` detects the displayed thread entity has changed (comparing by `EntityId`, not session ID), call `external_websocket_sync::unregister_thread()` before rebinding. The new entity registration happens automatically when thread_service re-registers it.

**History:** Added in `87632d00ce`. Detected by checking `active_thread.read(cx).thread == notification.thread_entity` in the `ThreadDisplayNotification` handler.

**Symptom:** After container restart, Zed works fine locally but all Helix messages are silently swallowed — no responses appear in the Helix session.

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
6. **Check `thread_service.rs` streaming path** — ensure stale pending entries are flushed when a new entry starts (Critical Fix #5)
7. **Run `cargo test -p acp_thread test_second_send`** — verifies `Stopped` invariant (Critical Fix #6)
8. **Check `thread_view.rs` unregistration** — ensure `unregister_thread()` is called when entity changes (Critical Fix #7)
9. **Check `agent_panel.rs` cfg-gated blocks** — callback setup, `from_existing_thread`, onboarding dismissal, `acp_history_store()`, entity-level split-brain detection, auto-follow activation
10. **Check `conversation_view.rs` cfg-gated blocks** — `HeadlessConnection` (needs `agent_id()` + correct `new_session(PathList)` signature), `from_existing_thread()`, THREAD_REGISTRY registration, `self.history()` method call (not field), `is_resume = load_session_id.is_some()` (not `resume_thread`), `unregister_thread()` on reset
11. **Check `from_existing_thread()` matches `ConnectedServerState` struct** — upstream may change required fields (currently: `connection`, `auth_state`, `active_id`, `threads` HashMap, `conversation` Entity, `history`, `_connection_entry_subscription`). `ConversationView` itself also requires `connection_store` and `connection_key` fields (no `login`/`history` direct fields).
12. **Check `connection.rs` `AgentConnection` trait** — if upstream added new methods, `HeadlessConnection` must implement them. Currently requires `agent_id()` and `new_session(project, PathList, cx)`. Check for compilation errors.
12a. **Check `AcpThreadEvent::Stopped` usage** — it's a tuple variant `Stopped(StopReason)`. Pattern matches must use `Stopped(_)`, emissions must pass a reason e.g. `AcpThreadEvent::Stopped(acp::StopReason::Cancelled)`.
13. **Check `thread_service.rs` uses new `AgentServer`/`AgentConnection` APIs** — `AgentServerDelegate::new(store, None)` (2 args), `server.connect(delegate, project, cx)` (3 args, returns `Rc` not tuple), `connection.new_session(project, PathList::new(&[cwd]), cx)`, `connection.load_session(session_id, project, PathList::new(&[cwd]), None, cx)` (5 args), `first_method.id()` (method not field)
14. **Check `types.rs` `ExternalAgent::server()` uses `CustomAgentServer::new(AgentId(...))`** — `Gemini`/`ClaudeCode` structs removed from `agent_servers`
15. **Check `workspace.rs` `follow()` and `update_follower_items()`** — `CollaboratorId::Agent` must not steal keyboard focus (no `window.focus()` call for Agent leader)
16. **Check `migrate.rs`** — migration banner returns `Hidden` in Helix builds
17. **Check `grep_tool.rs`** — `truncate_long_lines()` and `MAX_LINE_CHARS = 500` present
18. **Check `config_options.rs`** — `current_model_value()` method present
19. **Check `conversation_view/thread_view.rs` `current_model_id()`** — three-way fallback (selector → config_options → global registry)
20. **Check `extensions_ui.rs`** — agent keyword/upsell removal preserved
21. **Check `dev_container_suggest.rs`** — `suggest_dev_container` early return preserved
22. **Check `feature_flags/flags.rs`** — `AcpBetaFeatureFlag::enabled_for_all()` returns `true`
23. **Check `http_client_tls.rs`** — `NoCertVerifier` and `ZED_HTTP_INSECURE_TLS` support
24. **Check `reqwest_client.rs`** — insecure TLS support
25. **Check `title_bar`** — connection status indicator + `external_websocket_sync` dependency
26. **Check `agent_settings`** — `show_onboarding`, `auto_open_panel` fields
27. **Check `.dockerignore`** — simplified for Helix builds
28. **Check `SyncEvent::MessageAdded`** — has `entry_type`, `tool_name`, `tool_status` fields
29. **Check `SyncEvent::UiStateResponse`** — has `mcp_servers` and `active_model` fields
30. **Check `NativeAgent` multi-project**: `agent.projects.values().next()` to get `ProjectState`; no more flat `agent.project` or `agent.context_server_registry()` fields/methods
31. **Run `cargo check --package zed --features external_websocket_sync`** — must compile
32. **Run `cargo test -p external_websocket_sync`** — unit tests
33. **Run E2E test** after merge to verify all phases pass

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
| `818cf940e6` | Fix: adapt external_websocket_sync to upstream connect() API change |
| `0b9e2211dc` | Fix: wire up auto_open_panel setting to AgentPanel starts_open() |
| `2f74e89657` | Fix: disable migration banner in Helix builds (`migrate.rs`) |
| `f51c0d5dae` | Truncate long lines in grep tool output (500 char limit) |
| `1fab62117e` | Prevent keyboard focus stealing when following agent (`workspace.rs`) |
| `87632d00ce` | **Fix: thread entity split-brain after container restart (unregister on entity change)** |
| `3e4d7d7bbc` | Fix: wait for MCP tools to load before sending first WebSocket message |
| `d511c3e983` | Add Dockerfile.ci for E2E tests in CI |
| `e42b1ad95e` | Fix auto-follow mode and split-brain for external WebSocket sessions |
| `29f10aa7ad` | Emit MessageCompleted from Stopped event for all turn sources |
| `182cae0ead` | Fix missing message_completed in follow-up subscription |
| `91c281fb93` | Extract ensure_thread_subscription to fix missing event handlers |
| `c33ee0483b` | Add entry_type field to MessageAdded sync event |
| `1e66f0ada2` | Add ResponseEntries validation to E2E test |
| `6e4967240a` | **Fix: flush stale pending entries when different entry starts streaming** |
| `4e204c4d7d` | Handle ToolCall in NewEntry event (not just EntryUpdated) |
| `bfe84a2134` | Send structured tool_name and tool_status in message_added events |
| `e38aad1a18` | Clear persistent subscription on unregister to fix E2E timeout |
| `8b033a4451` | **Test: add Stopped emission and mid-stream interrupt E2E tests (Critical Fix #6)** |
