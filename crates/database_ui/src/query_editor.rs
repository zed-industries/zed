use editor::{Editor, EditorEvent};
use gpui::{
    actions, div, prelude::*, px, App, Context, CursorStyle, Entity, EventEmitter, FocusHandle,
    Focusable, MouseButton, SharedString, Subscription, Task,
};
use ui::{prelude::*, Button, ButtonStyle, Icon, IconName, Label, Tooltip};
use util::ResultExt as _;
use workspace::{
    Workspace,
    item::{Item, ItemEvent},
};

use database_core::{DatabaseType, QueryHistory, generate_csv, generate_json, generate_markdown, quote_identifier};
use db::kvp::KEY_VALUE_STORE;
use crate::connection_manager::ConnectionManager;
use crate::result_grid::{ResultGrid, ResultGridEvent};
use crate::results_table::SortDirection;

actions!(
    query_editor,
    [
        /// Opens a new query editor tab.
        OpenQueryEditor,
        /// Runs EXPLAIN on the current query.
        ExplainQuery,
        /// Formats the SQL query.
        FormatQuery,
        /// Cancels the currently executing query.
        CancelQuery,
        /// Toggles pinning on the current tab.
        TogglePinTab,
        /// Executes the query and saves results directly to a file.
        ExecuteToFile,
    ]
);

const MIN_SPLIT_RATIO: f32 = 0.1;
const MAX_SPLIT_RATIO: f32 = 0.9;
const DEFAULT_SPLIT_RATIO: f32 = 0.4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionState {
    Idle,
    Executing,
    Completed,
    Failed,
}

pub struct QueryEditorTab {
    focus_handle: FocusHandle,
    connection_manager: Entity<ConnectionManager>,
    editor: Entity<Editor>,
    result_grid: Entity<ResultGrid>,
    tab_name: String,

    execution_state: ExecutionState,
    current_query: Option<String>,
    query_error: Option<String>,
    query_task: Task<()>,
    query_history: QueryHistory,
    is_pinned: bool,
    is_explain: bool,

    current_page: usize,
    rows_per_page: usize,

    split_ratio: f32,
    split_dragging: bool,
    split_drag_start_y: f32,
    split_drag_start_ratio: f32,

    _subscriptions: Vec<Subscription>,
}

impl QueryEditorTab {
    pub fn new(
        connection_manager: Entity<ConnectionManager>,
        initial_sql: Option<String>,
        tab_name: Option<String>,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        let editor = cx.new(|cx| {
            let mut editor = Editor::auto_height(5, 20, window, cx);
            editor.set_placeholder_text("Enter SQL query...", window, cx);
            if let Some(sql) = initial_sql {
                editor.set_text(sql, window, cx);
            }
            editor
        });

        let result_grid = cx.new(|cx| {
            let mut grid = ResultGrid::new(cx);
            grid.set_connection_manager(connection_manager.clone());
            grid
        });

        let editor_subscription =
            cx.subscribe(&editor, |_this: &mut Self, _editor, event, cx| {
                if matches!(event, EditorEvent::BufferEdited { .. }) {
                    cx.notify();
                }
            });

        let grid_subscription =
            cx.subscribe(&result_grid, |this: &mut Self, _grid, event, cx| {
                match event {
                    ResultGridEvent::SortChanged(_sort) => {
                        this.current_page = 0;
                        this.fetch_current_page(cx);
                    }
                    ResultGridEvent::PageChanged(page) => {
                        this.current_page = *page;
                        this.fetch_current_page(cx);
                    }
                    ResultGridEvent::WhereClauseChanged(_) => {
                        this.current_page = 0;
                        this.fetch_current_page(cx);
                    }
                    ResultGridEvent::NavigateToForeignKey { table, column, value } => {
                        this.navigate_to_foreign_key(table.clone(), column.clone(), value.clone(), cx);
                    }
                    _ => {}
                }
            });

        let mut this = Self {
            focus_handle,
            connection_manager,
            editor,
            result_grid,
            tab_name: tab_name.unwrap_or_else(|| "Query".to_string()),
            execution_state: ExecutionState::Idle,
            current_query: None,
            query_error: None,
            query_task: Task::ready(()),
            query_history: QueryHistory::new(100),
            is_pinned: false,
            is_explain: false,
            current_page: 0,
            rows_per_page: 50,
            split_ratio: DEFAULT_SPLIT_RATIO,
            split_dragging: false,
            split_drag_start_y: 0.0,
            split_drag_start_ratio: DEFAULT_SPLIT_RATIO,
            _subscriptions: vec![editor_subscription, grid_subscription],
        };
        this.load_history(cx);
        this
    }

    fn execute_query(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) {
        let manager = self.connection_manager.read(cx);
        if manager.active_connection().is_none() {
            self.query_error = Some("No active connection".to_string());
            self.execution_state = ExecutionState::Failed;
            cx.notify();
            return;
        }
        if manager.active_db_connection().is_none() {
            self.query_error = Some("Connection is not active".to_string());
            self.execution_state = ExecutionState::Failed;
            cx.notify();
            return;
        }

        let sql = self.editor.read(cx).text(cx);
        let sql = sql.trim().to_string();
        if sql.is_empty() {
            return;
        }

        self.query_history.push(&sql);
        self.save_history(cx);
        self.execution_state = ExecutionState::Executing;
        self.query_error = None;
        self.current_page = 0;
        self.current_query = Some(sql.clone());

        self.result_grid.update(cx, |grid, cx| {
            grid.clear(cx);
        });
        cx.notify();

        let rows_per_page = self.rows_per_page;
        let query_task =
            self.connection_manager
                .read(cx)
                .execute_query(sql, rows_per_page, 0, cx);

        let is_explain = self.is_explain;
        self.is_explain = false;

        self.query_task = cx.spawn(async move |this, cx| {
            let result = query_task.await;

            this.update(cx, |this, cx| {
                match result {
                    Ok(result) => {
                        let result = if is_explain {
                            parse_explain_to_table(result)
                        } else {
                            result
                        };
                        this.result_grid.update(cx, |grid, cx| {
                            grid.set_result(result, cx);
                        });
                        this.query_error = None;
                        this.execution_state = ExecutionState::Completed;
                    }
                    Err(error) => {
                        this.query_error = Some(format!("{:#}", error));
                        this.execution_state = ExecutionState::Failed;
                    }
                }
                cx.emit(ItemEvent::UpdateTab);
                cx.notify();
            })
            .log_err();
        });
    }

    fn cancel_query(&mut self, cx: &mut Context<Self>) {
        if self.execution_state != ExecutionState::Executing {
            return;
        }

        self.connection_manager.read(cx).interrupt_active();
        self.query_task = Task::ready(());
        self.execution_state = ExecutionState::Idle;
        cx.notify();
    }

    fn explain_query(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) {
        let sql = self.editor.read(cx).text(cx);
        let sql = sql.trim().to_string();
        if sql.is_empty() {
            return;
        }

        let db_type = self
            .connection_manager
            .read(cx)
            .active_entry()
            .map(|e| e.config.database_type.clone())
            .unwrap_or(DatabaseType::Sqlite);

        let explain_sql = match db_type {
            DatabaseType::PostgreSql => format!("EXPLAIN ANALYZE {}", sql),
            DatabaseType::MySql => format!("EXPLAIN {}", sql),
            DatabaseType::Sqlite => format!("EXPLAIN QUERY PLAN {}", sql),
        };

        self.is_explain = true;
        self.editor.update(cx, |editor, cx| {
            editor.set_text(explain_sql, window, cx);
        });
        self.execute_query(window, cx);
    }

    fn navigate_to_foreign_key(
        &mut self,
        table: String,
        column: String,
        value: String,
        cx: &mut Context<Self>,
    ) {
        if self.connection_manager.read(cx).active_db_connection().is_none() {
            return;
        }

        let db_type = self
            .connection_manager
            .read(cx)
            .active_entry()
            .map(|e| e.config.database_type.clone())
            .unwrap_or(DatabaseType::Sqlite);

        let quoted_table = quote_identifier(&table, &db_type);
        let quoted_column = quote_identifier(&column, &db_type);
        let sql = format!(
            "SELECT * FROM {} WHERE {} = '{}'",
            quoted_table,
            quoted_column,
            value.replace('\'', "''")
        );

        self.current_query = Some(sql.clone());
        self.current_page = 0;
        self.execution_state = ExecutionState::Executing;
        self.query_error = None;

        self.result_grid.update(cx, |grid, cx| {
            grid.clear(cx);
        });
        cx.notify();

        let rows_per_page = self.rows_per_page;
        let query_task = self
            .connection_manager
            .read(cx)
            .execute_query(sql, rows_per_page, 0, cx);

        self.query_task = cx.spawn(async move |this, cx| {
            let result = query_task.await;

            this.update(cx, |this, cx| {
                match result {
                    Ok(result) => {
                        this.result_grid.update(cx, |grid, cx| {
                            grid.set_result(result, cx);
                        });
                        this.query_error = None;
                        this.execution_state = ExecutionState::Completed;
                    }
                    Err(error) => {
                        this.query_error = Some(format!("{:#}", error));
                        this.execution_state = ExecutionState::Failed;
                    }
                }
                cx.emit(ItemEvent::UpdateTab);
                cx.notify();
            })
            .log_err();
        });
    }

    fn format_query(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) {
        let sql = self.editor.read(cx).text(cx);
        let formatted = sqlformat::format(
            &sql,
            &sqlformat::QueryParams::None,
            sqlformat::FormatOptions::default(),
        );
        self.editor.update(cx, |editor, cx| {
            editor.set_text(formatted, window, cx);
        });
    }

    fn toggle_pin(&mut self, cx: &mut Context<Self>) {
        self.is_pinned = !self.is_pinned;
        cx.emit(ItemEvent::UpdateTab);
        cx.notify();
    }

    #[allow(dead_code)]
    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }

    fn save_history(&self, cx: &App) {
        let connection_name = self
            .connection_manager
            .read(cx)
            .active_entry()
            .map(|e| e.config.name.clone())
            .unwrap_or_default();
        if connection_name.is_empty() {
            return;
        }

        let entries = self.query_history.entries();
        if entries.is_empty() {
            return;
        }

        let key = format!("database_query_history-db_history_{}", connection_name);
        if let Ok(json) = serde_json::to_string(&entries) {
            db::write_and_log(cx, move || KEY_VALUE_STORE.write_kvp(key, json));
        }
    }

    fn load_history(&mut self, cx: &App) {
        let connection_name = self
            .connection_manager
            .read(cx)
            .active_entry()
            .map(|e| e.config.name.clone())
            .unwrap_or_default();
        if connection_name.is_empty() {
            return;
        }

        let key = format!("database_query_history-db_history_{}", connection_name);
        if let Ok(Some(json)) = KEY_VALUE_STORE.read_kvp(&key) {
            if let Ok(entries) = serde_json::from_str::<Vec<String>>(&json) {
                for entry in entries {
                    self.query_history.push(&entry);
                }
            }
        }
    }

    fn execute_to_file(&mut self, cx: &mut Context<Self>) {
        let manager = self.connection_manager.read(cx);
        if manager.active_db_connection().is_none() {
            self.query_error = Some("No active connection".to_string());
            self.execution_state = ExecutionState::Failed;
            cx.notify();
            return;
        }

        let sql = self.editor.read(cx).text(cx);
        let sql = sql.trim().to_string();
        if sql.is_empty() {
            return;
        }

        let save_dialog = cx.prompt_for_new_path(
            &std::path::PathBuf::default(),
            Some("query_results.csv"),
        );

        let query_task = self.connection_manager.read(cx).execute_query(sql, 100_000, 0, cx);

        self.execution_state = ExecutionState::Executing;
        cx.notify();

        self.query_task = cx.spawn(async move |this, cx| {
            let path = match save_dialog.await {
                Ok(Ok(Some(path))) => path,
                _ => {
                    this.update(cx, |this, cx| {
                        this.execution_state = ExecutionState::Idle;
                        cx.notify();
                    }).ok();
                    return;
                }
            };

            let result = query_task.await;

            this.update(cx, |this, cx| {
                match result {
                    Ok(query_result) => {
                        let csv = database_core::generate_csv(&query_result);
                        match std::fs::write(&path, csv.as_bytes()) {
                            Ok(()) => {
                                this.query_error = None;
                                this.execution_state = ExecutionState::Completed;
                                log::info!("Results exported to {}", path.display());
                            }
                            Err(error) => {
                                this.query_error = Some(format!("Failed to write file: {:#}", error));
                                this.execution_state = ExecutionState::Failed;
                            }
                        }
                    }
                    Err(error) => {
                        this.query_error = Some(format!("{:#}", error));
                        this.execution_state = ExecutionState::Failed;
                    }
                }
                cx.notify();
            }).ok();
        });
    }

    fn parse_tab_name_from_sql(sql: &str) -> Option<String> {
        for line in sql.lines().take(5) {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("-- @name") {
                return Some(rest.trim().to_string());
            }
            if let Some(rest) = trimmed.strip_prefix("/* @name") {
                if let Some(name) = rest.strip_suffix("*/") {
                    return Some(name.trim().to_string());
                }
            }
        }
        None
    }

    fn fetch_current_page(&mut self, cx: &mut Context<Self>) {
        if self.connection_manager.read(cx).active_db_connection().is_none() {
            return;
        }
        let Some(sql) = self.current_query.clone() else {
            return;
        };

        let sql = self.apply_sort_to_query(&sql, cx);

        self.execution_state = ExecutionState::Executing;
        cx.notify();

        let offset = self.current_page * self.rows_per_page;
        let rows_per_page = self.rows_per_page;
        let query_task =
            self.connection_manager
                .read(cx)
                .execute_query(sql, rows_per_page, offset, cx);

        let page = self.current_page;
        self.query_task = cx.spawn(async move |this, cx| {
            let result = query_task.await;

            this.update(cx, |this, cx| {
                match result {
                    Ok(result) => {
                        let page_offset = page * this.rows_per_page;
                        this.result_grid.update(cx, |grid, cx| {
                            grid.set_result(result, cx);
                            grid.set_page(page, page_offset, cx);
                        });
                        this.query_error = None;
                        this.execution_state = ExecutionState::Completed;
                    }
                    Err(error) => {
                        this.query_error = Some(format!("{:#}", error));
                        this.execution_state = ExecutionState::Failed;
                    }
                }
                cx.notify();
            })
            .log_err();
        });
    }

    fn apply_sort_to_query(&self, sql: &str, cx: &App) -> String {
        let grid = self.result_grid.read(cx);
        let sort_columns = grid.sort_columns();
        let where_clause = grid.where_clause();
        let has_sort = !sort_columns.is_empty();
        let has_where = !where_clause.trim().is_empty();

        if !has_sort && !has_where {
            return sql.to_string();
        }

        let result = grid.result();

        let db_type = self
            .connection_manager
            .read(cx)
            .active_entry()
            .map(|e| e.config.database_type.clone())
            .unwrap_or(DatabaseType::Sqlite);

        let mut query = format!("SELECT * FROM ({}) AS _q", sql);

        if has_where {
            query.push_str(&format!(" WHERE {}", where_clause));
        }

        if has_sort {
            if let Some(result) = result {
                let order_parts: Vec<String> = sort_columns
                    .iter()
                    .filter_map(|(col_index, direction)| {
                        let col_name = result.columns.get(*col_index)?;
                        let quoted_col = quote_identifier(col_name, &db_type);
                        let dir = match direction {
                            SortDirection::Ascending => "ASC",
                            SortDirection::Descending => "DESC",
                        };
                        Some(format!("{} {}", quoted_col, dir))
                    })
                    .collect();

                if !order_parts.is_empty() {
                    query.push_str(&format!(" ORDER BY {}", order_parts.join(", ")));
                }
            }
        }

        query
    }

    fn total_row_count(&self, cx: &App) -> usize {
        self.result_grid
            .read(cx)
            .result()
            .and_then(|r| r.total_row_count)
            .map(|c| c as usize)
            .unwrap_or_else(|| {
                self.result_grid
                    .read(cx)
                    .result()
                    .map(|r| r.rows.len())
                    .unwrap_or(0)
            })
    }

    fn total_pages(&self, cx: &App) -> usize {
        let total_rows = self.total_row_count(cx);
        if total_rows == 0 {
            1
        } else {
            total_rows.div_ceil(self.rows_per_page)
        }
    }

    fn go_to_previous_page(&mut self, cx: &mut Context<Self>) {
        if self.current_page > 0 {
            self.current_page -= 1;
            self.fetch_current_page(cx);
        }
    }

    fn go_to_next_page(&mut self, cx: &mut Context<Self>) {
        let total_pages = self.total_pages(cx);
        if self.current_page + 1 < total_pages {
            self.current_page += 1;
            self.fetch_current_page(cx);
        }
    }

    fn render_toolbar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let is_executing = self.execution_state == ExecutionState::Executing;
        let has_result = self.result_grid.read(cx).result().is_some();

        h_flex()
            .w_full()
            .gap_1()
            .px_2()
            .py_1()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                Button::new("execute", "Execute")
                    .style(ButtonStyle::Filled)
                    .icon(IconName::PlayFilled)
                    .icon_size(IconSize::Small)
                    .disabled(is_executing)
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.execute_query(window, cx);
                    }))
                    .tooltip(Tooltip::text("Execute query (Ctrl+Enter)")),
            )
            .when(is_executing, |el| {
                el.child(
                    Button::new("cancel", "Cancel")
                        .style(ButtonStyle::Subtle)
                        .icon(IconName::Close)
                        .icon_size(IconSize::Small)
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.cancel_query(cx);
                        })),
                )
            })
            .child(
                Button::new("explain", "Explain")
                    .style(ButtonStyle::Subtle)
                    .icon(IconName::ListTree)
                    .icon_size(IconSize::Small)
                    .disabled(is_executing)
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.explain_query(window, cx);
                    }))
                    .tooltip(Tooltip::text("Run EXPLAIN on query")),
            )
            .child(
                Button::new("format", "Format")
                    .style(ButtonStyle::Subtle)
                    .icon(IconName::TextSnippet)
                    .icon_size(IconSize::Small)
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.format_query(window, cx);
                    }))
                    .tooltip(Tooltip::text("Format SQL (Cmd/Ctrl+Shift+F)")),
            )
            .child(
                Button::new("pin-tab", "")
                    .icon(if self.is_pinned { IconName::Unpin } else { IconName::Pin })
                    .icon_size(IconSize::Small)
                    .style(if self.is_pinned { ButtonStyle::Filled } else { ButtonStyle::Subtle })
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.toggle_pin(cx);
                    }))
                    .tooltip(Tooltip::text(if self.is_pinned { "Unpin tab" } else { "Pin tab" })),
            )
            .child(
                Button::new("execute-to-file", "")
                    .icon(IconName::Download)
                    .icon_size(IconSize::Small)
                    .style(ButtonStyle::Subtle)
                    .disabled(is_executing)
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.execute_to_file(cx);
                    }))
                    .tooltip(Tooltip::text("Execute and save results to file")),
            )
            .child(div().flex_grow())
            .when(has_result, |el| {
                el.child(
                    Button::new("copy_csv", "CSV")
                        .style(ButtonStyle::Subtle)
                        .icon(IconName::Copy)
                        .icon_size(IconSize::Small)
                        .on_click(cx.listener(|this, _, _window, cx| {
                            if let Some(result) = this.result_grid.read(cx).result() {
                                let csv = generate_csv(result);
                                cx.write_to_clipboard(gpui::ClipboardItem::new_string(csv));
                            }
                        }))
                        .tooltip(Tooltip::text("Copy results as CSV")),
                )
                .child(
                    Button::new("copy_json", "JSON")
                        .style(ButtonStyle::Subtle)
                        .icon(IconName::Copy)
                        .icon_size(IconSize::Small)
                        .on_click(cx.listener(|this, _, _window, cx| {
                            if let Some(result) = this.result_grid.read(cx).result() {
                                let json = generate_json(result);
                                cx.write_to_clipboard(gpui::ClipboardItem::new_string(json));
                            }
                        }))
                        .tooltip(Tooltip::text("Copy results as JSON")),
                )
                .child(
                    Button::new("copy_md", "MD")
                        .style(ButtonStyle::Subtle)
                        .icon(IconName::Copy)
                        .icon_size(IconSize::Small)
                        .on_click(cx.listener(|this, _, _window, cx| {
                            if let Some(result) = this.result_grid.read(cx).result() {
                                let md = generate_markdown(result);
                                cx.write_to_clipboard(gpui::ClipboardItem::new_string(md));
                            }
                        }))
                        .tooltip(Tooltip::text("Copy results as Markdown")),
                )
            })
    }

    fn render_results_area(&self, cx: &mut Context<Self>) -> impl IntoElement {
        if self.execution_state == ExecutionState::Executing {
            return div()
                .flex_grow()
                .items_center()
                .justify_center()
                .child(Label::new("Executing query...").color(Color::Muted))
                .into_any_element();
        }

        if let Some(error_text) = &self.query_error {
            return div()
                .flex_grow()
                .p_2()
                .child(
                    Label::new(SharedString::from(error_text.clone()))
                        .color(Color::Error)
                        .size(LabelSize::Small),
                )
                .into_any_element();
        }

        if self.result_grid.read(cx).result().is_none() {
            return div()
                .flex_grow()
                .items_center()
                .justify_center()
                .child(
                    Label::new("Run a query to see results")
                        .color(Color::Muted)
                        .size(LabelSize::Small),
                )
                .into_any_element();
        }

        let total_pages = self.total_pages(cx);
        let current_page = self.current_page;
        let has_prev = current_page > 0;
        let has_next = current_page + 1 < total_pages;

        let pagination_bar = h_flex()
            .w_full()
            .px_2()
            .py_1()
            .gap_1()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .justify_center()
            .items_center()
            .child(
                Button::new("prev_page", "")
                    .icon(IconName::ChevronLeft)
                    .icon_size(IconSize::Small)
                    .style(ButtonStyle::Subtle)
                    .disabled(!has_prev)
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.go_to_previous_page(cx);
                    })),
            )
            .child(
                Label::new(format!("Page {} / {}", current_page + 1, total_pages))
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .child(
                Button::new("next_page", "")
                    .icon(IconName::ChevronRight)
                    .icon_size(IconSize::Small)
                    .style(ButtonStyle::Subtle)
                    .disabled(!has_next)
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.go_to_next_page(cx);
                    })),
            );

        v_flex()
            .flex_grow()
            .child(self.result_grid.clone())
            .child(pagination_bar)
            .into_any_element()
    }

    fn render_split_handle(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("split-handle")
            .flex_none()
            .h(px(6.0))
            .w_full()
            .cursor(CursorStyle::ResizeUpDown)
            .bg(cx.theme().colors().border)
            .hover(|style| style.bg(cx.theme().colors().border_focused))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event: &gpui::MouseDownEvent, _window, cx| {
                    this.split_dragging = true;
                    this.split_drag_start_y = event.position.y.as_f32();
                    this.split_drag_start_ratio = this.split_ratio;
                    cx.notify();
                }),
            )
    }
}

impl Focusable for QueryEditorTab {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<ItemEvent> for QueryEditorTab {}

impl Render for QueryEditorTab {
    fn render(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        let split_ratio = self.split_ratio;
        let is_dragging = self.split_dragging;

        v_flex()
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .on_action(cx.listener(|this, _: &super::database_panel::ExecuteQuery, window, cx| {
                this.execute_query(window, cx);
            }))
            .on_action(cx.listener(|this, _: &super::database_panel::CancelQuery, _window, cx| {
                this.cancel_query(cx);
            }))
            .on_action(cx.listener(|this, _: &CancelQuery, _window, cx| {
                this.cancel_query(cx);
            }))
            .on_action(cx.listener(|this, _: &ExplainQuery, window, cx| {
                this.explain_query(window, cx);
            }))
            .on_action(cx.listener(|this, _: &FormatQuery, window, cx| {
                this.format_query(window, cx);
            }))
            .on_action(cx.listener(|this, _: &TogglePinTab, _window, cx| {
                this.toggle_pin(cx);
            }))
            .on_action(cx.listener(|this, _: &ExecuteToFile, _window, cx| {
                this.execute_to_file(cx);
            }))
            .when(is_dragging, |el| {
                el.on_mouse_move(cx.listener(
                    move |this, event: &gpui::MouseMoveEvent, window, cx| {
                        if this.split_dragging {
                            let total_height = window.viewport_size().height.as_f32();
                            if total_height > 0.0 {
                                let delta_y = event.position.y.as_f32() - this.split_drag_start_y;
                                let delta_ratio = delta_y / total_height;
                                let new_ratio = (this.split_drag_start_ratio + delta_ratio)
                                    .clamp(MIN_SPLIT_RATIO, MAX_SPLIT_RATIO);
                                this.split_ratio = new_ratio;
                                cx.notify();
                            }
                        }
                    },
                ))
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(|this, _, _window, cx| {
                        this.split_dragging = false;
                        cx.notify();
                    }),
                )
            })
            .child(
                div()
                    .id("query-editor-sql")
                    .flex_none()
                    .h(gpui::relative(split_ratio))
                    .min_h(px(60.0))
                    .overflow_y_scroll()
                    .p_2()
                    .child(self.editor.clone()),
            )
            .child(self.render_split_handle(cx))
            .child(self.render_toolbar(cx))
            .child(
                div()
                    .flex_grow()
                    .id("query-editor-results")
                    .overflow_hidden()
                    .child(self.render_results_area(cx)),
            )
    }
}

impl Item for QueryEditorTab {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        let sql = self.editor.read(cx).text(cx);
        let base_name = if let Some(name) = Self::parse_tab_name_from_sql(&sql) {
            name
        } else {
            self.tab_name.clone()
        };
        if self.is_pinned {
            SharedString::from(format!("[P] {}", base_name))
        } else {
            SharedString::from(base_name)
        }
    }

    fn tab_icon(&self, _window: &gpui::Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::DatabaseZap))
    }

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(ItemEvent)) {
        f(*event)
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("database query editor")
    }
}

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
            workspace.register_action(open_query_editor);
        },
    )
    .detach();
}

fn open_query_editor(
    workspace: &mut Workspace,
    _action: &OpenQueryEditor,
    window: &mut gpui::Window,
    cx: &mut Context<Workspace>,
) {
    let panel = workspace.panel::<super::database_panel::DatabasePanel>(cx);
    let Some(panel) = panel else {
        return;
    };

    let connection_manager = panel.read(cx).connection_manager(cx);

    let editor = Box::new(cx.new(|cx| {
        QueryEditorTab::new(connection_manager, None, None, window, cx)
    }));

    workspace.add_item_to_active_pane(editor, None, true, window, cx);
}

fn parse_explain_to_table(result: database_core::QueryResult) -> database_core::QueryResult {
    if result.columns.len() != 1 || result.rows.is_empty() {
        return result;
    }

    let raw_lines: Vec<String> = result
        .rows
        .iter()
        .filter_map(|row| row.first())
        .map(|cell| cell.to_string())
        .collect();

    if raw_lines.is_empty() {
        return result;
    }

    let first_line = raw_lines[0].trim();
    let is_postgres = first_line.contains("->") || first_line.starts_with("Seq Scan")
        || first_line.starts_with("Sort") || first_line.starts_with("Hash")
        || first_line.starts_with("Nested Loop") || first_line.starts_with("Index");

    if is_postgres {
        parse_postgres_explain(&raw_lines, &result)
    } else {
        result
    }
}

fn parse_postgres_explain(
    lines: &[String],
    original: &database_core::QueryResult,
) -> database_core::QueryResult {
    use database_core::CellValue;

    let columns = vec![
        "Operation".to_string(),
        "Object".to_string(),
        "Rows".to_string(),
        "Width".to_string(),
        "Cost".to_string(),
        "Actual Time".to_string(),
        "Actual Rows".to_string(),
    ];

    let mut rows = Vec::new();

    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let cleaned = trimmed.trim_start_matches("->").trim();

        let (operation, rest) = if let Some(paren_pos) = cleaned.find('(') {
            (cleaned[..paren_pos].trim().to_string(), &cleaned[paren_pos..])
        } else {
            (cleaned.to_string(), "")
        };

        let object = extract_explain_field(&operation, " on ");
        let (op_name, _) = if let Some(pos) = operation.find(" on ") {
            (operation[..pos].to_string(), &operation[pos..])
        } else {
            (operation.clone(), "")
        };

        let rows_est = extract_parenthesized_value(rest, "rows=");
        let width = extract_parenthesized_value(rest, "width=");
        let cost = extract_parenthesized_value(rest, "cost=");
        let actual_time = extract_parenthesized_value(rest, "actual time=");
        let actual_rows = extract_parenthesized_value(rest, "rows=")
            .filter(|_| rest.contains("actual"));

        rows.push(vec![
            CellValue::Text(op_name),
            if object.is_empty() { CellValue::Null } else { CellValue::Text(object) },
            rows_est.map(|v| CellValue::Text(v)).unwrap_or(CellValue::Null),
            width.map(|v| CellValue::Text(v)).unwrap_or(CellValue::Null),
            cost.map(|v| CellValue::Text(v)).unwrap_or(CellValue::Null),
            actual_time.map(|v| CellValue::Text(v)).unwrap_or(CellValue::Null),
            actual_rows.map(|v| CellValue::Text(v)).unwrap_or(CellValue::Null),
        ]);
    }

    if rows.is_empty() {
        return original.clone();
    }

    database_core::QueryResult {
        columns,
        rows,
        total_row_count: None,
        affected_rows: None,
        execution_time: original.execution_time,
    }
}

fn extract_explain_field(text: &str, prefix: &str) -> String {
    if let Some(pos) = text.to_lowercase().find(&prefix.to_lowercase()) {
        let after = &text[pos + prefix.len()..];
        after.split_whitespace().next().unwrap_or("").to_string()
    } else {
        String::new()
    }
}

fn extract_parenthesized_value(text: &str, key: &str) -> Option<String> {
    let lower = text.to_lowercase();
    let key_lower = key.to_lowercase();
    if let Some(pos) = lower.find(&key_lower) {
        let after = &text[pos + key.len()..];
        let value: String = after
            .chars()
            .take_while(|c| !matches!(c, ')' | ' ' | '\t'))
            .collect();
        if value.is_empty() {
            None
        } else {
            Some(value)
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tab_name_dash_dash() {
        let sql = "-- @name User Lookup\nSELECT * FROM users WHERE id = 1";
        assert_eq!(
            QueryEditorTab::parse_tab_name_from_sql(sql),
            Some("User Lookup".to_string())
        );
    }

    #[test]
    fn test_parse_tab_name_block_comment() {
        let sql = "/* @name Monthly Report */\nSELECT COUNT(*) FROM orders";
        assert_eq!(
            QueryEditorTab::parse_tab_name_from_sql(sql),
            Some("Monthly Report".to_string())
        );
    }

    #[test]
    fn test_parse_tab_name_none() {
        let sql = "SELECT * FROM users";
        assert_eq!(QueryEditorTab::parse_tab_name_from_sql(sql), None);
    }

    #[test]
    fn test_parse_tab_name_later_line() {
        let sql = "-- This is a query\n-- for testing\n-- @name Late Name\nSELECT 1";
        assert_eq!(
            QueryEditorTab::parse_tab_name_from_sql(sql),
            Some("Late Name".to_string())
        );
    }

    #[test]
    fn test_parse_tab_name_beyond_line_5() {
        let sql = "-- line 1\n-- line 2\n-- line 3\n-- line 4\n-- line 5\n-- @name Too Late\nSELECT 1";
        assert_eq!(QueryEditorTab::parse_tab_name_from_sql(sql), None);
    }

    #[test]
    fn test_parse_explain_postgres() {
        use database_core::{CellValue, QueryResult};
        use std::time::Duration;

        let result = QueryResult {
            columns: vec!["QUERY PLAN".to_string()],
            rows: vec![
                vec![CellValue::Text("Seq Scan on users  (cost=0.00..1.50 rows=50 width=120)".to_string())],
                vec![CellValue::Text("  ->  Index Scan on users_pkey  (cost=0.15..8.17 rows=1 width=120)".to_string())],
            ],
            total_row_count: None,
            affected_rows: None,
            execution_time: Duration::from_millis(5),
        };

        let parsed = parse_explain_to_table(result);
        assert_eq!(parsed.columns.len(), 7);
        assert_eq!(parsed.columns[0], "Operation");
        assert_eq!(parsed.columns[1], "Object");
        assert_eq!(parsed.rows.len(), 2);

        if let CellValue::Text(op) = &parsed.rows[0][0] {
            assert!(op.contains("Seq Scan"));
        } else {
            panic!("Expected Text for operation");
        }
    }

    #[test]
    fn test_parse_explain_non_postgres_passthrough() {
        use database_core::{CellValue, QueryResult};
        use std::time::Duration;

        let result = QueryResult {
            columns: vec!["id".to_string(), "name".to_string()],
            rows: vec![
                vec![CellValue::Integer(1), CellValue::Text("test".to_string())],
            ],
            total_row_count: None,
            affected_rows: None,
            execution_time: Duration::from_millis(1),
        };

        let parsed = parse_explain_to_table(result.clone());
        assert_eq!(parsed.columns.len(), 2);
        assert_eq!(parsed.rows.len(), 1);
    }

    #[test]
    fn test_extract_parenthesized_value() {
        assert_eq!(
            extract_parenthesized_value("(cost=0.00..1.50 rows=50 width=120)", "rows="),
            Some("50".to_string())
        );
        assert_eq!(
            extract_parenthesized_value("(cost=0.00..1.50 rows=50 width=120)", "width="),
            Some("120".to_string())
        );
        assert_eq!(
            extract_parenthesized_value("no match here", "rows="),
            None
        );
    }
}
