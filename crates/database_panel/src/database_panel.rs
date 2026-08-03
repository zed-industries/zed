mod connection_modal;
mod schema;

use std::sync::Arc;

use anyhow::Result;
use fs::Fs;
use gpui::{
    Action, App, AsyncWindowContext, ClipboardItem, Context, DismissEvent, Entity, EventEmitter,
    FocusHandle, Focusable, Pixels, Point, Render, ScrollStrategy, SharedString, Subscription,
    UniformListScrollHandle, WeakEntity, Window, actions, anchored, deferred, px, uniform_list,
};
use gpui_tokio::Tokio;
use schema::{ColumnInfo, ConnectionConfig, DatabaseInfo, TableInfo, TableType};
use settings::{
    DatabaseAdapter, DatabaseConnectionContent, DockSide, RegisterSetting, Settings, SettingsStore,
};
use ui::{CommonAnimationExt as _, ContextMenu, Tooltip, prelude::*};
use workspace::{
    Panel, Workspace,
    dock::{DockPosition, PanelEvent},
};

pub use connection_modal::ConnectionModal;

actions!(
    database_panel,
    [
        /// Toggles focus on the database panel.
        ToggleFocus,
        /// Opens a dialog to add a new database connection.
        AddConnection,
        /// Removes the selected database connection.
        RemoveConnection,
        /// Expands the selected entry in the database panel.
        ExpandSelectedEntry,
        /// Collapses the selected entry in the database panel.
        CollapseSelectedEntry,
        /// Reloads the schema information of the selected entry, or of all
        /// connections when nothing is selected.
        Refresh,
        /// Copies the name of the selected entry to the clipboard.
        CopyName,
    ]
);

const DATABASE_PANEL_KEY: &str = "DatabasePanel";
const TREE_INDENT: f32 = 12.0;

#[derive(Debug, Clone, RegisterSetting)]
pub struct DatabasePanelSettings {
    pub button: bool,
    pub default_width: Pixels,
    pub dock: DockSide,
    pub connections: Vec<DatabaseConnectionContent>,
}

impl Settings for DatabasePanelSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let panel = content.database_panel.as_ref().unwrap();
        Self {
            button: panel.button.unwrap(),
            default_width: panel.default_width.map(px).unwrap(),
            dock: panel.dock.unwrap(),
            connections: panel.connections.clone().unwrap_or_default(),
        }
    }
}

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            workspace.toggle_panel_focus::<DatabasePanel>(window, cx);
        });
        workspace.register_action(|workspace, _: &AddConnection, window, cx| {
            ConnectionModal::toggle(workspace, window, cx);
        });
    })
    .detach();
}

enum LoadState<T> {
    NotLoaded,
    Loading,
    Loaded(T),
    Error(SharedString),
}

struct ConnectionState {
    content_index: usize,
    name: SharedString,
    config: ConnectionConfig,
    expanded: bool,
    databases: LoadState<Vec<DatabaseState>>,
}

struct DatabaseState {
    info: DatabaseInfo,
    expanded: bool,
    tables: LoadState<Vec<TableState>>,
}

struct TableState {
    info: TableInfo,
    expanded: bool,
    columns: LoadState<Vec<ColumnInfo>>,
}

#[derive(Debug, Clone)]
enum ListEntry {
    Connection {
        connection_ix: usize,
    },
    Database {
        connection_ix: usize,
        database_ix: usize,
    },
    Table {
        connection_ix: usize,
        database_ix: usize,
        table_ix: usize,
    },
    Column {
        connection_ix: usize,
        database_ix: usize,
        table_ix: usize,
        column_ix: usize,
    },
    Status {
        depth: usize,
        loading: bool,
        message: SharedString,
    },
}

impl ListEntry {
    fn depth(&self) -> usize {
        match self {
            ListEntry::Connection { .. } => 0,
            ListEntry::Database { .. } => 1,
            ListEntry::Table { .. } => 2,
            ListEntry::Column { .. } => 3,
            ListEntry::Status { depth, .. } => *depth,
        }
    }
}

/// A dock panel listing the configured database connections and lazily
/// exposing their schemas as a `connection > database > table > column` tree,
/// in the style of PhpStorm's database explorer. The panel is read-only: it
/// only ever introspects schemas and never modifies the databases.
pub struct DatabasePanel {
    workspace: WeakEntity<Workspace>,
    fs: Arc<dyn Fs>,
    focus_handle: FocusHandle,
    connections: Vec<ConnectionState>,
    entries: Vec<ListEntry>,
    selected_index: Option<usize>,
    scroll_handle: UniformListScrollHandle,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    _settings_subscription: Subscription,
}

fn resolve_connection(
    content: &DatabaseConnectionContent,
) -> Option<(SharedString, ConnectionConfig)> {
    match content.adapter.unwrap_or_default() {
        DatabaseAdapter::Sqlite => {
            let path = content.path.clone()?;
            let name = content.name.clone().unwrap_or_else(|| {
                std::path::Path::new(&path)
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_else(|| path.clone())
            });
            Some((name.into(), ConnectionConfig::Sqlite { path }))
        }
        DatabaseAdapter::Mariadb => {
            let host = content.host.clone().unwrap_or_else(|| "localhost".into());
            let port = content.port.unwrap_or(schema::MARIADB_DEFAULT_PORT);
            let username = content.username.clone().unwrap_or_else(|| "root".into());
            let name = content
                .name
                .clone()
                .unwrap_or_else(|| format!("{username}@{host}"));
            Some((
                name.into(),
                ConnectionConfig::MariaDb {
                    host,
                    port,
                    username,
                    password: content.password.clone(),
                    databases: content.databases.clone(),
                },
            ))
        }
    }
}

impl DatabasePanel {
    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<Entity<Self>> {
        workspace.update_in(&mut cx, |workspace, _, cx| Self::new(workspace, cx))
    }

    fn new(workspace: &mut Workspace, cx: &mut Context<Workspace>) -> Entity<Self> {
        let fs = workspace.project().read(cx).fs().clone();
        let weak_workspace = workspace.weak_handle();
        cx.new(|cx| {
            let mut this = Self {
                workspace: weak_workspace,
                fs,
                focus_handle: cx.focus_handle(),
                connections: Vec::new(),
                entries: Vec::new(),
                selected_index: None,
                scroll_handle: UniformListScrollHandle::new(),
                context_menu: None,
                _settings_subscription: cx.observe_global::<SettingsStore>(
                    |this: &mut Self, cx| this.settings_changed(cx),
                ),
            };
            this.settings_changed(cx);
            this
        })
    }

    fn settings_changed(&mut self, cx: &mut Context<Self>) {
        let contents = DatabasePanelSettings::get_global(cx).connections.clone();
        let mut previous = std::mem::take(&mut self.connections);
        for (content_index, content) in contents.iter().enumerate() {
            let Some((name, config)) = resolve_connection(content) else {
                continue;
            };
            let state = previous
                .iter()
                .position(|connection| connection.config == config)
                .map(|position| previous.remove(position));
            let mut state = state.unwrap_or_else(|| ConnectionState {
                content_index,
                name: name.clone(),
                config,
                expanded: false,
                databases: LoadState::NotLoaded,
            });
            state.content_index = content_index;
            state.name = name;
            self.connections.push(state);
        }
        self.rebuild_entries(cx);
    }

    fn rebuild_entries(&mut self, cx: &mut Context<Self>) {
        let mut entries = Vec::new();
        for (connection_ix, connection) in self.connections.iter().enumerate() {
            entries.push(ListEntry::Connection { connection_ix });
            if !connection.expanded {
                continue;
            }
            match &connection.databases {
                LoadState::NotLoaded => {}
                LoadState::Loading => entries.push(ListEntry::Status {
                    depth: 1,
                    loading: true,
                    message: "Connecting…".into(),
                }),
                LoadState::Error(error) => entries.push(ListEntry::Status {
                    depth: 1,
                    loading: false,
                    message: error.clone(),
                }),
                LoadState::Loaded(databases) => {
                    for (database_ix, database) in databases.iter().enumerate() {
                        entries.push(ListEntry::Database {
                            connection_ix,
                            database_ix,
                        });
                        if !database.expanded {
                            continue;
                        }
                        match &database.tables {
                            LoadState::NotLoaded => {}
                            LoadState::Loading => entries.push(ListEntry::Status {
                                depth: 2,
                                loading: true,
                                message: "Loading tables…".into(),
                            }),
                            LoadState::Error(error) => entries.push(ListEntry::Status {
                                depth: 2,
                                loading: false,
                                message: error.clone(),
                            }),
                            LoadState::Loaded(tables) => {
                                for (table_ix, table) in tables.iter().enumerate() {
                                    entries.push(ListEntry::Table {
                                        connection_ix,
                                        database_ix,
                                        table_ix,
                                    });
                                    if !table.expanded {
                                        continue;
                                    }
                                    match &table.columns {
                                        LoadState::NotLoaded => {}
                                        LoadState::Loading => entries.push(ListEntry::Status {
                                            depth: 3,
                                            loading: true,
                                            message: "Loading columns…".into(),
                                        }),
                                        LoadState::Error(error) => {
                                            entries.push(ListEntry::Status {
                                                depth: 3,
                                                loading: false,
                                                message: error.clone(),
                                            })
                                        }
                                        LoadState::Loaded(columns) => {
                                            for column_ix in 0..columns.len() {
                                                entries.push(ListEntry::Column {
                                                    connection_ix,
                                                    database_ix,
                                                    table_ix,
                                                    column_ix,
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self.entries = entries;
        if let Some(selected) = self.selected_index {
            if self.entries.is_empty() {
                self.selected_index = None;
            } else if selected >= self.entries.len() {
                self.selected_index = Some(self.entries.len() - 1);
            }
        }
        cx.notify();
    }

    fn database_state_mut(
        &mut self,
        connection_ix: usize,
        database_ix: usize,
    ) -> Option<&mut DatabaseState> {
        match &mut self.connections.get_mut(connection_ix)?.databases {
            LoadState::Loaded(databases) => databases.get_mut(database_ix),
            _ => None,
        }
    }

    fn table_state_mut(
        &mut self,
        connection_ix: usize,
        database_ix: usize,
        table_ix: usize,
    ) -> Option<&mut TableState> {
        match &mut self.database_state_mut(connection_ix, database_ix)?.tables {
            LoadState::Loaded(tables) => tables.get_mut(table_ix),
            _ => None,
        }
    }

    fn is_expanded(&self, entry: &ListEntry) -> Option<bool> {
        match *entry {
            ListEntry::Connection { connection_ix } => {
                Some(self.connections.get(connection_ix)?.expanded)
            }
            ListEntry::Database {
                connection_ix,
                database_ix,
            } => match &self.connections.get(connection_ix)?.databases {
                LoadState::Loaded(databases) => Some(databases.get(database_ix)?.expanded),
                _ => None,
            },
            ListEntry::Table {
                connection_ix,
                database_ix,
                table_ix,
            } => match &self.connections.get(connection_ix)?.databases {
                LoadState::Loaded(databases) => match &databases.get(database_ix)?.tables {
                    LoadState::Loaded(tables) => Some(tables.get(table_ix)?.expanded),
                    _ => None,
                },
                _ => None,
            },
            ListEntry::Column { .. } | ListEntry::Status { .. } => None,
        }
    }

    fn toggle_expanded(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(entry) = self.entries.get(index).cloned() else {
            return;
        };
        match entry {
            ListEntry::Connection { connection_ix } => {
                let Some(connection) = self.connections.get_mut(connection_ix) else {
                    return;
                };
                connection.expanded = !connection.expanded;
                if connection.expanded
                    && matches!(
                        connection.databases,
                        LoadState::NotLoaded | LoadState::Error(_)
                    )
                {
                    self.load_databases(connection_ix, cx);
                }
            }
            ListEntry::Database {
                connection_ix,
                database_ix,
            } => {
                let Some(database) = self.database_state_mut(connection_ix, database_ix) else {
                    return;
                };
                database.expanded = !database.expanded;
                if database.expanded
                    && matches!(database.tables, LoadState::NotLoaded | LoadState::Error(_))
                {
                    self.load_tables(connection_ix, database_ix, cx);
                }
            }
            ListEntry::Table {
                connection_ix,
                database_ix,
                table_ix,
            } => {
                let Some(table) = self.table_state_mut(connection_ix, database_ix, table_ix) else {
                    return;
                };
                table.expanded = !table.expanded;
                if table.expanded
                    && matches!(table.columns, LoadState::NotLoaded | LoadState::Error(_))
                {
                    self.load_columns(connection_ix, database_ix, table_ix, cx);
                }
            }
            ListEntry::Column { .. } | ListEntry::Status { .. } => return,
        }
        self.selected_index = Some(index);
        self.rebuild_entries(cx);
    }

    fn load_databases(&mut self, connection_ix: usize, cx: &mut Context<Self>) {
        let Some(connection) = self.connections.get_mut(connection_ix) else {
            return;
        };
        connection.databases = LoadState::Loading;
        let config = connection.config.clone();
        let task = Tokio::spawn_result(cx, {
            let config = config.clone();
            async move { schema::list_databases(config).await }
        });
        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| {
                let Some(connection) = this.connections.get_mut(connection_ix) else {
                    return;
                };
                if connection.config != config {
                    return;
                }
                connection.databases = match result {
                    Ok(databases) => LoadState::Loaded(
                        databases
                            .into_iter()
                            .map(|info| DatabaseState {
                                info,
                                expanded: false,
                                tables: LoadState::NotLoaded,
                            })
                            .collect(),
                    ),
                    Err(error) => LoadState::Error(format!("{error:#}").into()),
                };
                this.rebuild_entries(cx);
            })
        })
        .detach_and_log_err(cx);
    }

    fn load_tables(&mut self, connection_ix: usize, database_ix: usize, cx: &mut Context<Self>) {
        let Some(connection) = self.connections.get(connection_ix) else {
            return;
        };
        let config = connection.config.clone();
        let Some(database) = self.database_state_mut(connection_ix, database_ix) else {
            return;
        };
        let database_name = database.info.name.clone();
        database.tables = LoadState::Loading;
        let task = Tokio::spawn_result(cx, {
            let config = config.clone();
            let database_name = database_name.clone();
            async move { schema::list_tables(config, database_name).await }
        });
        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| {
                if this
                    .connections
                    .get(connection_ix)
                    .is_none_or(|connection| connection.config != config)
                {
                    return;
                }
                let Some(database) = this.database_state_mut(connection_ix, database_ix) else {
                    return;
                };
                if database.info.name != database_name {
                    return;
                }
                database.tables = match result {
                    Ok(tables) => LoadState::Loaded(
                        tables
                            .into_iter()
                            .map(|info| TableState {
                                info,
                                expanded: false,
                                columns: LoadState::NotLoaded,
                            })
                            .collect(),
                    ),
                    Err(error) => LoadState::Error(format!("{error:#}").into()),
                };
                this.rebuild_entries(cx);
            })
        })
        .detach_and_log_err(cx);
    }

    fn load_columns(
        &mut self,
        connection_ix: usize,
        database_ix: usize,
        table_ix: usize,
        cx: &mut Context<Self>,
    ) {
        let Some(connection) = self.connections.get(connection_ix) else {
            return;
        };
        let config = connection.config.clone();
        let Some(database) = self.database_state_mut(connection_ix, database_ix) else {
            return;
        };
        let database_name = database.info.name.clone();
        let Some(table) = self.table_state_mut(connection_ix, database_ix, table_ix) else {
            return;
        };
        let table_name = table.info.name.clone();
        table.columns = LoadState::Loading;
        let task = Tokio::spawn_result(cx, {
            let config = config.clone();
            let table_name = table_name.clone();
            async move { schema::list_columns(config, database_name, table_name).await }
        });
        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| {
                if this
                    .connections
                    .get(connection_ix)
                    .is_none_or(|connection| connection.config != config)
                {
                    return;
                }
                let Some(table) = this.table_state_mut(connection_ix, database_ix, table_ix) else {
                    return;
                };
                if table.info.name != table_name {
                    return;
                }
                table.columns = match result {
                    Ok(columns) => LoadState::Loaded(columns),
                    Err(error) => LoadState::Error(format!("{error:#}").into()),
                };
                this.rebuild_entries(cx);
            })
        })
        .detach_and_log_err(cx);
    }

    fn select_next(&mut self, _: &menu::SelectNext, _: &mut Window, cx: &mut Context<Self>) {
        if self.entries.is_empty() {
            return;
        }
        let index = self
            .selected_index
            .map_or(0, |index| (index + 1).min(self.entries.len() - 1));
        self.select_index(index, cx);
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.entries.is_empty() {
            return;
        }
        let index = self
            .selected_index
            .map_or(0, |index| index.saturating_sub(1));
        self.select_index(index, cx);
    }

    fn select_first(&mut self, _: &menu::SelectFirst, _: &mut Window, cx: &mut Context<Self>) {
        if !self.entries.is_empty() {
            self.select_index(0, cx);
        }
    }

    fn select_last(&mut self, _: &menu::SelectLast, _: &mut Window, cx: &mut Context<Self>) {
        if !self.entries.is_empty() {
            self.select_index(self.entries.len() - 1, cx);
        }
    }

    fn select_index(&mut self, index: usize, cx: &mut Context<Self>) {
        self.selected_index = Some(index);
        self.scroll_handle
            .scroll_to_item(index, ScrollStrategy::Center);
        cx.notify();
    }

    fn confirm(&mut self, _: &menu::Confirm, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(index) = self.selected_index {
            self.toggle_expanded(index, cx);
        }
    }

    fn expand_selected_entry(
        &mut self,
        _: &ExpandSelectedEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(index) = self.selected_index else {
            return;
        };
        let Some(entry) = self.entries.get(index).cloned() else {
            return;
        };
        match self.is_expanded(&entry) {
            Some(false) => self.toggle_expanded(index, cx),
            _ => self.select_next(&menu::SelectNext, window, cx),
        }
    }

    fn collapse_selected_entry(
        &mut self,
        _: &CollapseSelectedEntry,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(index) = self.selected_index else {
            return;
        };
        let Some(entry) = self.entries.get(index).cloned() else {
            return;
        };
        if self.is_expanded(&entry) == Some(true) {
            self.toggle_expanded(index, cx);
            return;
        }
        // Move to the parent entry: the closest previous entry that is less deep.
        let depth = entry.depth();
        if let Some(parent_index) = self.entries[..index]
            .iter()
            .rposition(|candidate| candidate.depth() < depth)
        {
            self.select_index(parent_index, cx);
        }
    }

    fn refresh(&mut self, _: &Refresh, _: &mut Window, cx: &mut Context<Self>) {
        match self.selected_index {
            Some(index) => self.refresh_entry(index, cx),
            None => self.refresh_all(cx),
        }
    }

    fn refresh_all(&mut self, cx: &mut Context<Self>) {
        for connection_ix in 0..self.connections.len() {
            let Some(connection) = self.connections.get_mut(connection_ix) else {
                continue;
            };
            connection.databases = LoadState::NotLoaded;
            if connection.expanded {
                self.load_databases(connection_ix, cx);
            }
        }
        self.rebuild_entries(cx);
    }

    fn refresh_entry(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(entry) = self.entries.get(index).cloned() else {
            return;
        };
        match entry {
            ListEntry::Connection { connection_ix } => {
                let Some(connection) = self.connections.get_mut(connection_ix) else {
                    return;
                };
                connection.databases = LoadState::NotLoaded;
                if connection.expanded {
                    self.load_databases(connection_ix, cx);
                }
            }
            ListEntry::Database {
                connection_ix,
                database_ix,
            } => {
                let Some(database) = self.database_state_mut(connection_ix, database_ix) else {
                    return;
                };
                database.tables = LoadState::NotLoaded;
                if database.expanded {
                    self.load_tables(connection_ix, database_ix, cx);
                }
            }
            ListEntry::Table {
                connection_ix,
                database_ix,
                table_ix,
            }
            | ListEntry::Column {
                connection_ix,
                database_ix,
                table_ix,
                ..
            } => {
                let Some(table) = self.table_state_mut(connection_ix, database_ix, table_ix) else {
                    return;
                };
                table.columns = LoadState::NotLoaded;
                if table.expanded {
                    self.load_columns(connection_ix, database_ix, table_ix, cx);
                }
            }
            _ => return,
        }
        self.rebuild_entries(cx);
    }

    fn entry_name(&self, entry: &ListEntry) -> Option<SharedString> {
        match *entry {
            ListEntry::Connection { connection_ix } => {
                Some(self.connections.get(connection_ix)?.name.clone())
            }
            ListEntry::Database {
                connection_ix,
                database_ix,
            } => {
                let connection = self.connections.get(connection_ix)?;
                match &connection.databases {
                    LoadState::Loaded(databases) => {
                        Some(databases.get(database_ix)?.info.name.clone().into())
                    }
                    _ => None,
                }
            }
            ListEntry::Table {
                connection_ix,
                database_ix,
                table_ix,
            } => {
                let connection = self.connections.get(connection_ix)?;
                match &connection.databases {
                    LoadState::Loaded(databases) => match &databases.get(database_ix)?.tables {
                        LoadState::Loaded(tables) => {
                            Some(tables.get(table_ix)?.info.name.clone().into())
                        }
                        _ => None,
                    },
                    _ => None,
                }
            }
            ListEntry::Column {
                connection_ix,
                database_ix,
                table_ix,
                column_ix,
            } => {
                let connection = self.connections.get(connection_ix)?;
                match &connection.databases {
                    LoadState::Loaded(databases) => match &databases.get(database_ix)?.tables {
                        LoadState::Loaded(tables) => match &tables.get(table_ix)?.columns {
                            LoadState::Loaded(columns) => {
                                Some(columns.get(column_ix)?.name.clone().into())
                            }
                            _ => None,
                        },
                        _ => None,
                    },
                    _ => None,
                }
            }
            ListEntry::Status { .. } => None,
        }
    }

    fn copy_name(&mut self, _: &CopyName, _: &mut Window, cx: &mut Context<Self>) {
        let Some(index) = self.selected_index else {
            return;
        };
        let Some(entry) = self.entries.get(index) else {
            return;
        };
        if let Some(name) = self.entry_name(entry) {
            cx.write_to_clipboard(ClipboardItem::new_string(name.to_string()));
        }
    }

    fn add_connection(&mut self, _: &AddConnection, window: &mut Window, cx: &mut Context<Self>) {
        self.open_connection_modal(window, cx);
    }

    fn open_connection_modal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                ConnectionModal::toggle(workspace, window, cx);
            });
        }
    }

    fn remove_connection(&mut self, _: &RemoveConnection, _: &mut Window, cx: &mut Context<Self>) {
        let Some(index) = self.selected_index else {
            return;
        };
        let Some(ListEntry::Connection { connection_ix }) = self.entries.get(index) else {
            return;
        };
        let Some(connection) = self.connections.get(*connection_ix) else {
            return;
        };
        let content_index = connection.content_index;
        settings::update_settings_file(self.fs.clone(), cx, move |content, _| {
            if let Some(connections) = content
                .database_panel
                .get_or_insert_default()
                .connections
                .as_mut()
                && content_index < connections.len()
            {
                connections.remove(content_index);
            }
        });
    }

    fn deploy_context_menu(
        &mut self,
        position: Point<Pixels>,
        index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(entry) = self.entries.get(index).cloned() else {
            return;
        };
        if matches!(entry, ListEntry::Status { .. }) {
            return;
        }
        self.selected_index = Some(index);
        let context_menu = ContextMenu::build(window, cx, |menu, _, _| {
            menu.context(self.focus_handle.clone())
                .action("Refresh", Refresh.boxed_clone())
                .action("Copy Name", CopyName.boxed_clone())
                .when(matches!(entry, ListEntry::Connection { .. }), |menu| {
                    menu.separator()
                        .action("Remove Connection", RemoveConnection.boxed_clone())
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

    fn row_base(
        &self,
        index: usize,
        depth: usize,
        selected: bool,
        cx: &mut Context<Self>,
    ) -> gpui::Stateful<Div> {
        let colors = cx.theme().colors();
        h_flex()
            .h(rems(1.5))
            .w_full()
            .min_w_0()
            .px_1()
            .gap_1()
            .pl(px(depth as f32 * TREE_INDENT + 4.0))
            .when(selected, |this| this.bg(colors.element_selected))
            .when(!selected, |this| {
                this.hover(|style| style.bg(colors.ghost_element_hover))
            })
            .id(("database-panel-entry", index))
    }

    fn render_entry(&self, index: usize, cx: &mut Context<Self>) -> AnyElement {
        let Some(entry) = self.entries.get(index) else {
            return div().into_any_element();
        };
        let selected = self.selected_index == Some(index);
        let depth = entry.depth();

        if let ListEntry::Status {
            loading, message, ..
        } = entry
        {
            let icon = if *loading {
                Icon::new(IconName::LoadCircle)
                    .size(IconSize::XSmall)
                    .color(Color::Muted)
                    .with_keyed_rotate_animation(("database-panel-loading", index), 2)
                    .into_any_element()
            } else {
                Icon::new(IconName::XCircle)
                    .size(IconSize::XSmall)
                    .color(Color::Error)
                    .into_any_element()
            };
            let tooltip = (!loading).then(|| message.clone());
            return self
                .row_base(index, depth, false, cx)
                .child(icon)
                .child(
                    Label::new(message.clone())
                        .size(LabelSize::Small)
                        .color(if *loading { Color::Muted } else { Color::Error })
                        .single_line()
                        .truncate(),
                )
                .when_some(tooltip, |this, message| {
                    this.tooltip(Tooltip::text(message))
                })
                .into_any_element();
        }

        let expanded = self.is_expanded(entry);
        let (icon, name, detail, tooltip): (
            AnyElement,
            SharedString,
            Option<SharedString>,
            Option<SharedString>,
        ) = match *entry {
            ListEntry::Connection { connection_ix } => {
                let Some(connection) = self.connections.get(connection_ix) else {
                    return div().into_any_element();
                };
                let icon_name = match connection.config {
                    ConnectionConfig::Sqlite { .. } => IconName::Database,
                    ConnectionConfig::MariaDb { .. } => IconName::Server,
                };
                let color = match &connection.databases {
                    LoadState::Loaded(_) => Color::Success,
                    LoadState::Error(_) => Color::Error,
                    _ => Color::Muted,
                };
                let (detail, tooltip) = match &connection.config {
                    ConnectionConfig::Sqlite { path } => {
                        ("sqlite".into(), format!("SQLite · {path}"))
                    }
                    ConnectionConfig::MariaDb {
                        host,
                        port,
                        username,
                        ..
                    } => (
                        format!("{host}:{port}").into(),
                        format!("MariaDB · {username}@{host}:{port}"),
                    ),
                };
                (
                    Icon::new(icon_name)
                        .size(IconSize::Small)
                        .color(color)
                        .into_any_element(),
                    connection.name.clone(),
                    Some(detail),
                    Some(tooltip.into()),
                )
            }
            ListEntry::Database {
                connection_ix,
                database_ix,
            } => {
                let Some(connection) = self.connections.get(connection_ix) else {
                    return div().into_any_element();
                };
                let LoadState::Loaded(databases) = &connection.databases else {
                    return div().into_any_element();
                };
                let Some(database) = databases.get(database_ix) else {
                    return div().into_any_element();
                };
                let mut parts = Vec::new();
                if let Some(charset) = &database.info.charset {
                    parts.push(charset.clone());
                }
                if let Some(collation) = &database.info.collation {
                    parts.push(collation.clone());
                }
                let tooltip = (!parts.is_empty()).then(|| SharedString::from(parts.join(" · ")));
                (
                    Icon::new(IconName::Database)
                        .size(IconSize::Small)
                        .color(Color::Muted)
                        .into_any_element(),
                    database.info.name.clone().into(),
                    None,
                    tooltip,
                )
            }
            ListEntry::Table {
                connection_ix,
                database_ix,
                table_ix,
            } => {
                let Some(connection) = self.connections.get(connection_ix) else {
                    return div().into_any_element();
                };
                let LoadState::Loaded(databases) = &connection.databases else {
                    return div().into_any_element();
                };
                let Some(database) = databases.get(database_ix) else {
                    return div().into_any_element();
                };
                let LoadState::Loaded(tables) = &database.tables else {
                    return div().into_any_element();
                };
                let Some(table) = tables.get(table_ix) else {
                    return div().into_any_element();
                };
                let info = &table.info;
                let icon_name = match info.table_type {
                    TableType::Table => IconName::Table,
                    TableType::View => IconName::Eye,
                };
                let detail = info
                    .row_count
                    .map(|count| SharedString::from(format!("{} rows", format_count(count))));
                let mut parts = Vec::new();
                if info.table_type == TableType::View {
                    parts.push("view".to_string());
                }
                if let Some(engine) = &info.engine {
                    parts.push(engine.clone());
                }
                if let Some(count) = info.row_count {
                    parts.push(format!("~{} rows", format_count(count)));
                }
                if let Some(size) = info.size_bytes {
                    parts.push(format_size(size));
                }
                if let Some(collation) = &info.collation {
                    parts.push(collation.clone());
                }
                if let Some(comment) = &info.comment {
                    parts.push(comment.clone());
                }
                let tooltip = (!parts.is_empty()).then(|| SharedString::from(parts.join(" · ")));
                (
                    Icon::new(icon_name)
                        .size(IconSize::Small)
                        .color(Color::Muted)
                        .into_any_element(),
                    info.name.clone().into(),
                    detail,
                    tooltip,
                )
            }
            ListEntry::Column {
                connection_ix,
                database_ix,
                table_ix,
                column_ix,
            } => {
                let Some(connection) = self.connections.get(connection_ix) else {
                    return div().into_any_element();
                };
                let LoadState::Loaded(databases) = &connection.databases else {
                    return div().into_any_element();
                };
                let Some(database) = databases.get(database_ix) else {
                    return div().into_any_element();
                };
                let LoadState::Loaded(tables) = &database.tables else {
                    return div().into_any_element();
                };
                let Some(table) = tables.get(table_ix) else {
                    return div().into_any_element();
                };
                let LoadState::Loaded(columns) = &table.columns else {
                    return div().into_any_element();
                };
                let Some(column) = columns.get(column_ix) else {
                    return div().into_any_element();
                };
                let (icon_name, icon_color) = if column.primary_key {
                    (IconName::Key, Color::Warning)
                } else if column.foreign_key.is_some() {
                    (IconName::Key, Color::Info)
                } else {
                    (IconName::Columns, Color::Muted)
                };
                let mut parts = vec![column.data_type.clone()];
                parts.push(if column.nullable {
                    "nullable".to_string()
                } else {
                    "not null".to_string()
                });
                if let Some(default) = &column.default {
                    parts.push(format!("default: {default}"));
                }
                if column.primary_key {
                    parts.push("primary key".to_string());
                }
                if column.unique {
                    parts.push("unique".to_string());
                }
                if column.auto_increment {
                    parts.push("auto increment".to_string());
                }
                if let Some(target) = &column.foreign_key {
                    parts.push(format!("references {target}"));
                }
                if let Some(charset) = &column.charset {
                    parts.push(charset.clone());
                }
                if let Some(collation) = &column.collation {
                    parts.push(collation.clone());
                }
                if let Some(comment) = &column.comment {
                    parts.push(comment.clone());
                }
                (
                    Icon::new(icon_name)
                        .size(IconSize::Small)
                        .color(icon_color)
                        .into_any_element(),
                    column.name.clone().into(),
                    Some(column.data_type.clone().into()),
                    Some(parts.join(" · ").into()),
                )
            }
            ListEntry::Status { .. } => return div().into_any_element(),
        };

        let chevron = match expanded {
            Some(expanded) => Icon::new(if expanded {
                IconName::ChevronDown
            } else {
                IconName::ChevronRight
            })
            .size(IconSize::XSmall)
            .color(Color::Muted)
            .into_any_element(),
            None => div().w_3().flex_shrink_0().into_any_element(),
        };

        self.row_base(index, depth, selected, cx)
            .cursor_pointer()
            .child(chevron)
            .child(icon)
            .child(
                Label::new(name)
                    .size(LabelSize::Small)
                    .single_line()
                    .truncate(),
            )
            .when_some(detail, |this, detail| {
                this.child(
                    Label::new(detail)
                        .size(LabelSize::XSmall)
                        .color(Color::Muted)
                        .single_line()
                        .truncate(),
                )
            })
            .when_some(tooltip, |this, tooltip| {
                this.tooltip(Tooltip::text(tooltip))
            })
            .on_click(cx.listener(move |this, _, window, cx| {
                this.focus_handle.focus(window, cx);
                this.toggle_expanded(index, cx);
            }))
            .on_mouse_down(
                gpui::MouseButton::Right,
                cx.listener(move |this, event: &gpui::MouseDownEvent, window, cx| {
                    cx.stop_propagation();
                    this.deploy_context_menu(event.position, index, window, cx);
                }),
            )
            .into_any_element()
    }

    fn render_header(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .px_2()
            .py_1()
            .gap_1()
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                Label::new("Connections")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        IconButton::new("database-panel-refresh", IconName::RotateCw)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted)
                            .tooltip(Tooltip::text("Refresh All Connections"))
                            .on_click(cx.listener(|this, _, _, cx| this.refresh_all(cx))),
                    )
                    .child(
                        IconButton::new("database-panel-add", IconName::Plus)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted)
                            .tooltip(Tooltip::text("Add Connection"))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.open_connection_modal(window, cx)
                            })),
                    ),
            )
    }
}

fn format_count(count: u64) -> String {
    if count >= 1_000_000 {
        format!("{:.1}M", count as f64 / 1_000_000.0)
    } else if count >= 10_000 {
        format!("{:.1}k", count as f64 / 1_000.0)
    } else {
        count.to_string()
    }
}

fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit + 1 < UNITS.len() {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{bytes} B")
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}

impl Render for DatabasePanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entry_count = self.entries.len();
        let has_connections = !self.connections.is_empty();

        v_flex()
            .id("database-panel")
            .key_context(DATABASE_PANEL_KEY)
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().panel_background)
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::expand_selected_entry))
            .on_action(cx.listener(Self::collapse_selected_entry))
            .on_action(cx.listener(Self::refresh))
            .on_action(cx.listener(Self::copy_name))
            .on_action(cx.listener(Self::add_connection))
            .on_action(cx.listener(Self::remove_connection))
            .child(self.render_header(cx))
            .when(!has_connections, |this| {
                this.child(
                    v_flex()
                        .size_full()
                        .items_center()
                        .justify_center()
                        .gap_2()
                        .child(
                            Label::new("No database connections")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                        .child(
                            Button::new("database-panel-add-first", "Add Connection").on_click(
                                cx.listener(|this, _, window, cx| {
                                    this.open_connection_modal(window, cx)
                                }),
                            ),
                        ),
                )
            })
            .when(has_connections, |this| {
                this.child(
                    uniform_list(
                        "database-panel-entries",
                        entry_count,
                        cx.processor(|this, range: std::ops::Range<usize>, _window, cx| {
                            range.map(|index| this.render_entry(index, cx)).collect()
                        }),
                    )
                    .size_full()
                    .track_scroll(&self.scroll_handle),
                )
            })
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

impl Panel for DatabasePanel {
    fn persistent_name() -> &'static str {
        "Database Panel"
    }

    fn panel_key() -> &'static str {
        DATABASE_PANEL_KEY
    }

    fn position(&self, _: &Window, cx: &App) -> DockPosition {
        match DatabasePanelSettings::get_global(cx).dock {
            DockSide::Left => DockPosition::Left,
            DockSide::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
        settings::update_settings_file(self.fs.clone(), cx, move |settings, _| {
            let dock = match position {
                DockPosition::Left | DockPosition::Bottom => DockSide::Left,
                DockPosition::Right => DockSide::Right,
            };
            settings.database_panel.get_or_insert_default().dock = Some(dock);
        });
    }

    fn default_size(&self, _: &Window, cx: &App) -> Pixels {
        DatabasePanelSettings::get_global(cx).default_width
    }

    fn icon(&self, _: &Window, cx: &App) -> Option<IconName> {
        DatabasePanelSettings::get_global(cx)
            .button
            .then_some(IconName::Database)
    }

    fn icon_tooltip(&self, _: &Window, _: &App) -> Option<&'static str> {
        Some("Database Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        8
    }

    fn hide_button_setting(&self, _: &App) -> Option<workspace::HideStatusItem> {
        Some(workspace::HideStatusItem::new(|settings| {
            settings.database_panel.get_or_insert_default().button = Some(false);
        }))
    }
}

impl Focusable for DatabasePanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for DatabasePanel {}
