use call::ActiveCall;
use editor::Editor;
use gpui::TestAppContext;
use serde_json::json;

use super::TestServer;

#[gpui::test]
async fn test_debug_panel_following(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    let executor = cx_a.executor();
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    cx_a.update(editor::init);
    cx_b.update(editor::init);

    client_a
        .fs()
        .insert_tree(
            "/a",
            // TODO: Make these good files for debugging
            json!({
                "test.txt": "one\ntwo\nthree",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();

    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();

    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);

    // Client A opens an editor.
    let _pane_a = workspace_a.update(cx_a, |workspace, _| workspace.active_pane().clone());
    let editor_a = workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.open_path((worktree_id, "test.txt"), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    let peer_id_a = client_a.peer_id().unwrap();

    // Client B follows A
    workspace_b.update(cx_b, |workspace, cx| workspace.follow(peer_id_a, cx));

    let _editor_b2 = workspace_b.update(cx_b, |workspace, cx| {
        workspace
            .active_item(cx)
            .unwrap()
            .downcast::<Editor>()
            .unwrap()
    });

    // Start a fake debugging session in a (see: other tests which setup fake language servers for a model)
    // Add a breakpoint
    editor_a.update(cx_a, |editor, cx| {
        editor.move_down(&editor::actions::MoveDown, cx);
        editor.select_right(&editor::actions::SelectRight, cx);
        editor.toggle_breakpoint(&editor::actions::ToggleBreakpoint, cx);
    });

    // Start debugging

    // TODO:
    // 2. Sanity check: make sure a looks right
    // 3. Check that b looks right
}
