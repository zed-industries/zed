# Session: Thread Content Editor - GUI for editing saved agent conversations

Date: 2026-03-21

## Goal

Build a GUI editor that lets users selectively remove messages from saved agent conversation threads. Users click an edit button in the thread history panel (left dock), which opens a new tab in the center pane showing all messages with checkboxes. They can uncheck messages to remove them, use "Delete from here" to truncate, then Save to write back to the database.

## Background Research Completed

### How threads are stored
- **Location**: `~/Library/Application Support/Zed/threads/threads.db` (SQLite)
- **Format**: Each thread is a Zstd-compressed JSON blob in the `data` column
- **Schema**: `id TEXT PRIMARY KEY, summary TEXT, updated_at TEXT, data_type TEXT, data BLOB`
- **Version**: DbThread version `"0.3.0"`

### Key data structures
- **`DbThread`** (`crates/agent/src/db.rs:35`): `title`, `messages: Vec<DbMessage>`, `updated_at`, `detailed_summary`, `cumulative_token_usage`, `request_token_usage`, `model`, `completion_mode`, `profile`, `imported`
- **`DbMessage`** is alias for `crate::Message` (`crates/agent/src/db.rs:22`)
- **`Message`** enum (`crates/agent/src/thread.rs:117`): `User(UserMessage)`, `Agent(AgentMessage)`, `Resume`
- **`UserMessage`** (`thread.rs`): `id: UserMessageId`, `content: Vec<UserMessageContent>`
- **`UserMessageContent`**: `Text(String)`, `Mention { uri, content }`, `Image(...)`
- **`AgentMessage`** (`thread.rs`): `content: Vec<AgentMessageContent>`, `tool_results`, `reasoning_details`
- **`AgentMessageContent`**: `Text(String)`, `Thinking { text, signature }`, `RedactedThinking(String)`, `ToolUse(LanguageModelToolUse)`

### DB access methods
- `db.load_thread(id: acp::SessionId) -> Task<Result<Option<DbThread>>>` (`db.rs:437`)
- `db.save_thread(id: acp::SessionId, thread: DbThread) -> Task<Result<()>>` (`db.rs:463`)
- Compression: Zstd level 3 on save, auto-decompressed on load

### Why the existing edit/rewind doesn't work for production agents
- `AcpConnection` (`crates/agent_servers/src/acp.rs:269`) doesn't implement `truncate()` - uses default which returns `None`
- This causes `message.id` to be `None` for all production agents (`acp_thread.rs:1871`)
- Which makes the pencil icon show "Editing unavailable" (`thread_view.rs:2530-2556`)
- The ACP protocol (`agent-client-protocol` v0.9.3, external crate) has no truncate/rewind method
- Protocol has `fork_session` (unstable) but it forks the ENTIRE session, not at a specific point
- **This thread content editor is a workaround** - edit the saved data directly

## Design Decisions (User-approved)

1. **Destructive replace** (not forking) - user chose simpler approach
2. **Simplified message view** with checkboxes - not raw JSON or markdown
3. **Checkbox per message** - checked = keep, unchecked = remove on save
4. **"Delete from here" action** - right-click context menu unchecks a message and everything below
5. **Center editor pane** - opens as a new tab like any file
6. **Save closes the tab** - prevents accidental overwrites

## Files Changed

### Created
- `docs/plans/2026-03-21-thread-editor-design.md` - Full implementation plan with 9 tasks, exact code snippets, and file paths

### Not Yet Modified (planned)
These files need to be created/modified during implementation:

## Implementation Plan (9 Tasks)

### Task 1: Create ThreadContentEditor struct and module
- **Create**: `crates/agent_ui/src/thread_content_editor.rs`
- **Modify**: `crates/agent_ui/src/agent_ui.rs` - add `pub(crate) mod thread_content_editor;` after line 20

The struct needs:
```rust
pub struct ThreadContentEditor {
    thread_id: acp::SessionId,
    title: SharedString,
    messages: Vec<MessageEntry>,       // Each has: message, checked: bool, preview: String, role
    scroll_handle: UniformListScrollHandle,
    focus: FocusHandle,
    db: std::sync::Arc<ThreadDatabase>,  // NOTE: verify exact type - might just be the db.rs struct
    workspace: WeakEntity<Workspace>,
    is_dirty: bool,
}
```

Events: `pub enum Event { Close }` -> maps to `ItemEvent::CloseItem`

### Task 2: Constructor and data loading
- `MessageEntry::from_message(Message) -> Self` - extracts role and text preview (truncated to ~200 chars)
- `ThreadContentEditor::new(thread_id, db_thread, db, workspace, cx)` - creates from loaded DbThread

### Task 3: Render trait (toolbar + message list)
- Toolbar: `h_flex` with title label, spacer, Save button (disabled when not dirty), Cancel button
- Message list: `uniform_list` with `cx.processor` callback
- Each row: `right_click_menu` wrapping a `ListItem` with checkbox + role label + preview text
- Context menu: "Delete from here" entry

### Task 4: Toggle/save/uncheck_from actions
- `toggle_message(ix)` - flips checkbox, sets `is_dirty = true`
- `uncheck_from(ix)` - unchecks ix and everything below
- `save()` - filters to checked messages, loads existing DbThread to preserve metadata, saves back to SQLite, emits `Event::Close`

### Task 5: Item trait implementation
- `tab_content_text` -> `"Edit: {title}"`
- `tab_icon` -> `IconName::Pencil`
- `to_item_events` -> `Event::Close` maps to `ItemEvent::CloseItem`
- `clone_on_split` -> `Task::ready(None)` (singleton)

### Task 6: Open function
- `ThreadContentEditor::open(thread_id, db, workspace, window, cx) -> Task<Result<()>>`
- Loads thread from DB, creates the editor entity, adds to active pane via `workspace.add_item_to_active_pane()`

### Task 7: Add edit button to thread history panel
- **Modify**: `crates/agent_ui/src/acp/thread_history.rs`
- Two places need changes:
  1. `render_entry_from_sessions` (line ~584): Change `.end_slot::<IconButton>` to `.end_slot::<AnyElement>` with `h_flex` containing both pencil and trash buttons
  2. `AcpHistoryEntryElement::render` (line ~817): Same dual-button pattern
- Add `edit_thread_content(ix, window, cx)` method

### Task 8: Wire up AgentPanel
- **Modify**: `crates/agent_ui/src/agent_panel.rs`
- Add `edit_thread_content(session_id, window, cx)` method
- Gets DB from thread store, workspace handle, calls `ThreadContentEditor::open()`
- **IMPORTANT**: Need to verify how to access the thread database. Check if `ThreadStore` has a `database()` method or if DB is accessed differently. The DB struct in `crates/agent/src/db.rs` doesn't have a name like "ThreadDatabase" - look for the actual struct name and how it's accessed.

### Task 9: Verify compilation and test

## Critical Implementation Details

### GPUI patterns to use
- **Checkbox**: `Checkbox::new(id, ToggleState::Selected/Unselected).on_click(handler)` - from `crates/ui/src/components/toggle.rs`
- **uniform_list**: `uniform_list("id", count, cx.processor(|this, range, window, cx| vec_of_elements))` - see `thread_history.rs` for example
- **right_click_menu**: `right_click_menu("id").trigger(|_,_,_| element).menu(|window, cx| ContextMenu::build(...))` - see `thread_view.rs` for example
- **Item trait**: Must implement `EventEmitter<Event>`, `Focusable`, `Render`, and `Item` - see `workspace/src/shared_screen.rs` for simple example
- **Adding to workspace**: `workspace.add_item_to_active_pane(Box::new(entity), None, true, window, cx)` - see `thread_view.rs:5886` (`open_thread_as_markdown`)

### Import paths (verified)
- `agent::db::DbThread` - the thread data struct
- `agent::thread::Message` (also aliased as `agent::db::DbMessage`) - the message enum
- `agent::thread::UserMessage`, `agent::thread::UserMessageContent`
- `agent::thread::AgentMessage`, `agent::thread::AgentMessageContent`
- `agent_client_protocol as acp` - for `acp::SessionId`
- `ui::Checkbox` / `ui::ToggleState` - checkbox component
- `workspace::item::{Item, ItemEvent}` - workspace item trait

### DB struct name - NEEDS VERIFICATION
The database struct in `crates/agent/src/db.rs` - search for the struct that has `load_thread` and `save_thread` methods. It's NOT called "ThreadDatabase". Find the actual name before implementing.

```bash
grep "pub struct" crates/agent/src/db.rs
# Will show: DbThreadMetadata, DbThread, SharedThread
# The struct with load_thread/save_thread methods needs to be found
```

Look at line ~330-370 of `db.rs` for the struct that owns the SQLite connection.

### ListItem end_slot type
Currently uses `.end_slot::<IconButton>(...)`. To add multiple buttons, change to `.end_slot::<AnyElement>(...)` wrapping an `h_flex()`. **Verify this compiles** - the `ListItem` generic parameter might need a specific trait bound.

## Current Status
- Full design doc written and saved at `docs/plans/2026-03-21-thread-editor-design.md`
- Plan file also at `/Users/alesloas/.claude/plans/jiggly-sparking-sutton.md` (the earlier feasibility analysis)
- NO code has been written yet - implementation hasn't started
- All research and design is complete
- User has approved the design

## Notes for Next Session
- Start with Task 1 from the implementation plan
- First thing: verify the DB struct name (`grep "impl.*{" crates/agent/src/db.rs | head -20`)
- Verify `end_slot::<AnyElement>` works on `ListItem` before committing to that approach
- The `agent_ui` crate already depends on `agent` crate (see Cargo.toml line 23: `agent.workspace = true`)
- Run `./script/clippy` instead of `cargo clippy` per CLAUDE.md
- Don't use `--release` flag per CLAUDE.md
- Test with `cargo run` (which runs Zed in dev mode)
