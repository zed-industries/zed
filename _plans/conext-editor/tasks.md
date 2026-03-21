# Thread Content Editor - Task Checklist

## Pre-implementation
- [ ] Verify DB struct name (`grep "impl" crates/agent/src/db.rs` to find struct with `load_thread`/`save_thread`)
- [ ] Verify `ListItem::end_slot::<AnyElement>` compiles (or find alternative)
- [ ] Verify how `AcpThreadHistory` accesses the database (trace from `remove_thread`)

## Implementation

### Task 1: Create ThreadContentEditor struct and module
- [ ] Add `pub(crate) mod thread_content_editor;` to `crates/agent_ui/src/agent_ui.rs`
- [ ] Create `crates/agent_ui/src/thread_content_editor.rs` with struct, MessageEntry, MessageRole, Event enum
- [ ] Implement `EventEmitter<Event>` and `Focusable`
- [ ] Commit

### Task 2: Constructor and data loading
- [ ] Implement `MessageEntry::from_message()` (extracts role + preview text, truncates to 200 chars)
- [ ] Implement `ThreadContentEditor::new()` constructor
- [ ] Commit

### Task 3: Render trait (toolbar + message list)
- [ ] Implement `render_toolbar()` - title label, Save button (disabled when not dirty), Cancel button
- [ ] Implement `render_message_row()` - right_click_menu wrapping ListItem with checkbox + role label + preview
- [ ] Implement `Render` trait - v_flex with toolbar + uniform_list with WithScrollbar
- [ ] Commit

### Task 4: Toggle/save/uncheck_from actions
- [ ] Implement `toggle_message(ix)` - flip checkbox, set is_dirty
- [ ] Implement `uncheck_from(ix)` - uncheck ix and everything below
- [ ] Implement `save()` - filter checked messages, load existing DbThread, update messages, save to DB, emit Close
- [ ] Commit

### Task 5: Item trait
- [ ] Implement `Item` for `ThreadContentEditor` (tab_content_text, tab_icon, to_item_events, clone_on_split, is_singleton)
- [ ] Commit

### Task 6: Open function
- [ ] Implement `ThreadContentEditor::open()` - loads from DB, creates entity, adds to workspace active pane
- [ ] Commit

### Task 7: Add edit button to thread history panel
- [ ] Modify `render_entry_from_sessions` (~line 584) - change end_slot from single IconButton to h_flex with pencil + trash
- [ ] Modify `AcpHistoryEntryElement::render` (~line 817) - same dual-button pattern
- [ ] Add `edit_thread_content()` method to AcpThreadHistory
- [ ] Commit

### Task 8: Wire up AgentPanel
- [ ] Add `edit_thread_content()` method to AgentPanel
- [ ] Wire it to call `ThreadContentEditor::open()`
- [ ] Commit

### Task 9: Verify and fix compilation
- [ ] Run `./script/clippy` and fix errors
- [ ] Build with `cargo run` and test manually
- [ ] Verify: hover shows pencil + trash icons
- [ ] Verify: pencil opens ThreadContentEditor tab
- [ ] Verify: checkboxes toggle, Save enables when dirty
- [ ] Verify: right-click "Delete from here" unchecks messages below
- [ ] Verify: Save writes to DB and closes tab
- [ ] Verify: Cancel closes without saving
- [ ] Verify: reopening thread shows only kept messages
- [ ] Final commit
