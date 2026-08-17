#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let api = octocrab::instance();
    let issues_api = api.issues("rust-lang", "rust");
    let one_issue = issues_api.list().per_page(1).send().await?.take_items();
    issues_api.unlock(one_issue.first().unwrap().number).await?;

    Ok(())
}
