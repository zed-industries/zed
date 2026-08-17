use octocrab::{params, Octocrab};

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let octocrab = Octocrab::default();

    let mut current_page = octocrab
        .orgs("rust-lang")
        .list_repos()
        .repo_type(params::repos::Type::Sources)
        .per_page(100)
        .send()
        .await?;
    let mut prs = current_page.take_items();

    while let Ok(Some(mut new_page)) = octocrab.get_page(&current_page.next).await {
        prs.extend(new_page.take_items());

        for pr in prs.drain(..) {
            println!("{pr:?}");
        }

        current_page = new_page;
    }

    Ok(())
}
