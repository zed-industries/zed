use std::path::Path;

use call::ActiveCall;
use collections::HashMap;
use git::{
    repository::RepoPath,
    status::{DiffStat, FileStatus, StatusCode, TrackedStatus},
};
use git_ui::git_panel::GitPanel;
use gpui::{AppContext as _, TestAppContext, UpdateGlobal as _};
use project::ProjectPath;
use serde_json::json;
use settings::SettingsStore;
use util::{path, rel_path::rel_path};

use crate::TestServer;

#[gpui::test]
async fn test_diff_stat_sync_between_host_and_downstream_client(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(cx_a.background_executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;

    client_a
        .fs()
        .insert_tree(
            path!("/a"),
            json!({
                ".git": {},
                "src": {
                    "lib.rs": "line1\nline2\nline3\n",
                    "new_file.rs": "added1\nadded2\n",
                },
                "README.md": "# project 1",
            }),
        )
        .await;

    let dot_git = Path::new(path!("/a/.git"));
    client_a.fs().set_head_for_repo(
        dot_git,
        &[
            ("src/lib.rs", "line1\nold_line2\n".into()),
            ("src/deleted.rs", "was_here\n".into()),
        ],
        "deadbeef",
    );
    client_a.fs().set_index_for_repo(
        dot_git,
        &[
            ("src/lib.rs", "line1\nold_line2\nline3\nline4\n".into()),
            ("src/staged_only.rs", "x\ny\n".into()),
        ],
    );

    cx_a.update(git_ui::init);
    cx_b.update(git_ui::init);

    let (project_a, worktree_id) = client_a.build_local_project(path!("/a"), cx_a).await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    let project_b = client_b.join_remote_project(project_id, cx_b).await;

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

    // ── Assertion 1: diff_stats setting is off → both panels have empty diff stats ──
    let stats_a = panel_a.read_with(cx_a, |panel, _| panel.diff_stats().clone());
    let stats_b = panel_b.read_with(cx_b, |panel, _| panel.diff_stats().clone());
    assert!(
        stats_a.is_empty(),
        "host should have no diff stats when setting is disabled"
    );
    assert!(
        stats_b.is_empty(),
        "remote should have no diff stats when setting is disabled"
    );

    // ── Assertion 2: enable diff_stats only on remote → only remote has data ──
    cx_b.update(|_, cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.git_panel.get_or_insert_default().diff_stats = Some(true);
            });
        });
    });
    cx_a.run_until_parked();

    let stats_a = panel_a.read_with(cx_a, |panel, _| panel.diff_stats().clone());
    let stats_b = panel_b.read_with(cx_b, |panel, _| panel.diff_stats().clone());
    assert!(
        stats_a.is_empty(),
        "host should still have no diff stats when its setting is disabled"
    );
    assert!(
        !stats_b.is_empty(),
        "remote should have diff stats after enabling the setting"
    );

    // ── Assertion 3: enable on both → same data, checked against expected values ──
    cx_a.update(|_, cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.git_panel.get_or_insert_default().diff_stats = Some(true);
            });
        });
    });
    cx_a.run_until_parked();

    let stats_a = panel_a.read_with(cx_a, |panel, _| panel.diff_stats().clone());
    let stats_b = panel_b.read_with(cx_b, |panel, _| panel.diff_stats().clone());

    // GitPanel combines unstaged (HeadToWorktree) and staged (HeadToIndex) by summing per path.
    //
    // HeadToWorktree:
    //   src/lib.rs:      head="line1\nold_line2\n"(2 lines) vs worktree="line1\nline2\nline3\n"(3 lines) → +3 -2
    //   src/deleted.rs:  head="was_here\n"(1 line) vs worktree=missing → +0 -1
    //   src/new_file.rs: head=missing vs worktree="added1\nadded2\n"(2 lines) → +2 -0
    //   README.md:       head=missing vs worktree="# project 1"(1 line) → +1 -0
    //
    // HeadToIndex:
    //   src/lib.rs:       head="line1\nold_line2\n"(2 lines) vs index="line1\nold_line2\nline3\nline4\n"(4 lines) → +4 -2
    //   src/deleted.rs:   head="was_here\n"(1 line) vs index=missing → +0 -1
    //   src/staged_only.rs: head=missing vs index="x\ny\n"(2 lines) → +2 -0
    let mut expected: HashMap<RepoPath, DiffStat> = HashMap::default();
    expected.insert(
        RepoPath::new("src/lib.rs").unwrap(),
        DiffStat {
            added: 3 + 4,
            deleted: 2 + 2,
        },
    );
    expected.insert(
        RepoPath::new("src/deleted.rs").unwrap(),
        DiffStat {
            added: 0,
            deleted: 1 + 1,
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
    expected.insert(
        RepoPath::new("src/staged_only.rs").unwrap(),
        DiffStat {
            added: 2,
            deleted: 0,
        },
    );
    assert_eq!(stats_a, expected, "host diff stats should match expected");
    assert_eq!(stats_b, expected, "remote diff stats should match expected");
    assert_eq!(stats_a, stats_b, "host and remote should agree");

    // ── Assertion 4: update a file on host, save → remote picks up new diff stats ──
    let buffer_a = project_a
        .update(cx_a, |p, cx| {
            p.open_buffer((worktree_id, rel_path("src/lib.rs")), cx)
        })
        .await
        .unwrap();

    // Also open the buffer on the remote so its GitPanel sees the BufferEvent::Saved
    // and triggers a diff stats refresh.
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

    let stats_a = panel_a.read_with(cx_a, |panel, _| panel.diff_stats().clone());
    let stats_b = panel_b.read_with(cx_b, |panel, _| panel.diff_stats().clone());

    // After adding "line4\n", worktree src/lib.rs is now "line1\nline2\nline3\nline4\n" (4 lines).
    // HeadToWorktree for src/lib.rs: head=2 lines vs worktree=4 lines → +4 -2
    // Combined with HeadToIndex: +4 + +4 = 8, -2 + -2 = 4
    let mut expected_after_edit = expected.clone();
    expected_after_edit.insert(
        RepoPath::new("src/lib.rs").unwrap(),
        DiffStat {
            added: 4 + 4,
            deleted: 2 + 2,
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
            workspace::Workspace::new(
                None,
                project_b.clone(),
                client_b.app_state.clone(),
                window,
                cx,
            )
        });
        workspace::MultiWorkspace::new(workspace, window, cx)
    });
    let cx_b = &mut gpui::VisualTestContext::from_window(*window_b, cx_b);
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
        workspace
            .active_item(cx)
            .unwrap()
            .act_as::<git_ui::project_diff::ProjectDiff>(cx)
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
