#[cfg(feature = "sqlx-dep")]
pub use sqlx::error::Error as SqlxError;

#[cfg(feature = "sqlx-mysql")]
pub use sqlx::mysql::MySqlDatabaseError as SqlxMySqlError;

#[cfg(feature = "sqlx-postgres")]
pub use sqlx::postgres::PgDatabaseError as SqlxPostgresError;

#[cfg(feature = "sqlx-sqlite")]
pub use sqlx::sqlite::SqliteError as SqlxSqliteError;

use thiserror::Error;

/// An error from unsuccessful database operations
#[derive(Error, Debug)]
pub enum DbErr {
    /// This error can happen when the connection pool is fully-utilized
    #[error("Failed to acquire connection from pool: {0}")]
    ConnectionAcquire(#[source] ConnAcquireErr),
    /// Runtime type conversion error
    #[error("Error converting `{from}` into `{into}`: {source}")]
    TryIntoErr {
        /// From type
        from: &'static str,
        /// Into type
        into: &'static str,
        /// TryError
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    /// There was a problem with the database connection
    #[error("Connection Error: {0}")]
    Conn(#[source] RuntimeErr),
    /// An operation did not execute successfully
    #[error("Execution Error: {0}")]
    Exec(#[source] RuntimeErr),
    /// An error occurred while performing a query
    #[error("Query Error: {0}")]
    Query(#[source] RuntimeErr),
    /// Type error: the specified type cannot be converted from u64. This is not a runtime error.
    #[error("Type '{0}' cannot be converted from u64")]
    ConvertFromU64(&'static str),
    /// After an insert statement it was impossible to retrieve the last_insert_id
    #[error("Failed to unpack last_insert_id")]
    UnpackInsertId,
    /// When updating, a model should know its primary key to check
    /// if the record has been correctly updated, otherwise this error will occur
    #[error("Failed to get primary key from model")]
    UpdateGetPrimaryKey,
    /// The record was not found in the database
    #[error("RecordNotFound Error: {0}")]
    RecordNotFound(String),
    /// Thrown by `TryFrom<ActiveModel>`, which assumes all attributes are set/unchanged
    #[error("Attribute {0} is NotSet")]
    AttrNotSet(String),
    /// A custom error
    #[error("Custom Error: {0}")]
    Custom(String),
    /// Error occurred while parsing value as target type
    #[error("Type Error: {0}")]
    Type(String),
    /// Error occurred while parsing json value as target type
    #[error("Json Error: {0}")]
    Json(String),
    /// A migration error
    #[error("Migration Error: {0}")]
    Migration(String),
    /// None of the records are inserted,
    /// that probably means all of them conflict with existing records in the table
    #[error("None of the records are inserted")]
    RecordNotInserted,
    /// None of the records are updated, that means a WHERE condition has no matches.
    /// May be the table is empty or the record does not exist
    #[error("None of the records are updated")]
    RecordNotUpdated,
}

/// An error from trying to get a row from a Model
#[derive(Debug)]
pub enum TryGetError {
    /// A database error was encountered as defined in [crate::DbErr]
    DbErr(DbErr),
    /// A null value was encountered
    Null(String),
}

/// Connection Acquire error
#[derive(Error, Debug, PartialEq, Eq)]
pub enum ConnAcquireErr {
    /// Connection pool timed out
    #[error("Connection pool timed out")]
    Timeout,
    /// Connection closed
    #[error("Connection closed")]
    ConnectionClosed,
}

/// Runtime error
#[derive(Error, Debug)]
pub enum RuntimeErr {
    /// SQLx Error
    #[cfg(feature = "sqlx-dep")]
    #[error("{0}")]
    SqlxError(#[source] sqlx::error::Error),
    /// Error generated from within SeaORM
    #[error("{0}")]
    Internal(String),
}

impl PartialEq for DbErr {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for DbErr {}

/// Error during `impl FromStr for Entity::Column`
#[derive(Error, Debug)]
#[error("Failed to match \"{0}\" as Column")]
pub struct ColumnFromStrErr(pub String);

#[allow(dead_code)]
pub(crate) fn conn_err<T>(s: T) -> DbErr
where
    T: ToString,
{
    DbErr::Conn(RuntimeErr::Internal(s.to_string()))
}

#[allow(dead_code)]
pub(crate) fn exec_err<T>(s: T) -> DbErr
where
    T: ToString,
{
    DbErr::Exec(RuntimeErr::Internal(s.to_string()))
}

#[allow(dead_code)]
pub(crate) fn query_err<T>(s: T) -> DbErr
where
    T: ToString,
{
    DbErr::Query(RuntimeErr::Internal(s.to_string()))
}

#[allow(dead_code)]
pub(crate) fn type_err<T>(s: T) -> DbErr
where
    T: ToString,
{
    DbErr::Type(s.to_string())
}

#[allow(dead_code)]
pub(crate) fn json_err<T>(s: T) -> DbErr
where
    T: ToString,
{
    DbErr::Json(s.to_string())
}

/// An error from unsuccessful SQL query
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SqlErr {
    /// Error for duplicate record in unique field or primary key field
    #[error("Unique Constraint Violated: {0}")]
    UniqueConstraintViolation(String),
    /// Error for Foreign key constraint
    #[error("Foreign Key Constraint Violated: {0}")]
    ForeignKeyConstraintViolation(String),
}

#[allow(dead_code)]
impl DbErr {
    /// Convert generic DbErr by sqlx to SqlErr, return none if the error is not any type of SqlErr
    pub fn sql_err(&self) -> Option<SqlErr> {
        #[cfg(any(
            feature = "sqlx-mysql",
            feature = "sqlx-postgres",
            feature = "sqlx-sqlite"
        ))]
        {
            use std::ops::Deref;
            if let DbErr::Exec(RuntimeErr::SqlxError(sqlx::Error::Database(e)))
            | DbErr::Query(RuntimeErr::SqlxError(sqlx::Error::Database(e))) = self
            {
                let error_code = e.code().unwrap_or_default();
                let _error_code_expanded = error_code.deref();
                #[cfg(feature = "sqlx-mysql")]
                if e.try_downcast_ref::<sqlx::mysql::MySqlDatabaseError>()
                    .is_some()
                {
                    let error_number = e
                        .try_downcast_ref::<sqlx::mysql::MySqlDatabaseError>()?
                        .number();
                    match error_number {
                        // 1022 Can't write; duplicate key in table '%s'
                        // 1062 Duplicate entry '%s' for key %d
                        // 1169 Can't write, because of unique constraint, to table '%s'
                        // 1586 Duplicate entry '%s' for key '%s'
                        1022 | 1062 | 1169 | 1586 => {
                            return Some(SqlErr::UniqueConstraintViolation(e.message().into()))
                        }
                        // 1216 Cannot add or update a child row: a foreign key constraint fails
                        // 1217 Cannot delete or update a parent row: a foreign key constraint fails
                        // 1451 Cannot delete or update a parent row: a foreign key constraint fails (%s)
                        // 1452 Cannot add or update a child row: a foreign key constraint fails (%s)
                        // 1557 Upholding foreign key constraints for table '%s', entry '%s', key %d would lead to a duplicate entry
                        // 1761 Foreign key constraint for table '%s', record '%s' would lead to a duplicate entry in table '%s', key '%s'
                        // 1762 Foreign key constraint for table '%s', record '%s' would lead to a duplicate entry in a child table
                        1216 | 1217 | 1451 | 1452 | 1557 | 1761 | 1762 => {
                            return Some(SqlErr::ForeignKeyConstraintViolation(e.message().into()))
                        }
                        _ => return None,
                    }
                }
                #[cfg(feature = "sqlx-postgres")]
                if e.try_downcast_ref::<sqlx::postgres::PgDatabaseError>()
                    .is_some()
                {
                    match _error_code_expanded {
                        "23505" => {
                            return Some(SqlErr::UniqueConstraintViolation(e.message().into()))
                        }
                        "23503" => {
                            return Some(SqlErr::ForeignKeyConstraintViolation(e.message().into()))
                        }
                        _ => return None,
                    }
                }
                #[cfg(feature = "sqlx-sqlite")]
                if e.try_downcast_ref::<sqlx::sqlite::SqliteError>().is_some() {
                    match _error_code_expanded {
                        // error code 1555 refers to the primary key's unique constraint violation
                        // error code 2067 refers to the UNIQUE unique constraint violation
                        "1555" | "2067" => {
                            return Some(SqlErr::UniqueConstraintViolation(e.message().into()))
                        }
                        "787" => {
                            return Some(SqlErr::ForeignKeyConstraintViolation(e.message().into()))
                        }
                        _ => return None,
                    }
                }
            }
        }
        None
    }
}
