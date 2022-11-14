pub mod kvp;

use std::fs;
use std::path::Path;

#[cfg(any(test, feature = "test-support"))]
use anyhow::Result;
use indoc::indoc;
#[cfg(any(test, feature = "test-support"))]
use sqlez::connection::Connection;
use sqlez::domain::Domain;
use sqlez::thread_safe_connection::ThreadSafeConnection;

const INITIALIZE_QUERY: &'static str = indoc! {"
    PRAGMA journal_mode=WAL;
    PRAGMA synchronous=NORMAL;
    PRAGMA foreign_keys=TRUE;
    PRAGMA case_sensitive_like=TRUE;
"};

/// Open or create a database at the given directory path.
pub fn open_file_db<D: Domain>() -> ThreadSafeConnection<D> {
    // Use 0 for now. Will implement incrementing and clearing of old db files soon TM
    let current_db_dir = (*util::paths::DB_DIR).join(Path::new(&format!(
        "0-{}",
        *util::channel::RELEASE_CHANNEL_NAME
    )));
    fs::create_dir_all(&current_db_dir).expect("Should be able to create the database directory");
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
