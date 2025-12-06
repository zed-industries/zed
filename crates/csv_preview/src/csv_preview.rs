use editor::Editor;
use gpui::{AppContext, Entity, EventEmitter, FocusHandle, Focusable, actions};
use log::info;
use ui::{SharedString, prelude::*};
use workspace::{Item, Workspace};

actions!(csv, [OpenPreview]);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            println!("No window yet");
            return;
        };
        CsvPreviewView::register(workspace, window, cx);
    })
    .detach()
}

pub struct CsvPreviewView {
    focus_handle: FocusHandle,
    editor: Entity<Editor>,
    contents: ParsedCsv,
    counter: usize,
}
impl CsvPreviewView {
    pub fn register(
        workspace: &mut Workspace,
        _window: &mut Window,
        _cx: &mut Context<'_, Workspace>,
    ) {
        // Register open preview action
        workspace.register_action(|workspace, _: &OpenPreview, window, cx| {
            info!("Open preview called");
            let maybe_editor = {
                let and_then = workspace
                    .active_item(cx)
                    .and_then(|item| item.act_as::<Editor>(cx));
                let Some(editor) = and_then else {
                    info!("No editor");
                    return;
                };
                if Self::is_csv_file(&editor, cx) {
                    info!("Editor is csv");
                    Some(editor)
                } else {
                    info!("Editor is not csv");
                    None
                }
            };

            let Some(editor) = maybe_editor else {
                info!("No CSV editor found");
                return;
            };

            let view = CsvPreviewView::from_editor(&editor, cx);
            info!("Created CSV View");
            workspace.active_pane().update(cx, |pane, cx| {
                // TODO: handle existing pane
                info!("Attaching CSV View");
                pane.add_item(Box::new(view.clone()), true, true, None, window, cx)
            });
            cx.notify();
        });
    }

    fn is_csv_file(editor: &Entity<Editor>, cx: &mut Context<Workspace>) -> bool {
        let buffer = editor.read(cx).buffer().read(cx);
        let Some(buffer) = buffer.as_singleton() else {
            info!("Buffer is not singleton");
            return false;
        };

        // Check file extension instead of language detection
        if let Some(file) = buffer.read(cx).file() {
            let path = file.path();
            let extension = path.extension();
            let is_csv = extension == Some("csv");
            info!(
                "File path: {:?}, extension: {:?}, is_csv: {}",
                path, extension, is_csv
            );
            is_csv
        } else {
            info!("Buffer has no associated file");
            false
        }
    }

    fn from_editor(editor: &Entity<Editor>, cx: &mut Context<Workspace>) -> Entity<Self> {
        let raw_text = editor
            .read(cx)
            .buffer()
            .read(cx)
            .as_singleton()
            .map(|b| b.read(cx).text())
            .unwrap_or_else(|| "".to_string());

        cx.new(|cx| Self {
            focus_handle: cx.focus_handle(),
            editor: editor.clone(),
            contents: ParsedCsv::from_str(raw_text),
            counter: 0,
        })
    }
}

pub struct ParsedCsv {
    pub headers: Vec<SharedString>,
    pub rows: Vec<Vec<SharedString>>,
}

impl ParsedCsv {
    pub fn from_str(raw_csv: String) -> ParsedCsv {
        let lines: Vec<&str> = raw_csv.lines().collect();

        if lines.is_empty() {
            return ParsedCsv {
                headers: vec![],
                rows: vec![],
            };
        }

        // Parse headers from first line
        let headers: Vec<SharedString> = Self::parse_csv_line(lines[0])
            .into_iter()
            .map(|s| s.into())
            .collect();

        // Parse data rows from remaining lines
        let rows: Vec<Vec<SharedString>> = lines
            .iter()
            .skip(1)
            .map(|line| {
                Self::parse_csv_line(line)
                    .into_iter()
                    .map(|s| s.into())
                    .collect()
            })
            .collect();

        ParsedCsv { headers, rows }
    }

    fn parse_csv_line(line: &str) -> Vec<String> {
        let mut fields = Vec::new();
        let mut current_field = String::new();
        let mut in_quotes = false;
        let mut chars = line.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                '"' => {
                    if in_quotes && chars.peek() == Some(&'"') {
                        // Escaped quote within quoted field
                        current_field.push('"');
                        chars.next(); // Skip the second quote
                    } else {
                        // Toggle quote state
                        in_quotes = !in_quotes;
                    }
                }
                ',' if !in_quotes => {
                    // Field separator
                    fields.push(current_field.trim().to_string());
                    current_field.clear();
                }
                _ => {
                    current_field.push(ch);
                }
            }
        }

        // Add the last field
        fields.push(current_field.trim().to_string());
        fields
    }
}

impl Focusable for CsvPreviewView {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for CsvPreviewView {}

/// Icon and description as tab
impl Item for CsvPreviewView {
    type Event = ();

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> ui::SharedString {
        "CSV Preview".into()
    }

    // fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {}
}

/// Main trait to render the content of the CSV preview in pane
impl Render for CsvPreviewView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.contents.headers.is_empty() {
            return v_flex()
                .size_full()
                .p_4()
                .child(div().child("No CSV content"));
        }

        // Create header row with padding for alignment
        let max_widths = self.calculate_column_widths();
        let header_row = self.format_row(&self.contents.headers, &max_widths);

        // Create separator line
        let separator = max_widths
            .iter()
            .map(|&width| "-".repeat(width))
            .collect::<Vec<_>>()
            .join("-+-");

        // Create data rows
        let data_rows: Vec<String> = self
            .contents
            .rows
            .iter()
            .map(|row| self.format_row(row, &max_widths))
            .collect();

        let all_content = format!("{}\n{}\n{}", header_row, separator, data_rows.join("\n"));

        v_flex()
            .w_full()
            .h_full()
            .p_4()
            .child(
                div()
                    .text_xl()
                    .font_weight(gpui::FontWeight::BOLD)
                    .mb_4()
                    .child("CSV Preview"),
            )
            .child(
                h_flex()
                    .items_center()
                    .gap_3()
                    .mb_4()
                    .child(
                        Button::new("increment_counter", "Increment Counter").on_click(
                            cx.listener(|this, _event, _window, cx| {
                                this.counter += 1;
                                cx.notify();
                            }),
                        ),
                    )
                    .child(format!("Count: {}", self.counter)),
            )
            .child(
                div()
                    .font_family("monospace")
                    .overflow_hidden()
                    .w_full()
                    .h_full()
                    .child(all_content),
            )
    }
}

impl CsvPreviewView {
    fn calculate_column_widths(&self) -> Vec<usize> {
        if self.contents.headers.is_empty() {
            return vec![];
        }

        let mut widths = self
            .contents
            .headers
            .iter()
            .map(|h| h.len())
            .collect::<Vec<_>>();

        for row in &self.contents.rows {
            for (i, cell) in row.iter().enumerate() {
                if i < widths.len() {
                    widths[i] = widths[i].max(cell.len());
                } else {
                    widths.push(cell.len());
                }
            }
        }

        widths
    }

    fn format_row(&self, row: &[SharedString], widths: &[usize]) -> String {
        row.iter()
            .enumerate()
            .map(|(i, cell)| {
                let width = widths.get(i).copied().unwrap_or(cell.len());
                format!("{:<width$}", cell.as_ref(), width = width)
            })
            .collect::<Vec<_>>()
            .join(" | ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_parsing_basic() {
        let csv_content = "Name,Age,City\nJohn,25,New York\nJane,30,San Francisco".to_string();
        let parsed = ParsedCsv::from_str(csv_content);

        assert_eq!(parsed.headers.len(), 3);
        assert_eq!(parsed.headers[0].as_ref(), "Name");
        assert_eq!(parsed.headers[1].as_ref(), "Age");
        assert_eq!(parsed.headers[2].as_ref(), "City");

        assert_eq!(parsed.rows.len(), 2);
        assert_eq!(parsed.rows[0].len(), 3);
        assert_eq!(parsed.rows[0][0].as_ref(), "John");
        assert_eq!(parsed.rows[0][1].as_ref(), "25");
        assert_eq!(parsed.rows[0][2].as_ref(), "New York");
    }

    #[test]
    fn test_csv_parsing_with_quotes() {
        let csv_content = r#"Name,Description
John,"A person with, comma"
Jane,"Another ""quoted"" field""#
            .to_string();
        let parsed = ParsedCsv::from_str(csv_content);

        assert_eq!(parsed.headers.len(), 2);
        assert_eq!(parsed.rows.len(), 2);
        assert_eq!(parsed.rows[0][1].as_ref(), "A person with, comma");
        assert_eq!(parsed.rows[1][1].as_ref(), r#"Another "quoted" field"#);
    }

    #[test]
    fn test_empty_csv() {
        let parsed = ParsedCsv::from_str("".to_string());
        assert_eq!(parsed.headers.len(), 0);
        assert_eq!(parsed.rows.len(), 0);
    }
}
