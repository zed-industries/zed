/// Defines the result of executing an operation
#[derive(Debug)]
pub struct ExecResult {
    /// The type of result from the execution depending on the feature flag enabled
    /// to choose a database backend
    pub(crate) result: ExecResultHolder,
}

/// Holds a result depending on the database backend chosen by the feature flag
#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub(crate) enum ExecResultHolder {
    /// Holds the result of executing an operation on a MySQL database
    #[cfg(feature = "sqlx-mysql")]
    SqlxMySql(sqlx::mysql::MySqlQueryResult),
    /// Holds the result of executing an operation on a PostgreSQL database
    #[cfg(feature = "sqlx-postgres")]
    SqlxPostgres(sqlx::postgres::PgQueryResult),
    /// Holds the result of executing an operation on a SQLite database
    #[cfg(feature = "sqlx-sqlite")]
    SqlxSqlite(sqlx::sqlite::SqliteQueryResult),
    /// Holds the result of executing an operation on the Mock database
    #[cfg(feature = "mock")]
    Mock(crate::MockExecResult),
    /// Holds the result of executing an operation on the Proxy database
    #[cfg(feature = "proxy")]
    Proxy(crate::ProxyExecResult),
}

// ExecResult //

impl ExecResult {
    /// Get the last id after `AUTOINCREMENT` is done on the primary key
    ///
    /// # Panics
    ///
    /// Postgres does not support retrieving last insert id this way except through `RETURNING` clause
    pub fn last_insert_id(&self) -> u64 {
        match &self.result {
            #[cfg(feature = "sqlx-mysql")]
            ExecResultHolder::SqlxMySql(result) => result.last_insert_id(),
            #[cfg(feature = "sqlx-postgres")]
            ExecResultHolder::SqlxPostgres(_) => {
                panic!("Should not retrieve last_insert_id this way")
            }
            #[cfg(feature = "sqlx-sqlite")]
            ExecResultHolder::SqlxSqlite(result) => {
                let last_insert_rowid = result.last_insert_rowid();
                if last_insert_rowid < 0 {
                    unreachable!("negative last_insert_rowid")
                } else {
                    last_insert_rowid as u64
                }
            }
            #[cfg(feature = "mock")]
            ExecResultHolder::Mock(result) => result.last_insert_id,
            #[cfg(feature = "proxy")]
            ExecResultHolder::Proxy(result) => result.last_insert_id,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }

    /// Get the number of rows affected by the operation
    pub fn rows_affected(&self) -> u64 {
        match &self.result {
            #[cfg(feature = "sqlx-mysql")]
            ExecResultHolder::SqlxMySql(result) => result.rows_affected(),
            #[cfg(feature = "sqlx-postgres")]
            ExecResultHolder::SqlxPostgres(result) => result.rows_affected(),
            #[cfg(feature = "sqlx-sqlite")]
            ExecResultHolder::SqlxSqlite(result) => result.rows_affected(),
            #[cfg(feature = "mock")]
            ExecResultHolder::Mock(result) => result.rows_affected,
            #[cfg(feature = "proxy")]
            ExecResultHolder::Proxy(result) => result.rows_affected,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
