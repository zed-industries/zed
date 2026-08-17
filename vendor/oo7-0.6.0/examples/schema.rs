use oo7::{Secret, SecretSchema, file::UnlockedKeyring};

#[derive(SecretSchema, Debug, Default)]
#[schema(name = "org.example.Password")]
struct PasswordSchema {
    username: String,
    server: String,
    port: Option<u16>,
    protocol: Option<String>,
}

#[tokio::main]
async fn main() -> oo7::Result<()> {
    let temp_dir = tempfile::tempdir().unwrap();
    let keyring_path = temp_dir.path().join("test.keyring");
    let keyring = UnlockedKeyring::load(&keyring_path, Secret::text("test_password")).await?;

    println!("=== Creating items ===");

    let schema_full = PasswordSchema {
        username: "alice".to_string(),
        server: "example.com".to_string(),
        port: Some(8080),
        protocol: Some("https".to_string()),
    };

    keyring
        .create_item("Alice's Password", &schema_full, "secret123", true)
        .await?;
    println!("Created: {:?}", schema_full);

    let schema_minimal = PasswordSchema {
        username: "bob".to_string(),
        server: "test.org".to_string(),
        ..Default::default()
    };

    keyring
        .create_item("Bob's Password", &schema_minimal, "secret456", true)
        .await?;
    println!("Created: {:?}", schema_minimal);

    println!("\n=== Searching ===");

    let search_schema = PasswordSchema {
        username: "alice".to_string(),
        server: "example.com".to_string(),
        ..Default::default()
    };

    let items = keyring.search_items(&search_schema).await?;
    println!("Found {} item(s)", items.len());

    for item in &items {
        let unlocked = item.as_unlocked();
        println!("  Label: {}", unlocked.label());
        println!("  Secret: {:?}", unlocked.secret());
    }

    println!("\n=== Typed attributes ===");

    if let Some(item) = items.first() {
        let schema = item.as_unlocked().attributes_as::<PasswordSchema>()?;
        println!("Username: {}", schema.username);
        println!("Server: {}", schema.server);
        println!("Port: {:?}", schema.port);
        println!("Protocol: {:?}", schema.protocol);
    }

    keyring.delete(&[("username", "alice")]).await?;
    keyring.delete(&[("username", "bob")]).await?;

    Ok(())
}
