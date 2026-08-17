use crate::{error::*, ExecResult, ExecResultHolder, QueryResult, QueryResultRow, Statement};

use sea_query::{Value, ValueType};
use std::{collections::BTreeMap, fmt::Debug};

/// Defines the [ProxyDatabaseTrait] to save the functions
#[async_trait::async_trait]
pub trait ProxyDatabaseTrait: Send + Sync + std::fmt::Debug {
    /// Execute a query in the [ProxyDatabase], and return the query results
    async fn query(&self, statement: Statement) -> Result<Vec<ProxyRow>, DbErr>;

    /// Execute a command in the [ProxyDatabase], and report the number of rows affected
    async fn execute(&self, statement: Statement) -> Result<ProxyExecResult, DbErr>;

    /// Begin a transaction in the [ProxyDatabase]
    async fn begin(&self) {}

    /// Commit a transaction in the [ProxyDatabase]
    async fn commit(&self) {}

    /// Rollback a transaction in the [ProxyDatabase]
    async fn rollback(&self) {}

    /// Ping the [ProxyDatabase], it should return an error if the database is not available
    async fn ping(&self) -> Result<(), DbErr> {
        Ok(())
    }
}

/// Defines the results obtained from a [ProxyDatabase]
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ProxyExecResult {
    /// The last inserted id on auto-increment
    pub last_insert_id: u64,
    /// The number of rows affected by the database operation
    pub rows_affected: u64,
}

impl ProxyExecResult {
    /// Create a new [ProxyExecResult] from the last inserted id and the number of rows affected
    pub fn new(last_insert_id: u64, rows_affected: u64) -> Self {
        Self {
            last_insert_id,
            rows_affected,
        }
    }
}

impl Default for ExecResultHolder {
    fn default() -> Self {
        Self::Proxy(ProxyExecResult::default())
    }
}

impl From<ProxyExecResult> for ExecResult {
    fn from(result: ProxyExecResult) -> Self {
        Self {
            result: ExecResultHolder::Proxy(result),
        }
    }
}

impl From<ExecResult> for ProxyExecResult {
    fn from(result: ExecResult) -> Self {
        match result.result {
            #[cfg(feature = "sqlx-mysql")]
            ExecResultHolder::SqlxMySql(result) => Self {
                last_insert_id: result.last_insert_id() as u64,
                rows_affected: result.rows_affected(),
            },
            #[cfg(feature = "sqlx-postgres")]
            ExecResultHolder::SqlxPostgres(result) => Self {
                last_insert_id: 0,
                rows_affected: result.rows_affected(),
            },
            #[cfg(feature = "sqlx-sqlite")]
            ExecResultHolder::SqlxSqlite(result) => Self {
                last_insert_id: result.last_insert_rowid() as u64,
                rows_affected: result.rows_affected(),
            },
            #[cfg(feature = "mock")]
            ExecResultHolder::Mock(result) => Self {
                last_insert_id: result.last_insert_id,
                rows_affected: result.rows_affected,
            },
            ExecResultHolder::Proxy(result) => result,
        }
    }
}

/// Defines the structure of a Row for the [ProxyDatabase]
/// which is just a [BTreeMap]<[String], [Value]>
#[derive(Clone, Debug)]
pub struct ProxyRow {
    /// The values of the single row
    pub values: BTreeMap<String, Value>,
}

impl ProxyRow {
    /// Create a new [ProxyRow] from a [BTreeMap]<[String], [Value]>
    pub fn new(values: BTreeMap<String, Value>) -> Self {
        Self { values }
    }
}

impl Default for ProxyRow {
    fn default() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }
}

impl From<BTreeMap<String, Value>> for ProxyRow {
    fn from(values: BTreeMap<String, Value>) -> Self {
        Self { values }
    }
}

impl From<ProxyRow> for BTreeMap<String, Value> {
    fn from(row: ProxyRow) -> Self {
        row.values
    }
}

impl From<ProxyRow> for Vec<(String, Value)> {
    fn from(row: ProxyRow) -> Self {
        row.values.into_iter().collect()
    }
}

impl From<ProxyRow> for QueryResult {
    fn from(row: ProxyRow) -> Self {
        QueryResult {
            row: QueryResultRow::Proxy(row),
        }
    }
}

#[cfg(feature = "with-json")]
impl Into<serde_json::Value> for ProxyRow {
    fn into(self) -> serde_json::Value {
        self.values
            .into_iter()
            .map(|(k, v)| (k, sea_query::sea_value_to_json_value(&v)))
            .collect()
    }
}

/// Convert [QueryResult] to [ProxyRow]
pub fn from_query_result_to_proxy_row(result: &QueryResult) -> ProxyRow {
    match &result.row {
        #[cfg(feature = "sqlx-mysql")]
        QueryResultRow::SqlxMySql(row) => crate::from_sqlx_mysql_row_to_proxy_row(&row),
        #[cfg(feature = "sqlx-postgres")]
        QueryResultRow::SqlxPostgres(row) => crate::from_sqlx_postgres_row_to_proxy_row(&row),
        #[cfg(feature = "sqlx-sqlite")]
        QueryResultRow::SqlxSqlite(row) => crate::from_sqlx_sqlite_row_to_proxy_row(&row),
        #[cfg(feature = "mock")]
        QueryResultRow::Mock(row) => ProxyRow {
            values: row.values.clone(),
        },
        QueryResultRow::Proxy(row) => row.to_owned(),
    }
}

impl ProxyRow {
    /// Get a value from the [ProxyRow]
    pub fn try_get<T, I: crate::ColIdx>(&self, index: I) -> Result<T, DbErr>
    where
        T: ValueType,
    {
        if let Some(index) = index.as_str() {
            T::try_from(
                self.values
                    .get(index)
                    .ok_or_else(|| query_err(format!("No column for ColIdx {index:?}")))?
                    .clone(),
            )
            .map_err(type_err)
        } else if let Some(index) = index.as_usize() {
            let (_, value) = self
                .values
                .iter()
                .nth(*index)
                .ok_or_else(|| query_err(format!("Column at index {index} not found")))?;
            T::try_from(value.clone()).map_err(type_err)
        } else {
            unreachable!("Missing ColIdx implementation for ProxyRow");
        }
    }

    /// An iterator over the keys and values of a proxy row
    pub fn into_column_value_tuples(self) -> impl Iterator<Item = (String, Value)> {
        self.values.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        entity::*, tests_cfg::*, Database, DbBackend, DbErr, ProxyDatabaseTrait, ProxyExecResult,
        ProxyRow, Statement,
    };
    use std::sync::{Arc, Mutex};

    #[derive(Debug)]
    struct ProxyDb {}

    impl ProxyDatabaseTrait for ProxyDb {
        async fn query(&self, statement: Statement) -> Result<Vec<ProxyRow>, DbErr> {
            println!("SQL query: {}", statement.sql);
            Ok(vec![].into())
        }

        async fn execute(&self, statement: Statement) -> Result<ProxyExecResult, DbErr> {
            println!("SQL execute: {}", statement.sql);
            Ok(ProxyExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            })
        }
    }

    #[smol_potat::test]
    async fn create_proxy_conn() {
        let _db = Database::connect_proxy(DbBackend::MySql, Arc::new(Box::new(ProxyDb {})))
            .await
            .unwrap();
    }

    #[smol_potat::test]
    async fn select_rows() {
        let db = Database::connect_proxy(DbBackend::MySql, Arc::new(Box::new(ProxyDb {})))
            .await
            .unwrap();

        let _ = cake::Entity::find().all(&db).await;
    }

    #[smol_potat::test]
    async fn insert_one_row() {
        let db = Database::connect_proxy(DbBackend::MySql, Arc::new(Box::new(ProxyDb {})))
            .await
            .unwrap();

        let item = cake::ActiveModel {
            id: NotSet,
            name: Set("Alice".to_string()),
        };

        cake::Entity::insert(item).exec(&db).await.unwrap();
    }
}
