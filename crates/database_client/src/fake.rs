use std::collections::VecDeque;
use std::sync::Mutex;

use anyhow::{Result, anyhow, bail};

use crate::*;

/// In-memory [`DatabaseClient`] returning canned data and recording each call.
///
/// Available in tests and behind the `test-support` feature so that UI crates
/// can drive the panel without a live PostgreSQL server.
pub struct FakeDatabaseClient {
    pub databases: Vec<String>,
    pub schemas: Vec<String>,
    pub tables: Vec<TableInfo>,
    pub structure: TableStructure,
    pub query_result: QueryResult,
    pub error: Option<String>,
    /// FIFO queue of results consumed one at a time by `run_query`, falling
    /// back to `query_result` once empty. Lets a single test drive multiple
    /// distinct `run_query` calls (e.g. successive pages) with different
    /// responses.
    queued_results: Mutex<VecDeque<QueryResult>>,
    /// When set, fails only `run_query` (unlike `error`, which fails every
    /// method including the eager `table_structure` load).
    run_query_error: Mutex<Option<String>>,
    /// When set, fails only `apply_edits`, unlike `error` which fails every
    /// method. Lets a test drive a table to a fully loaded, editable state
    /// (structure and first page both succeed) and then exercise the save
    /// error path in isolation.
    apply_edits_error: Mutex<Option<String>>,
    /// Canned response for `preview_write`.
    pub write_preview: WritePreview,
    /// Canned response for `commit_write`.
    pub write_outcome: WriteOutcome,
    /// When set, fails only `preview_write`, unlike `error` which fails every
    /// method.
    preview_write_error: Mutex<Option<String>>,
    /// When set, fails only `commit_write`, unlike `error` which fails every
    /// method.
    commit_write_error: Mutex<Option<String>>,
    calls: Mutex<Vec<String>>,
}

impl Default for FakeDatabaseClient {
    fn default() -> Self {
        Self::new()
    }
}

impl FakeDatabaseClient {
    pub fn new() -> Self {
        let structure = TableStructure {
            columns: vec![
                ColumnInfo {
                    name: "id".into(),
                    data_type: "integer".into(),
                    udt_name: "int4".into(),
                    udt_schema: "pg_catalog".into(),
                    is_nullable: false,
                    default: None,
                    is_primary_key: true,
                },
                ColumnInfo {
                    name: "name".into(),
                    data_type: "text".into(),
                    udt_name: "text".into(),
                    udt_schema: "pg_catalog".into(),
                    is_nullable: true,
                    default: None,
                    is_primary_key: false,
                },
            ],
            foreign_keys: Vec::new(),
            indexes: Vec::new(),
        };
        let query_result = QueryResult {
            columns: vec!["count".into()],
            rows: vec![vec![Some("3".into())]],
            truncated: false,
            command_tag: Some("SELECT 1".into()),
        };
        Self {
            databases: vec!["app".into(), "postgres".into()],
            schemas: vec!["public".into()],
            tables: vec![
                TableInfo {
                    name: "users".into(),
                    is_view: false,
                },
                TableInfo {
                    name: "orders_view".into(),
                    is_view: true,
                },
            ],
            structure,
            query_result,
            error: None,
            queued_results: Mutex::new(VecDeque::new()),
            run_query_error: Mutex::new(None),
            apply_edits_error: Mutex::new(None),
            write_preview: WritePreview::default(),
            write_outcome: WriteOutcome::default(),
            preview_write_error: Mutex::new(None),
            commit_write_error: Mutex::new(None),
            calls: Mutex::new(Vec::new()),
        }
    }

    /// Pushes a result onto the FIFO queue consumed by `run_query`. Once the
    /// queue is drained, `run_query` falls back to `query_result`.
    pub fn push_query_result(&self, result: QueryResult) {
        if let Ok(mut queue) = self.queued_results.lock() {
            queue.push_back(result);
        }
    }

    /// Sets or clears an error that fails only `run_query`, unlike `error`
    /// which fails every method.
    pub fn set_run_query_error(&self, error: Option<String>) {
        if let Ok(mut slot) = self.run_query_error.lock() {
            *slot = error;
        }
    }

    /// Sets or clears an error that fails only `apply_edits`, unlike `error`
    /// which fails every method.
    pub fn set_apply_edits_error(&self, error: Option<String>) {
        if let Ok(mut slot) = self.apply_edits_error.lock() {
            *slot = error;
        }
    }

    /// Sets or clears an error that fails only `preview_write`, unlike `error`
    /// which fails every method.
    pub fn set_preview_write_error(&self, error: Option<String>) {
        if let Ok(mut slot) = self.preview_write_error.lock() {
            *slot = error;
        }
    }

    /// Sets or clears an error that fails only `commit_write`, unlike `error`
    /// which fails every method.
    pub fn set_commit_write_error(&self, error: Option<String>) {
        if let Ok(mut slot) = self.commit_write_error.lock() {
            *slot = error;
        }
    }

    /// Constructs a client whose every method fails with `message`.
    pub fn with_error(message: &str) -> Self {
        Self {
            error: Some(message.to_string()),
            ..Self::new()
        }
    }

    /// Returns the recorded calls in order.
    pub fn calls(&self) -> Vec<String> {
        self.calls
            .lock()
            .map(|calls| calls.clone())
            .unwrap_or_default()
    }

    fn record(&self, call: impl Into<String>) {
        if let Ok(mut calls) = self.calls.lock() {
            calls.push(call.into());
        }
    }

    fn check_error(&self) -> Result<()> {
        if let Some(message) = &self.error {
            return Err(anyhow!("{message}"));
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl DatabaseClient for FakeDatabaseClient {
    async fn test_connection(&self) -> Result<()> {
        self.check_error()?;
        self.record("test_connection");
        Ok(())
    }

    async fn list_databases(&self) -> Result<Vec<String>> {
        self.check_error()?;
        self.record("list_databases");
        Ok(self.databases.clone())
    }

    async fn list_schemas(&self, database: &str) -> Result<Vec<String>> {
        self.check_error()?;
        self.record(format!("list_schemas {database}"));
        Ok(self.schemas.clone())
    }

    async fn list_tables(&self, database: &str, schema: &str) -> Result<Vec<TableInfo>> {
        self.check_error()?;
        self.record(format!("list_tables {database} {schema}"));
        Ok(self.tables.clone())
    }

    async fn table_structure(&self, table: &TableRef) -> Result<TableStructure> {
        self.check_error()?;
        self.record(format!("table_structure {}", table.name));
        Ok(self.structure.clone())
    }

    async fn run_query(&self, database: &str, sql: &str, max_rows: usize) -> Result<QueryResult> {
        self.check_error()?;
        self.record(format!(
            "run_query {database} max_rows={max_rows} sql={sql}"
        ));
        if let Ok(slot) = self.run_query_error.lock()
            && let Some(message) = slot.as_ref()
        {
            return Err(anyhow!("{message}"));
        }
        if let Ok(mut queue) = self.queued_results.lock()
            && let Some(result) = queue.pop_front()
        {
            return Ok(result);
        }
        Ok(self.query_result.clone())
    }

    async fn apply_edits(
        &self,
        _table: &TableRef,
        _columns: &[ColumnInfo],
        edits: &TableEdits,
    ) -> Result<AppliedCounts> {
        self.check_error()?;
        if let Ok(slot) = self.apply_edits_error.lock()
            && let Some(message) = slot.as_ref()
        {
            return Err(anyhow!("{message}"));
        }
        self.record(format!(
            "apply_edits u={} i={} d={}",
            edits.updates.len(),
            edits.inserts.len(),
            edits.deletes.len()
        ));
        Ok(AppliedCounts {
            updated: edits.updates.len(),
            inserted: edits.inserts.len(),
            deleted: edits.deletes.len(),
        })
    }

    async fn preview_write(
        &self,
        database: &str,
        sql: &str,
        kind: WriteKind,
        _update_target: Option<TableRef>,
        _max_rows: usize,
    ) -> Result<WritePreview> {
        self.check_error()?;
        self.record(format!(
            "preview_write {database} kind={} sql={sql}",
            write_kind_label(kind)
        ));
        if let Ok(slot) = self.preview_write_error.lock()
            && let Some(message) = slot.as_ref()
        {
            return Err(anyhow!("{message}"));
        }
        Ok(self.write_preview.clone())
    }

    async fn commit_write(
        &self,
        database: &str,
        sql: &str,
        expected_rows_affected: Option<u64>,
    ) -> Result<WriteOutcome> {
        self.check_error()?;
        self.record(format!(
            "commit_write {database} sql={sql} expected_rows_affected={expected_rows_affected:?}"
        ));
        if let Ok(slot) = self.commit_write_error.lock()
            && let Some(message) = slot.as_ref()
        {
            return Err(anyhow!("{message}"));
        }
        if let Some(expected) = expected_rows_affected
            && expected != self.write_outcome.rows_affected
        {
            bail!(
                "aborted: rows_affected mismatch (expected {expected}, actual {})",
                self.write_outcome.rows_affected
            );
        }
        Ok(self.write_outcome.clone())
    }

    async fn cancel_running(&self) -> Result<()> {
        self.check_error()?;
        self.record("cancel_running");
        Ok(())
    }
}

/// Renders a [`WriteKind`] for the `preview_write` call-log line.
fn write_kind_label(kind: WriteKind) -> &'static str {
    match kind {
        WriteKind::Insert => "insert",
        WriteKind::Update => "update",
        WriteKind::Delete => "delete",
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[tokio::test]
    async fn fake_client_returns_canned_data_and_logs_calls() {
        let fake = FakeDatabaseClient::new();
        assert_eq!(
            fake.list_databases().await.unwrap(),
            vec!["app", "postgres"]
        );
        let result = fake
            .run_query("app", "SELECT * FROM users", 100)
            .await
            .unwrap();
        assert_eq!(result.rows.len(), 1);
        assert!(
            fake.calls()
                .iter()
                .any(|call| call.starts_with("run_query app"))
        );
    }

    #[tokio::test]
    async fn fake_client_error_mode() {
        let fake = FakeDatabaseClient::with_error("boom");
        let error = fake.list_databases().await.unwrap_err();
        assert!(error.to_string().contains("boom"));
    }

    #[tokio::test]
    async fn fake_apply_edits_records_and_counts() {
        let fake = FakeDatabaseClient::new();
        let table = TableRef {
            database: "app".into(),
            schema: "public".into(),
            name: "users".into(),
        };
        let edits = TableEdits {
            updates: vec![RowUpdate {
                key: RowKey {
                    columns: vec!["id".into()],
                    values: vec![Some("1".into())],
                },
                set: vec![("name".into(), EditCell::Value("Alice2".into()))],
            }],
            inserts: vec![RowInsert {
                values: vec![
                    ("id".into(), EditCell::Value("4".into())),
                    ("name".into(), EditCell::Value("Dave".into())),
                ],
            }],
            deletes: vec![RowDelete {
                key: RowKey {
                    columns: vec!["id".into()],
                    values: vec![Some("2".into())],
                },
            }],
        };
        let counts = fake
            .apply_edits(&table, &fake.structure.columns.clone(), &edits)
            .await
            .unwrap();
        assert_eq!(
            counts,
            AppliedCounts {
                updated: 1,
                inserted: 1,
                deleted: 1,
            }
        );
        assert!(
            fake.calls()
                .iter()
                .any(|call| call == "apply_edits u=1 i=1 d=1")
        );
    }

    #[tokio::test]
    async fn fake_run_query_queue_and_error() {
        let mut fake = FakeDatabaseClient::new();
        fake.query_result = QueryResult {
            columns: vec!["a".into()],
            ..Default::default()
        };
        let fake = Arc::new(fake);
        fake.push_query_result(QueryResult {
            columns: vec!["first".into()],
            ..Default::default()
        });
        fake.push_query_result(QueryResult {
            columns: vec!["second".into()],
            ..Default::default()
        });
        assert_eq!(
            fake.run_query("db", "SELECT 1", 10).await.unwrap().columns,
            vec!["first"]
        );
        assert_eq!(
            fake.run_query("db", "SELECT 1", 10).await.unwrap().columns,
            vec!["second"]
        );
        // queue empty -> falls back to query_result
        assert_eq!(
            fake.run_query("db", "SELECT 1", 10).await.unwrap().columns,
            vec!["a"]
        );
        fake.set_run_query_error(Some("boom".into()));
        assert!(fake.run_query("db", "SELECT 1", 10).await.is_err());
        let table = TableRef {
            database: "app".into(),
            schema: "public".into(),
            name: "users".into(),
        };
        assert!(fake.table_structure(&table).await.is_ok()); // other methods unaffected
    }

    #[tokio::test]
    async fn fake_preview_write_returns_canned_value_and_records_exact_sql() {
        let mut fake = FakeDatabaseClient::new();
        fake.write_preview = WritePreview {
            rows_affected: 2,
            columns: vec!["id".into(), "name".into()],
            before: Some(vec![vec![Some("1".into()), Some("old".into())]]),
            after: Some(vec![vec![Some("1".into()), Some("new".into())]]),
            preview_truncated: false,
            note: None,
        };
        let fake = Arc::new(fake);
        let table = TableRef {
            database: "app".into(),
            schema: "public".into(),
            name: "users".into(),
        };
        let result = fake
            .preview_write(
                "app",
                "UPDATE users SET name = 'new' WHERE id = 1",
                WriteKind::Update,
                Some(table),
                100,
            )
            .await
            .unwrap();
        assert_eq!(result, fake.write_preview);
        assert!(fake.calls().iter().any(|call| call
            == "preview_write app kind=update sql=UPDATE users SET name = 'new' WHERE id = 1"));
    }

    #[tokio::test]
    async fn fake_preview_write_error_knob_is_isolated() {
        let fake = FakeDatabaseClient::new();
        fake.set_preview_write_error(Some("boom".into()));
        let error = fake
            .preview_write(
                "app",
                "DELETE FROM t WHERE id=1",
                WriteKind::Delete,
                None,
                100,
            )
            .await
            .unwrap_err();
        assert!(error.to_string().contains("boom"));
        // Other methods unaffected.
        assert!(fake.list_databases().await.is_ok());
    }

    #[tokio::test]
    async fn fake_commit_write_returns_canned_value_and_records_exact_sql() {
        let mut fake = FakeDatabaseClient::new();
        fake.write_outcome = WriteOutcome {
            rows_affected: 1,
            columns: vec!["id".into()],
            returned: vec![vec![Some("3".into())]],
        };
        let fake = Arc::new(fake);
        let result = fake
            .commit_write("app", "INSERT INTO t (id) VALUES (3)", None)
            .await
            .unwrap();
        assert_eq!(result, fake.write_outcome);
        assert!(fake.calls().iter().any(|call| call
            == "commit_write app sql=INSERT INTO t (id) VALUES (3) expected_rows_affected=None"));
    }

    #[tokio::test]
    async fn fake_commit_write_error_knob_is_isolated() {
        let fake = FakeDatabaseClient::new();
        fake.set_commit_write_error(Some("boom".into()));
        let error = fake
            .commit_write("app", "DELETE FROM t WHERE id=1", None)
            .await
            .unwrap_err();
        assert!(error.to_string().contains("boom"));
        // Other methods unaffected.
        assert!(fake.list_databases().await.is_ok());
    }

    #[tokio::test]
    async fn fake_commit_write_aborts_on_rows_affected_mismatch() {
        let mut fake = FakeDatabaseClient::new();
        fake.write_outcome = WriteOutcome {
            rows_affected: 2,
            columns: vec!["id".into()],
            returned: vec![vec![Some("1".into())], vec![Some("2".into())]],
        };
        let fake = Arc::new(fake);
        let error = fake
            .commit_write("app", "DELETE FROM t WHERE id>0", Some(1))
            .await
            .unwrap_err();
        assert!(error.to_string().contains("aborted"));
        assert!(error.to_string().contains("mismatch"));
        assert!(
            fake.calls()
                .iter()
                .any(|call| call.contains("expected_rows_affected=Some(1)"))
        );
    }

    #[tokio::test]
    async fn fake_commit_write_matching_expected_rows_succeeds() {
        let mut fake = FakeDatabaseClient::new();
        fake.write_outcome = WriteOutcome {
            rows_affected: 1,
            columns: vec!["id".into()],
            returned: vec![vec![Some("1".into())]],
        };
        let fake = Arc::new(fake);
        let result = fake
            .commit_write("app", "DELETE FROM t WHERE id=1", Some(1))
            .await
            .unwrap();
        assert_eq!(result, fake.write_outcome);
    }

    #[tokio::test]
    async fn fake_commit_write_explicit_error_takes_precedence_over_mismatch() {
        let mut fake = FakeDatabaseClient::new();
        fake.write_outcome = WriteOutcome {
            rows_affected: 2,
            columns: vec!["id".into()],
            returned: vec![vec![Some("1".into())], vec![Some("2".into())]],
        };
        fake.set_commit_write_error(Some("explicit boom".into()));
        let fake = Arc::new(fake);
        let error = fake
            .commit_write("app", "DELETE FROM t WHERE id>0", Some(1))
            .await
            .unwrap_err();
        assert!(error.to_string().contains("explicit boom"));
    }
}
