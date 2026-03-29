use crate::TestServer;
use channel::ChannelStore;
use collab_ui::CollabPanel;
use gpui::TestAppContext;
use rpc::proto;

#[gpui::test]
async fn test_favorite_channels(cx: &mut TestAppContext) {
    let (server, client) = TestServer::start1(cx).await;
    let _channel_a = server
        .make_channel("channel-a", None, (&client, cx), &mut [])
        .await;
    let channel_b = server
        .make_channel("channel-b", None, (&client, cx), &mut [])
        .await;
    let channel_c = server
        .make_channel("channel-c", None, (&client, cx), &mut [])
        .await;

    let (workspace, cx) = client.build_test_workspace(cx).await;
    let panel = workspace.update_in(cx, |workspace, window, cx| {
        let panel = CollabPanel::new(workspace, window, cx);
        workspace.add_panel(panel.clone(), window, cx);
        panel
    });
    cx.run_until_parked();

    // Verify initial state: just channels, no favorites.
    assert_eq!(
        panel.read_with(cx, |panel, _| panel.entries_as_strings()),
        &[
            "[Channels]",
            "  #️⃣ channel-a",
            "  #️⃣ channel-b",
            "  #️⃣ channel-c",
            "[Contacts]",
        ]
    );

    // Favorite channel-b and channel-c.
    panel.update(cx, |panel, cx| {
        panel.toggle_favorite_channel(channel_b, cx);
        panel.toggle_favorite_channel(channel_c, cx);
    });
    assert_eq!(
        panel.read_with(cx, |panel, _| panel.entries_as_strings()),
        &[
            "[Favorites]",
            "  #️⃣ channel-b",
            "  #️⃣ channel-c",
            "[Channels]",
            "  #️⃣ channel-a",
            "  #️⃣ channel-b",
            "  #️⃣ channel-c",
            "[Contacts]",
        ]
    );

    // Reorder favorites: move channel-b down (should swap with channel-c).
    // The Channels section should remain unchanged.
    panel.update(cx, |panel, cx| {
        panel.reorder_favorite(channel_b, proto::reorder_channel::Direction::Down, cx);
    });
    assert_eq!(
        panel.read_with(cx, |panel, _| panel.entries_as_strings()),
        &[
            "[Favorites]",
            "  #️⃣ channel-c",
            "  #️⃣ channel-b",
            "[Channels]",
            "  #️⃣ channel-a",
            "  #️⃣ channel-b",
            "  #️⃣ channel-c",
            "[Contacts]",
        ]
    );

    // Reorder favorites: move channel-b up (should swap back with channel-c).
    // The Channels section should remain unchanged.
    panel.update(cx, |panel, cx| {
        panel.reorder_favorite(channel_b, proto::reorder_channel::Direction::Up, cx);
    });
    assert_eq!(
        panel.read_with(cx, |panel, _| panel.entries_as_strings()),
        &[
            "[Favorites]",
            "  #️⃣ channel-b",
            "  #️⃣ channel-c",
            "[Channels]",
            "  #️⃣ channel-a",
            "  #️⃣ channel-b",
            "  #️⃣ channel-c",
            "[Contacts]",
        ]
    );

    // Reorder favorites: move channel-b up when it's already first (should be no-op).
    panel.update(cx, |panel, cx| {
        panel.reorder_favorite(channel_b, proto::reorder_channel::Direction::Up, cx);
    });
    assert_eq!(
        panel.read_with(cx, |panel, _| panel.entries_as_strings()),
        &[
            "[Favorites]",
            "  #️⃣ channel-b",
            "  #️⃣ channel-c",
            "[Channels]",
            "  #️⃣ channel-a",
            "  #️⃣ channel-b",
            "  #️⃣ channel-c",
            "[Contacts]",
        ]
    );

    // Unfavorite channel-b.
    panel.update(cx, |panel, cx| {
        panel.toggle_favorite_channel(channel_b, cx);
    });
    assert_eq!(
        panel.read_with(cx, |panel, _| panel.entries_as_strings()),
        &[
            "[Favorites]",
            "  #️⃣ channel-c",
            "[Channels]",
            "  #️⃣ channel-a",
            "  #️⃣ channel-b",
            "  #️⃣ channel-c",
            "[Contacts]",
        ]
    );

    // Unfavorite channel-c: favorites section should disappear.
    panel.update(cx, |panel, cx| {
        panel.toggle_favorite_channel(channel_c, cx);
    });
    assert_eq!(
        panel.read_with(cx, |panel, _| panel.entries_as_strings()),
        &[
            "[Channels]",
            "  #️⃣ channel-a",
            "  #️⃣ channel-b",
            "  #️⃣ channel-c",
            "[Contacts]",
        ]
    );
}

#[gpui::test]
async fn test_reorder_channels_does_not_affect_favorites(cx: &mut TestAppContext) {
    let (server, client) = TestServer::start1(cx).await;
    let root = server
        .make_channel("root", None, (&client, cx), &mut [])
        .await;
    let channel_a = server
        .make_channel("channel-a", Some(root), (&client, cx), &mut [])
        .await;
    let channel_b = server
        .make_channel("channel-b", Some(root), (&client, cx), &mut [])
        .await;
    let _channel_c = server
        .make_channel("channel-c", Some(root), (&client, cx), &mut [])
        .await;

    let (workspace, cx) = client.build_test_workspace(cx).await;
    let panel = workspace.update_in(cx, |workspace, window, cx| {
        let panel = CollabPanel::new(workspace, window, cx);
        workspace.add_panel(panel.clone(), window, cx);
        panel
    });
    cx.run_until_parked();

    // Favorite channel-a and channel-b (in that order).
    panel.update(cx, |panel, cx| {
        panel.toggle_favorite_channel(channel_a, cx);
        panel.toggle_favorite_channel(channel_b, cx);
    });
    assert_eq!(
        panel.read_with(cx, |panel, _| panel.entries_as_strings()),
        &[
            "[Favorites]",
            "  #️⃣ channel-a",
            "  #️⃣ channel-b",
            "[Channels]",
            "  v root",
            "    #️⃣ channel-a",
            "    #️⃣ channel-b",
            "    #️⃣ channel-c",
            "[Contacts]",
        ]
    );

    // Reorder the real channels: move channel-a down (swaps with channel-b).
    // The Favorites section should remain unchanged.
    cx.read(ChannelStore::global)
        .update(cx, |store, cx| {
            store.reorder_channel(channel_a, proto::reorder_channel::Direction::Down, cx)
        })
        .await
        .unwrap();
    cx.run_until_parked();

    // Channels section should reflect the reorder, but favorites stay the same.
    assert_eq!(
        panel.read_with(cx, |panel, _| panel.entries_as_strings()),
        &[
            "[Favorites]",
            "  #️⃣ channel-a",
            "  #️⃣ channel-b",
            "[Channels]",
            "  v root",
            "    #️⃣ channel-b",
            "    #️⃣ channel-a",
            "    #️⃣ channel-c",
            "[Contacts]",
        ]
    );
}
