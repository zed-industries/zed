# AI Agents GUI Update - Tasks

## Current Status: Ready for manual testing

## Completed
- [x] **Edit flow (pencil button)**
  - [x] `AcpSessionTruncator` enables pencil button for all ACP agents
  - [x] `rewind()` removes entries locally (instant, seamless)
  - [x] `.jsonl` file truncated on disk for persistence
  - [x] Edited message sent to same session
  - Note: ACP server keeps old messages in memory (protocol limitation). When session is reopened from history, it loads from the truncated file correctly.

- [x] **Fork button**
  - [x] Split icon button next to edit controls on user messages
  - [x] Creates truncated `.jsonl` copy with new session UUID
  - [x] Shows toast notification with session ID
  - [x] Forked session appears in history panel and can be opened from there
  - Note: Fork does NOT open the forked thread directly (ACP `load_session` can't stream history for files the server didn't create). User opens from history panel instead.

- [x] **File infrastructure**
  - [x] `session_file_path()` on `AgentConnection` trait
  - [x] Claude Code: `~/.claude/projects/{sanitized-path}/{session_id}.jsonl`
  - [x] Codex: recursive scan via `find_codex_session_file()`
  - [x] `truncate_session_file()` async function

## Architecture

### Edit flow:
```
pencil → edit → enter → regenerate() → rewind()
  → entries.truncate(ix)           # instant UI update
  → truncate_session_file()        # disk persistence
  → send_impl()                    # edited message to same session
```

### Fork flow:
```
fork button → fork_conversation()
  → read + truncate file content
  → write to new {uuid}.jsonl
  → show toast: "Forked session saved. Open from history."
```

### Known ACP limitation:
The ACP protocol has no `truncate_session` or `close_session` RPC. The server (CLI process) maintains conversation history in memory. We can't make it forget messages. The file truncation ensures correct state on next load from history.

## Files modified:
- `crates/acp_thread/src/connection.rs` — `session_file_path()` trait method
- `crates/acp_thread/src/acp_thread.rs` — `rewind()` with file truncation, `truncate_session_file()`
- `crates/agent_servers/src/acp.rs` — `AcpSessionTruncator`, `truncate()`, `session_file_path()`, `find_codex_session_file()`
- `crates/agent_servers/Cargo.toml` — added `dirs` dependency
- `crates/agent_ui/src/acp/thread_view.rs` — fork button, `fork_conversation()`, import updates
- `crates/agent_ui/src/agent_panel.rs` — (cleaned up, no fork_thread needed)
