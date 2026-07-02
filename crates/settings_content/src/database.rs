use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};

#[with_fallible_options]
#[derive(Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug)]
pub struct DatabaseSettingsContent {
    /// Number of rows per page in the database table data view.
    ///
    /// Default: 100
    pub page_size: Option<u32>,
    /// Statement timeout for database queries, in seconds.
    ///
    /// Default: 30
    pub query_timeout_seconds: Option<u64>,
    /// Maximum number of rows the MCP run_query tool returns.
    ///
    /// Default: 200
    pub mcp_max_rows: Option<u32>,
    /// Configured database connections. Passwords are stored in the system keychain.
    ///
    /// Default: []
    pub connections: Option<Vec<DatabaseConnectionContent>>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom, Debug)]
pub struct DatabaseConnectionContent {
    /// Unique display name of the connection.
    pub name: String,
    /// Server host name or IP address.
    pub host: String,
    /// Server port.
    pub port: u16,
    /// Initial database to connect to.
    pub database: String,
    /// User name.
    pub user: String,
}
