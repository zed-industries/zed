use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder().personal_token(token).build()?;
    octocrab
        .repos("XAMPPRocky", "octocrab")
        .merge("feature/1", "master")
        .commit_message("This is a custom merge-commit message")
        .send()
        .await?;

    Ok(())
}
