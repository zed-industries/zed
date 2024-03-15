use crate::{rpc::RECONNECT_TIMEOUT, tests::TestServer};
use channel::{ChannelChat, ChannelMessageId, MessageParams};
use collab_ui::chat_panel::ChatPanel;
use gpui::{BackgroundExecutor, Model, TestAppContext};
use rpc::Notification;
use workspace::dock::Panel;

#[gpui::test]
async fn test_basic_channel_messages(
    executor: BackgroundExecutor,
    mut cx_a: &mut TestAppContext,
    mut cx_b: &mut TestAppContext,
    mut cx_c: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let client_c = server.create_client(cx_c, "user_c").await;

    let channel_id = server
        .make_channel(
            "the-channel",
            None,
            (&client_a, cx_a),
            &mut [(&client_b, cx_b), (&client_c, cx_c)],
        )
        .await;

    let channel_chat_a = client_a
        .channel_store()
        .update(cx_a, |store, cx| store.open_channel_chat(channel_id, cx))
        .await
        .unwrap();
    let channel_chat_b = client_b
        .channel_store()
        .update(cx_b, |store, cx| store.open_channel_chat(channel_id, cx))
        .await
        .unwrap();

    let message_id = channel_chat_a
        .update(cx_a, |c, cx| {
            c.send_message(
                MessageParams {
                    text: "hi @user_c!".into(),
                    mentions: vec![(3..10, client_c.id())],
                    reply_to_message_id: None,
                },
                cx,
            )
            .unwrap()
        })
        .await
        .unwrap();
    channel_chat_a
        .update(cx_a, |c, cx| c.send_message("two".into(), cx).unwrap())
        .await
        .unwrap();

    executor.run_until_parked();
    channel_chat_b
        .update(cx_b, |c, cx| c.send_message("three".into(), cx).unwrap())
        .await
        .unwrap();

    executor.run_until_parked();

    let channel_chat_c = client_c
        .channel_store()
        .update(cx_c, |store, cx| store.open_channel_chat(channel_id, cx))
        .await
        .unwrap();

    for (chat, cx) in [
        (&channel_chat_a, &mut cx_a),
        (&channel_chat_b, &mut cx_b),
        (&channel_chat_c, &mut cx_c),
    ] {
        chat.update(*cx, |c, _| {
            assert_eq!(
                c.messages()
                    .iter()
                    .map(|m| (m.body.as_str(), m.mentions.as_slice()))
                    .collect::<Vec<_>>(),
                vec![
                    ("hi @user_c!", [(3..10, client_c.id())].as_slice()),
                    ("two", &[]),
                    ("three", &[])
                ],
                "results for user {}",
                c.client().id(),
            );
        });
    }

    client_c.notification_store().update(cx_c, |store, _| {
        assert_eq!(store.notification_count(), 2);
        assert_eq!(store.unread_notification_count(), 1);
        assert_eq!(
            store.notification_at(0).unwrap().notification,
            Notification::ChannelMessageMention {
                message_id,
                sender_id: client_a.id(),
                channel_id: channel_id.0,
            }
        );
        assert_eq!(
            store.notification_at(1).unwrap().notification,
            Notification::ChannelInvitation {
                channel_id: channel_id.0,
                channel_name: "the-channel".to_string(),
                inviter_id: client_a.id()
            }
        );
    });
}

#[gpui::test]
async fn test_rejoin_channel_chat(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    let channel_id = server
        .make_channel(
            "the-channel",
            None,
            (&client_a, cx_a),
            &mut [(&client_b, cx_b)],
        )
        .await;

    let channel_chat_a = client_a
        .channel_store()
        .update(cx_a, |store, cx| store.open_channel_chat(channel_id, cx))
        .await
        .unwrap();
    let channel_chat_b = client_b
        .channel_store()
        .update(cx_b, |store, cx| store.open_channel_chat(channel_id, cx))
        .await
        .unwrap();

    channel_chat_a
        .update(cx_a, |c, cx| c.send_message("one".into(), cx).unwrap())
        .await
        .unwrap();
    channel_chat_b
        .update(cx_b, |c, cx| c.send_message("two".into(), cx).unwrap())
        .await
        .unwrap();

    server.forbid_connections();
    server.disconnect_client(client_a.peer_id().unwrap());

    // While client A is disconnected, clients A and B both send new messages.
    channel_chat_a
        .update(cx_a, |c, cx| c.send_message("three".into(), cx).unwrap())
        .await
        .unwrap_err();
    channel_chat_a
        .update(cx_a, |c, cx| c.send_message("four".into(), cx).unwrap())
        .await
        .unwrap_err();
    channel_chat_b
        .update(cx_b, |c, cx| c.send_message("five".into(), cx).unwrap())
        .await
        .unwrap();
    channel_chat_b
        .update(cx_b, |c, cx| c.send_message("six".into(), cx).unwrap())
        .await
        .unwrap();

    // Client A reconnects.
    server.allow_connections();
    executor.advance_clock(RECONNECT_TIMEOUT);

    // Client A fetches the messages that were sent while they were disconnected
    // and resends their own messages which failed to send.
    let expected_messages = &["one", "two", "five", "six", "three", "four"];
    assert_messages(&channel_chat_a, expected_messages, cx_a);
    assert_messages(&channel_chat_b, expected_messages, cx_b);
}

#[gpui::test]
async fn test_remove_channel_message(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let client_c = server.create_client(cx_c, "user_c").await;

    let channel_id = server
        .make_channel(
            "the-channel",
            None,
            (&client_a, cx_a),
            &mut [(&client_b, cx_b), (&client_c, cx_c)],
        )
        .await;

    let channel_chat_a = client_a
        .channel_store()
        .update(cx_a, |store, cx| store.open_channel_chat(channel_id, cx))
        .await
        .unwrap();
    let channel_chat_b = client_b
        .channel_store()
        .update(cx_b, |store, cx| store.open_channel_chat(channel_id, cx))
        .await
        .unwrap();

    // Client A sends some messages.
    channel_chat_a
        .update(cx_a, |c, cx| c.send_message("one".into(), cx).unwrap())
        .await
        .unwrap();
    channel_chat_a
        .update(cx_a, |c, cx| c.send_message("two".into(), cx).unwrap())
        .await
        .unwrap();
    channel_chat_a
        .update(cx_a, |c, cx| c.send_message("three".into(), cx).unwrap())
        .await
        .unwrap();

    // Clients A and B see all of the messages.
    executor.run_until_parked();
    let expected_messages = &["one", "two", "three"];
    assert_messages(&channel_chat_a, expected_messages, cx_a);
    assert_messages(&channel_chat_b, expected_messages, cx_b);

    // Client A deletes one of their messages.
    channel_chat_a
        .update(cx_a, |c, cx| {
            let ChannelMessageId::Saved(id) = c.message(1).id else {
                panic!("message not saved")
            };
            c.remove_message(id, cx)
        })
        .await
        .unwrap();

    // Client B sees that the message is gone.
    executor.run_until_parked();
    let expected_messages = &["one", "three"];
    assert_messages(&channel_chat_a, expected_messages, cx_a);
    assert_messages(&channel_chat_b, expected_messages, cx_b);

    // Client C joins the channel chat, and does not see the deleted message.
    let channel_chat_c = client_c
        .channel_store()
        .update(cx_c, |store, cx| store.open_channel_chat(channel_id, cx))
        .await
        .unwrap();
    assert_messages(&channel_chat_c, expected_messages, cx_c);
}

#[track_caller]
fn assert_messages(chat: &Model<ChannelChat>, messages: &[&str], cx: &mut TestAppContext) {
    assert_eq!(
        chat.read_with(cx, |chat, _| {
            chat.messages()
                .iter()
                .map(|m| m.body.clone())
                .collect::<Vec<_>>()
        }),
        messages
    );
}

#[gpui::test]
async fn test_channel_message_changes(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    let channel_id = server
        .make_channel(
            "the-channel",
            None,
            (&client_a, cx_a),
            &mut [(&client_b, cx_b)],
        )
        .await;

    // Client A sends a message, client B should see that there is a new message.
    let channel_chat_a = client_a
        .channel_store()
        .update(cx_a, |store, cx| store.open_channel_chat(channel_id, cx))
        .await
        .unwrap();

    channel_chat_a
        .update(cx_a, |c, cx| c.send_message("one".into(), cx).unwrap())
        .await
        .unwrap();

    executor.run_until_parked();

    let b_has_messages = cx_b.update(|cx| {
        client_b
            .channel_store()
            .read(cx)
            .has_new_messages(channel_id)
    });

    assert!(b_has_messages);

    // Opening the chat should clear the changed flag.
    cx_b.update(|cx| {
        collab_ui::init(&client_b.app_state, cx);
    });
    let project_b = client_b.build_empty_local_project(cx_b);
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);

    let chat_panel_b = workspace_b.update(cx_b, |workspace, cx| ChatPanel::new(workspace, cx));
    chat_panel_b
        .update(cx_b, |chat_panel, cx| {
            chat_panel.set_active(true, cx);
            chat_panel.select_channel(channel_id, None, cx)
        })
        .await
        .unwrap();

    executor.run_until_parked();

    let b_has_messages = cx_b.update(|cx| {
        client_b
            .channel_store()
            .read(cx)
            .has_new_messages(channel_id)
    });

    assert!(!b_has_messages);

    // Sending a message while the chat is open should not change the flag.
    channel_chat_a
        .update(cx_a, |c, cx| c.send_message("two".into(), cx).unwrap())
        .await
        .unwrap();

    executor.run_until_parked();

    let b_has_messages = cx_b.update(|cx| {
        client_b
            .channel_store()
            .read(cx)
            .has_new_messages(channel_id)
    });

    assert!(!b_has_messages);

    // Sending a message while the chat is closed should change the flag.
    chat_panel_b.update(cx_b, |chat_panel, cx| {
        chat_panel.set_active(false, cx);
    });

    // Sending a message while the chat is open should not change the flag.
    channel_chat_a
        .update(cx_a, |c, cx| c.send_message("three".into(), cx).unwrap())
        .await
        .unwrap();

    executor.run_until_parked();

    let b_has_messages = cx_b.update(|cx| {
        client_b
            .channel_store()
            .read(cx)
            .has_new_messages(channel_id)
    });

    assert!(b_has_messages);

    // Closing the chat should re-enable change tracking
    cx_b.update(|_| drop(chat_panel_b));

    channel_chat_a
        .update(cx_a, |c, cx| c.send_message("four".into(), cx).unwrap())
        .await
        .unwrap();

    executor.run_until_parked();

    let b_has_messages = cx_b.update(|cx| {
        client_b
            .channel_store()
            .read(cx)
            .has_new_messages(channel_id)
    });

    assert!(b_has_messages);
}

#[gpui::test]
async fn test_chat_replies(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    let mut server = TestServer::start(cx_a.executor()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    let channel_id = server
        .make_channel(
            "the-channel",
            None,
            (&client_a, cx_a),
            &mut [(&client_b, cx_b)],
        )
        .await;

    // Client A sends a message, client B should see that there is a new message.
    let channel_chat_a = client_a
        .channel_store()
        .update(cx_a, |store, cx| store.open_channel_chat(channel_id, cx))
        .await
        .unwrap();

    let channel_chat_b = client_b
        .channel_store()
        .update(cx_b, |store, cx| store.open_channel_chat(channel_id, cx))
        .await
        .unwrap();

    let msg_id = channel_chat_a
        .update(cx_a, |c, cx| c.send_message("one".into(), cx).unwrap())
        .await
        .unwrap();

    cx_a.run_until_parked();

    let reply_id = channel_chat_b
        .update(cx_b, |c, cx| {
            c.send_message(
                MessageParams {
                    text: "reply".into(),
                    reply_to_message_id: Some(msg_id),
                    mentions: Vec::new(),
                },
                cx,
            )
            .unwrap()
        })
        .await
        .unwrap();

    cx_a.run_until_parked();

    channel_chat_a.update(cx_a, |channel_chat, _| {
        assert_eq!(
            channel_chat
                .find_loaded_message(reply_id)
                .unwrap()
                .reply_to_message_id,
            Some(msg_id),
        )
    });
}

#[gpui::test]
async fn test_chat_editing(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    let mut server = TestServer::start(cx_a.executor()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    let channel_id = server
        .make_channel(
            "the-channel",
            None,
            (&client_a, cx_a),
            &mut [(&client_b, cx_b)],
        )
        .await;

    // Client A sends a message, client B should see that there is a new message.
    let channel_chat_a = client_a
        .channel_store()
        .update(cx_a, |store, cx| store.open_channel_chat(channel_id, cx))
        .await
        .unwrap();

    let channel_chat_b = client_b
        .channel_store()
        .update(cx_b, |store, cx| store.open_channel_chat(channel_id, cx))
        .await
        .unwrap();

    let msg_id = channel_chat_a
        .update(cx_a, |c, cx| {
            c.send_message(
                MessageParams {
                    text: "Initial message".into(),
                    reply_to_message_id: None,
                    mentions: Vec::new(),
                },
                cx,
            )
            .unwrap()
        })
        .await
        .unwrap();

    cx_a.run_until_parked();

    channel_chat_a
        .update(cx_a, |c, cx| {
            c.update_message(
                msg_id,
                MessageParams {
                    text: "Updated body".into(),
                    reply_to_message_id: None,
                    mentions: Vec::new(),
                },
                cx,
            )
            .unwrap()
        })
        .await
        .unwrap();

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    channel_chat_a.update(cx_a, |channel_chat, _| {
        let update_message = channel_chat.find_loaded_message(msg_id).unwrap();

        assert_eq!(update_message.body, "Updated body");
        assert_eq!(update_message.mentions, Vec::new());
    });
    channel_chat_b.update(cx_b, |channel_chat, _| {
        let update_message = channel_chat.find_loaded_message(msg_id).unwrap();

        assert_eq!(update_message.body, "Updated body");
        assert_eq!(update_message.mentions, Vec::new());
    });

    // test mentions are updated correctly

    client_b.notification_store().read_with(cx_b, |store, _| {
        assert_eq!(store.notification_count(), 1);
        let entry = store.notification_at(0).unwrap();
        assert!(matches!(
            entry.notification,
            Notification::ChannelInvitation { .. }
        ),);
    });

    channel_chat_a
        .update(cx_a, |c, cx| {
            c.update_message(
                msg_id,
                MessageParams {
                    text: "Updated body including a mention for @user_b".into(),
                    reply_to_message_id: None,
                    mentions: vec![(37..45, client_b.id())],
                },
                cx,
            )
            .unwrap()
        })
        .await
        .unwrap();

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    channel_chat_a.update(cx_a, |channel_chat, _| {
        assert_eq!(
            channel_chat.find_loaded_message(msg_id).unwrap().body,
            "Updated body including a mention for @user_b",
        )
    });
    channel_chat_b.update(cx_b, |channel_chat, _| {
        assert_eq!(
            channel_chat.find_loaded_message(msg_id).unwrap().body,
            "Updated body including a mention for @user_b",
        )
    });
    client_b.notification_store().read_with(cx_b, |store, _| {
        assert_eq!(store.notification_count(), 2);
        let entry = store.notification_at(0).unwrap();
        assert_eq!(
            entry.notification,
            Notification::ChannelMessageMention {
                message_id: msg_id,
                sender_id: client_a.id(),
                channel_id: channel_id.0,
            }
        );
    });
}
