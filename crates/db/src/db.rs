pub mod kvp;
pub mod query;

// Re-export
pub use anyhow;
use anyhow::Context;
pub use indoc::indoc;
pub use lazy_static;
use parking_lot::{Mutex, RwLock};
pub use smol;
pub use sqlez;
pub use sqlez_macros;
pub use util::channel::{RELEASE_CHANNEL, RELEASE_CHANNEL_NAME};
pub use util::paths::DB_DIR;

use sqlez::domain::Migrator;
use sqlez::thread_safe_connection::ThreadSafeConnection;
use sqlez_macros::sql;
use std::fs::{create_dir_all, remove_dir_all};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use util::{async_iife, ResultExt};
use util::channel::ReleaseChannel;

const CONNECTION_INITIALIZE_QUERY: &'static str = sql!(
    PRAGMA synchronous=NORMAL;
    PRAGMA busy_timeout=1;
    PRAGMA foreign_keys=TRUE;
    PRAGMA case_sensitive_like=TRUE;
);

const DB_INITIALIZE_QUERY: &'static str = sql!(
    PRAGMA journal_mode=WAL;
);

const FALLBACK_DB_NAME: &'static str = "FALLBACK_MEMORY_DB";

const DB_FILE_NAME: &'static str = "db.sqlite";

lazy_static::lazy_static! {
    static ref DB_FILE_OPERATIONS: Mutex<()> = Mutex::new(());
    static ref DB_WIPED: RwLock<bool> = RwLock::new(false);
    pub static ref BACKUP_DB_PATH: RwLock<Option<PathBuf>> = RwLock::new(None);
    pub static ref ALL_FILE_DB_FAILED: AtomicBool = AtomicBool::new(false);    
}

/// Open or create a database at the given directory path.
/// This will retry a couple times if there are failures. If opening fails once, the db directory
/// is moved to a backup folder and a new one is created. If that fails, a shared in memory db is created.
/// In either case, static variables are set so that the user can be notified.
pub async fn open_db<M: Migrator + 'static>(wipe_db: bool, db_dir: &Path, release_channel: &ReleaseChannel) -> ThreadSafeConnection<M> {
    let release_channel_name = release_channel.dev_name();
    let main_db_dir = db_dir.join(Path::new(&format!("0-{}", release_channel_name)));

    // If WIPE_DB, delete 0-{channel}
    if release_channel == &ReleaseChannel::Dev
        && wipe_db
        && !*DB_WIPED.read()
    {
        let mut db_wiped = DB_WIPED.write();
        if !*db_wiped {
            remove_dir_all(&main_db_dir).ok();
            *db_wiped = true;
        }
    }

    let connection = async_iife!({
        // Note: This still has a race condition where 1 set of migrations succeeds
        // (e.g. (Workspace, Editor)) and another fails (e.g. (Workspace, Terminal))
        // This will cause the first connection to have the database taken out 
        // from under it. This *should* be fine though. The second dabatase failure will
        // cause errors in the log and so should be observed by developers while writing
        // soon-to-be good migrations. If user databases are corrupted, we toss them out
        // and try again from a blank. As long as running all migrations from start to end 
        // on a blank database is ok, this race condition will never be triggered.
        //
        // Basically: Don't ever push invalid migrations to stable or everyone will have
        // a bad time.
        
        // If no db folder, create one at 0-{channel}
        create_dir_all(&main_db_dir).context("Could not create db directory")?;
        let db_path = main_db_dir.join(Path::new(DB_FILE_NAME));
        
        // Optimistically open databases in parallel
        if !DB_FILE_OPERATIONS.is_locked() {
            // Try building a connection
            if let Some(connection) = open_main_db(&db_path).await {
                return Ok(connection)
            };
        }
        
        // Take a lock in the failure case so that we move the db once per process instead 
        // of potentially multiple times from different threads. This shouldn't happen in the
        // normal path
        let _lock = DB_FILE_OPERATIONS.lock();
        if let Some(connection) = open_main_db(&db_path).await {
            return Ok(connection)
        };
        
        let backup_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System clock is set before the unix timestamp, Zed does not support this region of spacetime")
            .as_millis();
        
        // If failed, move 0-{channel} to {current unix timestamp}-{channel}
        let backup_db_dir = db_dir.join(Path::new(&format!(
            "{}-{}",
            backup_timestamp,
            release_channel_name,
        )));

        std::fs::rename(&main_db_dir, &backup_db_dir)
            .context("Failed clean up corrupted database, panicking.")?;

        // Set a static ref with the failed timestamp and error so we can notify the user
        {
            let mut guard = BACKUP_DB_PATH.write();
            *guard = Some(backup_db_dir);
        }
        
        // Create a new 0-{channel}
        create_dir_all(&main_db_dir).context("Should be able to create the database directory")?;
        let db_path = main_db_dir.join(Path::new(DB_FILE_NAME));

        // Try again
        open_main_db(&db_path).await.context("Could not newly created db")
    }).await.log_err();

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

        #[cfg(any(test, feature = "test-support"))]
        $crate::lazy_static::lazy_static! {
            pub static ref $id: $t = $t($crate::smol::block_on($crate::open_test_db(stringify!($id))));
        }

        #[cfg(not(any(test, feature = "test-support")))]
        $crate::lazy_static::lazy_static! {
            pub static ref $id: $t = $t($crate::smol::block_on($crate::open_db(std::env::var("WIPE_DB").is_ok(), &$crate::DB_DIR, &$crate::RELEASE_CHANNEL)));
        }
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
        $crate::lazy_static::lazy_static! {
            pub static ref $id: $t = $t($crate::smol::block_on($crate::open_test_db(stringify!($id))));
        }

        #[cfg(not(any(test, feature = "test-support")))]
        $crate::lazy_static::lazy_static! {
            pub static ref $id: $t = $t($crate::smol::block_on($crate::open_db(std::env::var("WIPE_DB").is_ok(), &$crate::DB_DIR, &$crate::RELEASE_CHANNEL)));
        }
    };
}

#[cfg(test)]
mod tests {
    use std::{thread, fs};

    use sqlez::{domain::Domain, connection::Connection};
    use sqlez_macros::sql;
    use tempdir::TempDir;
    use util::channel::ReleaseChannel;

    use crate::{open_db, DB_FILE_NAME};
    
    // Test that wipe_db exists and works and gives a new db
    #[gpui::test]
    async fn test_wipe_db() {
        enum TestDB {}
        
        impl Domain for TestDB {
            fn name() -> &'static str {
                "db_tests"
            }
            
            fn migrations() -> &'static [&'static str] {
                &[sql!(
                    CREATE TABLE test(value);
                )]
            }
        }
        
        let tempdir = TempDir::new("DbTests").unwrap();
        
        // Create a db and insert a marker value
        let test_db = open_db::<TestDB>(false, tempdir.path(), &util::channel::ReleaseChannel::Dev).await;
        test_db.write(|connection|  
            connection.exec(sql!(
                INSERT INTO test(value) VALUES (10)
            )).unwrap()().unwrap()
        ).await;
        drop(test_db);
        
        // Opening db with wipe clears once and removes the marker value
        let mut guards = vec![];
        for _ in 0..5 {
            let path = tempdir.path().to_path_buf();
            let guard = thread::spawn(move || smol::block_on(async {
                let test_db = open_db::<TestDB>(true, &path, &ReleaseChannel::Dev).await;
                
                assert!(test_db.select_row::<()>(sql!(SELECT value FROM test)).unwrap()().unwrap().is_none())
            }));
            
            guards.push(guard);
        }
        
        for guard in guards {
            guard.join().unwrap();
        }
    }
        
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
                &[sql!(CREATE TABLE test(value);),
                    // failure because test already exists
                  sql!(CREATE TABLE test(value);)]
            }
        }
       
        let tempdir = TempDir::new("DbTests").unwrap();
        let _bad_db = open_db::<BadDB>(false, tempdir.path(), &util::channel::ReleaseChannel::Dev).await;
    }
    
    /// Test that DB exists but corrupted (causing recreate)
    #[gpui::test]
    async fn test_db_corruption() {
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
       
        let tempdir = TempDir::new("DbTests").unwrap();
        {
            let corrupt_db = open_db::<CorruptedDB>(false, tempdir.path(), &util::channel::ReleaseChannel::Dev).await;
            assert!(corrupt_db.persistent());
        }
        
        let good_db = open_db::<GoodDB>(false, tempdir.path(), &util::channel::ReleaseChannel::Dev).await;
        assert!(good_db.select_row::<usize>("SELECT * FROM test2").unwrap()().unwrap().is_none());
        
        let mut corrupted_backup_dir = fs::read_dir(
            tempdir.path()
        ).unwrap().find(|entry| {
            !entry.as_ref().unwrap().file_name().to_str().unwrap().starts_with("0")
        }
        ).unwrap().unwrap().path();
        corrupted_backup_dir.push(DB_FILE_NAME);
        
        dbg!(&corrupted_backup_dir);
        
        let backup = Connection::open_file(&corrupted_backup_dir.to_string_lossy());
        assert!(backup.select_row::<usize>("SELECT * FROM test").unwrap()().unwrap().is_none());
    }
}
