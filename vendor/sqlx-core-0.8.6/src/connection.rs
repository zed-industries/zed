use crate::database::{Database, HasStatementCache};
use crate::error::Error;

use crate::transaction::{Transaction, TransactionManager};
use futures_core::future::BoxFuture;
use log::LevelFilter;
use std::borrow::Cow;
use std::fmt::Debug;
use std::str::FromStr;
use std::time::Duration;
use url::Url;

/// Represents a single database connection.
pub trait Connection: Send {
    type Database: Database<Connection = Self>;

    type Options: ConnectOptions<Connection = Self>;

    /// Explicitly close this database connection.
    ///
    /// This notifies the database server that the connection is closing so that it can
    /// free up any server-side resources in use.
    ///
    /// While connections can simply be dropped to clean up local resources,
    /// the `Drop` handler itself cannot notify the server that the connection is being closed
    /// because that may require I/O to send a termination message. That can result in a delay
    /// before the server learns that the connection is gone, usually from a TCP keepalive timeout.
    ///
    /// Creating and dropping many connections in short order without calling `.close()` may
    /// lead to errors from the database server because those senescent connections will still
    /// count against any connection limit or quota that is configured.
    ///
    /// Therefore it is recommended to call `.close()` on a connection when you are done using it
    /// and to `.await` the result to ensure the termination message is sent.
    fn close(self) -> BoxFuture<'static, Result<(), Error>>;

    /// Immediately close the connection without sending a graceful shutdown.
    ///
    /// This should still at least send a TCP `FIN` frame to let the server know we're dying.
    #[doc(hidden)]
    fn close_hard(self) -> BoxFuture<'static, Result<(), Error>>;

    /// Checks if a connection to the database is still valid.
    fn ping(&mut self) -> BoxFuture<'_, Result<(), Error>>;

    /// Begin a new transaction or establish a savepoint within the active transaction.
    ///
    /// Returns a [`Transaction`] for controlling and tracking the new transaction.
    fn begin(&mut self) -> BoxFuture<'_, Result<Transaction<'_, Self::Database>, Error>>
    where
        Self: Sized;

    /// Begin a new transaction with a custom statement.
    ///
    /// Returns a [`Transaction`] for controlling and tracking the new transaction.
    ///
    /// Returns an error if the connection is already in a transaction or if
    /// `statement` does not put the connection into a transaction.
    fn begin_with(
        &mut self,
        statement: impl Into<Cow<'static, str>>,
    ) -> BoxFuture<'_, Result<Transaction<'_, Self::Database>, Error>>
    where
        Self: Sized,
    {
        Transaction::begin(self, Some(statement.into()))
    }

    /// Returns `true` if the connection is currently in a transaction.
    ///
    /// # Note: Automatic Rollbacks May Not Be Counted
    /// Certain database errors (such as a serializable isolation failure)
    /// can cause automatic rollbacks of a transaction
    /// which may not be indicated in the return value of this method.
    #[inline]
    fn is_in_transaction(&self) -> bool {
        <Self::Database as Database>::TransactionManager::get_transaction_depth(self) != 0
    }

    /// Execute the function inside a transaction.
    ///
    /// If the function returns an error, the transaction will be rolled back. If it does not
    /// return an error, the transaction will be committed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use sqlx::postgres::{PgConnection, PgRow};
    /// use sqlx::Connection;
    ///
    /// # pub async fn _f(conn: &mut PgConnection) -> sqlx::Result<Vec<PgRow>> {
    /// conn.transaction(|txn| Box::pin(async move {
    ///     sqlx::query("select * from ..").fetch_all(&mut **txn).await
    /// })).await
    /// # }
    /// ```
    fn transaction<'a, F, R, E>(&'a mut self, callback: F) -> BoxFuture<'a, Result<R, E>>
    where
        for<'c> F: FnOnce(&'c mut Transaction<'_, Self::Database>) -> BoxFuture<'c, Result<R, E>>
            + 'a
            + Send
            + Sync,
        Self: Sized,
        R: Send,
        E: From<Error> + Send,
    {
        Box::pin(async move {
            let mut transaction = self.begin().await?;
            let ret = callback(&mut transaction).await;

            match ret {
                Ok(ret) => {
                    transaction.commit().await?;

                    Ok(ret)
                }
                Err(err) => {
                    transaction.rollback().await?;

                    Err(err)
                }
            }
        })
    }

    /// The number of statements currently cached in the connection.
    fn cached_statements_size(&self) -> usize
    where
        Self::Database: HasStatementCache,
    {
        0
    }

    /// Removes all statements from the cache, closing them on the server if
    /// needed.
    fn clear_cached_statements(&mut self) -> BoxFuture<'_, Result<(), Error>>
    where
        Self::Database: HasStatementCache,
    {
        Box::pin(async move { Ok(()) })
    }

    /// Restore any buffers in the connection to their default capacity, if possible.
    ///
    /// Sending a large query or receiving a resultset with many columns can cause the connection
    /// to allocate additional buffer space to fit the data which is retained afterwards in
    /// case it's needed again. This can give the outward appearance of a memory leak, but is
    /// in fact the intended behavior.
    ///
    /// Calling this method tells the connection to release that excess memory if it can,
    /// though be aware that calling this too often can cause unnecessary thrashing or
    /// fragmentation in the global allocator. If there's still data in the connection buffers
    /// (unlikely if the last query was run to completion) then it may need to be moved to
    /// allow the buffers to shrink.
    fn shrink_buffers(&mut self);

    #[doc(hidden)]
    fn flush(&mut self) -> BoxFuture<'_, Result<(), Error>>;

    #[doc(hidden)]
    fn should_flush(&self) -> bool;

    /// Establish a new database connection.
    ///
    /// A value of [`Options`][Self::Options] is parsed from the provided connection string. This parsing
    /// is database-specific.
    #[inline]
    fn connect(url: &str) -> BoxFuture<'static, Result<Self, Error>>
    where
        Self: Sized,
    {
        let options = url.parse();

        Box::pin(async move { Self::connect_with(&options?).await })
    }

    /// Establish a new database connection with the provided options.
    fn connect_with(options: &Self::Options) -> BoxFuture<'_, Result<Self, Error>>
    where
        Self: Sized,
    {
        options.connect()
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct LogSettings {
    pub statements_level: LevelFilter,
    pub slow_statements_level: LevelFilter,
    pub slow_statements_duration: Duration,
}

impl Default for LogSettings {
    fn default() -> Self {
        LogSettings {
            statements_level: LevelFilter::Debug,
            slow_statements_level: LevelFilter::Warn,
            slow_statements_duration: Duration::from_secs(1),
        }
    }
}

impl LogSettings {
    pub fn log_statements(&mut self, level: LevelFilter) {
        self.statements_level = level;
    }
    pub fn log_slow_statements(&mut self, level: LevelFilter, duration: Duration) {
        self.slow_statements_level = level;
        self.slow_statements_duration = duration;
    }
}

pub trait ConnectOptions: 'static + Send + Sync + FromStr<Err = Error> + Debug + Clone {
    type Connection: Connection<Options = Self> + ?Sized;

    /// Parse the `ConnectOptions` from a URL.
    fn from_url(url: &Url) -> Result<Self, Error>;

    /// Get a connection URL that may be used to connect to the same database as this `ConnectOptions`.
    ///
    /// ### Note: Lossy
    /// Any flags or settings which do not have a representation in the URL format will be lost.
    /// They will fall back to their default settings when the URL is parsed.
    ///
    /// The only settings guaranteed to be preserved are:
    /// * Username
    /// * Password
    /// * Hostname
    /// * Port
    /// * Database name
    /// * Unix socket or SQLite database file path
    /// * SSL mode (if applicable)
    /// * SSL CA certificate path
    /// * SSL client certificate path
    /// * SSL client key path
    ///
    /// Additional settings are driver-specific. Refer to the source of a given implementation
    /// to see which options are preserved in the URL.
    ///
    /// ### Panics
    /// This defaults to `unimplemented!()`.
    ///
    /// Individual drivers should override this to implement the intended behavior.
    fn to_url_lossy(&self) -> Url {
        unimplemented!()
    }

    /// Establish a new database connection with the options specified by `self`.
    fn connect(&self) -> BoxFuture<'_, Result<Self::Connection, Error>>
    where
        Self::Connection: Sized;

    /// Log executed statements with the specified `level`
    fn log_statements(self, level: LevelFilter) -> Self;

    /// Log executed statements with a duration above the specified `duration`
    /// at the specified `level`.
    fn log_slow_statements(self, level: LevelFilter, duration: Duration) -> Self;

    /// Entirely disables statement logging (both slow and regular).
    fn disable_statement_logging(self) -> Self {
        self.log_statements(LevelFilter::Off)
            .log_slow_statements(LevelFilter::Off, Duration::default())
    }
}
