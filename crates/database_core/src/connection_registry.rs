use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, RwLock};

use crate::connection::{ConnectionConfig, DatabaseConnection};
use crate::schema::DatabaseSchema;

pub struct RegisteredConnection {
    pub config: ConnectionConfig,
    pub connection: Arc<dyn DatabaseConnection>,
    pub schema: Option<DatabaseSchema>,
}

static CONNECTION_REGISTRY: LazyLock<RwLock<HashMap<String, RegisteredConnection>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub fn register_connection(
    name: String,
    config: ConnectionConfig,
    connection: Arc<dyn DatabaseConnection>,
    schema: Option<DatabaseSchema>,
) {
    if let Ok(mut registry) = CONNECTION_REGISTRY.write() {
        registry.insert(
            name,
            RegisteredConnection {
                config,
                connection,
                schema,
            },
        );
    }
}

pub fn unregister_connection(name: &str) {
    if let Ok(mut registry) = CONNECTION_REGISTRY.write() {
        registry.remove(name);
    }
}

pub fn get_connection(
    name: &str,
) -> Option<(Arc<dyn DatabaseConnection>, ConnectionConfig, Option<DatabaseSchema>)> {
    let registry = CONNECTION_REGISTRY.read().ok()?;
    let entry = registry.get(name)?;
    Some((
        entry.connection.clone(),
        entry.config.clone(),
        entry.schema.clone(),
    ))
}

pub fn list_connections() -> Vec<(String, ConnectionConfig)> {
    let registry = match CONNECTION_REGISTRY.read() {
        Ok(r) => r,
        Err(_) => return Vec::new(),
    };
    registry
        .iter()
        .map(|(name, entry)| (name.clone(), entry.config.clone()))
        .collect()
}

pub fn update_connection_schema(name: &str, schema: DatabaseSchema) {
    if let Ok(mut registry) = CONNECTION_REGISTRY.write() {
        if let Some(entry) = registry.get_mut(name) {
            entry.schema = Some(schema);
        }
    }
}

pub fn clear_all_connections() {
    if let Ok(mut registry) = CONNECTION_REGISTRY.write() {
        registry.clear();
    }
}

pub fn connection_count() -> usize {
    CONNECTION_REGISTRY
        .read()
        .map(|registry| registry.len())
        .unwrap_or(0)
}

static MCP_SOCKET_PATH: LazyLock<RwLock<Option<PathBuf>>> =
    LazyLock::new(|| RwLock::new(None));

pub fn set_mcp_socket_path(path: Option<PathBuf>) {
    if let Ok(mut guard) = MCP_SOCKET_PATH.write() {
        *guard = path;
    }
}

pub fn get_mcp_socket_path() -> Option<PathBuf> {
    MCP_SOCKET_PATH.read().ok()?.clone()
}
