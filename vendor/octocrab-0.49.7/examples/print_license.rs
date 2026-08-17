#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let license = octocrab::instance()
        .repos("rust-lang", "rust")
        .license()
        .await?;

    println!("{license:#?}");

    Ok(())
}
