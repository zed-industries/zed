use crate::TestServer;
use call::ActiveCall;
use gpui::{App, BackgroundExecutor, Entity, TestAppContext, TestScreenCaptureSource};
use project::Project;
use serde_json::json;
use util::path;
use workspace::Workspace;

use super::TestClient;

struct AutoWatchTestSetup {
    client_a: TestClient,
    _client_b: TestClient,
    _client_c: TestClient,
    project_a: Entity<Project>,
}

async fn setup_auto_watch_test(
    server: &mut TestServer,
    user_a: &mut TestAppContext,
    user_b: &mut TestAppContext,
    user_c: &mut TestAppContext,
) -> AutoWatchTestSetup {
    let client_a = server.create_client(user_a, "user_a").await;
    let client_b = server.create_client(user_b, "user_b").await;
    let client_c = server.create_client(user_c, "user_c").await;
    server
        .create_room(&mut [
            (&client_a, user_a),
            (&client_b, user_b),
            (&client_c, user_c),
        ])
        .await;

    let active_call_a = user_a.read(ActiveCall::global);

    client_a
        .fs()
        .insert_tree(path!("/a"), json!({ "file.txt": "content" }))
        .await;
    let (project_a, _worktree_id) = client_a.build_local_project(path!("/a"), user_a).await;
    active_call_a
        .update(user_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();

    AutoWatchTestSetup {
        client_a,
        _client_b: client_b,
        _client_c: client_c,
        project_a,
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
    let (workspace_a, user_a) = setup.client_a.build_workspace(&setup.project_a, user_a);
    executor.run_until_parked();

    start_screen_share(user_b).await;
    executor.run_until_parked();

    workspace_a.update_in(user_a, |workspace, window, cx| {
        workspace.toggle_auto_watch(window, cx);
    });
    executor.run_until_parked();

    workspace_a.update(user_a, |workspace, cx| {
        assert_active_matches_title(workspace, "user_b's screen", cx);
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
    let (workspace_a, user_a) = setup.client_a.build_workspace(&setup.project_a, user_a);

    workspace_a.update_in(user_a, |workspace, window, cx| {
        workspace.toggle_auto_watch(window, cx);
    });

    start_screen_share(user_b).await;
    executor.run_until_parked();

    workspace_a.update(user_a, |workspace, cx| {
        assert_active_matches_title(workspace, "user_b's screen", cx);
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
    let (workspace_a, user_a) = setup.client_a.build_workspace(&setup.project_a, user_a);

    workspace_a.update_in(user_a, |workspace, window, cx| {
        workspace.toggle_auto_watch(window, cx);
    });

    start_screen_share(user_b).await;
    executor.run_until_parked();

    workspace_a.update(user_a, |workspace, cx| {
        assert_active_matches_title(workspace, "user_b's screen", cx);
    });

    start_screen_share(user_c).await;
    executor.run_until_parked();

    stop_screen_share(user_b);
    executor.run_until_parked();

    workspace_a.update(user_a, |workspace, cx| {
        assert_active_matches_title(workspace, "user_c's screen", cx);
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
    let (workspace_a, user_a) = setup.client_a.build_workspace(&setup.project_a, user_a);

    start_screen_share(user_a).await;
    executor.run_until_parked();
    start_screen_share(user_b).await;
    executor.run_until_parked();

    // Should NOT open B's screen cause we are sharing
    workspace_a.update_in(user_a, |workspace, window, cx| {
        workspace.toggle_auto_watch(window, cx);
    });
    executor.run_until_parked();

    // Ensure that no screen share is found in user a's tab bar
    workspace_a.update(user_a, |workspace, cx| {
        let has_shared_screen_tab = workspace
            .active_pane()
            .read(cx)
            .items()
            .any(|item| item.tab_content_text(0, cx).contains("screen"));
        assert!(
            !has_shared_screen_tab,
            "should not open anyone's screen share when toggling on while sharing"
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
    let (workspace_a, user_a) = setup.client_a.build_workspace(&setup.project_a, user_a);

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
        assert_active_matches_title(workspace, "user_b's screen", cx);
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
    let (workspace_a, user_a) = setup.client_a.build_workspace(&setup.project_a, user_a);

    workspace_a.update_in(user_a, |workspace, window, cx| {
        workspace.toggle_auto_watch(window, cx);
    });
    start_screen_share(user_b).await;
    executor.run_until_parked();

    workspace_a.update(user_a, |workspace, cx| {
        assert_active_matches_title(workspace, "user_b's screen", cx);
    });

    workspace_a.update_in(user_a, |workspace, window, cx| {
        workspace.toggle_auto_watch(window, cx);
    });

    workspace_a.update(user_a, |workspace, cx| {
        assert_active_matches_title(workspace, "user_b's screen", cx);
    });
}

#[track_caller]
fn assert_active_matches_title(workspace: &Workspace, expected_title: &str, cx: &App) {
    let active_item = workspace.active_item(cx).expect("no active item");
    assert_eq!(
        active_item.tab_content_text(0, cx),
        expected_title,
        "expected active item to be '{}'",
        expected_title
    );
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
