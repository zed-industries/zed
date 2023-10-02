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

test_both_dbs!(
    test_channel_message_new_notification,
    test_channel_message_new_notification_postgres,
    test_channel_message_new_notification_sqlite
);

async fn test_channel_message_new_notification(db: &Arc<Database>) {
    let user_a = db
        .create_user(
            "user_a@example.com",
            false,
            NewUserParams {
                github_login: "user_a".into(),
                github_user_id: 1,
                invite_count: 0,
            },
        )
        .await
        .unwrap()
        .user_id;
    let user_b = db
        .create_user(
            "user_b@example.com",
            false,
            NewUserParams {
                github_login: "user_b".into(),
                github_user_id: 1,
                invite_count: 0,
            },
        )
        .await
        .unwrap()
        .user_id;

    let channel = db
        .create_channel("channel", None, "room", user_a)
        .await
        .unwrap();

    db.invite_channel_member(channel, user_b, user_a, false)
        .await
        .unwrap();

    db.respond_to_channel_invite(channel, user_b, true)
        .await
        .unwrap();

    let owner_id = db.create_server("test").await.unwrap().0 as u32;

    // Zero case: no messages at all
    assert!(!db.has_new_message_tx(channel, user_b).await.unwrap());

    let a_connection_id = rpc::ConnectionId { owner_id, id: 0 };
    db.join_channel_chat(channel, a_connection_id, user_a)
        .await
        .unwrap();

    let _ = db
        .create_channel_message(channel, user_a, "1", OffsetDateTime::now_utc(), 1)
        .await
        .unwrap();

    let (second_message, _) = db
        .create_channel_message(channel, user_a, "2", OffsetDateTime::now_utc(), 2)
        .await
        .unwrap();

    let _ = db
        .create_channel_message(channel, user_a, "3", OffsetDateTime::now_utc(), 3)
        .await
        .unwrap();

    // Smoke test: can we detect a new message?
    assert!(db.has_new_message_tx(channel, user_b).await.unwrap());

    let b_connection_id = rpc::ConnectionId { owner_id, id: 1 };
    db.join_channel_chat(channel, b_connection_id, user_b)
        .await
        .unwrap();

    // Joining the channel should _not_ update us to the latest message
    assert!(db.has_new_message_tx(channel, user_b).await.unwrap());

    // Reading the earlier messages should not change that we have new messages
    let _ = db
        .get_channel_messages(channel, user_b, 1, Some(second_message))
        .await
        .unwrap();

    assert!(db.has_new_message_tx(channel, user_b).await.unwrap());

    // This constraint is currently inexpressible, creating a message implicitly broadcasts
    // it to all participants
    //
    // Creating new messages when we haven't read the latest one should not change the flag
    // let _ = db
    //     .create_channel_message(channel, user_a, "4", OffsetDateTime::now_utc(), 4)
    //     .await
    //     .unwrap();
    // assert!(db.has_new_message_tx(channel, user_b).await.unwrap());

    // But reading the latest message should clear the flag
    let _ = db
        .get_channel_messages(channel, user_b, 4, None)
        .await
        .unwrap();

    assert!(!db.has_new_message_tx(channel, user_b).await.unwrap());

    // And future messages should not reset the flag
    let _ = db
        .create_channel_message(channel, user_a, "5", OffsetDateTime::now_utc(), 5)
        .await
        .unwrap();

    assert!(!db.has_new_message_tx(channel, user_b).await.unwrap());

    let _ = db
        .create_channel_message(channel, user_b, "6", OffsetDateTime::now_utc(), 6)
        .await
        .unwrap();

    assert!(!db.has_new_message_tx(channel, user_b).await.unwrap());

    // And we should start seeing the flag again after we've left the channel
    db.leave_channel_chat(channel, b_connection_id, user_b)
        .await
        .unwrap();

    assert!(!db.has_new_message_tx(channel, user_b).await.unwrap());

    let _ = db
        .create_channel_message(channel, user_a, "7", OffsetDateTime::now_utc(), 7)
        .await
        .unwrap();

    assert!(db.has_new_message_tx(channel, user_b).await.unwrap());
}
