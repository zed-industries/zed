#![deny(unsafe_code)]
#![cfg_attr(not(test), deny(clippy::unwrap_used))]

mod connection;
mod connection_registry;
mod errors;
mod export;
mod query_history;
mod query_result;
mod schema;
mod tool_core;

pub use connection::{
    ConnectionConfig, DatabaseConnection, DatabaseDriver, DatabaseType, DriverRegistry,
    FilterCondition, FilterOp, FilteredQuery, MysqlConnection, MysqlDriver, PostgresConnection,
    PostgresDriver, ReadOnlyGuard, SecurePassword, SqliteConnection, SqliteDriver, SshAuthMethod,
    SshTunnel, SshTunnelConfig, SslConfig, SslMode, StatementType, build_filtered_query,
    classify_statement, close_tunnel, create_connection, default_registry,
    escape_sqlite_identifier, establish_ssh_tunnel, quote_identifier,
};
pub use connection_registry::{
    clear_all_connections, connection_count, get_connection, get_mcp_socket_path, list_connections,
    register_connection, set_mcp_socket_path, unregister_connection, update_connection_schema,
};
pub use errors::DatabaseError;
pub use export::{
    generate_csv, generate_ddl_from_schema, generate_html, generate_json, generate_markdown,
    generate_sql_ddl_dml, generate_sql_insert, generate_table_ddl, generate_tsv, generate_xlsx,
};
pub use query_history::{NavigateResult, QueryHistory};
pub use query_result::{CellValue, QueryResult};
pub use schema::{
    ColumnInfo, DatabaseSchema, ForeignKeyInfo, IndexInfo, IntrospectionLevel, SchemaCache,
    TableInfo, TableKind,
};
pub use tool_core::{
    build_explain_sql, describe_object_core, execute_query_core, explain_query_core,
    format_connection_not_found, get_schema_core, list_objects_core, modify_data_core,
};
