use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let octocrab = Octocrab::builder().build()?;
    let runs = octocrab
        .workflows("rust-lang-ci", "rust")
        .list_all_runs()
        .per_page(2)
        .branch("master")
        .event("push")
        .status("success")
        .send()
        .await?;

    for run in runs {
        println!("Run:");
        println!("  ID: {}", run.id);
        println!("  Name: {}", run.name);
        println!("  Event: {}", run.event);
        println!("  Branch: {}", run.head_branch);
        println!("  Created At: {}", run.created_at);
        println!("  Commit:");
        println!("    Author: {}", run.head_commit.author.name);
        println!("    Message: {}", run.head_commit.message);
        println!()
    }

    Ok(())
}
