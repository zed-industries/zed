//! **PostgreSQL** database driver.

#[macro_use]
extern crate sqlx_core;

use crate::executor::Executor;

mod advisory_lock;
mod arguments;
mod column;
mod connection;
mod copy;
mod database;
mod error;
mod io;
mod listener;
mod message;
mod options;
mod query_result;
mod row;
mod statement;
mod transaction;
mod type_checking;
mod type_info;
pub mod types;
mod value;

#[cfg(feature = "any")]
// We are hiding the any module with its AnyConnectionBackend trait
// so that IDEs don't show it in the autocompletion list
// and end users don't accidentally use it. This can result in
// nested transactions not behaving as expected.
// For more information, see https://github.com/launchbadge/sqlx/pull/3254#issuecomment-2144043823
#[doc(hidden)]
pub mod any;

#[doc(hidden)]
pub use copy::PG_COPY_MAX_DATA_LEN;

#[cfg(feature = "migrate")]
mod migrate;

#[cfg(feature = "migrate")]
mod testing;

pub(crate) use sqlx_core::driver_prelude::*;

pub use advisory_lock::{PgAdvisoryLock, PgAdvisoryLockGuard, PgAdvisoryLockKey};
pub use arguments::{PgArgumentBuffer, PgArguments};
pub use column::PgColumn;
pub use connection::PgConnection;
pub use copy::{PgCopyIn, PgPoolCopyExt};
pub use database::Postgres;
pub use error::{PgDatabaseError, PgErrorPosition};
pub use listener::{PgListener, PgNotification};
pub use message::PgSeverity;
pub use options::{PgConnectOptions, PgSslMode};
pub use query_result::PgQueryResult;
pub use row::PgRow;
pub use statement::PgStatement;
pub use transaction::PgTransactionManager;
pub use type_info::{PgTypeInfo, PgTypeKind};
pub use types::PgHasArrayType;
pub use value::{PgValue, PgValueFormat, PgValueRef};

/// An alias for [`Pool`][crate::pool::Pool], specialized for Postgres.
pub type PgPool = crate::pool::Pool<Postgres>;

/// An alias for [`PoolOptions`][crate::pool::PoolOptions], specialized for Postgres.
pub type PgPoolOptions = crate::pool::PoolOptions<Postgres>;

/// An alias for [`Executor<'_, Database = Postgres>`][Executor].
pub trait PgExecutor<'c>: Executor<'c, Database = Postgres> {}
impl<'c, T: Executor<'c, Database = Postgres>> PgExecutor<'c> for T {}

/// An alias for [`Transaction`][crate::transaction::Transaction], specialized for Postgres.
pub type PgTransaction<'c> = crate::transaction::Transaction<'c, Postgres>;

impl_into_arguments_for_arguments!(PgArguments);
impl_acquire!(Postgres, PgConnection);
impl_column_index_for_row!(PgRow);
impl_column_index_for_statement!(PgStatement);
impl_encode_for_option!(Postgres);
