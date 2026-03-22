# AI Agents GUI Update - Implementation Plan

## Status: IN_PROGRESS
## Last Updated: 2026-03-22

## Goal
Enable the pencil/edit button on user messages in the Zed agent panel (right dock) for ACP-based agents (Codex CLI, Claude Code). When clicked, the user edits a message inline, presses enter, entries below vanish instantly, the `.jsonl` session file is truncated in the background, and the edited message is sent to the agent.

## Context

The agent panel shows a pencil icon on every user message, but it's disabled for all production ACP agents with the tooltip: "Editing previous messages is not available for [agent] yet." The entire edit/rewind/resend flow already works in tests — the only blocker is that `AcpConnection` doesn't implement the `truncate()` trait method (returns default `None`).

### Prior Work in This Branch
This branch (`terminal-tab-customization`) already contains significant custom work:
1. **ThreadContentEditor** (`crates/agent_ui/src/thread_content_editor.rs`) — a workspace tab that opens `.jsonl` session files, shows user/assistant messages with checkboxes for deletion. Uses truncation-point selection (click a message = select everything from there to end).
2. **Codex CLI tab** in thread history panel — second tab alongside "Claude CLI" for browsing Codex sessions scoped to the current project.
3. **Codex resume fix** — uses `codex resume <id>` (subcommand) not `codex --resume <id>` (flag).
4. **Terminal improvements** — Codex sessions now open as regular terminals (not task terminals) to avoid play button, blue dot, and rename issues.

## Technical Approach

### Architecture Overview

The edit flow chain:
```
UI: pencil button (thread_view.rs:2495)
  → message.id must be Some(UserMessageId)
  → enabled when connection.truncate() returns Some

User edits + presses enter:
  → thread_view.rs: regenerate() (line 1703)
    → acp_thread.rs: rewind(user_message_id) (line 2084)
      → truncate.run() — no-op for ACP, returns Ok
      → entries.truncate(ix) — removes local entries instantly
      → background: truncate .jsonl file on disk
    → thread_view.rs: send_impl(message_editor) (line 1750)
      → acp_thread.rs: submit_user_message() (line 1860)
        → PromptRequest::new(session_id, edited_message)
        → connection.prompt() — sends to same session
```

### Step 1: Enable pencil button — implement truncate() on AcpConnection

**File:** `/Volumes/Code/GitHub/zed/crates/agent_servers/src/acp.rs`

Add after `AcpSessionModes` struct (~line 884):
```rust
struct AcpSessionTruncator;

impl AgentSessionTruncate for AcpSessionTruncator {
    fn run(&self, _message_id: UserMessageId, _cx: &mut App) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }
}
```

Add import at top:
```rust
use acp_thread::{AgentSessionTruncate, UserMessageId};
```

In `impl AgentConnection for AcpConnection` block (after `cancel()` at ~line 815):
```rust
fn truncate(
    &self,
    _session_id: &acp::SessionId,
    _cx: &App,
) -> Option<Rc<dyn AgentSessionTruncate>> {
    Some(Rc::new(AcpSessionTruncator))
}
```

**Why no-op works:** The ACP protocol has no `truncate_session` RPC. The server maintains conversation history. A no-op truncation enables the UI flow — `rewind()` removes entries locally, `submit_user_message()` sends the edited message to the same session. The agent still has prior context (all messages including "deleted" ones), but the UI is correct and the agent focuses on the latest prompt.

### Step 2: Background file truncation in rewind()

**File:** `/Volumes/Code/GitHub/zed/crates/acp_thread/src/acp_thread.rs`

In `rewind()` method (~line 2084), after entries are truncated locally, add background file truncation:

```rust
// After this.entries.truncate(ix):
if let Some(file_path) = this.session_file_path() {
    let kept_count = ix;
    cx.background_spawn(async move {
        if let Err(e) = truncate_session_file(&file_path, kept_count).await {
            log::error!("Failed to truncate session file: {}", e);
        }
    }).detach();
}
```

### Step 3: Derive session file path

**File:** `/Volumes/Code/GitHub/zed/crates/acp_thread/src/acp_thread.rs`

Add method to AcpThread:
```rust
fn session_file_path(&self) -> Option<PathBuf> {
    // Use AgentConnection trait to get file path info
    // For Claude Code: ~/.claude/projects/{sanitized-project-path}/{session_id}.jsonl
    // For Codex: search ~/.codex/sessions/ or use stored path
    self.connection.session_file_path(&self.session_id)
}
```

This requires adding `session_file_path()` to the `AgentConnection` trait in `connection.rs` (default returns `None`), and implementing it on `AcpConnection` in `acp.rs`.

**File:** `/Volumes/Code/GitHub/zed/crates/acp_thread/src/connection.rs`
```rust
fn session_file_path(&self, _session_id: &acp::SessionId) -> Option<PathBuf> {
    None
}
```

**File:** `/Volumes/Code/GitHub/zed/crates/agent_servers/src/acp.rs`
```rust
fn session_file_path(&self, session_id: &acp::SessionId) -> Option<PathBuf> {
    // Derive based on server_name and root_dir
    let home = dirs::home_dir()?;
    if self.server_name.contains("Claude") {
        let folder = self.root_dir.to_string_lossy().replace('/', "-");
        let dir = home.join(".claude/projects").join(&folder);
        Some(dir.join(format!("{}.jsonl", session_id.0)))
    } else if self.server_name.contains("Codex") || self.server_name.contains("codex") {
        // For Codex, would need to search or store the path
        // Could reuse CodexSessionIndex scanning logic
        None // TODO: implement for Codex
    } else {
        None
    }
}
```

**File:** `/Volumes/Code/GitHub/zed/crates/agent/src/claude_code_sessions.rs`
Make `sessions_dir_for_project()` public so it can be reused.

### Step 4: File truncation function

**File:** `/Volumes/Code/GitHub/zed/crates/acp_thread/src/acp_thread.rs`

```rust
async fn truncate_session_file(file_path: &Path, keep_entry_count: usize) -> Result<()> {
    let content = smol::fs::read_to_string(file_path).await?;
    let lines: Vec<&str> = content.lines().collect();

    let mut visible_count = 0;
    let mut truncate_at_line = lines.len();
    for (i, line) in lines.iter().enumerate() {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(line) {
            let msg_type = value.get("type").and_then(|t| t.as_str()).unwrap_or("");
            let is_visible = matches!(msg_type, "user" | "assistant")
                || (msg_type == "response_item"
                    && matches!(
                        value.pointer("/payload/role").and_then(|r| r.as_str()),
                        Some("user") | Some("assistant")
                    ));
            if is_visible {
                visible_count += 1;
                if visible_count > keep_entry_count {
                    truncate_at_line = i;
                    break;
                }
            }
        }
    }

    let kept = lines[..truncate_at_line].join("\n") + "\n";
    smol::fs::write(file_path, kept.as_bytes()).await?;
    Ok(())
}
```

## Files to Create/Modify

| Action | File | Purpose |
|--------|------|---------|
| EDIT | `crates/agent_servers/src/acp.rs` | Add `AcpSessionTruncator` struct + `truncate()` impl on `AcpConnection` + `session_file_path()` |
| EDIT | `crates/acp_thread/src/acp_thread.rs` | Add background file truncation in `rewind()` + `truncate_session_file()` function |
| EDIT | `crates/acp_thread/src/connection.rs` | Add `session_file_path()` default method to `AgentConnection` trait |
| EDIT | `crates/agent/src/claude_code_sessions.rs` | Make `sessions_dir_for_project()` public |

## Integration Points

- `AgentSessionTruncate` trait (`connection.rs:130`) — only 2 existing implementations (both test stubs returning `Ok(())`)
- `AgentConnection` trait (`connection.rs:42`) — implemented by `AcpConnection` (production) + `StubAgentConnection` + `FakeAgentConnection` (tests)
- `rewind()` method (`acp_thread.rs:2084`) — called by `regenerate()` in thread_view.rs
- `regenerate()` method (`thread_view.rs:1703`) — called when user edits message and presses enter
- `submit_user_message()` (`acp_thread.rs:1860`) — creates PromptRequest with session_id + message content

## Decisions Made

1. **No-op truncation (not session restart):** The ACP protocol has no truncate RPC. Creating a new session would lose prior context. A no-op truncation keeps the same session (agent has all context including "deleted" messages), but the UI shows the correct truncated view. Practically, the agent focuses on the latest prompt.

2. **Background file modification:** The `.jsonl` file is truncated in a background task so the UI stays responsive. File truncation is for persistence — when the session is reopened from history, the CLI re-reads the truncated file.

3. **Seamless UX:** No close/reopen of the thread pane. Entries below the edit point are removed instantly via `entries.truncate(ix)` + `EntriesRemoved` event. The edited message is sent immediately. No blinking.

4. **Claude Code file path derivation:** `~/.claude/projects/{sanitized-project-path}/{session_id}.jsonl` where sanitized path replaces `/` with `-`. Reuses existing `sessions_dir_for_project()` from `claude_code_sessions.rs`.

5. **Codex file path:** TBD during implementation — may need to store the file path in AcpSession metadata or search the `.codex/sessions/` directory tree.
