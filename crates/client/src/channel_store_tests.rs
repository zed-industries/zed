use super::*;
use util::http::FakeHttpClient;

#[gpui::test]
fn test_update_channels(cx: &mut AppContext) {
    let http = FakeHttpClient::with_404_response();
    let client = Client::new(http.clone(), cx);
    let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http, cx));

    let channel_store = cx.add_model(|cx| ChannelStore::new(client, user_store, cx));

    update_channels(
        &channel_store,
        proto::UpdateChannels {
            channels: vec![
                proto::Channel {
                    id: 1,
                    name: "b".to_string(),
                    parent_id: None,
                },
                proto::Channel {
                    id: 2,
                    name: "a".to_string(),
                    parent_id: None,
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
                    parent_id: Some(1),
                },
                proto::Channel {
                    id: 4,
                    name: "y".to_string(),
                    parent_id: Some(2),
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
    let http = FakeHttpClient::with_404_response();
    let client = Client::new(http.clone(), cx);
    let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http, cx));

    let channel_store = cx.add_model(|cx| ChannelStore::new(client, user_store, cx));

    update_channels(
        &channel_store,
        proto::UpdateChannels {
            channels: vec![
                proto::Channel {
                    id: 0,
                    name: "a".to_string(),
                    parent_id: None,
                },
                proto::Channel {
                    id: 1,
                    name: "b".to_string(),
                    parent_id: Some(0),
                },
                proto::Channel {
                    id: 2,
                    name: "c".to_string(),
                    parent_id: Some(1),
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
            remove_channels: vec![1, 2],
            ..Default::default()
        },
        cx,
    );

    // Make sure that the 1/2/3 path is gone
    assert_channels(&channel_store, &[(0, "a".to_string(), true)], cx);
}

fn update_channels(
    channel_store: &ModelHandle<ChannelStore>,
    message: proto::UpdateChannels,
    cx: &mut AppContext,
) {
    channel_store.update(cx, |store, cx| store.update_channels(message, cx));
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
