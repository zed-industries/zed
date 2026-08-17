#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let diff = octocrab::instance()
        .pulls("rust-lang", "rust")
        .get_diff(72033)
        .await?;

    println!("{diff}");

    Ok(())
}
