use std::path::Path;

use call::ActiveCall;
use collections::HashMap;
use git::{repository::RepoPath, status::DiffStat};
use git_ui::git_panel::GitPanel;
use gpui::TestAppContext;
use serde_json::json;

use util::{path, rel_path::rel_path};

use crate::TestServer;

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
        for entry in snapshot.diff_stats_by_path.iter() {
            stats.insert(entry.repo_path.clone(), entry.diff_stat);
        }
        stats
    })
}

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

    // Diff stats are populated from the repository snapshot, which is computed
    // regardless of the diff_stats UI setting. Verify the snapshot has data.
    let stats_a = collect_diff_stats(&panel_a, cx_a);
    let stats_b = collect_diff_stats(&panel_b, cx_b);

    // The fake git repo compares HEAD vs working tree (like `git diff --numstat HEAD`).
    // HEAD has: src/lib.rs ("line1\nold_line2\n"), src/deleted.rs ("was_here\n")
    // Worktree has: src/lib.rs ("line1\nline2\nline3\n"), src/new_file.rs ("added1\nadded2\n"), README.md ("# project 1")
    //
    // src/lib.rs:      head=2 lines vs worktree=3 lines → +3 -2
    // src/deleted.rs:  head=1 line vs worktree=missing → +0 -1
    // src/new_file.rs: head=missing vs worktree=2 lines → +2 -0
    // README.md:       head=missing vs worktree=1 line → +1 -0
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

    // ── Update a file on host, save → remote picks up new diff stats ──
    let buffer_a = project_a
        .update(cx_a, |p, cx| {
            p.open_buffer((worktree_id, rel_path("src/lib.rs")), cx)
        })
        .await
        .unwrap();

    // Also open the buffer on the remote so its project sees the update.
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

    // After adding "line4\n", worktree src/lib.rs is now "line1\nline2\nline3\nline4\n" (4 lines).
    // HEAD vs worktree: head=2 lines vs worktree=4 lines → +4 -2
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
}
