use std::ops::Range;
use std::sync::Arc;
use std::time::{Duration, Instant};

use database_client::{DatabaseClient, QueryResult};
use editor::{Editor, EditorMode};
use gpui::{
    AnyElement, App, Context, Entity, EventEmitter, FocusHandle, Focusable, Task, WeakEntity,
    Window, actions,
};
use language::{Buffer, LanguageRegistry};
use multi_buffer::MultiBuffer;
use ui::{
    AbsoluteLength, ColumnWidthConfig, ResizableColumnsState, Table, TableInteractionState,
    TableResizeBehavior, Tooltip, prelude::*,
};
use util::ResultExt as _;
use workspace::{Workspace, item::Item};

actions!(
    database,
    [
        /// Runs the SQL in the query editor and shows the results below.
        RunQuery,
        /// Cancels the currently-running query.
        CancelQuery,
    ]
);

/// The maximum number of rows the UI requests for a single query. Results
/// beyond this are truncated server-side and flagged in the status line.
const UI_MAX_QUERY_ROWS: usize = 1000;

/// The default column width for the resizable results grid.
const COLUMN_WIDTH: f32 = 180.;

/// The in-flight state of the current query.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunState {
    Idle,
    Running,
    Error(String),
}

/// A workspace tab pairing a SQL editor with the results of the last run query.
///
/// Queries run against a fixed `connection`/`database` through the shared
/// [`DatabaseClient`]; only the most recent run is kept, and starting or
/// cancelling a run aborts any prior in-flight request.
pub struct SqlQueryView {
    focus_handle: FocusHandle,
    client: Arc<dyn DatabaseClient>,
    connection: String,
    database: String,
    editor: Entity<Editor>,
    run_state: RunState,
    result: Option<QueryResult>,
    /// The wall-clock duration of the last completed run, shown in the status line.
    elapsed: Option<Duration>,
    /// A short status message shown when there is no result to summarize
    /// (e.g. "Cancelled").
    status_message: Option<String>,
    interaction: Entity<TableInteractionState>,
    /// Recreated whenever the rendered column set changes so the grid keeps the
    /// right number of resize handles.
    column_widths: Option<Entity<ResizableColumnsState>>,
    _run_task: Option<Task<()>>,
}

impl SqlQueryView {
    pub fn new(
        client: Arc<dyn DatabaseClient>,
        connection: String,
        database: String,
        language_registry: Arc<LanguageRegistry>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let editor = cx.new(|cx| {
                let buffer = cx.new(|cx| {
                    let buffer = Buffer::local("", cx);
                    buffer.set_language_registry(language_registry.clone());
                    buffer
                });
                let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
                let mut editor = Editor::new(EditorMode::full(), buffer, None, window, cx);
                editor.set_placeholder_text("Enter SQL…", window, cx);
                editor
            });

            // SQL ships as an external extension; if it is not installed the
            // buffer stays plain text, which is fine for a scratch editor.
            cx.spawn(async move |this, cx| {
                let sql = language_registry.language_for_name("SQL").await.ok();
                if sql.is_none() {
                    log::debug!("SQL language unavailable; query editor stays plain text");
                }
                this.update(cx, |this: &mut Self, cx| {
                    if let Some(buffer) = this.editor.read(cx).buffer().read(cx).as_singleton() {
                        buffer.update(cx, |buffer, cx| buffer.set_language(sql, cx));
                    }
                })
                .log_err();
            })
            .detach();

            let interaction = cx.new(|cx| TableInteractionState::new(cx));
            Self {
                focus_handle: cx.focus_handle(),
                client,
                connection,
                database,
                editor,
                run_state: RunState::Idle,
                result: None,
                elapsed: None,
                status_message: None,
                interaction,
                column_widths: None,
                _run_task: None,
            }
        })
    }

    pub fn result(&self) -> Option<&QueryResult> {
        self.result.as_ref()
    }

    pub fn error(&self) -> Option<&str> {
        match &self.run_state {
            RunState::Error(message) => Some(message),
            _ => None,
        }
    }

    pub fn running(&self) -> bool {
        self.run_state == RunState::Running
    }

    /// Replaces the editor text, primarily so tests can seed a query.
    pub fn set_query_text(&mut self, text: impl Into<String>, window: &mut Window, cx: &mut App) {
        self.editor.update(cx, |editor, cx| {
            editor.set_text(text.into(), window, cx);
        });
    }

    /// Runs the current editor text against the connection's database. Empty
    /// queries are a no-op; starting a run aborts any prior in-flight request.
    pub fn run_query(&mut self, cx: &mut Context<Self>) {
        let sql = self.editor.read(cx).text(cx);
        if sql.trim().is_empty() {
            return;
        }

        self.run_state = RunState::Running;
        self.status_message = None;
        cx.notify();

        let client = self.client.clone();
        let database = self.database.clone();
        let task = gpui_tokio::Tokio::spawn_result(cx, async move {
            let started = Instant::now();
            let result = client.run_query(&database, &sql, UI_MAX_QUERY_ROWS).await;
            result.map(|result| (result, started.elapsed()))
        });

        self._run_task = Some(cx.spawn(async move |this, cx| {
            let outcome = task.await;
            this.update(cx, |this, cx| {
                match outcome {
                    Ok((result, elapsed)) => {
                        this.set_column_widths(result.columns.len(), cx);
                        this.result = Some(result);
                        this.elapsed = Some(elapsed);
                        this.run_state = RunState::Idle;
                    }
                    Err(error) => {
                        this.run_state = RunState::Error(error.to_string());
                    }
                }
                cx.notify();
            })
            .log_err();
        }));
    }

    /// Cancels the in-flight query: signals the server, aborts the local task,
    /// and returns to idle with a "Cancelled" status.
    pub fn cancel_query(&mut self, cx: &mut Context<Self>) {
        if self.run_state != RunState::Running {
            return;
        }
        // Drop the in-flight task so its abort guard fires.
        self._run_task = None;

        let client = self.client.clone();
        gpui_tokio::Tokio::spawn_result(cx, async move { client.cancel_running().await })
            .detach_and_log_err(cx);

        self.run_state = RunState::Idle;
        self.status_message = Some("Cancelled".to_string());
        cx.notify();
    }

    /// Recreates the resizable-columns state when the number of result columns
    /// changes, so the grid renders the correct number of resize handles.
    fn set_column_widths(&mut self, cols: usize, cx: &mut Context<Self>) {
        if cols == 0 {
            self.column_widths = None;
            return;
        }
        let matches = self
            .column_widths
            .as_ref()
            .is_some_and(|widths| widths.read(cx).cols() == cols);
        if matches {
            return;
        }
        self.column_widths = Some(cx.new(|_cx| {
            ResizableColumnsState::new(
                cols,
                vec![AbsoluteLength::Pixels(px(COLUMN_WIDTH)); cols],
                vec![TableResizeBehavior::Resizable; cols],
            )
        }));
    }

    fn status_text(&self) -> String {
        match &self.run_state {
            RunState::Running => "Running…".to_string(),
            RunState::Error(_) => String::new(),
            RunState::Idle => {
                if let Some(result) = &self.result {
                    let mut parts = Vec::new();
                    if let Some(tag) = &result.command_tag {
                        parts.push(tag.clone());
                    }
                    parts.push(format!("{} rows", result.rows.len()));
                    if result.truncated {
                        parts.push("(truncated)".to_string());
                    }
                    if let Some(elapsed) = self.elapsed {
                        parts.push(format!("{} ms", elapsed.as_millis()));
                    }
                    parts.join(" · ")
                } else if let Some(message) = &self.status_message {
                    message.clone()
                } else {
                    String::new()
                }
            }
        }
    }

    fn render_toolbar(&self, cx: &Context<Self>) -> AnyElement {
        let running = self.running();
        let status = self.status_text();

        h_flex()
            .w_full()
            .px_2()
            .py_1()
            .gap_2()
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        IconButton::new("db-run-query", IconName::PlayFilled)
                            .icon_size(IconSize::Small)
                            .disabled(running)
                            .tooltip(move |_window, cx| {
                                Tooltip::for_action("Run Query", &RunQuery, cx)
                            })
                            .on_click(cx.listener(|this, _, _, cx| this.run_query(cx))),
                    )
                    .when(running, |this| {
                        this.child(
                            IconButton::new("db-cancel-query", IconName::Stop)
                                .icon_size(IconSize::Small)
                                .tooltip(Tooltip::text("Cancel Query"))
                                .on_click(cx.listener(|this, _, _, cx| this.cancel_query(cx))),
                        )
                    }),
            )
            .child(
                Label::new(status)
                    .color(Color::Muted)
                    .size(LabelSize::Small),
            )
            .into_any_element()
    }

    fn render_results(&self, cx: &Context<Self>) -> AnyElement {
        if let RunState::Error(message) = &self.run_state {
            return v_flex()
                .size_full()
                .p_4()
                .child(Label::new(message.clone()).color(Color::Error))
                .into_any_element();
        }

        let Some(result) = self.result.clone() else {
            return v_flex()
                .size_full()
                .items_center()
                .justify_center()
                .child(Label::new("Run a query to see results").color(Color::Muted))
                .into_any_element();
        };

        if result.columns.is_empty() {
            let summary = result
                .command_tag
                .unwrap_or_else(|| "Query completed".to_string());
            return v_flex()
                .size_full()
                .items_center()
                .justify_center()
                .child(Label::new(summary).color(Color::Muted))
                .into_any_element();
        }

        let Some(widths) = self.column_widths.clone() else {
            return v_flex().into_any_element();
        };

        let headers: Vec<AnyElement> = result
            .columns
            .iter()
            .map(|column| Label::new(column.clone()).into_any_element())
            .collect();

        let column_count = result.columns.len();
        let rows = Arc::new(result.rows);

        Table::new(column_count)
            .interactable(&self.interaction)
            .striped()
            .width_config(ColumnWidthConfig::Resizable(widths))
            .header(headers)
            .uniform_list(
                "db-query-rows",
                rows.len(),
                cx.processor(move |_this, range: Range<usize>, _window, _cx| {
                    range
                        .filter_map(|row_index| {
                            let row = rows.get(row_index)?;
                            let cells: Vec<AnyElement> = (0..column_count)
                                .map(|col| render_cell(row.get(col).and_then(|cell| cell.clone())))
                                .collect();
                            Some(cells)
                        })
                        .collect()
                }),
            )
            .into_any_element()
    }
}

impl Render for SqlQueryView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("SqlQueryEditor")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(|this, _: &RunQuery, _, cx| this.run_query(cx)))
            .on_action(cx.listener(|this, _: &CancelQuery, _, cx| this.cancel_query(cx)))
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(
                div()
                    .h(rems(12.))
                    .w_full()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .overflow_hidden()
                    .child(self.editor.clone()),
            )
            .child(self.render_toolbar(cx))
            .child(
                v_flex()
                    .flex_1()
                    .size_full()
                    .overflow_hidden()
                    .child(self.render_results(cx)),
            )
    }
}

impl Focusable for SqlQueryView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl EventEmitter<()> for SqlQueryView {}

impl Item for SqlQueryView {
    type Event = ();

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::DatabaseZap))
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        format!("SQL: {}/{}", self.connection, self.database).into()
    }

    fn is_dirty(&self, _cx: &App) -> bool {
        false
    }
}

/// Renders a single result cell, showing a muted italic `NULL` for absent values.
fn render_cell(value: Option<String>) -> AnyElement {
    match value {
        Some(value) => div()
            .w_full()
            .whitespace_nowrap()
            .text_ellipsis()
            .child(value)
            .into_any_element(),
        None => div()
            .w_full()
            .child(Label::new("NULL").color(Color::Muted).italic())
            .into_any_element(),
    }
}

/// Opens a new SQL query tab in the workspace's active pane.
pub fn open_sql_query_tab(
    workspace: &WeakEntity<Workspace>,
    client: Arc<dyn DatabaseClient>,
    connection: String,
    database: String,
    language_registry: Arc<LanguageRegistry>,
    window: &mut Window,
    cx: &mut App,
) {
    workspace
        .update(cx, |workspace, cx| {
            let view =
                SqlQueryView::new(client, connection, database, language_registry, window, cx);
            workspace.active_pane().update(cx, |pane, cx| {
                pane.add_item(Box::new(view), true, true, None, window, cx);
            });
        })
        .log_err();
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use database_client::DatabaseClient;
    use database_client::fake::FakeDatabaseClient;
    use gpui::{TestAppContext, VisualTestContext};
    use language::LanguageRegistry;

    use super::{RunState, SqlQueryView};

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            gpui_tokio::init(cx);
            crate::init(cx);
        });
    }

    fn language_registry(cx: &mut VisualTestContext) -> Arc<LanguageRegistry> {
        Arc::new(LanguageRegistry::test(cx.executor()))
    }

    /// Drives the deterministic scheduler while giving the real tokio runtime a
    /// chance to complete cross-thread work, until `condition` holds or a bound
    /// is reached. Requires `cx.executor().allow_parking()`.
    async fn wait_until(
        cx: &mut VisualTestContext,
        condition: impl Fn(&mut VisualTestContext) -> bool,
    ) {
        for _ in 0..200 {
            cx.run_until_parked();
            if condition(cx) {
                return;
            }
            cx.background_executor
                .timer(std::time::Duration::from_millis(5))
                .await;
        }
        cx.run_until_parked();
        assert!(
            condition(cx),
            "condition did not become true within the time bound"
        );
    }

    #[gpui::test]
    async fn run_executes_and_stores_result(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let registry = language_registry(cx);
        let view = cx.update(|window, cx| {
            SqlQueryView::new(client, "local".into(), "app".into(), registry, window, cx)
        });

        view.update_in(cx, |view, window, cx| {
            view.set_query_text("select 1", window, cx);
            view.run_query(cx);
        });

        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.result().is_some())
        })
        .await;

        view.read_with(cx, |view, _| {
            assert!(view.result().is_some(), "result should be stored");
            assert_eq!(view.error(), None);
            assert!(!view.running());
        });
        assert!(
            fake.calls()
                .iter()
                .any(|call| call.starts_with("run_query app")),
            "run_query should have been called: {:?}",
            fake.calls()
        );
    }

    #[gpui::test]
    async fn run_error_surfaces_message(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::with_error("syntax error"));
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let registry = language_registry(cx);
        let view = cx.update(|window, cx| {
            SqlQueryView::new(client, "local".into(), "app".into(), registry, window, cx)
        });

        view.update_in(cx, |view, window, cx| {
            view.set_query_text("select bogus", window, cx);
            view.run_query(cx);
        });

        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.error().is_some())
        })
        .await;

        view.read_with(cx, |view, _| {
            let message = view.error().expect("error should be surfaced");
            assert!(
                message.contains("syntax error"),
                "unexpected error message: {message}"
            );
            assert!(view.result().is_none());
        });
    }

    #[gpui::test]
    async fn cancel_calls_client(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let registry = language_registry(cx);
        let view = cx.update(|window, cx| {
            SqlQueryView::new(client, "local".into(), "app".into(), registry, window, cx)
        });

        // Put the view into the running state, then cancel.
        view.update_in(cx, |view, window, cx| {
            view.set_query_text("select 1", window, cx);
            view.run_query(cx);
            assert!(view.running(), "view should be running after run_query");
            view.cancel_query(cx);
        });

        view.read_with(cx, |view, _| {
            assert_eq!(view.run_state, RunState::Idle);
            assert!(!view.running());
        });

        wait_until(cx, |cx| {
            let _ = cx;
            fake.calls().iter().any(|call| call == "cancel_running")
        })
        .await;

        assert!(
            fake.calls().iter().any(|call| call == "cancel_running"),
            "cancel_running should have been called: {:?}",
            fake.calls()
        );
    }
}
