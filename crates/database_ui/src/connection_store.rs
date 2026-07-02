use std::sync::Arc;
use std::time::Duration;

use database_client::{ConnectionConfig, DatabaseClient, TableInfo};
use gpui::{App, Context, EventEmitter};
use settings::{Settings, SettingsStore};

use crate::DatabaseSettings;

/// Events emitted by [`ConnectionStore`] so the panel can surface failures to
/// the workspace (e.g. as an error notification).
#[derive(Clone, Debug)]
pub enum ConnectionStoreEvent {
    ConnectionError { name: String, message: String },
}

/// Constructs a [`DatabaseClient`] from a connection config and its password.
///
/// Production wires this to [`crate::default_client_factory`], which builds a
/// `PostgresClient`. Tests inject a factory returning a `FakeDatabaseClient`.
pub type ClientFactory =
    Arc<dyn Fn(&ConnectionConfig, &str) -> Arc<dyn DatabaseClient> + Send + Sync>;

/// The keychain lookup URL for a connection's password.
pub fn credentials_url(connection_name: &str) -> String {
    format!("zed-database://{connection_name}")
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

pub struct DatabaseNode {
    pub name: String,
    /// `None` until schemas have been loaded for this database.
    pub schemas: Option<Vec<SchemaNode>>,
    /// Set while a `load_schemas` request is in flight.
    pub loading: bool,
    /// The last schema-load error, shown inline under the database row.
    pub error: Option<String>,
}

pub struct SchemaNode {
    pub name: String,
    /// `None` until tables have been loaded for this schema.
    pub tables: Option<Vec<TableInfo>>,
    /// Set while a `load_tables` request is in flight.
    pub loading: bool,
    /// The last table-load error, shown inline under the schema row.
    pub error: Option<String>,
}

pub struct ConnectionState {
    pub config: ConnectionConfig,
    pub client: Option<Arc<dyn DatabaseClient>>,
    pub status: ConnectionStatus,
    /// `None` until `connect` has successfully listed the databases.
    pub databases: Option<Vec<DatabaseNode>>,
}

impl ConnectionState {
    fn new(config: ConnectionConfig) -> Self {
        Self {
            config,
            client: None,
            status: ConnectionStatus::Disconnected,
            databases: None,
        }
    }
}

/// Holds the configured connections, their live [`DatabaseClient`]s, and the
/// lazily-loaded metadata tree (databases -> schemas -> tables).
///
/// Every load method spawns work through `gpui_tokio` and writes the result
/// back into `self` followed by `cx.notify()` so the panel re-renders.
pub struct ConnectionStore {
    connections: Vec<ConnectionState>,
    client_factory: ClientFactory,
    _settings_subscription: gpui::Subscription,
}

impl ConnectionStore {
    pub fn new(client_factory: ClientFactory, cx: &mut Context<Self>) -> Self {
        let connections = DatabaseSettings::get_global(cx)
            .connections
            .iter()
            .cloned()
            .map(ConnectionState::new)
            .collect();
        let settings_subscription = cx.observe_global::<SettingsStore>(Self::sync_from_settings);
        Self {
            connections,
            client_factory,
            _settings_subscription: settings_subscription,
        }
    }

    pub fn connections(&self) -> &[ConnectionState] {
        &self.connections
    }

    pub fn client_for(&self, connection_name: &str) -> Option<Arc<dyn DatabaseClient>> {
        self.connections
            .iter()
            .find(|connection| connection.config.name == connection_name)
            .and_then(|connection| connection.client.clone())
    }

    /// Reconciles `self.connections` with the current settings: adds newly
    /// configured connections as `Disconnected`, drops removed ones, and keeps
    /// live state for connections whose config is unchanged.
    fn sync_from_settings(&mut self, cx: &mut Context<Self>) {
        let configs: Vec<ConnectionConfig> = DatabaseSettings::get_global(cx).connections.clone();

        let mut changed = false;

        // Drop connections that no longer exist in settings.
        let before = self.connections.len();
        self.connections.retain(|connection| {
            configs
                .iter()
                .any(|config| config.name == connection.config.name)
        });
        changed |= self.connections.len() != before;

        for config in &configs {
            match self
                .connections
                .iter_mut()
                .find(|connection| connection.config.name == config.name)
            {
                Some(existing) => {
                    // A config edit (host/port/etc.) invalidates the live tree.
                    if &existing.config != config {
                        existing.config = config.clone();
                        existing.client = None;
                        existing.status = ConnectionStatus::Disconnected;
                        existing.databases = None;
                        changed = true;
                    }
                }
                None => {
                    self.connections.push(ConnectionState::new(config.clone()));
                    changed = true;
                }
            }
        }

        if changed {
            cx.notify();
        }
    }

    fn connection_index(&self, connection_name: &str) -> Option<usize> {
        self.connections
            .iter()
            .position(|connection| connection.config.name == connection_name)
    }

    /// Production connect path: reads the password from the system keychain on
    /// the foreground thread and then delegates to [`Self::connect_with_password`].
    ///
    /// A missing keychain entry surfaces as [`ConnectionStatus::Error`].
    pub fn connect(&mut self, connection_name: &str, cx: &mut Context<Self>) {
        let Some(index) = self.connection_index(connection_name) else {
            return;
        };
        if self.connections[index].status == ConnectionStatus::Connecting {
            return;
        }
        self.connections[index].status = ConnectionStatus::Connecting;
        cx.notify();

        let connection_name = connection_name.to_string();
        let url = credentials_url(&connection_name);
        let provider = zed_credentials_provider::global(cx);

        cx.spawn(async move |this, cx| {
            let read = provider.read_credentials(&url, cx).await;
            let password = match read {
                Ok(Some((_user, password_bytes))) => match String::from_utf8(password_bytes) {
                    Ok(password) => password,
                    Err(error) => {
                        this.update(cx, |this, cx| {
                            this.set_error(
                                &connection_name,
                                format!("saved password is not valid UTF-8: {error}"),
                                cx,
                            );
                        })
                        .ok();
                        return;
                    }
                },
                Ok(None) => {
                    this.update(cx, |this, cx| {
                        this.set_error(
                            &connection_name,
                            "no saved password — edit the connection".to_string(),
                            cx,
                        );
                    })
                    .ok();
                    return;
                }
                Err(error) => {
                    this.update(cx, |this, cx| {
                        this.set_error(
                            &connection_name,
                            format!("failed to read keychain: {error}"),
                            cx,
                        );
                    })
                    .ok();
                    return;
                }
            };

            this.update(cx, |this, cx| {
                this.connect_with_password(&connection_name, password, cx);
            })
            .ok();
        })
        .detach();
    }

    /// Shared connect core (and the entry point tests use, since it does not
    /// touch the keychain): builds a client via the factory, verifies the
    /// connection, and lists the databases.
    pub fn connect_with_password(
        &mut self,
        connection_name: &str,
        password: String,
        cx: &mut Context<Self>,
    ) {
        let Some(index) = self.connection_index(connection_name) else {
            return;
        };

        let client = (self.client_factory)(&self.connections[index].config, &password);
        self.connections[index].client = Some(client.clone());
        self.connections[index].status = ConnectionStatus::Connecting;
        cx.notify();

        let connection_name = connection_name.to_string();
        let task = gpui_tokio::Tokio::spawn_result(cx, async move {
            client.test_connection().await?;
            client.list_databases().await
        });

        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| match result {
                Ok(database_names) => {
                    if let Some(connection) = this.connection_mut(&connection_name) {
                        connection.status = ConnectionStatus::Connected;
                        connection.databases = Some(
                            database_names
                                .into_iter()
                                .map(|name| DatabaseNode {
                                    name,
                                    schemas: None,
                                    loading: false,
                                    error: None,
                                })
                                .collect(),
                        );
                    }
                    cx.notify();
                }
                Err(error) => {
                    this.set_error(&connection_name, error.to_string(), cx);
                }
            })
            .ok();
        })
        .detach();
    }

    pub fn load_schemas(&mut self, connection_name: &str, database: &str, cx: &mut Context<Self>) {
        let Some(client) = self.client_for(connection_name) else {
            return;
        };
        let Some(database_node) = self.database_mut(connection_name, database) else {
            return;
        };
        if database_node.loading || database_node.schemas.is_some() {
            return;
        }
        database_node.loading = true;
        database_node.error = None;
        cx.notify();

        let connection_name = connection_name.to_string();
        let database = database.to_string();
        let task = gpui_tokio::Tokio::spawn_result(cx, {
            let database = database.clone();
            async move { client.list_schemas(&database).await }
        });

        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| {
                if let Some(database_node) = this.database_mut(&connection_name, &database) {
                    database_node.loading = false;
                    match result {
                        Ok(schema_names) => {
                            database_node.error = None;
                            database_node.schemas = Some(
                                schema_names
                                    .into_iter()
                                    .map(|name| SchemaNode {
                                        name,
                                        tables: None,
                                        loading: false,
                                        error: None,
                                    })
                                    .collect(),
                            );
                        }
                        Err(error) => {
                            database_node.error = Some(error.to_string());
                        }
                    }
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    pub fn load_tables(
        &mut self,
        connection_name: &str,
        database: &str,
        schema: &str,
        cx: &mut Context<Self>,
    ) {
        let Some(client) = self.client_for(connection_name) else {
            return;
        };
        let Some(schema_node) = self.schema_mut(connection_name, database, schema) else {
            return;
        };
        if schema_node.loading || schema_node.tables.is_some() {
            return;
        }
        schema_node.loading = true;
        schema_node.error = None;
        cx.notify();

        let connection_name = connection_name.to_string();
        let database = database.to_string();
        let schema = schema.to_string();
        let task = gpui_tokio::Tokio::spawn_result(cx, {
            let database = database.clone();
            let schema = schema.clone();
            async move { client.list_tables(&database, &schema).await }
        });

        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| {
                if let Some(schema_node) = this.schema_mut(&connection_name, &database, &schema) {
                    schema_node.loading = false;
                    match result {
                        Ok(tables) => {
                            schema_node.error = None;
                            schema_node.tables = Some(tables);
                        }
                        Err(error) => {
                            schema_node.error = Some(error.to_string());
                        }
                    }
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// Discards the connection's cached tree and reconnects from scratch.
    pub fn refresh(&mut self, connection_name: &str, cx: &mut Context<Self>) {
        if let Some(connection) = self.connection_mut(connection_name) {
            connection.client = None;
            connection.databases = None;
            connection.status = ConnectionStatus::Disconnected;
        } else {
            return;
        }
        cx.notify();
        self.connect(connection_name, cx);
    }

    fn set_error(&mut self, connection_name: &str, message: String, cx: &mut Context<Self>) {
        if let Some(connection) = self.connection_mut(connection_name) {
            connection.client = None;
            connection.status = ConnectionStatus::Error(message.clone());
        }
        cx.emit(ConnectionStoreEvent::ConnectionError {
            name: connection_name.to_string(),
            message,
        });
        cx.notify();
    }

    fn connection_mut(&mut self, connection_name: &str) -> Option<&mut ConnectionState> {
        self.connections
            .iter_mut()
            .find(|connection| connection.config.name == connection_name)
    }

    fn database_mut(&mut self, connection_name: &str, database: &str) -> Option<&mut DatabaseNode> {
        self.connection_mut(connection_name)?
            .databases
            .as_mut()?
            .iter_mut()
            .find(|node| node.name == database)
    }

    fn schema_mut(
        &mut self,
        connection_name: &str,
        database: &str,
        schema: &str,
    ) -> Option<&mut SchemaNode> {
        self.database_mut(connection_name, database)?
            .schemas
            .as_mut()?
            .iter_mut()
            .find(|node| node.name == schema)
    }
}

impl EventEmitter<ConnectionStoreEvent> for ConnectionStore {}

/// The default production client factory: builds a `PostgresClient` configured
/// with the current query timeout from [`DatabaseSettings`].
pub fn default_client_factory(cx: &App) -> ClientFactory {
    let timeout = Duration::from_secs(DatabaseSettings::get_global(cx).query_timeout_seconds);
    Arc::new(move |config: &ConnectionConfig, password: &str| {
        Arc::new(database_client::postgres::PostgresClient::new(
            config.clone(),
            password.to_string(),
            timeout,
            database_client::SessionMode::ReadWrite,
        )) as Arc<dyn DatabaseClient>
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use database_client::fake::FakeDatabaseClient;
    use gpui::{AppContext as _, BorrowAppContext as _, TestAppContext};
    use settings::SettingsStore;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            gpui_tokio::init(cx);
            crate::init(cx);
        });
    }

    fn set_one_connection(cx: &mut TestAppContext) {
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.database.get_or_insert_default().connections =
                        Some(vec![settings::DatabaseConnectionContent {
                            name: "local".into(),
                            host: "127.0.0.1".into(),
                            port: 5432,
                            database: "postgres".into(),
                            user: "postgres".into(),
                        }]);
                });
            });
        });
    }

    /// Drives the deterministic scheduler while giving the real tokio runtime a
    /// chance to complete cross-thread work, until `condition` holds or a bound
    /// is reached. Requires `cx.executor().allow_parking()`.
    async fn wait_until(cx: &mut TestAppContext, condition: impl Fn(&mut TestAppContext) -> bool) {
        for _ in 0..200 {
            cx.run_until_parked();
            if condition(cx) {
                return;
            }
            cx.executor()
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
    async fn connect_populates_databases_from_client(cx: &mut TestAppContext) {
        init_test(cx);
        // The client runs on the real tokio runtime, so let `run_until_parked`
        // block until the cross-thread result arrives.
        cx.executor().allow_parking();
        set_one_connection(cx);

        let fake = Arc::new(FakeDatabaseClient::new());
        let factory: ClientFactory = Arc::new(move |_, _| fake.clone() as Arc<dyn DatabaseClient>);
        let store = cx.new(|cx| ConnectionStore::new(factory, cx));
        store.update(cx, |store, cx| {
            store.connect_with_password("local", "pw".into(), cx)
        });
        wait_until(cx, |cx| {
            store.read_with(cx, |store, _| {
                store.connections()[0].status == ConnectionStatus::Connected
            })
        })
        .await;

        store.read_with(cx, |store, _| {
            let connection = &store.connections()[0];
            assert_eq!(connection.status, ConnectionStatus::Connected);
            assert_eq!(
                connection
                    .databases
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|database| database.name.as_str())
                    .collect::<Vec<_>>(),
                vec!["app", "postgres"]
            );
        });
    }

    #[gpui::test]
    async fn connect_error_sets_error_status(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        set_one_connection(cx);

        let fake = Arc::new(FakeDatabaseClient::with_error("connection refused"));
        let factory: ClientFactory = Arc::new(move |_, _| fake.clone() as Arc<dyn DatabaseClient>);
        let store = cx.new(|cx| ConnectionStore::new(factory, cx));
        store.update(cx, |store, cx| {
            store.connect_with_password("local", "pw".into(), cx)
        });
        wait_until(cx, |cx| {
            store.read_with(cx, |store, _| {
                matches!(store.connections()[0].status, ConnectionStatus::Error(_))
            })
        })
        .await;

        store.read_with(cx, |store, _| {
            let connection = &store.connections()[0];
            assert!(
                matches!(&connection.status, ConnectionStatus::Error(message) if message.contains("connection refused")),
                "unexpected status: {:?}",
                connection.status
            );
            assert!(connection.databases.is_none());
        });
    }

    #[gpui::test]
    async fn load_schemas_and_tables_populate_lazily(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        set_one_connection(cx);

        let fake = Arc::new(FakeDatabaseClient::new());
        let factory: ClientFactory = Arc::new(move |_, _| fake.clone() as Arc<dyn DatabaseClient>);
        let store = cx.new(|cx| ConnectionStore::new(factory, cx));
        store.update(cx, |store, cx| {
            store.connect_with_password("local", "pw".into(), cx)
        });
        wait_until(cx, |cx| {
            store.read_with(cx, |store, _| {
                store.connections()[0].status == ConnectionStatus::Connected
            })
        })
        .await;

        store.update(cx, |store, cx| store.load_schemas("local", "app", cx));
        wait_until(cx, |cx| {
            store.read_with(cx, |store, _| {
                store.connections()[0].databases.as_ref().unwrap()[0]
                    .schemas
                    .is_some()
            })
        })
        .await;

        store.read_with(cx, |store, _| {
            let database = &store.connections()[0].databases.as_ref().unwrap()[0];
            assert_eq!(
                database
                    .schemas
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|schema| schema.name.as_str())
                    .collect::<Vec<_>>(),
                vec!["public"]
            );
        });

        store.update(cx, |store, cx| {
            store.load_tables("local", "app", "public", cx)
        });
        wait_until(cx, |cx| {
            store.read_with(cx, |store, _| {
                store.connections()[0].databases.as_ref().unwrap()[0]
                    .schemas
                    .as_ref()
                    .unwrap()[0]
                    .tables
                    .is_some()
            })
        })
        .await;

        store.read_with(cx, |store, _| {
            let schema = &store.connections()[0].databases.as_ref().unwrap()[0]
                .schemas
                .as_ref()
                .unwrap()[0];
            let tables = schema.tables.as_ref().unwrap();
            assert_eq!(tables.len(), 2);
            assert_eq!(tables[0].name, "users");
            assert!(!tables[0].is_view);
            assert_eq!(tables[1].name, "orders_view");
            assert!(tables[1].is_view);
        });
    }

    #[gpui::test]
    async fn settings_changes_sync_connections(cx: &mut TestAppContext) {
        init_test(cx);
        set_one_connection(cx);

        let fake = Arc::new(FakeDatabaseClient::new());
        let factory: ClientFactory = Arc::new(move |_, _| fake.clone() as Arc<dyn DatabaseClient>);
        let store = cx.new(|cx| ConnectionStore::new(factory, cx));
        store.read_with(cx, |store, _| {
            assert_eq!(store.connections().len(), 1);
        });

        // Add a second connection through settings.
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|settings_store, cx| {
                settings_store.update_user_settings(cx, |settings| {
                    settings.database.get_or_insert_default().connections = Some(vec![
                        settings::DatabaseConnectionContent {
                            name: "local".into(),
                            host: "127.0.0.1".into(),
                            port: 5432,
                            database: "postgres".into(),
                            user: "postgres".into(),
                        },
                        settings::DatabaseConnectionContent {
                            name: "prod".into(),
                            host: "db.example.com".into(),
                            port: 5432,
                            database: "postgres".into(),
                            user: "readonly".into(),
                        },
                    ]);
                });
            });
        });
        cx.run_until_parked();

        store.read_with(cx, |store, _| {
            assert_eq!(store.connections().len(), 2);
            assert_eq!(store.connections()[1].config.name, "prod");
        });
    }
}
