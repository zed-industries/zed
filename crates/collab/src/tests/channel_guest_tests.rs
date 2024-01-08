use crate::tests::TestServer;
use call::ActiveCall;
use editor::Editor;
use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext};
use rpc::proto;
use workspace::Workspace;

#[gpui::test]
async fn test_channel_guests(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let active_call_a = cx_a.read(ActiveCall::global);

    let channel_id = server
        .make_public_channel("the-channel", &client_a, cx_a)
        .await;

    // Client A shares a project in the channel
    let project_a = client_a.build_test_project(cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.join_channel(channel_id, cx))
        .await
        .unwrap();
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    cx_a.executor().run_until_parked();

    // Client B joins channel A as a guest
    cx_b.update(|cx| workspace::join_channel(channel_id, client_b.app_state.clone(), None, cx))
        .await
        .unwrap();

    // b should be following a in the shared project.
    // B is a guest,
    executor.run_until_parked();

    let active_call_b = cx_b.read(ActiveCall::global);
    let project_b =
        active_call_b.read_with(cx_b, |call, _| call.location().unwrap().upgrade().unwrap());
    assert_eq!(
        project_b.read_with(cx_b, |project, _| project.remote_id()),
        Some(project_id),
    );
    assert!(project_b.read_with(cx_b, |project, _| project.is_read_only()));
    assert!(project_b
        .update(cx_b, |project, cx| {
            let worktree_id = project.worktrees().next().unwrap().read(cx).id();
            project.create_entry((worktree_id, "b.txt"), false, cx)
        })
        .await
        .is_err())
}

#[gpui::test]
async fn test_channel_guest_promotion(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    let mut server = TestServer::start(cx_a.executor()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    let channel_id = server
        .make_public_channel("the-channel", &client_a, cx_a)
        .await;

    let project_a = client_a.build_test_project(cx_a).await;
    cx_a.update(|cx| workspace::join_channel(channel_id, client_a.app_state.clone(), None, cx))
        .await
        .unwrap();

    // Client A shares a project in the channel
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    cx_a.run_until_parked();

    // Client B joins channel A as a guest
    cx_b.update(|cx| workspace::join_channel(channel_id, client_b.app_state.clone(), None, cx))
        .await
        .unwrap();
    cx_a.run_until_parked();

    let project_b =
        active_call_b.read_with(cx_b, |call, _| call.location().unwrap().upgrade().unwrap());

    // client B opens 1.txt
    let (workspace, cx_b) = client_b.active_workspace(cx_b);
    cx_b.simulate_keystrokes("cmd-p 1 enter");
    let editor_b = cx_b.update(|cx| workspace.read(cx).active_item_as::<Editor>(cx).unwrap());

    active_call_a
        .update(cx_a, |call, cx| {
            call.room().unwrap().update(cx, |room, cx| {
                room.set_participant_role(
                    client_b.user_id().unwrap(),
                    proto::ChannelRole::Member,
                    cx,
                )
            })
        })
        .await
        .unwrap();

    cx_a.run_until_parked();

    assert!(project_b.read_with(cx_b, |project, _| !project.is_read_only()));
    assert!(!editor_b.update(cx_b, |e, cx| e.read_only(cx)));
}
