#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let markdown = "**Markdown**";
    print!(
        "{}",
        octocrab::instance().markdown().render_raw(markdown).await?
    );
    Ok(())
}
