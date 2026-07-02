pub mod sql;

#[cfg(any(test, feature = "test-support"))]
pub mod fake;
pub mod postgres;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub database: String, // стартовая база
    pub user: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableRef {
    pub database: String,
    pub schema: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableInfo {
    pub name: String,
    pub is_view: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String, // information_schema.columns.data_type ("integer", "text", ...)
    pub udt_name: String,  // udt_name ("int4", "text", ...) — для кастов параметров
    pub is_nullable: bool,
    pub default: Option<String>,
    pub is_primary_key: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignKey {
    pub column: String,
    pub references_schema: String,
    pub references_table: String,
    pub references_column: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexInfo {
    pub name: String,
    pub definition: String, // pg_indexes.indexdef целиком
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TableStructure {
    pub columns: Vec<ColumnInfo>,
    pub foreign_keys: Vec<ForeignKey>,
    pub indexes: Vec<IndexInfo>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterOp {
    Eq,
    NotEq,
    Gt,
    Lt,
    Contains,
    IsNull,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Filter {
    pub column: String,
    pub op: FilterOp,
    pub value: String, // игнорируется для IsNull
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sort {
    pub column: String,
    pub direction: SortDirection,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SelectSpec {
    pub filters: Vec<Filter>,
    pub sort: Option<Sort>,
    pub limit: usize,
    pub offset: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RowsPage {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Option<String>>>, // все значения текстом; None = NULL
    pub has_more: bool,                 // запрашивали limit+1
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Option<String>>>,
    pub truncated: bool,             // обрезано по max_rows
    pub command_tag: Option<String>, // например "SELECT 42"
}

#[async_trait::async_trait]
pub trait DatabaseClient: Send + Sync {
    async fn test_connection(&self) -> Result<()>;
    async fn list_databases(&self) -> Result<Vec<String>>;
    async fn list_schemas(&self, database: &str) -> Result<Vec<String>>;
    async fn list_tables(&self, database: &str, schema: &str) -> Result<Vec<TableInfo>>;
    async fn table_structure(&self, table: &TableRef) -> Result<TableStructure>;
    async fn fetch_rows(&self, table: &TableRef, spec: &SelectSpec) -> Result<RowsPage>;
    async fn run_query(&self, database: &str, sql: &str, max_rows: usize) -> Result<QueryResult>;
    /// Sends a cancel signal to the server for all in-flight queries of this client.
    async fn cancel_running(&self) -> Result<()>;
}
