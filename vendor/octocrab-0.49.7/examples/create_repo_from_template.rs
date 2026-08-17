use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder()
        .personal_token(token.to_string())
        .build()?;

    let repository = octocrab.repos("rust-lang", "rust-template");
    repository
        .generate("new-repository")
        .owner("new-owner")
        .description("New description")
        .private(true)
        .include_all_branches(true)
        .send()
        .await?;

    Ok(())
}
