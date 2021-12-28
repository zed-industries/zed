use db::{Db, UserId};
use rand::prelude::*;
use time::{Duration, OffsetDateTime};

#[allow(unused)]
#[path = "../db.rs"]
mod db;

#[async_std::main]
async fn main() {
    let mut rng = StdRng::from_entropy();
    let database_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL env var");
    let db = Db::new(&database_url, 5)
        .await
        .expect("failed to connect to postgres database");

    let zed_users = ["nathansobo", "maxbrunsfeld", "as-cii", "iamnbutler"];
    let mut zed_user_ids = Vec::<UserId>::new();
    for zed_user in zed_users {
        if let Some(user) = db
            .get_user_by_github_login(zed_user)
            .await
            .expect("failed to fetch user")
        {
            zed_user_ids.push(user.id);
        } else {
            zed_user_ids.push(
                db.create_user(zed_user, true)
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
