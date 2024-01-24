use std::{assert_eq, path::Path, time::Duration};

use super::*;
use editor::Editor;
use gpui::{Entity, TestAppContext, VisualTestContext};
use menu::{Confirm, SelectNext};
use serde_json::json;
use workspace::{AppState, Workspace};

#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}

#[gpui::test]
async fn test_matching_paths(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            "/root",
            json!({
                "a": {
                    "banana": "",
                    "bandana": "",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;

    let (picker, workspace, cx) = build_find_picker(project, cx);

    cx.simulate_input("bna");
    picker.update(cx, |picker, _| {
        assert_eq!(picker.delegate.matches.len(), 2);
    });
    cx.dispatch_action(SelectNext);
    cx.dispatch_action(Confirm);
    cx.read(|cx| {
        let active_editor = workspace.read(cx).active_item_as::<Editor>(cx).unwrap();
        assert_eq!(active_editor.read(cx).title(cx), "bandana");
    });

    for bandana_query in [
        "bandana",
        " bandana",
        "bandana ",
        " bandana ",
        " ndan ",
        " band ",
    ] {
        picker
            .update(cx, |picker, cx| {
                picker
                    .delegate
                    .update_matches(bandana_query.to_string(), cx)
            })
            .await;
        picker.update(cx, |picker, _| {
            assert_eq!(
                picker.delegate.matches.len(),
                1,
                "Wrong number of matches for bandana query '{bandana_query}'"
            );
        });
        cx.dispatch_action(SelectNext);
        cx.dispatch_action(Confirm);
        cx.read(|cx| {
            let active_editor = workspace.read(cx).active_item_as::<Editor>(cx).unwrap();
            assert_eq!(
                active_editor.read(cx).title(cx),
                "bandana",
                "Wrong match for bandana query '{bandana_query}'"
            );
        });
    }
}

#[gpui::test]
async fn test_absolute_paths(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            "/root",
            json!({
                "a": {
                    "file1.txt": "",
                    "b": {
                        "file2.txt": "",
                    },
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;

    let (picker, workspace, cx) = build_find_picker(project, cx);

    let matching_abs_path = "/root/a/b/file2.txt";
    picker
        .update(cx, |picker, cx| {
            picker
                .delegate
                .update_matches(matching_abs_path.to_string(), cx)
        })
        .await;
    picker.update(cx, |picker, _| {
        assert_eq!(
            collect_search_results(picker),
            vec![PathBuf::from("a/b/file2.txt")],
            "Matching abs path should be the only match"
        )
    });
    cx.dispatch_action(SelectNext);
    cx.dispatch_action(Confirm);
    cx.read(|cx| {
        let active_editor = workspace.read(cx).active_item_as::<Editor>(cx).unwrap();
        assert_eq!(active_editor.read(cx).title(cx), "file2.txt");
    });

    let mismatching_abs_path = "/root/a/b/file1.txt";
    picker
        .update(cx, |picker, cx| {
            picker
                .delegate
                .update_matches(mismatching_abs_path.to_string(), cx)
        })
        .await;
    picker.update(cx, |picker, _| {
        assert_eq!(
            collect_search_results(picker),
            Vec::<PathBuf>::new(),
            "Mismatching abs path should produce no matches"
        )
    });
}

#[gpui::test]
async fn test_complex_path(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            "/root",
            json!({
                "其他": {
                    "S数据表格": {
                        "task.xlsx": "some content",
                    },
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;

    let (picker, workspace, cx) = build_find_picker(project, cx);

    cx.simulate_input("t");
    picker.update(cx, |picker, _| {
        assert_eq!(picker.delegate.matches.len(), 1);
        assert_eq!(
            collect_search_results(picker),
            vec![PathBuf::from("其他/S数据表格/task.xlsx")],
        )
    });
    cx.dispatch_action(SelectNext);
    cx.dispatch_action(Confirm);
    cx.read(|cx| {
        let active_editor = workspace.read(cx).active_item_as::<Editor>(cx).unwrap();
        assert_eq!(active_editor.read(cx).title(cx), "task.xlsx");
    });
}

#[gpui::test]
async fn test_row_column_numbers_query_inside_file(cx: &mut TestAppContext) {
    let app_state = init_test(cx);

    let first_file_name = "first.rs";
    let first_file_contents = "// First Rust file";
    app_state
        .fs
        .as_fake()
        .insert_tree(
            "/src",
            json!({
                "test": {
                    first_file_name: first_file_contents,
                    "second.rs": "// Second Rust file",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), ["/src".as_ref()], cx).await;

    let (picker, workspace, cx) = build_find_picker(project, cx);

    let file_query = &first_file_name[..3];
    let file_row = 1;
    let file_column = 3;
    assert!(file_column <= first_file_contents.len());
    let query_inside_file = format!("{file_query}:{file_row}:{file_column}");
    picker
        .update(cx, |finder, cx| {
            finder
                .delegate
                .update_matches(query_inside_file.to_string(), cx)
        })
        .await;
    picker.update(cx, |finder, _| {
        let finder = &finder.delegate;
        assert_eq!(finder.matches.len(), 1);
        let latest_search_query = finder
            .latest_search_query
            .as_ref()
            .expect("Finder should have a query after the update_matches call");
        assert_eq!(latest_search_query.path_like.raw_query, query_inside_file);
        assert_eq!(
            latest_search_query.path_like.file_query_end,
            Some(file_query.len())
        );
        assert_eq!(latest_search_query.row, Some(file_row));
        assert_eq!(latest_search_query.column, Some(file_column as u32));
    });

    cx.dispatch_action(SelectNext);
    cx.dispatch_action(Confirm);

    let editor = cx.update(|cx| workspace.read(cx).active_item_as::<Editor>(cx).unwrap());
    cx.executor().advance_clock(Duration::from_secs(2));

    editor.update(cx, |editor, cx| {
            let all_selections = editor.selections.all_adjusted(cx);
            assert_eq!(
                all_selections.len(),
                1,
                "Expected to have 1 selection (caret) after file finder confirm, but got: {all_selections:?}"
            );
            let caret_selection = all_selections.into_iter().next().unwrap();
            assert_eq!(caret_selection.start, caret_selection.end,
                "Caret selection should have its start and end at the same position");
            assert_eq!(file_row, caret_selection.start.row + 1,
                "Query inside file should get caret with the same focus row");
            assert_eq!(file_column, caret_selection.start.column as usize + 1,
                "Query inside file should get caret with the same focus column");
        });
}

#[gpui::test]
async fn test_row_column_numbers_query_outside_file(cx: &mut TestAppContext) {
    let app_state = init_test(cx);

    let first_file_name = "first.rs";
    let first_file_contents = "// First Rust file";
    app_state
        .fs
        .as_fake()
        .insert_tree(
            "/src",
            json!({
                "test": {
                    first_file_name: first_file_contents,
                    "second.rs": "// Second Rust file",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), ["/src".as_ref()], cx).await;

    let (picker, workspace, cx) = build_find_picker(project, cx);

    let file_query = &first_file_name[..3];
    let file_row = 200;
    let file_column = 300;
    assert!(file_column > first_file_contents.len());
    let query_outside_file = format!("{file_query}:{file_row}:{file_column}");
    picker
        .update(cx, |picker, cx| {
            picker
                .delegate
                .update_matches(query_outside_file.to_string(), cx)
        })
        .await;
    picker.update(cx, |finder, _| {
        let delegate = &finder.delegate;
        assert_eq!(delegate.matches.len(), 1);
        let latest_search_query = delegate
            .latest_search_query
            .as_ref()
            .expect("Finder should have a query after the update_matches call");
        assert_eq!(latest_search_query.path_like.raw_query, query_outside_file);
        assert_eq!(
            latest_search_query.path_like.file_query_end,
            Some(file_query.len())
        );
        assert_eq!(latest_search_query.row, Some(file_row));
        assert_eq!(latest_search_query.column, Some(file_column as u32));
    });

    cx.dispatch_action(SelectNext);
    cx.dispatch_action(Confirm);

    let editor = cx.update(|cx| workspace.read(cx).active_item_as::<Editor>(cx).unwrap());
    cx.executor().advance_clock(Duration::from_secs(2));

    editor.update(cx, |editor, cx| {
            let all_selections = editor.selections.all_adjusted(cx);
            assert_eq!(
                all_selections.len(),
                1,
                "Expected to have 1 selection (caret) after file finder confirm, but got: {all_selections:?}"
            );
            let caret_selection = all_selections.into_iter().next().unwrap();
            assert_eq!(caret_selection.start, caret_selection.end,
                "Caret selection should have its start and end at the same position");
            assert_eq!(0, caret_selection.start.row,
                "Excessive rows (as in query outside file borders) should get trimmed to last file row");
            assert_eq!(first_file_contents.len(), caret_selection.start.column as usize,
                "Excessive columns (as in query outside file borders) should get trimmed to selected row's last column");
        });
}

#[gpui::test]
async fn test_matching_cancellation(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            "/dir",
            json!({
                "hello": "",
                "goodbye": "",
                "halogen-light": "",
                "happiness": "",
                "height": "",
                "hi": "",
                "hiccup": "",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), ["/dir".as_ref()], cx).await;

    let (picker, _, cx) = build_find_picker(project, cx);

    let query = test_path_like("hi");
    picker
        .update(cx, |picker, cx| {
            picker.delegate.spawn_search(query.clone(), cx)
        })
        .await;

    picker.update(cx, |picker, _cx| {
        assert_eq!(picker.delegate.matches.len(), 5)
    });

    picker.update(cx, |picker, cx| {
        let delegate = &mut picker.delegate;
        assert!(
            delegate.matches.history.is_empty(),
            "Search matches expected"
        );
        let matches = delegate.matches.search.clone();

        // Simulate a search being cancelled after the time limit,
        // returning only a subset of the matches that would have been found.
        drop(delegate.spawn_search(query.clone(), cx));
        delegate.set_search_matches(
            delegate.latest_search_id,
            true, // did-cancel
            query.clone(),
            vec![matches[1].clone(), matches[3].clone()],
            cx,
        );

        // Simulate another cancellation.
        drop(delegate.spawn_search(query.clone(), cx));
        delegate.set_search_matches(
            delegate.latest_search_id,
            true, // did-cancel
            query.clone(),
            vec![matches[0].clone(), matches[2].clone(), matches[3].clone()],
            cx,
        );

        assert!(
            delegate.matches.history.is_empty(),
            "Search matches expected"
        );
        assert_eq!(delegate.matches.search.as_slice(), &matches[0..4]);
    });
}

#[gpui::test]
async fn test_ignored_root(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            "/ancestor",
            json!({
                ".gitignore": "ignored-root",
                "ignored-root": {
                    "happiness": "",
                    "height": "",
                    "hi": "",
                    "hiccup": "",
                },
                "tracked-root": {
                    ".gitignore": "height",
                    "happiness": "",
                    "height": "",
                    "hi": "",
                    "hiccup": "",
                },
            }),
        )
        .await;

    let project = Project::test(
        app_state.fs.clone(),
        [
            "/ancestor/tracked-root".as_ref(),
            "/ancestor/ignored-root".as_ref(),
        ],
        cx,
    )
    .await;

    let (picker, _, cx) = build_find_picker(project, cx);

    picker
        .update(cx, |picker, cx| {
            picker.delegate.spawn_search(test_path_like("hi"), cx)
        })
        .await;
    picker.update(cx, |picker, _| assert_eq!(picker.delegate.matches.len(), 7));
}

#[gpui::test]
async fn test_single_file_worktrees(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree("/root", json!({ "the-parent-dir": { "the-file": "" } }))
        .await;

    let project = Project::test(
        app_state.fs.clone(),
        ["/root/the-parent-dir/the-file".as_ref()],
        cx,
    )
    .await;

    let (picker, _, cx) = build_find_picker(project, cx);

    // Even though there is only one worktree, that worktree's filename
    // is included in the matching, because the worktree is a single file.
    picker
        .update(cx, |picker, cx| {
            picker.delegate.spawn_search(test_path_like("thf"), cx)
        })
        .await;
    cx.read(|cx| {
        let picker = picker.read(cx);
        let delegate = &picker.delegate;
        assert!(
            delegate.matches.history.is_empty(),
            "Search matches expected"
        );
        let matches = delegate.matches.search.clone();
        assert_eq!(matches.len(), 1);

        let (file_name, file_name_positions, full_path, full_path_positions) =
            delegate.labels_for_path_match(&matches[0]);
        assert_eq!(file_name, "the-file");
        assert_eq!(file_name_positions, &[0, 1, 4]);
        assert_eq!(full_path, "the-file");
        assert_eq!(full_path_positions, &[0, 1, 4]);
    });

    // Since the worktree root is a file, searching for its name followed by a slash does
    // not match anything.
    picker
        .update(cx, |f, cx| {
            f.delegate.spawn_search(test_path_like("thf/"), cx)
        })
        .await;
    picker.update(cx, |f, _| assert_eq!(f.delegate.matches.len(), 0));
}

#[gpui::test]
async fn test_path_distance_ordering(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            "/root",
            json!({
                "dir1": { "a.txt": "" },
                "dir2": {
                    "a.txt": "",
                    "b.txt": ""
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project, cx));

    let worktree_id = cx.read(|cx| {
        let worktrees = workspace.read(cx).worktrees(cx).collect::<Vec<_>>();
        assert_eq!(worktrees.len(), 1);
        WorktreeId::from_usize(worktrees[0].entity_id().as_u64() as usize)
    });

    // When workspace has an active item, sort items which are closer to that item
    // first when they have the same name. In this case, b.txt is closer to dir2's a.txt
    // so that one should be sorted earlier
    let b_path = ProjectPath {
        worktree_id,
        path: Arc::from(Path::new("dir2/b.txt")),
    };
    workspace
        .update(cx, |workspace, cx| {
            workspace.open_path(b_path, None, true, cx)
        })
        .await
        .unwrap();
    let finder = open_file_picker(&workspace, cx);
    finder
        .update(cx, |f, cx| {
            f.delegate.spawn_search(test_path_like("a.txt"), cx)
        })
        .await;

    finder.update(cx, |f, _| {
        let delegate = &f.delegate;
        assert!(
            delegate.matches.history.is_empty(),
            "Search matches expected"
        );
        let matches = delegate.matches.search.clone();
        assert_eq!(matches[0].path.as_ref(), Path::new("dir2/a.txt"));
        assert_eq!(matches[1].path.as_ref(), Path::new("dir1/a.txt"));
    });
}

#[gpui::test]
async fn test_search_worktree_without_files(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            "/root",
            json!({
                "dir1": {},
                "dir2": {
                    "dir3": {}
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;
    let (picker, _workspace, cx) = build_find_picker(project, cx);

    picker
        .update(cx, |f, cx| {
            f.delegate.spawn_search(test_path_like("dir"), cx)
        })
        .await;
    cx.read(|cx| {
        let finder = picker.read(cx);
        assert_eq!(finder.delegate.matches.len(), 0);
    });
}

#[gpui::test]
async fn test_query_history(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            "/src",
            json!({
                "test": {
                    "first.rs": "// First Rust file",
                    "second.rs": "// Second Rust file",
                    "third.rs": "// Third Rust file",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), ["/src".as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project, cx));
    let worktree_id = cx.read(|cx| {
        let worktrees = workspace.read(cx).worktrees(cx).collect::<Vec<_>>();
        assert_eq!(worktrees.len(), 1);
        WorktreeId::from_usize(worktrees[0].entity_id().as_u64() as usize)
    });

    // Open and close panels, getting their history items afterwards.
    // Ensure history items get populated with opened items, and items are kept in a certain order.
    // The history lags one opened buffer behind, since it's updated in the search panel only on its reopen.
    //
    // TODO: without closing, the opened items do not propagate their history changes for some reason
    // it does work in real app though, only tests do not propagate.
    workspace.update(cx, |_, cx| cx.focused());

    let initial_history = open_close_queried_buffer("fir", 1, "first.rs", &workspace, cx).await;
    assert!(
        initial_history.is_empty(),
        "Should have no history before opening any files"
    );

    let history_after_first =
        open_close_queried_buffer("sec", 1, "second.rs", &workspace, cx).await;
    assert_eq!(
        history_after_first,
        vec![FoundPath::new(
            ProjectPath {
                worktree_id,
                path: Arc::from(Path::new("test/first.rs")),
            },
            Some(PathBuf::from("/src/test/first.rs"))
        )],
        "Should show 1st opened item in the history when opening the 2nd item"
    );

    let history_after_second =
        open_close_queried_buffer("thi", 1, "third.rs", &workspace, cx).await;
    assert_eq!(
        history_after_second,
        vec![
            FoundPath::new(
                ProjectPath {
                    worktree_id,
                    path: Arc::from(Path::new("test/second.rs")),
                },
                Some(PathBuf::from("/src/test/second.rs"))
            ),
            FoundPath::new(
                ProjectPath {
                    worktree_id,
                    path: Arc::from(Path::new("test/first.rs")),
                },
                Some(PathBuf::from("/src/test/first.rs"))
            ),
        ],
        "Should show 1st and 2nd opened items in the history when opening the 3rd item. \
    2nd item should be the first in the history, as the last opened."
    );

    let history_after_third =
        open_close_queried_buffer("sec", 1, "second.rs", &workspace, cx).await;
    assert_eq!(
                history_after_third,
                vec![
                    FoundPath::new(
                        ProjectPath {
                            worktree_id,
                            path: Arc::from(Path::new("test/third.rs")),
                        },
                        Some(PathBuf::from("/src/test/third.rs"))
                    ),
                    FoundPath::new(
                        ProjectPath {
                            worktree_id,
                            path: Arc::from(Path::new("test/second.rs")),
                        },
                        Some(PathBuf::from("/src/test/second.rs"))
                    ),
                    FoundPath::new(
                        ProjectPath {
                            worktree_id,
                            path: Arc::from(Path::new("test/first.rs")),
                        },
                        Some(PathBuf::from("/src/test/first.rs"))
                    ),
                ],
                "Should show 1st, 2nd and 3rd opened items in the history when opening the 2nd item again. \
    3rd item should be the first in the history, as the last opened."
            );

    let history_after_second_again =
        open_close_queried_buffer("thi", 1, "third.rs", &workspace, cx).await;
    assert_eq!(
                history_after_second_again,
                vec![
                    FoundPath::new(
                        ProjectPath {
                            worktree_id,
                            path: Arc::from(Path::new("test/second.rs")),
                        },
                        Some(PathBuf::from("/src/test/second.rs"))
                    ),
                    FoundPath::new(
                        ProjectPath {
                            worktree_id,
                            path: Arc::from(Path::new("test/third.rs")),
                        },
                        Some(PathBuf::from("/src/test/third.rs"))
                    ),
                    FoundPath::new(
                        ProjectPath {
                            worktree_id,
                            path: Arc::from(Path::new("test/first.rs")),
                        },
                        Some(PathBuf::from("/src/test/first.rs"))
                    ),
                ],
                "Should show 1st, 2nd and 3rd opened items in the history when opening the 3rd item again. \
    2nd item, as the last opened, 3rd item should go next as it was opened right before."
            );
}

#[gpui::test]
async fn test_external_files_history(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            "/src",
            json!({
                "test": {
                    "first.rs": "// First Rust file",
                    "second.rs": "// Second Rust file",
                }
            }),
        )
        .await;

    app_state
        .fs
        .as_fake()
        .insert_tree(
            "/external-src",
            json!({
                "test": {
                    "third.rs": "// Third Rust file",
                    "fourth.rs": "// Fourth Rust file",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), ["/src".as_ref()], cx).await;
    cx.update(|cx| {
        project.update(cx, |project, cx| {
            project.find_or_create_local_worktree("/external-src", false, cx)
        })
    })
    .detach();
    cx.background_executor.run_until_parked();

    let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project, cx));
    let worktree_id = cx.read(|cx| {
        let worktrees = workspace.read(cx).worktrees(cx).collect::<Vec<_>>();
        assert_eq!(worktrees.len(), 1,);

        WorktreeId::from_usize(worktrees[0].entity_id().as_u64() as usize)
    });
    workspace
        .update(cx, |workspace, cx| {
            workspace.open_abs_path(PathBuf::from("/external-src/test/third.rs"), false, cx)
        })
        .detach();
    cx.background_executor.run_until_parked();
    let external_worktree_id = cx.read(|cx| {
        let worktrees = workspace.read(cx).worktrees(cx).collect::<Vec<_>>();
        assert_eq!(
            worktrees.len(),
            2,
            "External file should get opened in a new worktree"
        );

        WorktreeId::from_usize(
            worktrees
                .into_iter()
                .find(|worktree| worktree.entity_id().as_u64() as usize != worktree_id.to_usize())
                .expect("New worktree should have a different id")
                .entity_id()
                .as_u64() as usize,
        )
    });
    cx.dispatch_action(workspace::CloseActiveItem { save_intent: None });

    let initial_history_items =
        open_close_queried_buffer("sec", 1, "second.rs", &workspace, cx).await;
    assert_eq!(
        initial_history_items,
        vec![FoundPath::new(
            ProjectPath {
                worktree_id: external_worktree_id,
                path: Arc::from(Path::new("")),
            },
            Some(PathBuf::from("/external-src/test/third.rs"))
        )],
        "Should show external file with its full path in the history after it was open"
    );

    let updated_history_items =
        open_close_queried_buffer("fir", 1, "first.rs", &workspace, cx).await;
    assert_eq!(
        updated_history_items,
        vec![
            FoundPath::new(
                ProjectPath {
                    worktree_id,
                    path: Arc::from(Path::new("test/second.rs")),
                },
                Some(PathBuf::from("/src/test/second.rs"))
            ),
            FoundPath::new(
                ProjectPath {
                    worktree_id: external_worktree_id,
                    path: Arc::from(Path::new("")),
                },
                Some(PathBuf::from("/external-src/test/third.rs"))
            ),
        ],
        "Should keep external file with history updates",
    );
}

#[gpui::test]
async fn test_toggle_panel_new_selections(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            "/src",
            json!({
                "test": {
                    "first.rs": "// First Rust file",
                    "second.rs": "// Second Rust file",
                    "third.rs": "// Third Rust file",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), ["/src".as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project, cx));

    // generate some history to select from
    open_close_queried_buffer("fir", 1, "first.rs", &workspace, cx).await;
    cx.executor().run_until_parked();
    open_close_queried_buffer("sec", 1, "second.rs", &workspace, cx).await;
    open_close_queried_buffer("thi", 1, "third.rs", &workspace, cx).await;
    let current_history = open_close_queried_buffer("sec", 1, "second.rs", &workspace, cx).await;

    for expected_selected_index in 0..current_history.len() {
        cx.dispatch_action(Toggle);
        let picker = active_file_picker(&workspace, cx);
        let selected_index = picker.update(cx, |picker, _| picker.delegate.selected_index());
        assert_eq!(
            selected_index, expected_selected_index,
            "Should select the next item in the history"
        );
    }

    cx.dispatch_action(Toggle);
    let selected_index = workspace.update(cx, |workspace, cx| {
        workspace
            .active_modal::<FileFinder>(cx)
            .unwrap()
            .read(cx)
            .picker
            .read(cx)
            .delegate
            .selected_index()
    });
    assert_eq!(
        selected_index, 0,
        "Should wrap around the history and start all over"
    );
}

#[gpui::test]
async fn test_search_preserves_history_items(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            "/src",
            json!({
                "test": {
                    "first.rs": "// First Rust file",
                    "second.rs": "// Second Rust file",
                    "third.rs": "// Third Rust file",
                    "fourth.rs": "// Fourth Rust file",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), ["/src".as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project, cx));
    let worktree_id = cx.read(|cx| {
        let worktrees = workspace.read(cx).worktrees(cx).collect::<Vec<_>>();
        assert_eq!(worktrees.len(), 1,);

        WorktreeId::from_usize(worktrees[0].entity_id().as_u64() as usize)
    });

    // generate some history to select from
    open_close_queried_buffer("fir", 1, "first.rs", &workspace, cx).await;
    open_close_queried_buffer("sec", 1, "second.rs", &workspace, cx).await;
    open_close_queried_buffer("thi", 1, "third.rs", &workspace, cx).await;
    open_close_queried_buffer("sec", 1, "second.rs", &workspace, cx).await;

    let finder = open_file_picker(&workspace, cx);
    let first_query = "f";
    finder
        .update(cx, |finder, cx| {
            finder.delegate.update_matches(first_query.to_string(), cx)
        })
        .await;
    finder.update(cx, |finder, _| {
            let delegate = &finder.delegate;
            assert_eq!(delegate.matches.history.len(), 1, "Only one history item contains {first_query}, it should be present and others should be filtered out");
            let history_match = delegate.matches.history.first().unwrap();
            assert!(history_match.1.is_some(), "Should have path matches for history items after querying");
            assert_eq!(history_match.0, FoundPath::new(
                ProjectPath {
                    worktree_id,
                    path: Arc::from(Path::new("test/first.rs")),
                },
                Some(PathBuf::from("/src/test/first.rs"))
            ));
            assert_eq!(delegate.matches.search.len(), 1, "Only one non-history item contains {first_query}, it should be present");
            assert_eq!(delegate.matches.search.first().unwrap().path.as_ref(), Path::new("test/fourth.rs"));
        });

    let second_query = "fsdasdsa";
    let finder = active_file_picker(&workspace, cx);
    finder
        .update(cx, |finder, cx| {
            finder.delegate.update_matches(second_query.to_string(), cx)
        })
        .await;
    finder.update(cx, |finder, _| {
        let delegate = &finder.delegate;
        assert!(
            delegate.matches.history.is_empty(),
            "No history entries should match {second_query}"
        );
        assert!(
            delegate.matches.search.is_empty(),
            "No search entries should match {second_query}"
        );
    });

    let first_query_again = first_query;

    let finder = active_file_picker(&workspace, cx);
    finder
        .update(cx, |finder, cx| {
            finder
                .delegate
                .update_matches(first_query_again.to_string(), cx)
        })
        .await;
    finder.update(cx, |finder, _| {
            let delegate = &finder.delegate;
            assert_eq!(delegate.matches.history.len(), 1, "Only one history item contains {first_query_again}, it should be present and others should be filtered out, even after non-matching query");
            let history_match = delegate.matches.history.first().unwrap();
            assert!(history_match.1.is_some(), "Should have path matches for history items after querying");
            assert_eq!(history_match.0, FoundPath::new(
                ProjectPath {
                    worktree_id,
                    path: Arc::from(Path::new("test/first.rs")),
                },
                Some(PathBuf::from("/src/test/first.rs"))
            ));
            assert_eq!(delegate.matches.search.len(), 1, "Only one non-history item contains {first_query_again}, it should be present, even after non-matching query");
            assert_eq!(delegate.matches.search.first().unwrap().path.as_ref(), Path::new("test/fourth.rs"));
        });
}

#[gpui::test]
async fn test_history_items_vs_very_good_external_match(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            "/src",
            json!({
                "collab_ui": {
                    "first.rs": "// First Rust file",
                    "second.rs": "// Second Rust file",
                    "third.rs": "// Third Rust file",
                    "collab_ui.rs": "// Fourth Rust file",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), ["/src".as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project, cx));
    // generate some history to select from
    open_close_queried_buffer("fir", 1, "first.rs", &workspace, cx).await;
    open_close_queried_buffer("sec", 1, "second.rs", &workspace, cx).await;
    open_close_queried_buffer("thi", 1, "third.rs", &workspace, cx).await;
    open_close_queried_buffer("sec", 1, "second.rs", &workspace, cx).await;

    let finder = open_file_picker(&workspace, cx);
    let query = "collab_ui";
    cx.simulate_input(query);
    finder.update(cx, |finder, _| {
            let delegate = &finder.delegate;
            assert!(
                delegate.matches.history.is_empty(),
                "History items should not math query {query}, they should be matched by name only"
            );

            let search_entries = delegate
                .matches
                .search
                .iter()
                .map(|path_match| path_match.path.to_path_buf())
                .collect::<Vec<_>>();
            assert_eq!(
                search_entries,
                vec![
                    PathBuf::from("collab_ui/collab_ui.rs"),
                    PathBuf::from("collab_ui/third.rs"),
                    PathBuf::from("collab_ui/first.rs"),
                    PathBuf::from("collab_ui/second.rs"),
                ],
                "Despite all search results having the same directory name, the most matching one should be on top"
            );
        });
}

#[gpui::test]
async fn test_nonexistent_history_items_not_shown(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            "/src",
            json!({
                "test": {
                    "first.rs": "// First Rust file",
                    "nonexistent.rs": "// Second Rust file",
                    "third.rs": "// Third Rust file",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), ["/src".as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project, cx)); // generate some history to select from
    open_close_queried_buffer("fir", 1, "first.rs", &workspace, cx).await;
    open_close_queried_buffer("non", 1, "nonexistent.rs", &workspace, cx).await;
    open_close_queried_buffer("thi", 1, "third.rs", &workspace, cx).await;
    open_close_queried_buffer("fir", 1, "first.rs", &workspace, cx).await;

    let picker = open_file_picker(&workspace, cx);
    cx.simulate_input("rs");

    picker.update(cx, |finder, _| {
            let history_entries = finder.delegate
                .matches
                .history
                .iter()
                .map(|(_, path_match)| path_match.as_ref().expect("should have a path match").path.to_path_buf())
                .collect::<Vec<_>>();
            assert_eq!(
                history_entries,
                vec![
                    PathBuf::from("test/first.rs"),
                    PathBuf::from("test/third.rs"),
                ],
                "Should have all opened files in the history, except the ones that do not exist on disk"
            );
        });
}

async fn open_close_queried_buffer(
    input: &str,
    expected_matches: usize,
    expected_editor_title: &str,
    workspace: &View<Workspace>,
    cx: &mut gpui::VisualTestContext,
) -> Vec<FoundPath> {
    let picker = open_file_picker(&workspace, cx);
    cx.simulate_input(input);

    let history_items = picker.update(cx, |finder, _| {
        assert_eq!(
            finder.delegate.matches.len(),
            expected_matches,
            "Unexpected number of matches found for query {input}"
        );
        finder.delegate.history_items.clone()
    });

    cx.dispatch_action(SelectNext);
    cx.dispatch_action(Confirm);

    cx.read(|cx| {
        let active_editor = workspace.read(cx).active_item_as::<Editor>(cx).unwrap();
        let active_editor_title = active_editor.read(cx).title(cx);
        assert_eq!(
            expected_editor_title, active_editor_title,
            "Unexpected editor title for query {input}"
        );
    });

    cx.dispatch_action(workspace::CloseActiveItem { save_intent: None });

    history_items
}

fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
    cx.update(|cx| {
        let state = AppState::test(cx);
        theme::init(theme::LoadThemes::JustBase, cx);
        language::init(cx);
        super::init(cx);
        editor::init(cx);
        workspace::init_settings(cx);
        Project::init_settings(cx);
        state
    })
}

fn test_path_like(test_str: &str) -> PathLikeWithPosition<FileSearchQuery> {
    PathLikeWithPosition::parse_str(test_str, |path_like_str| {
        Ok::<_, std::convert::Infallible>(FileSearchQuery {
            raw_query: test_str.to_owned(),
            file_query_end: if path_like_str == test_str {
                None
            } else {
                Some(path_like_str.len())
            },
        })
    })
    .unwrap()
}

fn build_find_picker(
    project: Model<Project>,
    cx: &mut TestAppContext,
) -> (
    View<Picker<FileFinderDelegate>>,
    View<Workspace>,
    &mut VisualTestContext,
) {
    let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project, cx));
    let picker = open_file_picker(&workspace, cx);
    (picker, workspace, cx)
}

#[track_caller]
fn open_file_picker(
    workspace: &View<Workspace>,
    cx: &mut VisualTestContext,
) -> View<Picker<FileFinderDelegate>> {
    cx.dispatch_action(Toggle);
    active_file_picker(workspace, cx)
}

#[track_caller]
fn active_file_picker(
    workspace: &View<Workspace>,
    cx: &mut VisualTestContext,
) -> View<Picker<FileFinderDelegate>> {
    workspace.update(cx, |workspace, cx| {
        workspace
            .active_modal::<FileFinder>(cx)
            .unwrap()
            .read(cx)
            .picker
            .clone()
    })
}

fn collect_search_results(picker: &Picker<FileFinderDelegate>) -> Vec<PathBuf> {
    let matches = &picker.delegate.matches;
    assert!(
        matches.history.is_empty(),
        "Should have no history matches, but got: {:?}",
        matches.history
    );
    let mut results = matches
        .search
        .iter()
        .map(|path_match| Path::new(path_match.path_prefix.as_ref()).join(&path_match.path))
        .collect::<Vec<_>>();
    results.sort();
    results
}
