# Session: ACP Agent Message Editing, Fork, and History Panel Loading
Date: 2026-03-22

## Goal

Three interconnected features for the Zed agent panel (right dock) with ACP-based agents (Claude Code, Codex CLI):

1. **Message Editing**: Enable the pencil/edit button on user messages so users can edit a previous message, have entries below vanish, the `.jsonl` file truncated on disk, and the edited message re-sent with the server loading from the truncated file (not remembering deleted messages).

2. **Fork Conversation**: Add a fork button (Split icon) on user messages that creates a truncated copy of the `.jsonl` session file and notifies the user it's available in the history panel.

3. **Open CLI Sessions in Agent Panel** (planned, not yet implemented): Add right-click context menu on history panel items to choose between "Open in Terminal" (current behavior) or "Open in Agent Panel" (load via ACP streaming protocol).

## Files Changed

### Modified

#### `crates/acp_thread/src/connection.rs`
- **Lines ~73-83**: Added `session_file_path()` default method to `AgentConnection` trait (returns `None`)
- **Lines ~85-95**: Added `swap_session()` default method to `AgentConnection` trait (returns error). Takes old/new session IDs, thread weak entity, and swaps the server-side session by calling `load_session` with the new ID and re-registering in the sessions HashMap.

#### `crates/agent_servers/src/acp.rs`
- **Lines ~27-29**: Updated import to include `AgentSessionTruncate`, `UserMessageId` from `acp_thread`
- **Lines ~817-825**: Added `truncate()` method on `impl AgentConnection for AcpConnection` — returns `Some(Rc::new(AcpSessionTruncator))`. This is what enables the pencil button (makes `message_id = Some(UserMessageId::new())` at acp_thread.rs:1871).
- **Lines ~825-838**: Added `session_file_path()` on `AcpConnection` — derives file path:
  - Claude Code: `~/.claude/projects/{root_dir with / replaced by -}/{session_id}.jsonl`
  - Codex: scans `~/.codex/sessions/` recursively via `find_codex_session_file()`
- **Lines ~840-904**: Added `swap_session()` on `AcpConnection` — calls `conn.load_session()` (or `resume_session`) with new session_id on ACP server, creates new `AcpSession` entry, removes old session from HashMap.
- **Lines ~907-913**: Added `AcpSessionTruncator` struct implementing `AgentSessionTruncate` with no-op `run()`.
- **Lines ~1393-1413**: Added `find_codex_session_file()` function — recursively scans a directory for `.jsonl` files whose stem ends with the given session_id.

#### `crates/agent_servers/Cargo.toml`
- Added `dirs.workspace = true` dependency (for `dirs::home_dir()` in session file path derivation).

#### `crates/acp_thread/src/acp_thread.rs`
- **`rewind()` method (~lines 2084-2145)**: Heavily modified. After truncating entries locally and the `.jsonl` file on disk, it now:
  1. Generates a new session UUID
  2. **Renames** (not copies) the truncated file to `{new_uuid}.jsonl`
  3. Calls `connection.swap_session()` to load the new session on the ACP server
  4. Updates `self.session_id` to the new ID
  - This forces the server to re-read from the truncated file, so it only has context of the kept messages.
- **`truncate_session_file()` (~lines 2580-2612)**: New async function added before `#[cfg(test)]`. Reads the `.jsonl` file, counts visible entries (user/assistant for Claude Code format, response_item with user/assistant role for Codex format), and writes back only the lines up to `keep_entry_count`.

#### `crates/agent_ui/src/acp/thread_view.rs`
- **Line 12**: Added `Context as _` to anyhow import (needed for `.context()` calls)
- **Lines ~1758-1831**: Added `fork_conversation()` method:
  - Reads the session `.jsonl` file
  - Counts visible entries up to `entry_ix + 1`
  - Writes truncated content to new `{uuid}.jsonl` file
  - Shows a toast notification: "Forked conversation saved ({short_id}). Open it from the history panel."
  - Does NOT try to open the fork in a new thread (ACP server can't stream history for files it didn't create).
- **Lines ~2505-2520**: Added fork button (IconName::Split) in the user message button bar, between the cancel button and the regenerate button. Tooltip: "Fork conversation from this point into a new thread."

## Problems & Solutions

### Problem 1: No-op truncation doesn't reset server context
**Issue**: Initial implementation used a no-op `AcpSessionTruncator`. The `.jsonl` file was truncated on disk, but the ACP server still had all messages in memory. Agent would say "this is your 5th message" even after editing message 2.
**Cause**: ACP protocol has no `truncate_session` RPC. The server maintains conversation history per session_id. Sending to the same session means the server still has the "deleted" messages.
**Fix**: After truncating the file, rename it to a new session_id and call `load_session` on the server with the new ID. Server reads the renamed truncated file and only has the kept messages. The `swap_session()` method on `AgentConnection` handles this.

### Problem 2: Edit created duplicate files in history
**Issue**: First attempt at session swap used `smol::fs::copy()` to create a new file. This left both the original AND the copy in the project directory, cluttering the history panel.
**Cause**: Using `copy` instead of `rename`.
**Fix**: Changed to `smol::fs::rename()` — the original file is moved (not duplicated). One file, one session in history.

### Problem 3: Fork showed empty thread
**Issue**: Fork created the `.jsonl` file and tried to open it in a new `AcpThreadView` via `AgentPanel.fork_thread()`. The thread opened but showed no messages.
**Cause**: The ACP server (Claude Code CLI) doesn't stream conversation history for session IDs it didn't create. `load_session` succeeds (returns metadata) but doesn't stream `UserMessageChunk`/`AgentMessageChunk` events for unknown session files.
**Fix**: Changed fork to NOT open a new thread. Instead shows a toast notification telling the user the forked session is available in the history panel. When opened from history (potentially after CLI restart), it loads correctly.

### Problem 4: `log` crate not available in `acp_thread`
**Issue**: Used `log::error!()` in `acp_thread.rs` but the `log` crate isn't a dependency.
**Fix**: Changed to `.detach_and_log_err(cx)` pattern (using `util::ResultExt`) which is the crate's standard error reporting approach.

### Problem 5: `action_log.reject_all_edits()` return value not awaited
**Issue**: When restructuring `rewind()`, the `Task<()>` returned by `reject_all_edits` was dropped instead of awaited.
**Fix**: Collected it as a separate return value from the update closure and awaited it explicitly.

## Current Status

- **Edit flow (pencil button)**:
  - Pencil button enabled on all ACP agents (Claude Code, Codex)
  - Entries below edit point removed instantly from UI
  - `.jsonl` file truncated on disk
  - File renamed to new session ID, server loads from truncated file
  - Edited message sent to new session — server has correct context
  - **Known limitation**: If `load_session` fails for the new session ID (server can't find the renamed file), the edit still works locally but server context may not reset

- **Fork button**:
  - Split icon button appears next to edit controls on user messages
  - Creates truncated `.jsonl` copy with new UUID
  - Shows toast notification directing user to history panel
  - Forked session appears in history and can be opened from there

- **Open in Agent Panel** (planned, not implemented):
  - Research complete — the ACP streaming protocol already handles history loading
  - `load_session` → server streams `SessionUpdate` events → entries populate
  - Need: right-click context menu on history items, new event handler routing CLI sessions to `external_thread()` instead of `resume_cli_session()`
  - Estimated ~45 lines of code across `thread_history.rs` and `agent_panel.rs`

## Architecture Notes

### The Two Panel Systems
- `crates/agent_ui/` (v1) — RIGHT dock. Contains `thread_view.rs` (conversations), `thread_history.rs` (history list). This is where editing/forking happens.
- `crates/agent_ui_v2/` (v2) — LEFT dock. Contains `agents_panel.rs` with session tabs. Has its own `resume_cli_session()` for opening in terminal.
- **Both are active simultaneously.**

### ACP Session Loading Flow
```
load_thread(session_id)
  → AcpThread created with empty entries
  → Session registered in sessions HashMap (BEFORE awaiting RPC)
  → conn.load_session() RPC sent to ACP server
  → Server streams SessionUpdate events:
      UserMessageChunk → push_user_content_block()
      AgentMessageChunk → push_assistant_content_block()
      ToolCall → upsert_tool_call()
  → AcpThread.entries populated
  → AcpThreadEvent::NewEntry emitted for each
  → thread_view syncs entries to UI
```

### Session File Locations
- **Claude Code**: `~/.claude/projects/{path-with-slashes-as-dashes}/{session_id}.jsonl`
- **Codex**: `~/.codex/sessions/{year}/{month}/{day}/rollout-{timestamp}-{uuid}.jsonl`

### Key Line References
- Pencil button enabled: `acp_thread.rs:1871` — `message_id = Some(UserMessageId::new())` when `truncate()` returns `Some`
- Pencil button rendered: `thread_view.rs:~2495` — checks `message.id.is_some()`
- `regenerate()`: `thread_view.rs:~1703` — calls `rewind()` then `send_impl()`
- `rewind()`: `acp_thread.rs:~2084` — truncate entries, truncate file, rename, swap session
- `handle_session_update()`: `acp_thread.rs:~1284` — processes streaming events from server
- History open handler: `agent_panel.rs:~796` — routes CLI sessions to terminal vs agent panel
