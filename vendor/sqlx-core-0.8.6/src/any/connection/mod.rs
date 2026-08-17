use futures_core::future::BoxFuture;
use std::borrow::Cow;

use crate::any::{Any, AnyConnectOptions};
use crate::connection::{ConnectOptions, Connection};
use crate::error::Error;

use crate::database::Database;
pub use backend::AnyConnectionBackend;

use crate::transaction::Transaction;

mod backend;
mod executor;

/// A connection to _any_ SQLx database.
///
/// The database driver used is determined by the scheme
/// of the connection url.
///
/// ```text
/// postgres://postgres@localhost/test
/// sqlite://a.sqlite
/// ```
#[derive(Debug)]
pub struct AnyConnection {
    pub(crate) backend: Box<dyn AnyConnectionBackend>,
}

impl AnyConnection {
    /// Returns the name of the database backend in use (e.g. PostgreSQL, MySQL, SQLite, etc.)
    pub fn backend_name(&self) -> &str {
        self.backend.name()
    }

    pub(crate) fn connect(options: &AnyConnectOptions) -> BoxFuture<'_, crate::Result<Self>> {
        Box::pin(async {
            let driver = crate::any::driver::from_url(&options.database_url)?;
            (driver.connect)(options).await
        })
    }

    pub(crate) fn connect_with_db<DB: Database>(
        options: &AnyConnectOptions,
    ) -> BoxFuture<'_, crate::Result<Self>>
    where
        DB::Connection: AnyConnectionBackend,
        <DB::Connection as Connection>::Options:
            for<'a> TryFrom<&'a AnyConnectOptions, Error = Error>,
    {
        let res = TryFrom::try_from(options);

        Box::pin(async {
            let options: <DB::Connection as Connection>::Options = res?;

            Ok(AnyConnection {
                backend: Box::new(options.connect().await?),
            })
        })
    }

    #[cfg(feature = "migrate")]
    pub(crate) fn get_migrate(
        &mut self,
    ) -> crate::Result<&mut (dyn crate::migrate::Migrate + Send + 'static)> {
        self.backend.as_migrate()
    }
}

impl Connection for AnyConnection {
    type Database = Any;

    type Options = AnyConnectOptions;

    fn close(self) -> BoxFuture<'static, Result<(), Error>> {
        self.backend.close()
    }

    fn close_hard(self) -> BoxFuture<'static, Result<(), Error>> {
        self.backend.close()
    }

    fn ping(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        self.backend.ping()
    }

    fn begin(&mut self) -> BoxFuture<'_, Result<Transaction<'_, Self::Database>, Error>>
    where
        Self: Sized,
    {
        Transaction::begin(self, None)
    }

    fn begin_with(
        &mut self,
        statement: impl Into<Cow<'static, str>>,
    ) -> BoxFuture<'_, Result<Transaction<'_, Self::Database>, Error>>
    where
        Self: Sized,
    {
        Transaction::begin(self, Some(statement.into()))
    }

    fn cached_statements_size(&self) -> usize {
        self.backend.cached_statements_size()
    }

    fn clear_cached_statements(&mut self) -> BoxFuture<'_, crate::Result<()>> {
        self.backend.clear_cached_statements()
    }

    fn shrink_buffers(&mut self) {
        self.backend.shrink_buffers()
    }

    #[doc(hidden)]
    fn flush(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        self.backend.flush()
    }

    #[doc(hidden)]
    fn should_flush(&self) -> bool {
        self.backend.should_flush()
    }
}
