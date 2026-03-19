use fs::{FakeFs, Fs};
use gpui::{BackgroundExecutor, TestAppContext};
use serde_json::json;
use std::path::{Path, PathBuf};
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
        "feature-branch".to_string(),
        worktree_1_dir.clone(),
        Some("abc123".to_string()),
    )
    .await
    .unwrap();

    // List worktrees — should have main + one created
    let worktrees = repo.worktrees().await.unwrap();
    assert_eq!(worktrees.len(), 2);
    assert_eq!(worktrees[0].path, PathBuf::from("/project"));
    assert_eq!(worktrees[1].path, worktree_1_dir);
    assert_eq!(worktrees[1].ref_name.as_ref(), "refs/heads/feature-branch");
    assert_eq!(worktrees[1].sha.as_ref(), "abc123");

    // Directory should exist in FakeFs after create
    assert!(fs.is_dir(&worktrees_dir.join("feature-branch")).await);

    // Create a second worktree (without explicit commit)
    let worktree_2_dir = worktrees_dir.join("bugfix-branch");
    repo.create_worktree("bugfix-branch".to_string(), worktree_2_dir.clone(), None)
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

    repository.restore_checkpoint(checkpoint_1).await.unwrap();
    assert_eq!(
        fs.files_with_contents(Path::new("")),
        [
            (Path::new(path!("/bar/baz")).into(), b"qux".into()),
            (Path::new(path!("/foo/a")).into(), b"lorem".into()),
            (Path::new(path!("/foo/b")).into(), b"ipsum".into())
        ]
    );
}
