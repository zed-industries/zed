use crate::channel_chat::ChannelChatEvent;

use super::*;
use client::{Client, UserStore, test::FakeServer};
use clock::FakeSystemClock;
use gpui::{App, AppContext as _, Entity, SemanticVersion, TestAppContext};
use http_client::FakeHttpClient;
use rpc::proto::{self};
use settings::SettingsStore;

#[gpui::test]
fn test_update_channels(cx: &mut App) {
    let channel_store = init_test(cx);

    update_channels(
        &channel_store,
        proto::UpdateChannels {
            channels: vec![
                proto::Channel {
                    id: 1,
                    name: "b".to_string(),
                    visibility: proto::ChannelVisibility::Members as i32,
                    parent_path: Vec::new(),
                    channel_order: 1,
                },
                proto::Channel {
                    id: 2,
                    name: "a".to_string(),
                    visibility: proto::ChannelVisibility::Members as i32,
                    parent_path: Vec::new(),
                    channel_order: 2,
                },
            ],
            ..Default::default()
        },
        cx,
    );
    assert_channels(
        &channel_store,
        &[
            //
            (0, "b".to_string()),
            (0, "a".to_string()),
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
                    visibility: proto::ChannelVisibility::Members as i32,
                    parent_path: vec![1],
                    channel_order: 1,
                },
                proto::Channel {
                    id: 4,
                    name: "y".to_string(),
                    visibility: proto::ChannelVisibility::Members as i32,
                    parent_path: vec![2],
                    channel_order: 1,
                },
            ],
            ..Default::default()
        },
        cx,
    );
    assert_channels(
        &channel_store,
        &[
            (0, "b".to_string()),
            (1, "x".to_string()),
            (0, "a".to_string()),
            (1, "y".to_string()),
        ],
        cx,
    );
}

#[gpui::test]
fn test_update_channels_order_independent(cx: &mut App) {
    /// Based on: https://stackoverflow.com/a/59939809
    fn unique_permutations<T: Clone>(items: Vec<T>) -> Vec<Vec<T>> {
        if items.len() == 1 {
            vec![items]
        } else {
            let mut output: Vec<Vec<T>> = vec![];

            for (ix, first) in items.iter().enumerate() {
                let mut remaining_elements = items.clone();
                remaining_elements.remove(ix);
                for mut permutation in unique_permutations(remaining_elements) {
                    permutation.insert(0, first.clone());
                    output.push(permutation);
                }
            }
            output
        }
    }

    let test_data = vec![
        proto::Channel {
            id: 6,
            name: "β".to_string(),
            visibility: proto::ChannelVisibility::Members as i32,
            parent_path: vec![1, 3],
            channel_order: 1,
        },
        proto::Channel {
            id: 5,
            name: "α".to_string(),
            visibility: proto::ChannelVisibility::Members as i32,
            parent_path: vec![1],
            channel_order: 2,
        },
        proto::Channel {
            id: 3,
            name: "x".to_string(),
            visibility: proto::ChannelVisibility::Members as i32,
            parent_path: vec![1],
            channel_order: 1,
        },
        proto::Channel {
            id: 4,
            name: "y".to_string(),
            visibility: proto::ChannelVisibility::Members as i32,
            parent_path: vec![2],
            channel_order: 1,
        },
        proto::Channel {
            id: 1,
            name: "b".to_string(),
            visibility: proto::ChannelVisibility::Members as i32,
            parent_path: Vec::new(),
            channel_order: 1,
        },
        proto::Channel {
            id: 2,
            name: "a".to_string(),
            visibility: proto::ChannelVisibility::Members as i32,
            parent_path: Vec::new(),
            channel_order: 2,
        },
    ];

    let channel_store = init_test(cx);
    let permutations = unique_permutations(test_data);

    for test_instance in permutations {
        channel_store.update(cx, |channel_store, _| channel_store.reset());

        update_channels(
            &channel_store,
            proto::UpdateChannels {
                channels: test_instance,
                ..Default::default()
            },
            cx,
        );

        assert_channels(
            &channel_store,
            &[
                (0, "b".to_string()),
                (1, "x".to_string()),
                (2, "β".to_string()),
                (1, "α".to_string()),
                (0, "a".to_string()),
                (1, "y".to_string()),
            ],
            cx,
        );
    }
}

#[gpui::test]
fn test_dangling_channel_paths(cx: &mut App) {
    let channel_store = init_test(cx);

    update_channels(
        &channel_store,
        proto::UpdateChannels {
            channels: vec![
                proto::Channel {
                    id: 0,
                    name: "a".to_string(),
                    visibility: proto::ChannelVisibility::Members as i32,
                    parent_path: vec![],
                    channel_order: 1,
                },
                proto::Channel {
                    id: 1,
                    name: "b".to_string(),
                    visibility: proto::ChannelVisibility::Members as i32,
                    parent_path: vec![0],
                    channel_order: 1,
                },
                proto::Channel {
                    id: 2,
                    name: "c".to_string(),
                    visibility: proto::ChannelVisibility::Members as i32,
                    parent_path: vec![0, 1],
                    channel_order: 1,
                },
            ],
            ..Default::default()
        },
        cx,
    );
    // Sanity check
    assert_channels(
        &channel_store,
        &[
            //
            (0, "a".to_string()),
            (1, "b".to_string()),
            (2, "c".to_string()),
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
    assert_channels(&channel_store, &[(0, "a".to_string())], cx);
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
            visibility: proto::ChannelVisibility::Members as i32,
            parent_path: vec![],
            channel_order: 1,
        }],
        ..Default::default()
    });
    cx.executor().run_until_parked();
    cx.update(|cx| {
        assert_channels(&channel_store, &[(0, "the-channel".to_string())], cx);
    });

    // Join a channel and populate its existing messages.
    let channel = channel_store.update(cx, |store, cx| {
        let channel_id = store.ordered_channels().next().unwrap().1.id;
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
                    mentions: vec![],
                    nonce: Some(1.into()),
                    reply_to_message_id: None,
                    edited_at: None,
                },
                proto::ChannelMessage {
                    id: 11,
                    body: "b".into(),
                    timestamp: 1001,
                    sender_id: 6,
                    mentions: vec![],
                    nonce: Some(2.into()),
                    reply_to_message_id: None,
                    edited_at: None,
                },
            ],
            done: false,
        },
    );

    cx.executor().start_waiting();

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
                name: None,
            }],
        },
    );

    let channel = channel.await.unwrap();
    channel.update(cx, |channel, _| {
        assert_eq!(
            channel
                .messages_in_range(0..2)
                .map(|message| (message.sender.github_login.clone(), message.body.clone()))
                .collect::<Vec<_>>(),
            &[
                ("user-5".into(), "a".into()),
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
            mentions: vec![],
            nonce: Some(3.into()),
            reply_to_message_id: None,
            edited_at: None,
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
                name: None,
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
    channel.update(cx, |channel, _| {
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
        channel.load_more_messages(cx).unwrap().detach();
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
                    mentions: vec![],
                    reply_to_message_id: None,
                    edited_at: None,
                },
                proto::ChannelMessage {
                    id: 9,
                    body: "z".into(),
                    timestamp: 999,
                    sender_id: 6,
                    nonce: Some(5.into()),
                    mentions: vec![],
                    reply_to_message_id: None,
                    edited_at: None,
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
    channel.update(cx, |channel, _| {
        assert_eq!(
            channel
                .messages_in_range(0..2)
                .map(|message| (message.sender.github_login.clone(), message.body.clone()))
                .collect::<Vec<_>>(),
            &[
                ("user-5".into(), "y".into()),
                ("maxbrunsfeld".into(), "z".into())
            ]
        );
    });
}

fn init_test(cx: &mut App) -> Entity<ChannelStore> {
    let settings_store = SettingsStore::test(cx);
    cx.set_global(settings_store);
    release_channel::init(SemanticVersion::default(), cx);
    client::init_settings(cx);

    let clock = Arc::new(FakeSystemClock::new());
    let http = FakeHttpClient::with_404_response();
    let client = Client::new(clock, http.clone(), cx);
    let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));

    client::init(&client, cx);
    crate::init(&client, user_store, cx);

    ChannelStore::global(cx)
}

fn update_channels(
    channel_store: &Entity<ChannelStore>,
    message: proto::UpdateChannels,
    cx: &mut App,
) {
    let task = channel_store.update(cx, |store, cx| store.update_channels(message, cx));
    assert!(task.is_none());
}

#[track_caller]
fn assert_channels(
    channel_store: &Entity<ChannelStore>,
    expected_channels: &[(usize, String)],
    cx: &mut App,
) {
    let actual = channel_store.update(cx, |store, _| {
        store
            .ordered_channels()
            .map(|(depth, channel)| (depth, channel.name.to_string()))
            .collect::<Vec<_>>()
    });
    assert_eq!(actual, expected_channels);
}
