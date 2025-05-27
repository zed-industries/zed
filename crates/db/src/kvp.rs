use gpui::App;
use sqlez_macros::sql;
use util::ResultExt as _;

use crate::{define_connection, query, write_and_log};

define_connection!(pub static ref KEY_VALUE_STORE: KeyValueStore<()> =
    &[sql!(
        CREATE TABLE IF NOT EXISTS kv_store(
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        ) STRICT;
    )];
);

pub trait Dismissable {
    const KEY: &'static str;

    fn dismissed() -> bool {
        KEY_VALUE_STORE
            .read_kvp(Self::KEY)
            .log_err()
            .map_or(false, |s| s.is_some())
    }

    fn set_dismissed(is_dismissed: bool, cx: &mut App) {
        write_and_log(cx, move || async move {
            if is_dismissed {
                KEY_VALUE_STORE
                    .write_kvp(Self::KEY.into(), "1".into())
                    .await
            } else {
                KEY_VALUE_STORE.delete_kvp(Self::KEY.into()).await
            }
        })
    }
}

impl KeyValueStore {
    query! {
        pub fn read_kvp(key: &str) -> Result<Option<String>> {
            SELECT value FROM kv_store WHERE key = (?)
        }
    }

    pub async fn write_kvp(&self, key: String, value: String) -> anyhow::Result<()> {
        log::debug!("Writing key-value pair for key {key}");
        self.write_kvp_inner(key, value).await
    }

    query! {
        async fn write_kvp_inner(key: String, value: String) -> Result<()> {
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
        let db = KeyValueStore::open_test_db("test_kvp").await;

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

define_connection!(pub static ref GLOBAL_KEY_VALUE_STORE: GlobalKeyValueStore<()> =
    &[sql!(
        CREATE TABLE IF NOT EXISTS kv_store(
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        ) STRICT;
    )];
    global
);

impl GlobalKeyValueStore {
    query! {
        pub fn read_kvp(key: &str) -> Result<Option<String>> {
            SELECT value FROM kv_store WHERE key = (?)
        }
    }

    pub async fn write_kvp(&self, key: String, value: String) -> anyhow::Result<()> {
        log::debug!("Writing global key-value pair for key {key}");
        self.write_kvp_inner(key, value).await
    }

    query! {
        async fn write_kvp_inner(key: String, value: String) -> Result<()> {
            INSERT OR REPLACE INTO kv_store(key, value) VALUES ((?), (?))
        }
    }

    query! {
        pub async fn delete_kvp(key: String) -> Result<()> {
            DELETE FROM kv_store WHERE key = (?)
        }
    }
}
