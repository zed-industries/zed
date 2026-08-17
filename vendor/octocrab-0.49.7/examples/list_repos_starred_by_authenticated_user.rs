use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let repos = octocrab
        .current()
        .list_repos_starred_by_authenticated_user()
        .sort("updated")
        .per_page(100)
        .send()
        .await?;

    for repo in repos {
        println!("{}", repo.name);
    }

    Ok(())
}
