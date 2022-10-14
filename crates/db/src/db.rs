mod kvp;
mod migrations;
mod serialized_item;

use anyhow::Result;
use migrations::MIGRATIONS;
use parking_lot::Mutex;
use rusqlite::Connection;
use std::path::Path;
use std::sync::Arc;

pub use kvp::*;
pub use serialized_item::*;

pub struct Db {
    connection: Mutex<Connection>,
    in_memory: bool,
}

// To make a migration:
// Add to the migrations directory, a file with the name:
//  <NUMBER>_<DESCRIPTION>.sql. Migrations are executed in order of number

impl Db {
    /// Open or create a database at the given file path. Falls back to in memory database if the
    /// database at the given path is corrupted
    pub fn open(path: &Path) -> Result<Arc<Self>> {
        let conn = Connection::open(path)?;

        Self::initialize(conn, false).or_else(|_| Self::open_in_memory())
    }

    /// Open a in memory database for testing and as a fallback.
    pub fn open_in_memory() -> Result<Arc<Self>> {
        let conn = Connection::open_in_memory()?;

        Self::initialize(conn, true)
    }

    fn initialize(mut conn: Connection, in_memory: bool) -> Result<Arc<Self>> {
        MIGRATIONS.to_latest(&mut conn)?;

        Ok(Arc::new(Self {
            connection: Mutex::new(conn),
            in_memory,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    #[gpui::test]
    fn test_db() {
        let dir = TempDir::new("db-test").unwrap();
        let fake_db = Db::open_in_memory().unwrap();
        let real_db = Db::open(&dir.path().join("test.db")).unwrap();

        for db in [&real_db, &fake_db] {
            assert_eq!(db.read_kvp("key-1").unwrap(), None);

            db.write_kvp("key-1", "one").unwrap();
            assert_eq!(db.read_kvp("key-1").unwrap(), Some("one".to_string()));

            db.write_kvp("key-2", "two").unwrap();
            assert_eq!(db.read_kvp("key-2").unwrap(), Some("two".to_string()));

            db.delete_kvp("key-1").unwrap();
            assert_eq!(db.read_kvp("key-1").unwrap(), None);
        }

        drop(real_db);

        let real_db = Db::open(&dir.path().join("test.db")).unwrap();

        real_db.write_kvp("key-1", "one").unwrap();
        assert_eq!(real_db.read_kvp("key-1").unwrap(), None);

        real_db.write_kvp("key-2", "two").unwrap();
        assert_eq!(real_db.read_kvp("key-2").unwrap(), Some("two".to_string()));
    }
}
