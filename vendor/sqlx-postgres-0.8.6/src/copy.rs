use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;

use sqlx_core::bytes::{BufMut, Bytes};

use crate::connection::PgConnection;
use crate::error::{Error, Result};
use crate::ext::async_stream::TryAsyncStream;
use crate::io::AsyncRead;
use crate::message::{
    BackendMessageFormat, CommandComplete, CopyData, CopyDone, CopyFail, CopyInResponse,
    CopyOutResponse, CopyResponseData, Query, ReadyForQuery,
};
use crate::pool::{Pool, PoolConnection};
use crate::Postgres;

impl PgConnection {
    /// Issue a `COPY FROM STDIN` statement and transition the connection to streaming data
    /// to Postgres. This is a more efficient way to import data into Postgres as compared to
    /// `INSERT` but requires one of a few specific data formats (text/CSV/binary).
    ///
    /// If `statement` is anything other than a `COPY ... FROM STDIN ...` command, an error is
    /// returned.
    ///
    /// Command examples and accepted formats for `COPY` data are shown here:
    /// <https://www.postgresql.org/docs/current/sql-copy.html>
    ///
    /// ### Note
    /// [PgCopyIn::finish] or [PgCopyIn::abort] *must* be called when finished or the connection
    /// will return an error the next time it is used.
    pub async fn copy_in_raw(&mut self, statement: &str) -> Result<PgCopyIn<&mut Self>> {
        PgCopyIn::begin(self, statement).await
    }

    /// Issue a `COPY TO STDOUT` statement and transition the connection to streaming data
    /// from Postgres. This is a more efficient way to export data from Postgres but
    /// arrives in chunks of one of a few data formats (text/CSV/binary).
    ///
    /// If `statement` is anything other than a `COPY ... TO STDOUT ...` command,
    /// an error is returned.
    ///
    /// Note that once this process has begun, unless you read the stream to completion,
    /// it can only be canceled in two ways:
    ///
    /// 1. by closing the connection, or:
    /// 2. by using another connection to kill the server process that is sending the data as shown
    ///    [in this StackOverflow answer](https://stackoverflow.com/a/35319598).
    ///
    /// If you don't read the stream to completion, the next time the connection is used it will
    /// need to read and discard all the remaining queued data, which could take some time.
    ///
    /// Command examples and accepted formats for `COPY` data are shown here:
    /// <https://www.postgresql.org/docs/current/sql-copy.html>
    #[allow(clippy::needless_lifetimes)]
    pub async fn copy_out_raw<'c>(
        &'c mut self,
        statement: &str,
    ) -> Result<BoxStream<'c, Result<Bytes>>> {
        pg_begin_copy_out(self, statement).await
    }
}

/// Implements methods for directly executing `COPY FROM/TO STDOUT` on a [`PgPool`][crate::PgPool].
///
/// This is a replacement for the inherent methods on `PgPool` which could not exist
/// once the Postgres driver was moved out into its own crate.
pub trait PgPoolCopyExt {
    /// Issue a `COPY FROM STDIN` statement and begin streaming data to Postgres.
    /// This is a more efficient way to import data into Postgres as compared to
    /// `INSERT` but requires one of a few specific data formats (text/CSV/binary).
    ///
    /// A single connection will be checked out for the duration.
    ///
    /// If `statement` is anything other than a `COPY ... FROM STDIN ...` command, an error is
    /// returned.
    ///
    /// Command examples and accepted formats for `COPY` data are shown here:
    /// <https://www.postgresql.org/docs/current/sql-copy.html>
    ///
    /// ### Note
    /// [PgCopyIn::finish] or [PgCopyIn::abort] *must* be called when finished or the connection
    /// will return an error the next time it is used.
    fn copy_in_raw<'a>(
        &'a self,
        statement: &'a str,
    ) -> BoxFuture<'a, Result<PgCopyIn<PoolConnection<Postgres>>>>;

    /// Issue a `COPY TO STDOUT` statement and begin streaming data
    /// from Postgres. This is a more efficient way to export data from Postgres but
    /// arrives in chunks of one of a few data formats (text/CSV/binary).
    ///
    /// If `statement` is anything other than a `COPY ... TO STDOUT ...` command,
    /// an error is returned.
    ///
    /// Note that once this process has begun, unless you read the stream to completion,
    /// it can only be canceled in two ways:
    ///
    /// 1. by closing the connection, or:
    /// 2. by using another connection to kill the server process that is sending the data as shown
    ///    [in this StackOverflow answer](https://stackoverflow.com/a/35319598).
    ///
    /// If you don't read the stream to completion, the next time the connection is used it will
    /// need to read and discard all the remaining queued data, which could take some time.
    ///
    /// Command examples and accepted formats for `COPY` data are shown here:
    /// <https://www.postgresql.org/docs/current/sql-copy.html>
    fn copy_out_raw<'a>(
        &'a self,
        statement: &'a str,
    ) -> BoxFuture<'a, Result<BoxStream<'static, Result<Bytes>>>>;
}

impl PgPoolCopyExt for Pool<Postgres> {
    fn copy_in_raw<'a>(
        &'a self,
        statement: &'a str,
    ) -> BoxFuture<'a, Result<PgCopyIn<PoolConnection<Postgres>>>> {
        Box::pin(async { PgCopyIn::begin(self.acquire().await?, statement).await })
    }

    fn copy_out_raw<'a>(
        &'a self,
        statement: &'a str,
    ) -> BoxFuture<'a, Result<BoxStream<'static, Result<Bytes>>>> {
        Box::pin(async { pg_begin_copy_out(self.acquire().await?, statement).await })
    }
}

// (1 GiB - 1) - 1 - length prefix (4 bytes)
pub const PG_COPY_MAX_DATA_LEN: usize = 0x3fffffff - 1 - 4;

/// A connection in streaming `COPY FROM STDIN` mode.
///
/// Created by [PgConnection::copy_in_raw] or [Pool::copy_out_raw].
///
/// ### Note
/// [PgCopyIn::finish] or [PgCopyIn::abort] *must* be called when finished or the connection
/// will return an error the next time it is used.
#[must_use = "connection will error on next use if `.finish()` or `.abort()` is not called"]
pub struct PgCopyIn<C: DerefMut<Target = PgConnection>> {
    conn: Option<C>,
    response: CopyResponseData,
}

impl<C: DerefMut<Target = PgConnection>> PgCopyIn<C> {
    async fn begin(mut conn: C, statement: &str) -> Result<Self> {
        conn.wait_until_ready().await?;
        conn.inner.stream.send(Query(statement)).await?;

        let response = match conn.inner.stream.recv_expect::<CopyInResponse>().await {
            Ok(res) => res.0,
            Err(e) => {
                conn.inner.stream.recv().await?;
                return Err(e);
            }
        };

        Ok(PgCopyIn {
            conn: Some(conn),
            response,
        })
    }

    /// Returns `true` if Postgres is expecting data in text or CSV format.
    pub fn is_textual(&self) -> bool {
        self.response.format == 0
    }

    /// Returns the number of columns expected in the input.
    pub fn num_columns(&self) -> usize {
        assert_eq!(
            self.response.num_columns.unsigned_abs() as usize,
            self.response.format_codes.len(),
            "num_columns does not match format_codes.len()"
        );
        self.response.format_codes.len()
    }

    /// Check if a column is expecting data in text format (`true`) or binary format (`false`).
    ///
    /// ### Panics
    /// If `column` is out of range according to [`.num_columns()`][Self::num_columns].
    pub fn column_is_textual(&self, column: usize) -> bool {
        self.response.format_codes[column] == 0
    }

    /// Send a chunk of `COPY` data.
    ///
    /// The data is sent in chunks if it exceeds the maximum length of a `CopyData` message (1 GiB - 6
    /// bytes) and may be partially sent if this call is cancelled.
    ///
    /// If you're copying data from an `AsyncRead`, maybe consider [Self::read_from] instead.
    pub async fn send(&mut self, data: impl Deref<Target = [u8]>) -> Result<&mut Self> {
        for chunk in data.deref().chunks(PG_COPY_MAX_DATA_LEN) {
            self.conn
                .as_deref_mut()
                .expect("send_data: conn taken")
                .inner
                .stream
                .send(CopyData(chunk))
                .await?;
        }

        Ok(self)
    }

    /// Copy data directly from `source` to the database without requiring an intermediate buffer.
    ///
    /// `source` will be read to the end.
    ///
    /// ### Note: Completion Step Required
    /// You must still call either [Self::finish] or [Self::abort] to complete the process.
    ///
    /// ### Note: Runtime Features
    /// This method uses the `AsyncRead` trait which is re-exported from either Tokio or `async-std`
    /// depending on which runtime feature is used.
    ///
    /// The runtime features _used_ to be mutually exclusive, but are no longer.
    /// If both `runtime-async-std` and `runtime-tokio` features are enabled, the Tokio version
    /// takes precedent.
    pub async fn read_from(&mut self, mut source: impl AsyncRead + Unpin) -> Result<&mut Self> {
        let conn: &mut PgConnection = self.conn.as_deref_mut().expect("copy_from: conn taken");
        loop {
            let buf = conn.inner.stream.write_buffer_mut();

            // Write the CopyData format code and reserve space for the length.
            // This may end up sending an empty `CopyData` packet if, after this point,
            // we get canceled or read 0 bytes, but that should be fine.
            buf.put_slice(b"d\0\0\0\x04");

            let read = buf.read_from(&mut source).await?;

            if read == 0 {
                break;
            }

            // Write the length
            let read32 = i32::try_from(read)
                .map_err(|_| err_protocol!("number of bytes read exceeds 2^31 - 1: {}", read))?;

            (&mut buf.get_mut()[1..]).put_i32(read32 + 4);

            conn.inner.stream.flush().await?;
        }

        Ok(self)
    }

    /// Signal that the `COPY` process should be aborted and any data received should be discarded.
    ///
    /// The given message can be used for indicating the reason for the abort in the database logs.
    ///
    /// The server is expected to respond with an error, so only _unexpected_ errors are returned.
    pub async fn abort(mut self, msg: impl Into<String>) -> Result<()> {
        let mut conn = self
            .conn
            .take()
            .expect("PgCopyIn::fail_with: conn taken illegally");

        conn.inner.stream.send(CopyFail::new(msg)).await?;

        match conn.inner.stream.recv().await {
            Ok(msg) => Err(err_protocol!(
                "fail_with: expected ErrorResponse, got: {:?}",
                msg.format
            )),
            Err(Error::Database(e)) => {
                match e.code() {
                    Some(Cow::Borrowed("57014")) => {
                        // postgres abort received error code
                        conn.inner.stream.recv_expect::<ReadyForQuery>().await?;
                        Ok(())
                    }
                    _ => Err(Error::Database(e)),
                }
            }
            Err(e) => Err(e),
        }
    }

    /// Signal that the `COPY` process is complete.
    ///
    /// The number of rows affected is returned.
    pub async fn finish(mut self) -> Result<u64> {
        let mut conn = self
            .conn
            .take()
            .expect("CopyWriter::finish: conn taken illegally");

        conn.inner.stream.send(CopyDone).await?;
        let cc: CommandComplete = match conn.inner.stream.recv_expect().await {
            Ok(cc) => cc,
            Err(e) => {
                conn.inner.stream.recv().await?;
                return Err(e);
            }
        };

        conn.inner.stream.recv_expect::<ReadyForQuery>().await?;

        Ok(cc.rows_affected())
    }
}

impl<C: DerefMut<Target = PgConnection>> Drop for PgCopyIn<C> {
    fn drop(&mut self) {
        if let Some(mut conn) = self.conn.take() {
            conn.inner
                .stream
                .write_msg(CopyFail::new(
                    "PgCopyIn dropped without calling finish() or fail()",
                ))
                .expect("BUG: PgCopyIn abort message should not be too large");
        }
    }
}

async fn pg_begin_copy_out<'c, C: DerefMut<Target = PgConnection> + Send + 'c>(
    mut conn: C,
    statement: &str,
) -> Result<BoxStream<'c, Result<Bytes>>> {
    conn.wait_until_ready().await?;
    conn.inner.stream.send(Query(statement)).await?;

    let _: CopyOutResponse = conn.inner.stream.recv_expect().await?;

    let stream: TryAsyncStream<'c, Bytes> = try_stream! {
        loop {
            match conn.inner.stream.recv().await {
                Err(e) => {
                    conn.inner.stream.recv_expect::<ReadyForQuery>().await?;
                    return Err(e);
                },
                Ok(msg) => match msg.format {
                    BackendMessageFormat::CopyData => r#yield!(msg.decode::<CopyData<Bytes>>()?.0),
                    BackendMessageFormat::CopyDone => {
                        let _ = msg.decode::<CopyDone>()?;
                        conn.inner.stream.recv_expect::<CommandComplete>().await?;
                        conn.inner.stream.recv_expect::<ReadyForQuery>().await?;
                        return Ok(())
                    },
                    _ => return Err(err_protocol!("unexpected message format during copy out: {:?}", msg.format))
                }
            }
        }
    };

    Ok(Box::pin(stream))
}
