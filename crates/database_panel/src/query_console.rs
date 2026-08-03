use std::ops::Range;
use std::time::{Duration, Instant};

use editor::Editor;
use gpui::{
    AnyElement, App, Context, Entity, EventEmitter, FocusHandle, Focusable, Render, SharedString,
    Task, Window,
};
use gpui_tokio::Tokio;
use ui::{
    AbsoluteLength, ColumnWidthConfig, CommonAnimationExt as _, ResizableColumnsState, Table,
    TableInteractionState, TableResizeBehavior, Tooltip, prelude::*,
};
use workspace::{Item, Workspace};

use crate::{
    RunQuery,
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
            let editor = cx.new(|cx| {
                let mut editor = Editor::multi_line(window, cx);
                editor.set_placeholder_text("SELECT * FROM …", window, cx);
                if let Some(initial_query) = initial_query {
                    editor.set_text(initial_query, window, cx);
                }
                editor
            });
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
                run_task: None,
            }
        });
        workspace.add_item_to_active_pane(Box::new(console), None, true, window, cx);
    }

    fn run_query(&mut self, _: &RunQuery, _: &mut Window, cx: &mut Context<Self>) {
        let sql = self.editor.read(cx).text(cx);
        if sql.trim().is_empty() {
            return;
        }
        let config = self.config.clone();
        let database = self.database.clone();
        let started = Instant::now();
        self.state = QueryState::Running;
        let task = Tokio::spawn_result(cx, schema::run_query(config, database, sql));
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
                } else if result.truncated {
                    format!(
                        "first {} rows in {millis} ms (truncated)",
                        result.rows.len()
                    )
                } else {
                    format!("{} rows in {millis} ms", result.rows.len())
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
