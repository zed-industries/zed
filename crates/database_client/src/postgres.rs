use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
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

/// How long to wait for a TCP connection to a database before giving up, so a
/// black-holed host cannot block queries on every other database indefinitely.
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

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
/// indefinitely. `ReadOnly` sessions set `default_transaction_read_only=on` as a
/// backstop and additionally wrap each `run_query` in an explicit read-only
/// transaction (see [`PostgresClient::run_query`]) so a session GUC override
/// cannot turn a single statement into a write.
pub struct PostgresClient {
    config: ConnectionConfig,
    password: String,
    statement_timeout: Duration,
    mode: SessionMode,
    clients: AsyncMutex<HashMap<String, Arc<Client>>>,
    /// Cancel tokens for queries that are currently in flight, keyed by a unique
    /// id. Each running query registers its token and removes it on completion
    /// (via [`CancelGuard`]), so the map only ever holds genuinely in-flight
    /// queries rather than growing for the lifetime of the client.
    cancel_tokens: Arc<Mutex<HashMap<u64, CancelToken>>>,
    next_cancel_id: AtomicU64,
}

/// Keeps a registered cancel token alive for the duration of a query and removes
/// it from [`PostgresClient::cancel_tokens`] when dropped (on completion, error,
/// or cancellation), preventing the map from accumulating stale tokens.
struct CancelGuard {
    tokens: Arc<Mutex<HashMap<u64, CancelToken>>>,
    id: u64,
}

impl Drop for CancelGuard {
    fn drop(&mut self) {
        if let Ok(mut tokens) = self.tokens.lock() {
            tokens.remove(&self.id);
        }
    }
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
            cancel_tokens: Arc::new(Mutex::new(HashMap::new())),
            next_cancel_id: AtomicU64::new(0),
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
            .connect_timeout(CONNECT_TIMEOUT)
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
    ///
    /// The clients mutex is deliberately not held across `connect().await`: a
    /// hanging connect to one database must not block queries on every other
    /// database. A benign race where two callers connect concurrently is
    /// tolerated by keeping whichever client won the insert.
    async fn client_for(&self, database: &str) -> Result<Arc<Client>> {
        {
            let clients = self.clients.lock().await;
            if let Some(client) = clients.get(database) {
                if !client.is_closed() {
                    return Ok(client.clone());
                }
            }
        }

        let client = self.connect(database).await?;

        let mut clients = self.clients.lock().await;
        // Re-check under the lock in case another caller connected while we were
        // awaiting; if a live client is already cached, prefer it and drop ours.
        if let Some(existing) = clients.get(database) {
            if !existing.is_closed() {
                return Ok(existing.clone());
            }
        }
        clients.insert(database.to_string(), client.clone());
        Ok(client)
    }

    /// Registers a cancel token for an in-flight query and returns a guard that
    /// removes it on completion, so `cancel_running` only ever targets queries
    /// that are actually running.
    fn register_cancel(&self, client: &Client) -> CancelGuard {
        let id = self.next_cancel_id.fetch_add(1, Ordering::Relaxed);
        if let Ok(mut tokens) = self.cancel_tokens.lock() {
            tokens.insert(id, client.cancel_token());
        }
        CancelGuard {
            tokens: self.cancel_tokens.clone(),
            id,
        }
    }

    async fn columns(&self, table: &TableRef) -> Result<Vec<ColumnInfo>> {
        let client = self.client_for(&table.database).await?;
        let _cancel = self.register_cancel(&client);
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
                udt_schema: row.try_get(3)?,
                is_nullable: row.try_get(4)?,
                default: row.try_get(5)?,
                is_primary_key: row.try_get(6)?,
            });
        }
        Ok(columns)
    }
}

#[async_trait::async_trait]
impl DatabaseClient for PostgresClient {
    async fn test_connection(&self) -> Result<()> {
        let client = self.client_for(&self.config.database).await?;
        let _cancel = self.register_cancel(&client);
        client
            .simple_query("SELECT 1")
            .await
            .context("running connection test query")?;
        Ok(())
    }

    async fn list_databases(&self) -> Result<Vec<String>> {
        let client = self.client_for(&self.config.database).await?;
        let _cancel = self.register_cancel(&client);
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
        let _cancel = self.register_cancel(&client);
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
        let _cancel = self.register_cancel(&client);
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
        let fk_rows = {
            let _cancel = self.register_cancel(&client);
            client
                .query(FOREIGN_KEYS_SQL, &[&table.schema, &table.name])
                .await
                .with_context(|| {
                    format!("loading foreign keys for {}.{}", table.schema, table.name)
                })?
        };
        let mut foreign_keys = Vec::with_capacity(fk_rows.len());
        for row in fk_rows {
            foreign_keys.push(ForeignKey {
                column: row.try_get(0)?,
                references_schema: row.try_get(1)?,
                references_table: row.try_get(2)?,
                references_column: row.try_get(3)?,
            });
        }

        let index_rows = {
            let _cancel = self.register_cancel(&client);
            client
                .query(INDEXES_SQL, &[&table.schema, &table.name])
                .await
                .with_context(|| format!("loading indexes for {}.{}", table.schema, table.name))?
        };
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
        let _cancel = self.register_cancel(&client);
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

    /// Runs arbitrary user SQL and returns its result.
    ///
    /// For [`SessionMode::ReadOnly`] (MCP-driven sessions) the user SQL is wrapped
    /// in an explicit `BEGIN READ ONLY` / `ROLLBACK` transaction. This blocks all
    /// direct writes and single-statement mutations even if the SQL flips the
    /// `default_transaction_read_only` session GUC (which is only a backstop, not
    /// a hard boundary). The `ROLLBACK` runs even when the user SQL succeeds —
    /// a read-only transaction has nothing to keep — and always runs on the error
    /// path too, via a cleanup step, so a failed statement never leaves an open
    /// transaction on the cached connection.
    ///
    /// Residual limitation: a deliberately crafted multi-statement query such as
    /// `COMMIT; SET default_transaction_read_only=off; <write>` can still escape
    /// the wrapper by ending our transaction early. The only hard guarantee
    /// against writes is a PostgreSQL role granted just `SELECT` privileges;
    /// this wrapper is defense-in-depth on top of that.
    async fn run_query(&self, database: &str, sql: &str, max_rows: usize) -> Result<QueryResult> {
        let client = self.client_for(database).await?;
        let _cancel = self.register_cancel(&client);

        match self.mode {
            SessionMode::ReadWrite => {
                let messages = client.simple_query(sql).await.context("running query")?;
                parse_query_messages(sql, messages, max_rows)
            }
            SessionMode::ReadOnly => {
                client
                    .simple_query("BEGIN READ ONLY")
                    .await
                    .context("beginning read-only transaction")?;
                let result = client
                    .simple_query(sql)
                    .await
                    .context("running query")
                    .and_then(|messages| parse_query_messages(sql, messages, max_rows));
                // Always roll back: on success nothing needs keeping, and on
                // error this closes the aborted transaction so the cached
                // connection stays usable for later queries.
                if let Err(error) = client.simple_query("ROLLBACK").await {
                    log::warn!("failed to roll back read-only transaction: {error}");
                }
                result
            }
        }
    }

    async fn cancel_running(&self) -> Result<()> {
        let tokens: Vec<CancelToken> = {
            let mut guard = self
                .cancel_tokens
                .lock()
                .map_err(|_| anyhow!("cancel token lock poisoned"))?;
            std::mem::take(&mut *guard).into_values().collect()
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

/// Turns the messages from a `simple_query` into a [`QueryResult`].
///
/// `simple_query` may return results for several statements. To keep the
/// reported `columns` consistent with the reported `rows`, only the last result
/// set (the one whose `RowDescription` came most recently) is retained: a new
/// `RowDescription` resets the accumulated rows so earlier statements' rows are
/// never rendered under a later statement's headers. Callers that need every
/// statement's output should run one statement per call.
fn parse_query_messages(
    sql: &str,
    messages: Vec<SimpleQueryMessage>,
    max_rows: usize,
) -> Result<QueryResult> {
    let mut columns = Vec::new();
    let mut rows: Vec<Vec<Option<String>>> = Vec::new();
    let mut command_tag = None;
    let mut truncated = false;

    for message in messages {
        match message {
            SimpleQueryMessage::RowDescription(description) => {
                // A new result set begins: adopt its headers and discard any
                // rows accumulated for a previous statement so columns and rows
                // always describe the same result set.
                columns = description
                    .iter()
                    .map(|column| column.name().to_string())
                    .collect();
                rows.clear();
                truncated = false;
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
    use crate::{Filter, FilterOp, Sort, SortDirection};

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
        // The `shop` database (tables `customers`, `orders`; view `paid_orders`)
        // is the fixture exercised below; fall back to it when unset.
        let database = std::env::var("ZED_DB_TEST_DATABASE").unwrap_or_else(|_| "shop".into());
        let config = ConnectionConfig {
            name: "test".into(),
            host,
            port: 5432,
            database: database.clone(),
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

        let orders = TableRef {
            database: database.clone(),
            schema: "public".into(),
            name: "orders".into(),
        };

        // Filtering an int primary key by equality: the parameter is bound as
        // text and cast server-side to int4, so this must return exactly the one
        // matching row rather than failing at bind time.
        let by_id = SelectSpec {
            filters: vec![Filter {
                column: "id".into(),
                op: FilterOp::Eq,
                value: "3".into(),
            }],
            sort: None,
            limit: 100,
            offset: 0,
        };
        let page = read_only_client.fetch_rows(&orders, &by_id).await.unwrap();
        assert_eq!(page.rows.len(), 1, "id = 3 must match exactly one order");
        let id_index = page
            .columns
            .iter()
            .position(|name| name == "id")
            .expect("id column present");
        assert_eq!(page.rows[0][id_index].as_deref(), Some("3"));

        // Filtering a numeric column with a typed comparison (`>`): the cast
        // must preserve numeric ordering, not fall back to text comparison.
        let by_total = SelectSpec {
            filters: vec![Filter {
                column: "total".into(),
                op: FilterOp::Gt,
                value: "10".into(),
            }],
            sort: Some(Sort {
                column: "total".into(),
                direction: SortDirection::Asc,
            }),
            limit: 100,
            offset: 0,
        };
        let page = read_only_client
            .fetch_rows(&orders, &by_total)
            .await
            .unwrap();
        let total_index = page
            .columns
            .iter()
            .position(|name| name == "total")
            .expect("total column present");
        assert!(
            !page.rows.is_empty(),
            "total > 10 must match at least one order"
        );
        for row in &page.rows {
            let total: f64 = row[total_index]
                .as_deref()
                .expect("total is not null")
                .parse()
                .expect("total parses as a number");
            assert!(
                total > 10.0,
                "every returned total must exceed 10, got {total}"
            );
        }

        // read-only session: a direct write must be rejected by the server.
        let error = read_only_client
            .run_query(&database, "CREATE TABLE zed_should_fail(id int)", 10)
            .await;
        assert!(error.is_err(), "write must be rejected");

        // read-only session: flipping the session GUC and mutating in a single
        // statement must be blocked by the explicit read-only transaction, even
        // though `SET default_transaction_read_only=off` would defeat the GUC
        // backstop on its own.
        let error = read_only_client
            .run_query(
                &database,
                "SET default_transaction_read_only=off; DELETE FROM orders",
                10,
            )
            .await;
        assert!(
            error.is_err(),
            "read-only transaction must block a GUC-override write"
        );
        // The orders fixture must be untouched by the blocked write above.
        let page = read_only_client.fetch_rows(&orders, &by_id).await.unwrap();
        assert_eq!(
            page.rows.len(),
            1,
            "blocked DELETE must not have removed rows"
        );

        let read_write_client = PostgresClient::new(
            config,
            password,
            Duration::from_secs(30),
            SessionMode::ReadWrite,
        );
        // read-write session: a write must succeed.
        read_write_client
            .run_query(&database, "CREATE TEMPORARY TABLE zed_rw_check(id int)", 10)
            .await
            .unwrap();
    }
}
