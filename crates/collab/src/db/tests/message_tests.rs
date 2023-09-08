use crate::{
    db::{Database, NewUserParams},
    test_both_dbs,
};
use std::sync::Arc;
use time::OffsetDateTime;

test_both_dbs!(
    test_channel_message_nonces,
    test_channel_message_nonces_postgres,
    test_channel_message_nonces_sqlite
);

async fn test_channel_message_nonces(db: &Arc<Database>) {
    let user = db
        .create_user(
            "user@example.com",
            false,
            NewUserParams {
                github_login: "user".into(),
                github_user_id: 1,
                invite_count: 0,
            },
        )
        .await
        .unwrap()
        .user_id;
    let channel = db
        .create_channel("channel", None, "room", user)
        .await
        .unwrap();

    let owner_id = db.create_server("test").await.unwrap().0 as u32;

    db.join_channel_chat(channel, rpc::ConnectionId { owner_id, id: 0 }, user)
        .await
        .unwrap();

    let msg1_id = db
        .create_channel_message(channel, user, "1", OffsetDateTime::now_utc(), 1)
        .await
        .unwrap();
    let msg2_id = db
        .create_channel_message(channel, user, "2", OffsetDateTime::now_utc(), 2)
        .await
        .unwrap();
    let msg3_id = db
        .create_channel_message(channel, user, "3", OffsetDateTime::now_utc(), 1)
        .await
        .unwrap();
    let msg4_id = db
        .create_channel_message(channel, user, "4", OffsetDateTime::now_utc(), 2)
        .await
        .unwrap();

    assert_ne!(msg1_id, msg2_id);
    assert_eq!(msg1_id, msg3_id);
    assert_eq!(msg2_id, msg4_id);
}
