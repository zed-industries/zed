use std::sync::Arc;
use std::time::Duration;

use credentials_provider::CredentialsProvider;
use gpui::{App, AppContext as _, Context, EventEmitter, Task};
use util::ResultExt as _;

use database_core::{
    ConnectionConfig, DatabaseConnection, DatabaseError, DatabaseSchema, QueryResult,
    classify_statement, close_tunnel, create_connection, register_connection,
    unregister_connection, update_connection_schema,
};

pub struct ConnectionEntry {
    pub config: ConnectionConfig,
    pub connection: Option<Arc<dyn DatabaseConnection>>,
    pub schema: Option<DatabaseSchema>,
    pub error: Option<String>,
}

pub enum ConnectionManagerEvent {
    ConnectionAdded { index: usize },
    ConnectionRemoved { index: usize },
    ActiveConnectionChanged,
    SchemaUpdated { index: usize },
    ConnectionFailed { index: usize, error: String },
    ConnectionLost { index: usize },
    Reconnected { index: usize },
}

pub struct ConnectionManager {
    connections: Vec<ConnectionEntry>,
    active_connection: Option<usize>,
    connect_task: Task<()>,
    watchdog_task: Task<()>,
}

impl EventEmitter<ConnectionManagerEvent> for ConnectionManager {}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: Vec::new(),
            active_connection: None,
            connect_task: Task::ready(()),
            watchdog_task: Task::ready(()),
        }
    }

    pub fn connections(&self) -> &[ConnectionEntry] {
        &self.connections
    }

    pub fn active_connection(&self) -> Option<usize> {
        self.active_connection
    }

    pub fn set_active_connection(&mut self, index: Option<usize>, cx: &mut Context<Self>) {
        self.active_connection = index;
        cx.emit(ConnectionManagerEvent::ActiveConnectionChanged);
        cx.notify();
    }

    pub fn active_entry(&self) -> Option<&ConnectionEntry> {
        self.active_connection
            .and_then(|i| self.connections.get(i))
    }

    pub fn active_schema(&self) -> Option<&DatabaseSchema> {
        self.active_entry().and_then(|e| e.schema.as_ref())
    }

    pub fn active_db_connection(&self) -> Option<Arc<dyn DatabaseConnection>> {
        self.active_entry()
            .and_then(|e| e.connection.clone())
    }

    pub fn connection_configs(&self) -> Vec<ConnectionConfig> {
        self.connections.iter().map(|e| e.config.clone()).collect()
    }

    pub fn add_connection(&mut self, config: ConnectionConfig, cx: &mut Context<Self>) -> usize {
        let credential_key = config.credential_key();
        let password = config.password.clone();
        let username = config.user.clone().unwrap_or_default();

        self.connections.push(ConnectionEntry {
            config: config.clone(),
            connection: None,
            schema: None,
            error: None,
        });
        let index = self.connections.len() - 1;

        cx.emit(ConnectionManagerEvent::ConnectionAdded { index });

        let credentials_provider = <dyn CredentialsProvider>::global(cx);

        self.connect_task = cx.spawn(async move |this, cx| {
            if let Some(password) = &password {
                credentials_provider
                    .write_credentials(&credential_key, &username, password.as_bytes(), cx)
                    .await
                    .log_err();
            }

            let result = cx
                .background_spawn(async move {
                    let connection = create_connection(&config)?;
                    let schema = connection.fetch_schema()?;
                    Ok::<_, anyhow::Error>((connection, schema))
                })
                .await;

            this.update(cx, |this, cx| {
                match result {
                    Ok((connection, schema)) => {
                        if let Some(entry) = this.connections.get_mut(index) {
                            register_connection(
                                entry.config.name.clone(),
                                entry.config.clone(),
                                connection.clone(),
                                Some(schema.clone()),
                            );
                            entry.connection = Some(connection);
                            entry.schema = Some(schema);
                            entry.error = None;
                        }
                        cx.emit(ConnectionManagerEvent::SchemaUpdated { index });
                    }
                    Err(error) => {
                        let error_msg = format!("{:#}", error);
                        log::error!("database_viewer: connection failed: {}", error_msg);
                        if let Some(entry) = this.connections.get_mut(index) {
                            entry.error = Some(error_msg.clone());
                        }
                        cx.emit(ConnectionManagerEvent::ConnectionFailed {
                            index,
                            error: error_msg,
                        });
                    }
                }
                cx.notify();
            })
            .log_err();
        });

        index
    }

    pub fn remove_connection(&mut self, index: usize, cx: &mut Context<Self>) {
        if index >= self.connections.len() {
            return;
        }

        let credential_key = self.connections[index].config.credential_key();
        let connection_id = self.connections[index].config.id.clone();
        let name = self.connections[index].config.name.clone();
        unregister_connection(&name);
        close_tunnel(&connection_id);

        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        cx.spawn(async move |_this, cx| {
            credentials_provider
                .delete_credentials(&credential_key, cx)
                .await
                .log_err();
        })
        .detach();

        self.connections.remove(index);

        if self.connections.is_empty() {
            self.active_connection = None;
        } else if let Some(active) = self.active_connection {
            if active == index {
                self.active_connection =
                    Some(index.min(self.connections.len().saturating_sub(1)));
            } else if active > index {
                self.active_connection = Some(active - 1);
            }
        }

        cx.emit(ConnectionManagerEvent::ConnectionRemoved { index });
        cx.notify();
    }

    pub fn refresh_schema(&mut self, cx: &mut Context<Self>) {
        let Some(active_index) = self.active_connection else {
            return;
        };
        let Some(entry) = self.connections.get(active_index) else {
            return;
        };
        let Some(connection) = entry.connection.clone() else {
            return;
        };

        self.connect_task = cx.spawn(async move |this, cx| {
            let result = cx
                .background_spawn({
                    let connection = connection.clone();
                    async move { connection.fetch_schema() }
                })
                .await;

            this.update(cx, |this, cx| {
                match result {
                    Ok(schema) => {
                        if let Some(entry) = this.connections.get_mut(active_index) {
                            update_connection_schema(&entry.config.name, schema.clone());
                            entry.schema = Some(schema);
                            entry.error = None;
                        }
                        cx.emit(ConnectionManagerEvent::SchemaUpdated {
                            index: active_index,
                        });
                    }
                    Err(error) => {
                        if let Some(entry) = this.connections.get_mut(active_index) {
                            entry.error = Some(format!("{:#}", error));
                        }
                    }
                }
                cx.notify();
            })
            .log_err();
        });

        cx.notify();
    }

    pub fn execute_query(
        &self,
        sql: String,
        rows_per_page: usize,
        offset: usize,
        cx: &App,
    ) -> Task<anyhow::Result<QueryResult>> {
        let Some(entry) = self.active_entry() else {
            return Task::ready(Err(anyhow::anyhow!("No active connection")));
        };
        let Some(connection) = entry.connection.clone() else {
            return Task::ready(Err(anyhow::anyhow!("No active connection")));
        };

        if entry.config.read_only {
            let statement_type = classify_statement(&sql);
            if !statement_type.is_read_only() {
                return Task::ready(Err(DatabaseError::ReadOnlyViolation {
                    statement_type: format!("{:?}", statement_type),
                }
                .into()));
            }
        }

        cx.background_spawn(async move {
            connection.execute_query_paged(&sql, rows_per_page, offset)
        })
    }

    pub fn execute_raw_query(
        &self,
        sql: String,
        cx: &App,
    ) -> Task<anyhow::Result<QueryResult>> {
        let Some(entry) = self.active_entry() else {
            return Task::ready(Err(anyhow::anyhow!("No active connection")));
        };
        let Some(connection) = entry.connection.clone() else {
            return Task::ready(Err(anyhow::anyhow!("No active connection")));
        };

        if entry.config.read_only {
            let statement_type = classify_statement(&sql);
            if !statement_type.is_read_only() {
                return Task::ready(Err(DatabaseError::ReadOnlyViolation {
                    statement_type: format!("{:?}", statement_type),
                }
                .into()));
            }
        }

        cx.background_spawn(async move { connection.execute_query(&sql) })
    }

    pub fn interrupt_active(&self) {
        if let Some(connection) = self.active_db_connection() {
            connection.interrupt();
        }
    }

    pub fn is_connecting(&self) -> bool {
        self.connections
            .iter()
            .any(|e| e.connection.is_none() && e.error.is_none())
    }

    pub fn restore_connections(
        &mut self,
        configs: Vec<ConnectionConfig>,
        active_connection: Option<usize>,
        cx: &mut Context<Self>,
    ) {
        let credentials_provider = <dyn CredentialsProvider>::global(cx);

        for config in configs {
            let credential_key = config.credential_key();
            let credentials_provider = credentials_provider.clone();

            self.connections.push(ConnectionEntry {
                config: config.clone(),
                connection: None,
                schema: None,
                error: None,
            });
            let index = self.connections.len() - 1;

            cx.emit(ConnectionManagerEvent::ConnectionAdded { index });

            self.connect_task = cx.spawn(async move |this, cx| {
                let mut config = config;

                if let Ok(Some((_username, password_bytes))) =
                    credentials_provider.read_credentials(&credential_key, cx).await
                {
                    config.password = Some(String::from_utf8_lossy(&password_bytes).to_string());
                }

                let result = cx
                    .background_spawn(async move {
                        let connection = create_connection(&config)?;
                        let schema = connection.fetch_schema()?;
                        Ok::<_, anyhow::Error>((connection, schema))
                    })
                    .await;

                this.update(cx, |this, cx| {
                    match result {
                        Ok((connection, schema)) => {
                            if let Some(entry) = this.connections.get_mut(index) {
                                register_connection(
                                    entry.config.name.clone(),
                                    entry.config.clone(),
                                    connection.clone(),
                                    Some(schema.clone()),
                                );
                                entry.connection = Some(connection);
                                entry.schema = Some(schema);
                                entry.error = None;
                            }
                            cx.emit(ConnectionManagerEvent::SchemaUpdated { index });
                        }
                        Err(error) => {
                            let error_msg = format!("{:#}", error);
                            log::error!("database_viewer: connection restore failed: {}", error_msg);
                            if let Some(entry) = this.connections.get_mut(index) {
                                entry.error = Some(error_msg.clone());
                            }
                            cx.emit(ConnectionManagerEvent::ConnectionFailed {
                                index,
                                error: error_msg,
                            });
                        }
                    }
                    cx.notify();
                })
                .log_err();
            });
        }

        self.active_connection = active_connection;
    }

    pub fn start_watchdog(&mut self, cx: &mut Context<Self>) {
        self.watchdog_task = cx.spawn(async move |this, cx| {
            const PING_INTERVAL: Duration = Duration::from_secs(30);
            const MAX_BACKOFF: Duration = Duration::from_secs(30);

            loop {
                cx.background_executor()
                    .timer(PING_INTERVAL)
                    .await;

                let ping_results = this
                    .update(cx, |this, cx| {
                        let mut tasks = Vec::new();
                        for (index, entry) in this.connections.iter().enumerate() {
                            if let Some(connection) = &entry.connection {
                                let connection = connection.clone();
                                let task = cx.background_spawn(async move {
                                    let result = connection.execute_query("SELECT 1");
                                    (index, result.is_ok())
                                });
                                tasks.push(task);
                            }
                        }
                        tasks
                    })
                    .ok();

                let Some(tasks) = ping_results else {
                    break;
                };

                for task in tasks {
                    let (index, alive) = task.await;
                    if !alive {
                        log::warn!("database_viewer: connection {} lost, attempting reconnect", index);

                        let reconnect_info = this
                            .update(cx, |this, cx| {
                                cx.emit(ConnectionManagerEvent::ConnectionLost { index });
                                cx.notify();

                                let entry = this.connections.get(index)?;
                                let config = entry.config.clone();
                                let credential_key = config.credential_key();
                                let credentials_provider = <dyn CredentialsProvider>::global(cx);

                                Some((config, credential_key, credentials_provider))
                            })
                            .ok()
                            .flatten();

                        let Some((config, credential_key, credentials_provider)) = reconnect_info else {
                            continue;
                        };

                        let mut backoff = Duration::from_secs(1);
                        loop {
                            let mut reconnect_config = config.clone();
                            if let Ok(Some((_username, password_bytes))) =
                                credentials_provider.read_credentials(&credential_key, cx).await
                            {
                                reconnect_config.password = Some(String::from_utf8_lossy(&password_bytes).to_string());
                            }

                            let result = cx
                                .background_spawn({
                                    let config = reconnect_config;
                                    async move {
                                        let connection = create_connection(&config)?;
                                        let schema = connection.fetch_schema()?;
                                        Ok::<_, anyhow::Error>((connection, schema))
                                    }
                                })
                                .await;

                            match result {
                                Ok((connection, schema)) => {
                                    this.update(cx, |this, cx| {
                                        if let Some(entry) = this.connections.get_mut(index) {
                                            register_connection(
                                                entry.config.name.clone(),
                                                entry.config.clone(),
                                                connection.clone(),
                                                Some(schema.clone()),
                                            );
                                            entry.connection = Some(connection);
                                            entry.schema = Some(schema);
                                            entry.error = None;
                                        }
                                        cx.emit(ConnectionManagerEvent::Reconnected { index });
                                        cx.notify();
                                    })
                                    .log_err();
                                    log::info!("database_viewer: connection {} reconnected", index);
                                    break;
                                }
                                Err(error) => {
                                    log::warn!(
                                        "database_viewer: reconnect attempt failed for connection {}: {:#}",
                                        index, error
                                    );
                                    cx.background_executor()
                                        .timer(backoff)
                                        .await;
                                    backoff = (backoff * 2).min(MAX_BACKOFF);
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}
