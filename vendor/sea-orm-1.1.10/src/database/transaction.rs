use crate::{
    debug_print, error::*, AccessMode, ConnectionTrait, DbBackend, DbErr, ExecResult,
    InnerConnection, IsolationLevel, QueryResult, Statement, StreamTrait, TransactionStream,
    TransactionTrait,
};
#[cfg(feature = "sqlx-dep")]
use crate::{sqlx_error_to_exec_err, sqlx_error_to_query_err};
use futures_util::lock::Mutex;
#[cfg(feature = "sqlx-dep")]
use sqlx::TransactionManager;
use std::{future::Future, pin::Pin, sync::Arc};
use tracing::instrument;

// a Transaction is just a sugar for a connection where START TRANSACTION has been executed
/// Defines a database transaction, whether it is an open transaction and the type of
/// backend to use
pub struct DatabaseTransaction {
    conn: Arc<Mutex<InnerConnection>>,
    backend: DbBackend,
    open: bool,
    metric_callback: Option<crate::metric::Callback>,
}

impl std::fmt::Debug for DatabaseTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DatabaseTransaction")
    }
}

impl DatabaseTransaction {
    #[instrument(level = "trace", skip(metric_callback))]
    pub(crate) async fn begin(
        conn: Arc<Mutex<InnerConnection>>,
        backend: DbBackend,
        metric_callback: Option<crate::metric::Callback>,
        isolation_level: Option<IsolationLevel>,
        access_mode: Option<AccessMode>,
    ) -> Result<DatabaseTransaction, DbErr> {
        let res = DatabaseTransaction {
            conn,
            backend,
            open: true,
            metric_callback,
        };
        match *res.conn.lock().await {
            #[cfg(feature = "sqlx-mysql")]
            InnerConnection::MySql(ref mut c) => {
                // in MySQL SET TRANSACTION operations must be executed before transaction start
                crate::driver::sqlx_mysql::set_transaction_config(c, isolation_level, access_mode)
                    .await?;
                <sqlx::MySql as sqlx::Database>::TransactionManager::begin(c, None)
                    .await
                    .map_err(sqlx_error_to_query_err)
            }
            #[cfg(feature = "sqlx-postgres")]
            InnerConnection::Postgres(ref mut c) => {
                <sqlx::Postgres as sqlx::Database>::TransactionManager::begin(c, None)
                    .await
                    .map_err(sqlx_error_to_query_err)?;
                // in PostgreSQL SET TRANSACTION operations must be executed inside transaction
                crate::driver::sqlx_postgres::set_transaction_config(
                    c,
                    isolation_level,
                    access_mode,
                )
                .await
            }
            #[cfg(feature = "sqlx-sqlite")]
            InnerConnection::Sqlite(ref mut c) => {
                // in SQLite isolation level and access mode are global settings
                crate::driver::sqlx_sqlite::set_transaction_config(c, isolation_level, access_mode)
                    .await?;
                <sqlx::Sqlite as sqlx::Database>::TransactionManager::begin(c, None)
                    .await
                    .map_err(sqlx_error_to_query_err)
            }
            #[cfg(feature = "mock")]
            InnerConnection::Mock(ref mut c) => {
                c.begin();
                Ok(())
            }
            #[allow(unreachable_patterns)]
            _ => Err(conn_err("Disconnected")),
        }?;
        Ok(res)
    }

    /// Runs a transaction to completion returning an rolling back the transaction on
    /// encountering an error if it fails
    #[instrument(level = "trace", skip(callback))]
    pub(crate) async fn run<F, T, E>(self, callback: F) -> Result<T, TransactionError<E>>
    where
        F: for<'b> FnOnce(
                &'b DatabaseTransaction,
            ) -> Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'b>>
            + Send,
        T: Send,
        E: std::error::Error + Send,
    {
        let res = callback(&self).await.map_err(TransactionError::Transaction);
        if res.is_ok() {
            self.commit().await.map_err(TransactionError::Connection)?;
        } else {
            self.rollback()
                .await
                .map_err(TransactionError::Connection)?;
        }
        res
    }

    /// Commit a transaction atomically
    #[instrument(level = "trace")]
    #[allow(unreachable_code, unused_mut)]
    pub async fn commit(mut self) -> Result<(), DbErr> {
        match *self.conn.lock().await {
            #[cfg(feature = "sqlx-mysql")]
            InnerConnection::MySql(ref mut c) => {
                <sqlx::MySql as sqlx::Database>::TransactionManager::commit(c)
                    .await
                    .map_err(sqlx_error_to_query_err)
            }
            #[cfg(feature = "sqlx-postgres")]
            InnerConnection::Postgres(ref mut c) => {
                <sqlx::Postgres as sqlx::Database>::TransactionManager::commit(c)
                    .await
                    .map_err(sqlx_error_to_query_err)
            }
            #[cfg(feature = "sqlx-sqlite")]
            InnerConnection::Sqlite(ref mut c) => {
                <sqlx::Sqlite as sqlx::Database>::TransactionManager::commit(c)
                    .await
                    .map_err(sqlx_error_to_query_err)
            }
            #[cfg(feature = "mock")]
            InnerConnection::Mock(ref mut c) => {
                c.commit();
                Ok(())
            }
            #[allow(unreachable_patterns)]
            _ => Err(conn_err("Disconnected")),
        }?;
        self.open = false;
        Ok(())
    }

    /// rolls back a transaction in case error are encountered during the operation
    #[instrument(level = "trace")]
    #[allow(unreachable_code, unused_mut)]
    pub async fn rollback(mut self) -> Result<(), DbErr> {
        match *self.conn.lock().await {
            #[cfg(feature = "sqlx-mysql")]
            InnerConnection::MySql(ref mut c) => {
                <sqlx::MySql as sqlx::Database>::TransactionManager::rollback(c)
                    .await
                    .map_err(sqlx_error_to_query_err)
            }
            #[cfg(feature = "sqlx-postgres")]
            InnerConnection::Postgres(ref mut c) => {
                <sqlx::Postgres as sqlx::Database>::TransactionManager::rollback(c)
                    .await
                    .map_err(sqlx_error_to_query_err)
            }
            #[cfg(feature = "sqlx-sqlite")]
            InnerConnection::Sqlite(ref mut c) => {
                <sqlx::Sqlite as sqlx::Database>::TransactionManager::rollback(c)
                    .await
                    .map_err(sqlx_error_to_query_err)
            }
            #[cfg(feature = "mock")]
            InnerConnection::Mock(ref mut c) => {
                c.rollback();
                Ok(())
            }
            #[allow(unreachable_patterns)]
            _ => Err(conn_err("Disconnected")),
        }?;
        self.open = false;
        Ok(())
    }

    // the rollback is queued and will be performed on next async operation, like returning the connection to the pool
    #[instrument(level = "trace")]
    fn start_rollback(&mut self) -> Result<(), DbErr> {
        if self.open {
            if let Some(mut conn) = self.conn.try_lock() {
                match &mut *conn {
                    #[cfg(feature = "sqlx-mysql")]
                    InnerConnection::MySql(c) => {
                        <sqlx::MySql as sqlx::Database>::TransactionManager::start_rollback(c);
                    }
                    #[cfg(feature = "sqlx-postgres")]
                    InnerConnection::Postgres(c) => {
                        <sqlx::Postgres as sqlx::Database>::TransactionManager::start_rollback(c);
                    }
                    #[cfg(feature = "sqlx-sqlite")]
                    InnerConnection::Sqlite(c) => {
                        <sqlx::Sqlite as sqlx::Database>::TransactionManager::start_rollback(c);
                    }
                    #[cfg(feature = "mock")]
                    InnerConnection::Mock(c) => {
                        c.rollback();
                    }
                    #[allow(unreachable_patterns)]
                    _ => return Err(conn_err("Disconnected")),
                }
            } else {
                //this should never happen
                return Err(conn_err("Dropping a locked Transaction"));
            }
        }
        Ok(())
    }
}

impl Drop for DatabaseTransaction {
    fn drop(&mut self) {
        self.start_rollback().expect("Fail to rollback transaction");
    }
}

#[async_trait::async_trait]
impl ConnectionTrait for DatabaseTransaction {
    fn get_database_backend(&self) -> DbBackend {
        // this way we don't need to lock
        self.backend
    }

    #[instrument(level = "trace")]
    #[allow(unused_variables)]
    async fn execute(&self, stmt: Statement) -> Result<ExecResult, DbErr> {
        debug_print!("{}", stmt);

        match &mut *self.conn.lock().await {
            #[cfg(feature = "sqlx-mysql")]
            InnerConnection::MySql(conn) => {
                let query = crate::driver::sqlx_mysql::sqlx_query(&stmt);
                let conn: &mut sqlx::MySqlConnection = &mut *conn;
                crate::metric::metric!(self.metric_callback, &stmt, {
                    query.execute(conn).await.map(Into::into)
                })
                .map_err(sqlx_error_to_exec_err)
            }
            #[cfg(feature = "sqlx-postgres")]
            InnerConnection::Postgres(conn) => {
                let query = crate::driver::sqlx_postgres::sqlx_query(&stmt);
                let conn: &mut sqlx::PgConnection = &mut *conn;
                crate::metric::metric!(self.metric_callback, &stmt, {
                    query.execute(conn).await.map(Into::into)
                })
                .map_err(sqlx_error_to_exec_err)
            }
            #[cfg(feature = "sqlx-sqlite")]
            InnerConnection::Sqlite(conn) => {
                let query = crate::driver::sqlx_sqlite::sqlx_query(&stmt);
                let conn: &mut sqlx::SqliteConnection = &mut *conn;
                crate::metric::metric!(self.metric_callback, &stmt, {
                    query.execute(conn).await.map(Into::into)
                })
                .map_err(sqlx_error_to_exec_err)
            }
            #[cfg(feature = "mock")]
            InnerConnection::Mock(conn) => return conn.execute(stmt),
            #[allow(unreachable_patterns)]
            _ => Err(conn_err("Disconnected")),
        }
    }

    #[instrument(level = "trace")]
    #[allow(unused_variables)]
    async fn execute_unprepared(&self, sql: &str) -> Result<ExecResult, DbErr> {
        debug_print!("{}", sql);

        match &mut *self.conn.lock().await {
            #[cfg(feature = "sqlx-mysql")]
            InnerConnection::MySql(conn) => {
                let conn: &mut sqlx::MySqlConnection = &mut *conn;
                sqlx::Executor::execute(conn, sql)
                    .await
                    .map(Into::into)
                    .map_err(sqlx_error_to_exec_err)
            }
            #[cfg(feature = "sqlx-postgres")]
            InnerConnection::Postgres(conn) => {
                let conn: &mut sqlx::PgConnection = &mut *conn;
                sqlx::Executor::execute(conn, sql)
                    .await
                    .map(Into::into)
                    .map_err(sqlx_error_to_exec_err)
            }
            #[cfg(feature = "sqlx-sqlite")]
            InnerConnection::Sqlite(conn) => {
                let conn: &mut sqlx::SqliteConnection = &mut *conn;
                sqlx::Executor::execute(conn, sql)
                    .await
                    .map(Into::into)
                    .map_err(sqlx_error_to_exec_err)
            }
            #[cfg(feature = "mock")]
            InnerConnection::Mock(conn) => {
                let db_backend = conn.get_database_backend();
                let stmt = Statement::from_string(db_backend, sql);
                conn.execute(stmt)
            }
            #[allow(unreachable_patterns)]
            _ => Err(conn_err("Disconnected")),
        }
    }

    #[instrument(level = "trace")]
    #[allow(unused_variables)]
    async fn query_one(&self, stmt: Statement) -> Result<Option<QueryResult>, DbErr> {
        debug_print!("{}", stmt);

        match &mut *self.conn.lock().await {
            #[cfg(feature = "sqlx-mysql")]
            InnerConnection::MySql(conn) => {
                let query = crate::driver::sqlx_mysql::sqlx_query(&stmt);
                let conn: &mut sqlx::MySqlConnection = &mut *conn;
                crate::metric::metric!(self.metric_callback, &stmt, {
                    crate::sqlx_map_err_ignore_not_found(
                        query.fetch_one(conn).await.map(|row| Some(row.into())),
                    )
                })
            }
            #[cfg(feature = "sqlx-postgres")]
            InnerConnection::Postgres(conn) => {
                let query = crate::driver::sqlx_postgres::sqlx_query(&stmt);
                let conn: &mut sqlx::PgConnection = &mut *conn;
                crate::metric::metric!(self.metric_callback, &stmt, {
                    crate::sqlx_map_err_ignore_not_found(
                        query.fetch_one(conn).await.map(|row| Some(row.into())),
                    )
                })
            }
            #[cfg(feature = "sqlx-sqlite")]
            InnerConnection::Sqlite(conn) => {
                let query = crate::driver::sqlx_sqlite::sqlx_query(&stmt);
                let conn: &mut sqlx::SqliteConnection = &mut *conn;
                crate::metric::metric!(self.metric_callback, &stmt, {
                    crate::sqlx_map_err_ignore_not_found(
                        query.fetch_one(conn).await.map(|row| Some(row.into())),
                    )
                })
            }
            #[cfg(feature = "mock")]
            InnerConnection::Mock(conn) => return conn.query_one(stmt),
            #[allow(unreachable_patterns)]
            _ => Err(conn_err("Disconnected")),
        }
    }

    #[instrument(level = "trace")]
    #[allow(unused_variables)]
    async fn query_all(&self, stmt: Statement) -> Result<Vec<QueryResult>, DbErr> {
        debug_print!("{}", stmt);

        match &mut *self.conn.lock().await {
            #[cfg(feature = "sqlx-mysql")]
            InnerConnection::MySql(conn) => {
                let query = crate::driver::sqlx_mysql::sqlx_query(&stmt);
                let conn: &mut sqlx::MySqlConnection = &mut *conn;
                crate::metric::metric!(self.metric_callback, &stmt, {
                    query
                        .fetch_all(conn)
                        .await
                        .map(|rows| rows.into_iter().map(|r| r.into()).collect())
                        .map_err(sqlx_error_to_query_err)
                })
            }
            #[cfg(feature = "sqlx-postgres")]
            InnerConnection::Postgres(conn) => {
                let query = crate::driver::sqlx_postgres::sqlx_query(&stmt);
                let conn: &mut sqlx::PgConnection = &mut *conn;
                crate::metric::metric!(self.metric_callback, &stmt, {
                    query
                        .fetch_all(conn)
                        .await
                        .map(|rows| rows.into_iter().map(|r| r.into()).collect())
                        .map_err(sqlx_error_to_query_err)
                })
            }
            #[cfg(feature = "sqlx-sqlite")]
            InnerConnection::Sqlite(conn) => {
                let query = crate::driver::sqlx_sqlite::sqlx_query(&stmt);
                let conn: &mut sqlx::SqliteConnection = &mut *conn;
                crate::metric::metric!(self.metric_callback, &stmt, {
                    query
                        .fetch_all(conn)
                        .await
                        .map(|rows| rows.into_iter().map(|r| r.into()).collect())
                        .map_err(sqlx_error_to_query_err)
                })
            }
            #[cfg(feature = "mock")]
            InnerConnection::Mock(conn) => return conn.query_all(stmt),
            #[allow(unreachable_patterns)]
            _ => Err(conn_err("Disconnected")),
        }
    }
}

impl StreamTrait for DatabaseTransaction {
    type Stream<'a> = TransactionStream<'a>;

    #[instrument(level = "trace")]
    fn stream<'a>(
        &'a self,
        stmt: Statement,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Stream<'a>, DbErr>> + 'a + Send>> {
        Box::pin(async move {
            let conn = self.conn.lock().await;
            Ok(crate::TransactionStream::build(
                conn,
                stmt,
                self.metric_callback.clone(),
            ))
        })
    }
}

#[async_trait::async_trait]
impl TransactionTrait for DatabaseTransaction {
    #[instrument(level = "trace")]
    async fn begin(&self) -> Result<DatabaseTransaction, DbErr> {
        DatabaseTransaction::begin(
            Arc::clone(&self.conn),
            self.backend,
            self.metric_callback.clone(),
            None,
            None,
        )
        .await
    }

    #[instrument(level = "trace")]
    async fn begin_with_config(
        &self,
        isolation_level: Option<IsolationLevel>,
        access_mode: Option<AccessMode>,
    ) -> Result<DatabaseTransaction, DbErr> {
        DatabaseTransaction::begin(
            Arc::clone(&self.conn),
            self.backend,
            self.metric_callback.clone(),
            isolation_level,
            access_mode,
        )
        .await
    }

    /// Execute the function inside a transaction.
    /// If the function returns an error, the transaction will be rolled back. If it does not return an error, the transaction will be committed.
    #[instrument(level = "trace", skip(_callback))]
    async fn transaction<F, T, E>(&self, _callback: F) -> Result<T, TransactionError<E>>
    where
        F: for<'c> FnOnce(
                &'c DatabaseTransaction,
            ) -> Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'c>>
            + Send,
        T: Send,
        E: std::error::Error + Send,
    {
        let transaction = self.begin().await.map_err(TransactionError::Connection)?;
        transaction.run(_callback).await
    }

    /// Execute the function inside a transaction with isolation level and/or access mode.
    /// If the function returns an error, the transaction will be rolled back. If it does not return an error, the transaction will be committed.
    #[instrument(level = "trace", skip(_callback))]
    async fn transaction_with_config<F, T, E>(
        &self,
        _callback: F,
        isolation_level: Option<IsolationLevel>,
        access_mode: Option<AccessMode>,
    ) -> Result<T, TransactionError<E>>
    where
        F: for<'c> FnOnce(
                &'c DatabaseTransaction,
            ) -> Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'c>>
            + Send,
        T: Send,
        E: std::error::Error + Send,
    {
        let transaction = self
            .begin_with_config(isolation_level, access_mode)
            .await
            .map_err(TransactionError::Connection)?;
        transaction.run(_callback).await
    }
}

/// Defines errors for handling transaction failures
#[derive(Debug)]
pub enum TransactionError<E>
where
    E: std::error::Error,
{
    /// A Database connection error
    Connection(DbErr),
    /// An error occurring when doing database transactions
    Transaction(E),
}

impl<E> std::fmt::Display for TransactionError<E>
where
    E: std::error::Error,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionError::Connection(e) => std::fmt::Display::fmt(e, f),
            TransactionError::Transaction(e) => std::fmt::Display::fmt(e, f),
        }
    }
}

impl<E> std::error::Error for TransactionError<E> where E: std::error::Error {}

impl<E> From<DbErr> for TransactionError<E>
where
    E: std::error::Error,
{
    fn from(e: DbErr) -> Self {
        Self::Connection(e)
    }
}
