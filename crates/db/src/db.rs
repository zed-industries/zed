use anyhow::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Pool, Sqlite, SqlitePool};
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

pub struct Db(DbStore);

enum DbStore {
    Null,
    Live(Pool<Sqlite>),
}

// Things we need to think about:
// Concurrency? - Needs some research
// We need to configure or setup our database, create the tables and such

// Write our first migration
//

// To make a migration:
// Add to the migrations directory, a file with the name:
//  <NUMBER>_<DESCRIPTION>.sql. Migrations are executed in order of number

impl Db {
    /// Open or create a database at the given file path.
    pub fn open(path: &Path) -> Result<Arc<Self>> {
        let options = SqliteConnectOptions::from_str(path)?.create_if_missing(true);

        Self::initialize(options)
    }

    /// Open a fake database for testing.
    #[cfg(any(test, feature = "test-support"))]
    pub fn open_fake() -> Arc<Self> {
        let options = SqliteConnectOptions::from_str(":memory:")?;

        Self::initialize(options)
    }

    fn initialize(options: SqliteConnectOptions) -> Result<Arc<Self>> {
        let pool = Pool::<Sqlite>::connect_with(options)?;

        sqlx::migrate!().run(&pool).await?;

        Ok(Arc::new(Self(DbStore::Live(pool))))
    }

    /// Open a null database that stores no data, for use as a fallback
    /// when there is an error opening the real database.
    pub fn null() -> Arc<Self> {
        Arc::new(Self(DbStore::Null))
    }

    pub fn read<K, I>(&self, keys: I) -> Result<Vec<Option<Vec<u8>>>>
    where
        K: AsRef<[u8]>,
        I: IntoIterator<Item = K>,
    {
        match &self.0 {
            DbStore::Real(db) => db
                .multi_get(keys)
                .into_iter()
                .map(|e| e.map_err(Into::into))
                .collect(),

            DbStore::Null => Ok(keys.into_iter().map(|_| None).collect()),
        }
    }

    pub fn delete<K, I>(&self, keys: I) -> Result<()>
    where
        K: AsRef<[u8]>,
        I: IntoIterator<Item = K>,
    {
        match &self.0 {
            DbStore::Real(db) => {
                let mut batch = rocksdb::WriteBatch::default();
                for key in keys {
                    batch.delete(key);
                }
                db.write(batch)?;
            }

            DbStore::Null => {}
        }
        Ok(())
    }

    pub fn write<K, V, I>(&self, entries: I) -> Result<()>
    where
        K: AsRef<[u8]>,
        V: AsRef<[u8]>,
        I: IntoIterator<Item = (K, V)>,
    {
        match &self.0 {
            DbStore::Real(db) => {
                let mut batch = rocksdb::WriteBatch::default();
                for (key, value) in entries {
                    batch.put(key, value);
                }
                db.write(batch)?;
            }

            DbStore::Null => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    #[gpui::test]
    fn test_db() {
        let dir = TempDir::new("db-test").unwrap();
        let fake_db = Db::open_fake();
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
