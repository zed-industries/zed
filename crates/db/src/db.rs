mod kvp;
mod migrations;

use anyhow::Result;
use migrations::MIGRATIONS;
use parking_lot::Mutex;
use rusqlite::Connection;
use std::path::Path;
use std::sync::Arc;

pub use kvp::*;

pub struct Db {
    connecion: Connection,
    in_memory: bool,
}

// To make a migration:
// Add to the migrations directory, a file with the name:
//  <NUMBER>_<DESCRIPTION>.sql. Migrations are executed in order of number

impl Db {
    /// Open or create a database at the given file path. Falls back to in memory database if the
    /// database at the given path is corrupted
    pub fn open(path: &Path) -> Result<Arc<Mutex<Self>>> {
        let conn = Connection::open(path)?;

        Self::initialize(conn, false).or_else(|_| Self::open_in_memory())
    }

    /// Open a in memory database for testing and as a fallback.
    pub fn open_in_memory() -> Result<Arc<Mutex<Self>>> {
        let conn = Connection::open_in_memory()?;

        Self::initialize(conn, true)
    }

    fn initialize(mut conn: Connection, in_memory: bool) -> Result<Arc<Mutex<Self>>> {
        MIGRATIONS.to_latest(&mut conn)?;

        Ok(Arc::new(Mutex::new(Self {
            connecion: conn,
            in_memory,
        })))
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
            assert_eq!(
                db.read(["key-1", "key-2", "key-3"]).unwrap(),
                &[None, None, None]
            );

            db.write([("key-1", "one"), ("key-3", "three")]).unwrap();
            assert_eq!(
                db.read(["key-1", "key-2", "key-3"]).unwrap(),
                &[
                    Some("one".as_bytes().to_vec()),
                    None,
                    Some("three".as_bytes().to_vec())
                ]
            );

            db.delete(["key-3", "key-4"]).unwrap();
            assert_eq!(
                db.read(["key-1", "key-2", "key-3"]).unwrap(),
                &[Some("one".as_bytes().to_vec()), None, None,]
            );
        }

        drop(real_db);

        let real_db = Db::open(&dir.path().join("test.db")).unwrap();
        assert_eq!(
            real_db.read(["key-1", "key-2", "key-3"]).unwrap(),
            &[Some("one".as_bytes().to_vec()), None, None,]
        );
    }
}
