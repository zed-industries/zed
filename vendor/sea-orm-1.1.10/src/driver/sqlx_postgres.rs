use futures_util::lock::Mutex;
use log::LevelFilter;
use sea_query::Values;
use std::{fmt::Write, future::Future, pin::Pin, sync::Arc};

use sqlx::{
    pool::PoolConnection,
    postgres::{PgConnectOptions, PgQueryResult, PgRow},
    Connection, Executor, PgPool, Postgres,
};

use sea_query_binder::SqlxValues;
use tracing::instrument;

use crate::{
    debug_print, error::*, executor::*, AccessMode, ConnectOptions, DatabaseConnection,
    DatabaseTransaction, DbBackend, IsolationLevel, QueryStream, Statement, TransactionError,
};

use super::sqlx_common::*;

/// Defines the [sqlx::postgres] connector
#[derive(Debug)]
pub struct SqlxPostgresConnector;

/// Defines a sqlx PostgreSQL pool
#[derive(Clone)]
pub struct SqlxPostgresPoolConnection {
    pub(crate) pool: PgPool,
    metric_callback: Option<crate::metric::Callback>,
}

impl std::fmt::Debug for SqlxPostgresPoolConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SqlxPostgresPoolConnection {{ pool: {:?} }}", self.pool)
    }
}

impl From<PgPool> for SqlxPostgresPoolConnection {
    fn from(pool: PgPool) -> Self {
        SqlxPostgresPoolConnection {
            pool,
            metric_callback: None,
        }
    }
}

impl From<PgPool> for DatabaseConnection {
    fn from(pool: PgPool) -> Self {
        DatabaseConnection::SqlxPostgresPoolConnection(pool.into())
    }
}

impl SqlxPostgresConnector {
    /// Check if the URI provided corresponds to `postgres://` for a PostgreSQL database
    pub fn accepts(string: &str) -> bool {
        string.starts_with("postgres://") && string.parse::<PgConnectOptions>().is_ok()
    }

    /// Add configuration options for the PostgreSQL database
    #[instrument(level = "trace")]
    pub async fn connect(options: ConnectOptions) -> Result<DatabaseConnection, DbErr> {
        let mut opt = options
            .url
            .parse::<PgConnectOptions>()
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
        let set_search_path_sql = options.schema_search_path.as_ref().map(|schema| {
            let mut string = "SET search_path = ".to_owned();
            if schema.starts_with('"') {
                write!(&mut string, "{schema}").unwrap();
            } else {
                for (i, schema) in schema.split(',').enumerate() {
                    if i > 0 {
                        write!(&mut string, ",").unwrap();
                    }
                    if schema.starts_with('"') {
                        write!(&mut string, "{schema}").unwrap();
                    } else {
                        write!(&mut string, "\"{schema}\"").unwrap();
                    }
                }
            }
            string
        });
        let lazy = options.connect_lazy;
        let mut pool_options = options.sqlx_pool_options();
        if let Some(sql) = set_search_path_sql {
            pool_options = pool_options.after_connect(move |conn, _| {
                let sql = sql.clone();
                Box::pin(async move {
                    sqlx::Executor::execute(conn, sql.as_str())
                        .await
                        .map(|_| ())
                })
            });
        }
        let pool = if lazy {
            pool_options.connect_lazy_with(opt)
        } else {
            pool_options
                .connect_with(opt)
                .await
                .map_err(sqlx_error_to_conn_err)?
        };
        Ok(DatabaseConnection::SqlxPostgresPoolConnection(
            SqlxPostgresPoolConnection {
                pool,
                metric_callback: None,
            },
        ))
    }
}

impl SqlxPostgresConnector {
    /// Instantiate a sqlx pool connection to a [DatabaseConnection]
    pub fn from_sqlx_postgres_pool(pool: PgPool) -> DatabaseConnection {
        DatabaseConnection::SqlxPostgresPoolConnection(SqlxPostgresPoolConnection {
            pool,
            metric_callback: None,
        })
    }
}

impl SqlxPostgresPoolConnection {
    /// Execute a [Statement] on a PostgreSQL backend
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

    /// Execute an unprepared SQL statement on a PostgreSQL backend
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
        DatabaseTransaction::new_postgres(
            conn,
            self.metric_callback.clone(),
            isolation_level,
            access_mode,
        )
        .await
    }

    /// Create a PostgreSQL transaction
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
        let transaction = DatabaseTransaction::new_postgres(
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

    /// Explicitly close the Postgres connection.
    /// See [`Self::close_by_ref`] for usage with references.
    pub async fn close(self) -> Result<(), DbErr> {
        self.close_by_ref().await
    }

    /// Explicitly close the Postgres connection
    pub async fn close_by_ref(&self) -> Result<(), DbErr> {
        self.pool.close().await;
        Ok(())
    }
}

impl From<PgRow> for QueryResult {
    fn from(row: PgRow) -> QueryResult {
        QueryResult {
            row: QueryResultRow::SqlxPostgres(row),
        }
    }
}

impl From<PgQueryResult> for ExecResult {
    fn from(result: PgQueryResult) -> ExecResult {
        ExecResult {
            result: ExecResultHolder::SqlxPostgres(result),
        }
    }
}

pub(crate) fn sqlx_query(stmt: &Statement) -> sqlx::query::Query<'_, Postgres, SqlxValues> {
    let values = stmt
        .values
        .as_ref()
        .map_or(Values(Vec::new()), |values| values.clone());
    sqlx::query_with(&stmt.sql, SqlxValues(values))
}

pub(crate) async fn set_transaction_config(
    conn: &mut PoolConnection<Postgres>,
    isolation_level: Option<IsolationLevel>,
    access_mode: Option<AccessMode>,
) -> Result<(), DbErr> {
    if let Some(isolation_level) = isolation_level {
        let stmt = Statement {
            sql: format!("SET TRANSACTION ISOLATION LEVEL {isolation_level}"),
            values: None,
            db_backend: DbBackend::Postgres,
        };
        let query = sqlx_query(&stmt);
        conn.execute(query).await.map_err(sqlx_error_to_exec_err)?;
    }
    if let Some(access_mode) = access_mode {
        let stmt = Statement {
            sql: format!("SET TRANSACTION {access_mode}"),
            values: None,
            db_backend: DbBackend::Postgres,
        };
        let query = sqlx_query(&stmt);
        conn.execute(query).await.map_err(sqlx_error_to_exec_err)?;
    }
    Ok(())
}

impl
    From<(
        PoolConnection<sqlx::Postgres>,
        Statement,
        Option<crate::metric::Callback>,
    )> for crate::QueryStream
{
    fn from(
        (conn, stmt, metric_callback): (
            PoolConnection<sqlx::Postgres>,
            Statement,
            Option<crate::metric::Callback>,
        ),
    ) -> Self {
        crate::QueryStream::build(
            stmt,
            crate::InnerConnection::Postgres(conn),
            metric_callback,
        )
    }
}

impl crate::DatabaseTransaction {
    pub(crate) async fn new_postgres(
        inner: PoolConnection<sqlx::Postgres>,
        metric_callback: Option<crate::metric::Callback>,
        isolation_level: Option<IsolationLevel>,
        access_mode: Option<AccessMode>,
    ) -> Result<crate::DatabaseTransaction, DbErr> {
        Self::begin(
            Arc::new(Mutex::new(crate::InnerConnection::Postgres(inner))),
            crate::DbBackend::Postgres,
            metric_callback,
            isolation_level,
            access_mode,
        )
        .await
    }
}

#[cfg(feature = "proxy")]
pub(crate) fn from_sqlx_postgres_row_to_proxy_row(row: &sqlx::postgres::PgRow) -> crate::ProxyRow {
    // https://docs.rs/sqlx-postgres/0.7.2/src/sqlx_postgres/type_info.rs.html
    // https://docs.rs/sqlx-postgres/0.7.2/sqlx_postgres/types/index.html
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
                        "BOOL" => Value::Bool(Some(
                            row.try_get(c.ordinal()).expect("Failed to get boolean"),
                        )),
                        #[cfg(feature = "postgres-array")]
                        "BOOL[]" => Value::Array(
                            sea_query::ArrayType::Bool,
                            Some(Box::new(
                                row.try_get::<Vec<bool>, _>(c.ordinal())
                                    .expect("Failed to get boolean array")
                                    .iter()
                                    .map(|val| Value::Bool(Some(*val)))
                                    .collect(),
                            )),
                        ),

                        "\"CHAR\"" => Value::TinyInt(Some(
                            row.try_get(c.ordinal())
                                .expect("Failed to get small integer"),
                        )),
                        #[cfg(feature = "postgres-array")]
                        "\"CHAR\"[]" => Value::Array(
                            sea_query::ArrayType::TinyInt,
                            Some(Box::new(
                                row.try_get::<Vec<i8>, _>(c.ordinal())
                                    .expect("Failed to get small integer array")
                                    .iter()
                                    .map(|val| Value::TinyInt(Some(*val)))
                                    .collect(),
                            )),
                        ),

                        "SMALLINT" | "SMALLSERIAL" | "INT2" => Value::SmallInt(Some(
                            row.try_get(c.ordinal())
                                .expect("Failed to get small integer"),
                        )),
                        #[cfg(feature = "postgres-array")]
                        "SMALLINT[]" | "SMALLSERIAL[]" | "INT2[]" => Value::Array(
                            sea_query::ArrayType::SmallInt,
                            Some(Box::new(
                                row.try_get::<Vec<i16>, _>(c.ordinal())
                                    .expect("Failed to get small integer array")
                                    .iter()
                                    .map(|val| Value::SmallInt(Some(*val)))
                                    .collect(),
                            )),
                        ),

                        "INT" | "SERIAL" | "INT4" => Value::Int(Some(
                            row.try_get(c.ordinal()).expect("Failed to get integer"),
                        )),
                        #[cfg(feature = "postgres-array")]
                        "INT[]" | "SERIAL[]" | "INT4[]" => Value::Array(
                            sea_query::ArrayType::Int,
                            Some(Box::new(
                                row.try_get::<Vec<i32>, _>(c.ordinal())
                                    .expect("Failed to get integer array")
                                    .iter()
                                    .map(|val| Value::Int(Some(*val)))
                                    .collect(),
                            )),
                        ),

                        "BIGINT" | "BIGSERIAL" | "INT8" => Value::BigInt(Some(
                            row.try_get(c.ordinal()).expect("Failed to get big integer"),
                        )),
                        #[cfg(feature = "postgres-array")]
                        "BIGINT[]" | "BIGSERIAL[]" | "INT8[]" => Value::Array(
                            sea_query::ArrayType::BigInt,
                            Some(Box::new(
                                row.try_get::<Vec<i64>, _>(c.ordinal())
                                    .expect("Failed to get big integer array")
                                    .iter()
                                    .map(|val| Value::BigInt(Some(*val)))
                                    .collect(),
                            )),
                        ),

                        "FLOAT4" | "REAL" => Value::Float(Some(
                            row.try_get(c.ordinal()).expect("Failed to get float"),
                        )),
                        #[cfg(feature = "postgres-array")]
                        "FLOAT4[]" | "REAL[]" => Value::Array(
                            sea_query::ArrayType::Float,
                            Some(Box::new(
                                row.try_get::<Vec<f32>, _>(c.ordinal())
                                    .expect("Failed to get float array")
                                    .iter()
                                    .map(|val| Value::Float(Some(*val)))
                                    .collect(),
                            )),
                        ),

                        "FLOAT8" | "DOUBLE PRECISION" => Value::Double(Some(
                            row.try_get(c.ordinal()).expect("Failed to get double"),
                        )),
                        #[cfg(feature = "postgres-array")]
                        "FLOAT8[]" | "DOUBLE PRECISION[]" => Value::Array(
                            sea_query::ArrayType::Double,
                            Some(Box::new(
                                row.try_get::<Vec<f64>, _>(c.ordinal())
                                    .expect("Failed to get double array")
                                    .iter()
                                    .map(|val| Value::Double(Some(*val)))
                                    .collect(),
                            )),
                        ),

                        "VARCHAR" | "CHAR" | "TEXT" | "NAME" => Value::String(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get string"),
                        ))),
                        #[cfg(feature = "postgres-array")]
                        "VARCHAR[]" | "CHAR[]" | "TEXT[]" | "NAME[]" => Value::Array(
                            sea_query::ArrayType::String,
                            Some(Box::new(
                                row.try_get::<Vec<String>, _>(c.ordinal())
                                    .expect("Failed to get string array")
                                    .iter()
                                    .map(|val| Value::String(Some(Box::new(val.clone()))))
                                    .collect(),
                            )),
                        ),

                        "BYTEA" => Value::Bytes(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get bytes"),
                        ))),
                        #[cfg(feature = "postgres-array")]
                        "BYTEA[]" => Value::Array(
                            sea_query::ArrayType::Bytes,
                            Some(Box::new(
                                row.try_get::<Vec<Vec<u8>>, _>(c.ordinal())
                                    .expect("Failed to get bytes array")
                                    .iter()
                                    .map(|val| Value::Bytes(Some(Box::new(val.clone()))))
                                    .collect(),
                            )),
                        ),

                        #[cfg(feature = "with-bigdecimal")]
                        "NUMERIC" => Value::BigDecimal(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get numeric"),
                        ))),
                        #[cfg(all(
                            feature = "with-rust_decimal",
                            not(feature = "with-bigdecimal")
                        ))]
                        "NUMERIC" => Value::Decimal(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get numeric"),
                        ))),

                        #[cfg(all(feature = "with-bigdecimal", feature = "postgres-array"))]
                        "NUMERIC[]" => Value::Array(
                            sea_query::ArrayType::BigDecimal,
                            Some(Box::new(
                                row.try_get::<Vec<bigdecimal::BigDecimal>, _>(c.ordinal())
                                    .expect("Failed to get numeric array")
                                    .iter()
                                    .map(|val| Value::BigDecimal(Some(Box::new(val.clone()))))
                                    .collect(),
                            )),
                        ),
                        #[cfg(all(
                            feature = "with-rust_decimal",
                            not(feature = "with-bigdecimal"),
                            feature = "postgres-array"
                        ))]
                        "NUMERIC[]" => Value::Array(
                            sea_query::ArrayType::Decimal,
                            Some(Box::new(
                                row.try_get::<Vec<rust_decimal::Decimal>, _>(c.ordinal())
                                    .expect("Failed to get numeric array")
                                    .iter()
                                    .map(|val| Value::Decimal(Some(Box::new(val.clone()))))
                                    .collect(),
                            )),
                        ),

                        "OID" => Value::BigInt(Some(
                            row.try_get(c.ordinal()).expect("Failed to get oid"),
                        )),
                        #[cfg(feature = "postgres-array")]
                        "OID[]" => Value::Array(
                            sea_query::ArrayType::BigInt,
                            Some(Box::new(
                                row.try_get::<Vec<i64>, _>(c.ordinal())
                                    .expect("Failed to get oid array")
                                    .iter()
                                    .map(|val| Value::BigInt(Some(*val)))
                                    .collect(),
                            )),
                        ),

                        "JSON" | "JSONB" => Value::Json(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get json"),
                        ))),
                        #[cfg(any(feature = "json-array", feature = "postgres-array"))]
                        "JSON[]" | "JSONB[]" => Value::Array(
                            sea_query::ArrayType::Json,
                            Some(Box::new(
                                row.try_get::<Vec<serde_json::Value>, _>(c.ordinal())
                                    .expect("Failed to get json array")
                                    .iter()
                                    .map(|val| Value::Json(Some(Box::new(val.clone()))))
                                    .collect(),
                            )),
                        ),

                        #[cfg(feature = "with-ipnetwork")]
                        "INET" | "CIDR" => Value::IpNetwork(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get ip address"),
                        ))),
                        #[cfg(feature = "with-ipnetwork")]
                        "INET[]" | "CIDR[]" => Value::Array(
                            sea_query::ArrayType::IpNetwork,
                            Some(Box::new(
                                row.try_get::<Vec<ipnetwork::IpNetwork>, _>(c.ordinal())
                                    .expect("Failed to get ip address array")
                                    .iter()
                                    .map(|val| Value::IpNetwork(Some(Box::new(val.clone()))))
                                    .collect(),
                            )),
                        ),

                        #[cfg(feature = "with-mac_address")]
                        "MACADDR" | "MACADDR8" => Value::MacAddress(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get mac address"),
                        ))),
                        #[cfg(all(feature = "with-mac_address", feature = "postgres-array"))]
                        "MACADDR[]" | "MACADDR8[]" => Value::Array(
                            sea_query::ArrayType::MacAddress,
                            Some(Box::new(
                                row.try_get::<Vec<mac_address::MacAddress>, _>(c.ordinal())
                                    .expect("Failed to get mac address array")
                                    .iter()
                                    .map(|val| Value::MacAddress(Some(Box::new(val.clone()))))
                                    .collect(),
                            )),
                        ),

                        #[cfg(feature = "with-chrono")]
                        "TIMESTAMP" => Value::ChronoDateTime(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get timestamp"),
                        ))),
                        #[cfg(all(feature = "with-time", not(feature = "with-chrono")))]
                        "TIMESTAMP" => Value::TimeDateTime(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get timestamp"),
                        ))),

                        #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                        "TIMESTAMP[]" => Value::Array(
                            sea_query::ArrayType::ChronoDateTime,
                            Some(Box::new(
                                row.try_get::<Vec<chrono::NaiveDateTime>, _>(c.ordinal())
                                    .expect("Failed to get timestamp array")
                                    .iter()
                                    .map(|val| Value::ChronoDateTime(Some(Box::new(val.clone()))))
                                    .collect(),
                            )),
                        ),
                        #[cfg(all(
                            feature = "with-time",
                            not(feature = "with-chrono"),
                            feature = "postgres-array"
                        ))]
                        "TIMESTAMP[]" => Value::Array(
                            sea_query::ArrayType::TimeDateTime,
                            Some(Box::new(
                                row.try_get::<Vec<time::OffsetDateTime>, _>(c.ordinal())
                                    .expect("Failed to get timestamp array")
                                    .iter()
                                    .map(|val| Value::TimeDateTime(Some(Box::new(val.clone()))))
                                    .collect(),
                            )),
                        ),

                        #[cfg(feature = "with-chrono")]
                        "DATE" => Value::ChronoDate(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get date"),
                        ))),
                        #[cfg(all(feature = "with-time", not(feature = "with-chrono")))]
                        "DATE" => Value::TimeDate(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get date"),
                        ))),

                        #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                        "DATE[]" => Value::Array(
                            sea_query::ArrayType::ChronoDate,
                            Some(Box::new(
                                row.try_get::<Vec<chrono::NaiveDate>, _>(c.ordinal())
                                    .expect("Failed to get date array")
                                    .iter()
                                    .map(|val| Value::ChronoDate(Some(Box::new(val.clone()))))
                                    .collect(),
                            )),
                        ),
                        #[cfg(all(
                            feature = "with-time",
                            not(feature = "with-chrono"),
                            feature = "postgres-array"
                        ))]
                        "DATE[]" => Value::Array(
                            sea_query::ArrayType::TimeDate,
                            Some(Box::new(
                                row.try_get::<Vec<time::Date>, _>(c.ordinal())
                                    .expect("Failed to get date array")
                                    .iter()
                                    .map(|val| Value::TimeDate(Some(Box::new(val.clone()))))
                                    .collect(),
                            )),
                        ),

                        #[cfg(feature = "with-chrono")]
                        "TIME" => Value::ChronoTime(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get time"),
                        ))),
                        #[cfg(all(feature = "with-time", not(feature = "with-chrono")))]
                        "TIME" => Value::TimeTime(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get time"),
                        ))),

                        #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                        "TIME[]" => Value::Array(
                            sea_query::ArrayType::ChronoTime,
                            Some(Box::new(
                                row.try_get::<Vec<chrono::NaiveTime>, _>(c.ordinal())
                                    .expect("Failed to get time array")
                                    .iter()
                                    .map(|val| Value::ChronoTime(Some(Box::new(val.clone()))))
                                    .collect(),
                            )),
                        ),
                        #[cfg(all(
                            feature = "with-time",
                            not(feature = "with-chrono"),
                            feature = "postgres-array"
                        ))]
                        "TIME[]" => Value::Array(
                            sea_query::ArrayType::TimeTime,
                            Some(Box::new(
                                row.try_get::<Vec<time::Time>, _>(c.ordinal())
                                    .expect("Failed to get time array")
                                    .iter()
                                    .map(|val| Value::TimeTime(Some(Box::new(val.clone()))))
                                    .collect(),
                            )),
                        ),

                        #[cfg(feature = "with-chrono")]
                        "TIMESTAMPTZ" => Value::ChronoDateTimeUtc(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get timestamptz"),
                        ))),
                        #[cfg(all(feature = "with-time", not(feature = "with-chrono")))]
                        "TIMESTAMPTZ" => Value::TimeDateTime(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get timestamptz"),
                        ))),

                        #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                        "TIMESTAMPTZ[]" => Value::Array(
                            sea_query::ArrayType::ChronoDateTimeUtc,
                            Some(Box::new(
                                row.try_get::<Vec<chrono::DateTime<chrono::Utc>>, _>(c.ordinal())
                                    .expect("Failed to get timestamptz array")
                                    .iter()
                                    .map(|val| {
                                        Value::ChronoDateTimeUtc(Some(Box::new(val.clone())))
                                    })
                                    .collect(),
                            )),
                        ),
                        #[cfg(all(
                            feature = "with-time",
                            not(feature = "with-chrono"),
                            feature = "postgres-array"
                        ))]
                        "TIMESTAMPTZ[]" => Value::Array(
                            sea_query::ArrayType::TimeDateTime,
                            Some(Box::new(
                                row.try_get::<Vec<time::OffsetDateTime>, _>(c.ordinal())
                                    .expect("Failed to get timestamptz array")
                                    .iter()
                                    .map(|val| Value::TimeDateTime(Some(Box::new(val.clone()))))
                                    .collect(),
                            )),
                        ),

                        #[cfg(feature = "with-chrono")]
                        "TIMETZ" => Value::ChronoTime(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get timetz"),
                        ))),
                        #[cfg(all(feature = "with-time", not(feature = "with-chrono")))]
                        "TIMETZ" => Value::TimeTime(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get timetz"),
                        ))),

                        #[cfg(all(feature = "with-chrono", feature = "postgres-array"))]
                        "TIMETZ[]" => Value::Array(
                            sea_query::ArrayType::ChronoTime,
                            Some(Box::new(
                                row.try_get::<Vec<chrono::NaiveTime>, _>(c.ordinal())
                                    .expect("Failed to get timetz array")
                                    .iter()
                                    .map(|val| Value::ChronoTime(Some(Box::new(val.clone()))))
                                    .collect(),
                            )),
                        ),
                        #[cfg(all(
                            feature = "with-time",
                            not(feature = "with-chrono"),
                            feature = "postgres-array"
                        ))]
                        "TIMETZ[]" => Value::Array(
                            sea_query::ArrayType::TimeTime,
                            Some(Box::new(
                                row.try_get::<Vec<time::Time>, _>(c.ordinal())
                                    .expect("Failed to get timetz array")
                                    .iter()
                                    .map(|val| Value::TimeTime(Some(Box::new(val.clone()))))
                                    .collect(),
                            )),
                        ),

                        #[cfg(feature = "with-uuid")]
                        "UUID" => Value::Uuid(Some(Box::new(
                            row.try_get(c.ordinal()).expect("Failed to get uuid"),
                        ))),

                        #[cfg(all(feature = "with-uuid", feature = "postgres-array"))]
                        "UUID[]" => Value::Array(
                            sea_query::ArrayType::Uuid,
                            Some(Box::new(
                                row.try_get::<Vec<uuid::Uuid>, _>(c.ordinal())
                                    .expect("Failed to get uuid array")
                                    .iter()
                                    .map(|val| Value::Uuid(Some(Box::new(val.clone()))))
                                    .collect(),
                            )),
                        ),

                        _ => unreachable!("Unknown column type: {}", c.type_info().name()),
                    },
                )
            })
            .collect(),
    }
}
