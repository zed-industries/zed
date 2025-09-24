use std::{future::IntoFuture, path::Path, time::Duration};

use super::*;
use editor::Editor;
use gpui::{Entity, TestAppContext, VisualTestContext};
use menu::{Confirm, SelectNext, SelectPrevious};
use pretty_assertions::{assert_eq, assert_matches};
use project::{FS_WATCH_LATENCY, RemoveOptions};
use serde_json::json;
use util::{path, rel_path::rel_path};
use workspace::{AppState, CloseActiveItem, OpenOptions, ToggleFileFinder, Workspace};

#[ctor::ctor]
fn init_logger() {
    zlog::init_test();
}

#[test]
fn test_path_elision() {
    #[track_caller]
    fn check(path: &str, budget: usize, matches: impl IntoIterator<Item = usize>, expected: &str) {
        let mut path = path.to_owned();
        let slice = PathComponentSlice::new(&path);
        let matches = Vec::from_iter(matches);
        if let Some(range) = slice.elision_range(budget - 1, &matches) {
            path.replace_range(range, "…");
        }
        assert_eq!(path, expected);
    }

    // Simple cases, mostly to check that different path shapes are handled gracefully.
    check("p/a/b/c/d/", 6, [], "p/…/d/");
    check("p/a/b/c/d/", 1, [2, 4, 6], "p/a/b/c/d/");
    check("p/a/b/c/d/", 10, [2, 6], "p/a/…/c/d/");
    check("p/a/b/c/d/", 8, [6], "p/…/c/d/");

    check("p/a/b/c/d", 5, [], "p/…/d");
    check("p/a/b/c/d", 9, [2, 4, 6], "p/a/b/c/d");
    check("p/a/b/c/d", 9, [2, 6], "p/a/…/c/d");
    check("p/a/b/c/d", 7, [6], "p/…/c/d");

    check("/p/a/b/c/d/", 7, [], "/p/…/d/");
    check("/p/a/b/c/d/", 11, [3, 5, 7], "/p/a/b/c/d/");
    check("/p/a/b/c/d/", 11, [3, 7], "/p/a/…/c/d/");
    check("/p/a/b/c/d/", 9, [7], "/p/…/c/d/");

    // If the budget can't be met, no elision is done.
    check(
        "project/dir/child/grandchild",
        5,
        [],
        "project/dir/child/grandchild",
    );

    // The longest unmatched segment is picked for elision.
    check(
        "project/one/two/X/three/sub",
        21,
        [16],
        "project/…/X/three/sub",
    );

    // Elision stops when the budget is met, even though there are more components in the chosen segment.
    // It proceeds from the end of the unmatched segment that is closer to the midpoint of the path.
    check(
        "project/one/two/three/X/sub",
        21,
        [22],
        "project/…/three/X/sub",
    )
}

#[test]
fn test_custom_project_search_ordering_in_file_finder() {
    let mut file_finder_sorted_output = vec![
        ProjectPanelOrdMatch(PathMatch {
            score: 0.5,
            positions: Vec::new(),
            worktree_id: 0,
            path: rel_path("b0.5").into(),
            path_prefix: rel_path("").into(),
            distance_to_relative_ancestor: 0,
            is_dir: false,
        }),
        ProjectPanelOrdMatch(PathMatch {
            score: 1.0,
            positions: Vec::new(),
            worktree_id: 0,
            path: rel_path("c1.0").into(),
            path_prefix: rel_path("").into(),
            distance_to_relative_ancestor: 0,
            is_dir: false,
        }),
        ProjectPanelOrdMatch(PathMatch {
            score: 1.0,
            positions: Vec::new(),
            worktree_id: 0,
            path: rel_path("a1.0").into(),
            path_prefix: rel_path("").into(),
            distance_to_relative_ancestor: 0,
            is_dir: false,
        }),
        ProjectPanelOrdMatch(PathMatch {
            score: 0.5,
            positions: Vec::new(),
            worktree_id: 0,
            path: rel_path("a0.5").into(),
            path_prefix: rel_path("").into(),
            distance_to_relative_ancestor: 0,
            is_dir: false,
        }),
        ProjectPanelOrdMatch(PathMatch {
            score: 1.0,
            positions: Vec::new(),
            worktree_id: 0,
            path: rel_path("b1.0").into(),
            path_prefix: rel_path("").into(),
            distance_to_relative_ancestor: 0,
            is_dir: false,
        }),
    ];
    file_finder_sorted_output.sort_by(|a, b| b.cmp(a));

    assert_eq!(
        file_finder_sorted_output,
        vec![
            ProjectPanelOrdMatch(PathMatch {
                score: 1.0,
                positions: Vec::new(),
                worktree_id: 0,
                path: rel_path("a1.0").into(),
                path_prefix: rel_path("").into(),
                distance_to_relative_ancestor: 0,
                is_dir: false,
            }),
            ProjectPanelOrdMatch(PathMatch {
                score: 1.0,
                positions: Vec::new(),
                worktree_id: 0,
                path: rel_path("b1.0").into(),
                path_prefix: rel_path("").into(),
                distance_to_relative_ancestor: 0,
                is_dir: false,
            }),
            ProjectPanelOrdMatch(PathMatch {
                score: 1.0,
                positions: Vec::new(),
                worktree_id: 0,
                path: rel_path("c1.0").into(),
                path_prefix: rel_path("").into(),
                distance_to_relative_ancestor: 0,
                is_dir: false,
            }),
            ProjectPanelOrdMatch(PathMatch {
                score: 0.5,
                positions: Vec::new(),
                worktree_id: 0,
                path: rel_path("a0.5").into(),
                path_prefix: rel_path("").into(),
                distance_to_relative_ancestor: 0,
                is_dir: false,
            }),
            ProjectPanelOrdMatch(PathMatch {
                score: 0.5,
                positions: Vec::new(),
                worktree_id: 0,
                path: rel_path("b0.5").into(),
                path_prefix: rel_path("").into(),
                distance_to_relative_ancestor: 0,
                is_dir: false,
            }),
        ]
    );
}

#[gpui::test]
async fn test_matching_paths(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "a": {
                    "banana": "",
                    "bandana": "",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;

    let (picker, workspace, cx) = build_find_picker(project, cx);

    cx.simulate_input("bna");
    picker.update(cx, |picker, _| {
        assert_eq!(picker.delegate.matches.len(), 3);
    });
    cx.dispatch_action(SelectNext);
    cx.dispatch_action(Confirm);
    cx.read(|cx| {
        let active_editor = workspace.read(cx).active_item_as::<Editor>(cx).unwrap();
        assert_eq!(active_editor.read(cx).title(cx), "bandana");
    });

    for bandana_query in [
        "bandana",
        "./bandana",
        ".\\bandana",
        util::path!("a/bandana"),
        "b/bandana",
        "b\\bandana",
        " bandana",
        "bandana ",
        " bandana ",
        " ndan ",
        " band ",
        "a bandana",
        "bandana:",
    ] {
        picker
            .update_in(cx, |picker, window, cx| {
                picker
                    .delegate
                    .update_matches(bandana_query.to_string(), window, cx)
            })
            .await;
        picker.update(cx, |picker, _| {
            assert_eq!(
                picker.delegate.matches.len(),
                // existence of CreateNew option depends on whether path already exists
                if bandana_query == util::path!("a/bandana") {
                    1
                } else {
                    2
                },
                "Wrong number of matches for bandana query '{bandana_query}'. Matches: {:?}",
                picker.delegate.matches
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
async fn test_matching_paths_with_colon(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "a": {
                    "foo:bar.rs": "",
                    "foo.rs": "",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;

    let (picker, _, cx) = build_find_picker(project, cx);

    // 'foo:' matches both files
    cx.simulate_input("foo:");
    picker.update(cx, |picker, _| {
        assert_eq!(picker.delegate.matches.len(), 3);
        assert_match_at_position(picker, 0, "foo.rs");
        assert_match_at_position(picker, 1, "foo:bar.rs");
    });

    // 'foo:b' matches one of the files
    cx.simulate_input("b");
    picker.update(cx, |picker, _| {
        assert_eq!(picker.delegate.matches.len(), 2);
        assert_match_at_position(picker, 0, "foo:bar.rs");
    });

    cx.dispatch_action(editor::actions::Backspace);

    // 'foo:1' matches both files, specifying which row to jump to
    cx.simulate_input("1");
    picker.update(cx, |picker, _| {
        assert_eq!(picker.delegate.matches.len(), 3);
        assert_match_at_position(picker, 0, "foo.rs");
        assert_match_at_position(picker, 1, "foo:bar.rs");
    });
}

#[gpui::test]
async fn test_unicode_paths(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "a": {
                    "İg": " ",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;

    let (picker, workspace, cx) = build_find_picker(project, cx);

    cx.simulate_input("g");
    picker.update(cx, |picker, _| {
        assert_eq!(picker.delegate.matches.len(), 2);
        assert_match_at_position(picker, 1, "g");
    });
    cx.dispatch_action(Confirm);
    cx.read(|cx| {
        let active_editor = workspace.read(cx).active_item_as::<Editor>(cx).unwrap();
        assert_eq!(active_editor.read(cx).title(cx), "İg");
    });
}

#[gpui::test]
async fn test_absolute_paths(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
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

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;

    let (picker, workspace, cx) = build_find_picker(project, cx);

    let matching_abs_path = path!("/root/a/b/file2.txt").to_string();
    picker
        .update_in(cx, |picker, window, cx| {
            picker
                .delegate
                .update_matches(matching_abs_path, window, cx)
        })
        .await;
    picker.update(cx, |picker, _| {
        assert_eq!(
            collect_search_matches(picker).search_paths_only(),
            vec![rel_path("a/b/file2.txt").into()],
            "Matching abs path should be the only match"
        )
    });
    cx.dispatch_action(SelectNext);
    cx.dispatch_action(Confirm);
    cx.read(|cx| {
        let active_editor = workspace.read(cx).active_item_as::<Editor>(cx).unwrap();
        assert_eq!(active_editor.read(cx).title(cx), "file2.txt");
    });

    let mismatching_abs_path = path!("/root/a/b/file1.txt").to_string();
    picker
        .update_in(cx, |picker, window, cx| {
            picker
                .delegate
                .update_matches(mismatching_abs_path, window, cx)
        })
        .await;
    picker.update(cx, |picker, _| {
        assert_eq!(
            collect_search_matches(picker).search_paths_only(),
            Vec::new(),
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
            path!("/root"),
            json!({
                "其他": {
                    "S数据表格": {
                        "task.xlsx": "some content",
                    },
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;

    let (picker, workspace, cx) = build_find_picker(project, cx);

    cx.simulate_input("t");
    picker.update(cx, |picker, _| {
        assert_eq!(picker.delegate.matches.len(), 2);
        assert_eq!(
            collect_search_matches(picker).search_paths_only(),
            vec![rel_path("其他/S数据表格/task.xlsx").into()],
        )
    });
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
            path!("/src"),
            json!({
                "test": {
                    first_file_name: first_file_contents,
                    "second.rs": "// Second Rust file",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/src").as_ref()], cx).await;

    let (picker, workspace, cx) = build_find_picker(project, cx);

    let file_query = &first_file_name[..3];
    let file_row = 1;
    let file_column = 3;
    assert!(file_column <= first_file_contents.len());
    let query_inside_file = format!("{file_query}:{file_row}:{file_column}");
    picker
        .update_in(cx, |finder, window, cx| {
            finder
                .delegate
                .update_matches(query_inside_file.to_string(), window, cx)
        })
        .await;
    picker.update(cx, |finder, _| {
        assert_match_at_position(finder, 1, &query_inside_file.to_string());
        let finder = &finder.delegate;
        assert_eq!(finder.matches.len(), 2);
        let latest_search_query = finder
            .latest_search_query
            .as_ref()
            .expect("Finder should have a query after the update_matches call");
        assert_eq!(latest_search_query.raw_query, query_inside_file);
        assert_eq!(latest_search_query.file_query_end, Some(file_query.len()));
        assert_eq!(latest_search_query.path_position.row, Some(file_row));
        assert_eq!(
            latest_search_query.path_position.column,
            Some(file_column as u32)
        );
    });

    cx.dispatch_action(Confirm);

    let editor = cx.update(|_, cx| workspace.read(cx).active_item_as::<Editor>(cx).unwrap());
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
            path!("/src"),
            json!({
                "test": {
                    first_file_name: first_file_contents,
                    "second.rs": "// Second Rust file",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/src").as_ref()], cx).await;

    let (picker, workspace, cx) = build_find_picker(project, cx);

    let file_query = &first_file_name[..3];
    let file_row = 200;
    let file_column = 300;
    assert!(file_column > first_file_contents.len());
    let query_outside_file = format!("{file_query}:{file_row}:{file_column}");
    picker
        .update_in(cx, |picker, window, cx| {
            picker
                .delegate
                .update_matches(query_outside_file.to_string(), window, cx)
        })
        .await;
    picker.update(cx, |finder, _| {
        assert_match_at_position(finder, 1, &query_outside_file.to_string());
        let delegate = &finder.delegate;
        assert_eq!(delegate.matches.len(), 2);
        let latest_search_query = delegate
            .latest_search_query
            .as_ref()
            .expect("Finder should have a query after the update_matches call");
        assert_eq!(latest_search_query.raw_query, query_outside_file);
        assert_eq!(latest_search_query.file_query_end, Some(file_query.len()));
        assert_eq!(latest_search_query.path_position.row, Some(file_row));
        assert_eq!(
            latest_search_query.path_position.column,
            Some(file_column as u32)
        );
    });

    cx.dispatch_action(Confirm);

    let editor = cx.update(|_, cx| workspace.read(cx).active_item_as::<Editor>(cx).unwrap());
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

    let query = test_path_position("hi");
    picker
        .update_in(cx, |picker, window, cx| {
            picker.delegate.spawn_search(query.clone(), window, cx)
        })
        .await;

    picker.update(cx, |picker, _cx| {
        // CreateNew option not shown in this case since file already exists
        assert_eq!(picker.delegate.matches.len(), 5);
    });

    picker.update_in(cx, |picker, window, cx| {
        let matches = collect_search_matches(picker).search_matches_only();
        let delegate = &mut picker.delegate;

        // Simulate a search being cancelled after the time limit,
        // returning only a subset of the matches that would have been found.
        drop(delegate.spawn_search(query.clone(), window, cx));
        delegate.set_search_matches(
            delegate.latest_search_id,
            true, // did-cancel
            query.clone(),
            vec![
                ProjectPanelOrdMatch(matches[1].clone()),
                ProjectPanelOrdMatch(matches[3].clone()),
            ],
            cx,
        );

        // Simulate another cancellation.
        drop(delegate.spawn_search(query.clone(), window, cx));
        delegate.set_search_matches(
            delegate.latest_search_id,
            true, // did-cancel
            query.clone(),
            vec![
                ProjectPanelOrdMatch(matches[0].clone()),
                ProjectPanelOrdMatch(matches[2].clone()),
                ProjectPanelOrdMatch(matches[3].clone()),
            ],
            cx,
        );

        assert_eq!(
            collect_search_matches(picker)
                .search_matches_only()
                .as_slice(),
            &matches[0..4]
        );
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
                    ".gitignore": "height*",
                    "happiness": "",
                    "height": "",
                    "heights": {
                        "height_1": "",
                        "height_2": "",
                    },
                    "hi": "",
                    "hiccup": "",
                },
            }),
        )
        .await;

    let project = Project::test(
        app_state.fs.clone(),
        [
            Path::new(path!("/ancestor/tracked-root")),
            Path::new(path!("/ancestor/ignored-root")),
        ],
        cx,
    )
    .await;
    let (picker, workspace, cx) = build_find_picker(project, cx);

    picker
        .update_in(cx, |picker, window, cx| {
            picker
                .delegate
                .spawn_search(test_path_position("hi"), window, cx)
        })
        .await;
    picker.update(cx, |picker, _| {
        let matches = collect_search_matches(picker);
        assert_eq!(matches.history.len(), 0);
        assert_eq!(
            matches.search,
            vec![
                rel_path("ignored-root/hi").into(),
                rel_path("tracked-root/hi").into(),
                rel_path("ignored-root/hiccup").into(),
                rel_path("tracked-root/hiccup").into(),
                rel_path("ignored-root/height").into(),
                rel_path("ignored-root/happiness").into(),
                rel_path("tracked-root/happiness").into(),
            ],
            "All ignored files that were indexed are found for default ignored mode"
        );
    });
    cx.dispatch_action(ToggleIncludeIgnored);
    picker
        .update_in(cx, |picker, window, cx| {
            picker
                .delegate
                .spawn_search(test_path_position("hi"), window, cx)
        })
        .await;
    picker.update(cx, |picker, _| {
        let matches = collect_search_matches(picker);
        assert_eq!(matches.history.len(), 0);
        assert_eq!(
            matches.search,
            vec![
                rel_path("ignored-root/hi").into(),
                rel_path("tracked-root/hi").into(),
                rel_path("ignored-root/hiccup").into(),
                rel_path("tracked-root/hiccup").into(),
                rel_path("ignored-root/height").into(),
                rel_path("tracked-root/height").into(),
                rel_path("ignored-root/happiness").into(),
                rel_path("tracked-root/happiness").into(),
            ],
            "All ignored files should be found, for the toggled on ignored mode"
        );
    });

    picker
        .update_in(cx, |picker, window, cx| {
            picker.delegate.include_ignored = Some(false);
            picker
                .delegate
                .spawn_search(test_path_position("hi"), window, cx)
        })
        .await;
    picker.update(cx, |picker, _| {
        let matches = collect_search_matches(picker);
        assert_eq!(matches.history.len(), 0);
        assert_eq!(
            matches.search,
            vec![
                rel_path("tracked-root/hi").into(),
                rel_path("tracked-root/hiccup").into(),
                rel_path("tracked-root/happiness").into(),
            ],
            "Only non-ignored files should be found for the turned off ignored mode"
        );
    });

    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_abs_path(
                PathBuf::from(path!("/ancestor/tracked-root/heights/height_1")),
                OpenOptions {
                    visible: Some(OpenVisible::None),
                    ..OpenOptions::default()
                },
                window,
                cx,
            )
        })
        .await
        .unwrap();
    cx.run_until_parked();
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.active_pane().update(cx, |pane, cx| {
                pane.close_active_item(&CloseActiveItem::default(), window, cx)
            })
        })
        .await
        .unwrap();
    cx.run_until_parked();

    picker
        .update_in(cx, |picker, window, cx| {
            picker.delegate.include_ignored = None;
            picker
                .delegate
                .spawn_search(test_path_position("hi"), window, cx)
        })
        .await;
    picker.update(cx, |picker, _| {
        let matches = collect_search_matches(picker);
        assert_eq!(matches.history.len(), 0);
        assert_eq!(
            matches.search,
            vec![
                rel_path("ignored-root/hi").into(),
                rel_path("tracked-root/hi").into(),
                rel_path("ignored-root/hiccup").into(),
                rel_path("tracked-root/hiccup").into(),
                rel_path("ignored-root/height").into(),
                rel_path("ignored-root/happiness").into(),
                rel_path("tracked-root/happiness").into(),
            ],
            "Only for the worktree with the ignored root, all indexed ignored files are found in the auto ignored mode"
        );
    });

    picker
        .update_in(cx, |picker, window, cx| {
            picker.delegate.include_ignored = Some(true);
            picker
                .delegate
                .spawn_search(test_path_position("hi"), window, cx)
        })
        .await;
    picker.update(cx, |picker, _| {
        let matches = collect_search_matches(picker);
        assert_eq!(matches.history.len(), 0);
        assert_eq!(
            matches.search,
            vec![
                rel_path("ignored-root/hi").into(),
                rel_path("tracked-root/hi").into(),
                rel_path("ignored-root/hiccup").into(),
                rel_path("tracked-root/hiccup").into(),
                rel_path("ignored-root/height").into(),
                rel_path("tracked-root/height").into(),
                rel_path("tracked-root/heights/height_1").into(),
                rel_path("tracked-root/heights/height_2").into(),
                rel_path("ignored-root/happiness").into(),
                rel_path("tracked-root/happiness").into(),
            ],
            "All ignored files that were indexed are found in the turned on ignored mode"
        );
    });

    picker
        .update_in(cx, |picker, window, cx| {
            picker.delegate.include_ignored = Some(false);
            picker
                .delegate
                .spawn_search(test_path_position("hi"), window, cx)
        })
        .await;
    picker.update(cx, |picker, _| {
        let matches = collect_search_matches(picker);
        assert_eq!(matches.history.len(), 0);
        assert_eq!(
            matches.search,
            vec![
                rel_path("tracked-root/hi").into(),
                rel_path("tracked-root/hiccup").into(),
                rel_path("tracked-root/happiness").into(),
            ],
            "Only non-ignored files should be found for the turned off ignored mode"
        );
    });
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
        .update_in(cx, |picker, window, cx| {
            picker
                .delegate
                .spawn_search(test_path_position("thf"), window, cx)
        })
        .await;
    cx.read(|cx| {
        let picker = picker.read(cx);
        let delegate = &picker.delegate;
        let matches = collect_search_matches(picker).search_matches_only();
        assert_eq!(matches.len(), 1);

        let (file_name, file_name_positions, full_path, full_path_positions) =
            delegate.labels_for_path_match(&matches[0], PathStyle::local());
        assert_eq!(file_name, "the-file");
        assert_eq!(file_name_positions, &[0, 1, 4]);
        assert_eq!(full_path, "");
        assert_eq!(full_path_positions, &[0; 0]);
    });

    // Since the worktree root is a file, searching for its name followed by a slash does
    // not match anything.
    picker
        .update_in(cx, |picker, window, cx| {
            picker
                .delegate
                .spawn_search(test_path_position("thf/"), window, cx)
        })
        .await;
    picker.update(cx, |f, _| assert_eq!(f.delegate.matches.len(), 0));
}

#[gpui::test]
async fn test_create_file_for_multiple_worktrees(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/roota"),
            json!({ "the-parent-dira": { "filea": "" } }),
        )
        .await;

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/rootb"),
            json!({ "the-parent-dirb": { "fileb": "" } }),
        )
        .await;

    let project = Project::test(
        app_state.fs.clone(),
        [path!("/roota").as_ref(), path!("/rootb").as_ref()],
        cx,
    )
    .await;

    let (workspace, cx) = cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));
    let (_worktree_id1, worktree_id2) = cx.read(|cx| {
        let worktrees = workspace.read(cx).worktrees(cx).collect::<Vec<_>>();
        (
            WorktreeId::from_usize(worktrees[0].entity_id().as_u64() as usize),
            WorktreeId::from_usize(worktrees[1].entity_id().as_u64() as usize),
        )
    });

    let b_path = ProjectPath {
        worktree_id: worktree_id2,
        path: rel_path("the-parent-dirb/fileb").into(),
    };
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_path(b_path, None, true, window, cx)
        })
        .await
        .unwrap();

    let finder = open_file_picker(&workspace, cx);

    finder
        .update_in(cx, |f, window, cx| {
            f.delegate.spawn_search(
                test_path_position(path!("the-parent-dirb/filec")),
                window,
                cx,
            )
        })
        .await;
    cx.run_until_parked();
    finder.update_in(cx, |picker, window, cx| {
        assert_eq!(picker.delegate.matches.len(), 1);
        picker.delegate.confirm(false, window, cx)
    });
    cx.run_until_parked();
    cx.read(|cx| {
        let active_editor = workspace.read(cx).active_item_as::<Editor>(cx).unwrap();
        let project_path = active_editor.read(cx).project_path(cx);
        assert_eq!(
            project_path,
            Some(ProjectPath {
                worktree_id: worktree_id2,
                path: rel_path("the-parent-dirb/filec").into()
            })
        );
    });
}

#[gpui::test]
async fn test_create_file_no_focused_with_multiple_worktrees(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/roota"),
            json!({ "the-parent-dira": { "filea": "" } }),
        )
        .await;

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/rootb"),
            json!({ "the-parent-dirb": { "fileb": "" } }),
        )
        .await;

    let project = Project::test(
        app_state.fs.clone(),
        [path!("/roota").as_ref(), path!("/rootb").as_ref()],
        cx,
    )
    .await;

    let (workspace, cx) = cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));
    let (_worktree_id1, worktree_id2) = cx.read(|cx| {
        let worktrees = workspace.read(cx).worktrees(cx).collect::<Vec<_>>();
        (worktrees[0].read(cx).id(), worktrees[1].read(cx).id())
    });

    let finder = open_file_picker(&workspace, cx);

    finder
        .update_in(cx, |f, window, cx| {
            f.delegate
                .spawn_search(test_path_position(path!("rootb/filec")), window, cx)
        })
        .await;
    cx.run_until_parked();
    finder.update_in(cx, |picker, window, cx| {
        assert_eq!(picker.delegate.matches.len(), 1);
        picker.delegate.confirm(false, window, cx)
    });
    cx.run_until_parked();
    cx.read(|cx| {
        let active_editor = workspace.read(cx).active_item_as::<Editor>(cx).unwrap();
        let project_path = active_editor.read(cx).project_path(cx);
        assert_eq!(
            project_path,
            Some(ProjectPath {
                worktree_id: worktree_id2,
                path: rel_path("filec").into()
            })
        );
    });
}

#[gpui::test]
async fn test_path_distance_ordering(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "dir1": { "a.txt": "" },
                "dir2": {
                    "a.txt": "",
                    "b.txt": ""
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

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
        path: rel_path("dir2/b.txt").into(),
    };
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_path(b_path, None, true, window, cx)
        })
        .await
        .unwrap();
    let finder = open_file_picker(&workspace, cx);
    finder
        .update_in(cx, |f, window, cx| {
            f.delegate
                .spawn_search(test_path_position("a.txt"), window, cx)
        })
        .await;

    finder.update(cx, |picker, _| {
        let matches = collect_search_matches(picker).search_paths_only();
        assert_eq!(matches[0].as_ref(), rel_path("dir2/a.txt"));
        assert_eq!(matches[1].as_ref(), rel_path("dir1/a.txt"));
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
        .update_in(cx, |f, window, cx| {
            f.delegate
                .spawn_search(test_path_position("dir"), window, cx)
        })
        .await;
    cx.read(|cx| {
        let finder = picker.read(cx);
        assert_eq!(finder.delegate.matches.len(), 1);
        assert_match_at_position(finder, 0, "dir");
    });
}

#[gpui::test]
async fn test_query_history(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/src"),
            json!({
                "test": {
                    "first.rs": "// First Rust file",
                    "second.rs": "// Second Rust file",
                    "third.rs": "// Third Rust file",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/src").as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));
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
    workspace.update_in(cx, |_workspace, window, cx| window.focused(cx));

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
                path: rel_path("test/first.rs").into(),
            },
            PathBuf::from(path!("/src/test/first.rs"))
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
                    path: rel_path("test/second.rs").into(),
                },
                PathBuf::from(path!("/src/test/second.rs"))
            ),
            FoundPath::new(
                ProjectPath {
                    worktree_id,
                    path: rel_path("test/first.rs").into(),
                },
                PathBuf::from(path!("/src/test/first.rs"))
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
                    path: rel_path("test/third.rs").into(),
                },
                PathBuf::from(path!("/src/test/third.rs"))
            ),
            FoundPath::new(
                ProjectPath {
                    worktree_id,
                    path: rel_path("test/second.rs").into(),
                },
                PathBuf::from(path!("/src/test/second.rs"))
            ),
            FoundPath::new(
                ProjectPath {
                    worktree_id,
                    path: rel_path("test/first.rs").into(),
                },
                PathBuf::from(path!("/src/test/first.rs"))
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
                    path: rel_path("test/second.rs").into(),
                },
                PathBuf::from(path!("/src/test/second.rs"))
            ),
            FoundPath::new(
                ProjectPath {
                    worktree_id,
                    path: rel_path("test/third.rs").into(),
                },
                PathBuf::from(path!("/src/test/third.rs"))
            ),
            FoundPath::new(
                ProjectPath {
                    worktree_id,
                    path: rel_path("test/first.rs").into(),
                },
                PathBuf::from(path!("/src/test/first.rs"))
            ),
        ],
        "Should show 1st, 2nd and 3rd opened items in the history when opening the 3rd item again. \
    2nd item, as the last opened, 3rd item should go next as it was opened right before."
    );
}

#[gpui::test]
async fn test_history_match_positions(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/src"),
            json!({
                "test": {
                    "first.rs": "// First Rust file",
                    "second.rs": "// Second Rust file",
                    "third.rs": "// Third Rust file",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/src").as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

    workspace.update_in(cx, |_workspace, window, cx| window.focused(cx));

    open_close_queried_buffer("efir", 1, "first.rs", &workspace, cx).await;
    let history = open_close_queried_buffer("second", 1, "second.rs", &workspace, cx).await;
    assert_eq!(history.len(), 1);

    let picker = open_file_picker(&workspace, cx);
    cx.simulate_input("fir");
    picker.update_in(cx, |finder, window, cx| {
        let matches = &finder.delegate.matches.matches;
        assert_matches!(
            matches.as_slice(),
            [Match::History { .. }, Match::CreateNew { .. }]
        );
        assert_eq!(
            matches[0].panel_match().unwrap().0.path.as_ref(),
            rel_path("test/first.rs")
        );
        assert_eq!(matches[0].panel_match().unwrap().0.positions, &[5, 6, 7]);

        let (file_label, path_label) =
            finder
                .delegate
                .labels_for_match(&finder.delegate.matches.matches[0], window, cx);
        assert_eq!(file_label.text(), "first.rs");
        assert_eq!(file_label.highlight_indices(), &[0, 1, 2]);
        assert_eq!(
            path_label.text(),
            format!("test{}", PathStyle::local().separator())
        );
        assert_eq!(path_label.highlight_indices(), &[] as &[usize]);
    });
}

#[gpui::test]
async fn test_external_files_history(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/src"),
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
            path!("/external-src"),
            json!({
                "test": {
                    "third.rs": "// Third Rust file",
                    "fourth.rs": "// Fourth Rust file",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/src").as_ref()], cx).await;
    cx.update(|cx| {
        project.update(cx, |project, cx| {
            project.find_or_create_worktree(path!("/external-src"), false, cx)
        })
    })
    .detach();
    cx.background_executor.run_until_parked();

    let (workspace, cx) = cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));
    let worktree_id = cx.read(|cx| {
        let worktrees = workspace.read(cx).worktrees(cx).collect::<Vec<_>>();
        assert_eq!(worktrees.len(), 1,);

        WorktreeId::from_usize(worktrees[0].entity_id().as_u64() as usize)
    });
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_abs_path(
                PathBuf::from(path!("/external-src/test/third.rs")),
                OpenOptions {
                    visible: Some(OpenVisible::None),
                    ..Default::default()
                },
                window,
                cx,
            )
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
    cx.dispatch_action(workspace::CloseActiveItem {
        save_intent: None,
        close_pinned: false,
    });

    let initial_history_items =
        open_close_queried_buffer("sec", 1, "second.rs", &workspace, cx).await;
    assert_eq!(
        initial_history_items,
        vec![FoundPath::new(
            ProjectPath {
                worktree_id: external_worktree_id,
                path: rel_path("").into(),
            },
            PathBuf::from(path!("/external-src/test/third.rs"))
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
                    path: rel_path("test/second.rs").into(),
                },
                PathBuf::from(path!("/src/test/second.rs"))
            ),
            FoundPath::new(
                ProjectPath {
                    worktree_id: external_worktree_id,
                    path: rel_path("").into(),
                },
                PathBuf::from(path!("/external-src/test/third.rs"))
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
            path!("/src"),
            json!({
                "test": {
                    "first.rs": "// First Rust file",
                    "second.rs": "// Second Rust file",
                    "third.rs": "// Third Rust file",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/src").as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

    // generate some history to select from
    open_close_queried_buffer("fir", 1, "first.rs", &workspace, cx).await;
    open_close_queried_buffer("sec", 1, "second.rs", &workspace, cx).await;
    open_close_queried_buffer("thi", 1, "third.rs", &workspace, cx).await;
    let current_history = open_close_queried_buffer("sec", 1, "second.rs", &workspace, cx).await;

    for expected_selected_index in 0..current_history.len() {
        cx.dispatch_action(ToggleFileFinder::default());
        let picker = active_file_picker(&workspace, cx);
        let selected_index = picker.update(cx, |picker, _| picker.delegate.selected_index());
        assert_eq!(
            selected_index, expected_selected_index,
            "Should select the next item in the history"
        );
    }

    cx.dispatch_action(ToggleFileFinder::default());
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
            path!("/src"),
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

    let project = Project::test(app_state.fs.clone(), [path!("/src").as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));
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
        .update_in(cx, |finder, window, cx| {
            finder
                .delegate
                .update_matches(first_query.to_string(), window, cx)
        })
        .await;
    finder.update(cx, |picker, _| {
            let matches = collect_search_matches(picker);
            assert_eq!(matches.history.len(), 1, "Only one history item contains {first_query}, it should be present and others should be filtered out");
            let history_match = matches.history_found_paths.first().expect("Should have path matches for history items after querying");
            assert_eq!(history_match, &FoundPath::new(
                ProjectPath {
                    worktree_id,
                    path: rel_path("test/first.rs").into(),
                },
                PathBuf::from(path!("/src/test/first.rs")),
            ));
            assert_eq!(matches.search.len(), 1, "Only one non-history item contains {first_query}, it should be present");
            assert_eq!(matches.search.first().unwrap().as_ref(), rel_path("test/fourth.rs"));
        });

    let second_query = "fsdasdsa";
    let finder = active_file_picker(&workspace, cx);
    finder
        .update_in(cx, |finder, window, cx| {
            finder
                .delegate
                .update_matches(second_query.to_string(), window, cx)
        })
        .await;
    finder.update(cx, |picker, _| {
        assert!(
            collect_search_matches(picker)
                .search_paths_only()
                .is_empty(),
            "No search entries should match {second_query}"
        );
    });

    let first_query_again = first_query;

    let finder = active_file_picker(&workspace, cx);
    finder
        .update_in(cx, |finder, window, cx| {
            finder
                .delegate
                .update_matches(first_query_again.to_string(), window, cx)
        })
        .await;
    finder.update(cx, |picker, _| {
            let matches = collect_search_matches(picker);
            assert_eq!(matches.history.len(), 1, "Only one history item contains {first_query_again}, it should be present and others should be filtered out, even after non-matching query");
            let history_match = matches.history_found_paths.first().expect("Should have path matches for history items after querying");
            assert_eq!(history_match, &FoundPath::new(
                ProjectPath {
                    worktree_id,
                    path: rel_path("test/first.rs").into(),
                },
                PathBuf::from(path!("/src/test/first.rs"))
            ));
            assert_eq!(matches.search.len(), 1, "Only one non-history item contains {first_query_again}, it should be present, even after non-matching query");
            assert_eq!(matches.search.first().unwrap().as_ref(), rel_path("test/fourth.rs"));
        });
}

#[gpui::test]
async fn test_search_sorts_history_items(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "test": {
                    "1_qw": "// First file that matches the query",
                    "2_second": "// Second file",
                    "3_third": "// Third file",
                    "4_fourth": "// Fourth file",
                    "5_qwqwqw": "// A file with 3 more matches than the first one",
                    "6_qwqwqw": "// Same query matches as above, but closer to the end of the list due to the name",
                    "7_qwqwqw": "// One more, same amount of query matches as above",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));
    // generate some history to select from
    open_close_queried_buffer("1", 1, "1_qw", &workspace, cx).await;
    open_close_queried_buffer("2", 1, "2_second", &workspace, cx).await;
    open_close_queried_buffer("3", 1, "3_third", &workspace, cx).await;
    open_close_queried_buffer("2", 1, "2_second", &workspace, cx).await;
    open_close_queried_buffer("6", 1, "6_qwqwqw", &workspace, cx).await;

    let finder = open_file_picker(&workspace, cx);
    let query = "qw";
    finder
        .update_in(cx, |finder, window, cx| {
            finder
                .delegate
                .update_matches(query.to_string(), window, cx)
        })
        .await;
    finder.update(cx, |finder, _| {
        let search_matches = collect_search_matches(finder);
        assert_eq!(
            search_matches.history,
            vec![
                rel_path("test/1_qw").into(),
                rel_path("test/6_qwqwqw").into()
            ],
        );
        assert_eq!(
            search_matches.search,
            vec![
                rel_path("test/5_qwqwqw").into(),
                rel_path("test/7_qwqwqw").into()
            ],
        );
    });
}

#[gpui::test]
async fn test_select_current_open_file_when_no_history(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "test": {
                    "1_qw": "",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));
    // Open new buffer
    open_queried_buffer("1", 1, "1_qw", &workspace, cx).await;

    let picker = open_file_picker(&workspace, cx);
    picker.update(cx, |finder, _| {
        assert_match_selection(finder, 0, "1_qw");
    });
}

#[gpui::test]
async fn test_keep_opened_file_on_top_of_search_results_and_select_next_one(
    cx: &mut TestAppContext,
) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/src"),
            json!({
                "test": {
                    "bar.rs": "// Bar file",
                    "lib.rs": "// Lib file",
                    "maaa.rs": "// Maaaaaaa",
                    "main.rs": "// Main file",
                    "moo.rs": "// Moooooo",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/src").as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

    open_close_queried_buffer("bar", 1, "bar.rs", &workspace, cx).await;
    open_close_queried_buffer("lib", 1, "lib.rs", &workspace, cx).await;
    open_queried_buffer("main", 1, "main.rs", &workspace, cx).await;

    // main.rs is on top, previously used is selected
    let picker = open_file_picker(&workspace, cx);
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 3);
        assert_match_selection(finder, 0, "main.rs");
        assert_match_at_position(finder, 1, "lib.rs");
        assert_match_at_position(finder, 2, "bar.rs");
    });

    // all files match, main.rs is still on top, but the second item is selected
    picker
        .update_in(cx, |finder, window, cx| {
            finder
                .delegate
                .update_matches(".rs".to_string(), window, cx)
        })
        .await;
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 6);
        assert_match_at_position(finder, 0, "main.rs");
        assert_match_selection(finder, 1, "bar.rs");
        assert_match_at_position(finder, 2, "lib.rs");
        assert_match_at_position(finder, 3, "moo.rs");
        assert_match_at_position(finder, 4, "maaa.rs");
        assert_match_at_position(finder, 5, ".rs");
    });

    // main.rs is not among matches, select top item
    picker
        .update_in(cx, |finder, window, cx| {
            finder.delegate.update_matches("b".to_string(), window, cx)
        })
        .await;
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 3);
        assert_match_at_position(finder, 0, "bar.rs");
        assert_match_at_position(finder, 1, "lib.rs");
        assert_match_at_position(finder, 2, "b");
    });

    // main.rs is back, put it on top and select next item
    picker
        .update_in(cx, |finder, window, cx| {
            finder.delegate.update_matches("m".to_string(), window, cx)
        })
        .await;
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 4);
        assert_match_at_position(finder, 0, "main.rs");
        assert_match_selection(finder, 1, "moo.rs");
        assert_match_at_position(finder, 2, "maaa.rs");
        assert_match_at_position(finder, 3, "m");
    });

    // get back to the initial state
    picker
        .update_in(cx, |finder, window, cx| {
            finder.delegate.update_matches("".to_string(), window, cx)
        })
        .await;
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 3);
        assert_match_selection(finder, 0, "main.rs");
        assert_match_at_position(finder, 1, "lib.rs");
        assert_match_at_position(finder, 2, "bar.rs");
    });
}

#[gpui::test]
async fn test_setting_auto_select_first_and_select_active_file(cx: &mut TestAppContext) {
    let app_state = init_test(cx);

    cx.update(|cx| {
        let settings = *FileFinderSettings::get_global(cx);

        FileFinderSettings::override_global(
            FileFinderSettings {
                skip_focus_for_active_in_search: false,
                ..settings
            },
            cx,
        );
    });

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/src"),
            json!({
                "test": {
                    "bar.rs": "// Bar file",
                    "lib.rs": "// Lib file",
                    "maaa.rs": "// Maaaaaaa",
                    "main.rs": "// Main file",
                    "moo.rs": "// Moooooo",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/src").as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

    open_close_queried_buffer("bar", 1, "bar.rs", &workspace, cx).await;
    open_close_queried_buffer("lib", 1, "lib.rs", &workspace, cx).await;
    open_queried_buffer("main", 1, "main.rs", &workspace, cx).await;

    // main.rs is on top, previously used is selected
    let picker = open_file_picker(&workspace, cx);
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 3);
        assert_match_selection(finder, 0, "main.rs");
        assert_match_at_position(finder, 1, "lib.rs");
        assert_match_at_position(finder, 2, "bar.rs");
    });

    // all files match, main.rs is on top, and is selected
    picker
        .update_in(cx, |finder, window, cx| {
            finder
                .delegate
                .update_matches(".rs".to_string(), window, cx)
        })
        .await;
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 6);
        assert_match_selection(finder, 0, "main.rs");
        assert_match_at_position(finder, 1, "bar.rs");
        assert_match_at_position(finder, 2, "lib.rs");
        assert_match_at_position(finder, 3, "moo.rs");
        assert_match_at_position(finder, 4, "maaa.rs");
        assert_match_at_position(finder, 5, ".rs");
    });
}

#[gpui::test]
async fn test_non_separate_history_items(cx: &mut TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/src"),
            json!({
                "test": {
                    "bar.rs": "// Bar file",
                    "lib.rs": "// Lib file",
                    "maaa.rs": "// Maaaaaaa",
                    "main.rs": "// Main file",
                    "moo.rs": "// Moooooo",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/src").as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

    open_close_queried_buffer("bar", 1, "bar.rs", &workspace, cx).await;
    open_close_queried_buffer("lib", 1, "lib.rs", &workspace, cx).await;
    open_queried_buffer("main", 1, "main.rs", &workspace, cx).await;

    cx.dispatch_action(ToggleFileFinder::default());
    let picker = active_file_picker(&workspace, cx);
    // main.rs is on top, previously used is selected
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 3);
        assert_match_selection(finder, 0, "main.rs");
        assert_match_at_position(finder, 1, "lib.rs");
        assert_match_at_position(finder, 2, "bar.rs");
    });

    // all files match, main.rs is still on top, but the second item is selected
    picker
        .update_in(cx, |finder, window, cx| {
            finder
                .delegate
                .update_matches(".rs".to_string(), window, cx)
        })
        .await;
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 6);
        assert_match_at_position(finder, 0, "main.rs");
        assert_match_selection(finder, 1, "moo.rs");
        assert_match_at_position(finder, 2, "bar.rs");
        assert_match_at_position(finder, 3, "lib.rs");
        assert_match_at_position(finder, 4, "maaa.rs");
        assert_match_at_position(finder, 5, ".rs");
    });

    // main.rs is not among matches, select top item
    picker
        .update_in(cx, |finder, window, cx| {
            finder.delegate.update_matches("b".to_string(), window, cx)
        })
        .await;
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 3);
        assert_match_at_position(finder, 0, "bar.rs");
        assert_match_at_position(finder, 1, "lib.rs");
        assert_match_at_position(finder, 2, "b");
    });

    // main.rs is back, put it on top and select next item
    picker
        .update_in(cx, |finder, window, cx| {
            finder.delegate.update_matches("m".to_string(), window, cx)
        })
        .await;
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 4);
        assert_match_at_position(finder, 0, "main.rs");
        assert_match_selection(finder, 1, "moo.rs");
        assert_match_at_position(finder, 2, "maaa.rs");
        assert_match_at_position(finder, 3, "m");
    });

    // get back to the initial state
    picker
        .update_in(cx, |finder, window, cx| {
            finder.delegate.update_matches("".to_string(), window, cx)
        })
        .await;
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 3);
        assert_match_selection(finder, 0, "main.rs");
        assert_match_at_position(finder, 1, "lib.rs");
        assert_match_at_position(finder, 2, "bar.rs");
    });
}

#[gpui::test]
async fn test_history_items_shown_in_order_of_open(cx: &mut TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/test"),
            json!({
                "test": {
                    "1.txt": "// One",
                    "2.txt": "// Two",
                    "3.txt": "// Three",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/test").as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

    open_queried_buffer("1", 1, "1.txt", &workspace, cx).await;
    open_queried_buffer("2", 1, "2.txt", &workspace, cx).await;
    open_queried_buffer("3", 1, "3.txt", &workspace, cx).await;

    let picker = open_file_picker(&workspace, cx);
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 3);
        assert_match_selection(finder, 0, "3.txt");
        assert_match_at_position(finder, 1, "2.txt");
        assert_match_at_position(finder, 2, "1.txt");
    });

    cx.dispatch_action(SelectNext);
    cx.dispatch_action(Confirm); // Open 2.txt

    let picker = open_file_picker(&workspace, cx);
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 3);
        assert_match_selection(finder, 0, "2.txt");
        assert_match_at_position(finder, 1, "3.txt");
        assert_match_at_position(finder, 2, "1.txt");
    });

    cx.dispatch_action(SelectNext);
    cx.dispatch_action(SelectNext);
    cx.dispatch_action(Confirm); // Open 1.txt

    let picker = open_file_picker(&workspace, cx);
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 3);
        assert_match_selection(finder, 0, "1.txt");
        assert_match_at_position(finder, 1, "2.txt");
        assert_match_at_position(finder, 2, "3.txt");
    });
}

#[gpui::test]
async fn test_selected_history_item_stays_selected_on_worktree_updated(cx: &mut TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/test"),
            json!({
                "test": {
                    "1.txt": "// One",
                    "2.txt": "// Two",
                    "3.txt": "// Three",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/test").as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

    open_close_queried_buffer("1", 1, "1.txt", &workspace, cx).await;
    open_close_queried_buffer("2", 1, "2.txt", &workspace, cx).await;
    open_close_queried_buffer("3", 1, "3.txt", &workspace, cx).await;

    let picker = open_file_picker(&workspace, cx);
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 3);
        assert_match_selection(finder, 0, "3.txt");
        assert_match_at_position(finder, 1, "2.txt");
        assert_match_at_position(finder, 2, "1.txt");
    });

    cx.dispatch_action(SelectNext);

    // Add more files to the worktree to trigger update matches
    for i in 0..5 {
        let filename = if cfg!(windows) {
            format!("C:/test/{}.txt", 4 + i)
        } else {
            format!("/test/{}.txt", 4 + i)
        };
        app_state
            .fs
            .create_file(Path::new(&filename), Default::default())
            .await
            .expect("unable to create file");
    }

    cx.executor().advance_clock(FS_WATCH_LATENCY);

    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 3);
        assert_match_at_position(finder, 0, "3.txt");
        assert_match_selection(finder, 1, "2.txt");
        assert_match_at_position(finder, 2, "1.txt");
    });
}

#[gpui::test]
async fn test_history_items_vs_very_good_external_match(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/src"),
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

    let project = Project::test(app_state.fs.clone(), [path!("/src").as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));
    // generate some history to select from
    open_close_queried_buffer("fir", 1, "first.rs", &workspace, cx).await;
    open_close_queried_buffer("sec", 1, "second.rs", &workspace, cx).await;
    open_close_queried_buffer("thi", 1, "third.rs", &workspace, cx).await;
    open_close_queried_buffer("sec", 1, "second.rs", &workspace, cx).await;

    let finder = open_file_picker(&workspace, cx);
    let query = "collab_ui";
    cx.simulate_input(query);
    finder.update(cx, |picker, _| {
            let search_entries = collect_search_matches(picker).search_paths_only();
            assert_eq!(
                search_entries,
                vec![
                    rel_path("collab_ui/collab_ui.rs").into(),
                    rel_path("collab_ui/first.rs").into(),
                    rel_path("collab_ui/third.rs").into(),
                    rel_path("collab_ui/second.rs").into(),
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
            path!("/src"),
            json!({
                "test": {
                    "first.rs": "// First Rust file",
                    "nonexistent.rs": "// Second Rust file",
                    "third.rs": "// Third Rust file",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/src").as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx)); // generate some history to select from
    open_close_queried_buffer("fir", 1, "first.rs", &workspace, cx).await;
    open_close_queried_buffer("non", 1, "nonexistent.rs", &workspace, cx).await;
    open_close_queried_buffer("thi", 1, "third.rs", &workspace, cx).await;
    open_close_queried_buffer("fir", 1, "first.rs", &workspace, cx).await;
    app_state
        .fs
        .remove_file(
            Path::new(path!("/src/test/nonexistent.rs")),
            RemoveOptions::default(),
        )
        .await
        .unwrap();
    cx.run_until_parked();

    let picker = open_file_picker(&workspace, cx);
    cx.simulate_input("rs");

    picker.update(cx, |picker, _| {
        assert_eq!(
            collect_search_matches(picker).history,
            vec![
                rel_path("test/first.rs").into(),
                rel_path("test/third.rs").into(),
            ],
            "Should have all opened files in the history, except the ones that do not exist on disk"
        );
    });
}

#[gpui::test]
async fn test_search_results_refreshed_on_worktree_updates(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            "/src",
            json!({
                "lib.rs": "// Lib file",
                "main.rs": "// Bar file",
                "read.me": "// Readme file",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), ["/src".as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    // Initial state
    let picker = open_file_picker(&workspace, cx);
    cx.simulate_input("rs");
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 3);
        assert_match_at_position(finder, 0, "lib.rs");
        assert_match_at_position(finder, 1, "main.rs");
        assert_match_at_position(finder, 2, "rs");
    });

    // Delete main.rs
    app_state
        .fs
        .remove_file("/src/main.rs".as_ref(), Default::default())
        .await
        .expect("unable to remove file");
    cx.executor().advance_clock(FS_WATCH_LATENCY);

    // main.rs is in not among search results anymore
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 2);
        assert_match_at_position(finder, 0, "lib.rs");
        assert_match_at_position(finder, 1, "rs");
    });

    // Create util.rs
    app_state
        .fs
        .create_file("/src/util.rs".as_ref(), Default::default())
        .await
        .expect("unable to create file");
    cx.executor().advance_clock(FS_WATCH_LATENCY);

    // util.rs is among search results
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 3);
        assert_match_at_position(finder, 0, "lib.rs");
        assert_match_at_position(finder, 1, "util.rs");
        assert_match_at_position(finder, 2, "rs");
    });
}

#[gpui::test]
async fn test_search_results_refreshed_on_adding_and_removing_worktrees(
    cx: &mut gpui::TestAppContext,
) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            "/test",
            json!({
                "project_1": {
                    "bar.rs": "// Bar file",
                    "lib.rs": "// Lib file",
                },
                "project_2": {
                    "Cargo.toml": "// Cargo file",
                    "main.rs": "// Main file",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), ["/test/project_1".as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let worktree_1_id = project.update(cx, |project, cx| {
        let worktree = project.worktrees(cx).last().expect("worktree not found");
        worktree.read(cx).id()
    });

    // Initial state
    let picker = open_file_picker(&workspace, cx);
    cx.simulate_input("rs");
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 3);
        assert_match_at_position(finder, 0, "bar.rs");
        assert_match_at_position(finder, 1, "lib.rs");
        assert_match_at_position(finder, 2, "rs");
    });

    // Add new worktree
    project
        .update(cx, |project, cx| {
            project
                .find_or_create_worktree("/test/project_2", true, cx)
                .into_future()
        })
        .await
        .expect("unable to create workdir");
    cx.executor().advance_clock(FS_WATCH_LATENCY);

    // main.rs is among search results
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 4);
        assert_match_at_position(finder, 0, "bar.rs");
        assert_match_at_position(finder, 1, "lib.rs");
        assert_match_at_position(finder, 2, "main.rs");
        assert_match_at_position(finder, 3, "rs");
    });

    // Remove the first worktree
    project.update(cx, |project, cx| {
        project.remove_worktree(worktree_1_id, cx);
    });
    cx.executor().advance_clock(FS_WATCH_LATENCY);

    // Files from the first worktree are not in the search results anymore
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 2);
        assert_match_at_position(finder, 0, "main.rs");
        assert_match_at_position(finder, 1, "rs");
    });
}

#[gpui::test]
async fn test_selected_match_stays_selected_after_matches_refreshed(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state.fs.as_fake().insert_tree("/src", json!({})).await;

    app_state
        .fs
        .create_dir("/src/even".as_ref())
        .await
        .expect("unable to create dir");

    let initial_files_num = 5;
    for i in 0..initial_files_num {
        let filename = format!("/src/even/file_{}.txt", 10 + i);
        app_state
            .fs
            .create_file(Path::new(&filename), Default::default())
            .await
            .expect("unable to create file");
    }

    let project = Project::test(app_state.fs.clone(), ["/src".as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    // Initial state
    let picker = open_file_picker(&workspace, cx);
    cx.simulate_input("file");
    let selected_index = 3;
    // Checking only the filename, not the whole path
    let selected_file = format!("file_{}.txt", 10 + selected_index);
    // Select even/file_13.txt
    for _ in 0..selected_index {
        cx.dispatch_action(SelectNext);
    }

    picker.update(cx, |finder, _| {
        assert_match_selection(finder, selected_index, &selected_file)
    });

    // Add more matches to the search results
    let files_to_add = 10;
    for i in 0..files_to_add {
        let filename = format!("/src/file_{}.txt", 20 + i);
        app_state
            .fs
            .create_file(Path::new(&filename), Default::default())
            .await
            .expect("unable to create file");
    }
    cx.executor().advance_clock(FS_WATCH_LATENCY);

    // file_13.txt is still selected
    picker.update(cx, |finder, _| {
        let expected_selected_index = selected_index + files_to_add;
        assert_match_selection(finder, expected_selected_index, &selected_file);
    });
}

#[gpui::test]
async fn test_first_match_selected_if_previous_one_is_not_in_the_match_list(
    cx: &mut gpui::TestAppContext,
) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            "/src",
            json!({
                "file_1.txt": "// file_1",
                "file_2.txt": "// file_2",
                "file_3.txt": "// file_3",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), ["/src".as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    // Initial state
    let picker = open_file_picker(&workspace, cx);
    cx.simulate_input("file");
    // Select even/file_2.txt
    cx.dispatch_action(SelectNext);

    // Remove the selected entry
    app_state
        .fs
        .remove_file("/src/file_2.txt".as_ref(), Default::default())
        .await
        .expect("unable to remove file");
    cx.executor().advance_clock(FS_WATCH_LATENCY);

    // file_1.txt is now selected
    picker.update(cx, |finder, _| {
        assert_match_selection(finder, 0, "file_1.txt");
    });
}

#[gpui::test]
async fn test_keeps_file_finder_open_after_modifier_keys_release(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/test"),
            json!({
                "1.txt": "// One",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/test").as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

    open_queried_buffer("1", 1, "1.txt", &workspace, cx).await;

    cx.simulate_modifiers_change(Modifiers::secondary_key());
    open_file_picker(&workspace, cx);

    cx.simulate_modifiers_change(Modifiers::none());
    active_file_picker(&workspace, cx);
}

#[gpui::test]
async fn test_opens_file_on_modifier_keys_release(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/test"),
            json!({
                "1.txt": "// One",
                "2.txt": "// Two",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/test").as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

    open_queried_buffer("1", 1, "1.txt", &workspace, cx).await;
    open_queried_buffer("2", 1, "2.txt", &workspace, cx).await;

    cx.simulate_modifiers_change(Modifiers::secondary_key());
    let picker = open_file_picker(&workspace, cx);
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 2);
        assert_match_selection(finder, 0, "2.txt");
        assert_match_at_position(finder, 1, "1.txt");
    });

    cx.dispatch_action(SelectNext);
    cx.simulate_modifiers_change(Modifiers::none());
    cx.read(|cx| {
        let active_editor = workspace.read(cx).active_item_as::<Editor>(cx).unwrap();
        assert_eq!(active_editor.read(cx).title(cx), "1.txt");
    });
}

#[gpui::test]
async fn test_switches_between_release_norelease_modes_on_forward_nav(
    cx: &mut gpui::TestAppContext,
) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/test"),
            json!({
                "1.txt": "// One",
                "2.txt": "// Two",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/test").as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

    open_queried_buffer("1", 1, "1.txt", &workspace, cx).await;
    open_queried_buffer("2", 1, "2.txt", &workspace, cx).await;

    // Open with a shortcut
    cx.simulate_modifiers_change(Modifiers::secondary_key());
    let picker = open_file_picker(&workspace, cx);
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 2);
        assert_match_selection(finder, 0, "2.txt");
        assert_match_at_position(finder, 1, "1.txt");
    });

    // Switch to navigating with other shortcuts
    // Don't open file on modifiers release
    cx.simulate_modifiers_change(Modifiers::control());
    cx.dispatch_action(SelectNext);
    cx.simulate_modifiers_change(Modifiers::none());
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 2);
        assert_match_at_position(finder, 0, "2.txt");
        assert_match_selection(finder, 1, "1.txt");
    });

    // Back to navigation with initial shortcut
    // Open file on modifiers release
    cx.simulate_modifiers_change(Modifiers::secondary_key());
    cx.dispatch_action(ToggleFileFinder::default());
    cx.simulate_modifiers_change(Modifiers::none());
    cx.read(|cx| {
        let active_editor = workspace.read(cx).active_item_as::<Editor>(cx).unwrap();
        assert_eq!(active_editor.read(cx).title(cx), "2.txt");
    });
}

#[gpui::test]
async fn test_switches_between_release_norelease_modes_on_backward_nav(
    cx: &mut gpui::TestAppContext,
) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/test"),
            json!({
                "1.txt": "// One",
                "2.txt": "// Two",
                "3.txt": "// Three"
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/test").as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

    open_queried_buffer("1", 1, "1.txt", &workspace, cx).await;
    open_queried_buffer("2", 1, "2.txt", &workspace, cx).await;
    open_queried_buffer("3", 1, "3.txt", &workspace, cx).await;

    // Open with a shortcut
    cx.simulate_modifiers_change(Modifiers::secondary_key());
    let picker = open_file_picker(&workspace, cx);
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 3);
        assert_match_selection(finder, 0, "3.txt");
        assert_match_at_position(finder, 1, "2.txt");
        assert_match_at_position(finder, 2, "1.txt");
    });

    // Switch to navigating with other shortcuts
    // Don't open file on modifiers release
    cx.simulate_modifiers_change(Modifiers::control());
    cx.dispatch_action(menu::SelectPrevious);
    cx.simulate_modifiers_change(Modifiers::none());
    picker.update(cx, |finder, _| {
        assert_eq!(finder.delegate.matches.len(), 3);
        assert_match_at_position(finder, 0, "3.txt");
        assert_match_at_position(finder, 1, "2.txt");
        assert_match_selection(finder, 2, "1.txt");
    });

    // Back to navigation with initial shortcut
    // Open file on modifiers release
    cx.simulate_modifiers_change(Modifiers::secondary_key());
    cx.dispatch_action(SelectPrevious); // <-- File Finder's SelectPrevious, not menu's
    cx.simulate_modifiers_change(Modifiers::none());
    cx.read(|cx| {
        let active_editor = workspace.read(cx).active_item_as::<Editor>(cx).unwrap();
        assert_eq!(active_editor.read(cx).title(cx), "3.txt");
    });
}

#[gpui::test]
async fn test_extending_modifiers_does_not_confirm_selection(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/test"),
            json!({
                "1.txt": "// One",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/test").as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

    open_queried_buffer("1", 1, "1.txt", &workspace, cx).await;

    cx.simulate_modifiers_change(Modifiers::secondary_key());
    open_file_picker(&workspace, cx);

    cx.simulate_modifiers_change(Modifiers::command_shift());
    active_file_picker(&workspace, cx);
}

#[gpui::test]
async fn test_repeat_toggle_action(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            "/test",
            json!({
                "00.txt": "",
                "01.txt": "",
                "02.txt": "",
                "03.txt": "",
                "04.txt": "",
                "05.txt": "",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), ["/test".as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));

    cx.dispatch_action(ToggleFileFinder::default());
    let picker = active_file_picker(&workspace, cx);

    picker.update_in(cx, |picker, window, cx| {
        picker.update_matches(".txt".to_string(), window, cx)
    });

    cx.run_until_parked();

    picker.update(cx, |picker, _| {
        assert_eq!(picker.delegate.matches.len(), 7);
        assert_eq!(picker.delegate.selected_index, 0);
    });

    // When toggling repeatedly, the picker scrolls to reveal the selected item.
    cx.dispatch_action(ToggleFileFinder::default());
    cx.dispatch_action(ToggleFileFinder::default());
    cx.dispatch_action(ToggleFileFinder::default());

    cx.run_until_parked();

    picker.update(cx, |picker, _| {
        assert_eq!(picker.delegate.matches.len(), 7);
        assert_eq!(picker.delegate.selected_index, 3);
    });
}

async fn open_close_queried_buffer(
    input: &str,
    expected_matches: usize,
    expected_editor_title: &str,
    workspace: &Entity<Workspace>,
    cx: &mut gpui::VisualTestContext,
) -> Vec<FoundPath> {
    let history_items = open_queried_buffer(
        input,
        expected_matches,
        expected_editor_title,
        workspace,
        cx,
    )
    .await;

    cx.dispatch_action(workspace::CloseActiveItem {
        save_intent: None,
        close_pinned: false,
    });

    history_items
}

async fn open_queried_buffer(
    input: &str,
    expected_matches: usize,
    expected_editor_title: &str,
    workspace: &Entity<Workspace>,
    cx: &mut gpui::VisualTestContext,
) -> Vec<FoundPath> {
    let picker = open_file_picker(workspace, cx);
    cx.simulate_input(input);

    let history_items = picker.update(cx, |finder, _| {
        assert_eq!(
            finder.delegate.matches.len(),
            expected_matches + 1, // +1 from CreateNew option
            "Unexpected number of matches found for query `{input}`, matches: {:?}",
            finder.delegate.matches
        );
        finder.delegate.history_items.clone()
    });

    cx.dispatch_action(Confirm);

    cx.read(|cx| {
        let active_editor = workspace.read(cx).active_item_as::<Editor>(cx).unwrap();
        let active_editor_title = active_editor.read(cx).title(cx);
        assert_eq!(
            expected_editor_title, active_editor_title,
            "Unexpected editor title for query `{input}`"
        );
    });

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

fn test_path_position(test_str: &str) -> FileSearchQuery {
    let path_position = PathWithPosition::parse_str(test_str);

    FileSearchQuery {
        raw_query: test_str.to_owned(),
        file_query_end: if path_position.path.to_str().unwrap() == test_str {
            None
        } else {
            Some(path_position.path.to_str().unwrap().len())
        },
        path_position,
    }
}

fn build_find_picker(
    project: Entity<Project>,
    cx: &mut TestAppContext,
) -> (
    Entity<Picker<FileFinderDelegate>>,
    Entity<Workspace>,
    &mut VisualTestContext,
) {
    let (workspace, cx) = cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));
    let picker = open_file_picker(&workspace, cx);
    (picker, workspace, cx)
}

#[track_caller]
fn open_file_picker(
    workspace: &Entity<Workspace>,
    cx: &mut VisualTestContext,
) -> Entity<Picker<FileFinderDelegate>> {
    cx.dispatch_action(ToggleFileFinder {
        separate_history: true,
    });
    active_file_picker(workspace, cx)
}

#[track_caller]
fn active_file_picker(
    workspace: &Entity<Workspace>,
    cx: &mut VisualTestContext,
) -> Entity<Picker<FileFinderDelegate>> {
    workspace.update(cx, |workspace, cx| {
        workspace
            .active_modal::<FileFinder>(cx)
            .expect("file finder is not open")
            .read(cx)
            .picker
            .clone()
    })
}

#[derive(Debug, Default)]
struct SearchEntries {
    history: Vec<Arc<RelPath>>,
    history_found_paths: Vec<FoundPath>,
    search: Vec<Arc<RelPath>>,
    search_matches: Vec<PathMatch>,
}

impl SearchEntries {
    #[track_caller]
    fn search_paths_only(self) -> Vec<Arc<RelPath>> {
        assert!(
            self.history.is_empty(),
            "Should have no history matches, but got: {:?}",
            self.history
        );
        self.search
    }

    #[track_caller]
    fn search_matches_only(self) -> Vec<PathMatch> {
        assert!(
            self.history.is_empty(),
            "Should have no history matches, but got: {:?}",
            self.history
        );
        self.search_matches
    }
}

fn collect_search_matches(picker: &Picker<FileFinderDelegate>) -> SearchEntries {
    let mut search_entries = SearchEntries::default();
    for m in &picker.delegate.matches.matches {
        match &m {
            Match::History {
                path: history_path,
                panel_match: path_match,
            } => {
                if let Some(path_match) = path_match.as_ref() {
                    search_entries
                        .history
                        .push(path_match.0.path_prefix.join(&path_match.0.path));
                } else {
                    // This occurs when the query is empty and we show history matches
                    // that are outside the project.
                    panic!("currently not exercised in tests");
                }
                search_entries
                    .history_found_paths
                    .push(history_path.clone());
            }
            Match::Search(path_match) => {
                search_entries
                    .search
                    .push(path_match.0.path_prefix.join(&path_match.0.path));
                search_entries.search_matches.push(path_match.0.clone());
            }
            Match::CreateNew(_) => {}
        }
    }
    search_entries
}

#[track_caller]
fn assert_match_selection(
    finder: &Picker<FileFinderDelegate>,
    expected_selection_index: usize,
    expected_file_name: &str,
) {
    assert_eq!(
        finder.delegate.selected_index(),
        expected_selection_index,
        "Match is not selected"
    );
    assert_match_at_position(finder, expected_selection_index, expected_file_name);
}

#[track_caller]
fn assert_match_at_position(
    finder: &Picker<FileFinderDelegate>,
    match_index: usize,
    expected_file_name: &str,
) {
    let match_item = finder
        .delegate
        .matches
        .get(match_index)
        .unwrap_or_else(|| panic!("Finder has no match for index {match_index}"));
    let match_file_name = match &match_item {
        Match::History { path, .. } => path.absolute.file_name().and_then(|s| s.to_str()),
        Match::Search(path_match) => path_match.0.path.file_name(),
        Match::CreateNew(project_path) => project_path.path.file_name(),
    }
    .unwrap();
    assert_eq!(match_file_name, expected_file_name);
}

#[gpui::test]
async fn test_filename_precedence(cx: &mut TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/src"),
            json!({
                "layout": {
                    "app.css": "",
                    "app.d.ts": "",
                    "app.html": "",
                    "+page.svelte": "",
                },
                "routes": {
                    "+layout.svelte": "",
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/src").as_ref()], cx).await;
    let (picker, _, cx) = build_find_picker(project, cx);

    cx.simulate_input("layout");

    picker.update(cx, |finder, _| {
        let search_matches = collect_search_matches(finder).search_paths_only();

        assert_eq!(
            search_matches,
            vec![
                rel_path("routes/+layout.svelte").into(),
                rel_path("layout/app.css").into(),
                rel_path("layout/app.d.ts").into(),
                rel_path("layout/app.html").into(),
                rel_path("layout/+page.svelte").into(),
            ],
            "File with 'layout' in filename should be prioritized over files in 'layout' directory"
        );
    });
}
