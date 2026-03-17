use fs::{FakeFs, Fs};
use gpui::{BackgroundExecutor, TestAppContext};
use serde_json::json;
use std::path::{Path, PathBuf};
use util::path;

#[gpui::test]
async fn test_fake_worktree_lifecycle(cx: &mut TestAppContext) {
    let worktree_dir_settings = &["../worktrees", ".git/zed-worktrees", "my-worktrees/"];

    for worktree_dir_setting in worktree_dir_settings {
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

        let expected_dir = git::repository::resolve_worktree_directory(
            Path::new("/project"),
            worktree_dir_setting,
        );

        // Create a worktree
        repo.create_worktree(
            "feature-branch".to_string(),
            expected_dir.clone(),
            Some("abc123".to_string()),
        )
        .await
        .unwrap();

        // List worktrees — should have main + one created
        let worktrees = repo.worktrees().await.unwrap();
        assert_eq!(worktrees.len(), 2);
        assert_eq!(worktrees[0].path, PathBuf::from("/project"));
        assert_eq!(
            worktrees[1].path,
            expected_dir.join("feature-branch"),
            "failed for worktree_directory setting: {worktree_dir_setting:?}"
        );
        assert_eq!(worktrees[1].ref_name.as_ref(), "refs/heads/feature-branch");
        assert_eq!(worktrees[1].sha.as_ref(), "abc123");

        // Directory should exist in FakeFs after create
        assert!(
            fs.is_dir(&expected_dir.join("feature-branch")).await,
            "worktree directory should be created in FakeFs for setting {worktree_dir_setting:?}"
        );

        // Create a second worktree (without explicit commit)
        repo.create_worktree("bugfix-branch".to_string(), expected_dir.clone(), None)
            .await
            .unwrap();

        let worktrees = repo.worktrees().await.unwrap();
        assert_eq!(worktrees.len(), 3);
        assert!(
            fs.is_dir(&expected_dir.join("bugfix-branch")).await,
            "second worktree directory should be created in FakeFs for setting {worktree_dir_setting:?}"
        );

        // Rename the first worktree
        repo.rename_worktree(
            expected_dir.join("feature-branch"),
            expected_dir.join("renamed-branch"),
        )
        .await
        .unwrap();

        let worktrees = repo.worktrees().await.unwrap();
        assert_eq!(worktrees.len(), 3);
        assert!(
            worktrees
                .iter()
                .any(|w| w.path == expected_dir.join("renamed-branch")),
            "renamed worktree should exist at new path for setting {worktree_dir_setting:?}"
        );
        assert!(
            worktrees
                .iter()
                .all(|w| w.path != expected_dir.join("feature-branch")),
            "old path should no longer exist for setting {worktree_dir_setting:?}"
        );

        // Directory should be moved in FakeFs after rename
        assert!(
            !fs.is_dir(&expected_dir.join("feature-branch")).await,
            "old worktree directory should not exist after rename for setting {worktree_dir_setting:?}"
        );
        assert!(
            fs.is_dir(&expected_dir.join("renamed-branch")).await,
            "new worktree directory should exist after rename for setting {worktree_dir_setting:?}"
        );

        // Rename a nonexistent worktree should fail
        let result = repo
            .rename_worktree(PathBuf::from("/nonexistent"), PathBuf::from("/somewhere"))
            .await;
        assert!(result.is_err());

        // Remove a worktree
        repo.remove_worktree(expected_dir.join("renamed-branch"), false)
            .await
            .unwrap();

        let worktrees = repo.worktrees().await.unwrap();
        assert_eq!(worktrees.len(), 2);
        assert_eq!(worktrees[0].path, PathBuf::from("/project"));
        assert_eq!(worktrees[1].path, expected_dir.join("bugfix-branch"));

        // Directory should be removed from FakeFs after remove
        assert!(
            !fs.is_dir(&expected_dir.join("renamed-branch")).await,
            "worktree directory should be removed from FakeFs for setting {worktree_dir_setting:?}"
        );

        // Remove a nonexistent worktree should fail
        let result = repo
            .remove_worktree(PathBuf::from("/nonexistent"), false)
            .await;
        assert!(result.is_err());

        // Remove the last worktree
        repo.remove_worktree(expected_dir.join("bugfix-branch"), false)
            .await
            .unwrap();

        let worktrees = repo.worktrees().await.unwrap();
        assert_eq!(worktrees.len(), 1);
        assert_eq!(worktrees[0].path, PathBuf::from("/project"));
        assert!(
            !fs.is_dir(&expected_dir.join("bugfix-branch")).await,
            "last worktree directory should be removed from FakeFs for setting {worktree_dir_setting:?}"
        );
    }
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
