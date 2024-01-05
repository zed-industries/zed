use crate::tests::TestServer;
use call::ActiveCall;
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

    let channel_id = server
        .make_channel("the-channel", None, (&client_a, cx_a), &mut [])
        .await;

    client_a
        .channel_store()
        .update(cx_a, |channel_store, cx| {
            channel_store.set_channel_visibility(channel_id, proto::ChannelVisibility::Public, cx)
        })
        .await
        .unwrap();

    client_a
        .fs()
        .insert_tree(
            "/a",
            serde_json::json!({
                "a.txt": "a-contents",
            }),
        )
        .await;

    let active_call_a = cx_a.read(ActiveCall::global);

    // Client A shares a project in the channel
    active_call_a
        .update(cx_a, |call, cx| call.join_channel(channel_id, cx))
        .await
        .unwrap();
    let (project_a, _) = client_a.build_local_project("/a", cx_a).await;
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
    cx_a.executor().run_until_parked();

    // todo!() the test window does not call activation handlers
    // correctly yet, so this API does not work.
    // let project_b = active_call_b.read_with(cx_b, |call, _| {
    //     call.location()
    //         .unwrap()
    //         .upgrade()
    //         .expect("should not be weak")
    // });

    let window_b = cx_b.update(|cx| cx.active_window().unwrap());
    let cx_b = &mut VisualTestContext::from_window(window_b, cx_b);

    let workspace_b = window_b
        .downcast::<Workspace>()
        .unwrap()
        .root_view(cx_b)
        .unwrap();
    let project_b = workspace_b.update(cx_b, |workspace, _| workspace.project().clone());

    assert_eq!(
        project_b.read_with(cx_b, |project, _| project.remote_id()),
        Some(project_id),
    );
    assert!(project_b.read_with(cx_b, |project, _| project.is_read_only()))
}
