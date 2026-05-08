use crate::TestServer;
use call::ActiveCall;
use client::ChannelId;
use gpui::{App, BackgroundExecutor, Entity, TestAppContext, TestScreenCaptureSource};
use project::Project;
use rpc::proto::PeerId;
use workspace::{AutoWatch, SharedScreen, Workspace};

use super::TestClient;

struct AutoWatchTestSetup {
    client_a: TestClient,
    client_b: TestClient,
    client_c: TestClient,
    channel_id: ChannelId,
    user_a_project: Entity<Project>,
    user_b_project: Entity<Project>,
}

async fn setup_auto_watch_test(
    server: &mut TestServer,
    user_a: &mut TestAppContext,
    user_b: &mut TestAppContext,
    user_c: &mut TestAppContext,
) -> AutoWatchTestSetup {
    setup_auto_watch_test_with_initial_participants(server, user_a, user_b, user_c, true).await
}

async fn setup_auto_watch_late_joiner_test(
    server: &mut TestServer,
    user_a: &mut TestAppContext,
    user_b: &mut TestAppContext,
    user_c: &mut TestAppContext,
) -> AutoWatchTestSetup {
    setup_auto_watch_test_with_initial_participants(server, user_a, user_b, user_c, false).await
}

async fn setup_auto_watch_test_with_initial_participants(
    server: &mut TestServer,
    user_a: &mut TestAppContext,
    user_b: &mut TestAppContext,
    user_c: &mut TestAppContext,
    join_user_c: bool,
) -> AutoWatchTestSetup {
    let client_a = server.create_client(user_a, "user_a").await;
    let client_b = server.create_client(user_b, "user_b").await;
    let client_c = server.create_client(user_c, "user_c").await;
    let channel_id = server
        .make_channel(
            "the-channel",
            None,
            (&client_a, user_a),
            &mut [(&client_b, user_b), (&client_c, user_c)],
        )
        .await;

    let user_a_project = client_a.build_empty_local_project(false, user_a);
    let user_b_project = client_b.build_empty_local_project(false, user_b);

    let active_call_a = user_a.read(ActiveCall::global);
    active_call_a
        .update(user_a, |call, cx| call.join_channel(channel_id, cx))
        .await
        .unwrap();
    let active_call_b = user_b.read(ActiveCall::global);
    active_call_b
        .update(user_b, |call, cx| call.join_channel(channel_id, cx))
        .await
        .unwrap();

    if join_user_c {
        let active_call_c = user_c.read(ActiveCall::global);
        active_call_c
            .update(user_c, |call, cx| call.join_channel(channel_id, cx))
            .await
            .unwrap();
    }

    AutoWatchTestSetup {
        client_a,
        client_b,
        client_c,
        channel_id,
        user_a_project,
        user_b_project,
    }
}

#[gpui::test]
async fn test_auto_watch_opens_existing_share_on_toggle(
    executor: BackgroundExecutor,
    user_a: &mut TestAppContext,
    user_b: &mut TestAppContext,
    user_c: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let setup = setup_auto_watch_test(&mut server, user_a, user_b, user_c).await;
    let (workspace_a, user_a) = setup
        .client_a
        .build_workspace(&setup.user_a_project, user_a);
    executor.run_until_parked();

    start_screen_share(user_b).await;
    executor.run_until_parked();

    workspace_a.update_in(user_a, |workspace, window, cx| {
        workspace.toggle_auto_watch(window, cx);
    });
    executor.run_until_parked();

    workspace_a.update(user_a, |workspace, cx| {
        assert_active_item_is_screen_share_for_peer(
            workspace,
            setup.client_b.peer_id().unwrap(),
            cx,
        );
    });
}

#[gpui::test]
async fn test_auto_watch_opens_share_when_no_one_is_sharing_yet(
    executor: BackgroundExecutor,
    user_a: &mut TestAppContext,
    user_b: &mut TestAppContext,
    user_c: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let setup = setup_auto_watch_test(&mut server, user_a, user_b, user_c).await;
    let (workspace_a, user_a) = setup
        .client_a
        .build_workspace(&setup.user_a_project, user_a);

    workspace_a.update_in(user_a, |workspace, window, cx| {
        workspace.toggle_auto_watch(window, cx);
    });

    start_screen_share(user_b).await;
    executor.run_until_parked();

    workspace_a.update(user_a, |workspace, cx| {
        assert_active_item_is_screen_share_for_peer(
            workspace,
            setup.client_b.peer_id().unwrap(),
            cx,
        );
    });
}

#[gpui::test]
async fn test_auto_watch_switches_to_next_share_on_share_end(
    executor: BackgroundExecutor,
    user_a: &mut TestAppContext,
    user_b: &mut TestAppContext,
    user_c: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let setup = setup_auto_watch_test(&mut server, user_a, user_b, user_c).await;
    let (workspace_a, user_a) = setup
        .client_a
        .build_workspace(&setup.user_a_project, user_a);

    workspace_a.update_in(user_a, |workspace, window, cx| {
        workspace.toggle_auto_watch(window, cx);
    });

    start_screen_share(user_b).await;
    executor.run_until_parked();

    workspace_a.update(user_a, |workspace, cx| {
        assert_active_item_is_screen_share_for_peer(
            workspace,
            setup.client_b.peer_id().unwrap(),
            cx,
        );
    });

    start_screen_share(user_c).await;
    executor.run_until_parked();

    stop_screen_share(user_b);
    executor.run_until_parked();

    workspace_a.update(user_a, |workspace, cx| {
        assert_active_item_is_screen_share_for_peer(
            workspace,
            setup.client_c.peer_id().unwrap(),
            cx,
        );
    });
}

#[gpui::test]
async fn test_auto_watch_ignores_shares_while_user_is_sharing(
    executor: BackgroundExecutor,
    user_a: &mut TestAppContext,
    user_b: &mut TestAppContext,
    user_c: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let setup = setup_auto_watch_test(&mut server, user_a, user_b, user_c).await;
    let (workspace_a, user_a) = setup
        .client_a
        .build_workspace(&setup.user_a_project, user_a);

    start_screen_share(user_a).await;
    executor.run_until_parked();
    start_screen_share(user_b).await;
    executor.run_until_parked();

    // Should NOT open B's screen cause we are sharing
    workspace_a.update_in(user_a, |workspace, window, cx| {
        workspace.toggle_auto_watch(window, cx);
    });
    executor.run_until_parked();

    workspace_a.update(user_a, |workspace, cx| {
        assert_no_screen_share_tabs_exist(
            workspace,
            "should not open anyone's screen share when toggling on while sharing",
            cx,
        );
    });
}

#[gpui::test]
async fn test_auto_watch_opens_share_after_local_user_stops_sharing(
    executor: BackgroundExecutor,
    user_a: &mut TestAppContext,
    user_b: &mut TestAppContext,
    user_c: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let setup = setup_auto_watch_test(&mut server, user_a, user_b, user_c).await;
    let (workspace_a, user_a) = setup
        .client_a
        .build_workspace(&setup.user_a_project, user_a);

    workspace_a.update_in(user_a, |workspace, window, cx| {
        workspace.toggle_auto_watch(window, cx);
    });
    start_screen_share(user_a).await;
    executor.run_until_parked();

    start_screen_share(user_b).await;
    executor.run_until_parked();

    stop_screen_share(user_a);
    executor.run_until_parked();

    workspace_a.update(user_a, |workspace, cx| {
        assert_active_item_is_screen_share_for_peer(
            workspace,
            setup.client_b.peer_id().unwrap(),
            cx,
        );
    });
}

#[gpui::test]
async fn test_auto_watch_toggle_off_leaves_tabs_open(
    executor: BackgroundExecutor,
    user_a: &mut TestAppContext,
    user_b: &mut TestAppContext,
    user_c: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let setup = setup_auto_watch_test(&mut server, user_a, user_b, user_c).await;
    let (workspace_a, user_a) = setup
        .client_a
        .build_workspace(&setup.user_a_project, user_a);

    workspace_a.update_in(user_a, |workspace, window, cx| {
        workspace.toggle_auto_watch(window, cx);
    });
    start_screen_share(user_b).await;
    executor.run_until_parked();

    workspace_a.update(user_a, |workspace, cx| {
        assert_active_item_is_screen_share_for_peer(
            workspace,
            setup.client_b.peer_id().unwrap(),
            cx,
        );
    });

    workspace_a.update_in(user_a, |workspace, window, cx| {
        workspace.toggle_auto_watch(window, cx);
    });

    workspace_a.update(user_a, |workspace, cx| {
        assert_active_item_is_screen_share_for_peer(
            workspace,
            setup.client_b.peer_id().unwrap(),
            cx,
        );
    });
}

#[gpui::test]
async fn test_auto_watch_reopens_screen_share_from_returning_channel_participant(
    executor: BackgroundExecutor,
    user_a: &mut TestAppContext,
    user_b: &mut TestAppContext,
    user_c: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let setup = setup_auto_watch_late_joiner_test(&mut server, user_a, user_b, user_c).await;
    let (workspace_a, user_a) = setup
        .client_a
        .build_workspace(&setup.user_a_project, user_a);
    let (workspace_b, user_b) = setup
        .client_b
        .build_workspace(&setup.user_b_project, user_b);

    workspace_a.update_in(user_a, |workspace, window, cx| {
        workspace.toggle_auto_watch(window, cx);
    });
    workspace_b.update_in(user_b, |workspace, window, cx| {
        workspace.toggle_auto_watch(window, cx);
    });
    executor.run_until_parked();

    let active_call_c = user_c.read(ActiveCall::global);
    active_call_c
        .update(user_c, |call, cx| call.join_channel(setup.channel_id, cx))
        .await
        .unwrap();
    executor.run_until_parked();

    start_screen_share(user_c).await;
    executor.run_until_parked();

    workspace_a.update(user_a, |workspace, cx| {
        assert_active_item_is_screen_share_for_peer(
            workspace,
            setup.client_c.peer_id().unwrap(),
            cx,
        );
    });
    workspace_b.update(user_b, |workspace, cx| {
        assert_active_item_is_screen_share_for_peer(
            workspace,
            setup.client_c.peer_id().unwrap(),
            cx,
        );
    });

    active_call_c
        .update(user_c, |call, cx| call.hang_up(cx))
        .await
        .unwrap();
    executor.run_until_parked();

    workspace_a.update(user_a, |workspace, cx| {
        assert_no_screen_share_tabs_exist(
            workspace,
            "user A should stop seeing user C's screen after user C hangs up",
            cx,
        );
    });
    workspace_b.update(user_b, |workspace, cx| {
        assert_no_screen_share_tabs_exist(
            workspace,
            "user B should stop seeing user C's screen after user C hangs up",
            cx,
        );
    });

    let active_call_c = user_c.read(ActiveCall::global);
    active_call_c
        .update(user_c, |call, cx| call.join_channel(setup.channel_id, cx))
        .await
        .unwrap();
    executor.run_until_parked();

    start_screen_share(user_c).await;
    executor.run_until_parked();

    workspace_a.update(user_a, |workspace, cx| {
        assert_active_item_is_screen_share_for_peer(
            workspace,
            setup.client_c.peer_id().unwrap(),
            cx,
        );
    });
    workspace_b.update(user_b, |workspace, cx| {
        assert_active_item_is_screen_share_for_peer(
            workspace,
            setup.client_c.peer_id().unwrap(),
            cx,
        );
    });
}

#[gpui::test]
async fn test_auto_watch_is_disabled_when_following_collaborator(
    executor: BackgroundExecutor,
    user_a: &mut TestAppContext,
    user_b: &mut TestAppContext,
    user_c: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let setup = setup_auto_watch_test(&mut server, user_a, user_b, user_c).await;
    let (workspace_a, user_a) = setup
        .client_a
        .build_workspace(&setup.user_a_project, user_a);
    let user_b_peer_id = setup.client_b.peer_id().unwrap();

    workspace_a.update_in(user_a, |workspace, window, cx| {
        workspace.toggle_auto_watch(window, cx);
    });
    start_screen_share(user_b).await;
    executor.run_until_parked();

    workspace_a.update(user_a, |workspace, cx| {
        assert_active_item_is_screen_share_for_peer(
            workspace,
            setup.client_b.peer_id().unwrap(),
            cx,
        );
    });

    workspace_a.update_in(user_a, |workspace, window, cx| {
        workspace.follow(user_b_peer_id, window, cx);
    });
    executor.run_until_parked();

    workspace_a.update(user_a, |workspace, _cx| {
        assert_eq!(*workspace.auto_watch_state(), AutoWatch::Off);
    });
}

#[track_caller]
fn assert_no_screen_share_tabs_exist(workspace: &Workspace, message: &str, cx: &App) {
    let has_shared_screen_tab = workspace
        .active_pane()
        .read(cx)
        .items()
        .any(|item| item.downcast::<SharedScreen>().is_some());
    assert!(!has_shared_screen_tab, "{message}");
}

#[track_caller]
fn assert_active_item_is_screen_share_for_peer(workspace: &Workspace, peer_id: PeerId, cx: &App) {
    let active_item = workspace.active_item(cx).expect("no active item");
    let shared_screen = active_item
        .downcast::<SharedScreen>()
        .expect("expected active item to be a shared screen");
    assert_eq!(shared_screen.read(cx).peer_id, peer_id);
}

async fn start_screen_share(cx: &mut TestAppContext) {
    let display = TestScreenCaptureSource::new();
    cx.set_screen_capture_sources(vec![display]);
    let screen = cx
        .update(|cx| cx.screen_capture_sources())
        .await
        .unwrap()
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    let active_call = cx.read(ActiveCall::global);
    active_call
        .update(cx, |call, cx| {
            call.room()
                .unwrap()
                .update(cx, |room, cx| room.share_screen(screen, cx))
        })
        .await
        .unwrap();
}

#[track_caller]
fn stop_screen_share(cx: &mut TestAppContext) {
    let active_call = cx.read(ActiveCall::global);
    active_call
        .update(cx, |call, cx| {
            call.room()
                .unwrap()
                .update(cx, |room, cx| room.unshare_screen(true, cx))
        })
        .unwrap();
}
