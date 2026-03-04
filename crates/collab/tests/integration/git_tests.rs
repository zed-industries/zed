use std::path::{Path, PathBuf};

use call::ActiveCall;
use git::status::{FileStatus, StatusCode, TrackedStatus};
use git_ui::project_diff::ProjectDiff;
use gpui::{AppContext as _, BackgroundExecutor, TestAppContext, VisualTestContext};
use project::ProjectPath;
use serde_json::json;
use util::{path, rel_path::rel_path};
use workspace::{MultiWorkspace, Workspace};

use crate::TestServer;

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

    client_a.fs().set_head_and_index_for_repo(
        Path::new(path!("/a/.git")),
        &[
            ("changed.txt", "before\n".to_string()),
            ("unchanged.txt", "unchanged\n".to_string()),
            ("deleted.txt", "deleted\n".to_string()),
            ("secret.pem", "shh\n".to_string()),
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
    let window_b = cx_b.add_window(|window, cx| {
        let workspace = cx.new(|cx| {
            Workspace::new(
                None,
                project_b.clone(),
                client_b.app_state.clone(),
                window,
                cx,
            )
        });
        MultiWorkspace::new(workspace, window, cx)
    });
    let cx_b = &mut VisualTestContext::from_window(*window_b, cx_b);
    let workspace_b = window_b
        .root(cx_b)
        .unwrap()
        .read_with(cx_b, |multi_workspace, _| {
            multi_workspace.workspace().clone()
        });

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
            vec![
                rel_path("changed.txt").into_arc(),
                rel_path("deleted.txt").into_arc(),
                rel_path("created.txt").into_arc()
            ]
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
            path: rel_path("unchanged.txt").into(),
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
            vec![
                rel_path("deleted.txt").into_arc(),
                rel_path("unchanged.txt").into_arc(),
                rel_path("created.txt").into_arc()
            ]
        );
    });
}

#[gpui::test]
async fn test_remote_git_worktrees(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs()
        .insert_tree(
            path!("/project"),
            json!({ ".git": {}, "file.txt": "content" }),
        )
        .await;

    let (project_a, _) = client_a.build_local_project(path!("/project"), cx_a).await;

    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    executor.run_until_parked();

    let repo_b = cx_b.update(|cx| project_b.read(cx).active_repository(cx).unwrap());

    // Initially only the main worktree (the repo itself) should be present
    let worktrees = cx_b
        .update(|cx| repo_b.update(cx, |repository, _| repository.worktrees()))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(worktrees.len(), 1);
    assert_eq!(worktrees[0].path, PathBuf::from(path!("/project")));

    // Client B creates a git worktree via the remote project
    let worktree_directory = PathBuf::from(path!("/project"));
    cx_b.update(|cx| {
        repo_b.update(cx, |repository, _| {
            repository.create_worktree(
                "feature-branch".to_string(),
                worktree_directory.clone(),
                Some("abc123".to_string()),
            )
        })
    })
    .await
    .unwrap()
    .unwrap();

    executor.run_until_parked();

    // Client B lists worktrees — should see main + the one just created
    let worktrees = cx_b
        .update(|cx| repo_b.update(cx, |repository, _| repository.worktrees()))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(worktrees.len(), 2);
    assert_eq!(worktrees[0].path, PathBuf::from(path!("/project")));
    assert_eq!(worktrees[1].path, worktree_directory.join("feature-branch"));
    assert_eq!(worktrees[1].ref_name.as_ref(), "refs/heads/feature-branch");
    assert_eq!(worktrees[1].sha.as_ref(), "abc123");

    // Verify from the host side that the worktree was actually created
    let host_worktrees = {
        let repo_a = cx_a.update(|cx| {
            project_a
                .read(cx)
                .repositories(cx)
                .values()
                .next()
                .unwrap()
                .clone()
        });
        cx_a.update(|cx| repo_a.update(cx, |repository, _| repository.worktrees()))
            .await
            .unwrap()
            .unwrap()
    };
    assert_eq!(host_worktrees.len(), 2);
    assert_eq!(host_worktrees[0].path, PathBuf::from(path!("/project")));
    assert_eq!(
        host_worktrees[1].path,
        worktree_directory.join("feature-branch")
    );

    // Client B creates a second git worktree without an explicit commit
    cx_b.update(|cx| {
        repo_b.update(cx, |repository, _| {
            repository.create_worktree(
                "bugfix-branch".to_string(),
                worktree_directory.clone(),
                None,
            )
        })
    })
    .await
    .unwrap()
    .unwrap();

    executor.run_until_parked();

    // Client B lists worktrees — should now have main + two created
    let worktrees = cx_b
        .update(|cx| repo_b.update(cx, |repository, _| repository.worktrees()))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(worktrees.len(), 3);

    let feature_worktree = worktrees
        .iter()
        .find(|worktree| worktree.ref_name.as_ref() == "refs/heads/feature-branch")
        .expect("should find feature-branch worktree");
    assert_eq!(
        feature_worktree.path,
        worktree_directory.join("feature-branch")
    );

    let bugfix_worktree = worktrees
        .iter()
        .find(|worktree| worktree.ref_name.as_ref() == "refs/heads/bugfix-branch")
        .expect("should find bugfix-branch worktree");
    assert_eq!(
        bugfix_worktree.path,
        worktree_directory.join("bugfix-branch")
    );
    assert_eq!(bugfix_worktree.sha.as_ref(), "fake-sha");
}
