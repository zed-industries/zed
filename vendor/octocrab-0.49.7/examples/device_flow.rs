use http::header::ACCEPT;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let client_id = secrecy::SecretString::from(std::env::var("GITHUB_CLIENT_ID").unwrap());
    let crab = octocrab::Octocrab::builder()
        .base_uri("https://github.com")?
        .add_header(ACCEPT, "application/json".to_string())
        .build()?;

    let codes = crab
        .authenticate_as_device(&client_id, ["public_repo", "read:org"])
        .await?;
    println!(
        "Go to {} and enter code {}",
        codes.verification_uri, codes.user_code
    );
    let auth = codes.poll_until_available(&crab, &client_id).await?;

    println!("Authorization succeeded with access to {:?}", auth.scope);
    Ok(())
}
