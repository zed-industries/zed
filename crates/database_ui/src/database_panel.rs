use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Context as _;
use collections::HashSet;
use db::kvp::KEY_VALUE_STORE;
use editor::{Editor, EditorEvent};
use fs::Fs;
use gpui::{
    actions, anchored, deferred, div, prelude::*, px, App, AsyncWindowContext, ClipboardItem,
    Context, Corner, DismissEvent, Entity, EventEmitter, ExternalPaths, FocusHandle, Focusable,
    MouseButton, MouseMoveEvent, MouseUpEvent, Pixels, Point, SharedString, Subscription, Task,
    UniformListScrollHandle, WeakEntity,
};
use panel::PanelHeader;
use project;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use ui::{
    prelude::*, Banner, Button, ButtonStyle, ContextMenu, Icon, IconName, Label,
    PopoverMenu, Severity, Tooltip,
};
use util::ResultExt as _;
use workspace::{
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};

use database_core::{
    ConnectionConfig, DatabaseType,
    NavigateResult, QueryHistory, QueryResult, quote_identifier,
    generate_csv, generate_json, generate_html, generate_markdown, generate_sql_ddl_dml,
    generate_sql_insert, generate_tsv,
};
use crate::connection_dialog::{ConnectionDialog, ShowConnectionDialog, CONNECTION_COLORS};
use crate::connection_manager::{ConnectionManager, ConnectionManagerEvent};
use crate::connection_ui::{AddConnectionFlow, ConnectionForm};
use crate::database_panel_settings::DatabasePanelSettings;
use crate::export_dialog::{ExportDialog, ShowExportDialog};
use crate::results_table::{MIN_COLUMN_WIDTH, ROWS_PER_PAGE_OPTIONS, SortDirection, TableConfig, render_results_table, render_status_bar};
use crate::schema_tree::{SchemaNodeId, flatten_schema, render_schema_tree};

const DATABASE_PANEL_KEY: &str = "DatabaseViewer";
const MAX_QUERY_HISTORY: usize = 100;

#[allow(dead_code)]
enum ExportFormat {
    Csv,
    Tsv,
    Json,
    SqlInsert,
    SqlDdlDml,
    Markdown,
    Html,
}

actions!(
    database_panel,
    [
        /// Toggles the database panel.
        Toggle,
        /// Toggles focus on the database panel.
        ToggleFocus,
        /// Executes the current SQL query.
        ExecuteQuery,
        /// Cancels the currently executing query.
        CancelQuery,
        /// Copies the selected cell value.
        CopyCellValue,
        /// Copies all values of the selected row.
        CopyRowValues,
        /// Copies all results as CSV.
        CopyAllResultsAsCsv,
        /// Copies all results as JSON.
        CopyAllResultsAsJson,
        /// Moves grid selection left.
        GridMoveLeft,
        /// Moves grid selection right.
        GridMoveRight,
        /// Runs EXPLAIN on the current query.
        ExplainQuery,
        /// Navigates to the previous query history entry.
        PreviousHistoryEntry,
        /// Navigates to the next query history entry.
        NextHistoryEntry,
        /// Exports results as CSV to a file.
        ExportResultsCsv,
        /// Exports results as JSON to a file.
        ExportResultsJson,
        /// Saves the current query as a favorite.
        SaveQuery,
        /// Shows the saved queries list.
        ShowSavedQueries,
        /// Opens the native file picker to select SQLite files.
        OpenSqliteFile,
        /// Shows the PostgreSQL connection string form.
        ConnectPostgresql,
        /// Shows the MySQL connection string form.
        ConnectMysql,
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        register(workspace);
    })
    .detach();
}

fn register(workspace: &mut Workspace) {
    workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
        workspace.toggle_panel_focus::<DatabasePanel>(window, cx);
    });
    workspace.register_action(|workspace, _: &Toggle, window, cx| {
        if !workspace.toggle_panel_focus::<DatabasePanel>(window, cx) {
            workspace.close_panel::<DatabasePanel>(window, cx);
        }
    });
    workspace.register_action(|workspace, _: &ShowConnectionDialog, window, cx| {
        workspace.toggle_modal(window, cx, |window, cx| {
            ConnectionDialog::new(window, cx)
        });
    });
    workspace.register_action(|workspace, _: &ShowExportDialog, window, cx| {
        let panel = workspace.panel::<DatabasePanel>(cx);
        if let Some(panel) = panel {
            panel.update(cx, |panel, cx| {
                panel.show_export_dialog(window, cx);
            });
        }
    });
}

#[derive(Clone, Serialize, Deserialize)]
struct SavedQuery {
    name: String,
    sql: String,
}

#[derive(Clone)]
struct ResultTab {
    label: String,
    query: String,
    result: QueryResult,
    page: usize,
    sort_columns: Vec<(usize, SortDirection)>,
    column_widths: Vec<f32>,
}

#[derive(Serialize, Deserialize)]
struct SerializedDatabasePanel {
    width: Option<Pixels>,
    #[serde(default)]
    connections: Vec<ConnectionConfig>,
    #[serde(default)]
    active_connection: Option<usize>,
    #[serde(default)]
    expanded_nodes: Vec<SchemaNodeId>,
    #[serde(default = "default_rows_per_page")]
    rows_per_page: usize,
    #[serde(default)]
    query_history: Vec<String>,
    #[serde(default)]
    saved_queries: Vec<SavedQuery>,
    #[serde(default = "default_true")]
    connections_section_expanded: bool,
}

fn default_rows_per_page() -> usize {
    50
}

fn default_true() -> bool {
    true
}

pub struct DatabasePanel {
    focus_handle: FocusHandle,
    width: Option<Pixels>,
    fs: Arc<dyn Fs>,
    _workspace: WeakEntity<Workspace>,

    connection_manager: Entity<ConnectionManager>,
    connections_section_expanded: bool,

    schema_flattened_nodes: Vec<crate::schema_tree::FlattenedNode>,
    expanded_nodes: HashSet<SchemaNodeId>,
    selected_node: Option<usize>,
    schema_scroll_handle: UniformListScrollHandle,
    schema_filter_editor: Entity<Editor>,
    schema_filter: String,

    query_editor: Entity<Editor>,
    current_query: Option<String>,
    query_result: Option<QueryResult>,
    query_error: Option<String>,
    results_scroll_handle: UniformListScrollHandle,

    current_page: usize,
    rows_per_page: usize,

    sort_columns: Vec<(usize, SortDirection)>,
    selected_cell: Option<(usize, usize)>,
    column_widths: Vec<f32>,

    query_history: QueryHistory,

    expanded_cell: Option<(String, String)>,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,

    is_loading: bool,
    query_task: Task<()>,

    resizing_column: Option<(usize, f32)>,

    result_tabs: Vec<ResultTab>,
    active_result_tab: Option<usize>,

    saved_queries: Vec<SavedQuery>,
    show_saved_queries: bool,

    connection_form: ConnectionForm,
    connected_paths: HashSet<PathBuf>,
    serialization_key: Option<String>,

    _subscriptions: Vec<Subscription>,
}

impl DatabasePanel {
    fn new(
        workspace: &mut Workspace,
        window: &mut gpui::Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let fs = workspace.app_state().fs.clone();
        let project = workspace.project().clone();
        let serialization_key = Self::serialization_key(workspace);

        let connection_form = ConnectionForm::new(window, cx);

        let panel = cx.new(|cx| {
            let focus_handle = cx.focus_handle();

            let connection_manager = cx.new(|_cx| ConnectionManager::new());

            let query_editor = cx.new(|cx| {
                let mut editor = Editor::auto_height(3, 10, window, cx);
                editor.set_placeholder_text("Enter SQL query...", window, cx);
                editor
            });

            let schema_filter_editor = cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_placeholder_text("Filter tables...", window, cx);
                editor
            });

            let _settings_subscription = cx.observe_global::<SettingsStore>(|_, cx| {
                cx.notify();
            });

            let _filter_subscription =
                cx.subscribe(&schema_filter_editor, |this: &mut Self, _editor, event, cx| {
                    if matches!(event, EditorEvent::BufferEdited { .. }) {
                        this.schema_filter = this.schema_filter_editor.read(cx).text(cx).to_string();
                        this.rebuild_flattened_nodes(cx);
                        cx.notify();
                    }
                });

            let _connection_manager_subscription =
                cx.subscribe(&connection_manager, |this: &mut Self, _manager, event, cx| {
                    match event {
                        ConnectionManagerEvent::ConnectionAdded { .. }
                        | ConnectionManagerEvent::ConnectionRemoved { .. }
                        | ConnectionManagerEvent::ActiveConnectionChanged => {
                            this.rebuild_flattened_nodes(cx);
                            this.serialize(cx);
                            cx.notify();
                        }
                        ConnectionManagerEvent::SchemaUpdated { .. } => {
                            this.is_loading = false;
                            this.rebuild_flattened_nodes(cx);
                            this.serialize(cx);
                            cx.notify();
                        }
                        ConnectionManagerEvent::ConnectionFailed { .. } => {
                            this.is_loading = false;
                            cx.notify();
                        }
                        ConnectionManagerEvent::ConnectionLost { .. } => {
                            cx.notify();
                        }
                        ConnectionManagerEvent::Reconnected { .. } => {
                            this.rebuild_flattened_nodes(cx);
                            cx.notify();
                        }
                    }
                });

            let mut expanded_nodes = HashSet::default();
            expanded_nodes.insert(SchemaNodeId::TablesHeader);

            Self {
                focus_handle,
                width: None,
                fs,
                _workspace: workspace.weak_handle(),
                connection_manager,
                connections_section_expanded: true,
                schema_flattened_nodes: Vec::new(),
                expanded_nodes,
                selected_node: None,
                schema_scroll_handle: UniformListScrollHandle::new(),
                schema_filter_editor,
                schema_filter: String::new(),
                query_editor,
                current_query: None,
                query_result: None,
                query_error: None,
                results_scroll_handle: UniformListScrollHandle::new(),
                current_page: 0,
                rows_per_page: 50,
                sort_columns: Vec::new(),
                selected_cell: None,
                column_widths: Vec::new(),
                query_history: QueryHistory::new(MAX_QUERY_HISTORY),
                expanded_cell: None,
                context_menu: None,
                is_loading: false,
                query_task: Task::ready(()),
                resizing_column: None,
                result_tabs: Vec::new(),
                active_result_tab: None,
                saved_queries: Vec::new(),
                show_saved_queries: false,
                connection_form,
                _subscriptions: vec![_settings_subscription, _filter_subscription, _connection_manager_subscription],
                connected_paths: HashSet::default(),
                serialization_key,
            }
        });

        let project_subscription = cx.subscribe_in(&project, window, {
            let panel = panel.downgrade();
            move |workspace, _project, event, _window, cx| {
                if matches!(
                    event,
                    project::Event::WorktreeUpdatedEntries(_, _)
                        | project::Event::WorktreeAdded(_)
                ) {
                    let paths = detect_database_files(workspace, cx);
                    if let Some(panel) = panel.upgrade() {
                        panel.update(cx, |panel, cx| {
                            let new_paths: Vec<PathBuf> = paths
                                .into_iter()
                                .filter(|p| !panel.connected_paths.contains(p))
                                .collect();
                            if new_paths.is_empty() {
                                return;
                            }
                            for path in &new_paths {
                                panel.connected_paths.insert(path.clone());
                            }
                            panel.connect_detected_databases(new_paths, cx);
                        });
                    }
                }
            }
        });

        panel.update(cx, |panel, _cx| {
            panel._subscriptions.push(project_subscription);
        });

        panel
    }

    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> anyhow::Result<Entity<Self>> {
        let serialized_panel = match workspace
            .read_with(&cx, |workspace, _| Self::serialization_key(workspace))
            .ok()
            .flatten()
        {
            Some(serialization_key) => cx
                .background_spawn(async move { KEY_VALUE_STORE.read_kvp(&serialization_key) })
                .await
                .context("reading database panel from key value store")
                .log_err()
                .flatten()
                .map(|panel| serde_json::from_str::<SerializedDatabasePanel>(&panel))
                .transpose()
                .log_err()
                .flatten(),
            None => None,
        };

        workspace.update_in(&mut cx, |workspace, window, cx| {
            let panel = DatabasePanel::new(workspace, window, cx);
            if let Some(serialized) = serialized_panel {
                panel.update(cx, |panel, cx| {
                    panel.width = serialized.width;
                    panel.rows_per_page = serialized.rows_per_page;
                    panel.expanded_nodes = serialized.expanded_nodes.into_iter().collect();
                    panel.query_history = QueryHistory::with_entries(serialized.query_history, MAX_QUERY_HISTORY);
                    panel.saved_queries = serialized.saved_queries;
                    panel.connections_section_expanded = serialized.connections_section_expanded;

                    if !serialized.connections.is_empty() {
                        panel.connection_manager.update(cx, |manager, cx| {
                            manager.restore_connections(
                                serialized.connections,
                                serialized.active_connection,
                                cx,
                            );
                        });
                    }

                    cx.notify();
                });
            }
            panel
        })
    }

    fn serialization_key(workspace: &Workspace) -> Option<String> {
        workspace
            .database_id()
            .map(|id| i64::from(id).to_string())
            .or(workspace.session_id())
            .map(|id| format!("{}-{:?}", DATABASE_PANEL_KEY, id))
    }

    fn serialize(&self, cx: &mut Context<Self>) {
        let Some(serialization_key) = self.serialization_key.clone() else {
            return;
        };
        let manager = self.connection_manager.read(cx);
        let serialized_panel = SerializedDatabasePanel {
            width: self.width,
            connections: manager.connection_configs(),
            active_connection: manager.active_connection(),
            expanded_nodes: self.expanded_nodes.iter().cloned().collect(),
            rows_per_page: self.rows_per_page,
            query_history: self.query_history.entries().to_vec(),
            saved_queries: self.saved_queries.clone(),
            connections_section_expanded: self.connections_section_expanded,
        };
        let Some(serialized) = serde_json::to_string(&serialized_panel).log_err() else {
            return;
        };

        cx.background_spawn(async move {
            KEY_VALUE_STORE
                .write_kvp(serialization_key, serialized)
                .await
                .log_err();
        })
        .detach();
    }

    fn open_sqlite_file_picker(&mut self, cx: &mut Context<Self>) {
        let prompt = cx.prompt_for_paths(gpui::PathPromptOptions {
            files: true,
            directories: false,
            multiple: true,
            prompt: None,
        });

        cx.spawn(async move |this, cx| {
            let paths = match prompt.await {
                Ok(Ok(Some(paths))) => paths,
                _ => return,
            };

            this.update(cx, |this, cx| {
                for path in paths {
                    if this.connected_paths.contains(&path) {
                        continue;
                    }
                    this.connected_paths.insert(path.clone());

                    let name = path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| path.to_string_lossy().to_string());

                    let config = ConnectionConfig::sqlite(name, path);
                    let index = this.connection_manager.update(cx, |manager, cx| {
                        manager.add_connection(config, cx)
                    });
                    this.connection_manager.update(cx, |manager, cx| {
                        manager.set_active_connection(Some(index), cx);
                    });
                }
                this.connection_form.dismiss();
                cx.notify();
            })
            .log_err();
        })
        .detach();
    }

    fn show_postgres_connection_form(&mut self, cx: &mut Context<Self>) {
        self.connection_form.show_postgres_form();
        self.connections_section_expanded = true;
        cx.notify();
    }

    fn show_mysql_connection_form(&mut self, cx: &mut Context<Self>) {
        self.connection_form.show_mysql_form();
        self.connections_section_expanded = true;
        cx.notify();
    }

    fn submit_active_connection(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) {
        match &self.connection_form.active_flow {
            Some(AddConnectionFlow::PostgreSql) => self.submit_postgres_connection(window, cx),
            Some(AddConnectionFlow::MySql) => self.submit_mysql_connection(window, cx),
            None => {}
        }
    }

    fn submit_postgres_connection(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) {
        match self.connection_form.build_postgres_config(cx) {
            Ok(config) => {
                let index = self.connection_manager.update(cx, |manager, cx| {
                    manager.add_connection(config, cx)
                });
                self.connection_manager.update(cx, |manager, cx| {
                    manager.set_active_connection(Some(index), cx);
                });
                self.connection_form.clear(window, cx);
                self.connection_form.dismiss();
                cx.notify();
            }
            Err(error) => {
                self.connection_form.error = Some(error);
                cx.notify();
            }
        }
    }

    fn submit_mysql_connection(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) {
        match self.connection_form.build_mysql_config(cx) {
            Ok(config) => {
                let index = self.connection_manager.update(cx, |manager, cx| {
                    manager.add_connection(config, cx)
                });
                self.connection_manager.update(cx, |manager, cx| {
                    manager.set_active_connection(Some(index), cx);
                });
                self.connection_form.clear(window, cx);
                self.connection_form.dismiss();
                cx.notify();
            }
            Err(error) => {
                self.connection_form.error = Some(error);
                cx.notify();
            }
        }
    }

    fn show_connection_context_menu(
        &mut self,
        index: usize,
        position: Point<Pixels>,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) {
        let handle = cx.entity().downgrade();
        let manager = self.connection_manager.read(cx);
        let is_active = manager.active_connection() == Some(index);
        let connection_name = manager
            .connections()
            .get(index)
            .map(|e| e.config.display_name())
            .unwrap_or_default();

        let context_menu = ContextMenu::build(window, cx, move |menu, _window, _cx| {
            let mut menu = menu;
            if !is_active {
                menu = menu.entry("Set Active", None, {
                    let handle = handle.clone();
                    move |_window, cx| {
                        if let Some(panel) = handle.upgrade() {
                            panel.update(cx, |this, cx| {
                                this.connection_manager.update(cx, |manager, cx| {
                                    manager.set_active_connection(Some(index), cx);
                                });
                            });
                        }
                    }
                });
            }
            menu = menu.entry("Refresh Schema", None, {
                let handle = handle.clone();
                move |_window, cx| {
                    if let Some(panel) = handle.upgrade() {
                        panel.update(cx, |this, cx| {
                            this.connection_manager.update(cx, |manager, cx| {
                                manager.set_active_connection(Some(index), cx);
                                manager.refresh_schema(cx);
                            });
                        });
                    }
                }
            });
            menu = menu.separator();
            let name_for_copy = connection_name.clone();
            menu = menu.entry("Copy Connection Name", None, move |_window, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(name_for_copy.clone()));
            });
            menu = menu.separator();
            menu.entry("Remove", None, {
                let handle = handle.clone();
                move |_window, cx| {
                    if let Some(panel) = handle.upgrade() {
                        panel.update(cx, |this, cx| {
                            this.connection_manager.update(cx, |manager, cx| {
                                manager.remove_connection(index, cx);
                            });
                            this.query_result = None;
                            this.query_error = None;
                        });
                    }
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

    fn add_connection(&mut self, config: ConnectionConfig, cx: &mut Context<Self>) -> usize {
        self.is_loading = true;
        self.connection_manager.update(cx, |manager, cx| {
            manager.add_connection(config, cx)
        })
    }

    fn refresh_schema(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) {
        self.is_loading = true;
        cx.notify();
        self.connection_manager.update(cx, |manager, cx| {
            manager.refresh_schema(cx);
        });
    }

    fn toggle_schema_node(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(node) = self.schema_flattened_nodes.get(index) else {
            return;
        };

        if !node.expandable {
            self.selected_node = Some(index);
            cx.notify();
            return;
        }

        let node_id = node.id.clone();
        if self.expanded_nodes.contains(&node_id) {
            self.expanded_nodes.remove(&node_id);
        } else {
            self.expanded_nodes.insert(node_id);
        }
        self.selected_node = Some(index);

        self.rebuild_flattened_nodes(cx);
        self.serialize(cx);
        cx.notify();
    }

    fn select_next_node(&mut self, cx: &mut Context<Self>) {
        if self.schema_flattened_nodes.is_empty() {
            return;
        }
        let next = match self.selected_node {
            Some(current) => (current + 1).min(self.schema_flattened_nodes.len() - 1),
            None => 0,
        };
        self.selected_node = Some(next);
        self.schema_scroll_handle
            .scroll_to_item(next, gpui::ScrollStrategy::Top);
        cx.notify();
    }

    fn select_previous_node(&mut self, cx: &mut Context<Self>) {
        if self.schema_flattened_nodes.is_empty() {
            return;
        }
        let prev = match self.selected_node {
            Some(current) => current.saturating_sub(1),
            None => 0,
        };
        self.selected_node = Some(prev);
        self.schema_scroll_handle
            .scroll_to_item(prev, gpui::ScrollStrategy::Top);
        cx.notify();
    }

    fn confirm_selected_node(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) {
        let Some(index) = self.selected_node else {
            return;
        };
        let Some(node) = self.schema_flattened_nodes.get(index) else {
            return;
        };
        if node.expandable {
            if node.id.table_name().is_some() {
                self.handle_node_double_click(index, window, cx);
            } else {
                self.toggle_schema_node(index, cx);
            }
        } else {
            self.toggle_schema_node(index, cx);
        }
    }

    fn handle_node_double_click(
        &mut self,
        index: usize,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) {
        let Some(node) = self.schema_flattened_nodes.get(index) else {
            return;
        };

        if let Some(table_name) = node.id.table_name() {
            let db_type = self
                .active_database_type(cx)
                .unwrap_or(DatabaseType::Sqlite);
            let quoted = quote_identifier(table_name, &db_type);
            let is_sqlite = db_type == DatabaseType::Sqlite;
            let is_virtual = self.is_virtual_table(table_name, cx);

            let query = if !is_sqlite || is_virtual {
                format!("SELECT * FROM {}", quoted)
            } else {
                format!("SELECT rowid, * FROM {}", quoted)
            };
            self.query_editor.update(cx, |editor, cx| {
                editor.set_text(query, window, cx);
            });
            self.execute_query(window, cx);
        }
    }

    pub fn connection_manager(&self, _cx: &App) -> Entity<ConnectionManager> {
        self.connection_manager.clone()
    }

    fn active_database_type(&self, cx: &App) -> Option<DatabaseType> {
        let manager = self.connection_manager.read(cx);
        manager.active_entry().map(|e| e.config.database_type.clone())
    }

    fn is_virtual_table(&self, table_name: &str, cx: &App) -> bool {
        let manager = self.connection_manager.read(cx);
        let Some(schema) = manager.active_schema() else {
            return false;
        };
        schema
            .tables
            .iter()
            .any(|t| t.name == table_name && t.is_virtual)
    }

    fn rebuild_flattened_nodes(&mut self, cx: &App) {
        let manager = self.connection_manager.read(cx);
        if let Some(schema) = manager.active_schema() {
            self.schema_flattened_nodes =
                flatten_schema(schema, &self.expanded_nodes, &self.schema_filter);
        } else {
            self.schema_flattened_nodes.clear();
        }
    }

    fn toggle_sort_column(&mut self, col_index: usize, cx: &mut Context<Self>) {
        if let Some(pos) = self.sort_columns.iter().position(|(idx, _)| *idx == col_index) {
            match self.sort_columns[pos].1 {
                SortDirection::Ascending => {
                    self.sort_columns[pos].1 = SortDirection::Descending;
                }
                SortDirection::Descending => {
                    self.sort_columns.remove(pos);
                }
            }
        } else {
            self.sort_columns = vec![(col_index, SortDirection::Ascending)];
        }
        self.current_page = 0;
        self.selected_cell = None;
        self.fetch_current_page(cx);
    }

    fn apply_sort_to_query(&self, sql: &str) -> String {
        let trimmed = sql.trim().trim_end_matches(';').trim();
        if !self.sort_columns.is_empty() && trimmed.to_uppercase().starts_with("SELECT") {
            let order_parts: Vec<String> = self
                .sort_columns
                .iter()
                .map(|(col_index, direction)| {
                    let dir = match direction {
                        SortDirection::Ascending => "ASC",
                        SortDirection::Descending => "DESC",
                    };
                    format!("{} {}", col_index + 1, dir)
                })
                .collect();
            return format!(
                "SELECT * FROM ({}) AS _sorted ORDER BY {}",
                trimmed,
                order_parts.join(", ")
            );
        }
        trimmed.to_string()
    }

    fn select_cell(&mut self, row: usize, col: usize, cx: &mut Context<Self>) {
        self.selected_cell = Some((row, col));
        cx.notify();
    }

    fn grid_move_down(&mut self, cx: &mut Context<Self>) {
        let Some((row, col)) = self.selected_cell else {
            return;
        };
        let row_count = self
            .query_result
            .as_ref()
            .map(|r| r.rows.len())
            .unwrap_or(0);
        if row + 1 < row_count {
            self.selected_cell = Some((row + 1, col));
            self.results_scroll_handle
                .scroll_to_item(row + 1, gpui::ScrollStrategy::Top);
            cx.notify();
        }
    }

    fn grid_move_up(&mut self, cx: &mut Context<Self>) {
        let Some((row, col)) = self.selected_cell else {
            return;
        };
        if row > 0 {
            self.selected_cell = Some((row - 1, col));
            self.results_scroll_handle
                .scroll_to_item(row - 1, gpui::ScrollStrategy::Top);
            cx.notify();
        }
    }

    fn grid_move_right(&mut self, cx: &mut Context<Self>) {
        let Some((row, col)) = self.selected_cell else {
            return;
        };
        let col_count = self
            .query_result
            .as_ref()
            .map(|r| r.columns.len())
            .unwrap_or(0);
        if col + 1 < col_count {
            self.selected_cell = Some((row, col + 1));
            cx.notify();
        }
    }

    fn grid_move_left(&mut self, cx: &mut Context<Self>) {
        let Some((row, col)) = self.selected_cell else {
            return;
        };
        if col > 0 {
            self.selected_cell = Some((row, col - 1));
            cx.notify();
        }
    }

    fn expand_selected_cell(&mut self, cx: &mut Context<Self>) {
        let Some((row, col)) = self.selected_cell else {
            return;
        };
        let Some(result) = &self.query_result else {
            return;
        };
        let column_name = result
            .columns
            .get(col)
            .cloned()
            .unwrap_or_default();
        if let Some(cell) = result.rows.get(row).and_then(|r| r.get(col)) {
            self.expanded_cell = Some((column_name, cell.to_string()));
            cx.notify();
        }
    }

    fn copy_selected_cell(&mut self, cx: &mut Context<Self>) {
        let Some((row, col)) = self.selected_cell else {
            return;
        };
        let Some(result) = &self.query_result else {
            return;
        };
        if let Some(cell) = result.rows.get(row).and_then(|r| r.get(col)) {
            cx.write_to_clipboard(ClipboardItem::new_string(cell.to_string()));
        }
    }

    fn copy_selected_row(&mut self, cx: &mut Context<Self>) {
        let Some((row, _)) = self.selected_cell else {
            return;
        };
        let Some(result) = &self.query_result else {
            return;
        };
        if let Some(row_data) = result.rows.get(row) {
            let text: Vec<String> = row_data.iter().map(|cell| cell.to_string()).collect();
            cx.write_to_clipboard(ClipboardItem::new_string(text.join("\t")));
        }
    }

    fn copy_all_results_as_csv(&mut self, cx: &mut Context<Self>) {
        let Some(result) = &self.query_result else {
            return;
        };
        cx.write_to_clipboard(ClipboardItem::new_string(generate_csv(result)));
    }

    fn copy_all_results_as_json(&mut self, cx: &mut Context<Self>) {
        let Some(result) = &self.query_result else {
            return;
        };
        cx.write_to_clipboard(ClipboardItem::new_string(generate_json(result)));
    }

    fn deploy_results_context_menu(
        &mut self,
        row: usize,
        col: usize,
        position: Point<Pixels>,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) {
        self.selected_cell = Some((row, col));

        let context_menu = ContextMenu::build(window, cx, |menu, _window, _cx| {
            menu.action("Copy Cell", Box::new(CopyCellValue))
                .action("Copy Row", Box::new(CopyRowValues))
                .separator()
                .action("Copy All as CSV", Box::new(CopyAllResultsAsCsv))
                .action("Copy All as JSON", Box::new(CopyAllResultsAsJson))
        });

        window.focus(&context_menu.focus_handle(cx), cx);
        let subscription = cx.subscribe(&context_menu, |this, _, _: &DismissEvent, cx| {
            this.context_menu.take();
            cx.notify();
        });
        self.context_menu = Some((context_menu, position, subscription));
        cx.notify();
    }

    fn show_schema_context_menu(
        &mut self,
        index: usize,
        position: Point<Pixels>,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) {
        let Some(node) = self.schema_flattened_nodes.get(index) else {
            return;
        };

        self.selected_node = Some(index);
        let node_id = node.id.clone();
        let node_label = node.label.to_string();
        let handle = cx.entity().downgrade();

        let db_type = self
            .active_database_type(cx)
            .unwrap_or(DatabaseType::Sqlite);
        let is_sqlite = db_type == DatabaseType::Sqlite;

        let context_menu = ContextMenu::build(window, cx, move |menu, _window, _cx| {
            match &node_id {
                SchemaNodeId::Table(table_name) => {
                    let quoted = quote_identifier(table_name, &db_type);
                    let select_query = format!("SELECT * FROM {}", quoted);
                    let create_query = if is_sqlite {
                        format!(
                            "SELECT sql FROM sqlite_master WHERE type='table' AND name='{}'",
                            table_name.replace('\'', "''")
                        )
                    } else {
                        format!(
                            "SELECT column_name, data_type, is_nullable FROM information_schema.columns WHERE table_name = '{}'",
                            table_name.replace('\'', "''")
                        )
                    };
                    let table_name_copy = table_name.clone();

                    menu.entry("SELECT * FROM ...", None, {
                        let handle = handle.clone();
                        move |window, cx| {
                            if let Some(panel) = handle.upgrade() {
                                panel.update(cx, |this, cx| {
                                    this.query_editor.update(cx, |editor, cx| {
                                        editor.set_text(select_query.clone(), window, cx);
                                    });
                                    this.execute_query(window, cx);
                                });
                            }
                        }
                    })
                    .entry("Show CREATE TABLE", None, {
                        let handle = handle.clone();
                        move |window, cx| {
                            if let Some(panel) = handle.upgrade() {
                                panel.update(cx, |this, cx| {
                                    this.query_editor.update(cx, |editor, cx| {
                                        editor.set_text(create_query.clone(), window, cx);
                                    });
                                    this.execute_query(window, cx);
                                });
                            }
                        }
                    })
                    .separator()
                    .entry("Copy Name", None, move |_window, cx| {
                        cx.write_to_clipboard(ClipboardItem::new_string(
                            table_name_copy.clone(),
                        ));
                    })
                }
                SchemaNodeId::Column(_, column_name) => {
                    let column_name_copy = column_name.clone();
                    let column_name_insert = column_name.clone();

                    menu.entry("Copy Name", None, move |_window, cx| {
                        cx.write_to_clipboard(ClipboardItem::new_string(
                            column_name_copy.clone(),
                        ));
                    })
                    .entry("Add to Query", None, {
                        let handle = handle.clone();
                        move |window, cx| {
                            if let Some(panel) = handle.upgrade() {
                                panel.update(cx, |this, cx| {
                                    this.query_editor.update(cx, |editor, cx| {
                                        editor.insert(&column_name_insert, window, cx);
                                    });
                                });
                            }
                        }
                    })
                }
                SchemaNodeId::Index(_, _) => {
                    let label = node_label.clone();
                    menu.entry("Copy Name", None, move |_window, cx| {
                        cx.write_to_clipboard(ClipboardItem::new_string(label.clone()));
                    })
                }
                _ => menu,
            }
        });

        window.focus(&context_menu.focus_handle(cx), cx);
        let subscription = cx.subscribe(&context_menu, |this, _, _: &DismissEvent, cx| {
            this.context_menu.take();
            cx.notify();
        });
        self.context_menu = Some((context_menu, position, subscription));
        cx.notify();
    }

    fn previous_history_entry(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) {
        if let Some(entry) = self.query_history.navigate_previous() {
            let entry = entry.to_string();
            self.query_editor.update(cx, |editor, cx| {
                editor.set_text(entry, window, cx);
            });
            cx.notify();
        }
    }

    fn next_history_entry(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) {
        match self.query_history.navigate_next() {
            NavigateResult::Entry(entry) => {
                self.query_editor.update(cx, |editor, cx| {
                    editor.set_text(entry, window, cx);
                });
                cx.notify();
            }
            NavigateResult::Cleared => {
                self.query_editor.update(cx, |editor, cx| {
                    editor.set_text("", window, cx);
                });
                cx.notify();
            }
            NavigateResult::AtEnd => {}
        }
    }

    fn dismiss_context_menu_or_clear_filter(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) {
        if self.context_menu.is_some() {
            self.context_menu.take();
            cx.notify();
            return;
        }
        if self.expanded_cell.is_some() {
            self.expanded_cell = None;
            cx.notify();
            return;
        }
        if self.selected_cell.is_some() {
            self.selected_cell = None;
            cx.notify();
            return;
        }
        if self.is_loading {
            self.cancel_query(cx);
            return;
        }
        if !self.schema_filter.is_empty() {
            self.schema_filter.clear();
            self.schema_filter_editor.update(cx, |editor, cx| {
                editor.set_text("", window, cx);
            });
            self.rebuild_flattened_nodes(cx);
            cx.notify();
        }
    }

    fn execute_query(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) {
        let manager = self.connection_manager.read(cx);
        if manager.active_connection().is_none() {
            self.query_error = Some("No active connection".to_string());
            cx.notify();
            return;
        }
        if manager.active_db_connection().is_none() {
            self.query_error = Some("Connection is not active".to_string());
            cx.notify();
            return;
        }

        let sql = self.query_editor.read(cx).text(cx);
        let sql = sql.trim().to_string();
        if sql.is_empty() {
            return;
        }

        self.query_history.push(&sql);

        self.is_loading = true;
        self.query_error = None;
        self.current_page = 0;
        self.sort_columns.clear();
        self.selected_cell = None;
        self.column_widths.clear();
        self.current_query = Some(sql.clone());
        cx.notify();

        let rows_per_page = self.rows_per_page;
        let query_task = self.connection_manager.read(cx).execute_query(
            sql,
            rows_per_page,
            0,
            cx,
        );

        self.query_task = cx.spawn(async move |this, cx| {
            let result = query_task.await;

            this.update(cx, |this, cx| {
                this.is_loading = false;
                match result {
                    Ok(result) => {
                        this.query_result = Some(result);
                        this.query_error = None;
                    }
                    Err(error) => {
                        this.query_error = Some(format!("{:#}", error));
                        this.query_result = None;
                    }
                }
                cx.notify();
            })
            .log_err();
        });
    }

    fn explain_query(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) {
        let sql = self.query_editor.read(cx).text(cx);
        let sql = sql.trim().to_string();
        if sql.is_empty() {
            return;
        }

        let is_sqlite = self
            .active_database_type(cx)
            .map(|dt| dt == DatabaseType::Sqlite)
            .unwrap_or(true);

        let explain_sql = if is_sqlite {
            format!("EXPLAIN QUERY PLAN {}", sql)
        } else {
            format!("EXPLAIN ANALYZE {}", sql)
        };

        self.query_editor.update(cx, |editor, cx| {
            editor.set_text(explain_sql.as_str(), window, cx);
        });
        self.execute_query(window, cx);
    }

    fn cancel_query(&mut self, cx: &mut Context<Self>) {
        if !self.is_loading {
            return;
        }

        self.connection_manager.read(cx).interrupt_active();

        self.query_task = Task::ready(());
        self.is_loading = false;
        cx.notify();
    }

    fn export_results_to_file(
        &mut self,
        format: ExportFormat,
        cx: &mut Context<Self>,
    ) {
        let Some(result) = &self.query_result else {
            return;
        };

        let table_name = "query_results";
        let content = match format {
            ExportFormat::Csv => generate_csv(result),
            ExportFormat::Tsv => generate_tsv(result),
            ExportFormat::Json => generate_json(result),
            ExportFormat::SqlInsert => generate_sql_insert(result, table_name),
            ExportFormat::SqlDdlDml => generate_sql_ddl_dml(result, table_name),
            ExportFormat::Markdown => generate_markdown(result),
            ExportFormat::Html => generate_html(result),
        };

        let extension = match format {
            ExportFormat::Csv => "csv",
            ExportFormat::Tsv => "tsv",
            ExportFormat::Json => "json",
            ExportFormat::SqlInsert | ExportFormat::SqlDdlDml => "sql",
            ExportFormat::Markdown => "md",
            ExportFormat::Html => "html",
        };

        let default_name = format!("query_results.{}", extension);
        let save_dialog = cx.prompt_for_new_path(&PathBuf::default(), Some(&default_name));

        cx.spawn(async move |_this, _cx| {
            let path = match save_dialog.await {
                Ok(Ok(Some(path))) => path,
                Ok(Ok(None)) => return,
                Ok(Err(error)) => {
                    log::error!("database_viewer: file dialog error: {:#}", error);
                    return;
                }
                Err(_) => return,
            };

            if let Err(error) = std::fs::write(&path, content.as_bytes()) {
                log::error!("database_viewer: failed to export results: {:#}", error);
            }
        }).detach();
    }

    fn show_export_dialog(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) {
        let Some(result) = &self.query_result else {
            return;
        };

        let result_clone = result.clone();
        let table_name = "query_results".to_string();

        if let Some(workspace) = self._workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                workspace.toggle_modal(window, cx, |window, cx| {
                    ExportDialog::new(result_clone, table_name, window, cx)
                });
            });
        }
    }

    fn save_current_query(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) {
        let sql = self.query_editor.read(cx).text(cx);
        let sql = sql.trim().to_string();
        if sql.is_empty() {
            return;
        }

        let name = match sql.char_indices().nth(40) {
            Some((byte_index, _)) => format!("{}...", &sql[..byte_index]),
            None => sql.clone(),
        };

        if self.saved_queries.iter().any(|q| q.sql == sql) {
            return;
        }

        self.saved_queries.push(SavedQuery {
            name,
            sql,
        });
        self.serialize(cx);
        cx.notify();
    }

    fn load_saved_query(
        &mut self,
        index: usize,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) {
        let Some(saved) = self.saved_queries.get(index) else {
            return;
        };
        let sql = saved.sql.clone();
        self.query_editor.update(cx, |editor, cx| {
            editor.set_text(sql, window, cx);
        });
        self.show_saved_queries = false;
        cx.notify();
    }

    fn remove_saved_query(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.saved_queries.len() {
            self.saved_queries.remove(index);
            self.serialize(cx);
            cx.notify();
        }
    }

    fn pin_result_to_tab(&mut self, cx: &mut Context<Self>) {
        let Some(result) = &self.query_result else {
            return;
        };
        let query = self.current_query.clone().unwrap_or_default();

        let label = match query.char_indices().nth(20) {
            Some((byte_index, _)) => format!("{}...", &query[..byte_index]),
            None => query.clone(),
        };

        self.result_tabs.push(ResultTab {
            label,
            query,
            result: result.clone(),
            page: self.current_page,
            sort_columns: self.sort_columns.clone(),
            column_widths: self.column_widths.clone(),
        });
        self.active_result_tab = Some(self.result_tabs.len() - 1);
        cx.notify();
    }

    fn select_result_tab(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.result_tabs.len() {
            let tab = &self.result_tabs[index];
            self.query_result = Some(tab.result.clone());
            self.current_query = Some(tab.query.clone());
            self.current_page = tab.page;
            self.sort_columns = tab.sort_columns.clone();
            self.column_widths = tab.column_widths.clone();
            self.active_result_tab = Some(index);
            self.selected_cell = None;
            cx.notify();
        }
    }

    fn close_result_tab(&mut self, index: usize, cx: &mut Context<Self>) {
        if index >= self.result_tabs.len() {
            return;
        }
        self.result_tabs.remove(index);

        if self.result_tabs.is_empty() {
            self.active_result_tab = None;
        } else if let Some(active) = self.active_result_tab {
            if active == index {
                let new_active = index.min(self.result_tabs.len().saturating_sub(1));
                self.active_result_tab = Some(new_active);
                self.select_result_tab(new_active, cx);
                return;
            } else if active > index {
                self.active_result_tab = Some(active - 1);
            }
        }
        cx.notify();
    }

    fn start_column_resize(
        &mut self,
        col_index: usize,
        start_x: Pixels,
        cx: &mut Context<Self>,
    ) {
        self.ensure_column_widths(cx);
        self.resizing_column = Some((col_index, start_x.as_f32()));
    }

    fn handle_resize_move(&mut self, position_x: f32, cx: &mut Context<Self>) {
        let Some((col_index, start_x)) = self.resizing_column else {
            return;
        };
        let settings = DatabasePanelSettings::get_global(cx);
        let current_width = self
            .column_widths
            .get(col_index)
            .copied()
            .unwrap_or(settings.default_column_width);
        let delta = position_x - start_x;
        let new_width = (current_width + delta).max(MIN_COLUMN_WIDTH);

        if let Some(width) = self.column_widths.get_mut(col_index) {
            *width = new_width;
        }
        self.resizing_column = Some((col_index, position_x));
        cx.notify();
    }

    fn stop_column_resize(&mut self) {
        self.resizing_column = None;
    }

    fn ensure_column_widths(&mut self, cx: &App) {
        let column_count = self
            .query_result
            .as_ref()
            .map(|r| r.columns.len())
            .unwrap_or(0);
        if self.column_widths.len() < column_count {
            let settings = DatabasePanelSettings::get_global(cx);
            self.column_widths
                .resize(column_count, settings.default_column_width);
        }
    }

    fn fetch_current_page(&mut self, cx: &mut Context<Self>) {
        if self.connection_manager.read(cx).active_db_connection().is_none() {
            return;
        }
        let Some(sql) = self.current_query.clone() else {
            return;
        };

        let sql = self.apply_sort_to_query(&sql);

        self.is_loading = true;
        cx.notify();

        let offset = self.current_page * self.rows_per_page;
        let rows_per_page = self.rows_per_page;
        let query_task = self
            .connection_manager
            .read(cx)
            .execute_query(sql, rows_per_page, offset, cx);

        self.query_task = cx.spawn(async move |this, cx| {
            let result = query_task.await;

            this.update(cx, |this, cx| {
                this.is_loading = false;
                match result {
                    Ok(result) => {
                        this.query_result = Some(result);
                        this.query_error = None;
                    }
                    Err(error) => {
                        this.query_error = Some(format!("{:#}", error));
                    }
                }
                cx.notify();
            })
            .log_err();
        });
    }

    fn total_row_count(&self) -> usize {
        self.query_result
            .as_ref()
            .and_then(|r| r.total_row_count)
            .map(|c| c as usize)
            .unwrap_or_else(|| {
                self.query_result
                    .as_ref()
                    .map(|r| r.rows.len())
                    .unwrap_or(0)
            })
    }

    fn total_pages(&self) -> usize {
        let total_rows = self.total_row_count();
        if total_rows == 0 {
            1
        } else {
            total_rows.div_ceil(self.rows_per_page)
        }
    }

    fn go_to_previous_page(&mut self, cx: &mut Context<Self>) {
        if self.current_page > 0 {
            self.current_page -= 1;
            self.results_scroll_handle.scroll_to_item(0, gpui::ScrollStrategy::Top);
            self.fetch_current_page(cx);
        }
    }

    fn go_to_next_page(&mut self, cx: &mut Context<Self>) {
        let total_pages = self.total_pages();
        if self.current_page + 1 < total_pages {
            self.current_page += 1;
            self.results_scroll_handle.scroll_to_item(0, gpui::ScrollStrategy::Top);
            self.fetch_current_page(cx);
        }
    }

    fn go_to_first_page(&mut self, cx: &mut Context<Self>) {
        if self.current_page != 0 {
            self.current_page = 0;
            self.results_scroll_handle.scroll_to_item(0, gpui::ScrollStrategy::Top);
            self.fetch_current_page(cx);
        }
    }

    fn go_to_last_page(&mut self, cx: &mut Context<Self>) {
        let last = self.total_pages().saturating_sub(1);
        if self.current_page != last {
            self.current_page = last;
            self.results_scroll_handle.scroll_to_item(0, gpui::ScrollStrategy::Top);
            self.fetch_current_page(cx);
        }
    }

    fn set_rows_per_page(&mut self, count: usize, cx: &mut Context<Self>) {
        self.rows_per_page = count;
        self.current_page = 0;
        self.results_scroll_handle.scroll_to_item(0, gpui::ScrollStrategy::Top);
        self.serialize(cx);
        self.fetch_current_page(cx);
    }

    fn select_connection(&mut self, index: usize, cx: &mut Context<Self>) {
        self.connection_manager.update(cx, |manager, cx| {
            manager.set_active_connection(Some(index), cx);
        });
    }

    fn handle_external_paths_drop(
        &mut self,
        external_paths: &ExternalPaths,
        cx: &mut Context<Self>,
    ) {
        let db_paths: Vec<PathBuf> = external_paths
            .paths()
            .iter()
            .filter(|path| {
                path.extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| DATABASE_EXTENSIONS.contains(&ext))
                    .unwrap_or(false)
            })
            .filter(|path| !self.connected_paths.contains(*path))
            .cloned()
            .collect();

        if db_paths.is_empty() {
            return;
        }

        for path in &db_paths {
            self.connected_paths.insert(path.clone());
        }
        self.connect_detected_databases(db_paths, cx);
    }

    fn connect_detected_databases(&mut self, paths: Vec<PathBuf>, cx: &mut Context<Self>) {
        for path in paths {
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| path.to_string_lossy().to_string());

            let config = ConnectionConfig::sqlite(name, path);
            self.add_connection(config, cx);
        }

        let manager = self.connection_manager.read(cx);
        if !manager.connections().is_empty() && manager.active_connection().is_none() {
            self.connection_manager.update(cx, |manager, cx| {
                manager.set_active_connection(Some(0), cx);
            });
        }
    }

    fn render_pagination_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let total_pages = self.total_pages();
        let current_page = self.current_page;
        let total_rows = self.total_row_count();
        let start_row = current_page * self.rows_per_page + 1;
        let end_row = ((current_page + 1) * self.rows_per_page).min(total_rows);
        let rows_per_page = self.rows_per_page;

        let range_text = if total_rows == 0 {
            "0 rows".to_string()
        } else {
            format!("{}-{} of {}", start_row, end_row, total_rows)
        };

        let is_first_page = current_page == 0;
        let is_last_page = current_page + 1 >= total_pages;

        h_flex()
            .flex_none()
            .h(px(26.0))
            .w_full()
            .px_2()
            .items_center()
            .justify_between()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        Label::new("Rows/page:")
                            .size(LabelSize::XSmall)
                            .color(ui::Color::Muted),
                    )
                    .children(ROWS_PER_PAGE_OPTIONS.iter().map(|&count| {
                        let is_active = rows_per_page == count;
                        Button::new(
                            SharedString::from(format!("page-size-{}", count)),
                            format!("{}", count),
                        )
                        .style(if is_active {
                            ButtonStyle::Filled
                        } else {
                            ButtonStyle::Subtle
                        })
                        .label_size(LabelSize::XSmall)
                        .on_click(cx.listener(move |this, _, _window, cx| {
                            this.set_rows_per_page(count, cx);
                        }))
                    })),
            )
            .child(
                h_flex()
                    .gap_1()
                    .items_center()
                    .child(
                        Button::new("page-first", "")
                            .icon(IconName::ArrowLeft)
                            .icon_size(IconSize::XSmall)
                            .style(ButtonStyle::Subtle)
                            .disabled(is_first_page)
                            .tooltip(Tooltip::text("First page"))
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.go_to_first_page(cx);
                            })),
                    )
                    .child(
                        Button::new("page-prev", "")
                            .icon(IconName::ChevronLeft)
                            .icon_size(IconSize::XSmall)
                            .style(ButtonStyle::Subtle)
                            .disabled(is_first_page)
                            .tooltip(Tooltip::text("Previous page"))
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.go_to_previous_page(cx);
                            })),
                    )
                    .child(
                        Label::new(range_text)
                            .size(LabelSize::XSmall)
                            .color(ui::Color::Muted),
                    )
                    .child(
                        Button::new("page-next", "")
                            .icon(IconName::ChevronRight)
                            .icon_size(IconSize::XSmall)
                            .style(ButtonStyle::Subtle)
                            .disabled(is_last_page)
                            .tooltip(Tooltip::text("Next page"))
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.go_to_next_page(cx);
                            })),
                    )
                    .child(
                        Button::new("page-last", "")
                            .icon(IconName::ArrowRight)
                            .icon_size(IconSize::XSmall)
                            .style(ButtonStyle::Subtle)
                            .disabled(is_last_page)
                            .tooltip(Tooltip::text("Last page"))
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.go_to_last_page(cx);
                            })),
                    ),
            )
    }

    fn render_result_tabs(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let active_tab = self.active_result_tab;
        let mut tabs = h_flex()
            .id("result-tabs")
            .w_full()
            .px_1()
            .py(px(2.0))
            .gap(px(2.0))
            .overflow_x_scroll()
            .border_t_1()
            .border_color(cx.theme().colors().border);

        for (index, tab) in self.result_tabs.iter().enumerate() {
            let is_active = active_tab == Some(index);
            let label = tab.label.clone();

            tabs = tabs.child(
                div()
                    .id(SharedString::from(format!("result-tab-{}", index)))
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    .px_2()
                    .py(px(2.0))
                    .rounded_md()
                    .cursor_pointer()
                    .text_size(px(11.0))
                    .when(is_active, |this| {
                        this.bg(cx.theme().colors().ghost_element_selected)
                    })
                    .when(!is_active, |this| {
                        this.hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
                    })
                    .on_click(cx.listener(move |this, _, _window, cx| {
                        this.select_result_tab(index, cx);
                    }))
                    .child(
                        Icon::new(IconName::Pin)
                            .size(IconSize::XSmall)
                            .color(ui::Color::Muted),
                    )
                    .child(
                        Label::new(label)
                            .size(LabelSize::XSmall)
                            .color(if is_active {
                                ui::Color::Default
                            } else {
                                ui::Color::Muted
                            }),
                    )
                    .child(
                        div()
                            .id(SharedString::from(format!("tab-close-{}", index)))
                            .cursor_pointer()
                            .rounded_sm()
                            .hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                this.close_result_tab(index, cx);
                            }))
                            .child(
                                Icon::new(IconName::Close)
                                    .size(IconSize::XSmall)
                                    .color(ui::Color::Muted),
                            ),
                    ),
            );
        }

        tabs
    }

    fn render_saved_queries_panel(
        &self,
        _window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let mut panel = v_flex()
            .w_full()
            .max_h(px(200.0))
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().surface_background);

        panel = panel.child(
            h_flex()
                .h(px(26.0))
                .w_full()
                .px_2()
                .items_center()
                .justify_between()
                .child(
                    Label::new("Saved Queries")
                        .size(LabelSize::Small)
                        .weight(gpui::FontWeight::BOLD),
                )
                .child(
                    Button::new("close-saved-queries", "")
                        .icon(IconName::Close)
                        .icon_size(IconSize::XSmall)
                        .style(ButtonStyle::Subtle)
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.show_saved_queries = false;
                            cx.notify();
                        })),
                ),
        );

        if self.saved_queries.is_empty() {
            panel = panel.child(
                div()
                    .p_2()
                    .child(
                        Label::new("No saved queries. Use the bookmark button to save a query.")
                            .size(LabelSize::XSmall)
                            .color(ui::Color::Muted),
                    ),
            );
        } else {
            let mut list = v_flex()
                .id("saved-queries-list")
                .w_full()
                .overflow_y_scroll()
                .max_h(px(170.0));

            for (index, saved) in self.saved_queries.iter().enumerate() {
                let name = saved.name.clone();

                list = list.child(
                    div()
                        .id(SharedString::from(format!("saved-query-{}", index)))
                        .flex()
                        .items_center()
                        .justify_between()
                        .w_full()
                        .px_2()
                        .py(px(2.0))
                        .cursor_pointer()
                        .hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.load_saved_query(index, window, cx);
                        }))
                        .child(
                            h_flex()
                                .gap_1()
                                .overflow_hidden()
                                .flex_grow()
                                .child(
                                    Icon::new(IconName::Star)
                                        .size(IconSize::XSmall)
                                        .color(ui::Color::Muted),
                                )
                                .child(
                                    Label::new(name)
                                        .size(LabelSize::XSmall)
                                        .color(ui::Color::Default)
                                        .single_line(),
                                ),
                        )
                        .child(
                            div()
                                .id(SharedString::from(format!("remove-saved-{}", index)))
                                .cursor_pointer()
                                .rounded_sm()
                                .hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
                                .on_click(cx.listener(move |this, _, _window, cx| {
                                    this.remove_saved_query(index, cx);
                                }))
                                .child(
                                    Icon::new(IconName::Close)
                                        .size(IconSize::XSmall)
                                        .color(ui::Color::Muted),
                                ),
                        ),
                );
            }

            panel = panel.child(list);
        }

        panel
    }

    fn render_empty_state(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .flex_grow()
            .items_center()
            .justify_center()
            .gap_2()
            .p_4()
            .child(
                Icon::new(IconName::DatabaseZap)
                    .size(IconSize::Medium)
                    .color(ui::Color::Muted),
            )
            .child(
                Label::new("No database connected")
                    .size(LabelSize::Small)
                    .color(ui::Color::Muted),
            )
            .child(
                Label::new("Use the + button above to add a connection.")
                    .size(LabelSize::XSmall)
                    .color(ui::Color::Muted),
            )
    }

    fn render_connections_section(
        &self,
        _window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let connection_count = self.connection_manager.read(cx).connections().len();
        let is_expanded = self.connections_section_expanded;

        let mut section = v_flex()
            .w_full()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                h_flex()
                    .id("connections-header")
                    .h(px(28.0))
                    .w_full()
                    .px_2()
                    .items_center()
                    .justify_between()
                    .cursor_pointer()
                    .hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.connections_section_expanded = !this.connections_section_expanded;
                        this.serialize(cx);
                        cx.notify();
                    }))
                    .child(
                        h_flex()
                            .gap_1()
                            .items_center()
                            .child(
                                Icon::new(if is_expanded {
                                    IconName::ChevronDown
                                } else {
                                    IconName::ChevronRight
                                })
                                .size(IconSize::XSmall)
                                .color(ui::Color::Muted),
                            )
                            .child(
                                Label::new(format!("Connections ({})", connection_count))
                                    .size(LabelSize::Small)
                                    .weight(gpui::FontWeight::BOLD),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .child(self.render_add_connection_button(cx))
                            .child(
                                Button::new("refresh-schema", "")
                                    .icon(IconName::ArrowCircle)
                                    .icon_size(IconSize::Small)
                                    .style(ButtonStyle::Subtle)
                                    .tooltip(Tooltip::text("Refresh Schema"))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.refresh_schema(window, cx);
                                    })),
                            ),
                    ),
            );

        if is_expanded {
            let mut list = v_flex().w_full();
            for index in 0..self.connection_manager.read(cx).connections().len() {
                list = list.child(self.render_connection_list_item(index, cx));
            }
            section = section.child(list);

            if self.connection_form.active_flow.is_some() {
                section = section.child(self.render_inline_connection_form(cx));
            }
        }

        section
    }

    fn render_add_connection_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let handle = cx.entity().downgrade();

        PopoverMenu::new("add-connection-menu")
            .trigger(
                Button::new("add-connection-btn", "")
                    .icon(IconName::Plus)
                    .icon_size(IconSize::Small)
                    .style(ButtonStyle::Subtle)
                    .tooltip(Tooltip::text("Add Connection")),
            )
            .menu({
                let handle = handle.clone();
                move |window, cx| {
                    let handle = handle.clone();
                    let menu = ContextMenu::build(window, cx, move |menu, _window, _cx| {
                        menu.entry("Open SQLite File...", None, {
                            let handle = handle.clone();
                            move |_window, cx| {
                                if let Some(panel) = handle.upgrade() {
                                    panel.update(cx, |this, cx| {
                                        this.open_sqlite_file_picker(cx);
                                    });
                                }
                            }
                        })
                        .entry("Connect to PostgreSQL...", None, {
                            let handle = handle.clone();
                            move |_window, cx| {
                                if let Some(panel) = handle.upgrade() {
                                    panel.update(cx, |this, cx| {
                                        this.show_postgres_connection_form(cx);
                                    });
                                }
                            }
                        })
                        .entry("Connect to MySQL...", None, {
                            let handle = handle.clone();
                            move |_window, cx| {
                                if let Some(panel) = handle.upgrade() {
                                    panel.update(cx, |this, cx| {
                                        this.show_mysql_connection_form(cx);
                                    });
                                }
                            }
                        })
                    });
                    Some(menu)
                }
            })
    }

    fn render_connection_list_item(
        &self,
        index: usize,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let manager = self.connection_manager.read(cx);
        let Some(entry) = manager.connections().get(index) else {
            return div().into_any_element();
        };

        let is_active = manager.active_connection() == Some(index);
        let has_error = entry.error.is_some();
        let is_connected = entry.connection.is_some();
        let is_read_only = entry.config.read_only;
        let display_name = entry.config.display_name();
        let db_type_label = entry.config.database_type.to_string();
        let color_index = entry.config.color_index;

        let status_color = if has_error {
            gpui::red()
        } else if is_connected {
            gpui::green()
        } else {
            gpui::yellow()
        };

        let connection_color = CONNECTION_COLORS
            .get(color_index % CONNECTION_COLORS.len())
            .copied()
            .unwrap_or(0x3B82F6);
        let connection_rgba = gpui::Rgba {
            r: ((connection_color >> 16) & 0xFF) as f32 / 255.0,
            g: ((connection_color >> 8) & 0xFF) as f32 / 255.0,
            b: (connection_color & 0xFF) as f32 / 255.0,
            a: 1.0,
        };

        div()
            .id(SharedString::from(format!("conn-item-{}", index)))
            .flex()
            .items_center()
            .w_full()
            .h(px(26.0))
            .pr_2()
            .gap_1()
            .cursor_pointer()
            .when(is_active, |this| {
                this.bg(cx.theme().colors().ghost_element_selected)
            })
            .hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
            .on_click(cx.listener(move |this, _, _window, cx| {
                this.select_connection(index, cx);
            }))
            .on_mouse_down(MouseButton::Right, cx.listener(move |this, event: &gpui::MouseDownEvent, window, cx| {
                this.show_connection_context_menu(index, event.position, window, cx);
            }))
            .child(
                div()
                    .w(px(4.0))
                    .h_full()
                    .flex_shrink_0()
                    .bg(connection_rgba),
            )
            .child(
                div()
                    .w(px(6.0))
                    .h(px(6.0))
                    .flex_shrink_0()
                    .rounded_full()
                    .bg(status_color),
            )
            .child(
                Label::new(display_name)
                    .size(LabelSize::XSmall)
                    .color(if has_error {
                        ui::Color::Error
                    } else if is_active {
                        ui::Color::Default
                    } else {
                        ui::Color::Muted
                    })
                    .single_line(),
            )
            .child(
                Label::new(format!("({})", db_type_label))
                    .size(LabelSize::XSmall)
                    .color(ui::Color::Muted),
            )
            .when(is_read_only, |this| {
                this.child(
                    Icon::new(IconName::FileLock)
                        .size(IconSize::XSmall)
                        .color(ui::Color::Muted),
                )
            })
            .into_any_element()
    }

    fn render_inline_connection_form(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let form = &self.connection_form;

        let flow_label = match &form.active_flow {
            Some(AddConnectionFlow::PostgreSql) => "Connect to PostgreSQL",
            Some(AddConnectionFlow::MySql) => "Connect to MySQL",
            None => "",
        };

        let field = match &form.active_flow {
            Some(AddConnectionFlow::PostgreSql) => form.pg_connection_string_field.clone(),
            Some(AddConnectionFlow::MySql) => form.mysql_connection_string_field.clone(),
            None => form.pg_connection_string_field.clone(),
        };

        let mut form_element = v_flex()
            .w_full()
            .p_2()
            .gap_2()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().surface_background)
            .child(
                Label::new(flow_label)
                    .size(LabelSize::XSmall)
                    .weight(gpui::FontWeight::BOLD)
                    .color(ui::Color::Muted),
            )
            .child(field);

        if let Some(error) = &form.error {
            form_element = form_element.child(
                Banner::new()
                    .severity(Severity::Error)
                    .child(Label::new(SharedString::from(error.clone()))),
            );
        }

        form_element.child(
            h_flex()
                .justify_end()
                .gap_1()
                .child(
                    Button::new("cancel-conn-btn", "Cancel")
                        .style(ButtonStyle::Subtle)
                        .label_size(LabelSize::Small)
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.connection_form.dismiss();
                            cx.notify();
                        })),
                )
                .child(
                    Button::new("connect-conn-btn", "Connect")
                        .style(ButtonStyle::Filled)
                        .label_size(LabelSize::Small)
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.submit_active_connection(window, cx);
                        })),
                ),
        )
    }

    fn render_schema_section(
        &self,
        _window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let entity = cx.entity().downgrade();
        let entity_for_secondary = cx.entity().downgrade();

        let active_error = self
            .connection_manager
            .read(cx)
            .active_entry()
            .and_then(|e| e.error.as_ref())
            .cloned();

        if self.schema_flattened_nodes.is_empty() && self.schema_filter.is_empty() {
            let has_error = active_error.is_some();
            let message = active_error
                .map(SharedString::from)
                .unwrap_or_else(|| SharedString::from("No schema loaded"));
            let color = if has_error {
                ui::Color::Error
            } else {
                ui::Color::Muted
            };

            return v_flex()
                .flex_grow()
                .items_center()
                .justify_center()
                .p_2()
                .child(Label::new(message).size(LabelSize::Small).color(color))
                .into_any_element();
        }

        let selected = self.selected_node;
        let scroll_handle = self.schema_scroll_handle.clone();

        v_flex()
            .flex_grow()
            .w_full()
            .overflow_hidden()
            .child(
                div()
                    .w_full()
                    .px_1()
                    .py(px(2.0))
                    .child(
                        div()
                            .w_full()
                            .h(px(24.0))
                            .border_1()
                            .border_color(cx.theme().colors().border)
                            .rounded_md()
                            .px_1()
                            .child(self.schema_filter_editor.clone()),
                    ),
            )
            .child(
                render_schema_tree(
                    &self.schema_flattened_nodes,
                    selected,
                    &scroll_handle,
                    {
                        move |index, click_count, window, cx| {
                            if click_count >= 2 {
                                entity
                                    .update(cx, |panel, cx| {
                                        panel.handle_node_double_click(index, window, cx);
                                    })
                                    .log_err();
                            } else {
                                entity
                                    .update(cx, |panel, cx| {
                                        panel.toggle_schema_node(index, cx);
                                    })
                                    .log_err();
                            }
                        }
                    },
                    {
                        move |index, position, window, cx| {
                            entity_for_secondary
                                .update(cx, |panel, cx| {
                                    panel.selected_node = Some(index);
                                    panel.show_schema_context_menu(index, position, window, cx);
                                })
                                .log_err();
                        }
                    },
                ),
            )
            .into_any_element()
    }

    fn render_query_section(
        &self,
        _window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let can_go_back = self.query_history.can_go_back();
        let can_go_forward = self.query_history.can_go_forward();

        v_flex()
            .flex_none()
            .w_full()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .child(
                h_flex()
                    .h(px(28.0))
                    .w_full()
                    .px_2()
                    .items_center()
                    .justify_between()
                    .child(
                        Label::new("SQL Query")
                            .size(LabelSize::Small)
                            .weight(gpui::FontWeight::BOLD),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .child(
                                Button::new("history-prev", "")
                                    .icon(IconName::ArrowUp)
                                    .icon_size(IconSize::XSmall)
                                    .style(ButtonStyle::Subtle)
                                    .disabled(!can_go_back)
                                    .tooltip(Tooltip::text("Previous Query (Alt+Up)"))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.previous_history_entry(window, cx);
                                    })),
                            )
                            .child(
                                Button::new("history-next", "")
                                    .icon(IconName::ArrowDown)
                                    .icon_size(IconSize::XSmall)
                                    .style(ButtonStyle::Subtle)
                                    .disabled(!can_go_forward)
                                    .tooltip(Tooltip::text("Next Query (Alt+Down)"))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.next_history_entry(window, cx);
                                    })),
                            )
                            .child(
                                Button::new("save-query-btn", "")
                                    .icon(IconName::Star)
                                    .icon_size(IconSize::XSmall)
                                    .style(ButtonStyle::Subtle)
                                    .tooltip(Tooltip::text("Save Query"))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.save_current_query(window, cx);
                                    })),
                            )
                            .child(
                                Button::new("show-saved-btn", "")
                                    .icon(IconName::StarFilled)
                                    .icon_size(IconSize::XSmall)
                                    .style(if self.show_saved_queries {
                                        ButtonStyle::Filled
                                    } else {
                                        ButtonStyle::Subtle
                                    })
                                    .tooltip(Tooltip::text("Saved Queries"))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.show_saved_queries = !this.show_saved_queries;
                                        cx.notify();
                                    })),
                            )
                            .child(
                                Button::new("run-query", "Run")
                                    .icon(IconName::PlayFilled)
                                    .icon_size(IconSize::Small)
                                    .style(ButtonStyle::Filled)
                                    .tooltip(move |_window, cx| {
                                        Tooltip::for_action(
                                            "Execute Query",
                                            &ExecuteQuery,
                                            cx,
                                        )
                                    })
                                    .when(
                                        self.connection_manager.read(cx).active_connection().is_none(),
                                        |this| this.disabled(true),
                                    )
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.execute_query(window, cx);
                                    })),
                            ),
                    ),
            )
            .child(
                div()
                    .id("query-editor-container")
                    .w_full()
                    .min_h(px(60.0))
                    .max_h(px(200.0))
                    .overflow_y_scroll()
                    .px_1()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(self.query_editor.clone()),
            )
    }

    fn render_results_section(
        &self,
        _window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let has_results = self.query_result.is_some();

        let mut container = v_flex()
            .flex_grow()
            .w_full()
            .overflow_hidden()
            .child(
                h_flex()
                    .h(px(28.0))
                    .w_full()
                    .px_2()
                    .items_center()
                    .justify_between()
                    .border_t_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        Label::new("Results")
                            .size(LabelSize::Small)
                            .weight(gpui::FontWeight::BOLD),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .child(
                                Button::new("copy-csv", "")
                                    .icon(IconName::Copy)
                                    .icon_size(IconSize::XSmall)
                                    .style(ButtonStyle::Subtle)
                                    .disabled(!has_results)
                                    .tooltip(Tooltip::text("Copy All as CSV"))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.copy_all_results_as_csv(cx);
                                    })),
                            )
                            .child(
                                Button::new("copy-json", "")
                                    .icon(IconName::FileCode)
                                    .icon_size(IconSize::XSmall)
                                    .style(ButtonStyle::Subtle)
                                    .disabled(!has_results)
                                    .tooltip(Tooltip::text("Copy All as JSON"))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.copy_all_results_as_json(cx);
                                    })),
                            )
                            .child(
                                Button::new("export-dialog", "")
                                    .icon(IconName::Download)
                                    .icon_size(IconSize::XSmall)
                                    .style(ButtonStyle::Subtle)
                                    .disabled(!has_results)
                                    .tooltip(Tooltip::text("Export Results..."))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.show_export_dialog(window, cx);
                                    })),
                            )
                            .child(
                                Button::new("pin-tab", "")
                                    .icon(IconName::Pin)
                                    .icon_size(IconSize::XSmall)
                                    .style(ButtonStyle::Subtle)
                                    .disabled(!has_results)
                                    .tooltip(Tooltip::text("Pin result to tab"))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.pin_result_to_tab(cx);
                                    })),
                            ),
                    ),
            );

        if !self.result_tabs.is_empty() {
            container = container.child(self.render_result_tabs(cx));
        }

        if self.is_loading {
            container = container.child(
                div()
                    .flex_grow()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        Label::new("Executing query...")
                            .size(LabelSize::Small)
                            .color(ui::Color::Muted),
                    ),
            );
            return container;
        }

        if let Some(error) = &self.query_error {
            container = container.child(
                div()
                    .p_2()
                    .child(
                        Label::new(SharedString::from(error.clone()))
                            .size(LabelSize::Small)
                            .color(ui::Color::Error),
                    ),
            );
        } else if let Some(result) = &self.query_result {
            let page_offset = self.current_page * self.rows_per_page;

            let on_header_click: Arc<dyn Fn(usize, &mut gpui::Window, &mut App) + Send + Sync> = {
                let handle = cx.entity().downgrade();
                Arc::new(move |col_index, _window, cx| {
                    if let Some(panel) = handle.upgrade() {
                        panel.update(cx, |this, cx| {
                            this.toggle_sort_column(col_index, cx);
                        });
                    }
                })
            };

            let on_cell_click: Arc<dyn Fn(usize, usize, usize, &mut gpui::Window, &mut App) + Send + Sync> = {
                let handle = cx.entity().downgrade();
                Arc::new(move |row, col, click_count, _window, cx| {
                    if let Some(panel) = handle.upgrade() {
                        panel.update(cx, |this, cx| {
                            this.select_cell(row, col, cx);
                            if click_count >= 2 {
                                this.expand_selected_cell(cx);
                            }
                        });
                    }
                })
            };

            let on_cell_secondary_click: Arc<dyn Fn(usize, usize, Point<Pixels>, &mut gpui::Window, &mut App) + Send + Sync> = {
                let handle = cx.entity().downgrade();
                Arc::new(move |row, col, position, window, cx| {
                    if let Some(panel) = handle.upgrade() {
                        panel.update(cx, |this, cx| {
                            this.deploy_results_context_menu(row, col, position, window, cx);
                        });
                    }
                })
            };

            let on_resize_start: Arc<dyn Fn(usize, Pixels, &mut gpui::Window, &mut App) + Send + Sync> = {
                let handle = cx.entity().downgrade();
                Arc::new(move |col_index, start_x, _window, cx| {
                    if let Some(panel) = handle.upgrade() {
                        panel.update(cx, |this, cx| {
                            this.start_column_resize(col_index, start_x, cx);
                        });
                    }
                })
            };

            let settings = DatabasePanelSettings::get_global(cx);
            let table_config = TableConfig {
                default_column_width: settings.default_column_width,
                column_widths: self.column_widths.clone(),
                max_cell_chars: settings.max_cell_display_chars,
                row_height: settings.row_height,
                header_height: settings.header_height,
                ..Default::default()
            };

            container = container
                .child(render_results_table(
                    result,
                    page_offset,
                    &self.results_scroll_handle,
                    &self.sort_columns,
                    self.selected_cell,
                    on_header_click,
                    on_cell_click,
                    on_cell_secondary_click,
                    on_resize_start,
                    &table_config,
                    cx,
                ))
                .child(self.render_pagination_bar(cx))
                .child(render_status_bar(result, self.total_row_count(), cx));
        } else {
            container = container.child(
                div()
                    .flex_grow()
                    .items_center()
                    .justify_center()
                    .child(
                        Label::new("Run a query to see results")
                            .size(LabelSize::Small)
                            .color(ui::Color::Muted),
                    ),
            );
        }

        if let Some((column_name, cell_value)) = &self.expanded_cell {
            container = container.child(
                div()
                    .id("expanded-cell-container")
                    .flex_none()
                    .w_full()
                    .max_h(px(200.0))
                    .border_t_1()
                    .border_color(cx.theme().colors().border)
                    .bg(cx.theme().colors().surface_background)
                    .overflow_y_scroll()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .p_2()
                            .gap_1()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        Label::new(SharedString::from(column_name.clone()))
                                            .size(LabelSize::Small)
                                            .weight(gpui::FontWeight::BOLD)
                                            .color(ui::Color::Muted),
                                    )
                                    .child(
                                        Button::new("close-expanded", "")
                                            .icon(IconName::Close)
                                            .icon_size(IconSize::XSmall)
                                            .style(ButtonStyle::Subtle)
                                            .on_click(cx.listener(|this, _, _window, cx| {
                                                this.expanded_cell = None;
                                                cx.notify();
                                            })),
                                    ),
                            )
                            .child(
                                Label::new(SharedString::from(cell_value.clone()))
                                    .size(LabelSize::Small)
                                    .color(if cell_value == "NULL" {
                                        ui::Color::Muted
                                    } else {
                                        ui::Color::Default
                                    }),
                            ),
                    ),
            );
        }

        container
    }
}

const DATABASE_EXTENSIONS: &[&str] = &["db", "sqlite", "sqlite3"];

fn detect_database_files(workspace: &Workspace, cx: &App) -> Vec<PathBuf> {
    let project = workspace.project().read(cx);
    let mut found_paths = Vec::new();

    for worktree in project.visible_worktrees(cx) {
        let worktree_ref = worktree.read(cx);
        let root_abs_path = worktree_ref.abs_path();
        let snapshot = worktree_ref.snapshot();

        for entry in snapshot.entries(true, 0) {
            if !entry.is_file() {
                continue;
            }
            let extension = entry
                .path
                .as_unix_str()
                .rsplit('.')
                .next();

            if let Some(ext) = extension {
                if DATABASE_EXTENSIONS.contains(&ext) {
                    let abs_path = root_abs_path.join(entry.path.as_std_path());
                    found_paths.push(abs_path);
                }
            }
        }
    }

    found_paths
}

impl Render for DatabasePanel {
    fn render(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_connections = !self.connection_manager.read(cx).connections().is_empty();

        v_flex()
            .id("database_panel")
            .key_context("DatabasePanel")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(|this, _: &ExecuteQuery, window, cx| {
                this.execute_query(window, cx);
            }))
            .on_action(cx.listener(|this, _: &CancelQuery, _window, cx| {
                this.cancel_query(cx);
            }))
            .on_action(cx.listener(|this, _: &ExplainQuery, window, cx| {
                this.explain_query(window, cx);
            }))
            .on_action(cx.listener(|this, _: &menu::SelectNext, _window, cx| {
                if this.selected_cell.is_some() {
                    this.grid_move_down(cx);
                } else {
                    this.select_next_node(cx);
                }
            }))
            .on_action(cx.listener(|this, _: &menu::SelectPrevious, _window, cx| {
                if this.selected_cell.is_some() {
                    this.grid_move_up(cx);
                } else {
                    this.select_previous_node(cx);
                }
            }))
            .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| {
                this.confirm_selected_node(window, cx);
            }))
            .on_action(cx.listener(|this, _: &menu::Cancel, window, cx| {
                this.dismiss_context_menu_or_clear_filter(window, cx);
            }))
            .on_action(cx.listener(|this, _: &GridMoveLeft, _window, cx| {
                this.grid_move_left(cx);
            }))
            .on_action(cx.listener(|this, _: &GridMoveRight, _window, cx| {
                this.grid_move_right(cx);
            }))
            .on_action(cx.listener(|this, _: &CopyCellValue, _window, cx| {
                this.copy_selected_cell(cx);
            }))
            .on_action(cx.listener(|this, _: &CopyRowValues, _window, cx| {
                this.copy_selected_row(cx);
            }))
            .on_action(cx.listener(|this, _: &CopyAllResultsAsCsv, _window, cx| {
                this.copy_all_results_as_csv(cx);
            }))
            .on_action(cx.listener(|this, _: &CopyAllResultsAsJson, _window, cx| {
                this.copy_all_results_as_json(cx);
            }))
            .on_action(cx.listener(|this, _: &PreviousHistoryEntry, window, cx| {
                this.previous_history_entry(window, cx);
            }))
            .on_action(cx.listener(|this, _: &NextHistoryEntry, window, cx| {
                this.next_history_entry(window, cx);
            }))
            .on_action(cx.listener(|this, _: &ExportResultsCsv, _window, cx| {
                this.export_results_to_file(ExportFormat::Csv, cx);
            }))
            .on_action(cx.listener(|this, _: &ExportResultsJson, _window, cx| {
                this.export_results_to_file(ExportFormat::Json, cx);
            }))
            .on_action(cx.listener(|this, _: &ShowExportDialog, window, cx| {
                this.show_export_dialog(window, cx);
            }))
            .on_action(cx.listener(|this, _: &SaveQuery, window, cx| {
                this.save_current_query(window, cx);
            }))
            .on_action(cx.listener(|this, _: &ShowSavedQueries, _window, cx| {
                this.show_saved_queries = !this.show_saved_queries;
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &OpenSqliteFile, _window, cx| {
                this.open_sqlite_file_picker(cx);
            }))
            .on_action(cx.listener(|this, _: &ConnectPostgresql, _window, cx| {
                this.show_postgres_connection_form(cx);
            }))
            .on_action(cx.listener(|this, _: &ConnectMysql, _window, cx| {
                this.show_mysql_connection_form(cx);
            }))
            .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _window, cx| {
                if this.resizing_column.is_some() {
                    this.handle_resize_move(event.position.x.as_f32(), cx);
                }
            }))
            .on_mouse_up(MouseButton::Left, cx.listener(|this, _event: &MouseUpEvent, _window, _cx| {
                this.stop_column_resize();
            }))
            .drag_over::<ExternalPaths>(|style, _, _, _cx| {
                style.bg(gpui::opaque_grey(0.5, 0.1))
            })
            .on_drop(cx.listener(
                |this, external_paths: &ExternalPaths, _window, cx| {
                    this.handle_external_paths_drop(external_paths, cx);
                },
            ))
            .size_full()
            .overflow_hidden()
            .bg(cx.theme().colors().panel_background)
            .child(self.render_connections_section(window, cx))
            .when(!has_connections && self.connection_form.active_flow.is_none(), |this| {
                this.child(self.render_empty_state(cx))
            })
            .when(has_connections, |this| {
                this.child(self.render_schema_section(window, cx))
                    .child(self.render_query_section(window, cx))
                    .when(self.show_saved_queries, |this| {
                        this.child(self.render_saved_queries_panel(window, cx))
                    })
                    .child(self.render_results_section(window, cx))
            })
            .children(self.context_menu.as_ref().map(|(menu, position, _)| {
                deferred(
                    anchored()
                        .position(*position)
                        .anchor(Corner::TopLeft)
                        .child(menu.clone()),
                )
            }))
    }
}

impl Focusable for DatabasePanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for DatabasePanel {}

impl Panel for DatabasePanel {
    fn persistent_name() -> &'static str {
        "DatabaseViewer"
    }

    fn panel_key() -> &'static str {
        DATABASE_PANEL_KEY
    }

    fn position(&self, _window: &gpui::Window, cx: &App) -> DockPosition {
        DatabasePanelSettings::get_global(cx).dock
    }

    fn position_is_valid(&self, _position: DockPosition) -> bool {
        true
    }

    fn set_position(
        &mut self,
        position: DockPosition,
        _window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) {
        settings::update_settings_file(self.fs.clone(), cx, move |settings, _| {
            settings
                .database_panel
                .get_or_insert_default()
                .dock = Some(position.into());
        });
    }

    fn size(&self, _window: &gpui::Window, cx: &App) -> Pixels {
        self.width
            .unwrap_or_else(|| DatabasePanelSettings::get_global(cx).default_width)
    }

    fn set_size(
        &mut self,
        size: Option<Pixels>,
        _window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) {
        self.width = size;
        self.serialize(cx);
        cx.notify();
    }

    fn icon(&self, _window: &gpui::Window, cx: &App) -> Option<IconName> {
        DatabasePanelSettings::get_global(cx)
            .button
            .then_some(IconName::DatabaseZap)
    }

    fn icon_tooltip(&self, _window: &gpui::Window, _cx: &App) -> Option<&'static str> {
        Some("Database Viewer")
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        5
    }
}

impl PanelHeader for DatabasePanel {}
