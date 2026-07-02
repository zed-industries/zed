use database_client::ConnectionConfig;
use settings::{RegisterSetting, Settings};

#[derive(Debug, Clone, PartialEq, RegisterSetting)]
pub struct DatabaseSettings {
    pub page_size: u32,
    pub query_timeout_seconds: u64,
    pub mcp_max_rows: u32,
    pub connections: Vec<ConnectionConfig>,
}

impl Settings for DatabaseSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let database = content.database.clone().unwrap();
        Self {
            page_size: database.page_size.unwrap(),
            query_timeout_seconds: database.query_timeout_seconds.unwrap(),
            mcp_max_rows: database.mcp_max_rows.unwrap(),
            connections: database
                .connections
                .unwrap_or_default()
                .into_iter()
                .map(|connection| ConnectionConfig {
                    name: connection.name,
                    host: connection.host,
                    port: connection.port,
                    database: connection.database,
                    user: connection.user,
                })
                .collect(),
        }
    }
}
