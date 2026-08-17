//! Driver library for using SeaQuery with SQLx
//!
//! This library introduces various traits that add methods to the query types from `sea-query`.
//! For instance, using the [`SqlxBinder`] trait adds a [`SqlxBinder::build_sqlx`] method that
//! returns the query and a [`Values`] object, which can be directly passed to `sqlx`'s
//! [`sqlx::query_with`] method.

#[cfg(feature = "sqlx-any")]
mod sqlx_any;
#[cfg(feature = "sqlx-mysql")]
mod sqlx_mysql;
#[cfg(feature = "sqlx-postgres")]
mod sqlx_postgres;
#[cfg(feature = "sqlx-sqlite")]
mod sqlx_sqlite;

mod values;
pub use crate::values::SqlxValues;

#[cfg(any(
    feature = "sqlx-mysql",
    feature = "sqlx-postgres",
    feature = "sqlx-sqlite",
    feature = "sqlx-any"
))]
mod sqlx;
#[cfg(any(
    feature = "sqlx-mysql",
    feature = "sqlx-postgres",
    feature = "sqlx-sqlite",
    feature = "sqlx-any"
))]
pub use crate::sqlx::SqlxBinder;
