#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let octocrab = octocrab::Octocrab::builder()
        .personal_token(std::env::var("GITHUB_TOKEN").unwrap())
        .build()?;

    match octocrab
        .search()
        .issues_and_pull_requests("tokei is:pr")
        .send()
        .await
    {
        Ok(page) => println!("{page:#?}"),
        Err(error) => println!("{error:#?}"),
    }

    Ok(())
}
