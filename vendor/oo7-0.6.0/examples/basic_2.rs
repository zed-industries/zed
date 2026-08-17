use std::sync::OnceLock;

use oo7::Keyring;

static KEYRING: OnceLock<Keyring> = OnceLock::new();

#[tokio::main]
async fn main() -> oo7::Result<()> {
    let keyring = Keyring::new().await?;
    KEYRING.set(keyring).unwrap();

    let attributes = [("attr", "value")];

    KEYRING
        .get()
        .unwrap()
        .create_item("Some Label", &attributes, "secret", true)
        .await?;

    let items = KEYRING.get().unwrap().search_items(&attributes).await?;

    for item in items {
        println!("{}", item.label().await?);
        println!("{:#?}", item.attributes().await?);
        println!("{:#?}", item.secret().await?);
    }

    Ok(())
}
