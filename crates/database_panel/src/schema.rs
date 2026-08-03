//! Read-only schema introspection for the database panel.
//!
//! All functions here run on the Tokio runtime (via [`gpui_tokio::Tokio`])
//! because sqlx's drivers require it. A fresh connection is opened per request
//! and dropped afterwards, so the panel never holds long-lived connections to
//! the inspected databases.

use anyhow::{Context as _, Result};
use collections::{HashMap, HashSet};
use sqlx::{
    ConnectOptions as _, Row as _,
    mysql::{MySqlConnectOptions, MySqlConnection},
    sqlite::{SqliteConnectOptions, SqliteConnection},
};
use std::{future::Future, path::PathBuf, time::Duration};

const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const QUERY_TIMEOUT: Duration = Duration::from_secs(30);

pub const MARIADB_DEFAULT_PORT: u16 = 3306;
const MARIADB_SYSTEM_SCHEMAS: &[&str] =
    &["information_schema", "mysql", "performance_schema", "sys"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionConfig {
    Sqlite {
        path: String,
    },
    MariaDb {
        host: String,
        port: u16,
        username: String,
        password: Option<String>,
        /// Databases to show; empty means all non-system databases.
        databases: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabaseInfo {
    pub name: String,
    pub charset: Option<String>,
    pub collation: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableType {
    Table,
    View,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableInfo {
    pub name: String,
    pub table_type: TableType,
    pub engine: Option<String>,
    pub row_count: Option<u64>,
    pub size_bytes: Option<u64>,
    pub collation: Option<String>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnInfo {
    pub name: String,
    /// The full column type as reported by the database, e.g. `varchar(255)`.
    pub data_type: String,
    pub nullable: bool,
    pub default: Option<String>,
    pub primary_key: bool,
    pub unique: bool,
    pub auto_increment: bool,
    /// The referenced `table.column` when this column is a foreign key.
    pub foreign_key: Option<String>,
    pub charset: Option<String>,
    pub collation: Option<String>,
    pub comment: Option<String>,
}

pub async fn list_databases(config: ConnectionConfig) -> Result<Vec<DatabaseInfo>> {
    with_timeout(QUERY_TIMEOUT, async move {
        match &config {
            ConnectionConfig::Sqlite { path } => sqlite_list_databases(path).await,
            ConnectionConfig::MariaDb { databases, .. } => {
                let mut connection = open_mariadb(&config).await?;
                mariadb_list_databases(&mut connection, databases).await
            }
        }
    })
    .await
}

pub async fn list_tables(config: ConnectionConfig, database: String) -> Result<Vec<TableInfo>> {
    with_timeout(QUERY_TIMEOUT, async move {
        match &config {
            ConnectionConfig::Sqlite { path } => sqlite_list_tables(path, &database).await,
            ConnectionConfig::MariaDb { .. } => {
                let mut connection = open_mariadb(&config).await?;
                mariadb_list_tables(&mut connection, &database).await
            }
        }
    })
    .await
}

pub async fn list_columns(
    config: ConnectionConfig,
    database: String,
    table: String,
) -> Result<Vec<ColumnInfo>> {
    with_timeout(QUERY_TIMEOUT, async move {
        match &config {
            ConnectionConfig::Sqlite { path } => sqlite_list_columns(path, &database, &table).await,
            ConnectionConfig::MariaDb { .. } => {
                let mut connection = open_mariadb(&config).await?;
                mariadb_list_columns(&mut connection, &database, &table).await
            }
        }
    })
    .await
}

async fn with_timeout<T>(duration: Duration, future: impl Future<Output = Result<T>>) -> Result<T> {
    tokio::time::timeout(duration, future)
        .await
        .context("database request timed out")?
}

fn expand_path(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/") {
        util::paths::home_dir().join(rest)
    } else {
        PathBuf::from(path)
    }
}

async fn open_sqlite(path: &str) -> Result<SqliteConnection> {
    let expanded = expand_path(path);
    anyhow::ensure!(
        expanded.is_file(),
        "no database file at {}",
        expanded.display()
    );
    let options = SqliteConnectOptions::new()
        .filename(&expanded)
        .read_only(true)
        .busy_timeout(Duration::from_secs(5));
    tokio::time::timeout(CONNECT_TIMEOUT, options.connect())
        .await
        .with_context(|| format!("connecting to {} timed out", expanded.display()))?
        .with_context(|| format!("failed to open SQLite database at {}", expanded.display()))
}

async fn open_mariadb(config: &ConnectionConfig) -> Result<MySqlConnection> {
    let ConnectionConfig::MariaDb {
        host,
        port,
        username,
        password,
        ..
    } = config
    else {
        anyhow::bail!("not a MariaDB connection");
    };
    let mut options = MySqlConnectOptions::new()
        .host(host)
        .port(*port)
        .username(username);
    if let Some(password) = password {
        options = options.password(password);
    }
    tokio::time::timeout(CONNECT_TIMEOUT, options.connect())
        .await
        .with_context(|| format!("connecting to {host}:{port} timed out"))?
        .with_context(|| format!("failed to connect to MariaDB at {host}:{port}"))
}

/// Quotes an identifier (schema or table name) for interpolation into SQL that
/// does not support binding identifiers.
fn quote_identifier(identifier: &str) -> String {
    format!("\"{}\"", identifier.replace('"', "\"\""))
}

async fn sqlite_list_databases(path: &str) -> Result<Vec<DatabaseInfo>> {
    let mut connection = open_sqlite(path).await?;
    let rows = sqlx::query("SELECT name FROM pragma_database_list ORDER BY seq")
        .fetch_all(&mut connection)
        .await?;
    rows.into_iter()
        .map(|row| {
            Ok(DatabaseInfo {
                name: row.try_get("name")?,
                charset: None,
                collation: None,
            })
        })
        .collect()
}

async fn sqlite_list_tables(path: &str, database: &str) -> Result<Vec<TableInfo>> {
    let mut connection = open_sqlite(path).await?;
    let query = format!(
        "SELECT name, type FROM {}.sqlite_master \
         WHERE type IN ('table', 'view') AND name NOT LIKE 'sqlite_%' ORDER BY name",
        quote_identifier(database)
    );
    let rows = sqlx::query(&query).fetch_all(&mut connection).await?;
    rows.into_iter()
        .map(|row| {
            let kind: String = row.try_get("type")?;
            Ok(TableInfo {
                name: row.try_get("name")?,
                table_type: if kind == "view" {
                    TableType::View
                } else {
                    TableType::Table
                },
                engine: None,
                row_count: None,
                size_bytes: None,
                collation: None,
                comment: None,
            })
        })
        .collect()
}

async fn sqlite_list_columns(path: &str, database: &str, table: &str) -> Result<Vec<ColumnInfo>> {
    let mut connection = open_sqlite(path).await?;

    let foreign_keys: HashMap<String, String> =
        sqlx::query("SELECT \"from\", \"table\", \"to\" FROM pragma_foreign_key_list(?1, ?2)")
            .bind(table)
            .bind(database)
            .fetch_all(&mut connection)
            .await?
            .into_iter()
            .map(|row| {
                let from: String = row.try_get("from")?;
                let target_table: String = row.try_get("table")?;
                let target_column: Option<String> = row.try_get("to")?;
                let target = match target_column {
                    Some(column) => format!("{target_table}.{column}"),
                    None => target_table,
                };
                Ok((from, target))
            })
            .collect::<Result<_>>()?;

    // Columns covered by a single-column unique index are reported as unique;
    // multi-column unique indexes have no per-column meaning.
    let unique_index_columns = sqlx::query(
        "SELECT il.name AS index_name, ii.name AS column_name \
         FROM pragma_index_list(?1, ?2) AS il, pragma_index_info(il.name) AS ii \
         WHERE il.\"unique\" = 1",
    )
    .bind(table)
    .bind(database)
    .fetch_all(&mut connection)
    .await?;
    let mut index_columns: HashMap<String, Vec<String>> = HashMap::default();
    for row in unique_index_columns {
        let index_name: String = row.try_get("index_name")?;
        let column_name: String = row.try_get("column_name")?;
        index_columns
            .entry(index_name)
            .or_default()
            .push(column_name);
    }
    let unique_columns: HashSet<String> = index_columns
        .into_values()
        .filter(|columns| columns.len() == 1)
        .flatten()
        .collect();

    let rows = sqlx::query(
        "SELECT name, type, \"notnull\", dflt_value, pk \
         FROM pragma_table_info(?1, ?2) ORDER BY cid",
    )
    .bind(table)
    .bind(database)
    .fetch_all(&mut connection)
    .await?;
    rows.into_iter()
        .map(|row| {
            let name: String = row.try_get("name")?;
            let not_null: i64 = row.try_get("notnull")?;
            let primary_key: i64 = row.try_get("pk")?;
            Ok(ColumnInfo {
                data_type: row.try_get("type")?,
                nullable: not_null == 0 && primary_key == 0,
                default: row.try_get("dflt_value")?,
                primary_key: primary_key > 0,
                unique: unique_columns.contains(&name),
                auto_increment: false,
                foreign_key: foreign_keys.get(&name).cloned(),
                charset: None,
                collation: None,
                comment: None,
                name,
            })
        })
        .collect()
}

async fn mariadb_list_databases(
    connection: &mut MySqlConnection,
    databases: &[String],
) -> Result<Vec<DatabaseInfo>> {
    let rows = if databases.is_empty() {
        let placeholders = vec!["?"; MARIADB_SYSTEM_SCHEMAS.len()].join(", ");
        let query = format!(
            "SELECT schema_name AS name, \
                    default_character_set_name AS charset, \
                    default_collation_name AS collation \
             FROM information_schema.schemata \
             WHERE schema_name NOT IN ({placeholders}) \
             ORDER BY schema_name"
        );
        let mut query = sqlx::query(&query);
        for schema in MARIADB_SYSTEM_SCHEMAS {
            query = query.bind(schema);
        }
        query.fetch_all(&mut *connection).await?
    } else {
        let placeholders = vec!["?"; databases.len()].join(", ");
        let query = format!(
            "SELECT schema_name AS name, \
                    default_character_set_name AS charset, \
                    default_collation_name AS collation \
             FROM information_schema.schemata \
             WHERE schema_name IN ({placeholders}) \
             ORDER BY schema_name"
        );
        let mut query = sqlx::query(&query);
        for database in databases {
            query = query.bind(database);
        }
        query.fetch_all(&mut *connection).await?
    };
    rows.into_iter()
        .map(|row| {
            Ok(DatabaseInfo {
                name: row.try_get("name")?,
                charset: row.try_get("charset")?,
                collation: row.try_get("collation")?,
            })
        })
        .collect()
}

async fn mariadb_list_tables(
    connection: &mut MySqlConnection,
    database: &str,
) -> Result<Vec<TableInfo>> {
    let rows = sqlx::query(
        "SELECT table_name AS name, \
                table_type, \
                engine, \
                table_rows, \
                CAST(data_length + index_length AS UNSIGNED) AS total_size, \
                table_collation AS collation, \
                table_comment AS comment \
         FROM information_schema.tables \
         WHERE table_schema = ? \
         ORDER BY table_name",
    )
    .bind(database)
    .fetch_all(connection)
    .await?;
    rows.into_iter()
        .map(|row| {
            let kind: String = row.try_get("table_type")?;
            let table_type = if kind.contains("VIEW") {
                TableType::View
            } else {
                TableType::Table
            };
            let comment: Option<String> = row.try_get("comment")?;
            Ok(TableInfo {
                name: row.try_get("name")?,
                table_type,
                engine: row.try_get("engine")?,
                row_count: row.try_get("table_rows")?,
                size_bytes: row.try_get("total_size")?,
                collation: row.try_get("collation")?,
                // MariaDB reports the comment "VIEW" for every view.
                comment: comment.filter(|comment| !comment.is_empty() && comment != "VIEW"),
            })
        })
        .collect()
}

async fn mariadb_list_columns(
    connection: &mut MySqlConnection,
    database: &str,
    table: &str,
) -> Result<Vec<ColumnInfo>> {
    let foreign_keys: HashMap<String, String> = sqlx::query(
        "SELECT column_name, referenced_table_name, referenced_column_name \
         FROM information_schema.key_column_usage \
         WHERE table_schema = ? AND table_name = ? AND referenced_table_name IS NOT NULL",
    )
    .bind(database)
    .bind(table)
    .fetch_all(&mut *connection)
    .await?
    .into_iter()
    .map(|row| {
        let column: String = row.try_get("column_name")?;
        let target_table: String = row.try_get("referenced_table_name")?;
        let target_column: String = row.try_get("referenced_column_name")?;
        Ok((column, format!("{target_table}.{target_column}")))
    })
    .collect::<Result<_>>()?;

    let rows = sqlx::query(
        "SELECT column_name AS name, \
                column_type, \
                is_nullable, \
                column_default, \
                column_key, \
                extra, \
                character_set_name AS charset, \
                collation_name AS collation, \
                column_comment AS comment \
         FROM information_schema.columns \
         WHERE table_schema = ? AND table_name = ? \
         ORDER BY ordinal_position",
    )
    .bind(database)
    .bind(table)
    .fetch_all(connection)
    .await?;
    rows.into_iter()
        .map(|row| {
            let name: String = row.try_get("name")?;
            let is_nullable: String = row.try_get("is_nullable")?;
            let column_key: String = row.try_get("column_key")?;
            let extra: String = row.try_get("extra")?;
            let comment: Option<String> = row.try_get("comment")?;
            Ok(ColumnInfo {
                data_type: row.try_get("column_type")?,
                nullable: is_nullable == "YES",
                default: row.try_get("column_default")?,
                primary_key: column_key == "PRI",
                unique: column_key == "UNI",
                auto_increment: extra.contains("auto_increment"),
                foreign_key: foreign_keys.get(&name).cloned(),
                charset: row.try_get("charset")?,
                collation: row.try_get("collation")?,
                comment: comment.filter(|comment| !comment.is_empty()),
                name,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::Connection as _;

    #[test]
    fn test_sqlite_introspection() {
        let path = std::env::temp_dir().join(format!(
            "database_panel_schema_test_{}.sqlite",
            std::process::id()
        ));
        std::fs::remove_file(&path).ok();

        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(async {
            let options = SqliteConnectOptions::new()
                .filename(&path)
                .create_if_missing(true);
            let mut connection = options.connect().await.unwrap();
            sqlx::query(
                "CREATE TABLE users (\
                     id INTEGER PRIMARY KEY, \
                     email TEXT NOT NULL UNIQUE, \
                     name TEXT DEFAULT 'anon')",
            )
            .execute(&mut connection)
            .await
            .unwrap();
            sqlx::query(
                "CREATE TABLE posts (\
                     id INTEGER PRIMARY KEY, \
                     user_id INTEGER NOT NULL REFERENCES users(id))",
            )
            .execute(&mut connection)
            .await
            .unwrap();
            sqlx::query("CREATE VIEW v_users AS SELECT * FROM users")
                .execute(&mut connection)
                .await
                .unwrap();
            connection.close().await.unwrap();

            let config = ConnectionConfig::Sqlite {
                path: path.to_string_lossy().into_owned(),
            };

            let databases = list_databases(config.clone()).await.unwrap();
            assert_eq!(
                databases
                    .iter()
                    .map(|database| database.name.as_str())
                    .collect::<Vec<_>>(),
                ["main"]
            );

            let tables = list_tables(config.clone(), "main".into()).await.unwrap();
            let names: Vec<_> = tables
                .iter()
                .map(|table| (table.name.as_str(), table.table_type))
                .collect();
            assert_eq!(
                names,
                [
                    ("posts", TableType::Table),
                    ("users", TableType::Table),
                    ("v_users", TableType::View),
                ]
            );

            let columns = list_columns(config.clone(), "main".into(), "users".into())
                .await
                .unwrap();
            assert_eq!(columns.len(), 3);
            assert!(columns[0].primary_key && columns[0].name == "id");
            assert!(!columns[1].nullable && columns[1].unique && columns[1].name == "email");
            assert_eq!(columns[2].default.as_deref(), Some("'anon'"));

            let columns = list_columns(config, "main".into(), "posts".into())
                .await
                .unwrap();
            assert_eq!(columns[1].foreign_key.as_deref(), Some("users.id"));
        });

        std::fs::remove_file(&path).ok();
    }
}
