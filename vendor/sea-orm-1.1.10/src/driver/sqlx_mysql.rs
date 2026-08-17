use futures_util::lock::Mutex;
use log::LevelFilter;
use sea_query::Values;
use std::{future::Future, pin::Pin, sync::Arc};

use sqlx::{
    mysql::{MySqlConnectOptions, MySqlQueryResult, MySqlRow},
    pool::PoolConnection,
    Connection, Executor, MySql, MySqlPool,
};

use sea_query_binder::SqlxValues;
use tracing::instrument;

use crate::{
    debug_print, error::*, executor::*, AccessMode, ConnectOptions, DatabaseConnection,
    DatabaseTransaction, DbBackend, IsolationLevel, QueryStream, Statement, TransactionError,
};

use super::sqlx_common::*;

/// Defines the [sqlx::mysql] connector
#[derive(Debug)]
pub struct SqlxMySqlConnector;

/// Defines a sqlx MySQL pool
#[derive(Clone)]
pub struct SqlxMySqlPoolConnection {
    pub(crate) pool: MySqlPool,
    metric_callback: Option<crate::metric::Callback>,
}

impl std::fmt::Debug for SqlxMySqlPoolConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SqlxMySqlPoolConnection {{ pool: {:?} }}", self.pool)
    }
}

impl From<MySqlPool> for SqlxMySqlPoolConnection {
    fn from(pool: MySqlPool) -> Self {
        SqlxMySqlPoolConnection {
            pool,
            metric_callback: None,
        }
    }
}

impl From<MySqlPool> for DatabaseConnection {
    fn from(pool: MySqlPool) -> Self {
        DatabaseConnection::SqlxMySqlPoolConnection(pool.into())
    }
}

impl SqlxMySqlConnector {
    /// Check if the URI provided corresponds to `mysql://` for a MySQL database
    pub fn accepts(string: &str) -> bool {
        string.starts_with("mysql://") && string.parse::<MySqlConnectOptions>().is_ok()
    }

    /// Add configuration options for the MySQL database
    #[instrument(level = "trace")]
    pub async fn connect(options: ConnectOptions) -> Result<DatabaseConnection, DbErr> {
        let mut opt = options
            .url
            .parse::<MySqlConnectOptions>()
            .map_err(sqlx_error_to_conn_err)?;
        use sqlx::ConnectOptions;
        if !options.sqlx_logging {
            opt = opt.disable_statement_logging();
        } else {
            opt = opt.log_statements(options.sqlx_logging_level);
            if options.sqlx_slow_statements_logging_level != LevelFilter::Off {
                opt = opt.log_slow_statements(
                    options.sqlx_slow_statements_logging_level,
                    options.sqlx_slow_statements_logging_threshold,
                );
            }
        }
        let pool = if options.connect_lazy {
            options.sqlx_pool_options().connect_lazy_with(opt)
        } else {
            options
                .sqlx_pool_options()
                .connect_with(opt)
                .await
                .map_err(sqlx_error_to_conn_err)?
        };
        Ok(DatabaseConnection::SqlxMySqlPoolConnection(
            SqlxMySqlPoolConnection {
                pool,
                metric_callback: None,
            },
        ))
    }
}

impl SqlxMySqlConnector {
    /// Instantiate a sqlx pool connection to a [DatabaseConnection]
    pub fn from_sqlx_mysql_pool(pool: MySqlPool) -> DatabaseConnection {
        DatabaseConnection::SqlxMySqlPoolConnection(SqlxMySqlPoolConnection {
            pool,
            metric_callback: None,
        })
    }
}

impl SqlxMySqlPoolConnection {
    /// Execute a [Statement] on a MySQL backend
    #[instrument(level = "trace")]
    pub async fn execute(&self, stmt: Statement) -> Result<ExecResult, DbErr> {
        debug_print!("{}", stmt);

        let query = sqlx_query(&stmt);
        let mut conn = self.pool.acquire().await.map_err(sqlx_conn_acquire_err)?;
        crate::metric::metric!(self.metric_callback, &stmt, {
            match query.execute(&mut *conn).await {
                Ok(res) => Ok(res.into()),
                Err(err) => Err(sqlx_error_to_exec_err(err)),
            }
        })
    }

    /// Execute an unprepared SQL statement on a MySQL backend
    #[instrument(level = "trace")]
    pub async fn execute_unprepared(&self, sql: &str) -> Result<ExecResult, DbErr> {
        debug_print!("{}", sql);

        let conn = &mut self.pool.acquire().await.map_err(sqlx_conn_acquire_err)?;
        match conn.execute(sql).await {
            Ok(res) => Ok(res.into()),
            Err(err) => Err(sqlx_error_to_exec_err(err)),
        }
    }

    /// Get one result from a SQL query. Returns [Option::None] if no match was found
    #[instrument(level = "trace")]
    pub async fn query_one(&self, stmt: Statement) -> Result<Option<QueryResult>, DbErr> {
        debug_print!("{}", stmt);

        let query = sqlx_query(&stmt);
        let mut conn = self.pool.acquire().await.map_err(sqlx_conn_acquire_err)?;
        crate::metric::metric!(self.metric_callback, &stmt, {
            match query.fetch_one(&mut *conn).await {
                Ok(row) => Ok(Some(row.into())),
                Err(err) => match err {
                    sqlx::Error::RowNotFound => Ok(None),
                    _ => Err(sqlx_error_to_query_err(err)),
                },
            }
        })
    }

    /// Get the results of a query returning them as a Vec<[QueryResult]>
    #[instrument(level = "trace")]
    pub async fn query_all(&self, stmt: Statement) -> Result<Vec<QueryResult>, DbErr> {
        debug_print!("{}", stmt);

        let query = sqlx_query(&stmt);
        let mut conn = self.pool.acquire().await.map_err(sqlx_conn_acquire_err)?;
        crate::metric::metric!(self.metric_callback, &stmt, {
            match query.fetch_all(&mut *conn).await {
                Ok(rows) => Ok(rows.into_iter().map(|r| r.into()).collect()),
                Err(err) => Err(sqlx_error_to_query_err(err)),
            }
        })
    }

    /// Stream the results of executing a SQL query
    #[instrument(level = "trace")]
    pub async fn stream(&self, stmt: Statement) -> Result<QueryStream, DbErr> {
        debug_print!("{}", stmt);

        let conn = self.pool.acquire().await.map_err(sqlx_conn_acquire_err)?;
        Ok(QueryStream::from((
            conn,
            stmt,
            self.metric_callback.clone(),
        )))
    }

    /// Bundle a set of SQL statements that execute together.
    #[instrument(level = "trace")]
    pub async fn begin(
        &self,
        isolation_level: Option<IsolationLevel>,
        access_mode: Option<AccessMode>,
    ) -> Result<DatabaseTransaction, DbErr> {
        let conn = self.pool.acquire().await.map_err(sqlx_conn_acquire_err)?;
        DatabaseTransaction::new_mysql(
            conn,
            self.metric_callback.clone(),
            isolation_level,
            access_mode,
        )
        .await
    }

    /// Create a MySQL transaction
    #[instrument(level = "trace", skip(callback))]
    pub async fn transaction<F, T, E>(
        &self,
        callback: F,
        isolation_level: Option<IsolationLevel>,
        access_mode: Option<AccessMode>,
    ) -> Result<T, TransactionError<E>>
    where
        F: for<'b> FnOnce(
                &'b DatabaseTransaction,
            ) -> Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'b>>
            + Send,
        T: Send,
        E: std::error::Error + Send,
    {
        let conn = self.pool.acquire().await.map_err(sqlx_conn_acquire_err)?;
        let transaction = DatabaseTransaction::new_mysql(
            conn,
            self.metric_callback.clone(),
            isolation_level,
            access_mode,
        )
        .await
        .map_err(|e| TransactionError::Connection(e))?;
        transaction.run(callback).await
    }

    pub(crate) fn set_metric_callback<F>(&mut self, callback: F)
    where
        F: Fn(&crate::metric::Info<'_>) + Send + Sync + 'static,
    {
        self.metric_callback = Some(Arc::new(callback));
    }

    /// Checks if a connection to the database is still valid.
    pub async fn ping(&self) -> Result<(), DbErr> {
        let conn = &mut self.pool.acquire().await.map_err(sqlx_conn_acquire_err)?;
        match conn.ping().await {
            Ok(_) => Ok(()),
            Err(err) => Err(sqlx_error_to_conn_err(err)),
        }
    }

    /// Explicitly close the MySQL connection.
    /// See [`Self::close_by_ref`] for usage with references.
    pub async fn close(self) -> Result<(), DbErr> {
        self.close_by_ref().await
    }

    /// Explicitly close the MySQL connection
    pub async fn close_by_ref(&self) -> Result<(), DbErr> {
        self.pool.close().await;
        Ok(())
    }
}

impl From<MySqlRow> for QueryResult {
    fn from(row: MySqlRow) -> QueryResult {
        QueryResult {
            row: QueryResultRow::SqlxMySql(row),
        }
    }
}

impl From<MySqlQueryResult> for ExecResult {
    fn from(result: MySqlQueryResult) -> ExecResult {
        ExecResult {
            result: ExecResultHolder::SqlxMySql(result),
        }
    }
}

pub(crate) fn sqlx_query(stmt: &Statement) -> sqlx::query::Query<'_, MySql, SqlxValues> {
    let values = stmt
        .values
        .as_ref()
        .map_or(Values(Vec::new()), |values| values.clone());
    sqlx::query_with(&stmt.sql, SqlxValues(values))
}

pub(crate) async fn set_transaction_config(
    conn: &mut PoolConnection<MySql>,
    isolation_level: Option<IsolationLevel>,
    access_mode: Option<AccessMode>,
) -> Result<(), DbErr> {
    let mut settings = Vec::new();

    if let Some(isolation_level) = isolation_level {
        settings.push(format!("ISOLATION LEVEL {isolation_level}"));
    }

    if let Some(access_mode) = access_mode {
        settings.push(access_mode.to_string());
    }

    if !settings.is_empty() {
        let stmt = Statement {
            sql: format!("SET TRANSACTION {}", settings.join(", ")),
            values: None,
            db_backend: DbBackend::MySql,
        };
        let query = sqlx_query(&stmt);
        conn.execute(query).await.map_err(sqlx_error_to_exec_err)?;
    }
    Ok(())
}

impl
    From<(
        PoolConnection<sqlx::MySql>,
        Statement,
        Option<crate::metric::Callback>,
    )> for crate::QueryStream
{
    fn from(
        (conn, stmt, metric_callback): (
            PoolConnection<sqlx::MySql>,
            Statement,
            Option<crate::metric::Callback>,
        ),
    ) -> Self {
        crate::QueryStream::build(stmt, crate::InnerConnection::MySql(conn), metric_callback)
    }
}

impl crate::DatabaseTransaction {
    pub(crate) async fn new_mysql(
        inner: PoolConnection<sqlx::MySql>,
        metric_callback: Option<crate::metric::Callback>,
        isolation_level: Option<IsolationLevel>,
        access_mode: Option<AccessMode>,
    ) -> Result<crate::DatabaseTransaction, DbErr> {
        Self::begin(
            Arc::new(Mutex::new(crate::InnerConnection::MySql(inner))),
            crate::DbBackend::MySql,
            metric_callback,
            isolation_level,
            access_mode,
        )
        .await
    }
}

#[cfg(feature = "proxy")]
pub(crate) fn from_sqlx_mysql_row_to_proxy_row(row: &sqlx::mysql::MySqlRow) -> crate::ProxyRow {
    // https://docs.rs/sqlx-mysql/0.7.2/src/sqlx_mysql/protocol/text/column.rs.html
    // https://docs.rs/sqlx-mysql/0.7.2/sqlx_mysql/types/index.html
    use sea_query::Value;
    use sqlx::{Column, Row, TypeInfo};
    crate::ProxyRow {
        values: row
            .columns()
            .iter()
            .map(|c| {
                (
                    c.name().to_string(),
                    match c.type_info().name() {
                        "TINYINT(1)" | "BOOLEAN" => Value::Bool(Some(
                            row.try_get(c.ordinal()).expect("Failed to get boolean"),
                        )),
                        "TINYINT UNSIGNED" => Value::TinyUnsigned(Some(
                            row.try_get(c.ordinal())
                                .expect("Failed to get unsigned tiny integer"),
                        )),
                        "SMALLINT UNSIGNED" => Value::SmallUnsigned(Some(
                            row.try_get(c.ordinal())
                                .expect("Failed to get unsigned small integer"),
                        )),
                        "INT UNSIGNED" => Value::Unsigned(Some(
                            row.try_get(c.ordinal())
                                .expect("Failed to get unsigned integer"),
                        )),
                        "MEDIUMINT UNSIGNED" | "BIGINT UNSIGNED" => Value::BigUnsigned(Some(
                            row.try_get(c.ordinal())
                                .expect("Failed to get unsigned big integer"),
                        )),
                        "TINYINT" => Value::TinyInt(Some(
                            row.try_get(c.ordinal())
                                .expect("Failed to get tiny integer"),
                        )),
                        "SMALLINT" => Value::SmallInt(Some(
                            row.try_get(c.ordinal())
                                .expect("Failed to get small integer"),
                        )),
                        "INT" => Value::Int(Some(
                            row.try_get(c.ordinal()).expect("Failed to get integer"),
                        )),
                        "MEDIUMINT" | "BIGINT" => Value::BigInt(Some(
                            row.try_get(c.ordinal()).expect("Failed to get big integer"),
                        )),
                        "FLOAT" => Value::Float(Some(
                            row.try_get(c.ordinal()).expect("Failed to get float"),
                        )),
                        "DOUBLE" => Value::Double(Some(
                            row.try_get(c.ordinal()).expect("Failed to get double"),
                        )),

                        "BIT" | "BINARY" | "VARBINARY" | "TINYBLOB" | "BLOB" | "MEDIUMBLOB"
                        | "LONGBLOB" => Value::Bytes(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get bytes"),
                        ))),

                        "CHAR" | "VARCHAR" | "TINYTEXT" | "TEXT" | "MEDIUMTEXT" | "LONGTEXT" => {
                            Value::String(Some(Box::new(
                                row.try_get(c.ordinal()).expect("Failed to get string"),
                            )))
                        }

                        #[cfg(feature = "with-chrono")]
                        "TIMESTAMP" => Value::ChronoDateTimeUtc(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get timestamp"),
                        ))),
                        #[cfg(all(feature = "with-time", not(feature = "with-chrono")))]
                        "TIMESTAMP" => Value::TimeDateTime(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get timestamp"),
                        ))),

                        #[cfg(feature = "with-chrono")]
                        "DATE" => Value::ChronoDate(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get date"),
                        ))),
                        #[cfg(all(feature = "with-time", not(feature = "with-chrono")))]
                        "DATE" => Value::TimeDate(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get date"),
                        ))),

                        #[cfg(feature = "with-chrono")]
                        "TIME" => Value::ChronoTime(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get time"),
                        ))),
                        #[cfg(all(feature = "with-time", not(feature = "with-chrono")))]
                        "TIME" => Value::TimeTime(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get time"),
                        ))),

                        #[cfg(feature = "with-chrono")]
                        "DATETIME" => Value::ChronoDateTime(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get datetime"),
                        ))),
                        #[cfg(all(feature = "with-time", not(feature = "with-chrono")))]
                        "DATETIME" => Value::TimeDateTime(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get datetime"),
                        ))),

                        #[cfg(feature = "with-chrono")]
                        "YEAR" => Value::ChronoDate(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get year"),
                        ))),
                        #[cfg(all(feature = "with-time", not(feature = "with-chrono")))]
                        "YEAR" => Value::TimeDate(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get year"),
                        ))),

                        "ENUM" | "SET" | "GEOMETRY" => Value::String(Some(Box::new(
                            row.try_get(c.ordinal())
                                .expect("Failed to get serialized string"),
                        ))),

                        #[cfg(feature = "with-bigdecimal")]
                        "DECIMAL" => Value::BigDecimal(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get decimal"),
                        ))),
                        #[cfg(all(
                            feature = "with-rust_decimal",
                            not(feature = "with-bigdecimal")
                        ))]
                        "DECIMAL" => Value::Decimal(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get decimal"),
                        ))),

                        #[cfg(feature = "with-json")]
                        "JSON" => Value::Json(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get json"),
                        ))),

                        _ => unreachable!("Unknown column type: {}", c.type_info().name()),
                    },
                )
            })
            .collect(),
    }
}
