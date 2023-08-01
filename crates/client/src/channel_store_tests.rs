use util::http::FakeHttpClient;

use super::*;

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
            ..Default::default()
        },
        cx,
    );
    assert_channels(
        &channel_store,
        &[
            //
            (0, "a"),
            (0, "b"),
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
            //
            (0, "a"),
            (1, "y"),
            (0, "b"),
            (1, "x"),
        ],
        cx,
    );
}

fn update_channels(
    channel_store: &ModelHandle<ChannelStore>,
    message: proto::UpdateChannels,
    cx: &mut AppContext,
) {
    channel_store.update(cx, |store, cx| store.update_channels(message, cx));
}

fn assert_channels(
    channel_store: &ModelHandle<ChannelStore>,
    expected_channels: &[(usize, &str)],
    cx: &AppContext,
) {
    channel_store.read_with(cx, |store, _| {
        let actual = store
            .channels()
            .iter()
            .map(|c| (c.depth, c.name.as_str()))
            .collect::<Vec<_>>();
        assert_eq!(actual, expected_channels);
    });
}
