use crate::TestServer;
use collab_ui::CollabPanel;
use collab_ui::collab_panel::{MoveChannelDown, MoveChannelUp, ToggleSelectedChannelFavorite};
use gpui::TestAppContext;
use menu::{SelectNext, SelectPrevious};

#[gpui::test]
async fn test_reorder_favorite_channels_independently_of_channels(cx: &mut TestAppContext) {
    let (server, client) = TestServer::start1(cx).await;
    let root = server
        .make_channel("root", None, (&client, cx), &mut [])
        .await;
    let _ = server
        .make_channel("channel-a", Some(root), (&client, cx), &mut [])
        .await;
    let _ = server
        .make_channel("channel-b", Some(root), (&client, cx), &mut [])
        .await;
    let _ = server
        .make_channel("channel-c", Some(root), (&client, cx), &mut [])
        .await;

    let (workspace, cx) = client.build_test_workspace(cx).await;
    let panel = workspace.update_in(cx, |workspace, window, cx| {
        let panel = CollabPanel::new(workspace, window, cx);
        workspace.add_panel(panel.clone(), window, cx);
        panel
    });
    cx.run_until_parked();

    // Verify initial state.
    assert_eq!(
        panel.read_with(cx, |panel, _| panel.entries_as_strings()),
        &[
            "[Channels]",
            "  v root",
            "    #️⃣ channel-a",
            "    #️⃣ channel-b",
            "    #️⃣ channel-c",
            "[Contacts]",
        ]
    );

    // Select channel-b and favorite it.
    panel.update_in(cx, |panel, window, cx| {
        panel.select_next(&SelectNext, window, cx);
        panel.select_next(&SelectNext, window, cx);
        panel.select_next(&SelectNext, window, cx);
        panel.select_next(&SelectNext, window, cx);
    });
    assert_eq!(
        panel.read_with(cx, |panel, _| panel.entries_as_strings()),
        &[
            "[Channels]",
            "  v root",
            "    #️⃣ channel-a",
            "    #️⃣ channel-b  <== selected",
            "    #️⃣ channel-c",
            "[Contacts]",
        ]
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.toggle_selected_channel_favorite(&ToggleSelectedChannelFavorite, window, cx);
    });
    assert_eq!(
        panel.read_with(cx, |panel, _| panel.entries_as_strings()),
        &[
            "[Favorites]",
            "  #️⃣ channel-b",
            "[Channels]",
            "  v root",
            "    #️⃣ channel-a",
            "    #️⃣ channel-b  <== selected",
            "    #️⃣ channel-c",
            "[Contacts]",
        ]
    );

    // Select channel-c and favorite it.
    panel.update_in(cx, |panel, window, cx| {
        panel.select_next(&SelectNext, window, cx);
    });
    panel.update_in(cx, |panel, window, cx| {
        panel.toggle_selected_channel_favorite(&ToggleSelectedChannelFavorite, window, cx);
    });
    assert_eq!(
        panel.read_with(cx, |panel, _| panel.entries_as_strings()),
        &[
            "[Favorites]",
            "  #️⃣ channel-b",
            "  #️⃣ channel-c",
            "[Channels]",
            "  v root",
            "    #️⃣ channel-a",
            "    #️⃣ channel-b",
            "    #️⃣ channel-c  <== selected",
            "[Contacts]",
        ]
    );

    // Navigate up to favorite channel-b and move it down.
    // The Channels section should remain unchanged.
    panel.update_in(cx, |panel, window, cx| {
        panel.select_previous(&SelectPrevious, window, cx);
        panel.select_previous(&SelectPrevious, window, cx);
        panel.select_previous(&SelectPrevious, window, cx);
        panel.select_previous(&SelectPrevious, window, cx);
        panel.select_previous(&SelectPrevious, window, cx);
        panel.select_previous(&SelectPrevious, window, cx);
    });
    assert_eq!(
        panel.read_with(cx, |panel, _| panel.entries_as_strings()),
        &[
            "[Favorites]",
            "  #️⃣ channel-b  <== selected",
            "  #️⃣ channel-c",
            "[Channels]",
            "  v root",
            "    #️⃣ channel-a",
            "    #️⃣ channel-b",
            "    #️⃣ channel-c",
            "[Contacts]",
        ]
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.move_channel_down(&MoveChannelDown, window, cx);
    });
    assert_eq!(
        panel.read_with(cx, |panel, _| panel.entries_as_strings()),
        &[
            "[Favorites]",
            "  #️⃣ channel-c",
            "  #️⃣ channel-b  <== selected",
            "[Channels]",
            "  v root",
            "    #️⃣ channel-a",
            "    #️⃣ channel-b",
            "    #️⃣ channel-c",
            "[Contacts]",
        ]
    );

    // Move favorite channel-b back up.
    // The Channels section should remain unchanged.
    panel.update_in(cx, |panel, window, cx| {
        panel.move_channel_up(&MoveChannelUp, window, cx);
    });
    assert_eq!(
        panel.read_with(cx, |panel, _| panel.entries_as_strings()),
        &[
            "[Favorites]",
            "  #️⃣ channel-b  <== selected",
            "  #️⃣ channel-c",
            "[Channels]",
            "  v root",
            "    #️⃣ channel-a",
            "    #️⃣ channel-b",
            "    #️⃣ channel-c",
            "[Contacts]",
        ]
    );

    // Move favorite channel-b up when it's already first (should be no-op).
    panel.update_in(cx, |panel, window, cx| {
        panel.move_channel_up(&MoveChannelUp, window, cx);
    });
    assert_eq!(
        panel.read_with(cx, |panel, _| panel.entries_as_strings()),
        &[
            "[Favorites]",
            "  #️⃣ channel-b  <== selected",
            "  #️⃣ channel-c",
            "[Channels]",
            "  v root",
            "    #️⃣ channel-a",
            "    #️⃣ channel-b",
            "    #️⃣ channel-c",
            "[Contacts]",
        ]
    );

    // Unfavorite channel-b via action.
    // Selection should move to channel-b in the Channels section.
    panel.update_in(cx, |panel, window, cx| {
        panel.toggle_selected_channel_favorite(&ToggleSelectedChannelFavorite, window, cx);
    });
    assert_eq!(
        panel.read_with(cx, |panel, _| panel.entries_as_strings()),
        &[
            "[Favorites]",
            "  #️⃣ channel-c",
            "[Channels]",
            "  v root",
            "    #️⃣ channel-a",
            "    #️⃣ channel-b  <== selected",
            "    #️⃣ channel-c",
            "[Contacts]",
        ]
    );
}

#[gpui::test]
async fn test_reorder_channels_independently_of_favorites(cx: &mut TestAppContext) {
    let (server, client) = TestServer::start1(cx).await;
    let root = server
        .make_channel("root", None, (&client, cx), &mut [])
        .await;
    let _ = server
        .make_channel("channel-a", Some(root), (&client, cx), &mut [])
        .await;
    let _ = server
        .make_channel("channel-b", Some(root), (&client, cx), &mut [])
        .await;
    let _ = server
        .make_channel("channel-c", Some(root), (&client, cx), &mut [])
        .await;

    let (workspace, cx) = client.build_test_workspace(cx).await;
    let panel = workspace.update_in(cx, |workspace, window, cx| {
        let panel = CollabPanel::new(workspace, window, cx);
        workspace.add_panel(panel.clone(), window, cx);
        panel
    });
    cx.run_until_parked();

    // Select channel-a and channel-b, favorite them via action.
    panel.update_in(cx, |panel, window, cx| {
        panel.select_next(&SelectNext, window, cx);
        panel.select_next(&SelectNext, window, cx);
        panel.select_next(&SelectNext, window, cx);
    });
    assert_eq!(
        panel.read_with(cx, |panel, _| panel.entries_as_strings()),
        &[
            "[Channels]",
            "  v root",
            "    #️⃣ channel-a  <== selected",
            "    #️⃣ channel-b",
            "    #️⃣ channel-c",
            "[Contacts]",
        ]
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.toggle_selected_channel_favorite(&ToggleSelectedChannelFavorite, window, cx);
    });
    panel.update_in(cx, |panel, window, cx| {
        panel.select_next(&SelectNext, window, cx);
        panel.toggle_selected_channel_favorite(&ToggleSelectedChannelFavorite, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        panel.read_with(cx, |panel, _| panel.entries_as_strings()),
        &[
            "[Favorites]",
            "  #️⃣ channel-a",
            "  #️⃣ channel-b",
            "[Channels]",
            "  v root",
            "    #️⃣ channel-a",
            "    #️⃣ channel-b  <== selected",
            "    #️⃣ channel-c",
            "[Contacts]",
        ]
    );

    // Select channel-a in the Channels section and move it down.
    // The Favorites section should remain unchanged.
    panel.update_in(cx, |panel, window, cx| {
        panel.select_previous(&SelectPrevious, window, cx);
    });
    assert_eq!(
        panel.read_with(cx, |panel, _| panel.entries_as_strings()),
        &[
            "[Favorites]",
            "  #️⃣ channel-a",
            "  #️⃣ channel-b",
            "[Channels]",
            "  v root",
            "    #️⃣ channel-a  <== selected",
            "    #️⃣ channel-b",
            "    #️⃣ channel-c",
            "[Contacts]",
        ]
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.move_channel_down(&MoveChannelDown, window, cx);
    });
    cx.run_until_parked();

    // Channels section reflects the reorder; favorites stay the same.
    // Selection should remain on channel-a in the Channels section,
    // not jump to channel-a in Favorites.
    assert_eq!(
        panel.read_with(cx, |panel, _| panel.entries_as_strings()),
        &[
            "[Favorites]",
            "  #️⃣ channel-a",
            "  #️⃣ channel-b",
            "[Channels]",
            "  v root",
            "    #️⃣ channel-b",
            "    #️⃣ channel-a  <== selected",
            "    #️⃣ channel-c",
            "[Contacts]",
        ]
    );
}
