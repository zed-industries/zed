# Thread Content Editor - Task Checklist

## Pre-implementation
- [x] Verify DB struct name - `ThreadsDatabase` (pub(crate)). Use `Entity<ThreadStore>` instead.
- [x] Verify `ListItem::end_slot` - `end_slot<E: IntoElement>` works. Need `::<Div>` type annotation.
- [x] Verify how `AcpThreadHistory` accesses the database - emit event to AgentPanel which has `Entity<ThreadStore>`.

## Implementation

### Task 1-6: ThreadContentEditor (all complete)
- [x] Module declaration in `agent_ui.rs`
- [x] Struct, MessageEntry, MessageRole, Event enum
- [x] EventEmitter + Focusable
- [x] `MessageEntry::from_message()` with preview truncation
- [x] Constructor
- [x] `render_toolbar()` with Save/Cancel buttons
- [x] `render_message_row()` with checkbox + right-click menu
- [x] Render trait with uniform_list + vertical_scrollbar_for
- [x] `toggle_message()`, `uncheck_from()`, `save()`
- [x] Item trait (tab_content_text, tab_icon, to_item_events, is_dirty)
- [x] `open()` function

### Task 7: Edit button in thread history panel
- [x] `ThreadHistoryEvent::EditContent(acp::SessionId)` variant
- [x] `edit_thread_content()` method on AcpThreadHistory
- [x] Dual pencil+trash buttons in `render_entry_from_sessions`
- [x] Dual pencil+trash buttons in `AcpHistoryEntryElement::render`

### Task 8: Wire up AgentPanel
- [x] `edit_thread_content()` method on AgentPanel
- [x] Handle `ThreadHistoryEvent::EditContent` in subscription
- [x] Import `agent_client_protocol as acp`

### Task 9: Verify and fix compilation
- [x] `cargo build` passes with zero errors
- [x] Fixed: `agent::thread` is private → use `agent::*` re-exports
- [x] Fixed: `WithScrollbar` trait not in scope
- [x] Fixed: `Tooltip::text()` returns closure, pass directly
- [x] Fixed: `end_slot` needs `::<Div>` type annotation
- [x] Fixed: `cx.update()` returns Task directly in entity spawn context
- [x] Fixed: borrow/move issues with entity clones in closures
- [ ] Build with `cargo run` and test manually
- [ ] Verify: hover shows pencil + trash icons
- [ ] Verify: pencil opens ThreadContentEditor tab
- [ ] Verify: checkboxes toggle, Save enables when dirty
- [ ] Verify: right-click "Delete from here" unchecks messages below
- [ ] Verify: Save writes to DB and closes tab
- [ ] Verify: Cancel closes without saving
- [ ] Verify: reopening thread shows only kept messages
- [ ] Final commit
