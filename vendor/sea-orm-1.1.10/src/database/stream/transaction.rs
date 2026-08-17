#![allow(missing_docs)]

use std::{ops::DerefMut, pin::Pin, task::Poll};
use tracing::instrument;

#[cfg(feature = "sqlx-dep")]
use futures_util::TryStreamExt;
use futures_util::{lock::MutexGuard, Stream};

#[cfg(feature = "sqlx-dep")]
use sqlx::Executor;

use super::metric::MetricStream;
#[cfg(feature = "sqlx-dep")]
use crate::driver::*;
use crate::{DbErr, InnerConnection, QueryResult, Statement};

/// `TransactionStream` cannot be used in a `transaction` closure as it does not impl `Send`.
/// It seems to be a Rust limitation right now, and solution to work around this deemed to be extremely hard.
#[ouroboros::self_referencing]
pub struct TransactionStream<'a> {
    stmt: Statement,
    conn: MutexGuard<'a, InnerConnection>,
    metric_callback: Option<crate::metric::Callback>,
    #[borrows(mut conn, stmt, metric_callback)]
    #[not_covariant]
    stream: MetricStream<'this>,
}

impl std::fmt::Debug for TransactionStream<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TransactionStream")
    }
}

impl TransactionStream<'_> {
    #[instrument(level = "trace", skip(metric_callback))]
    #[allow(unused_variables)]
    pub(crate) fn build(
        conn: MutexGuard<'_, InnerConnection>,
        stmt: Statement,
        metric_callback: Option<crate::metric::Callback>,
    ) -> TransactionStream<'_> {
        TransactionStreamBuilder {
            stmt,
            conn,
            metric_callback,
            stream_builder: |conn, stmt, _metric_callback| match conn.deref_mut() {
                #[cfg(feature = "sqlx-mysql")]
                InnerConnection::MySql(c) => {
                    let query = crate::driver::sqlx_mysql::sqlx_query(stmt);
                    let _start = _metric_callback.is_some().then(std::time::SystemTime::now);
                    let stream = c
                        .fetch(query)
                        .map_ok(Into::into)
                        .map_err(sqlx_error_to_query_err);
                    let elapsed = _start.map(|s| s.elapsed().unwrap_or_default());
                    MetricStream::new(_metric_callback, stmt, elapsed, stream)
                }
                #[cfg(feature = "sqlx-postgres")]
                InnerConnection::Postgres(c) => {
                    let query = crate::driver::sqlx_postgres::sqlx_query(stmt);
                    let _start = _metric_callback.is_some().then(std::time::SystemTime::now);
                    let stream = c
                        .fetch(query)
                        .map_ok(Into::into)
                        .map_err(sqlx_error_to_query_err);
                    let elapsed = _start.map(|s| s.elapsed().unwrap_or_default());
                    MetricStream::new(_metric_callback, stmt, elapsed, stream)
                }
                #[cfg(feature = "sqlx-sqlite")]
                InnerConnection::Sqlite(c) => {
                    let query = crate::driver::sqlx_sqlite::sqlx_query(stmt);
                    let _start = _metric_callback.is_some().then(std::time::SystemTime::now);
                    let stream = c
                        .fetch(query)
                        .map_ok(Into::into)
                        .map_err(sqlx_error_to_query_err);
                    let elapsed = _start.map(|s| s.elapsed().unwrap_or_default());
                    MetricStream::new(_metric_callback, stmt, elapsed, stream)
                }
                #[cfg(feature = "mock")]
                InnerConnection::Mock(c) => {
                    let _start = _metric_callback.is_some().then(std::time::SystemTime::now);
                    let stream = c.fetch(stmt);
                    let elapsed = _start.map(|s| s.elapsed().unwrap_or_default());
                    MetricStream::new(_metric_callback, stmt, elapsed, stream)
                }
                #[cfg(feature = "proxy")]
                InnerConnection::Proxy(c) => {
                    todo!("Proxy connection is not supported")
                }
                #[allow(unreachable_patterns)]
                _ => unreachable!(),
            },
        }
        .build()
    }
}

impl Stream for TransactionStream<'_> {
    type Item = Result<QueryResult, DbErr>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        this.with_stream_mut(|stream| Pin::new(stream).poll_next(cx))
    }
}
