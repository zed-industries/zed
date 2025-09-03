pub mod kvp;
pub mod query;

// Re-export
pub use anyhow;
use anyhow::Context as _;
use gpui::{App, AppContext};
pub use indoc::indoc;
pub use paths::database_dir;
pub use smol;
pub use sqlez;
pub use sqlez_macros;

pub use release_channel::RELEASE_CHANNEL;
use sqlez::domain::Migrator;
use sqlez::thread_safe_connection::ThreadSafeConnection;
use sqlez_macros::sql;
use std::future::Future;
use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::{LazyLock, atomic::Ordering};
use util::{ResultExt, maybe};
use zed_env_vars::ZED_STATELESS;

const CONNECTION_INITIALIZE_QUERY: &str = sql!(
    PRAGMA foreign_keys=TRUE;
);

const DB_INITIALIZE_QUERY: &str = sql!(
    PRAGMA journal_mode=WAL;
    PRAGMA busy_timeout=1;
    PRAGMA case_sensitive_like=TRUE;
    PRAGMA synchronous=NORMAL;
);

const FALLBACK_DB_NAME: &str = "FALLBACK_MEMORY_DB";

const DB_FILE_NAME: &str = "db.sqlite";

pub static ALL_FILE_DB_FAILED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));

/// Open or create a database at the given directory path.
/// This will retry a couple times if there are failures. If opening fails once, the db directory
/// is moved to a backup folder and a new one is created. If that fails, a shared in memory db is created.
/// In either case, static variables are set so that the user can be notified.
pub async fn open_db<M: Migrator + 'static>(db_dir: &Path, scope: &str) -> ThreadSafeConnection {
    if *ZED_STATELESS {
        return open_fallback_db::<M>().await;
    }

    let main_db_dir = db_dir.join(format!("0-{}", scope));

    let connection = maybe!(async {
        smol::fs::create_dir_all(&main_db_dir)
            .await
            .context("Could not create db directory")
            .log_err()?;
        let db_path = main_db_dir.join(Path::new(DB_FILE_NAME));
        open_main_db::<M>(&db_path).await
    })
    .await;

    if let Some(connection) = connection {
        return connection;
    }

    // Set another static ref so that we can escalate the notification
    ALL_FILE_DB_FAILED.store(true, Ordering::Release);

    // If still failed, create an in memory db with a known name
    open_fallback_db::<M>().await
}

async fn open_main_db<M: Migrator>(db_path: &Path) -> Option<ThreadSafeConnection> {
    log::trace!("Opening database {}", db_path.display());
    ThreadSafeConnection::builder::<M>(db_path.to_string_lossy().as_ref(), true)
        .with_db_initialization_query(DB_INITIALIZE_QUERY)
        .with_connection_initialize_query(CONNECTION_INITIALIZE_QUERY)
        .build()
        .await
        .log_err()
}

async fn open_fallback_db<M: Migrator>() -> ThreadSafeConnection {
    log::warn!("Opening fallback in-memory database");
    ThreadSafeConnection::builder::<M>(FALLBACK_DB_NAME, false)
        .with_db_initialization_query(DB_INITIALIZE_QUERY)
        .with_connection_initialize_query(CONNECTION_INITIALIZE_QUERY)
        .build()
        .await
        .expect(
            "Fallback in memory database failed. Likely initialization queries or migrations have fundamental errors",
        )
}

#[cfg(any(test, feature = "test-support"))]
pub async fn open_test_db<M: Migrator>(db_name: &str) -> ThreadSafeConnection {
    use sqlez::thread_safe_connection::locking_queue;

    ThreadSafeConnection::builder::<M>(db_name, false)
        .with_db_initialization_query(DB_INITIALIZE_QUERY)
        .with_connection_initialize_query(CONNECTION_INITIALIZE_QUERY)
        // Serialize queued writes via a mutex and run them synchronously
        .with_write_queue_constructor(locking_queue())
        .build()
        .await
        .unwrap()
}

/// Implements a basic DB wrapper for a given domain
///
/// Arguments:
/// - static variable name for connection
/// - type of connection wrapper
/// - dependencies, whose migrations should be run prior to this domain's migrations
#[macro_export]
macro_rules! static_connection {
    ($id:ident, $t:ident, [ $($d:ty),* ] $(, $global:ident)?) => {
        impl ::std::ops::Deref for $t {
            type Target = $crate::sqlez::thread_safe_connection::ThreadSafeConnection;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl $t {
            #[cfg(any(test, feature = "test-support"))]
            pub async fn open_test_db(name: &'static str) -> Self {
                $t($crate::open_test_db::<$t>(name).await)
            }
        }

        #[cfg(any(test, feature = "test-support"))]
        pub static $id: std::sync::LazyLock<$t> = std::sync::LazyLock::new(|| {
            #[allow(unused_parens)]
            $t($crate::smol::block_on($crate::open_test_db::<($($d,)* $t)>(stringify!($id))))
        });

        #[cfg(not(any(test, feature = "test-support")))]
        pub static $id: std::sync::LazyLock<$t> = std::sync::LazyLock::new(|| {
            let db_dir = $crate::database_dir();
            let scope = if false $(|| stringify!($global) == "global")? {
                "global"
            } else {
                $crate::RELEASE_CHANNEL.dev_name()
            };
            #[allow(unused_parens)]
            $t($crate::smol::block_on($crate::open_db::<($($d,)* $t)>(db_dir, scope)))
        });
    }
}

pub fn write_and_log<F>(cx: &App, db_write: impl FnOnce() -> F + Send + 'static)
where
    F: Future<Output = anyhow::Result<()>> + Send,
{
    cx.background_spawn(async move { db_write().await.log_err() })
        .detach()
}

#[cfg(test)]
mod tests {
    use std::thread;

    use sqlez::domain::Domain;
    use sqlez_macros::sql;

    use crate::open_db;

    // Test bad migration panics
    #[gpui::test]
    #[should_panic]
    async fn test_bad_migration_panics() {
        enum BadDB {}

        impl Domain for BadDB {
            const NAME: &str = "db_tests";
            const MIGRATIONS: &[&str] = &[
                sql!(CREATE TABLE test(value);),
                // failure because test already exists
                sql!(CREATE TABLE test(value);),
            ];
        }

        let tempdir = tempfile::Builder::new()
            .prefix("DbTests")
            .tempdir()
            .unwrap();
        let _bad_db = open_db::<BadDB>(
            tempdir.path(),
            release_channel::ReleaseChannel::Dev.dev_name(),
        )
        .await;
    }

    /// Test that DB exists but corrupted (causing recreate)
    #[gpui::test]
    async fn test_db_corruption(cx: &mut gpui::TestAppContext) {
        cx.executor().allow_parking();

        enum CorruptedDB {}

        impl Domain for CorruptedDB {
            const NAME: &str = "db_tests";
            const MIGRATIONS: &[&str] = &[sql!(CREATE TABLE test(value);)];
        }

        enum GoodDB {}

        impl Domain for GoodDB {
            const NAME: &str = "db_tests"; //Notice same name
            const MIGRATIONS: &[&str] = &[sql!(CREATE TABLE test2(value);)];
        }

        let tempdir = tempfile::Builder::new()
            .prefix("DbTests")
            .tempdir()
            .unwrap();
        {
            let corrupt_db = open_db::<CorruptedDB>(
                tempdir.path(),
                release_channel::ReleaseChannel::Dev.dev_name(),
            )
            .await;
            assert!(corrupt_db.persistent());
        }

        let good_db = open_db::<GoodDB>(
            tempdir.path(),
            release_channel::ReleaseChannel::Dev.dev_name(),
        )
        .await;
        assert!(
            good_db.select_row::<usize>("SELECT * FROM test2").unwrap()()
                .unwrap()
                .is_none()
        );
    }

    /// Test that DB exists but corrupted (causing recreate)
    #[gpui::test(iterations = 30)]
    async fn test_simultaneous_db_corruption(cx: &mut gpui::TestAppContext) {
        cx.executor().allow_parking();

        enum CorruptedDB {}

        impl Domain for CorruptedDB {
            const NAME: &str = "db_tests";

            const MIGRATIONS: &[&str] = &[sql!(CREATE TABLE test(value);)];
        }

        enum GoodDB {}

        impl Domain for GoodDB {
            const NAME: &str = "db_tests"; //Notice same name
            const MIGRATIONS: &[&str] = &[sql!(CREATE TABLE test2(value);)]; // But different migration
        }

        let tempdir = tempfile::Builder::new()
            .prefix("DbTests")
            .tempdir()
            .unwrap();
        {
            // Setup the bad database
            let corrupt_db = open_db::<CorruptedDB>(
                tempdir.path(),
                release_channel::ReleaseChannel::Dev.dev_name(),
            )
            .await;
            assert!(corrupt_db.persistent());
        }

        // Try to connect to it a bunch of times at once
        let mut guards = vec![];
        for _ in 0..10 {
            let tmp_path = tempdir.path().to_path_buf();
            let guard = thread::spawn(move || {
                let good_db = smol::block_on(open_db::<GoodDB>(
                    tmp_path.as_path(),
                    release_channel::ReleaseChannel::Dev.dev_name(),
                ));
                assert!(
                    good_db.select_row::<usize>("SELECT * FROM test2").unwrap()()
                        .unwrap()
                        .is_none()
                );
            });

            guards.push(guard);
        }

        for guard in guards.into_iter() {
            assert!(guard.join().is_ok());
        }
    }
}
