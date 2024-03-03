use super::new_test_user;
use crate::{
    db::{ChannelRole, Database, MessageId},
    test_both_dbs,
};
use channel::mentions_to_proto;
use std::sync::Arc;
use time::OffsetDateTime;

test_both_dbs!(
    test_channel_message_retrieval,
    test_channel_message_retrieval_postgres,
    test_channel_message_retrieval_sqlite
);

async fn test_channel_message_retrieval(db: &Arc<Database>) {
    let user = new_test_user(db, "user@example.com").await;
    let channel = db.create_channel("channel", None, user).await.unwrap().0;

    let owner_id = db.create_server("test").await.unwrap().0 as u32;
    db.join_channel_chat(channel.id, rpc::ConnectionId { owner_id, id: 0 }, user)
        .await
        .unwrap();

    let mut all_messages = Vec::new();
    for i in 0..10 {
        all_messages.push(
            db.create_channel_message(
                channel.id,
                user,
                &i.to_string(),
                &[],
                OffsetDateTime::now_utc(),
                i,
                None,
            )
            .await
            .unwrap()
            .message_id
            .to_proto(),
        );
    }

    let messages = db
        .get_channel_messages(channel.id, user, 3, None)
        .await
        .unwrap()
        .into_iter()
        .map(|message| message.id)
        .collect::<Vec<_>>();
    assert_eq!(messages, &all_messages[7..10]);

    let messages = db
        .get_channel_messages(
            channel.id,
            user,
            4,
            Some(MessageId::from_proto(all_messages[6])),
        )
        .await
        .unwrap()
        .into_iter()
        .map(|message| message.id)
        .collect::<Vec<_>>();
    assert_eq!(messages, &all_messages[2..6]);
}

test_both_dbs!(
    test_channel_message_nonces,
    test_channel_message_nonces_postgres,
    test_channel_message_nonces_sqlite
);

async fn test_channel_message_nonces(db: &Arc<Database>) {
    let user_a = new_test_user(db, "user_a@example.com").await;
    let user_b = new_test_user(db, "user_b@example.com").await;
    let user_c = new_test_user(db, "user_c@example.com").await;
    let channel = db.create_root_channel("channel", user_a).await.unwrap();
    db.invite_channel_member(channel, user_b, user_a, ChannelRole::Member)
        .await
        .unwrap();
    db.invite_channel_member(channel, user_c, user_a, ChannelRole::Member)
        .await
        .unwrap();
    db.respond_to_channel_invite(channel, user_b, true)
        .await
        .unwrap();
    db.respond_to_channel_invite(channel, user_c, true)
        .await
        .unwrap();

    let owner_id = db.create_server("test").await.unwrap().0 as u32;
    db.join_channel_chat(channel, rpc::ConnectionId { owner_id, id: 0 }, user_a)
        .await
        .unwrap();
    db.join_channel_chat(channel, rpc::ConnectionId { owner_id, id: 1 }, user_b)
        .await
        .unwrap();

    // As user A, create messages that reuse the same nonces. The requests
    // succeed, but return the same ids.
    let id1 = db
        .create_channel_message(
            channel,
            user_a,
            "hi @user_b",
            &mentions_to_proto(&[(3..10, user_b.to_proto())]),
            OffsetDateTime::now_utc(),
            100,
            None,
        )
        .await
        .unwrap()
        .message_id;
    let id2 = db
        .create_channel_message(
            channel,
            user_a,
            "hello, fellow users",
            &mentions_to_proto(&[]),
            OffsetDateTime::now_utc(),
            200,
            None,
        )
        .await
        .unwrap()
        .message_id;
    let id3 = db
        .create_channel_message(
            channel,
            user_a,
            "bye @user_c (same nonce as first message)",
            &mentions_to_proto(&[(4..11, user_c.to_proto())]),
            OffsetDateTime::now_utc(),
            100,
            None,
        )
        .await
        .unwrap()
        .message_id;
    let id4 = db
        .create_channel_message(
            channel,
            user_a,
            "omg (same nonce as second message)",
            &mentions_to_proto(&[]),
            OffsetDateTime::now_utc(),
            200,
            None,
        )
        .await
        .unwrap()
        .message_id;

    // As a different user, reuse one of the same nonces. This request succeeds
    // and returns a different id.
    let id5 = db
        .create_channel_message(
            channel,
            user_b,
            "omg @user_a (same nonce as user_a's first message)",
            &mentions_to_proto(&[(4..11, user_a.to_proto())]),
            OffsetDateTime::now_utc(),
            100,
            None,
        )
        .await
        .unwrap()
        .message_id;

    assert_ne!(id1, id2);
    assert_eq!(id1, id3);
    assert_eq!(id2, id4);
    assert_ne!(id5, id1);

    let messages = db
        .get_channel_messages(channel, user_a, 5, None)
        .await
        .unwrap()
        .into_iter()
        .map(|m| (m.id, m.body, m.mentions))
        .collect::<Vec<_>>();
    assert_eq!(
        messages,
        &[
            (
                id1.to_proto(),
                "hi @user_b".into(),
                mentions_to_proto(&[(3..10, user_b.to_proto())]),
            ),
            (
                id2.to_proto(),
                "hello, fellow users".into(),
                mentions_to_proto(&[])
            ),
            (
                id5.to_proto(),
                "omg @user_a (same nonce as user_a's first message)".into(),
                mentions_to_proto(&[(4..11, user_a.to_proto())]),
            ),
        ]
    );
}

test_both_dbs!(
    test_unseen_channel_messages,
    test_unseen_channel_messages_postgres,
    test_unseen_channel_messages_sqlite
);

async fn test_unseen_channel_messages(db: &Arc<Database>) {
    let user = new_test_user(db, "user_a@example.com").await;
    let observer = new_test_user(db, "user_b@example.com").await;

    let channel_1 = db.create_root_channel("channel", user).await.unwrap();
    let channel_2 = db.create_root_channel("channel-2", user).await.unwrap();

    db.invite_channel_member(channel_1, observer, user, ChannelRole::Member)
        .await
        .unwrap();
    db.invite_channel_member(channel_2, observer, user, ChannelRole::Member)
        .await
        .unwrap();

    db.respond_to_channel_invite(channel_1, observer, true)
        .await
        .unwrap();
    db.respond_to_channel_invite(channel_2, observer, true)
        .await
        .unwrap();

    let owner_id = db.create_server("test").await.unwrap().0 as u32;
    let user_connection_id = rpc::ConnectionId { owner_id, id: 0 };

    db.join_channel_chat(channel_1, user_connection_id, user)
        .await
        .unwrap();

    let _ = db
        .create_channel_message(
            channel_1,
            user,
            "1_1",
            &[],
            OffsetDateTime::now_utc(),
            1,
            None,
        )
        .await
        .unwrap();

    let _ = db
        .create_channel_message(
            channel_1,
            user,
            "1_2",
            &[],
            OffsetDateTime::now_utc(),
            2,
            None,
        )
        .await
        .unwrap();

    let third_message = db
        .create_channel_message(
            channel_1,
            user,
            "1_3",
            &[],
            OffsetDateTime::now_utc(),
            3,
            None,
        )
        .await
        .unwrap()
        .message_id;

    db.join_channel_chat(channel_2, user_connection_id, user)
        .await
        .unwrap();

    let fourth_message = db
        .create_channel_message(
            channel_2,
            user,
            "2_1",
            &[],
            OffsetDateTime::now_utc(),
            4,
            None,
        )
        .await
        .unwrap()
        .message_id;

    // Check that observer has new messages
    let latest_messages = db
        .transaction(|tx| async move {
            db.latest_channel_messages(&[channel_1, channel_2], &tx)
                .await
        })
        .await
        .unwrap();

    assert_eq!(
        latest_messages,
        [
            rpc::proto::ChannelMessageId {
                channel_id: channel_1.to_proto(),
                message_id: third_message.to_proto(),
            },
            rpc::proto::ChannelMessageId {
                channel_id: channel_2.to_proto(),
                message_id: fourth_message.to_proto(),
            },
        ]
    );
}

test_both_dbs!(
    test_channel_message_mentions,
    test_channel_message_mentions_postgres,
    test_channel_message_mentions_sqlite
);

async fn test_channel_message_mentions(db: &Arc<Database>) {
    let user_a = new_test_user(db, "user_a@example.com").await;
    let user_b = new_test_user(db, "user_b@example.com").await;
    let user_c = new_test_user(db, "user_c@example.com").await;

    let channel = db
        .create_channel("channel", None, user_a)
        .await
        .unwrap()
        .0
        .id;
    db.invite_channel_member(channel, user_b, user_a, ChannelRole::Member)
        .await
        .unwrap();
    db.respond_to_channel_invite(channel, user_b, true)
        .await
        .unwrap();

    let owner_id = db.create_server("test").await.unwrap().0 as u32;
    let connection_id = rpc::ConnectionId { owner_id, id: 0 };
    db.join_channel_chat(channel, connection_id, user_a)
        .await
        .unwrap();

    db.create_channel_message(
        channel,
        user_a,
        "hi @user_b and @user_c",
        &mentions_to_proto(&[(3..10, user_b.to_proto()), (15..22, user_c.to_proto())]),
        OffsetDateTime::now_utc(),
        1,
        None,
    )
    .await
    .unwrap();
    db.create_channel_message(
        channel,
        user_a,
        "bye @user_c",
        &mentions_to_proto(&[(4..11, user_c.to_proto())]),
        OffsetDateTime::now_utc(),
        2,
        None,
    )
    .await
    .unwrap();
    db.create_channel_message(
        channel,
        user_a,
        "umm",
        &mentions_to_proto(&[]),
        OffsetDateTime::now_utc(),
        3,
        None,
    )
    .await
    .unwrap();
    db.create_channel_message(
        channel,
        user_a,
        "@user_b, stop.",
        &mentions_to_proto(&[(0..7, user_b.to_proto())]),
        OffsetDateTime::now_utc(),
        4,
        None,
    )
    .await
    .unwrap();

    let messages = db
        .get_channel_messages(channel, user_b, 5, None)
        .await
        .unwrap()
        .into_iter()
        .map(|m| (m.body, m.mentions))
        .collect::<Vec<_>>();
    assert_eq!(
        &messages,
        &[
            (
                "hi @user_b and @user_c".into(),
                mentions_to_proto(&[(3..10, user_b.to_proto()), (15..22, user_c.to_proto())]),
            ),
            (
                "bye @user_c".into(),
                mentions_to_proto(&[(4..11, user_c.to_proto())]),
            ),
            ("umm".into(), mentions_to_proto(&[]),),
            (
                "@user_b, stop.".into(),
                mentions_to_proto(&[(0..7, user_b.to_proto())]),
            ),
        ]
    );
}
