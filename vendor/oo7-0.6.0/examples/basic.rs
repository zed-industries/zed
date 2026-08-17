#[tokio::main]
async fn main() -> oo7::Result<()> {
    let keyring = oo7::Keyring::new().await?;
    let attributes = &[("attr", "value")];
    keyring
        .create_item("Some Label", attributes, "secret", true)
        .await?;

    let items = keyring.search_items(attributes).await?;

    for item in items {
        println!("{}", item.label().await?);
        println!("{:#?}", item.attributes().await?);
        println!("{:#?}", item.secret().await?);
    }

    Ok(())
}
