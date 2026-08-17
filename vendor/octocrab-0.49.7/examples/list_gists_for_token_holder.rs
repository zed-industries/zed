use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder().personal_token(token).build()?;
    let current_user_name = octocrab.current().user().await?.login;
    let mut current_gist_page = octocrab
        .current()
        .list_gists_for_authenticated_user()
        .per_page(1)
        .send()
        .await?;

    let mut gists = current_gist_page.take_items();
    while let Ok(Some(mut new_page)) = octocrab.get_page(&current_gist_page.next).await {
        gists.extend(new_page.take_items());
        current_gist_page = new_page;
    }

    println!(
        "User '{username}' has {count} gists:",
        username = current_user_name,
        count = gists.len()
    );
    println!("id | url | [files...] | description");
    for gist in gists {
        println!(
            "{id} | {url} | [{files}] | {description}",
            id = gist.id,
            url = gist.html_url,
            files = gist.files.into_keys().collect::<Vec<_>>().join(", "),
            description = gist
                .description
                .unwrap_or("<No description>".into())
                .escape_default(),
        );
    }

    Ok(())
}
