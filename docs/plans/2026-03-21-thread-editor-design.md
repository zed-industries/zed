# Thread Content Editor Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add a GUI editor that lets users selectively remove messages from saved agent conversation threads.

**Architecture:** A new `ThreadContentEditor` workspace item (opens as a tab in the center pane) that loads a thread from SQLite, displays messages as a scrollable list with checkboxes, and saves the filtered thread back to the database. Entry point is a new edit button on each row in the thread history panel.

**Tech Stack:** GPUI (Zed's UI framework), SQLite + Zstd (thread storage), existing `agent` and `agent_ui` crates.

---

### Task 1: Create ThreadContentEditor struct and module

**Files:**
- Create: `crates/agent_ui/src/thread_content_editor.rs`
- Modify: `crates/agent_ui/src/agent_ui.rs` (add `mod thread_content_editor;`)

**Step 1: Add module declaration**

In `crates/agent_ui/src/agent_ui.rs`, add after `mod text_thread_editor;` (line 20):
```rust
mod thread_content_editor;
```

**Step 2: Create the struct with basic state**

Create `crates/agent_ui/src/thread_content_editor.rs`:

```rust
use agent::db::{DbThread, ThreadDatabase};
use agent::thread::Message;
use agent_client_protocol as acp;
use gpui::{
    App, Context, Entity, EventEmitter, FocusHandle, Focusable, Render, Task,
    UniformListScrollHandle, WeakEntity, Window, uniform_list,
};
use std::ops::Range;
use ui::{
    prelude::*, Button, ButtonStyle, Checkbox, ContextMenu, ContextMenuEntry,
    IconButton, IconButtonShape, IconName, Label, LabelSize, ListItem, ListItemSpacing,
    Tooltip, WithScrollbar, right_click_menu,
};
use workspace::item::{Item, ItemEvent};
use workspace::Workspace;

pub struct ThreadContentEditor {
    thread_id: acp::SessionId,
    title: SharedString,
    messages: Vec<MessageEntry>,
    scroll_handle: UniformListScrollHandle,
    focus: FocusHandle,
    db: std::sync::Arc<ThreadDatabase>,
    workspace: WeakEntity<Workspace>,
    is_dirty: bool,
}

struct MessageEntry {
    message: Message,
    checked: bool,
    preview: String,
    role: MessageRole,
}

#[derive(Clone, Copy)]
enum MessageRole {
    User,
    Agent,
    Resume,
}

pub enum Event {
    Close,
}

impl EventEmitter<Event> for ThreadContentEditor {}

impl Focusable for ThreadContentEditor {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus.clone()
    }
}
```

**Step 3: Commit**
```bash
git add crates/agent_ui/src/thread_content_editor.rs crates/agent_ui/src/agent_ui.rs
git commit -m "feat: add ThreadContentEditor struct skeleton"
```

---

### Task 2: Implement constructor and data loading

**Files:**
- Modify: `crates/agent_ui/src/thread_content_editor.rs`

**Step 1: Implement message preview extraction**

```rust
impl MessageEntry {
    fn from_message(message: Message) -> Self {
        let (role, preview) = match &message {
            Message::User(user_msg) => {
                let text = user_msg
                    .content
                    .iter()
                    .filter_map(|c| match c {
                        agent::thread::UserMessageContent::Text(t) => Some(t.as_str()),
                        agent::thread::UserMessageContent::Mention { content, .. } => {
                            Some(content.as_str())
                        }
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
                (MessageRole::User, text)
            }
            Message::Agent(agent_msg) => {
                let mut parts = Vec::new();
                for content in &agent_msg.content {
                    match content {
                        agent::thread::AgentMessageContent::Text(t) => parts.push(t.clone()),
                        agent::thread::AgentMessageContent::ToolUse(tool) => {
                            parts.push(format!("[Tool: {}]", tool.name));
                        }
                        _ => {}
                    }
                }
                (MessageRole::Agent, parts.join(" "))
            }
            Message::Resume => (MessageRole::Resume, "Resume".to_string()),
        };

        // Truncate preview to ~200 chars
        let preview = if preview.len() > 200 {
            format!("{}...", &preview[..200])
        } else {
            preview
        };

        Self {
            message,
            checked: true,
            preview,
            role,
        }
    }
}
```

**Step 2: Implement constructor**

```rust
impl ThreadContentEditor {
    pub fn new(
        thread_id: acp::SessionId,
        db_thread: DbThread,
        db: std::sync::Arc<ThreadDatabase>,
        workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> Self {
        let title = db_thread.title.clone();
        let messages = db_thread
            .messages
            .into_iter()
            .map(MessageEntry::from_message)
            .collect();

        Self {
            thread_id,
            title,
            messages,
            scroll_handle: UniformListScrollHandle::new(),
            focus: cx.focus_handle(),
            db,
            workspace,
            is_dirty: false,
        }
    }
}
```

**Step 3: Commit**
```bash
git add crates/agent_ui/src/thread_content_editor.rs
git commit -m "feat: add ThreadContentEditor constructor and data loading"
```

---

### Task 3: Implement Render trait (toolbar + message list)

**Files:**
- Modify: `crates/agent_ui/src/thread_content_editor.rs`

**Step 1: Implement toolbar rendering**

```rust
impl ThreadContentEditor {
    fn render_toolbar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .w_full()
            .p_2()
            .gap_2()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().editor_background)
            .child(
                Label::new(format!("Edit: {}", self.title))
                    .size(LabelSize::Small)
                    .color(Color::Default),
            )
            .child(div().flex_grow())
            .child(
                Button::new("save", "Save")
                    .style(ButtonStyle::Filled)
                    .disabled(!self.is_dirty)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.save(cx);
                    })),
            )
            .child(
                Button::new("cancel", "Cancel")
                    .style(ButtonStyle::Subtle)
                    .on_click(cx.listener(|this, _, _, cx| {
                        cx.emit(Event::Close);
                    })),
            )
    }

    fn render_message_row(
        &self,
        ix: usize,
        entry: &MessageEntry,
        cx: &Context<Self>,
    ) -> AnyElement {
        let role_label = match entry.role {
            MessageRole::User => "USER",
            MessageRole::Agent => "AGENT",
            MessageRole::Resume => "RESUME",
        };

        let role_color = match entry.role {
            MessageRole::User => Color::Accent,
            MessageRole::Agent => Color::Success,
            MessageRole::Resume => Color::Muted,
        };

        let checked = entry.checked;
        let toggle_state = if checked {
            ToggleState::Selected
        } else {
            ToggleState::Unselected
        };

        let entity = cx.entity();

        right_click_menu(format!("msg-ctx-{}", ix))
            .trigger(move |_, _, _| {
                ListItem::new(("msg", ix))
                    .spacing(ListItemSpacing::Sparse)
                    .start_slot(
                        h_flex()
                            .gap_2()
                            .items_start()
                            .child(
                                Checkbox::new(("check", ix), toggle_state).on_click({
                                    let entity = entity.clone();
                                    move |_state, _window, cx| {
                                        entity.update(cx, |this, cx| {
                                            this.toggle_message(ix, cx);
                                        });
                                    }
                                }),
                            )
                            .child(
                                v_flex()
                                    .child(
                                        Label::new(format!("{} [{}]", role_label, ix + 1))
                                            .size(LabelSize::XSmall)
                                            .color(role_color),
                                    )
                                    .child(
                                        Label::new(entry.preview.clone())
                                            .size(LabelSize::Small)
                                            .color(if checked {
                                                Color::Default
                                            } else {
                                                Color::Muted
                                            }),
                                    ),
                            ),
                    )
                    .into_any_element()
            })
            .menu({
                let entity = entity.clone();
                move |window, cx| {
                    ContextMenu::build(window, cx, |menu, _, _cx| {
                        menu.item(
                            ContextMenuEntry::new("Delete from here").handler({
                                let entity = entity.clone();
                                move |_window, cx| {
                                    entity.update(cx, |this, cx| {
                                        this.uncheck_from(ix, cx);
                                    });
                                }
                            }),
                        )
                    })
                }
            })
            .into_any_element()
    }
}
```

**Step 2: Implement Render trait**

```rust
impl Render for ThreadContentEditor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let message_count = self.messages.len();

        v_flex()
            .key_context("ThreadContentEditor")
            .track_focus(&self.focus)
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(self.render_toolbar(cx))
            .child(
                div()
                    .flex_grow()
                    .child(
                        WithScrollbar::new(
                            uniform_list(
                                "thread-messages",
                                message_count,
                                cx.processor(
                                    |this: &mut Self, range: Range<usize>, _window, cx| {
                                        range
                                            .into_iter()
                                            .map(|ix| {
                                                let entry = &this.messages[ix];
                                                this.render_message_row(ix, entry, cx)
                                            })
                                            .collect()
                                    },
                                ),
                            )
                            .p_2()
                            .flex_grow()
                            .track_scroll(&self.scroll_handle),
                            &self.scroll_handle,
                            window,
                            cx,
                        ),
                    ),
            )
    }
}
```

**Step 3: Commit**
```bash
git add crates/agent_ui/src/thread_content_editor.rs
git commit -m "feat: implement ThreadContentEditor rendering with checkboxes"
```

---

### Task 4: Implement toggle/save/uncheck_from actions

**Files:**
- Modify: `crates/agent_ui/src/thread_content_editor.rs`

**Step 1: Implement actions**

```rust
impl ThreadContentEditor {
    fn toggle_message(&mut self, ix: usize, cx: &mut Context<Self>) {
        if let Some(entry) = self.messages.get_mut(ix) {
            entry.checked = !entry.checked;
            self.is_dirty = true;
            cx.notify();
        }
    }

    fn uncheck_from(&mut self, ix: usize, cx: &mut Context<Self>) {
        for i in ix..self.messages.len() {
            self.messages[i].checked = false;
        }
        self.is_dirty = true;
        cx.notify();
    }

    fn save(&mut self, cx: &mut Context<Self>) {
        let kept_messages: Vec<Message> = self
            .messages
            .iter()
            .filter(|e| e.checked)
            .map(|e| e.message.clone())
            .collect();

        if kept_messages.is_empty() {
            return;
        }

        let thread_id = self.thread_id.clone();
        let db = self.db.clone();

        // Load the existing thread to preserve non-message fields
        let load_task = db.load_thread(thread_id.clone());

        cx.spawn(async move |this, cx| {
            let existing = load_task.await?;
            let Some(mut db_thread) = existing else {
                anyhow::bail!("Thread not found in database");
            };

            db_thread.messages = kept_messages;
            db_thread.updated_at = chrono::Utc::now();

            db.save_thread(thread_id, db_thread).await?;

            this.update(cx, |_, cx| {
                cx.emit(Event::Close);
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }
}
```

**Step 2: Commit**
```bash
git add crates/agent_ui/src/thread_content_editor.rs
git commit -m "feat: implement toggle, uncheck_from, and save actions"
```

---

### Task 5: Implement Item trait

**Files:**
- Modify: `crates/agent_ui/src/thread_content_editor.rs`

**Step 1: Implement Item**

```rust
impl Item for ThreadContentEditor {
    type Event = Event;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        format!("Edit: {}", self.title).into()
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<ui::Icon> {
        Some(ui::Icon::new(IconName::Pencil))
    }

    fn tab_tooltip_text(&self, _cx: &App) -> Option<SharedString> {
        Some(format!("Editing thread: {}", self.title).into())
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(ItemEvent)) {
        match event {
            Event::Close => f(ItemEvent::CloseItem),
        }
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<workspace::WorkspaceId>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>> {
        Task::ready(None)
    }

    fn is_singleton(&self, _cx: &App) -> bool {
        true
    }
}
```

**Step 2: Commit**
```bash
git add crates/agent_ui/src/thread_content_editor.rs
git commit -m "feat: implement Item trait for ThreadContentEditor"
```

---

### Task 6: Add open_thread_editor function

**Files:**
- Modify: `crates/agent_ui/src/thread_content_editor.rs`
- Modify: `crates/agent_ui/src/agent_ui.rs` (make module public)

**Step 1: Change module visibility**

In `crates/agent_ui/src/agent_ui.rs`, change:
```rust
mod thread_content_editor;
```
to:
```rust
pub(crate) mod thread_content_editor;
```

**Step 2: Add the open function**

In `thread_content_editor.rs`:

```rust
impl ThreadContentEditor {
    pub fn open(
        thread_id: acp::SessionId,
        db: std::sync::Arc<ThreadDatabase>,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<anyhow::Result<()>> {
        let load_task = db.load_thread(thread_id.clone());
        let weak_workspace = workspace.downgrade();

        window.spawn(cx, async move |cx| {
            let db_thread = load_task.await?;
            let Some(db_thread) = db_thread else {
                anyhow::bail!("Thread not found");
            };

            cx.update(|window, cx| {
                let workspace_handle = weak_workspace.upgrade().context("workspace dropped")?;
                workspace_handle.update(cx, |workspace, cx| {
                    let editor = cx.new(|cx| {
                        ThreadContentEditor::new(
                            thread_id,
                            db_thread,
                            db,
                            workspace.weak_handle(),
                            cx,
                        )
                    });
                    workspace.add_item_to_active_pane(
                        Box::new(editor),
                        None,
                        true,
                        window,
                        cx,
                    );
                });
                anyhow::Ok(())
            })?
        })
    }
}
```

**Step 3: Commit**
```bash
git add crates/agent_ui/src/thread_content_editor.rs crates/agent_ui/src/agent_ui.rs
git commit -m "feat: add open function to ThreadContentEditor"
```

---

### Task 7: Add edit button to thread history panel

**Files:**
- Modify: `crates/agent_ui/src/acp/thread_history.rs`

**Step 1: Change end_slot to support multiple buttons**

In `render_entry_from_sessions` (around line 584), change the single trash button `end_slot` to an `h_flex` with both edit and trash buttons. Replace:
```rust
.end_slot::<IconButton>(if hovered {
    Some(
        IconButton::new("delete", IconName::Trash)
            // ...
    )
} else {
    None
})
```

With:
```rust
.end_slot::<AnyElement>(if hovered {
    Some(
        h_flex()
            .gap_0p5()
            .child(
                IconButton::new("edit-content", IconName::Pencil)
                    .shape(IconButtonShape::Square)
                    .icon_size(IconSize::XSmall)
                    .icon_color(Color::Muted)
                    .tooltip(|_window, _cx| Tooltip::text("Edit Thread Content"))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.edit_thread_content(ix, window, cx);
                        cx.stop_propagation()
                    })),
            )
            .child(
                IconButton::new("delete", IconName::Trash)
                    .shape(IconButtonShape::Square)
                    .icon_size(IconSize::XSmall)
                    .icon_color(Color::Muted)
                    .tooltip(move |_window, cx| {
                        Tooltip::for_action("Delete", &RemoveSelectedThread, cx)
                    })
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.remove_thread(ix, cx);
                        cx.stop_propagation()
                    })),
            )
            .into_any_element()
    )
} else {
    None
})
```

**Step 2: Add edit_thread_content method to AcpThreadHistory**

```rust
fn edit_thread_content(&self, visible_item_ix: usize, window: &mut Window, cx: &mut App) {
    let Some(entry) = self.get_history_entry(visible_item_ix) else {
        return;
    };
    let session_id = entry.session_id.clone();

    // Get the ThreadDatabase and workspace from the agent panel context
    // This will need to be wired through the AgentPanel
    // (implementation details depend on how AgentPanel exposes the DB)
}
```

Note: The exact wiring depends on how `AcpThreadHistory` accesses the `ThreadDatabase` and `Workspace`. This may require passing these through from `AgentPanel`. Check how `remove_thread` gets its dependencies.

**Step 3: Do the same for `AcpHistoryEntryElement::render` (line 817)**

Apply the same dual-button pattern to the `AcpHistoryEntryElement` component.

**Step 4: Commit**
```bash
git add crates/agent_ui/src/acp/thread_history.rs
git commit -m "feat: add edit button to thread history rows"
```

---

### Task 8: Wire up AgentPanel to open ThreadContentEditor

**Files:**
- Modify: `crates/agent_ui/src/agent_panel.rs`

**Step 1: Add method to AgentPanel to open the thread editor**

The agent panel needs a method that:
1. Gets the `ThreadDatabase` from the thread store
2. Gets the workspace handle
3. Calls `ThreadContentEditor::open()`

Look at how `AgentPanel::load_agent_thread()` works for the pattern of accessing thread data and opening items.

```rust
pub fn edit_thread_content(
    &self,
    session_id: acp::SessionId,
    window: &mut Window,
    cx: &mut App,
) {
    let Some(workspace) = self.workspace.upgrade() else {
        return;
    };
    let db = self.thread_store.read(cx).database().clone();

    ThreadContentEditor::open(session_id, db, workspace, window, cx)
        .detach_and_log_err(cx);
}
```

Note: Verify that `ThreadStore` exposes a `database()` method. If not, add one or access the DB directly from the `agent::db` module.

**Step 2: Commit**
```bash
git add crates/agent_ui/src/agent_panel.rs
git commit -m "feat: wire AgentPanel to open ThreadContentEditor"
```

---

### Task 9: Verify and fix compilation

**Step 1: Run clippy to catch issues**

```bash
./script/clippy
```

**Step 2: Fix any compilation errors**

Common issues to watch for:
- Import paths for `ThreadDatabase` - check exact path in `crates/agent/src/db.rs`
- Import paths for `Message` types - check `crates/agent/src/thread.rs`
- `ToggleState` import from `ui` crate
- `AnyElement` type parameter for `end_slot` - verify `ListItem` accepts it
- `WithScrollbar` usage - check exact API
- `cx.processor` vs closure syntax for `uniform_list`

**Step 3: Build and test**

```bash
cargo run
```

- Open agent panel
- Open thread history
- Hover over a thread row → should see pencil + trash icons
- Click pencil → should open ThreadContentEditor tab
- Verify checkboxes render, toggle works
- Right-click → "Delete from here" should uncheck messages below
- Save should write back and close tab

**Step 4: Commit fixes**
```bash
git add -A
git commit -m "fix: resolve compilation issues in ThreadContentEditor"
```

---

## Verification

1. Open Zed with `cargo run`
2. Open the agent panel and go to thread history
3. Hover over a saved thread → see edit (pencil) and delete (trash) buttons
4. Click the pencil → ThreadContentEditor opens as a new tab
5. All messages shown with checkboxes (all checked by default)
6. Toggle individual checkboxes → Save button becomes enabled
7. Right-click a message → "Delete from here" unchecks it and everything below
8. Click Save → thread saved with only checked messages, tab closes
9. Reopen the thread from history → only the kept messages appear
10. Click Cancel → tab closes without saving, original thread unchanged
