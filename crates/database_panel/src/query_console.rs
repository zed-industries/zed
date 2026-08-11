use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;
use std::time::{Duration, Instant};

use collections::HashSet;
use editor::{CompletionProvider, Editor};
use fuzzy::StringMatchCandidate;
use gpui::{
    AnyElement, App, ClipboardItem, Context, DismissEvent, Entity, EventEmitter, FocusHandle,
    Focusable, MouseButton, MouseDownEvent, MouseMoveEvent, Pixels, Point, Render, SharedString,
    Subscription, Task, WeakEntity, Window, anchored, deferred, hsla,
};
use language::ToOffset as _;
use language::language_settings::SoftWrap;
use project::CompletionDisplayOptions;
use gpui_tokio::Tokio;
use ui::{
    AbsoluteLength, ColumnWidthConfig, CommonAnimationExt as _, ContextMenu, ResizableColumnsState,
    Table, TableInteractionState, TableResizeBehavior, Tooltip, prelude::*,
};
use workspace::{Item, Workspace};

use settings::Settings as _;

use crate::{
    DatabasePanelSettings, RunQuery, SaveQuery, SaveQueryModal,
    schema::{self, CellValue, ColumnInfo, ConnectionConfig, QueryResult},
};

const INSPECTOR_WIDTH_KEY: &str = "database_panel_inspector_width::v1";
const INSPECTOR_MIN_WIDTH: f32 = 220.0;
const INSPECTOR_MAX_WIDTH: f32 = 1200.0;

/// A workspace item bound to one connection (and optionally one database):
/// a SQL editor on top and the query results rendered as a table below, in the
/// style of PhpStorm's query console. Queries run on a fresh connection; for
/// SQLite the connection is read-only, for MariaDB the server's permissions
/// decide what the query may do.
pub struct QueryConsole {
    focus_handle: FocusHandle,
    workspace: WeakEntity<Workspace>,
    editor: Entity<Editor>,
    connection_name: SharedString,
    config: ConnectionConfig,
    database: Option<String>,
    state: QueryState,
    table_interaction_state: Entity<TableInteractionState>,
    column_widths: Option<Entity<ResizableColumnsState>>,
    /// The SQL of the last executed run, so page navigation replays it even
    /// after the editor content changed.
    last_sql: Option<String>,
    /// Holding the task means starting a new query cancels the in-flight one.
    run_task: Option<Task<()>>,
    /// The `(row, column)` result cell shown in the value inspector pane.
    selected_cell: Option<(usize, usize)>,
    /// Read-only soft-wrapped editor showing the selected cell's full value.
    inspector_editor: Entity<Editor>,
    inspector_width: Pixels,
    /// `(pointer x, width)` at the start of an inspector resize drag.
    inspector_resize: Option<(Pixels, Pixels)>,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    schema_index: Rc<RefCell<Option<SchemaIndex>>>,
    /// Result columns resolved against the schema index, cached per result.
    /// The flag records whether the index was loaded at computation time, so
    /// the cache is rebuilt once the index arrives.
    column_meta: Option<(bool, Vec<ColumnMeta>)>,
    /// The saved query this console is linked to: set when opened from the
    /// saved-query picker or after saving, and drives the edit/delete
    /// toolbar buttons and the tab title.
    saved_query_name: Option<String>,
}

/// Presentation metadata of one result column.
#[derive(Clone, Default)]
struct ColumnMeta {
    /// The schema's full column type (e.g. `varchar(255)`) when every schema
    /// column with that name agrees, else the driver's result type.
    type_label: Option<SharedString>,
    primary_key: bool,
    /// The referenced `table.column` when the column is a foreign key.
    foreign_key: Option<String>,
    kind: ValueKind,
}

/// Coarse value category derived from the column type, driving the result
/// cell colors.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum ValueKind {
    #[default]
    Other,
    Number,
    Date,
    Blob,
    Bool,
}

fn classify_type(type_label: Option<&str>) -> ValueKind {
    let Some(label) = type_label else {
        return ValueKind::Other;
    };
    let label = label.to_lowercase();
    // MariaDB reports booleans as tinyint(1), which must win over the
    // integer check.
    if label.contains("bool") || label.starts_with("tinyint(1)") {
        ValueKind::Bool
    } else if label.contains("blob") || label.contains("binary") {
        ValueKind::Blob
    } else if label.contains("date") || label.contains("time") || label == "year" {
        ValueKind::Date
    } else if label.contains("int")
        || label.contains("decimal")
        || label.contains("float")
        || label.contains("double")
        || label.contains("real")
        || label.contains("numeric")
        || label.contains("serial")
    {
        ValueKind::Number
    } else {
        ValueKind::Other
    }
}

fn date_color() -> Color {
    Color::Custom(hsla(330.0 / 360.0, 0.60, 0.70, 1.0))
}

fn cell_color(value: &CellValue, kind: ValueKind) -> Color {
    if value.is_null() {
        return Color::Muted;
    }
    match kind {
        ValueKind::Date => return date_color(),
        ValueKind::Blob => return Color::Success,
        ValueKind::Bool => return Color::Warning,
        ValueKind::Number | ValueKind::Other => {}
    }
    match value {
        CellValue::Bool(_) => Color::Warning,
        CellValue::Integer(_) | CellValue::UInteger(_) | CellValue::Real(_) => Color::Info,
        CellValue::Bytes(_) => Color::Success,
        _ => {
            if kind == ValueKind::Number {
                Color::Info
            } else {
                Color::Default
            }
        }
    }
}

enum QueryState {
    Idle,
    Running,
    Finished {
        result: QueryResult,
        duration: Duration,
    },
    Failed {
        error: SharedString,
    },
}

impl QueryConsole {
    pub fn open(
        workspace: &mut Workspace,
        connection_name: SharedString,
        config: ConnectionConfig,
        database: Option<String>,
        initial_query: Option<String>,
        auto_run: bool,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        Self::open_internal(
            workspace,
            connection_name,
            config,
            database,
            initial_query,
            auto_run,
            None,
            window,
            cx,
        );
    }

    /// Opens a console linked to the saved query `saved_query_name` and runs
    /// it immediately.
    pub fn open_saved(
        workspace: &mut Workspace,
        connection_name: SharedString,
        config: ConnectionConfig,
        database: Option<String>,
        sql: String,
        saved_query_name: String,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        Self::open_internal(
            workspace,
            connection_name,
            config,
            database,
            Some(sql),
            true,
            Some(saved_query_name),
            window,
            cx,
        );
    }

    fn open_internal(
        workspace: &mut Workspace,
        connection_name: SharedString,
        config: ConnectionConfig,
        database: Option<String>,
        initial_query: Option<String>,
        auto_run: bool,
        saved_query_name: Option<String>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let language_registry = workspace.project().read(cx).languages().clone();
        let weak_workspace = workspace.weak_handle();
        let console = cx.new(|cx| {
            let schema_index = Rc::new(RefCell::new(None));
            let editor = cx.new(|cx| {
                let mut editor = Editor::multi_line(window, cx);
                editor.set_placeholder_text("SELECT * FROM …", window, cx);
                if let Some(initial_query) = initial_query {
                    editor.set_text(initial_query, window, cx);
                }
                editor.set_completion_provider(Some(Rc::new(SqlCompletionProvider {
                    schema: schema_index.clone(),
                })));
                editor
            });
            let inspector_editor = cx.new(|cx| {
                let mut editor = Editor::multi_line(window, cx);
                editor.set_read_only(true);
                editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
                editor
            });
            Self::load_schema_index(&config, &database, schema_index.clone(), cx);
            // The SQL language ships as an extension; skip highlighting when
            // it is not installed.
            cx.spawn({
                let editor = editor.clone();
                async move |_, cx| {
                    let Ok(language) = language_registry.language_for_name("SQL").await else {
                        return;
                    };
                    editor.update(cx, |editor, cx| {
                        if let Some(buffer) = editor.buffer().read(cx).as_singleton() {
                            buffer.update(cx, |buffer, cx| buffer.set_language(Some(language), cx));
                        }
                    });
                }
            })
            .detach();
            Self::load_inspector_width(cx);
            Self {
                focus_handle: cx.focus_handle(),
                workspace: weak_workspace,
                editor,
                connection_name,
                config,
                database,
                state: QueryState::Idle,
                table_interaction_state: cx.new(|cx| TableInteractionState::new(cx)),
                column_widths: None,
                last_sql: None,
                run_task: None,
                selected_cell: None,
                inspector_editor,
                inspector_width: px(360.),
                inspector_resize: None,
                context_menu: None,
                schema_index,
                column_meta: None,
                saved_query_name,
            }
        });
        workspace.add_item_to_active_pane(Box::new(console.clone()), None, true, window, cx);
        if auto_run {
            console.update(cx, |console, cx| console.run_query(&RunQuery, window, cx));
        }
    }

    /// Restores the persisted inspector pane width, if any.
    fn load_inspector_width(cx: &mut Context<Self>) {
        let kvp = db::kvp::KeyValueStore::global(cx);
        let task = cx.background_spawn(async move {
            kvp.read_kvp(INSPECTOR_WIDTH_KEY)
                .ok()
                .flatten()
                .and_then(|value| value.parse::<f32>().ok())
        });
        cx.spawn(async move |this, cx| {
            if let Some(width) = task.await {
                this.update(cx, |this, cx| {
                    this.inspector_width =
                        px(width.clamp(INSPECTOR_MIN_WIDTH, INSPECTOR_MAX_WIDTH));
                    cx.notify();
                })
                .ok();
            }
        })
        .detach();
    }

    fn finish_inspector_resize(&mut self, cx: &mut Context<Self>) {
        if self.inspector_resize.take().is_none() {
            return;
        }
        let width = f32::from(self.inspector_width);
        let kvp = db::kvp::KeyValueStore::global(cx);
        cx.background_spawn(async move {
            kvp.write_kvp(INSPECTOR_WIDTH_KEY.to_string(), width.to_string())
                .await
        })
        .detach_and_log_err(cx);
        cx.notify();
    }

    /// Fetches the table and column names of the console's database in the
    /// background so [`SqlCompletionProvider`] can offer them. Without a
    /// selected MariaDB database there is no schema to index and completions
    /// fall back to keywords only.
    fn load_schema_index(
        config: &ConnectionConfig,
        database: &Option<String>,
        schema_index: Rc<RefCell<Option<SchemaIndex>>>,
        cx: &mut Context<Self>,
    ) {
        let index_database = match (config, database) {
            (ConnectionConfig::Sqlite { .. }, _) => "main".to_string(),
            (ConnectionConfig::MariaDb { .. }, Some(database)) => database.clone(),
            (ConnectionConfig::MariaDb { .. }, None) => return,
        };
        let task = Tokio::spawn_result(cx, {
            let config = config.clone();
            async move {
                let tables = schema::list_tables(config.clone(), index_database.clone()).await?;
                let columns = schema::list_all_columns(config, index_database).await?;
                anyhow::Ok((tables, columns))
            }
        });
        cx.spawn(async move |_, _| {
            let (tables, mut columns) = task.await?;
            let tables = tables
                .into_iter()
                .map(|table| {
                    let table_columns = columns.remove(&table.name).unwrap_or_default();
                    (table.name, table_columns)
                })
                .collect();
            *schema_index.borrow_mut() = Some(SchemaIndex { tables });
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn run_query(&mut self, _: &RunQuery, _: &mut Window, cx: &mut Context<Self>) {
        let sql = self.editor.read(cx).text(cx);
        if sql.trim().is_empty() {
            return;
        }
        self.last_sql = Some(sql.clone());
        self.run_at_offset(sql, 0, cx);
    }

    fn run_page(&mut self, offset: usize, cx: &mut Context<Self>) {
        let Some(sql) = self.last_sql.clone() else {
            return;
        };
        self.run_at_offset(sql, offset, cx);
    }

    fn run_at_offset(&mut self, sql: String, offset: usize, cx: &mut Context<Self>) {
        let config = self.config.clone();
        let database = self.database.clone();
        let page_size = DatabasePanelSettings::get_global(cx).query_page_size;
        let started = Instant::now();
        self.state = QueryState::Running;
        let task = Tokio::spawn_result(
            cx,
            schema::run_query(config, database, sql, offset, page_size),
        );
        self.run_task = Some(cx.spawn(async move |this, cx| {
            let query_result = task.await;
            this.update(cx, |this, cx| {
                match query_result {
                    Ok(result) => {
                        this.column_widths = Some(Self::column_widths_for(&result, cx));
                        // Row indexes point into the previous result set, so a
                        // kept selection would show a value from another row.
                        this.selected_cell = None;
                        this.column_meta = None;
                        this.state = QueryState::Finished {
                            result,
                            duration: started.elapsed(),
                        };
                    }
                    Err(error) => {
                        this.state = QueryState::Failed {
                            error: format!("{error:#}").into(),
                        };
                    }
                }
                cx.notify();
            })
            .ok();
        }));
        cx.notify();
    }

    fn column_widths_for(
        result: &QueryResult,
        cx: &mut Context<Self>,
    ) -> Entity<ResizableColumnsState> {
        let cols = result.columns.len().max(1);
        let mut widths = Vec::with_capacity(cols);
        for index in 0..cols {
            let mut chars = result
                .columns
                .get(index)
                .map_or(0, |column| column.name.len());
            for row in result.rows.iter().take(50) {
                if let Some(value) = row.get(index) {
                    chars = chars.max(value.display().len());
                }
            }
            let width = (chars as f32 * 8.0 + 24.0).clamp(80.0, 400.0);
            widths.push(AbsoluteLength::Pixels(px(width)));
        }
        cx.new(|_| {
            ResizableColumnsState::new(cols, widths, vec![TableResizeBehavior::Resizable; cols])
        })
    }

    fn status_text(&self) -> Option<(SharedString, Color)> {
        match &self.state {
            QueryState::Idle => None,
            QueryState::Running => Some(("Running…".into(), Color::Muted)),
            QueryState::Finished { result, duration } => {
                let millis = duration.as_millis();
                let text = if result.columns.is_empty() {
                    format!(
                        "{} rows affected in {millis} ms",
                        result.affected_rows.unwrap_or(0)
                    )
                } else if result.rows.is_empty() {
                    format!("no rows in {millis} ms")
                } else if result.offset == 0 && !result.has_more {
                    format!("{} rows in {millis} ms", result.rows.len())
                } else {
                    let first = result.offset + 1;
                    let last = result.offset + result.rows.len();
                    let more = if result.has_more { "+" } else { "" };
                    format!("rows {first}–{last}{more} in {millis} ms")
                };
                Some((text.into(), Color::Muted))
            }
            QueryState::Failed { .. } => Some(("Query failed".into(), Color::Error)),
        }
    }

    fn render_toolbar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let target = match &self.database {
            Some(database) => format!("{} · {database}", self.connection_name),
            None => self.connection_name.to_string(),
        };
        h_flex()
            .px_2()
            .py_1()
            .gap_2()
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        IconButton::new("run-query", IconName::PlayFilled)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Success)
                            .tooltip(Tooltip::text("Run Query (Ctrl+Enter)"))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.run_query(&RunQuery, window, cx)
                            })),
                    )
                    .map(|toolbar| match self.saved_query_name.clone() {
                        None => toolbar.child(
                            IconButton::new("save-query", IconName::Bookmark)
                                .icon_size(IconSize::Small)
                                .icon_color(Color::Muted)
                                .tooltip(Tooltip::text("Save Query…"))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.save_query(&SaveQuery, window, cx)
                                })),
                        ),
                        Some(name) => toolbar
                            .child(
                                IconButton::new("edit-saved-query", IconName::Pencil)
                                    .icon_size(IconSize::Small)
                                    .icon_color(Color::Muted)
                                    .tooltip(Tooltip::text(format!(
                                        "Edit Saved Query \"{name}\""
                                    )))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.save_query(&SaveQuery, window, cx)
                                    })),
                            )
                            .child(
                                IconButton::new("delete-saved-query", IconName::Trash)
                                    .icon_size(IconSize::Small)
                                    .icon_color(Color::Muted)
                                    .tooltip(Tooltip::text("Delete Saved Query"))
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.delete_saved_query(cx)
                                    })),
                            ),
                    })
                    .child(
                        Label::new(target)
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                            .single_line()
                            .truncate(),
                    ),
            )
            .child(
                h_flex()
                    .gap_2()
                    .children(self.status_text().map(|(text, color)| {
                        h_flex()
                            .gap_1()
                            .when(matches!(self.state, QueryState::Running), |this| {
                                this.child(
                                    Icon::new(IconName::LoadCircle)
                                        .size(IconSize::XSmall)
                                        .color(Color::Muted)
                                        .with_rotate_animation(2),
                                )
                            })
                            .child(Label::new(text).size(LabelSize::Small).color(color))
                    }))
                    .children(self.render_pagination(cx)),
            )
    }

    /// Previous/next page buttons, shown once a result set is paginated, in
    /// the style of PhpStorm's query console.
    fn render_pagination(&self, cx: &mut Context<Self>) -> Option<impl IntoElement> {
        let QueryState::Finished { result, .. } = &self.state else {
            return None;
        };
        if result.columns.is_empty() || (result.offset == 0 && !result.has_more) {
            return None;
        }
        let offset = result.offset;
        let page_len = result.rows.len();
        let has_more = result.has_more;
        let page_size = DatabasePanelSettings::get_global(cx).query_page_size;
        Some(
            h_flex()
                .gap_0p5()
                .child(
                    IconButton::new("query-previous-page", IconName::ChevronLeft)
                        .icon_size(IconSize::Small)
                        .disabled(offset == 0)
                        .tooltip(Tooltip::text("Previous Page"))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.run_page(offset.saturating_sub(page_size), cx)
                        })),
                )
                .child(
                    IconButton::new("query-next-page", IconName::ChevronRight)
                        .icon_size(IconSize::Small)
                        .disabled(!has_more)
                        .tooltip(Tooltip::text("Next Page"))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.run_page(offset + page_len, cx)
                        })),
                ),
        )
    }

    fn render_results(&self, cx: &mut Context<Self>) -> AnyElement {
        let centered_message = |message: SharedString, color: Color| {
            v_flex()
                .size_full()
                .items_center()
                .justify_center()
                .child(Label::new(message).size(LabelSize::Small).color(color))
                .into_any_element()
        };
        match &self.state {
            QueryState::Idle => centered_message("Run a query to see results".into(), Color::Muted),
            QueryState::Running => v_flex()
                .size_full()
                .items_center()
                .justify_center()
                .child(
                    Icon::new(IconName::LoadCircle)
                        .color(Color::Muted)
                        .with_rotate_animation(2),
                )
                .into_any_element(),
            QueryState::Failed { error } => div()
                .id("query-error")
                .size_full()
                .overflow_scroll()
                .p_3()
                .text_size(rems(0.875))
                .text_color(cx.theme().status().error)
                .child(error.clone())
                .into_any_element(),
            QueryState::Finished { result, .. } => {
                if result.columns.is_empty() {
                    let affected = result.affected_rows.unwrap_or(0);
                    return centered_message(
                        format!("{affected} rows affected").into(),
                        Color::Muted,
                    );
                }
                if result.rows.is_empty() {
                    return centered_message("No rows returned".into(), Color::Muted);
                }
                let Some(column_widths) = self.column_widths.clone() else {
                    return centered_message("No rows returned".into(), Color::Muted);
                };
                let cols = result.columns.len();
                let headers: Vec<AnyElement> = result
                    .columns
                    .iter()
                    .enumerate()
                    .map(|(index, column)| {
                        div()
                            .id(("query-header", index))
                            .px_1()
                            .whitespace_nowrap()
                            .overflow_hidden()
                            .child(
                                Label::new(SharedString::from(column.name.clone()))
                                    .size(LabelSize::Small)
                                    .color(Color::Default),
                            )
                            .when_some(
                                self.column_meta(index)
                                    .and_then(|meta| meta.type_label.clone()),
                                |this, type_label| this.tooltip(Tooltip::text(type_label)),
                            )
                            .into_any_element()
                    })
                    .collect();
                Table::new(cols)
                    .interactable(&self.table_interaction_state)
                    .width_config(ColumnWidthConfig::Resizable(column_widths))
                    .header(headers)
                    .striped()
                    .uniform_list(
                        "query-results",
                        result.rows.len(),
                        cx.processor(|this, range: Range<usize>, _window, cx| {
                            let selected_cell = this.selected_cell;
                            let selected_bg = cx.theme().colors().element_selected;
                            let metas = this
                                .column_meta
                                .as_ref()
                                .map(|(_, metas)| metas.clone())
                                .unwrap_or_default();
                            let QueryState::Finished { result, .. } = &this.state else {
                                return Vec::new();
                            };
                            let cols = result.columns.len();
                            range
                                .filter_map(|row_index| {
                                    let row = result.rows.get(row_index)?;
                                    Some(
                                        row.iter()
                                            .enumerate()
                                            .map(|(column_index, value)| {
                                                let is_null = value.is_null();
                                                let selected = selected_cell
                                                    == Some((row_index, column_index));
                                                let meta = metas.get(column_index);
                                                let kind = meta
                                                    .map(|meta| meta.kind)
                                                    .unwrap_or_default();
                                                let key_icon =
                                                    meta.and_then(|meta| {
                                                        if meta.primary_key {
                                                            Some(Color::Warning)
                                                        } else if meta.foreign_key.is_some()
                                                            && !is_null
                                                        {
                                                            Some(Color::Info)
                                                        } else {
                                                            None
                                                        }
                                                    });
                                                let label = Label::new(SharedString::from(
                                                    value.display(),
                                                ))
                                                .size(LabelSize::Small)
                                                .color(cell_color(value, kind))
                                                .when(is_null, |label| label.italic())
                                                .single_line()
                                                .truncate();
                                                div()
                                                    .id((
                                                        "query-cell",
                                                        row_index * cols + column_index,
                                                    ))
                                                    .px_1()
                                                    .flex()
                                                    .flex_row()
                                                    .items_center()
                                                    .gap_1()
                                                    .overflow_hidden()
                                                    .when(selected, |this| this.bg(selected_bg))
                                                    .children(key_icon.map(|color| {
                                                        Icon::new(IconName::Key)
                                                            .size(IconSize::XSmall)
                                                            .color(color)
                                                            .into_any_element()
                                                    }))
                                                    .child(label)
                                                    .on_click(cx.listener(
                                                        move |this, _, window, cx| {
                                                            this.select_cell(
                                                                row_index,
                                                                column_index,
                                                                window,
                                                                cx,
                                                            );
                                                        },
                                                    ))
                                                    .on_mouse_down(
                                                        MouseButton::Right,
                                                        cx.listener(
                                                            move |this,
                                                                  event: &gpui::MouseDownEvent,
                                                                  window,
                                                                  cx| {
                                                                cx.stop_propagation();
                                                                this.deploy_cell_context_menu(
                                                                    event.position,
                                                                    row_index,
                                                                    column_index,
                                                                    window,
                                                                    cx,
                                                                );
                                                            },
                                                        ),
                                                    )
                                                    .into_any_element()
                                            })
                                            .collect::<Vec<_>>(),
                                    )
                                })
                                .collect()
                        }),
                    )
                    .into_any_element()
            }
        }
    }

    fn column_meta(&self, column_index: usize) -> Option<&ColumnMeta> {
        self.column_meta.as_ref()?.1.get(column_index)
    }

    /// Computes [`ColumnMeta`] for the current result when missing, and again
    /// once the schema index finishes loading.
    fn ensure_column_meta(&mut self) {
        let needs = match (&self.state, &self.column_meta) {
            (QueryState::Finished { .. }, None) => true,
            (QueryState::Finished { .. }, Some((schema_was_loaded, _))) => {
                !schema_was_loaded && self.schema_index.borrow().is_some()
            }
            _ => false,
        };
        if needs {
            self.column_meta = Some(self.compute_column_meta());
        }
    }

    /// Resolves each result column against the schema index by name. A column
    /// counts as resolved only when every schema column with that name agrees
    /// on type, primary key, and foreign key, so joins over homonymous
    /// columns never show wrong metadata.
    fn compute_column_meta(&self) -> (bool, Vec<ColumnMeta>) {
        let QueryState::Finished { result, .. } = &self.state else {
            return (false, Vec::new());
        };
        let schema = self.schema_index.borrow();
        let metas = result
            .columns
            .iter()
            .map(|column| {
                let mut resolved: Option<&ColumnInfo> = None;
                let mut ambiguous = false;
                if let Some(index) = schema.as_ref() {
                    for candidate in index
                        .tables
                        .iter()
                        .flat_map(|(_, columns)| columns.iter())
                        .filter(|candidate| candidate.name == column.name)
                    {
                        match resolved {
                            Some(existing)
                                if existing.data_type == candidate.data_type
                                    && existing.primary_key == candidate.primary_key
                                    && existing.foreign_key == candidate.foreign_key => {}
                            Some(_) => ambiguous = true,
                            None => resolved = Some(candidate),
                        }
                    }
                }
                let resolved = resolved.filter(|_| !ambiguous);
                let type_label = resolved
                    .map(|info| SharedString::from(info.data_type.clone()))
                    .or_else(|| {
                        column
                            .type_name
                            .as_ref()
                            .map(|name| SharedString::from(name.to_lowercase()))
                    });
                ColumnMeta {
                    kind: classify_type(type_label.as_deref()),
                    type_label,
                    primary_key: resolved.is_some_and(|info| info.primary_key),
                    foreign_key: resolved.and_then(|info| info.foreign_key.clone()),
                }
            })
            .collect();
        (schema.is_some(), metas)
    }

    fn select_cell(
        &mut self,
        row: usize,
        column: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let QueryState::Finished { result, .. } = &self.state else {
            return;
        };
        let Some(value) = result.rows.get(row).and_then(|values| values.get(column)) else {
            return;
        };
        let text = inspector_text(value);
        self.selected_cell = Some((row, column));
        self.inspector_editor.update(cx, |editor, cx| {
            editor.set_read_only(false);
            editor.set_text(text, window, cx);
            editor.set_read_only(true);
        });
        cx.notify();
    }

    /// The `table.column` a "Show Relation" on this cell would look up: the
    /// cell's column must be a resolved foreign key and the value non-null.
    fn relation_target(&self, row: usize, column: usize) -> Option<String> {
        let QueryState::Finished { result, .. } = &self.state else {
            return None;
        };
        let value = result.rows.get(row)?.get(column)?;
        if value.is_null() {
            return None;
        }
        self.column_meta(column)?.foreign_key.clone()
    }

    /// Opens a console on the row the foreign-key cell references and runs it.
    fn show_relation(
        &mut self,
        row: usize,
        column: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(target) = self.relation_target(row, column) else {
            return;
        };
        let QueryState::Finished { result, .. } = &self.state else {
            return;
        };
        let Some(value) = result.rows.get(row).and_then(|values| values.get(column)) else {
            return;
        };
        let (table, target_column) = match target.split_once('.') {
            Some((table, column)) => (table.to_string(), column.to_string()),
            None => (target, "id".to_string()),
        };
        let sql = format!(
            "SELECT * FROM {} WHERE {} = {};",
            schema::quote_ident(&self.config, &table),
            schema::quote_ident(&self.config, &target_column),
            schema::sql_literal(value),
        );
        let name = self.connection_name.clone();
        let config = self.config.clone();
        let database = self.database.clone();
        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                QueryConsole::open(workspace, name, config, database, Some(sql), true, window, cx);
            });
        }
    }

    /// Opens the save modal; for a console linked to a saved query this edits
    /// it (rename and refresh its SQL).
    fn save_query(&mut self, _: &SaveQuery, window: &mut Window, cx: &mut Context<Self>) {
        let sql = self.editor.read(cx).text(cx);
        if sql.trim().is_empty() {
            return;
        }
        let connection = self.connection_name.to_string();
        let database = self.database.clone();
        let original_name = self.saved_query_name.clone();
        let console = cx.weak_entity();
        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                SaveQueryModal::toggle(
                    workspace,
                    console,
                    connection,
                    database,
                    sql,
                    original_name,
                    window,
                    cx,
                );
            });
        }
    }

    pub(crate) fn set_saved_query_name(&mut self, name: Option<String>, cx: &mut Context<Self>) {
        self.saved_query_name = name;
        cx.notify();
    }

    /// Unlinks the console from its saved query and removes it from the
    /// persisted list.
    fn delete_saved_query(&mut self, cx: &mut Context<Self>) {
        let Some(name) = self.saved_query_name.take() else {
            return;
        };
        let kvp = db::kvp::KeyValueStore::global(cx);
        cx.background_spawn(async move {
            let mut queries = crate::saved_queries::read_saved_queries(&kvp);
            queries.retain(|existing| existing.name != name);
            crate::saved_queries::write_saved_queries(kvp, &queries).await
        })
        .detach_and_log_err(cx);
        cx.notify();
    }

    fn copy_cell(&self, row: usize, column: usize, cx: &mut Context<Self>) {
        let QueryState::Finished { result, .. } = &self.state else {
            return;
        };
        let Some(value) = result.rows.get(row).and_then(|values| values.get(column)) else {
            return;
        };
        cx.write_to_clipboard(ClipboardItem::new_string(value.display()));
    }

    fn copy_row_as_json(&self, row: usize, cx: &mut Context<Self>) {
        let QueryState::Finished { result, .. } = &self.state else {
            return;
        };
        let Some(values) = result.rows.get(row) else {
            return;
        };
        let object: serde_json::Map<String, serde_json::Value> = result
            .columns
            .iter()
            .zip(values)
            .map(|(column, value)| (column.name.clone(), value.to_json()))
            .collect();
        if let Ok(json) = serde_json::to_string_pretty(&serde_json::Value::Object(object)) {
            cx.write_to_clipboard(ClipboardItem::new_string(json));
        }
    }

    fn deploy_cell_context_menu(
        &mut self,
        position: Point<Pixels>,
        row: usize,
        column: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_cell(row, column, window, cx);
        let relation = self.relation_target(row, column);
        let console = cx.weak_entity();
        let context_menu = ContextMenu::build(window, cx, |menu, _, _| {
            menu.when_some(relation, |menu, target| {
                menu.entry(format!("Show Relation ({target})"), None, {
                    let console = console.clone();
                    move |window, cx| {
                        console
                            .update(cx, |this, cx| this.show_relation(row, column, window, cx))
                            .ok();
                    }
                })
                .separator()
            })
            .entry("Copy Cell", None, {
                let console = console.clone();
                move |_, cx| {
                    console
                        .update(cx, |this, cx| this.copy_cell(row, column, cx))
                        .ok();
                }
            })
            .entry("Copy Row as JSON", None, {
                let console = console.clone();
                move |_, cx| {
                    console
                        .update(cx, |this, cx| this.copy_row_as_json(row, cx))
                        .ok();
                }
            })
        });
        window.focus(&context_menu.focus_handle(cx), cx);
        let subscription = cx.subscribe(&context_menu, |this, _, _: &DismissEvent, cx| {
            this.context_menu.take();
            cx.notify();
        });
        self.context_menu = Some((context_menu, position, subscription));
        cx.notify();
    }

    /// The value inspector pane: the selected cell's column name and type on
    /// top of a read-only soft-wrapped editor holding the full value.
    fn render_inspector(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let (row, column) = self.selected_cell?;
        let QueryState::Finished { result, .. } = &self.state else {
            return None;
        };
        let column_info = result.columns.get(column)?;
        let value = result.rows.get(row)?.get(column)?;
        let resolved = self.column_meta(column);
        let mut parts = Vec::new();
        if let Some(type_label) = resolved.and_then(|meta| meta.type_label.clone()) {
            parts.push(type_label.to_string());
        }
        if resolved.is_some_and(|meta| meta.primary_key) {
            parts.push("primary key".to_string());
        }
        if let Some(target) = resolved.and_then(|meta| meta.foreign_key.as_ref()) {
            parts.push(format!("→ {target}"));
        }
        if let CellValue::Text(text) = value {
            parts.push(format!("{} chars", text.chars().count()));
        }
        let meta = SharedString::from(parts.join(" · "));
        let colors = cx.theme().colors();
        Some(
            h_flex()
                .h_full()
                .flex_shrink_0()
                .child(
                    div()
                        .id("inspector-resize-handle")
                        .w(px(5.))
                        .h_full()
                        .cursor_col_resize()
                        .hover(|style| style.bg(colors.border))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, event: &MouseDownEvent, _, cx| {
                                cx.stop_propagation();
                                this.inspector_resize =
                                    Some((event.position.x, this.inspector_width));
                            }),
                        ),
                )
                .child(
            v_flex()
                .w(self.inspector_width)
                .flex_shrink_0()
                .h_full()
                .border_l_1()
                .border_color(colors.border)
                .child(
                    h_flex()
                        .px_2()
                        .py_1()
                        .gap_1()
                        .justify_between()
                        .border_b_1()
                        .border_color(colors.border)
                        .child(
                            v_flex()
                                .min_w_0()
                                .child(
                                    Label::new(SharedString::from(column_info.name.clone()))
                                        .size(LabelSize::Small)
                                        .single_line()
                                        .truncate(),
                                )
                                .when(!meta.is_empty(), |this| {
                                    this.child(
                                        Label::new(meta)
                                            .size(LabelSize::XSmall)
                                            .color(Color::Muted)
                                            .single_line()
                                            .truncate(),
                                    )
                                }),
                        )
                        .child(
                            h_flex()
                                .gap_0p5()
                                .child(
                                    IconButton::new("inspector-copy", IconName::Copy)
                                        .icon_size(IconSize::Small)
                                        .icon_color(Color::Muted)
                                        .tooltip(Tooltip::text("Copy Value"))
                                        .on_click(cx.listener(move |this, _, _, cx| {
                                            this.copy_cell(row, column, cx);
                                        })),
                                )
                                .child(
                                    IconButton::new("inspector-close", IconName::Close)
                                        .icon_size(IconSize::Small)
                                        .icon_color(Color::Muted)
                                        .tooltip(Tooltip::text("Close"))
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.selected_cell = None;
                                            cx.notify();
                                        })),
                                ),
                        ),
                )
                .child(
                    div()
                        .flex_1()
                        .min_h_0()
                        .overflow_hidden()
                        .child(self.inspector_editor.clone()),
                ),
                )
                .into_any_element(),
        )
    }
}

/// The inspector shows JSON values pretty-printed, including JSON stored in
/// text columns (MariaDB's JSON columns are `LONGTEXT` under the hood).
fn inspector_text(value: &CellValue) -> String {
    match value {
        CellValue::Json(json) => {
            serde_json::to_string_pretty(json).unwrap_or_else(|_| json.to_string())
        }
        CellValue::Text(text) => {
            let trimmed = text.trim_start();
            if (trimmed.starts_with('{') || trimmed.starts_with('['))
                && let Ok(json) = serde_json::from_str::<serde_json::Value>(text)
                && let Ok(pretty) = serde_json::to_string_pretty(&json)
            {
                return pretty;
            }
            text.clone()
        }
        other => other.display(),
    }
}

impl Render for QueryConsole {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.ensure_column_meta();
        v_flex()
            .id("query-console")
            .key_context("QueryConsole")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .on_action(cx.listener(Self::run_query))
            .on_action(cx.listener(Self::save_query))
            .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _, cx| {
                let Some((start_x, start_width)) = this.inspector_resize else {
                    return;
                };
                if event.pressed_button != Some(MouseButton::Left) {
                    this.finish_inspector_resize(cx);
                    return;
                }
                let width = (start_width + start_x - event.position.x).clamp(
                    px(INSPECTOR_MIN_WIDTH),
                    px(INSPECTOR_MAX_WIDTH),
                );
                if width != this.inspector_width {
                    this.inspector_width = width;
                    cx.notify();
                }
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.finish_inspector_resize(cx);
                }),
            )
            .child(self.render_toolbar(cx))
            .child(
                div()
                    .h(rems(12.))
                    .flex_shrink_0()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .overflow_hidden()
                    .child(self.editor.clone()),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .min_h_0()
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .h_full()
                            .child(self.render_results(cx)),
                    )
                    .children(self.render_inspector(cx)),
            )
            .children(self.context_menu.as_ref().map(|(menu, position, _)| {
                deferred(
                    anchored()
                        .position(*position)
                        .anchor(gpui::Anchor::TopLeft)
                        .child(menu.clone()),
                )
                .with_priority(3)
            }))
    }
}

/// Tables and columns of the console's database, filled asynchronously after
/// the console opens. Used for completions and to resolve a result column's
/// schema type in the value inspector.
struct SchemaIndex {
    /// `(table name, columns)` pairs.
    tables: Vec<(String, Vec<ColumnInfo>)>,
}

const SQL_KEYWORDS: &[&str] = &[
    "SELECT", "FROM", "WHERE", "JOIN", "LEFT", "RIGHT", "INNER", "OUTER", "CROSS", "ON", "USING",
    "GROUP", "BY", "ORDER", "ASC", "DESC", "LIMIT", "OFFSET", "HAVING", "DISTINCT", "AS", "AND",
    "OR", "NOT", "NULL", "IS", "IN", "LIKE", "BETWEEN", "EXISTS", "UNION", "ALL", "CASE", "WHEN",
    "THEN", "ELSE", "END", "COUNT", "SUM", "AVG", "MIN", "MAX", "INSERT", "INTO", "VALUES",
    "UPDATE", "SET", "DELETE", "CREATE", "TABLE", "ALTER", "DROP", "INDEX", "VIEW", "EXPLAIN",
];

/// Completes SQL keywords plus the table and column names indexed by
/// [`QueryConsole::load_schema_index`]. After `table.` only that table's
/// columns are offered.
struct SqlCompletionProvider {
    schema: Rc<RefCell<Option<SchemaIndex>>>,
}

impl CompletionProvider for SqlCompletionProvider {
    fn completions(
        &self,
        buffer: &Entity<language::Buffer>,
        buffer_position: language::Anchor,
        _trigger: editor::CompletionContext,
        _window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Task<anyhow::Result<Vec<project::CompletionResponse>>> {
        let buffer = buffer.read(cx);
        let snapshot = buffer.text_snapshot();
        let offset = buffer_position.to_offset(buffer);
        let word_start = offset
            - snapshot
                .reversed_chars_at(offset)
                .take_while(|char| char.is_ascii_alphanumeric() || *char == '_')
                .map(char::len_utf8)
                .sum::<usize>();
        let replace_range = buffer.anchor_before(word_start)..buffer_position;

        // An identifier right before `table.` restricts candidates to that
        // table's columns. Quoted table names are not resolved.
        let mut qualifier = None;
        let mut before_word = snapshot.reversed_chars_at(word_start);
        if before_word.next() == Some('.') {
            let name: String = before_word
                .take_while(|char| char.is_ascii_alphanumeric() || *char == '_')
                .collect();
            if !name.is_empty() {
                qualifier = Some(name.chars().rev().collect::<String>());
            }
        }

        let mut items: Vec<String> = Vec::new();
        let schema = self.schema.borrow();
        match (schema.as_ref(), qualifier) {
            (Some(index), Some(qualifier)) => {
                let qualifier = qualifier.to_lowercase();
                if let Some((_, columns)) = index
                    .tables
                    .iter()
                    .find(|(table, _)| table.to_lowercase() == qualifier)
                {
                    items.extend(columns.iter().map(|column| column.name.clone()));
                }
            }
            (None, Some(_)) => {}
            (index, None) => {
                if let Some(index) = index {
                    let mut seen_columns = HashSet::default();
                    for (table, columns) in &index.tables {
                        items.push(table.clone());
                        for column in columns {
                            if seen_columns.insert(column.name.as_str()) {
                                items.push(column.name.clone());
                            }
                        }
                    }
                }
                items.extend(SQL_KEYWORDS.iter().map(|keyword| keyword.to_string()));
            }
        }
        drop(schema);

        let query: String = snapshot.text_for_range(word_start..offset).collect();
        let candidates: Vec<StringMatchCandidate> = items
            .iter()
            .enumerate()
            .map(|(ix, item)| StringMatchCandidate::new(ix, item))
            .collect();
        let executor = cx.background_executor().clone();
        let executor_for_fuzzy = executor.clone();
        executor.spawn(async move {
            let matches = fuzzy::match_strings(
                &candidates,
                &query,
                false,
                true,
                items.len(),
                &Default::default(),
                executor_for_fuzzy,
            )
            .await;
            let completions: Vec<project::Completion> = matches
                .iter()
                .take(50)
                .filter_map(|string_match| {
                    let item = items.get(string_match.candidate_id)?;
                    Some(project::Completion {
                        replace_range: replace_range.clone(),
                        label: language::CodeLabel::plain(item.clone(), None),
                        new_text: item.clone(),
                        documentation: None,
                        source: project::CompletionSource::Custom,
                        icon_path: None,
                        icon_color: None,
                        match_start: None,
                        snippet_deduplication_key: None,
                        insert_text_mode: None,
                        confirm: None,
                        group: None,
                    })
                })
                .collect();
            Ok(vec![project::CompletionResponse {
                completions,
                display_options: CompletionDisplayOptions {
                    dynamic_width: true,
                },
                is_incomplete: false,
            }])
        })
    }

    fn is_completion_trigger(
        &self,
        _buffer: &Entity<language::Buffer>,
        _position: language::Anchor,
        text: &str,
        _trigger_in_words: bool,
        _cx: &mut Context<Editor>,
    ) -> bool {
        text.chars().last().is_some_and(|last_char| {
            last_char.is_ascii_alphanumeric() || last_char == '_' || last_char == '.'
        })
    }
}

impl Focusable for QueryConsole {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl EventEmitter<()> for QueryConsole {}

impl Item for QueryConsole {
    type Event = ();

    fn tab_icon(&self, _: &Window, _: &App) -> Option<Icon> {
        Some(Icon::new(IconName::Database))
    }

    fn tab_content_text(&self, _: usize, _: &App) -> SharedString {
        if let Some(name) = &self.saved_query_name {
            return name.clone().into();
        }
        match &self.database {
            Some(database) => format!("{} · {database}", self.connection_name).into(),
            None => format!("Query: {}", self.connection_name).into(),
        }
    }
}
