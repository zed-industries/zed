use std::path::PathBuf;

use gpui::{
    actions, prelude::*, px, App, ClipboardItem, Context, DismissEvent, EventEmitter, FocusHandle,
    Focusable, SharedString, Task, Window,
};
use util::ResultExt as _;
use ui::{prelude::*, Banner, Button, ButtonStyle, Label, Severity, Tooltip};
use workspace::ModalView;

use database_core::{
    QueryResult, generate_csv, generate_html, generate_json, generate_markdown,
    generate_sql_ddl_dml, generate_sql_insert, generate_tsv, generate_xlsx,
};

actions!(
    export_dialog,
    [
        /// Opens the export dialog.
        ShowExportDialog,
    ]
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormatOption {
    Csv,
    Tsv,
    Json,
    SqlInsert,
    SqlDdlDml,
    Markdown,
    Html,
    Excel,
}

impl ExportFormatOption {
    fn label(&self) -> &'static str {
        match self {
            ExportFormatOption::Csv => "CSV",
            ExportFormatOption::Tsv => "TSV",
            ExportFormatOption::Json => "JSON",
            ExportFormatOption::SqlInsert => "SQL INSERT",
            ExportFormatOption::SqlDdlDml => "SQL DDL+DML",
            ExportFormatOption::Markdown => "Markdown",
            ExportFormatOption::Html => "HTML",
            ExportFormatOption::Excel => "Excel",
        }
    }

    fn file_extension(&self) -> &'static str {
        match self {
            ExportFormatOption::Csv => "csv",
            ExportFormatOption::Tsv => "tsv",
            ExportFormatOption::Json => "json",
            ExportFormatOption::SqlInsert | ExportFormatOption::SqlDdlDml => "sql",
            ExportFormatOption::Markdown => "md",
            ExportFormatOption::Html => "html",
            ExportFormatOption::Excel => "xlsx",
        }
    }

    fn is_binary(&self) -> bool {
        matches!(self, ExportFormatOption::Excel)
    }

    fn all() -> &'static [ExportFormatOption] {
        &[
            ExportFormatOption::Csv,
            ExportFormatOption::Tsv,
            ExportFormatOption::Json,
            ExportFormatOption::SqlInsert,
            ExportFormatOption::SqlDdlDml,
            ExportFormatOption::Markdown,
            ExportFormatOption::Html,
            ExportFormatOption::Excel,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExportDestination {
    Clipboard,
    File,
}

impl ExportDestination {
    fn label(&self) -> &'static str {
        match self {
            ExportDestination::Clipboard => "Clipboard",
            ExportDestination::File => "File",
        }
    }
}

pub struct ExportDialog {
    focus_handle: FocusHandle,
    result: QueryResult,
    table_name: String,
    selected_format: ExportFormatOption,
    selected_destination: ExportDestination,
    status_message: Option<Result<String, String>>,
    _export_task: Task<()>,
}

impl EventEmitter<DismissEvent> for ExportDialog {}

impl ModalView for ExportDialog {
    fn fade_out_background(&self) -> bool {
        true
    }
}

impl Focusable for ExportDialog {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ExportDialog {
    pub fn new(
        result: QueryResult,
        table_name: String,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        Self {
            focus_handle,
            result,
            table_name,
            selected_format: ExportFormatOption::Csv,
            selected_destination: ExportDestination::Clipboard,
            status_message: None,
            _export_task: Task::ready(()),
        }
    }

    fn dismiss(&mut self, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn generate_export_content(&self) -> String {
        match self.selected_format {
            ExportFormatOption::Csv => generate_csv(&self.result),
            ExportFormatOption::Tsv => generate_tsv(&self.result),
            ExportFormatOption::Json => generate_json(&self.result),
            ExportFormatOption::SqlInsert => generate_sql_insert(&self.result, &self.table_name),
            ExportFormatOption::SqlDdlDml => {
                generate_sql_ddl_dml(&self.result, &self.table_name)
            }
            ExportFormatOption::Markdown => generate_markdown(&self.result),
            ExportFormatOption::Html => generate_html(&self.result),
            ExportFormatOption::Excel => String::new(),
        }
    }

    fn generate_export_bytes(&self) -> Result<Vec<u8>, String> {
        match self.selected_format {
            ExportFormatOption::Excel => {
                generate_xlsx(&self.result).map_err(|error| format!("Excel generation error: {:#}", error))
            }
            _ => Ok(self.generate_export_content().into_bytes()),
        }
    }

    fn perform_export(&mut self, cx: &mut Context<Self>) {
        if self.selected_format.is_binary() && self.selected_destination == ExportDestination::Clipboard {
            self.status_message = Some(Err("Excel format cannot be copied to clipboard. Use File destination.".to_string()));
            cx.notify();
            return;
        }

        match self.selected_destination {
            ExportDestination::Clipboard => {
                let content = self.generate_export_content();
                cx.write_to_clipboard(ClipboardItem::new_string(content));
                self.status_message = Some(Ok("Copied to clipboard".to_string()));
                cx.notify();
            }
            ExportDestination::File => {
                let bytes = match self.generate_export_bytes() {
                    Ok(bytes) => bytes,
                    Err(error) => {
                        self.status_message = Some(Err(error));
                        cx.notify();
                        return;
                    }
                };

                let extension = self.selected_format.file_extension();
                let default_name = format!("query_results.{}", extension);
                let save_dialog =
                    cx.prompt_for_new_path(&PathBuf::default(), Some(&default_name));

                self._export_task = cx.spawn(async move |this, cx| {
                    let path = match save_dialog.await {
                        Ok(Ok(Some(path))) => path,
                        Ok(Ok(None)) => return,
                        Ok(Err(error)) => {
                            this.update(cx, |this, cx| {
                                this.status_message =
                                    Some(Err(format!("File dialog error: {:#}", error)));
                                cx.notify();
                            })
                            .log_err();
                            return;
                        }
                        Err(_) => return,
                    };

                    match std::fs::write(&path, &bytes) {
                        Ok(()) => {
                            this.update(cx, |this, cx| {
                                this.status_message = Some(Ok(format!(
                                    "Exported to {}",
                                    path.display()
                                )));
                                cx.notify();
                            })
                            .log_err();
                        }
                        Err(error) => {
                            this.update(cx, |this, cx| {
                                this.status_message =
                                    Some(Err(format!("Failed to write file: {:#}", error)));
                                cx.notify();
                            })
                            .log_err();
                        }
                    }
                });
            }
        }
    }

    fn render_format_options(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let label = v_flex().gap(px(4.0)).child(
            Label::new("Format")
                .size(LabelSize::Small)
                .color(ui::Color::Muted),
        );

        let mut row = h_flex().gap_1().flex_wrap();

        for format in ExportFormatOption::all() {
            let is_selected = self.selected_format == *format;
            let format_value = *format;

            row = row.child(
                Button::new(
                    SharedString::from(format!("fmt-{}", format.label())),
                    format.label(),
                )
                .style(if is_selected {
                    ButtonStyle::Filled
                } else {
                    ButtonStyle::Subtle
                })
                .label_size(LabelSize::Small)
                .on_click(cx.listener(move |this, _, _window, cx| {
                    this.selected_format = format_value;
                    this.status_message = None;
                    cx.notify();
                })),
            );
        }

        label.child(row)
    }

    fn render_destination_options(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let label = v_flex().gap(px(4.0)).child(
            Label::new("Destination")
                .size(LabelSize::Small)
                .color(ui::Color::Muted),
        );

        let mut row = h_flex().gap_1();

        for destination in [ExportDestination::Clipboard, ExportDestination::File] {
            let is_selected = self.selected_destination == destination;

            row = row.child(
                Button::new(
                    SharedString::from(format!("dest-{}", destination.label())),
                    destination.label(),
                )
                .style(if is_selected {
                    ButtonStyle::Filled
                } else {
                    ButtonStyle::Subtle
                })
                .label_size(LabelSize::Small)
                .on_click(cx.listener(move |this, _, _window, cx| {
                    this.selected_destination = destination;
                    this.status_message = None;
                    cx.notify();
                })),
            );
        }

        label.child(row)
    }
}

impl Render for ExportDialog {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let row_count = self.result.rows.len();
        let col_count = self.result.columns.len();

        let mut content = v_flex()
            .w(px(380.0))
            .p_4()
            .gap_3()
            .child(
                h_flex()
                    .justify_between()
                    .child(
                        Label::new("Export Results")
                            .size(LabelSize::Large)
                            .weight(gpui::FontWeight::BOLD),
                    )
                    .child(
                        Button::new("close-export-dialog", "")
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
                    "{} rows, {} columns",
                    row_count, col_count
                )))
                .size(LabelSize::Small)
                .color(ui::Color::Muted),
            )
            .child(self.render_format_options(cx))
            .child(self.render_destination_options(cx));

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
                    Button::new("cancel-export-btn", "Cancel")
                        .style(ButtonStyle::Subtle)
                        .label_size(LabelSize::Small)
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.dismiss(cx);
                        })),
                )
                .child(
                    Button::new("export-btn", "Export")
                        .style(ButtonStyle::Filled)
                        .label_size(LabelSize::Small)
                        .tooltip(Tooltip::text("Export results in the selected format"))
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.perform_export(cx);
                        })),
                ),
        );

        content
    }
}
