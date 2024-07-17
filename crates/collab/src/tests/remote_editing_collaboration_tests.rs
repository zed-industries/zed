use crate::tests::TestServer;
use call::ActiveCall;
use fs::{FakeFs, Fs as _};
use gpui::{Context as _, TestAppContext};
use remote::SshSession;
use remote_server::HeadlessProject;
use serde_json::json;
use std::{path::Path, sync::Arc};

#[gpui::test]
async fn test_sharing_an_ssh_remote_project(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    let executor = cx_a.executor();
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;

    // Set up project on remote FS
    let (client_ssh, server_ssh) = SshSession::fake(cx_a, server_cx);
    let remote_fs = FakeFs::new(server_cx.executor());
    remote_fs
        .insert_tree(
            "/code",
            json!({
                "project1": {
                    "README.md": "# project 1",
                    "src": {
                        "lib.rs": "fn one() -> usize { 1 }"
                    }
                },
                "project2": {
                    "README.md": "# project 2",
                },
            }),
        )
        .await;

    // User A connects to the remote project via SSH.
    server_cx.update(HeadlessProject::init);
    let _headless_project =
        server_cx.new_model(|cx| HeadlessProject::new(server_ssh, remote_fs.clone(), cx));

    let (project_a, worktree_id) = client_a
        .build_ssh_project("/code/project1", client_ssh, cx_a)
        .await;

    // User A shares the remote project.
    let active_call_a = cx_a.read(ActiveCall::global);
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    // User B joins the project.
    let project_b = client_b.build_dev_server_project(project_id, cx_b).await;
    let worktree_b = project_b
        .update(cx_b, |project, cx| project.worktree_for_id(worktree_id, cx))
        .unwrap();

    executor.run_until_parked();
    worktree_b.update(cx_b, |worktree, _cx| {
        assert_eq!(
            worktree.paths().map(Arc::as_ref).collect::<Vec<_>>(),
            vec![
                Path::new("README.md"),
                Path::new("src"),
                Path::new("src/lib.rs"),
            ]
        );
    });

    // User B can open buffers in the remote project.
    let buffer_b = project_b
        .update(cx_b, |project, cx| {
            project.open_buffer((worktree_id, "src/lib.rs"), cx)
        })
        .await
        .unwrap();
    buffer_b.update(cx_b, |buffer, cx| {
        assert_eq!(buffer.text(), "fn one() -> usize { 1 }");
        let ix = buffer.text().find('1').unwrap();
        buffer.edit([(ix..ix + 1, "100")], None, cx);
    });

    project_b
        .update(cx_b, |project, cx| project.save_buffer(buffer_b, cx))
        .await
        .unwrap();
    assert_eq!(
        remote_fs
            .load("/code/project1/src/lib.rs".as_ref())
            .await
            .unwrap(),
        "fn one() -> usize { 100 }"
    );
}
