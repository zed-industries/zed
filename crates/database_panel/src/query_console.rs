use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;
use std::time::{Duration, Instant};

use collections::HashSet;
use editor::{CompletionProvider, Editor};
use fuzzy::StringMatchCandidate;
use gpui::{
    AnyElement, App, Context, Entity, EventEmitter, FocusHandle, Focusable, Render, SharedString,
    Task, Window,
};
use language::ToOffset as _;
use project::CompletionDisplayOptions;
use gpui_tokio::Tokio;
use ui::{
    AbsoluteLength, ColumnWidthConfig, CommonAnimationExt as _, ResizableColumnsState, Table,
    TableInteractionState, TableResizeBehavior, Tooltip, prelude::*,
};
use workspace::{Item, Workspace};

use settings::Settings as _;

use crate::{
    DatabasePanelSettings, RunQuery,
    schema::{self, ConnectionConfig, QueryResult},
};

/// A workspace item bound to one connection (and optionally one database):
/// a SQL editor on top and the query results rendered as a table below, in the
/// style of PhpStorm's query console. Queries run on a fresh connection; for
/// SQLite the connection is read-only, for MariaDB the server's permissions
/// decide what the query may do.
pub struct QueryConsole {
    focus_handle: FocusHandle,
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
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let language_registry = workspace.project().read(cx).languages().clone();
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
            Self::load_schema_index(&config, &database, schema_index, cx);
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
            Self {
                focus_handle: cx.focus_handle(),
                editor,
                connection_name,
                config,
                database,
                state: QueryState::Idle,
                table_interaction_state: cx.new(|cx| TableInteractionState::new(cx)),
                column_widths: None,
                last_sql: None,
                run_task: None,
            }
        });
        workspace.add_item_to_active_pane(Box::new(console), None, true, window, cx);
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
                    let column_names = columns
                        .remove(&table.name)
                        .unwrap_or_default()
                        .into_iter()
                        .map(|column| column.name)
                        .collect();
                    (table.name, column_names)
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
            let mut chars = result.columns.get(index).map_or(0, |name| name.len());
            for row in result.rows.iter().take(50) {
                if let Some(value) = row.get(index) {
                    chars = chars.max(value.len());
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
                    .map(|name| {
                        div()
                            .px_1()
                            .whitespace_nowrap()
                            .overflow_hidden()
                            .child(
                                Label::new(SharedString::from(name.clone()))
                                    .size(LabelSize::Small)
                                    .color(Color::Default),
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
                        cx.processor(|this, range: Range<usize>, _window, _cx| {
                            let QueryState::Finished { result, .. } = &this.state else {
                                return Vec::new();
                            };
                            range
                                .filter_map(|row_index| {
                                    let row = result.rows.get(row_index)?;
                                    Some(
                                        row.iter()
                                            .map(|value| {
                                                let is_null = value == "NULL";
                                                div()
                                                    .px_1()
                                                    .whitespace_nowrap()
                                                    .text_ellipsis()
                                                    .overflow_hidden()
                                                    .child(
                                                        Label::new(SharedString::from(
                                                            value.clone(),
                                                        ))
                                                        .size(LabelSize::Small)
                                                        .color(if is_null {
                                                            Color::Muted
                                                        } else {
                                                            Color::Default
                                                        }),
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
}

impl Render for QueryConsole {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("query-console")
            .key_context("QueryConsole")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .on_action(cx.listener(Self::run_query))
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
            .child(div().flex_1().min_h_0().child(self.render_results(cx)))
    }
}

/// Table and column names of the console's database, filled asynchronously
/// after the console opens.
struct SchemaIndex {
    /// `(table name, column names)` pairs.
    tables: Vec<(String, Vec<String>)>,
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
                    items.extend(columns.iter().cloned());
                }
            }
            (None, Some(_)) => {}
            (index, None) => {
                if let Some(index) = index {
                    let mut seen_columns = HashSet::default();
                    for (table, columns) in &index.tables {
                        items.push(table.clone());
                        for column in columns {
                            if seen_columns.insert(column.as_str()) {
                                items.push(column.clone());
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
        match &self.database {
            Some(database) => format!("{} · {database}", self.connection_name).into(),
            None => format!("Query: {}", self.connection_name).into(),
        }
    }
}
