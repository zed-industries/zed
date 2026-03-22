# AI Agents GUI Update - Important Context

## Critical Information

### The Two Agent Panel Systems
- **`crates/agent_ui/`** — v1 agent panel, renders in the RIGHT dock. Contains `thread_view.rs` which has the pencil button and `regenerate()` flow.
- **`crates/agent_ui_v2/`** — v2 agent panel, renders in the LEFT dock. Contains `agents_panel.rs` and `thread_history.rs`. This is what the user sees for the history/sessions list.
- **BOTH are active.** The right dock panel (`agent_ui`) is where conversations happen. The left dock panel (`agent_ui_v2`) is where session history/tabs are.
- Previous work accidentally edited the wrong crate — always verify which panel you're modifying.

### ACP Protocol Limitation
- The ACP protocol (external crate `agent-client-protocol` v0.9.3) has **NO** `truncate_session` or `rewind_session` method.
- Each `prompt()` call sends only `session_id + message_content`. The server (CLI process) maintains full conversation history.
- There is no way to tell the server to forget messages. That's why `truncate()` is a no-op.
- `fork_session` exists but copies FULL history (no truncation parameter).

### Session File Locations
- **Claude Code:** `~/.claude/projects/{sanitized-project-path}/{session_id}.jsonl`
  - Sanitized path: replace `/` with `-` (e.g., `/Volumes/Code/GitHub/zed` → `-Volumes-Code-GitHub-zed`)
- **Codex CLI:** `~/.codex/sessions/{year}/{month}/{day}/rollout-{timestamp}-{id}.jsonl`
  - Session ID in ACP may not directly map to filename
  - File path stored in session meta as `codex_file_path`

### JSONL Entry Types
**Claude Code format:**
- `type: "user"` — user message, content at `message.content` (string or array)
- `type: "assistant"` — assistant response, content at `message.content`
- `type: "progress"` — tool call progress
- Other types: `queue-operation`, `file-history-snapshot`

**Codex CLI format:**
- `type: "response_item"` with `payload.role: "user"|"assistant"` — messages, content at `payload.content[]`
- `type: "session_meta"` — metadata with `payload.cwd`
- `type: "event_msg"`, `type: "turn_context"` — non-message entries

## Caveats

- **Pencil button only works on NEW messages:** The `message.id` is assigned at message creation time (`submit_user_message` line 1871). Messages created before `truncate()` was implemented have `id=None` and can't be edited. Only messages sent AFTER the code change will have the pencil enabled.

- **Agent still has "deleted" messages:** The no-op truncation means the ACP server still has the full conversation in memory. The agent may occasionally reference "deleted" content. This is a known limitation of the ACP protocol.

- **File truncation is best-effort:** The background file truncation task could fail (permission issues, file locked). This is logged but doesn't block the UI flow. The file may be slightly out of sync until the next write.

- **`acp_thread` crate doesn't depend on `agent` crate:** The `session_file_path()` derivation logic might need to live in `agent_servers` (which depends on both) or use the `AgentConnection` trait method approach.

- **Test stubs return no-op truncate:** `StubAgentSessionEditor` and `FakeAgentSessionEditor` both return `Task::ready(Ok(()))` from `run()`. They don't implement `new_session_id()` or `session_file_path()`. Adding default methods to the trait keeps them working.

## Dependencies

- `agent-client-protocol` v0.9.3 (external, on crates.io) — ACP types
- `serde_json` — for parsing JSONL lines
- `smol::fs` — for async file read/write in background task
- `dirs` crate — for `home_dir()` to derive session file paths

## Testing Notes

1. **Compilation:** `cargo check -p agent_servers -p acp_thread -p agent`
2. **Existing tests:** `cargo test -p acp_thread` — must pass (stubs are unchanged)
3. **Manual testing:**
   - Open agent panel (right dock) → click `+` → choose Codex CLI or Claude Code
   - Send 2-3 messages, get responses
   - Click pencil on a user message → edit text → press enter
   - Verify: messages below vanish instantly, no flicker
   - Verify: agent responds to edited message
   - Verify: `.jsonl` file on disk is truncated (check with `cat` or ThreadContentEditor)
   - Close and reopen session from history → verify truncated conversation

## Known Limitations

1. **No true server-side truncation:** Agent still has old messages in memory. Requires ACP protocol change to fix.
2. **Codex file path TBD:** Deriving Codex session file path from session_id is non-trivial. May need to store the path or search.
3. **Only for ACP agents:** Native Zed Agent threads use SQLite, different mechanism entirely.
4. **Can't edit messages sent before code change:** Old messages have `id=None`.

## Related Files

- `/Users/alesloas/.claude/plans/jiggly-sparking-sutton.md` — approved plan file
- `/Volumes/Code/GitHub/zed/_plans/conext-editor/tasks.md` — ThreadContentEditor tasks (partially related)
- `/Volumes/Code/GitHub/zed/crates/agent_ui/src/thread_content_editor.rs` — existing file editing code (reusable patterns)

## Recovery Instructions

If resuming after compaction:
1. Read this file first for context and caveats
2. Read `task.md` for current progress (check which step you're on)
3. Read `plans.md` for full technical implementation details
4. Check git status for any uncommitted work
5. The branch is `terminal-tab-customization`
6. Key files to read before coding:
   - `crates/agent_servers/src/acp.rs` — where truncate() goes
   - `crates/acp_thread/src/acp_thread.rs:2084` — rewind() method
   - `crates/acp_thread/src/connection.rs:130` — AgentSessionTruncate trait
   - `crates/agent_ui/src/acp/thread_view.rs:1703` — regenerate() flow
