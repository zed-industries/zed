use collab::{
    db::{self, NewUserParams},
    env::load_dotenv,
    executor::Executor,
};
use db::{ConnectOptions, Database};
use serde::{de::DeserializeOwned, Deserialize};
use std::{fmt::Write, fs};

#[derive(Debug, Deserialize)]
struct GitHubUser {
    id: i32,
    login: String,
    email: Option<String>,
}

#[tokio::main]
async fn main() {
    load_dotenv().expect("failed to load .env.toml file");

    let mut admin_logins = load_admins("crates/collab/.admins.default.json")
        .expect("failed to load default admins file");
    if let Ok(other_admins) = load_admins("./.admins.json") {
        admin_logins.extend(other_admins);
    }

    let database_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL env var");
    let db = Database::new(ConnectOptions::new(database_url), Executor::Production)
        .await
        .expect("failed to connect to postgres database");
    let client = reqwest::Client::new();

    // Create admin users for all of the users in `.admins.toml` or `.admins.default.toml`.
    for admin_login in admin_logins {
        let user = fetch_github::<GitHubUser>(
            &client,
            &format!("https://api.github.com/users/{admin_login}"),
        )
        .await;
        db.create_user(
            &user.email.unwrap_or(format!("{admin_login}@example.com")),
            true,
            NewUserParams {
                github_login: user.login,
                github_user_id: user.id,
            },
        )
        .await
        .expect("failed to create admin user");
    }

    // Fetch 100 other random users from GitHub and insert them into the database.
    let mut user_count = db
        .get_all_users(0, 200)
        .await
        .expect("failed to load users from db")
        .len();
    let mut last_user_id = None;
    while user_count < 100 {
        let mut uri = "https://api.github.com/users?per_page=100".to_string();
        if let Some(last_user_id) = last_user_id {
            write!(&mut uri, "&since={}", last_user_id).unwrap();
        }
        let users = fetch_github::<Vec<GitHubUser>>(&client, &uri).await;

        for github_user in users {
            last_user_id = Some(github_user.id);
            user_count += 1;
            db.get_or_create_user_by_github_account(
                &github_user.login,
                Some(github_user.id),
                github_user.email.as_deref(),
            )
            .await
            .expect("failed to insert user");
        }
    }
}

fn load_admins(path: &str) -> anyhow::Result<Vec<String>> {
    let file_content = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&file_content)?)
}

async fn fetch_github<T: DeserializeOwned>(client: &reqwest::Client, url: &str) -> T {
    let response = client
        .get(url)
        .header("user-agent", "zed")
        .send()
        .await
        .unwrap_or_else(|_| panic!("failed to fetch '{}'", url));
    response
        .json()
        .await
        .unwrap_or_else(|_| panic!("failed to deserialize github user from '{}'", url))
}
