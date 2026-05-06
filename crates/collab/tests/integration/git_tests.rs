use std::path::{self, Path, PathBuf};

use call::ActiveCall;
use client::RECEIVE_TIMEOUT;
use collections::HashMap;
use git::{
    Oid,
    repository::{CommitData, RepoPath, Worktree as GitWorktree},
    status::{DiffStat, FileStatus, StatusCode, TrackedStatus},
};
use git_ui::{git_panel::GitPanel, project_diff::ProjectDiff};
use gpui::{AppContext as _, BackgroundExecutor, SharedString, TestAppContext, VisualTestContext};
use project::{
    ProjectPath,
    git_store::{CommitDataState, Repository},
};
use serde_json::json;

use util::{path, rel_path::rel_path};
use workspace::{MultiWorkspace, Workspace};

use crate::TestServer;

#[gpui::test]
async fn test_root_repo_common_dir_sync(
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

    // Set up a project whose root IS a git repository.
    client_a
        .fs()
        .insert_tree(
            path!("/project"),
            json!({ ".git": {}, "file.txt": "content" }),
        )
        .await;

    let (project_a, _) = client_a.build_local_project(path!("/project"), cx_a).await;
    executor.run_until_parked();

    // Host should see root_repo_common_dir pointing to .git at the root.
    let host_common_dir = project_a.read_with(cx_a, |project, cx| {
        let worktree = project.worktrees(cx).next().unwrap();
        worktree.read(cx).snapshot().root_repo_common_dir().cloned()
    });
    assert_eq!(
        host_common_dir.as_deref(),
        Some(path::Path::new(path!("/project/.git"))),
    );

    // Share the project and have client B join.
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    executor.run_until_parked();

    // Guest should see the same root_repo_common_dir as the host.
    let guest_common_dir = project_b.read_with(cx_b, |project, cx| {
        let worktree = project.worktrees(cx).next().unwrap();
        worktree.read(cx).snapshot().root_repo_common_dir().cloned()
    });
    assert_eq!(
        guest_common_dir, host_common_dir,
        "guest should see the same root_repo_common_dir as host",
    );
}

fn collect_diff_stats<C: gpui::AppContext>(
    panel: &gpui::Entity<GitPanel>,
    cx: &C,
) -> HashMap<RepoPath, DiffStat> {
    panel.read_with(cx, |panel, cx| {
        let Some(repo) = panel.active_repository() else {
            return HashMap::default();
        };
        let snapshot = repo.read(cx).snapshot();
        let mut stats = HashMap::default();
        for entry in snapshot.statuses_by_path.iter() {
            if let Some(diff_stat) = entry.diff_stat {
                stats.insert(entry.repo_path.clone(), diff_stat);
            }
        }
        stats
    })
}

async fn load_commit_data_batch(
    repository: &gpui::Entity<Repository>,
    shas: &[Oid],
    executor: &BackgroundExecutor,
    cx: &mut TestAppContext,
) -> HashMap<Oid, CommitData> {
    let states = cx.update(|cx| {
        shas.iter()
            .map(|sha| {
                (
                    *sha,
                    repository.update(cx, |repository, cx| {
                        repository.fetch_commit_data(*sha, true, cx).clone()
                    }),
                )
            })
            .collect::<Vec<_>>()
    });

    executor.run_until_parked();

    let mut commit_data = HashMap::default();
    for (sha, state) in states {
        let data = match state {
            CommitDataState::Loaded(data) => data.as_ref().clone(),
            CommitDataState::Loading(Some(shared)) => shared.await.unwrap().as_ref().clone(),
            CommitDataState::Loading(None) => {
                panic!("fetch_commit_data(..., true) should return an await-result state")
            }
        };
        commit_data.insert(sha, data);
    }

    commit_data
}

fn branch_list_snapshot(
    project: &gpui::Entity<project::Project>,
    cx: &mut TestAppContext,
) -> (Option<String>, Vec<String>) {
    project.read_with(cx, |project, cx| {
        let repos = project.repositories(cx);
        assert_eq!(repos.len(), 1, "project should have exactly 1 repository");
        let repo = repos.values().next().unwrap();
        let snapshot = repo.read(cx).snapshot();
        (
            snapshot
                .branch
                .as_ref()
                .map(|branch| branch.name().to_string()),
            snapshot
                .branch_list
                .iter()
                .map(|branch| branch.ref_name.to_string())
                .collect(),
        )
    })
}

fn assert_remote_cache_matches_local_cache(
    local_repository: &gpui::Entity<Repository>,
    remote_repository: &gpui::Entity<Repository>,
    cx_local: &mut TestAppContext,
    cx_remote: &mut TestAppContext,
) {
    let local_cache = cx_local.update(|cx| {
        local_repository.update(cx, |repository, _| repository.loaded_commit_data_for_test())
    });
    let remote_cache = cx_remote.update(|cx| {
        remote_repository.update(cx, |repository, _| repository.loaded_commit_data_for_test())
    });

    for (sha, remote_commit_data) in &remote_cache {
        let local_commit_data = local_cache
            .get(sha)
            .unwrap_or_else(|| panic!("local cache missing commit data for {sha}"));
        assert_eq!(
            local_commit_data.sha, remote_commit_data.sha,
            "local and remote cache should agree on sha for {sha}"
        );
        assert_eq!(
            local_commit_data.parents, remote_commit_data.parents,
            "local and remote cache should agree on parents for {sha}"
        );
        assert_eq!(
            local_commit_data.author_name, remote_commit_data.author_name,
            "local and remote cache should agree on author_name for {sha}"
        );
        assert_eq!(
            local_commit_data.author_email, remote_commit_data.author_email,
            "local and remote cache should agree on author_email for {sha}"
        );
        assert_eq!(
            local_commit_data.commit_timestamp, remote_commit_data.commit_timestamp,
            "local and remote cache should agree on commit_timestamp for {sha}"
        );
        assert_eq!(
            local_commit_data.subject, remote_commit_data.subject,
            "local and remote cache should agree on subject for {sha}"
        );
        assert_eq!(
            local_commit_data.message, remote_commit_data.message,
            "local and remote cache should agree on message for {sha}"
        );
    }
}

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
                git::repository::CreateWorktreeTarget::NewBranch {
                    branch_name: "feature-branch".to_string(),
                    base_sha: Some("abc123".to_string()),
                },
                worktree_directory.join("feature-branch"),
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
    assert_eq!(
        worktrees[1].ref_name,
        Some("refs/heads/feature-branch".into())
    );
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
                git::repository::CreateWorktreeTarget::NewBranch {
                    branch_name: "bugfix-branch".to_string(),
                    base_sha: None,
                },
                worktree_directory.join("bugfix-branch"),
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
        .find(|worktree| worktree.ref_name == Some("refs/heads/feature-branch".into()))
        .expect("should find feature-branch worktree");
    assert_eq!(
        feature_worktree.path,
        worktree_directory.join("feature-branch")
    );

    let bugfix_worktree = worktrees
        .iter()
        .find(|worktree| worktree.ref_name == Some("refs/heads/bugfix-branch".into()))
        .expect("should find bugfix-branch worktree");
    assert_eq!(
        bugfix_worktree.path,
        worktree_directory.join("bugfix-branch")
    );
    assert_eq!(bugfix_worktree.sha.as_ref(), "fake-sha");

    // Client B (guest) attempts to rename a worktree. This should fail
    // because worktree renaming is not forwarded through collab
    let rename_result = cx_b
        .update(|cx| {
            repo_b.update(cx, |repository, _| {
                repository.rename_worktree(
                    worktree_directory.join("feature-branch"),
                    worktree_directory.join("renamed-branch"),
                )
            })
        })
        .await
        .unwrap();
    assert!(
        rename_result.is_err(),
        "Guest should not be able to rename worktrees via collab"
    );

    executor.run_until_parked();

    // Verify worktrees are unchanged — still 3
    let worktrees = cx_b
        .update(|cx| repo_b.update(cx, |repository, _| repository.worktrees()))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        worktrees.len(),
        3,
        "Worktree count should be unchanged after failed rename"
    );

    // Client B (guest) attempts to remove a worktree. This should fail
    // because worktree removal is not forwarded through collab
    let remove_result = cx_b
        .update(|cx| {
            repo_b.update(cx, |repository, _| {
                repository.remove_worktree(worktree_directory.join("feature-branch"), false)
            })
        })
        .await
        .unwrap();
    assert!(
        remove_result.is_err(),
        "Guest should not be able to remove worktrees via collab"
    );

    executor.run_until_parked();

    // Verify worktrees are unchanged — still 3
    let worktrees = cx_b
        .update(|cx| repo_b.update(cx, |repository, _| repository.worktrees()))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        worktrees.len(),
        3,
        "Worktree count should be unchanged after failed removal"
    );
}

#[gpui::test]
async fn test_remote_git_head_sha(
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
    let local_head_sha = cx_a.update(|cx| {
        project_a
            .read(cx)
            .active_repository(cx)
            .unwrap()
            .update(cx, |repository, _| repository.head_sha())
    });
    let local_head_sha = local_head_sha.await.unwrap().unwrap();

    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    executor.run_until_parked();

    let remote_head_sha = cx_b.update(|cx| {
        project_b
            .read(cx)
            .active_repository(cx)
            .unwrap()
            .update(cx, |repository, _| repository.head_sha())
    });
    let remote_head_sha = remote_head_sha.await.unwrap();

    assert_eq!(remote_head_sha.unwrap(), local_head_sha);
}

#[gpui::test]
async fn test_remote_git_commit_data_batches(
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

    let commit_shas = [
        "0123456789abcdef0123456789abcdef01234567"
            .parse::<Oid>()
            .unwrap(),
        "1111111111111111111111111111111111111111"
            .parse::<Oid>()
            .unwrap(),
        "2222222222222222222222222222222222222222"
            .parse::<Oid>()
            .unwrap(),
        "3333333333333333333333333333333333333333"
            .parse::<Oid>()
            .unwrap(),
    ];

    client_a.fs().set_commit_data(
        Path::new(path!("/project/.git")),
        commit_shas.iter().enumerate().map(|(index, sha)| {
            (
                CommitData {
                    sha: *sha,
                    parents: Default::default(),
                    author_name: SharedString::from(format!("Author {index}")),
                    author_email: SharedString::from(format!("author{index}@example.com")),
                    commit_timestamp: 1_700_000_000 + index as i64,
                    subject: SharedString::from(format!("Subject {index}")),
                    message: SharedString::from(format!("Subject {index}\n\nBody {index}")),
                },
                false,
            )
        }),
    );

    let (project_a, _) = client_a.build_local_project(path!("/project"), cx_a).await;
    executor.run_until_parked();

    let repo_a = cx_a.update(|cx| project_a.read(cx).active_repository(cx).unwrap());

    let primed_before = load_commit_data_batch(&repo_a, &commit_shas[..2], &executor, cx_a).await;
    assert_eq!(
        primed_before.len(),
        2,
        "host should prime two commits before sharing"
    );

    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    executor.run_until_parked();

    let repo_b = cx_b.update(|cx| project_b.read(cx).active_repository(cx).unwrap());

    let remote_batch_one =
        load_commit_data_batch(&repo_b, &commit_shas[..3], &executor, cx_b).await;
    assert_eq!(remote_batch_one.len(), 3);
    for (index, sha) in commit_shas[..3].iter().enumerate() {
        let commit_data = remote_batch_one.get(sha).unwrap();
        assert_eq!(commit_data.sha, *sha);
        assert_eq!(commit_data.subject.as_ref(), format!("Subject {index}"));
        assert_eq!(
            commit_data.message.as_ref(),
            format!("Subject {index}\n\nBody {index}")
        );
    }

    let primed_after = load_commit_data_batch(&repo_a, &commit_shas[2..], &executor, cx_a).await;
    assert_eq!(
        primed_after.len(),
        2,
        "host should prime remaining commits after remote fetches"
    );

    let remote_batch_two =
        load_commit_data_batch(&repo_b, &commit_shas[1..], &executor, cx_b).await;
    assert_eq!(remote_batch_two.len(), 3);

    assert_remote_cache_matches_local_cache(&repo_a, &repo_b, cx_a, cx_b);
}

#[gpui::test]
async fn test_branch_list_sync(
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
    client_a.fs().insert_branches(
        Path::new(path!("/project/.git")),
        &["main", "feature-1", "feature-2"],
    );

    let (project_a, _) = client_a.build_local_project(path!("/project"), cx_a).await;
    executor.run_until_parked();

    let host_snapshot = branch_list_snapshot(&project_a, cx_a);
    assert_eq!(host_snapshot.0.as_deref(), Some("main"));
    assert_eq!(
        host_snapshot.1,
        vec![
            "refs/heads/feature-1".to_string(),
            "refs/heads/feature-2".to_string(),
            "refs/heads/main".to_string(),
        ]
    );

    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    executor.run_until_parked();

    let repo_b = cx_b.update(|cx| project_b.read(cx).active_repository(cx).unwrap());

    cx_b.update(|cx| {
        repo_b.update(cx, |repository, _cx| {
            repository.create_branch("totally-new-branch".to_string(), None)
        })
    })
    .await
    .unwrap()
    .unwrap();

    cx_b.update(|cx| {
        repo_b.update(cx, |repository, _cx| {
            repository.change_branch("totally-new-branch".to_string())
        })
    })
    .await
    .unwrap()
    .unwrap();

    executor.run_until_parked();

    let host_snapshot_after_update = branch_list_snapshot(&project_a, cx_a);
    assert_eq!(
        host_snapshot_after_update.0.as_deref(),
        Some("totally-new-branch")
    );
    assert_eq!(
        host_snapshot_after_update.1,
        vec![
            "refs/heads/feature-1".to_string(),
            "refs/heads/feature-2".to_string(),
            "refs/heads/main".to_string(),
            "refs/heads/totally-new-branch".to_string(),
        ]
    );

    let guest_snapshot_after_update = branch_list_snapshot(&project_b, cx_b);
    assert_eq!(guest_snapshot_after_update, host_snapshot_after_update);
}

#[gpui::test]
async fn test_linked_worktrees_sync(
    executor: BackgroundExecutor,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let client_c = server.create_client(cx_c, "user_c").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    // Set up a git repo with two linked worktrees already present.
    client_a
        .fs()
        .insert_tree(
            path!("/project"),
            json!({ ".git": {}, "file.txt": "content" }),
        )
        .await;

    let fs = client_a.fs();
    fs.add_linked_worktree_for_repo(
        Path::new(path!("/project/.git")),
        true,
        GitWorktree {
            path: PathBuf::from(path!("/worktrees/feature-branch")),
            ref_name: Some("refs/heads/feature-branch".into()),
            sha: "bbb222".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;
    fs.add_linked_worktree_for_repo(
        Path::new(path!("/project/.git")),
        true,
        GitWorktree {
            path: PathBuf::from(path!("/worktrees/bugfix-branch")),
            ref_name: Some("refs/heads/bugfix-branch".into()),
            sha: "ccc333".into(),
            is_main: false,
            is_bare: false,
        },
    )
    .await;

    let (project_a, _) = client_a.build_local_project(path!("/project"), cx_a).await;

    // Wait for git scanning to complete on the host.
    executor.run_until_parked();

    // Verify the host sees 2 linked worktrees (main worktree is filtered out).
    let host_linked = project_a.read_with(cx_a, |project, cx| {
        let repos = project.repositories(cx);
        assert_eq!(repos.len(), 1, "host should have exactly 1 repository");
        let repo = repos.values().next().unwrap();
        repo.read(cx).linked_worktrees().to_vec()
    });
    assert_eq!(
        host_linked.len(),
        2,
        "host should have 2 linked worktrees (main filtered out)"
    );
    assert_eq!(
        host_linked[0].path,
        PathBuf::from(path!("/worktrees/bugfix-branch"))
    );
    assert_eq!(
        host_linked[0].ref_name,
        Some("refs/heads/bugfix-branch".into())
    );
    assert_eq!(host_linked[0].sha.as_ref(), "ccc333");
    assert_eq!(
        host_linked[1].path,
        PathBuf::from(path!("/worktrees/feature-branch"))
    );
    assert_eq!(
        host_linked[1].ref_name,
        Some("refs/heads/feature-branch".into())
    );
    assert_eq!(host_linked[1].sha.as_ref(), "bbb222");

    // Share the project and have client B join.
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    executor.run_until_parked();

    // Verify the guest sees the same linked worktrees as the host.
    let guest_linked = project_b.read_with(cx_b, |project, cx| {
        let repos = project.repositories(cx);
        assert_eq!(repos.len(), 1, "guest should have exactly 1 repository");
        let repo = repos.values().next().unwrap();
        repo.read(cx).linked_worktrees().to_vec()
    });
    assert_eq!(
        guest_linked, host_linked,
        "guest's linked_worktrees should match host's after initial sync"
    );

    // Now mutate: add a third linked worktree on the host side.
    client_a
        .fs()
        .add_linked_worktree_for_repo(
            Path::new(path!("/project/.git")),
            true,
            GitWorktree {
                path: PathBuf::from(path!("/worktrees/hotfix-branch")),
                ref_name: Some("refs/heads/hotfix-branch".into()),
                sha: "ddd444".into(),
                is_main: false,
                is_bare: false,
            },
        )
        .await;

    // Wait for the host to re-scan and propagate the update.
    executor.run_until_parked();

    // Verify host now sees 3 linked worktrees.
    let host_linked_updated = project_a.read_with(cx_a, |project, cx| {
        let repos = project.repositories(cx);
        let repo = repos.values().next().unwrap();
        repo.read(cx).linked_worktrees().to_vec()
    });
    assert_eq!(
        host_linked_updated.len(),
        3,
        "host should now have 3 linked worktrees"
    );
    assert_eq!(
        host_linked_updated[2].path,
        PathBuf::from(path!("/worktrees/hotfix-branch"))
    );

    // Verify the guest also received the update.
    let guest_linked_updated = project_b.read_with(cx_b, |project, cx| {
        let repos = project.repositories(cx);
        let repo = repos.values().next().unwrap();
        repo.read(cx).linked_worktrees().to_vec()
    });
    assert_eq!(
        guest_linked_updated, host_linked_updated,
        "guest's linked_worktrees should match host's after update"
    );

    // Now mutate: remove one linked worktree from the host side.
    client_a
        .fs()
        .remove_worktree_for_repo(
            Path::new(path!("/project/.git")),
            true,
            "refs/heads/bugfix-branch",
        )
        .await;

    executor.run_until_parked();

    // Verify host now sees 2 linked worktrees (feature-branch and hotfix-branch).
    let (host_linked_after_removal, host_git_paths_after_removal) =
        project_a.read_with(cx_a, |project, cx| {
            let repos = project.repositories(cx);
            let repo = repos.values().next().unwrap();
            let repo = repo.read(cx);
            (
                repo.linked_worktrees().to_vec(),
                (
                    repo.repository_dir_abs_path.to_path_buf(),
                    repo.common_dir_abs_path.to_path_buf(),
                ),
            )
        });
    assert_eq!(
        host_linked_after_removal.len(),
        2,
        "host should have 2 linked worktrees after removal"
    );
    assert!(
        host_linked_after_removal
            .iter()
            .all(|wt| wt.ref_name != Some("refs/heads/bugfix-branch".into())),
        "bugfix-branch should have been removed"
    );

    // Verify the guest also reflects the removal.
    let guest_linked_after_removal = project_b.read_with(cx_b, |project, cx| {
        let repos = project.repositories(cx);
        let repo = repos.values().next().unwrap();
        repo.read(cx).linked_worktrees().to_vec()
    });
    assert_eq!(
        guest_linked_after_removal, host_linked_after_removal,
        "guest's linked_worktrees should match host's after removal"
    );

    // Test DB roundtrip: client C joins late, getting state from the database.
    // This verifies that linked_worktrees are persisted and restored correctly.
    let project_c = client_c.join_remote_project(project_id, cx_c).await;
    executor.run_until_parked();

    let late_joiner_linked = project_c.read_with(cx_c, |project, cx| {
        let repos = project.repositories(cx);
        assert_eq!(
            repos.len(),
            1,
            "late joiner should have exactly 1 repository"
        );
        let repo = repos.values().next().unwrap();
        repo.read(cx).linked_worktrees().to_vec()
    });
    assert_eq!(
        late_joiner_linked, host_linked_after_removal,
        "late-joining client's linked_worktrees should match host's (DB roundtrip)"
    );
    let late_joiner_git_paths = project_c.read_with(cx_c, |project, cx| {
        let repos = project.repositories(cx);
        let repo = repos.values().next().unwrap();
        let repo = repo.read(cx);
        (
            repo.repository_dir_abs_path.to_path_buf(),
            repo.common_dir_abs_path.to_path_buf(),
        )
    });
    assert_eq!(
        late_joiner_git_paths, host_git_paths_after_removal,
        "late-joining client's git directory paths should match host's (DB roundtrip)"
    );

    // Test reconnection: disconnect client B (guest) and reconnect.
    // After rejoining, client B should get linked_worktrees back from the DB.
    server.disconnect_client(client_b.peer_id().unwrap());
    executor.advance_clock(RECEIVE_TIMEOUT);
    executor.run_until_parked();

    // Client B reconnects automatically.
    executor.advance_clock(RECEIVE_TIMEOUT);
    executor.run_until_parked();

    // Verify client B still has the correct linked worktrees after reconnection.
    let (guest_linked_after_reconnect, guest_git_paths_after_reconnect) =
        project_b.read_with(cx_b, |project, cx| {
            let repos = project.repositories(cx);
            assert_eq!(
                repos.len(),
                1,
                "guest should still have exactly 1 repository after reconnect"
            );
            let repo = repos.values().next().unwrap();
            let repo = repo.read(cx);
            (
                repo.linked_worktrees().to_vec(),
                (
                    repo.repository_dir_abs_path.to_path_buf(),
                    repo.common_dir_abs_path.to_path_buf(),
                ),
            )
        });
    assert_eq!(
        guest_linked_after_reconnect, host_linked_after_removal,
        "guest's linked_worktrees should survive guest disconnect/reconnect"
    );
    assert_eq!(
        guest_git_paths_after_reconnect, host_git_paths_after_removal,
        "guest's git directory paths should survive guest disconnect/reconnect"
    );
}

#[gpui::test]
async fn test_diff_stat_sync_between_host_and_downstream_client(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
) {
    let mut server = TestServer::start(cx_a.background_executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let client_c = server.create_client(cx_c, "user_c").await;

    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;

    let fs = client_a.fs();
    fs.insert_tree(
        path!("/code"),
        json!({
            "project1": {
                ".git": {},
                "src": {
                    "lib.rs": "line1\nline2\nline3\n",
                    "new_file.rs": "added1\nadded2\n",
                },
                "README.md": "# project 1",
            }
        }),
    )
    .await;

    let dot_git = Path::new(path!("/code/project1/.git"));
    fs.set_head_for_repo(
        dot_git,
        &[
            ("src/lib.rs", "line1\nold_line2\n".into()),
            ("src/deleted.rs", "was_here\n".into()),
        ],
        "deadbeef",
    );
    fs.set_index_for_repo(
        dot_git,
        &[
            ("src/lib.rs", "line1\nold_line2\nline3\nline4\n".into()),
            ("src/staged_only.rs", "x\ny\n".into()),
            ("src/new_file.rs", "added1\nadded2\n".into()),
            ("README.md", "# project 1".into()),
        ],
    );

    let (project_a, worktree_id) = client_a
        .build_local_project(path!("/code/project1"), cx_a)
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    let _project_c = client_c.join_remote_project(project_id, cx_c).await;
    cx_a.run_until_parked();

    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);

    let panel_a = workspace_a.update_in(cx_a, GitPanel::new_test);
    workspace_a.update_in(cx_a, |workspace, window, cx| {
        workspace.add_panel(panel_a.clone(), window, cx);
    });

    let panel_b = workspace_b.update_in(cx_b, GitPanel::new_test);
    workspace_b.update_in(cx_b, |workspace, window, cx| {
        workspace.add_panel(panel_b.clone(), window, cx);
    });

    cx_a.run_until_parked();

    let stats_a = collect_diff_stats(&panel_a, cx_a);
    let stats_b = collect_diff_stats(&panel_b, cx_b);

    let mut expected: HashMap<RepoPath, DiffStat> = HashMap::default();
    expected.insert(
        RepoPath::new("src/lib.rs").unwrap(),
        DiffStat {
            added: 3,
            deleted: 2,
        },
    );
    expected.insert(
        RepoPath::new("src/deleted.rs").unwrap(),
        DiffStat {
            added: 0,
            deleted: 1,
        },
    );
    expected.insert(
        RepoPath::new("src/new_file.rs").unwrap(),
        DiffStat {
            added: 2,
            deleted: 0,
        },
    );
    expected.insert(
        RepoPath::new("README.md").unwrap(),
        DiffStat {
            added: 1,
            deleted: 0,
        },
    );
    assert_eq!(stats_a, expected, "host diff stats should match expected");
    assert_eq!(stats_a, stats_b, "host and remote should agree");

    let buffer_a = project_a
        .update(cx_a, |p, cx| {
            p.open_buffer((worktree_id, rel_path("src/lib.rs")), cx)
        })
        .await
        .unwrap();

    let _buffer_b = project_b
        .update(cx_b, |p, cx| {
            p.open_buffer((worktree_id, rel_path("src/lib.rs")), cx)
        })
        .await
        .unwrap();
    cx_a.run_until_parked();

    buffer_a.update(cx_a, |buf, cx| {
        buf.edit([(buf.len()..buf.len(), "line4\n")], None, cx);
    });
    project_a
        .update(cx_a, |project, cx| {
            project.save_buffer(buffer_a.clone(), cx)
        })
        .await
        .unwrap();
    cx_a.run_until_parked();

    let stats_a = collect_diff_stats(&panel_a, cx_a);
    let stats_b = collect_diff_stats(&panel_b, cx_b);

    let mut expected_after_edit = expected.clone();
    expected_after_edit.insert(
        RepoPath::new("src/lib.rs").unwrap(),
        DiffStat {
            added: 4,
            deleted: 2,
        },
    );
    assert_eq!(
        stats_a, expected_after_edit,
        "host diff stats should reflect the edit"
    );
    assert_eq!(
        stats_b, expected_after_edit,
        "remote diff stats should reflect the host's edit"
    );

    let active_call_b = cx_b.read(ActiveCall::global);
    active_call_b
        .update(cx_b, |call, cx| call.hang_up(cx))
        .await
        .unwrap();
    cx_a.run_until_parked();

    let user_id_b = client_b.current_user_id(cx_b).to_proto();
    active_call_a
        .update(cx_a, |call, cx| call.invite(user_id_b, None, cx))
        .await
        .unwrap();
    cx_b.run_until_parked();
    let active_call_b = cx_b.read(ActiveCall::global);
    active_call_b
        .update(cx_b, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();
    cx_a.run_until_parked();

    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    cx_a.run_until_parked();

    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);
    let panel_b = workspace_b.update_in(cx_b, GitPanel::new_test);
    workspace_b.update_in(cx_b, |workspace, window, cx| {
        workspace.add_panel(panel_b.clone(), window, cx);
    });
    cx_b.run_until_parked();

    let stats_b = collect_diff_stats(&panel_b, cx_b);
    assert_eq!(
        stats_b, expected_after_edit,
        "remote diff stats should be restored from the database after rejoining the call"
    );
}
