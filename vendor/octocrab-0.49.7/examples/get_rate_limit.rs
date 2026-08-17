use octocrab::Octocrab;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ratelimit = octocrab::instance().ratelimit().get().await?;
    println!("{}", serde_json::to_string_pretty(&ratelimit)?);

    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let octocrab = Octocrab::builder().personal_token(token).build()?;
    let ratelimit = octocrab.ratelimit().get().await?;
    println!("{}", serde_json::to_string_pretty(&ratelimit)?);
    Ok(())
}
