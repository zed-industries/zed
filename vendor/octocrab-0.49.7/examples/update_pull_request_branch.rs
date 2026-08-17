#!/usr/bin/env rust-script

//! Dependencies can be specified in the script file itself as follows:
//!
//! ```cargo
//! [dependencies]
//! tokio = { version = "1.6.1", default-features = false, features = ["macros", "rt-multi-thread", "time"] }
//! octocrab = { path = "../" }
//! ```

use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let update = octocrab
        .pulls("XAMPPRocky", "octocrab")
        .update_branch(200)
        .await?;

    println!("Result of pull request update: {update}",);

    Ok(())
}
