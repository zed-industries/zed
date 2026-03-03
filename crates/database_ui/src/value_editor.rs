#![allow(dead_code)]

use editor::Editor;
use gpui::{
    actions, prelude::*, px, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle,
    Focusable, SharedString, Window,
};
use ui::{prelude::*, Button, ButtonStyle, Label};
use workspace::ModalView;

use database_core::CellValue;

actions!(
    value_editor,
    [
        /// Opens the value editor for the selected cell.
        OpenValueEditor,
    ]
);

pub enum ValueEditorEvent {
    ValueSaved {
        row: usize,
        col: usize,
        value: CellValue,
    },
}

pub struct ValueEditor {
    focus_handle: FocusHandle,
    editor: Entity<Editor>,
    row: usize,
    col: usize,
    column_name: String,
    original_value: CellValue,
    value_type: ValueType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValueType {
    Text,
    Json,
    Blob,
}

impl EventEmitter<DismissEvent> for ValueEditor {}
impl EventEmitter<ValueEditorEvent> for ValueEditor {}

impl ModalView for ValueEditor {
    fn fade_out_background(&self) -> bool {
        true
    }
}

impl Focusable for ValueEditor {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ValueEditor {
    pub fn new(
        row: usize,
        col: usize,
        column_name: String,
        value: &CellValue,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        let (display_text, value_type) = match value {
            CellValue::Null => ("NULL".to_string(), ValueType::Text),
            CellValue::Json(s) => (format_json(s), ValueType::Json),
            CellValue::Text(s) => {
                if is_json(s) {
                    (format_json(s), ValueType::Json)
                } else {
                    (s.clone(), ValueType::Text)
                }
            }
            CellValue::Blob(bytes) => (format_hex(bytes), ValueType::Blob),
            other => (other.to_string(), ValueType::Text),
        };

        let editor = cx.new(|cx| {
            let mut editor = Editor::auto_height(5, 30, window, cx);
            editor.set_text(display_text, window, cx);
            editor
        });

        Self {
            focus_handle,
            editor,
            row,
            col,
            column_name,
            original_value: value.clone(),
            value_type,
        }
    }

    fn save_value(&mut self, cx: &mut Context<Self>) {
        let text = self.editor.read(cx).text(cx);
        let value = match self.value_type {
            ValueType::Blob => self.original_value.clone(),
            _ => {
                if text == "NULL" || text.is_empty() {
                    CellValue::Null
                } else {
                    CellValue::Text(text)
                }
            }
        };

        cx.emit(ValueEditorEvent::ValueSaved {
            row: self.row,
            col: self.col,
            value,
        });
        cx.emit(DismissEvent);
    }

    fn dismiss(&mut self, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }
}

impl Render for ValueEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let type_label = match self.value_type {
            ValueType::Text => "Text",
            ValueType::Json => "JSON",
            ValueType::Blob => "Binary (read-only)",
        };

        v_flex()
            .w(px(500.0))
            .max_h(px(400.0))
            .p_4()
            .gap_2()
            .child(
                h_flex()
                    .justify_between()
                    .child(
                        Label::new(SharedString::from(format!(
                            "Edit: {}",
                            self.column_name
                        )))
                        .size(LabelSize::Large)
                        .weight(gpui::FontWeight::BOLD),
                    )
                    .child(
                        Label::new(type_label)
                            .size(LabelSize::Small)
                            .color(ui::Color::Muted),
                    ),
            )
            .child(
                div()
                    .id("value-editor-content")
                    .flex_grow()
                    .max_h(px(300.0))
                    .overflow_y_scroll()
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .rounded_md()
                    .p_1()
                    .child(self.editor.clone()),
            )
            .child(
                h_flex()
                    .justify_end()
                    .gap_1()
                    .child(
                        Button::new("cancel-value-editor", "Cancel")
                            .style(ButtonStyle::Subtle)
                            .label_size(LabelSize::Small)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.dismiss(cx);
                            })),
                    )
                    .when(self.value_type != ValueType::Blob, |this| {
                        this.child(
                            Button::new("save-value-editor", "Save")
                                .style(ButtonStyle::Filled)
                                .label_size(LabelSize::Small)
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.save_value(cx);
                                })),
                        )
                    }),
            )
    }
}

fn is_json(text: &str) -> bool {
    let trimmed = text.trim();
    (trimmed.starts_with('{') && trimmed.ends_with('}'))
        || (trimmed.starts_with('[') && trimmed.ends_with(']'))
}

fn format_json(text: &str) -> String {
    serde_json::from_str::<serde_json::Value>(text)
        .and_then(|value| serde_json::to_string_pretty(&value))
        .unwrap_or_else(|_| text.to_string())
}

fn format_hex(bytes: &[u8]) -> String {
    let mut result = String::new();
    for (i, chunk) in bytes.chunks(16).enumerate() {
        let offset = i * 16;
        result.push_str(&format!("{:08x}  ", offset));

        for (j, byte) in chunk.iter().enumerate() {
            result.push_str(&format!("{:02x} ", byte));
            if j == 7 {
                result.push(' ');
            }
        }

        for _ in chunk.len()..16 {
            result.push_str("   ");
        }
        if chunk.len() <= 8 {
            result.push(' ');
        }

        result.push_str(" |");
        for byte in chunk {
            if byte.is_ascii_graphic() || *byte == b' ' {
                result.push(*byte as char);
            } else {
                result.push('.');
            }
        }
        result.push_str("|\n");
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_json_object() {
        assert!(is_json(r#"{"key": "value"}"#));
    }

    #[test]
    fn test_is_json_array() {
        assert!(is_json(r#"[1, 2, 3]"#));
    }

    #[test]
    fn test_is_not_json() {
        assert!(!is_json("hello world"));
    }

    #[test]
    fn test_format_json_pretty() {
        let input = r#"{"a":1,"b":2}"#;
        let result = format_json(input);
        assert!(result.contains("  \"a\": 1"));
    }

    #[test]
    fn test_format_json_invalid_passthrough() {
        let input = "{not valid json}";
        let result = format_json(input);
        assert_eq!(result, input);
    }

    #[test]
    fn test_format_hex() {
        let bytes = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f];
        let result = format_hex(&bytes);
        assert!(result.contains("48 65 6c 6c 6f"));
        assert!(result.contains("|Hello|"));
    }
}
