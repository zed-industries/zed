//! **MySQL** database driver.
#![deny(clippy::cast_possible_truncation)]
#![deny(clippy::cast_possible_wrap)]
#![deny(clippy::cast_sign_loss)]

#[macro_use]
extern crate sqlx_core;

use crate::executor::Executor;

pub(crate) use sqlx_core::driver_prelude::*;

#[cfg(feature = "any")]
pub mod any;

mod arguments;
mod collation;
mod column;
mod connection;
mod database;
mod error;
mod io;
mod options;
mod protocol;
mod query_result;
mod row;
mod statement;
mod transaction;
mod type_checking;
mod type_info;
pub mod types;
mod value;

#[cfg(feature = "migrate")]
mod migrate;

#[cfg(feature = "migrate")]
mod testing;

pub use arguments::MySqlArguments;
pub use column::MySqlColumn;
pub use connection::MySqlConnection;
pub use database::MySql;
pub use error::MySqlDatabaseError;
pub use options::{MySqlConnectOptions, MySqlSslMode};
pub use query_result::MySqlQueryResult;
pub use row::MySqlRow;
pub use statement::MySqlStatement;
pub use transaction::MySqlTransactionManager;
pub use type_info::MySqlTypeInfo;
pub use value::{MySqlValue, MySqlValueFormat, MySqlValueRef};

/// An alias for [`Pool`][crate::pool::Pool], specialized for MySQL.
pub type MySqlPool = crate::pool::Pool<MySql>;

/// An alias for [`PoolOptions`][crate::pool::PoolOptions], specialized for MySQL.
pub type MySqlPoolOptions = crate::pool::PoolOptions<MySql>;

/// An alias for [`Executor<'_, Database = MySql>`][Executor].
pub trait MySqlExecutor<'c>: Executor<'c, Database = MySql> {}
impl<'c, T: Executor<'c, Database = MySql>> MySqlExecutor<'c> for T {}

/// An alias for [`Transaction`][crate::transaction::Transaction], specialized for MySQL.
pub type MySqlTransaction<'c> = crate::transaction::Transaction<'c, MySql>;

// NOTE: required due to the lack of lazy normalization
impl_into_arguments_for_arguments!(MySqlArguments);
impl_acquire!(MySql, MySqlConnection);
impl_column_index_for_row!(MySqlRow);
impl_column_index_for_statement!(MySqlStatement);

// required because some databases have a different handling of NULL
impl_encode_for_option!(MySql);
