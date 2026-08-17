use crate::{
    DatabaseTransaction, DbBackend, DbErr, ExecResult, QueryResult, Statement, TransactionError,
};
use futures_util::Stream;
use std::{future::Future, pin::Pin};

/// The generic API for a database connection that can perform query or execute statements.
/// It abstracts database connection and transaction
#[async_trait::async_trait]
pub trait ConnectionTrait: Sync {
    /// Fetch the database backend as specified in [DbBackend].
    /// This depends on feature flags enabled.
    fn get_database_backend(&self) -> DbBackend;

    /// Execute a [Statement]
    async fn execute(&self, stmt: Statement) -> Result<ExecResult, DbErr>;

    /// Execute a unprepared [Statement]
    async fn execute_unprepared(&self, sql: &str) -> Result<ExecResult, DbErr>;

    /// Execute a [Statement] and return a query
    async fn query_one(&self, stmt: Statement) -> Result<Option<QueryResult>, DbErr>;

    /// Execute a [Statement] and return a collection Vec<[QueryResult]> on success
    async fn query_all(&self, stmt: Statement) -> Result<Vec<QueryResult>, DbErr>;

    /// Check if the connection supports `RETURNING` syntax on insert and update
    fn support_returning(&self) -> bool {
        let db_backend = self.get_database_backend();
        db_backend.support_returning()
    }

    /// Check if the connection is a test connection for the Mock database
    fn is_mock_connection(&self) -> bool {
        false
    }
}

/// Stream query results
pub trait StreamTrait: Send + Sync {
    /// Create a stream for the [QueryResult]
    type Stream<'a>: Stream<Item = Result<QueryResult, DbErr>> + Send
    where
        Self: 'a;

    /// Execute a [Statement] and return a stream of results
    fn stream<'a>(
        &'a self,
        stmt: Statement,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Stream<'a>, DbErr>> + 'a + Send>>;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Isolation level
pub enum IsolationLevel {
    /// Consistent reads within the same transaction read the snapshot established by the first read.
    RepeatableRead,
    /// Each consistent read, even within the same transaction, sets and reads its own fresh snapshot.
    ReadCommitted,
    /// SELECT statements are performed in a nonlocking fashion, but a possible earlier version of a row might be used.
    ReadUncommitted,
    /// All statements of the current transaction can only see rows committed before the first query or data-modification statement was executed in this transaction.
    Serializable,
}

impl std::fmt::Display for IsolationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IsolationLevel::RepeatableRead => write!(f, "REPEATABLE READ"),
            IsolationLevel::ReadCommitted => write!(f, "READ COMMITTED"),
            IsolationLevel::ReadUncommitted => write!(f, "READ UNCOMMITTED"),
            IsolationLevel::Serializable => write!(f, "SERIALIZABLE"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Access mode
pub enum AccessMode {
    /// Data can't be modified in this transaction
    ReadOnly,
    /// Data can be modified in this transaction (default)
    ReadWrite,
}

impl std::fmt::Display for AccessMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccessMode::ReadOnly => write!(f, "READ ONLY"),
            AccessMode::ReadWrite => write!(f, "READ WRITE"),
        }
    }
}

/// Spawn database transaction
#[async_trait::async_trait]
pub trait TransactionTrait {
    /// Execute SQL `BEGIN` transaction.
    /// Returns a Transaction that can be committed or rolled back
    async fn begin(&self) -> Result<DatabaseTransaction, DbErr>;

    /// Execute SQL `BEGIN` transaction with isolation level and/or access mode.
    /// Returns a Transaction that can be committed or rolled back
    async fn begin_with_config(
        &self,
        isolation_level: Option<IsolationLevel>,
        access_mode: Option<AccessMode>,
    ) -> Result<DatabaseTransaction, DbErr>;

    /// Execute the function inside a transaction.
    /// If the function returns an error, the transaction will be rolled back. If it does not return an error, the transaction will be committed.
    async fn transaction<F, T, E>(&self, callback: F) -> Result<T, TransactionError<E>>
    where
        F: for<'c> FnOnce(
                &'c DatabaseTransaction,
            ) -> Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'c>>
            + Send,
        T: Send,
        E: std::error::Error + Send;

    /// Execute the function inside a transaction with isolation level and/or access mode.
    /// If the function returns an error, the transaction will be rolled back. If it does not return an error, the transaction will be committed.
    async fn transaction_with_config<F, T, E>(
        &self,
        callback: F,
        isolation_level: Option<IsolationLevel>,
        access_mode: Option<AccessMode>,
    ) -> Result<T, TransactionError<E>>
    where
        F: for<'c> FnOnce(
                &'c DatabaseTransaction,
            ) -> Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'c>>
            + Send,
        T: Send,
        E: std::error::Error + Send;
}
