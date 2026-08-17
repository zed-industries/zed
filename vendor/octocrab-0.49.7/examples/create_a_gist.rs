use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    println!("Creating a gist with hello world in rust on your account");
    let gist = octocrab
        .gists()
        .create()
        .file(
            "hello_world.rs",
            "fn main() {\n println!(\"Hello World!\");\n}",
        )
        // Optional Parameters
        .description("Hello World in Rust")
        .public(false)
        .send()
        .await?;
    println!("Done, created: {url}", url = gist.html_url);
    Ok(())
}
