use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let repo = octocrab
        .repos("rust-lang", "rust")
        .is_collaborator("Roger-luo")
        .await?;

    if repo {
        println!("Roger-luo is a collaborator of rust-lang/rust");
    } else {
        println!("Roger-luo is not a collaborator of rust-lang/rust");
    }

    Ok(())
}
