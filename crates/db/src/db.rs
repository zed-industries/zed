pub mod kvp;

use std::fs::create_dir_all;
use std::path::Path;

#[cfg(any(test, feature = "test-support"))]
use anyhow::Result;
use indoc::indoc;
#[cfg(any(test, feature = "test-support"))]
use sqlez::connection::Connection;
use sqlez::domain::{Domain, Migrator};
use sqlez::thread_safe_connection::ThreadSafeConnection;
use util::channel::RELEASE_CHANNEL_NAME;
use util::paths::DB_DIR;

const INITIALIZE_QUERY: &'static str = indoc! {"
    PRAGMA journal_mode=WAL;
    PRAGMA synchronous=NORMAL;
    PRAGMA busy_timeout=1;
    PRAGMA foreign_keys=TRUE;
    PRAGMA case_sensitive_like=TRUE;
"};

/// Open or create a database at the given directory path.
pub fn open_file_db<M: Migrator>() -> ThreadSafeConnection<M> {
    // Use 0 for now. Will implement incrementing and clearing of old db files soon TM
    let current_db_dir = (*DB_DIR).join(Path::new(&format!("0-{}", *RELEASE_CHANNEL_NAME)));

    // if *RELEASE_CHANNEL == ReleaseChannel::Dev {
    //     remove_dir_all(&current_db_dir).ok();
    // }

    create_dir_all(&current_db_dir).expect("Should be able to create the database directory");
    let db_path = current_db_dir.join(Path::new("db.sqlite"));

    ThreadSafeConnection::new(db_path.to_string_lossy().as_ref(), true)
        .with_initialize_query(INITIALIZE_QUERY)
}

pub fn open_memory_db<D: Domain>(db_name: &str) -> ThreadSafeConnection<D> {
    ThreadSafeConnection::new(db_name, false).with_initialize_query(INITIALIZE_QUERY)
}

#[cfg(any(test, feature = "test-support"))]
pub fn write_db_to<D: Domain, P: AsRef<Path>>(
    conn: &ThreadSafeConnection<D>,
    dest: P,
) -> Result<()> {
    let destination = Connection::open_file(dest.as_ref().to_string_lossy().as_ref());
    conn.backup_main(&destination)
}

/// Implements a basic DB wrapper for a given domain
#[macro_export]
macro_rules! connection {
    ($id:ident: $t:ident<$d:ty>) => {
        pub struct $t(::sqlez::thread_safe_connection::ThreadSafeConnection<$d>);

        impl ::std::ops::Deref for $t {
            type Target = ::sqlez::thread_safe_connection::ThreadSafeConnection<$d>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        lazy_static! {
            pub static ref $id: $t = $t(::db::open_file_db());
        }
    };
}
