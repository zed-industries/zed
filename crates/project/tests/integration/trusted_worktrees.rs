use std::{cell::RefCell, path::PathBuf, rc::Rc};

use collections::HashSet;
use gpui::{Entity, TestAppContext};
use serde_json::json;
use settings::SettingsStore;
use util::path;

use crate::{FakeFs, Project};

use project::{trusted_worktrees::*, worktree_store::WorktreeStore};

fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        if cx.try_global::<SettingsStore>().is_none() {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        }
        if cx.try_global::<TrustedWorktrees>().is_some() {
            cx.remove_global::<TrustedWorktrees>();
        }
    });
}

fn init_trust_global(
    worktree_store: Entity<WorktreeStore>,
    cx: &mut TestAppContext,
) -> Entity<TrustedWorktreesStore> {
    cx.update(|cx| {
        init(DbTrustedPaths::default(), cx);
        track_worktree_trust(worktree_store, None, None, None, cx);
        TrustedWorktrees::try_get_global(cx).expect("global should be set")
    })
}

#[cfg(windows)]
#[gpui::test]
async fn test_differently_cased_db_trusted_parent_path(cx: &mut TestAppContext) {
    use fs::RealFs;
    use std::path::Path;
    use util::{paths::SanitizedPath, test::TempTree};

    init_test(cx);
    cx.executor().allow_parking();

    let dir = TempTree::new(json!({ "Dev": { "one": {}, "two": {} } }));
    let root = SanitizedPath::new(dir.path()).as_path().to_path_buf();
    cx.update(|cx| {
        init(
            DbTrustedPaths::from_iter([(None, HashSet::from_iter([root.join("DEV")]))]),
            cx,
        )
    });
    let one = root.join("Dev").join("one");
    let two = root.join("Dev").join("two");
    let untracked_project =
        Project::test(RealFs::new(None, cx.executor()), [one.as_path()], cx).await;
    // Both tracked stores are registered before either opens a child of the trusted parent.
    let no_paths: [&Path; 0] = [];
    let project_a =
        Project::test_with_worktree_trust(RealFs::new(None, cx.executor()), no_paths, cx).await;
    let project_b =
        Project::test_with_worktree_trust(RealFs::new(None, cx.executor()), no_paths, cx).await;
    for (project, path) in [(&project_a, &one), (&project_b, &two)] {
        project
            .update(cx, |project, cx| {
                project.find_or_create_worktree(path, true, cx)
            })
            .await
            .unwrap();
        cx.run_until_parked();
    }

    let trusted_worktrees = cx.update(|cx| TrustedWorktrees::try_get_global(cx).unwrap());
    for (project, trusted) in [
        (project_a, true),
        (project_b, true),
        (untracked_project, false),
    ] {
        let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
        let worktree_id = worktree_store.read_with(cx, |store, cx| {
            store.worktrees().next().unwrap().read(cx).id()
        });
        trusted_worktrees.update(cx, |store, cx| {
            assert_eq!(
                store.can_trust(&worktree_store, worktree_id, cx),
                trusted,
                "a differently cased trusted parent should trust its subdirectories \
                 in every worktree store tracked for trust, and only there"
            );
        });
    }
}

#[cfg(windows)]
#[gpui::test]
async fn test_differently_cased_db_trusted_path_on_mapped_drive(cx: &mut TestAppContext) {
    use fs::{Fs as _, RealFs};
    use std::path::Path;
    use util::{ResultExt as _, paths::SanitizedPath, test::TempTree};

    /// A free drive letter mapped to the directory's administrative network share.
    struct MappedDrive(PathBuf);

    impl MappedDrive {
        #[allow(clippy::disallowed_methods)]
        fn new(target: &Path) -> Option<Self> {
            let share = format!(
                r"\\localhost\{}",
                target.to_string_lossy().replacen(':', "$", 1)
            );
            if !Path::new(&share).exists() {
                return None;
            }
            ('M'..='Z').rev().find_map(|letter| {
                let root = PathBuf::from(format!("{letter}:\\"));
                if root.exists() {
                    return None;
                }
                std::process::Command::new("net")
                    .args(["use", &format!("{letter}:"), &share, "/persistent:no"])
                    .output()
                    .ok()?
                    .status
                    .success()
                    .then(|| Self(root))
            })
        }
    }

    impl Drop for MappedDrive {
        #[allow(clippy::disallowed_methods)]
        fn drop(&mut self) {
            let drive = self.0.to_string_lossy();
            std::process::Command::new("net")
                .args(["use", drive.trim_end_matches('\\'), "/delete", "/y"])
                .output()
                .log_err();
        }
    }

    init_test(cx);
    cx.executor().allow_parking();

    let dir = TempTree::new(json!({ "Project": {} }));
    let Some(drive) = MappedDrive::new(SanitizedPath::new(dir.path()).as_path()) else {
        eprintln!("skipping: no drive letter could be mapped to a network share");
        return;
    };
    cx.update(|cx| {
        init(
            DbTrustedPaths::from_iter([(None, HashSet::from_iter([drive.0.join("project")]))]),
            cx,
        )
    });
    let fs = RealFs::new(None, cx.executor());
    // Zed opens a worktree by its canonical path, which keeps the mapped drive letter.
    let worktree_path = fs.canonicalize(&drive.0.join("project")).await.unwrap();
    assert_eq!(worktree_path, drive.0.join("Project"));
    let project = Project::test_with_worktree_trust(fs, [worktree_path.as_path()], cx).await;
    let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
    let worktree_id = worktree_store.read_with(cx, |store, cx| {
        store.worktrees().next().unwrap().read(cx).id()
    });

    let trusted_worktrees = cx.update(|cx| TrustedWorktrees::try_get_global(cx).unwrap());
    trusted_worktrees.update(cx, |store, cx| {
        assert!(
            store.can_trust(&worktree_store, worktree_id, cx),
            "a differently cased trusted path on a mapped drive should match its directory"
        );
    });
}

#[cfg(windows)]
#[gpui::test]
async fn test_differently_cased_db_trusted_path_longer_than_max_path(cx: &mut TestAppContext) {
    use fs::{Fs as _, RealFs};
    use util::{paths::SanitizedPath, test::TempTree};

    init_test(cx);
    cx.executor().allow_parking();

    let dir = TempTree::new(json!({}));
    let root = dir.path().join("review");
    // A long project trusted itself, and one trusted through its shorter parent: both get saved
    // in the extended-length `\\?\C:\` form that the project path keeps.
    let deep_parent = ["a", "b", "c"]
        .iter()
        .fold(root.join("deep"), |path, letter| {
            path.join(letter.repeat(90))
        });
    let short_parent = root.join("Dev");
    let fs = RealFs::new(None, cx.executor());
    for path in [
        deep_parent.join("Project"),
        short_parent.join("x".repeat(240)),
    ] {
        fs.create_dir(&path).await.unwrap();
    }
    cx.update(|cx| {
        init(
            DbTrustedPaths::from_iter([(
                None,
                HashSet::from_iter([deep_parent.join("project"), root.join("dev")]),
            )]),
            cx,
        )
    });
    let mut worktree_paths = Vec::new();
    for path in [
        deep_parent.join("project"),
        root.join("dev").join("x".repeat(240)),
    ] {
        let worktree_path = fs.canonicalize(&path).await.unwrap();
        assert!(
            worktree_path.as_os_str().len() > 260
                && SanitizedPath::new(&worktree_path).as_path() == worktree_path
                && worktree_path.to_string_lossy().starts_with(r"\\?\"),
            "worktree paths longer than `MAX_PATH` should keep their `\\\\?\\` form"
        );
        worktree_paths.push(worktree_path);
    }
    assert_eq!(worktree_paths[0], deep_parent.join("Project"));
    let project =
        Project::test_with_worktree_trust(fs, worktree_paths.iter().map(|path| path.as_path()), cx)
            .await;
    let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
    let worktree_ids = worktree_store.read_with(cx, |store, cx| {
        store
            .worktrees()
            .map(|worktree| worktree.read(cx).id())
            .collect::<Vec<_>>()
    });
    assert_eq!(worktree_ids.len(), 2);

    let trusted_worktrees = cx.update(|cx| TrustedWorktrees::try_get_global(cx).unwrap());
    trusted_worktrees.update(cx, |store, cx| {
        for worktree_id in worktree_ids {
            assert!(
                store.can_trust(&worktree_store, worktree_id, cx),
                "a differently cased trusted path should match its directory longer than `MAX_PATH`"
            );
        }
    });
}

#[cfg(windows)]
#[gpui::test]
async fn test_differently_cased_db_trusted_path_under_unlistable_parent(cx: &mut TestAppContext) {
    use fs::RealFs;
    use std::path::Path;
    use util::{paths::SanitizedPath, test::TempTree};

    /// Denies listing a directory's contents while still allowing to traverse it.
    struct UnlistableDirectory(PathBuf, String);

    impl UnlistableDirectory {
        #[allow(clippy::disallowed_methods)]
        fn new(path: &Path) -> Option<Self> {
            let user = format!(
                "{}\\{}",
                std::env::var("USERDOMAIN").ok()?,
                std::env::var("USERNAME").ok()?
            );
            let denied = std::process::Command::new("icacls")
                .arg(path)
                .args(["/deny", &format!("{user}:(RD)")])
                .output()
                .ok()?
                .status
                .success();
            let unlistable = denied.then(|| Self(path.to_path_buf(), user))?;
            // Elevated or privileged accounts may still list the directory.
            std::fs::read_dir(path).is_err().then_some(unlistable)
        }
    }

    impl Drop for UnlistableDirectory {
        #[allow(clippy::disallowed_methods)]
        fn drop(&mut self) {
            let restored = std::process::Command::new("icacls")
                .arg(&self.0)
                .args(["/remove:d", &self.1])
                .output();
            if !restored
                .as_ref()
                .is_ok_and(|output| output.status.success())
            {
                eprintln!("failed to restore listing {:?}: {restored:?}", self.0);
            }
        }
    }

    init_test(cx);
    cx.executor().allow_parking();

    let dir = TempTree::new(json!({ "Private": { "Project": {} } }));
    let root = SanitizedPath::new(dir.path()).as_path().to_path_buf();
    let Some(_unlistable) = UnlistableDirectory::new(&root.join("Private")) else {
        eprintln!("skipping: directory listing could not be denied");
        return;
    };
    cx.update(|cx| {
        init(
            DbTrustedPaths::from_iter([(
                None,
                HashSet::from_iter([root.join("Private").join("project")]),
            )]),
            cx,
        )
    });
    let project = Project::test_with_worktree_trust(
        RealFs::new(None, cx.executor()),
        [root.join("Private").join("Project").as_path()],
        cx,
    )
    .await;
    let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
    let worktree_id = worktree_store.read_with(cx, |store, cx| {
        store.worktrees().next().unwrap().read(cx).id()
    });

    let trusted_worktrees = cx.update(|cx| TrustedWorktrees::try_get_global(cx).unwrap());
    trusted_worktrees.update(cx, |store, cx| {
        assert!(
            store.can_trust(&worktree_store, worktree_id, cx),
            "a differently cased trusted path should match its directory \
             even when a parent directory cannot be listed"
        );
    });
}

#[cfg(windows)]
#[gpui::test]
async fn test_db_trusted_paths_on_case_sensitive_fake_fs(cx: &mut TestAppContext) {
    use std::path::Path;

    init_test(cx);

    // A case-sensitive file system, where distinct directories and link targets may have names
    // differing only by case.
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({ "safe": {}, "SAFE": {}, "only": {}, "LINK": {}, "PARENT": { "project": {} } }),
    )
    .await;
    fs.insert_symlink(path!("/root/link"), PathBuf::from(path!("/root/LINK")))
        .await;
    fs.insert_symlink(path!("/root/parent"), PathBuf::from(path!("/root/PARENT")))
        .await;
    cx.update(|cx| {
        init(
            DbTrustedPaths::from_iter([(
                None,
                HashSet::from_iter([
                    PathBuf::from(path!("/root/safe")),
                    PathBuf::from(path!("/root/ONLY")),
                    PathBuf::from(path!("/root/link")),
                    PathBuf::from(path!("/root/parent/project")),
                ]),
            )]),
            cx,
        )
    });
    let expected_trust = [
        (path!("/root/safe"), true, "`safe` is trusted"),
        (
            path!("/root/SAFE"),
            false,
            "trusting `safe` must not trust the distinct `SAFE` directory",
        ),
        (
            path!("/root/only"),
            false,
            "`ONLY` names no directory on a case-sensitive file system",
        ),
        (
            path!("/root/LINK"),
            false,
            "a trusted link must not extend trust to its target",
        ),
        (
            path!("/root/PARENT/project"),
            false,
            "a link in a parent directory must not extend trust to its target",
        ),
    ];
    let project = Project::test_with_worktree_trust(
        fs,
        expected_trust.iter().map(|(path, _, _)| Path::new(path)),
        cx,
    )
    .await;

    let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
    let worktree_ids = worktree_store.read_with(cx, |store, cx| {
        store
            .worktrees()
            .map(|worktree| {
                let worktree = worktree.read(cx);
                (worktree.abs_path().to_path_buf(), worktree.id())
            })
            .collect::<collections::HashMap<_, _>>()
    });
    let trusted_worktrees = cx.update(|cx| TrustedWorktrees::try_get_global(cx).unwrap());
    trusted_worktrees.update(cx, |store, cx| {
        for (path, trusted, message) in expected_trust {
            let worktree_id = worktree_ids[Path::new(path)];
            assert_eq!(
                store.can_trust(&worktree_store, worktree_id, cx),
                trusted,
                "{message}"
            );
        }
    });
}

#[cfg(windows)]
#[gpui::test]
async fn test_db_trusted_path_in_case_sensitive_directory(cx: &mut TestAppContext) {
    use fs::{Fs as _, RealFs};
    use util::{command::new_command, paths::SanitizedPath, test::TempTree};

    init_test(cx);
    cx.executor().allow_parking();

    let dir = TempTree::new(json!({ "case-test": {} }));
    let root = SanitizedPath::new(dir.path()).as_path().join("case-test");
    let enabled = new_command("fsutil")
        .args(["file", "setCaseSensitiveInfo"])
        .arg(&root)
        .arg("enable")
        .output()
        .await
        .is_ok_and(|output| output.status.success());
    if !enabled {
        eprintln!("skipping: per-directory case sensitivity is not available");
        return;
    }
    let fs = RealFs::new(None, cx.executor());
    for name in ["safe", "SAFE", "only", "LINK", "PARENT\\project"] {
        fs.create_dir(&root.join(name)).await.unwrap();
    }
    // Junctions whose targets differ from their own names only by case.
    for (link, target) in [("link", "LINK"), ("parent", "PARENT")] {
        fs.create_symlink(&root.join(link), root.join(target))
            .await
            .unwrap();
    }

    cx.update(|cx| {
        init(
            DbTrustedPaths::from_iter([(
                None,
                HashSet::from_iter([
                    root.join("safe"),
                    root.join("ONLY"),
                    root.join("link"),
                    root.join("parent").join("project"),
                ]),
            )]),
            cx,
        )
    });
    let project = Project::test_with_worktree_trust(
        fs,
        [
            root.join("safe").as_path(),
            root.join("SAFE").as_path(),
            root.join("only").as_path(),
            root.join("LINK").as_path(),
            root.join("PARENT").join("project").as_path(),
        ],
        cx,
    )
    .await;
    cx.run_until_parked();
    let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
    let worktree_ids = worktree_store.read_with(cx, |store, cx| {
        store
            .worktrees()
            .map(|worktree| {
                let worktree = worktree.read(cx);
                (worktree.abs_path().to_path_buf(), worktree.id())
            })
            .collect::<collections::HashMap<_, _>>()
    });

    let trusted_worktrees = cx.update(|cx| TrustedWorktrees::try_get_global(cx).unwrap());
    trusted_worktrees.update(cx, |store, cx| {
        let mut can_trust =
            |name: &str| store.can_trust(&worktree_store, worktree_ids[&root.join(name)], cx);
        assert!(can_trust("safe"));
        assert!(
            !can_trust("SAFE"),
            "trusting `safe` must not trust the distinct `SAFE` directory"
        );
        assert!(
            !can_trust("only"),
            "`ONLY` names no directory in a case-sensitive directory"
        );
        assert!(
            !can_trust("LINK"),
            "a trusted junction must not extend trust to its target"
        );
        assert!(
            !can_trust("PARENT\\project"),
            "a junction in a parent directory must not extend trust to its target"
        );
    });
}

#[gpui::test]
async fn test_single_worktree_trust(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/root"), json!({ "main.rs": "fn main() {}" }))
        .await;

    let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
    let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
    let worktree_id = worktree_store.read_with(cx, |store, cx| {
        store.worktrees().next().unwrap().read(cx).id()
    });

    let trusted_worktrees = init_trust_global(worktree_store.clone(), cx);

    let events: Rc<RefCell<Vec<TrustedWorktreesEvent>>> = Rc::default();
    cx.update({
        let events = events.clone();
        |cx| {
            cx.subscribe(&trusted_worktrees, move |_, event, _| {
                events.borrow_mut().push(match event {
                    TrustedWorktreesEvent::Trusted(host, paths) => {
                        TrustedWorktreesEvent::Trusted(host.clone(), paths.clone())
                    }
                    TrustedWorktreesEvent::Restricted(host, paths) => {
                        TrustedWorktreesEvent::Restricted(host.clone(), paths.clone())
                    }
                });
            })
        }
    })
    .detach();

    let can_trust = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, worktree_id, cx)
    });
    assert!(!can_trust, "worktree should be restricted by default");

    {
        let events = events.borrow();
        assert_eq!(events.len(), 1);
        match &events[0] {
            TrustedWorktreesEvent::Restricted(event_worktree_store, paths) => {
                assert_eq!(event_worktree_store, &worktree_store.downgrade());
                assert!(paths.contains(&PathTrust::Worktree(worktree_id)));
            }
            _ => panic!("expected Restricted event"),
        }
    }

    let has_restricted = trusted_worktrees.read_with(cx, |store, cx| {
        store.has_restricted_worktrees(&worktree_store, cx)
    });
    assert!(has_restricted, "should have restricted worktrees");

    let restricted = trusted_worktrees.read_with(cx, |trusted_worktrees, cx| {
        trusted_worktrees.restricted_worktrees(&worktree_store, cx)
    });
    assert!(restricted.iter().any(|(id, _)| *id == worktree_id));

    events.borrow_mut().clear();

    let can_trust_again = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, worktree_id, cx)
    });
    assert!(!can_trust_again, "worktree should still be restricted");
    assert!(
        events.borrow().is_empty(),
        "no duplicate Restricted event on repeated can_trust"
    );

    trusted_worktrees.update(cx, |store, cx| {
        store.trust(
            &worktree_store,
            HashSet::from_iter([PathTrust::Worktree(worktree_id)]),
            cx,
        );
    });

    {
        let events = events.borrow();
        assert_eq!(events.len(), 1);
        match &events[0] {
            TrustedWorktreesEvent::Trusted(event_worktree_store, paths) => {
                assert_eq!(event_worktree_store, &worktree_store.downgrade());
                assert!(paths.contains(&PathTrust::Worktree(worktree_id)));
            }
            _ => panic!("expected Trusted event"),
        }
    }

    let can_trust_after = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, worktree_id, cx)
    });
    assert!(can_trust_after, "worktree should be trusted after trust()");

    let has_restricted_after = trusted_worktrees.read_with(cx, |store, cx| {
        store.has_restricted_worktrees(&worktree_store, cx)
    });
    assert!(
        !has_restricted_after,
        "should have no restricted worktrees after trust"
    );

    let restricted_after = trusted_worktrees.read_with(cx, |trusted_worktrees, cx| {
        trusted_worktrees.restricted_worktrees(&worktree_store, cx)
    });
    assert!(
        restricted_after.is_empty(),
        "restricted set should be empty"
    );
}

#[gpui::test]
async fn test_single_file_worktree_trust(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/root"), json!({ "foo.rs": "fn foo() {}" }))
        .await;

    let project = Project::test(fs, [path!("/root/foo.rs").as_ref()], cx).await;
    let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
    let worktree_id = worktree_store.read_with(cx, |store, cx| {
        let worktree = store.worktrees().next().unwrap();
        let worktree = worktree.read(cx);
        assert!(worktree.is_single_file(), "expected single-file worktree");
        worktree.id()
    });

    let trusted_worktrees = init_trust_global(worktree_store.clone(), cx);

    let events: Rc<RefCell<Vec<TrustedWorktreesEvent>>> = Rc::default();
    cx.update({
        let events = events.clone();
        |cx| {
            cx.subscribe(&trusted_worktrees, move |_, event, _| {
                events.borrow_mut().push(match event {
                    TrustedWorktreesEvent::Trusted(host, paths) => {
                        TrustedWorktreesEvent::Trusted(host.clone(), paths.clone())
                    }
                    TrustedWorktreesEvent::Restricted(host, paths) => {
                        TrustedWorktreesEvent::Restricted(host.clone(), paths.clone())
                    }
                });
            })
        }
    })
    .detach();

    let can_trust = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, worktree_id, cx)
    });
    assert!(
        !can_trust,
        "single-file worktree should be restricted by default"
    );

    {
        let events = events.borrow();
        assert_eq!(events.len(), 1);
        match &events[0] {
            TrustedWorktreesEvent::Restricted(event_worktree_store, paths) => {
                assert_eq!(event_worktree_store, &worktree_store.downgrade());
                assert!(paths.contains(&PathTrust::Worktree(worktree_id)));
            }
            _ => panic!("expected Restricted event"),
        }
    }

    events.borrow_mut().clear();

    trusted_worktrees.update(cx, |store, cx| {
        store.trust(
            &worktree_store,
            HashSet::from_iter([PathTrust::Worktree(worktree_id)]),
            cx,
        );
    });

    {
        let events = events.borrow();
        assert_eq!(events.len(), 1);
        match &events[0] {
            TrustedWorktreesEvent::Trusted(event_worktree_store, paths) => {
                assert_eq!(event_worktree_store, &worktree_store.downgrade());
                assert!(paths.contains(&PathTrust::Worktree(worktree_id)));
            }
            _ => panic!("expected Trusted event"),
        }
    }

    let can_trust_after = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, worktree_id, cx)
    });
    assert!(
        can_trust_after,
        "single-file worktree should be trusted after trust()"
    );
}

#[gpui::test]
async fn test_multiple_single_file_worktrees_trust_one(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            "a.rs": "fn a() {}",
            "b.rs": "fn b() {}",
            "c.rs": "fn c() {}"
        }),
    )
    .await;

    let project = Project::test(
        fs,
        [
            path!("/root/a.rs").as_ref(),
            path!("/root/b.rs").as_ref(),
            path!("/root/c.rs").as_ref(),
        ],
        cx,
    )
    .await;
    let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
    let worktree_ids: Vec<_> = worktree_store.read_with(cx, |store, cx| {
        store
            .worktrees()
            .map(|worktree| {
                let worktree = worktree.read(cx);
                assert!(worktree.is_single_file());
                worktree.id()
            })
            .collect()
    });
    assert_eq!(worktree_ids.len(), 3);

    let trusted_worktrees = init_trust_global(worktree_store.clone(), cx);

    for &worktree_id in &worktree_ids {
        let can_trust = trusted_worktrees.update(cx, |store, cx| {
            store.can_trust(&worktree_store, worktree_id, cx)
        });
        assert!(
            !can_trust,
            "worktree {worktree_id:?} should be restricted initially"
        );
    }

    trusted_worktrees.update(cx, |store, cx| {
        store.trust(
            &worktree_store,
            HashSet::from_iter([PathTrust::Worktree(worktree_ids[1])]),
            cx,
        );
    });

    let can_trust_0 = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, worktree_ids[0], cx)
    });
    let can_trust_1 = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, worktree_ids[1], cx)
    });
    let can_trust_2 = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, worktree_ids[2], cx)
    });

    assert!(!can_trust_0, "worktree 0 should still be restricted");
    assert!(can_trust_1, "worktree 1 should be trusted");
    assert!(!can_trust_2, "worktree 2 should still be restricted");
}

#[gpui::test]
async fn test_two_directory_worktrees_separate_trust(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/projects"),
        json!({
            "project_a": { "main.rs": "fn main() {}" },
            "project_b": { "lib.rs": "pub fn lib() {}" }
        }),
    )
    .await;

    let project = Project::test(
        fs,
        [
            path!("/projects/project_a").as_ref(),
            path!("/projects/project_b").as_ref(),
        ],
        cx,
    )
    .await;
    let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
    let worktree_ids: Vec<_> = worktree_store.read_with(cx, |store, cx| {
        store
            .worktrees()
            .map(|worktree| {
                let worktree = worktree.read(cx);
                assert!(!worktree.is_single_file());
                worktree.id()
            })
            .collect()
    });
    assert_eq!(worktree_ids.len(), 2);

    let trusted_worktrees = init_trust_global(worktree_store.clone(), cx);

    let can_trust_a = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, worktree_ids[0], cx)
    });
    let can_trust_b = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, worktree_ids[1], cx)
    });
    assert!(!can_trust_a, "project_a should be restricted initially");
    assert!(!can_trust_b, "project_b should be restricted initially");

    trusted_worktrees.update(cx, |store, cx| {
        store.trust(
            &worktree_store,
            HashSet::from_iter([PathTrust::Worktree(worktree_ids[0])]),
            cx,
        );
    });

    let can_trust_a = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, worktree_ids[0], cx)
    });
    let can_trust_b = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, worktree_ids[1], cx)
    });
    assert!(can_trust_a, "project_a should be trusted after trust()");
    assert!(!can_trust_b, "project_b should still be restricted");

    trusted_worktrees.update(cx, |store, cx| {
        store.trust(
            &worktree_store,
            HashSet::from_iter([PathTrust::Worktree(worktree_ids[1])]),
            cx,
        );
    });

    let can_trust_a = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, worktree_ids[0], cx)
    });
    let can_trust_b = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, worktree_ids[1], cx)
    });
    assert!(can_trust_a, "project_a should remain trusted");
    assert!(can_trust_b, "project_b should now be trusted");
}

#[gpui::test]
async fn test_directory_worktree_trust_enables_single_file(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/"),
        json!({
            "project": { "main.rs": "fn main() {}" },
            "standalone.rs": "fn standalone() {}"
        }),
    )
    .await;

    let project = Project::test(
        fs,
        [path!("/project").as_ref(), path!("/standalone.rs").as_ref()],
        cx,
    )
    .await;
    let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
    let (dir_worktree_id, file_worktree_id) = worktree_store.read_with(cx, |store, cx| {
        let worktrees: Vec<_> = store.worktrees().collect();
        assert_eq!(worktrees.len(), 2);
        let (dir_worktree, file_worktree) = if worktrees[0].read(cx).is_single_file() {
            (&worktrees[1], &worktrees[0])
        } else {
            (&worktrees[0], &worktrees[1])
        };
        assert!(!dir_worktree.read(cx).is_single_file());
        assert!(file_worktree.read(cx).is_single_file());
        (dir_worktree.read(cx).id(), file_worktree.read(cx).id())
    });

    let trusted_worktrees = init_trust_global(worktree_store.clone(), cx);

    let can_trust_file = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, file_worktree_id, cx)
    });
    assert!(
        !can_trust_file,
        "single-file worktree should be restricted initially"
    );

    let can_trust_directory = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, dir_worktree_id, cx)
    });
    assert!(
        !can_trust_directory,
        "directory worktree should be restricted initially"
    );

    trusted_worktrees.update(cx, |store, cx| {
        store.trust(
            &worktree_store,
            HashSet::from_iter([PathTrust::Worktree(dir_worktree_id)]),
            cx,
        );
    });

    let can_trust_dir = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, dir_worktree_id, cx)
    });
    let can_trust_file_after = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, file_worktree_id, cx)
    });
    assert!(can_trust_dir, "directory worktree should be trusted");
    assert!(
        can_trust_file_after,
        "single-file worktree should be trusted after directory worktree trust"
    );
}

#[gpui::test]
async fn test_parent_path_trust_enables_single_file(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/"),
        json!({
            "project": { "main.rs": "fn main() {}" },
            "standalone.rs": "fn standalone() {}"
        }),
    )
    .await;

    let project = Project::test(
        fs,
        [path!("/project").as_ref(), path!("/standalone.rs").as_ref()],
        cx,
    )
    .await;
    let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
    let (dir_worktree_id, file_worktree_id) = worktree_store.read_with(cx, |store, cx| {
        let worktrees: Vec<_> = store.worktrees().collect();
        assert_eq!(worktrees.len(), 2);
        let (dir_worktree, file_worktree) = if worktrees[0].read(cx).is_single_file() {
            (&worktrees[1], &worktrees[0])
        } else {
            (&worktrees[0], &worktrees[1])
        };
        assert!(!dir_worktree.read(cx).is_single_file());
        assert!(file_worktree.read(cx).is_single_file());
        (dir_worktree.read(cx).id(), file_worktree.read(cx).id())
    });

    let trusted_worktrees = init_trust_global(worktree_store.clone(), cx);

    let can_trust_file = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, file_worktree_id, cx)
    });
    assert!(
        !can_trust_file,
        "single-file worktree should be restricted initially"
    );

    let can_trust_directory = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, dir_worktree_id, cx)
    });
    assert!(
        !can_trust_directory,
        "directory worktree should be restricted initially"
    );

    trusted_worktrees.update(cx, |store, cx| {
        store.trust(
            &worktree_store,
            HashSet::from_iter([PathTrust::AbsPath(PathBuf::from(path!("/project")))]),
            cx,
        );
    });

    let can_trust_dir = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, dir_worktree_id, cx)
    });
    let can_trust_file_after = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, file_worktree_id, cx)
    });
    assert!(
        can_trust_dir,
        "directory worktree should be trusted after its parent is trusted"
    );
    assert!(
        can_trust_file_after,
        "single-file worktree should be trusted after directory worktree trust via its parent directory trust"
    );
}

#[gpui::test]
async fn test_abs_path_trust_covers_multiple_worktrees(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            "project_a": { "main.rs": "fn main() {}" },
            "project_b": { "lib.rs": "pub fn lib() {}" }
        }),
    )
    .await;

    let project = Project::test(
        fs,
        [
            path!("/root/project_a").as_ref(),
            path!("/root/project_b").as_ref(),
        ],
        cx,
    )
    .await;
    let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
    let worktree_ids: Vec<_> = worktree_store.read_with(cx, |store, cx| {
        store
            .worktrees()
            .map(|worktree| worktree.read(cx).id())
            .collect()
    });
    assert_eq!(worktree_ids.len(), 2);

    let trusted_worktrees = init_trust_global(worktree_store.clone(), cx);

    for &worktree_id in &worktree_ids {
        let can_trust = trusted_worktrees.update(cx, |store, cx| {
            store.can_trust(&worktree_store, worktree_id, cx)
        });
        assert!(!can_trust, "worktree should be restricted initially");
    }

    trusted_worktrees.update(cx, |store, cx| {
        store.trust(
            &worktree_store,
            HashSet::from_iter([PathTrust::AbsPath(PathBuf::from(path!("/root")))]),
            cx,
        );
    });

    for &worktree_id in &worktree_ids {
        let can_trust = trusted_worktrees.update(cx, |store, cx| {
            store.can_trust(&worktree_store, worktree_id, cx)
        });
        assert!(
            can_trust,
            "worktree should be trusted after parent path trust"
        );
    }
}

#[gpui::test]
async fn test_auto_trust_all(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/"),
        json!({
            "project_a": { "main.rs": "fn main() {}" },
            "project_b": { "lib.rs": "pub fn lib() {}" },
            "single.rs": "fn single() {}"
        }),
    )
    .await;

    let project = Project::test(
        fs,
        [
            path!("/project_a").as_ref(),
            path!("/project_b").as_ref(),
            path!("/single.rs").as_ref(),
        ],
        cx,
    )
    .await;
    let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
    let worktree_ids: Vec<_> = worktree_store.read_with(cx, |store, cx| {
        store
            .worktrees()
            .map(|worktree| worktree.read(cx).id())
            .collect()
    });
    assert_eq!(worktree_ids.len(), 3);

    let trusted_worktrees = init_trust_global(worktree_store.clone(), cx);

    let events: Rc<RefCell<Vec<TrustedWorktreesEvent>>> = Rc::default();
    cx.update({
        let events = events.clone();
        |cx| {
            cx.subscribe(&trusted_worktrees, move |_, event, _| {
                events.borrow_mut().push(match event {
                    TrustedWorktreesEvent::Trusted(host, paths) => {
                        TrustedWorktreesEvent::Trusted(host.clone(), paths.clone())
                    }
                    TrustedWorktreesEvent::Restricted(host, paths) => {
                        TrustedWorktreesEvent::Restricted(host.clone(), paths.clone())
                    }
                });
            })
        }
    })
    .detach();

    for &worktree_id in &worktree_ids {
        let can_trust = trusted_worktrees.update(cx, |store, cx| {
            store.can_trust(&worktree_store, worktree_id, cx)
        });
        assert!(!can_trust, "worktree should be restricted initially");
    }

    let has_restricted = trusted_worktrees.read_with(cx, |store, cx| {
        store.has_restricted_worktrees(&worktree_store, cx)
    });
    assert!(has_restricted, "should have restricted worktrees");

    events.borrow_mut().clear();

    trusted_worktrees.update(cx, |store, cx| {
        store.auto_trust_all(cx);
    });

    for &worktree_id in &worktree_ids {
        let can_trust = trusted_worktrees.update(cx, |store, cx| {
            store.can_trust(&worktree_store, worktree_id, cx)
        });
        assert!(
            can_trust,
            "worktree {worktree_id:?} should be trusted after auto_trust_all"
        );
    }

    let has_restricted_after = trusted_worktrees.read_with(cx, |store, cx| {
        store.has_restricted_worktrees(&worktree_store, cx)
    });
    assert!(
        !has_restricted_after,
        "should have no restricted worktrees after auto_trust_all"
    );

    let trusted_event_count = events
        .borrow()
        .iter()
        .filter(|e| matches!(e, TrustedWorktreesEvent::Trusted(..)))
        .count();
    assert!(
        trusted_event_count > 0,
        "should have emitted Trusted events"
    );
}

#[gpui::test]
async fn test_trust_restrict_trust_cycle(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/root"), json!({ "main.rs": "fn main() {}" }))
        .await;

    let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
    let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
    let worktree_id = worktree_store.read_with(cx, |store, cx| {
        store.worktrees().next().unwrap().read(cx).id()
    });

    let trusted_worktrees = init_trust_global(worktree_store.clone(), cx);

    let events: Rc<RefCell<Vec<TrustedWorktreesEvent>>> = Rc::default();
    cx.update({
        let events = events.clone();
        |cx| {
            cx.subscribe(&trusted_worktrees, move |_, event, _| {
                events.borrow_mut().push(match event {
                    TrustedWorktreesEvent::Trusted(host, paths) => {
                        TrustedWorktreesEvent::Trusted(host.clone(), paths.clone())
                    }
                    TrustedWorktreesEvent::Restricted(host, paths) => {
                        TrustedWorktreesEvent::Restricted(host.clone(), paths.clone())
                    }
                });
            })
        }
    })
    .detach();

    let can_trust = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, worktree_id, cx)
    });
    assert!(!can_trust, "should be restricted initially");
    assert_eq!(events.borrow().len(), 1);
    events.borrow_mut().clear();

    trusted_worktrees.update(cx, |store, cx| {
        store.trust(
            &worktree_store,
            HashSet::from_iter([PathTrust::Worktree(worktree_id)]),
            cx,
        );
    });
    let can_trust = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, worktree_id, cx)
    });
    assert!(can_trust, "should be trusted after trust()");
    assert_eq!(events.borrow().len(), 1);
    assert!(matches!(
        &events.borrow()[0],
        TrustedWorktreesEvent::Trusted(..)
    ));
    events.borrow_mut().clear();

    trusted_worktrees.update(cx, |store, cx| {
        store.restrict(
            worktree_store.downgrade(),
            HashSet::from_iter([PathTrust::Worktree(worktree_id)]),
            cx,
        );
    });
    let can_trust = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, worktree_id, cx)
    });
    assert!(!can_trust, "should be restricted after restrict()");
    assert_eq!(events.borrow().len(), 1);
    assert!(matches!(
        &events.borrow()[0],
        TrustedWorktreesEvent::Restricted(..)
    ));

    let has_restricted = trusted_worktrees.read_with(cx, |store, cx| {
        store.has_restricted_worktrees(&worktree_store, cx)
    });
    assert!(has_restricted);
    events.borrow_mut().clear();

    trusted_worktrees.update(cx, |store, cx| {
        store.trust(
            &worktree_store,
            HashSet::from_iter([PathTrust::Worktree(worktree_id)]),
            cx,
        );
    });
    let can_trust = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, worktree_id, cx)
    });
    assert!(can_trust, "should be trusted again after second trust()");
    assert_eq!(events.borrow().len(), 1);
    assert!(matches!(
        &events.borrow()[0],
        TrustedWorktreesEvent::Trusted(..)
    ));

    let has_restricted = trusted_worktrees.read_with(cx, |store, cx| {
        store.has_restricted_worktrees(&worktree_store, cx)
    });
    assert!(!has_restricted);
}

#[gpui::test]
async fn test_multi_host_trust_isolation(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/"),
        json!({
            "local_project": { "main.rs": "fn main() {}" },
            "remote_project": { "lib.rs": "pub fn lib() {}" }
        }),
    )
    .await;

    let project = Project::test(
        fs,
        [
            path!("/local_project").as_ref(),
            path!("/remote_project").as_ref(),
        ],
        cx,
    )
    .await;
    let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
    let worktree_ids: Vec<_> = worktree_store.read_with(cx, |store, cx| {
        store
            .worktrees()
            .map(|worktree| worktree.read(cx).id())
            .collect()
    });
    assert_eq!(worktree_ids.len(), 2);
    let local_worktree = worktree_ids[0];
    let _remote_worktree = worktree_ids[1];

    let trusted_worktrees = init_trust_global(worktree_store.clone(), cx);

    let can_trust_local = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, local_worktree, cx)
    });
    assert!(!can_trust_local, "local worktree restricted on host_a");

    trusted_worktrees.update(cx, |store, cx| {
        store.trust(
            &worktree_store,
            HashSet::from_iter([PathTrust::Worktree(local_worktree)]),
            cx,
        );
    });

    let can_trust_local_after = trusted_worktrees.update(cx, |store, cx| {
        store.can_trust(&worktree_store, local_worktree, cx)
    });
    assert!(
        can_trust_local_after,
        "local worktree should be trusted on local host"
    );
}

#[gpui::test]
async fn test_invisible_worktree_stores_do_not_affect_trust(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/"),
        json!({
            "visible": { "main.rs": "fn main() {}" },
            "other": { "a.rs": "fn other() {}" },
            "invisible": { "b.rs": "fn invisible() {}" }
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/visible").as_ref()], cx).await;
    let worktree_store = project.read_with(cx, |project, _| project.worktree_store());
    let visible_worktree_id = worktree_store.read_with(cx, |store, cx| {
        store
            .worktrees()
            .find(|worktree| worktree.read(cx).root_dir().unwrap().ends_with("visible"))
            .expect("visible worktree")
            .read(cx)
            .id()
    });
    let trusted_worktrees = init_trust_global(worktree_store.clone(), cx);

    let events: Rc<RefCell<Vec<TrustedWorktreesEvent>>> = Rc::default();
    cx.update({
        let events = events.clone();
        |cx| {
            cx.subscribe(&trusted_worktrees, move |_, event, _| {
                events.borrow_mut().push(match event {
                    TrustedWorktreesEvent::Trusted(host, paths) => {
                        TrustedWorktreesEvent::Trusted(host.clone(), paths.clone())
                    }
                    TrustedWorktreesEvent::Restricted(host, paths) => {
                        TrustedWorktreesEvent::Restricted(host.clone(), paths.clone())
                    }
                });
            })
        }
    })
    .detach();

    assert!(
        !trusted_worktrees.update(cx, |store, cx| {
            store.can_trust(&worktree_store, visible_worktree_id, cx)
        }),
        "visible worktree should be restricted initially"
    );
    assert_eq!(
        HashSet::from_iter([(visible_worktree_id)]),
        trusted_worktrees.read_with(cx, |store, _| {
            store.restricted_worktrees_for_store(&worktree_store)
        }),
        "only visible worktree should be restricted",
    );

    let (new_visible_worktree, new_invisible_worktree) =
        worktree_store.update(cx, |worktree_store, cx| {
            let new_visible_worktree = worktree_store.create_worktree("/other", true, cx);
            let new_invisible_worktree = worktree_store.create_worktree("/invisible", false, cx);
            (new_visible_worktree, new_invisible_worktree)
        });
    let (new_visible_worktree, new_invisible_worktree) = (
        new_visible_worktree.await.unwrap(),
        new_invisible_worktree.await.unwrap(),
    );

    let new_visible_worktree_id =
        new_visible_worktree.read_with(cx, |new_visible_worktree, _| new_visible_worktree.id());
    assert!(
        !trusted_worktrees.update(cx, |store, cx| {
            store.can_trust(&worktree_store, new_visible_worktree_id, cx)
        }),
        "new visible worktree should be restricted initially",
    );
    assert!(
        trusted_worktrees.update(cx, |store, cx| {
            store.can_trust(&worktree_store, new_invisible_worktree.read(cx).id(), cx)
        }),
        "invisible worktree should be skipped",
    );
    assert_eq!(
        HashSet::from_iter([visible_worktree_id, new_visible_worktree_id]),
        trusted_worktrees.read_with(cx, |store, _| {
            store.restricted_worktrees_for_store(&worktree_store)
        }),
        "only visible worktrees should be restricted"
    );
}

#[gpui::test]
async fn test_windows_style_abs_path_trust_from_unix_client(cx: &mut TestAppContext) {
    use gpui::AppContext as _;
    use project::worktree_store::WorktreeIdCounter;
    use rpc::{AnyProtoClient, NoopProtoClient, proto};
    use util::paths::PathStyle;
    use worktree::Worktree;

    init_test(cx);

    let client = AnyProtoClient::new(NoopProtoClient::new());
    let worktree_store = cx.update(|cx| {
        cx.new(|_| {
            WorktreeStore::remote(
                true,
                client.clone(),
                1,
                PathStyle::Windows,
                WorktreeIdCounter::default(),
            )
        })
    });

    let make_worktree = |id: u64, name: &str, abs_path: &str, cx: &mut TestAppContext| {
        let metadata = proto::WorktreeMetadata {
            id,
            root_name: name.to_string(),
            visible: true,
            abs_path: abs_path.to_string(),
            root_repo_common_dir: None,
            root_repo_is_linked_worktree: false,
        };
        let worktree = cx.update(|cx| {
            Worktree::remote(
                1,
                clock::ReplicaId::new(1),
                metadata,
                client.clone(),
                PathStyle::Windows,
                cx,
            )
        });
        worktree.update(cx, |worktree, _| {
            worktree
                .as_remote()
                .unwrap()
                .update_from_remote(proto::UpdateWorktree {
                    project_id: 1,
                    worktree_id: id,
                    abs_path: abs_path.to_string(),
                    root_name: name.to_string(),
                    updated_entries: vec![proto::Entry {
                        id: 1,
                        is_dir: true,
                        path: "".to_string(),
                        inode: 1,
                        mtime: Some(proto::Timestamp {
                            seconds: 0,
                            nanos: 0,
                        }),
                        is_ignored: false,
                        is_hidden: false,
                        is_external: false,
                        is_fifo: false,
                        size: None,
                        canonical_path: None,
                        is_unloaded: false,
                    }],
                    removed_entries: vec![],
                    scan_id: 1,
                    is_last_update: true,
                    updated_repositories: vec![],
                    removed_repositories: vec![],
                    root_repo_common_dir: None,
                    root_repo_is_linked_worktree: false,
                });
        });
        cx.run_until_parked();
        worktree
    };

    let worktree_t1 = make_worktree(1, "t1", r"C:\Users\me\dev\wt\t1", cx);
    let worktree_t2 = make_worktree(2, "t2", r"C:\Users\me\dev\wt\t2", cx);
    let t1_id = worktree_t1.read_with(cx, |worktree, _| worktree.id());
    let t2_id = worktree_t2.read_with(cx, |worktree, _| worktree.id());
    worktree_store.update(cx, |store, cx| {
        store.add(&worktree_t1, cx);
        store.add(&worktree_t2, cx);
    });
    assert!(
        !worktree_t1.read_with(cx, |worktree, _| worktree.is_single_file()),
        "t1 must be a folder worktree for this test to be meaningful"
    );

    let trusted_worktrees = init_trust_global(worktree_store.clone(), cx);

    assert!(
        !trusted_worktrees.update(cx, |store, cx| store.can_trust(&worktree_store, t1_id, cx)),
        "t1 should be restricted by default"
    );

    trusted_worktrees.update(cx, |store, cx| {
        store.trust(
            &worktree_store,
            HashSet::from_iter([
                PathTrust::Worktree(t1_id),
                PathTrust::AbsPath(PathBuf::from(r"C:\Users\me\dev\wt")),
            ]),
            cx,
        );
    });

    assert!(
        trusted_worktrees.update(cx, |store, cx| store.can_trust(&worktree_store, t2_id, cx)),
        "t2 should be auto-trusted by the trusted parent scope C:\\Users\\me\\dev\\wt"
    );
}
