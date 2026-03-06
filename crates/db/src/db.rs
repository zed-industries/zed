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
#[cfg(any(test, feature = "test-support"))]
use std::collections::HashMap;
use std::future::Future;
use std::path::Path;
#[cfg(any(test, feature = "test-support"))]
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::{LazyLock, atomic::Ordering};
#[cfg(any(test, feature = "test-support"))]
use std::{borrow::Cow, ops::Deref};
use util::{ResultExt, maybe};
use zed_env_vars::ZED_STATELESS;

const CONNECTION_INITIALIZE_QUERY: &str = sql!(
    PRAGMA foreign_keys=TRUE;
);

const DB_INITIALIZE_QUERY: &str = sql!(
    PRAGMA journal_mode=WAL;
    PRAGMA busy_timeout=500;
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

    let db_name = scoped_test_db_name(db_name);
    ThreadSafeConnection::builder::<M>(&db_name, false)
        .with_db_initialization_query(DB_INITIALIZE_QUERY)
        .with_connection_initialize_query(CONNECTION_INITIALIZE_QUERY)
        // Serialize queued writes via a mutex and run them synchronously
        .with_write_queue_constructor(locking_queue())
        .build()
        .await
        .unwrap()
}

#[cfg(any(test, feature = "test-support"))]
fn scoped_test_db_name(db_name: &str) -> Cow<'_, str> {
    let Some(test_name) = current_test_scope_name() else {
        return Cow::Borrowed(db_name);
    };

    let mut scoped_name = String::with_capacity(db_name.len() + test_name.len() + 2);
    scoped_name.push_str(db_name);
    scoped_name.push('@');
    for ch in test_name.chars() {
        if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') {
            scoped_name.push(ch);
        } else {
            scoped_name.push('_');
        }
    }

    Cow::Owned(scoped_name)
}

#[cfg(any(test, feature = "test-support"))]
fn current_test_scope_name() -> Option<String> {
    if let Some(test_name) = gpui::current_test_name() {
        return Some(test_name.to_string());
    }

    let current_thread = std::thread::current();
    if let Some(test_name) = current_thread.name() {
        return Some(test_name.to_string());
    }

    Some(format!("thread_{:?}", current_thread.id()))
}

#[cfg(any(test, feature = "test-support"))]
pub struct TestScopedStatic<T: Send + Sync + 'static> {
    initializer: fn() -> T,
    values: Mutex<HashMap<String, &'static T>>,
}

#[cfg(any(test, feature = "test-support"))]
impl<T: Send + Sync + 'static> TestScopedStatic<T> {
    pub fn new(initializer: fn() -> T) -> Self {
        Self {
            initializer,
            values: Mutex::new(HashMap::new()),
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
impl<T: Send + Sync + 'static> Deref for TestScopedStatic<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let scope_name = current_test_scope_name().unwrap_or_else(|| "default".to_string());
        let mut values = self.values.lock().unwrap();
        *values
            .entry(scope_name)
            .or_insert_with(|| Box::leak(Box::new((self.initializer)())))
    }
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
            pub async fn open_test_db(name: &str) -> Self {
                $t($crate::open_test_db::<$t>(name).await)
            }
        }

        #[cfg(any(test, feature = "test-support"))]
        pub static $id: std::sync::LazyLock<$crate::TestScopedStatic<$t>> =
            std::sync::LazyLock::new(|| {
                fn initializer() -> $t {
                    #[allow(unused_parens)]
                    $t($crate::smol::block_on(
                        $crate::open_test_db::<($($d,)* $t)>(stringify!($id))
                    ))
                }

                $crate::TestScopedStatic::new(initializer)
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
    use anyhow::Result;
    use std::thread;

    use sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection};
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

    pub struct ScopedStaticDb(ThreadSafeConnection);

    impl Domain for ScopedStaticDb {
        const NAME: &str = "test_scoped_static_db";
        const MIGRATIONS: &[&str] = &[sql!(
            CREATE TABLE IF NOT EXISTS scoped_values(
                value INTEGER NOT NULL
            ) STRICT;
        )];
    }

    crate::static_connection!(SCOPED_STATIC_DB, ScopedStaticDb, []);

    impl ScopedStaticDb {
        fn replace_value(&self, value: i64) -> Result<()> {
            smol::block_on(self.write(move |connection| {
                connection.exec("DELETE FROM scoped_values")?()?;
                connection.exec_bound("INSERT INTO scoped_values(value) VALUES (?)")?(value)?;
                anyhow::Ok(())
            }))
        }

        fn read_value(&self) -> Result<Option<i64>> {
            self.select_row::<i64>("SELECT value FROM scoped_values")
                .unwrap()()
        }
    }

    #[test]
    fn static_test_connections_are_scoped_by_test_name() {
        gpui::with_test_name(
            Some("static_test_connections_are_scoped_by_test_name_a"),
            || {
                assert_eq!(SCOPED_STATIC_DB.read_value().unwrap(), None);
                SCOPED_STATIC_DB.replace_value(7).unwrap();
                assert_eq!(SCOPED_STATIC_DB.read_value().unwrap(), Some(7));
            },
        );

        gpui::with_test_name(
            Some("static_test_connections_are_scoped_by_test_name_b"),
            || {
                assert_eq!(SCOPED_STATIC_DB.read_value().unwrap(), None);
                SCOPED_STATIC_DB.replace_value(11).unwrap();
                assert_eq!(SCOPED_STATIC_DB.read_value().unwrap(), Some(11));
            },
        );
    }

    #[test]
    fn static_test_connections_share_state_across_threads_with_same_test_name() {
        gpui::with_test_name(
            Some("static_test_connections_share_state_across_threads_with_same_test_name"),
            || {
                SCOPED_STATIC_DB.replace_value(13).unwrap();

                let test_name = gpui::current_test_name();
                let thread = thread::spawn(move || {
                    gpui::with_test_name(test_name, || SCOPED_STATIC_DB.read_value().unwrap())
                });
                assert_eq!(thread.join().unwrap(), Some(13));
            },
        );
    }
}
