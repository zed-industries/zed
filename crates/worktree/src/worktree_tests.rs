use crate::{Entry, EntryKind, Event, PathChange, Worktree, WorktreeModelHandle};
use anyhow::Result;
use encoding_rs;
use fs::{FakeFs, Fs, RealFs, RemoveOptions};
use git::{DOT_GIT, GITIGNORE, REPO_EXCLUDE};
use gpui::{AppContext as _, BackgroundExecutor, BorrowAppContext, Context, Task, TestAppContext};
use parking_lot::Mutex;
use postage::stream::Stream;
use pretty_assertions::assert_eq;
use rand::prelude::*;

use serde_json::json;
use settings::SettingsStore;
use std::{
    env,
    fmt::Write,
    mem,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{
    ResultExt, path,
    paths::PathStyle,
    rel_path::{RelPath, rel_path},
    test::TempTree,
};

#[gpui::test]
async fn test_traversal(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/root",
        json!({
           ".gitignore": "a/b\n",
           "a": {
               "b": "",
               "c": "",
           }
        }),
    )
    .await;

    let tree = Worktree::local(
        Path::new("/root"),
        true,
        fs,
        Default::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();
    cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
        .await;

    tree.read_with(cx, |tree, _| {
        assert_eq!(
            tree.entries(false, 0)
                .map(|entry| entry.path.as_ref())
                .collect::<Vec<_>>(),
            vec![
                rel_path(""),
                rel_path(".gitignore"),
                rel_path("a"),
                rel_path("a/c"),
            ]
        );
        assert_eq!(
            tree.entries(true, 0)
                .map(|entry| entry.path.as_ref())
                .collect::<Vec<_>>(),
            vec![
                rel_path(""),
                rel_path(".gitignore"),
                rel_path("a"),
                rel_path("a/b"),
                rel_path("a/c"),
            ]
        );
    })
}

#[gpui::test(iterations = 10)]
async fn test_circular_symlinks(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/root",
        json!({
            "lib": {
                "a": {
                    "a.txt": ""
                },
                "b": {
                    "b.txt": ""
                }
            }
        }),
    )
    .await;
    fs.create_symlink("/root/lib/a/lib".as_ref(), "..".into())
        .await
        .unwrap();
    fs.create_symlink("/root/lib/b/lib".as_ref(), "..".into())
        .await
        .unwrap();

    let tree = Worktree::local(
        Path::new("/root"),
        true,
        fs.clone(),
        Default::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();

    cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
        .await;

    tree.read_with(cx, |tree, _| {
        assert_eq!(
            tree.entries(false, 0)
                .map(|entry| entry.path.as_ref())
                .collect::<Vec<_>>(),
            vec![
                rel_path(""),
                rel_path("lib"),
                rel_path("lib/a"),
                rel_path("lib/a/a.txt"),
                rel_path("lib/a/lib"),
                rel_path("lib/b"),
                rel_path("lib/b/b.txt"),
                rel_path("lib/b/lib"),
            ]
        );
    });

    fs.rename(
        Path::new("/root/lib/a/lib"),
        Path::new("/root/lib/a/lib-2"),
        Default::default(),
    )
    .await
    .unwrap();
    cx.executor().run_until_parked();
    tree.read_with(cx, |tree, _| {
        assert_eq!(
            tree.entries(false, 0)
                .map(|entry| entry.path.as_ref())
                .collect::<Vec<_>>(),
            vec![
                rel_path(""),
                rel_path("lib"),
                rel_path("lib/a"),
                rel_path("lib/a/a.txt"),
                rel_path("lib/a/lib-2"),
                rel_path("lib/b"),
                rel_path("lib/b/b.txt"),
                rel_path("lib/b/lib"),
            ]
        );
    });
}

#[gpui::test]
async fn test_symlinks_pointing_outside(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/root",
        json!({
            "dir1": {
                "deps": {
                    // symlinks here
                },
                "src": {
                    "a.rs": "",
                    "b.rs": "",
                },
            },
            "dir2": {
                "src": {
                    "c.rs": "",
                    "d.rs": "",
                }
            },
            "dir3": {
                "deps": {},
                "src": {
                    "e.rs": "",
                    "f.rs": "",
                },
            }
        }),
    )
    .await;

    // These symlinks point to directories outside of the worktree's root, dir1.
    fs.create_symlink("/root/dir1/deps/dep-dir2".as_ref(), "../../dir2".into())
        .await
        .unwrap();
    fs.create_symlink("/root/dir1/deps/dep-dir3".as_ref(), "../../dir3".into())
        .await
        .unwrap();

    let tree = Worktree::local(
        Path::new("/root/dir1"),
        true,
        fs.clone(),
        Default::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();

    cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
        .await;

    let tree_updates = Arc::new(Mutex::new(Vec::new()));
    tree.update(cx, |_, cx| {
        let tree_updates = tree_updates.clone();
        cx.subscribe(&tree, move |_, _, event, _| {
            if let Event::UpdatedEntries(update) = event {
                tree_updates.lock().extend(
                    update
                        .iter()
                        .map(|(path, _, change)| (path.clone(), *change)),
                );
            }
        })
        .detach();
    });

    // The symlinked directories are not scanned by default.
    tree.read_with(cx, |tree, _| {
        assert_eq!(
            tree.entries(true, 0)
                .map(|entry| (entry.path.as_ref(), entry.is_external))
                .collect::<Vec<_>>(),
            vec![
                (rel_path(""), false),
                (rel_path("deps"), false),
                (rel_path("deps/dep-dir2"), true),
                (rel_path("deps/dep-dir3"), true),
                (rel_path("src"), false),
                (rel_path("src/a.rs"), false),
                (rel_path("src/b.rs"), false),
            ]
        );

        assert_eq!(
            tree.entry_for_path(rel_path("deps/dep-dir2")).unwrap().kind,
            EntryKind::UnloadedDir
        );
    });

    // Expand one of the symlinked directories.
    tree.read_with(cx, |tree, _| {
        tree.as_local()
            .unwrap()
            .refresh_entries_for_paths(vec![rel_path("deps/dep-dir3").into()])
    })
    .recv()
    .await;

    // The expanded directory's contents are loaded. Subdirectories are
    // not scanned yet.
    tree.read_with(cx, |tree, _| {
        assert_eq!(
            tree.entries(true, 0)
                .map(|entry| (entry.path.as_ref(), entry.is_external))
                .collect::<Vec<_>>(),
            vec![
                (rel_path(""), false),
                (rel_path("deps"), false),
                (rel_path("deps/dep-dir2"), true),
                (rel_path("deps/dep-dir3"), true),
                (rel_path("deps/dep-dir3/deps"), true),
                (rel_path("deps/dep-dir3/src"), true),
                (rel_path("src"), false),
                (rel_path("src/a.rs"), false),
                (rel_path("src/b.rs"), false),
            ]
        );
    });
    assert_eq!(
        mem::take(&mut *tree_updates.lock()),
        &[
            (rel_path("deps/dep-dir3").into(), PathChange::Loaded),
            (rel_path("deps/dep-dir3/deps").into(), PathChange::Loaded),
            (rel_path("deps/dep-dir3/src").into(), PathChange::Loaded)
        ]
    );

    // Expand a subdirectory of one of the symlinked directories.
    tree.read_with(cx, |tree, _| {
        tree.as_local()
            .unwrap()
            .refresh_entries_for_paths(vec![rel_path("deps/dep-dir3/src").into()])
    })
    .recv()
    .await;

    // The expanded subdirectory's contents are loaded.
    tree.read_with(cx, |tree, _| {
        assert_eq!(
            tree.entries(true, 0)
                .map(|entry| (entry.path.as_ref(), entry.is_external))
                .collect::<Vec<_>>(),
            vec![
                (rel_path(""), false),
                (rel_path("deps"), false),
                (rel_path("deps/dep-dir2"), true),
                (rel_path("deps/dep-dir3"), true),
                (rel_path("deps/dep-dir3/deps"), true),
                (rel_path("deps/dep-dir3/src"), true),
                (rel_path("deps/dep-dir3/src/e.rs"), true),
                (rel_path("deps/dep-dir3/src/f.rs"), true),
                (rel_path("src"), false),
                (rel_path("src/a.rs"), false),
                (rel_path("src/b.rs"), false),
            ]
        );
    });

    assert_eq!(
        mem::take(&mut *tree_updates.lock()),
        &[
            (rel_path("deps/dep-dir3/src").into(), PathChange::Loaded),
            (
                rel_path("deps/dep-dir3/src/e.rs").into(),
                PathChange::Loaded
            ),
            (
                rel_path("deps/dep-dir3/src/f.rs").into(),
                PathChange::Loaded
            )
        ]
    );
}

#[cfg(target_os = "macos")]
#[gpui::test]
async fn test_renaming_case_only(cx: &mut TestAppContext) {
    cx.executor().allow_parking();
    init_test(cx);

    const OLD_NAME: &str = "aaa.rs";
    const NEW_NAME: &str = "AAA.rs";

    let fs = Arc::new(RealFs::new(None, cx.executor()));
    let temp_root = TempTree::new(json!({
        OLD_NAME: "",
    }));

    let tree = Worktree::local(
        temp_root.path(),
        true,
        fs.clone(),
        Default::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();

    cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
        .await;
    tree.read_with(cx, |tree, _| {
        assert_eq!(
            tree.entries(true, 0)
                .map(|entry| entry.path.as_ref())
                .collect::<Vec<_>>(),
            vec![rel_path(""), rel_path(OLD_NAME)]
        );
    });

    fs.rename(
        &temp_root.path().join(OLD_NAME),
        &temp_root.path().join(NEW_NAME),
        fs::RenameOptions {
            overwrite: true,
            ignore_if_exists: true,
            create_parents: false,
        },
    )
    .await
    .unwrap();

    tree.flush_fs_events(cx).await;

    tree.read_with(cx, |tree, _| {
        assert_eq!(
            tree.entries(true, 0)
                .map(|entry| entry.path.as_ref())
                .collect::<Vec<_>>(),
            vec![rel_path(""), rel_path(NEW_NAME)]
        );
    });
}

#[gpui::test]
async fn test_open_gitignored_files(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/root",
        json!({
            ".gitignore": "node_modules\n",
            "one": {
                "node_modules": {
                    "a": {
                        "a1.js": "a1",
                        "a2.js": "a2",
                    },
                    "b": {
                        "b1.js": "b1",
                        "b2.js": "b2",
                    },
                    "c": {
                        "c1.js": "c1",
                        "c2.js": "c2",
                    }
                },
            },
            "two": {
                "x.js": "",
                "y.js": "",
            },
        }),
    )
    .await;

    let tree = Worktree::local(
        Path::new("/root"),
        true,
        fs.clone(),
        Default::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();

    cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
        .await;

    tree.read_with(cx, |tree, _| {
        assert_eq!(
            tree.entries(true, 0)
                .map(|entry| (entry.path.as_ref(), entry.is_ignored))
                .collect::<Vec<_>>(),
            vec![
                (rel_path(""), false),
                (rel_path(".gitignore"), false),
                (rel_path("one"), false),
                (rel_path("one/node_modules"), true),
                (rel_path("two"), false),
                (rel_path("two/x.js"), false),
                (rel_path("two/y.js"), false),
            ]
        );
    });

    // Open a file that is nested inside of a gitignored directory that
    // has not yet been expanded.
    let prev_read_dir_count = fs.read_dir_call_count();
    let loaded = tree
        .update(cx, |tree, cx| {
            tree.load_file(rel_path("one/node_modules/b/b1.js"), cx)
        })
        .await
        .unwrap();

    tree.read_with(cx, |tree, _| {
        assert_eq!(
            tree.entries(true, 0)
                .map(|entry| (entry.path.as_ref(), entry.is_ignored))
                .collect::<Vec<_>>(),
            vec![
                (rel_path(""), false),
                (rel_path(".gitignore"), false),
                (rel_path("one"), false),
                (rel_path("one/node_modules"), true),
                (rel_path("one/node_modules/a"), true),
                (rel_path("one/node_modules/b"), true),
                (rel_path("one/node_modules/b/b1.js"), true),
                (rel_path("one/node_modules/b/b2.js"), true),
                (rel_path("one/node_modules/c"), true),
                (rel_path("two"), false),
                (rel_path("two/x.js"), false),
                (rel_path("two/y.js"), false),
            ]
        );

        assert_eq!(
            loaded.file.path.as_ref(),
            rel_path("one/node_modules/b/b1.js")
        );

        // Only the newly-expanded directories are scanned.
        assert_eq!(fs.read_dir_call_count() - prev_read_dir_count, 2);
    });

    // Open another file in a different subdirectory of the same
    // gitignored directory.
    let prev_read_dir_count = fs.read_dir_call_count();
    let loaded = tree
        .update(cx, |tree, cx| {
            tree.load_file(rel_path("one/node_modules/a/a2.js"), cx)
        })
        .await
        .unwrap();

    tree.read_with(cx, |tree, _| {
        assert_eq!(
            tree.entries(true, 0)
                .map(|entry| (entry.path.as_ref(), entry.is_ignored))
                .collect::<Vec<_>>(),
            vec![
                (rel_path(""), false),
                (rel_path(".gitignore"), false),
                (rel_path("one"), false),
                (rel_path("one/node_modules"), true),
                (rel_path("one/node_modules/a"), true),
                (rel_path("one/node_modules/a/a1.js"), true),
                (rel_path("one/node_modules/a/a2.js"), true),
                (rel_path("one/node_modules/b"), true),
                (rel_path("one/node_modules/b/b1.js"), true),
                (rel_path("one/node_modules/b/b2.js"), true),
                (rel_path("one/node_modules/c"), true),
                (rel_path("two"), false),
                (rel_path("two/x.js"), false),
                (rel_path("two/y.js"), false),
            ]
        );

        assert_eq!(
            loaded.file.path.as_ref(),
            rel_path("one/node_modules/a/a2.js")
        );

        // Only the newly-expanded directory is scanned.
        assert_eq!(fs.read_dir_call_count() - prev_read_dir_count, 1);
    });

    let path = PathBuf::from("/root/one/node_modules/c/lib");

    // No work happens when files and directories change within an unloaded directory.
    let prev_fs_call_count = fs.read_dir_call_count() + fs.metadata_call_count();
    // When we open a directory, we check each ancestor whether it's a git
    // repository. That means we have an fs.metadata call per ancestor that we
    // need to subtract here.
    let ancestors = path.ancestors().count();

    fs.create_dir(path.as_ref()).await.unwrap();
    cx.executor().run_until_parked();

    assert_eq!(
        fs.read_dir_call_count() + fs.metadata_call_count() - prev_fs_call_count - ancestors,
        0
    );
}

#[gpui::test]
async fn test_dirs_no_longer_ignored(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/root",
        json!({
            ".gitignore": "node_modules\n",
            "a": {
                "a.js": "",
            },
            "b": {
                "b.js": "",
            },
            "node_modules": {
                "c": {
                    "c.js": "",
                },
                "d": {
                    "d.js": "",
                    "e": {
                        "e1.js": "",
                        "e2.js": "",
                    },
                    "f": {
                        "f1.js": "",
                        "f2.js": "",
                    }
                },
            },
        }),
    )
    .await;

    let tree = Worktree::local(
        Path::new("/root"),
        true,
        fs.clone(),
        Default::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();

    cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
        .await;

    // Open a file within the gitignored directory, forcing some of its
    // subdirectories to be read, but not all.
    let read_dir_count_1 = fs.read_dir_call_count();
    tree.read_with(cx, |tree, _| {
        tree.as_local()
            .unwrap()
            .refresh_entries_for_paths(vec![rel_path("node_modules/d/d.js").into()])
    })
    .recv()
    .await;

    // Those subdirectories are now loaded.
    tree.read_with(cx, |tree, _| {
        assert_eq!(
            tree.entries(true, 0)
                .map(|e| (e.path.as_ref(), e.is_ignored))
                .collect::<Vec<_>>(),
            &[
                (rel_path(""), false),
                (rel_path(".gitignore"), false),
                (rel_path("a"), false),
                (rel_path("a/a.js"), false),
                (rel_path("b"), false),
                (rel_path("b/b.js"), false),
                (rel_path("node_modules"), true),
                (rel_path("node_modules/c"), true),
                (rel_path("node_modules/d"), true),
                (rel_path("node_modules/d/d.js"), true),
                (rel_path("node_modules/d/e"), true),
                (rel_path("node_modules/d/f"), true),
            ]
        );
    });
    let read_dir_count_2 = fs.read_dir_call_count();
    assert_eq!(read_dir_count_2 - read_dir_count_1, 2);

    // Update the gitignore so that node_modules is no longer ignored,
    // but a subdirectory is ignored
    fs.save("/root/.gitignore".as_ref(), &"e".into(), Default::default())
        .await
        .unwrap();
    cx.executor().run_until_parked();

    // All of the directories that are no longer ignored are now loaded.
    tree.read_with(cx, |tree, _| {
        assert_eq!(
            tree.entries(true, 0)
                .map(|e| (e.path.as_ref(), e.is_ignored))
                .collect::<Vec<_>>(),
            &[
                (rel_path(""), false),
                (rel_path(".gitignore"), false),
                (rel_path("a"), false),
                (rel_path("a/a.js"), false),
                (rel_path("b"), false),
                (rel_path("b/b.js"), false),
                // This directory is no longer ignored
                (rel_path("node_modules"), false),
                (rel_path("node_modules/c"), false),
                (rel_path("node_modules/c/c.js"), false),
                (rel_path("node_modules/d"), false),
                (rel_path("node_modules/d/d.js"), false),
                // This subdirectory is now ignored
                (rel_path("node_modules/d/e"), true),
                (rel_path("node_modules/d/f"), false),
                (rel_path("node_modules/d/f/f1.js"), false),
                (rel_path("node_modules/d/f/f2.js"), false),
            ]
        );
    });

    // Each of the newly-loaded directories is scanned only once.
    let read_dir_count_3 = fs.read_dir_call_count();
    assert_eq!(read_dir_count_3 - read_dir_count_2, 2);
}

#[gpui::test]
async fn test_write_file(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let dir = TempTree::new(json!({
        ".git": {},
        ".gitignore": "ignored-dir\n",
        "tracked-dir": {},
        "ignored-dir": {}
    }));

    let worktree = Worktree::local(
        dir.path(),
        true,
        Arc::new(RealFs::new(None, cx.executor())),
        Default::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();

    #[cfg(not(target_os = "macos"))]
    fs::fs_watcher::global(|_| {}).unwrap();

    cx.read(|cx| worktree.read(cx).as_local().unwrap().scan_complete())
        .await;
    worktree.flush_fs_events(cx).await;

    worktree
        .update(cx, |tree, cx| {
            tree.write_file(
                rel_path("tracked-dir/file.txt").into(),
                "hello".into(),
                Default::default(),
                encoding_rs::UTF_8,
                false,
                cx,
            )
        })
        .await
        .unwrap();
    worktree
        .update(cx, |tree, cx| {
            tree.write_file(
                rel_path("ignored-dir/file.txt").into(),
                "world".into(),
                Default::default(),
                encoding_rs::UTF_8,
                false,
                cx,
            )
        })
        .await
        .unwrap();
    worktree.read_with(cx, |tree, _| {
        let tracked = tree
            .entry_for_path(rel_path("tracked-dir/file.txt"))
            .unwrap();
        let ignored = tree
            .entry_for_path(rel_path("ignored-dir/file.txt"))
            .unwrap();
        assert!(!tracked.is_ignored);
        assert!(ignored.is_ignored);
    });
}

#[gpui::test]
async fn test_file_scan_inclusions(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let dir = TempTree::new(json!({
        ".gitignore": "**/target\n/node_modules\ntop_level.txt\n",
        "target": {
            "index": "blah2"
        },
        "node_modules": {
            ".DS_Store": "",
            "prettier": {
                "package.json": "{}",
            },
            "package.json": "//package.json"
        },
        "src": {
            ".DS_Store": "",
            "foo": {
                "foo.rs": "mod another;\n",
                "another.rs": "// another",
            },
            "bar": {
                "bar.rs": "// bar",
            },
            "lib.rs": "mod foo;\nmod bar;\n",
        },
        "top_level.txt": "top level file",
        ".DS_Store": "",
    }));
    cx.update(|cx| {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.worktree.file_scan_exclusions = Some(vec![]);
                settings.project.worktree.file_scan_inclusions = Some(vec![
                    "node_modules/**/package.json".to_string(),
                    "**/.DS_Store".to_string(),
                ]);
            });
        });
    });

    let tree = Worktree::local(
        dir.path(),
        true,
        Arc::new(RealFs::new(None, cx.executor())),
        Default::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();
    cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
        .await;
    tree.flush_fs_events(cx).await;
    tree.read_with(cx, |tree, _| {
        // Assert that file_scan_inclusions overrides  file_scan_exclusions.
        check_worktree_entries(
            tree,
            &[],
            &["target", "node_modules"],
            &["src/lib.rs", "src/bar/bar.rs", ".gitignore"],
            &[
                "node_modules/prettier/package.json",
                ".DS_Store",
                "node_modules/.DS_Store",
                "src/.DS_Store",
            ],
        )
    });
}

#[gpui::test]
async fn test_file_scan_exclusions_overrules_inclusions(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let dir = TempTree::new(json!({
        ".gitignore": "**/target\n/node_modules\n",
        "target": {
            "index": "blah2"
        },
        "node_modules": {
            ".DS_Store": "",
            "prettier": {
                "package.json": "{}",
            },
        },
        "src": {
            ".DS_Store": "",
            "foo": {
                "foo.rs": "mod another;\n",
                "another.rs": "// another",
            },
        },
        ".DS_Store": "",
    }));

    cx.update(|cx| {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.worktree.file_scan_exclusions =
                    Some(vec!["**/.DS_Store".to_string()]);
                settings.project.worktree.file_scan_inclusions =
                    Some(vec!["**/.DS_Store".to_string()]);
            });
        });
    });

    let tree = Worktree::local(
        dir.path(),
        true,
        Arc::new(RealFs::new(None, cx.executor())),
        Default::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();
    cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
        .await;
    tree.flush_fs_events(cx).await;
    tree.read_with(cx, |tree, _| {
        // Assert that file_scan_inclusions overrides  file_scan_exclusions.
        check_worktree_entries(
            tree,
            &[".DS_Store, src/.DS_Store"],
            &["target", "node_modules"],
            &["src/foo/another.rs", "src/foo/foo.rs", ".gitignore"],
            &[],
        )
    });
}

#[gpui::test]
async fn test_file_scan_inclusions_reindexes_on_setting_change(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let dir = TempTree::new(json!({
        ".gitignore": "**/target\n/node_modules/\n",
        "target": {
            "index": "blah2"
        },
        "node_modules": {
            ".DS_Store": "",
            "prettier": {
                "package.json": "{}",
            },
        },
        "src": {
            ".DS_Store": "",
            "foo": {
                "foo.rs": "mod another;\n",
                "another.rs": "// another",
            },
        },
        ".DS_Store": "",
    }));

    cx.update(|cx| {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.worktree.file_scan_exclusions = Some(vec![]);
                settings.project.worktree.file_scan_inclusions =
                    Some(vec!["node_modules/**".to_string()]);
            });
        });
    });
    let tree = Worktree::local(
        dir.path(),
        true,
        Arc::new(RealFs::new(None, cx.executor())),
        Default::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();
    cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
        .await;
    tree.flush_fs_events(cx).await;

    tree.read_with(cx, |tree, _| {
        assert!(
            tree.entry_for_path(rel_path("node_modules"))
                .is_some_and(|f| f.is_always_included)
        );
        assert!(
            tree.entry_for_path(rel_path("node_modules/prettier/package.json"))
                .is_some_and(|f| f.is_always_included)
        );
    });

    cx.update(|cx| {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.worktree.file_scan_exclusions = Some(vec![]);
                settings.project.worktree.file_scan_inclusions = Some(vec![]);
            });
        });
    });
    cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
        .await;
    tree.flush_fs_events(cx).await;

    tree.read_with(cx, |tree, _| {
        assert!(
            tree.entry_for_path(rel_path("node_modules"))
                .is_some_and(|f| !f.is_always_included)
        );
        assert!(
            tree.entry_for_path(rel_path("node_modules/prettier/package.json"))
                .is_some_and(|f| !f.is_always_included)
        );
    });
}

#[gpui::test]
async fn test_file_scan_exclusions(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let dir = TempTree::new(json!({
        ".gitignore": "**/target\n/node_modules\n",
        "target": {
            "index": "blah2"
        },
        "node_modules": {
            ".DS_Store": "",
            "prettier": {
                "package.json": "{}",
            },
        },
        "src": {
            ".DS_Store": "",
            "foo": {
                "foo.rs": "mod another;\n",
                "another.rs": "// another",
            },
            "bar": {
                "bar.rs": "// bar",
            },
            "lib.rs": "mod foo;\nmod bar;\n",
        },
        ".DS_Store": "",
    }));
    cx.update(|cx| {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.worktree.file_scan_exclusions =
                    Some(vec!["**/foo/**".to_string(), "**/.DS_Store".to_string()]);
            });
        });
    });

    let tree = Worktree::local(
        dir.path(),
        true,
        Arc::new(RealFs::new(None, cx.executor())),
        Default::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();
    cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
        .await;
    tree.flush_fs_events(cx).await;
    tree.read_with(cx, |tree, _| {
        check_worktree_entries(
            tree,
            &[
                "src/foo/foo.rs",
                "src/foo/another.rs",
                "node_modules/.DS_Store",
                "src/.DS_Store",
                ".DS_Store",
            ],
            &["target", "node_modules"],
            &["src/lib.rs", "src/bar/bar.rs", ".gitignore"],
            &[],
        )
    });

    cx.update(|cx| {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.worktree.file_scan_exclusions =
                    Some(vec!["**/node_modules/**".to_string()]);
            });
        });
    });
    tree.flush_fs_events(cx).await;
    cx.executor().run_until_parked();
    tree.read_with(cx, |tree, _| {
        check_worktree_entries(
            tree,
            &[
                "node_modules/prettier/package.json",
                "node_modules/.DS_Store",
                "node_modules",
            ],
            &["target"],
            &[
                ".gitignore",
                "src/lib.rs",
                "src/bar/bar.rs",
                "src/foo/foo.rs",
                "src/foo/another.rs",
                "src/.DS_Store",
                ".DS_Store",
            ],
            &[],
        )
    });
}

#[gpui::test]
async fn test_hidden_files(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let dir = TempTree::new(json!({
        ".gitignore": "**/target\n",
        ".hidden_file": "content",
        ".hidden_dir": {
            "nested.rs": "code",
        },
        "src": {
            "visible.rs": "code",
        },
        "logs": {
            "app.log": "logs",
            "debug.log": "logs",
        },
        "visible.txt": "content",
    }));

    let tree = Worktree::local(
        dir.path(),
        true,
        Arc::new(RealFs::new(None, cx.executor())),
        Default::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();
    cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
        .await;
    tree.flush_fs_events(cx).await;

    tree.read_with(cx, |tree, _| {
        assert_eq!(
            tree.entries(true, 0)
                .map(|entry| (entry.path.as_ref(), entry.is_hidden))
                .collect::<Vec<_>>(),
            vec![
                (rel_path(""), false),
                (rel_path(".gitignore"), true),
                (rel_path(".hidden_dir"), true),
                (rel_path(".hidden_dir/nested.rs"), true),
                (rel_path(".hidden_file"), true),
                (rel_path("logs"), false),
                (rel_path("logs/app.log"), false),
                (rel_path("logs/debug.log"), false),
                (rel_path("src"), false),
                (rel_path("src/visible.rs"), false),
                (rel_path("visible.txt"), false),
            ]
        );
    });

    cx.update(|cx| {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.worktree.hidden_files = Some(vec!["**/*.log".to_string()]);
            });
        });
    });
    tree.flush_fs_events(cx).await;
    cx.executor().run_until_parked();

    tree.read_with(cx, |tree, _| {
        assert_eq!(
            tree.entries(true, 0)
                .map(|entry| (entry.path.as_ref(), entry.is_hidden))
                .collect::<Vec<_>>(),
            vec![
                (rel_path(""), false),
                (rel_path(".gitignore"), false),
                (rel_path(".hidden_dir"), false),
                (rel_path(".hidden_dir/nested.rs"), false),
                (rel_path(".hidden_file"), false),
                (rel_path("logs"), false),
                (rel_path("logs/app.log"), true),
                (rel_path("logs/debug.log"), true),
                (rel_path("src"), false),
                (rel_path("src/visible.rs"), false),
                (rel_path("visible.txt"), false),
            ]
        );
    });
}

#[gpui::test]
async fn test_fs_events_in_exclusions(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let dir = TempTree::new(json!({
        ".git": {
            "HEAD": "ref: refs/heads/main\n",
            "foo": "bar",
        },
        ".gitignore": "**/target\n/node_modules\ntest_output\n",
        "target": {
            "index": "blah2"
        },
        "node_modules": {
            ".DS_Store": "",
            "prettier": {
                "package.json": "{}",
            },
        },
        "src": {
            ".DS_Store": "",
            "foo": {
                "foo.rs": "mod another;\n",
                "another.rs": "// another",
            },
            "bar": {
                "bar.rs": "// bar",
            },
            "lib.rs": "mod foo;\nmod bar;\n",
        },
        ".DS_Store": "",
    }));
    cx.update(|cx| {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.worktree.file_scan_exclusions = Some(vec![
                    "**/.git".to_string(),
                    "node_modules/".to_string(),
                    "build_output".to_string(),
                ]);
            });
        });
    });

    let tree = Worktree::local(
        dir.path(),
        true,
        Arc::new(RealFs::new(None, cx.executor())),
        Default::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();
    cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
        .await;
    tree.flush_fs_events(cx).await;
    tree.read_with(cx, |tree, _| {
        check_worktree_entries(
            tree,
            &[
                ".git/HEAD",
                ".git/foo",
                "node_modules",
                "node_modules/.DS_Store",
                "node_modules/prettier",
                "node_modules/prettier/package.json",
            ],
            &["target"],
            &[
                ".DS_Store",
                "src/.DS_Store",
                "src/lib.rs",
                "src/foo/foo.rs",
                "src/foo/another.rs",
                "src/bar/bar.rs",
                ".gitignore",
            ],
            &[],
        )
    });

    let new_excluded_dir = dir.path().join("build_output");
    let new_ignored_dir = dir.path().join("test_output");
    std::fs::create_dir_all(&new_excluded_dir)
        .unwrap_or_else(|e| panic!("Failed to create a {new_excluded_dir:?} directory: {e}"));
    std::fs::create_dir_all(&new_ignored_dir)
        .unwrap_or_else(|e| panic!("Failed to create a {new_ignored_dir:?} directory: {e}"));
    let node_modules_dir = dir.path().join("node_modules");
    let dot_git_dir = dir.path().join(".git");
    let src_dir = dir.path().join("src");
    for existing_dir in [&node_modules_dir, &dot_git_dir, &src_dir] {
        assert!(
            existing_dir.is_dir(),
            "Expect {existing_dir:?} to be present in the FS already"
        );
    }

    for directory_for_new_file in [
        new_excluded_dir,
        new_ignored_dir,
        node_modules_dir,
        dot_git_dir,
        src_dir,
    ] {
        std::fs::write(directory_for_new_file.join("new_file"), "new file contents")
            .unwrap_or_else(|e| {
                panic!("Failed to create in {directory_for_new_file:?} a new file: {e}")
            });
    }
    tree.flush_fs_events(cx).await;

    tree.read_with(cx, |tree, _| {
        check_worktree_entries(
            tree,
            &[
                ".git/HEAD",
                ".git/foo",
                ".git/new_file",
                "node_modules",
                "node_modules/.DS_Store",
                "node_modules/prettier",
                "node_modules/prettier/package.json",
                "node_modules/new_file",
                "build_output",
                "build_output/new_file",
                "test_output/new_file",
            ],
            &["target", "test_output"],
            &[
                ".DS_Store",
                "src/.DS_Store",
                "src/lib.rs",
                "src/foo/foo.rs",
                "src/foo/another.rs",
                "src/bar/bar.rs",
                "src/new_file",
                ".gitignore",
            ],
            &[],
        )
    });
}

#[gpui::test]
async fn test_fs_events_in_dot_git_worktree(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let dir = TempTree::new(json!({
        ".git": {
            "HEAD": "ref: refs/heads/main\n",
            "foo": "foo contents",
        },
    }));
    let dot_git_worktree_dir = dir.path().join(".git");

    let tree = Worktree::local(
        dot_git_worktree_dir.clone(),
        true,
        Arc::new(RealFs::new(None, cx.executor())),
        Default::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();
    cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
        .await;
    tree.flush_fs_events(cx).await;
    tree.read_with(cx, |tree, _| {
        check_worktree_entries(tree, &[], &["HEAD", "foo"], &[], &[])
    });

    std::fs::write(dot_git_worktree_dir.join("new_file"), "new file contents")
        .unwrap_or_else(|e| panic!("Failed to create in {dot_git_worktree_dir:?} a new file: {e}"));
    tree.flush_fs_events(cx).await;
    tree.read_with(cx, |tree, _| {
        check_worktree_entries(tree, &[], &["HEAD", "foo", "new_file"], &[], &[])
    });
}

#[gpui::test(iterations = 30)]
async fn test_create_directory_during_initial_scan(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/root",
        json!({
            "b": {},
            "c": {},
            "d": {},
        }),
    )
    .await;

    let tree = Worktree::local(
        "/root".as_ref(),
        true,
        fs,
        Default::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();

    let snapshot1 = tree.update(cx, |tree, cx| {
        let tree = tree.as_local_mut().unwrap();
        let snapshot = Arc::new(Mutex::new(tree.snapshot()));
        tree.observe_updates(0, cx, {
            let snapshot = snapshot.clone();
            let settings = tree.settings();
            move |update| {
                snapshot
                    .lock()
                    .apply_remote_update(update, &settings.file_scan_inclusions);
                async { true }
            }
        });
        snapshot
    });

    let entry = tree
        .update(cx, |tree, cx| {
            tree.as_local_mut()
                .unwrap()
                .create_entry(rel_path("a/e").into(), true, None, cx)
        })
        .await
        .unwrap()
        .into_included()
        .unwrap();
    assert!(entry.is_dir());

    cx.executor().run_until_parked();
    tree.read_with(cx, |tree, _| {
        assert_eq!(
            tree.entry_for_path(rel_path("a/e")).unwrap().kind,
            EntryKind::Dir
        );
    });

    let snapshot2 = tree.update(cx, |tree, _| tree.as_local().unwrap().snapshot());
    assert_eq!(
        snapshot1.lock().entries(true, 0).collect::<Vec<_>>(),
        snapshot2.entries(true, 0).collect::<Vec<_>>()
    );
}

#[gpui::test]
async fn test_create_dir_all_on_create_entry(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();

    let fs_fake = FakeFs::new(cx.background_executor.clone());
    fs_fake
        .insert_tree(
            "/root",
            json!({
                "a": {},
            }),
        )
        .await;

    let tree_fake = Worktree::local(
        "/root".as_ref(),
        true,
        fs_fake,
        Default::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();

    let entry = tree_fake
        .update(cx, |tree, cx| {
            tree.as_local_mut().unwrap().create_entry(
                rel_path("a/b/c/d.txt").into(),
                false,
                None,
                cx,
            )
        })
        .await
        .unwrap()
        .into_included()
        .unwrap();
    assert!(entry.is_file());

    cx.executor().run_until_parked();
    tree_fake.read_with(cx, |tree, _| {
        assert!(
            tree.entry_for_path(rel_path("a/b/c/d.txt"))
                .unwrap()
                .is_file()
        );
        assert!(tree.entry_for_path(rel_path("a/b/c")).unwrap().is_dir());
        assert!(tree.entry_for_path(rel_path("a/b")).unwrap().is_dir());
    });

    let fs_real = Arc::new(RealFs::new(None, cx.executor()));
    let temp_root = TempTree::new(json!({
        "a": {}
    }));

    let tree_real = Worktree::local(
        temp_root.path(),
        true,
        fs_real,
        Default::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();

    let entry = tree_real
        .update(cx, |tree, cx| {
            tree.as_local_mut().unwrap().create_entry(
                rel_path("a/b/c/d.txt").into(),
                false,
                None,
                cx,
            )
        })
        .await
        .unwrap()
        .into_included()
        .unwrap();
    assert!(entry.is_file());

    cx.executor().run_until_parked();
    tree_real.read_with(cx, |tree, _| {
        assert!(
            tree.entry_for_path(rel_path("a/b/c/d.txt"))
                .unwrap()
                .is_file()
        );
        assert!(tree.entry_for_path(rel_path("a/b/c")).unwrap().is_dir());
        assert!(tree.entry_for_path(rel_path("a/b")).unwrap().is_dir());
    });

    // Test smallest change
    let entry = tree_real
        .update(cx, |tree, cx| {
            tree.as_local_mut().unwrap().create_entry(
                rel_path("a/b/c/e.txt").into(),
                false,
                None,
                cx,
            )
        })
        .await
        .unwrap()
        .into_included()
        .unwrap();
    assert!(entry.is_file());

    cx.executor().run_until_parked();
    tree_real.read_with(cx, |tree, _| {
        assert!(
            tree.entry_for_path(rel_path("a/b/c/e.txt"))
                .unwrap()
                .is_file()
        );
    });

    // Test largest change
    let entry = tree_real
        .update(cx, |tree, cx| {
            tree.as_local_mut().unwrap().create_entry(
                rel_path("d/e/f/g.txt").into(),
                false,
                None,
                cx,
            )
        })
        .await
        .unwrap()
        .into_included()
        .unwrap();
    assert!(entry.is_file());

    cx.executor().run_until_parked();
    tree_real.read_with(cx, |tree, _| {
        assert!(
            tree.entry_for_path(rel_path("d/e/f/g.txt"))
                .unwrap()
                .is_file()
        );
        assert!(tree.entry_for_path(rel_path("d/e/f")).unwrap().is_dir());
        assert!(tree.entry_for_path(rel_path("d/e")).unwrap().is_dir());
        assert!(tree.entry_for_path(rel_path("d")).unwrap().is_dir());
    });
}

#[gpui::test]
async fn test_create_file_in_expanded_gitignored_dir(cx: &mut TestAppContext) {
    // Tests the behavior of our worktree refresh when a file in a gitignored directory
    // is created.
    init_test(cx);
    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/root",
        json!({
            ".gitignore": "ignored_dir\n",
            "ignored_dir": {
                "existing_file.txt": "existing content",
                "another_file.txt": "another content",
            },
        }),
    )
    .await;

    let tree = Worktree::local(
        Path::new("/root"),
        true,
        fs.clone(),
        Default::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();

    cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
        .await;

    tree.read_with(cx, |tree, _| {
        let ignored_dir = tree.entry_for_path(rel_path("ignored_dir")).unwrap();
        assert!(ignored_dir.is_ignored);
        assert_eq!(ignored_dir.kind, EntryKind::UnloadedDir);
    });

    tree.update(cx, |tree, cx| {
        tree.load_file(rel_path("ignored_dir/existing_file.txt"), cx)
    })
    .await
    .unwrap();

    tree.read_with(cx, |tree, _| {
        let ignored_dir = tree.entry_for_path(rel_path("ignored_dir")).unwrap();
        assert!(ignored_dir.is_ignored);
        assert_eq!(ignored_dir.kind, EntryKind::Dir);

        assert!(
            tree.entry_for_path(rel_path("ignored_dir/existing_file.txt"))
                .is_some()
        );
        assert!(
            tree.entry_for_path(rel_path("ignored_dir/another_file.txt"))
                .is_some()
        );
    });

    let entry = tree
        .update(cx, |tree, cx| {
            tree.create_entry(rel_path("ignored_dir/new_file.txt").into(), false, None, cx)
        })
        .await
        .unwrap();
    assert!(entry.into_included().is_some());

    cx.executor().run_until_parked();

    tree.read_with(cx, |tree, _| {
        let ignored_dir = tree.entry_for_path(rel_path("ignored_dir")).unwrap();
        assert!(ignored_dir.is_ignored);
        assert_eq!(
            ignored_dir.kind,
            EntryKind::Dir,
            "ignored_dir should still be loaded, not UnloadedDir"
        );

        assert!(
            tree.entry_for_path(rel_path("ignored_dir/existing_file.txt"))
                .is_some(),
            "existing_file.txt should still be visible"
        );
        assert!(
            tree.entry_for_path(rel_path("ignored_dir/another_file.txt"))
                .is_some(),
            "another_file.txt should still be visible"
        );
        assert!(
            tree.entry_for_path(rel_path("ignored_dir/new_file.txt"))
                .is_some(),
            "new_file.txt should be visible"
        );
    });
}

#[gpui::test]
async fn test_fs_event_for_gitignored_dir_does_not_lose_contents(cx: &mut TestAppContext) {
    // Tests the behavior of our worktree refresh when a directory modification for a gitignored directory
    // is triggered.
    init_test(cx);
    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/root",
        json!({
            ".gitignore": "ignored_dir\n",
            "ignored_dir": {
                "file1.txt": "content1",
                "file2.txt": "content2",
            },
        }),
    )
    .await;

    let tree = Worktree::local(
        Path::new("/root"),
        true,
        fs.clone(),
        Default::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();

    cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
        .await;

    // Load a file to expand the ignored directory
    tree.update(cx, |tree, cx| {
        tree.load_file(rel_path("ignored_dir/file1.txt"), cx)
    })
    .await
    .unwrap();

    tree.read_with(cx, |tree, _| {
        let ignored_dir = tree.entry_for_path(rel_path("ignored_dir")).unwrap();
        assert_eq!(ignored_dir.kind, EntryKind::Dir);
        assert!(
            tree.entry_for_path(rel_path("ignored_dir/file1.txt"))
                .is_some()
        );
        assert!(
            tree.entry_for_path(rel_path("ignored_dir/file2.txt"))
                .is_some()
        );
    });

    fs.emit_fs_event("/root/ignored_dir", Some(fs::PathEventKind::Changed));
    tree.flush_fs_events(cx).await;

    tree.read_with(cx, |tree, _| {
        let ignored_dir = tree.entry_for_path(rel_path("ignored_dir")).unwrap();
        assert_eq!(
            ignored_dir.kind,
            EntryKind::Dir,
            "ignored_dir should still be loaded (Dir), not UnloadedDir"
        );
        assert!(
            tree.entry_for_path(rel_path("ignored_dir/file1.txt"))
                .is_some(),
            "file1.txt should still be visible after directory fs event"
        );
        assert!(
            tree.entry_for_path(rel_path("ignored_dir/file2.txt"))
                .is_some(),
            "file2.txt should still be visible after directory fs event"
        );
    });
}

#[gpui::test(iterations = 100)]
async fn test_random_worktree_operations_during_initial_scan(
    cx: &mut TestAppContext,
    mut rng: StdRng,
) {
    init_test(cx);
    let operations = env::var("OPERATIONS")
        .map(|o| o.parse().unwrap())
        .unwrap_or(5);
    let initial_entries = env::var("INITIAL_ENTRIES")
        .map(|o| o.parse().unwrap())
        .unwrap_or(20);

    let root_dir = Path::new(path!("/test"));
    let fs = FakeFs::new(cx.background_executor.clone()) as Arc<dyn Fs>;
    fs.as_fake().insert_tree(root_dir, json!({})).await;
    for _ in 0..initial_entries {
        randomly_mutate_fs(&fs, root_dir, 1.0, &mut rng).await;
    }
    log::info!("generated initial tree");

    let worktree = Worktree::local(
        root_dir,
        true,
        fs.clone(),
        Default::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();

    let mut snapshots = vec![worktree.read_with(cx, |tree, _| tree.as_local().unwrap().snapshot())];
    let updates = Arc::new(Mutex::new(Vec::new()));
    worktree.update(cx, |tree, cx| {
        check_worktree_change_events(tree, cx);

        tree.as_local_mut().unwrap().observe_updates(0, cx, {
            let updates = updates.clone();
            move |update| {
                updates.lock().push(update);
                async { true }
            }
        });
    });

    for _ in 0..operations {
        worktree
            .update(cx, |worktree, cx| {
                randomly_mutate_worktree(worktree, &mut rng, cx)
            })
            .await
            .log_err();
        worktree.read_with(cx, |tree, _| {
            tree.as_local().unwrap().snapshot().check_invariants(true)
        });

        if rng.random_bool(0.6) {
            snapshots.push(worktree.read_with(cx, |tree, _| tree.as_local().unwrap().snapshot()));
        }
    }

    worktree
        .update(cx, |tree, _| tree.as_local_mut().unwrap().scan_complete())
        .await;

    cx.executor().run_until_parked();

    let final_snapshot = worktree.read_with(cx, |tree, _| {
        let tree = tree.as_local().unwrap();
        let snapshot = tree.snapshot();
        snapshot.check_invariants(true);
        snapshot
    });

    let settings = worktree.read_with(cx, |tree, _| tree.as_local().unwrap().settings());

    for (i, snapshot) in snapshots.into_iter().enumerate().rev() {
        let mut updated_snapshot = snapshot.clone();
        for update in updates.lock().iter() {
            if update.scan_id >= updated_snapshot.scan_id() as u64 {
                updated_snapshot
                    .apply_remote_update(update.clone(), &settings.file_scan_inclusions);
            }
        }

        assert_eq!(
            updated_snapshot.entries(true, 0).collect::<Vec<_>>(),
            final_snapshot.entries(true, 0).collect::<Vec<_>>(),
            "wrong updates after snapshot {i}: {updates:#?}",
        );
    }
}

#[gpui::test(iterations = 100)]
async fn test_random_worktree_changes(cx: &mut TestAppContext, mut rng: StdRng) {
    init_test(cx);
    let operations = env::var("OPERATIONS")
        .map(|o| o.parse().unwrap())
        .unwrap_or(40);
    let initial_entries = env::var("INITIAL_ENTRIES")
        .map(|o| o.parse().unwrap())
        .unwrap_or(20);

    let root_dir = Path::new(path!("/test"));
    let fs = FakeFs::new(cx.background_executor.clone()) as Arc<dyn Fs>;
    fs.as_fake().insert_tree(root_dir, json!({})).await;
    for _ in 0..initial_entries {
        randomly_mutate_fs(&fs, root_dir, 1.0, &mut rng).await;
    }
    log::info!("generated initial tree");

    let worktree = Worktree::local(
        root_dir,
        true,
        fs.clone(),
        Default::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();

    let updates = Arc::new(Mutex::new(Vec::new()));
    worktree.update(cx, |tree, cx| {
        check_worktree_change_events(tree, cx);

        tree.as_local_mut().unwrap().observe_updates(0, cx, {
            let updates = updates.clone();
            move |update| {
                updates.lock().push(update);
                async { true }
            }
        });
    });

    worktree
        .update(cx, |tree, _| tree.as_local_mut().unwrap().scan_complete())
        .await;

    fs.as_fake().pause_events();
    let mut snapshots = Vec::new();
    let mut mutations_len = operations;
    while mutations_len > 1 {
        if rng.random_bool(0.2) {
            worktree
                .update(cx, |worktree, cx| {
                    randomly_mutate_worktree(worktree, &mut rng, cx)
                })
                .await
                .log_err();
        } else {
            randomly_mutate_fs(&fs, root_dir, 1.0, &mut rng).await;
        }

        let buffered_event_count = fs.as_fake().buffered_event_count();
        if buffered_event_count > 0 && rng.random_bool(0.3) {
            let len = rng.random_range(0..=buffered_event_count);
            log::info!("flushing {} events", len);
            fs.as_fake().flush_events(len);
        } else {
            randomly_mutate_fs(&fs, root_dir, 0.6, &mut rng).await;
            mutations_len -= 1;
        }

        cx.executor().run_until_parked();
        if rng.random_bool(0.2) {
            log::info!("storing snapshot {}", snapshots.len());
            let snapshot = worktree.read_with(cx, |tree, _| tree.as_local().unwrap().snapshot());
            snapshots.push(snapshot);
        }
    }

    log::info!("quiescing");
    fs.as_fake().flush_events(usize::MAX);
    cx.executor().run_until_parked();

    let snapshot = worktree.read_with(cx, |tree, _| tree.as_local().unwrap().snapshot());
    snapshot.check_invariants(true);
    let expanded_paths = snapshot
        .expanded_entries()
        .map(|e| e.path.clone())
        .collect::<Vec<_>>();

    {
        let new_worktree = Worktree::local(
            root_dir,
            true,
            fs.clone(),
            Default::default(),
            true,
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        new_worktree
            .update(cx, |tree, _| tree.as_local_mut().unwrap().scan_complete())
            .await;
        new_worktree
            .update(cx, |tree, _| {
                tree.as_local_mut()
                    .unwrap()
                    .refresh_entries_for_paths(expanded_paths)
            })
            .recv()
            .await;
        let new_snapshot =
            new_worktree.read_with(cx, |tree, _| tree.as_local().unwrap().snapshot());
        assert_eq!(
            snapshot.entries_without_ids(true),
            new_snapshot.entries_without_ids(true)
        );
    }

    let settings = worktree.read_with(cx, |tree, _| tree.as_local().unwrap().settings());

    for (i, mut prev_snapshot) in snapshots.into_iter().enumerate().rev() {
        for update in updates.lock().iter() {
            if update.scan_id >= prev_snapshot.scan_id() as u64 {
                prev_snapshot.apply_remote_update(update.clone(), &settings.file_scan_inclusions);
            }
        }

        assert_eq!(
            prev_snapshot
                .entries(true, 0)
                .map(ignore_pending_dir)
                .collect::<Vec<_>>(),
            snapshot
                .entries(true, 0)
                .map(ignore_pending_dir)
                .collect::<Vec<_>>(),
            "wrong updates after snapshot {i}: {updates:#?}",
        );
    }

    fn ignore_pending_dir(entry: &Entry) -> Entry {
        let mut entry = entry.clone();
        if entry.kind.is_dir() {
            entry.kind = EntryKind::Dir
        }
        entry
    }
}

// The worktree's `UpdatedEntries` event can be used to follow along with
// all changes to the worktree's snapshot.
fn check_worktree_change_events(tree: &mut Worktree, cx: &mut Context<Worktree>) {
    let mut entries = tree.entries(true, 0).cloned().collect::<Vec<_>>();
    cx.subscribe(&cx.entity(), move |tree, _, event, _| {
        if let Event::UpdatedEntries(changes) = event {
            for (path, _, change_type) in changes.iter() {
                let entry = tree.entry_for_path(path).cloned();
                let ix = match entries.binary_search_by_key(&path, |e| &e.path) {
                    Ok(ix) | Err(ix) => ix,
                };
                match change_type {
                    PathChange::Added => entries.insert(ix, entry.unwrap()),
                    PathChange::Removed => drop(entries.remove(ix)),
                    PathChange::Updated => {
                        let entry = entry.unwrap();
                        let existing_entry = entries.get_mut(ix).unwrap();
                        assert_eq!(existing_entry.path, entry.path);
                        *existing_entry = entry;
                    }
                    PathChange::AddedOrUpdated | PathChange::Loaded => {
                        let entry = entry.unwrap();
                        if entries.get(ix).map(|e| &e.path) == Some(&entry.path) {
                            *entries.get_mut(ix).unwrap() = entry;
                        } else {
                            entries.insert(ix, entry);
                        }
                    }
                }
            }

            let new_entries = tree.entries(true, 0).cloned().collect::<Vec<_>>();
            assert_eq!(entries, new_entries, "incorrect changes: {:?}", changes);
        }
    })
    .detach();
}

fn randomly_mutate_worktree(
    worktree: &mut Worktree,
    rng: &mut impl Rng,
    cx: &mut Context<Worktree>,
) -> Task<Result<()>> {
    log::info!("mutating worktree");
    let worktree = worktree.as_local_mut().unwrap();
    let snapshot = worktree.snapshot();
    let entry = snapshot.entries(false, 0).choose(rng).unwrap();

    match rng.random_range(0_u32..100) {
        0..=33 if entry.path.as_ref() != RelPath::empty() => {
            log::info!("deleting entry {:?} ({})", entry.path, entry.id.0);
            worktree.delete_entry(entry.id, false, cx).unwrap()
        }
        _ => {
            if entry.is_dir() {
                let child_path = entry.path.join(rel_path(&random_filename(rng)));
                let is_dir = rng.random_bool(0.3);
                log::info!(
                    "creating {} at {:?}",
                    if is_dir { "dir" } else { "file" },
                    child_path,
                );
                let task = worktree.create_entry(child_path, is_dir, None, cx);
                cx.background_spawn(async move {
                    task.await?;
                    Ok(())
                })
            } else {
                log::info!("overwriting file {:?} ({})", &entry.path, entry.id.0);
                let task = worktree.write_file(
                    entry.path.clone(),
                    "".into(),
                    Default::default(),
                    encoding_rs::UTF_8,
                    false,
                    cx,
                );
                cx.background_spawn(async move {
                    task.await?;
                    Ok(())
                })
            }
        }
    }
}

async fn randomly_mutate_fs(
    fs: &Arc<dyn Fs>,
    root_path: &Path,
    insertion_probability: f64,
    rng: &mut impl Rng,
) {
    log::info!("mutating fs");
    let mut files = Vec::new();
    let mut dirs = Vec::new();
    for path in fs.as_fake().paths(false) {
        if path.starts_with(root_path) {
            if fs.is_file(&path).await {
                files.push(path);
            } else {
                dirs.push(path);
            }
        }
    }

    if (files.is_empty() && dirs.len() == 1) || rng.random_bool(insertion_probability) {
        let path = dirs.choose(rng).unwrap();
        let new_path = path.join(random_filename(rng));

        if rng.random() {
            log::info!(
                "creating dir {:?}",
                new_path.strip_prefix(root_path).unwrap()
            );
            fs.create_dir(&new_path).await.unwrap();
        } else {
            log::info!(
                "creating file {:?}",
                new_path.strip_prefix(root_path).unwrap()
            );
            fs.create_file(&new_path, Default::default()).await.unwrap();
        }
    } else if rng.random_bool(0.05) {
        let ignore_dir_path = dirs.choose(rng).unwrap();
        let ignore_path = ignore_dir_path.join(GITIGNORE);

        let subdirs = dirs
            .iter()
            .filter(|d| d.starts_with(ignore_dir_path))
            .cloned()
            .collect::<Vec<_>>();
        let subfiles = files
            .iter()
            .filter(|d| d.starts_with(ignore_dir_path))
            .cloned()
            .collect::<Vec<_>>();
        let files_to_ignore = {
            let len = rng.random_range(0..=subfiles.len());
            subfiles.choose_multiple(rng, len)
        };
        let dirs_to_ignore = {
            let len = rng.random_range(0..subdirs.len());
            subdirs.choose_multiple(rng, len)
        };

        let mut ignore_contents = String::new();
        for path_to_ignore in files_to_ignore.chain(dirs_to_ignore) {
            writeln!(
                ignore_contents,
                "{}",
                path_to_ignore
                    .strip_prefix(ignore_dir_path)
                    .unwrap()
                    .to_str()
                    .unwrap()
            )
            .unwrap();
        }
        log::info!(
            "creating gitignore {:?} with contents:\n{}",
            ignore_path.strip_prefix(root_path).unwrap(),
            ignore_contents
        );
        fs.save(
            &ignore_path,
            &ignore_contents.as_str().into(),
            Default::default(),
        )
        .await
        .unwrap();
    } else {
        let old_path = {
            let file_path = files.choose(rng);
            let dir_path = dirs[1..].choose(rng);
            file_path.into_iter().chain(dir_path).choose(rng).unwrap()
        };

        let is_rename = rng.random();
        if is_rename {
            let new_path_parent = dirs
                .iter()
                .filter(|d| !d.starts_with(old_path))
                .choose(rng)
                .unwrap();

            let overwrite_existing_dir =
                !old_path.starts_with(new_path_parent) && rng.random_bool(0.3);
            let new_path = if overwrite_existing_dir {
                fs.remove_dir(
                    new_path_parent,
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: true,
                    },
                )
                .await
                .unwrap();
                new_path_parent.to_path_buf()
            } else {
                new_path_parent.join(random_filename(rng))
            };

            log::info!(
                "renaming {:?} to {}{:?}",
                old_path.strip_prefix(root_path).unwrap(),
                if overwrite_existing_dir {
                    "overwrite "
                } else {
                    ""
                },
                new_path.strip_prefix(root_path).unwrap()
            );
            fs.rename(
                old_path,
                &new_path,
                fs::RenameOptions {
                    overwrite: true,
                    ignore_if_exists: true,
                    create_parents: false,
                },
            )
            .await
            .unwrap();
        } else if fs.is_file(old_path).await {
            log::info!(
                "deleting file {:?}",
                old_path.strip_prefix(root_path).unwrap()
            );
            fs.remove_file(old_path, Default::default()).await.unwrap();
        } else {
            log::info!(
                "deleting dir {:?}",
                old_path.strip_prefix(root_path).unwrap()
            );
            fs.remove_dir(
                old_path,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await
            .unwrap();
        }
    }
}

fn random_filename(rng: &mut impl Rng) -> String {
    (0..6)
        .map(|_| rng.sample(rand::distr::Alphanumeric))
        .map(char::from)
        .collect()
}

#[gpui::test]
async fn test_private_single_file_worktree(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree("/", json!({".env": "PRIVATE=secret\n"}))
        .await;
    let tree = Worktree::local(
        Path::new("/.env"),
        true,
        fs.clone(),
        Default::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();
    cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
        .await;
    tree.read_with(cx, |tree, _| {
        let entry = tree.entry_for_path(rel_path("")).unwrap();
        assert!(entry.is_private);
    });
}

#[gpui::test]
async fn test_repository_above_root(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(executor);
    fs.insert_tree(
        path!("/root"),
        json!({
            ".git": {},
            "subproject": {
                "a.txt": "A"
            }
        }),
    )
    .await;
    let worktree = Worktree::local(
        path!("/root/subproject").as_ref(),
        true,
        fs.clone(),
        Arc::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();
    worktree
        .update(cx, |worktree, _| {
            worktree.as_local().unwrap().scan_complete()
        })
        .await;
    cx.run_until_parked();
    let repos = worktree.update(cx, |worktree, _| {
        worktree
            .as_local()
            .unwrap()
            .git_repositories
            .values()
            .map(|entry| entry.work_directory_abs_path.clone())
            .collect::<Vec<_>>()
    });
    pretty_assertions::assert_eq!(repos, [Path::new(path!("/root")).into()]);

    fs.touch_path(path!("/root/subproject")).await;
    worktree
        .update(cx, |worktree, _| {
            worktree.as_local().unwrap().scan_complete()
        })
        .await;
    cx.run_until_parked();

    let repos = worktree.update(cx, |worktree, _| {
        worktree
            .as_local()
            .unwrap()
            .git_repositories
            .values()
            .map(|entry| entry.work_directory_abs_path.clone())
            .collect::<Vec<_>>()
    });
    pretty_assertions::assert_eq!(repos, [Path::new(path!("/root")).into()]);
}

#[gpui::test]
async fn test_global_gitignore(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    init_test(cx);

    let home = paths::home_dir();
    let fs = FakeFs::new(executor);
    fs.insert_tree(
        home,
        json!({
            ".config": {
                "git": {
                    "ignore": "foo\n/bar\nbaz\n"
                }
            },
            "project": {
                ".git": {},
                ".gitignore": "!baz",
                "foo": "",
                "bar": "",
                "sub": {
                    "bar": "",
                },
                "subrepo": {
                    ".git": {},
                    "bar": ""
                },
                "baz": ""
            }
        }),
    )
    .await;
    let worktree = Worktree::local(
        home.join("project"),
        true,
        fs.clone(),
        Arc::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();
    worktree
        .update(cx, |worktree, _| {
            worktree.as_local().unwrap().scan_complete()
        })
        .await;
    cx.run_until_parked();

    // .gitignore overrides excludesFile, and anchored paths in excludesFile are resolved
    // relative to the nearest containing repository
    worktree.update(cx, |worktree, _cx| {
        check_worktree_entries(
            worktree,
            &[],
            &["foo", "bar", "subrepo/bar"],
            &["sub/bar", "baz"],
            &[],
        );
    });

    // Ignore statuses are updated when excludesFile changes
    fs.write(
        &home.join(".config").join("git").join("ignore"),
        "/bar\nbaz\n".as_bytes(),
    )
    .await
    .unwrap();
    worktree
        .update(cx, |worktree, _| {
            worktree.as_local().unwrap().scan_complete()
        })
        .await;
    cx.run_until_parked();

    worktree.update(cx, |worktree, _cx| {
        check_worktree_entries(
            worktree,
            &[],
            &["bar", "subrepo/bar"],
            &["foo", "sub/bar", "baz"],
            &[],
        );
    });

    // Statuses are updated when .git added/removed
    fs.remove_dir(
        &home.join("project").join("subrepo").join(".git"),
        RemoveOptions {
            recursive: true,
            ..Default::default()
        },
    )
    .await
    .unwrap();
    worktree
        .update(cx, |worktree, _| {
            worktree.as_local().unwrap().scan_complete()
        })
        .await;
    cx.run_until_parked();

    worktree.update(cx, |worktree, _cx| {
        check_worktree_entries(
            worktree,
            &[],
            &["bar"],
            &["foo", "sub/bar", "baz", "subrepo/bar"],
            &[],
        );
    });
}

#[gpui::test]
async fn test_repo_exclude(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(executor);
    let project_dir = Path::new(path!("/project"));
    fs.insert_tree(
        project_dir,
        json!({
            ".git": {
                "info": {
                    "exclude": ".env.*"
                }
            },
            ".env.example": "secret=xxxx",
            ".env.local": "secret=1234",
            ".gitignore": "!.env.example",
            "README.md": "# Repo Exclude",
            "src": {
                "main.rs": "fn main() {}",
            },
        }),
    )
    .await;

    let worktree = Worktree::local(
        project_dir,
        true,
        fs.clone(),
        Default::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();
    worktree
        .update(cx, |worktree, _| {
            worktree.as_local().unwrap().scan_complete()
        })
        .await;
    cx.run_until_parked();

    // .gitignore overrides .git/info/exclude
    worktree.update(cx, |worktree, _cx| {
        let expected_excluded_paths = [];
        let expected_ignored_paths = [".env.local"];
        let expected_tracked_paths = [".env.example", "README.md", "src/main.rs"];
        let expected_included_paths = [];

        check_worktree_entries(
            worktree,
            &expected_excluded_paths,
            &expected_ignored_paths,
            &expected_tracked_paths,
            &expected_included_paths,
        );
    });

    // Ignore statuses are updated when .git/info/exclude file changes
    fs.write(
        &project_dir.join(DOT_GIT).join(REPO_EXCLUDE),
        ".env.example".as_bytes(),
    )
    .await
    .unwrap();
    worktree
        .update(cx, |worktree, _| {
            worktree.as_local().unwrap().scan_complete()
        })
        .await;
    cx.run_until_parked();

    worktree.update(cx, |worktree, _cx| {
        let expected_excluded_paths = [];
        let expected_ignored_paths = [];
        let expected_tracked_paths = [".env.example", ".env.local", "README.md", "src/main.rs"];
        let expected_included_paths = [];

        check_worktree_entries(
            worktree,
            &expected_excluded_paths,
            &expected_ignored_paths,
            &expected_tracked_paths,
            &expected_included_paths,
        );
    });
}

#[track_caller]
fn check_worktree_entries(
    tree: &Worktree,
    expected_excluded_paths: &[&str],
    expected_ignored_paths: &[&str],
    expected_tracked_paths: &[&str],
    expected_included_paths: &[&str],
) {
    for path in expected_excluded_paths {
        let entry = tree.entry_for_path(rel_path(path));
        assert!(
            entry.is_none(),
            "expected path '{path}' to be excluded, but got entry: {entry:?}",
        );
    }
    for path in expected_ignored_paths {
        let entry = tree
            .entry_for_path(rel_path(path))
            .unwrap_or_else(|| panic!("Missing entry for expected ignored path '{path}'"));
        assert!(
            entry.is_ignored,
            "expected path '{path}' to be ignored, but got entry: {entry:?}",
        );
    }
    for path in expected_tracked_paths {
        let entry = tree
            .entry_for_path(rel_path(path))
            .unwrap_or_else(|| panic!("Missing entry for expected tracked path '{path}'"));
        assert!(
            !entry.is_ignored || entry.is_always_included,
            "expected path '{path}' to be tracked, but got entry: {entry:?}",
        );
    }
    for path in expected_included_paths {
        let entry = tree
            .entry_for_path(rel_path(path))
            .unwrap_or_else(|| panic!("Missing entry for expected included path '{path}'"));
        assert!(
            entry.is_always_included,
            "expected path '{path}' to always be included, but got entry: {entry:?}",
        );
    }
}

fn init_test(cx: &mut gpui::TestAppContext) {
    zlog::init_test();

    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
    });
}

#[gpui::test]
async fn test_load_file_encoding(cx: &mut TestAppContext) {
    init_test(cx);

    struct TestCase {
        name: &'static str,
        bytes: Vec<u8>,
        expected_text: &'static str,
    }

    // --- Success Cases ---
    let success_cases = vec![
        TestCase {
            name: "utf8.txt",
            bytes: "こんにちは".as_bytes().to_vec(),
            expected_text: "こんにちは",
        },
        TestCase {
            name: "sjis.txt",
            bytes: vec![0x82, 0xb1, 0x82, 0xf1, 0x82, 0xc9, 0x82, 0xbf, 0x82, 0xcd],
            expected_text: "こんにちは",
        },
        TestCase {
            name: "eucjp.txt",
            bytes: vec![0xa4, 0xb3, 0xa4, 0xf3, 0xa4, 0xcb, 0xa4, 0xc1, 0xa4, 0xcf],
            expected_text: "こんにちは",
        },
        TestCase {
            name: "iso2022jp.txt",
            bytes: vec![
                0x1b, 0x24, 0x42, 0x24, 0x33, 0x24, 0x73, 0x24, 0x4b, 0x24, 0x41, 0x24, 0x4f, 0x1b,
                0x28, 0x42,
            ],
            expected_text: "こんにちは",
        },
        TestCase {
            name: "win1252.txt",
            bytes: vec![0x43, 0x61, 0x66, 0xe9],
            expected_text: "Café",
        },
        TestCase {
            name: "gbk.txt",
            bytes: vec![
                0xbd, 0xf1, 0xcc, 0xec, 0xcc, 0xec, 0xc6, 0xf8, 0xb2, 0xbb, 0xb4, 0xed,
            ],
            expected_text: "今天天气不错",
        },
        // UTF-16LE with BOM
        TestCase {
            name: "utf16le_bom.txt",
            bytes: vec![
                0xFF, 0xFE, // BOM
                0x53, 0x30, 0x93, 0x30, 0x6B, 0x30, 0x61, 0x30, 0x6F, 0x30,
            ],
            expected_text: "こんにちは",
        },
        // UTF-16BE with BOM
        TestCase {
            name: "utf16be_bom.txt",
            bytes: vec![
                0xFE, 0xFF, // BOM
                0x30, 0x53, 0x30, 0x93, 0x30, 0x6B, 0x30, 0x61, 0x30, 0x6F,
            ],
            expected_text: "こんにちは",
        },
        // UTF-16LE without BOM (ASCII only)
        // This relies on the "null byte heuristic" we implemented.
        // "ABC" -> 41 00 42 00 43 00
        TestCase {
            name: "utf16le_ascii_no_bom.txt",
            bytes: vec![0x41, 0x00, 0x42, 0x00, 0x43, 0x00],
            expected_text: "ABC",
        },
    ];

    // --- Failure Cases ---
    let failure_cases = vec![
        // Binary File (Should be detected by heuristic and return Error)
        // Contains random bytes and mixed nulls that don't match UTF-16 patterns
        TestCase {
            name: "binary.bin",
            bytes: vec![0x00, 0xFF, 0x12, 0x00, 0x99, 0x88, 0x77, 0x66, 0x00],
            expected_text: "", // Not used
        },
    ];

    let root_path = if cfg!(windows) {
        Path::new("C:\\root")
    } else {
        Path::new("/root")
    };

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.create_dir(root_path).await.unwrap();

    for case in success_cases.iter().chain(failure_cases.iter()) {
        let path = root_path.join(case.name);
        fs.write(&path, &case.bytes).await.unwrap();
    }

    let tree = Worktree::local(
        root_path,
        true,
        fs,
        Default::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();

    cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
        .await;

    let rel_path = |name: &str| {
        RelPath::new(&Path::new(name), PathStyle::local())
            .unwrap()
            .into_arc()
    };

    // Run Success Tests
    for case in success_cases {
        let loaded = tree
            .update(cx, |tree, cx| tree.load_file(&rel_path(case.name), cx))
            .await;
        if let Err(e) = &loaded {
            panic!("Failed to load success case '{}': {:?}", case.name, e);
        }
        let loaded = loaded.unwrap();
        assert_eq!(
            loaded.text, case.expected_text,
            "Encoding mismatch for file: {}",
            case.name
        );
    }

    // Run Failure Tests
    for case in failure_cases {
        let loaded = tree
            .update(cx, |tree, cx| tree.load_file(&rel_path(case.name), cx))
            .await;
        assert!(
            loaded.is_err(),
            "Failure case '{}' unexpectedly succeeded! It should have been detected as binary.",
            case.name
        );
        let err_msg = loaded.unwrap_err().to_string();
        println!("Got expected error for {}: {}", case.name, err_msg);
    }
}

#[gpui::test]
async fn test_write_file_encoding(cx: &mut gpui::TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());

    let root_path = if cfg!(windows) {
        Path::new("C:\\root")
    } else {
        Path::new("/root")
    };
    fs.create_dir(root_path).await.unwrap();

    let worktree = Worktree::local(
        root_path,
        true,
        fs.clone(),
        Default::default(),
        true,
        &mut cx.to_async(),
    )
    .await
    .unwrap();

    // Define test case structure
    struct TestCase {
        name: &'static str,
        text: &'static str,
        encoding: &'static encoding_rs::Encoding,
        has_bom: bool,
        expected_bytes: Vec<u8>,
    }

    let cases = vec![
        // Shift_JIS with Japanese
        TestCase {
            name: "Shift_JIS with Japanese",
            text: "こんにちは",
            encoding: encoding_rs::SHIFT_JIS,
            has_bom: false,
            expected_bytes: vec![0x82, 0xb1, 0x82, 0xf1, 0x82, 0xc9, 0x82, 0xbf, 0x82, 0xcd],
        },
        // UTF-8 No BOM
        TestCase {
            name: "UTF-8 No BOM",
            text: "AB",
            encoding: encoding_rs::UTF_8,
            has_bom: false,
            expected_bytes: vec![0x41, 0x42],
        },
        // UTF-8 with BOM
        TestCase {
            name: "UTF-8 with BOM",
            text: "AB",
            encoding: encoding_rs::UTF_8,
            has_bom: true,
            expected_bytes: vec![0xEF, 0xBB, 0xBF, 0x41, 0x42],
        },
        // UTF-16LE No BOM with Japanese
        // NOTE: This passes thanks to the manual encoding fix implemented in `write_file`.
        TestCase {
            name: "UTF-16LE No BOM with Japanese",
            text: "こんにちは",
            encoding: encoding_rs::UTF_16LE,
            has_bom: false,
            expected_bytes: vec![0x53, 0x30, 0x93, 0x30, 0x6b, 0x30, 0x61, 0x30, 0x6f, 0x30],
        },
        // UTF-16LE with BOM
        TestCase {
            name: "UTF-16LE with BOM",
            text: "A",
            encoding: encoding_rs::UTF_16LE,
            has_bom: true,
            expected_bytes: vec![0xFF, 0xFE, 0x41, 0x00],
        },
        // UTF-16BE No BOM with Japanese
        // NOTE: This passes thanks to the manual encoding fix.
        TestCase {
            name: "UTF-16BE No BOM with Japanese",
            text: "こんにちは",
            encoding: encoding_rs::UTF_16BE,
            has_bom: false,
            expected_bytes: vec![0x30, 0x53, 0x30, 0x93, 0x30, 0x6b, 0x30, 0x61, 0x30, 0x6f],
        },
        // UTF-16BE with BOM
        TestCase {
            name: "UTF-16BE with BOM",
            text: "A",
            encoding: encoding_rs::UTF_16BE,
            has_bom: true,
            expected_bytes: vec![0xFE, 0xFF, 0x00, 0x41],
        },
    ];

    for (i, case) in cases.into_iter().enumerate() {
        let file_name = format!("test_{}.txt", i);
        let path: Arc<Path> = Path::new(&file_name).into();
        let file_path = root_path.join(&file_name);

        fs.insert_file(&file_path, "".into()).await;

        let rel_path = RelPath::new(&path, PathStyle::local()).unwrap().into_arc();
        let text = text::Rope::from(case.text);

        let task = worktree.update(cx, |wt, cx| {
            wt.write_file(
                rel_path,
                text,
                text::LineEnding::Unix,
                case.encoding,
                case.has_bom,
                cx,
            )
        });

        if let Err(e) = task.await {
            panic!("Unexpected error in case '{}': {:?}", case.name, e);
        }

        let bytes = fs.load_bytes(&file_path).await.unwrap();

        assert_eq!(
            bytes, case.expected_bytes,
            "case '{}' mismatch. Expected {:?}, but got {:?}",
            case.name, case.expected_bytes, bytes
        );
    }
}
