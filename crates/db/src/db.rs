pub mod kvp;
pub mod query;

// Re-export
pub use anyhow;
use anyhow::Context as _;
pub use gpui;
use gpui::{App, AppContext, Global};
pub use indoc::indoc;
pub use inventory;
pub use paths::database_dir;
pub use sqlez;
pub use sqlez_macros;
pub use uuid;

use async_lock::Mutex;
pub use release_channel::RELEASE_CHANNEL;
use release_channel::ReleaseChannel;
use sqlez::domain::Migrator;
use sqlez::thread_safe_connection::ThreadSafeConnection;
use sqlez_macros::sql;
use std::fs::create_dir_all;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::{LazyLock, atomic::Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use util::ResultExt;
use zed_env_vars::ZED_STATELESS;

/// A migration registered via `static_connection!` and collected at link time.
pub struct DomainMigration {
    pub name: &'static str,
    pub migrations: &'static [&'static str],
    pub dependencies: &'static [&'static str],
    pub should_allow_migration_change: fn(usize, &str, &str) -> bool,
}

inventory::collect!(DomainMigration);

/// The shared database connection backing all domain-specific DB wrappers.
/// Set as a GPUI global per-App. Falls back to a shared LazyLock if not set.
pub struct AppDatabase(pub ThreadSafeConnection);

impl Global for AppDatabase {}

/// Migrator that runs all inventory-registered domain migrations.
pub struct AppMigrator;

impl Migrator for AppMigrator {
    fn migrate(connection: &sqlez::connection::Connection) -> anyhow::Result<()> {
        let registrations: Vec<&DomainMigration> = inventory::iter::<DomainMigration>().collect();
        let sorted = topological_sort(&registrations);
        for reg in &sorted {
            let mut should_allow = reg.should_allow_migration_change;
            connection.migrate(reg.name, reg.migrations, &mut should_allow)?;
        }
        Ok(())
    }
}

impl AppDatabase {
    /// Opens the production database and runs all inventory-registered
    /// migrations in dependency order.
    pub fn new() -> Self {
        let db_dir = database_dir();
        let connection = gpui::block_on(open_db::<AppMigrator>(db_dir, *RELEASE_CHANNEL));
        Self(connection)
    }

    /// Creates a new in-memory database with a unique name and runs all
    /// inventory-registered migrations in dependency order.
    #[cfg(any(test, feature = "test-support"))]
    pub fn test_new() -> Self {
        let name = format!("test-db-{}", uuid::Uuid::new_v4());
        let connection = gpui::block_on(open_test_db::<AppMigrator>(&name));
        Self(connection)
    }

    /// Returns the per-App connection if set, otherwise falls back to
    /// the shared LazyLock.
    pub fn global(cx: &App) -> &ThreadSafeConnection {
        #[allow(unreachable_code)]
        if let Some(db) = cx.try_global::<Self>() {
            return &db.0;
        } else {
            #[cfg(any(feature = "test-support", test))]
            return &TEST_APP_DATABASE.0;

            panic!("database not initialized")
        }
    }
}

fn topological_sort<'a>(registrations: &[&'a DomainMigration]) -> Vec<&'a DomainMigration> {
    let mut sorted: Vec<&DomainMigration> = Vec::new();
    let mut visited: std::collections::HashSet<&str> = std::collections::HashSet::new();

    fn visit<'a>(
        name: &str,
        registrations: &[&'a DomainMigration],
        sorted: &mut Vec<&'a DomainMigration>,
        visited: &mut std::collections::HashSet<&'a str>,
    ) {
        if visited.contains(name) {
            return;
        }
        if let Some(reg) = registrations.iter().find(|r| r.name == name) {
            for dep in reg.dependencies {
                visit(dep, registrations, sorted, visited);
            }
            visited.insert(reg.name);
            sorted.push(reg);
        }
    }

    for reg in registrations {
        visit(reg.name, registrations, &mut sorted, &mut visited);
    }
    sorted
}

/// Shared fallback `AppDatabase` used when no per-App global is set.
#[cfg(any(test, feature = "test-support"))]
static TEST_APP_DATABASE: LazyLock<AppDatabase> = LazyLock::new(AppDatabase::test_new);

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

/// Directories that `recover_corrupt_db` renamed aside this process's lifetime.
/// Append-only; one entry per successful recovery. Read via [`recovered_db_backups`].
static RECOVERED_DB_BACKUPS: std::sync::Mutex<Vec<PathBuf>> = std::sync::Mutex::new(Vec::new());

/// Serializes concurrent recovery attempts so simultaneous callers (e.g. multiple
/// domain initializations during startup) only produce one backup dir per scope.
static RECOVERY_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

/// Returns every directory that has been renamed aside by corrupt-database
/// recovery during this process's lifetime. Empty when no recovery ran.
pub fn recovered_db_backups() -> Vec<PathBuf> {
    RECOVERED_DB_BACKUPS
        .lock()
        .map(|guard| guard.clone())
        .unwrap_or_default()
}

/// A type that can be used as a database scope for path construction.
pub trait DbScope {
    fn scope_name(&self) -> &str;
}

impl DbScope for ReleaseChannel {
    fn scope_name(&self) -> &str {
        self.dev_name()
    }
}

/// A database scope shared across all release channels.
pub struct GlobalDbScope;

impl DbScope for GlobalDbScope {
    fn scope_name(&self) -> &str {
        "global"
    }
}

/// Returns the path to the `AppDatabase` SQLite file for the given scope
/// under `db_dir`.
pub fn db_path(db_dir: &Path, scope: impl DbScope) -> PathBuf {
    db_dir
        .join(format!("0-{}", scope.scope_name()))
        .join(DB_FILE_NAME)
}

/// Open or create a database at the given directory path.
///
/// If the persistent database fails to open (e.g. corruption after a crash, a
/// stray `-wal` file locked by another process, or a migration that left the
/// file in an inconsistent state), the scope directory is renamed aside as
/// `{unix_ms}-{scope}/` and a fresh one is created in its place. Only if that
/// recovery attempt also fails do we fall back to a shared in-memory database
/// and set `ALL_FILE_DB_FAILED` so the UI can notify the user.
pub async fn open_db<M: Migrator + 'static>(
    db_dir: &Path,
    scope: impl DbScope,
) -> ThreadSafeConnection {
    if *ZED_STATELESS {
        return open_fallback_db::<M>().await;
    }

    let scope_name = scope.scope_name().to_owned();
    let db_path = db_path(db_dir, scope);

    if let Some(parent) = db_path.parent()
        && create_dir_all(parent)
            .context("Could not create db directory")
            .log_err()
            .is_none()
    {
        ALL_FILE_DB_FAILED.store(true, Ordering::Release);
        return open_fallback_db::<M>().await;
    }

    if let Some(connection) = open_main_db::<M>(&db_path).await {
        return connection;
    }

    if let Some(connection) = recover_corrupt_db::<M>(&db_path, &scope_name).await {
        return connection;
    }

    ALL_FILE_DB_FAILED.store(true, Ordering::Release);
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

/// Move a failed scope directory aside and try once more on a clean one.
///
/// Serialized by `RECOVERY_LOCK` so simultaneous callers (e.g. multiple
/// domain initializations during startup) only produce one backup dir.
async fn recover_corrupt_db<M: Migrator>(
    db_path: &Path,
    scope_name: &str,
) -> Option<ThreadSafeConnection> {
    let _guard = RECOVERY_LOCK.lock().await;

    // Another caller may have already recovered while we were waiting.
    if let Some(connection) = open_main_db::<M>(db_path).await {
        return Some(connection);
    }

    let scope_dir = db_path.parent()?;
    let db_dir = scope_dir.parent()?;
    let timestamp_millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .inspect_err(|err| {
            log::warn!("Skipping database recovery: system clock is before UNIX_EPOCH ({err})")
        })
        .ok()?
        .as_millis();
    let backup_dir = db_dir.join(format!("{timestamp_millis}-{scope_name}"));

    std::fs::rename(scope_dir, &backup_dir)
        .with_context(|| {
            format!(
                "Failed to move corrupt database to {}",
                backup_dir.display()
            )
        })
        .log_err()?;
    log::warn!(
        "Database at {} failed to open; moved to {} and retrying with a fresh file.",
        db_path.display(),
        backup_dir.display(),
    );

    create_dir_all(scope_dir)
        .context("Could not recreate database directory after backup")
        .log_err()?;

    let connection = open_main_db::<M>(db_path).await?;
    if let Ok(mut backups) = RECOVERED_DB_BACKUPS.lock() {
        backups.push(backup_dir);
    }
    Some(connection)
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
/// - type of connection wrapper
/// - dependencies, whose migrations should be run prior to this domain's migrations
#[macro_export]
macro_rules! static_connection {
    ($t:ident, [ $($d:ty),* ]) => {
        impl ::std::ops::Deref for $t {
            type Target = $crate::sqlez::thread_safe_connection::ThreadSafeConnection;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::std::clone::Clone for $t {
            fn clone(&self) -> Self {
                $t(self.0.clone())
            }
        }

        impl $t {
            /// Returns an instance backed by the per-App database if set,
            /// or the shared fallback connection otherwise.
            pub fn global(cx: &$crate::gpui::App) -> Self {
                $t($crate::AppDatabase::global(cx).clone())
            }

            #[cfg(any(test, feature = "test-support"))]
            pub async fn open_test_db(name: &'static str) -> Self {
                $t($crate::open_test_db::<$t>(name).await)
            }
        }

        $crate::inventory::submit! {
            $crate::DomainMigration {
                name: <$t as $crate::sqlez::domain::Domain>::NAME,
                migrations: <$t as $crate::sqlez::domain::Domain>::MIGRATIONS,
                dependencies: &[$(<$d as $crate::sqlez::domain::Domain>::NAME),*],
                should_allow_migration_change: <$t as $crate::sqlez::domain::Domain>::should_allow_migration_change,
            }
        }
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
        let _bad_db = open_db::<BadDB>(tempdir.path(), release_channel::ReleaseChannel::Dev).await;
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
            let corrupt_db =
                open_db::<CorruptedDB>(tempdir.path(), release_channel::ReleaseChannel::Dev).await;
            assert!(corrupt_db.persistent());
        }

        let good_db = open_db::<GoodDB>(tempdir.path(), release_channel::ReleaseChannel::Dev).await;
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
            let corrupt_db =
                open_db::<CorruptedDB>(tempdir.path(), release_channel::ReleaseChannel::Dev).await;
            assert!(corrupt_db.persistent());
        }

        // Try to connect to it a bunch of times at once
        let mut guards = vec![];
        for _ in 0..10 {
            let tmp_path = tempdir.path().to_path_buf();
            let guard = thread::spawn(move || {
                let good_db = gpui::block_on(open_db::<GoodDB>(
                    tmp_path.as_path(),
                    release_channel::ReleaseChannel::Dev,
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

    /// When `open_main_db` fails, the scope directory should be renamed to a
    /// timestamped backup and a fresh persistent DB should be opened in its
    /// place — not the shared in-memory fallback.
    #[gpui::test]
    async fn test_corrupt_db_is_backed_up_and_recovered(cx: &mut gpui::TestAppContext) {
        cx.executor().allow_parking();

        enum OldSchema {}
        impl Domain for OldSchema {
            const NAME: &str = "recovery_tests";
            const MIGRATIONS: &[&str] = &[sql!(CREATE TABLE test(value);)];
        }

        enum NewSchema {}
        impl Domain for NewSchema {
            const NAME: &str = "recovery_tests"; // same domain name, different migration
            const MIGRATIONS: &[&str] = &[sql!(CREATE TABLE test2(value);)];
        }

        let tempdir = tempfile::Builder::new()
            .prefix("DbRecoveryTests")
            .tempdir()
            .unwrap();

        {
            let old_db =
                open_db::<OldSchema>(tempdir.path(), release_channel::ReleaseChannel::Dev).await;
            assert!(old_db.persistent());
        }

        let recovered =
            open_db::<NewSchema>(tempdir.path(), release_channel::ReleaseChannel::Dev).await;

        assert!(
            recovered.persistent(),
            "recovery should reopen an on-disk db, not fall back to memory"
        );
        assert!(
            recovered
                .select_row::<usize>("SELECT * FROM test2")
                .unwrap()()
            .unwrap()
            .is_none(),
            "the fresh db should have run NewSchema's migrations"
        );

        let mut scope_dirs: Vec<_> = std::fs::read_dir(tempdir.path())
            .unwrap()
            .filter_map(Result::ok)
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|name| name.ends_with("-dev"))
            .collect();
        scope_dirs.sort();
        assert_eq!(
            scope_dirs.len(),
            2,
            "expected `0-dev/` alongside a single `{{ts}}-dev/` backup, got {scope_dirs:?}"
        );
        assert!(scope_dirs.iter().any(|n| n == "0-dev"));
        assert!(
            scope_dirs
                .iter()
                .any(|n| n != "0-dev" && n.ends_with("-dev")),
            "expected a timestamped backup directory, got {scope_dirs:?}"
        );

        let backups = crate::recovered_db_backups();
        assert!(
            backups.iter().any(|p| p.starts_with(tempdir.path())),
            "recovery should publish a backup path under {:?}, got {backups:?}",
            tempdir.path(),
        );
    }

    /// Reopening the scope after recovery should find the fresh db and return
    /// a persistent connection without producing a second backup — what a user
    /// experiences on the startup *after* the corrupt one.
    #[gpui::test]
    async fn test_reopen_after_recovery_is_clean(cx: &mut gpui::TestAppContext) {
        cx.executor().allow_parking();

        enum OldSchema {}
        impl Domain for OldSchema {
            const NAME: &str = "reopen_after_recovery_tests";
            const MIGRATIONS: &[&str] = &[sql!(CREATE TABLE test(value);)];
        }

        enum NewSchema {}
        impl Domain for NewSchema {
            const NAME: &str = "reopen_after_recovery_tests";
            const MIGRATIONS: &[&str] = &[sql!(CREATE TABLE test2(value);)];
        }

        let tempdir = tempfile::Builder::new()
            .prefix("DbReopenAfterRecoveryTests")
            .tempdir()
            .unwrap();

        {
            let old_db =
                open_db::<OldSchema>(tempdir.path(), release_channel::ReleaseChannel::Dev).await;
            assert!(old_db.persistent());
        }
        {
            let recovered =
                open_db::<NewSchema>(tempdir.path(), release_channel::ReleaseChannel::Dev).await;
            assert!(recovered.persistent());
        }

        let backups_before = count_backup_dirs(tempdir.path());

        {
            let reopened =
                open_db::<NewSchema>(tempdir.path(), release_channel::ReleaseChannel::Dev).await;
            assert!(reopened.persistent());
        }

        assert_eq!(
            count_backup_dirs(tempdir.path()),
            backups_before,
            "a clean reopen must not create another backup"
        );
    }

    fn count_backup_dirs(dir: &std::path::Path) -> usize {
        std::fs::read_dir(dir)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|e| {
                let name = e.file_name();
                let name = name.to_string_lossy();
                name.ends_with("-dev") && name != "0-dev"
            })
            .count()
    }
}
