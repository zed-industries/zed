use oo7::dbus::Service;

#[tokio::test]
#[cfg(feature = "tokio")]
#[ignore = "Requires prompting"]
async fn create_collection() {
    let service = Service::new().await.unwrap();
    let collection = service
        .create_collection("somelabel", None, None)
        .await
        .unwrap();

    let found_collection = service.with_label("somelabel").await.unwrap();
    assert!(found_collection.is_some());

    assert_eq!(
        found_collection.unwrap().label().await.unwrap(),
        collection.label().await.unwrap()
    );

    collection.delete(None).await.unwrap();

    let found_collection = service.with_label("somelabel").await.unwrap();
    assert!(found_collection.is_none());
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn default_collections() {
    let service = Service::new().await.unwrap();

    assert!(service.default_collection().await.is_ok());
    assert!(service.session_collection().await.is_ok());
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn encrypted_session() {
    let service = Service::encrypted().await.unwrap();
    assert!(service.default_collection().await.is_ok());
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn plain_session() {
    let service = Service::plain().await.unwrap();
    assert!(service.default_collection().await.is_ok());
}
