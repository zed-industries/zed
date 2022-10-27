use anyhow::Result;
use rusqlite::OptionalExtension;

use super::Db;

pub(crate) const KVP_M_1: &str = "
CREATE TABLE kv_store(
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
) STRICT;
";

impl Db {
    pub fn read_kvp(&self, key: &str) -> Result<Option<String>> {
        self.real()
            .map(|db| {
                let lock = db.connection.lock();
                let mut stmt = lock.prepare_cached("SELECT value FROM kv_store WHERE key = (?)")?;

                Ok(stmt.query_row([key], |row| row.get(0)).optional()?)
            })
            .unwrap_or(Ok(None))
    }

    pub fn write_kvp(&self, key: &str, value: &str) -> Result<()> {
        self.real()
            .map(|db| {
                let lock = db.connection.lock();

                let mut stmt = lock.prepare_cached(
                    "INSERT OR REPLACE INTO kv_store(key, value) VALUES ((?), (?))",
                )?;

                stmt.execute([key, value])?;

                Ok(())
            })
            .unwrap_or(Ok(()))
    }

    pub fn delete_kvp(&self, key: &str) -> Result<()> {
        self.real()
            .map(|db| {
                let lock = db.connection.lock();

                let mut stmt = lock.prepare_cached("DELETE FROM kv_store WHERE key = (?)")?;

                stmt.execute([key])?;

                Ok(())
            })
            .unwrap_or(Ok(()))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn test_kvp() -> Result<()> {
        let db = Db::open_in_memory();

        assert_eq!(db.read_kvp("key-1")?, None);

        db.write_kvp("key-1", "one")?;
        assert_eq!(db.read_kvp("key-1")?, Some("one".to_string()));

        db.write_kvp("key-1", "one-2")?;
        assert_eq!(db.read_kvp("key-1")?, Some("one-2".to_string()));

        db.write_kvp("key-2", "two")?;
        assert_eq!(db.read_kvp("key-2")?, Some("two".to_string()));

        db.delete_kvp("key-1")?;
        assert_eq!(db.read_kvp("key-1")?, None);

        Ok(())
    }
}
