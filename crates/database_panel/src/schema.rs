//! Read-only schema introspection for the database panel.
//!
//! All functions here run on the Tokio runtime (via [`gpui_tokio::Tokio`])
//! because sqlx's drivers require it. A fresh connection is opened per request
//! and dropped afterwards, so the panel never holds long-lived connections to
//! the inspected databases.

use anyhow::{Context as _, Result};
use collections::{HashMap, HashSet};
use futures::TryStreamExt as _;
use sqlx::{
    Column as _, ConnectOptions as _, Either, Row as _, ValueRef as _,
    mysql::{MySqlConnectOptions, MySqlConnection, MySqlRow},
    sqlite::{SqliteConnectOptions, SqliteConnection, SqliteRow},
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

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DatabaseInfo {
    pub name: String,
    pub charset: Option<String>,
    pub collation: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TableType {
    Table,
    View,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TableInfo {
    pub name: String,
    pub table_type: TableType,
    pub engine: Option<String>,
    pub row_count: Option<u64>,
    pub size_bytes: Option<u64>,
    pub collation: Option<String>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

/// Lists the columns of every table of a database, keyed by table name.
/// Used to index a whole schema for filtering and completions without opening
/// one connection per table.
pub async fn list_all_columns(
    config: ConnectionConfig,
    database: String,
) -> Result<HashMap<String, Vec<ColumnInfo>>> {
    with_timeout(QUERY_TIMEOUT, async move {
        match &config {
            ConnectionConfig::Sqlite { path } => sqlite_list_all_columns(path, &database).await,
            ConnectionConfig::MariaDb { .. } => {
                let mut connection = open_mariadb(&config).await?;
                mariadb_list_all_columns(&mut connection, &database).await
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
    open_mariadb_with_database(config, None).await
}

async fn open_mariadb_with_database(
    config: &ConnectionConfig,
    database: Option<&str>,
) -> Result<MySqlConnection> {
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
    if let Some(database) = database {
        options = options.database(database);
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
    sqlite_table_columns(&mut connection, database, table).await
}

async fn sqlite_list_all_columns(
    path: &str,
    database: &str,
) -> Result<HashMap<String, Vec<ColumnInfo>>> {
    let mut connection = open_sqlite(path).await?;
    let query = format!(
        "SELECT name FROM {}.sqlite_master \
         WHERE type IN ('table', 'view') AND name NOT LIKE 'sqlite_%' ORDER BY name",
        quote_identifier(database)
    );
    let tables: Vec<String> = sqlx::query(&query)
        .fetch_all(&mut connection)
        .await?
        .into_iter()
        .map(|row| Ok(row.try_get("name")?))
        .collect::<Result<_>>()?;
    let mut columns = HashMap::default();
    for table in tables {
        let table_columns = sqlite_table_columns(&mut connection, database, &table).await?;
        columns.insert(table, table_columns);
    }
    Ok(columns)
}

async fn sqlite_table_columns(
    connection: &mut SqliteConnection,
    database: &str,
    table: &str,
) -> Result<Vec<ColumnInfo>> {
    let foreign_keys: HashMap<String, String> =
        sqlx::query("SELECT \"from\", \"table\", \"to\" FROM pragma_foreign_key_list(?1, ?2)")
            .bind(table)
            .bind(database)
            .fetch_all(&mut *connection)
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
    .fetch_all(&mut *connection)
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
    .fetch_all(&mut *connection)
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

async fn mariadb_list_all_columns(
    connection: &mut MySqlConnection,
    database: &str,
) -> Result<HashMap<String, Vec<ColumnInfo>>> {
    let foreign_keys: HashMap<(String, String), String> = sqlx::query(
        "SELECT table_name, column_name, referenced_table_name, referenced_column_name \
         FROM information_schema.key_column_usage \
         WHERE table_schema = ? AND referenced_table_name IS NOT NULL",
    )
    .bind(database)
    .fetch_all(&mut *connection)
    .await?
    .into_iter()
    .map(|row| {
        let table: String = row.try_get("table_name")?;
        let column: String = row.try_get("column_name")?;
        let target_table: String = row.try_get("referenced_table_name")?;
        let target_column: String = row.try_get("referenced_column_name")?;
        Ok(((table, column), format!("{target_table}.{target_column}")))
    })
    .collect::<Result<_>>()?;

    let rows = sqlx::query(
        "SELECT table_name, \
                column_name AS name, \
                column_type, \
                is_nullable, \
                column_default, \
                column_key, \
                extra, \
                character_set_name AS charset, \
                collation_name AS collation, \
                column_comment AS comment \
         FROM information_schema.columns \
         WHERE table_schema = ? \
         ORDER BY table_name, ordinal_position",
    )
    .bind(database)
    .fetch_all(connection)
    .await?;
    let mut columns: HashMap<String, Vec<ColumnInfo>> = HashMap::default();
    for row in rows {
        let table: String = row.try_get("table_name")?;
        let name: String = row.try_get("name")?;
        let is_nullable: String = row.try_get("is_nullable")?;
        let column_key: String = row.try_get("column_key")?;
        let extra: String = row.try_get("extra")?;
        let comment: Option<String> = row.try_get("comment")?;
        let column = ColumnInfo {
            data_type: row.try_get("column_type")?,
            nullable: is_nullable == "YES",
            default: row.try_get("column_default")?,
            primary_key: column_key == "PRI",
            unique: column_key == "UNI",
            auto_increment: extra.contains("auto_increment"),
            foreign_key: foreign_keys.get(&(table.clone(), name.clone())).cloned(),
            charset: row.try_get("charset")?,
            collation: row.try_get("collation")?,
            comment: comment.filter(|comment| !comment.is_empty()),
            name,
        };
        columns.entry(table).or_default().push(column);
    }
    Ok(columns)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub affected_rows: Option<u64>,
    /// Index of the first returned row within the full result set.
    pub offset: usize,
    /// Whether the result set continues past the returned page.
    pub has_more: bool,
}

/// Runs `sql` and returns the page of at most `limit` rows starting at
/// `offset`. Paging happens while streaming the result set, so it works for
/// any statement without rewriting the SQL; earlier rows are still produced
/// by the database and skipped.
pub async fn run_query(
    config: ConnectionConfig,
    database: Option<String>,
    sql: String,
    offset: usize,
    limit: usize,
) -> Result<QueryResult> {
    with_timeout(QUERY_TIMEOUT, async move {
        match &config {
            ConnectionConfig::Sqlite { path } => {
                let mut connection = open_sqlite(path).await?;
                sqlite_run_query(&mut connection, &sql, offset, limit).await
            }
            ConnectionConfig::MariaDb { .. } => {
                let mut connection =
                    open_mariadb_with_database(&config, database.as_deref()).await?;
                mariadb_run_query(&mut connection, &sql, offset, limit).await
            }
        }
    })
    .await
}

async fn sqlite_run_query(
    connection: &mut SqliteConnection,
    sql: &str,
    offset: usize,
    limit: usize,
) -> Result<QueryResult> {
    let mut stream = sqlx::raw_sql(sql).fetch_many(connection);
    let mut result = QueryResult {
        columns: Vec::new(),
        rows: Vec::new(),
        affected_rows: None,
        offset,
        has_more: false,
    };
    let mut skipped = 0;
    while let Some(item) = stream.try_next().await? {
        match item {
            Either::Left(done) => {
                *result.affected_rows.get_or_insert(0) += done.rows_affected();
            }
            Either::Right(row) => {
                if result.columns.is_empty() {
                    result.columns = row
                        .columns()
                        .iter()
                        .map(|column| column.name().to_string())
                        .collect();
                }
                if skipped < offset {
                    skipped += 1;
                    continue;
                }
                if result.rows.len() >= limit {
                    result.has_more = true;
                    break;
                }
                result.rows.push(
                    (0..row.columns().len())
                        .map(|index| sqlite_value_to_string(&row, index))
                        .collect(),
                );
            }
        }
    }
    Ok(result)
}

async fn mariadb_run_query(
    connection: &mut MySqlConnection,
    sql: &str,
    offset: usize,
    limit: usize,
) -> Result<QueryResult> {
    let mut stream = sqlx::raw_sql(sql).fetch_many(connection);
    let mut result = QueryResult {
        columns: Vec::new(),
        rows: Vec::new(),
        affected_rows: None,
        offset,
        has_more: false,
    };
    let mut skipped = 0;
    while let Some(item) = stream.try_next().await? {
        match item {
            Either::Left(done) => {
                *result.affected_rows.get_or_insert(0) += done.rows_affected();
            }
            Either::Right(row) => {
                if result.columns.is_empty() {
                    result.columns = row
                        .columns()
                        .iter()
                        .map(|column| column.name().to_string())
                        .collect();
                }
                if skipped < offset {
                    skipped += 1;
                    continue;
                }
                if result.rows.len() >= limit {
                    result.has_more = true;
                    break;
                }
                result.rows.push(
                    (0..row.columns().len())
                        .map(|index| mariadb_value_to_string(&row, index))
                        .collect(),
                );
            }
        }
    }
    Ok(result)
}

fn sqlite_value_to_string(row: &SqliteRow, index: usize) -> String {
    match row.try_get_raw(index) {
        Ok(value) if value.is_null() => return "NULL".to_string(),
        Ok(_) => {}
        Err(_) => return "?".to_string(),
    }
    if let Ok(value) = row.try_get::<String, _>(index) {
        return value;
    }
    if let Ok(value) = row.try_get::<i64, _>(index) {
        return value.to_string();
    }
    if let Ok(value) = row.try_get::<f64, _>(index) {
        return value.to_string();
    }
    if let Ok(value) = row.try_get::<bool, _>(index) {
        return value.to_string();
    }
    if let Ok(value) = row.try_get::<Vec<u8>, _>(index) {
        return bytes_to_string(value);
    }
    "<unsupported>".to_string()
}

fn mariadb_value_to_string(row: &MySqlRow, index: usize) -> String {
    match row.try_get_raw(index) {
        Ok(value) if value.is_null() => return "NULL".to_string(),
        Ok(_) => {}
        Err(_) => return "?".to_string(),
    }
    if let Ok(value) = row.try_get::<String, _>(index) {
        return value;
    }
    if let Ok(value) = row.try_get::<i64, _>(index) {
        return value.to_string();
    }
    if let Ok(value) = row.try_get::<u64, _>(index) {
        return value.to_string();
    }
    if let Ok(value) = row.try_get::<f64, _>(index) {
        return value.to_string();
    }
    if let Ok(value) = row.try_get::<bool, _>(index) {
        return value.to_string();
    }
    if let Ok(value) = row.try_get::<chrono::NaiveDateTime, _>(index) {
        return value.to_string();
    }
    if let Ok(value) = row.try_get::<chrono::DateTime<chrono::Utc>, _>(index) {
        return value.to_string();
    }
    if let Ok(value) = row.try_get::<chrono::NaiveDate, _>(index) {
        return value.to_string();
    }
    if let Ok(value) = row.try_get::<chrono::NaiveTime, _>(index) {
        return value.to_string();
    }
    // MySQL's native JSON type is not decodable as String.
    if let Ok(value) = row.try_get::<serde_json::Value, _>(index) {
        return value.to_string();
    }
    if let Ok(value) = row.try_get::<Vec<u8>, _>(index) {
        return bytes_to_string(value);
    }
    "<unsupported>".to_string()
}

/// MariaDB stores JSON as `LONGTEXT` with a binary collation, and columns of
/// binary types often hold readable text; show the text when the bytes are
/// valid UTF-8 instead of hiding it behind a byte count.
fn bytes_to_string(bytes: Vec<u8>) -> String {
    match String::from_utf8(bytes) {
        Ok(text) => text,
        Err(error) => format!("<{} bytes>", error.as_bytes().len()),
    }
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
            sqlx::query(
                "INSERT INTO users (email, name) VALUES ('a@example.com', 'Ada'), \
                 ('b@example.com', NULL)",
            )
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

            let columns = list_columns(config.clone(), "main".into(), "posts".into())
                .await
                .unwrap();
            assert_eq!(columns[1].foreign_key.as_deref(), Some("users.id"));

            let all_columns = list_all_columns(config.clone(), "main".into()).await.unwrap();
            assert_eq!(all_columns.len(), 3);
            assert_eq!(all_columns["users"].len(), 3);
            assert_eq!(all_columns["posts"][1].foreign_key.as_deref(), Some("users.id"));

            let result = run_query(
                config.clone(),
                None,
                "SELECT id, email, name FROM users ORDER BY id".into(),
                0,
                1000,
            )
            .await
            .unwrap();
            assert_eq!(result.columns, ["id", "email", "name"]);
            assert_eq!(
                result.rows,
                [
                    ["1", "a@example.com", "Ada"],
                    ["2", "b@example.com", "NULL"],
                ]
            );
            assert!(!result.has_more);

            let first_page = run_query(
                config.clone(),
                None,
                "SELECT id FROM users ORDER BY id".into(),
                0,
                1,
            )
            .await
            .unwrap();
            assert_eq!(first_page.rows, [["1"]]);
            assert!(first_page.has_more);

            let second_page = run_query(
                config.clone(),
                None,
                "SELECT id FROM users ORDER BY id".into(),
                1,
                1,
            )
            .await
            .unwrap();
            assert_eq!(second_page.rows, [["2"]]);
            assert_eq!(second_page.offset, 1);
            assert!(!second_page.has_more);

            // The SQLite connection is opened read-only, so writes must fail.
            let write = run_query(
                config,
                None,
                "INSERT INTO users (email) VALUES ('c@example.com')".into(),
                0,
                1000,
            )
            .await;
            assert!(write.is_err());
        });

        std::fs::remove_file(&path).ok();
    }
}
