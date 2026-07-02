use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{Context as _, Result, anyhow};
use tokio::sync::Mutex as AsyncMutex;
use tokio_postgres::types::ToSql;
use tokio_postgres::{CancelToken, Client, NoTls, SimpleQueryMessage};

use crate::sql::{
    self, COLUMNS_SQL, FOREIGN_KEYS_SQL, INDEXES_SQL, LIST_DATABASES_SQL, LIST_SCHEMAS_SQL,
    LIST_TABLES_SQL,
};
use crate::{
    ColumnInfo, ConnectionConfig, DatabaseClient, ForeignKey, IndexInfo, QueryResult, RowsPage,
    SelectSpec, TableInfo, TableRef, TableStructure,
};

/// Whether a [`PostgresClient`] may execute writes. MCP-driven sessions must
/// stay `ReadOnly` so an agent can never mutate data through the tool.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionMode {
    ReadWrite,
    ReadOnly,
}

/// A [`DatabaseClient`] backed by `tokio-postgres`.
///
/// Connections are opened lazily per database and cached. Every session is
/// configured with a `statement_timeout` so the tool cannot hang the server
/// indefinitely, and `ReadOnly` sessions additionally set
/// `default_transaction_read_only=on` so the tool cannot mutate data.
pub struct PostgresClient {
    config: ConnectionConfig,
    password: String,
    statement_timeout: Duration,
    mode: SessionMode,
    clients: AsyncMutex<HashMap<String, Arc<Client>>>,
    cancel_tokens: Mutex<Vec<CancelToken>>,
}

impl PostgresClient {
    pub fn new(
        config: ConnectionConfig,
        password: String,
        statement_timeout: Duration,
        mode: SessionMode,
    ) -> Self {
        Self {
            config,
            password,
            statement_timeout,
            mode,
            clients: AsyncMutex::new(HashMap::new()),
            cancel_tokens: Mutex::new(Vec::new()),
        }
    }

    fn build_config(&self, database: &str) -> tokio_postgres::Config {
        let mut config = tokio_postgres::Config::new();
        config
            .host(&self.config.host)
            .port(self.config.port)
            .user(&self.config.user)
            .password(&self.password)
            .dbname(database)
            .application_name("zed-database")
            .options(session_options(self.statement_timeout, self.mode));
        config
    }

    /// Opens a fresh connection to `database`, spawning its background driver.
    async fn connect(&self, database: &str) -> Result<Arc<Client>> {
        let config = self.build_config(database);
        let (client, connection) = config
            .connect(NoTls)
            .await
            .with_context(|| format!("connecting to database {database}"))?;
        let database = database.to_string();
        tokio::spawn(async move {
            if let Err(error) = connection.await {
                log::warn!("postgres connection to {database} closed with error: {error}");
            }
        });
        Ok(Arc::new(client))
    }

    /// Returns a cached client for `database`, reconnecting if the cached one is closed.
    async fn client_for(&self, database: &str) -> Result<Arc<Client>> {
        let mut clients = self.clients.lock().await;
        if let Some(client) = clients.get(database) {
            if !client.is_closed() {
                return Ok(client.clone());
            }
        }
        let client = self.connect(database).await?;
        clients.insert(database.to_string(), client.clone());
        Ok(client)
    }

    /// Registers a cancel token so an in-flight query can be aborted by `cancel_running`.
    fn register_cancel(&self, client: &Client) {
        if let Ok(mut tokens) = self.cancel_tokens.lock() {
            tokens.push(client.cancel_token());
        }
    }

    async fn columns(&self, table: &TableRef) -> Result<Vec<ColumnInfo>> {
        let client = self.client_for(&table.database).await?;
        self.register_cancel(&client);
        let rows = client
            .query(COLUMNS_SQL, &[&table.schema, &table.name])
            .await
            .with_context(|| format!("loading columns for {}.{}", table.schema, table.name))?;
        let mut columns = Vec::with_capacity(rows.len());
        for row in rows {
            columns.push(ColumnInfo {
                name: row.try_get(0)?,
                data_type: row.try_get(1)?,
                udt_name: row.try_get(2)?,
                is_nullable: row.try_get(3)?,
                default: row.try_get(4)?,
                is_primary_key: row.try_get(5)?,
            });
        }
        Ok(columns)
    }
}

#[async_trait::async_trait]
impl DatabaseClient for PostgresClient {
    async fn test_connection(&self) -> Result<()> {
        let client = self.client_for(&self.config.database).await?;
        self.register_cancel(&client);
        client
            .simple_query("SELECT 1")
            .await
            .context("running connection test query")?;
        Ok(())
    }

    async fn list_databases(&self) -> Result<Vec<String>> {
        let client = self.client_for(&self.config.database).await?;
        self.register_cancel(&client);
        let rows = client
            .query(LIST_DATABASES_SQL, &[])
            .await
            .context("listing databases")?;
        let mut databases = Vec::with_capacity(rows.len());
        for row in rows {
            databases.push(row.try_get::<_, String>(0)?);
        }
        Ok(databases)
    }

    async fn list_schemas(&self, database: &str) -> Result<Vec<String>> {
        let client = self.client_for(database).await?;
        self.register_cancel(&client);
        let rows = client
            .query(LIST_SCHEMAS_SQL, &[])
            .await
            .context("listing schemas")?;
        let mut schemas = Vec::with_capacity(rows.len());
        for row in rows {
            schemas.push(row.try_get::<_, String>(0)?);
        }
        Ok(schemas)
    }

    async fn list_tables(&self, database: &str, schema: &str) -> Result<Vec<TableInfo>> {
        let client = self.client_for(database).await?;
        self.register_cancel(&client);
        let rows = client
            .query(LIST_TABLES_SQL, &[&schema])
            .await
            .context("listing tables")?;
        let mut tables = Vec::with_capacity(rows.len());
        for row in rows {
            let name: String = row.try_get(0)?;
            let table_type: String = row.try_get(1)?;
            tables.push(TableInfo {
                name,
                is_view: table_type == "VIEW",
            });
        }
        Ok(tables)
    }

    async fn table_structure(&self, table: &TableRef) -> Result<TableStructure> {
        let columns = self.columns(table).await?;

        let client = self.client_for(&table.database).await?;
        self.register_cancel(&client);
        let fk_rows = client
            .query(FOREIGN_KEYS_SQL, &[&table.schema, &table.name])
            .await
            .with_context(|| format!("loading foreign keys for {}.{}", table.schema, table.name))?;
        let mut foreign_keys = Vec::with_capacity(fk_rows.len());
        for row in fk_rows {
            foreign_keys.push(ForeignKey {
                column: row.try_get(0)?,
                references_schema: row.try_get(1)?,
                references_table: row.try_get(2)?,
                references_column: row.try_get(3)?,
            });
        }

        self.register_cancel(&client);
        let index_rows = client
            .query(INDEXES_SQL, &[&table.schema, &table.name])
            .await
            .with_context(|| format!("loading indexes for {}.{}", table.schema, table.name))?;
        let mut indexes = Vec::with_capacity(index_rows.len());
        for row in index_rows {
            indexes.push(IndexInfo {
                name: row.try_get(0)?,
                definition: row.try_get(1)?,
            });
        }

        Ok(TableStructure {
            columns,
            foreign_keys,
            indexes,
        })
    }

    async fn fetch_rows(&self, table: &TableRef, spec: &SelectSpec) -> Result<RowsPage> {
        let columns = self.columns(table).await?;
        let built = sql::build_select(table, &columns, spec)?;

        let client = self.client_for(&table.database).await?;
        self.register_cancel(&client);
        let param_refs: Vec<&(dyn ToSql + Sync)> = built
            .params
            .iter()
            .map(|param| param as &(dyn ToSql + Sync))
            .collect();
        let rows = client
            .query(&built.sql, &param_refs)
            .await
            .with_context(|| format!("fetching rows from {}.{}", table.schema, table.name))?;

        let has_more = rows.len() > spec.limit;
        let take = rows.len().min(spec.limit);
        let mut result_rows = Vec::with_capacity(take);
        for row in rows.iter().take(take) {
            let mut values = Vec::with_capacity(columns.len());
            for index in 0..columns.len() {
                values.push(row.try_get::<_, Option<String>>(index)?);
            }
            result_rows.push(values);
        }

        Ok(RowsPage {
            columns: columns.into_iter().map(|column| column.name).collect(),
            rows: result_rows,
            has_more,
        })
    }

    async fn run_query(&self, database: &str, sql: &str, max_rows: usize) -> Result<QueryResult> {
        let client = self.client_for(database).await?;
        self.register_cancel(&client);
        let messages = client.simple_query(sql).await.context("running query")?;

        let mut columns = Vec::new();
        let mut rows: Vec<Vec<Option<String>>> = Vec::new();
        let mut command_tag = None;
        let mut truncated = false;

        for message in messages {
            match message {
                SimpleQueryMessage::RowDescription(description) => {
                    columns = description
                        .iter()
                        .map(|column| column.name().to_string())
                        .collect();
                }
                SimpleQueryMessage::Row(row) => {
                    if columns.is_empty() {
                        columns = row
                            .columns()
                            .iter()
                            .map(|column| column.name().to_string())
                            .collect();
                    }
                    if rows.len() >= max_rows {
                        truncated = true;
                        continue;
                    }
                    let mut values = Vec::with_capacity(row.len());
                    for index in 0..row.len() {
                        values.push(row.try_get(index)?.map(|value| value.to_string()));
                    }
                    rows.push(values);
                }
                SimpleQueryMessage::CommandComplete(count) => {
                    command_tag = Some(format!("{} {count}", command_verb(sql)));
                }
                _ => {}
            }
        }

        Ok(QueryResult {
            columns,
            rows,
            truncated,
            command_tag,
        })
    }

    async fn cancel_running(&self) -> Result<()> {
        let tokens = {
            let mut guard = self
                .cancel_tokens
                .lock()
                .map_err(|_| anyhow!("cancel token lock poisoned"))?;
            std::mem::take(&mut *guard)
        };

        let mut first_error = None;
        for token in tokens {
            if let Err(error) = token.cancel_query(NoTls).await {
                log::warn!("failed to cancel running query: {error}");
                if first_error.is_none() {
                    first_error = Some(error);
                }
            }
        }

        if let Some(error) = first_error {
            return Err(anyhow!(error).context("cancelling running queries"));
        }
        Ok(())
    }
}

/// Builds the `-c` options string for a session's `statement_timeout` and,
/// for [`SessionMode::ReadOnly`], `default_transaction_read_only`.
fn session_options(statement_timeout: Duration, mode: SessionMode) -> String {
    let timeout_ms = statement_timeout.as_millis();
    match mode {
        SessionMode::ReadOnly => {
            format!("-c default_transaction_read_only=on -c statement_timeout={timeout_ms}")
        }
        SessionMode::ReadWrite => format!("-c statement_timeout={timeout_ms}"),
    }
}

/// Extracts the leading SQL verb (uppercased) to reconstruct a command tag,
/// since `simple_query` only reports the affected row count.
fn command_verb(sql: &str) -> String {
    sql.trim_start()
        .split(|c: char| c.is_whitespace() || c == '(')
        .next()
        .unwrap_or("")
        .to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_verb_extracts_leading_keyword() {
        assert_eq!(command_verb("  select * from users"), "SELECT");
        assert_eq!(command_verb("INSERT INTO t VALUES (1)"), "INSERT");
        assert_eq!(command_verb(""), "");
    }

    #[test]
    fn session_options_read_only_disables_writes() {
        let options = session_options(Duration::from_secs(30), SessionMode::ReadOnly);
        assert!(options.contains("default_transaction_read_only=on"));
        assert!(options.contains("statement_timeout=30000"));
    }

    #[test]
    fn session_options_read_write_allows_writes() {
        let options = session_options(Duration::from_secs(30), SessionMode::ReadWrite);
        assert!(!options.contains("default_transaction_read_only=on"));
        assert!(options.contains("statement_timeout=30000"));
    }

    #[tokio::test]
    #[ignore = "requires live postgres: ZED_DB_TEST_HOST/PORT/USER/PASSWORD"]
    async fn postgres_client_smoke() {
        let host = std::env::var("ZED_DB_TEST_HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let config = ConnectionConfig {
            name: "test".into(),
            host,
            port: 5432,
            database: "postgres".into(),
            user: "postgres".into(),
        };
        let password = std::env::var("ZED_DB_TEST_PASSWORD").unwrap_or_else(|_| "postgres".into());
        let read_only_client = PostgresClient::new(
            config.clone(),
            password.clone(),
            Duration::from_secs(30),
            SessionMode::ReadOnly,
        );
        read_only_client.test_connection().await.unwrap();
        assert!(!read_only_client.list_databases().await.unwrap().is_empty());
        // read-only session: a write must be rejected by the server.
        let error = read_only_client
            .run_query("postgres", "CREATE TABLE zed_should_fail(id int)", 10)
            .await;
        assert!(error.is_err(), "write must be rejected");

        let read_write_client = PostgresClient::new(
            config,
            password,
            Duration::from_secs(30),
            SessionMode::ReadWrite,
        );
        // read-write session: a write must succeed.
        read_write_client
            .run_query(
                "postgres",
                "CREATE TEMPORARY TABLE zed_rw_check(id int)",
                10,
            )
            .await
            .unwrap();
    }
}
