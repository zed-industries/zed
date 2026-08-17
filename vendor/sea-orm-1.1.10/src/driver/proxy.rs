use crate::{
    debug_print, error::*, DatabaseConnection, DbBackend, ExecResult, ProxyDatabaseTrait,
    QueryResult, Statement,
};
use std::{fmt::Debug, sync::Arc};
use tracing::instrument;

/// Defines a database driver for the [ProxyDatabase]
#[derive(Debug)]
pub struct ProxyDatabaseConnector;

/// Defines a connection for the [ProxyDatabase]
#[derive(Debug)]
pub struct ProxyDatabaseConnection {
    db_backend: DbBackend,
    proxy: Arc<Box<dyn ProxyDatabaseTrait>>,
}

impl ProxyDatabaseConnector {
    /// Check if the database URI given and the [DatabaseBackend](crate::DatabaseBackend) selected are the same
    #[allow(unused_variables)]
    pub fn accepts(string: &str) -> bool {
        // As this is a proxy database, it accepts any URI
        true
    }

    /// Connect to the [ProxyDatabase]
    #[allow(unused_variables)]
    #[instrument(level = "trace")]
    pub fn connect(
        db_type: DbBackend,
        func: Arc<Box<dyn ProxyDatabaseTrait>>,
    ) -> Result<DatabaseConnection, DbErr> {
        Ok(DatabaseConnection::ProxyDatabaseConnection(Arc::new(
            ProxyDatabaseConnection::new(db_type, func),
        )))
    }
}

impl ProxyDatabaseConnection {
    /// Create a connection to the [ProxyDatabase]
    pub fn new(db_backend: DbBackend, funcs: Arc<Box<dyn ProxyDatabaseTrait>>) -> Self {
        Self {
            db_backend,
            proxy: funcs.to_owned(),
        }
    }

    /// Get the [DatabaseBackend](crate::DatabaseBackend) being used by the [ProxyDatabase]
    pub fn get_database_backend(&self) -> DbBackend {
        self.db_backend
    }

    /// Execute the SQL statement in the [ProxyDatabase]
    #[instrument(level = "trace")]
    pub async fn execute(&self, statement: Statement) -> Result<ExecResult, DbErr> {
        debug_print!("{}", statement);
        Ok(self.proxy.execute(statement).await?.into())
    }

    /// Return one [QueryResult] if the query was successful
    #[instrument(level = "trace")]
    pub async fn query_one(&self, statement: Statement) -> Result<Option<QueryResult>, DbErr> {
        debug_print!("{}", statement);
        let result = self.proxy.query(statement).await?;

        if let Some(first) = result.first() {
            return Ok(Some(QueryResult {
                row: crate::QueryResultRow::Proxy(first.to_owned()),
            }));
        } else {
            return Ok(None);
        }
    }

    /// Return all [QueryResult]s if the query was successful
    #[instrument(level = "trace")]
    pub async fn query_all(&self, statement: Statement) -> Result<Vec<QueryResult>, DbErr> {
        debug_print!("{}", statement);
        let result = self.proxy.query(statement).await?;

        Ok(result
            .into_iter()
            .map(|row| QueryResult {
                row: crate::QueryResultRow::Proxy(row),
            })
            .collect())
    }

    /// Create a statement block  of SQL statements that execute together.
    #[instrument(level = "trace")]
    pub async fn begin(&self) {
        self.proxy.begin().await
    }

    /// Commit a transaction atomically to the database
    #[instrument(level = "trace")]
    pub async fn commit(&self) {
        self.proxy.commit().await
    }

    /// Roll back a faulty transaction
    #[instrument(level = "trace")]
    pub async fn rollback(&self) {
        self.proxy.rollback().await
    }

    /// Checks if a connection to the database is still valid.
    pub async fn ping(&self) -> Result<(), DbErr> {
        self.proxy.ping().await
    }
}

impl
    From<(
        Arc<crate::ProxyDatabaseConnection>,
        Statement,
        Option<crate::metric::Callback>,
    )> for crate::QueryStream
{
    fn from(
        (conn, stmt, metric_callback): (
            Arc<crate::ProxyDatabaseConnection>,
            Statement,
            Option<crate::metric::Callback>,
        ),
    ) -> Self {
        crate::QueryStream::build(stmt, crate::InnerConnection::Proxy(conn), metric_callback)
    }
}

impl crate::DatabaseTransaction {
    pub(crate) async fn new_proxy(
        inner: Arc<crate::ProxyDatabaseConnection>,
        metric_callback: Option<crate::metric::Callback>,
    ) -> Result<crate::DatabaseTransaction, DbErr> {
        use futures_util::lock::Mutex;
        let backend = inner.get_database_backend();
        Self::begin(
            Arc::new(Mutex::new(crate::InnerConnection::Proxy(inner))),
            backend,
            metric_callback,
            None,
            None,
        )
        .await
    }
}
