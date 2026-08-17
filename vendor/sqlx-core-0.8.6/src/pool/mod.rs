//! Provides the connection pool for asynchronous SQLx connections.
//!
//! Opening a database connection for each and every operation to the database can quickly
//! become expensive. Furthermore, sharing a database connection between threads and functions
//! can be difficult to express in Rust.
//!
//! A connection pool is a standard technique that can manage opening and re-using connections.
//! Normally it also enforces a maximum number of connections as these are an expensive resource
//! on the database server.
//!
//! SQLx provides a canonical connection pool implementation intended to satisfy the majority
//! of use cases.
//!
//! See [Pool] for details.
//!
//! Type aliases are provided for each database to make it easier to sprinkle `Pool` through
//! your codebase:
//!
//! * [MssqlPool][crate::mssql::MssqlPool] (MSSQL)
//! * [MySqlPool][crate::mysql::MySqlPool] (MySQL)
//! * [PgPool][crate::postgres::PgPool] (PostgreSQL)
//! * [SqlitePool][crate::sqlite::SqlitePool] (SQLite)
//!
//! # Opening a connection pool
//!
//! A new connection pool with a default configuration can be created by supplying `Pool`
//! with the database driver and a connection string.
//!
//! ```rust,ignore
//! use sqlx::Pool;
//! use sqlx::postgres::Postgres;
//!
//! let pool = Pool::<Postgres>::connect("postgres://").await?;
//! ```
//!
//! For convenience, database-specific type aliases are provided:
//!
//! ```rust,ignore
//! use sqlx::mssql::MssqlPool;
//!
//! let pool = MssqlPool::connect("mssql://").await?;
//! ```
//!
//! # Using a connection pool
//!
//! A connection pool implements [`Executor`][crate::executor::Executor] and can be used directly
//! when executing a query. Notice that only an immutable reference (`&Pool`) is needed.
//!
//! ```rust,ignore
//! sqlx::query("DELETE FROM articles").execute(&pool).await?;
//! ```
//!
//! A connection or transaction may also be manually acquired with
//! [`Pool::acquire`] or
//! [`Pool::begin`].

use std::borrow::Cow;
use std::fmt;
use std::future::Future;
use std::pin::{pin, Pin};
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use event_listener::EventListener;
use futures_core::FusedFuture;
use futures_util::FutureExt;

use crate::connection::Connection;
use crate::database::Database;
use crate::error::Error;
use crate::transaction::Transaction;

pub use self::connection::PoolConnection;
use self::inner::PoolInner;
#[doc(hidden)]
pub use self::maybe::MaybePoolConnection;
pub use self::options::{PoolConnectionMetadata, PoolOptions};

#[macro_use]
mod executor;

#[macro_use]
pub mod maybe;

mod connection;
mod inner;
mod options;

/// An asynchronous pool of SQLx database connections.
///
/// Create a pool with [Pool::connect] or [Pool::connect_with] and then call [Pool::acquire]
/// to get a connection from the pool; when the connection is dropped it will return to the pool
/// so it can be reused.
///
/// You can also pass `&Pool` directly anywhere an `Executor` is required; this will automatically
/// checkout a connection for you.
///
/// See [the module documentation](crate::pool) for examples.
///
/// The pool has a maximum connection limit that it will not exceed; if `acquire()` is called
/// when at this limit and all connections are checked out, the task will be made to wait until
/// a connection becomes available.
///
/// You can configure the connection limit, and other parameters, using [PoolOptions].
///
/// Calls to `acquire()` are fair, i.e. fulfilled on a first-come, first-serve basis.
///
/// `Pool` is `Send`, `Sync` and `Clone`. It is intended to be created once at the start of your
/// application/daemon/web server/etc. and then shared with all tasks throughout the process'
/// lifetime. How best to accomplish this depends on your program architecture.
///
/// In Actix-Web, for example, you can efficiently share a single pool with all request handlers
/// using [web::ThinData].
///
/// Cloning `Pool` is cheap as it is simply a reference-counted handle to the inner pool state.
/// When the last remaining handle to the pool is dropped, the connections owned by the pool are
/// immediately closed (also by dropping). `PoolConnection` returned by [Pool::acquire] and
/// `Transaction` returned by [Pool::begin] both implicitly hold a reference to the pool for
/// their lifetimes.
///
/// If you prefer to explicitly shutdown the pool and gracefully close its connections (which
/// depending on the database type, may include sending a message to the database server that the
/// connection is being closed), you can call [Pool::close] which causes all waiting and subsequent
/// calls to [Pool::acquire] to return [Error::PoolClosed], and waits until all connections have
/// been returned to the pool and gracefully closed.
///
/// Type aliases are provided for each database to make it easier to sprinkle `Pool` through
/// your codebase:
///
/// * [MssqlPool][crate::mssql::MssqlPool] (MSSQL)
/// * [MySqlPool][crate::mysql::MySqlPool] (MySQL)
/// * [PgPool][crate::postgres::PgPool] (PostgreSQL)
/// * [SqlitePool][crate::sqlite::SqlitePool] (SQLite)
///
/// [web::ThinData]: https://docs.rs/actix-web/4.9.0/actix_web/web/struct.ThinData.html
///
/// ### Note: Drop Behavior
/// Due to a lack of async `Drop`, dropping the last `Pool` handle may not immediately clean
/// up connections by itself. The connections will be dropped locally, which is sufficient for
/// SQLite, but for client/server databases like MySQL and Postgres, that only closes the
/// client side of the connection. The server will not know the connection is closed until
/// potentially much later: this is usually dictated by the TCP keepalive timeout in the server
/// settings.
///
/// Because the connection may not be cleaned up immediately on the server side, you may run
/// into errors regarding connection limits if you are creating and dropping many pools in short
/// order.
///
/// We recommend calling [`.close().await`] to gracefully close the pool and its connections
/// when you are done using it. This will also wake any tasks that are waiting on an `.acquire()`
/// call, so for long-lived applications it's a good idea to call `.close()` during shutdown.
///
/// If you're writing tests, consider using `#[sqlx::test]` which handles the lifetime of
/// the pool for you.
///
/// [`.close().await`]: Pool::close
///
/// ### Why Use a Pool?
///
/// A single database connection (in general) cannot be used by multiple threads simultaneously
/// for various reasons, but an application or web server will typically need to execute numerous
/// queries or commands concurrently (think of concurrent requests against a web server; many or all
/// of them will probably need to hit the database).
///
/// You could place the connection in a `Mutex` but this will make it a huge bottleneck.
///
/// Naively, you might also think to just open a new connection per request, but this
/// has a number of other caveats, generally due to the high overhead involved in working with
/// a fresh connection. Examples to follow.
///
/// Connection pools facilitate reuse of connections to _amortize_ these costs, helping to ensure
/// that you're not paying for them each time you need a connection.
///
/// ##### 1. Overhead of Opening a Connection
/// Opening a database connection is not exactly a cheap operation.
///
/// For SQLite, it means numerous requests to the filesystem and memory allocations, while for
/// server-based databases it involves performing DNS resolution, opening a new TCP connection and
/// allocating buffers.
///
/// Each connection involves a nontrivial allocation of resources for the database server, usually
/// including spawning a new thread or process specifically to handle the connection, both for
/// concurrency and isolation of faults.
///
/// Additionally, database connections typically involve a complex handshake including
/// authentication, negotiation regarding connection parameters (default character sets, timezones,
/// locales, supported features) and upgrades to encrypted tunnels.
///
/// If `acquire()` is called on a pool with all connections checked out but it is not yet at its
/// connection limit (see next section), then a new connection is immediately opened, so this pool
/// does not _automatically_ save you from the overhead of creating a new connection.
///
/// However, because this pool by design enforces _reuse_ of connections, this overhead cost
/// is not paid each and every time you need a connection. In fact, if you set
/// [the `min_connections` option in PoolOptions][PoolOptions::min_connections], the pool will
/// create that many connections up-front so that they are ready to go when a request comes in,
/// and maintain that number on a best-effort basis for consistent performance.
///
/// ##### 2. Connection Limits (MySQL, MSSQL, Postgres)
/// Database servers usually place hard limits on the number of connections that are allowed open at
/// any given time, to maintain performance targets and prevent excessive allocation of resources,
/// such as RAM, journal files, disk caches, etc.
///
/// These limits have different defaults per database flavor, and may vary between different
/// distributions of the same database, but are typically configurable on server start;
/// if you're paying for managed database hosting then the connection limit will typically vary with
/// your pricing tier.
///
/// In MySQL, the default limit is typically 150, plus 1 which is reserved for a user with the
/// `CONNECTION_ADMIN` privilege so you can still access the server to diagnose problems even
/// with all connections being used.
///
/// In MSSQL the only documentation for the default maximum limit is that it depends on the version
/// and server configuration.
///
/// In Postgres, the default limit is typically 100, minus 3 which are reserved for superusers
/// (putting the default limit for unprivileged users at 97 connections).
///
/// In any case, exceeding these limits results in an error when opening a new connection, which
/// in a web server context will turn into a `500 Internal Server Error` if not handled, but should
/// be turned into either `403 Forbidden` or `429 Too Many Requests` depending on your rate-limiting
/// scheme. However, in a web context, telling a client "go away, maybe try again later" results in
/// a sub-optimal user experience.
///
/// Instead, with a connection pool, clients are made to wait in a fair queue for a connection to
/// become available; by using a single connection pool for your whole application, you can ensure
/// that you don't exceed the connection limit of your database server while allowing response
/// time to degrade gracefully at high load.
///
/// Of course, if multiple applications are connecting to the same database server, then you
/// should ensure that the connection limits for all applications add up to your server's maximum
/// connections or less.
///
/// ##### 3. Resource Reuse
/// The first time you execute a query against your database, the database engine must first turn
/// the SQL into an actionable _query plan_ which it may then execute against the database. This
/// involves parsing the SQL query, validating and analyzing it, and in the case of Postgres 12+ and
/// SQLite, generating code to execute the query plan (native or bytecode, respectively).
///
/// These database servers provide a way to amortize this overhead by _preparing_ the query,
/// associating it with an object ID and placing its query plan in a cache to be referenced when
/// it is later executed.
///
/// Prepared statements have other features, like bind parameters, which make them safer and more
/// ergonomic to use as well. By design, SQLx pushes you towards using prepared queries/statements
/// via the [Query][crate::query::Query] API _et al._ and the `query!()` macro _et al._, for
/// reasons of safety, ergonomics, and efficiency.
///
/// However, because database connections are typically isolated from each other in the database
/// server (either by threads or separate processes entirely), they don't typically share prepared
/// statements between connections so this work must be redone _for each connection_.
///
/// As with section 1, by facilitating reuse of connections, `Pool` helps to ensure their prepared
/// statements (and thus cached query plans) can be reused as much as possible, thus amortizing
/// the overhead involved.
///
/// Depending on the database server, a connection will have caches for all kinds of other data as
/// well and queries will generally benefit from these caches being "warm" (populated with data).
pub struct Pool<DB: Database>(pub(crate) Arc<PoolInner<DB>>);

/// A future that resolves when the pool is closed.
///
/// See [`Pool::close_event()`] for details.
pub struct CloseEvent {
    listener: Option<EventListener>,
}

impl<DB: Database> Pool<DB> {
    /// Create a new connection pool with a default pool configuration and
    /// the given connection URL, and immediately establish one connection.
    ///
    /// Refer to the relevant `ConnectOptions` impl for your database for the expected URL format:
    ///
    /// * Postgres: [`PgConnectOptions`][crate::postgres::PgConnectOptions]
    /// * MySQL: [`MySqlConnectOptions`][crate::mysql::MySqlConnectOptions]
    /// * SQLite: [`SqliteConnectOptions`][crate::sqlite::SqliteConnectOptions]
    /// * MSSQL: [`MssqlConnectOptions`][crate::mssql::MssqlConnectOptions]
    ///
    /// The default configuration is mainly suited for testing and light-duty applications.
    /// For production applications, you'll likely want to make at least few tweaks.
    ///
    /// See [`PoolOptions::new()`] for details.
    pub async fn connect(url: &str) -> Result<Self, Error> {
        PoolOptions::<DB>::new().connect(url).await
    }

    /// Create a new connection pool with a default pool configuration and
    /// the given `ConnectOptions`, and immediately establish one connection.
    ///
    /// The default configuration is mainly suited for testing and light-duty applications.
    /// For production applications, you'll likely want to make at least few tweaks.
    ///
    /// See [`PoolOptions::new()`] for details.
    pub async fn connect_with(
        options: <DB::Connection as Connection>::Options,
    ) -> Result<Self, Error> {
        PoolOptions::<DB>::new().connect_with(options).await
    }

    /// Create a new connection pool with a default pool configuration and
    /// the given connection URL.
    ///
    /// The pool will establish connections only as needed.
    ///
    /// Refer to the relevant [`ConnectOptions`][crate::connection::ConnectOptions] impl for your database for the expected URL format:
    ///
    /// * Postgres: [`PgConnectOptions`][crate::postgres::PgConnectOptions]
    /// * MySQL: [`MySqlConnectOptions`][crate::mysql::MySqlConnectOptions]
    /// * SQLite: [`SqliteConnectOptions`][crate::sqlite::SqliteConnectOptions]
    /// * MSSQL: [`MssqlConnectOptions`][crate::mssql::MssqlConnectOptions]
    ///
    /// The default configuration is mainly suited for testing and light-duty applications.
    /// For production applications, you'll likely want to make at least few tweaks.
    ///
    /// See [`PoolOptions::new()`] for details.
    pub fn connect_lazy(url: &str) -> Result<Self, Error> {
        PoolOptions::<DB>::new().connect_lazy(url)
    }

    /// Create a new connection pool with a default pool configuration and
    /// the given `ConnectOptions`.
    ///
    /// The pool will establish connections only as needed.
    ///
    /// The default configuration is mainly suited for testing and light-duty applications.
    /// For production applications, you'll likely want to make at least few tweaks.
    ///
    /// See [`PoolOptions::new()`] for details.
    pub fn connect_lazy_with(options: <DB::Connection as Connection>::Options) -> Self {
        PoolOptions::<DB>::new().connect_lazy_with(options)
    }

    /// Retrieves a connection from the pool.
    ///
    /// The total time this method is allowed to execute is capped by
    /// [`PoolOptions::acquire_timeout`].
    /// If that timeout elapses, this will return [`Error::PoolClosed`].
    ///
    /// ### Note: Cancellation/Timeout May Drop Connections
    /// If `acquire` is cancelled or times out after it acquires a connection from the idle queue or
    /// opens a new one, it will drop that connection because we don't want to assume it
    /// is safe to return to the pool, and testing it to see if it's safe to release could introduce
    /// subtle bugs if not implemented correctly. To avoid that entirely, we've decided to not
    /// gracefully handle cancellation here.
    ///
    /// However, if your workload is sensitive to dropped connections such as using an in-memory
    /// SQLite database with a pool size of 1, you can pretty easily ensure that a cancelled
    /// `acquire()` call will never drop connections by tweaking your [`PoolOptions`]:
    ///
    /// * Set [`test_before_acquire(false)`][PoolOptions::test_before_acquire]
    /// * Never set [`before_acquire`][PoolOptions::before_acquire] or
    ///   [`after_connect`][PoolOptions::after_connect].
    ///
    /// This should eliminate any potential `.await` points between acquiring a connection and
    /// returning it.
    pub fn acquire(&self) -> impl Future<Output = Result<PoolConnection<DB>, Error>> + 'static {
        let shared = self.0.clone();
        async move { shared.acquire().await.map(|conn| conn.reattach()) }
    }

    /// Attempts to retrieve a connection from the pool if there is one available.
    ///
    /// Returns `None` immediately if there are no idle connections available in the pool
    /// or there are tasks waiting for a connection which have yet to wake.
    pub fn try_acquire(&self) -> Option<PoolConnection<DB>> {
        self.0.try_acquire().map(|conn| conn.into_live().reattach())
    }

    /// Retrieves a connection and immediately begins a new transaction.
    pub async fn begin(&self) -> Result<Transaction<'static, DB>, Error> {
        Transaction::begin(
            MaybePoolConnection::PoolConnection(self.acquire().await?),
            None,
        )
        .await
    }

    /// Attempts to retrieve a connection and immediately begins a new transaction if successful.
    pub async fn try_begin(&self) -> Result<Option<Transaction<'static, DB>>, Error> {
        match self.try_acquire() {
            Some(conn) => Transaction::begin(MaybePoolConnection::PoolConnection(conn), None)
                .await
                .map(Some),

            None => Ok(None),
        }
    }

    /// Retrieves a connection and immediately begins a new transaction using `statement`.
    pub async fn begin_with(
        &self,
        statement: impl Into<Cow<'static, str>>,
    ) -> Result<Transaction<'static, DB>, Error> {
        Transaction::begin(
            MaybePoolConnection::PoolConnection(self.acquire().await?),
            Some(statement.into()),
        )
        .await
    }

    /// Attempts to retrieve a connection and, if successful, immediately begins a new
    /// transaction using `statement`.
    pub async fn try_begin_with(
        &self,
        statement: impl Into<Cow<'static, str>>,
    ) -> Result<Option<Transaction<'static, DB>>, Error> {
        match self.try_acquire() {
            Some(conn) => Transaction::begin(
                MaybePoolConnection::PoolConnection(conn),
                Some(statement.into()),
            )
            .await
            .map(Some),

            None => Ok(None),
        }
    }

    /// Shut down the connection pool, immediately waking all tasks waiting for a connection.
    ///
    /// Upon calling this method, any currently waiting or subsequent calls to [`Pool::acquire`] and
    /// the like will immediately return [`Error::PoolClosed`] and no new connections will be opened.
    /// Checked-out connections are unaffected, but will be gracefully closed on-drop
    /// rather than being returned to the pool.
    ///
    /// Returns a `Future` which can be `.await`ed to ensure all connections are
    /// gracefully closed. It will first close any idle connections currently waiting in the pool,
    /// then wait for all checked-out connections to be returned or closed.
    ///
    /// Waiting for connections to be gracefully closed is optional, but will allow the database
    /// server to clean up the resources sooner rather than later. This is especially important
    /// for tests that create a new pool every time, otherwise you may see errors about connection
    /// limits being exhausted even when running tests in a single thread.
    ///
    /// If the returned `Future` is not run to completion, any remaining connections will be dropped
    /// when the last handle for the given pool instance is dropped, which could happen in a task
    /// spawned by `Pool` internally and so may be unpredictable otherwise.
    ///
    /// `.close()` may be safely called and `.await`ed on multiple handles concurrently.
    pub fn close(&self) -> impl Future<Output = ()> + '_ {
        self.0.close()
    }

    /// Returns `true` if [`.close()`][Pool::close] has been called on the pool, `false` otherwise.
    pub fn is_closed(&self) -> bool {
        self.0.is_closed()
    }

    /// Get a future that resolves when [`Pool::close()`] is called.
    ///
    /// If the pool is already closed, the future resolves immediately.
    ///
    /// This can be used to cancel long-running operations that hold onto a [`PoolConnection`]
    /// so they don't prevent the pool from closing (which would otherwise wait until all
    /// connections are returned).
    ///
    /// Examples
    /// ========
    /// These examples use Postgres and Tokio, but should suffice to demonstrate the concept.
    ///
    /// Do something when the pool is closed:
    /// ```rust,no_run
    /// # async fn bleh() -> sqlx::Result<()> {
    /// use sqlx::PgPool;
    ///
    /// let pool = PgPool::connect("postgresql://...").await?;
    ///
    /// let pool2 = pool.clone();
    ///
    /// tokio::spawn(async move {
    ///     // Demonstrates that `CloseEvent` is itself a `Future` you can wait on.
    ///     // This lets you implement any kind of on-close event that you like.
    ///     pool2.close_event().await;
    ///
    ///     println!("Pool is closing!");
    ///
    ///     // Imagine maybe recording application statistics or logging a report, etc.
    /// });
    ///
    /// // The rest of the application executes normally...
    ///
    /// // Close the pool before the application exits...
    /// pool.close().await;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Cancel a long-running operation:
    /// ```rust,no_run
    /// # async fn bleh() -> sqlx::Result<()> {
    /// use sqlx::{Executor, PgPool};
    ///
    /// let pool = PgPool::connect("postgresql://...").await?;
    ///
    /// let pool2 = pool.clone();
    ///
    /// tokio::spawn(async move {
    ///     // `do_until` yields the inner future's output wrapped in `sqlx::Result`,
    ///     // in this case giving a double-wrapped result.
    ///     let res: sqlx::Result<sqlx::Result<()>> = pool2.close_event().do_until(async {
    ///         // This statement normally won't return for 30 days!
    ///         // (Assuming the connection doesn't time out first, of course.)
    ///         pool2.execute("SELECT pg_sleep('30 days')").await?;
    ///
    ///         // If the pool is closed before the statement completes, this won't be printed.
    ///         // This is because `.do_until()` cancels the future it's given if the
    ///         // pool is closed first.
    ///         println!("Waited!");
    ///
    ///         Ok(())
    ///     }).await;
    ///
    ///     match res {
    ///         Ok(Ok(())) => println!("Wait succeeded"),
    ///         Ok(Err(e)) => println!("Error from inside do_until: {e:?}"),
    ///         Err(e) => println!("Error from do_until: {e:?}"),
    ///     }
    /// });
    ///
    /// // This normally wouldn't return until the above statement completed and the connection
    /// // was returned to the pool. However, thanks to `.do_until()`, the operation was
    /// // cancelled as soon as we called `.close().await`.
    /// pool.close().await;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn close_event(&self) -> CloseEvent {
        self.0.close_event()
    }

    /// Returns the number of connections currently active. This includes idle connections.
    pub fn size(&self) -> u32 {
        self.0.size()
    }

    /// Returns the number of connections active and idle (not in use).
    pub fn num_idle(&self) -> usize {
        self.0.num_idle()
    }

    /// Gets a clone of the connection options for this pool
    pub fn connect_options(&self) -> Arc<<DB::Connection as Connection>::Options> {
        self.0
            .connect_options
            .read()
            .expect("write-lock holder panicked")
            .clone()
    }

    /// Updates the connection options this pool will use when opening any future connections.  Any
    /// existing open connection in the pool will be left as-is.
    pub fn set_connect_options(&self, connect_options: <DB::Connection as Connection>::Options) {
        // technically write() could also panic if the current thread already holds the lock,
        // but because this method can't be re-entered by the same thread that shouldn't be a problem
        let mut guard = self
            .0
            .connect_options
            .write()
            .expect("write-lock holder panicked");
        *guard = Arc::new(connect_options);
    }

    /// Get the options for this pool
    pub fn options(&self) -> &PoolOptions<DB> {
        &self.0.options
    }
}

/// Returns a new [Pool] tied to the same shared connection pool.
impl<DB: Database> Clone for Pool<DB> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<DB: Database> fmt::Debug for Pool<DB> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Pool")
            .field("size", &self.0.size())
            .field("num_idle", &self.0.num_idle())
            .field("is_closed", &self.0.is_closed())
            .field("options", &self.0.options)
            .finish()
    }
}

impl CloseEvent {
    /// Execute the given future until it returns or the pool is closed.
    ///
    /// Cancels the future and returns `Err(PoolClosed)` if/when the pool is closed.
    /// If the pool was already closed, the future is never run.
    pub async fn do_until<Fut: Future>(&mut self, fut: Fut) -> Result<Fut::Output, Error> {
        // Check that the pool wasn't closed already.
        //
        // We use `poll_immediate()` as it will use the correct waker instead of
        // a no-op one like `.now_or_never()`, but it won't actually suspend execution here.
        futures_util::future::poll_immediate(&mut *self)
            .await
            .map_or(Ok(()), |_| Err(Error::PoolClosed))?;

        let mut fut = pin!(fut);

        // I find that this is clearer in intent than `futures_util::future::select()`
        // or `futures_util::select_biased!{}` (which isn't enabled anyway).
        std::future::poll_fn(|cx| {
            // Poll `fut` first as the wakeup event is more likely for it than `self`.
            if let Poll::Ready(ret) = fut.as_mut().poll(cx) {
                return Poll::Ready(Ok(ret));
            }

            // Can't really factor out mapping to `Err(Error::PoolClosed)` though it seems like
            // we should because that results in a different `Ok` type each time.
            //
            // Ideally we'd map to something like `Result<!, Error>` but using `!` as a type
            // is not allowed on stable Rust yet.
            self.poll_unpin(cx).map(|_| Err(Error::PoolClosed))
        })
        .await
    }
}

impl Future for CloseEvent {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(listener) = &mut self.listener {
            futures_core::ready!(listener.poll_unpin(cx));
        }

        // `EventListener` doesn't like being polled after it yields, and even if it did it
        // would probably just wait for the next event, neither of which we want.
        //
        // So this way, once we get our close event, we fuse this future to immediately return.
        self.listener = None;

        Poll::Ready(())
    }
}

impl FusedFuture for CloseEvent {
    fn is_terminated(&self) -> bool {
        self.listener.is_none()
    }
}

/// get the time between the deadline and now and use that as our timeout
///
/// returns `Error::PoolTimedOut` if the deadline is in the past
fn deadline_as_timeout(deadline: Instant) -> Result<Duration, Error> {
    deadline
        .checked_duration_since(Instant::now())
        .ok_or(Error::PoolTimedOut)
}

#[test]
#[allow(dead_code)]
fn assert_pool_traits() {
    fn assert_send_sync<T: Send + Sync>() {}
    fn assert_clone<T: Clone>() {}

    fn assert_pool<DB: Database>() {
        assert_send_sync::<Pool<DB>>();
        assert_clone::<Pool<DB>>();
    }
}
