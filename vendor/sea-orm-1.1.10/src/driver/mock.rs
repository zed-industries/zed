use crate::{
    debug_print, error::*, DatabaseConnection, DbBackend, ExecResult, MockDatabase, QueryResult,
    Statement, Transaction,
};
use futures_util::Stream;
use std::{
    fmt::Debug,
    pin::Pin,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};
use tracing::instrument;

/// Defines a database driver for the [MockDatabase]
#[derive(Debug)]
pub struct MockDatabaseConnector;

/// Defines a connection for the [MockDatabase]
#[derive(Debug)]
pub struct MockDatabaseConnection {
    execute_counter: AtomicUsize,
    query_counter: AtomicUsize,
    mocker: Mutex<Box<dyn MockDatabaseTrait>>,
}

/// A Trait for any type wanting to perform operations on the [MockDatabase]
pub trait MockDatabaseTrait: Send + Debug {
    /// Execute a statement in the [MockDatabase]
    fn execute(&mut self, counter: usize, stmt: Statement) -> Result<ExecResult, DbErr>;

    /// Execute a SQL query in the [MockDatabase]
    fn query(&mut self, counter: usize, stmt: Statement) -> Result<Vec<QueryResult>, DbErr>;

    /// Create a transaction that can be committed atomically
    fn begin(&mut self);

    /// Commit a successful transaction atomically into the [MockDatabase]
    fn commit(&mut self);

    /// Roll back a transaction since errors were encountered
    fn rollback(&mut self);

    /// Get all logs from a [MockDatabase] and return a [Transaction]
    fn drain_transaction_log(&mut self) -> Vec<Transaction>;

    /// Get the backend being used in the [MockDatabase]
    fn get_database_backend(&self) -> DbBackend;

    /// Ping the [MockDatabase]
    fn ping(&self) -> Result<(), DbErr>;
}

impl MockDatabaseConnector {
    /// Check if the database URI given and the [DatabaseBackend](crate::DatabaseBackend) selected are the same
    #[allow(unused_variables)]
    pub fn accepts(string: &str) -> bool {
        #[cfg(feature = "sqlx-mysql")]
        if DbBackend::MySql.is_prefix_of(string) {
            return true;
        }
        #[cfg(feature = "sqlx-postgres")]
        if DbBackend::Postgres.is_prefix_of(string) {
            return true;
        }
        #[cfg(feature = "sqlx-sqlite")]
        if DbBackend::Sqlite.is_prefix_of(string) {
            return true;
        }
        false
    }

    /// Connect to the [MockDatabase]
    #[allow(unused_variables)]
    #[instrument(level = "trace")]
    pub async fn connect(string: &str) -> Result<DatabaseConnection, DbErr> {
        macro_rules! connect_mock_db {
            ( $syntax: expr ) => {
                Ok(DatabaseConnection::MockDatabaseConnection(Arc::new(
                    MockDatabaseConnection::new(MockDatabase::new($syntax)),
                )))
            };
        }

        #[cfg(feature = "sqlx-mysql")]
        if crate::SqlxMySqlConnector::accepts(string) {
            return connect_mock_db!(DbBackend::MySql);
        }
        #[cfg(feature = "sqlx-postgres")]
        if crate::SqlxPostgresConnector::accepts(string) {
            return connect_mock_db!(DbBackend::Postgres);
        }
        #[cfg(feature = "sqlx-sqlite")]
        if crate::SqlxSqliteConnector::accepts(string) {
            return connect_mock_db!(DbBackend::Sqlite);
        }
        connect_mock_db!(DbBackend::Postgres)
    }
}

impl MockDatabaseConnection {
    /// Create a connection to the [MockDatabase]
    pub fn new<M: 'static>(m: M) -> Self
    where
        M: MockDatabaseTrait,
    {
        Self {
            execute_counter: AtomicUsize::new(0),
            query_counter: AtomicUsize::new(0),
            mocker: Mutex::new(Box::new(m)),
        }
    }

    pub(crate) fn get_mocker_mutex(&self) -> &Mutex<Box<dyn MockDatabaseTrait>> {
        &self.mocker
    }

    /// Get the [DatabaseBackend](crate::DatabaseBackend) being used by the [MockDatabase]
    ///
    /// # Panics
    ///
    /// Will panic if the lock cannot be acquired.
    pub fn get_database_backend(&self) -> DbBackend {
        self.mocker
            .lock()
            .expect("Fail to acquire mocker")
            .get_database_backend()
    }

    /// Execute the SQL statement in the [MockDatabase]
    #[instrument(level = "trace")]
    pub fn execute(&self, statement: Statement) -> Result<ExecResult, DbErr> {
        debug_print!("{}", statement);
        let counter = self.execute_counter.fetch_add(1, Ordering::SeqCst);
        self.mocker
            .lock()
            .map_err(exec_err)?
            .execute(counter, statement)
    }

    /// Return one [QueryResult] if the query was successful
    #[instrument(level = "trace")]
    pub fn query_one(&self, statement: Statement) -> Result<Option<QueryResult>, DbErr> {
        debug_print!("{}", statement);
        let counter = self.query_counter.fetch_add(1, Ordering::SeqCst);
        let result = self
            .mocker
            .lock()
            .map_err(query_err)?
            .query(counter, statement)?;
        Ok(result.into_iter().next())
    }

    /// Return all [QueryResult]s if the query was successful
    #[instrument(level = "trace")]
    pub fn query_all(&self, statement: Statement) -> Result<Vec<QueryResult>, DbErr> {
        debug_print!("{}", statement);
        let counter = self.query_counter.fetch_add(1, Ordering::SeqCst);
        self.mocker
            .lock()
            .map_err(query_err)?
            .query(counter, statement)
    }

    /// Return [QueryResult]s  from a multi-query operation
    #[instrument(level = "trace")]
    pub fn fetch(
        &self,
        statement: &Statement,
    ) -> Pin<Box<dyn Stream<Item = Result<QueryResult, DbErr>> + Send>> {
        match self.query_all(statement.clone()) {
            Ok(v) => Box::pin(futures_util::stream::iter(v.into_iter().map(Ok))),
            Err(e) => Box::pin(futures_util::stream::iter(Some(Err(e)).into_iter())),
        }
    }

    /// Create a statement block  of SQL statements that execute together.
    #[instrument(level = "trace")]
    pub fn begin(&self) {
        self.mocker
            .lock()
            .expect("Failed to acquire mocker")
            .begin()
    }

    /// Commit a transaction atomically to the database
    #[instrument(level = "trace")]
    pub fn commit(&self) {
        self.mocker
            .lock()
            .expect("Failed to acquire mocker")
            .commit()
    }

    /// Roll back a faulty transaction
    #[instrument(level = "trace")]
    pub fn rollback(&self) {
        self.mocker
            .lock()
            .expect("Failed to acquire mocker")
            .rollback()
    }

    /// Checks if a connection to the database is still valid.
    pub fn ping(&self) -> Result<(), DbErr> {
        self.mocker.lock().map_err(query_err)?.ping()
    }
}

impl
    From<(
        Arc<crate::MockDatabaseConnection>,
        Statement,
        Option<crate::metric::Callback>,
    )> for crate::QueryStream
{
    fn from(
        (conn, stmt, metric_callback): (
            Arc<crate::MockDatabaseConnection>,
            Statement,
            Option<crate::metric::Callback>,
        ),
    ) -> Self {
        crate::QueryStream::build(stmt, crate::InnerConnection::Mock(conn), metric_callback)
    }
}

impl crate::DatabaseTransaction {
    pub(crate) async fn new_mock(
        inner: Arc<crate::MockDatabaseConnection>,
        metric_callback: Option<crate::metric::Callback>,
    ) -> Result<crate::DatabaseTransaction, DbErr> {
        use futures_util::lock::Mutex;
        let backend = inner.get_database_backend();
        Self::begin(
            Arc::new(Mutex::new(crate::InnerConnection::Mock(inner))),
            backend,
            metric_callback,
            None,
            None,
        )
        .await
    }
}
