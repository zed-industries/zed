# Open CLI Sessions in Agent Panel — Important Context

## Critical Information

### The Two Panel Systems
- **`crates/agent_ui/`** (v1) — RIGHT dock. Contains `thread_view.rs` (conversations), `thread_history.rs` (history list), `agent_panel.rs` (panel orchestration). This is where the agent panel GUI lives.
- **`crates/agent_ui_v2/`** (v2) — LEFT dock. Contains `agents_panel.rs` with session tabs. Has its own `resume_cli_session()` for opening in terminal.
- **BOTH are active.** The right dock is where conversations happen. The left dock has the session history tabs (Claude CLI / Codex CLI).
- The history panel in the RIGHT dock is `thread_history.rs`. The tabs in the LEFT dock are in `agents_panel.rs`.

### This Feature Targets the RIGHT Dock History Panel
The right-click context menu goes on `thread_history.rs` items. The `OpenInPanel` event is handled by `agent_panel.rs`. Both are in `crates/agent_ui/`.

### ACP Server Streams History — No Client Parsing Needed
When `load_session` is called on the ACP server, the server reads the `.jsonl` file and streams conversation history as `SessionUpdate` events:
- `UserMessageChunk` → user messages
- `AgentMessageChunk` → assistant messages
- `ToolCall` / `ToolCallUpdate` → tool calls
- `Plan` → plan entries

The `AcpThread` is created with empty entries, registered in the sessions map FIRST (so streamed events can attach), then entries populate as events arrive. This is NOT synchronous — entries arrive asynchronously during the `load_session` RPC.

### CLI Session Metadata
CLI sessions have metadata stored in `AgentSessionInfo.meta`:
- `CLI_SOURCE_KEY` (value: `"claude"` or `"codex"`) — identifies the CLI tool
- `CLI_PROJECT_PATH_KEY` (value: absolute path) — the project directory

These constants are defined in `crates/agent/src/claude_code_sessions.rs` and re-exported via `crates/agent/src/agent.rs`.

### ExternalAgent Variants
```rust
pub enum ExternalAgent {
    Gemini,
    ClaudeCode,
    Codex,
    NativeAgent,
    Custom { name: SharedString },
}
```
Each variant knows how to start its CLI process and create a connection.

## Caveats

- **Session must be loadable by the CLI:** `load_session(session_id)` tells the CLI to find and load the session. If the CLI process has been restarted since the session was created, it reads from the `.jsonl` file on disk. If the file was renamed (due to edit/rewind), the old session_id won't find the file — only the new session_id will work.

- **Codex session ID mapping:** Codex filenames are `rollout-{timestamp}-{uuid}.jsonl`. The session_id is the UUID extracted from the filename. The CLI must be able to map session_id → file for `load_session` to work. May need testing.

- **No context menu exists yet:** The history panel items (`thread_history.rs`) have no right-click menu. There are click handlers and icon buttons (notepad, trash) but no `ContextMenu` or `right_click_menu`.

- **`external_thread()` starts the CLI if needed:** When `external_thread()` is called with `ExternalAgent::ClaudeCode`, it will start the Claude Code CLI process if not already running, establish the ACP connection, then call `load_thread()`.

- **History panel items may not have CLI metadata:** Sessions loaded via ACP directly (not from `.jsonl` scanning) may not have `CLI_SOURCE_KEY` in their metadata. The context menu should check for this and only show "Open in Terminal" / "Open in Agent Panel" options when it's a CLI session.

## Dependencies

- `ui::right_click_menu` — UI primitive for context menus
- `ui::ContextMenu` — the menu builder
- `acp_thread::AgentSessionInfo` — session info with metadata
- `agent::CLI_SOURCE_KEY`, `agent::CLI_PROJECT_PATH_KEY` — metadata keys

## Testing Notes

1. **Build:** `env -u CLAUDECODE cargo run` (unset CLAUDECODE env var when launching dev Zed from a Claude Code session)
2. **Test right-click menu:**
   - Open history panel (right dock)
   - Right-click a Claude Code session → should see "Open in Terminal" and "Open in Agent Panel"
   - Click "Open in Agent Panel" → should load conversation in the agent panel with full history
   - Click "Open in Terminal" → should open in terminal (existing behavior)
3. **Verify history loads:**
   - Messages should appear in the agent panel (user messages, assistant responses)
   - Should be able to continue the conversation by typing a new message
4. **Test with Codex sessions:** May or may not work depending on session_id mapping

## Known Limitations

1. **Can't open renamed sessions:** If a session was edited (rewind), the file was renamed to a new UUID. The old session_id in the history panel won't match the file anymore. Only the new session appears in history after a refresh.
2. **Codex support uncertain:** Codex session_id to filename mapping is non-trivial. May need additional work.
3. **No offline/client-side loading:** All history loading goes through the ACP server. If the CLI process can't be started, the session can't be loaded in the agent panel.

## Related Files

- `/Volumes/Code/GitHub/zed/_context/chat-to-agent-panel/_context.md` — session context from this conversation
- `/Volumes/Code/GitHub/zed/_plans/ai-agents-gui-update/task.md` — prior work on edit/fork features
- `/Volumes/Code/GitHub/zed/crates/agent_ui/src/acp/thread_history.rs` — history panel (target for context menu)
- `/Volumes/Code/GitHub/zed/crates/agent_ui/src/agent_panel.rs` — panel orchestration (target for event handler)
- `/Volumes/Code/GitHub/zed/crates/agent_ui/src/acp/thread_view.rs` — thread view (already handles loaded sessions)
- `/Volumes/Code/GitHub/zed/crates/agent_servers/src/acp.rs` — `load_thread()`, `session_notification()`, ACP protocol

## Recovery Instructions

If resuming after compaction:
1. Read this file first for context and caveats
2. Read `task.md` for current progress
3. Read `plans.md` for full technical implementation details
4. Check git status for any uncommitted work
5. The branch is `terminal-tab-customization`
6. Key files to read before coding:
   - `crates/agent_ui/src/acp/thread_history.rs` — where context menu goes
   - `crates/agent_ui/src/agent_panel.rs:796` — current event handler for `ThreadHistoryEvent::Open`
   - `crates/agent_ui/src/acp/thread_view.rs:619` — `initial_state()` showing how loaded sessions populate
   - `crates/agent_servers/src/acp.rs:572` — `load_thread()` implementation
