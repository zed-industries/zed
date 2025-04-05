use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use call::ActiveCall;
use git::status::{FileStatus, StatusCode, TrackedStatus};
use git_ui::project_diff::ProjectDiff;
use gpui::{TestAppContext, VisualTestContext};
use project::ProjectPath;
use serde_json::json;
use util::path;
use workspace::Workspace;

//
use crate::tests::TestServer;

#[gpui::test]
async fn test_project_diff(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    let mut server = TestServer::start(cx_a.background_executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    cx_a.set_name("cx_a");
    cx_b.set_name("cx_b");

    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;

    client_a
        .fs()
        .insert_tree(
            path!("/a"),
            json!({
                ".git": {},
                "changed.txt": "after\n",
                "unchanged.txt": "unchanged\n",
                "created.txt": "created\n",
                "secret.pem": "secret-changed\n",
            }),
        )
        .await;

    client_a.fs().set_git_content_for_repo(
        Path::new(path!("/a/.git")),
        &[
            ("changed.txt".into(), "before\n".to_string(), None),
            ("unchanged.txt".into(), "unchanged\n".to_string(), None),
            ("deleted.txt".into(), "deleted\n".to_string(), None),
            ("secret.pem".into(), "shh\n".to_string(), None),
        ],
    );
    let (project_a, worktree_id) = client_a.build_local_project(path!("/a"), cx_a).await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    cx_b.update(editor::init);
    cx_b.update(git_ui::init);
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    let workspace_b = cx_b.add_window(|window, cx| {
        Workspace::new(
            None,
            project_b.clone(),
            client_b.app_state.clone(),
            window,
            cx,
        )
    });
    let cx_b = &mut VisualTestContext::from_window(*workspace_b, cx_b);
    let workspace_b = workspace_b.root(cx_b).unwrap();

    cx_b.update(|window, cx| {
        window
            .focused(cx)
            .unwrap()
            .dispatch_action(&git_ui::project_diff::Diff, window, cx)
    });
    let diff = workspace_b.update(cx_b, |workspace, cx| {
        workspace.active_item(cx).unwrap().act_as::<ProjectDiff>(cx)
    });
    let diff = diff.unwrap();
    cx_b.run_until_parked();

    diff.update(cx_b, |diff, cx| {
        assert_eq!(
            diff.excerpt_paths(cx),
            vec!["changed.txt", "deleted.txt", "created.txt"]
        );
    });

    client_a
        .fs()
        .insert_tree(
            path!("/a"),
            json!({
                ".git": {},
                "changed.txt": "before\n",
                "unchanged.txt": "changed\n",
                "created.txt": "created\n",
                "secret.pem": "secret-changed\n",
            }),
        )
        .await;
    cx_b.run_until_parked();

    project_b.update(cx_b, |project, cx| {
        let project_path = ProjectPath {
            worktree_id,
            path: Arc::from(PathBuf::from("unchanged.txt")),
        };
        let status = project.project_path_git_status(&project_path, cx);
        assert_eq!(
            status.unwrap(),
            FileStatus::Tracked(TrackedStatus {
                worktree_status: StatusCode::Modified,
                index_status: StatusCode::Unmodified,
            })
        );
    });

    diff.update(cx_b, |diff, cx| {
        assert_eq!(
            diff.excerpt_paths(cx),
            vec!["deleted.txt", "unchanged.txt", "created.txt"]
        );
    });
}
