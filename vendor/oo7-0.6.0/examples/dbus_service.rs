#[tokio::main]
async fn main() -> oo7::Result<()> {
    let service = oo7::dbus::Service::new().await?;

    let collection = service.default_collection().await?;
    let items = collection.search_items(&[("type", "token")]).await?;
    for item in items {
        println!("{}", item.label().await?);
        println!("{}", item.is_locked().await?);
        println!("{:#?}", item.created().await?);
        println!("{:#?}", item.modified().await?);
        println!("{:#?}", item.attributes().await?);
        println!("{:#?}", item.secret().await?);
    }
    Ok(())
}
