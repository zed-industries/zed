use fs::{FakeFs, Fs};
use git::repository::repo_path;
use git::status::{StatusCode, UnmergedStatus, UnmergedStatusCode};
use gpui::{BackgroundExecutor, TestAppContext};
use serde_json::json;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use util::path;

#[gpui::test]
async fn test_fake_worktree_lifecycle(cx: &mut TestAppContext) {
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/project", json!({".git": {}, "file.txt": "content"}))
        .await;
    let repo = fs
        .open_repo(Path::new("/project/.git"), None)
        .expect("should open fake repo");

    // Initially only the main worktree exists
    let worktrees = repo.worktrees().await.unwrap();
    assert_eq!(worktrees.len(), 1);
    assert_eq!(worktrees[0].path, PathBuf::from("/project"));

    fs.create_dir("/my-worktrees".as_ref()).await.unwrap();
    let worktrees_dir = Path::new("/my-worktrees");

    // Create a worktree
    let worktree_1_dir = worktrees_dir.join("feature-branch");
    repo.create_worktree(
        git::repository::CreateWorktreeTarget::NewBranch {
            branch_name: "feature-branch".to_string(),
            base_sha: Some("abc123".to_string()),
        },
        worktree_1_dir.clone(),
    )
    .await
    .unwrap();

    // List worktrees — should have main + one created
    let worktrees = repo.worktrees().await.unwrap();
    assert_eq!(worktrees.len(), 2);
    assert_eq!(worktrees[0].path, PathBuf::from("/project"));
    assert_eq!(worktrees[1].path, worktree_1_dir);
    assert_eq!(
        worktrees[1].ref_name,
        Some("refs/heads/feature-branch".into())
    );
    assert_eq!(worktrees[1].sha.as_ref(), "abc123");

    // Directory should exist in FakeFs after create
    assert!(fs.is_dir(&worktrees_dir.join("feature-branch")).await);

    // Create a second worktree (without explicit commit)
    let worktree_2_dir = worktrees_dir.join("bugfix-branch");
    repo.create_worktree(
        git::repository::CreateWorktreeTarget::NewBranch {
            branch_name: "bugfix-branch".to_string(),
            base_sha: None,
        },
        worktree_2_dir.clone(),
    )
    .await
    .unwrap();

    let worktrees = repo.worktrees().await.unwrap();
    assert_eq!(worktrees.len(), 3);
    assert!(fs.is_dir(&worktree_2_dir).await);

    // Rename the first worktree
    repo.rename_worktree(worktree_1_dir, worktrees_dir.join("renamed-branch"))
        .await
        .unwrap();

    let worktrees = repo.worktrees().await.unwrap();
    assert_eq!(worktrees.len(), 3);
    assert!(
        worktrees
            .iter()
            .any(|w| w.path == worktrees_dir.join("renamed-branch")),
    );
    assert!(
        worktrees
            .iter()
            .all(|w| w.path != worktrees_dir.join("feature-branch")),
    );

    // Directory should be moved in FakeFs after rename
    assert!(!fs.is_dir(&worktrees_dir.join("feature-branch")).await);
    assert!(fs.is_dir(&worktrees_dir.join("renamed-branch")).await);

    // Rename a nonexistent worktree should fail
    let result = repo
        .rename_worktree(PathBuf::from("/nonexistent"), PathBuf::from("/somewhere"))
        .await;
    assert!(result.is_err());

    // Remove a worktree
    repo.remove_worktree(worktrees_dir.join("renamed-branch"), false)
        .await
        .unwrap();

    let worktrees = repo.worktrees().await.unwrap();
    assert_eq!(worktrees.len(), 2);
    assert_eq!(worktrees[0].path, PathBuf::from("/project"));
    assert_eq!(worktrees[1].path, worktree_2_dir);

    // Directory should be removed from FakeFs after remove
    assert!(!fs.is_dir(&worktrees_dir.join("renamed-branch")).await);

    // Remove a nonexistent worktree should fail
    let result = repo
        .remove_worktree(PathBuf::from("/nonexistent"), false)
        .await;
    assert!(result.is_err());

    // Remove the last worktree
    repo.remove_worktree(worktree_2_dir.clone(), false)
        .await
        .unwrap();

    let worktrees = repo.worktrees().await.unwrap();
    assert_eq!(worktrees.len(), 1);
    assert_eq!(worktrees[0].path, PathBuf::from("/project"));
    assert!(!fs.is_dir(&worktree_2_dir).await);
}

#[gpui::test]
async fn test_checkpoints(executor: BackgroundExecutor) {
    let fs = FakeFs::new(executor);
    fs.insert_tree(
        path!("/"),
        json!({
            "bar": {
                "baz": "qux"
            },
            "foo": {
                ".git": {},
                "a": "lorem",
                "b": "ipsum",
            },
        }),
    )
    .await;
    fs.with_git_state(Path::new("/foo/.git"), true, |_git| {})
        .unwrap();
    let repository = fs
        .open_repo(Path::new("/foo/.git"), Some("git".as_ref()))
        .unwrap();

    let checkpoint_1 = repository.checkpoint().await.unwrap();
    fs.write(Path::new("/foo/b"), b"IPSUM").await.unwrap();
    fs.write(Path::new("/foo/c"), b"dolor").await.unwrap();
    let checkpoint_2 = repository.checkpoint().await.unwrap();
    let checkpoint_3 = repository.checkpoint().await.unwrap();

    assert!(
        repository
            .compare_checkpoints(checkpoint_2.clone(), checkpoint_3.clone())
            .await
            .unwrap()
    );
    assert!(
        !repository
            .compare_checkpoints(checkpoint_1.clone(), checkpoint_2.clone())
            .await
            .unwrap()
    );

    repository
        .restore_checkpoint(checkpoint_1.clone())
        .await
        .unwrap();
    assert_eq!(
        fs.files_with_contents(Path::new("")),
        [
            (Path::new(path!("/bar/baz")).into(), b"qux".into()),
            (Path::new(path!("/foo/a")).into(), b"lorem".into()),
            (Path::new(path!("/foo/b")).into(), b"ipsum".into())
        ]
    );

    // diff_checkpoints: identical checkpoints produce empty diff
    let diff = repository
        .diff_checkpoints(checkpoint_2.clone(), checkpoint_3.clone())
        .await
        .unwrap();
    assert!(
        diff.is_empty(),
        "identical checkpoints should produce empty diff"
    );

    // diff_checkpoints: different checkpoints produce non-empty diff
    let diff = repository
        .diff_checkpoints(checkpoint_1.clone(), checkpoint_2.clone())
        .await
        .unwrap();
    assert!(diff.contains("b"), "diff should mention changed file 'b'");
    assert!(diff.contains("c"), "diff should mention added file 'c'");
}

#[gpui::test]
async fn test_checkout_refuses_dangling_symlink_parent(cx: &mut TestAppContext) {
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/root"), json!({ ".git": {}, "src": {} }))
        .await;
    fs.insert_symlink(path!("/root/src/generated"), PathBuf::from("../missing"))
        .await;
    fs.set_head_and_index_for_repo(
        path!("/root/.git").as_ref(),
        &[("src/generated/file.txt", "tracked contents".into())],
    );

    let repository = fs.open_repo(path!("/root/.git").as_ref(), None).unwrap();
    let error = repository
        .checkout_files(
            "HEAD".to_string(),
            vec![repo_path("src/generated/file.txt")],
            Arc::new(Default::default()),
        )
        .await
        .unwrap_err();

    assert!(
        format!("{error:#}").contains("would be removed"),
        "unexpected error: {error:#}"
    );
    assert_eq!(
        fs.read_link(path!("/root/src/generated").as_ref())
            .await
            .unwrap(),
        PathBuf::from("../missing")
    );
}

#[gpui::test]
async fn test_checkout_resolves_conflicts(cx: &mut TestAppContext) {
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({ ".git": {}, "file.txt": "conflicted contents" }),
    )
    .await;
    fs.set_head_and_index_for_repo(
        path!("/root/.git").as_ref(),
        &[("file.txt", "original contents".into())],
    );
    fs.set_unmerged_paths_for_repo(
        path!("/root/.git").as_ref(),
        &[(
            repo_path("file.txt"),
            UnmergedStatus {
                first_head: UnmergedStatusCode::Updated,
                second_head: UnmergedStatusCode::Updated,
            },
        )],
    );

    let repository = fs.open_repo(path!("/root/.git").as_ref(), None).unwrap();
    repository
        .checkout_files(
            "HEAD".to_string(),
            vec![repo_path("file.txt")],
            Arc::new(Default::default()),
        )
        .await
        .unwrap();

    assert_eq!(
        fs.load(path!("/root/file.txt").as_ref()).await.unwrap(),
        "original contents"
    );
    let status = repository.status(&[repo_path("")]).await.unwrap();
    assert!(
        status.entries.is_empty(),
        "expected clean status after checkout, got {:?}",
        status.entries
    );
}

#[gpui::test]
async fn test_staging_and_unstaging_resolve_conflicts(cx: &mut TestAppContext) {
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({ ".git": {}, "file.txt": "resolved contents" }),
    )
    .await;
    fs.set_head_and_index_for_repo(
        path!("/root/.git").as_ref(),
        &[("file.txt", "original contents".into())],
    );
    let conflicted = || {
        (
            repo_path("file.txt"),
            UnmergedStatus {
                first_head: UnmergedStatusCode::Updated,
                second_head: UnmergedStatusCode::Updated,
            },
        )
    };
    fs.set_unmerged_paths_for_repo(path!("/root/.git").as_ref(), &[conflicted()]);

    let repository = fs.open_repo(path!("/root/.git").as_ref(), None).unwrap();
    repository
        .stage_paths(vec![repo_path("file.txt")], Arc::new(Default::default()))
        .await
        .unwrap();
    let status = repository.status(&[repo_path("")]).await.unwrap();
    assert_eq!(
        status.entries.as_ref(),
        [(repo_path("file.txt"), StatusCode::Modified.index())]
    );

    fs.set_unmerged_paths_for_repo(path!("/root/.git").as_ref(), &[conflicted()]);
    repository
        .unstage_paths(vec![repo_path("file.txt")], Arc::new(Default::default()))
        .await
        .unwrap();
    let status = repository.status(&[repo_path("")]).await.unwrap();
    assert_eq!(
        status.entries.as_ref(),
        [(repo_path("file.txt"), StatusCode::Modified.worktree())]
    );
}
