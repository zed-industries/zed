use crate::init_test;
use fs::FakeFs;
use gpui::TestAppContext;
use paths;
use project::Project;
use serde_json::json;
use std::path::Path;
use util::path;

/// Verify that default_worktree_path returns the visible directory worktree's path,
/// not the home directory, when a worktree exists.
#[gpui::test]
async fn test_default_worktree_path_uses_project_dir(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/project"), json!({"main.rs": "fn main() {}"}))
        .await;

    let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;

    project.update(cx, |project, cx| {
        let path = project.environment().read(cx).default_worktree_path(cx);
        assert_eq!(path.as_ref(), Path::new(path!("/project")));
    });
}

/// Verify that default_worktree_path falls back to home_dir when no worktrees exist.
#[gpui::test]
async fn test_default_worktree_path_falls_back_to_home_dir(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    let project = Project::test(fs, [], cx).await;

    project.update(cx, |project, cx| {
        let path = project.environment().read(cx).default_worktree_path(cx);
        assert_eq!(path.as_ref(), paths::home_dir().as_path());
    });
}

/// Verify that directory worktrees take priority over single-file worktrees
/// when resolving the default environment path.
#[gpui::test]
async fn test_default_worktree_path_prefers_directory_over_single_file(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            "project": { "main.rs": "fn main() {}" },
            "standalone.rs": ""
        }),
    )
    .await;

    let project = Project::test(
        fs,
        [
            Path::new(path!("/root/standalone.rs")),
            Path::new(path!("/root/project")),
        ],
        cx,
    )
    .await;

    project.update(cx, |project, cx| {
        let path = project.environment().read(cx).default_worktree_path(cx);
        assert_eq!(path.as_ref(), Path::new(path!("/root/project")));
    });
}
