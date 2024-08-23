pub mod kvp;
pub mod query;

// Re-export
pub use anyhow;
use anyhow::Context;
use gpui::AppContext;
pub use indoc::indoc;
pub use paths::database_dir;
pub use smol;
pub use sqlez;
pub use sqlez_macros;

use release_channel::ReleaseChannel;
pub use release_channel::RELEASE_CHANNEL;
use sqlez::domain::Migrator;
use sqlez::thread_safe_connection::ThreadSafeConnection;
use sqlez_macros::sql;
use std::env;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::LazyLock;
use util::{maybe, ResultExt};

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

pub static ZED_STATELESS: LazyLock<bool> =
    LazyLock::new(|| env::var("ZED_STATELESS").map_or(false, |v| !v.is_empty()));

pub static ALL_FILE_DB_FAILED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));

/// Open or create a database at the given directory path.
/// This will retry a couple times if there are failures. If opening fails once, the db directory
/// is moved to a backup folder and a new one is created. If that fails, a shared in memory db is created.
/// In either case, static variables are set so that the user can be notified.
pub async fn open_db<M: Migrator + 'static>(
    db_dir: &Path,
    release_channel: &ReleaseChannel,
) -> ThreadSafeConnection<M> {
    if *ZED_STATELESS {
        return open_fallback_db().await;
    }

    let release_channel_name = release_channel.dev_name();
    let main_db_dir = db_dir.join(Path::new(&format!("0-{}", release_channel_name)));

    let connection = maybe!(async {
        smol::fs::create_dir_all(&main_db_dir)
            .await
            .context("Could not create db directory")
            .log_err()?;
        let db_path = main_db_dir.join(Path::new(DB_FILE_NAME));
        open_main_db(&db_path).await
    })
    .await;

    if let Some(connection) = connection {
        return connection;
    }

    // Set another static ref so that we can escalate the notification
    ALL_FILE_DB_FAILED.store(true, Ordering::Release);

    // If still failed, create an in memory db with a known name
    open_fallback_db().await
}

async fn open_main_db<M: Migrator>(db_path: &PathBuf) -> Option<ThreadSafeConnection<M>> {
    log::info!("Opening main db");
    ThreadSafeConnection::<M>::builder(db_path.to_string_lossy().as_ref(), true)
        .with_db_initialization_query(DB_INITIALIZE_QUERY)
        .with_connection_initialize_query(CONNECTION_INITIALIZE_QUERY)
        .build()
        .await
        .log_err()
}

async fn open_fallback_db<M: Migrator>() -> ThreadSafeConnection<M> {
    log::info!("Opening fallback db");
    ThreadSafeConnection::<M>::builder(FALLBACK_DB_NAME, false)
        .with_db_initialization_query(DB_INITIALIZE_QUERY)
        .with_connection_initialize_query(CONNECTION_INITIALIZE_QUERY)
        .build()
        .await
        .expect(
            "Fallback in memory database failed. Likely initialization queries or migrations have fundamental errors",
        )
}

#[cfg(any(test, feature = "test-support"))]
pub async fn open_test_db<M: Migrator>(db_name: &str) -> ThreadSafeConnection<M> {
    use sqlez::thread_safe_connection::locking_queue;

    ThreadSafeConnection::<M>::builder(db_name, false)
        .with_db_initialization_query(DB_INITIALIZE_QUERY)
        .with_connection_initialize_query(CONNECTION_INITIALIZE_QUERY)
        // Serialize queued writes via a mutex and run them synchronously
        .with_write_queue_constructor(locking_queue())
        .build()
        .await
        .unwrap()
}

/// Implements a basic DB wrapper for a given domain
#[macro_export]
macro_rules! define_connection {
    (pub static ref $id:ident: $t:ident<()> = $migrations:expr;) => {
        pub struct $t($crate::sqlez::thread_safe_connection::ThreadSafeConnection<$t>);

        impl ::std::ops::Deref for $t {
            type Target = $crate::sqlez::thread_safe_connection::ThreadSafeConnection<$t>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl $crate::sqlez::domain::Domain for $t {
            fn name() -> &'static str {
                stringify!($t)
            }

            fn migrations() -> &'static [&'static str] {
                $migrations
            }
        }

        use std::sync::LazyLock;
        #[cfg(any(test, feature = "test-support"))]
        pub static $id: LazyLock<$t> = LazyLock::new(|| {
            $t($crate::smol::block_on($crate::open_test_db(stringify!($id))))
        });

        #[cfg(not(any(test, feature = "test-support")))]
        pub static $id: LazyLock<$t> = LazyLock::new(|| {
            $t($crate::smol::block_on($crate::open_db($crate::database_dir(), &$crate::RELEASE_CHANNEL)))
        });
    };
    (pub static ref $id:ident: $t:ident<$($d:ty),+> = $migrations:expr;) => {
        pub struct $t($crate::sqlez::thread_safe_connection::ThreadSafeConnection<( $($d),+, $t )>);

        impl ::std::ops::Deref for $t {
            type Target = $crate::sqlez::thread_safe_connection::ThreadSafeConnection<($($d),+, $t)>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl $crate::sqlez::domain::Domain for $t {
            fn name() -> &'static str {
                stringify!($t)
            }

            fn migrations() -> &'static [&'static str] {
                $migrations
            }
        }

        #[cfg(any(test, feature = "test-support"))]
        pub static $id: std::sync::LazyLock<$t> = std::sync::LazyLock::new(|| {
            $t($crate::smol::block_on($crate::open_test_db(stringify!($id))))
        });

        #[cfg(not(any(test, feature = "test-support")))]
        pub static $id: std::sync::LazyLock<$t> = std::sync::LazyLock::new(|| {
            $t($crate::smol::block_on($crate::open_db($crate::database_dir(), &$crate::RELEASE_CHANNEL)))
        });
    };
}

pub fn write_and_log<F>(cx: &mut AppContext, db_write: impl FnOnce() -> F + Send + 'static)
where
    F: Future<Output = anyhow::Result<()>> + Send,
{
    cx.background_executor()
        .spawn(async move { db_write().await.log_err() })
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
            fn name() -> &'static str {
                "db_tests"
            }

            fn migrations() -> &'static [&'static str] {
                &[
                    sql!(CREATE TABLE test(value);),
                    // failure because test already exists
                    sql!(CREATE TABLE test(value);),
                ]
            }
        }

        let tempdir = tempfile::Builder::new()
            .prefix("DbTests")
            .tempdir()
            .unwrap();
        let _bad_db = open_db::<BadDB>(tempdir.path(), &release_channel::ReleaseChannel::Dev).await;
    }

    /// Test that DB exists but corrupted (causing recreate)
    #[gpui::test]
    async fn test_db_corruption(cx: &mut gpui::TestAppContext) {
        cx.executor().allow_parking();

        enum CorruptedDB {}

        impl Domain for CorruptedDB {
            fn name() -> &'static str {
                "db_tests"
            }

            fn migrations() -> &'static [&'static str] {
                &[sql!(CREATE TABLE test(value);)]
            }
        }

        enum GoodDB {}

        impl Domain for GoodDB {
            fn name() -> &'static str {
                "db_tests" //Notice same name
            }

            fn migrations() -> &'static [&'static str] {
                &[sql!(CREATE TABLE test2(value);)] //But different migration
            }
        }

        let tempdir = tempfile::Builder::new()
            .prefix("DbTests")
            .tempdir()
            .unwrap();
        {
            let corrupt_db =
                open_db::<CorruptedDB>(tempdir.path(), &release_channel::ReleaseChannel::Dev).await;
            assert!(corrupt_db.persistent());
        }

        let good_db =
            open_db::<GoodDB>(tempdir.path(), &release_channel::ReleaseChannel::Dev).await;
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
            fn name() -> &'static str {
                "db_tests"
            }

            fn migrations() -> &'static [&'static str] {
                &[sql!(CREATE TABLE test(value);)]
            }
        }

        enum GoodDB {}

        impl Domain for GoodDB {
            fn name() -> &'static str {
                "db_tests" //Notice same name
            }

            fn migrations() -> &'static [&'static str] {
                &[sql!(CREATE TABLE test2(value);)] //But different migration
            }
        }

        let tempdir = tempfile::Builder::new()
            .prefix("DbTests")
            .tempdir()
            .unwrap();
        {
            // Setup the bad database
            let corrupt_db =
                open_db::<CorruptedDB>(tempdir.path(), &release_channel::ReleaseChannel::Dev).await;
            assert!(corrupt_db.persistent());
        }

        // Try to connect to it a bunch of times at once
        let mut guards = vec![];
        for _ in 0..10 {
            let tmp_path = tempdir.path().to_path_buf();
            let guard = thread::spawn(move || {
                let good_db = smol::block_on(open_db::<GoodDB>(
                    tmp_path.as_path(),
                    &release_channel::ReleaseChannel::Dev,
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
