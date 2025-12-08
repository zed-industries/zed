use super::*;
use collections::HashSet;
use editor::MultiBufferOffset;
use gpui::{Empty, Entity, TestAppContext, VisualTestContext, WindowHandle};
use pretty_assertions::assert_eq;
use project::FakeFs;
use serde_json::json;
use settings::{ProjectPanelAutoOpenSettings, SettingsStore};
use std::path::{Path, PathBuf};
use util::{path, paths::PathStyle, rel_path::rel_path};
use workspace::{
    AppState, ItemHandle, Pane,
    item::{Item, ProjectItem},
    register_project_item,
};

#[gpui::test]
async fn test_visible_list(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root1",
        json!({
            ".dockerignore": "",
            ".git": {
                "HEAD": "",
            },
            "a": {
                "0": { "q": "", "r": "", "s": "" },
                "1": { "t": "", "u": "" },
                "2": { "v": "", "w": "", "x": "", "y": "" },
            },
            "b": {
                "3": { "Q": "" },
                "4": { "R": "", "S": "", "T": "", "U": "" },
            },
            "C": {
                "5": {},
                "6": { "V": "", "W": "" },
                "7": { "X": "" },
                "8": { "Y": {}, "Z": "" }
            }
        }),
    )
    .await;
    fs.insert_tree(
        "/root2",
        json!({
            "d": {
                "9": ""
            },
            "e": {}
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root1".as_ref(), "/root2".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &[
            "v root1",
            "    > .git",
            "    > a",
            "    > b",
            "    > C",
            "      .dockerignore",
            "v root2",
            "    > d",
            "    > e",
        ]
    );

    toggle_expand_dir(&panel, "root1/b", cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &[
            "v root1",
            "    > .git",
            "    > a",
            "    v b  <== selected",
            "        > 3",
            "        > 4",
            "    > C",
            "      .dockerignore",
            "v root2",
            "    > d",
            "    > e",
        ]
    );

    assert_eq!(
        visible_entries_as_strings(&panel, 6..9, cx),
        &[
            //
            "    > C",
            "      .dockerignore",
            "v root2",
        ]
    );
}

#[gpui::test]
async fn test_opening_file(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
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

    let project = Project::test(fs.clone(), [path!("/src").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    toggle_expand_dir(&panel, "src/test", cx);
    select_path(&panel, "src/test/first.rs", cx);
    panel.update_in(cx, |panel, window, cx| panel.open(&Open, window, cx));
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v src",
            "    v test",
            "          first.rs  <== selected  <== marked",
            "          second.rs",
            "          third.rs"
        ]
    );
    ensure_single_file_is_opened(&workspace, "test/first.rs", cx);

    select_path(&panel, "src/test/second.rs", cx);
    panel.update_in(cx, |panel, window, cx| panel.open(&Open, window, cx));
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v src",
            "    v test",
            "          first.rs",
            "          second.rs  <== selected  <== marked",
            "          third.rs"
        ]
    );
    ensure_single_file_is_opened(&workspace, "test/second.rs", cx);
}

#[gpui::test]
async fn test_exclusions_in_visible_list(cx: &mut gpui::TestAppContext) {
    init_test(cx);
    cx.update(|cx| {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.worktree.file_scan_exclusions =
                    Some(vec!["**/.git".to_string(), "**/4/**".to_string()]);
            });
        });
    });

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/root1",
        json!({
            ".dockerignore": "",
            ".git": {
                "HEAD": "",
            },
            "a": {
                "0": { "q": "", "r": "", "s": "" },
                "1": { "t": "", "u": "" },
                "2": { "v": "", "w": "", "x": "", "y": "" },
            },
            "b": {
                "3": { "Q": "" },
                "4": { "R": "", "S": "", "T": "", "U": "" },
            },
            "C": {
                "5": {},
                "6": { "V": "", "W": "" },
                "7": { "X": "" },
                "8": { "Y": {}, "Z": "" }
            }
        }),
    )
    .await;
    fs.insert_tree(
        "/root2",
        json!({
            "d": {
                "4": ""
            },
            "e": {}
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root1".as_ref(), "/root2".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &[
            "v root1",
            "    > a",
            "    > b",
            "    > C",
            "      .dockerignore",
            "v root2",
            "    > d",
            "    > e",
        ]
    );

    toggle_expand_dir(&panel, "root1/b", cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &[
            "v root1",
            "    > a",
            "    v b  <== selected",
            "        > 3",
            "    > C",
            "      .dockerignore",
            "v root2",
            "    > d",
            "    > e",
        ]
    );

    toggle_expand_dir(&panel, "root2/d", cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &[
            "v root1",
            "    > a",
            "    v b",
            "        > 3",
            "    > C",
            "      .dockerignore",
            "v root2",
            "    v d  <== selected",
            "    > e",
        ]
    );

    toggle_expand_dir(&panel, "root2/e", cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &[
            "v root1",
            "    > a",
            "    v b",
            "        > 3",
            "    > C",
            "      .dockerignore",
            "v root2",
            "    v d",
            "    v e  <== selected",
        ]
    );
}

#[gpui::test]
async fn test_auto_collapse_dir_paths(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root1"),
        json!({
            "dir_1": {
                "nested_dir_1": {
                    "nested_dir_2": {
                        "nested_dir_3": {
                            "file_a.java": "// File contents",
                            "file_b.java": "// File contents",
                            "file_c.java": "// File contents",
                            "nested_dir_4": {
                                "nested_dir_5": {
                                    "file_d.java": "// File contents",
                                }
                            }
                        }
                    }
                }
            }
        }),
    )
    .await;
    fs.insert_tree(
        path!("/root2"),
        json!({
            "dir_2": {
                "file_1.java": "// File contents",
            }
        }),
    )
    .await;

    // Test 1: Multiple worktrees with auto_fold_dirs = true
    let project = Project::test(
        fs.clone(),
        [path!("/root1").as_ref(), path!("/root2").as_ref()],
        cx,
    )
    .await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    cx.update(|_, cx| {
        let settings = *ProjectPanelSettings::get_global(cx);
        ProjectPanelSettings::override_global(
            ProjectPanelSettings {
                auto_fold_dirs: true,
                sort_mode: settings::ProjectPanelSortMode::DirectoriesFirst,
                ..settings
            },
            cx,
        );
    });
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > dir_1/nested_dir_1/nested_dir_2/nested_dir_3",
            "v root2",
            "    > dir_2",
        ]
    );

    toggle_expand_dir(
        &panel,
        "root1/dir_1/nested_dir_1/nested_dir_2/nested_dir_3",
        cx,
    );
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    v dir_1/nested_dir_1/nested_dir_2/nested_dir_3  <== selected",
            "        > nested_dir_4/nested_dir_5",
            "          file_a.java",
            "          file_b.java",
            "          file_c.java",
            "v root2",
            "    > dir_2",
        ]
    );

    toggle_expand_dir(
        &panel,
        "root1/dir_1/nested_dir_1/nested_dir_2/nested_dir_3/nested_dir_4/nested_dir_5",
        cx,
    );
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    v dir_1/nested_dir_1/nested_dir_2/nested_dir_3",
            "        v nested_dir_4/nested_dir_5  <== selected",
            "              file_d.java",
            "          file_a.java",
            "          file_b.java",
            "          file_c.java",
            "v root2",
            "    > dir_2",
        ]
    );
    toggle_expand_dir(&panel, "root2/dir_2", cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    v dir_1/nested_dir_1/nested_dir_2/nested_dir_3",
            "        v nested_dir_4/nested_dir_5",
            "              file_d.java",
            "          file_a.java",
            "          file_b.java",
            "          file_c.java",
            "v root2",
            "    v dir_2  <== selected",
            "          file_1.java",
        ]
    );

    // Test 2: Single worktree with auto_fold_dirs = true and hide_root = true
    {
        let project = Project::test(fs.clone(), [path!("/root1").as_ref()], cx).await;
        let workspace =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        cx.update(|_, cx| {
            let settings = *ProjectPanelSettings::get_global(cx);
            ProjectPanelSettings::override_global(
                ProjectPanelSettings {
                    auto_fold_dirs: true,
                    hide_root: true,
                    ..settings
                },
                cx,
            );
        });
        let panel = workspace.update(cx, ProjectPanel::new).unwrap();
        cx.run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &["> dir_1/nested_dir_1/nested_dir_2/nested_dir_3"],
            "Single worktree with hide_root=true should hide root and show auto-folded paths"
        );

        toggle_expand_dir(
            &panel,
            "root1/dir_1/nested_dir_1/nested_dir_2/nested_dir_3",
            cx,
        );
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v dir_1/nested_dir_1/nested_dir_2/nested_dir_3  <== selected",
                "    > nested_dir_4/nested_dir_5",
                "      file_a.java",
                "      file_b.java",
                "      file_c.java",
            ],
            "Expanded auto-folded path with hidden root should show contents without root prefix"
        );

        toggle_expand_dir(
            &panel,
            "root1/dir_1/nested_dir_1/nested_dir_2/nested_dir_3/nested_dir_4/nested_dir_5",
            cx,
        );
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v dir_1/nested_dir_1/nested_dir_2/nested_dir_3",
                "    v nested_dir_4/nested_dir_5  <== selected",
                "          file_d.java",
                "      file_a.java",
                "      file_b.java",
                "      file_c.java",
            ],
            "Nested expansion with hidden root should maintain proper indentation"
        );
    }
}

#[gpui::test(iterations = 30)]
async fn test_editing_files(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root1",
        json!({
            ".dockerignore": "",
            ".git": {
                "HEAD": "",
            },
            "a": {
                "0": { "q": "", "r": "", "s": "" },
                "1": { "t": "", "u": "" },
                "2": { "v": "", "w": "", "x": "", "y": "" },
            },
            "b": {
                "3": { "Q": "" },
                "4": { "R": "", "S": "", "T": "", "U": "" },
            },
            "C": {
                "5": {},
                "6": { "V": "", "W": "" },
                "7": { "X": "" },
                "8": { "Y": {}, "Z": "" }
            }
        }),
    )
    .await;
    fs.insert_tree(
        "/root2",
        json!({
            "d": {
                "9": ""
            },
            "e": {}
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root1".as_ref(), "/root2".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace
        .update(cx, |workspace, window, cx| {
            let panel = ProjectPanel::new(workspace, window, cx);
            workspace.add_panel(panel.clone(), window, cx);
            panel
        })
        .unwrap();
    cx.run_until_parked();

    select_path(&panel, "root1", cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1  <== selected",
            "    > .git",
            "    > a",
            "    > b",
            "    > C",
            "      .dockerignore",
            "v root2",
            "    > d",
            "    > e",
        ]
    );

    // Add a file with the root folder selected. The filename editor is placed
    // before the first file in the root folder.
    panel.update_in(cx, |panel, window, cx| panel.new_file(&NewFile, window, cx));
    cx.run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(panel.filename_editor.read(cx).is_focused(window));
    });
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "    > a",
            "    > b",
            "    > C",
            "      [EDITOR: '']  <== selected",
            "      .dockerignore",
            "v root2",
            "    > d",
            "    > e",
        ]
    );

    let confirm = panel.update_in(cx, |panel, window, cx| {
        panel.filename_editor.update(cx, |editor, cx| {
            editor.set_text("the-new-filename", window, cx)
        });
        panel.confirm_edit(true, window, cx).unwrap()
    });
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "    > a",
            "    > b",
            "    > C",
            "      [PROCESSING: 'the-new-filename']  <== selected",
            "      .dockerignore",
            "v root2",
            "    > d",
            "    > e",
        ]
    );

    confirm.await.unwrap();
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "    > a",
            "    > b",
            "    > C",
            "      .dockerignore",
            "      the-new-filename  <== selected  <== marked",
            "v root2",
            "    > d",
            "    > e",
        ]
    );

    select_path(&panel, "root1/b", cx);
    panel.update_in(cx, |panel, window, cx| panel.new_file(&NewFile, window, cx));
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "    > a",
            "    v b",
            "        > 3",
            "        > 4",
            "          [EDITOR: '']  <== selected",
            "    > C",
            "      .dockerignore",
            "      the-new-filename",
        ]
    );

    panel
        .update_in(cx, |panel, window, cx| {
            panel.filename_editor.update(cx, |editor, cx| {
                editor.set_text("another-filename.txt", window, cx)
            });
            panel.confirm_edit(true, window, cx).unwrap()
        })
        .await
        .unwrap();
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "    > a",
            "    v b",
            "        > 3",
            "        > 4",
            "          another-filename.txt  <== selected  <== marked",
            "    > C",
            "      .dockerignore",
            "      the-new-filename",
        ]
    );

    select_path(&panel, "root1/b/another-filename.txt", cx);
    panel.update_in(cx, |panel, window, cx| panel.rename(&Rename, window, cx));
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "    > a",
            "    v b",
            "        > 3",
            "        > 4",
            "          [EDITOR: 'another-filename.txt']  <== selected  <== marked",
            "    > C",
            "      .dockerignore",
            "      the-new-filename",
        ]
    );

    let confirm = panel.update_in(cx, |panel, window, cx| {
        panel.filename_editor.update(cx, |editor, cx| {
            let file_name_selections = editor
                .selections
                .all::<MultiBufferOffset>(&editor.display_snapshot(cx));
            assert_eq!(
                file_name_selections.len(),
                1,
                "File editing should have a single selection, but got: {file_name_selections:?}"
            );
            let file_name_selection = &file_name_selections[0];
            assert_eq!(
                file_name_selection.start,
                MultiBufferOffset(0),
                "Should select the file name from the start"
            );
            assert_eq!(
                file_name_selection.end,
                MultiBufferOffset("another-filename".len()),
                "Should not select file extension"
            );

            editor.set_text("a-different-filename.tar.gz", window, cx)
        });
        panel.confirm_edit(true, window, cx).unwrap()
    });
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "    > a",
            "    v b",
            "        > 3",
            "        > 4",
            "          [PROCESSING: 'a-different-filename.tar.gz']  <== selected  <== marked",
            "    > C",
            "      .dockerignore",
            "      the-new-filename",
        ]
    );

    confirm.await.unwrap();
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "    > a",
            "    v b",
            "        > 3",
            "        > 4",
            "          a-different-filename.tar.gz  <== selected",
            "    > C",
            "      .dockerignore",
            "      the-new-filename",
        ]
    );

    panel.update_in(cx, |panel, window, cx| panel.rename(&Rename, window, cx));
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "    > a",
            "    v b",
            "        > 3",
            "        > 4",
            "          [EDITOR: 'a-different-filename.tar.gz']  <== selected",
            "    > C",
            "      .dockerignore",
            "      the-new-filename",
        ]
    );

    panel.update_in(cx, |panel, window, cx| {
            panel.filename_editor.update(cx, |editor, cx| {
                let file_name_selections = editor.selections.all::<MultiBufferOffset>(&editor.display_snapshot(cx));
                assert_eq!(file_name_selections.len(), 1, "File editing should have a single selection, but got: {file_name_selections:?}");
                let file_name_selection = &file_name_selections[0];
                assert_eq!(file_name_selection.start, MultiBufferOffset(0), "Should select the file name from the start");
                assert_eq!(file_name_selection.end, MultiBufferOffset("a-different-filename.tar".len()), "Should not select file extension, but still may select anything up to the last dot..");

            });
            panel.cancel(&menu::Cancel, window, cx)
        });
    cx.run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        panel.new_directory(&NewDirectory, window, cx)
    });
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "    > a",
            "    v b",
            "        > [EDITOR: '']  <== selected",
            "        > 3",
            "        > 4",
            "          a-different-filename.tar.gz",
            "    > C",
            "      .dockerignore",
        ]
    );

    let confirm = panel.update_in(cx, |panel, window, cx| {
        panel
            .filename_editor
            .update(cx, |editor, cx| editor.set_text("new-dir", window, cx));
        panel.confirm_edit(true, window, cx).unwrap()
    });
    panel.update_in(cx, |panel, window, cx| {
        panel.select_next(&Default::default(), window, cx)
    });
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "    > a",
            "    v b",
            "        > [PROCESSING: 'new-dir']",
            "        > 3  <== selected",
            "        > 4",
            "          a-different-filename.tar.gz",
            "    > C",
            "      .dockerignore",
        ]
    );

    confirm.await.unwrap();
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "    > a",
            "    v b",
            "        > 3  <== selected",
            "        > 4",
            "        > new-dir",
            "          a-different-filename.tar.gz",
            "    > C",
            "      .dockerignore",
        ]
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.rename(&Default::default(), window, cx)
    });
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "    > a",
            "    v b",
            "        > [EDITOR: '3']  <== selected",
            "        > 4",
            "        > new-dir",
            "          a-different-filename.tar.gz",
            "    > C",
            "      .dockerignore",
        ]
    );

    // Dismiss the rename editor when it loses focus.
    workspace.update(cx, |_, window, _| window.blur()).unwrap();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "    > a",
            "    v b",
            "        > 3  <== selected",
            "        > 4",
            "        > new-dir",
            "          a-different-filename.tar.gz",
            "    > C",
            "      .dockerignore",
        ]
    );

    // Test empty filename and filename with only whitespace
    panel.update_in(cx, |panel, window, cx| panel.new_file(&NewFile, window, cx));
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "    > a",
            "    v b",
            "        v 3",
            "              [EDITOR: '']  <== selected",
            "              Q",
            "        > 4",
            "        > new-dir",
            "          a-different-filename.tar.gz",
        ]
    );
    panel.update_in(cx, |panel, window, cx| {
        panel.filename_editor.update(cx, |editor, cx| {
            editor.set_text("", window, cx);
        });
        assert!(panel.confirm_edit(true, window, cx).is_none());
        panel.filename_editor.update(cx, |editor, cx| {
            editor.set_text("   ", window, cx);
        });
        assert!(panel.confirm_edit(true, window, cx).is_none());
        panel.cancel(&menu::Cancel, window, cx);
        panel.update_visible_entries(None, false, false, window, cx);
    });
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "    > a",
            "    v b",
            "        v 3  <== selected",
            "              Q",
            "        > 4",
            "        > new-dir",
            "          a-different-filename.tar.gz",
            "    > C",
        ]
    );
}

#[gpui::test(iterations = 10)]
async fn test_adding_directories_via_file(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root1",
        json!({
            ".dockerignore": "",
            ".git": {
                "HEAD": "",
            },
            "a": {
                "0": { "q": "", "r": "", "s": "" },
                "1": { "t": "", "u": "" },
                "2": { "v": "", "w": "", "x": "", "y": "" },
            },
            "b": {
                "3": { "Q": "" },
                "4": { "R": "", "S": "", "T": "", "U": "" },
            },
            "C": {
                "5": {},
                "6": { "V": "", "W": "" },
                "7": { "X": "" },
                "8": { "Y": {}, "Z": "" }
            }
        }),
    )
    .await;
    fs.insert_tree(
        "/root2",
        json!({
            "d": {
                "9": ""
            },
            "e": {}
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root1".as_ref(), "/root2".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace
        .update(cx, |workspace, window, cx| {
            let panel = ProjectPanel::new(workspace, window, cx);
            workspace.add_panel(panel.clone(), window, cx);
            panel
        })
        .unwrap();
    cx.run_until_parked();

    select_path(&panel, "root1", cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1  <== selected",
            "    > .git",
            "    > a",
            "    > b",
            "    > C",
            "      .dockerignore",
            "v root2",
            "    > d",
            "    > e",
        ]
    );

    // Add a file with the root folder selected. The filename editor is placed
    // before the first file in the root folder.
    panel.update_in(cx, |panel, window, cx| panel.new_file(&NewFile, window, cx));
    cx.run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(panel.filename_editor.read(cx).is_focused(window));
    });
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "    > a",
            "    > b",
            "    > C",
            "      [EDITOR: '']  <== selected",
            "      .dockerignore",
            "v root2",
            "    > d",
            "    > e",
        ]
    );

    let confirm = panel.update_in(cx, |panel, window, cx| {
        panel.filename_editor.update(cx, |editor, cx| {
            editor.set_text("/bdir1/dir2/the-new-filename", window, cx)
        });
        panel.confirm_edit(true, window, cx).unwrap()
    });

    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "    > a",
            "    > b",
            "    > C",
            "      [PROCESSING: 'bdir1/dir2/the-new-filename']  <== selected",
            "      .dockerignore",
            "v root2",
            "    > d",
            "    > e",
        ]
    );

    confirm.await.unwrap();
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..13, cx),
        &[
            "v root1",
            "    > .git",
            "    > a",
            "    > b",
            "    v bdir1",
            "        v dir2",
            "              the-new-filename  <== selected  <== marked",
            "    > C",
            "      .dockerignore",
            "v root2",
            "    > d",
            "    > e",
        ]
    );
}

#[gpui::test]
async fn test_adding_directory_via_file(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root1"),
        json!({
            ".dockerignore": "",
            ".git": {
                "HEAD": "",
            },
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/root1").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace
        .update(cx, |workspace, window, cx| {
            let panel = ProjectPanel::new(workspace, window, cx);
            workspace.add_panel(panel.clone(), window, cx);
            panel
        })
        .unwrap();
    cx.run_until_parked();

    select_path(&panel, "root1", cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &["v root1  <== selected", "    > .git", "      .dockerignore",]
    );

    // Add a file with the root folder selected. The filename editor is placed
    // before the first file in the root folder.
    panel.update_in(cx, |panel, window, cx| panel.new_file(&NewFile, window, cx));
    cx.run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(panel.filename_editor.read(cx).is_focused(window));
    });
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "      [EDITOR: '']  <== selected",
            "      .dockerignore",
        ]
    );

    let confirm = panel.update_in(cx, |panel, window, cx| {
        // If we want to create a subdirectory, there should be no prefix slash.
        panel
            .filename_editor
            .update(cx, |editor, cx| editor.set_text("new_dir/", window, cx));
        panel.confirm_edit(true, window, cx).unwrap()
    });

    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "      [PROCESSING: 'new_dir']  <== selected",
            "      .dockerignore",
        ]
    );

    confirm.await.unwrap();
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "    v new_dir  <== selected",
            "      .dockerignore",
        ]
    );

    // Test filename with whitespace
    select_path(&panel, "root1", cx);
    panel.update_in(cx, |panel, window, cx| panel.new_file(&NewFile, window, cx));
    let confirm = panel.update_in(cx, |panel, window, cx| {
        // If we want to create a subdirectory, there should be no prefix slash.
        panel
            .filename_editor
            .update(cx, |editor, cx| editor.set_text("new dir 2/", window, cx));
        panel.confirm_edit(true, window, cx).unwrap()
    });
    confirm.await.unwrap();
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "    v new dir 2  <== selected",
            "    v new_dir",
            "      .dockerignore",
        ]
    );

    // Test filename ends with "\"
    #[cfg(target_os = "windows")]
    {
        select_path(&panel, "root1", cx);
        panel.update_in(cx, |panel, window, cx| panel.new_file(&NewFile, window, cx));
        let confirm = panel.update_in(cx, |panel, window, cx| {
            // If we want to create a subdirectory, there should be no prefix slash.
            panel
                .filename_editor
                .update(cx, |editor, cx| editor.set_text("new_dir_3\\", window, cx));
            panel.confirm_edit(true, window, cx).unwrap()
        });
        confirm.await.unwrap();
        cx.run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > .git",
                "    v new dir 2",
                "    v new_dir",
                "    v new_dir_3  <== selected",
                "      .dockerignore",
            ]
        );
    }
}

#[gpui::test]
async fn test_copy_paste(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root1",
        json!({
            "one.two.txt": "",
            "one.txt": ""
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root1".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    panel.update_in(cx, |panel, window, cx| {
        panel.select_next(&Default::default(), window, cx);
        panel.select_next(&Default::default(), window, cx);
    });

    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &[
            //
            "v root1",
            "      one.txt  <== selected",
            "      one.two.txt",
        ]
    );

    // Regression test - file name is created correctly when
    // the copied file's name contains multiple dots.
    panel.update_in(cx, |panel, window, cx| {
        panel.copy(&Default::default(), window, cx);
        panel.paste(&Default::default(), window, cx);
    });
    cx.executor().run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(panel.filename_editor.read(cx).is_focused(window));
    });
    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &[
            //
            "v root1",
            "      one.txt",
            "      [EDITOR: 'one copy.txt']  <== selected  <== marked",
            "      one.two.txt",
        ]
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.filename_editor.update(cx, |editor, cx| {
            let file_name_selections = editor
                .selections
                .all::<MultiBufferOffset>(&editor.display_snapshot(cx));
            assert_eq!(
                file_name_selections.len(),
                1,
                "File editing should have a single selection, but got: {file_name_selections:?}"
            );
            let file_name_selection = &file_name_selections[0];
            assert_eq!(
                file_name_selection.start,
                MultiBufferOffset("one".len()),
                "Should select the file name disambiguation after the original file name"
            );
            assert_eq!(
                file_name_selection.end,
                MultiBufferOffset("one copy".len()),
                "Should select the file name disambiguation until the extension"
            );
        });
        assert!(panel.confirm_edit(true, window, cx).is_none());
    });

    panel.update_in(cx, |panel, window, cx| {
        panel.paste(&Default::default(), window, cx);
    });
    cx.executor().run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(panel.filename_editor.read(cx).is_focused(window));
    });
    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &[
            //
            "v root1",
            "      one.txt",
            "      one copy.txt",
            "      [EDITOR: 'one copy 1.txt']  <== selected  <== marked",
            "      one.two.txt",
        ]
    );

    panel.update_in(cx, |panel, window, cx| {
        assert!(panel.confirm_edit(true, window, cx).is_none())
    });
}

#[gpui::test]
async fn test_cut_paste(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "one.txt": "",
            "two.txt": "",
            "a": {},
            "b": {}
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    select_path_with_mark(&panel, "root/one.txt", cx);
    select_path_with_mark(&panel, "root/two.txt", cx);

    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &[
            "v root",
            "    > a",
            "    > b",
            "      one.txt  <== marked",
            "      two.txt  <== selected  <== marked",
        ]
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.cut(&Default::default(), window, cx);
    });

    select_path(&panel, "root/a", cx);

    panel.update_in(cx, |panel, window, cx| {
        panel.paste(&Default::default(), window, cx);
        panel.update_visible_entries(None, false, false, window, cx);
    });
    cx.executor().run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &[
            "v root",
            "    v a",
            "          one.txt  <== marked",
            "          two.txt  <== selected  <== marked",
            "    > b",
        ],
        "Cut entries should be moved on first paste."
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.cancel(&menu::Cancel {}, window, cx)
    });
    cx.executor().run_until_parked();

    select_path(&panel, "root/b", cx);

    panel.update_in(cx, |panel, window, cx| {
        panel.paste(&Default::default(), window, cx);
    });
    cx.executor().run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &[
            "v root",
            "    v a",
            "          one.txt",
            "          two.txt",
            "    v b",
            "          one.txt",
            "          two.txt  <== selected",
        ],
        "Cut entries should only be copied for the second paste!"
    );
}

#[gpui::test]
async fn test_cut_paste_between_different_worktrees(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root1",
        json!({
            "one.txt": "",
            "two.txt": "",
            "three.txt": "",
            "a": {
                "0": { "q": "", "r": "", "s": "" },
                "1": { "t": "", "u": "" },
                "2": { "v": "", "w": "", "x": "", "y": "" },
            },
        }),
    )
    .await;

    fs.insert_tree(
        "/root2",
        json!({
            "one.txt": "",
            "two.txt": "",
            "four.txt": "",
            "b": {
                "3": { "Q": "" },
                "4": { "R": "", "S": "", "T": "", "U": "" },
            },
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root1".as_ref(), "/root2".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    select_path(&panel, "root1/three.txt", cx);
    panel.update_in(cx, |panel, window, cx| {
        panel.cut(&Default::default(), window, cx);
    });

    select_path(&panel, "root2/one.txt", cx);
    panel.update_in(cx, |panel, window, cx| {
        panel.select_next(&Default::default(), window, cx);
        panel.paste(&Default::default(), window, cx);
    });
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &[
            //
            "v root1",
            "    > a",
            "      one.txt",
            "      two.txt",
            "v root2",
            "    > b",
            "      four.txt",
            "      one.txt",
            "      three.txt  <== selected  <== marked",
            "      two.txt",
        ]
    );

    select_path(&panel, "root1/a", cx);
    panel.update_in(cx, |panel, window, cx| {
        panel.cut(&Default::default(), window, cx);
    });
    select_path(&panel, "root2/two.txt", cx);
    panel.update_in(cx, |panel, window, cx| {
        panel.select_next(&Default::default(), window, cx);
        panel.paste(&Default::default(), window, cx);
    });

    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &[
            //
            "v root1",
            "      one.txt",
            "      two.txt",
            "v root2",
            "    > a  <== selected",
            "    > b",
            "      four.txt",
            "      one.txt",
            "      three.txt  <== marked",
            "      two.txt",
        ]
    );
}

#[gpui::test]
async fn test_copy_paste_between_different_worktrees(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root1",
        json!({
            "one.txt": "",
            "two.txt": "",
            "three.txt": "",
            "a": {
                "0": { "q": "", "r": "", "s": "" },
                "1": { "t": "", "u": "" },
                "2": { "v": "", "w": "", "x": "", "y": "" },
            },
        }),
    )
    .await;

    fs.insert_tree(
        "/root2",
        json!({
            "one.txt": "",
            "two.txt": "",
            "four.txt": "",
            "b": {
                "3": { "Q": "" },
                "4": { "R": "", "S": "", "T": "", "U": "" },
            },
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root1".as_ref(), "/root2".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    select_path(&panel, "root1/three.txt", cx);
    panel.update_in(cx, |panel, window, cx| {
        panel.copy(&Default::default(), window, cx);
    });

    select_path(&panel, "root2/one.txt", cx);
    panel.update_in(cx, |panel, window, cx| {
        panel.select_next(&Default::default(), window, cx);
        panel.paste(&Default::default(), window, cx);
    });
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &[
            //
            "v root1",
            "    > a",
            "      one.txt",
            "      three.txt",
            "      two.txt",
            "v root2",
            "    > b",
            "      four.txt",
            "      one.txt",
            "      three.txt  <== selected  <== marked",
            "      two.txt",
        ]
    );

    select_path(&panel, "root1/three.txt", cx);
    panel.update_in(cx, |panel, window, cx| {
        panel.copy(&Default::default(), window, cx);
    });
    select_path(&panel, "root2/two.txt", cx);
    panel.update_in(cx, |panel, window, cx| {
        panel.select_next(&Default::default(), window, cx);
        panel.paste(&Default::default(), window, cx);
    });

    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &[
            //
            "v root1",
            "    > a",
            "      one.txt",
            "      three.txt",
            "      two.txt",
            "v root2",
            "    > b",
            "      four.txt",
            "      one.txt",
            "      three.txt",
            "      [EDITOR: 'three copy.txt']  <== selected  <== marked",
            "      two.txt",
        ]
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.cancel(&menu::Cancel {}, window, cx)
    });
    cx.executor().run_until_parked();

    select_path(&panel, "root1/a", cx);
    panel.update_in(cx, |panel, window, cx| {
        panel.copy(&Default::default(), window, cx);
    });
    select_path(&panel, "root2/two.txt", cx);
    panel.update_in(cx, |panel, window, cx| {
        panel.select_next(&Default::default(), window, cx);
        panel.paste(&Default::default(), window, cx);
    });

    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &[
            //
            "v root1",
            "    > a",
            "      one.txt",
            "      three.txt",
            "      two.txt",
            "v root2",
            "    > a  <== selected",
            "    > b",
            "      four.txt",
            "      one.txt",
            "      three.txt",
            "      three copy.txt",
            "      two.txt",
        ]
    );
}

#[gpui::test]
async fn test_copy_paste_directory(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "a": {
                "one.txt": "",
                "two.txt": "",
                "inner_dir": {
                    "three.txt": "",
                    "four.txt": "",
                }
            },
            "b": {}
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    select_path(&panel, "root/a", cx);
    panel.update_in(cx, |panel, window, cx| {
        panel.copy(&Default::default(), window, cx);
        panel.select_next(&Default::default(), window, cx);
        panel.paste(&Default::default(), window, cx);
    });
    cx.executor().run_until_parked();

    let pasted_dir = find_project_entry(&panel, "root/b/a", cx);
    assert_ne!(pasted_dir, None, "Pasted directory should have an entry");

    let pasted_dir_file = find_project_entry(&panel, "root/b/a/one.txt", cx);
    assert_ne!(
        pasted_dir_file, None,
        "Pasted directory file should have an entry"
    );

    let pasted_dir_inner_dir = find_project_entry(&panel, "root/b/a/inner_dir", cx);
    assert_ne!(
        pasted_dir_inner_dir, None,
        "Directories inside pasted directory should have an entry"
    );

    toggle_expand_dir(&panel, "root/b/a", cx);
    toggle_expand_dir(&panel, "root/b/a/inner_dir", cx);

    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &[
            //
            "v root",
            "    > a",
            "    v b",
            "        v a",
            "            v inner_dir  <== selected",
            "                  four.txt",
            "                  three.txt",
            "              one.txt",
            "              two.txt",
        ]
    );

    select_path(&panel, "root", cx);
    panel.update_in(cx, |panel, window, cx| {
        panel.paste(&Default::default(), window, cx)
    });
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &[
            //
            "v root",
            "    > a",
            "    > [EDITOR: 'a copy']  <== selected",
            "    v b",
            "        v a",
            "            v inner_dir",
            "                  four.txt",
            "                  three.txt",
            "              one.txt",
            "              two.txt"
        ]
    );

    let confirm = panel.update_in(cx, |panel, window, cx| {
        panel
            .filename_editor
            .update(cx, |editor, cx| editor.set_text("c", window, cx));
        panel.confirm_edit(true, window, cx).unwrap()
    });
    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &[
            //
            "v root",
            "    > a",
            "    > [PROCESSING: 'c']  <== selected",
            "    v b",
            "        v a",
            "            v inner_dir",
            "                  four.txt",
            "                  three.txt",
            "              one.txt",
            "              two.txt"
        ]
    );

    confirm.await.unwrap();

    panel.update_in(cx, |panel, window, cx| {
        panel.paste(&Default::default(), window, cx)
    });
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &[
            //
            "v root",
            "    > a",
            "    v b",
            "        v a",
            "            v inner_dir",
            "                  four.txt",
            "                  three.txt",
            "              one.txt",
            "              two.txt",
            "    v c",
            "        > a  <== selected",
            "        > inner_dir",
            "          one.txt",
            "          two.txt",
        ]
    );
}

#[gpui::test]
async fn test_copy_paste_directory_with_sibling_file(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/test",
        json!({
            "dir1": {
                "a.txt": "",
                "b.txt": "",
            },
            "dir2": {},
            "c.txt": "",
            "d.txt": "",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/test".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    toggle_expand_dir(&panel, "test/dir1", cx);

    cx.simulate_modifiers_change(gpui::Modifiers {
        control: true,
        ..Default::default()
    });

    select_path_with_mark(&panel, "test/dir1", cx);
    select_path_with_mark(&panel, "test/c.txt", cx);

    assert_eq!(
        visible_entries_as_strings(&panel, 0..15, cx),
        &[
            "v test",
            "    v dir1  <== marked",
            "          a.txt",
            "          b.txt",
            "    > dir2",
            "      c.txt  <== selected  <== marked",
            "      d.txt",
        ],
        "Initial state before copying dir1 and c.txt"
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.copy(&Default::default(), window, cx);
    });
    select_path(&panel, "test/dir2", cx);
    panel.update_in(cx, |panel, window, cx| {
        panel.paste(&Default::default(), window, cx);
    });
    cx.executor().run_until_parked();

    toggle_expand_dir(&panel, "test/dir2/dir1", cx);

    assert_eq!(
        visible_entries_as_strings(&panel, 0..15, cx),
        &[
            "v test",
            "    v dir1  <== marked",
            "          a.txt",
            "          b.txt",
            "    v dir2",
            "        v dir1  <== selected",
            "              a.txt",
            "              b.txt",
            "          c.txt",
            "      c.txt  <== marked",
            "      d.txt",
        ],
        "Should copy dir1 as well as c.txt into dir2"
    );

    // Disambiguating multiple files should not open the rename editor.
    select_path(&panel, "test/dir2", cx);
    panel.update_in(cx, |panel, window, cx| {
        panel.paste(&Default::default(), window, cx);
    });
    cx.executor().run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..15, cx),
        &[
            "v test",
            "    v dir1  <== marked",
            "          a.txt",
            "          b.txt",
            "    v dir2",
            "        v dir1",
            "              a.txt",
            "              b.txt",
            "        > dir1 copy  <== selected",
            "          c.txt",
            "          c copy.txt",
            "      c.txt  <== marked",
            "      d.txt",
        ],
        "Should copy dir1 as well as c.txt into dir2 and disambiguate them without opening the rename editor"
    );
}

#[gpui::test]
async fn test_copy_paste_nested_and_root_entries(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/test",
        json!({
            "dir1": {
                "a.txt": "",
                "b.txt": "",
            },
            "dir2": {},
            "c.txt": "",
            "d.txt": "",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/test".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    toggle_expand_dir(&panel, "test/dir1", cx);

    cx.simulate_modifiers_change(gpui::Modifiers {
        control: true,
        ..Default::default()
    });

    select_path_with_mark(&panel, "test/dir1/a.txt", cx);
    select_path_with_mark(&panel, "test/dir1", cx);
    select_path_with_mark(&panel, "test/c.txt", cx);

    assert_eq!(
        visible_entries_as_strings(&panel, 0..15, cx),
        &[
            "v test",
            "    v dir1  <== marked",
            "          a.txt  <== marked",
            "          b.txt",
            "    > dir2",
            "      c.txt  <== selected  <== marked",
            "      d.txt",
        ],
        "Initial state before copying a.txt, dir1 and c.txt"
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.copy(&Default::default(), window, cx);
    });
    select_path(&panel, "test/dir2", cx);
    panel.update_in(cx, |panel, window, cx| {
        panel.paste(&Default::default(), window, cx);
    });
    cx.executor().run_until_parked();

    toggle_expand_dir(&panel, "test/dir2/dir1", cx);

    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v test",
            "    v dir1  <== marked",
            "          a.txt  <== marked",
            "          b.txt",
            "    v dir2",
            "        v dir1  <== selected",
            "              a.txt",
            "              b.txt",
            "          c.txt",
            "      c.txt  <== marked",
            "      d.txt",
        ],
        "Should copy dir1 and c.txt into dir2. a.txt is already present in copied dir1."
    );
}

#[gpui::test]
async fn test_remove_opened_file(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
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

    let project = Project::test(fs.clone(), [path!("/src").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    toggle_expand_dir(&panel, "src/test", cx);
    select_path(&panel, "src/test/first.rs", cx);
    panel.update_in(cx, |panel, window, cx| panel.open(&Open, window, cx));
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v src",
            "    v test",
            "          first.rs  <== selected  <== marked",
            "          second.rs",
            "          third.rs"
        ]
    );
    ensure_single_file_is_opened(&workspace, "test/first.rs", cx);

    submit_deletion(&panel, cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v src",
            "    v test",
            "          second.rs  <== selected",
            "          third.rs"
        ],
        "Project panel should have no deleted file, no other file is selected in it"
    );
    ensure_no_open_items_and_panes(&workspace, cx);

    panel.update_in(cx, |panel, window, cx| panel.open(&Open, window, cx));
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v src",
            "    v test",
            "          second.rs  <== selected  <== marked",
            "          third.rs"
        ]
    );
    ensure_single_file_is_opened(&workspace, "test/second.rs", cx);

    workspace
        .update(cx, |workspace, window, cx| {
            let active_items = workspace
                .panes()
                .iter()
                .filter_map(|pane| pane.read(cx).active_item())
                .collect::<Vec<_>>();
            assert_eq!(active_items.len(), 1);
            let open_editor = active_items
                .into_iter()
                .next()
                .unwrap()
                .downcast::<Editor>()
                .expect("Open item should be an editor");
            open_editor.update(cx, |editor, cx| {
                editor.set_text("Another text!", window, cx)
            });
        })
        .unwrap();
    submit_deletion_skipping_prompt(&panel, cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &["v src", "    v test", "          third.rs  <== selected"],
        "Project panel should have no deleted file, with one last file remaining"
    );
    ensure_no_open_items_and_panes(&workspace, cx);
}

#[gpui::test]
async fn test_auto_open_new_file_when_enabled(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);
    set_auto_open_settings(
        cx,
        ProjectPanelAutoOpenSettings {
            on_create: Some(true),
            ..Default::default()
        },
    );

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/root"), json!({})).await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    panel.update_in(cx, |panel, window, cx| panel.new_file(&NewFile, window, cx));
    cx.run_until_parked();
    panel
        .update_in(cx, |panel, window, cx| {
            panel.filename_editor.update(cx, |editor, cx| {
                editor.set_text("auto-open.rs", window, cx);
            });
            panel.confirm_edit(true, window, cx).unwrap()
        })
        .await
        .unwrap();
    cx.run_until_parked();

    ensure_single_file_is_opened(&workspace, "auto-open.rs", cx);
}

#[gpui::test]
async fn test_auto_open_new_file_when_disabled(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);
    set_auto_open_settings(
        cx,
        ProjectPanelAutoOpenSettings {
            on_create: Some(false),
            ..Default::default()
        },
    );

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/root"), json!({})).await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    panel.update_in(cx, |panel, window, cx| panel.new_file(&NewFile, window, cx));
    cx.run_until_parked();
    panel
        .update_in(cx, |panel, window, cx| {
            panel.filename_editor.update(cx, |editor, cx| {
                editor.set_text("manual-open.rs", window, cx);
            });
            panel.confirm_edit(true, window, cx).unwrap()
        })
        .await
        .unwrap();
    cx.run_until_parked();

    ensure_no_open_items_and_panes(&workspace, cx);
}

#[gpui::test]
async fn test_auto_open_on_paste_when_enabled(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);
    set_auto_open_settings(
        cx,
        ProjectPanelAutoOpenSettings {
            on_paste: Some(true),
            ..Default::default()
        },
    );

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            "src": {
                "original.rs": ""
            },
            "target": {}
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    toggle_expand_dir(&panel, "root/src", cx);
    toggle_expand_dir(&panel, "root/target", cx);

    select_path(&panel, "root/src/original.rs", cx);
    panel.update_in(cx, |panel, window, cx| {
        panel.copy(&Default::default(), window, cx);
    });

    select_path(&panel, "root/target", cx);
    panel.update_in(cx, |panel, window, cx| {
        panel.paste(&Default::default(), window, cx);
    });
    cx.executor().run_until_parked();

    ensure_single_file_is_opened(&workspace, "target/original.rs", cx);
}

#[gpui::test]
async fn test_auto_open_on_paste_when_disabled(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);
    set_auto_open_settings(
        cx,
        ProjectPanelAutoOpenSettings {
            on_paste: Some(false),
            ..Default::default()
        },
    );

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            "src": {
                "original.rs": ""
            },
            "target": {}
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    toggle_expand_dir(&panel, "root/src", cx);
    toggle_expand_dir(&panel, "root/target", cx);

    select_path(&panel, "root/src/original.rs", cx);
    panel.update_in(cx, |panel, window, cx| {
        panel.copy(&Default::default(), window, cx);
    });

    select_path(&panel, "root/target", cx);
    panel.update_in(cx, |panel, window, cx| {
        panel.paste(&Default::default(), window, cx);
    });
    cx.executor().run_until_parked();

    ensure_no_open_items_and_panes(&workspace, cx);
    assert!(
        find_project_entry(&panel, "root/target/original.rs", cx).is_some(),
        "Pasted entry should exist even when auto-open is disabled"
    );
}

#[gpui::test]
async fn test_auto_open_on_drop_when_enabled(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);
    set_auto_open_settings(
        cx,
        ProjectPanelAutoOpenSettings {
            on_drop: Some(true),
            ..Default::default()
        },
    );

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/root"), json!({})).await;

    let temp_dir = tempfile::tempdir().unwrap();
    let external_path = temp_dir.path().join("dropped.rs");
    std::fs::write(&external_path, "// dropped").unwrap();
    fs.insert_tree_from_real_fs(temp_dir.path(), temp_dir.path())
        .await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    let root_entry = find_project_entry(&panel, "root", cx).unwrap();
    panel.update_in(cx, |panel, window, cx| {
        panel.drop_external_files(std::slice::from_ref(&external_path), root_entry, window, cx);
    });
    cx.executor().run_until_parked();

    ensure_single_file_is_opened(&workspace, "dropped.rs", cx);
}

#[gpui::test]
async fn test_auto_open_on_drop_when_disabled(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);
    set_auto_open_settings(
        cx,
        ProjectPanelAutoOpenSettings {
            on_drop: Some(false),
            ..Default::default()
        },
    );

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/root"), json!({})).await;

    let temp_dir = tempfile::tempdir().unwrap();
    let external_path = temp_dir.path().join("manual.rs");
    std::fs::write(&external_path, "// dropped").unwrap();
    fs.insert_tree_from_real_fs(temp_dir.path(), temp_dir.path())
        .await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    let root_entry = find_project_entry(&panel, "root", cx).unwrap();
    panel.update_in(cx, |panel, window, cx| {
        panel.drop_external_files(std::slice::from_ref(&external_path), root_entry, window, cx);
    });
    cx.executor().run_until_parked();

    ensure_no_open_items_and_panes(&workspace, cx);
    assert!(
        find_project_entry(&panel, "root/manual.rs", cx).is_some(),
        "Dropped entry should exist even when auto-open is disabled"
    );
}

#[gpui::test]
async fn test_create_duplicate_items(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
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

    let project = Project::test(fs.clone(), ["/src".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace
        .update(cx, |workspace, window, cx| {
            let panel = ProjectPanel::new(workspace, window, cx);
            workspace.add_panel(panel.clone(), window, cx);
            panel
        })
        .unwrap();
    cx.run_until_parked();

    select_path(&panel, "src", cx);
    panel.update_in(cx, |panel, window, cx| panel.confirm(&Confirm, window, cx));
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            //
            "v src  <== selected",
            "    > test"
        ]
    );
    panel.update_in(cx, |panel, window, cx| {
        panel.new_directory(&NewDirectory, window, cx)
    });
    cx.run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(panel.filename_editor.read(cx).is_focused(window));
    });
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            //
            "v src",
            "    > [EDITOR: '']  <== selected",
            "    > test"
        ]
    );
    panel.update_in(cx, |panel, window, cx| {
        panel
            .filename_editor
            .update(cx, |editor, cx| editor.set_text("test", window, cx));
        assert!(
            panel.confirm_edit(true, window, cx).is_none(),
            "Should not allow to confirm on conflicting new directory name"
        );
    });
    cx.executor().run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(
            panel.state.edit_state.is_some(),
            "Edit state should not be None after conflicting new directory name"
        );
        panel.cancel(&menu::Cancel, window, cx);
        panel.update_visible_entries(None, false, false, window, cx);
    });
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            //
            "v src  <== selected",
            "    > test"
        ],
        "File list should be unchanged after failed folder create confirmation"
    );

    select_path(&panel, "src/test", cx);
    panel.update_in(cx, |panel, window, cx| panel.confirm(&Confirm, window, cx));
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            //
            "v src",
            "    > test  <== selected"
        ]
    );
    panel.update_in(cx, |panel, window, cx| panel.new_file(&NewFile, window, cx));
    cx.run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(panel.filename_editor.read(cx).is_focused(window));
    });
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v src",
            "    v test",
            "          [EDITOR: '']  <== selected",
            "          first.rs",
            "          second.rs",
            "          third.rs"
        ]
    );
    panel.update_in(cx, |panel, window, cx| {
        panel
            .filename_editor
            .update(cx, |editor, cx| editor.set_text("first.rs", window, cx));
        assert!(
            panel.confirm_edit(true, window, cx).is_none(),
            "Should not allow to confirm on conflicting new file name"
        );
    });
    cx.executor().run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(
            panel.state.edit_state.is_some(),
            "Edit state should not be None after conflicting new file name"
        );
        panel.cancel(&menu::Cancel, window, cx);
        panel.update_visible_entries(None, false, false, window, cx);
    });
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v src",
            "    v test  <== selected",
            "          first.rs",
            "          second.rs",
            "          third.rs"
        ],
        "File list should be unchanged after failed file create confirmation"
    );

    select_path(&panel, "src/test/first.rs", cx);
    panel.update_in(cx, |panel, window, cx| panel.confirm(&Confirm, window, cx));
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v src",
            "    v test",
            "          first.rs  <== selected",
            "          second.rs",
            "          third.rs"
        ],
    );
    panel.update_in(cx, |panel, window, cx| panel.rename(&Rename, window, cx));
    cx.executor().run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(panel.filename_editor.read(cx).is_focused(window));
    });
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v src",
            "    v test",
            "          [EDITOR: 'first.rs']  <== selected",
            "          second.rs",
            "          third.rs"
        ]
    );
    panel.update_in(cx, |panel, window, cx| {
        panel
            .filename_editor
            .update(cx, |editor, cx| editor.set_text("second.rs", window, cx));
        assert!(
            panel.confirm_edit(true, window, cx).is_none(),
            "Should not allow to confirm on conflicting file rename"
        )
    });
    cx.executor().run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(
            panel.state.edit_state.is_some(),
            "Edit state should not be None after conflicting file rename"
        );
        panel.cancel(&menu::Cancel, window, cx);
    });
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v src",
            "    v test",
            "          first.rs  <== selected",
            "          second.rs",
            "          third.rs"
        ],
        "File list should be unchanged after failed rename confirmation"
    );
}

// NOTE: This test is skipped on Windows, because on Windows,
// when it triggers the lsp store it converts `/src/test/first copy.txt` into an uri
// but it fails with message `"/src\\test\\first copy.txt" is not parseable as an URI`
#[gpui::test]
#[cfg_attr(target_os = "windows", ignore)]
async fn test_create_duplicate_items_and_check_history(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/src",
        json!({
            "test": {
                "first.txt": "// First Txt file",
                "second.txt": "// Second Txt file",
                "third.txt": "// Third Txt file",
            }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/src".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace
        .update(cx, |workspace, window, cx| {
            let panel = ProjectPanel::new(workspace, window, cx);
            workspace.add_panel(panel.clone(), window, cx);
            panel
        })
        .unwrap();
    cx.run_until_parked();

    select_path(&panel, "src", cx);
    panel.update_in(cx, |panel, window, cx| panel.confirm(&Confirm, window, cx));
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            //
            "v src  <== selected",
            "    > test"
        ]
    );
    panel.update_in(cx, |panel, window, cx| {
        panel.new_directory(&NewDirectory, window, cx)
    });
    cx.run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(panel.filename_editor.read(cx).is_focused(window));
    });
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            //
            "v src",
            "    > [EDITOR: '']  <== selected",
            "    > test"
        ]
    );
    panel.update_in(cx, |panel, window, cx| {
        panel
            .filename_editor
            .update(cx, |editor, cx| editor.set_text("test", window, cx));
        assert!(
            panel.confirm_edit(true, window, cx).is_none(),
            "Should not allow to confirm on conflicting new directory name"
        );
    });
    cx.executor().run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(
            panel.state.edit_state.is_some(),
            "Edit state should not be None after conflicting new directory name"
        );
        panel.cancel(&menu::Cancel, window, cx);
        panel.update_visible_entries(None, false, false, window, cx);
    });
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            //
            "v src  <== selected",
            "    > test"
        ],
        "File list should be unchanged after failed folder create confirmation"
    );

    select_path(&panel, "src/test", cx);
    panel.update_in(cx, |panel, window, cx| panel.confirm(&Confirm, window, cx));
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            //
            "v src",
            "    > test  <== selected"
        ]
    );
    panel.update_in(cx, |panel, window, cx| panel.new_file(&NewFile, window, cx));
    cx.run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(panel.filename_editor.read(cx).is_focused(window));
    });
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v src",
            "    v test",
            "          [EDITOR: '']  <== selected",
            "          first.txt",
            "          second.txt",
            "          third.txt"
        ]
    );
    panel.update_in(cx, |panel, window, cx| {
        panel
            .filename_editor
            .update(cx, |editor, cx| editor.set_text("first.txt", window, cx));
        assert!(
            panel.confirm_edit(true, window, cx).is_none(),
            "Should not allow to confirm on conflicting new file name"
        );
    });
    cx.executor().run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(
            panel.state.edit_state.is_some(),
            "Edit state should not be None after conflicting new file name"
        );
        panel.cancel(&menu::Cancel, window, cx);
        panel.update_visible_entries(None, false, false, window, cx);
    });
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v src",
            "    v test  <== selected",
            "          first.txt",
            "          second.txt",
            "          third.txt"
        ],
        "File list should be unchanged after failed file create confirmation"
    );

    select_path(&panel, "src/test/first.txt", cx);
    panel.update_in(cx, |panel, window, cx| panel.confirm(&Confirm, window, cx));
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v src",
            "    v test",
            "          first.txt  <== selected",
            "          second.txt",
            "          third.txt"
        ],
    );
    panel.update_in(cx, |panel, window, cx| panel.rename(&Rename, window, cx));
    cx.executor().run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(panel.filename_editor.read(cx).is_focused(window));
    });
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v src",
            "    v test",
            "          [EDITOR: 'first.txt']  <== selected",
            "          second.txt",
            "          third.txt"
        ]
    );
    panel.update_in(cx, |panel, window, cx| {
        panel
            .filename_editor
            .update(cx, |editor, cx| editor.set_text("second.txt", window, cx));
        assert!(
            panel.confirm_edit(true, window, cx).is_none(),
            "Should not allow to confirm on conflicting file rename"
        )
    });
    cx.executor().run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(
            panel.state.edit_state.is_some(),
            "Edit state should not be None after conflicting file rename"
        );
        panel.cancel(&menu::Cancel, window, cx);
    });
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v src",
            "    v test",
            "          first.txt  <== selected",
            "          second.txt",
            "          third.txt"
        ],
        "File list should be unchanged after failed rename confirmation"
    );
    panel.update_in(cx, |panel, window, cx| panel.open(&Open, window, cx));
    cx.executor().run_until_parked();
    // Try to duplicate and check history
    panel.update_in(cx, |panel, window, cx| {
        panel.duplicate(&Duplicate, window, cx)
    });
    cx.executor().run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v src",
            "    v test",
            "          first.txt",
            "          [EDITOR: 'first copy.txt']  <== selected  <== marked",
            "          second.txt",
            "          third.txt"
        ],
    );

    let confirm = panel.update_in(cx, |panel, window, cx| {
        panel
            .filename_editor
            .update(cx, |editor, cx| editor.set_text("fourth.txt", window, cx));
        panel.confirm_edit(true, window, cx).unwrap()
    });
    confirm.await.unwrap();
    cx.executor().run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v src",
            "    v test",
            "          first.txt",
            "          fourth.txt  <== selected",
            "          second.txt",
            "          third.txt"
        ],
        "File list should be different after rename confirmation"
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.update_visible_entries(None, false, false, window, cx);
    });
    cx.executor().run_until_parked();

    select_path(&panel, "src/test/first.txt", cx);
    panel.update_in(cx, |panel, window, cx| panel.open(&Open, window, cx));
    cx.executor().run_until_parked();

    workspace
        .read_with(cx, |this, cx| {
            assert!(
                this.recent_navigation_history_iter(cx)
                    .any(|(project_path, abs_path)| {
                        project_path.path == Arc::from(rel_path("test/fourth.txt"))
                            && abs_path == Some(PathBuf::from(path!("/src/test/fourth.txt")))
                    })
            );
        })
        .unwrap();
}

// NOTE: This test is skipped on Windows, because on Windows,
// when it triggers the lsp store it converts `/src/test/first.txt` into an uri
// but it fails with message `"/src\\test\\first.txt" is not parseable as an URI`
#[gpui::test]
#[cfg_attr(target_os = "windows", ignore)]
async fn test_rename_item_and_check_history(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/src",
        json!({
            "test": {
                "first.txt": "// First Txt file",
                "second.txt": "// Second Txt file",
                "third.txt": "// Third Txt file",
            }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/src".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace
        .update(cx, |workspace, window, cx| {
            let panel = ProjectPanel::new(workspace, window, cx);
            workspace.add_panel(panel.clone(), window, cx);
            panel
        })
        .unwrap();
    cx.run_until_parked();

    select_path(&panel, "src", cx);
    panel.update_in(cx, |panel, window, cx| panel.confirm(&Confirm, window, cx));
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            //
            "v src  <== selected",
            "    > test"
        ]
    );

    select_path(&panel, "src/test", cx);
    panel.update_in(cx, |panel, window, cx| panel.confirm(&Confirm, window, cx));
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            //
            "v src",
            "    > test  <== selected"
        ]
    );
    panel.update_in(cx, |panel, window, cx| panel.new_file(&NewFile, window, cx));
    cx.run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(panel.filename_editor.read(cx).is_focused(window));
    });

    select_path(&panel, "src/test/first.txt", cx);
    panel.update_in(cx, |panel, window, cx| panel.open(&Open, window, cx));
    cx.executor().run_until_parked();

    panel.update_in(cx, |panel, window, cx| panel.rename(&Rename, window, cx));
    cx.executor().run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v src",
            "    v test",
            "          [EDITOR: 'first.txt']  <== selected  <== marked",
            "          second.txt",
            "          third.txt"
        ],
    );

    let confirm = panel.update_in(cx, |panel, window, cx| {
        panel
            .filename_editor
            .update(cx, |editor, cx| editor.set_text("fourth.txt", window, cx));
        panel.confirm_edit(true, window, cx).unwrap()
    });
    confirm.await.unwrap();
    cx.executor().run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v src",
            "    v test",
            "          fourth.txt  <== selected",
            "          second.txt",
            "          third.txt"
        ],
        "File list should be different after rename confirmation"
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.update_visible_entries(None, false, false, window, cx);
    });
    cx.executor().run_until_parked();

    select_path(&panel, "src/test/second.txt", cx);
    panel.update_in(cx, |panel, window, cx| panel.open(&Open, window, cx));
    cx.executor().run_until_parked();

    workspace
        .read_with(cx, |this, cx| {
            assert!(
                this.recent_navigation_history_iter(cx)
                    .any(|(project_path, abs_path)| {
                        project_path.path == Arc::from(rel_path("test/fourth.txt"))
                            && abs_path == Some(PathBuf::from(path!("/src/test/fourth.txt")))
                    })
            );
        })
        .unwrap();
}

#[gpui::test]
async fn test_select_git_entry(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            "tree1": {
                ".git": {},
                "dir1": {
                    "modified1.txt": "1",
                    "unmodified1.txt": "1",
                    "modified2.txt": "1",
                },
                "dir2": {
                    "modified3.txt": "1",
                    "unmodified2.txt": "1",
                },
                "modified4.txt": "1",
                "unmodified3.txt": "1",
            },
            "tree2": {
                ".git": {},
                "dir3": {
                    "modified5.txt": "1",
                    "unmodified4.txt": "1",
                },
                "modified6.txt": "1",
                "unmodified5.txt": "1",
            }
        }),
    )
    .await;

    // Mark files as git modified
    fs.set_head_and_index_for_repo(
        path!("/root/tree1/.git").as_ref(),
        &[
            ("dir1/modified1.txt", "modified".into()),
            ("dir1/modified2.txt", "modified".into()),
            ("modified4.txt", "modified".into()),
            ("dir2/modified3.txt", "modified".into()),
        ],
    );
    fs.set_head_and_index_for_repo(
        path!("/root/tree2/.git").as_ref(),
        &[
            ("dir3/modified5.txt", "modified".into()),
            ("modified6.txt", "modified".into()),
        ],
    );

    let project = Project::test(
        fs.clone(),
        [path!("/root/tree1").as_ref(), path!("/root/tree2").as_ref()],
        cx,
    )
    .await;

    let (scan1_complete, scan2_complete) = project.update(cx, |project, cx| {
        let mut worktrees = project.worktrees(cx);
        let worktree1 = worktrees.next().unwrap();
        let worktree2 = worktrees.next().unwrap();
        (
            worktree1.read(cx).as_local().unwrap().scan_complete(),
            worktree2.read(cx).as_local().unwrap().scan_complete(),
        )
    });
    scan1_complete.await;
    scan2_complete.await;
    cx.run_until_parked();

    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    // Check initial state
    assert_eq!(
        visible_entries_as_strings(&panel, 0..15, cx),
        &[
            "v tree1",
            "    > .git",
            "    > dir1",
            "    > dir2",
            "      modified4.txt",
            "      unmodified3.txt",
            "v tree2",
            "    > .git",
            "    > dir3",
            "      modified6.txt",
            "      unmodified5.txt"
        ],
    );

    // Test selecting next modified entry
    panel.update_in(cx, |panel, window, cx| {
        panel.select_next_git_entry(&SelectNextGitEntry, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..6, cx),
        &[
            "v tree1",
            "    > .git",
            "    v dir1",
            "          modified1.txt  <== selected",
            "          modified2.txt",
            "          unmodified1.txt",
        ],
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.select_next_git_entry(&SelectNextGitEntry, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..6, cx),
        &[
            "v tree1",
            "    > .git",
            "    v dir1",
            "          modified1.txt",
            "          modified2.txt  <== selected",
            "          unmodified1.txt",
        ],
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.select_next_git_entry(&SelectNextGitEntry, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 6..9, cx),
        &[
            "    v dir2",
            "          modified3.txt  <== selected",
            "          unmodified2.txt",
        ],
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.select_next_git_entry(&SelectNextGitEntry, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 9..11, cx),
        &["      modified4.txt  <== selected", "      unmodified3.txt",],
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.select_next_git_entry(&SelectNextGitEntry, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 13..16, cx),
        &[
            "    v dir3",
            "          modified5.txt  <== selected",
            "          unmodified4.txt",
        ],
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.select_next_git_entry(&SelectNextGitEntry, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 16..18, cx),
        &["      modified6.txt  <== selected", "      unmodified5.txt",],
    );

    // Wraps around to first modified file
    panel.update_in(cx, |panel, window, cx| {
        panel.select_next_git_entry(&SelectNextGitEntry, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..18, cx),
        &[
            "v tree1",
            "    > .git",
            "    v dir1",
            "          modified1.txt  <== selected",
            "          modified2.txt",
            "          unmodified1.txt",
            "    v dir2",
            "          modified3.txt",
            "          unmodified2.txt",
            "      modified4.txt",
            "      unmodified3.txt",
            "v tree2",
            "    > .git",
            "    v dir3",
            "          modified5.txt",
            "          unmodified4.txt",
            "      modified6.txt",
            "      unmodified5.txt",
        ],
    );

    // Wraps around again to last modified file
    panel.update_in(cx, |panel, window, cx| {
        panel.select_prev_git_entry(&SelectPrevGitEntry, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 16..18, cx),
        &["      modified6.txt  <== selected", "      unmodified5.txt",],
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.select_prev_git_entry(&SelectPrevGitEntry, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 13..16, cx),
        &[
            "    v dir3",
            "          modified5.txt  <== selected",
            "          unmodified4.txt",
        ],
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.select_prev_git_entry(&SelectPrevGitEntry, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 9..11, cx),
        &["      modified4.txt  <== selected", "      unmodified3.txt",],
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.select_prev_git_entry(&SelectPrevGitEntry, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 6..9, cx),
        &[
            "    v dir2",
            "          modified3.txt  <== selected",
            "          unmodified2.txt",
        ],
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.select_prev_git_entry(&SelectPrevGitEntry, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..6, cx),
        &[
            "v tree1",
            "    > .git",
            "    v dir1",
            "          modified1.txt",
            "          modified2.txt  <== selected",
            "          unmodified1.txt",
        ],
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.select_prev_git_entry(&SelectPrevGitEntry, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..6, cx),
        &[
            "v tree1",
            "    > .git",
            "    v dir1",
            "          modified1.txt  <== selected",
            "          modified2.txt",
            "          unmodified1.txt",
        ],
    );
}

#[gpui::test]
async fn test_select_directory(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/project_root",
        json!({
            "dir_1": {
                "nested_dir": {
                    "file_a.py": "# File contents",
                }
            },
            "file_1.py": "# File contents",
            "dir_2": {

            },
            "dir_3": {

            },
            "file_2.py": "# File contents",
            "dir_4": {

            },
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/project_root".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    panel.update_in(cx, |panel, window, cx| panel.open(&Open, window, cx));
    cx.executor().run_until_parked();
    select_path(&panel, "project_root/dir_1", cx);
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v project_root",
            "    > dir_1  <== selected",
            "    > dir_2",
            "    > dir_3",
            "    > dir_4",
            "      file_1.py",
            "      file_2.py",
        ]
    );
    panel.update_in(cx, |panel, window, cx| {
        panel.select_prev_directory(&SelectPrevDirectory, window, cx)
    });

    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v project_root  <== selected",
            "    > dir_1",
            "    > dir_2",
            "    > dir_3",
            "    > dir_4",
            "      file_1.py",
            "      file_2.py",
        ]
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.select_prev_directory(&SelectPrevDirectory, window, cx)
    });

    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v project_root",
            "    > dir_1",
            "    > dir_2",
            "    > dir_3",
            "    > dir_4  <== selected",
            "      file_1.py",
            "      file_2.py",
        ]
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.select_next_directory(&SelectNextDirectory, window, cx)
    });

    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v project_root  <== selected",
            "    > dir_1",
            "    > dir_2",
            "    > dir_3",
            "    > dir_4",
            "      file_1.py",
            "      file_2.py",
        ]
    );
}

#[gpui::test]
async fn test_select_first_last(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/project_root",
        json!({
            "dir_1": {
                "nested_dir": {
                    "file_a.py": "# File contents",
                }
            },
            "file_1.py": "# File contents",
            "file_2.py": "# File contents",
            "zdir_2": {
                "nested_dir2": {
                    "file_b.py": "# File contents",
                }
            },
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/project_root".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v project_root",
            "    > dir_1",
            "    > zdir_2",
            "      file_1.py",
            "      file_2.py",
        ]
    );
    panel.update_in(cx, |panel, window, cx| {
        panel.select_first(&SelectFirst, window, cx)
    });

    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v project_root  <== selected",
            "    > dir_1",
            "    > zdir_2",
            "      file_1.py",
            "      file_2.py",
        ]
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.select_last(&SelectLast, window, cx)
    });

    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v project_root",
            "    > dir_1",
            "    > zdir_2",
            "      file_1.py",
            "      file_2.py  <== selected",
        ]
    );

    cx.update(|_, cx| {
        let settings = *ProjectPanelSettings::get_global(cx);
        ProjectPanelSettings::override_global(
            ProjectPanelSettings {
                hide_root: true,
                ..settings
            },
            cx,
        );
    });

    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    #[rustfmt::skip]
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "> dir_1",
            "> zdir_2",
            "  file_1.py",
            "  file_2.py",
        ],
        "With hide_root=true, root should be hidden"
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.select_first(&SelectFirst, window, cx)
    });

    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "> dir_1  <== selected",
            "> zdir_2",
            "  file_1.py",
            "  file_2.py",
        ],
        "With hide_root=true, first entry should be dir_1, not the hidden root"
    );
}

#[gpui::test]
async fn test_dir_toggle_collapse(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/project_root",
        json!({
            "dir_1": {
                "nested_dir": {
                    "file_a.py": "# File contents",
                }
            },
            "file_1.py": "# File contents",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/project_root".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    panel.update_in(cx, |panel, window, cx| panel.open(&Open, window, cx));
    cx.executor().run_until_parked();
    select_path(&panel, "project_root/dir_1", cx);
    panel.update_in(cx, |panel, window, cx| panel.open(&Open, window, cx));
    select_path(&panel, "project_root/dir_1/nested_dir", cx);
    panel.update_in(cx, |panel, window, cx| panel.open(&Open, window, cx));
    panel.update_in(cx, |panel, window, cx| panel.open(&Open, window, cx));
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v project_root",
            "    v dir_1",
            "        > nested_dir  <== selected",
            "      file_1.py",
        ]
    );
}

#[gpui::test]
async fn test_collapse_all_entries(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/project_root",
        json!({
            "dir_1": {
                "nested_dir": {
                    "file_a.py": "# File contents",
                    "file_b.py": "# File contents",
                    "file_c.py": "# File contents",
                },
                "file_1.py": "# File contents",
                "file_2.py": "# File contents",
                "file_3.py": "# File contents",
            },
            "dir_2": {
                "file_1.py": "# File contents",
                "file_2.py": "# File contents",
                "file_3.py": "# File contents",
            }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/project_root".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    panel.update_in(cx, |panel, window, cx| {
        panel.collapse_all_entries(&CollapseAllEntries, window, cx)
    });
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &["v project_root", "    > dir_1", "    > dir_2",]
    );

    // Open dir_1 and make sure nested_dir was collapsed when running collapse_all_entries
    toggle_expand_dir(&panel, "project_root/dir_1", cx);
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v project_root",
            "    v dir_1  <== selected",
            "        > nested_dir",
            "          file_1.py",
            "          file_2.py",
            "          file_3.py",
            "    > dir_2",
        ]
    );
}

#[gpui::test]
async fn test_collapse_all_entries_multiple_worktrees(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    let worktree_content = json!({
        "dir_1": {
            "file_1.py": "# File contents",
        },
        "dir_2": {
            "file_1.py": "# File contents",
        }
    });

    fs.insert_tree("/project_root_1", worktree_content.clone())
        .await;
    fs.insert_tree("/project_root_2", worktree_content).await;

    let project = Project::test(
        fs.clone(),
        ["/project_root_1".as_ref(), "/project_root_2".as_ref()],
        cx,
    )
    .await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    panel.update_in(cx, |panel, window, cx| {
        panel.collapse_all_entries(&CollapseAllEntries, window, cx)
    });
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &["> project_root_1", "> project_root_2",]
    );
}

#[gpui::test]
async fn test_collapse_all_entries_with_collapsed_root(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/project_root",
        json!({
            "dir_1": {
                "nested_dir": {
                    "file_a.py": "# File contents",
                    "file_b.py": "# File contents",
                    "file_c.py": "# File contents",
                },
                "file_1.py": "# File contents",
                "file_2.py": "# File contents",
                "file_3.py": "# File contents",
            },
            "dir_2": {
                "file_1.py": "# File contents",
                "file_2.py": "# File contents",
                "file_3.py": "# File contents",
            }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/project_root".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    // Open project_root/dir_1 to ensure that a nested directory is expanded
    toggle_expand_dir(&panel, "project_root/dir_1", cx);
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v project_root",
            "    v dir_1  <== selected",
            "        > nested_dir",
            "          file_1.py",
            "          file_2.py",
            "          file_3.py",
            "    > dir_2",
        ]
    );

    // Close root directory
    toggle_expand_dir(&panel, "project_root", cx);
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &["> project_root  <== selected"]
    );

    // Run collapse_all_entries and make sure root is not expanded
    panel.update_in(cx, |panel, window, cx| {
        panel.collapse_all_entries(&CollapseAllEntries, window, cx)
    });
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &["> project_root  <== selected"]
    );
}

#[gpui::test]
async fn test_new_file_move(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.as_fake().insert_tree(path!("/root"), json!({})).await;
    let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    // Make a new buffer with no backing file
    workspace
        .update(cx, |workspace, window, cx| {
            Editor::new_file(workspace, &Default::default(), window, cx)
        })
        .unwrap();

    cx.executor().run_until_parked();

    // "Save as" the buffer, creating a new backing file for it
    let save_task = workspace
        .update(cx, |workspace, window, cx| {
            workspace.save_active_item(workspace::SaveIntent::Save, window, cx)
        })
        .unwrap();

    cx.executor().run_until_parked();
    cx.simulate_new_path_selection(|_| Some(PathBuf::from(path!("/root/new"))));
    save_task.await.unwrap();

    // Rename the file
    select_path(&panel, "root/new", cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &["v root", "      new  <== selected  <== marked"]
    );
    panel.update_in(cx, |panel, window, cx| panel.rename(&Rename, window, cx));
    panel.update_in(cx, |panel, window, cx| {
        panel
            .filename_editor
            .update(cx, |editor, cx| editor.set_text("newer", window, cx));
    });
    panel.update_in(cx, |panel, window, cx| panel.confirm(&Confirm, window, cx));

    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &["v root", "      newer  <== selected"]
    );

    workspace
        .update(cx, |workspace, window, cx| {
            workspace.save_active_item(workspace::SaveIntent::Save, window, cx)
        })
        .unwrap()
        .await
        .unwrap();

    cx.executor().run_until_parked();
    // assert that saving the file doesn't restore "new"
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &["v root", "      newer  <== selected"]
    );
}

// NOTE: This test is skipped on Windows, because on Windows, unlike on Unix,
// you can't rename a directory which some program has already open. This is a
// limitation of the Windows. Since Zed will have the root open, it will hold an open handle
// to it, and thus renaming it will fail on Windows.
// See: https://stackoverflow.com/questions/41365318/access-is-denied-when-renaming-folder
// See: https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/ns-ntifs-_file_rename_information
#[gpui::test]
#[cfg_attr(target_os = "windows", ignore)]
async fn test_rename_root_of_worktree(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root1",
        json!({
            "dir1": {
                "file1.txt": "content 1",
            },
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root1".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    toggle_expand_dir(&panel, "root1/dir1", cx);

    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &["v root1", "    v dir1  <== selected", "          file1.txt",],
        "Initial state with worktrees"
    );

    select_path(&panel, "root1", cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &["v root1  <== selected", "    v dir1", "          file1.txt",],
    );

    // Rename root1 to new_root1
    panel.update_in(cx, |panel, window, cx| panel.rename(&Rename, window, cx));

    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v [EDITOR: 'root1']  <== selected",
            "    v dir1",
            "          file1.txt",
        ],
    );

    let confirm = panel.update_in(cx, |panel, window, cx| {
        panel
            .filename_editor
            .update(cx, |editor, cx| editor.set_text("new_root1", window, cx));
        panel.confirm_edit(true, window, cx).unwrap()
    });
    confirm.await.unwrap();
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v new_root1  <== selected",
            "    v dir1",
            "          file1.txt",
        ],
        "Should update worktree name"
    );

    // Ensure internal paths have been updated
    select_path(&panel, "new_root1/dir1/file1.txt", cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v new_root1",
            "    v dir1",
            "          file1.txt  <== selected",
        ],
        "Files in renamed worktree are selectable"
    );
}

#[gpui::test]
async fn test_rename_with_hide_root(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root1",
        json!({
            "dir1": { "file1.txt": "content" },
            "file2.txt": "content",
        }),
    )
    .await;
    fs.insert_tree("/root2", json!({ "file3.txt": "content" }))
        .await;

    // Test 1: Single worktree, hide_root=true - rename should be blocked
    {
        let project = Project::test(fs.clone(), ["/root1".as_ref()], cx).await;
        let workspace =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);

        cx.update(|_, cx| {
            let settings = *ProjectPanelSettings::get_global(cx);
            ProjectPanelSettings::override_global(
                ProjectPanelSettings {
                    hide_root: true,
                    ..settings
                },
                cx,
            );
        });

        let panel = workspace.update(cx, ProjectPanel::new).unwrap();
        cx.run_until_parked();

        panel.update(cx, |panel, cx| {
            let project = panel.project.read(cx);
            let worktree = project.visible_worktrees(cx).next().unwrap();
            let root_entry = worktree.read(cx).root_entry().unwrap();
            panel.state.selection = Some(SelectedEntry {
                worktree_id: worktree.read(cx).id(),
                entry_id: root_entry.id,
            });
        });

        panel.update_in(cx, |panel, window, cx| panel.rename(&Rename, window, cx));

        assert!(
            panel.read_with(cx, |panel, _| panel.state.edit_state.is_none()),
            "Rename should be blocked when hide_root=true with single worktree"
        );
    }

    // Test 2: Multiple worktrees, hide_root=true - rename should work
    {
        let project = Project::test(fs.clone(), ["/root1".as_ref(), "/root2".as_ref()], cx).await;
        let workspace =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);

        cx.update(|_, cx| {
            let settings = *ProjectPanelSettings::get_global(cx);
            ProjectPanelSettings::override_global(
                ProjectPanelSettings {
                    hide_root: true,
                    ..settings
                },
                cx,
            );
        });

        let panel = workspace.update(cx, ProjectPanel::new).unwrap();
        cx.run_until_parked();

        select_path(&panel, "root1", cx);
        panel.update_in(cx, |panel, window, cx| panel.rename(&Rename, window, cx));

        #[cfg(target_os = "windows")]
        assert!(
            panel.read_with(cx, |panel, _| panel.state.edit_state.is_none()),
            "Rename should be blocked on Windows even with multiple worktrees"
        );

        #[cfg(not(target_os = "windows"))]
        {
            assert!(
                panel.read_with(cx, |panel, _| panel.state.edit_state.is_some()),
                "Rename should work with multiple worktrees on non-Windows when hide_root=true"
            );
            panel.update_in(cx, |panel, window, cx| {
                panel.cancel(&menu::Cancel, window, cx)
            });
        }
    }
}

#[gpui::test]
async fn test_multiple_marked_entries(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/project_root",
        json!({
            "dir_1": {
                "nested_dir": {
                    "file_a.py": "# File contents",
                }
            },
            "file_1.py": "# File contents",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/project_root".as_ref()], cx).await;
    let worktree_id = cx.update(|cx| project.read(cx).worktrees(cx).next().unwrap().read(cx).id());
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    cx.update(|window, cx| {
        panel.update(cx, |this, cx| {
            this.select_next(&Default::default(), window, cx);
            this.expand_selected_entry(&Default::default(), window, cx);
        })
    });
    cx.run_until_parked();

    cx.update(|window, cx| {
        panel.update(cx, |this, cx| {
            this.expand_selected_entry(&Default::default(), window, cx);
        })
    });
    cx.run_until_parked();

    cx.update(|window, cx| {
        panel.update(cx, |this, cx| {
            this.select_next(&Default::default(), window, cx);
            this.expand_selected_entry(&Default::default(), window, cx);
        })
    });
    cx.run_until_parked();

    cx.update(|window, cx| {
        panel.update(cx, |this, cx| {
            this.select_next(&Default::default(), window, cx);
        })
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v project_root",
            "    v dir_1",
            "        v nested_dir",
            "              file_a.py  <== selected",
            "      file_1.py",
        ]
    );
    let modifiers_with_shift = gpui::Modifiers {
        shift: true,
        ..Default::default()
    };
    cx.run_until_parked();
    cx.simulate_modifiers_change(modifiers_with_shift);
    cx.update(|window, cx| {
        panel.update(cx, |this, cx| {
            this.select_next(&Default::default(), window, cx);
        })
    });
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v project_root",
            "    v dir_1",
            "        v nested_dir",
            "              file_a.py",
            "      file_1.py  <== selected  <== marked",
        ]
    );
    cx.update(|window, cx| {
        panel.update(cx, |this, cx| {
            this.select_previous(&Default::default(), window, cx);
        })
    });
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v project_root",
            "    v dir_1",
            "        v nested_dir",
            "              file_a.py  <== selected  <== marked",
            "      file_1.py  <== marked",
        ]
    );
    cx.update(|window, cx| {
        panel.update(cx, |this, cx| {
            let drag = DraggedSelection {
                active_selection: this.state.selection.unwrap(),
                marked_selections: this.marked_entries.clone().into(),
            };
            let target_entry = this
                .project
                .read(cx)
                .entry_for_path(&(worktree_id, rel_path("")).into(), cx)
                .unwrap();
            this.drag_onto(&drag, target_entry.id, false, window, cx);
        });
    });
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v project_root",
            "    v dir_1",
            "        v nested_dir",
            "      file_1.py  <== marked",
            "      file_a.py  <== selected  <== marked",
        ]
    );
    // ESC clears out all marks
    cx.update(|window, cx| {
        panel.update(cx, |this, cx| {
            this.cancel(&menu::Cancel, window, cx);
        })
    });
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v project_root",
            "    v dir_1",
            "        v nested_dir",
            "      file_1.py",
            "      file_a.py  <== selected",
        ]
    );
    // ESC clears out all marks
    cx.update(|window, cx| {
        panel.update(cx, |this, cx| {
            this.select_previous(&SelectPrevious, window, cx);
            this.select_next(&SelectNext, window, cx);
        })
    });
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v project_root",
            "    v dir_1",
            "        v nested_dir",
            "      file_1.py  <== marked",
            "      file_a.py  <== selected  <== marked",
        ]
    );
    cx.simulate_modifiers_change(Default::default());
    cx.update(|window, cx| {
        panel.update(cx, |this, cx| {
            this.cut(&Cut, window, cx);
            this.select_previous(&SelectPrevious, window, cx);
            this.select_previous(&SelectPrevious, window, cx);

            this.paste(&Paste, window, cx);
            this.update_visible_entries(None, false, false, window, cx);
        })
    });
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v project_root",
            "    v dir_1",
            "        v nested_dir",
            "              file_1.py  <== marked",
            "              file_a.py  <== selected  <== marked",
        ]
    );
    cx.simulate_modifiers_change(modifiers_with_shift);
    cx.update(|window, cx| {
        panel.update(cx, |this, cx| {
            this.expand_selected_entry(&Default::default(), window, cx);
            this.select_next(&SelectNext, window, cx);
            this.select_next(&SelectNext, window, cx);
        })
    });
    submit_deletion(&panel, cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v project_root",
            "    v dir_1",
            "        v nested_dir  <== selected",
        ]
    );
}

#[gpui::test]
async fn test_dragged_selection_resolve_entry(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "a": {
                "b": {
                    "c": {
                        "d": {}
                    }
                }
            },
            "target_destination": {}
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    cx.update(|_, cx| {
        let settings = *ProjectPanelSettings::get_global(cx);
        ProjectPanelSettings::override_global(
            ProjectPanelSettings {
                auto_fold_dirs: true,
                ..settings
            },
            cx,
        );
    });

    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    // Case 1: Move last dir 'd' - should move only 'd', leaving 'a/b/c'
    select_path(&panel, "root/a/b/c/d", cx);
    panel.update_in(cx, |panel, window, cx| {
        let drag = DraggedSelection {
            active_selection: SelectedEntry {
                worktree_id: panel.state.selection.as_ref().unwrap().worktree_id,
                entry_id: panel.resolve_entry(panel.state.selection.as_ref().unwrap().entry_id),
            },
            marked_selections: Arc::new([*panel.state.selection.as_ref().unwrap()]),
        };
        let target_entry = panel
            .project
            .read(cx)
            .visible_worktrees(cx)
            .next()
            .unwrap()
            .read(cx)
            .entry_for_path(rel_path("target_destination"))
            .unwrap();
        panel.drag_onto(&drag, target_entry.id, false, window, cx);
    });
    cx.executor().run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root",
            "    > a/b/c",
            "    > target_destination/d  <== selected"
        ],
        "Moving last empty directory 'd' should leave 'a/b/c' and move only 'd'"
    );

    // Reset
    select_path(&panel, "root/target_destination/d", cx);
    panel.update_in(cx, |panel, window, cx| {
        let drag = DraggedSelection {
            active_selection: SelectedEntry {
                worktree_id: panel.state.selection.as_ref().unwrap().worktree_id,
                entry_id: panel.resolve_entry(panel.state.selection.as_ref().unwrap().entry_id),
            },
            marked_selections: Arc::new([*panel.state.selection.as_ref().unwrap()]),
        };
        let target_entry = panel
            .project
            .read(cx)
            .visible_worktrees(cx)
            .next()
            .unwrap()
            .read(cx)
            .entry_for_path(rel_path("a/b/c"))
            .unwrap();
        panel.drag_onto(&drag, target_entry.id, false, window, cx);
    });
    cx.executor().run_until_parked();

    // Case 2: Move middle dir 'b' - should move 'b/c/d', leaving only 'a'
    select_path(&panel, "root/a/b", cx);
    panel.update_in(cx, |panel, window, cx| {
        let drag = DraggedSelection {
            active_selection: SelectedEntry {
                worktree_id: panel.state.selection.as_ref().unwrap().worktree_id,
                entry_id: panel.resolve_entry(panel.state.selection.as_ref().unwrap().entry_id),
            },
            marked_selections: Arc::new([*panel.state.selection.as_ref().unwrap()]),
        };
        let target_entry = panel
            .project
            .read(cx)
            .visible_worktrees(cx)
            .next()
            .unwrap()
            .read(cx)
            .entry_for_path(rel_path("target_destination"))
            .unwrap();
        panel.drag_onto(&drag, target_entry.id, false, window, cx);
    });
    cx.executor().run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &["v root", "    v a", "    > target_destination/b/c/d"],
        "Moving middle directory 'b' should leave only 'a' and move 'b/c/d'"
    );

    // Reset
    select_path(&panel, "root/target_destination/b", cx);
    panel.update_in(cx, |panel, window, cx| {
        let drag = DraggedSelection {
            active_selection: SelectedEntry {
                worktree_id: panel.state.selection.as_ref().unwrap().worktree_id,
                entry_id: panel.resolve_entry(panel.state.selection.as_ref().unwrap().entry_id),
            },
            marked_selections: Arc::new([*panel.state.selection.as_ref().unwrap()]),
        };
        let target_entry = panel
            .project
            .read(cx)
            .visible_worktrees(cx)
            .next()
            .unwrap()
            .read(cx)
            .entry_for_path(rel_path("a"))
            .unwrap();
        panel.drag_onto(&drag, target_entry.id, false, window, cx);
    });
    cx.executor().run_until_parked();

    // Case 3: Move first dir 'a' - should move whole 'a/b/c/d'
    select_path(&panel, "root/a", cx);
    panel.update_in(cx, |panel, window, cx| {
        let drag = DraggedSelection {
            active_selection: SelectedEntry {
                worktree_id: panel.state.selection.as_ref().unwrap().worktree_id,
                entry_id: panel.resolve_entry(panel.state.selection.as_ref().unwrap().entry_id),
            },
            marked_selections: Arc::new([*panel.state.selection.as_ref().unwrap()]),
        };
        let target_entry = panel
            .project
            .read(cx)
            .visible_worktrees(cx)
            .next()
            .unwrap()
            .read(cx)
            .entry_for_path(rel_path("target_destination"))
            .unwrap();
        panel.drag_onto(&drag, target_entry.id, false, window, cx);
    });
    cx.executor().run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &["v root", "    > target_destination/a/b/c/d"],
        "Moving first directory 'a' should move whole 'a/b/c/d' chain"
    );
}

#[gpui::test]
async fn test_drag_entries_between_different_worktrees(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root_a",
        json!({
            "src": {
                "lib.rs": "",
                "main.rs": ""
            },
            "docs": {
                "guide.md": ""
            },
            "multi": {
                "alpha.txt": "",
                "beta.txt": ""
            }
        }),
    )
    .await;
    fs.insert_tree(
        "/root_b",
        json!({
            "dst": {
                "existing.md": ""
            },
            "target.txt": ""
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root_a".as_ref(), "/root_b".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    // Case 1: move a file onto a directory in another worktree.
    select_path(&panel, "root_a/src/main.rs", cx);
    drag_selection_to(&panel, "root_b/dst", false, cx);
    assert!(
        find_project_entry(&panel, "root_b/dst/main.rs", cx).is_some(),
        "Dragged file should appear under destination worktree"
    );
    assert_eq!(
        find_project_entry(&panel, "root_a/src/main.rs", cx),
        None,
        "Dragged file should be removed from the source worktree"
    );

    // Case 2: drop a file onto another worktree file so it lands in the parent directory.
    select_path(&panel, "root_a/docs/guide.md", cx);
    drag_selection_to(&panel, "root_b/dst/existing.md", true, cx);
    assert!(
        find_project_entry(&panel, "root_b/dst/guide.md", cx).is_some(),
        "Dropping onto a file should place the entry beside the target file"
    );
    assert_eq!(
        find_project_entry(&panel, "root_a/docs/guide.md", cx),
        None,
        "Source file should be removed after the move"
    );

    // Case 3: move an entire directory.
    select_path(&panel, "root_a/src", cx);
    drag_selection_to(&panel, "root_b/dst", false, cx);
    assert!(
        find_project_entry(&panel, "root_b/dst/src/lib.rs", cx).is_some(),
        "Dragging a directory should move its nested contents"
    );
    assert_eq!(
        find_project_entry(&panel, "root_a/src", cx),
        None,
        "Directory should no longer exist in the source worktree"
    );

    // Case 4: multi-selection drag between worktrees.
    panel.update(cx, |panel, _| panel.marked_entries.clear());
    select_path_with_mark(&panel, "root_a/multi/alpha.txt", cx);
    select_path_with_mark(&panel, "root_a/multi/beta.txt", cx);
    drag_selection_to(&panel, "root_b/dst", false, cx);
    assert!(
        find_project_entry(&panel, "root_b/dst/alpha.txt", cx).is_some()
            && find_project_entry(&panel, "root_b/dst/beta.txt", cx).is_some(),
        "All marked entries should move to the destination worktree"
    );
    assert_eq!(
        find_project_entry(&panel, "root_a/multi/alpha.txt", cx),
        None,
        "Marked entries should be removed from the origin worktree"
    );
    assert_eq!(
        find_project_entry(&panel, "root_a/multi/beta.txt", cx),
        None,
        "Marked entries should be removed from the origin worktree"
    );
}

#[gpui::test]
async fn test_autoreveal_and_gitignored_files(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);
    cx.update(|cx| {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.worktree.file_scan_exclusions = Some(Vec::new());
                settings
                    .project_panel
                    .get_or_insert_default()
                    .auto_reveal_entries = Some(false);
            });
        })
    });

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/project_root",
        json!({
            ".git": {},
            ".gitignore": "**/gitignored_dir",
            "dir_1": {
                "file_1.py": "# File 1_1 contents",
                "file_2.py": "# File 1_2 contents",
                "file_3.py": "# File 1_3 contents",
                "gitignored_dir": {
                    "file_a.py": "# File contents",
                    "file_b.py": "# File contents",
                    "file_c.py": "# File contents",
                },
            },
            "dir_2": {
                "file_1.py": "# File 2_1 contents",
                "file_2.py": "# File 2_2 contents",
                "file_3.py": "# File 2_3 contents",
            }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/project_root".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v project_root",
            "    > .git",
            "    > dir_1",
            "    > dir_2",
            "      .gitignore",
        ]
    );

    let dir_1_file = find_project_entry(&panel, "project_root/dir_1/file_1.py", cx)
        .expect("dir 1 file is not ignored and should have an entry");
    let dir_2_file = find_project_entry(&panel, "project_root/dir_2/file_1.py", cx)
        .expect("dir 2 file is not ignored and should have an entry");
    let gitignored_dir_file =
        find_project_entry(&panel, "project_root/dir_1/gitignored_dir/file_a.py", cx);
    assert_eq!(
        gitignored_dir_file, None,
        "File in the gitignored dir should not have an entry before its dir is toggled"
    );

    toggle_expand_dir(&panel, "project_root/dir_1", cx);
    toggle_expand_dir(&panel, "project_root/dir_1/gitignored_dir", cx);
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v project_root",
            "    > .git",
            "    v dir_1",
            "        v gitignored_dir  <== selected",
            "              file_a.py",
            "              file_b.py",
            "              file_c.py",
            "          file_1.py",
            "          file_2.py",
            "          file_3.py",
            "    > dir_2",
            "      .gitignore",
        ],
        "Should show gitignored dir file list in the project panel"
    );
    let gitignored_dir_file =
        find_project_entry(&panel, "project_root/dir_1/gitignored_dir/file_a.py", cx)
            .expect("after gitignored dir got opened, a file entry should be present");

    toggle_expand_dir(&panel, "project_root/dir_1/gitignored_dir", cx);
    toggle_expand_dir(&panel, "project_root/dir_1", cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v project_root",
            "    > .git",
            "    > dir_1  <== selected",
            "    > dir_2",
            "      .gitignore",
        ],
        "Should hide all dir contents again and prepare for the auto reveal test"
    );

    for file_entry in [dir_1_file, dir_2_file, gitignored_dir_file] {
        panel.update(cx, |panel, cx| {
            panel.project.update(cx, |_, cx| {
                cx.emit(project::Event::ActiveEntryChanged(Some(file_entry)))
            })
        });
        cx.run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v project_root",
                "    > .git",
                "    > dir_1  <== selected",
                "    > dir_2",
                "      .gitignore",
            ],
            "When no auto reveal is enabled, the selected entry should not be revealed in the project panel"
        );
    }

    cx.update(|_, cx| {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings
                    .project_panel
                    .get_or_insert_default()
                    .auto_reveal_entries = Some(true)
            });
        })
    });

    panel.update(cx, |panel, cx| {
        panel.project.update(cx, |_, cx| {
            cx.emit(project::Event::ActiveEntryChanged(Some(dir_1_file)))
        })
    });
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v project_root",
            "    > .git",
            "    v dir_1",
            "        > gitignored_dir",
            "          file_1.py  <== selected  <== marked",
            "          file_2.py",
            "          file_3.py",
            "    > dir_2",
            "      .gitignore",
        ],
        "When auto reveal is enabled, not ignored dir_1 entry should be revealed"
    );

    panel.update(cx, |panel, cx| {
        panel.project.update(cx, |_, cx| {
            cx.emit(project::Event::ActiveEntryChanged(Some(dir_2_file)))
        })
    });
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v project_root",
            "    > .git",
            "    v dir_1",
            "        > gitignored_dir",
            "          file_1.py",
            "          file_2.py",
            "          file_3.py",
            "    v dir_2",
            "          file_1.py  <== selected  <== marked",
            "          file_2.py",
            "          file_3.py",
            "      .gitignore",
        ],
        "When auto reveal is enabled, not ignored dir_2 entry should be revealed"
    );

    panel.update(cx, |panel, cx| {
        panel.project.update(cx, |_, cx| {
            cx.emit(project::Event::ActiveEntryChanged(Some(
                gitignored_dir_file,
            )))
        })
    });
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v project_root",
            "    > .git",
            "    v dir_1",
            "        > gitignored_dir",
            "          file_1.py",
            "          file_2.py",
            "          file_3.py",
            "    v dir_2",
            "          file_1.py  <== selected  <== marked",
            "          file_2.py",
            "          file_3.py",
            "      .gitignore",
        ],
        "When auto reveal is enabled, a gitignored selected entry should not be revealed in the project panel"
    );

    panel.update(cx, |panel, cx| {
        panel.project.update(cx, |_, cx| {
            cx.emit(project::Event::RevealInProjectPanel(gitignored_dir_file))
        })
    });
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v project_root",
            "    > .git",
            "    v dir_1",
            "        v gitignored_dir",
            "              file_a.py  <== selected  <== marked",
            "              file_b.py",
            "              file_c.py",
            "          file_1.py",
            "          file_2.py",
            "          file_3.py",
            "    v dir_2",
            "          file_1.py",
            "          file_2.py",
            "          file_3.py",
            "      .gitignore",
        ],
        "When a gitignored entry is explicitly revealed, it should be shown in the project tree"
    );
}

#[gpui::test]
async fn test_gitignored_and_always_included(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);
    cx.update(|cx| {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.worktree.file_scan_exclusions = Some(Vec::new());
                settings.project.worktree.file_scan_inclusions =
                    Some(vec!["always_included_but_ignored_dir/*".to_string()]);
                settings
                    .project_panel
                    .get_or_insert_default()
                    .auto_reveal_entries = Some(false)
            });
        })
    });

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/project_root",
        json!({
            ".git": {},
            ".gitignore": "**/gitignored_dir\n/always_included_but_ignored_dir",
            "dir_1": {
                "file_1.py": "# File 1_1 contents",
                "file_2.py": "# File 1_2 contents",
                "file_3.py": "# File 1_3 contents",
                "gitignored_dir": {
                    "file_a.py": "# File contents",
                    "file_b.py": "# File contents",
                    "file_c.py": "# File contents",
                },
            },
            "dir_2": {
                "file_1.py": "# File 2_1 contents",
                "file_2.py": "# File 2_2 contents",
                "file_3.py": "# File 2_3 contents",
            },
            "always_included_but_ignored_dir": {
                "file_a.py": "# File contents",
                "file_b.py": "# File contents",
                "file_c.py": "# File contents",
            },
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/project_root".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v project_root",
            "    > .git",
            "    > always_included_but_ignored_dir",
            "    > dir_1",
            "    > dir_2",
            "      .gitignore",
        ]
    );

    let gitignored_dir_file =
        find_project_entry(&panel, "project_root/dir_1/gitignored_dir/file_a.py", cx);
    let always_included_but_ignored_dir_file = find_project_entry(
        &panel,
        "project_root/always_included_but_ignored_dir/file_a.py",
        cx,
    )
    .expect("file that is .gitignored but set to always be included should have an entry");
    assert_eq!(
        gitignored_dir_file, None,
        "File in the gitignored dir should not have an entry unless its directory is toggled"
    );

    toggle_expand_dir(&panel, "project_root/dir_1", cx);
    cx.run_until_parked();
    cx.update(|_, cx| {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings
                    .project_panel
                    .get_or_insert_default()
                    .auto_reveal_entries = Some(true)
            });
        })
    });

    panel.update(cx, |panel, cx| {
        panel.project.update(cx, |_, cx| {
            cx.emit(project::Event::ActiveEntryChanged(Some(
                always_included_but_ignored_dir_file,
            )))
        })
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v project_root",
            "    > .git",
            "    v always_included_but_ignored_dir",
            "          file_a.py  <== selected  <== marked",
            "          file_b.py",
            "          file_c.py",
            "    v dir_1",
            "        > gitignored_dir",
            "          file_1.py",
            "          file_2.py",
            "          file_3.py",
            "    > dir_2",
            "      .gitignore",
        ],
        "When auto reveal is enabled, a gitignored but always included selected entry should be revealed in the project panel"
    );
}

#[gpui::test]
async fn test_explicit_reveal(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);
    cx.update(|cx| {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.worktree.file_scan_exclusions = Some(Vec::new());
                settings
                    .project_panel
                    .get_or_insert_default()
                    .auto_reveal_entries = Some(false)
            });
        })
    });

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/project_root",
        json!({
            ".git": {},
            ".gitignore": "**/gitignored_dir",
            "dir_1": {
                "file_1.py": "# File 1_1 contents",
                "file_2.py": "# File 1_2 contents",
                "file_3.py": "# File 1_3 contents",
                "gitignored_dir": {
                    "file_a.py": "# File contents",
                    "file_b.py": "# File contents",
                    "file_c.py": "# File contents",
                },
            },
            "dir_2": {
                "file_1.py": "# File 2_1 contents",
                "file_2.py": "# File 2_2 contents",
                "file_3.py": "# File 2_3 contents",
            }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/project_root".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v project_root",
            "    > .git",
            "    > dir_1",
            "    > dir_2",
            "      .gitignore",
        ]
    );

    let dir_1_file = find_project_entry(&panel, "project_root/dir_1/file_1.py", cx)
        .expect("dir 1 file is not ignored and should have an entry");
    let dir_2_file = find_project_entry(&panel, "project_root/dir_2/file_1.py", cx)
        .expect("dir 2 file is not ignored and should have an entry");
    let gitignored_dir_file =
        find_project_entry(&panel, "project_root/dir_1/gitignored_dir/file_a.py", cx);
    assert_eq!(
        gitignored_dir_file, None,
        "File in the gitignored dir should not have an entry before its dir is toggled"
    );

    toggle_expand_dir(&panel, "project_root/dir_1", cx);
    toggle_expand_dir(&panel, "project_root/dir_1/gitignored_dir", cx);
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v project_root",
            "    > .git",
            "    v dir_1",
            "        v gitignored_dir  <== selected",
            "              file_a.py",
            "              file_b.py",
            "              file_c.py",
            "          file_1.py",
            "          file_2.py",
            "          file_3.py",
            "    > dir_2",
            "      .gitignore",
        ],
        "Should show gitignored dir file list in the project panel"
    );
    let gitignored_dir_file =
        find_project_entry(&panel, "project_root/dir_1/gitignored_dir/file_a.py", cx)
            .expect("after gitignored dir got opened, a file entry should be present");

    toggle_expand_dir(&panel, "project_root/dir_1/gitignored_dir", cx);
    toggle_expand_dir(&panel, "project_root/dir_1", cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v project_root",
            "    > .git",
            "    > dir_1  <== selected",
            "    > dir_2",
            "      .gitignore",
        ],
        "Should hide all dir contents again and prepare for the explicit reveal test"
    );

    for file_entry in [dir_1_file, dir_2_file, gitignored_dir_file] {
        panel.update(cx, |panel, cx| {
            panel.project.update(cx, |_, cx| {
                cx.emit(project::Event::ActiveEntryChanged(Some(file_entry)))
            })
        });
        cx.run_until_parked();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v project_root",
                "    > .git",
                "    > dir_1  <== selected",
                "    > dir_2",
                "      .gitignore",
            ],
            "When no auto reveal is enabled, the selected entry should not be revealed in the project panel"
        );
    }

    panel.update(cx, |panel, cx| {
        panel.project.update(cx, |_, cx| {
            cx.emit(project::Event::RevealInProjectPanel(dir_1_file))
        })
    });
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v project_root",
            "    > .git",
            "    v dir_1",
            "        > gitignored_dir",
            "          file_1.py  <== selected  <== marked",
            "          file_2.py",
            "          file_3.py",
            "    > dir_2",
            "      .gitignore",
        ],
        "With no auto reveal, explicit reveal should show the dir_1 entry in the project panel"
    );

    panel.update(cx, |panel, cx| {
        panel.project.update(cx, |_, cx| {
            cx.emit(project::Event::RevealInProjectPanel(dir_2_file))
        })
    });
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v project_root",
            "    > .git",
            "    v dir_1",
            "        > gitignored_dir",
            "          file_1.py",
            "          file_2.py",
            "          file_3.py",
            "    v dir_2",
            "          file_1.py  <== selected  <== marked",
            "          file_2.py",
            "          file_3.py",
            "      .gitignore",
        ],
        "With no auto reveal, explicit reveal should show the dir_2 entry in the project panel"
    );

    panel.update(cx, |panel, cx| {
        panel.project.update(cx, |_, cx| {
            cx.emit(project::Event::RevealInProjectPanel(gitignored_dir_file))
        })
    });
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v project_root",
            "    > .git",
            "    v dir_1",
            "        v gitignored_dir",
            "              file_a.py  <== selected  <== marked",
            "              file_b.py",
            "              file_c.py",
            "          file_1.py",
            "          file_2.py",
            "          file_3.py",
            "    v dir_2",
            "          file_1.py",
            "          file_2.py",
            "          file_3.py",
            "      .gitignore",
        ],
        "With no auto reveal, explicit reveal should show the gitignored entry in the project panel"
    );
}

#[gpui::test]
async fn test_creating_excluded_entries(cx: &mut gpui::TestAppContext) {
    init_test(cx);
    cx.update(|cx| {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.worktree.file_scan_exclusions =
                    Some(vec!["excluded_dir".to_string(), "**/.git".to_string()]);
            });
        });
    });

    cx.update(|cx| {
        register_project_item::<TestProjectItemView>(cx);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root1",
        json!({
            ".dockerignore": "",
            ".git": {
                "HEAD": "",
            },
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root1".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace
        .update(cx, |workspace, window, cx| {
            let panel = ProjectPanel::new(workspace, window, cx);
            workspace.add_panel(panel.clone(), window, cx);
            panel
        })
        .unwrap();
    cx.run_until_parked();

    select_path(&panel, "root1", cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &["v root1  <== selected", "      .dockerignore",]
    );
    workspace
        .update(cx, |workspace, _, cx| {
            assert!(
                workspace.active_item(cx).is_none(),
                "Should have no active items in the beginning"
            );
        })
        .unwrap();

    let excluded_file_path = ".git/COMMIT_EDITMSG";
    let excluded_dir_path = "excluded_dir";

    panel.update_in(cx, |panel, window, cx| panel.new_file(&NewFile, window, cx));
    cx.run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(panel.filename_editor.read(cx).is_focused(window));
    });
    panel
        .update_in(cx, |panel, window, cx| {
            panel.filename_editor.update(cx, |editor, cx| {
                editor.set_text(excluded_file_path, window, cx)
            });
            panel.confirm_edit(true, window, cx).unwrap()
        })
        .await
        .unwrap();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..13, cx),
        &["v root1", "      .dockerignore"],
        "Excluded dir should not be shown after opening a file in it"
    );
    panel.update_in(cx, |panel, window, cx| {
        assert!(
            !panel.filename_editor.read(cx).is_focused(window),
            "Should have closed the file name editor"
        );
    });
    workspace
        .update(cx, |workspace, _, cx| {
            let active_entry_path = workspace
                .active_item(cx)
                .expect("should have opened and activated the excluded item")
                .act_as::<TestProjectItemView>(cx)
                .expect("should have opened the corresponding project item for the excluded item")
                .read(cx)
                .path
                .clone();
            assert_eq!(
                active_entry_path.path.as_ref(),
                rel_path(excluded_file_path),
                "Should open the excluded file"
            );

            assert!(
                workspace.notification_ids().is_empty(),
                "Should have no notifications after opening an excluded file"
            );
        })
        .unwrap();
    assert!(
        fs.is_file(Path::new("/root1/.git/COMMIT_EDITMSG")).await,
        "Should have created the excluded file"
    );

    select_path(&panel, "root1", cx);
    panel.update_in(cx, |panel, window, cx| {
        panel.new_directory(&NewDirectory, window, cx)
    });
    cx.run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(panel.filename_editor.read(cx).is_focused(window));
    });
    panel
        .update_in(cx, |panel, window, cx| {
            panel.filename_editor.update(cx, |editor, cx| {
                editor.set_text(excluded_file_path, window, cx)
            });
            panel.confirm_edit(true, window, cx).unwrap()
        })
        .await
        .unwrap();
    cx.run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..13, cx),
        &["v root1", "      .dockerignore"],
        "Should not change the project panel after trying to create an excluded directorya directory with the same name as the excluded file"
    );
    panel.update_in(cx, |panel, window, cx| {
        assert!(
            !panel.filename_editor.read(cx).is_focused(window),
            "Should have closed the file name editor"
        );
    });
    workspace
        .update(cx, |workspace, _, cx| {
            let notifications = workspace.notification_ids();
            assert_eq!(
                notifications.len(),
                1,
                "Should receive one notification with the error message"
            );
            workspace.dismiss_notification(notifications.first().unwrap(), cx);
            assert!(workspace.notification_ids().is_empty());
        })
        .unwrap();

    select_path(&panel, "root1", cx);
    panel.update_in(cx, |panel, window, cx| {
        panel.new_directory(&NewDirectory, window, cx)
    });
    cx.run_until_parked();

    panel.update_in(cx, |panel, window, cx| {
        assert!(panel.filename_editor.read(cx).is_focused(window));
    });

    panel
        .update_in(cx, |panel, window, cx| {
            panel.filename_editor.update(cx, |editor, cx| {
                editor.set_text(excluded_dir_path, window, cx)
            });
            panel.confirm_edit(true, window, cx).unwrap()
        })
        .await
        .unwrap();

    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..13, cx),
        &["v root1", "      .dockerignore"],
        "Should not change the project panel after trying to create an excluded directory"
    );
    panel.update_in(cx, |panel, window, cx| {
        assert!(
            !panel.filename_editor.read(cx).is_focused(window),
            "Should have closed the file name editor"
        );
    });
    workspace
        .update(cx, |workspace, _, cx| {
            let notifications = workspace.notification_ids();
            assert_eq!(
                notifications.len(),
                1,
                "Should receive one notification explaining that no directory is actually shown"
            );
            workspace.dismiss_notification(notifications.first().unwrap(), cx);
            assert!(workspace.notification_ids().is_empty());
        })
        .unwrap();
    assert!(
        fs.is_dir(Path::new("/root1/excluded_dir")).await,
        "Should have created the excluded directory"
    );
}

#[gpui::test]
async fn test_selection_restored_when_creation_cancelled(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
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

    let project = Project::test(fs.clone(), ["/src".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace
        .update(cx, |workspace, window, cx| {
            let panel = ProjectPanel::new(workspace, window, cx);
            workspace.add_panel(panel.clone(), window, cx);
            panel
        })
        .unwrap();
    cx.run_until_parked();

    select_path(&panel, "src", cx);
    panel.update_in(cx, |panel, window, cx| panel.confirm(&Confirm, window, cx));
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            //
            "v src  <== selected",
            "    > test"
        ]
    );
    panel.update_in(cx, |panel, window, cx| {
        panel.new_directory(&NewDirectory, window, cx)
    });
    cx.executor().run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(panel.filename_editor.read(cx).is_focused(window));
    });
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            //
            "v src",
            "    > [EDITOR: '']  <== selected",
            "    > test"
        ]
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.cancel(&menu::Cancel, window, cx);
        panel.update_visible_entries(None, false, false, window, cx);
    });
    cx.executor().run_until_parked();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            //
            "v src  <== selected",
            "    > test"
        ]
    );
}

#[gpui::test]
async fn test_basic_file_deletion_scenarios(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "dir1": {
                "subdir1": {},
                "file1.txt": "",
                "file2.txt": "",
            },
            "dir2": {
                "subdir2": {},
                "file3.txt": "",
                "file4.txt": "",
            },
            "file5.txt": "",
            "file6.txt": "",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    toggle_expand_dir(&panel, "root/dir1", cx);
    toggle_expand_dir(&panel, "root/dir2", cx);

    // Test Case 1: Delete middle file in directory
    select_path(&panel, "root/dir1/file1.txt", cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..15, cx),
        &[
            "v root",
            "    v dir1",
            "        > subdir1",
            "          file1.txt  <== selected",
            "          file2.txt",
            "    v dir2",
            "        > subdir2",
            "          file3.txt",
            "          file4.txt",
            "      file5.txt",
            "      file6.txt",
        ],
        "Initial state before deleting middle file"
    );

    submit_deletion(&panel, cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..15, cx),
        &[
            "v root",
            "    v dir1",
            "        > subdir1",
            "          file2.txt  <== selected",
            "    v dir2",
            "        > subdir2",
            "          file3.txt",
            "          file4.txt",
            "      file5.txt",
            "      file6.txt",
        ],
        "Should select next file after deleting middle file"
    );

    // Test Case 2: Delete last file in directory
    submit_deletion(&panel, cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..15, cx),
        &[
            "v root",
            "    v dir1",
            "        > subdir1  <== selected",
            "    v dir2",
            "        > subdir2",
            "          file3.txt",
            "          file4.txt",
            "      file5.txt",
            "      file6.txt",
        ],
        "Should select next directory when last file is deleted"
    );

    // Test Case 3: Delete root level file
    select_path(&panel, "root/file6.txt", cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..15, cx),
        &[
            "v root",
            "    v dir1",
            "        > subdir1",
            "    v dir2",
            "        > subdir2",
            "          file3.txt",
            "          file4.txt",
            "      file5.txt",
            "      file6.txt  <== selected",
        ],
        "Initial state before deleting root level file"
    );

    submit_deletion(&panel, cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..15, cx),
        &[
            "v root",
            "    v dir1",
            "        > subdir1",
            "    v dir2",
            "        > subdir2",
            "          file3.txt",
            "          file4.txt",
            "      file5.txt  <== selected",
        ],
        "Should select prev entry at root level"
    );
}

#[gpui::test]
async fn test_deletion_gitignored(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            "aa": "// Testing 1",
            "bb": "// Testing 2",
            "cc": "// Testing 3",
            "dd": "// Testing 4",
            "ee": "// Testing 5",
            "ff": "// Testing 6",
            "gg": "// Testing 7",
            "hh": "// Testing 8",
            "ii": "// Testing 8",
            ".gitignore": "bb\ndd\nee\nff\nii\n'",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    // Test 1: Auto selection with one gitignored file next to the deleted file
    cx.update(|_, cx| {
        let settings = *ProjectPanelSettings::get_global(cx);
        ProjectPanelSettings::override_global(
            ProjectPanelSettings {
                hide_gitignore: true,
                ..settings
            },
            cx,
        );
    });

    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    select_path(&panel, "root/aa", cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root",
            "      .gitignore",
            "      aa  <== selected",
            "      cc",
            "      gg",
            "      hh"
        ],
        "Initial state should hide files on .gitignore"
    );

    submit_deletion(&panel, cx);

    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root",
            "      .gitignore",
            "      cc  <== selected",
            "      gg",
            "      hh"
        ],
        "Should select next entry not on .gitignore"
    );

    // Test 2: Auto selection with many gitignored files next to the deleted file
    submit_deletion(&panel, cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root",
            "      .gitignore",
            "      gg  <== selected",
            "      hh"
        ],
        "Should select next entry not on .gitignore"
    );

    // Test 3: Auto selection of entry before deleted file
    select_path(&panel, "root/hh", cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root",
            "      .gitignore",
            "      gg",
            "      hh  <== selected"
        ],
        "Should select next entry not on .gitignore"
    );
    submit_deletion(&panel, cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &["v root", "      .gitignore", "      gg  <== selected"],
        "Should select next entry not on .gitignore"
    );
}

#[gpui::test]
async fn test_nested_deletion_gitignore(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            "dir1": {
                "file1": "// Testing",
                "file2": "// Testing",
                "file3": "// Testing"
            },
            "aa": "// Testing",
            ".gitignore": "file1\nfile3\n",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    cx.update(|_, cx| {
        let settings = *ProjectPanelSettings::get_global(cx);
        ProjectPanelSettings::override_global(
            ProjectPanelSettings {
                hide_gitignore: true,
                ..settings
            },
            cx,
        );
    });

    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    // Test 1: Visible items should exclude files on gitignore
    toggle_expand_dir(&panel, "root/dir1", cx);
    select_path(&panel, "root/dir1/file2", cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root",
            "    v dir1",
            "          file2  <== selected",
            "      .gitignore",
            "      aa"
        ],
        "Initial state should hide files on .gitignore"
    );
    submit_deletion(&panel, cx);

    // Test 2: Auto selection should go to the parent
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root",
            "    v dir1  <== selected",
            "      .gitignore",
            "      aa"
        ],
        "Initial state should hide files on .gitignore"
    );
}

#[gpui::test]
async fn test_complex_selection_scenarios(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "dir1": {
                "subdir1": {
                    "a.txt": "",
                    "b.txt": ""
                },
                "file1.txt": "",
            },
            "dir2": {
                "subdir2": {
                    "c.txt": "",
                    "d.txt": ""
                },
                "file2.txt": "",
            },
            "file3.txt": "",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    toggle_expand_dir(&panel, "root/dir1", cx);
    toggle_expand_dir(&panel, "root/dir1/subdir1", cx);
    toggle_expand_dir(&panel, "root/dir2", cx);
    toggle_expand_dir(&panel, "root/dir2/subdir2", cx);

    // Test Case 1: Select and delete nested directory with parent
    cx.simulate_modifiers_change(gpui::Modifiers {
        control: true,
        ..Default::default()
    });
    select_path_with_mark(&panel, "root/dir1/subdir1", cx);
    select_path_with_mark(&panel, "root/dir1", cx);

    assert_eq!(
        visible_entries_as_strings(&panel, 0..15, cx),
        &[
            "v root",
            "    v dir1  <== selected  <== marked",
            "        v subdir1  <== marked",
            "              a.txt",
            "              b.txt",
            "          file1.txt",
            "    v dir2",
            "        v subdir2",
            "              c.txt",
            "              d.txt",
            "          file2.txt",
            "      file3.txt",
        ],
        "Initial state before deleting nested directory with parent"
    );

    submit_deletion(&panel, cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..15, cx),
        &[
            "v root",
            "    v dir2  <== selected",
            "        v subdir2",
            "              c.txt",
            "              d.txt",
            "          file2.txt",
            "      file3.txt",
        ],
        "Should select next directory after deleting directory with parent"
    );

    // Test Case 2: Select mixed files and directories across levels
    select_path_with_mark(&panel, "root/dir2/subdir2/c.txt", cx);
    select_path_with_mark(&panel, "root/dir2/file2.txt", cx);
    select_path_with_mark(&panel, "root/file3.txt", cx);

    assert_eq!(
        visible_entries_as_strings(&panel, 0..15, cx),
        &[
            "v root",
            "    v dir2",
            "        v subdir2",
            "              c.txt  <== marked",
            "              d.txt",
            "          file2.txt  <== marked",
            "      file3.txt  <== selected  <== marked",
        ],
        "Initial state before deleting"
    );

    submit_deletion(&panel, cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..15, cx),
        &[
            "v root",
            "    v dir2  <== selected",
            "        v subdir2",
            "              d.txt",
        ],
        "Should select sibling directory"
    );
}

#[gpui::test]
async fn test_delete_all_files_and_directories(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "dir1": {
                "subdir1": {
                    "a.txt": "",
                    "b.txt": ""
                },
                "file1.txt": "",
            },
            "dir2": {
                "subdir2": {
                    "c.txt": "",
                    "d.txt": ""
                },
                "file2.txt": "",
            },
            "file3.txt": "",
            "file4.txt": "",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    toggle_expand_dir(&panel, "root/dir1", cx);
    toggle_expand_dir(&panel, "root/dir1/subdir1", cx);
    toggle_expand_dir(&panel, "root/dir2", cx);
    toggle_expand_dir(&panel, "root/dir2/subdir2", cx);

    // Test Case 1: Select all root files and directories
    cx.simulate_modifiers_change(gpui::Modifiers {
        control: true,
        ..Default::default()
    });
    select_path_with_mark(&panel, "root/dir1", cx);
    select_path_with_mark(&panel, "root/dir2", cx);
    select_path_with_mark(&panel, "root/file3.txt", cx);
    select_path_with_mark(&panel, "root/file4.txt", cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v root",
            "    v dir1  <== marked",
            "        v subdir1",
            "              a.txt",
            "              b.txt",
            "          file1.txt",
            "    v dir2  <== marked",
            "        v subdir2",
            "              c.txt",
            "              d.txt",
            "          file2.txt",
            "      file3.txt  <== marked",
            "      file4.txt  <== selected  <== marked",
        ],
        "State before deleting all contents"
    );

    submit_deletion(&panel, cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &["v root  <== selected"],
        "Only empty root directory should remain after deleting all contents"
    );
}

#[gpui::test]
async fn test_nested_selection_deletion(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "dir1": {
                "subdir1": {
                    "file_a.txt": "content a",
                    "file_b.txt": "content b",
                },
                "subdir2": {
                    "file_c.txt": "content c",
                },
                "file1.txt": "content 1",
            },
            "dir2": {
                "file2.txt": "content 2",
            },
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    toggle_expand_dir(&panel, "root/dir1", cx);
    toggle_expand_dir(&panel, "root/dir1/subdir1", cx);
    toggle_expand_dir(&panel, "root/dir2", cx);
    cx.simulate_modifiers_change(gpui::Modifiers {
        control: true,
        ..Default::default()
    });

    // Test Case 1: Select parent directory, subdirectory, and a file inside the subdirectory
    select_path_with_mark(&panel, "root/dir1", cx);
    select_path_with_mark(&panel, "root/dir1/subdir1", cx);
    select_path_with_mark(&panel, "root/dir1/subdir1/file_a.txt", cx);

    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v root",
            "    v dir1  <== marked",
            "        v subdir1  <== marked",
            "              file_a.txt  <== selected  <== marked",
            "              file_b.txt",
            "        > subdir2",
            "          file1.txt",
            "    v dir2",
            "          file2.txt",
        ],
        "State with parent dir, subdir, and file selected"
    );
    submit_deletion(&panel, cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &["v root", "    v dir2  <== selected", "          file2.txt",],
        "Only dir2 should remain after deletion"
    );
}

#[gpui::test]
async fn test_multiple_worktrees_deletion(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    // First worktree
    fs.insert_tree(
        "/root1",
        json!({
            "dir1": {
                "file1.txt": "content 1",
                "file2.txt": "content 2",
            },
            "dir2": {
                "file3.txt": "content 3",
            },
        }),
    )
    .await;

    // Second worktree
    fs.insert_tree(
        "/root2",
        json!({
            "dir3": {
                "file4.txt": "content 4",
                "file5.txt": "content 5",
            },
            "file6.txt": "content 6",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root1".as_ref(), "/root2".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    // Expand all directories for testing
    toggle_expand_dir(&panel, "root1/dir1", cx);
    toggle_expand_dir(&panel, "root1/dir2", cx);
    toggle_expand_dir(&panel, "root2/dir3", cx);

    // Test Case 1: Delete files across different worktrees
    cx.simulate_modifiers_change(gpui::Modifiers {
        control: true,
        ..Default::default()
    });
    select_path_with_mark(&panel, "root1/dir1/file1.txt", cx);
    select_path_with_mark(&panel, "root2/dir3/file4.txt", cx);

    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v root1",
            "    v dir1",
            "          file1.txt  <== marked",
            "          file2.txt",
            "    v dir2",
            "          file3.txt",
            "v root2",
            "    v dir3",
            "          file4.txt  <== selected  <== marked",
            "          file5.txt",
            "      file6.txt",
        ],
        "Initial state with files selected from different worktrees"
    );

    submit_deletion(&panel, cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v root1",
            "    v dir1",
            "          file2.txt",
            "    v dir2",
            "          file3.txt",
            "v root2",
            "    v dir3",
            "          file5.txt  <== selected",
            "      file6.txt",
        ],
        "Should select next file in the last worktree after deletion"
    );

    // Test Case 2: Delete directories from different worktrees
    select_path_with_mark(&panel, "root1/dir1", cx);
    select_path_with_mark(&panel, "root2/dir3", cx);

    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v root1",
            "    v dir1  <== marked",
            "          file2.txt",
            "    v dir2",
            "          file3.txt",
            "v root2",
            "    v dir3  <== selected  <== marked",
            "          file5.txt",
            "      file6.txt",
        ],
        "State with directories marked from different worktrees"
    );

    submit_deletion(&panel, cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v root1",
            "    v dir2",
            "          file3.txt",
            "v root2",
            "      file6.txt  <== selected",
        ],
        "Should select remaining file in last worktree after directory deletion"
    );

    // Test Case 4: Delete all remaining files except roots
    select_path_with_mark(&panel, "root1/dir2/file3.txt", cx);
    select_path_with_mark(&panel, "root2/file6.txt", cx);

    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v root1",
            "    v dir2",
            "          file3.txt  <== marked",
            "v root2",
            "      file6.txt  <== selected  <== marked",
        ],
        "State with all remaining files marked"
    );

    submit_deletion(&panel, cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &["v root1", "    v dir2", "v root2  <== selected"],
        "Second parent root should be selected after deleting"
    );
}

#[gpui::test]
async fn test_selection_vs_marked_entries_priority(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "dir1": {
                "file1.txt": "",
                "file2.txt": "",
                "file3.txt": "",
            },
            "dir2": {
                "file4.txt": "",
                "file5.txt": "",
            },
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    toggle_expand_dir(&panel, "root/dir1", cx);
    toggle_expand_dir(&panel, "root/dir2", cx);

    cx.simulate_modifiers_change(gpui::Modifiers {
        control: true,
        ..Default::default()
    });

    select_path_with_mark(&panel, "root/dir1/file2.txt", cx);
    select_path(&panel, "root/dir1/file1.txt", cx);

    assert_eq!(
        visible_entries_as_strings(&panel, 0..15, cx),
        &[
            "v root",
            "    v dir1",
            "          file1.txt  <== selected",
            "          file2.txt  <== marked",
            "          file3.txt",
            "    v dir2",
            "          file4.txt",
            "          file5.txt",
        ],
        "Initial state with one marked entry and different selection"
    );

    // Delete should operate on the selected entry (file1.txt)
    submit_deletion(&panel, cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..15, cx),
        &[
            "v root",
            "    v dir1",
            "          file2.txt  <== selected  <== marked",
            "          file3.txt",
            "    v dir2",
            "          file4.txt",
            "          file5.txt",
        ],
        "Should delete selected file, not marked file"
    );

    select_path_with_mark(&panel, "root/dir1/file3.txt", cx);
    select_path_with_mark(&panel, "root/dir2/file4.txt", cx);
    select_path(&panel, "root/dir2/file5.txt", cx);

    assert_eq!(
        visible_entries_as_strings(&panel, 0..15, cx),
        &[
            "v root",
            "    v dir1",
            "          file2.txt  <== marked",
            "          file3.txt  <== marked",
            "    v dir2",
            "          file4.txt  <== marked",
            "          file5.txt  <== selected",
        ],
        "Initial state with multiple marked entries and different selection"
    );

    // Delete should operate on all marked entries, ignoring the selection
    submit_deletion(&panel, cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..15, cx),
        &[
            "v root",
            "    v dir1",
            "    v dir2",
            "          file5.txt  <== selected",
        ],
        "Should delete all marked files, leaving only the selected file"
    );
}

#[gpui::test]
async fn test_selection_fallback_to_next_highest_worktree(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root_b",
        json!({
            "dir1": {
                "file1.txt": "content 1",
                "file2.txt": "content 2",
            },
        }),
    )
    .await;

    fs.insert_tree(
        "/root_c",
        json!({
            "dir2": {},
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root_b".as_ref(), "/root_c".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    toggle_expand_dir(&panel, "root_b/dir1", cx);
    toggle_expand_dir(&panel, "root_c/dir2", cx);

    cx.simulate_modifiers_change(gpui::Modifiers {
        control: true,
        ..Default::default()
    });
    select_path_with_mark(&panel, "root_b/dir1/file1.txt", cx);
    select_path_with_mark(&panel, "root_b/dir1/file2.txt", cx);

    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v root_b",
            "    v dir1",
            "          file1.txt  <== marked",
            "          file2.txt  <== selected  <== marked",
            "v root_c",
            "    v dir2",
        ],
        "Initial state with files marked in root_b"
    );

    submit_deletion(&panel, cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v root_b",
            "    v dir1  <== selected",
            "v root_c",
            "    v dir2",
        ],
        "After deletion in root_b as it's last deletion, selection should be in root_b"
    );

    select_path_with_mark(&panel, "root_c/dir2", cx);

    submit_deletion(&panel, cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &["v root_b", "    v dir1", "v root_c  <== selected",],
        "After deleting from root_c, it should remain in root_c"
    );
}

fn toggle_expand_dir(panel: &Entity<ProjectPanel>, path: &str, cx: &mut VisualTestContext) {
    let path = rel_path(path);
    panel.update_in(cx, |panel, window, cx| {
        for worktree in panel.project.read(cx).worktrees(cx).collect::<Vec<_>>() {
            let worktree = worktree.read(cx);
            if let Ok(relative_path) = path.strip_prefix(worktree.root_name()) {
                let entry_id = worktree.entry_for_path(relative_path).unwrap().id;
                panel.toggle_expanded(entry_id, window, cx);
                return;
            }
        }
        panic!("no worktree for path {:?}", path);
    });
    cx.run_until_parked();
}

#[gpui::test]
async fn test_expand_all_for_entry(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            ".gitignore": "**/ignored_dir\n**/ignored_nested",
            "dir1": {
                "empty1": {
                    "empty2": {
                        "empty3": {
                            "file.txt": ""
                        }
                    }
                },
                "subdir1": {
                    "file1.txt": "",
                    "file2.txt": "",
                    "ignored_nested": {
                        "ignored_file.txt": ""
                    }
                },
                "ignored_dir": {
                    "subdir": {
                        "deep_file.txt": ""
                    }
                }
            }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    // Test 1: When auto-fold is enabled
    cx.update(|_, cx| {
        let settings = *ProjectPanelSettings::get_global(cx);
        ProjectPanelSettings::override_global(
            ProjectPanelSettings {
                auto_fold_dirs: true,
                ..settings
            },
            cx,
        );
    });

    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &["v root", "    > dir1", "      .gitignore",],
        "Initial state should show collapsed root structure"
    );

    toggle_expand_dir(&panel, "root/dir1", cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v root",
            "    v dir1  <== selected",
            "        > empty1/empty2/empty3",
            "        > ignored_dir",
            "        > subdir1",
            "      .gitignore",
        ],
        "Should show first level with auto-folded dirs and ignored dir visible"
    );

    let entry_id = find_project_entry(&panel, "root/dir1", cx).unwrap();
    panel.update_in(cx, |panel, window, cx| {
        let project = panel.project.read(cx);
        let worktree = project.worktrees(cx).next().unwrap().read(cx);
        panel.expand_all_for_entry(worktree.id(), entry_id, cx);
        panel.update_visible_entries(None, false, false, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v root",
            "    v dir1  <== selected",
            "        v empty1",
            "            v empty2",
            "                v empty3",
            "                      file.txt",
            "        > ignored_dir",
            "        v subdir1",
            "            > ignored_nested",
            "              file1.txt",
            "              file2.txt",
            "      .gitignore",
        ],
        "After expand_all with auto-fold: should not expand ignored_dir, should expand folded dirs, and should not expand ignored_nested"
    );

    // Test 2: When auto-fold is disabled
    cx.update(|_, cx| {
        let settings = *ProjectPanelSettings::get_global(cx);
        ProjectPanelSettings::override_global(
            ProjectPanelSettings {
                auto_fold_dirs: false,
                ..settings
            },
            cx,
        );
    });

    panel.update_in(cx, |panel, window, cx| {
        panel.collapse_all_entries(&CollapseAllEntries, window, cx);
    });

    toggle_expand_dir(&panel, "root/dir1", cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v root",
            "    v dir1  <== selected",
            "        > empty1",
            "        > ignored_dir",
            "        > subdir1",
            "      .gitignore",
        ],
        "With auto-fold disabled: should show all directories separately"
    );

    let entry_id = find_project_entry(&panel, "root/dir1", cx).unwrap();
    panel.update_in(cx, |panel, window, cx| {
        let project = panel.project.read(cx);
        let worktree = project.worktrees(cx).next().unwrap().read(cx);
        panel.expand_all_for_entry(worktree.id(), entry_id, cx);
        panel.update_visible_entries(None, false, false, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v root",
            "    v dir1  <== selected",
            "        v empty1",
            "            v empty2",
            "                v empty3",
            "                      file.txt",
            "        > ignored_dir",
            "        v subdir1",
            "            > ignored_nested",
            "              file1.txt",
            "              file2.txt",
            "      .gitignore",
        ],
        "After expand_all without auto-fold: should expand all dirs normally, \
         expand ignored_dir itself but not its subdirs, and not expand ignored_nested"
    );

    // Test 3: When explicitly called on ignored directory
    let ignored_dir_entry = find_project_entry(&panel, "root/dir1/ignored_dir", cx).unwrap();
    panel.update_in(cx, |panel, window, cx| {
        let project = panel.project.read(cx);
        let worktree = project.worktrees(cx).next().unwrap().read(cx);
        panel.expand_all_for_entry(worktree.id(), ignored_dir_entry, cx);
        panel.update_visible_entries(None, false, false, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v root",
            "    v dir1  <== selected",
            "        v empty1",
            "            v empty2",
            "                v empty3",
            "                      file.txt",
            "        v ignored_dir",
            "            v subdir",
            "                  deep_file.txt",
            "        v subdir1",
            "            > ignored_nested",
            "              file1.txt",
            "              file2.txt",
            "      .gitignore",
        ],
        "After expand_all on ignored_dir: should expand all contents of the ignored directory"
    );
}

#[gpui::test]
async fn test_collapse_all_for_entry(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            "dir1": {
                "subdir1": {
                    "nested1": {
                        "file1.txt": "",
                        "file2.txt": ""
                    },
                },
                "subdir2": {
                    "file4.txt": ""
                }
            },
            "dir2": {
                "single_file": {
                    "file5.txt": ""
                }
            }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    // Test 1: Basic collapsing
    {
        let panel = workspace.update(cx, ProjectPanel::new).unwrap();
        cx.run_until_parked();

        toggle_expand_dir(&panel, "root/dir1", cx);
        toggle_expand_dir(&panel, "root/dir1/subdir1", cx);
        toggle_expand_dir(&panel, "root/dir1/subdir1/nested1", cx);
        toggle_expand_dir(&panel, "root/dir1/subdir2", cx);

        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v root",
                "    v dir1",
                "        v subdir1",
                "            v nested1",
                "                  file1.txt",
                "                  file2.txt",
                "        v subdir2  <== selected",
                "              file4.txt",
                "    > dir2",
            ],
            "Initial state with everything expanded"
        );

        let entry_id = find_project_entry(&panel, "root/dir1", cx).unwrap();
        panel.update_in(cx, |panel, window, cx| {
            let project = panel.project.read(cx);
            let worktree = project.worktrees(cx).next().unwrap().read(cx);
            panel.collapse_all_for_entry(worktree.id(), entry_id, cx);
            panel.update_visible_entries(None, false, false, window, cx);
        });
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &["v root", "    > dir1", "    > dir2",],
            "All subdirs under dir1 should be collapsed"
        );
    }

    // Test 2: With auto-fold enabled
    {
        cx.update(|_, cx| {
            let settings = *ProjectPanelSettings::get_global(cx);
            ProjectPanelSettings::override_global(
                ProjectPanelSettings {
                    auto_fold_dirs: true,
                    ..settings
                },
                cx,
            );
        });

        let panel = workspace.update(cx, ProjectPanel::new).unwrap();
        cx.run_until_parked();

        toggle_expand_dir(&panel, "root/dir1", cx);
        toggle_expand_dir(&panel, "root/dir1/subdir1", cx);
        toggle_expand_dir(&panel, "root/dir1/subdir1/nested1", cx);

        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v root",
                "    v dir1",
                "        v subdir1/nested1  <== selected",
                "              file1.txt",
                "              file2.txt",
                "        > subdir2",
                "    > dir2/single_file",
            ],
            "Initial state with some dirs expanded"
        );

        let entry_id = find_project_entry(&panel, "root/dir1", cx).unwrap();
        panel.update(cx, |panel, cx| {
            let project = panel.project.read(cx);
            let worktree = project.worktrees(cx).next().unwrap().read(cx);
            panel.collapse_all_for_entry(worktree.id(), entry_id, cx);
        });

        toggle_expand_dir(&panel, "root/dir1", cx);

        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v root",
                "    v dir1  <== selected",
                "        > subdir1/nested1",
                "        > subdir2",
                "    > dir2/single_file",
            ],
            "Subdirs should be collapsed and folded with auto-fold enabled"
        );
    }

    // Test 3: With auto-fold disabled
    {
        cx.update(|_, cx| {
            let settings = *ProjectPanelSettings::get_global(cx);
            ProjectPanelSettings::override_global(
                ProjectPanelSettings {
                    auto_fold_dirs: false,
                    ..settings
                },
                cx,
            );
        });

        let panel = workspace.update(cx, ProjectPanel::new).unwrap();
        cx.run_until_parked();

        toggle_expand_dir(&panel, "root/dir1", cx);
        toggle_expand_dir(&panel, "root/dir1/subdir1", cx);
        toggle_expand_dir(&panel, "root/dir1/subdir1/nested1", cx);

        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v root",
                "    v dir1",
                "        v subdir1",
                "            v nested1  <== selected",
                "                  file1.txt",
                "                  file2.txt",
                "        > subdir2",
                "    > dir2",
            ],
            "Initial state with some dirs expanded and auto-fold disabled"
        );

        let entry_id = find_project_entry(&panel, "root/dir1", cx).unwrap();
        panel.update(cx, |panel, cx| {
            let project = panel.project.read(cx);
            let worktree = project.worktrees(cx).next().unwrap().read(cx);
            panel.collapse_all_for_entry(worktree.id(), entry_id, cx);
        });

        toggle_expand_dir(&panel, "root/dir1", cx);

        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                "v root",
                "    v dir1  <== selected",
                "        > subdir1",
                "        > subdir2",
                "    > dir2",
            ],
            "Subdirs should be collapsed but not folded with auto-fold disabled"
        );
    }
}

#[gpui::test]
async fn test_create_entries_without_selection(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            "dir1": {
                "file1.txt": "",
            },
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let panel = workspace
        .update(cx, |workspace, window, cx| {
            let panel = ProjectPanel::new(workspace, window, cx);
            workspace.add_panel(panel.clone(), window, cx);
            panel
        })
        .unwrap();
    cx.run_until_parked();

    #[rustfmt::skip]
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v root",
            "    > dir1",
        ],
        "Initial state with nothing selected"
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.new_file(&NewFile, window, cx);
    });
    cx.run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(panel.filename_editor.read(cx).is_focused(window));
    });
    panel
        .update_in(cx, |panel, window, cx| {
            panel.filename_editor.update(cx, |editor, cx| {
                editor.set_text("hello_from_no_selections", window, cx)
            });
            panel.confirm_edit(true, window, cx).unwrap()
        })
        .await
        .unwrap();
    cx.run_until_parked();
    #[rustfmt::skip]
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "v root",
            "    > dir1",
            "      hello_from_no_selections  <== selected  <== marked",
        ],
        "A new file is created under the root directory"
    );
}

#[gpui::test]
async fn test_create_entries_without_selection_hide_root(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            "existing_dir": {
                "existing_file.txt": "",
            },
            "existing_file.txt": "",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    cx.update(|_, cx| {
        let settings = *ProjectPanelSettings::get_global(cx);
        ProjectPanelSettings::override_global(
            ProjectPanelSettings {
                hide_root: true,
                ..settings
            },
            cx,
        );
    });

    let panel = workspace
        .update(cx, |workspace, window, cx| {
            let panel = ProjectPanel::new(workspace, window, cx);
            workspace.add_panel(panel.clone(), window, cx);
            panel
        })
        .unwrap();
    cx.run_until_parked();

    #[rustfmt::skip]
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "> existing_dir",
            "  existing_file.txt",
        ],
        "Initial state with hide_root=true, root should be hidden and nothing selected"
    );

    panel.update(cx, |panel, _| {
        assert!(
            panel.state.selection.is_none(),
            "Should have no selection initially"
        );
    });

    // Test 1: Create new file when no entry is selected
    panel.update_in(cx, |panel, window, cx| {
        panel.new_file(&NewFile, window, cx);
    });
    cx.run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(panel.filename_editor.read(cx).is_focused(window));
    });
    cx.run_until_parked();
    #[rustfmt::skip]
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "> existing_dir",
            "  [EDITOR: '']  <== selected",
            "  existing_file.txt",
        ],
        "Editor should appear at root level when hide_root=true and no selection"
    );

    let confirm = panel.update_in(cx, |panel, window, cx| {
        panel.filename_editor.update(cx, |editor, cx| {
            editor.set_text("new_file_at_root.txt", window, cx)
        });
        panel.confirm_edit(true, window, cx).unwrap()
    });
    confirm.await.unwrap();
    cx.run_until_parked();

    #[rustfmt::skip]
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "> existing_dir",
            "  existing_file.txt",
            "  new_file_at_root.txt  <== selected  <== marked",
        ],
        "New file should be created at root level and visible without root prefix"
    );

    assert!(
        fs.is_file(Path::new("/root/new_file_at_root.txt")).await,
        "File should be created in the actual root directory"
    );

    // Test 2: Create new directory when no entry is selected
    panel.update(cx, |panel, _| {
        panel.state.selection = None;
    });

    panel.update_in(cx, |panel, window, cx| {
        panel.new_directory(&NewDirectory, window, cx);
    });
    cx.run_until_parked();

    panel.update_in(cx, |panel, window, cx| {
        assert!(panel.filename_editor.read(cx).is_focused(window));
    });

    #[rustfmt::skip]
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "> [EDITOR: '']  <== selected",
            "> existing_dir",
            "  existing_file.txt",
            "  new_file_at_root.txt",
        ],
        "Directory editor should appear at root level when hide_root=true and no selection"
    );

    let confirm = panel.update_in(cx, |panel, window, cx| {
        panel.filename_editor.update(cx, |editor, cx| {
            editor.set_text("new_dir_at_root", window, cx)
        });
        panel.confirm_edit(true, window, cx).unwrap()
    });
    confirm.await.unwrap();
    cx.run_until_parked();

    #[rustfmt::skip]
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            "> existing_dir",
            "v new_dir_at_root  <== selected",
            "  existing_file.txt",
            "  new_file_at_root.txt",
        ],
        "New directory should be created at root level and visible without root prefix"
    );

    assert!(
        fs.is_dir(Path::new("/root/new_dir_at_root")).await,
        "Directory should be created in the actual root directory"
    );
}

#[gpui::test]
async fn test_highlight_entry_for_external_drag(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "dir1": {
                "file1.txt": "",
                "dir2": {
                    "file2.txt": ""
                }
            },
            "file3.txt": ""
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    panel.update(cx, |panel, cx| {
        let project = panel.project.read(cx);
        let worktree = project.visible_worktrees(cx).next().unwrap();
        let worktree = worktree.read(cx);

        // Test 1: Target is a directory, should highlight the directory itself
        let dir_entry = worktree.entry_for_path(rel_path("dir1")).unwrap();
        let result = panel.highlight_entry_for_external_drag(dir_entry, worktree);
        assert_eq!(
            result,
            Some(dir_entry.id),
            "Should highlight directory itself"
        );

        // Test 2: Target is nested file, should highlight immediate parent
        let nested_file = worktree
            .entry_for_path(rel_path("dir1/dir2/file2.txt"))
            .unwrap();
        let nested_parent = worktree.entry_for_path(rel_path("dir1/dir2")).unwrap();
        let result = panel.highlight_entry_for_external_drag(nested_file, worktree);
        assert_eq!(
            result,
            Some(nested_parent.id),
            "Should highlight immediate parent"
        );

        // Test 3: Target is root level file, should highlight root
        let root_file = worktree.entry_for_path(rel_path("file3.txt")).unwrap();
        let result = panel.highlight_entry_for_external_drag(root_file, worktree);
        assert_eq!(
            result,
            Some(worktree.root_entry().unwrap().id),
            "Root level file should return None"
        );

        // Test 4: Target is root itself, should highlight root
        let root_entry = worktree.root_entry().unwrap();
        let result = panel.highlight_entry_for_external_drag(root_entry, worktree);
        assert_eq!(
            result,
            Some(root_entry.id),
            "Root level file should return None"
        );
    });
}

#[gpui::test]
async fn test_highlight_entry_for_selection_drag(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "parent_dir": {
                "child_file.txt": "",
                "sibling_file.txt": "",
                "child_dir": {
                    "nested_file.txt": ""
                }
            },
            "other_dir": {
                "other_file.txt": ""
            }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    panel.update(cx, |panel, cx| {
        let project = panel.project.read(cx);
        let worktree = project.visible_worktrees(cx).next().unwrap();
        let worktree_id = worktree.read(cx).id();
        let worktree = worktree.read(cx);

        let parent_dir = worktree.entry_for_path(rel_path("parent_dir")).unwrap();
        let child_file = worktree
            .entry_for_path(rel_path("parent_dir/child_file.txt"))
            .unwrap();
        let sibling_file = worktree
            .entry_for_path(rel_path("parent_dir/sibling_file.txt"))
            .unwrap();
        let child_dir = worktree
            .entry_for_path(rel_path("parent_dir/child_dir"))
            .unwrap();
        let other_dir = worktree.entry_for_path(rel_path("other_dir")).unwrap();
        let other_file = worktree
            .entry_for_path(rel_path("other_dir/other_file.txt"))
            .unwrap();

        // Test 1: Single item drag, don't highlight parent directory
        let dragged_selection = DraggedSelection {
            active_selection: SelectedEntry {
                worktree_id,
                entry_id: child_file.id,
            },
            marked_selections: Arc::new([SelectedEntry {
                worktree_id,
                entry_id: child_file.id,
            }]),
        };
        let result =
            panel.highlight_entry_for_selection_drag(parent_dir, worktree, &dragged_selection, cx);
        assert_eq!(result, None, "Should not highlight parent of dragged item");

        // Test 2: Single item drag, don't highlight sibling files
        let result = panel.highlight_entry_for_selection_drag(
            sibling_file,
            worktree,
            &dragged_selection,
            cx,
        );
        assert_eq!(result, None, "Should not highlight sibling files");

        // Test 3: Single item drag, highlight unrelated directory
        let result =
            panel.highlight_entry_for_selection_drag(other_dir, worktree, &dragged_selection, cx);
        assert_eq!(
            result,
            Some(other_dir.id),
            "Should highlight unrelated directory"
        );

        // Test 4: Single item drag, highlight sibling directory
        let result =
            panel.highlight_entry_for_selection_drag(child_dir, worktree, &dragged_selection, cx);
        assert_eq!(
            result,
            Some(child_dir.id),
            "Should highlight sibling directory"
        );

        // Test 5: Multiple items drag, highlight parent directory
        let dragged_selection = DraggedSelection {
            active_selection: SelectedEntry {
                worktree_id,
                entry_id: child_file.id,
            },
            marked_selections: Arc::new([
                SelectedEntry {
                    worktree_id,
                    entry_id: child_file.id,
                },
                SelectedEntry {
                    worktree_id,
                    entry_id: sibling_file.id,
                },
            ]),
        };
        let result =
            panel.highlight_entry_for_selection_drag(parent_dir, worktree, &dragged_selection, cx);
        assert_eq!(
            result,
            Some(parent_dir.id),
            "Should highlight parent with multiple items"
        );

        // Test 6: Target is file in different directory, highlight parent
        let result =
            panel.highlight_entry_for_selection_drag(other_file, worktree, &dragged_selection, cx);
        assert_eq!(
            result,
            Some(other_dir.id),
            "Should highlight parent of target file"
        );

        // Test 7: Target is directory, always highlight
        let result =
            panel.highlight_entry_for_selection_drag(child_dir, worktree, &dragged_selection, cx);
        assert_eq!(
            result,
            Some(child_dir.id),
            "Should always highlight directories"
        );
    });
}

#[gpui::test]
async fn test_highlight_entry_for_selection_drag_cross_worktree(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root1",
        json!({
            "src": {
                "main.rs": "",
                "lib.rs": ""
            }
        }),
    )
    .await;
    fs.insert_tree(
        "/root2",
        json!({
            "src": {
                "main.rs": "",
                "test.rs": ""
            }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root1".as_ref(), "/root2".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    panel.update(cx, |panel, cx| {
        let project = panel.project.read(cx);
        let worktrees: Vec<_> = project.visible_worktrees(cx).collect();

        let worktree_a = &worktrees[0];
        let main_rs_from_a = worktree_a
            .read(cx)
            .entry_for_path(rel_path("src/main.rs"))
            .unwrap();

        let worktree_b = &worktrees[1];
        let src_dir_from_b = worktree_b.read(cx).entry_for_path(rel_path("src")).unwrap();
        let main_rs_from_b = worktree_b
            .read(cx)
            .entry_for_path(rel_path("src/main.rs"))
            .unwrap();

        // Test dragging file from worktree A onto parent of file with same relative path in worktree B
        let dragged_selection = DraggedSelection {
            active_selection: SelectedEntry {
                worktree_id: worktree_a.read(cx).id(),
                entry_id: main_rs_from_a.id,
            },
            marked_selections: Arc::new([SelectedEntry {
                worktree_id: worktree_a.read(cx).id(),
                entry_id: main_rs_from_a.id,
            }]),
        };

        let result = panel.highlight_entry_for_selection_drag(
            src_dir_from_b,
            worktree_b.read(cx),
            &dragged_selection,
            cx,
        );
        assert_eq!(
            result,
            Some(src_dir_from_b.id),
            "Should highlight target directory from different worktree even with same relative path"
        );

        // Test dragging file from worktree A onto file with same relative path in worktree B
        let result = panel.highlight_entry_for_selection_drag(
            main_rs_from_b,
            worktree_b.read(cx),
            &dragged_selection,
            cx,
        );
        assert_eq!(
            result,
            Some(src_dir_from_b.id),
            "Should highlight parent of target file from different worktree"
        );
    });
}

#[gpui::test]
async fn test_should_highlight_background_for_selection_drag(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root1",
        json!({
            "parent_dir": {
                "child_file.txt": "",
                "nested_dir": {
                    "nested_file.txt": ""
                }
            },
            "root_file.txt": ""
        }),
    )
    .await;

    fs.insert_tree(
        "/root2",
        json!({
            "other_dir": {
                "other_file.txt": ""
            }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root1".as_ref(), "/root2".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    panel.update(cx, |panel, cx| {
        let project = panel.project.read(cx);
        let worktrees: Vec<_> = project.visible_worktrees(cx).collect();
        let worktree1 = worktrees[0].read(cx);
        let worktree2 = worktrees[1].read(cx);
        let worktree1_id = worktree1.id();
        let _worktree2_id = worktree2.id();

        let root1_entry = worktree1.root_entry().unwrap();
        let root2_entry = worktree2.root_entry().unwrap();
        let _parent_dir = worktree1.entry_for_path(rel_path("parent_dir")).unwrap();
        let child_file = worktree1
            .entry_for_path(rel_path("parent_dir/child_file.txt"))
            .unwrap();
        let nested_file = worktree1
            .entry_for_path(rel_path("parent_dir/nested_dir/nested_file.txt"))
            .unwrap();
        let root_file = worktree1.entry_for_path(rel_path("root_file.txt")).unwrap();

        // Test 1: Multiple entries - should always highlight background
        let multiple_dragged_selection = DraggedSelection {
            active_selection: SelectedEntry {
                worktree_id: worktree1_id,
                entry_id: child_file.id,
            },
            marked_selections: Arc::new([
                SelectedEntry {
                    worktree_id: worktree1_id,
                    entry_id: child_file.id,
                },
                SelectedEntry {
                    worktree_id: worktree1_id,
                    entry_id: nested_file.id,
                },
            ]),
        };

        let result = panel.should_highlight_background_for_selection_drag(
            &multiple_dragged_selection,
            root1_entry.id,
            cx,
        );
        assert!(result, "Should highlight background for multiple entries");

        // Test 2: Single entry with non-empty parent path - should highlight background
        let nested_dragged_selection = DraggedSelection {
            active_selection: SelectedEntry {
                worktree_id: worktree1_id,
                entry_id: nested_file.id,
            },
            marked_selections: Arc::new([SelectedEntry {
                worktree_id: worktree1_id,
                entry_id: nested_file.id,
            }]),
        };

        let result = panel.should_highlight_background_for_selection_drag(
            &nested_dragged_selection,
            root1_entry.id,
            cx,
        );
        assert!(result, "Should highlight background for nested file");

        // Test 3: Single entry at root level, same worktree - should NOT highlight background
        let root_file_dragged_selection = DraggedSelection {
            active_selection: SelectedEntry {
                worktree_id: worktree1_id,
                entry_id: root_file.id,
            },
            marked_selections: Arc::new([SelectedEntry {
                worktree_id: worktree1_id,
                entry_id: root_file.id,
            }]),
        };

        let result = panel.should_highlight_background_for_selection_drag(
            &root_file_dragged_selection,
            root1_entry.id,
            cx,
        );
        assert!(
            !result,
            "Should NOT highlight background for root file in same worktree"
        );

        // Test 4: Single entry at root level, different worktree - should highlight background
        let result = panel.should_highlight_background_for_selection_drag(
            &root_file_dragged_selection,
            root2_entry.id,
            cx,
        );
        assert!(
            result,
            "Should highlight background for root file from different worktree"
        );

        // Test 5: Single entry in subdirectory - should highlight background
        let child_file_dragged_selection = DraggedSelection {
            active_selection: SelectedEntry {
                worktree_id: worktree1_id,
                entry_id: child_file.id,
            },
            marked_selections: Arc::new([SelectedEntry {
                worktree_id: worktree1_id,
                entry_id: child_file.id,
            }]),
        };

        let result = panel.should_highlight_background_for_selection_drag(
            &child_file_dragged_selection,
            root1_entry.id,
            cx,
        );
        assert!(
            result,
            "Should highlight background for file with non-empty parent path"
        );
    });
}

#[gpui::test]
async fn test_hide_root(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root1",
        json!({
            "dir1": {
                "file1.txt": "content",
                "file2.txt": "content",
            },
            "dir2": {
                "file3.txt": "content",
            },
            "file4.txt": "content",
        }),
    )
    .await;

    fs.insert_tree(
        "/root2",
        json!({
            "dir3": {
                "file5.txt": "content",
            },
            "file6.txt": "content",
        }),
    )
    .await;

    // Test 1: Single worktree with hide_root = false
    {
        let project = Project::test(fs.clone(), ["/root1".as_ref()], cx).await;
        let workspace =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);

        cx.update(|_, cx| {
            let settings = *ProjectPanelSettings::get_global(cx);
            ProjectPanelSettings::override_global(
                ProjectPanelSettings {
                    hide_root: false,
                    ..settings
                },
                cx,
            );
        });

        let panel = workspace.update(cx, ProjectPanel::new).unwrap();
        cx.run_until_parked();

        #[rustfmt::skip]
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > dir1",
                "    > dir2",
                "      file4.txt",
            ],
            "With hide_root=false and single worktree, root should be visible"
        );
    }

    // Test 2: Single worktree with hide_root = true
    {
        let project = Project::test(fs.clone(), ["/root1".as_ref()], cx).await;
        let workspace =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);

        // Set hide_root to true
        cx.update(|_, cx| {
            let settings = *ProjectPanelSettings::get_global(cx);
            ProjectPanelSettings::override_global(
                ProjectPanelSettings {
                    hide_root: true,
                    ..settings
                },
                cx,
            );
        });

        let panel = workspace.update(cx, ProjectPanel::new).unwrap();
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &["> dir1", "> dir2", "  file4.txt",],
            "With hide_root=true and single worktree, root should be hidden"
        );

        // Test expanding directories still works without root
        toggle_expand_dir(&panel, "root1/dir1", cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v dir1  <== selected",
                "      file1.txt",
                "      file2.txt",
                "> dir2",
                "  file4.txt",
            ],
            "Should be able to expand directories even when root is hidden"
        );
    }

    // Test 3: Multiple worktrees with hide_root = true
    {
        let project = Project::test(fs.clone(), ["/root1".as_ref(), "/root2".as_ref()], cx).await;
        let workspace =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);

        // Set hide_root to true
        cx.update(|_, cx| {
            let settings = *ProjectPanelSettings::get_global(cx);
            ProjectPanelSettings::override_global(
                ProjectPanelSettings {
                    hide_root: true,
                    ..settings
                },
                cx,
            );
        });

        let panel = workspace.update(cx, ProjectPanel::new).unwrap();
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > dir1",
                "    > dir2",
                "      file4.txt",
                "v root2",
                "    > dir3",
                "      file6.txt",
            ],
            "With hide_root=true and multiple worktrees, roots should still be visible"
        );
    }

    // Test 4: Multiple worktrees with hide_root = false
    {
        let project = Project::test(fs.clone(), ["/root1".as_ref(), "/root2".as_ref()], cx).await;
        let workspace =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let cx = &mut VisualTestContext::from_window(*workspace, cx);

        cx.update(|_, cx| {
            let settings = *ProjectPanelSettings::get_global(cx);
            ProjectPanelSettings::override_global(
                ProjectPanelSettings {
                    hide_root: false,
                    ..settings
                },
                cx,
            );
        });

        let panel = workspace.update(cx, ProjectPanel::new).unwrap();
        cx.run_until_parked();

        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > dir1",
                "    > dir2",
                "      file4.txt",
                "v root2",
                "    > dir3",
                "      file6.txt",
            ],
            "With hide_root=false and multiple worktrees, roots should be visible"
        );
    }
}

#[gpui::test]
async fn test_compare_selected_files(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "file1.txt": "content of file1",
            "file2.txt": "content of file2",
            "dir1": {
                "file3.txt": "content of file3"
            }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    let file1_path = "root/file1.txt";
    let file2_path = "root/file2.txt";
    select_path_with_mark(&panel, file1_path, cx);
    select_path_with_mark(&panel, file2_path, cx);

    panel.update_in(cx, |panel, window, cx| {
        panel.compare_marked_files(&CompareMarkedFiles, window, cx);
    });
    cx.executor().run_until_parked();

    workspace
        .update(cx, |workspace, _, cx| {
            let active_items = workspace
                .panes()
                .iter()
                .filter_map(|pane| pane.read(cx).active_item())
                .collect::<Vec<_>>();
            assert_eq!(active_items.len(), 1);
            let diff_view = active_items
                .into_iter()
                .next()
                .unwrap()
                .downcast::<FileDiffView>()
                .expect("Open item should be an FileDiffView");
            assert_eq!(diff_view.tab_content_text(0, cx), "file1.txt ↔ file2.txt");
            assert_eq!(
                diff_view.tab_tooltip_text(cx).unwrap(),
                format!(
                    "{} ↔ {}",
                    rel_path(file1_path).display(PathStyle::local()),
                    rel_path(file2_path).display(PathStyle::local())
                )
            );
        })
        .unwrap();

    let file1_entry_id = find_project_entry(&panel, file1_path, cx).unwrap();
    let file2_entry_id = find_project_entry(&panel, file2_path, cx).unwrap();
    let worktree_id = panel.update(cx, |panel, cx| {
        panel
            .project
            .read(cx)
            .worktrees(cx)
            .next()
            .unwrap()
            .read(cx)
            .id()
    });

    let expected_entries = [
        SelectedEntry {
            worktree_id,
            entry_id: file1_entry_id,
        },
        SelectedEntry {
            worktree_id,
            entry_id: file2_entry_id,
        },
    ];
    panel.update(cx, |panel, _cx| {
        assert_eq!(
            &panel.marked_entries, &expected_entries,
            "Should keep marked entries after comparison"
        );
    });

    panel.update(cx, |panel, cx| {
        panel.project.update(cx, |_, cx| {
            cx.emit(project::Event::RevealInProjectPanel(file2_entry_id))
        })
    });

    panel.update(cx, |panel, _cx| {
        assert_eq!(
            &panel.marked_entries, &expected_entries,
            "Marked entries should persist after focusing back on the project panel"
        );
    });
}

#[gpui::test]
async fn test_compare_files_context_menu(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "file1.txt": "content of file1",
            "file2.txt": "content of file2",
            "dir1": {},
            "dir2": {
                "file3.txt": "content of file3"
            }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    // Test 1: When only one file is selected, there should be no compare option
    select_path(&panel, "root/file1.txt", cx);

    let selected_files = panel.update(cx, |panel, cx| panel.file_abs_paths_to_diff(cx));
    assert_eq!(
        selected_files, None,
        "Should not have compare option when only one file is selected"
    );

    // Test 2: When multiple files are selected, there should be a compare option
    select_path_with_mark(&panel, "root/file1.txt", cx);
    select_path_with_mark(&panel, "root/file2.txt", cx);

    let selected_files = panel.update(cx, |panel, cx| panel.file_abs_paths_to_diff(cx));
    assert!(
        selected_files.is_some(),
        "Should have files selected for comparison"
    );
    if let Some((file1, file2)) = selected_files {
        assert!(
            file1.to_string_lossy().ends_with("file1.txt")
                && file2.to_string_lossy().ends_with("file2.txt"),
            "Should have file1.txt and file2.txt as the selected files when multi-selecting"
        );
    }

    // Test 3: Selecting a directory shouldn't count as a comparable file
    select_path_with_mark(&panel, "root/dir1", cx);

    let selected_files = panel.update(cx, |panel, cx| panel.file_abs_paths_to_diff(cx));
    assert!(
        selected_files.is_some(),
        "Directory selection should not affect comparable files"
    );
    if let Some((file1, file2)) = selected_files {
        assert!(
            file1.to_string_lossy().ends_with("file1.txt")
                && file2.to_string_lossy().ends_with("file2.txt"),
            "Selecting a directory should not affect the number of comparable files"
        );
    }

    // Test 4: Selecting one more file
    select_path_with_mark(&panel, "root/dir2/file3.txt", cx);

    let selected_files = panel.update(cx, |panel, cx| panel.file_abs_paths_to_diff(cx));
    assert!(
        selected_files.is_some(),
        "Directory selection should not affect comparable files"
    );
    if let Some((file1, file2)) = selected_files {
        assert!(
            file1.to_string_lossy().ends_with("file2.txt")
                && file2.to_string_lossy().ends_with("file3.txt"),
            "Selecting a directory should not affect the number of comparable files"
        );
    }
}

#[gpui::test]
async fn test_hide_hidden_entries(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            ".hidden-file.txt": "hidden file content",
            "visible-file.txt": "visible file content",
            ".hidden-parent-dir": {
                "nested-dir": {
                    "file.txt": "file content",
                }
            },
            "visible-dir": {
                "file-in-visible.txt": "file content",
                "nested": {
                    ".hidden-nested-dir": {
                        ".double-hidden-dir": {
                            "deep-file-1.txt": "deep content 1",
                            "deep-file-2.txt": "deep content 2"
                        },
                        "hidden-nested-file-1.txt": "hidden nested 1",
                        "hidden-nested-file-2.txt": "hidden nested 2"
                    },
                    "visible-nested-file.txt": "visible nested content"
                }
            }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    cx.update(|_, cx| {
        let settings = *ProjectPanelSettings::get_global(cx);
        ProjectPanelSettings::override_global(
            ProjectPanelSettings {
                hide_hidden: false,
                ..settings
            },
            cx,
        );
    });

    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    toggle_expand_dir(&panel, "root/.hidden-parent-dir", cx);
    toggle_expand_dir(&panel, "root/.hidden-parent-dir/nested-dir", cx);
    toggle_expand_dir(&panel, "root/visible-dir", cx);
    toggle_expand_dir(&panel, "root/visible-dir/nested", cx);
    toggle_expand_dir(&panel, "root/visible-dir/nested/.hidden-nested-dir", cx);
    toggle_expand_dir(
        &panel,
        "root/visible-dir/nested/.hidden-nested-dir/.double-hidden-dir",
        cx,
    );

    let expanded = [
        "v root",
        "    v .hidden-parent-dir",
        "        v nested-dir",
        "              file.txt",
        "    v visible-dir",
        "        v nested",
        "            v .hidden-nested-dir",
        "                v .double-hidden-dir  <== selected",
        "                      deep-file-1.txt",
        "                      deep-file-2.txt",
        "                  hidden-nested-file-1.txt",
        "                  hidden-nested-file-2.txt",
        "              visible-nested-file.txt",
        "          file-in-visible.txt",
        "      .hidden-file.txt",
        "      visible-file.txt",
    ];

    assert_eq!(
        visible_entries_as_strings(&panel, 0..30, cx),
        &expanded,
        "With hide_hidden=false, contents of hidden nested directory should be visible"
    );

    cx.update(|_, cx| {
        let settings = *ProjectPanelSettings::get_global(cx);
        ProjectPanelSettings::override_global(
            ProjectPanelSettings {
                hide_hidden: true,
                ..settings
            },
            cx,
        );
    });

    panel.update_in(cx, |panel, window, cx| {
        panel.update_visible_entries(None, false, false, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..30, cx),
        &[
            "v root",
            "    v visible-dir",
            "        v nested",
            "              visible-nested-file.txt",
            "          file-in-visible.txt",
            "      visible-file.txt",
        ],
        "With hide_hidden=false, contents of hidden nested directory should be visible"
    );

    panel.update_in(cx, |panel, window, cx| {
        let settings = *ProjectPanelSettings::get_global(cx);
        ProjectPanelSettings::override_global(
            ProjectPanelSettings {
                hide_hidden: false,
                ..settings
            },
            cx,
        );
        panel.update_visible_entries(None, false, false, window, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..30, cx),
        &expanded,
        "With hide_hidden=false, deeply nested hidden directories and their contents should be visible"
    );
}

fn select_path(panel: &Entity<ProjectPanel>, path: &str, cx: &mut VisualTestContext) {
    let path = rel_path(path);
    panel.update_in(cx, |panel, window, cx| {
        for worktree in panel.project.read(cx).worktrees(cx).collect::<Vec<_>>() {
            let worktree = worktree.read(cx);
            if let Ok(relative_path) = path.strip_prefix(worktree.root_name()) {
                let entry_id = worktree.entry_for_path(relative_path).unwrap().id;
                panel.update_visible_entries(
                    Some((worktree.id(), entry_id)),
                    false,
                    false,
                    window,
                    cx,
                );
                return;
            }
        }
        panic!("no worktree for path {:?}", path);
    });
    cx.run_until_parked();
}

fn select_path_with_mark(panel: &Entity<ProjectPanel>, path: &str, cx: &mut VisualTestContext) {
    let path = rel_path(path);
    panel.update(cx, |panel, cx| {
        for worktree in panel.project.read(cx).worktrees(cx).collect::<Vec<_>>() {
            let worktree = worktree.read(cx);
            if let Ok(relative_path) = path.strip_prefix(worktree.root_name()) {
                let entry_id = worktree.entry_for_path(relative_path).unwrap().id;
                let entry = crate::SelectedEntry {
                    worktree_id: worktree.id(),
                    entry_id,
                };
                if !panel.marked_entries.contains(&entry) {
                    panel.marked_entries.push(entry);
                }
                panel.state.selection = Some(entry);
                return;
            }
        }
        panic!("no worktree for path {:?}", path);
    });
}

fn drag_selection_to(
    panel: &Entity<ProjectPanel>,
    target_path: &str,
    is_file: bool,
    cx: &mut VisualTestContext,
) {
    let target_entry = find_project_entry(panel, target_path, cx)
        .unwrap_or_else(|| panic!("no entry for target path {target_path:?}"));

    panel.update_in(cx, |panel, window, cx| {
        let selection = panel
            .state
            .selection
            .expect("a selection is required before dragging");
        let drag = DraggedSelection {
            active_selection: SelectedEntry {
                worktree_id: selection.worktree_id,
                entry_id: panel.resolve_entry(selection.entry_id),
            },
            marked_selections: Arc::from(panel.marked_entries.clone()),
        };
        panel.drag_onto(&drag, target_entry, is_file, window, cx);
    });
    cx.executor().run_until_parked();
}

fn find_project_entry(
    panel: &Entity<ProjectPanel>,
    path: &str,
    cx: &mut VisualTestContext,
) -> Option<ProjectEntryId> {
    let path = rel_path(path);
    panel.update(cx, |panel, cx| {
        for worktree in panel.project.read(cx).worktrees(cx).collect::<Vec<_>>() {
            let worktree = worktree.read(cx);
            if let Ok(relative_path) = path.strip_prefix(worktree.root_name()) {
                return worktree.entry_for_path(relative_path).map(|entry| entry.id);
            }
        }
        panic!("no worktree for path {path:?}");
    })
}

fn visible_entries_as_strings(
    panel: &Entity<ProjectPanel>,
    range: Range<usize>,
    cx: &mut VisualTestContext,
) -> Vec<String> {
    let mut result = Vec::new();
    let mut project_entries = HashSet::default();
    let mut has_editor = false;

    panel.update_in(cx, |panel, window, cx| {
        panel.for_each_visible_entry(range, window, cx, |project_entry, details, _, _| {
            if details.is_editing {
                assert!(!has_editor, "duplicate editor entry");
                has_editor = true;
            } else {
                assert!(
                    project_entries.insert(project_entry),
                    "duplicate project entry {:?} {:?}",
                    project_entry,
                    details
                );
            }

            let indent = "    ".repeat(details.depth);
            let icon = if details.kind.is_dir() {
                if details.is_expanded { "v " } else { "> " }
            } else {
                "  "
            };
            #[cfg(windows)]
            let filename = details.filename.replace("\\", "/");
            #[cfg(not(windows))]
            let filename = details.filename;
            let name = if details.is_editing {
                format!("[EDITOR: '{}']", filename)
            } else if details.is_processing {
                format!("[PROCESSING: '{}']", filename)
            } else {
                filename
            };
            let selected = if details.is_selected {
                "  <== selected"
            } else {
                ""
            };
            let marked = if details.is_marked {
                "  <== marked"
            } else {
                ""
            };

            result.push(format!("{indent}{icon}{name}{selected}{marked}"));
        });
    });

    result
}

/// Test that missing sort_mode field defaults to DirectoriesFirst
#[gpui::test]
async fn test_sort_mode_default_fallback(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    // Verify that when sort_mode is not specified, it defaults to DirectoriesFirst
    let default_settings = cx.read(|cx| *ProjectPanelSettings::get_global(cx));
    assert_eq!(
        default_settings.sort_mode,
        settings::ProjectPanelSortMode::DirectoriesFirst,
        "sort_mode should default to DirectoriesFirst"
    );
}

/// Test sort modes: DirectoriesFirst (default) vs Mixed
#[gpui::test]
async fn test_sort_mode_directories_first(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "zebra.txt": "",
            "Apple": {},
            "banana.rs": "",
            "Carrot": {},
            "aardvark.txt": "",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    // Default sort mode should be DirectoriesFirst
    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &[
            "v root",
            "    > Apple",
            "    > Carrot",
            "      aardvark.txt",
            "      banana.rs",
            "      zebra.txt",
        ]
    );
}

#[gpui::test]
async fn test_sort_mode_mixed(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "Zebra.txt": "",
            "apple": {},
            "Banana.rs": "",
            "carrot": {},
            "Aardvark.txt": "",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    // Switch to Mixed mode
    cx.update(|_, cx| {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project_panel.get_or_insert_default().sort_mode =
                    Some(settings::ProjectPanelSortMode::Mixed);
            });
        });
    });

    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    // Mixed mode: case-insensitive sorting
    // Aardvark < apple < Banana < carrot < Zebra (all case-insensitive)
    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &[
            "v root",
            "      Aardvark.txt",
            "    > apple",
            "      Banana.rs",
            "    > carrot",
            "      Zebra.txt",
        ]
    );
}

#[gpui::test]
async fn test_sort_mode_files_first(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "Zebra.txt": "",
            "apple": {},
            "Banana.rs": "",
            "carrot": {},
            "Aardvark.txt": "",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    // Switch to FilesFirst mode
    cx.update(|_, cx| {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project_panel.get_or_insert_default().sort_mode =
                    Some(settings::ProjectPanelSortMode::FilesFirst);
            });
        });
    });

    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    // FilesFirst mode: files first, then directories (both case-insensitive)
    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &[
            "v root",
            "      Aardvark.txt",
            "      Banana.rs",
            "      Zebra.txt",
            "    > apple",
            "    > carrot",
        ]
    );
}

#[gpui::test]
async fn test_sort_mode_toggle(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "file2.txt": "",
            "dir1": {},
            "file1.txt": "",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    cx.run_until_parked();

    // Initially DirectoriesFirst
    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &["v root", "    > dir1", "      file1.txt", "      file2.txt",]
    );

    // Toggle to Mixed
    cx.update(|_, cx| {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project_panel.get_or_insert_default().sort_mode =
                    Some(settings::ProjectPanelSortMode::Mixed);
            });
        });
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &["v root", "    > dir1", "      file1.txt", "      file2.txt",]
    );

    // Toggle back to DirectoriesFirst
    cx.update(|_, cx| {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project_panel.get_or_insert_default().sort_mode =
                    Some(settings::ProjectPanelSortMode::DirectoriesFirst);
            });
        });
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..50, cx),
        &["v root", "    > dir1", "      file1.txt", "      file2.txt",]
    );
}

fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        theme::init(theme::LoadThemes::JustBase, cx);
        crate::init(cx);

        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings
                    .project_panel
                    .get_or_insert_default()
                    .auto_fold_dirs = Some(false);
                settings.project.worktree.file_scan_exclusions = Some(Vec::new());
            });
        });
    });
}

fn init_test_with_editor(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let app_state = AppState::test(cx);
        theme::init(theme::LoadThemes::JustBase, cx);
        editor::init(cx);
        crate::init(cx);
        workspace::init(app_state, cx);

        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings
                    .project_panel
                    .get_or_insert_default()
                    .auto_fold_dirs = Some(false);
                settings.project.worktree.file_scan_exclusions = Some(Vec::new())
            });
        });
    });
}

fn set_auto_open_settings(
    cx: &mut TestAppContext,
    auto_open_settings: ProjectPanelAutoOpenSettings,
) {
    cx.update(|cx| {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project_panel.get_or_insert_default().auto_open = Some(auto_open_settings);
            });
        })
    });
}

fn ensure_single_file_is_opened(
    window: &WindowHandle<Workspace>,
    expected_path: &str,
    cx: &mut TestAppContext,
) {
    window
        .update(cx, |workspace, _, cx| {
            let worktrees = workspace.worktrees(cx).collect::<Vec<_>>();
            assert_eq!(worktrees.len(), 1);
            let worktree_id = worktrees[0].read(cx).id();

            let open_project_paths = workspace
                .panes()
                .iter()
                .filter_map(|pane| pane.read(cx).active_item()?.project_path(cx))
                .collect::<Vec<_>>();
            assert_eq!(
                open_project_paths,
                vec![ProjectPath {
                    worktree_id,
                    path: Arc::from(rel_path(expected_path))
                }],
                "Should have opened file, selected in project panel"
            );
        })
        .unwrap();
}

fn submit_deletion(panel: &Entity<ProjectPanel>, cx: &mut VisualTestContext) {
    assert!(
        !cx.has_pending_prompt(),
        "Should have no prompts before the deletion"
    );
    panel.update_in(cx, |panel, window, cx| {
        panel.delete(&Delete { skip_prompt: false }, window, cx)
    });
    assert!(
        cx.has_pending_prompt(),
        "Should have a prompt after the deletion"
    );
    cx.simulate_prompt_answer("Delete");
    assert!(
        !cx.has_pending_prompt(),
        "Should have no prompts after prompt was replied to"
    );
    cx.executor().run_until_parked();
}

fn submit_deletion_skipping_prompt(panel: &Entity<ProjectPanel>, cx: &mut VisualTestContext) {
    assert!(
        !cx.has_pending_prompt(),
        "Should have no prompts before the deletion"
    );
    panel.update_in(cx, |panel, window, cx| {
        panel.delete(&Delete { skip_prompt: true }, window, cx)
    });
    assert!(!cx.has_pending_prompt(), "Should have received no prompts");
    cx.executor().run_until_parked();
}

fn ensure_no_open_items_and_panes(workspace: &WindowHandle<Workspace>, cx: &mut VisualTestContext) {
    assert!(
        !cx.has_pending_prompt(),
        "Should have no prompts after deletion operation closes the file"
    );
    workspace
        .read_with(cx, |workspace, cx| {
            let open_project_paths = workspace
                .panes()
                .iter()
                .filter_map(|pane| pane.read(cx).active_item()?.project_path(cx))
                .collect::<Vec<_>>();
            assert!(
                open_project_paths.is_empty(),
                "Deleted file's buffer should be closed, but got open files: {open_project_paths:?}"
            );
        })
        .unwrap();
}

struct TestProjectItemView {
    focus_handle: FocusHandle,
    path: ProjectPath,
}

struct TestProjectItem {
    path: ProjectPath,
}

impl project::ProjectItem for TestProjectItem {
    fn try_open(
        _project: &Entity<Project>,
        path: &ProjectPath,
        cx: &mut App,
    ) -> Option<Task<anyhow::Result<Entity<Self>>>> {
        let path = path.clone();
        Some(cx.spawn(async move |cx| cx.new(|_| Self { path })))
    }

    fn entry_id(&self, _: &App) -> Option<ProjectEntryId> {
        None
    }

    fn project_path(&self, _: &App) -> Option<ProjectPath> {
        Some(self.path.clone())
    }

    fn is_dirty(&self) -> bool {
        false
    }
}

impl ProjectItem for TestProjectItemView {
    type Item = TestProjectItem;

    fn for_project_item(
        _: Entity<Project>,
        _: Option<&Pane>,
        project_item: Entity<Self::Item>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self
    where
        Self: Sized,
    {
        Self {
            path: project_item.update(cx, |project_item, _| project_item.path.clone()),
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Item for TestProjectItemView {
    type Event = ();

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Test".into()
    }
}

impl EventEmitter<()> for TestProjectItemView {}

impl Focusable for TestProjectItemView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TestProjectItemView {
    fn render(&mut self, _window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}
