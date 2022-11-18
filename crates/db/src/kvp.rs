use anyhow::Result;
use indoc::indoc;

use sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection};
use std::ops::Deref;

lazy_static::lazy_static! {
    pub static ref KEY_VALUE_STORE: KeyValueStore =
        KeyValueStore(crate::open_file_db());
}

#[derive(Clone)]
pub struct KeyValueStore(ThreadSafeConnection<KeyValueStore>);

impl Domain for KeyValueStore {
    fn name() -> &'static str {
        "kvp"
    }

    fn migrations() -> &'static [&'static str] {
        &[indoc! {"
           CREATE TABLE kv_store(
               key TEXT PRIMARY KEY,
               value TEXT NOT NULL
           ) STRICT;
       "}]
    }
}

impl Deref for KeyValueStore {
    type Target = ThreadSafeConnection<KeyValueStore>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl KeyValueStore {
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

    use crate::kvp::KeyValueStore;

    #[test]
    fn test_kvp() -> Result<()> {
        let db = KeyValueStore(crate::open_memory_db(Some("test_kvp")));

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
