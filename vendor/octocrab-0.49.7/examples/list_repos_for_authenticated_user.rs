use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let my_repos = octocrab
        .current()
        .list_repos_for_authenticated_user()
        .type_("owner")
        .sort("updated")
        .per_page(100)
        .send()
        .await?;

    for repo in my_repos {
        println!("{}", repo.name);
    }

    Ok(())
}
