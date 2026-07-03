use std::sync::Mutex;

use anyhow::{Result, anyhow};

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
    pub page: RowsPage,
    pub query_result: QueryResult,
    pub error: Option<String>,
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
        let page = RowsPage {
            columns: vec!["id".into(), "name".into()],
            rows: vec![
                vec![Some("1".into()), Some("Alice".into())],
                vec![Some("2".into()), Some("Bob".into())],
                vec![Some("3".into()), None],
            ],
            has_more: true,
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
            page,
            query_result,
            error: None,
            calls: Mutex::new(Vec::new()),
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

    async fn fetch_rows(&self, table: &TableRef, spec: &SelectSpec) -> Result<RowsPage> {
        self.check_error()?;
        self.record(format!(
            "fetch_rows {} limit={} offset={} sort={:?} filters={}",
            table.name,
            spec.limit,
            spec.offset,
            spec.sort.as_ref().map(|sort| &sort.column),
            spec.filters.len()
        ));
        Ok(self.page.clone())
    }

    async fn run_query(&self, database: &str, sql: &str, max_rows: usize) -> Result<QueryResult> {
        self.check_error()?;
        self.record(format!(
            "run_query {database} max_rows={max_rows} sql={sql}"
        ));
        Ok(self.query_result.clone())
    }

    async fn cancel_running(&self) -> Result<()> {
        self.check_error()?;
        self.record("cancel_running");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fake_client_returns_canned_data_and_logs_calls() {
        let fake = FakeDatabaseClient::new();
        assert_eq!(
            fake.list_databases().await.unwrap(),
            vec!["app", "postgres"]
        );
        let spec = SelectSpec {
            limit: 100,
            ..Default::default()
        };
        let table = TableRef {
            database: "app".into(),
            schema: "public".into(),
            name: "users".into(),
        };
        let page = fake.fetch_rows(&table, &spec).await.unwrap();
        assert_eq!(page.rows.len(), 3);
        assert!(
            fake.calls()
                .iter()
                .any(|call| call.starts_with("fetch_rows users"))
        );
    }

    #[tokio::test]
    async fn fake_client_error_mode() {
        let fake = FakeDatabaseClient::with_error("boom");
        let error = fake.list_databases().await.unwrap_err();
        assert!(error.to_string().contains("boom"));
    }
}
