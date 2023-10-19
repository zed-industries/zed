use crate::{
    db::{ChannelRole, Database, MessageId, NewUserParams},
    test_both_dbs,
};
use std::sync::Arc;
use time::OffsetDateTime;

test_both_dbs!(
    test_channel_message_retrieval,
    test_channel_message_retrieval_postgres,
    test_channel_message_retrieval_sqlite
);

async fn test_channel_message_retrieval(db: &Arc<Database>) {
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
    let channel = db.create_channel("channel", None, user).await.unwrap();

    let owner_id = db.create_server("test").await.unwrap().0 as u32;
    db.join_channel_chat(channel, rpc::ConnectionId { owner_id, id: 0 }, user)
        .await
        .unwrap();

    let mut all_messages = Vec::new();
    for i in 0..10 {
        all_messages.push(
            db.create_channel_message(channel, user, &i.to_string(), OffsetDateTime::now_utc(), i)
                .await
                .unwrap()
                .0
                .to_proto(),
        );
    }

    let messages = db
        .get_channel_messages(channel, user, 3, None)
        .await
        .unwrap()
        .into_iter()
        .map(|message| message.id)
        .collect::<Vec<_>>();
    assert_eq!(messages, &all_messages[7..10]);

    let messages = db
        .get_channel_messages(
            channel,
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
    let channel = db.create_channel("channel", None, user).await.unwrap();

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
    let user = db
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
    let observer = db
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

    let channel_1 = db.create_channel("channel", None, user).await.unwrap();

    let channel_2 = db.create_channel("channel-2", None, user).await.unwrap();

    db.invite_channel_member(channel_1, observer, user, ChannelRole::Member)
        .await
        .unwrap();

    db.respond_to_channel_invite(channel_1, observer, true)
        .await
        .unwrap();

    db.invite_channel_member(channel_2, observer, user, ChannelRole::Member)
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
        .create_channel_message(channel_1, user, "1_1", OffsetDateTime::now_utc(), 1)
        .await
        .unwrap();

    let (second_message, _, _) = db
        .create_channel_message(channel_1, user, "1_2", OffsetDateTime::now_utc(), 2)
        .await
        .unwrap();

    let (third_message, _, _) = db
        .create_channel_message(channel_1, user, "1_3", OffsetDateTime::now_utc(), 3)
        .await
        .unwrap();

    db.join_channel_chat(channel_2, user_connection_id, user)
        .await
        .unwrap();

    let (fourth_message, _, _) = db
        .create_channel_message(channel_2, user, "2_1", OffsetDateTime::now_utc(), 4)
        .await
        .unwrap();

    // Check that observer has new messages
    let unseen_messages = db
        .transaction(|tx| async move {
            db.unseen_channel_messages(observer, &[channel_1, channel_2], &*tx)
                .await
        })
        .await
        .unwrap();

    assert_eq!(
        unseen_messages,
        [
            rpc::proto::UnseenChannelMessage {
                channel_id: channel_1.to_proto(),
                message_id: third_message.to_proto(),
            },
            rpc::proto::UnseenChannelMessage {
                channel_id: channel_2.to_proto(),
                message_id: fourth_message.to_proto(),
            },
        ]
    );

    // Observe the second message
    db.observe_channel_message(channel_1, observer, second_message)
        .await
        .unwrap();

    // Make sure the observer still has a new message
    let unseen_messages = db
        .transaction(|tx| async move {
            db.unseen_channel_messages(observer, &[channel_1, channel_2], &*tx)
                .await
        })
        .await
        .unwrap();
    assert_eq!(
        unseen_messages,
        [
            rpc::proto::UnseenChannelMessage {
                channel_id: channel_1.to_proto(),
                message_id: third_message.to_proto(),
            },
            rpc::proto::UnseenChannelMessage {
                channel_id: channel_2.to_proto(),
                message_id: fourth_message.to_proto(),
            },
        ]
    );

    // Observe the third message,
    db.observe_channel_message(channel_1, observer, third_message)
        .await
        .unwrap();

    // Make sure the observer does not have a new method
    let unseen_messages = db
        .transaction(|tx| async move {
            db.unseen_channel_messages(observer, &[channel_1, channel_2], &*tx)
                .await
        })
        .await
        .unwrap();

    assert_eq!(
        unseen_messages,
        [rpc::proto::UnseenChannelMessage {
            channel_id: channel_2.to_proto(),
            message_id: fourth_message.to_proto(),
        }]
    );

    // Observe the second message again, should not regress our observed state
    db.observe_channel_message(channel_1, observer, second_message)
        .await
        .unwrap();

    // Make sure the observer does not have a new message
    let unseen_messages = db
        .transaction(|tx| async move {
            db.unseen_channel_messages(observer, &[channel_1, channel_2], &*tx)
                .await
        })
        .await
        .unwrap();
    assert_eq!(
        unseen_messages,
        [rpc::proto::UnseenChannelMessage {
            channel_id: channel_2.to_proto(),
            message_id: fourth_message.to_proto(),
        }]
    );
}
