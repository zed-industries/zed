use crate::db::{self, ChannelRole, NewUserParams};

use anyhow::Context;
use chrono::{DateTime, Utc};
use db::Database;
use serde::{de::DeserializeOwned, Deserialize};
use std::{fmt::Write, fs, path::Path};

use crate::Config;

#[derive(Debug, Deserialize)]
struct GithubUser {
    id: i32,
    login: String,
    email: Option<String>,
    created_at: DateTime<Utc>,
}

/// A GitHub user returned from the [List users](https://docs.github.com/en/rest/users/users?apiVersion=2022-11-28#list-users) endpoint.
///
/// Notably, this data type does not have the `created_at` field.
#[derive(Debug, Deserialize)]
struct ListGithubUser {
    id: i32,
    login: String,
    email: Option<String>,
}

#[derive(Deserialize)]
struct SeedConfig {
    /// Which users to create as admins.
    admins: Vec<String>,
    /// Which channels to create (all admins are invited to all channels).
    channels: Vec<String>,
    /// Number of random users to create from the Github API.
    number_of_users: Option<usize>,
}

pub async fn seed(config: &Config, db: &Database, force: bool) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    if !db.get_all_users(0, 1).await?.is_empty() && !force {
        return Ok(());
    }

    let seed_path = config
        .seed_path
        .as_ref()
        .context("called seed with no SEED_PATH")?;

    let seed_config = load_admins(seed_path)
        .context(format!("failed to load {}", seed_path.to_string_lossy()))?;

    let mut first_user = None;
    let mut others = vec![];

    let flag_names = ["remoting", "language-models"];
    let mut flags = Vec::new();

    let existing_feature_flags = db.list_feature_flags().await?;

    for flag_name in flag_names {
        if existing_feature_flags
            .iter()
            .any(|flag| flag.flag == flag_name)
        {
            log::info!("Flag {flag_name:?} already exists");
            continue;
        }

        let flag = db
            .create_user_flag(flag_name, false)
            .await
            .unwrap_or_else(|err| panic!("failed to create flag: '{flag_name}': {err}"));
        flags.push(flag);
    }

    for admin_login in seed_config.admins {
        let user = fetch_github::<GithubUser>(
            &client,
            &format!("https://api.github.com/users/{admin_login}"),
        )
        .await;
        let user = db
            .create_user(
                &user.email.unwrap_or(format!("{admin_login}@example.com")),
                true,
                NewUserParams {
                    github_login: user.login,
                    github_user_id: user.id,
                },
            )
            .await
            .context("failed to create admin user")?;
        if first_user.is_none() {
            first_user = Some(user.user_id);
        } else {
            others.push(user.user_id)
        }

        for flag in &flags {
            db.add_user_flag(user.user_id, *flag)
                .await
                .context(format!(
                    "Unable to enable flag '{}' for user '{}'",
                    flag, user.user_id
                ))?;
        }
    }

    for channel in seed_config.channels {
        let (channel, _) = db
            .create_channel(&channel, None, first_user.unwrap())
            .await
            .context("failed to create channel")?;

        for user_id in &others {
            db.invite_channel_member(
                channel.id,
                *user_id,
                first_user.unwrap(),
                ChannelRole::Admin,
            )
            .await
            .context("failed to add user to channel")?;
        }
    }

    // TODO: Fix this later
    if let Some(number_of_users) = seed_config.number_of_users {
        // Fetch 100 other random users from GitHub and insert them into the database
        // (for testing autocompleters, etc.)
        let mut user_count = db
            .get_all_users(0, 200)
            .await
            .expect("failed to load users from db")
            .len();
        let mut last_user_id = None;
        while user_count < number_of_users {
            let mut uri = "https://api.github.com/users?per_page=100".to_string();
            if let Some(last_user_id) = last_user_id {
                write!(&mut uri, "&since={}", last_user_id).unwrap();
            }
            let users = fetch_github::<Vec<ListGithubUser>>(&client, &uri).await;

            for github_user in users {
                log::info!("Seeding {:?} from GitHub", github_user.login);

                // Fetch the user to get their `created_at` timestamp, since it
                // isn't on the list response.
                let github_user: GithubUser = fetch_github(
                    &client,
                    &format!("https://api.github.com/user/{}", github_user.id),
                )
                .await;

                last_user_id = Some(github_user.id);
                user_count += 1;
                let user = db
                    .get_or_create_user_by_github_account(
                        &github_user.login,
                        github_user.id,
                        github_user.email.as_deref(),
                        github_user.created_at,
                        None,
                    )
                    .await
                    .expect("failed to insert user");

                for flag in &flags {
                    db.add_user_flag(user.id, *flag).await.context(format!(
                        "Unable to enable flag '{}' for user '{}'",
                        flag, user.id
                    ))?;
                }

                // Sleep to avoid getting rate-limited by GitHub.
                tokio::time::sleep(std::time::Duration::from_millis(250)).await;
            }
        }
    }

    Ok(())
}

fn load_admins(path: impl AsRef<Path>) -> anyhow::Result<SeedConfig> {
    let file_content = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&file_content)?)
}

async fn fetch_github<T: DeserializeOwned>(client: &reqwest::Client, url: &str) -> T {
    let response = client
        .get(url)
        .header("user-agent", "zed")
        .send()
        .await
        .unwrap_or_else(|error| panic!("failed to fetch '{url}': {error}"));
    response
        .json()
        .await
        .unwrap_or_else(|error| panic!("failed to deserialize github user from '{url}': {error}"))
}
