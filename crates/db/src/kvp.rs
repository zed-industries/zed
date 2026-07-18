use anyhow::Context as _;
use gpui::App;
use sqlez_macros::sql;
use util::ResultExt as _;

use crate::{
    query,
    sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
    write_and_log,
};

pub struct KeyValueStore(crate::sqlez::thread_safe_connection::ThreadSafeConnection);

impl KeyValueStore {
    pub fn from_app_db(db: &crate::AppDatabase) -> Self {
        Self(db.0.clone())
    }
}

impl Domain for KeyValueStore {
    const NAME: &str = stringify!(KeyValueStore);

    const MIGRATIONS: &[&str] = &[
        sql!(
            CREATE TABLE IF NOT EXISTS kv_store(
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            ) STRICT;
        ),
        sql!(
            CREATE TABLE IF NOT EXISTS scoped_kv_store(
                namespace TEXT NOT NULL,
                key TEXT NOT NULL,
                value TEXT NOT NULL,
                PRIMARY KEY(namespace, key)
            ) STRICT;
        ),
    ];
}

crate::static_connection!(KeyValueStore, []);

pub trait Dismissable {
    const KEY: &'static str;

    fn dismissed(cx: &App) -> bool {
        KeyValueStore::global(cx)
            .read_kvp(Self::KEY)
            .log_err()
            .is_some_and(|s| s.is_some())
    }

    fn set_dismissed(is_dismissed: bool, cx: &mut App) {
        let db = KeyValueStore::global(cx);
        write_and_log(cx, move || async move {
            if is_dismissed {
                db.write_kvp(Self::KEY.into(), "1".into()).await
            } else {
                db.delete_kvp(Self::KEY.into()).await
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

    pub fn scoped<'a>(&'a self, namespace: &'a str) -> ScopedKeyValueStore<'a> {
        ScopedKeyValueStore {
            store: self,
            namespace,
        }
    }
}

pub struct ScopedKeyValueStore<'a> {
    store: &'a KeyValueStore,
    namespace: &'a str,
}

impl ScopedKeyValueStore<'_> {
    pub fn read(&self, key: &str) -> anyhow::Result<Option<String>> {
        self.store.select_row_bound::<(&str, &str), String>(
            "SELECT value FROM scoped_kv_store WHERE namespace = (?) AND key = (?)",
        )?((self.namespace, key))
        .context("Failed to read from scoped_kv_store")
    }

    pub async fn write(&self, key: String, value: String) -> anyhow::Result<()> {
        let namespace = self.namespace.to_owned();
        self.store
            .write(move |connection| {
                connection.exec_bound::<(&str, &str, &str)>(
                    "INSERT OR REPLACE INTO scoped_kv_store(namespace, key, value) VALUES ((?), (?), (?))",
                )?((&namespace, &key, &value))
                .context("Failed to write to scoped_kv_store")
            })
            .await
    }

    pub async fn delete(&self, key: String) -> anyhow::Result<()> {
        let namespace = self.namespace.to_owned();
        self.store
            .write(move |connection| {
                connection.exec_bound::<(&str, &str)>(
                    "DELETE FROM scoped_kv_store WHERE namespace = (?) AND key = (?)",
                )?((&namespace, &key))
                .context("Failed to delete from scoped_kv_store")
            })
            .await
    }

    pub async fn delete_all(&self) -> anyhow::Result<()> {
        let namespace = self.namespace.to_owned();
        self.store
            .write(move |connection| {
                connection
                    .exec_bound::<&str>("DELETE FROM scoped_kv_store WHERE namespace = (?)")?(
                    &namespace,
                )
                .context("Failed to delete_all from scoped_kv_store")
            })
            .await
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

    #[gpui::test]
    async fn test_scoped_kvp() {
        let db = KeyValueStore::open_test_db("test_scoped_kvp").await;

        let scope_a = db.scoped("namespace-a");
        let scope_b = db.scoped("namespace-b");

        // Reading a missing key returns None
        assert_eq!(scope_a.read("key-1").unwrap(), None);

        // Writing and reading back a key works
        scope_a
            .write("key-1".to_string(), "value-a1".to_string())
            .await
            .unwrap();
        assert_eq!(scope_a.read("key-1").unwrap(), Some("value-a1".to_string()));

        // Two namespaces with the same key don't collide
        scope_b
            .write("key-1".to_string(), "value-b1".to_string())
            .await
            .unwrap();
        assert_eq!(scope_a.read("key-1").unwrap(), Some("value-a1".to_string()));
        assert_eq!(scope_b.read("key-1").unwrap(), Some("value-b1".to_string()));

        // delete removes a single key without affecting others in the namespace
        scope_a
            .write("key-2".to_string(), "value-a2".to_string())
            .await
            .unwrap();
        scope_a.delete("key-1".to_string()).await.unwrap();
        assert_eq!(scope_a.read("key-1").unwrap(), None);
        assert_eq!(scope_a.read("key-2").unwrap(), Some("value-a2".to_string()));
        assert_eq!(scope_b.read("key-1").unwrap(), Some("value-b1".to_string()));

        // delete_all removes all keys in a namespace without affecting other namespaces
        scope_a
            .write("key-3".to_string(), "value-a3".to_string())
            .await
            .unwrap();
        scope_a.delete_all().await.unwrap();
        assert_eq!(scope_a.read("key-2").unwrap(), None);
        assert_eq!(scope_a.read("key-3").unwrap(), None);
        assert_eq!(scope_b.read("key-1").unwrap(), Some("value-b1".to_string()));
    }
}

pub struct GlobalKeyValueStore(ThreadSafeConnection);

impl Domain for GlobalKeyValueStore {
    const NAME: &str = stringify!(GlobalKeyValueStore);
    const MIGRATIONS: &[&str] = &[sql!(
        CREATE TABLE IF NOT EXISTS kv_store(
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        ) STRICT;
    )];
}

impl std::ops::Deref for GlobalKeyValueStore {
    type Target = ThreadSafeConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

static GLOBAL_KEY_VALUE_STORE: std::sync::LazyLock<GlobalKeyValueStore> =
    std::sync::LazyLock::new(|| {
        let db_dir = crate::database_dir();
        GlobalKeyValueStore(gpui::block_on(crate::open_db::<GlobalKeyValueStore>(
            db_dir,
            crate::GlobalDbScope,
        )))
    });

impl GlobalKeyValueStore {
    pub fn global() -> &'static Self {
        &GLOBAL_KEY_VALUE_STORE
    }

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
