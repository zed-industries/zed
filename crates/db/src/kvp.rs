use sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection};
use sqlez_macros::sql;

use crate::{open_file_db, open_memory_db, query};

pub struct KeyValueStore(ThreadSafeConnection<KeyValueStore>);

impl std::ops::Deref for KeyValueStore {
    type Target = ThreadSafeConnection<KeyValueStore>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

lazy_static::lazy_static! {
    pub static ref KEY_VALUE_STORE: KeyValueStore = KeyValueStore(if cfg!(any(test, feature = "test-support")) {
        smol::block_on(open_memory_db("KEY_VALUE_STORE"))
    } else {
        smol::block_on(open_file_db())
    });
}

impl Domain for KeyValueStore {
    fn name() -> &'static str {
        "kvp"
    }

    fn migrations() -> &'static [&'static str] {
        &[sql!(
            CREATE TABLE kv_store(
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            ) STRICT;
        )]
    }
}

impl KeyValueStore {
    query! {
        pub fn read_kvp(key: &str) -> Result<Option<String>> {
            SELECT value FROM kv_store WHERE key = (?)
        }
    }

    query! {
        pub async fn write_kvp(key: String, value: String) -> Result<()> {
            INSERT OR REPLACE INTO kv_store(key, value) VALUES ((?), (?))
        }
    }

    query! {
        pub async fn delete_kvp(key: String) -> Result<()> {
            DELETE FROM kv_store WHERE key = (?)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::kvp::KeyValueStore;

    #[gpui::test]
    async fn test_kvp() {
        let db = KeyValueStore(crate::open_memory_db("test_kvp").await);

        assert_eq!(db.read_kvp("key-1").unwrap(), None);

        db.write_kvp("key-1".to_string(), "one".to_string())
            .await
            .unwrap();
        assert_eq!(db.read_kvp("key-1").unwrap(), Some("one".to_string()));

        db.write_kvp("key-1".to_string(), "one-2".to_string())
            .await
            .unwrap();
        assert_eq!(db.read_kvp("key-1").unwrap(), Some("one-2".to_string()));

        db.write_kvp("key-2".to_string(), "two".to_string())
            .await
            .unwrap();
        assert_eq!(db.read_kvp("key-2").unwrap(), Some("two".to_string()));

        db.delete_kvp("key-1".to_string()).await.unwrap();
        assert_eq!(db.read_kvp("key-1").unwrap(), None);
    }
}
