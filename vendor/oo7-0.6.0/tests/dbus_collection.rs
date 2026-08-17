use oo7::dbus::Service;

async fn create_item(service: Service, encrypted: bool) {
    let attributes = if encrypted {
        &[("type", "encrypted-type-test")]
    } else {
        &[("type", "plain-type-test")]
    };
    let secret = oo7::Secret::text("a password");

    let collection = service.default_collection().await.unwrap();
    let n_search_items = collection.search_items(&attributes).await.unwrap().len();

    let item = collection
        .create_item("A secret", &attributes, secret.clone(), true, None)
        .await
        .unwrap();

    assert_eq!(item.secret().await.unwrap(), secret);
    assert_eq!(
        item.attributes().await.unwrap().get("type").unwrap(),
        attributes[0].1,
    );

    assert_eq!(
        collection.search_items(&attributes).await.unwrap().len(),
        n_search_items + 1
    );

    item.delete(None).await.unwrap();

    assert_eq!(
        collection.search_items(&attributes).await.unwrap().len(),
        n_search_items
    );
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn create_plain_item() {
    let service = Service::plain().await.unwrap();
    create_item(service, false).await;
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn create_encrypted_item() {
    let service = Service::encrypted().await.unwrap();
    create_item(service, true).await;
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn attribute_search_patterns() {
    let service = Service::plain().await.unwrap();
    let collection = service.default_collection().await.unwrap();

    let secret = oo7::Secret::text("search test");

    // Create items with unique test attributes
    let item1 = collection
        .create_item(
            "Pattern Test 1",
            &[("test-pattern", "pattern-test-a"), ("category", "group1")],
            secret.clone(),
            true,
            None,
        )
        .await
        .unwrap();

    let item2 = collection
        .create_item(
            "Pattern Test 2",
            &[("test-pattern", "pattern-test-a"), ("category", "group2")],
            secret.clone(),
            true,
            None,
        )
        .await
        .unwrap();

    let item3 = collection
        .create_item(
            "Pattern Test 3",
            &[("test-pattern", "pattern-test-b"), ("category", "group1")],
            secret.clone(),
            true,
            None,
        )
        .await
        .unwrap();

    // Search by test-pattern - should find items with pattern-test-a
    let pattern_a_items = collection
        .search_items(&[("test-pattern", "pattern-test-a")])
        .await
        .unwrap();
    let found_paths: std::collections::HashSet<_> =
        pattern_a_items.iter().map(|item| item.path()).collect();
    assert!(found_paths.contains(item1.path()));
    assert!(found_paths.contains(item2.path()));

    // Search by category - should find items in group1
    let group1_items = collection
        .search_items(&[("category", "group1")])
        .await
        .unwrap();
    let found_group1_paths: std::collections::HashSet<_> =
        group1_items.iter().map(|item| item.path()).collect();
    assert!(found_group1_paths.contains(item1.path()));
    assert!(found_group1_paths.contains(item3.path()));

    // Search by both attributes - should find only item1
    let specific_items = collection
        .search_items(&[("test-pattern", "pattern-test-a"), ("category", "group1")])
        .await
        .unwrap();
    let found_specific_paths: std::collections::HashSet<_> =
        specific_items.iter().map(|item| item.path()).collect();
    assert!(found_specific_paths.contains(item1.path()));

    item1.delete(None).await.unwrap();
    item2.delete(None).await.unwrap();
    item3.delete(None).await.unwrap();
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn items() {
    let service = Service::plain().await.unwrap();
    let collection = service.default_collection().await.unwrap();

    let secret = oo7::Secret::text("items test");

    // Create some test items with unique attributes
    let item1 = collection
        .create_item(
            "Test Item 1",
            &[("test", "items-test-1"), ("unique", "test-1")],
            secret.clone(),
            true,
            None,
        )
        .await
        .unwrap();

    let item2 = collection
        .create_item(
            "Test Item 2",
            &[("test", "items-test-2"), ("unique", "test-2")],
            secret.clone(),
            true,
            None,
        )
        .await
        .unwrap();

    // Get all items and verify our items are included by path
    let all_items = collection.items().await.unwrap();
    let item_paths: std::collections::HashSet<_> =
        all_items.iter().map(|item| item.path()).collect();

    assert!(item_paths.contains(item1.path()));
    assert!(item_paths.contains(item2.path()));

    // Clean up
    item1.delete(None).await.unwrap();
    item2.delete(None).await.unwrap();
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn label_mutation() {
    let service = Service::plain().await.unwrap();
    let collection = service.session_collection().await.unwrap();

    let initial_label = collection.label().await.unwrap();

    collection.set_label("Updated Label").await.unwrap();
    assert_eq!(collection.label().await.unwrap(), "Updated Label");
    assert_ne!(collection.label().await.unwrap(), initial_label);

    // Restore original label
    collection.set_label(&initial_label).await.unwrap();
    assert_eq!(collection.label().await.unwrap(), initial_label);
}
