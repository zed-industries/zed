use std::sync::Arc;

#[cfg(feature = "async-std")]
use async_lock::RwLock;
use oo7::{Keyring, Secret, dbus, file};
use tempfile::tempdir;
#[cfg(feature = "tokio")]
use tokio::sync::RwLock;

async fn all_backends(temp_dir: tempfile::TempDir) -> Vec<Keyring> {
    let mut backends = Vec::new();

    let keyring_path = temp_dir.path().join("test.keyring");
    let secret = Secret::from([1, 2].into_iter().cycle().take(64).collect::<Vec<_>>());
    let unlocked = file::UnlockedKeyring::load(&keyring_path, secret)
        .await
        .unwrap();
    let keyring = Keyring::File(Arc::new(RwLock::new(Some(file::Keyring::Unlocked(
        unlocked,
    )))));

    backends.push(keyring);

    let service = dbus::Service::new().await.unwrap();
    if let Ok(collection) = service.default_collection().await {
        backends.push(Keyring::DBus(collection));
    }

    backends
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn create_and_retrieve_items() {
    let temp_dir = tempdir().unwrap();
    let backends = all_backends(temp_dir).await;

    for (idx, keyring) in backends.iter().enumerate() {
        println!("Running test on backend {}", idx);

        keyring
            .create_item(
                "Item 1",
                &[
                    ("test-name", "create_and_retrieve_items"),
                    ("user", "alice"),
                ],
                "secret1",
                false,
            )
            .await
            .unwrap();
        keyring
            .create_item(
                "Item 2",
                &[("test-name", "create_and_retrieve_items"), ("user", "bob")],
                "secret2",
                false,
            )
            .await
            .unwrap();

        let items = keyring
            .search_items(&[("test-name", "create_and_retrieve_items")])
            .await
            .unwrap();
        assert_eq!(items.len(), 2);

        let alice_items = keyring
            .search_items(&[
                ("test-name", "create_and_retrieve_items"),
                ("user", "alice"),
            ])
            .await
            .unwrap();
        assert_eq!(alice_items.len(), 1);
        assert_eq!(alice_items[0].label().await.unwrap(), "Item 1");

        keyring
            .delete(&[("test-name", "create_and_retrieve_items")])
            .await
            .unwrap();
    }
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn delete_items() {
    let temp_dir = tempdir().unwrap();
    let backends = all_backends(temp_dir).await;

    for (idx, keyring) in backends.iter().enumerate() {
        println!("Running test on backend {}", idx);

        keyring
            .create_item(
                "Item 1",
                &[("test-name", "delete_items"), ("app", "test")],
                "secret1",
                false,
            )
            .await
            .unwrap();
        keyring
            .create_item(
                "Item 2",
                &[("test-name", "delete_items"), ("app", "other")],
                "secret2",
                false,
            )
            .await
            .unwrap();

        keyring
            .delete(&[("test-name", "delete_items"), ("app", "test")])
            .await
            .unwrap();

        let items = keyring
            .search_items(&[("test-name", "delete_items")])
            .await
            .unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].label().await.unwrap(), "Item 2");

        keyring
            .delete(&[("test-name", "delete_items")])
            .await
            .unwrap();
    }
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn item_update_label() {
    let temp_dir = tempdir().unwrap();
    let backends = all_backends(temp_dir).await;

    for (idx, keyring) in backends.iter().enumerate() {
        println!("Running test on backend {}", idx);

        keyring
            .create_item(
                "Original Label",
                &[("test-name", "item_update_label")],
                "secret",
                false,
            )
            .await
            .unwrap();

        let items = keyring
            .search_items(&[("test-name", "item_update_label")])
            .await
            .unwrap();
        let item = &items[0];

        assert_eq!(item.label().await.unwrap(), "Original Label");

        item.set_label("New Label").await.unwrap();
        assert_eq!(item.label().await.unwrap(), "New Label");

        let items = keyring
            .search_items(&[("test-name", "item_update_label")])
            .await
            .unwrap();
        assert_eq!(items[0].label().await.unwrap(), "New Label");

        keyring
            .delete(&[("test-name", "item_update_label")])
            .await
            .unwrap();
    }
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn item_update_attributes() {
    let temp_dir = tempdir().unwrap();
    let backends = all_backends(temp_dir).await;

    for (idx, keyring) in backends.iter().enumerate() {
        println!("Running test on backend {}", idx);

        keyring
            .create_item(
                "Test",
                &[("test-name", "item_update_attributes"), ("version", "1.0")],
                "secret",
                false,
            )
            .await
            .unwrap();

        let items = keyring
            .search_items(&[("test-name", "item_update_attributes")])
            .await
            .unwrap();
        let item = &items[0];

        item.set_attributes(&[("test-name", "item_update_attributes"), ("version", "2.0")])
            .await
            .unwrap();

        let attrs = item.attributes().await.unwrap();
        assert_eq!(attrs.get("version").unwrap(), "2.0");

        // Test edge case: set_attributes when item doesn't exist in keyring
        if idx == 0 {
            keyring
                .delete(&[("test-name", "item_update_attributes")])
                .await
                .unwrap();

            item.set_attributes(&[("test-name", "item_update_attributes"), ("version", "3.0")])
                .await
                .unwrap();

            let new_items = keyring
                .search_items(&[("test-name", "item_update_attributes")])
                .await
                .unwrap();
            assert_eq!(new_items.len(), 1);
        }

        keyring
            .delete(&[("test-name", "item_update_attributes")])
            .await
            .unwrap();
    }
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn item_update_secret() {
    let temp_dir = tempdir().unwrap();
    let backends = all_backends(temp_dir).await;

    for (idx, keyring) in backends.iter().enumerate() {
        println!("Running test on backend {}", idx);

        keyring
            .create_item(
                "Test",
                &[("test-name", "item_update_secret")],
                "old_secret",
                false,
            )
            .await
            .unwrap();

        let items = keyring
            .search_items(&[("test-name", "item_update_secret")])
            .await
            .unwrap();
        let item = &items[0];

        assert_eq!(item.secret().await.unwrap(), Secret::text("old_secret"));

        item.set_secret("new_secret").await.unwrap();
        assert_eq!(item.secret().await.unwrap(), Secret::text("new_secret"));

        keyring
            .delete(&[("test-name", "item_update_secret")])
            .await
            .unwrap();
    }
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn item_delete() {
    let temp_dir = tempdir().unwrap();
    let backends = all_backends(temp_dir).await;

    for (idx, keyring) in backends.iter().enumerate() {
        println!("Running test on backend {}", idx);

        keyring
            .create_item(
                "Item 1",
                &[("test-name", "item_delete"), ("id", "1")],
                "secret1",
                false,
            )
            .await
            .unwrap();
        keyring
            .create_item(
                "Item 2",
                &[("test-name", "item_delete"), ("id", "2")],
                "secret2",
                false,
            )
            .await
            .unwrap();

        let items = keyring
            .search_items(&[("test-name", "item_delete")])
            .await
            .unwrap();
        assert_eq!(items.len(), 2);

        items[0].delete().await.unwrap();

        let items = keyring
            .search_items(&[("test-name", "item_delete")])
            .await
            .unwrap();
        assert_eq!(items.len(), 1);

        keyring
            .delete(&[("test-name", "item_delete")])
            .await
            .unwrap();
    }
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn item_replace() {
    let temp_dir = tempdir().unwrap();
    let backends = all_backends(temp_dir).await;

    for (idx, keyring) in backends.iter().enumerate() {
        println!("Running test on backend {}", idx);

        keyring
            .create_item("Item 1", &[("test-name", "item_replace")], "secret1", false)
            .await
            .unwrap();

        keyring
            .create_item("Item 2", &[("test-name", "item_replace")], "secret2", true)
            .await
            .unwrap();

        let items = keyring
            .search_items(&[("test-name", "item_replace")])
            .await
            .unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].label().await.unwrap(), "Item 2");
        assert_eq!(items[0].secret().await.unwrap(), Secret::text("secret2"));

        // Cleanup
        keyring
            .delete(&[("test-name", "item_replace")])
            .await
            .unwrap();
    }
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn item_timestamps() {
    let temp_dir = tempdir().unwrap();
    let backends = all_backends(temp_dir).await;

    for (idx, keyring) in backends.iter().enumerate() {
        println!("Running test on backend {}", idx);

        keyring
            .create_item("Test", &[("test-name", "item_timestamps")], "secret", false)
            .await
            .unwrap();

        let items = keyring
            .search_items(&[("test-name", "item_timestamps")])
            .await
            .unwrap();
        let item = &items[0];

        let created = item.created().await.unwrap();
        let modified = item.modified().await.unwrap();

        assert!(created.as_secs() > 0);
        assert!(modified.as_secs() > 0);

        assert!(modified >= created);

        // Cleanup
        keyring
            .delete(&[("test-name", "item_timestamps")])
            .await
            .unwrap();
    }
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn item_is_locked() {
    let temp_dir = tempdir().unwrap();
    let backends = all_backends(temp_dir).await;

    for (idx, keyring) in backends.iter().enumerate() {
        println!("Running test on backend {}", idx);

        keyring
            .create_item("Test", &[("test-name", "item_is_locked")], "secret", false)
            .await
            .unwrap();

        let items = keyring
            .search_items(&[("test-name", "item_is_locked")])
            .await
            .unwrap();
        let item = &items[0];

        assert!(!item.is_locked().await.unwrap());

        let all_items = keyring.items().await.unwrap();
        assert!(!all_items.is_empty());

        keyring
            .delete(&[("test-name", "item_is_locked")])
            .await
            .unwrap();
    }
}

// File-backend specific tests, as the DBus one require prompting
#[tokio::test]
#[cfg(feature = "tokio")]
async fn file_keyring_lock_unlock() {
    let temp_dir = tempdir().unwrap();
    let backends = all_backends(temp_dir).await;
    let keyring = &backends[0];

    assert!(!keyring.is_locked().await.unwrap());

    keyring.lock().await.unwrap();
    assert!(keyring.is_locked().await.unwrap());

    // Test edge case: locking an already locked keyring
    keyring.lock().await.unwrap();
    assert!(keyring.is_locked().await.unwrap());

    let result = keyring
        .create_item("test", &[("app", "test")], "secret", false)
        .await;
    assert!(matches!(result, Err(oo7::Error::File(file::Error::Locked))));

    if let Keyring::File(kg) = &keyring {
        let mut kg_guard = kg.write().await;
        if let Some(file::Keyring::Locked(locked)) = kg_guard.take() {
            let secret = Secret::from([1, 2].into_iter().cycle().take(64).collect::<Vec<_>>());

            let unlocked = unsafe { locked.unlock_unchecked(secret).await.unwrap() };
            *kg_guard = Some(file::Keyring::Unlocked(unlocked));
        }
    }

    assert!(!keyring.is_locked().await.unwrap());
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn file_item_lock_unlock() {
    let temp_dir = tempdir().unwrap();
    let backends = all_backends(temp_dir).await;
    let keyring = &backends[0];

    keyring
        .create_item("Test Item", &[("app", "test")], "secret", false)
        .await
        .unwrap();

    let items = keyring.items().await.unwrap();
    let item = &items[0];

    assert!(!item.is_locked().await.unwrap());
    assert_eq!(item.secret().await.unwrap(), Secret::text("secret"));

    // Test edge case: unlocking an already unlocked item
    item.unlock().await.unwrap();
    assert!(!item.is_locked().await.unwrap());

    item.lock().await.unwrap();
    assert!(item.is_locked().await.unwrap());

    // Test edge case: locking an already locked item
    item.lock().await.unwrap();
    assert!(item.is_locked().await.unwrap());

    let result = item.secret().await;
    assert!(matches!(result, Err(oo7::Error::File(file::Error::Locked))));

    // Unlock the item
    item.unlock().await.unwrap();
    assert!(!item.is_locked().await.unwrap());
    assert_eq!(item.secret().await.unwrap(), Secret::text("secret"));
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn file_locked_item_operations_fail() {
    let temp_dir = tempdir().unwrap();
    let backends = all_backends(temp_dir).await;
    let keyring = &backends[0];

    keyring
        .create_item("Test", &[("app", "test")], "secret", false)
        .await
        .unwrap();

    let items = keyring.items().await.unwrap();
    let item = &items[0];

    item.lock().await.unwrap();

    assert!(matches!(
        item.label().await,
        Err(oo7::Error::File(file::Error::Locked))
    ));
    assert!(matches!(
        item.attributes().await,
        Err(oo7::Error::File(file::Error::Locked))
    ));
    assert!(matches!(
        item.secret().await,
        Err(oo7::Error::File(file::Error::Locked))
    ));
    assert!(matches!(
        item.set_label("new").await,
        Err(oo7::Error::File(file::Error::Locked))
    ));
    assert!(matches!(
        item.set_attributes(&[("app", "test")]).await,
        Err(oo7::Error::File(file::Error::Locked))
    ));
    assert!(matches!(
        item.set_secret("new").await,
        Err(oo7::Error::File(file::Error::Locked))
    ));
    assert!(matches!(
        item.delete().await,
        Err(oo7::Error::File(file::Error::Locked))
    ));
    assert!(matches!(
        item.created().await,
        Err(oo7::Error::File(file::Error::Locked))
    ));
    assert!(matches!(
        item.modified().await,
        Err(oo7::Error::File(file::Error::Locked))
    ));
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn file_locked_keyring_operations_fail() {
    let temp_dir = tempdir().unwrap();
    let backends = all_backends(temp_dir).await;
    let keyring = &backends[0];

    keyring
        .create_item("Test", &[("app", "test")], "secret", false)
        .await
        .unwrap();

    let items = keyring.items().await.unwrap();
    let item = &items[0];

    keyring.lock().await.unwrap();

    assert!(matches!(
        keyring
            .create_item("test", &[("app", "test")], "secret", false)
            .await,
        Err(oo7::Error::File(file::Error::Locked))
    ));
    assert!(matches!(
        keyring.items().await,
        Err(oo7::Error::File(file::Error::Locked))
    ));
    assert!(matches!(
        keyring.search_items(&[("app", "test")]).await,
        Err(oo7::Error::File(file::Error::Locked))
    ));
    assert!(matches!(
        keyring.delete(&[("app", "test")]).await,
        Err(oo7::Error::File(file::Error::Locked))
    ));

    assert!(matches!(
        item.set_label("new label").await,
        Err(oo7::Error::File(file::Error::Locked))
    ));
    assert!(matches!(
        item.set_attributes(&[("app", "new")]).await,
        Err(oo7::Error::File(file::Error::Locked))
    ));
    assert!(matches!(
        item.set_secret("new secret").await,
        Err(oo7::Error::File(file::Error::Locked))
    ));
    assert!(matches!(
        item.delete().await,
        Err(oo7::Error::File(file::Error::Locked))
    ));
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn file_item_lock_with_locked_keyring_fails() {
    let temp_dir = tempdir().unwrap();
    let backends = all_backends(temp_dir).await;
    let keyring = &backends[0];

    keyring
        .create_item("Test", &[("app", "test")], "secret", false)
        .await
        .unwrap();

    let items = keyring.items().await.unwrap();
    let item = &items[0];

    keyring.lock().await.unwrap();

    let result = item.lock().await;
    assert!(matches!(result, Err(oo7::Error::File(file::Error::Locked))));
}
