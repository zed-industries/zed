use std::collections::HashSet;
use std::ops::Range;
use std::sync::Arc;

use database_client::{TableInfo, TableRef};
use fs::Fs;
use gpui::{
    Action, AnyElement, App, AsyncWindowContext, Context, DismissEvent, Entity, EventEmitter,
    FocusHandle, Focusable, ListSizingBehavior, MouseDownEvent, ParentElement, Pixels, Point,
    Render, Styled, Subscription, UniformListScrollHandle, WeakEntity, Window, actions, anchored,
    deferred, px, uniform_list,
};
use settings::update_settings_file;
use ui::{ContextMenu, IconName, ListItem, Tooltip, prelude::*};
use util::ResultExt as _;
use workspace::{
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};

use crate::connection_modal::ConnectionModal;
use crate::connection_store::{
    ClientFactory, ConnectionStatus, ConnectionStore, ConnectionStoreEvent, credentials_url,
    default_client_factory,
};

actions!(
    database_panel,
    [
        /// Toggles the database panel.
        Toggle,
        /// Toggles focus on the database panel.
        ToggleFocus,
        /// Opens the new connection dialog.
        AddConnection,
        /// Reconnects the selected connection and reloads its tree.
        RefreshConnection,
        /// Opens the edit dialog for the selected connection.
        EditConnection,
        /// Removes the selected connection and deletes its saved password.
        RemoveConnection,
        /// Opens a new SQL query editor for the selected connection.
        NewSqlQuery,
    ]
);

/// Identifies an expandable node in the connections tree.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TreeNodeId {
    Connection(String),
    Database(String, String),
    Schema(String, String, String),
}

/// A single visible row in the flattened tree, fed to `uniform_list`.
enum TreeRow {
    Connection {
        name: String,
        status: ConnectionStatus,
        expanded: bool,
    },
    Database {
        connection: String,
        name: String,
        expanded: bool,
        loading: bool,
    },
    Schema {
        connection: String,
        database: String,
        name: String,
        expanded: bool,
        loading: bool,
    },
    Table {
        connection: String,
        database: String,
        schema: String,
        info: TableInfo,
    },
    Loading {
        depth: usize,
    },
    Error {
        depth: usize,
        message: String,
    },
}

pub struct DatabasePanel {
    workspace: WeakEntity<Workspace>,
    fs: Arc<dyn Fs>,
    store: Entity<ConnectionStore>,
    focus_handle: FocusHandle,
    expanded: HashSet<TreeNodeId>,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    /// The connection the currently-open context menu (and its actions) target.
    menu_target: Option<String>,
    /// The database the currently-open context menu targets, when it was opened
    /// on a database node. `None` means the menu was opened on a connection, in
    /// which case `NewSqlQuery` uses the connection's starting database.
    menu_target_database: Option<String>,
    scroll_handle: UniformListScrollHandle,
    _subscriptions: Vec<Subscription>,
}

impl DatabasePanel {
    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> anyhow::Result<Entity<Self>> {
        workspace.update_in(&mut cx, |workspace, window, cx| {
            Self::new(workspace, window, cx)
        })
    }

    fn new(
        workspace: &mut Workspace,
        _window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let fs = workspace.app_state().fs.clone();
        let workspace_handle = cx.entity().downgrade();
        cx.new(|cx| {
            let client_factory: ClientFactory = default_client_factory(cx);
            let store = cx.new(|cx| ConnectionStore::new(client_factory, cx));
            let mut subscriptions = vec![cx.observe(&store, |_, _, cx| cx.notify())];
            subscriptions.push(cx.subscribe(&store, Self::on_store_event));
            DatabasePanel {
                workspace: workspace_handle,
                fs,
                store,
                focus_handle: cx.focus_handle(),
                expanded: HashSet::new(),
                context_menu: None,
                menu_target: None,
                menu_target_database: None,
                scroll_handle: UniformListScrollHandle::new(),
                _subscriptions: subscriptions,
            }
        })
    }

    fn on_store_event(
        &mut self,
        _store: Entity<ConnectionStore>,
        event: &ConnectionStoreEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            ConnectionStoreEvent::ConnectionError { name, message } => {
                let message = format!("{name}: {message}");
                self.workspace
                    .update(cx, |workspace, cx| {
                        workspace.show_error(message, cx);
                    })
                    .log_err();
            }
        }
    }

    fn is_expanded(&self, node: &TreeNodeId) -> bool {
        self.expanded.contains(node)
    }

    /// Walks the store's tree, emitting only the rows that are currently
    /// visible given the set of expanded nodes.
    fn build_rows(&self, cx: &App) -> Vec<TreeRow> {
        let store = self.store.read(cx);
        let mut rows = Vec::new();

        for connection in store.connections() {
            let connection_name = connection.config.name.clone();
            let connection_id = TreeNodeId::Connection(connection_name.clone());
            let connection_expanded = self.is_expanded(&connection_id);
            rows.push(TreeRow::Connection {
                name: connection_name.clone(),
                status: connection.status.clone(),
                expanded: connection_expanded,
            });

            if !connection_expanded {
                continue;
            }

            match &connection.databases {
                None => {
                    if connection.status == ConnectionStatus::Connecting {
                        rows.push(TreeRow::Loading { depth: 1 });
                    } else if let ConnectionStatus::Error(message) = &connection.status {
                        rows.push(TreeRow::Error {
                            depth: 1,
                            message: message.clone(),
                        });
                    }
                }
                Some(databases) => {
                    for database in databases {
                        let database_id =
                            TreeNodeId::Database(connection_name.clone(), database.name.clone());
                        let database_expanded = self.is_expanded(&database_id);
                        rows.push(TreeRow::Database {
                            connection: connection_name.clone(),
                            name: database.name.clone(),
                            expanded: database_expanded,
                            loading: database.loading,
                        });

                        if !database_expanded {
                            continue;
                        }

                        if let Some(message) = &database.error {
                            rows.push(TreeRow::Error {
                                depth: 2,
                                message: message.clone(),
                            });
                        }

                        match &database.schemas {
                            None => {
                                if database.loading {
                                    rows.push(TreeRow::Loading { depth: 2 });
                                }
                            }
                            Some(schemas) => {
                                for schema in schemas {
                                    let schema_id = TreeNodeId::Schema(
                                        connection_name.clone(),
                                        database.name.clone(),
                                        schema.name.clone(),
                                    );
                                    let schema_expanded = self.is_expanded(&schema_id);
                                    rows.push(TreeRow::Schema {
                                        connection: connection_name.clone(),
                                        database: database.name.clone(),
                                        name: schema.name.clone(),
                                        expanded: schema_expanded,
                                        loading: schema.loading,
                                    });

                                    if !schema_expanded {
                                        continue;
                                    }

                                    if let Some(message) = &schema.error {
                                        rows.push(TreeRow::Error {
                                            depth: 3,
                                            message: message.clone(),
                                        });
                                    }

                                    match &schema.tables {
                                        None => {
                                            if schema.loading {
                                                rows.push(TreeRow::Loading { depth: 3 });
                                            }
                                        }
                                        Some(tables) => {
                                            for info in tables {
                                                rows.push(TreeRow::Table {
                                                    connection: connection_name.clone(),
                                                    database: database.name.clone(),
                                                    schema: schema.name.clone(),
                                                    info: info.clone(),
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

        rows
    }

    fn toggle_node(&mut self, node: TreeNodeId, cx: &mut Context<Self>) {
        if self.expanded.remove(&node) {
            cx.notify();
            return;
        }
        self.expanded.insert(node.clone());

        // Trigger lazy loading for the newly-expanded node.
        match node {
            TreeNodeId::Connection(connection) => {
                let needs_connect = self.store.read(cx).connections().iter().any(|state| {
                    state.config.name == connection
                        && state.databases.is_none()
                        && state.status != ConnectionStatus::Connecting
                });
                if needs_connect {
                    self.store
                        .update(cx, |store, cx| store.connect(&connection, cx));
                }
            }
            TreeNodeId::Database(connection, database) => {
                self.store.update(cx, |store, cx| {
                    store.load_schemas(&connection, &database, cx)
                });
            }
            TreeNodeId::Schema(connection, database, schema) => {
                self.store.update(cx, |store, cx| {
                    store.load_tables(&connection, &database, &schema, cx)
                });
            }
        }
        cx.notify();
    }

    fn open_table(
        &mut self,
        connection: &str,
        database: &str,
        schema: &str,
        info: &TableInfo,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(client) = self.store.read(cx).client_for(connection) else {
            return;
        };
        let table = TableRef {
            database: database.to_string(),
            schema: schema.to_string(),
            name: info.name.clone(),
        };
        crate::open_table_tab(&self.workspace, client, table, window, cx);
    }

    fn deploy_connection_context_menu(
        &mut self,
        position: Point<Pixels>,
        connection_name: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let context_menu = ContextMenu::build(window, cx, |menu, _, _| {
            menu.context(self.focus_handle.clone())
                .action("Refresh", Box::new(RefreshConnection))
                .action("Edit Connection…", Box::new(EditConnection))
                .action("New SQL Query", Box::new(NewSqlQuery))
                .separator()
                .action("Remove Connection", Box::new(RemoveConnection))
        });
        window.focus(&context_menu.focus_handle(cx), cx);
        let subscription = cx.subscribe(&context_menu, |this, _, _: &DismissEvent, cx| {
            this.context_menu.take();
            cx.notify();
        });
        self.context_menu = Some((context_menu, position, subscription));
        self.menu_target = Some(connection_name);
        self.menu_target_database = None;
        cx.notify();
    }

    fn deploy_database_context_menu(
        &mut self,
        position: Point<Pixels>,
        connection_name: String,
        database: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let context_menu = ContextMenu::build(window, cx, |menu, _, _| {
            menu.context(self.focus_handle.clone())
                .action("New SQL Query", Box::new(NewSqlQuery))
        });
        window.focus(&context_menu.focus_handle(cx), cx);
        let subscription = cx.subscribe(&context_menu, |this, _, _: &DismissEvent, cx| {
            this.context_menu.take();
            cx.notify();
        });
        self.context_menu = Some((context_menu, position, subscription));
        self.menu_target = Some(connection_name);
        self.menu_target_database = Some(database);
        cx.notify();
    }

    fn refresh_connection(
        &mut self,
        _: &RefreshConnection,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(connection) = self.menu_target.clone() {
            self.store
                .update(cx, |store, cx| store.refresh(&connection, cx));
        }
    }

    fn add_connection(&mut self, _: &AddConnection, window: &mut Window, cx: &mut Context<Self>) {
        self.open_connection_modal(None, window, cx);
    }

    fn edit_connection(&mut self, _: &EditConnection, window: &mut Window, cx: &mut Context<Self>) {
        let Some(connection_name) = self.menu_target.clone() else {
            return;
        };
        let Some(config) = self
            .store
            .read(cx)
            .connections()
            .iter()
            .find(|connection| connection.config.name == connection_name)
            .map(|connection| connection.config.clone())
        else {
            return;
        };
        self.open_connection_modal(Some(config), window, cx);
    }

    /// Opens the add/edit connection modal, seeding it with the current set of
    /// connection names for the duplicate-name check.
    fn open_connection_modal(
        &mut self,
        existing: Option<database_client::ConnectionConfig>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let existing_names: Vec<String> = self
            .store
            .read(cx)
            .connections()
            .iter()
            .map(|connection| connection.config.name.clone())
            .collect();
        let client_factory = default_client_factory(cx);
        let fs = self.fs.clone();
        self.workspace
            .update(cx, |workspace, cx| {
                workspace.toggle_modal(window, cx, |window, cx| {
                    ConnectionModal::new(existing, existing_names, client_factory, fs, window, cx)
                });
            })
            .log_err();
    }

    fn new_sql_query(&mut self, _: &NewSqlQuery, window: &mut Window, cx: &mut Context<Self>) {
        let Some(connection) = self.menu_target.clone() else {
            return;
        };
        let Some(client) = self.store.read(cx).client_for(&connection) else {
            return;
        };
        // A menu opened on a database node targets that database; one opened on
        // the connection uses the connection's starting database.
        let database = self.menu_target_database.clone().or_else(|| {
            self.store
                .read(cx)
                .connections()
                .iter()
                .find(|state| state.config.name == connection)
                .map(|state| state.config.database.clone())
        });
        let Some(database) = database else {
            return;
        };

        let Some(project) = self
            .workspace
            .read_with(cx, |workspace, _| workspace.project().clone())
            .log_err()
        else {
            return;
        };
        let language_registry = project.read(cx).languages().clone();

        crate::open_sql_query_tab(
            &self.workspace,
            client,
            connection,
            database,
            language_registry,
            window,
            cx,
        );
    }

    fn remove_connection(
        &mut self,
        _: &RemoveConnection,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(connection) = self.menu_target.clone() else {
            return;
        };

        let removed = connection.clone();
        update_settings_file(self.fs.clone(), cx, move |settings, _| {
            if let Some(connections) = settings
                .database
                .as_mut()
                .and_then(|database| database.connections.as_mut())
            {
                connections.retain(|entry| entry.name != removed);
            }
        });

        let url = credentials_url(&connection);
        let provider = zed_credentials_provider::global(cx);
        cx.spawn(async move |_, cx| {
            provider.delete_credentials(&url, cx).await.log_err();
        })
        .detach();
    }

    fn render_row(&self, row: &TreeRow, index: usize, cx: &Context<Self>) -> AnyElement {
        match row {
            TreeRow::Connection {
                name,
                status,
                expanded,
            } => self.render_connection_row(index, name, status, *expanded, cx),
            TreeRow::Database {
                connection,
                name,
                expanded,
                loading,
            } => self.render_database_row(index, connection, name, *expanded, *loading, cx),
            TreeRow::Schema {
                connection,
                database,
                name,
                expanded,
                loading,
            } => self.render_schema_row(index, connection, database, name, *expanded, *loading, cx),
            TreeRow::Table {
                connection,
                database,
                schema,
                info,
            } => self.render_table_row(index, connection, database, schema, info, cx),
            TreeRow::Loading { depth } => ListItem::new(index)
                .indent_level(*depth)
                .child(
                    Label::new("Loading…")
                        .color(Color::Muted)
                        .size(LabelSize::Small),
                )
                .into_any_element(),
            TreeRow::Error { depth, message } => ListItem::new(index)
                .indent_level(*depth)
                .child(
                    Label::new(message.clone())
                        .color(Color::Error)
                        .size(LabelSize::Small),
                )
                .tooltip({
                    let message = message.clone();
                    move |_, cx| Tooltip::simple(message.clone(), cx)
                })
                .into_any_element(),
        }
    }

    fn render_connection_row(
        &self,
        index: usize,
        name: &str,
        status: &ConnectionStatus,
        expanded: bool,
        cx: &Context<Self>,
    ) -> AnyElement {
        let node = TreeNodeId::Connection(name.to_string());
        let is_error = matches!(status, ConnectionStatus::Error(_));
        let icon_color = if is_error {
            Color::Error
        } else {
            Color::Default
        };
        let connection_name = name.to_string();

        let mut item = ListItem::new(index)
            .indent_level(0)
            .toggle(Some(expanded))
            .on_toggle(cx.listener({
                let node = node.clone();
                move |this, _, _, cx| this.toggle_node(node.clone(), cx)
            }))
            .child(
                h_flex()
                    .gap_1p5()
                    .child(
                        Icon::new(IconName::DatabaseZap)
                            .color(icon_color)
                            .size(IconSize::Small),
                    )
                    .child(Label::new(connection_name.clone())),
            )
            .on_click(cx.listener(move |this, _, _, cx| this.toggle_node(node.clone(), cx)))
            .on_secondary_mouse_down(cx.listener(
                move |this, event: &MouseDownEvent, window, cx| {
                    cx.stop_propagation();
                    this.deploy_connection_context_menu(
                        event.position,
                        connection_name.clone(),
                        window,
                        cx,
                    );
                },
            ));

        if let ConnectionStatus::Error(message) = status {
            let message = message.clone();
            item = item.tooltip(move |_, cx| Tooltip::simple(message.clone(), cx));
        }

        item.into_any_element()
    }

    fn render_database_row(
        &self,
        index: usize,
        connection: &str,
        name: &str,
        expanded: bool,
        loading: bool,
        cx: &Context<Self>,
    ) -> AnyElement {
        let node = TreeNodeId::Database(connection.to_string(), name.to_string());
        let icon = if expanded {
            IconName::FolderOpen
        } else {
            IconName::Folder
        };
        let connection_name = connection.to_string();
        let database_name = name.to_string();
        ListItem::new(index)
            .indent_level(1)
            .toggle(Some(expanded))
            .on_toggle(cx.listener({
                let node = node.clone();
                move |this, _, _, cx| this.toggle_node(node.clone(), cx)
            }))
            .child(
                h_flex()
                    .gap_1p5()
                    .child(Icon::new(icon).color(Color::Muted).size(IconSize::Small))
                    .child(Label::new(name.to_string()))
                    .when(loading, |this| {
                        this.child(Label::new("…").color(Color::Muted).size(LabelSize::Small))
                    }),
            )
            .on_click(cx.listener(move |this, _, _, cx| this.toggle_node(node.clone(), cx)))
            .on_secondary_mouse_down(cx.listener(
                move |this, event: &MouseDownEvent, window, cx| {
                    cx.stop_propagation();
                    this.deploy_database_context_menu(
                        event.position,
                        connection_name.clone(),
                        database_name.clone(),
                        window,
                        cx,
                    );
                },
            ))
            .into_any_element()
    }

    fn render_schema_row(
        &self,
        index: usize,
        connection: &str,
        database: &str,
        name: &str,
        expanded: bool,
        loading: bool,
        cx: &Context<Self>,
    ) -> AnyElement {
        let node = TreeNodeId::Schema(
            connection.to_string(),
            database.to_string(),
            name.to_string(),
        );
        ListItem::new(index)
            .indent_level(2)
            .toggle(Some(expanded))
            .on_toggle(cx.listener({
                let node = node.clone();
                move |this, _, _, cx| this.toggle_node(node.clone(), cx)
            }))
            .child(
                h_flex()
                    .gap_1p5()
                    .child(
                        Icon::new(IconName::Book)
                            .color(Color::Muted)
                            .size(IconSize::Small),
                    )
                    .child(Label::new(name.to_string()))
                    .when(loading, |this| {
                        this.child(Label::new("…").color(Color::Muted).size(LabelSize::Small))
                    }),
            )
            .on_click(cx.listener(move |this, _, _, cx| this.toggle_node(node.clone(), cx)))
            .into_any_element()
    }

    fn render_table_row(
        &self,
        index: usize,
        connection: &str,
        database: &str,
        schema: &str,
        info: &TableInfo,
        cx: &Context<Self>,
    ) -> AnyElement {
        let icon = if info.is_view {
            IconName::Eye
        } else {
            IconName::FileTree
        };
        let label = if info.is_view {
            format!("{} (view)", info.name)
        } else {
            info.name.clone()
        };
        let connection = connection.to_string();
        let database = database.to_string();
        let schema = schema.to_string();
        let info = info.clone();
        ListItem::new(index)
            .indent_level(3)
            .child(
                h_flex()
                    .gap_1p5()
                    .child(Icon::new(icon).color(Color::Muted).size(IconSize::Small))
                    .child(Label::new(label)),
            )
            .on_click(cx.listener(move |this, _, window, cx| {
                this.open_table(&connection, &database, &schema, &info, window, cx)
            }))
            .into_any_element()
    }
}

impl Render for DatabasePanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let rows = self.build_rows(cx);

        let content = if rows.is_empty() {
            v_flex()
                .size_full()
                .items_center()
                .justify_center()
                .gap_2()
                .child(Label::new("No connections").color(Color::Muted))
                .child(
                    Button::new("add-connection-empty", "Add Connection").on_click(
                        |_, window, cx| {
                            window.dispatch_action(AddConnection.boxed_clone(), cx);
                        },
                    ),
                )
                .into_any_element()
        } else {
            let rows = Arc::new(rows);
            uniform_list(
                "database-tree",
                rows.len(),
                cx.processor(move |this, range: Range<usize>, _window, cx| {
                    range
                        .filter_map(|index| {
                            rows.get(index).map(|row| this.render_row(row, index, cx))
                        })
                        .collect::<Vec<_>>()
                }),
            )
            .size_full()
            .with_sizing_behavior(ListSizingBehavior::Infer)
            .track_scroll(&self.scroll_handle)
            .into_any_element()
        };

        v_flex()
            .key_context("DatabasePanel")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::add_connection))
            .on_action(cx.listener(Self::refresh_connection))
            .on_action(cx.listener(Self::edit_connection))
            .on_action(cx.listener(Self::remove_connection))
            .on_action(cx.listener(Self::new_sql_query))
            .size_full()
            .bg(cx.theme().colors().panel_background)
            .child(
                h_flex()
                    .px_2()
                    .py_1()
                    .justify_between()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(Label::new("Databases"))
                    .child(
                        IconButton::new("add-connection", IconName::Plus)
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::text("Add Connection"))
                            .on_click(|_, window, cx| {
                                window.dispatch_action(AddConnection.boxed_clone(), cx);
                            }),
                    ),
            )
            .child(
                v_flex()
                    .flex_1()
                    .size_full()
                    .overflow_hidden()
                    .child(content),
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

impl Focusable for DatabasePanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for DatabasePanel {}

impl Panel for DatabasePanel {
    fn persistent_name() -> &'static str {
        "DatabasePanel"
    }

    fn panel_key() -> &'static str {
        "DatabasePanel"
    }

    fn position(&self, _: &Window, _: &App) -> DockPosition {
        DockPosition::Left
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, _: DockPosition, _: &mut Window, _: &mut Context<Self>) {}

    fn default_size(&self, _: &Window, _: &App) -> Pixels {
        px(240.)
    }

    fn icon(&self, _: &Window, _: &App) -> Option<ui::IconName> {
        Some(ui::IconName::DatabaseZap)
    }

    fn icon_tooltip(&self, _: &Window, _: &App) -> Option<&'static str> {
        Some("Database Panel")
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        6
    }
}
