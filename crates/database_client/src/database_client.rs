pub mod sql;

#[cfg(any(test, feature = "test-support"))]
pub mod fake;
pub mod postgres;

pub use postgres::SessionMode;
pub use sql::quote_ident;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub database: String, // the database to connect to on startup
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
    pub udt_name: String,  // udt_name ("int4", "text", ...) — used to cast filter parameters
    pub udt_schema: String, // udt_schema; schema-qualifies the udt so types outside search_path resolve
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
    pub definition: String, // the full pg_indexes.indexdef
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
    IsNotNull,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Filter {
    pub column: String,
    pub op: FilterOp,
    pub value: String, // ignored for IsNull and IsNotNull
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
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Option<String>>>,
    pub truncated: bool, // true when rows were dropped to respect max_rows
    pub command_tag: Option<String>, // e.g. "SELECT 42"
}

/// A cell value in a row edit. `Value` binds a text parameter cast to the
/// column's type; `Null` emits a literal `NULL` with no parameter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditCell {
    Value(String),
    Null,
}

/// Identifies a row by its primary-key column names and their original values.
/// `Hash` is derived because it is used as a `HashMap`/`HashSet` key when
/// buffering edits.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RowKey {
    pub columns: Vec<String>,
    pub values: Vec<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowUpdate {
    pub key: RowKey,
    pub set: Vec<(String, EditCell)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowInsert {
    pub values: Vec<(String, EditCell)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowDelete {
    pub key: RowKey,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TableEdits {
    pub updates: Vec<RowUpdate>,
    pub inserts: Vec<RowInsert>,
    pub deletes: Vec<RowDelete>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AppliedCounts {
    pub updated: usize,
    pub inserted: usize,
    pub deleted: usize,
}

/// The DML kind of a write statement submitted through the MCP write tools.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteKind {
    Insert,
    Update,
    Delete,
}

/// The result of transactionally previewing a write: the statement was run and
/// then rolled back, so the database is unchanged. See
/// [`DatabaseClient::preview_write`].
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WritePreview {
    pub rows_affected: u64,
    pub columns: Vec<String>,
    // Per spec: Insert -> after=Some, before=None; Delete -> before=Some (the
    // rows that would be deleted), after=None; Update -> after=Some (post-update
    // via RETURNING), before=Some (pre-update via PK) or None with a note.
    pub before: Option<Vec<Vec<Option<String>>>>,
    pub after: Option<Vec<Vec<Option<String>>>>,
    pub preview_truncated: bool,
    pub note: Option<String>,
}

/// The result of committing a write: the statement was run and committed.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WriteOutcome {
    pub rows_affected: u64,
    pub columns: Vec<String>,
    pub returned: Vec<Vec<Option<String>>>,
}

#[async_trait::async_trait]
pub trait DatabaseClient: Send + Sync {
    async fn test_connection(&self) -> Result<()>;
    async fn list_databases(&self) -> Result<Vec<String>>;
    async fn list_schemas(&self, database: &str) -> Result<Vec<String>>;
    async fn list_tables(&self, database: &str, schema: &str) -> Result<Vec<TableInfo>>;
    async fn table_structure(&self, table: &TableRef) -> Result<TableStructure>;
    async fn run_query(&self, database: &str, sql: &str, max_rows: usize) -> Result<QueryResult>;
    /// Applies a batch of row edits (deletes, updates, inserts) in a single
    /// transaction, rolling the whole batch back on any error.
    async fn apply_edits(
        &self,
        table: &TableRef,
        columns: &[ColumnInfo],
        edits: &TableEdits,
    ) -> Result<AppliedCounts>;
    /// Runs a single INSERT/UPDATE/DELETE statement inside a transaction that is
    /// always rolled back, so the database is left unchanged, and returns the
    /// before/after row images that the statement would have produced. Requires
    /// a [`SessionMode::ReadWrite`] session (mirrors [`Self::apply_edits`]'s
    /// guard) even though nothing is persisted, since the statement still runs
    /// against the server.
    async fn preview_write(
        &self,
        database: &str,
        sql: &str,
        kind: WriteKind,
        update_target: Option<TableRef>,
        max_rows: usize,
    ) -> Result<WritePreview>;
    /// Runs a single INSERT/UPDATE/DELETE statement inside a transaction that is
    /// committed on success (or rolled back and the error returned on failure).
    ///
    /// Commits `sql` as a single write. If `expected_rows_affected` is `Some(n)`
    /// and the statement affects a different number of rows, the transaction is
    /// rolled back and an error is returned (the approved preview is stale).
    async fn commit_write(
        &self,
        database: &str,
        sql: &str,
        expected_rows_affected: Option<u64>,
    ) -> Result<WriteOutcome>;
    /// Sends a cancel signal to the server for all in-flight queries of this client.
    async fn cancel_running(&self) -> Result<()>;
}
