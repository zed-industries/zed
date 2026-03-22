# Open CLI Sessions in Agent Panel — Implementation Plan

## Status: IN_PROGRESS (planning complete, implementation pending)
## Last Updated: 2026-03-22

## Goal
Add right-click context menu on CLI session history items (Claude Code, Codex) to choose between "Open in Terminal" (current) or "Open in Agent Panel" (new). The agent panel option loads the conversation via ACP streaming — server reads the `.jsonl` file and streams `SessionUpdate` events that populate the thread entries.

## Context

Currently, CLI sessions (Claude Code, Codex) in the history panel ALWAYS open in a terminal via `resume_cli_session()`. There's no way to view or continue them in the agent panel GUI (right dock). The ACP protocol already supports loading sessions via `load_session` → server streams history as `SessionUpdate` events. The infrastructure exists — we just need the routing.

### How Session Loading Already Works

```
connection.load_thread(session_id)
  → AcpThread created with empty entries
  → Session registered in sessions HashMap (BEFORE awaiting RPC — critical for catching streamed events)
  → conn.load_session(LoadSessionRequest) sent to ACP server
  → Server reads {session_id}.jsonl from disk
  → Server streams SessionUpdate events through the ACP event stream:
      UserMessageChunk → thread.push_user_content_block()
      AgentMessageChunk → thread.push_assistant_content_block()
      AgentThoughtChunk → thread.push_assistant_content_block(is_thought=true)
      ToolCall → thread.upsert_tool_call()
      ToolCallUpdate → thread.update_tool_call()
      Plan → thread.update_plan()
  → AcpThread.entries populated
  → AcpThreadEvent::NewEntry emitted per entry
  → thread_view's handle_thread_event syncs entries to UI
```

This flow is already proven — it's how non-CLI ACP sessions load. The only reason CLI sessions don't use it is the routing logic in `agent_panel.rs`.

### Current Routing Logic (agent_panel.rs:796-822)

```rust
ThreadHistoryEvent::Open(thread) => {
    let cli_source = thread.meta.as_ref().and_then(|m| {
        let cli = m.get(CLI_SOURCE_KEY)?.as_str()?;
        let path = m.get(CLI_PROJECT_PATH_KEY)?.as_str()?;
        Some((cli.to_string(), std::path::PathBuf::from(path)))
    });

    if let Some((cli_command, project_path)) = cli_source {
        // CLI sessions → ALWAYS terminal
        this.resume_cli_session(&cli_command, &thread.session_id.0, &project_path, window, cx);
    } else {
        // Non-CLI sessions → agent panel GUI
        this.external_thread(Some(ExternalAgent::NativeAgent), Some(thread.clone()), None, window, cx);
    }
}
```

## Technical Approach

### Step 1: Add right-click context menu to history items

**File:** `crates/agent_ui/src/acp/thread_history.rs`

Add a context menu on each session row in `render_entry()`. The menu should have two options for CLI sessions:
- "Open in Terminal" — emits existing `ThreadHistoryEvent::Open`
- "Open in Agent Panel" — emits new `ThreadHistoryEvent::OpenInPanel`

For non-CLI sessions, the context menu could have:
- "Open" — emits `ThreadHistoryEvent::Open`
- "Edit Content" — emits existing `ThreadHistoryEvent::EditContent`
- "Delete" — emits existing `ThreadHistoryEvent::Delete`

**Implementation pattern:** Use the existing `right_click_menu` utility from `ui` crate. Look at how context menus are used elsewhere in the codebase for the pattern.

```rust
// In render_entry(), wrap the row element with right_click_menu:
.child(
    right_click_menu(element_id)
        .menu(move |window, cx| {
            let menu = ContextMenu::build(window, cx, |menu, _, _| {
                if is_cli_session {
                    menu.entry("Open in Terminal", None, move |window, cx| {
                        // emit ThreadHistoryEvent::Open
                    })
                    .entry("Open in Agent Panel", None, move |window, cx| {
                        // emit ThreadHistoryEvent::OpenInPanel
                    })
                } else {
                    menu.entry("Open", None, move |window, cx| {
                        // emit ThreadHistoryEvent::Open
                    })
                }
            });
            Some(menu)
        })
        .child(existing_row_content)
)
```

### Step 2: Add new event variant

**File:** `crates/agent_ui/src/acp/thread_history.rs`

Add to `ThreadHistoryEvent` enum:
```rust
pub enum ThreadHistoryEvent {
    Open(AgentSessionInfo),
    OpenInPanel(AgentSessionInfo),  // NEW
    EditContent(acp::SessionId),
    Delete(acp::SessionId),
    // ... other variants
}
```

### Step 3: Handle new event in agent_panel.rs

**File:** `crates/agent_ui/src/agent_panel.rs`

In the `ThreadHistoryEvent` handler (around line 796), add:

```rust
ThreadHistoryEvent::OpenInPanel(thread) => {
    // Determine which ExternalAgent type based on CLI source metadata
    let agent_type = thread.meta.as_ref().and_then(|m| {
        let cli = m.get(CLI_SOURCE_KEY)?.as_str()?;
        if cli.contains("codex") {
            Some(ExternalAgent::Codex)
        } else {
            Some(ExternalAgent::ClaudeCode)
        }
    }).unwrap_or(ExternalAgent::ClaudeCode);

    this.external_thread(
        Some(agent_type),
        Some(thread.clone()),
        None,
        window,
        cx,
    );
}
```

This routes the session through `external_thread()` → `AcpThreadView::new()` → `initial_state()` → `load_thread()` → `load_session()` → server streams history.

### Step 4: Verify load_thread handles CLI session IDs

The `load_thread` implementation in `acp.rs` calls `conn.load_session(LoadSessionRequest::new(session_id, cwd))`. The ACP server (Claude Code CLI) should recognize the session_id and find the corresponding `.jsonl` file. This should work because:
- Claude Code stores sessions as `{session_id}.jsonl` in the project directory
- The same CLI process that created the session is still running (or will be started by `connect()`)

For Codex, the session_id to filename mapping is different — may need testing.

## Files to Create/Modify

| Action | File | Purpose |
|--------|------|---------|
| EDIT | `crates/agent_ui/src/acp/thread_history.rs` | Add right-click context menu, new event variant |
| EDIT | `crates/agent_ui/src/agent_panel.rs` | Handle `OpenInPanel` event, route to `external_thread()` |

## Integration Points

- `ThreadHistoryEvent` enum — consumed by `agent_panel.rs` via `cx.subscribe`
- `external_thread()` method on `AgentPanel` — already handles loading sessions via ACP
- `ExternalAgent` enum (`agent_ui.rs:161`) — `ClaudeCode`, `Codex`, `NativeAgent`, `Gemini`, `Custom`
- `CLI_SOURCE_KEY` / `CLI_PROJECT_PATH_KEY` constants — used to identify CLI sessions in metadata
- `right_click_menu` from `ui` crate — UI primitive for context menus

## Decisions Made

1. **Right-click menu (not changing default click):** Double-click keeps current behavior (terminal). This is least disruptive. Users who want the agent panel use right-click.

2. **Route through existing `external_thread()`:** No new loading mechanism needed. The ACP streaming protocol already handles history. Just need to route CLI sessions to it.

3. **Use `ExternalAgent` variants:** The `external_thread()` method determines which CLI to start/connect to based on `ExternalAgent`. For CLI sessions, we derive this from the session metadata.

4. **Server-driven loading only:** No client-side `.jsonl` parsing. The ACP server reads the file and streams events. This keeps the client simple and uses the battle-tested streaming path.
