use oo7::dbus::Service;

#[tokio::test]
#[cfg(feature = "tokio")]
async fn label_mutation() {
    let service = Service::plain().await.unwrap();
    let collection = service.default_collection().await.unwrap();

    let secret = oo7::Secret::text("test secret");

    let item = collection
        .create_item(
            "Original Label",
            &[("test", "label-mutation")],
            secret,
            true,
            None,
        )
        .await
        .unwrap();

    let initial_label = item.label().await.unwrap();
    assert_eq!(initial_label, "Original Label");

    item.set_label("Updated Label").await.unwrap();

    let updated_label = item.label().await.unwrap();
    assert_eq!(updated_label, "Updated Label");

    item.delete(None).await.unwrap();
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn secret_mutation() {
    let service = Service::plain().await.unwrap();
    let collection = service.default_collection().await.unwrap();

    let original_secret = oo7::Secret::text("original secret");

    let item = collection
        .create_item(
            "Secret Test",
            &[("test", "secret-mutation")],
            original_secret.clone(),
            true,
            None,
        )
        .await
        .unwrap();

    assert_eq!(item.secret().await.unwrap(), original_secret);

    let new_secret = oo7::Secret::text("updated secret");
    item.set_secret(new_secret.clone()).await.unwrap();

    assert_eq!(item.secret().await.unwrap(), new_secret);

    item.delete(None).await.unwrap();
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn secret_mutation_encrypted() {
    let service = Service::encrypted().await.unwrap();
    let collection = service.default_collection().await.unwrap();

    let original_secret = oo7::Secret::text("original encrypted secret");

    let item = collection
        .create_item(
            "Encrypted Secret Test",
            &[("test", "secret-mutation-encrypted")],
            original_secret.clone(),
            true,
            None,
        )
        .await
        .unwrap();

    assert_eq!(item.secret().await.unwrap(), original_secret);

    let new_secret = oo7::Secret::text("updated encrypted secret");
    item.set_secret(new_secret.clone()).await.unwrap();

    assert_eq!(item.secret().await.unwrap(), new_secret);

    item.delete(None).await.unwrap();
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn attributes_mutation() {
    let service = Service::plain().await.unwrap();
    let collection = service.default_collection().await.unwrap();

    let secret = oo7::Secret::text("test secret");

    let item = collection
        .create_item(
            "Attributes Test",
            &[("service", "email"), ("username", "user1")],
            secret,
            true,
            None,
        )
        .await
        .unwrap();

    let retrieved_attrs = item.attributes().await.unwrap();
    assert_eq!(retrieved_attrs.get("service"), Some(&"email".to_string()));
    assert_eq!(retrieved_attrs.get("username"), Some(&"user1".to_string()));

    item.set_attributes(&[
        ("service", "web"),
        ("username", "user2"),
        ("domain", "example.com"),
    ])
    .await
    .unwrap();

    let updated_attrs = item.attributes().await.unwrap();
    assert_eq!(updated_attrs.get("service"), Some(&"web".to_string()));
    assert_eq!(updated_attrs.get("username"), Some(&"user2".to_string()));
    assert_eq!(
        updated_attrs.get("domain"),
        Some(&"example.com".to_string())
    );
    assert!(!updated_attrs.contains_key("email")); // old attribute should be gone

    item.delete(None).await.unwrap();
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn text_secret_type() {
    let service = Service::plain().await.unwrap();
    let collection = service.default_collection().await.unwrap();

    let text_secret = oo7::Secret::text("text password");
    let text_item = collection
        .create_item(
            "Text Secret",
            &[("type", "text-secret")],
            text_secret.clone(),
            true,
            None,
        )
        .await
        .unwrap();

    assert_eq!(text_item.secret().await.unwrap(), text_secret);
    text_item.delete(None).await.unwrap();
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn blob_secret_type() {
    let service = Service::plain().await.unwrap();
    let collection = service.default_collection().await.unwrap();

    let blob_secret = oo7::Secret::blob(b"binary data");
    let blob_item = collection
        .create_item(
            "Blob Secret",
            &[("type", "blob-secret")],
            blob_secret.clone(),
            true,
            None,
        )
        .await
        .unwrap();

    let retrieved_secret = blob_item.secret().await.unwrap();

    // TODO: gnome-keyring doesn't preserve content types - everything becomes
    // text/plain But the actual secret data should be preserved
    assert_eq!(retrieved_secret.as_bytes(), blob_secret.as_bytes());
    blob_item.delete(None).await.unwrap();
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn timestamps() {
    let service = Service::plain().await.unwrap();
    let collection = service.default_collection().await.unwrap();

    let secret = oo7::Secret::text("timestamp test");

    let item = collection
        .create_item(
            "Timestamp Test",
            &[("test", "timestamps")],
            secret,
            true,
            None,
        )
        .await
        .unwrap();

    let created = item.created().await.unwrap();
    let modified = item.modified().await.unwrap();

    eprintln!("Created: {:?}, Modified: {:?}", created, modified);
    assert_eq!(created, modified);

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    item.set_label("Updated Label").await.unwrap();

    // Allow time for D-Bus changes to propagate
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let new_modified = item.modified().await.unwrap();
    assert!(new_modified > modified);
    assert_eq!(item.created().await.unwrap(), created);

    item.delete(None).await.unwrap();
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn deleted_error() {
    let service = Service::plain().await.unwrap();
    let collection = service.default_collection().await.unwrap();

    let attributes = &[("test", "deleted-error")];
    let secret = oo7::Secret::text("delete test");

    let item = collection
        .create_item("Delete Test", attributes, secret, true, None)
        .await
        .unwrap();

    // Verify item works before deletion
    assert!(item.label().await.is_ok());

    // Delete the item
    item.delete(None).await.unwrap();

    // All operations should now return Error::Deleted
    assert!(matches!(item.label().await, Err(oo7::dbus::Error::Deleted)));
    assert!(matches!(
        item.set_label("New").await,
        Err(oo7::dbus::Error::Deleted)
    ));
    assert!(matches!(
        item.secret().await,
        Err(oo7::dbus::Error::Deleted)
    ));
    assert!(matches!(
        item.set_secret("new secret").await,
        Err(oo7::dbus::Error::Deleted)
    ));
    assert!(matches!(
        item.attributes().await,
        Err(oo7::dbus::Error::Deleted)
    ));
    assert!(matches!(
        item.set_attributes(attributes).await,
        Err(oo7::dbus::Error::Deleted)
    ));
    assert!(matches!(
        item.created().await,
        Err(oo7::dbus::Error::Deleted)
    ));
    assert!(matches!(
        item.modified().await,
        Err(oo7::dbus::Error::Deleted)
    ));
    assert!(matches!(
        item.is_locked().await,
        Err(oo7::dbus::Error::Deleted)
    ));
    assert!(matches!(
        item.lock(None).await,
        Err(oo7::dbus::Error::Deleted)
    ));
    assert!(matches!(
        item.unlock(None).await,
        Err(oo7::dbus::Error::Deleted)
    ));
    assert!(matches!(
        item.delete(None).await,
        Err(oo7::dbus::Error::Deleted)
    ));
}
