use std::{collections::HashMap, path::PathBuf, sync::Arc};

#[cfg(feature = "async-std")]
use async_std::fs;
use oo7::{Secret, XDG_SCHEMA_ATTRIBUTE, file::*};
use tempfile::tempdir;
#[cfg(feature = "tokio")]
use tokio::fs;

fn strong_key() -> Secret {
    Secret::from([1, 2].into_iter().cycle().take(64).collect::<Vec<_>>())
}

#[tokio::test]
async fn repeated_write() -> Result<(), Error> {
    let path = PathBuf::from("../../tests/test.keyring");

    let secret = Secret::from(vec![1, 2]);
    let keyring = UnlockedKeyring::load(&path, secret).await?;

    keyring.write().await?;
    keyring.write().await?;

    Ok(())
}

#[tokio::test]
async fn delete() -> Result<(), Error> {
    let path = PathBuf::from("../../tests/test-delete.keyring");

    let keyring = UnlockedKeyring::load(&path, strong_key()).await?;
    let attributes: HashMap<&str, &str> = HashMap::default();
    keyring
        .create_item("Label", &attributes, "secret", false)
        .await?;

    keyring.delete_item_index(0).await?;

    let result = keyring.delete_item_index(100).await;

    assert!(matches!(result, Err(Error::InvalidItemIndex(100))));

    Ok(())
}

#[tokio::test]
async fn write_with_weak_key() -> Result<(), Error> {
    let path = PathBuf::from("../../tests/write_with_weak_key.keyring");

    let secret = Secret::from(vec![1, 2]);
    let keyring = UnlockedKeyring::load(&path, secret).await?;
    let attributes: HashMap<&str, &str> = HashMap::default();

    let result = keyring
        .create_item("label", &attributes, "my-password", false)
        .await;

    assert!(matches!(
        result,
        Err(Error::WeakKey(WeakKeyError::PasswordTooShort(2)))
    ));

    Ok(())
}

#[tokio::test]
async fn write_with_strong_key() -> Result<(), Error> {
    let path = PathBuf::from("../../tests/write_with_strong_key.keyring");

    let keyring = UnlockedKeyring::load(&path, strong_key()).await?;
    let attributes: HashMap<&str, &str> = HashMap::default();

    keyring
        .create_item("label", &attributes, "my-password", false)
        .await?;

    Ok(())
}

#[tokio::test]
async fn concurrent_writes() -> Result<(), Error> {
    let path = PathBuf::from("../../tests/concurrent_writes.keyring");

    let keyring = Arc::new(UnlockedKeyring::load(&path, strong_key()).await?);

    let keyring_clone = keyring.clone();
    let handle_1 = tokio::task::spawn(async move { keyring_clone.write().await });
    let handle_2 = tokio::task::spawn(async move { keyring.write().await });

    let (res_1, res_2) = futures_util::future::join(handle_1, handle_2).await;
    res_1.unwrap()?;
    res_2.unwrap()?;

    Ok(())
}

async fn check_items(keyring: &UnlockedKeyring) -> Result<(), Error> {
    assert_eq!(keyring.n_items().await, 1);
    let items: Result<Vec<_>, _> = keyring.items().await?.into_iter().collect();
    let items = items.expect("unable to retrieve items");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].as_unlocked().label(), "foo");
    assert_eq!(items[0].as_unlocked().secret(), Secret::blob("foo"));
    let attributes = items[0].as_unlocked().attributes();
    assert_eq!(attributes.len(), 2);
    assert_eq!(
        attributes
            .get(crate::XDG_SCHEMA_ATTRIBUTE)
            .map(|v| v.as_ref()),
        Some("org.gnome.keyring.Note")
    );

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn migrate_from_legacy() -> Result<(), Error> {
    let data_dir = tempdir()?;
    let v0_dir = data_dir.path().join("keyrings");
    let v1_dir = v0_dir.join("v1");
    fs::create_dir_all(&v1_dir).await?;

    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join("legacy.keyring");
    fs::copy(&fixture_path, &v0_dir.join("default.keyring")).await?;

    unsafe {
        std::env::set_var("XDG_DATA_HOME", data_dir.path());
    }

    assert!(!v1_dir.join("default.keyring").exists());

    let secret = Secret::blob("test");
    let keyring = UnlockedKeyring::open("default", secret).await?;

    check_items(&keyring).await?;

    keyring.write().await?;
    assert!(v1_dir.join("default.keyring").exists());

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn migrate() -> Result<(), Error> {
    let data_dir = tempdir()?;
    let v0_dir = data_dir.path().join("keyrings");
    let v1_dir = v0_dir.join("v1");
    fs::create_dir_all(&v1_dir).await?;

    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join("default.keyring");
    fs::copy(&fixture_path, &v0_dir.join("default.keyring")).await?;

    unsafe {
        std::env::set_var("XDG_DATA_HOME", data_dir.path());
    }

    let secret = Secret::blob("test");
    let keyring = UnlockedKeyring::open("default", secret).await?;

    assert!(!v1_dir.join("default.keyring").exists());

    check_items(&keyring).await?;

    keyring.write().await?;
    assert!(v1_dir.join("default.keyring").exists());

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn open_wrong_password() -> Result<(), Error> {
    let data_dir = tempdir()?;
    let v0_dir = data_dir.path().join("keyrings");
    let v1_dir = v0_dir.join("v1");
    fs::create_dir_all(&v1_dir).await?;

    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join("default.keyring");
    fs::copy(&fixture_path, &v1_dir.join("default.keyring")).await?;

    unsafe {
        std::env::set_var("XDG_DATA_HOME", data_dir.path());
    }

    let secret = Secret::blob("wrong");
    let keyring = UnlockedKeyring::open("default", secret).await;

    assert!(keyring.is_err());
    assert!(matches!(keyring.unwrap_err(), Error::IncorrectSecret));

    let secret = Secret::blob("test");
    let keyring = UnlockedKeyring::open("default", secret).await;

    assert!(keyring.is_ok());

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn open() -> Result<(), Error> {
    let data_dir = tempdir()?;
    let v0_dir = data_dir.path().join("keyrings");
    let v1_dir = v0_dir.join("v1");
    fs::create_dir_all(&v1_dir).await?;

    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join("default.keyring");
    fs::copy(&fixture_path, &v1_dir.join("default.keyring")).await?;

    unsafe {
        std::env::set_var("XDG_DATA_HOME", data_dir.path());
    }

    let secret = Secret::blob("test");
    let keyring = UnlockedKeyring::open("default", secret).await?;

    assert!(v1_dir.join("default.keyring").exists());

    check_items(&keyring).await?;

    keyring.write().await?;
    assert!(v1_dir.join("default.keyring").exists());

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn open_nonexistent() -> Result<(), Error> {
    let data_dir = tempdir()?;
    let v0_dir = data_dir.path().join("keyrings");
    let v1_dir = v0_dir.join("v1");
    fs::create_dir_all(&v1_dir).await?;

    unsafe {
        std::env::set_var("XDG_DATA_HOME", data_dir.path());
    }

    let secret = Secret::blob("test");
    let keyring = UnlockedKeyring::open("default", secret).await?;

    assert!(!v1_dir.join("default.keyring").exists());

    keyring
        .create_item(
            "foo",
            &[(crate::XDG_SCHEMA_ATTRIBUTE, "org.gnome.keyring.Note")],
            "foo",
            false,
        )
        .await?;
    keyring.write().await?;

    assert!(v1_dir.join("default.keyring").exists());

    Ok(())
}

#[tokio::test]
async fn delete_broken_items() -> Result<(), Error> {
    const VALID_TO_ADD: usize = 5;
    const BROKEN_TO_ADD: usize = 3;

    let data_dir = tempdir()?;
    let v0_dir = data_dir.path().join("keyrings");
    let v1_dir = v0_dir.join("v1");
    fs::create_dir_all(&v1_dir).await?;

    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join("default.keyring");
    let keyring_path = v1_dir.join("default.keyring");
    fs::copy(&fixture_path, &keyring_path).await?;

    // 1) Load with the correct password and add several valid items. This ensures
    //    valid_items > broken_items that we'll add later.
    let keyring = UnlockedKeyring::load(&keyring_path, Secret::blob("test")).await?;
    for i in 0..VALID_TO_ADD {
        keyring
            .create_item(
                &format!("valid {}", i),
                &[("attr_valid", "value")],
                format!("password_valid_{}", i),
                false,
            )
            .await?;
    }
    drop(keyring);

    // 2) Load_unchecked with the wrong password and add a few "broken" items.
    let keyring = unsafe {
        UnlockedKeyring::load_unchecked(&keyring_path, Secret::blob("wrong_password")).await?
    };
    for i in 0..BROKEN_TO_ADD {
        keyring
            .create_item(
                &format!("bad{}", i),
                &[("attr_bad", "value_bad")],
                format!("pw_bad{}", i),
                false,
            )
            .await?;
    }
    drop(keyring);

    // 3) Load with the correct password and run the deletion.
    let keyring = UnlockedKeyring::load(&keyring_path, Secret::blob("test")).await?;
    let removed = keyring.delete_broken_items().await?;
    assert!(
        removed >= BROKEN_TO_ADD,
        "expected at least {} broken items removed, got {}",
        BROKEN_TO_ADD,
        removed
    );

    // Second call should find nothing left to clean up.
    assert_eq!(keyring.delete_broken_items().await?, 0);

    fs::remove_file(keyring_path).await?;
    Ok(())
}

#[tokio::test]
async fn change_secret() -> Result<(), Error> {
    let data_dir = tempdir()?;
    let v0_dir = data_dir.path().join("keyrings");
    let v1_dir = v0_dir.join("v1");
    fs::create_dir_all(&v1_dir).await?;

    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join("default.keyring");
    let keyring_path = v1_dir.join("default.keyring");
    fs::copy(&fixture_path, &keyring_path).await?;

    let keyring = UnlockedKeyring::load(&keyring_path, Secret::blob("test")).await?;
    let attributes = &[("attr", "value")];
    let item_before = keyring
        .create_item("test", attributes, "password", false)
        .await?;
    let item_before = item_before.as_unlocked();

    let secret = Secret::blob("new_secret");
    keyring.change_secret(secret).await?;

    let secret = Secret::blob("new_secret");
    let keyring = UnlockedKeyring::load(&keyring_path, secret).await?;
    let item_now = keyring.lookup_item(attributes).await?.unwrap();
    let item_now = item_now.as_unlocked();

    assert_eq!(item_before.label(), item_now.label());
    assert_eq!(item_before.secret(), item_now.secret());
    assert_eq!(item_before.attributes(), item_now.attributes());

    // No items were broken during the secret change
    assert_eq!(keyring.delete_broken_items().await?, 0);

    fs::remove_file(keyring_path).await?;

    Ok(())
}

#[tokio::test]
async fn content_type() -> Result<(), Error> {
    let keyring = UnlockedKeyring::temporary(Secret::blob("test_password")).await?;

    // Add items with different MIME types
    keyring
        .create_item(
            "Text",
            &[("type", "text")],
            Secret::text("Hello, World!"),
            false,
        )
        .await?;

    keyring
        .create_item(
            "Password",
            &[("type", "password")],
            Secret::blob("super_secret_password"),
            false,
        )
        .await?;

    let items = keyring.search_items(&[("type", "text")]).await?;
    assert_eq!(items.len(), 1);
    assert_eq!(
        items[0].as_unlocked().secret().content_type(),
        oo7::ContentType::Text
    );

    let items = keyring.search_items(&[("type", "password")]).await?;
    assert_eq!(items.len(), 1);
    assert_eq!(
        items[0].as_unlocked().secret().content_type(),
        oo7::ContentType::Blob
    );

    Ok(())
}

#[tokio::test]
async fn wrong_password_error_type() -> Result<(), Error> {
    let temp_dir = tempdir().unwrap();
    let keyring_path = temp_dir.path().join("wrong_password_test.keyring");
    let correct_secret = Secret::from("correct-password-that-is-long-enough".as_bytes());
    let wrong_secret = Secret::from("wrong-password-that-is-long-enough".as_bytes());

    // Create a keyring with the correct password
    let keyring = UnlockedKeyring::load(&keyring_path, correct_secret).await?;
    keyring
        .create_item("Test Item", &[("app", "test")], "my-secret", false)
        .await?;

    // Try to load with wrong password
    let result = UnlockedKeyring::load(&keyring_path, wrong_secret).await;

    // Verify this returns IncorrectSecret, not ChecksumMismatch
    assert!(matches!(result, Err(Error::IncorrectSecret)));

    Ok(())
}

#[tokio::test]
async fn comprehensive_search_patterns() -> Result<(), Error> {
    let temp_dir = tempdir().unwrap();
    let keyring_path = temp_dir.path().join("search_test.keyring");
    let keyring = UnlockedKeyring::load(&keyring_path, strong_key()).await?;

    // Create diverse test data
    let test_items = vec![
        (
            "Email Password",
            vec![
                ("app", "email"),
                ("user", "alice@example.com"),
                ("type", "password"),
            ],
        ),
        (
            "Email Token",
            vec![
                ("app", "email"),
                ("user", "alice@example.com"),
                ("type", "token"),
            ],
        ),
        (
            "SSH Key",
            vec![("app", "ssh"), ("user", "alice"), ("type", "key")],
        ),
        (
            "Database Password",
            vec![
                ("app", "database"),
                ("env", "production"),
                ("type", "password"),
            ],
        ),
        (
            "API Key",
            vec![("app", "api"), ("service", "github"), ("type", "key")],
        ),
    ];

    for (i, (label, attrs)) in test_items.iter().enumerate() {
        let attrs_map: HashMap<&str, &str> = attrs.iter().cloned().collect();
        keyring
            .create_item(label, &attrs_map, format!("secret{}", i), false)
            .await?;
    }

    // Test exact match
    let exact = keyring
        .search_items(&[
            ("app", "email"),
            ("user", "alice@example.com"),
            ("type", "password"),
        ])
        .await?;
    assert_eq!(exact.len(), 1);
    assert_eq!(exact[0].as_unlocked().label(), "Email Password");

    // Test partial match - by app
    let email_items = keyring.search_items(&[("app", "email")]).await?;
    assert_eq!(email_items.len(), 2);

    // Test partial match - by type
    let passwords = keyring.search_items(&[("type", "password")]).await?;
    assert_eq!(passwords.len(), 2);

    let keys = keyring.search_items(&[("type", "key")]).await?;
    assert_eq!(keys.len(), 2);

    // Test no match
    let nonexistent = keyring.search_items(&[("app", "nonexistent")]).await?;
    assert_eq!(nonexistent.len(), 0);

    Ok(())
}

#[tokio::test]
async fn item_replacement_behavior() -> Result<(), Error> {
    let temp_dir = tempdir().unwrap();
    let keyring_path = temp_dir.path().join("replace_test.keyring");
    let keyring = UnlockedKeyring::load(&keyring_path, strong_key()).await?;

    let attrs = &[("app", "test"), ("user", "alice")];

    // Create initial item
    keyring
        .create_item("Original", attrs, "secret1", false)
        .await?;

    // Verify initial state
    let items = keyring.search_items(attrs).await?;
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].as_unlocked().label(), "Original");
    assert_eq!(items[0].as_unlocked().secret(), Secret::text("secret1"));

    // With replace=false, allows duplicates (discovered behavior)
    keyring
        .create_item("Duplicate", attrs, "secret2", false)
        .await?;

    // Verify we now have 2 items with same attributes
    let items = keyring.search_items(attrs).await?;
    assert_eq!(items.len(), 2);

    // Verify both items exist with different content
    let labels: Vec<_> = items.iter().map(|i| i.as_unlocked().label()).collect();
    assert!(labels.contains(&"Original"));
    assert!(labels.contains(&"Duplicate"));

    // Now test replace=true behavior - should remove existing items with same
    // attributes
    keyring
        .create_item("Replacement", attrs, "secret3", true)
        .await?;

    // After replace=true, should only have the new item
    let items = keyring.search_items(attrs).await?;
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].as_unlocked().label(), "Replacement");
    assert_eq!(items[0].as_unlocked().secret(), Secret::text("secret3"));

    // Test replace=true on empty attributes (should just add)
    let unique_attrs = &[("app", "unique"), ("user", "bob")];
    keyring
        .create_item("Unique Item", unique_attrs, "unique_secret", true)
        .await?;

    let unique_items = keyring.search_items(unique_attrs).await?;
    assert_eq!(unique_items.len(), 1);
    assert_eq!(unique_items[0].as_unlocked().label(), "Unique Item");

    // Test replace=true again on the unique item - should replace it
    keyring
        .create_item("Updated Unique", unique_attrs, "updated_secret", true)
        .await?;

    let unique_items = keyring.search_items(unique_attrs).await?;
    assert_eq!(unique_items.len(), 1);
    assert_eq!(unique_items[0].as_unlocked().label(), "Updated Unique");
    assert_eq!(
        unique_items[0].as_unlocked().secret(),
        Secret::text("updated_secret")
    );

    Ok(())
}

#[tokio::test]
async fn empty_keyring_operations() -> Result<(), Error> {
    let temp_dir = tempdir().unwrap();
    let keyring_path = temp_dir.path().join("empty_test.keyring");
    let keyring = UnlockedKeyring::load(&keyring_path, strong_key()).await?;

    // Test operations on empty keyring
    let items = keyring.items().await?;
    assert_eq!(items.len(), 0);

    let search_results = keyring.search_items(&[("any", "thing")]).await?;
    assert_eq!(search_results.len(), 0);

    // Delete on empty keyring should succeed
    keyring.delete(&[("nonexistent", "key")]).await?;

    // Verify still empty after delete
    assert_eq!(keyring.n_items().await, 0);

    // Test lookup on empty keyring
    let lookup_result = keyring.lookup_item(&[("test", "value")]).await?;
    assert!(lookup_result.is_none());

    Ok(())
}

#[tokio::test]
async fn secret_types_handling() -> Result<(), Error> {
    let temp_dir = tempdir().unwrap();
    let keyring_path = temp_dir.path().join("secret_types_test.keyring");
    let keyring = UnlockedKeyring::load(&keyring_path, strong_key()).await?;

    // Test text secret
    keyring
        .create_item(
            "Text Secret",
            &[("type", "text")],
            Secret::text("Hello, World!"),
            false,
        )
        .await?;

    // Test binary secret
    keyring
        .create_item(
            "Binary Secret",
            &[("type", "binary")],
            Secret::blob(&[0x00, 0x01, 0x02, 0xFF]),
            false,
        )
        .await?;

    // Test large secret
    let large_data = vec![42u8; 10000];
    keyring
        .create_item(
            "Large Secret",
            &[("type", "large")],
            Secret::blob(&large_data),
            false,
        )
        .await?;

    // Test empty secret
    keyring
        .create_item(
            "Empty Secret",
            &[("type", "empty")],
            Secret::text(""),
            false,
        )
        .await?;

    // Verify all secrets can be retrieved correctly
    let text_items = keyring.search_items(&[("type", "text")]).await?;
    assert_eq!(text_items.len(), 1);
    assert_eq!(
        text_items[0].as_unlocked().secret(),
        Secret::text("Hello, World!")
    );
    assert_eq!(
        text_items[0].as_unlocked().secret().content_type(),
        oo7::ContentType::Text
    );

    let binary_items = keyring.search_items(&[("type", "binary")]).await?;
    assert_eq!(binary_items.len(), 1);
    assert_eq!(
        &*binary_items[0].as_unlocked().secret(),
        &[0x00, 0x01, 0x02, 0xFF]
    );
    assert_eq!(
        binary_items[0].as_unlocked().secret().content_type(),
        oo7::ContentType::Blob
    );

    let large_items = keyring.search_items(&[("type", "large")]).await?;
    assert_eq!(large_items.len(), 1);
    assert_eq!(&*large_items[0].as_unlocked().secret(), &large_data);

    let empty_items = keyring.search_items(&[("type", "empty")]).await?;
    assert_eq!(empty_items.len(), 1);
    assert_eq!(empty_items[0].as_unlocked().secret(), Secret::text(""));

    Ok(())
}

#[tokio::test]
async fn item_lifecycle_operations() -> Result<(), Error> {
    let temp_dir = tempdir().unwrap();
    let keyring_path = temp_dir.path().join("lifecycle_test.keyring");
    let keyring = UnlockedKeyring::load(&keyring_path, strong_key()).await?;

    // Test creating multiple items
    keyring
        .create_item(
            "Test Item 1",
            &[("app", "myapp"), ("user", "alice")],
            "secret1",
            false,
        )
        .await?;

    keyring
        .create_item(
            "Test Item 2",
            &[("app", "myapp"), ("user", "bob")],
            "secret2",
            false,
        )
        .await?;

    // Test retrieving all items
    let items = keyring.items().await?;
    let valid_items: Vec<_> = items.into_iter().map(|r| r.unwrap()).collect();
    assert_eq!(valid_items.len(), 2);

    // Test searching by user
    let alice_items = keyring.search_items(&[("user", "alice")]).await?;
    assert_eq!(alice_items.len(), 1);
    assert_eq!(alice_items[0].as_unlocked().label(), "Test Item 1");
    assert_eq!(
        alice_items[0].as_unlocked().secret(),
        Secret::text("secret1")
    );

    // Test searching by app (should find both)
    let app_items = keyring.search_items(&[("app", "myapp")]).await?;
    assert_eq!(app_items.len(), 2);

    // Test deleting items
    keyring.delete(&[("user", "alice")]).await?;
    let remaining_items = keyring.items().await?;
    let valid_remaining: Vec<_> = remaining_items.into_iter().map(|r| r.unwrap()).collect();
    assert_eq!(valid_remaining.len(), 1);
    assert_eq!(valid_remaining[0].as_unlocked().label(), "Test Item 2");

    Ok(())
}

#[tokio::test]
async fn item_attribute_operations() -> Result<(), Error> {
    let temp_dir = tempdir().unwrap();
    let keyring_path = temp_dir.path().join("attr_test.keyring");
    let keyring = UnlockedKeyring::load(&keyring_path, strong_key()).await?;

    // Create item with initial attributes
    keyring
        .create_item(
            "Attribute Test",
            &[("app", "testapp"), ("version", "1.0"), ("env", "test")],
            "test-secret",
            false,
        )
        .await?;

    let items = keyring.search_items(&[("app", "testapp")]).await?;
    assert_eq!(items.len(), 1);
    let item = &items[0].as_unlocked();

    // Test reading attributes
    let attrs = item.attributes();
    assert_eq!(attrs.len(), 4); // 3 + xdg:schema
    assert_eq!(attrs.get("app").unwrap().to_string(), "testapp");
    assert_eq!(attrs.get("version").unwrap().to_string(), "1.0");
    assert_eq!(attrs.get("env").unwrap().to_string(), "test");

    // Test updating attributes
    let index = keyring
        .lookup_item_index(&[("app", "testapp")])
        .await?
        .unwrap();

    let items = keyring.items().await?;
    let mut item_to_replace = items.into_iter().next().unwrap().unwrap();

    if let Item::Unlocked(ref mut unlocked) = item_to_replace {
        unlocked.set_attributes(&[
            ("app", "testapp"),
            ("version", "2.0"),        // updated
            ("env", "production"),     // updated
            ("new_attr", "new_value"), // added
        ]);
    }

    keyring
        .replace_item_index(index, item_to_replace.as_unlocked())
        .await?;

    let updated_items = keyring.search_items(&[("app", "testapp")]).await?;
    assert_eq!(updated_items.len(), 1);
    let updated_attrs = updated_items[0].as_unlocked().attributes();
    assert_eq!(updated_attrs.get("version").unwrap().to_string(), "2.0");
    assert_eq!(updated_attrs.get("env").unwrap().to_string(), "production");
    assert_eq!(
        updated_attrs.get("new_attr").unwrap().to_string(),
        "new_value"
    );

    Ok(())
}

#[tokio::test]
async fn bulk_create_items() -> Result<(), Error> {
    let temp_dir = tempdir().unwrap();
    let keyring_path = temp_dir.path().join("bulk_create_test.keyring");
    let keyring = UnlockedKeyring::load(&keyring_path, strong_key()).await?;

    // Prepare multiple items to create at once
    let items_to_create = vec![
        (
            "Bulk Item 1".to_string(),
            HashMap::from([
                ("app".to_string(), "bulk-app".to_string()),
                ("user".to_string(), "user1".to_string()),
            ]),
            Secret::text("secret1"),
            false,
        ),
        (
            "Bulk Item 2".to_string(),
            HashMap::from([
                ("app".to_string(), "bulk-app".to_string()),
                ("user".to_string(), "user2".to_string()),
            ]),
            Secret::text("secret2"),
            false,
        ),
        (
            "Bulk Item 3".to_string(),
            HashMap::from([
                ("app".to_string(), "bulk-app".to_string()),
                ("user".to_string(), "user3".to_string()),
            ]),
            Secret::text("secret3"),
            false,
        ),
    ];

    // Create all items in bulk
    keyring.create_items(items_to_create).await?;
    // Verify all items were created
    let all_items = keyring.search_items(&[("app", "bulk-app")]).await?;
    assert_eq!(all_items.len(), 3);

    // Test replace=true in bulk create
    let replace_items = vec![(
        "Replaced Item".to_string(),
        HashMap::from([
            ("app".to_string(), "bulk-app".to_string()),
            ("user".to_string(), "user1".to_string()),
        ]),
        Secret::text("new_secret1"),
        true, // replace=true should remove existing item with same attributes
    )];

    keyring.create_items(replace_items).await?;

    // Verify the item was replaced - should still have 3 items total
    let all_items_after = keyring.search_items(&[("app", "bulk-app")]).await?;
    assert_eq!(all_items_after.len(), 3);
    Ok(())
}

#[tokio::test]
async fn partially_corrupted_keyring_error() -> Result<(), Error> {
    let temp_dir = tempdir().unwrap();
    let keyring_path = temp_dir.path().join("partially_corrupted.keyring");

    // Create keyring with correct password and add 2 valid items
    let correct_secret = Secret::from("correct-password-long-enough".as_bytes());
    let keyring = UnlockedKeyring::load(&keyring_path, correct_secret.clone()).await?;
    keyring
        .create_item("valid1", &[("attr", "value1")], "password1", false)
        .await?;
    keyring
        .create_item("valid2", &[("attr", "value2")], "password2", false)
        .await?;
    drop(keyring);

    // Load_unchecked with wrong password and add 3 broken items (more than valid)
    let wrong_secret = Secret::from("wrong-password-long-enough".as_bytes());
    let keyring = unsafe { UnlockedKeyring::load_unchecked(&keyring_path, wrong_secret).await? };
    keyring
        .create_item("broken1", &[("bad", "value1")], "bad_password1", false)
        .await?;
    keyring
        .create_item("broken2", &[("bad", "value2")], "bad_password2", false)
        .await?;
    keyring
        .create_item("broken3", &[("bad", "value3")], "bad_password3", false)
        .await?;
    drop(keyring);

    let result = UnlockedKeyring::load(&keyring_path, correct_secret).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        Error::PartiallyCorruptedKeyring {
            valid_items,
            broken_items,
        } => {
            assert_eq!(valid_items, 2);
            assert_eq!(broken_items, 3);
            assert!(broken_items > valid_items);
        }
        other => panic!("Expected PartiallyCorruptedKeyring, got: {:?}", other),
    }

    Ok(())
}

#[tokio::test]
async fn invalid_item_error_on_decrypt_failure() -> Result<(), Error> {
    let temp_dir = tempdir().unwrap();
    let keyring_path = temp_dir.path().join("invalid_item_test.keyring");

    // 1) Create keyring with correct password and add 2 items
    let correct_secret = Secret::from("correct-password-long-enough".as_bytes());
    let keyring = UnlockedKeyring::load(&keyring_path, correct_secret).await?;
    keyring
        .create_item("item1", &[("app", "test1")], "password1", false)
        .await?;
    keyring
        .create_item("item2", &[("app", "test2")], "password2", false)
        .await?;
    drop(keyring);

    // 2) Load_unchecked with wrong password - items won't decrypt
    let wrong_secret = Secret::from("wrong-password-long-enough".as_bytes());
    let keyring = unsafe { UnlockedKeyring::load_unchecked(&keyring_path, wrong_secret).await? };

    let items_result = keyring.items().await?;
    assert_eq!(items_result.len(), 2);

    assert!(matches!(
        items_result[0].as_ref().unwrap_err(),
        InvalidItemError { .. }
    ));
    assert!(matches!(
        items_result[1].as_ref().unwrap_err(),
        InvalidItemError { .. }
    ));

    Ok(())
}

#[tokio::test]
async fn replace_item_index_invalid() -> Result<(), Error> {
    let temp_dir = tempdir().unwrap();
    let keyring_path = temp_dir.path().join("replace_invalid_index.keyring");
    let keyring = UnlockedKeyring::load(&keyring_path, strong_key()).await?;

    // Create one item
    keyring
        .create_item("Test Item", &[("app", "test")], "secret", false)
        .await?;

    // Try to replace at invalid index
    let items = keyring.items().await?;
    let existing_item = items.into_iter().next().unwrap().unwrap();
    let result = keyring
        .replace_item_index(100, existing_item.as_unlocked())
        .await;

    assert!(matches!(result, Err(Error::InvalidItemIndex(100))));

    Ok(())
}

#[tokio::test]
async fn set_attributes() -> Result<(), Error> {
    let data_dir = tempdir().unwrap();
    let dir = data_dir.path().join("keyrings");
    fs::create_dir_all(&dir).await.unwrap();
    let path = dir.join("default.keyring");

    let keyring = UnlockedKeyring::load(&path, strong_key()).await?;

    let items = keyring.items().await?;
    assert_eq!(items.len(), 0);

    keyring
        .create_item("my item", &vec![("key", "value")], "my_secret", false)
        .await?;

    let mut items = keyring.items().await?;
    assert_eq!(items.len(), 1);
    let mut item = items.remove(0).unwrap();
    let item = item.as_mut_unlocked();
    assert_eq!(item.label(), "my item");
    assert_eq!(item.secret(), Secret::text("my_secret"));
    let attrs = item.attributes();
    assert_eq!(attrs.len(), 2);
    assert_eq!(attrs.get("key").unwrap(), "value");

    // Update attributes on the item
    item.set_attributes(&vec![("key", "changed_value"), ("new_key", "new_value")]);

    // Write the updated item back to the keyring at index 0
    keyring.replace_item_index(0, &item).await?;

    // Now retrieve the item again from the keyring to verify the changes persisted
    let mut items = keyring.items().await?;
    assert_eq!(items.len(), 1);
    let item = items.remove(0).unwrap();
    let item = item.as_unlocked();
    assert_eq!(item.label(), "my item");
    assert_eq!(item.secret(), Secret::text("my_secret"));
    let attrs = item.attributes();
    assert_eq!(attrs.len(), 3);
    assert_eq!(attrs.get("key").unwrap(), "changed_value");
    assert_eq!(attrs.get("new_key").unwrap(), "new_value");

    Ok(())
}
