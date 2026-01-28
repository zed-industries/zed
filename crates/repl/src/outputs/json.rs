//! # JSON Output for REPL
//!
//! This module provides an interactive JSON viewer for displaying JSON data in the REPL.
//! It supports collapsible/expandable tree views for objects and arrays, with syntax
//! highlighting for different value types.

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use gpui::{App, ClipboardItem, Context, Entity, Window, div, prelude::*};
use language::Buffer;
use serde_json::Value;
use ui::{Disclosure, prelude::*};

use crate::outputs::OutputContent;

pub struct JsonView {
    root: Value,
    expanded_paths: HashMap<String, bool>,
}

impl JsonView {
    pub fn from_value(value: Value) -> anyhow::Result<Self> {
        let mut expanded_paths = HashMap::new();
        expanded_paths.insert("root".to_string(), true);

        Ok(Self {
            root: value,
            expanded_paths,
        })
    }

    fn toggle_path(&mut self, path: &str, cx: &mut Context<Self>) {
        let current = self.expanded_paths.get(path).copied().unwrap_or(false);
        self.expanded_paths.insert(path.to_string(), !current);
        cx.notify();
    }

    fn is_expanded(&self, path: &str) -> bool {
        self.expanded_paths.get(path).copied().unwrap_or(false)
    }

    fn path_hash(path: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        path.hash(&mut hasher);
        hasher.finish()
    }

    fn render_value(
        &self,
        path: String,
        key: Option<&str>,
        value: &Value,
        depth: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let indent = depth * 12;

        match value {
            Value::Object(map) if map.is_empty() => {
                self.render_line(path, key, "{}", depth, Color::Muted, window, cx)
            }
            Value::Object(map) => {
                let is_expanded = self.is_expanded(&path);
                let preview = if is_expanded {
                    String::new()
                } else {
                    format!("{{ {} fields }}", map.len())
                };

                v_flex()
                    .child(
                        h_flex()
                            .gap_1()
                            .pl(px(indent as f32))
                            .cursor_pointer()
                            .on_mouse_down(
                                gpui::MouseButton::Left,
                                cx.listener({
                                    let path = path.clone();
                                    move |this, _, _, cx| {
                                        this.toggle_path(&path, cx);
                                    }
                                }),
                            )
                            .child(Disclosure::new(
                                ("json-disclosure", Self::path_hash(&path)),
                                is_expanded,
                            ))
                            .child(
                                h_flex()
                                    .gap_1()
                                    .when_some(key, |this, k| {
                                        this.child(
                                            Label::new(format!("{}: ", k)).color(Color::Accent),
                                        )
                                    })
                                    .when(!is_expanded, |this| {
                                        this.child(Label::new("{").color(Color::Muted))
                                            .child(
                                                Label::new(format!(" {} ", preview))
                                                    .color(Color::Muted),
                                            )
                                            .child(Label::new("}").color(Color::Muted))
                                    }),
                            ),
                    )
                    .when(is_expanded, |this| {
                        this.children(
                            map.iter()
                                .map(|(k, v)| {
                                    let child_path = format!("{}.{}", path, k);
                                    self.render_value(child_path, Some(k), v, depth + 1, window, cx)
                                })
                                .collect::<Vec<_>>(),
                        )
                    })
                    .into_any_element()
            }
            Value::Array(arr) if arr.is_empty() => {
                self.render_line(path, key, "[]", depth, Color::Muted, window, cx)
            }
            Value::Array(arr) => {
                let is_expanded = self.is_expanded(&path);
                let preview = if is_expanded {
                    String::new()
                } else {
                    format!("[ {} items ]", arr.len())
                };

                v_flex()
                    .child(
                        h_flex()
                            .gap_1()
                            .pl(px(indent as f32))
                            .cursor_pointer()
                            .on_mouse_down(
                                gpui::MouseButton::Left,
                                cx.listener({
                                    let path = path.clone();
                                    move |this, _, _, cx| {
                                        this.toggle_path(&path, cx);
                                    }
                                }),
                            )
                            .child(Disclosure::new(
                                ("json-disclosure", Self::path_hash(&path)),
                                is_expanded,
                            ))
                            .child(
                                h_flex()
                                    .gap_1()
                                    .when_some(key, |this, k| {
                                        this.child(
                                            Label::new(format!("{}: ", k)).color(Color::Accent),
                                        )
                                    })
                                    .when(!is_expanded, |this| {
                                        this.child(Label::new("[").color(Color::Muted))
                                            .child(
                                                Label::new(format!(" {} ", preview))
                                                    .color(Color::Muted),
                                            )
                                            .child(Label::new("]").color(Color::Muted))
                                    }),
                            ),
                    )
                    .when(is_expanded, |this| {
                        this.children(
                            arr.iter()
                                .enumerate()
                                .map(|(i, v)| {
                                    let child_path = format!("{}[{}]", path, i);
                                    self.render_value(child_path, None, v, depth + 1, window, cx)
                                })
                                .collect::<Vec<_>>(),
                        )
                    })
                    .into_any_element()
            }
            Value::String(s) => {
                let display = format!("\"{}\"", s);
                self.render_line(path, key, &display, depth, Color::Success, window, cx)
            }
            Value::Number(n) => {
                let display = n.to_string();
                self.render_line(path, key, &display, depth, Color::Modified, window, cx)
            }
            Value::Bool(b) => {
                let display = b.to_string();
                self.render_line(path, key, &display, depth, Color::Info, window, cx)
            }
            Value::Null => self.render_line(path, key, "null", depth, Color::Disabled, window, cx),
        }
    }

    fn render_line(
        &self,
        _path: String,
        key: Option<&str>,
        value: &str,
        depth: usize,
        color: Color,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> AnyElement {
        let indent = depth * 16;

        h_flex()
            .pl(px(indent as f32))
            .gap_1()
            .when_some(key, |this, k| {
                this.child(Label::new(format!("{}: ", k)).color(Color::Accent))
            })
            .child(Label::new(value.to_string()).color(color))
            .into_any_element()
    }
}

impl Render for JsonView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let root_clone = self.root.clone();
        let root_element = self.render_value("root".to_string(), None, &root_clone, 0, window, cx);
        div().w_full().child(root_element)
    }
}

impl OutputContent for JsonView {
    fn clipboard_content(&self, _window: &Window, _cx: &App) -> Option<ClipboardItem> {
        serde_json::to_string_pretty(&self.root)
            .ok()
            .map(ClipboardItem::new_string)
    }

    fn has_clipboard_content(&self, _window: &Window, _cx: &App) -> bool {
        true
    }

    fn has_buffer_content(&self, _window: &Window, _cx: &App) -> bool {
        true
    }

    fn buffer_content(&mut self, _window: &mut Window, cx: &mut App) -> Option<Entity<Buffer>> {
        let json_text = serde_json::to_string_pretty(&self.root).ok()?;
        let buffer = cx.new(|cx| {
            let mut buffer =
                Buffer::local(json_text, cx).with_language(language::PLAIN_TEXT.clone(), cx);
            buffer.set_capability(language::Capability::ReadOnly, cx);
            buffer
        });
        Some(buffer)
    }
}
