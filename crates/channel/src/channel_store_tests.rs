use crate::channel_chat::ChannelChatEvent;

use super::*;
use client::{test::FakeServer, Client, UserStore};
use gpui::{AppContext, ModelHandle, TestAppContext};
use rpc::proto;
use settings::SettingsStore;
use util::http::FakeHttpClient;

#[gpui::test]
fn test_update_channels(cx: &mut AppContext) {
    let channel_store = init_test(cx);

    update_channels(
        &channel_store,
        proto::UpdateChannels {
            channels: vec![
                proto::Channel {
                    id: 1,
                    name: "b".to_string(),
                },
                proto::Channel {
                    id: 2,
                    name: "a".to_string(),

                },
            ],
            channel_permissions: vec![proto::ChannelPermission {
                channel_id: 1,
                is_admin: true,
            }],
            ..Default::default()
        },
        cx,
    );
    assert_channels(
        &channel_store,
        &[
            //
            (0, "a".to_string(), false),
            (0, "b".to_string(), true),
        ],
        cx,
    );

    update_channels(
        &channel_store,
        proto::UpdateChannels {
            channels: vec![
                proto::Channel {
                    id: 3,
                    name: "x".to_string(),
                },
                proto::Channel {
                    id: 4,
                    name: "y".to_string(),
                },
            ],
            insert_edge: vec![
                proto::ChannelEdge {
                    parent_id: 1,
                    channel_id: 3,
                },
                proto::ChannelEdge {
                    parent_id: 2,
                    channel_id: 4,
                },
            ],
            ..Default::default()
        },
        cx,
    );
    assert_channels(
        &channel_store,
        &[
            (0, "a".to_string(), false),
            (1, "y".to_string(), false),
            (0, "b".to_string(), true),
            (1, "x".to_string(), true),
        ],
        cx,
    );
}

#[gpui::test]
fn test_dangling_channel_paths(cx: &mut AppContext) {
    let channel_store = init_test(cx);

    update_channels(
        &channel_store,
        proto::UpdateChannels {
            channels: vec![
                proto::Channel {
                    id: 0,
                    name: "a".to_string(),
                },
                proto::Channel {
                    id: 1,
                    name: "b".to_string(),
                },
                proto::Channel {
                    id: 2,
                    name: "c".to_string(),
                },
            ],
            insert_edge: vec![
                proto::ChannelEdge {
                    parent_id: 0,
                    channel_id: 1,
                },
                proto::ChannelEdge {
                    parent_id: 1,
                    channel_id: 2,
                },
            ],
            channel_permissions: vec![proto::ChannelPermission {
                channel_id: 0,
                is_admin: true,
            }],
            ..Default::default()
        },
        cx,
    );
    // Sanity check
    assert_channels(
        &channel_store,
        &[
            //
            (0, "a".to_string(), true),
            (1, "b".to_string(), true),
            (2, "c".to_string(), true),
        ],
        cx,
    );

    update_channels(
        &channel_store,
        proto::UpdateChannels {
            delete_channels: vec![1, 2],
            ..Default::default()
        },
        cx,
    );

    // Make sure that the 1/2/3 path is gone
    assert_channels(&channel_store, &[(0, "a".to_string(), true)], cx);
}

#[gpui::test]
async fn test_channel_messages(cx: &mut TestAppContext) {
    let user_id = 5;
    let channel_id = 5;
    let channel_store = cx.update(init_test);
    let client = channel_store.read_with(cx, |s, _| s.client());
    let server = FakeServer::for_client(user_id, &client, cx).await;

    // Get the available channels.
    server.send(proto::UpdateChannels {
        channels: vec![proto::Channel {
            id: channel_id,
            name: "the-channel".to_string(),
        }],
        ..Default::default()
    });
    cx.foreground().run_until_parked();
    cx.read(|cx| {
        assert_channels(&channel_store, &[(0, "the-channel".to_string(), false)], cx);
    });

    let get_users = server.receive::<proto::GetUsers>().await.unwrap();
    assert_eq!(get_users.payload.user_ids, vec![5]);
    server.respond(
        get_users.receipt(),
        proto::UsersResponse {
            users: vec![proto::User {
                id: 5,
                github_login: "nathansobo".into(),
                avatar_url: "http://avatar.com/nathansobo".into(),
            }],
        },
    );

    // Join a channel and populate its existing messages.
    let channel = channel_store.update(cx, |store, cx| {
        let channel_id = store.channels().next().unwrap().1.id;
        store.open_channel_chat(channel_id, cx)
    });
    let join_channel = server.receive::<proto::JoinChannelChat>().await.unwrap();
    server.respond(
        join_channel.receipt(),
        proto::JoinChannelChatResponse {
            messages: vec![
                proto::ChannelMessage {
                    id: 10,
                    body: "a".into(),
                    timestamp: 1000,
                    sender_id: 5,
                    nonce: Some(1.into()),
                },
                proto::ChannelMessage {
                    id: 11,
                    body: "b".into(),
                    timestamp: 1001,
                    sender_id: 6,
                    nonce: Some(2.into()),
                },
            ],
            done: false,
        },
    );

    cx.foreground().start_waiting();

    // Client requests all users for the received messages
    let mut get_users = server.receive::<proto::GetUsers>().await.unwrap();
    get_users.payload.user_ids.sort();
    assert_eq!(get_users.payload.user_ids, vec![6]);
    server.respond(
        get_users.receipt(),
        proto::UsersResponse {
            users: vec![proto::User {
                id: 6,
                github_login: "maxbrunsfeld".into(),
                avatar_url: "http://avatar.com/maxbrunsfeld".into(),
            }],
        },
    );

    let channel = channel.await.unwrap();
    channel.read_with(cx, |channel, _| {
        assert_eq!(
            channel
                .messages_in_range(0..2)
                .map(|message| (message.sender.github_login.clone(), message.body.clone()))
                .collect::<Vec<_>>(),
            &[
                ("nathansobo".into(), "a".into()),
                ("maxbrunsfeld".into(), "b".into())
            ]
        );
    });

    // Receive a new message.
    server.send(proto::ChannelMessageSent {
        channel_id,
        message: Some(proto::ChannelMessage {
            id: 12,
            body: "c".into(),
            timestamp: 1002,
            sender_id: 7,
            nonce: Some(3.into()),
        }),
    });

    // Client requests user for message since they haven't seen them yet
    let get_users = server.receive::<proto::GetUsers>().await.unwrap();
    assert_eq!(get_users.payload.user_ids, vec![7]);
    server.respond(
        get_users.receipt(),
        proto::UsersResponse {
            users: vec![proto::User {
                id: 7,
                github_login: "as-cii".into(),
                avatar_url: "http://avatar.com/as-cii".into(),
            }],
        },
    );

    assert_eq!(
        channel.next_event(cx).await,
        ChannelChatEvent::MessagesUpdated {
            old_range: 2..2,
            new_count: 1,
        }
    );
    channel.read_with(cx, |channel, _| {
        assert_eq!(
            channel
                .messages_in_range(2..3)
                .map(|message| (message.sender.github_login.clone(), message.body.clone()))
                .collect::<Vec<_>>(),
            &[("as-cii".into(), "c".into())]
        )
    });

    // Scroll up to view older messages.
    channel.update(cx, |channel, cx| {
        assert!(channel.load_more_messages(cx));
    });
    let get_messages = server.receive::<proto::GetChannelMessages>().await.unwrap();
    assert_eq!(get_messages.payload.channel_id, 5);
    assert_eq!(get_messages.payload.before_message_id, 10);
    server.respond(
        get_messages.receipt(),
        proto::GetChannelMessagesResponse {
            done: true,
            messages: vec![
                proto::ChannelMessage {
                    id: 8,
                    body: "y".into(),
                    timestamp: 998,
                    sender_id: 5,
                    nonce: Some(4.into()),
                },
                proto::ChannelMessage {
                    id: 9,
                    body: "z".into(),
                    timestamp: 999,
                    sender_id: 6,
                    nonce: Some(5.into()),
                },
            ],
        },
    );

    assert_eq!(
        channel.next_event(cx).await,
        ChannelChatEvent::MessagesUpdated {
            old_range: 0..0,
            new_count: 2,
        }
    );
    channel.read_with(cx, |channel, _| {
        assert_eq!(
            channel
                .messages_in_range(0..2)
                .map(|message| (message.sender.github_login.clone(), message.body.clone()))
                .collect::<Vec<_>>(),
            &[
                ("nathansobo".into(), "y".into()),
                ("maxbrunsfeld".into(), "z".into())
            ]
        );
    });
}

fn init_test(cx: &mut AppContext) -> ModelHandle<ChannelStore> {
    let http = FakeHttpClient::with_404_response();
    let client = Client::new(http.clone(), cx);
    let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http, cx));

    cx.foreground().forbid_parking();
    cx.set_global(SettingsStore::test(cx));
    crate::init(&client);
    client::init(&client, cx);

    cx.add_model(|cx| ChannelStore::new(client, user_store, cx))
}

fn update_channels(
    channel_store: &ModelHandle<ChannelStore>,
    message: proto::UpdateChannels,
    cx: &mut AppContext,
) {
    let task = channel_store.update(cx, |store, cx| store.update_channels(message, cx));
    assert!(task.is_none());
}

#[track_caller]
fn assert_channels(
    channel_store: &ModelHandle<ChannelStore>,
    expected_channels: &[(usize, String, bool)],
    cx: &AppContext,
) {
    let actual = channel_store.read_with(cx, |store, _| {
        store
            .channels()
            .map(|(depth, channel)| {
                (
                    depth,
                    channel.name.to_string(),
                    store.is_user_admin(channel.id),
                )
            })
            .collect::<Vec<_>>()
    });
    assert_eq!(actual, expected_channels);
}
