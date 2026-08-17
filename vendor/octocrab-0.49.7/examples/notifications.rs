use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let notifications = octocrab
        .activity()
        .notifications()
        .list()
        .all(true)
        .send()
        .await?;
    for n in notifications {
        println!("unread notification: {}", n.subject.title);
    }

    Ok(())
}
