use super::Db;
use anyhow::Result;
use indoc::indoc;
use sqlez::migrations::Migration;

pub(crate) const KVP_MIGRATION: Migration = Migration::new(
    "kvp",
    &[indoc! {"
    CREATE TABLE kv_store(
        key TEXT PRIMARY KEY,
        value TEXT NOT NULL
    ) STRICT;
    "}],
);

impl Db {
    pub fn read_kvp(&self, key: &str) -> Result<Option<String>> {
        self.select_row_bound("SELECT value FROM kv_store WHERE key = (?)")?(key)
    }

    pub fn write_kvp(&self, key: &str, value: &str) -> Result<()> {
        self.exec_bound("INSERT OR REPLACE INTO kv_store(key, value) VALUES ((?), (?))")?((
            key, value,
        ))?;

        Ok(())
    }

    pub fn delete_kvp(&self, key: &str) -> Result<()> {
        self.exec_bound("DELETE FROM kv_store WHERE key = (?)")?(key)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn test_kvp() -> Result<()> {
        let db = Db::open_in_memory("test_kvp");

        assert_eq!(db.read_kvp("key-1").unwrap(), None);

        db.write_kvp("key-1", "one").unwrap();
        assert_eq!(db.read_kvp("key-1").unwrap(), Some("one".to_string()));

        db.write_kvp("key-1", "one-2").unwrap();
        assert_eq!(db.read_kvp("key-1").unwrap(), Some("one-2".to_string()));

        db.write_kvp("key-2", "two").unwrap();
        assert_eq!(db.read_kvp("key-2").unwrap(), Some("two".to_string()));

        db.delete_kvp("key-1").unwrap();
        assert_eq!(db.read_kvp("key-1").unwrap(), None);

        Ok(())
    }
}
