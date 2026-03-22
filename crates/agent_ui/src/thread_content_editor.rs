use anyhow::Context as _;
use gpui::{
    App, Context, Entity, EventEmitter, FocusHandle, Focusable, Render, Task,
    UniformListScrollHandle, Window,
};
use std::ops::Range;
use std::path::PathBuf;
use ui::{
    prelude::*, Button, ButtonStyle, Checkbox, ContextMenu, ContextMenuEntry, ListItem,
    ListItemSpacing, WithScrollbar, right_click_menu,
};
use workspace::Workspace;
use workspace::item::{Item, ItemEvent};

pub struct ThreadContentEditor {
    file_path: PathBuf,
    title: SharedString,
    entries: Vec<JsonlEntry>,
    visible_indices: Vec<usize>,
    scroll_handle: UniformListScrollHandle,
    focus: FocusHandle,
    is_dirty: bool,
}

struct JsonlEntry {
    raw_line: String,
    checked: bool,
    preview: SharedString,
    entry_type: EntryType,
}

#[derive(Clone, Copy, PartialEq)]
enum EntryType {
    User,
    Assistant,
    Progress,
    System,
    Other,
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

fn is_system_content(value: &serde_json::Value) -> bool {
    if let Some(content) = value.pointer("/message/content").and_then(|c| c.as_str()) {
        if content.contains("<environment_context>")
            || content.contains("<INSTRUCTIONS>")
            || content.contains("SKILL.md")
            || content.contains("<system-reminder>")
        {
            return true;
        }
    }
    if let Some(arr) = value.pointer("/message/content").and_then(|c| c.as_array()) {
        for item in arr {
            if let Some(text) = item.get("text").and_then(|t| t.as_str()) {
                if text.contains("<environment_context>")
                    || text.contains("<INSTRUCTIONS>")
                    || text.contains("SKILL.md")
                    || text.contains("<system-reminder>")
                {
                    return true;
                }
            }
        }
    }
    if let Some(arr) = value.pointer("/payload/content").and_then(|c| c.as_array()) {
        for item in arr {
            if let Some(text) = item.get("text").and_then(|t| t.as_str()) {
                if text.contains("<environment_context>")
                    || text.contains("<INSTRUCTIONS>")
                    || text.contains("SKILL.md")
                    || text.contains("<system-reminder>")
                {
                    return true;
                }
            }
        }
    }
    false
}

fn extract_codex_content(value: &serde_json::Value) -> String {
    let Some(content) = value.pointer("/payload/content") else {
        return String::new();
    };

    if let Some(arr) = content.as_array() {
        let parts: Vec<String> = arr
            .iter()
            .filter_map(|item| {
                let item_type = item.get("type").and_then(|t| t.as_str()).unwrap_or("");
                match item_type {
                    "output_text" | "input_text" => {
                        item.get("text").and_then(|t| t.as_str()).map(|s| s.to_string())
                    }
                    _ => None,
                }
            })
            .collect();
        return parts.join(" ");
    }

    String::new()
}

fn extract_content_text(value: &serde_json::Value) -> String {
    let Some(content) = value.pointer("/message/content") else {
        return String::new();
    };

    if let Some(s) = content.as_str() {
        return s.to_string();
    }

    if let Some(arr) = content.as_array() {
        let parts: Vec<String> = arr
            .iter()
            .filter_map(|item| {
                let item_type = item.get("type").and_then(|t| t.as_str()).unwrap_or("");
                match item_type {
                    "text" => item.get("text").and_then(|t| t.as_str()).map(|s| s.to_string()),
                    _ => None,
                }
            })
            .collect();
        return parts.join(" ");
    }

    String::new()
}

impl JsonlEntry {
    fn from_line(line: String) -> Self {
        let (entry_type, preview_text) = match serde_json::from_str::<serde_json::Value>(&line) {
            Ok(value) => {
                let msg_type = value
                    .get("type")
                    .and_then(|t| t.as_str())
                    .unwrap_or("unknown");

                let is_system = is_system_content(&value);

                let (entry_type, preview) = if msg_type == "response_item" {
                    let role = value
                        .pointer("/payload/role")
                        .and_then(|r| r.as_str())
                        .unwrap_or("unknown");
                    let mut et = if is_system {
                        EntryType::System
                    } else {
                        match role {
                            "user" => EntryType::User,
                            "assistant" => EntryType::Assistant,
                            "developer" => EntryType::System,
                            _ => EntryType::Other,
                        }
                    };
                    let text = extract_codex_content(&value);
                    if text.is_empty()
                        && matches!(et, EntryType::User | EntryType::Assistant)
                    {
                        et = EntryType::Other;
                    }
                    (
                        et,
                        if text.is_empty() {
                            format!("[{}]", role)
                        } else {
                            text
                        },
                    )
                } else {
                    let mut et = if is_system {
                        EntryType::System
                    } else {
                        match msg_type {
                            "user" => EntryType::User,
                            "assistant" => EntryType::Assistant,
                            "progress" => EntryType::Progress,
                            _ => EntryType::Other,
                        }
                    };
                    let text = match et {
                        EntryType::User | EntryType::Assistant => {
                            let extracted = extract_content_text(&value);
                            if extracted.is_empty() {
                                et = EntryType::Other;
                                format!("[{}]", msg_type)
                            } else {
                                extracted
                            }
                        }
                        EntryType::Progress => {
                            let tool = value
                                .pointer("/data/type")
                                .and_then(|t| t.as_str())
                                .unwrap_or("progress");
                            format!("[{}]", tool)
                        }
                        EntryType::System => "[system]".to_string(),
                        EntryType::Other => format!("[{}]", msg_type),
                    };
                    (et, text)
                };

                (entry_type, preview)
            }
            Err(_) => (EntryType::Other, "[invalid JSON]".to_string()),
        };

        let cleaned = preview_text.trim().replace('\n', " ");
        let preview: SharedString = if cleaned.len() > 200 {
            let mut end = 200;
            while !cleaned.is_char_boundary(end) && end > 0 {
                end -= 1;
            }
            format!("{}...", &cleaned[..end]).into()
        } else {
            cleaned.into()
        };

        Self {
            raw_line: line,
            checked: false,
            preview,
            entry_type,
        }
    }
}

impl ThreadContentEditor {
    fn new(
        file_path: PathBuf,
        title: SharedString,
        lines: Vec<String>,
        cx: &mut Context<Self>,
    ) -> Self {
        let entries: Vec<JsonlEntry> = lines.into_iter().map(JsonlEntry::from_line).collect();
        let visible_indices = Self::compute_visible_indices(&entries);

        Self {
            file_path,
            title,
            entries,
            visible_indices,
            scroll_handle: UniformListScrollHandle::new(),
            focus: cx.focus_handle(),
            is_dirty: false,
        }
    }

    fn compute_visible_indices(entries: &[JsonlEntry]) -> Vec<usize> {
        entries
            .iter()
            .enumerate()
            .filter(|(_, e)| matches!(e.entry_type, EntryType::User | EntryType::Assistant))
            .map(|(i, _)| i)
            .collect()
    }

    pub fn open(
        file_path: PathBuf,
        title: SharedString,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<anyhow::Result<()>> {
        let weak_workspace = workspace.downgrade();
        let path = file_path.clone();

        window.spawn(cx, async move |cx| {
            let content =
                smol::fs::read_to_string(&path).await.with_context(|| {
                    format!("Failed to read session file: {}", path.display())
                })?;

            let lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();

            cx.update(|window, cx| {
                let workspace_handle = weak_workspace.upgrade().context("workspace dropped")?;
                workspace_handle.update(cx, |workspace, cx| {
                    let editor = cx.new(|cx| {
                        ThreadContentEditor::new(file_path, title, lines, cx)
                    });
                    workspace.add_item_to_active_pane(Box::new(editor), None, true, window, cx);
                });
                anyhow::Ok(())
            })?
        })
    }

    fn toggle_message(&mut self, display_ix: usize, cx: &mut Context<Self>) {
        let Some(&entry_ix) = self.visible_indices.get(display_ix) else {
            return;
        };
        self.entries[entry_ix].checked = !self.entries[entry_ix].checked;
        self.is_dirty = self.entries.iter().any(|e| e.checked);
        cx.notify();
    }

    fn delete_from_here(&mut self, display_ix: usize, cx: &mut Context<Self>) {
        for &idx in &self.visible_indices[display_ix..] {
            self.entries[idx].checked = true;
        }
        self.is_dirty = true;
        cx.notify();
    }

    fn select_all(&mut self, cx: &mut Context<Self>) {
        for &idx in &self.visible_indices {
            self.entries[idx].checked = true;
        }
        self.is_dirty = true;
        cx.notify();
    }

    fn deselect_all(&mut self, cx: &mut Context<Self>) {
        for entry in &mut self.entries {
            entry.checked = false;
        }
        self.is_dirty = false;
        cx.notify();
    }

    fn checked_count(&self) -> usize {
        self.entries.iter().filter(|e| e.checked).count()
    }

    fn delete_checked(&mut self, cx: &mut Context<Self>) {
        let kept_lines: Vec<String> = self
            .entries
            .iter()
            .filter(|e| !e.checked)
            .map(|e| e.raw_line.clone())
            .collect();

        if kept_lines.is_empty() {
            return;
        }

        let file_path = self.file_path.clone();
        let content = kept_lines.join("\n") + "\n";

        cx.spawn(async move |this, cx| {
            smol::fs::write(&file_path, content.as_bytes())
                .await
                .with_context(|| {
                    format!("Failed to write session file: {}", file_path.display())
                })?;

            this.update(cx, |this, cx| {
                this.entries.retain(|e| !e.checked);
                this.visible_indices = Self::compute_visible_indices(&this.entries);
                this.is_dirty = false;
                cx.notify();
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn render_toolbar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let checked = self.checked_count();

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
                Button::new("delete", format!("Delete {} Entries", checked))
                    .style(ButtonStyle::Filled)
                    .color(Color::Error)
                    .disabled(checked == 0)
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.delete_checked(cx);
                    })),
            )
            .child(
                Button::new("cancel", "Cancel")
                    .style(ButtonStyle::Subtle)
                    .on_click(cx.listener(|_this, _, _window, cx| {
                        cx.emit(Event::Close);
                    })),
            )
    }

    fn render_message_row(&self, display_ix: usize, cx: &Context<Self>) -> AnyElement {
        let entry_ix = self.visible_indices[display_ix];
        let entry = &self.entries[entry_ix];

        let role_label = match entry.entry_type {
            EntryType::User => "USER",
            EntryType::Assistant => "ASSISTANT",
            _ => "OTHER",
        };

        let role_color = match entry.entry_type {
            EntryType::User => Color::Accent,
            EntryType::Assistant => Color::Success,
            _ => Color::Muted,
        };

        let checked = entry.checked;
        let toggle_state = if checked {
            ToggleState::Selected
        } else {
            ToggleState::Unselected
        };

        let preview = entry.preview.clone();
        let label_text: SharedString = role_label.into();

        let entity = cx.entity();
        let entity_for_checkbox = entity.clone();
        let entity_for_menu = entity.clone();

        right_click_menu(format!("msg-ctx-{}", display_ix))
            .trigger(move |_, _, _| {
                ListItem::new(("msg", display_ix))
                    .spacing(ListItemSpacing::Sparse)
                    .start_slot(
                        h_flex()
                            .gap_2()
                            .items_start()
                            .child(
                                Checkbox::new(("check", display_ix), toggle_state).on_click({
                                    let entity = entity_for_checkbox.clone();
                                    move |_state, _window, cx| {
                                        entity.update(cx, |this, cx| {
                                            this.toggle_message(display_ix, cx);
                                        });
                                    }
                                }),
                            )
                            .child(
                                v_flex()
                                    .child(
                                        Label::new(label_text.clone())
                                            .size(LabelSize::XSmall)
                                            .color(role_color),
                                    )
                                    .child(
                                        Label::new(preview.clone())
                                            .size(LabelSize::Small)
                                            .color(if checked {
                                                Color::Error
                                            } else {
                                                Color::Default
                                            }),
                                    ),
                            ),
                    )
                    .into_any_element()
            })
            .menu(move |window, cx| {
                let entity = entity_for_menu.clone();
                ContextMenu::build(window, cx, move |menu, _window, _cx| {
                    menu.item(
                        ContextMenuEntry::new("Delete from here").handler({
                            let entity = entity.clone();
                            move |_window, cx| {
                                entity.update(cx, |this, cx| {
                                    this.delete_from_here(display_ix, cx);
                                });
                            }
                        }),
                    )
                    .item(
                        ContextMenuEntry::new("Select all").handler({
                            let entity = entity.clone();
                            move |_window, cx| {
                                entity.update(cx, |this, cx| {
                                    this.select_all(cx);
                                });
                            }
                        }),
                    )
                    .item(
                        ContextMenuEntry::new("Deselect all").handler({
                            let entity = entity.clone();
                            move |_window, cx| {
                                entity.update(cx, |this, cx| {
                                    this.deselect_all(cx);
                                });
                            }
                        }),
                    )
                })
            })
            .into_any_element()
    }
}

impl Render for ThreadContentEditor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let visible_count = self.visible_indices.len();

        v_flex()
            .key_context("ThreadContentEditor")
            .track_focus(&self.focus)
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(self.render_toolbar(cx))
            .child({
                v_flex()
                    .id("list-container")
                    .relative()
                    .overflow_hidden()
                    .flex_grow()
                    .child(
                        gpui::uniform_list(
                            "thread-messages",
                            visible_count,
                            cx.processor(
                                |this: &mut Self, range: Range<usize>, _window, cx| {
                                    range
                                        .into_iter()
                                        .map(|ix| this.render_message_row(ix, cx))
                                        .collect()
                                },
                            ),
                        )
                        .p_2()
                        .pr_4()
                        .track_scroll(&self.scroll_handle)
                        .flex_grow(),
                    )
                    .vertical_scrollbar_for(&self.scroll_handle, window, cx)
            })
    }
}

impl Item for ThreadContentEditor {
    type Event = Event;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        format!("Edit: {}", self.title).into()
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<ui::Icon> {
        Some(ui::Icon::new(IconName::Notepad))
    }

    fn tab_tooltip_text(&self, _cx: &App) -> Option<SharedString> {
        Some(format!("Editing thread: {}", self.title).into())
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(ItemEvent)) {
        match event {
            Event::Close => f(ItemEvent::CloseItem),
        }
    }

    fn is_dirty(&self, _cx: &App) -> bool {
        self.is_dirty
    }
}
