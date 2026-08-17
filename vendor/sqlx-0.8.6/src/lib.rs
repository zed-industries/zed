#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("lib.md")]

#[cfg(all(
    feature = "sqlite-preupdate-hook",
    not(any(feature = "sqlite", feature = "sqlite-unbundled"))
))]
compile_error!(
    "sqlite-preupdate-hook requires either 'sqlite' or 'sqlite-unbundled' to be enabled"
);

pub use sqlx_core::acquire::Acquire;
pub use sqlx_core::arguments::{Arguments, IntoArguments};
pub use sqlx_core::column::Column;
pub use sqlx_core::column::ColumnIndex;
pub use sqlx_core::connection::{ConnectOptions, Connection};
pub use sqlx_core::database::{self, Database};
pub use sqlx_core::describe::Describe;
pub use sqlx_core::executor::{Execute, Executor};
pub use sqlx_core::from_row::FromRow;
pub use sqlx_core::pool::{self, Pool};
#[doc(hidden)]
pub use sqlx_core::query::query_with_result as __query_with_result;
pub use sqlx_core::query::{query, query_with};
pub use sqlx_core::query_as::{query_as, query_as_with};
pub use sqlx_core::query_builder::{self, QueryBuilder};
#[doc(hidden)]
pub use sqlx_core::query_scalar::query_scalar_with_result as __query_scalar_with_result;
pub use sqlx_core::query_scalar::{query_scalar, query_scalar_with};
pub use sqlx_core::raw_sql::{raw_sql, RawSql};
pub use sqlx_core::row::Row;
pub use sqlx_core::statement::Statement;
pub use sqlx_core::transaction::{Transaction, TransactionManager};
pub use sqlx_core::type_info::TypeInfo;
pub use sqlx_core::types::Type;
pub use sqlx_core::value::{Value, ValueRef};
pub use sqlx_core::Either;

#[doc(inline)]
pub use sqlx_core::error::{self, Error, Result};

#[cfg(feature = "migrate")]
pub use sqlx_core::migrate;

#[cfg(feature = "mysql")]
#[cfg_attr(docsrs, doc(cfg(feature = "mysql")))]
#[doc(inline)]
pub use sqlx_mysql::{
    self as mysql, MySql, MySqlConnection, MySqlExecutor, MySqlPool, MySqlTransaction,
};

#[cfg(feature = "postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres")))]
#[doc(inline)]
pub use sqlx_postgres::{
    self as postgres, PgConnection, PgExecutor, PgPool, PgTransaction, Postgres,
};

#[cfg(feature = "_sqlite")]
#[cfg_attr(docsrs, doc(cfg(feature = "_sqlite")))]
#[doc(inline)]
pub use sqlx_sqlite::{
    self as sqlite, Sqlite, SqliteConnection, SqliteExecutor, SqlitePool, SqliteTransaction,
};

#[cfg(feature = "any")]
#[cfg_attr(docsrs, doc(cfg(feature = "any")))]
pub use crate::any::{reexports::*, Any, AnyExecutor};

#[cfg(any(feature = "derive", feature = "macros"))]
#[doc(hidden)]
pub extern crate sqlx_macros;

// derives
#[cfg(feature = "derive")]
#[doc(hidden)]
pub use sqlx_macros::{FromRow, Type};

// We can't do our normal facade approach with an attribute, but thankfully we can now
// have docs out-of-line quite easily.
#[doc = include_str!("macros/test.md")]
#[cfg(feature = "macros")]
pub use sqlx_macros::test;

#[doc(hidden)]
#[cfg(feature = "migrate")]
pub use sqlx_core::testing;

#[doc(hidden)]
pub use sqlx_core::rt::test_block_on;

#[cfg(feature = "any")]
pub mod any;

#[cfg(feature = "macros")]
mod macros;

// macro support
#[cfg(feature = "macros")]
#[doc(hidden)]
pub mod ty_match;

#[cfg(feature = "macros")]
#[doc(hidden)]
pub mod spec_error;

#[doc(hidden)]
pub use sqlx_core::rt as __rt;

/// Conversions between Rust and SQL types.
///
/// To see how each SQL type maps to a Rust type, see the corresponding `types` module for each
/// database:
///
///  * Postgres: [postgres::types]
///  * MySQL: [mysql::types]
///  * SQLite: [sqlite::types]
///
/// Any external types that have had [`Type`] implemented for, are re-exported in this module
/// for convenience as downstream users need to use a compatible version of the external crate
/// to take advantage of the implementation.
///
/// [`Type`]: types::Type
pub mod types {
    pub use sqlx_core::types::*;

    #[cfg(feature = "derive")]
    #[doc(hidden)]
    pub use sqlx_macros::Type;
}

/// Provides [`Encode`] for encoding values for the database.
pub mod encode {
    pub use sqlx_core::encode::{Encode, IsNull};

    #[cfg(feature = "derive")]
    #[doc(hidden)]
    pub use sqlx_macros::Encode;
}

pub use self::encode::Encode;

/// Provides [`Decode`] for decoding values from the database.
pub mod decode {
    pub use sqlx_core::decode::Decode;

    #[cfg(feature = "derive")]
    #[doc(hidden)]
    pub use sqlx_macros::Decode;
}

pub use self::decode::Decode;

/// Types and traits for the `query` family of functions and macros.
pub mod query {
    pub use sqlx_core::query::{Map, Query};
    pub use sqlx_core::query_as::QueryAs;
    pub use sqlx_core::query_scalar::QueryScalar;
}

/// Convenience re-export of common traits.
pub mod prelude {
    pub use super::Acquire;
    pub use super::ConnectOptions;
    pub use super::Connection;
    pub use super::Decode;
    pub use super::Encode;
    pub use super::Executor;
    pub use super::FromRow;
    pub use super::IntoArguments;
    pub use super::Row;
    pub use super::Statement;
    pub use super::Type;
}
