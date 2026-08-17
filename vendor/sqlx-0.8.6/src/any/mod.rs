//! **SEE DOCUMENTATION BEFORE USE**. Runtime-generic database driver.
#![doc = include_str!("install_drivers_note.md")]

use std::sync::Once;

pub use sqlx_core::any::driver::install_drivers;

pub use sqlx_core::any::{
    Any, AnyArguments, AnyConnectOptions, AnyExecutor, AnyPoolOptions, AnyQueryResult, AnyRow,
    AnyStatement, AnyTransactionManager, AnyTypeInfo, AnyTypeInfoKind, AnyValue, AnyValueRef,
};

#[allow(deprecated)]
pub use sqlx_core::any::AnyKind;

pub(crate) mod reexports {
    /// **SEE DOCUMENTATION BEFORE USE**. Type alias for `Pool<Any>`.
    #[doc = include_str!("install_drivers_note.md")]
    pub use sqlx_core::any::AnyPool;

    /// **SEE DOCUMENTATION BEFORE USE**. Runtime-generic database connection.
    #[doc = include_str!("install_drivers_note.md")]
    pub use sqlx_core::any::AnyConnection;
}

/// Install all currently compiled-in drivers for [`AnyConnection`] to use.
///
/// May be called multiple times; only the first call will install drivers, subsequent calls
/// will have no effect.
///
/// ### Panics
/// If [`install_drivers`] has already been called *not* through this function.
///
/// [`AnyConnection`]: sqlx_core::any::AnyConnection
pub fn install_default_drivers() {
    static ONCE: Once = Once::new();

    ONCE.call_once(|| {
        install_drivers(&[
            #[cfg(feature = "mysql")]
            sqlx_mysql::any::DRIVER,
            #[cfg(feature = "postgres")]
            sqlx_postgres::any::DRIVER,
            #[cfg(feature = "_sqlite")]
            sqlx_sqlite::any::DRIVER,
        ])
        .expect("non-default drivers already installed")
    });
}
