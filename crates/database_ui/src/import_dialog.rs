#![allow(dead_code)]

use gpui::{
    actions, prelude::*, px, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle,
    Focusable, SharedString, Task, Window,
};
use ui::{prelude::*, Banner, Button, ButtonStyle, Label, Severity, Tooltip};
use util::ResultExt as _;
use workspace::ModalView;

use crate::connection_manager::ConnectionManager;
use database_core::DatabaseType;

actions!(
    import_dialog,
    [
        /// Opens the CSV import dialog.
        ImportCsv,
    ]
);

pub struct ImportDialog {
    focus_handle: FocusHandle,
    connection_manager: Entity<ConnectionManager>,
    table_name: String,

    csv_headers: Vec<String>,
    csv_preview_rows: Vec<Vec<String>>,
    csv_all_rows: Vec<Vec<String>>,

    status_message: Option<Result<String, String>>,
    import_task: Task<()>,
}

impl EventEmitter<DismissEvent> for ImportDialog {}

impl ModalView for ImportDialog {
    fn fade_out_background(&self) -> bool {
        true
    }
}

impl Focusable for ImportDialog {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ImportDialog {
    pub fn new(
        connection_manager: Entity<ConnectionManager>,
        table_name: String,
        csv_content: String,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        let (headers, rows) = parse_csv(&csv_content);
        let preview_rows: Vec<Vec<String>> = rows.iter().take(10).cloned().collect();

        Self {
            focus_handle,
            connection_manager,
            table_name,
            csv_headers: headers,
            csv_preview_rows: preview_rows,
            csv_all_rows: rows,
            status_message: None,
            import_task: Task::ready(()),
        }
    }

    fn dismiss(&mut self, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn perform_import(&mut self, cx: &mut Context<Self>) {
        if self.csv_all_rows.is_empty() {
            self.status_message = Some(Err("No data to import".to_string()));
            cx.notify();
            return;
        }

        let db_type = self
            .connection_manager
            .read(cx)
            .active_entry()
            .map(|e| e.config.database_type.clone())
            .unwrap_or(DatabaseType::Sqlite);

        let sql = generate_insert_statements(
            &self.table_name,
            &self.csv_headers,
            &self.csv_all_rows,
            &db_type,
        );

        let wrapped_sql = format!("BEGIN;\n{}\nCOMMIT;", sql);

        let task = self
            .connection_manager
            .read(cx)
            .execute_raw_query(wrapped_sql, cx);

        let row_count = self.csv_all_rows.len();
        self.import_task = cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| {
                match result {
                    Ok(_) => {
                        this.status_message = Some(Ok(format!(
                            "Successfully imported {} rows",
                            row_count
                        )));
                    }
                    Err(error) => {
                        this.status_message =
                            Some(Err(format!("Import failed: {:#}", error)));
                    }
                }
                cx.notify();
            })
            .log_err();
        });
    }
}

impl Render for ImportDialog {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let total_rows = self.csv_all_rows.len();
        let col_count = self.csv_headers.len();

        let mut content = v_flex()
            .w(px(600.0))
            .max_h(px(500.0))
            .p_4()
            .gap_3()
            .child(
                h_flex()
                    .justify_between()
                    .child(
                        Label::new("Import CSV")
                            .size(LabelSize::Large)
                            .weight(gpui::FontWeight::BOLD),
                    )
                    .child(
                        Button::new("close-import-dialog", "")
                            .icon(ui::IconName::Close)
                            .icon_size(ui::IconSize::Small)
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.dismiss(cx);
                            })),
                    ),
            )
            .child(
                Label::new(SharedString::from(format!(
                    "Target: {} | {} rows, {} columns",
                    self.table_name, total_rows, col_count
                )))
                .size(LabelSize::Small)
                .color(ui::Color::Muted),
            );

        if !self.csv_preview_rows.is_empty() {
            let headers_text = self.csv_headers.join(" | ");
            let separator = self.csv_headers.iter().map(|h| "-".repeat(h.len().max(3))).collect::<Vec<_>>().join("-+-");

            let mut preview_lines = vec![headers_text, separator];
            for row in &self.csv_preview_rows {
                preview_lines.push(row.join(" | "));
            }
            if total_rows > 10 {
                preview_lines.push(format!("... and {} more rows", total_rows - 10));
            }

            content = content.child(
                div()
                    .id("csv-preview-scroll")
                    .max_h(px(200.0))
                    .overflow_y_scroll()
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .rounded_md()
                    .p_2()
                    .bg(cx.theme().colors().editor_background)
                    .child(
                        Label::new(SharedString::from(preview_lines.join("\n")))
                            .size(LabelSize::Small)
                            .color(ui::Color::Default),
                    ),
            );
        }

        if let Some(status) = &self.status_message {
            match status {
                Ok(message) => {
                    content = content.child(
                        Banner::new()
                            .severity(Severity::Success)
                            .child(Label::new(SharedString::from(message.clone()))),
                    );
                }
                Err(error) => {
                    content = content.child(
                        Banner::new()
                            .severity(Severity::Error)
                            .child(Label::new(SharedString::from(error.clone()))),
                    );
                }
            }
        }

        content = content.child(
            h_flex()
                .justify_end()
                .gap_1()
                .child(
                    Button::new("cancel-import-btn", "Cancel")
                        .style(ButtonStyle::Subtle)
                        .label_size(LabelSize::Small)
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.dismiss(cx);
                        })),
                )
                .child(
                    Button::new("import-btn", "Import")
                        .style(ButtonStyle::Filled)
                        .label_size(LabelSize::Small)
                        .tooltip(Tooltip::text("Import CSV data into table"))
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.perform_import(cx);
                        })),
                ),
        );

        content
    }
}

fn parse_csv(content: &str) -> (Vec<String>, Vec<Vec<String>>) {
    let mut lines = content.lines();

    let Some(header_line) = lines.next() else {
        return (Vec::new(), Vec::new());
    };

    let headers = parse_csv_line(header_line);
    let col_count = headers.len();

    let rows: Vec<Vec<String>> = lines
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut fields = parse_csv_line(line);
            fields.resize(col_count, String::new());
            fields.truncate(col_count);
            fields
        })
        .collect();

    (headers, rows)
}

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(character) = chars.next() {
        if in_quotes {
            if character == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    current.push('"');
                } else {
                    in_quotes = false;
                }
            } else {
                current.push(character);
            }
        } else {
            match character {
                ',' => {
                    fields.push(current.trim().to_string());
                    current = String::new();
                }
                '"' => {
                    in_quotes = true;
                }
                _ => {
                    current.push(character);
                }
            }
        }
    }
    fields.push(current.trim().to_string());
    fields
}

fn generate_insert_statements(
    table_name: &str,
    headers: &[String],
    rows: &[Vec<String>],
    db_type: &DatabaseType,
) -> String {
    let quoted_columns: Vec<String> = headers
        .iter()
        .map(|h| database_core::quote_identifier(h, db_type))
        .collect();
    let columns_str = quoted_columns.join(", ");

    let mut statements = Vec::new();
    for row in rows {
        let values: Vec<String> = row
            .iter()
            .map(|value| {
                if value.is_empty() || value.eq_ignore_ascii_case("null") {
                    "NULL".to_string()
                } else {
                    format!("'{}'", value.replace('\'', "''"))
                }
            })
            .collect();

        let table = database_core::quote_identifier(table_name, db_type);
        statements.push(format!(
            "INSERT INTO {} ({}) VALUES ({});",
            table,
            columns_str,
            values.join(", ")
        ));
    }

    statements.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv_basic() {
        let csv = "name,age,city\nAlice,30,Paris\nBob,25,London";
        let (headers, rows) = parse_csv(csv);
        assert_eq!(headers, vec!["name", "age", "city"]);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0], vec!["Alice", "30", "Paris"]);
        assert_eq!(rows[1], vec!["Bob", "25", "London"]);
    }

    #[test]
    fn test_parse_csv_quoted_fields() {
        let csv = r#"name,bio
"Alice","Has a ""quoted"" word"
"Bob","Line1, still line1""#;
        let (headers, rows) = parse_csv(csv);
        assert_eq!(headers, vec!["name", "bio"]);
        assert_eq!(rows[0][0], "Alice");
        assert_eq!(rows[0][1], "Has a \"quoted\" word");
        assert_eq!(rows[1][1], "Line1, still line1");
    }

    #[test]
    fn test_parse_csv_empty() {
        let (headers, rows) = parse_csv("");
        assert!(headers.is_empty());
        assert!(rows.is_empty());
    }

    #[test]
    fn test_generate_insert_statements() {
        let headers = vec!["name".to_string(), "age".to_string()];
        let rows = vec![
            vec!["Alice".to_string(), "30".to_string()],
            vec!["Bob".to_string(), "".to_string()],
        ];
        let sql = generate_insert_statements("users", &headers, &rows, &DatabaseType::Sqlite);
        assert!(sql.contains("INSERT INTO \"users\""));
        assert!(sql.contains("'Alice'"));
        assert!(sql.contains("NULL"));
    }

    #[test]
    fn test_generate_insert_sql_injection_safe() {
        let headers = vec!["name".to_string()];
        let rows = vec![vec!["O'Brien".to_string()]];
        let sql = generate_insert_statements("users", &headers, &rows, &DatabaseType::Sqlite);
        assert!(sql.contains("'O''Brien'"));
    }
}
