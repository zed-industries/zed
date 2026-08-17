pub use crate::error::TryGetError;
use crate::{
    error::{json_err, type_err, DbErr},
    SelectGetableValue, SelectorRaw, Statement,
};
use std::fmt;

#[cfg(any(feature = "mock", feature = "proxy"))]
use crate::debug_print;

#[cfg(feature = "sqlx-dep")]
use crate::driver::*;
#[cfg(feature = "sqlx-dep")]
use sqlx::Row;

/// Defines the result of a query operation on a Model
#[derive(Debug)]
pub struct QueryResult {
    pub(crate) row: QueryResultRow,
}

#[allow(clippy::enum_variant_names)]
pub(crate) enum QueryResultRow {
    #[cfg(feature = "sqlx-mysql")]
    SqlxMySql(sqlx::mysql::MySqlRow),
    #[cfg(feature = "sqlx-postgres")]
    SqlxPostgres(sqlx::postgres::PgRow),
    #[cfg(feature = "sqlx-sqlite")]
    SqlxSqlite(sqlx::sqlite::SqliteRow),
    #[cfg(feature = "mock")]
    Mock(crate::MockRow),
    #[cfg(feature = "proxy")]
    Proxy(crate::ProxyRow),
}

/// An interface to get a value from the query result
pub trait TryGetable: Sized {
    /// Get a value from the query result with an ColIdx
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError>;

    /// Get a value from the query result with prefixed column name
    fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
        if pre.is_empty() {
            Self::try_get_by(res, col)
        } else {
            Self::try_get_by(res, format!("{pre}{col}").as_str())
        }
    }

    /// Get a value from the query result based on the order in the select expressions
    fn try_get_by_index(res: &QueryResult, index: usize) -> Result<Self, TryGetError> {
        Self::try_get_by(res, index)
    }
}

impl From<TryGetError> for DbErr {
    fn from(e: TryGetError) -> DbErr {
        match e {
            TryGetError::DbErr(e) => e,
            TryGetError::Null(s) => {
                type_err(format!("A null value was encountered while decoding {s}"))
            }
        }
    }
}

impl From<DbErr> for TryGetError {
    fn from(e: DbErr) -> TryGetError {
        Self::DbErr(e)
    }
}

// QueryResult //

impl QueryResult {
    /// Get a value from the query result with an ColIdx
    pub fn try_get_by<T, I>(&self, index: I) -> Result<T, DbErr>
    where
        T: TryGetable,
        I: ColIdx,
    {
        Ok(T::try_get_by(self, index)?)
    }

    /// Get a value from the query result with an ColIdx
    pub fn try_get_by_nullable<T, I>(&self, index: I) -> Result<T, TryGetError>
    where
        T: TryGetable,
        I: ColIdx,
    {
        T::try_get_by(self, index)
    }

    /// Get a value from the query result with prefixed column name
    pub fn try_get<T>(&self, pre: &str, col: &str) -> Result<T, DbErr>
    where
        T: TryGetable,
    {
        Ok(T::try_get(self, pre, col)?)
    }

    /// Get a value from the query result with prefixed column name
    pub fn try_get_nullable<T>(&self, pre: &str, col: &str) -> Result<T, TryGetError>
    where
        T: TryGetable,
    {
        T::try_get(self, pre, col)
    }

    /// Get a value from the query result based on the order in the select expressions
    pub fn try_get_by_index<T>(&self, idx: usize) -> Result<T, DbErr>
    where
        T: TryGetable,
    {
        Ok(T::try_get_by_index(self, idx)?)
    }

    /// Get a value from the query result based on the order in the select expressions
    pub fn try_get_by_index_nullable<T>(&self, idx: usize) -> Result<T, TryGetError>
    where
        T: TryGetable,
    {
        T::try_get_by_index(self, idx)
    }

    /// Get a tuple value from the query result with prefixed column name
    pub fn try_get_many<T>(&self, pre: &str, cols: &[String]) -> Result<T, DbErr>
    where
        T: TryGetableMany,
    {
        Ok(T::try_get_many(self, pre, cols)?)
    }

    /// Get a tuple value from the query result based on the order in the select expressions
    pub fn try_get_many_by_index<T>(&self) -> Result<T, DbErr>
    where
        T: TryGetableMany,
    {
        Ok(T::try_get_many_by_index(self)?)
    }

    /// Retrieves the names of the columns in the result set
    pub fn column_names(&self) -> Vec<String> {
        #[cfg(feature = "sqlx-dep")]
        use sqlx::Column;

        match &self.row {
            #[cfg(feature = "sqlx-mysql")]
            QueryResultRow::SqlxMySql(row) => {
                row.columns().iter().map(|c| c.name().to_string()).collect()
            }
            #[cfg(feature = "sqlx-postgres")]
            QueryResultRow::SqlxPostgres(row) => {
                row.columns().iter().map(|c| c.name().to_string()).collect()
            }
            #[cfg(feature = "sqlx-sqlite")]
            QueryResultRow::SqlxSqlite(row) => {
                row.columns().iter().map(|c| c.name().to_string()).collect()
            }
            #[cfg(feature = "mock")]
            QueryResultRow::Mock(row) => row
                .clone()
                .into_column_value_tuples()
                .map(|(c, _)| c.to_string())
                .collect(),
            #[cfg(feature = "proxy")]
            QueryResultRow::Proxy(row) => row
                .clone()
                .into_column_value_tuples()
                .map(|(c, _)| c.to_string())
                .collect(),
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }

    /// Access the underlying `MySqlRow` if we use the MySQL backend.
    #[cfg(feature = "sqlx-mysql")]
    pub fn try_as_mysql_row(&self) -> Option<&sqlx::mysql::MySqlRow> {
        match &self.row {
            QueryResultRow::SqlxMySql(mysql_row) => Some(mysql_row),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }

    /// Access the underlying `PgRow` if we use the Postgres backend.
    #[cfg(feature = "sqlx-postgres")]
    pub fn try_as_pg_row(&self) -> Option<&sqlx::postgres::PgRow> {
        match &self.row {
            QueryResultRow::SqlxPostgres(pg_row) => Some(pg_row),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }

    /// Access the underlying `SqliteRow` if we use the SQLite backend.
    #[cfg(feature = "sqlx-sqlite")]
    pub fn try_as_sqlite_row(&self) -> Option<&sqlx::sqlite::SqliteRow> {
        match &self.row {
            QueryResultRow::SqlxSqlite(sqlite_row) => Some(sqlite_row),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }

    /// Access the underlying `MockRow` if we use a mock.
    #[cfg(feature = "mock")]
    pub fn try_as_mock_row(&self) -> Option<&crate::MockRow> {
        match &self.row {
            QueryResultRow::Mock(mock_row) => Some(mock_row),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }

    /// Access the underlying `ProxyRow` if we use a proxy.
    #[cfg(feature = "proxy")]
    pub fn try_as_proxy_row(&self) -> Option<&crate::ProxyRow> {
        match &self.row {
            QueryResultRow::Proxy(proxy_row) => Some(proxy_row),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
}

#[allow(unused_variables)]
impl fmt::Debug for QueryResultRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            #[cfg(feature = "sqlx-mysql")]
            Self::SqlxMySql(row) => write!(f, "{row:?}"),
            #[cfg(feature = "sqlx-postgres")]
            Self::SqlxPostgres(_) => write!(f, "QueryResultRow::SqlxPostgres cannot be inspected"),
            #[cfg(feature = "sqlx-sqlite")]
            Self::SqlxSqlite(_) => write!(f, "QueryResultRow::SqlxSqlite cannot be inspected"),
            #[cfg(feature = "mock")]
            Self::Mock(row) => write!(f, "{row:?}"),
            #[cfg(feature = "proxy")]
            Self::Proxy(row) => write!(f, "{row:?}"),
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}

// TryGetable //

impl<T: TryGetable> TryGetable for Option<T> {
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        match T::try_get_by(res, index) {
            Ok(v) => Ok(Some(v)),
            Err(TryGetError::Null(_)) => Ok(None),
            #[cfg(feature = "sqlx-dep")]
            Err(TryGetError::DbErr(DbErr::Query(crate::RuntimeErr::SqlxError(
                sqlx::Error::ColumnNotFound(_),
            )))) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

/// Column Index, used by [`TryGetable`]. Implemented for `&str` and `usize`
pub trait ColIdx: std::fmt::Debug + Copy {
    #[cfg(feature = "sqlx-mysql")]
    /// Type surrogate
    type SqlxMySqlIndex: sqlx::ColumnIndex<sqlx::mysql::MySqlRow>;
    #[cfg(feature = "sqlx-postgres")]
    /// Type surrogate
    type SqlxPostgresIndex: sqlx::ColumnIndex<sqlx::postgres::PgRow>;
    #[cfg(feature = "sqlx-sqlite")]
    /// Type surrogate
    type SqlxSqliteIndex: sqlx::ColumnIndex<sqlx::sqlite::SqliteRow>;

    #[cfg(feature = "sqlx-mysql")]
    /// Basically a no-op; only to satisfy trait bounds
    fn as_sqlx_mysql_index(&self) -> Self::SqlxMySqlIndex;
    #[cfg(feature = "sqlx-postgres")]
    /// Basically a no-op; only to satisfy trait bounds
    fn as_sqlx_postgres_index(&self) -> Self::SqlxPostgresIndex;
    #[cfg(feature = "sqlx-sqlite")]
    /// Basically a no-op; only to satisfy trait bounds
    fn as_sqlx_sqlite_index(&self) -> Self::SqlxSqliteIndex;

    /// Self must be `&str`, return `None` otherwise
    fn as_str(&self) -> Option<&str>;
    /// Self must be `usize`, return `None` otherwise
    fn as_usize(&self) -> Option<&usize>;
}

impl ColIdx for &str {
    #[cfg(feature = "sqlx-mysql")]
    type SqlxMySqlIndex = Self;
    #[cfg(feature = "sqlx-postgres")]
    type SqlxPostgresIndex = Self;
    #[cfg(feature = "sqlx-sqlite")]
    type SqlxSqliteIndex = Self;

    #[cfg(feature = "sqlx-mysql")]
    #[inline]
    fn as_sqlx_mysql_index(&self) -> Self::SqlxMySqlIndex {
        self
    }
    #[cfg(feature = "sqlx-postgres")]
    #[inline]
    fn as_sqlx_postgres_index(&self) -> Self::SqlxPostgresIndex {
        self
    }
    #[cfg(feature = "sqlx-sqlite")]
    #[inline]
    fn as_sqlx_sqlite_index(&self) -> Self::SqlxSqliteIndex {
        self
    }

    #[inline]
    fn as_str(&self) -> Option<&str> {
        Some(self)
    }
    #[inline]
    fn as_usize(&self) -> Option<&usize> {
        None
    }
}

impl ColIdx for usize {
    #[cfg(feature = "sqlx-mysql")]
    type SqlxMySqlIndex = Self;
    #[cfg(feature = "sqlx-postgres")]
    type SqlxPostgresIndex = Self;
    #[cfg(feature = "sqlx-sqlite")]
    type SqlxSqliteIndex = Self;

    #[cfg(feature = "sqlx-mysql")]
    #[inline]
    fn as_sqlx_mysql_index(&self) -> Self::SqlxMySqlIndex {
        *self
    }
    #[cfg(feature = "sqlx-postgres")]
    #[inline]
    fn as_sqlx_postgres_index(&self) -> Self::SqlxPostgresIndex {
        *self
    }
    #[cfg(feature = "sqlx-sqlite")]
    #[inline]
    fn as_sqlx_sqlite_index(&self) -> Self::SqlxSqliteIndex {
        *self
    }

    #[inline]
    fn as_str(&self) -> Option<&str> {
        None
    }
    #[inline]
    fn as_usize(&self) -> Option<&usize> {
        Some(self)
    }
}

macro_rules! try_getable_all {
    ( $type: ty ) => {
        impl TryGetable for $type {
            #[allow(unused_variables)]
            fn try_get_by<I: ColIdx>(res: &QueryResult, idx: I) -> Result<Self, TryGetError> {
                match &res.row {
                    #[cfg(feature = "sqlx-mysql")]
                    QueryResultRow::SqlxMySql(row) => row
                        .try_get::<Option<$type>, _>(idx.as_sqlx_mysql_index())
                        .map_err(|e| sqlx_error_to_query_err(e).into())
                        .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx))),
                    #[cfg(feature = "sqlx-postgres")]
                    QueryResultRow::SqlxPostgres(row) => row
                        .try_get::<Option<$type>, _>(idx.as_sqlx_postgres_index())
                        .map_err(|e| sqlx_error_to_query_err(e).into())
                        .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx))),
                    #[cfg(feature = "sqlx-sqlite")]
                    QueryResultRow::SqlxSqlite(row) => row
                        .try_get::<Option<$type>, _>(idx.as_sqlx_sqlite_index())
                        .map_err(|e| sqlx_error_to_query_err(e).into())
                        .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx))),
                    #[cfg(feature = "mock")]
                    QueryResultRow::Mock(row) => row.try_get(idx).map_err(|e| {
                        debug_print!("{:#?}", e.to_string());
                        err_null_idx_col(idx)
                    }),
                    #[cfg(feature = "proxy")]
                    QueryResultRow::Proxy(row) => row.try_get(idx).map_err(|e| {
                        debug_print!("{:#?}", e.to_string());
                        err_null_idx_col(idx)
                    }),
                    #[allow(unreachable_patterns)]
                    _ => unreachable!(),
                }
            }
        }
    };
}

macro_rules! try_getable_unsigned {
    ( $type: ty ) => {
        impl TryGetable for $type {
            #[allow(unused_variables)]
            fn try_get_by<I: ColIdx>(res: &QueryResult, idx: I) -> Result<Self, TryGetError> {
                match &res.row {
                    #[cfg(feature = "sqlx-mysql")]
                    QueryResultRow::SqlxMySql(row) => row
                        .try_get::<Option<$type>, _>(idx.as_sqlx_mysql_index())
                        .map_err(|e| sqlx_error_to_query_err(e).into())
                        .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx))),
                    #[cfg(feature = "sqlx-postgres")]
                    QueryResultRow::SqlxPostgres(_) => Err(type_err(format!(
                        "{} unsupported by sqlx-postgres",
                        stringify!($type)
                    ))
                    .into()),
                    #[cfg(feature = "sqlx-sqlite")]
                    QueryResultRow::SqlxSqlite(row) => row
                        .try_get::<Option<$type>, _>(idx.as_sqlx_sqlite_index())
                        .map_err(|e| sqlx_error_to_query_err(e).into())
                        .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx))),
                    #[cfg(feature = "mock")]
                    QueryResultRow::Mock(row) => row.try_get(idx).map_err(|e| {
                        debug_print!("{:#?}", e.to_string());
                        err_null_idx_col(idx)
                    }),
                    #[cfg(feature = "proxy")]
                    QueryResultRow::Proxy(row) => row.try_get(idx).map_err(|e| {
                        debug_print!("{:#?}", e.to_string());
                        err_null_idx_col(idx)
                    }),
                    #[allow(unreachable_patterns)]
                    _ => unreachable!(),
                }
            }
        }
    };
}

macro_rules! try_getable_mysql {
    ( $type: ty ) => {
        impl TryGetable for $type {
            #[allow(unused_variables)]
            fn try_get_by<I: ColIdx>(res: &QueryResult, idx: I) -> Result<Self, TryGetError> {
                match &res.row {
                    #[cfg(feature = "sqlx-mysql")]
                    QueryResultRow::SqlxMySql(row) => row
                        .try_get::<Option<$type>, _>(idx.as_sqlx_mysql_index())
                        .map_err(|e| sqlx_error_to_query_err(e).into())
                        .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx))),
                    #[cfg(feature = "sqlx-postgres")]
                    QueryResultRow::SqlxPostgres(_) => Err(type_err(format!(
                        "{} unsupported by sqlx-postgres",
                        stringify!($type)
                    ))
                    .into()),
                    #[cfg(feature = "sqlx-sqlite")]
                    QueryResultRow::SqlxSqlite(_) => Err(type_err(format!(
                        "{} unsupported by sqlx-sqlite",
                        stringify!($type)
                    ))
                    .into()),
                    #[cfg(feature = "mock")]
                    QueryResultRow::Mock(row) => row.try_get(idx).map_err(|e| {
                        debug_print!("{:#?}", e.to_string());
                        err_null_idx_col(idx)
                    }),
                    #[cfg(feature = "proxy")]
                    QueryResultRow::Proxy(row) => row.try_get(idx).map_err(|e| {
                        debug_print!("{:#?}", e.to_string());
                        err_null_idx_col(idx)
                    }),
                    #[allow(unreachable_patterns)]
                    _ => unreachable!(),
                }
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! try_getable_postgres {
    ( $type: ty ) => {
        impl TryGetable for $type {
            #[allow(unused_variables)]
            fn try_get_by<I: ColIdx>(res: &QueryResult, idx: I) -> Result<Self, TryGetError> {
                match &res.row {
                    #[cfg(feature = "sqlx-mysql")]
                    QueryResultRow::SqlxMySql(_) => Err(type_err(format!(
                        "{} unsupported by sqlx-mysql",
                        stringify!($type)
                    ))
                    .into()),
                    #[cfg(feature = "sqlx-postgres")]
                    QueryResultRow::SqlxPostgres(row) => row
                        .try_get::<Option<$type>, _>(idx.as_sqlx_postgres_index())
                        .map_err(|e| sqlx_error_to_query_err(e).into())
                        .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx))),
                    #[cfg(feature = "sqlx-sqlite")]
                    QueryResultRow::SqlxSqlite(_) => Err(type_err(format!(
                        "{} unsupported by sqlx-sqlite",
                        stringify!($type)
                    ))
                    .into()),
                    #[cfg(feature = "mock")]
                    QueryResultRow::Mock(row) => row.try_get(idx).map_err(|e| {
                        debug_print!("{:#?}", e.to_string());
                        err_null_idx_col(idx)
                    }),
                    #[cfg(feature = "proxy")]
                    QueryResultRow::Proxy(row) => row.try_get(idx).map_err(|e| {
                        debug_print!("{:#?}", e.to_string());
                        err_null_idx_col(idx)
                    }),
                    #[allow(unreachable_patterns)]
                    _ => unreachable!(),
                }
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! try_getable_date_time {
    ( $type: ty ) => {
        impl TryGetable for $type {
            #[allow(unused_variables)]
            fn try_get_by<I: ColIdx>(res: &QueryResult, idx: I) -> Result<Self, TryGetError> {
                match &res.row {
                    #[cfg(feature = "sqlx-mysql")]
                    QueryResultRow::SqlxMySql(row) => {
                        use chrono::{DateTime, Utc};
                        row.try_get::<Option<DateTime<Utc>>, _>(idx.as_sqlx_mysql_index())
                            .map_err(|e| sqlx_error_to_query_err(e).into())
                            .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx)))
                            .map(|v| v.into())
                    }
                    #[cfg(feature = "sqlx-postgres")]
                    QueryResultRow::SqlxPostgres(row) => row
                        .try_get::<Option<$type>, _>(idx.as_sqlx_postgres_index())
                        .map_err(|e| sqlx_error_to_query_err(e).into())
                        .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx))),
                    #[cfg(feature = "sqlx-sqlite")]
                    QueryResultRow::SqlxSqlite(row) => {
                        use chrono::{DateTime, Utc};
                        row.try_get::<Option<DateTime<Utc>>, _>(idx.as_sqlx_sqlite_index())
                            .map_err(|e| sqlx_error_to_query_err(e).into())
                            .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx)))
                            .map(|v| v.into())
                    }
                    #[cfg(feature = "mock")]
                    QueryResultRow::Mock(row) => row.try_get(idx).map_err(|e| {
                        debug_print!("{:#?}", e.to_string());
                        err_null_idx_col(idx)
                    }),
                    #[cfg(feature = "proxy")]
                    QueryResultRow::Proxy(row) => row.try_get(idx).map_err(|e| {
                        debug_print!("{:#?}", e.to_string());
                        err_null_idx_col(idx)
                    }),
                    #[allow(unreachable_patterns)]
                    _ => unreachable!(),
                }
            }
        }
    };
}

try_getable_all!(bool);
try_getable_all!(i8);
try_getable_all!(i16);
try_getable_all!(i32);
try_getable_all!(i64);
try_getable_unsigned!(u8);
try_getable_unsigned!(u16);
try_getable_mysql!(u64);
try_getable_all!(f32);
try_getable_all!(f64);
try_getable_all!(Vec<u8>);

#[cfg(feature = "with-json")]
try_getable_all!(serde_json::Value);

#[cfg(feature = "with-chrono")]
try_getable_all!(chrono::NaiveDate);

#[cfg(feature = "with-chrono")]
try_getable_all!(chrono::NaiveTime);

#[cfg(feature = "with-chrono")]
try_getable_all!(chrono::NaiveDateTime);

#[cfg(feature = "with-chrono")]
try_getable_date_time!(chrono::DateTime<chrono::FixedOffset>);

#[cfg(feature = "with-chrono")]
try_getable_all!(chrono::DateTime<chrono::Utc>);

#[cfg(feature = "with-chrono")]
try_getable_all!(chrono::DateTime<chrono::Local>);

#[cfg(feature = "with-time")]
try_getable_all!(time::Date);

#[cfg(feature = "with-time")]
try_getable_all!(time::Time);

#[cfg(feature = "with-time")]
try_getable_all!(time::PrimitiveDateTime);

#[cfg(feature = "with-time")]
try_getable_all!(time::OffsetDateTime);

#[cfg(feature = "with-rust_decimal")]
use rust_decimal::Decimal;

#[cfg(feature = "with-rust_decimal")]
impl TryGetable for Decimal {
    #[allow(unused_variables)]
    fn try_get_by<I: ColIdx>(res: &QueryResult, idx: I) -> Result<Self, TryGetError> {
        match &res.row {
            #[cfg(feature = "sqlx-mysql")]
            QueryResultRow::SqlxMySql(row) => row
                .try_get::<Option<Decimal>, _>(idx.as_sqlx_mysql_index())
                .map_err(|e| sqlx_error_to_query_err(e).into())
                .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx))),
            #[cfg(feature = "sqlx-postgres")]
            QueryResultRow::SqlxPostgres(row) => row
                .try_get::<Option<Decimal>, _>(idx.as_sqlx_postgres_index())
                .map_err(|e| sqlx_error_to_query_err(e).into())
                .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx))),
            #[cfg(feature = "sqlx-sqlite")]
            QueryResultRow::SqlxSqlite(row) => {
                let val: Option<f64> = row
                    .try_get(idx.as_sqlx_sqlite_index())
                    .map_err(sqlx_error_to_query_err)?;
                match val {
                    Some(v) => Decimal::try_from(v).map_err(|e| {
                        DbErr::TryIntoErr {
                            from: "f64",
                            into: "Decimal",
                            source: Box::new(e),
                        }
                        .into()
                    }),
                    None => Err(err_null_idx_col(idx)),
                }
            }
            #[cfg(feature = "mock")]
            #[allow(unused_variables)]
            QueryResultRow::Mock(row) => row.try_get(idx).map_err(|e| {
                debug_print!("{:#?}", e.to_string());
                err_null_idx_col(idx)
            }),
            #[cfg(feature = "proxy")]
            #[allow(unused_variables)]
            QueryResultRow::Proxy(row) => row.try_get(idx).map_err(|e| {
                debug_print!("{:#?}", e.to_string());
                err_null_idx_col(idx)
            }),
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}

#[cfg(feature = "with-bigdecimal")]
use bigdecimal::BigDecimal;

#[cfg(feature = "with-bigdecimal")]
impl TryGetable for BigDecimal {
    #[allow(unused_variables)]
    fn try_get_by<I: ColIdx>(res: &QueryResult, idx: I) -> Result<Self, TryGetError> {
        match &res.row {
            #[cfg(feature = "sqlx-mysql")]
            QueryResultRow::SqlxMySql(row) => row
                .try_get::<Option<BigDecimal>, _>(idx.as_sqlx_mysql_index())
                .map_err(|e| sqlx_error_to_query_err(e).into())
                .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx))),
            #[cfg(feature = "sqlx-postgres")]
            QueryResultRow::SqlxPostgres(row) => row
                .try_get::<Option<BigDecimal>, _>(idx.as_sqlx_postgres_index())
                .map_err(|e| sqlx_error_to_query_err(e).into())
                .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx))),
            #[cfg(feature = "sqlx-sqlite")]
            QueryResultRow::SqlxSqlite(row) => {
                let val: Option<f64> = row
                    .try_get(idx.as_sqlx_sqlite_index())
                    .map_err(sqlx_error_to_query_err)?;
                match val {
                    Some(v) => BigDecimal::try_from(v).map_err(|e| {
                        DbErr::TryIntoErr {
                            from: "f64",
                            into: "BigDecimal",
                            source: Box::new(e),
                        }
                        .into()
                    }),
                    None => Err(err_null_idx_col(idx)),
                }
            }
            #[cfg(feature = "mock")]
            #[allow(unused_variables)]
            QueryResultRow::Mock(row) => row.try_get(idx).map_err(|e| {
                debug_print!("{:#?}", e.to_string());
                err_null_idx_col(idx)
            }),
            #[cfg(feature = "proxy")]
            #[allow(unused_variables)]
            QueryResultRow::Proxy(row) => row.try_get(idx).map_err(|e| {
                debug_print!("{:#?}", e.to_string());
                err_null_idx_col(idx)
            }),
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}

#[allow(unused_macros)]
macro_rules! try_getable_uuid {
    ( $type: ty, $conversion_fn: expr ) => {
        #[allow(unused_variables, unreachable_code)]
        impl TryGetable for $type {
            fn try_get_by<I: ColIdx>(res: &QueryResult, idx: I) -> Result<Self, TryGetError> {
                let res: Result<uuid::Uuid, TryGetError> = match &res.row {
                    #[cfg(feature = "sqlx-mysql")]
                    QueryResultRow::SqlxMySql(row) => row
                        .try_get::<Option<uuid::Uuid>, _>(idx.as_sqlx_mysql_index())
                        .map_err(|e| sqlx_error_to_query_err(e).into())
                        .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx)))
                        .or_else(|_| {
                            // MariaDB's UUID type stores UUIDs as hyphenated strings.
                            // reference: https://github.com/SeaQL/sea-orm/pull/2485
                            row.try_get::<Option<Vec<u8>>, _>(idx.as_sqlx_mysql_index())
                                .map_err(|e| sqlx_error_to_query_err(e).into())
                                .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx)))
                                .map(|bytes| {
                                    String::from_utf8(bytes).map_err(|e| {
                                        DbErr::TryIntoErr {
                                            from: "Vec<u8>",
                                            into: "String",
                                            source: Box::new(e),
                                        }
                                        .into()
                                    })
                                })?
                                .and_then(|s| {
                                    uuid::Uuid::parse_str(&s).map_err(|e| {
                                        DbErr::TryIntoErr {
                                            from: "String",
                                            into: "uuid::Uuid",
                                            source: Box::new(e),
                                        }
                                        .into()
                                    })
                                })
                        }),
                    #[cfg(feature = "sqlx-postgres")]
                    QueryResultRow::SqlxPostgres(row) => row
                        .try_get::<Option<uuid::Uuid>, _>(idx.as_sqlx_postgres_index())
                        .map_err(|e| sqlx_error_to_query_err(e).into())
                        .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx))),
                    #[cfg(feature = "sqlx-sqlite")]
                    QueryResultRow::SqlxSqlite(row) => row
                        .try_get::<Option<uuid::Uuid>, _>(idx.as_sqlx_sqlite_index())
                        .map_err(|e| sqlx_error_to_query_err(e).into())
                        .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx))),
                    #[cfg(feature = "mock")]
                    #[allow(unused_variables)]
                    QueryResultRow::Mock(row) => row.try_get::<uuid::Uuid, _>(idx).map_err(|e| {
                        debug_print!("{:#?}", e.to_string());
                        err_null_idx_col(idx)
                    }),
                    #[cfg(feature = "proxy")]
                    #[allow(unused_variables)]
                    QueryResultRow::Proxy(row) => row.try_get::<uuid::Uuid, _>(idx).map_err(|e| {
                        debug_print!("{:#?}", e.to_string());
                        err_null_idx_col(idx)
                    }),
                    #[allow(unreachable_patterns)]
                    _ => unreachable!(),
                };
                res.map($conversion_fn)
            }
        }
    };
}

#[cfg(feature = "with-uuid")]
try_getable_uuid!(uuid::Uuid, Into::into);

#[cfg(feature = "with-uuid")]
try_getable_uuid!(uuid::fmt::Braced, uuid::Uuid::braced);

#[cfg(feature = "with-uuid")]
try_getable_uuid!(uuid::fmt::Hyphenated, uuid::Uuid::hyphenated);

#[cfg(feature = "with-uuid")]
try_getable_uuid!(uuid::fmt::Simple, uuid::Uuid::simple);

#[cfg(feature = "with-uuid")]
try_getable_uuid!(uuid::fmt::Urn, uuid::Uuid::urn);

#[cfg(feature = "with-ipnetwork")]
try_getable_postgres!(ipnetwork::IpNetwork);

impl TryGetable for u32 {
    #[allow(unused_variables)]
    fn try_get_by<I: ColIdx>(res: &QueryResult, idx: I) -> Result<Self, TryGetError> {
        match &res.row {
            #[cfg(feature = "sqlx-mysql")]
            QueryResultRow::SqlxMySql(row) => row
                .try_get::<Option<u32>, _>(idx.as_sqlx_mysql_index())
                .map_err(|e| sqlx_error_to_query_err(e).into())
                .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx))),
            #[cfg(feature = "sqlx-postgres")]
            QueryResultRow::SqlxPostgres(row) => {
                use sqlx::postgres::types::Oid;
                // Since 0.6.0, SQLx has dropped direct mapping from PostgreSQL's OID to Rust's `u32`;
                // Instead, `u32` was wrapped by a `sqlx::Oid`.
                row.try_get::<Option<Oid>, _>(idx.as_sqlx_postgres_index())
                    .map_err(|e| sqlx_error_to_query_err(e).into())
                    .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx)))
                    .map(|oid| oid.0)
            }
            #[cfg(feature = "sqlx-sqlite")]
            QueryResultRow::SqlxSqlite(row) => row
                .try_get::<Option<u32>, _>(idx.as_sqlx_sqlite_index())
                .map_err(|e| sqlx_error_to_query_err(e).into())
                .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx))),
            #[cfg(feature = "mock")]
            #[allow(unused_variables)]
            QueryResultRow::Mock(row) => row.try_get(idx).map_err(|e| {
                debug_print!("{:#?}", e.to_string());
                err_null_idx_col(idx)
            }),
            #[cfg(feature = "proxy")]
            #[allow(unused_variables)]
            QueryResultRow::Proxy(row) => row.try_get(idx).map_err(|e| {
                debug_print!("{:#?}", e.to_string());
                err_null_idx_col(idx)
            }),
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}

impl TryGetable for String {
    #[allow(unused_variables)]
    fn try_get_by<I: ColIdx>(res: &QueryResult, idx: I) -> Result<Self, TryGetError> {
        match &res.row {
            #[cfg(feature = "sqlx-mysql")]
            QueryResultRow::SqlxMySql(row) => row
                .try_get::<Option<Vec<u8>>, _>(idx.as_sqlx_mysql_index())
                .map_err(|e| sqlx_error_to_query_err(e).into())
                .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx)))
                .map(|bytes| {
                    String::from_utf8(bytes).map_err(|e| {
                        DbErr::TryIntoErr {
                            from: "Vec<u8>",
                            into: "String",
                            source: Box::new(e),
                        }
                        .into()
                    })
                })?,
            #[cfg(feature = "sqlx-postgres")]
            QueryResultRow::SqlxPostgres(row) => row
                .try_get::<Option<String>, _>(idx.as_sqlx_postgres_index())
                .map_err(|e| sqlx_error_to_query_err(e).into())
                .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx))),
            #[cfg(feature = "sqlx-sqlite")]
            QueryResultRow::SqlxSqlite(row) => row
                .try_get::<Option<String>, _>(idx.as_sqlx_sqlite_index())
                .map_err(|e| sqlx_error_to_query_err(e).into())
                .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx))),
            #[cfg(feature = "mock")]
            QueryResultRow::Mock(row) => row.try_get(idx).map_err(|e| {
                debug_print!("{:#?}", e.to_string());
                err_null_idx_col(idx)
            }),
            #[cfg(feature = "proxy")]
            QueryResultRow::Proxy(row) => row.try_get(idx).map_err(|e| {
                debug_print!("{:#?}", e.to_string());
                err_null_idx_col(idx)
            }),
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}

#[allow(dead_code)]
fn err_null_idx_col<I: ColIdx>(idx: I) -> TryGetError {
    TryGetError::Null(format!("{idx:?}"))
}

#[cfg(feature = "postgres-array")]
mod postgres_array {
    use super::*;

    #[allow(unused_macros)]
    macro_rules! try_getable_postgres_array {
        ( $type: ty ) => {
            #[allow(unused_variables)]
            impl TryGetable for Vec<$type> {
                fn try_get_by<I: ColIdx>(res: &QueryResult, idx: I) -> Result<Self, TryGetError> {
                    match &res.row {
                        #[cfg(feature = "sqlx-mysql")]
                        QueryResultRow::SqlxMySql(_) => Err(type_err(format!(
                            "{} unsupported by sqlx-mysql",
                            stringify!($type)
                        ))
                        .into()),
                        #[cfg(feature = "sqlx-postgres")]
                        QueryResultRow::SqlxPostgres(row) => row
                            .try_get::<Option<Vec<$type>>, _>(idx.as_sqlx_postgres_index())
                            .map_err(|e| sqlx_error_to_query_err(e).into())
                            .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx))),
                        #[cfg(feature = "sqlx-sqlite")]
                        QueryResultRow::SqlxSqlite(_) => Err(type_err(format!(
                            "{} unsupported by sqlx-sqlite",
                            stringify!($type)
                        ))
                        .into()),
                        #[cfg(feature = "mock")]
                        #[allow(unused_variables)]
                        QueryResultRow::Mock(row) => row.try_get(idx).map_err(|e| {
                            debug_print!("{:#?}", e.to_string());
                            err_null_idx_col(idx)
                        }),
                        #[cfg(feature = "proxy")]
                        #[allow(unused_variables)]
                        QueryResultRow::Proxy(row) => row.try_get(idx).map_err(|e| {
                            debug_print!("{:#?}", e.to_string());
                            err_null_idx_col(idx)
                        }),
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                }
            }
        };
    }

    try_getable_postgres_array!(bool);
    try_getable_postgres_array!(i8);
    try_getable_postgres_array!(i16);
    try_getable_postgres_array!(i32);
    try_getable_postgres_array!(i64);
    try_getable_postgres_array!(f32);
    try_getable_postgres_array!(f64);
    try_getable_postgres_array!(String);
    try_getable_postgres_array!(Vec<u8>);

    #[cfg(feature = "with-json")]
    try_getable_postgres_array!(serde_json::Value);

    #[cfg(feature = "with-chrono")]
    try_getable_postgres_array!(chrono::NaiveDate);

    #[cfg(feature = "with-chrono")]
    try_getable_postgres_array!(chrono::NaiveTime);

    #[cfg(feature = "with-chrono")]
    try_getable_postgres_array!(chrono::NaiveDateTime);

    #[cfg(feature = "with-chrono")]
    try_getable_postgres_array!(chrono::DateTime<chrono::FixedOffset>);

    #[cfg(feature = "with-chrono")]
    try_getable_postgres_array!(chrono::DateTime<chrono::Utc>);

    #[cfg(feature = "with-chrono")]
    try_getable_postgres_array!(chrono::DateTime<chrono::Local>);

    #[cfg(feature = "with-time")]
    try_getable_postgres_array!(time::Date);

    #[cfg(feature = "with-time")]
    try_getable_postgres_array!(time::Time);

    #[cfg(feature = "with-time")]
    try_getable_postgres_array!(time::PrimitiveDateTime);

    #[cfg(feature = "with-time")]
    try_getable_postgres_array!(time::OffsetDateTime);

    #[cfg(feature = "with-rust_decimal")]
    try_getable_postgres_array!(rust_decimal::Decimal);

    #[cfg(feature = "with-bigdecimal")]
    try_getable_postgres_array!(bigdecimal::BigDecimal);

    #[cfg(feature = "with-ipnetwork")]
    try_getable_postgres_array!(ipnetwork::IpNetwork);

    #[allow(unused_macros)]
    macro_rules! try_getable_postgres_array_uuid {
        ( $type: ty, $conversion_fn: expr ) => {
            #[allow(unused_variables, unreachable_code)]
            impl TryGetable for Vec<$type> {
                fn try_get_by<I: ColIdx>(res: &QueryResult, idx: I) -> Result<Self, TryGetError> {
                    let res: Result<Vec<uuid::Uuid>, TryGetError> = match &res.row {
                        #[cfg(feature = "sqlx-mysql")]
                        QueryResultRow::SqlxMySql(_) => Err(type_err(format!(
                            "{} unsupported by sqlx-mysql",
                            stringify!($type)
                        ))
                        .into()),
                        #[cfg(feature = "sqlx-postgres")]
                        QueryResultRow::SqlxPostgres(row) => row
                            .try_get::<Option<Vec<uuid::Uuid>>, _>(idx.as_sqlx_postgres_index())
                            .map_err(|e| sqlx_error_to_query_err(e).into())
                            .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx))),
                        #[cfg(feature = "sqlx-sqlite")]
                        QueryResultRow::SqlxSqlite(_) => Err(type_err(format!(
                            "{} unsupported by sqlx-sqlite",
                            stringify!($type)
                        ))
                        .into()),
                        #[cfg(feature = "mock")]
                        QueryResultRow::Mock(row) => {
                            row.try_get::<Vec<uuid::Uuid>, _>(idx).map_err(|e| {
                                debug_print!("{:#?}", e.to_string());
                                err_null_idx_col(idx)
                            })
                        }
                        #[cfg(feature = "proxy")]
                        QueryResultRow::Proxy(row) => {
                            row.try_get::<Vec<uuid::Uuid>, _>(idx).map_err(|e| {
                                debug_print!("{:#?}", e.to_string());
                                err_null_idx_col(idx)
                            })
                        }
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    };
                    res.map(|vec| vec.into_iter().map($conversion_fn).collect())
                }
            }
        };
    }

    #[cfg(feature = "with-uuid")]
    try_getable_postgres_array_uuid!(uuid::Uuid, Into::into);

    #[cfg(feature = "with-uuid")]
    try_getable_postgres_array_uuid!(uuid::fmt::Braced, uuid::Uuid::braced);

    #[cfg(feature = "with-uuid")]
    try_getable_postgres_array_uuid!(uuid::fmt::Hyphenated, uuid::Uuid::hyphenated);

    #[cfg(feature = "with-uuid")]
    try_getable_postgres_array_uuid!(uuid::fmt::Simple, uuid::Uuid::simple);

    #[cfg(feature = "with-uuid")]
    try_getable_postgres_array_uuid!(uuid::fmt::Urn, uuid::Uuid::urn);

    impl TryGetable for Vec<u32> {
        #[allow(unused_variables)]
        fn try_get_by<I: ColIdx>(res: &QueryResult, idx: I) -> Result<Self, TryGetError> {
            match &res.row {
                #[cfg(feature = "sqlx-mysql")]
                QueryResultRow::SqlxMySql(_) => {
                    Err(type_err(format!("{} unsupported by sqlx-mysql", stringify!($type))).into())
                }
                #[cfg(feature = "sqlx-postgres")]
                QueryResultRow::SqlxPostgres(row) => {
                    use sqlx::postgres::types::Oid;
                    // Since 0.6.0, SQLx has dropped direct mapping from PostgreSQL's OID to Rust's `u32`;
                    // Instead, `u32` was wrapped by a `sqlx::Oid`.
                    row.try_get::<Option<Vec<Oid>>, _>(idx.as_sqlx_postgres_index())
                        .map_err(|e| sqlx_error_to_query_err(e).into())
                        .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx)))
                        .map(|oids| oids.into_iter().map(|oid| oid.0).collect())
                }
                #[cfg(feature = "sqlx-sqlite")]
                QueryResultRow::SqlxSqlite(_) => Err(type_err(format!(
                    "{} unsupported by sqlx-sqlite",
                    stringify!($type)
                ))
                .into()),
                #[cfg(feature = "mock")]
                #[allow(unused_variables)]
                QueryResultRow::Mock(row) => row.try_get(idx).map_err(|e| {
                    debug_print!("{:#?}", e.to_string());
                    err_null_idx_col(idx)
                }),
                #[cfg(feature = "proxy")]
                #[allow(unused_variables)]
                QueryResultRow::Proxy(row) => row.try_get(idx).map_err(|e| {
                    debug_print!("{:#?}", e.to_string());
                    err_null_idx_col(idx)
                }),
                #[allow(unreachable_patterns)]
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(feature = "postgres-vector")]
impl TryGetable for pgvector::Vector {
    #[allow(unused_variables)]
    fn try_get_by<I: ColIdx>(res: &QueryResult, idx: I) -> Result<Self, TryGetError> {
        match &res.row {
            #[cfg(feature = "sqlx-mysql")]
            QueryResultRow::SqlxMySql(_) => {
                Err(type_err("Vector unsupported by sqlx-mysql").into())
            }
            #[cfg(feature = "sqlx-postgres")]
            QueryResultRow::SqlxPostgres(row) => row
                .try_get::<Option<pgvector::Vector>, _>(idx.as_sqlx_postgres_index())
                .map_err(|e| sqlx_error_to_query_err(e).into())
                .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx))),
            #[cfg(feature = "sqlx-sqlite")]
            QueryResultRow::SqlxSqlite(_) => {
                Err(type_err("Vector unsupported by sqlx-sqlite").into())
            }
            #[cfg(feature = "mock")]
            QueryResultRow::Mock(row) => row.try_get::<pgvector::Vector, _>(idx).map_err(|e| {
                debug_print!("{:#?}", e.to_string());
                err_null_idx_col(idx)
            }),
            #[cfg(feature = "proxy")]
            QueryResultRow::Proxy(row) => row.try_get::<pgvector::Vector, _>(idx).map_err(|e| {
                debug_print!("{:#?}", e.to_string());
                err_null_idx_col(idx)
            }),
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}

// TryGetableMany //

/// An interface to get a tuple value from the query result
pub trait TryGetableMany: Sized {
    /// Get a tuple value from the query result with prefixed column name
    fn try_get_many(res: &QueryResult, pre: &str, cols: &[String]) -> Result<Self, TryGetError>;

    /// Get a tuple value from the query result based on the order in the select expressions
    fn try_get_many_by_index(res: &QueryResult) -> Result<Self, TryGetError>;

    /// ```
    /// # use sea_orm::{error::*, tests_cfg::*, *};
    /// #
    /// # #[smol_potat::main]
    /// # #[cfg(all(feature = "mock", feature = "macros"))]
    /// # pub async fn main() -> Result<(), DbErr> {
    /// #
    /// # let db = MockDatabase::new(DbBackend::Postgres)
    /// #     .append_query_results([[
    /// #         maplit::btreemap! {
    /// #             "name" => Into::<Value>::into("Chocolate Forest"),
    /// #             "num_of_cakes" => Into::<Value>::into(1),
    /// #         },
    /// #         maplit::btreemap! {
    /// #             "name" => Into::<Value>::into("New York Cheese"),
    /// #             "num_of_cakes" => Into::<Value>::into(1),
    /// #         },
    /// #     ]])
    /// #     .into_connection();
    /// #
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, DeriveIden, EnumIter, TryGetableMany};
    ///
    /// #[derive(EnumIter, DeriveIden)]
    /// enum ResultCol {
    ///     Name,
    ///     NumOfCakes,
    /// }
    ///
    /// let res: Vec<(String, i32)> =
    ///     <(String, i32)>::find_by_statement::<ResultCol>(Statement::from_sql_and_values(
    ///         DbBackend::Postgres,
    ///         r#"SELECT "cake"."name", count("cake"."id") AS "num_of_cakes" FROM "cake""#,
    ///         [],
    ///     ))
    ///     .all(&db)
    ///     .await?;
    ///
    /// assert_eq!(
    ///     res,
    ///     [
    ///         ("Chocolate Forest".to_owned(), 1),
    ///         ("New York Cheese".to_owned(), 1),
    ///     ]
    /// );
    ///
    /// assert_eq!(
    ///     db.into_transaction_log(),
    ///     [Transaction::from_sql_and_values(
    ///         DbBackend::Postgres,
    ///         r#"SELECT "cake"."name", count("cake"."id") AS "num_of_cakes" FROM "cake""#,
    ///         []
    ///     ),]
    /// );
    /// #
    /// # Ok(())
    /// # }
    /// ```
    fn find_by_statement<C>(stmt: Statement) -> SelectorRaw<SelectGetableValue<Self, C>>
    where
        C: strum::IntoEnumIterator + sea_query::Iden,
    {
        SelectorRaw::<SelectGetableValue<Self, C>>::with_columns(stmt)
    }
}

impl<T> TryGetableMany for T
where
    T: TryGetable,
{
    fn try_get_many(res: &QueryResult, pre: &str, cols: &[String]) -> Result<Self, TryGetError> {
        try_get_many_with_slice_len_of(1, cols)?;
        T::try_get(res, pre, &cols[0])
    }

    fn try_get_many_by_index(res: &QueryResult) -> Result<Self, TryGetError> {
        T::try_get_by_index(res, 0)
    }
}

impl<T> TryGetableMany for (T,)
where
    T: TryGetableMany,
{
    fn try_get_many(res: &QueryResult, pre: &str, cols: &[String]) -> Result<Self, TryGetError> {
        T::try_get_many(res, pre, cols).map(|r| (r,))
    }

    fn try_get_many_by_index(res: &QueryResult) -> Result<Self, TryGetError> {
        T::try_get_many_by_index(res).map(|r| (r,))
    }
}

macro_rules! impl_try_get_many {
    ( $LEN:expr, $($T:ident : $N:expr),+ $(,)? ) => {
        impl< $($T),+ > TryGetableMany for ( $($T),+ )
        where
            $($T: TryGetable),+
        {
            fn try_get_many(res: &QueryResult, pre: &str, cols: &[String]) -> Result<Self, TryGetError> {
                try_get_many_with_slice_len_of($LEN, cols)?;
                Ok((
                    $($T::try_get(res, pre, &cols[$N])?),+
                ))
            }

            fn try_get_many_by_index(res: &QueryResult) -> Result<Self, TryGetError> {
                Ok((
                    $($T::try_get_by_index(res, $N)?),+
                ))
            }
        }
    };
}

#[rustfmt::skip]
mod impl_try_get_many {
    use super::*;

    impl_try_get_many!( 2, T0:0, T1:1);
    impl_try_get_many!( 3, T0:0, T1:1, T2:2);
    impl_try_get_many!( 4, T0:0, T1:1, T2:2, T3:3);
    impl_try_get_many!( 5, T0:0, T1:1, T2:2, T3:3, T4:4);
    impl_try_get_many!( 6, T0:0, T1:1, T2:2, T3:3, T4:4, T5:5);
    impl_try_get_many!( 7, T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6);
    impl_try_get_many!( 8, T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7);
    impl_try_get_many!( 9, T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8);
    impl_try_get_many!(10, T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9);
    impl_try_get_many!(11, T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10);
    impl_try_get_many!(12, T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11);
}

fn try_get_many_with_slice_len_of(len: usize, cols: &[String]) -> Result<(), TryGetError> {
    if cols.len() < len {
        Err(type_err(format!(
            "Expect {} column names supplied but got slice of length {}",
            len,
            cols.len()
        ))
        .into())
    } else {
        Ok(())
    }
}

/// An interface to get an array of values from the query result.
/// A type can only implement `ActiveEnum` or `TryGetableFromJson`, but not both.
/// A blanket impl is provided for `TryGetableFromJson`, while the impl for `ActiveEnum`
/// is provided by the `DeriveActiveEnum` macro. So as an end user you won't normally
/// touch this trait.
pub trait TryGetableArray: Sized {
    /// Just a delegate
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Vec<Self>, TryGetError>;
}

impl<T> TryGetable for Vec<T>
where
    T: TryGetableArray,
{
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        T::try_get_by(res, index)
    }
}

// TryGetableFromJson //

/// An interface to get a JSON from the query result
#[cfg(feature = "with-json")]
pub trait TryGetableFromJson: Sized
where
    for<'de> Self: serde::Deserialize<'de>,
{
    /// Get a JSON from the query result with prefixed column name
    #[allow(unused_variables, unreachable_code)]
    fn try_get_from_json<I: ColIdx>(res: &QueryResult, idx: I) -> Result<Self, TryGetError> {
        match &res.row {
            #[cfg(feature = "sqlx-mysql")]
            QueryResultRow::SqlxMySql(row) => row
                .try_get::<Option<sqlx::types::Json<Self>>, _>(idx.as_sqlx_mysql_index())
                .map_err(|e| sqlx_error_to_query_err(e).into())
                .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx)).map(|json| json.0)),
            #[cfg(feature = "sqlx-postgres")]
            QueryResultRow::SqlxPostgres(row) => row
                .try_get::<Option<sqlx::types::Json<Self>>, _>(idx.as_sqlx_postgres_index())
                .map_err(|e| sqlx_error_to_query_err(e).into())
                .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx)).map(|json| json.0)),
            #[cfg(feature = "sqlx-sqlite")]
            QueryResultRow::SqlxSqlite(row) => row
                .try_get::<Option<sqlx::types::Json<Self>>, _>(idx.as_sqlx_sqlite_index())
                .map_err(|e| sqlx_error_to_query_err(e).into())
                .and_then(|opt| opt.ok_or_else(|| err_null_idx_col(idx)).map(|json| json.0)),
            #[cfg(feature = "mock")]
            QueryResultRow::Mock(row) => row
                .try_get::<serde_json::Value, I>(idx)
                .map_err(|e| {
                    debug_print!("{:#?}", e.to_string());
                    err_null_idx_col(idx)
                })
                .and_then(|json| serde_json::from_value(json).map_err(|e| json_err(e).into())),
            #[cfg(feature = "proxy")]
            QueryResultRow::Proxy(row) => row
                .try_get::<serde_json::Value, I>(idx)
                .map_err(|e| {
                    debug_print!("{:#?}", e.to_string());
                    err_null_idx_col(idx)
                })
                .and_then(|json| serde_json::from_value(json).map_err(|e| json_err(e).into())),
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }

    /// Get a Vec<Self> from an Array of Json
    fn from_json_vec(value: serde_json::Value) -> Result<Vec<Self>, TryGetError> {
        match value {
            serde_json::Value::Array(values) => {
                let mut res = Vec::new();
                for item in values {
                    res.push(serde_json::from_value(item).map_err(json_err)?);
                }
                Ok(res)
            }
            _ => Err(TryGetError::DbErr(DbErr::Json(
                "Value is not an Array".to_owned(),
            ))),
        }
    }
}

#[cfg(feature = "with-json")]
impl<T> TryGetable for T
where
    T: TryGetableFromJson,
{
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        T::try_get_from_json(res, index)
    }
}

#[cfg(feature = "with-json")]
impl<T> TryGetableArray for T
where
    T: TryGetableFromJson,
{
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Vec<T>, TryGetError> {
        T::from_json_vec(serde_json::Value::try_get_by(res, index)?)
    }
}

// TryFromU64 //
/// Try to convert a type to a u64
pub trait TryFromU64: Sized {
    /// The method to convert the type to a u64
    fn try_from_u64(n: u64) -> Result<Self, DbErr>;
}

macro_rules! try_from_u64_err {
    ( $type: ty ) => {
        impl TryFromU64 for $type {
            fn try_from_u64(_: u64) -> Result<Self, DbErr> {
                Err(DbErr::ConvertFromU64(stringify!($type)))
            }
        }
    };

    ( $($gen_type: ident),* ) => {
        impl<$( $gen_type, )*> TryFromU64 for ($( $gen_type, )*)
        where
            $( $gen_type: TryFromU64, )*
        {
            fn try_from_u64(_: u64) -> Result<Self, DbErr> {
                Err(DbErr::ConvertFromU64(stringify!($($gen_type,)*)))
            }
        }
    };
}

#[rustfmt::skip]
mod try_from_u64_err {
    use super::*;

    try_from_u64_err!(T0, T1);
    try_from_u64_err!(T0, T1, T2);
    try_from_u64_err!(T0, T1, T2, T3);
    try_from_u64_err!(T0, T1, T2, T3, T4);
    try_from_u64_err!(T0, T1, T2, T3, T4, T5);
    try_from_u64_err!(T0, T1, T2, T3, T4, T5, T6);
    try_from_u64_err!(T0, T1, T2, T3, T4, T5, T6, T7);
    try_from_u64_err!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
    try_from_u64_err!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    try_from_u64_err!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    try_from_u64_err!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
}

macro_rules! try_from_u64_numeric {
    ( $type: ty ) => {
        impl TryFromU64 for $type {
            fn try_from_u64(n: u64) -> Result<Self, DbErr> {
                use std::convert::TryInto;
                n.try_into().map_err(|e| DbErr::TryIntoErr {
                    from: stringify!(u64),
                    into: stringify!($type),
                    source: Box::new(e),
                })
            }
        }
    };
}

try_from_u64_numeric!(i8);
try_from_u64_numeric!(i16);
try_from_u64_numeric!(i32);
try_from_u64_numeric!(i64);
try_from_u64_numeric!(u8);
try_from_u64_numeric!(u16);
try_from_u64_numeric!(u32);
try_from_u64_numeric!(u64);

macro_rules! try_from_u64_string {
    ( $type: ty ) => {
        impl TryFromU64 for $type {
            fn try_from_u64(n: u64) -> Result<Self, DbErr> {
                Ok(n.to_string())
            }
        }
    };
}

try_from_u64_string!(String);

try_from_u64_err!(bool);
try_from_u64_err!(f32);
try_from_u64_err!(f64);
try_from_u64_err!(Vec<u8>);

#[cfg(feature = "with-json")]
try_from_u64_err!(serde_json::Value);

#[cfg(feature = "with-chrono")]
try_from_u64_err!(chrono::NaiveDate);

#[cfg(feature = "with-chrono")]
try_from_u64_err!(chrono::NaiveTime);

#[cfg(feature = "with-chrono")]
try_from_u64_err!(chrono::NaiveDateTime);

#[cfg(feature = "with-chrono")]
try_from_u64_err!(chrono::DateTime<chrono::FixedOffset>);

#[cfg(feature = "with-chrono")]
try_from_u64_err!(chrono::DateTime<chrono::Utc>);

#[cfg(feature = "with-chrono")]
try_from_u64_err!(chrono::DateTime<chrono::Local>);

#[cfg(feature = "with-time")]
try_from_u64_err!(time::Date);

#[cfg(feature = "with-time")]
try_from_u64_err!(time::Time);

#[cfg(feature = "with-time")]
try_from_u64_err!(time::PrimitiveDateTime);

#[cfg(feature = "with-time")]
try_from_u64_err!(time::OffsetDateTime);

#[cfg(feature = "with-rust_decimal")]
try_from_u64_err!(rust_decimal::Decimal);

#[cfg(feature = "with-uuid")]
try_from_u64_err!(uuid::Uuid);

#[cfg(feature = "with-ipnetwork")]
try_from_u64_err!(ipnetwork::IpNetwork);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RuntimeErr;
    use sea_query::Value;
    use std::collections::BTreeMap;

    #[test]
    fn from_try_get_error() {
        // TryGetError::DbErr
        let try_get_error = TryGetError::DbErr(DbErr::Query(RuntimeErr::Internal(
            "expected error message".to_owned(),
        )));
        assert_eq!(
            DbErr::from(try_get_error),
            DbErr::Query(RuntimeErr::Internal("expected error message".to_owned()))
        );

        // TryGetError::Null
        let try_get_error = TryGetError::Null("column".to_owned());
        let expected = "A null value was encountered while decoding column".to_owned();
        assert_eq!(DbErr::from(try_get_error), DbErr::Type(expected));
    }

    #[test]
    fn build_with_query() {
        use sea_orm::{DbBackend, Statement};
        use sea_query::*;

        let base_query = SelectStatement::new()
            .column(Alias::new("id"))
            .expr(1i32)
            .column(Alias::new("next"))
            .column(Alias::new("value"))
            .from(Alias::new("table"))
            .to_owned();

        let cte_referencing = SelectStatement::new()
            .column(Alias::new("id"))
            .expr(Expr::col(Alias::new("depth")).add(1i32))
            .column(Alias::new("next"))
            .column(Alias::new("value"))
            .from(Alias::new("table"))
            .join(
                JoinType::InnerJoin,
                Alias::new("cte_traversal"),
                Expr::col((Alias::new("cte_traversal"), Alias::new("next")))
                    .equals((Alias::new("table"), Alias::new("id"))),
            )
            .to_owned();

        let common_table_expression = CommonTableExpression::new()
            .query(
                base_query
                    .clone()
                    .union(UnionType::All, cte_referencing)
                    .to_owned(),
            )
            .columns([
                Alias::new("id"),
                Alias::new("depth"),
                Alias::new("next"),
                Alias::new("value"),
            ])
            .table_name(Alias::new("cte_traversal"))
            .to_owned();

        let select = SelectStatement::new()
            .column(ColumnRef::Asterisk)
            .from(Alias::new("cte_traversal"))
            .to_owned();

        let with_clause = WithClause::new()
            .recursive(true)
            .cte(common_table_expression)
            .cycle(Cycle::new_from_expr_set_using(
                SimpleExpr::Column(ColumnRef::Column(Alias::new("id").into_iden())),
                Alias::new("looped"),
                Alias::new("traversal_path"),
            ))
            .to_owned();

        let with_query = select.with(with_clause).to_owned();

        assert_eq!(
            DbBackend::MySql.build(&with_query),
            Statement::from_sql_and_values(
                DbBackend::MySql,
                r#"WITH RECURSIVE `cte_traversal` (`id`, `depth`, `next`, `value`) AS (SELECT `id`, ?, `next`, `value` FROM `table` UNION ALL (SELECT `id`, `depth` + ?, `next`, `value` FROM `table` INNER JOIN `cte_traversal` ON `cte_traversal`.`next` = `table`.`id`)) SELECT * FROM `cte_traversal`"#,
                [1.into(), 1.into()]
            )
        );
    }

    #[test]
    fn column_names_from_query_result() {
        let mut values = BTreeMap::new();
        values.insert("id".to_string(), Value::Int(Some(1)));
        values.insert(
            "name".to_string(),
            Value::String(Some(Box::new("Abc".to_owned()))),
        );
        let query_result = QueryResult {
            row: QueryResultRow::Mock(crate::MockRow { values }),
        };
        assert_eq!(
            query_result.column_names(),
            vec!["id".to_owned(), "name".to_owned()]
        );
    }
}
