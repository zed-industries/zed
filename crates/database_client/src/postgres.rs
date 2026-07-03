use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{Context as _, Result, anyhow, bail};
use tokio::sync::Mutex as AsyncMutex;
use tokio_postgres::types::ToSql;
use tokio_postgres::{CancelToken, Client, NoTls, SimpleQueryMessage};

use crate::sql::{
    self, BuiltStatement, COLUMNS_SQL, FOREIGN_KEYS_SQL, INDEXES_SQL, LIST_DATABASES_SQL,
    LIST_SCHEMAS_SQL, LIST_TABLES_SQL,
};
use crate::{
    AppliedCounts, ColumnInfo, ConnectionConfig, DatabaseClient, ForeignKey, IndexInfo,
    QueryResult, TableEdits, TableInfo, TableRef, TableStructure,
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

    /// Builds the `tokio_postgres::Config` shared by both the cached-connection
    /// path and the dedicated-connection path. `mode` selects the session options
    /// (see [`session_options`]); the cache path passes `self.mode` while
    /// [`PostgresClient::connect_dedicated`] forces [`SessionMode::ReadWrite`].
    fn build_config(&self, database: &str, mode: SessionMode) -> tokio_postgres::Config {
        let mut config = tokio_postgres::Config::new();
        config
            .host(&self.config.host)
            .port(self.config.port)
            .user(&self.config.user)
            .password(&self.password)
            .dbname(database)
            .application_name("zed-database")
            .connect_timeout(CONNECT_TIMEOUT)
            .options(session_options(self.statement_timeout, mode));
        config
    }

    /// Opens a fresh connection to `database`, spawning its background driver.
    async fn connect(&self, database: &str) -> Result<Arc<Client>> {
        let config = self.build_config(database, self.mode);
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

    /// Opens a fresh, uncached read-write connection to `database` for a single
    /// transaction, spawning its background driver. Unlike [`PostgresClient::connect`]
    /// this is never stored in the client cache, so its transaction is fully
    /// isolated from concurrent reads on the shared cached connection. The
    /// returned client's driver task ends when the client is dropped.
    ///
    /// This is used by [`apply_edits`] so a concurrent read (e.g. a user-triggered
    /// reload during the save window) can never interleave into the open edit
    /// transaction. It always uses [`SessionMode::ReadWrite`] because it is the
    /// write path; callers must still enforce the read-only bail themselves.
    async fn connect_dedicated(&self, database: &str) -> Result<Client> {
        let config = self.build_config(database, SessionMode::ReadWrite);
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
        Ok(client)
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

    /// Applies a batch of edits in a single transaction on a read-write session.
    ///
    /// The order is `DELETE` → `UPDATE` → `INSERT`; each `DELETE`/`UPDATE` must
    /// affect exactly one row (fewer or more means the row is gone or was changed
    /// concurrently, and the whole batch is rolled back). The transaction body
    /// runs in [`execute_edits`], whose `Result` is matched here so that a
    /// best-effort `ROLLBACK` always runs on the error path — never leaving a
    /// dangling open transaction — while success commits.
    ///
    /// The transaction runs on a fresh dedicated connection (see
    /// [`PostgresClient::connect_dedicated`]) rather than the shared cached one, so
    /// a concurrent read on the cached connection can never interleave into this
    /// open transaction (seeing dirty rows or hitting "current transaction is
    /// aborted"). The dedicated connection is dropped when this method returns.
    async fn apply_edits(
        &self,
        table: &TableRef,
        columns: &[ColumnInfo],
        edits: &TableEdits,
    ) -> Result<AppliedCounts> {
        if self.mode == SessionMode::ReadOnly {
            bail!("apply_edits requires a read-write session");
        }

        let client = self.connect_dedicated(&table.database).await?;
        // Deliberately do NOT register this dedicated connection in the shared
        // `cancel_tokens` map. `cancel_running()` (wired to the query view's
        // Cancel button on the same client) would otherwise collaterally cancel
        // an in-flight save's statement and abort the whole edit transaction. The
        // dedicated connection is short-lived and isolated by design, so its
        // cancellation must stay independent of the shared query-cancel path.

        client
            .simple_query("BEGIN")
            .await
            .context("beginning edit transaction")?;

        match execute_edits(&client, table, columns, edits).await {
            Ok(counts) => {
                client
                    .simple_query("COMMIT")
                    .await
                    .context("committing edit transaction")?;
                Ok(counts)
            }
            Err(error) => {
                // Best-effort rollback: closing the aborted transaction is tidy but
                // not strictly required, since the dedicated connection is dropped
                // when this method returns. The original error is what the caller
                // needs, so a rollback failure is only logged.
                if let Err(rollback_error) = client.simple_query("ROLLBACK").await {
                    log::warn!("failed to roll back edit transaction: {rollback_error}");
                }
                Err(error)
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

/// Runs the DELETE → UPDATE → INSERT statements of a batch inside an already
/// open transaction, returning the accumulated counts. Any error here (including
/// a `DELETE`/`UPDATE` that did not affect exactly one row) propagates so the
/// caller can roll the whole batch back.
async fn execute_edits(
    client: &Client,
    table: &TableRef,
    columns: &[ColumnInfo],
    edits: &TableEdits,
) -> Result<AppliedCounts> {
    let mut counts = AppliedCounts::default();

    for delete in &edits.deletes {
        let statement = sql::build_delete(table, columns, delete)?;
        let affected = execute_statement(client, &statement)
            .await
            .context("applying row delete")?;
        if affected != 1 {
            bail!("row not found or changed concurrently");
        }
        counts.deleted += 1;
    }

    for update in &edits.updates {
        let statement = sql::build_update(table, columns, update)?;
        let affected = execute_statement(client, &statement)
            .await
            .context("applying row update")?;
        if affected != 1 {
            bail!("row not found or changed concurrently");
        }
        counts.updated += 1;
    }

    for insert in &edits.inserts {
        let statement = sql::build_insert(table, columns, insert)?;
        execute_statement(client, &statement)
            .await
            .context("applying row insert")?;
        counts.inserted += 1;
    }

    Ok(counts)
}

/// Executes a built statement, binding its text parameters as Rust `String`s
/// cast to the column's type server-side, and returns the number of rows it
/// affected.
async fn execute_statement(client: &Client, statement: &BuiltStatement) -> Result<u64> {
    let param_refs: Vec<&(dyn ToSql + Sync)> = statement
        .params
        .iter()
        .map(|param| param as &(dyn ToSql + Sync))
        .collect();
    let affected = client.execute(&statement.sql, &param_refs).await?;
    Ok(affected)
}

/// Builds the `-c` options string for a session's `statement_timeout` and,
/// for [`SessionMode::ReadOnly`], `default_transaction_read_only`.
///
/// Both modes also pin `standard_conforming_strings=on`: `query_state::render_sql`
/// inlines filter values as string literals via `escape_literal`/`escape_like_pattern`,
/// and that escaping is only correct when this setting is on. Pinning it here makes
/// the escaping correct regardless of the server's configured default.
fn session_options(statement_timeout: Duration, mode: SessionMode) -> String {
    let timeout_ms = statement_timeout.as_millis();
    match mode {
        SessionMode::ReadOnly => format!(
            "-c default_transaction_read_only=on -c standard_conforming_strings=on -c statement_timeout={timeout_ms}"
        ),
        SessionMode::ReadWrite => {
            format!("-c standard_conforming_strings=on -c statement_timeout={timeout_ms}")
        }
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
    use crate::{EditCell, RowDelete, RowInsert, RowKey, RowUpdate};

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

    #[test]
    fn session_options_pin_standard_conforming_strings() {
        // query_state::render_sql inlines values as string literals
        // (escape_literal/escape_like_pattern); that escaping is only correct
        // when the session has standard_conforming_strings=on, so both modes
        // must pin it regardless of the server's default.
        let read_only = session_options(Duration::from_secs(30), SessionMode::ReadOnly);
        let read_write = session_options(Duration::from_secs(30), SessionMode::ReadWrite);
        assert!(read_only.contains("standard_conforming_strings=on"));
        assert!(read_write.contains("standard_conforming_strings=on"));
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

        // Filtering an int primary key by equality: the literal is quoted as
        // text and Postgres coerces it to int4 via implicit cast, so this must
        // return exactly the one matching row rather than erroring.
        let page = read_only_client
            .run_query(
                &database,
                "SELECT * FROM \"public\".\"orders\" WHERE \"id\" = '3'",
                100,
            )
            .await
            .unwrap();
        assert_eq!(page.rows.len(), 1, "id = 3 must match exactly one order");
        let id_index = page
            .columns
            .iter()
            .position(|name| name == "id")
            .expect("id column present");
        assert_eq!(page.rows[0][id_index].as_deref(), Some("3"));

        // Filtering a numeric column with a typed comparison (`>`): the cast
        // must preserve numeric ordering, not fall back to text comparison.
        let page = read_only_client
            .run_query(
                &database,
                "SELECT * FROM \"public\".\"orders\" WHERE \"total\" > '10' ORDER BY \"total\" ASC",
                100,
            )
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
        let page = read_only_client
            .run_query(
                &database,
                "SELECT * FROM \"public\".\"orders\" WHERE \"id\" = '3'",
                100,
            )
            .await
            .unwrap();
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

    /// Opens a direct `tokio_postgres` connection for test setup/teardown and
    /// verification, spawning its background driver.
    async fn setup_connection(host: &str, database: &str, password: &str) -> Client {
        let mut config = tokio_postgres::Config::new();
        config
            .host(host)
            .port(5432)
            .user("postgres")
            .password(password)
            .dbname(database);
        let (client, connection) = config.connect(NoTls).await.expect("setup connection");
        tokio::spawn(async move {
            if let Err(error) = connection.await {
                eprintln!("setup connection closed with error: {error}");
            }
        });
        client
    }

    #[tokio::test]
    #[ignore = "requires live postgres: ZED_DB_TEST_HOST/PASSWORD"]
    async fn apply_edits_transaction_smoke() {
        let host = std::env::var("ZED_DB_TEST_HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let database = std::env::var("ZED_DB_TEST_DATABASE").unwrap_or_else(|_| "shop".into());
        let password = std::env::var("ZED_DB_TEST_PASSWORD").unwrap_or_else(|_| "postgres".into());

        // Direct connection drives table setup, teardown, and verification so the
        // assertions are independent of the code under test.
        let setup = setup_connection(&host, &database, &password).await;
        setup
            .simple_query("DROP TABLE IF EXISTS zed_edit_test")
            .await
            .expect("drop pre-existing test table");
        setup
            .simple_query(
                "CREATE TABLE zed_edit_test (id int PRIMARY KEY, name text); \
                 INSERT INTO zed_edit_test (id, name) VALUES (1, 'one'), (2, 'two')",
            )
            .await
            .expect("create and seed test table");

        let config = ConnectionConfig {
            name: "test".into(),
            host: host.clone(),
            port: 5432,
            database: database.clone(),
            user: "postgres".into(),
        };
        let client = PostgresClient::new(
            config,
            password.clone(),
            Duration::from_secs(30),
            SessionMode::ReadWrite,
        );
        let table = TableRef {
            database: database.clone(),
            schema: "public".into(),
            name: "zed_edit_test".into(),
        };
        let columns = client
            .columns(&table)
            .await
            .expect("load test table columns");

        let key = |id: &str| RowKey {
            columns: vec!["id".into()],
            values: vec![Some(id.to_string())],
        };

        // Update id=1's name, insert id=3, delete id=2, all in one transaction.
        let edits = TableEdits {
            updates: vec![RowUpdate {
                key: key("1"),
                set: vec![("name".into(), EditCell::Value("one-edited".into()))],
            }],
            inserts: vec![RowInsert {
                values: vec![
                    ("id".into(), EditCell::Value("3".into())),
                    ("name".into(), EditCell::Value("three".into())),
                ],
            }],
            deletes: vec![RowDelete { key: key("2") }],
        };
        let counts = client
            .apply_edits(&table, &columns, &edits)
            .await
            .expect("apply_edits succeeds");
        assert_eq!(
            counts,
            AppliedCounts {
                updated: 1,
                inserted: 1,
                deleted: 1,
            }
        );

        // Verify the final state via the independent setup connection.
        let rows = setup
            .query("SELECT id, name FROM zed_edit_test ORDER BY id", &[])
            .await
            .expect("read back rows");
        let state: Vec<(i32, String)> = rows
            .iter()
            .map(|row| (row.get::<_, i32>(0), row.get::<_, String>(1)))
            .collect();
        assert_eq!(
            state,
            vec![(1, "one-edited".to_string()), (3, "three".to_string()),],
            "id=1 updated, id=2 deleted, id=3 inserted"
        );

        // Negative case: an insert that violates the primary key must roll back
        // the whole batch, leaving the table exactly as it was above.
        let bad_edits = TableEdits {
            updates: vec![RowUpdate {
                key: key("1"),
                set: vec![("name".into(), EditCell::Value("should-not-stick".into()))],
            }],
            inserts: vec![RowInsert {
                // id=3 already exists → duplicate primary key.
                values: vec![
                    ("id".into(), EditCell::Value("3".into())),
                    ("name".into(), EditCell::Value("dup".into())),
                ],
            }],
            deletes: vec![],
        };
        let error = client.apply_edits(&table, &columns, &bad_edits).await;
        assert!(error.is_err(), "duplicate primary key must fail the batch");

        let rows = setup
            .query("SELECT id, name FROM zed_edit_test ORDER BY id", &[])
            .await
            .expect("read back rows after rollback");
        let state: Vec<(i32, String)> = rows
            .iter()
            .map(|row| (row.get::<_, i32>(0), row.get::<_, String>(1)))
            .collect();
        assert_eq!(
            state,
            vec![(1, "one-edited".to_string()), (3, "three".to_string()),],
            "failed batch must roll back the update too"
        );

        setup
            .simple_query("DROP TABLE zed_edit_test")
            .await
            .expect("drop test table");
    }

    /// An insert whose value map is empty (the user added a row and left every
    /// cell unset) must apply as `INSERT ... DEFAULT VALUES`, filling the
    /// `serial` primary key and the `default`-bearing column from their
    /// defaults, so the row is created rather than the batch being rejected.
    #[tokio::test]
    #[ignore = "requires live postgres: ZED_DB_TEST_HOST/PASSWORD"]
    async fn apply_edits_empty_insert_uses_default_values() {
        let host = std::env::var("ZED_DB_TEST_HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let database = std::env::var("ZED_DB_TEST_DATABASE").unwrap_or_else(|_| "shop".into());
        let password = std::env::var("ZED_DB_TEST_PASSWORD").unwrap_or_else(|_| "postgres".into());

        // Direct connection drives table setup, teardown, and verification so the
        // assertions are independent of the code under test.
        let setup = setup_connection(&host, &database, &password).await;
        setup
            .simple_query("DROP TABLE IF EXISTS zed_default_insert_test")
            .await
            .expect("drop pre-existing test table");
        // A `serial` PK and a `default`-bearing, nullable column, so an
        // all-default insert is valid on its own.
        setup
            .simple_query(
                "CREATE TABLE zed_default_insert_test (\
                   id serial PRIMARY KEY, \
                   label text DEFAULT 'unset')",
            )
            .await
            .expect("create test table");

        let config = ConnectionConfig {
            name: "test".into(),
            host: host.clone(),
            port: 5432,
            database: database.clone(),
            user: "postgres".into(),
        };
        let client = PostgresClient::new(
            config,
            password.clone(),
            Duration::from_secs(30),
            SessionMode::ReadWrite,
        );
        let table = TableRef {
            database: database.clone(),
            schema: "public".into(),
            name: "zed_default_insert_test".into(),
        };
        let columns = client
            .columns(&table)
            .await
            .expect("load test table columns");

        // One insert with an empty value map: everything defaults.
        let edits = TableEdits {
            updates: vec![],
            inserts: vec![RowInsert { values: vec![] }],
            deletes: vec![],
        };
        let counts = client
            .apply_edits(&table, &columns, &edits)
            .await
            .expect("apply_edits with an all-default insert succeeds");
        assert_eq!(
            counts,
            AppliedCounts {
                updated: 0,
                inserted: 1,
                deleted: 0,
            }
        );

        // Verify a row appeared with the defaulted column value.
        let rows = setup
            .query(
                "SELECT count(*)::int, min(label) FROM zed_default_insert_test",
                &[],
            )
            .await
            .expect("read back rows");
        let count: i32 = rows[0].get(0);
        let label: Option<String> = rows[0].get(1);
        assert_eq!(
            count, 1,
            "the all-default insert must create exactly one row"
        );
        assert_eq!(
            label.as_deref(),
            Some("unset"),
            "the defaulted column must take its default"
        );

        setup
            .simple_query("DROP TABLE zed_default_insert_test")
            .await
            .expect("drop test table");
    }
}
