use crate::any::connection::AnyConnectionBackend;
use crate::any::{AnyConnectOptions, AnyConnection};
use crate::common::DebugFn;
use crate::connection::Connection;
use crate::database::Database;
use crate::Error;
use futures_core::future::BoxFuture;
use once_cell::sync::OnceCell;
use std::fmt::{Debug, Formatter};
use url::Url;

static DRIVERS: OnceCell<&'static [AnyDriver]> = OnceCell::new();

#[macro_export]
macro_rules! declare_driver_with_optional_migrate {
    ($name:ident = $db:path) => {
        #[cfg(feature = "migrate")]
        pub const $name: $crate::any::driver::AnyDriver =
            $crate::any::driver::AnyDriver::with_migrate::<$db>();

        #[cfg(not(feature = "migrate"))]
        pub const $name: $crate::any::driver::AnyDriver =
            $crate::any::driver::AnyDriver::without_migrate::<$db>();
    };
}

#[non_exhaustive]
pub struct AnyDriver {
    pub(crate) name: &'static str,
    pub(crate) url_schemes: &'static [&'static str],
    pub(crate) connect:
        DebugFn<fn(&AnyConnectOptions) -> BoxFuture<'_, crate::Result<AnyConnection>>>,
    pub(crate) migrate_database: Option<AnyMigrateDatabase>,
}

impl AnyDriver {
    pub const fn without_migrate<DB: Database>() -> Self
    where
        DB::Connection: AnyConnectionBackend,
        <DB::Connection as Connection>::Options:
            for<'a> TryFrom<&'a AnyConnectOptions, Error = Error>,
    {
        Self {
            name: DB::NAME,
            url_schemes: DB::URL_SCHEMES,
            connect: DebugFn(AnyConnection::connect_with_db::<DB>),
            migrate_database: None,
        }
    }

    #[cfg(not(feature = "migrate"))]
    pub const fn with_migrate<DB: Database>() -> Self
    where
        DB::Connection: AnyConnectionBackend,
        <DB::Connection as Connection>::Options:
            for<'a> TryFrom<&'a AnyConnectOptions, Error = Error>,
    {
        Self::without_migrate::<DB>()
    }

    #[cfg(feature = "migrate")]
    pub const fn with_migrate<DB: Database + crate::migrate::MigrateDatabase>() -> Self
    where
        DB::Connection: AnyConnectionBackend,
        <DB::Connection as Connection>::Options:
            for<'a> TryFrom<&'a AnyConnectOptions, Error = Error>,
    {
        Self {
            migrate_database: Some(AnyMigrateDatabase {
                create_database: DebugFn(DB::create_database),
                database_exists: DebugFn(DB::database_exists),
                drop_database: DebugFn(DB::drop_database),
                force_drop_database: DebugFn(DB::force_drop_database),
            }),
            ..Self::without_migrate::<DB>()
        }
    }

    pub fn get_migrate_database(&self) -> crate::Result<&AnyMigrateDatabase> {
        self.migrate_database.as_ref()
            .ok_or_else(|| Error::Configuration(format!("{} driver does not support migrations or the `migrate` feature was not enabled for it", self.name).into()))
    }
}

impl Debug for AnyDriver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnyDriver")
            .field("name", &self.name)
            .field("url_schemes", &self.url_schemes)
            .finish()
    }
}

pub struct AnyMigrateDatabase {
    create_database: DebugFn<fn(&str) -> BoxFuture<'_, crate::Result<()>>>,
    database_exists: DebugFn<fn(&str) -> BoxFuture<'_, crate::Result<bool>>>,
    drop_database: DebugFn<fn(&str) -> BoxFuture<'_, crate::Result<()>>>,
    force_drop_database: DebugFn<fn(&str) -> BoxFuture<'_, crate::Result<()>>>,
}

impl AnyMigrateDatabase {
    pub fn create_database<'a>(&self, url: &'a str) -> BoxFuture<'a, crate::Result<()>> {
        (self.create_database)(url)
    }

    pub fn database_exists<'a>(&self, url: &'a str) -> BoxFuture<'a, crate::Result<bool>> {
        (self.database_exists)(url)
    }

    pub fn drop_database<'a>(&self, url: &'a str) -> BoxFuture<'a, crate::Result<()>> {
        (self.drop_database)(url)
    }

    pub fn force_drop_database<'a>(&self, url: &'a str) -> BoxFuture<'a, crate::Result<()>> {
        (self.force_drop_database)(url)
    }
}

/// Install the list of drivers for [`AnyConnection`] to use.
///
/// Must be called before an `AnyConnection` or `AnyPool` can be connected.
///
/// ### Errors
/// If called more than once.
pub fn install_drivers(
    drivers: &'static [AnyDriver],
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    DRIVERS
        .set(drivers)
        .map_err(|_| "drivers already installed".into())
}

pub(crate) fn from_url_str(url: &str) -> crate::Result<&'static AnyDriver> {
    from_url(&url.parse().map_err(Error::config)?)
}

pub(crate) fn from_url(url: &Url) -> crate::Result<&'static AnyDriver> {
    let scheme = url.scheme();

    let drivers: &[AnyDriver] = DRIVERS
        .get()
        .expect("No drivers installed. Please see the documentation in `sqlx::any` for details.");

    drivers
        .iter()
        .find(|driver| driver.url_schemes.contains(&url.scheme()))
        .ok_or_else(|| {
            Error::Configuration(format!("no driver found for URL scheme {scheme:?}").into())
        })
}
