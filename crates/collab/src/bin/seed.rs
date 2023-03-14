use collab::db;
use db::{ConnectOptions, Database};
use serde::{de::DeserializeOwned, Deserialize};
use std::fmt::Write;

#[derive(Debug, Deserialize)]
struct GitHubUser {
    id: i32,
    login: String,
    email: Option<String>,
}

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL env var");
    let db = Database::new(ConnectOptions::new(database_url))
        .await
        .expect("failed to connect to postgres database");
    let github_token = std::env::var("GITHUB_TOKEN").expect("missing GITHUB_TOKEN env var");
    let client = reqwest::Client::new();

    let mut current_user =
        fetch_github::<GitHubUser>(&client, &github_token, "https://api.github.com/user").await;
    current_user
        .email
        .get_or_insert_with(|| "placeholder@example.com".to_string());
    let staff_users = fetch_github::<Vec<GitHubUser>>(
        &client,
        &github_token,
        "https://api.github.com/orgs/zed-industries/teams/staff/members",
    )
    .await;

    let mut zed_users = Vec::new();
    zed_users.push((current_user, true));
    zed_users.extend(staff_users.into_iter().map(|user| (user, true)));

    let user_count = db
        .get_all_users(0, 200)
        .await
        .expect("failed to load users from db")
        .len();
    if user_count < 100 {
        let mut last_user_id = None;
        for _ in 0..10 {
            let mut uri = "https://api.github.com/users?per_page=100".to_string();
            if let Some(last_user_id) = last_user_id {
                write!(&mut uri, "&since={}", last_user_id).unwrap();
            }
            let users = fetch_github::<Vec<GitHubUser>>(&client, &github_token, &uri).await;
            if let Some(last_user) = users.last() {
                last_user_id = Some(last_user.id);
                zed_users.extend(users.into_iter().map(|user| (user, false)));
            } else {
                break;
            }
        }
    }

    for (github_user, admin) in zed_users {
        if db
            .get_user_by_github_login(&github_user.login)
            .await
            .expect("failed to fetch user")
            .is_none()
        {
            if let Some(email) = &github_user.email {
                db.create_user(
                    email,
                    admin,
                    db::NewUserParams {
                        github_login: github_user.login,
                        github_user_id: github_user.id,
                        invite_count: 5,
                    },
                )
                .await
                .expect("failed to insert user");
            } else if admin {
                db.create_user(
                    &format!("{}@zed.dev", github_user.login),
                    admin,
                    db::NewUserParams {
                        github_login: github_user.login,
                        github_user_id: github_user.id,
                        invite_count: 5,
                    },
                )
                .await
                .expect("failed to insert user");
            }
        }
    }
}

async fn fetch_github<T: DeserializeOwned>(
    client: &reqwest::Client,
    access_token: &str,
    url: &str,
) -> T {
    let response = client
        .get(url)
        .bearer_auth(&access_token)
        .header("user-agent", "zed")
        .send()
        .await
        .expect(&format!("failed to fetch '{}'", url));
    response
        .json()
        .await
        .expect(&format!("failed to deserialize github user from '{}'", url))
}
