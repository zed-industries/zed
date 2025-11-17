use super::*;
use client::{Client, UserStore};
use clock::FakeSystemClock;
use gpui::{App, AppContext as _, Entity, SemanticVersion};
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

fn init_test(cx: &mut App) -> Entity<ChannelStore> {
    let settings_store = SettingsStore::test(cx);
    cx.set_global(settings_store);
    release_channel::init(SemanticVersion::default(), cx);

    let clock = Arc::new(FakeSystemClock::new());
    let http = FakeHttpClient::with_404_response();
    let client = Client::new(clock, http, cx);
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
