mod kvp;
mod migrations;

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Result;
use log::error;
use parking_lot::Mutex;
use rusqlite::Connection;

use migrations::MIGRATIONS;

#[derive(Clone)]
pub enum Db {
    Real(Arc<RealDb>),
    Null,
}

pub struct RealDb {
    connection: Mutex<Connection>,
    path: Option<PathBuf>,
}

impl Db {
    /// Open or create a database at the given directory path.
    pub fn open(db_dir: &Path, channel: &'static str) -> Self {
        // Use 0 for now. Will implement incrementing and clearing of old db files soon TM
        let current_db_dir = db_dir.join(Path::new(&format!("0-{}", channel)));
        fs::create_dir_all(&current_db_dir)
            .expect("Should be able to create the database directory");
        let db_path = current_db_dir.join(Path::new("db.sqlite"));

        Connection::open(db_path)
            .map_err(Into::into)
            .and_then(|connection| Self::initialize(connection))
            .map(|connection| {
                Db::Real(Arc::new(RealDb {
                    connection,
                    path: Some(db_dir.to_path_buf()),
                }))
            })
            .unwrap_or_else(|e| {
                error!(
                    "Connecting to file backed db failed. Reverting to null db. {}",
                    e
                );
                Self::Null
            })
    }

    /// Open a in memory database for testing and as a fallback.
    #[cfg(any(test, feature = "test-support"))]
    pub fn open_in_memory() -> Self {
        Connection::open_in_memory()
            .map_err(Into::into)
            .and_then(|connection| Self::initialize(connection))
            .map(|connection| {
                Db::Real(Arc::new(RealDb {
                    connection,
                    path: None,
                }))
            })
            .unwrap_or_else(|e| {
                error!(
                    "Connecting to in memory db failed. Reverting to null db. {}",
                    e
                );
                Self::Null
            })
    }

    fn initialize(mut conn: Connection) -> Result<Mutex<Connection>> {
        MIGRATIONS.to_latest(&mut conn)?;

        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;
        conn.pragma_update(None, "foreign_keys", true)?;
        conn.pragma_update(None, "case_sensitive_like", true)?;

        Ok(Mutex::new(conn))
    }

    pub fn persisting(&self) -> bool {
        self.real().and_then(|db| db.path.as_ref()).is_some()
    }

    pub fn real(&self) -> Option<&RealDb> {
        match self {
            Db::Real(db) => Some(&db),
            _ => None,
        }
    }
}

impl Drop for Db {
    fn drop(&mut self) {
        match self {
            Db::Real(real_db) => {
                let lock = real_db.connection.lock();

                let _ = lock.pragma_update(None, "analysis_limit", "500");
                let _ = lock.pragma_update(None, "optimize", "");
            }
            Db::Null => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::migrations::MIGRATIONS;

    #[test]
    fn test_migrations() {
        assert!(MIGRATIONS.validate().is_ok());
    }
}
