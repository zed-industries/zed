use collab::{Error, Result};
use db::{Db, PostgresDb, UserId};
use rand::prelude::*;
use serde::{de::DeserializeOwned, Deserialize};
use std::fmt::Write;
use time::{Duration, OffsetDateTime};

#[allow(unused)]
#[path = "../db.rs"]
mod db;

#[derive(Debug, Deserialize)]
struct GitHubUser {
    id: usize,
    login: String,
    email: Option<String>,
}

#[tokio::main]
async fn main() {
    let mut rng = StdRng::from_entropy();
    let database_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL env var");
    let db = PostgresDb::new(&database_url, 5)
        .await
        .expect("failed to connect to postgres database");
    let github_token = std::env::var("GITHUB_TOKEN").expect("missing GITHUB_TOKEN env var");
    let client = reqwest::Client::new();

    let current_user =
        fetch_github::<GitHubUser>(&client, &github_token, "https://api.github.com/user").await;
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

    let mut zed_user_ids = Vec::<UserId>::new();
    for (github_user, admin) in zed_users {
        if let Some(user) = db
            .get_user_by_github_login(&github_user.login)
            .await
            .expect("failed to fetch user")
        {
            zed_user_ids.push(user.id);
        } else {
            zed_user_ids.push(
                db.create_user(&github_user.login, github_user.email.as_deref(), admin)
                    .await
                    .expect("failed to insert user"),
            );
        }
    }

    let zed_org_id = if let Some(org) = db
        .find_org_by_slug("zed")
        .await
        .expect("failed to fetch org")
    {
        org.id
    } else {
        db.create_org("Zed", "zed")
            .await
            .expect("failed to insert org")
    };

    let general_channel_id = if let Some(channel) = db
        .get_org_channels(zed_org_id)
        .await
        .expect("failed to fetch channels")
        .iter()
        .find(|c| c.name == "General")
    {
        channel.id
    } else {
        let channel_id = db
            .create_org_channel(zed_org_id, "General")
            .await
            .expect("failed to insert channel");

        let now = OffsetDateTime::now_utc();
        let max_seconds = Duration::days(100).as_seconds_f64();
        let mut timestamps = (0..1000)
            .map(|_| now - Duration::seconds_f64(rng.gen_range(0_f64..=max_seconds)))
            .collect::<Vec<_>>();
        timestamps.sort();
        for timestamp in timestamps {
            let sender_id = *zed_user_ids.choose(&mut rng).unwrap();
            let body = lipsum::lipsum_words(rng.gen_range(1..=50));
            db.create_channel_message(channel_id, sender_id, &body, timestamp, rng.gen())
                .await
                .expect("failed to insert message");
        }
        channel_id
    };

    for user_id in zed_user_ids {
        db.add_org_member(zed_org_id, user_id, true)
            .await
            .expect("failed to insert org membership");
        db.add_channel_member(general_channel_id, user_id, true)
            .await
            .expect("failed to insert channel membership");
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
