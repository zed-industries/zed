use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    // Note: this token must have the `admin:org` scope. An alternative use case
    // may be to authenticate as a GitHub App. See github_app_authentication.rs
    // for that.
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let runners = octocrab
        .actions()
        .list_org_self_hosted_runners("my-org")
        .per_page(100)
        .send()
        .await?;

    for runner in runners {
        println!("ID {}:", runner.id);
        println!("    Name:\t{}", runner.name);
        println!("    OS:\t\t{}", runner.os);
        println!("    Status:\t{}", runner.status);
        println!("    Busy:\t{}", runner.busy);
        print!("    Labels:\t[");
        for (index, label) in runner.labels.iter().enumerate() {
            print!("\"{}\"", label.name);
            if index != runner.labels.len() - 1 {
                print!(", ");
            }
        }
        println!("]");
    }

    Ok(())
}
