#![cfg(all(feature = "schema", feature = "tokio", feature = "native_crypto"))]

use oo7::{ContentType, Secret, SecretSchema, file::UnlockedKeyring};

#[derive(SecretSchema, Debug, Default)]
#[schema(name = "org.example.Test")]
struct TestSchema {
    username: String,
    port: Option<u16>,
}

#[tokio::test]
async fn schema_content_type_and_attributes() {
    let keyring = UnlockedKeyring::temporary(Secret::random().unwrap())
        .await
        .unwrap();

    keyring
        .create_item(
            "Text Item",
            &TestSchema {
                username: "alice".to_string(),
                port: Some(8080),
            },
            Secret::text("my-password"),
            true,
        )
        .await
        .unwrap();

    keyring
        .create_item(
            "Blob Item",
            &TestSchema {
                username: "bob".to_string(),
                port: None,
            },
            Secret::blob(b"binary data"),
            true,
        )
        .await
        .unwrap();

    let text_items = keyring
        .search_items(&TestSchema {
            username: "alice".to_string(),
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(text_items.len(), 1);
    let text_item = text_items[0].as_unlocked();

    assert_eq!(
        text_item.attributes().get("xdg:content-type").unwrap(),
        "text/plain"
    );
    assert_eq!(text_item.secret().content_type(), ContentType::Text);

    let schema = text_item.attributes_as::<TestSchema>().unwrap();
    assert_eq!(schema.username, "alice");
    assert_eq!(schema.port, Some(8080));

    let mut unlocked_item = text_items[0].as_unlocked().clone();
    unlocked_item.set_attributes(&TestSchema {
        username: "alice".to_string(),
        port: Some(9090),
    });

    assert_eq!(
        unlocked_item.attributes().get("xdg:content-type").unwrap(),
        "text/plain"
    );
    assert_eq!(unlocked_item.secret().content_type(), ContentType::Text);

    let updated_schema = unlocked_item.attributes_as::<TestSchema>().unwrap();
    assert_eq!(updated_schema.username, "alice");
    assert_eq!(updated_schema.port, Some(9090));

    let blob_items = keyring
        .search_items(&TestSchema {
            username: "bob".to_string(),
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(blob_items.len(), 1);
    let blob_item = blob_items[0].as_unlocked();

    assert_eq!(
        blob_item.attributes().get("xdg:content-type").unwrap(),
        "application/octet-stream"
    );
    assert_eq!(blob_item.secret().content_type(), ContentType::Blob);

    let schema = blob_item.attributes_as::<TestSchema>().unwrap();
    assert_eq!(schema.username, "bob");
    assert_eq!(schema.port, None);
}

#[derive(SecretSchema, Debug, Default)]
#[schema(name = "org.example.ErrorTest")]
struct ErrorSchema {
    required_field: String,
    optional_number: Option<u16>,
}

#[tokio::test]
async fn schema_error_handling() {
    use std::collections::HashMap;

    let mut attrs = HashMap::new();
    attrs.insert(
        "xdg:schema".to_string(),
        "org.example.ErrorTest".to_string(),
    );
    attrs.insert("required_field".to_string(), "value".to_string());
    attrs.insert("optional_number".to_string(), "42".to_string());

    let valid: Result<ErrorSchema, _> = attrs.clone().try_into();
    assert!(valid.is_ok());
    let schema = valid.unwrap();
    assert_eq!(schema.required_field, "value");
    assert_eq!(schema.optional_number, Some(42));

    let mut missing_field = attrs.clone();
    missing_field.remove("required_field");
    let result: Result<ErrorSchema, _> = missing_field.try_into();
    assert!(result.is_err());

    let mut wrong_schema = attrs.clone();
    wrong_schema.insert(
        "xdg:schema".to_string(),
        "org.example.WrongSchema".to_string(),
    );
    let result: Result<ErrorSchema, _> = wrong_schema.try_into();
    assert!(result.is_err());

    let mut invalid_value = attrs.clone();
    invalid_value.insert("optional_number".to_string(), "not_a_number".to_string());
    let result: Result<ErrorSchema, _> = invalid_value.try_into();
    assert!(result.is_err());
}
