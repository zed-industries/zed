use std::fmt::{self, Debug};
use std::io;
use std::str::from_utf8;

use futures_channel::mpsc;
use futures_core::future::BoxFuture;
use futures_core::stream::{BoxStream, Stream};
use futures_util::{FutureExt, StreamExt, TryFutureExt, TryStreamExt};
use sqlx_core::acquire::Acquire;
use sqlx_core::transaction::Transaction;
use sqlx_core::Either;
use tracing::Instrument;

use crate::describe::Describe;
use crate::error::Error;
use crate::executor::{Execute, Executor};
use crate::message::{BackendMessageFormat, Notification};
use crate::pool::PoolOptions;
use crate::pool::{Pool, PoolConnection};
use crate::{PgConnection, PgQueryResult, PgRow, PgStatement, PgTypeInfo, Postgres};

/// A stream of asynchronous notifications from Postgres.
///
/// This listener will auto-reconnect. If the active
/// connection being used ever dies, this listener will detect that event, create a
/// new connection, will re-subscribe to all of the originally specified channels, and will resume
/// operations as normal.
pub struct PgListener {
    pool: Pool<Postgres>,
    connection: Option<PoolConnection<Postgres>>,
    buffer_rx: mpsc::UnboundedReceiver<Notification>,
    buffer_tx: Option<mpsc::UnboundedSender<Notification>>,
    channels: Vec<String>,
    ignore_close_event: bool,
    eager_reconnect: bool,
}

/// An asynchronous notification from Postgres.
pub struct PgNotification(Notification);

impl PgListener {
    pub async fn connect(url: &str) -> Result<Self, Error> {
        // Create a pool of 1 without timeouts (as they don't apply here)
        // We only use the pool to handle re-connections
        let pool = PoolOptions::<Postgres>::new()
            .max_connections(1)
            .max_lifetime(None)
            .idle_timeout(None)
            .connect(url)
            .await?;

        let mut this = Self::connect_with(&pool).await?;
        // We don't need to handle close events
        this.ignore_close_event = true;

        Ok(this)
    }

    pub async fn connect_with(pool: &Pool<Postgres>) -> Result<Self, Error> {
        // Pull out an initial connection
        let mut connection = pool.acquire().await?;

        // Setup a notification buffer
        let (sender, receiver) = mpsc::unbounded();
        connection.inner.stream.notifications = Some(sender);

        Ok(Self {
            pool: pool.clone(),
            connection: Some(connection),
            buffer_rx: receiver,
            buffer_tx: None,
            channels: Vec::new(),
            ignore_close_event: false,
            eager_reconnect: true,
        })
    }

    /// Set whether or not to ignore [`Pool::close_event()`]. Defaults to `false`.
    ///
    /// By default, when [`Pool::close()`] is called on the pool this listener is using
    /// while [`Self::recv()`] or [`Self::try_recv()`] are waiting for a message, the wait is
    /// cancelled and `Err(PoolClosed)` is returned.
    ///
    /// This is because `Pool::close()` will wait until _all_ connections are returned and closed,
    /// including the one being used by this listener.
    ///
    /// Otherwise, `pool.close().await` would have to wait until `PgListener` encountered a
    /// need to acquire a new connection (timeout, error, etc.) and dropped the one it was
    /// currently holding, at which point `.recv()` or `.try_recv()` would return `Err(PoolClosed)`
    /// on the attempt to acquire a new connection anyway.
    ///
    /// However, if you want `PgListener` to ignore the close event and continue waiting for a
    /// message as long as it can, set this to `true`.
    ///
    /// Does nothing if this was constructed with [`PgListener::connect()`], as that creates an
    /// internal pool just for the new instance of `PgListener` which cannot be closed manually.
    pub fn ignore_pool_close_event(&mut self, val: bool) {
        self.ignore_close_event = val;
    }

    /// Set whether a lost connection in `try_recv()` should be re-established before it returns
    /// `Ok(None)`, or on the next call to `try_recv()`.
    ///
    /// By default, this is `true` and the connection is re-established before returning `Ok(None)`.
    ///
    /// If this is set to `false` then notifications will continue to be lost until the next call
    /// to `try_recv()`. If your recovery logic uses a different database connection then
    /// notifications that occur after it completes may be lost without any way to tell that they
    /// have been.
    pub fn eager_reconnect(&mut self, val: bool) {
        self.eager_reconnect = val;
    }

    /// Starts listening for notifications on a channel.
    /// The channel name is quoted here to ensure case sensitivity.
    pub async fn listen(&mut self, channel: &str) -> Result<(), Error> {
        self.connection()
            .await?
            .execute(&*format!(r#"LISTEN "{}""#, ident(channel)))
            .await?;

        self.channels.push(channel.to_owned());

        Ok(())
    }

    /// Starts listening for notifications on all channels.
    pub async fn listen_all(
        &mut self,
        channels: impl IntoIterator<Item = &str>,
    ) -> Result<(), Error> {
        let beg = self.channels.len();
        self.channels.extend(channels.into_iter().map(|s| s.into()));

        let query = build_listen_all_query(&self.channels[beg..]);
        self.connection().await?.execute(&*query).await?;

        Ok(())
    }

    /// Stops listening for notifications on a channel.
    /// The channel name is quoted here to ensure case sensitivity.
    pub async fn unlisten(&mut self, channel: &str) -> Result<(), Error> {
        // use RAW connection and do NOT re-connect automatically, since this is not required for
        // UNLISTEN (we've disconnected anyways)
        if let Some(connection) = self.connection.as_mut() {
            connection
                .execute(&*format!(r#"UNLISTEN "{}""#, ident(channel)))
                .await?;
        }

        if let Some(pos) = self.channels.iter().position(|s| s == channel) {
            self.channels.remove(pos);
        }

        Ok(())
    }

    /// Stops listening for notifications on all channels.
    pub async fn unlisten_all(&mut self) -> Result<(), Error> {
        // use RAW connection and do NOT re-connect automatically, since this is not required for
        // UNLISTEN (we've disconnected anyways)
        if let Some(connection) = self.connection.as_mut() {
            connection.execute("UNLISTEN *").await?;
        }

        self.channels.clear();

        Ok(())
    }

    #[inline]
    async fn connect_if_needed(&mut self) -> Result<(), Error> {
        if self.connection.is_none() {
            let mut connection = self.pool.acquire().await?;
            connection.inner.stream.notifications = self.buffer_tx.take();

            connection
                .execute(&*build_listen_all_query(&self.channels))
                .await?;

            self.connection = Some(connection);
        }

        Ok(())
    }

    #[inline]
    async fn connection(&mut self) -> Result<&mut PgConnection, Error> {
        // Ensure we have an active connection to work with.
        self.connect_if_needed().await?;

        Ok(self.connection.as_mut().unwrap())
    }

    /// Receives the next notification available from any of the subscribed channels.
    ///
    /// If the connection to PostgreSQL is lost, it is automatically reconnected on the next
    /// call to `recv()`, and should be entirely transparent (as long as it was just an
    /// intermittent network failure or long-lived connection reaper).
    ///
    /// As notifications are transient, any received while the connection was lost, will not
    /// be returned. If you'd prefer the reconnection to be explicit and have a chance to
    /// do something before, please see [`try_recv`](Self::try_recv).
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use sqlx::postgres::PgListener;
    /// #
    /// # sqlx::__rt::test_block_on(async move {
    /// let mut listener = PgListener::connect("postgres:// ...").await?;
    /// loop {
    ///     // ask for next notification, re-connecting (transparently) if needed
    ///     let notification = listener.recv().await?;
    ///
    ///     // handle notification, do something interesting
    /// }
    /// # Result::<(), sqlx::Error>::Ok(())
    /// # }).unwrap();
    /// ```
    pub async fn recv(&mut self) -> Result<PgNotification, Error> {
        loop {
            if let Some(notification) = self.try_recv().await? {
                return Ok(notification);
            }
        }
    }

    /// Receives the next notification available from any of the subscribed channels.
    ///
    /// If the connection to PostgreSQL is lost, `None` is returned, and the connection is
    /// reconnected either immediately, or on the next call to `try_recv()`, depending on
    /// the value of [`eager_reconnect`].
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use sqlx::postgres::PgListener;
    /// #
    /// # sqlx::__rt::test_block_on(async move {
    /// # let mut listener = PgListener::connect("postgres:// ...").await?;
    /// loop {
    ///     // start handling notifications, connecting if needed
    ///     while let Some(notification) = listener.try_recv().await? {
    ///         // handle notification
    ///     }
    ///
    ///     // connection lost, do something interesting
    /// }
    /// # Result::<(), sqlx::Error>::Ok(())
    /// # }).unwrap();
    /// ```
    ///
    /// [`eager_reconnect`]: PgListener::eager_reconnect
    pub async fn try_recv(&mut self) -> Result<Option<PgNotification>, Error> {
        // Flush the buffer first, if anything
        // This would only fill up if this listener is used as a connection
        if let Some(notification) = self.next_buffered() {
            return Ok(Some(notification));
        }

        // Fetch our `CloseEvent` listener, if applicable.
        let mut close_event = (!self.ignore_close_event).then(|| self.pool.close_event());

        loop {
            let next_message = self.connection().await?.inner.stream.recv_unchecked();

            let res = if let Some(ref mut close_event) = close_event {
                // cancels the wait and returns `Err(PoolClosed)` if the pool is closed
                // before `next_message` returns, or if the pool was already closed
                close_event.do_until(next_message).await?
            } else {
                next_message.await
            };

            let message = match res {
                Ok(message) => message,

                // The connection is dead, ensure that it is dropped,
                // update self state, and loop to try again.
                Err(Error::Io(err))
                    if matches!(
                        err.kind(),
                        io::ErrorKind::ConnectionAborted |
                        io::ErrorKind::UnexpectedEof |
                        // see ERRORS section in tcp(7) man page (https://man7.org/linux/man-pages/man7/tcp.7.html)
                        io::ErrorKind::TimedOut |
                        io::ErrorKind::BrokenPipe
                    ) =>
                {
                    if let Some(mut conn) = self.connection.take() {
                        self.buffer_tx = conn.inner.stream.notifications.take();
                        // Close the connection in a background task, so we can continue.
                        conn.close_on_drop();
                    }

                    if self.eager_reconnect {
                        self.connect_if_needed().await?;
                    }

                    // lost connection
                    return Ok(None);
                }

                // Forward other errors
                Err(error) => {
                    return Err(error);
                }
            };

            match message.format {
                // We've received an async notification, return it.
                BackendMessageFormat::NotificationResponse => {
                    return Ok(Some(PgNotification(message.decode()?)));
                }

                // Mark the connection as ready for another query
                BackendMessageFormat::ReadyForQuery => {
                    self.connection().await?.inner.pending_ready_for_query_count -= 1;
                }

                // Ignore unexpected messages
                _ => {}
            }
        }
    }

    /// Receives the next notification that already exists in the connection buffer, if any.
    ///
    /// This is similar to `try_recv`, except it will not wait if the connection has not yet received a notification.
    ///
    /// This is helpful if you want to retrieve all buffered notifications and process them in batches.
    pub fn next_buffered(&mut self) -> Option<PgNotification> {
        if let Ok(Some(notification)) = self.buffer_rx.try_next() {
            Some(PgNotification(notification))
        } else {
            None
        }
    }

    /// Consume this listener, returning a `Stream` of notifications.
    ///
    /// The backing connection will be automatically reconnected should it be lost.
    ///
    /// This has the same potential drawbacks as [`recv`](PgListener::recv).
    ///
    pub fn into_stream(mut self) -> impl Stream<Item = Result<PgNotification, Error>> + Unpin {
        Box::pin(try_stream! {
            loop {
                r#yield!(self.recv().await?);
            }
        })
    }
}

impl Drop for PgListener {
    fn drop(&mut self) {
        if let Some(mut conn) = self.connection.take() {
            let fut = async move {
                let _ = conn.execute("UNLISTEN *").await;

                // inline the drop handler from `PoolConnection` so it doesn't try to spawn another task
                // otherwise, it may trigger a panic if this task is dropped because the runtime is going away:
                // https://github.com/launchbadge/sqlx/issues/1389
                conn.return_to_pool().await;
            };

            // Unregister any listeners before returning the connection to the pool.
            crate::rt::spawn(fut.in_current_span());
        }
    }
}

impl<'c> Acquire<'c> for &'c mut PgListener {
    type Database = Postgres;
    type Connection = &'c mut PgConnection;

    fn acquire(self) -> BoxFuture<'c, Result<Self::Connection, Error>> {
        self.connection().boxed()
    }

    fn begin(self) -> BoxFuture<'c, Result<Transaction<'c, Self::Database>, Error>> {
        self.connection().and_then(|c| c.begin()).boxed()
    }
}

impl<'c> Executor<'c> for &'c mut PgListener {
    type Database = Postgres;

    fn fetch_many<'e, 'q, E>(
        self,
        query: E,
    ) -> BoxStream<'e, Result<Either<PgQueryResult, PgRow>, Error>>
    where
        'c: 'e,
        E: Execute<'q, Self::Database>,
        'q: 'e,
        E: 'q,
    {
        futures_util::stream::once(async move {
            // need some basic type annotation to help the compiler a bit
            let res: Result<_, Error> = Ok(self.connection().await?.fetch_many(query));
            res
        })
        .try_flatten()
        .boxed()
    }

    fn fetch_optional<'e, 'q, E>(self, query: E) -> BoxFuture<'e, Result<Option<PgRow>, Error>>
    where
        'c: 'e,
        E: Execute<'q, Self::Database>,
        'q: 'e,
        E: 'q,
    {
        async move { self.connection().await?.fetch_optional(query).await }.boxed()
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        query: &'q str,
        parameters: &'e [PgTypeInfo],
    ) -> BoxFuture<'e, Result<PgStatement<'q>, Error>>
    where
        'c: 'e,
    {
        async move {
            self.connection()
                .await?
                .prepare_with(query, parameters)
                .await
        }
        .boxed()
    }

    #[doc(hidden)]
    fn describe<'e, 'q: 'e>(
        self,
        query: &'q str,
    ) -> BoxFuture<'e, Result<Describe<Self::Database>, Error>>
    where
        'c: 'e,
    {
        async move { self.connection().await?.describe(query).await }.boxed()
    }
}

impl PgNotification {
    /// The process ID of the notifying backend process.
    #[inline]
    pub fn process_id(&self) -> u32 {
        self.0.process_id
    }

    /// The channel that the notify has been raised on. This can be thought
    /// of as the message topic.
    #[inline]
    pub fn channel(&self) -> &str {
        from_utf8(&self.0.channel).unwrap()
    }

    /// The payload of the notification. An empty payload is received as an
    /// empty string.
    #[inline]
    pub fn payload(&self) -> &str {
        from_utf8(&self.0.payload).unwrap()
    }
}

impl Debug for PgListener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PgListener").finish()
    }
}

impl Debug for PgNotification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PgNotification")
            .field("process_id", &self.process_id())
            .field("channel", &self.channel())
            .field("payload", &self.payload())
            .finish()
    }
}

fn ident(mut name: &str) -> String {
    // If the input string contains a NUL byte, we should truncate the
    // identifier.
    if let Some(index) = name.find('\0') {
        name = &name[..index];
    }

    // Any double quotes must be escaped
    name.replace('"', "\"\"")
}

fn build_listen_all_query(channels: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    channels.into_iter().fold(String::new(), |mut acc, chan| {
        acc.push_str(r#"LISTEN ""#);
        acc.push_str(&ident(chan.as_ref()));
        acc.push_str(r#"";"#);
        acc
    })
}

#[test]
fn test_build_listen_all_query_with_single_channel() {
    let output = build_listen_all_query(&["test"]);
    assert_eq!(output.as_str(), r#"LISTEN "test";"#);
}

#[test]
fn test_build_listen_all_query_with_multiple_channels() {
    let output = build_listen_all_query(&["channel.0", "channel.1"]);
    assert_eq!(output.as_str(), r#"LISTEN "channel.0";LISTEN "channel.1";"#);
}
