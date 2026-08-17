use futures_core::future::BoxFuture;
use sqlx_core::database::Database;
use std::borrow::Cow;

use crate::error::Error;
use crate::executor::Executor;

use crate::{PgConnection, Postgres};

pub(crate) use sqlx_core::transaction::*;

/// Implementation of [`TransactionManager`] for PostgreSQL.
pub struct PgTransactionManager;

impl TransactionManager for PgTransactionManager {
    type Database = Postgres;

    fn begin<'conn>(
        conn: &'conn mut PgConnection,
        statement: Option<Cow<'static, str>>,
    ) -> BoxFuture<'conn, Result<(), Error>> {
        Box::pin(async move {
            let depth = conn.inner.transaction_depth;
            let statement = match statement {
                // custom `BEGIN` statements are not allowed if we're already in
                // a transaction (we need to issue a `SAVEPOINT` instead)
                Some(_) if depth > 0 => return Err(Error::InvalidSavePointStatement),
                Some(statement) => statement,
                None => begin_ansi_transaction_sql(depth),
            };

            let rollback = Rollback::new(conn);
            rollback.conn.queue_simple_query(&statement)?;
            rollback.conn.wait_until_ready().await?;
            if !rollback.conn.in_transaction() {
                return Err(Error::BeginFailed);
            }
            rollback.conn.inner.transaction_depth += 1;
            rollback.defuse();

            Ok(())
        })
    }

    fn commit(conn: &mut PgConnection) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async move {
            if conn.inner.transaction_depth > 0 {
                conn.execute(&*commit_ansi_transaction_sql(conn.inner.transaction_depth))
                    .await?;

                conn.inner.transaction_depth -= 1;
            }

            Ok(())
        })
    }

    fn rollback(conn: &mut PgConnection) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async move {
            if conn.inner.transaction_depth > 0 {
                conn.execute(&*rollback_ansi_transaction_sql(
                    conn.inner.transaction_depth,
                ))
                .await?;

                conn.inner.transaction_depth -= 1;
            }

            Ok(())
        })
    }

    fn start_rollback(conn: &mut PgConnection) {
        if conn.inner.transaction_depth > 0 {
            conn.queue_simple_query(&rollback_ansi_transaction_sql(conn.inner.transaction_depth))
                .expect("BUG: Rollback query somehow too large for protocol");

            conn.inner.transaction_depth -= 1;
        }
    }

    fn get_transaction_depth(conn: &<Self::Database as Database>::Connection) -> usize {
        conn.inner.transaction_depth
    }
}

struct Rollback<'c> {
    conn: &'c mut PgConnection,
    defuse: bool,
}

impl Drop for Rollback<'_> {
    fn drop(&mut self) {
        if !self.defuse {
            PgTransactionManager::start_rollback(self.conn)
        }
    }
}

impl<'c> Rollback<'c> {
    fn new(conn: &'c mut PgConnection) -> Self {
        Self {
            conn,
            defuse: false,
        }
    }
    fn defuse(mut self) {
        self.defuse = true;
    }
}
