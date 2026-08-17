//! **SEE DOCUMENTATION BEFORE USE**. Generic database driver with the specific driver selected at runtime.
//!
//! The underlying database drivers are chosen at runtime from the list set via
//! [`install_drivers`][self::driver::install_drivers]. Any use of `AnyConnection` or `AnyPool`
//! without this will panic.
use crate::executor::Executor;

mod arguments;
pub(crate) mod column;
mod connection;
mod database;
mod error;
mod kind;
mod options;
mod query_result;
pub(crate) mod row;
mod statement;
mod transaction;
pub(crate) mod type_info;
pub mod types;
pub(crate) mod value;

pub mod driver;

#[cfg(feature = "migrate")]
mod migrate;

pub use arguments::{AnyArgumentBuffer, AnyArguments};
pub use column::AnyColumn;
pub use connection::AnyConnection;
// Used internally in `sqlx-macros`

use crate::encode::Encode;
pub use connection::AnyConnectionBackend;
pub use database::Any;
#[allow(deprecated)]
pub use kind::AnyKind;
pub use options::AnyConnectOptions;
pub use query_result::AnyQueryResult;
pub use row::AnyRow;
pub use statement::AnyStatement;
pub use transaction::AnyTransactionManager;
pub use type_info::{AnyTypeInfo, AnyTypeInfoKind};
pub use value::{AnyValue, AnyValueRef};

use crate::types::Type;
#[doc(hidden)]
pub use value::AnyValueKind;

pub type AnyPool = crate::pool::Pool<Any>;

pub type AnyPoolOptions = crate::pool::PoolOptions<Any>;

/// An alias for [`Executor<'_, Database = Any>`][Executor].
pub trait AnyExecutor<'c>: Executor<'c, Database = Any> {}
impl<'c, T: Executor<'c, Database = Any>> AnyExecutor<'c> for T {}

// NOTE: required due to the lack of lazy normalization
impl_into_arguments_for_arguments!(AnyArguments<'q>);
// impl_executor_for_pool_connection!(Any, AnyConnection, AnyRow);
// impl_executor_for_transaction!(Any, AnyRow);
impl_acquire!(Any, AnyConnection);
impl_column_index_for_row!(AnyRow);
impl_column_index_for_statement!(AnyStatement);
// impl_into_maybe_pool!(Any, AnyConnection);

// required because some databases have a different handling of NULL
impl<'q, T> Encode<'q, Any> for Option<T>
where
    T: Encode<'q, Any> + 'q + Type<Any>,
{
    fn encode_by_ref(
        &self,
        buf: &mut AnyArgumentBuffer<'q>,
    ) -> Result<crate::encode::IsNull, crate::error::BoxDynError> {
        if let Some(value) = self {
            value.encode_by_ref(buf)
        } else {
            buf.0.push(AnyValueKind::Null(T::type_info().kind));
            Ok(crate::encode::IsNull::Yes)
        }
    }
}
