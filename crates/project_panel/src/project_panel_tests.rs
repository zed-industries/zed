use super::*;
use collections::HashSet;
use gpui::{Empty, Entity, TestAppContext, VisualTestContext, WindowHandle};
use pretty_assertions::assert_eq;
use project::{FakeFs, WorktreeSettings};
use serde_json::json;
use settings::SettingsStore;
use std::path::{Path, PathBuf};
use util::{path, separator};
use workspace::{
    AppState, Pane,
    item::{Item, ProjectItem},
    register_project_item,
};

#[gpui::test]
async fn test_visible_list(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor().clone());
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

    let fs = FakeFs::new(cx.executor().clone());
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
            store.update_user_settings::<WorktreeSettings>(cx, |worktree_settings| {
                worktree_settings.file_scan_exclusions =
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

    let fs = FakeFs::new(cx.executor().clone());
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
                ..settings
            },
            cx,
        );
    });
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            separator!("v root1"),
            separator!("    > dir_1/nested_dir_1/nested_dir_2/nested_dir_3"),
            separator!("v root2"),
            separator!("    > dir_2"),
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
            separator!("v root1"),
            separator!("    v dir_1/nested_dir_1/nested_dir_2/nested_dir_3  <== selected"),
            separator!("        > nested_dir_4/nested_dir_5"),
            separator!("          file_a.java"),
            separator!("          file_b.java"),
            separator!("          file_c.java"),
            separator!("v root2"),
            separator!("    > dir_2"),
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
            separator!("v root1"),
            separator!("    v dir_1/nested_dir_1/nested_dir_2/nested_dir_3"),
            separator!("        v nested_dir_4/nested_dir_5  <== selected"),
            separator!("              file_d.java"),
            separator!("          file_a.java"),
            separator!("          file_b.java"),
            separator!("          file_c.java"),
            separator!("v root2"),
            separator!("    > dir_2"),
        ]
    );
    toggle_expand_dir(&panel, "root2/dir_2", cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            separator!("v root1"),
            separator!("    v dir_1/nested_dir_1/nested_dir_2/nested_dir_3"),
            separator!("        v nested_dir_4/nested_dir_5"),
            separator!("              file_d.java"),
            separator!("          file_a.java"),
            separator!("          file_b.java"),
            separator!("          file_c.java"),
            separator!("v root2"),
            separator!("    v dir_2  <== selected"),
            separator!("          file_1.java"),
        ]
    );
}

#[gpui::test(iterations = 30)]
async fn test_editing_files(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor().clone());
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
        panel.confirm_edit(window, cx).unwrap()
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
            panel.confirm_edit(window, cx).unwrap()
        })
        .await
        .unwrap();
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
            let file_name_selections = editor.selections.all::<usize>(cx);
            assert_eq!(
                file_name_selections.len(),
                1,
                "File editing should have a single selection, but got: {file_name_selections:?}"
            );
            let file_name_selection = &file_name_selections[0];
            assert_eq!(
                file_name_selection.start, 0,
                "Should select the file name from the start"
            );
            assert_eq!(
                file_name_selection.end,
                "another-filename".len(),
                "Should not select file extension"
            );

            editor.set_text("a-different-filename.tar.gz", window, cx)
        });
        panel.confirm_edit(window, cx).unwrap()
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
                let file_name_selections = editor.selections.all::<usize>(cx);
                assert_eq!(file_name_selections.len(), 1, "File editing should have a single selection, but got: {file_name_selections:?}");
                let file_name_selection = &file_name_selections[0];
                assert_eq!(file_name_selection.start, 0, "Should select the file name from the start");
                assert_eq!(file_name_selection.end, "a-different-filename.tar".len(), "Should not select file extension, but still may select anything up to the last dot..");

            });
            panel.cancel(&menu::Cancel, window, cx)
        });

    panel.update_in(cx, |panel, window, cx| {
        panel.new_directory(&NewDirectory, window, cx)
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
            "        > [EDITOR: '']  <== selected",
            "          a-different-filename.tar.gz",
            "    > C",
            "      .dockerignore",
        ]
    );

    let confirm = panel.update_in(cx, |panel, window, cx| {
        panel
            .filename_editor
            .update(cx, |editor, cx| editor.set_text("new-dir", window, cx));
        panel.confirm_edit(window, cx).unwrap()
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
            "        > 3",
            "        > 4",
            "        > [PROCESSING: 'new-dir']",
            "          a-different-filename.tar.gz  <== selected",
            "    > C",
            "      .dockerignore",
        ]
    );

    confirm.await.unwrap();
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "    > a",
            "    v b",
            "        > 3",
            "        > 4",
            "        > new-dir",
            "          a-different-filename.tar.gz  <== selected",
            "    > C",
            "      .dockerignore",
        ]
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.rename(&Default::default(), window, cx)
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
            "        > new-dir",
            "          [EDITOR: 'a-different-filename.tar.gz']  <== selected",
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
            "        > 3",
            "        > 4",
            "        > new-dir",
            "          a-different-filename.tar.gz  <== selected",
            "    > C",
            "      .dockerignore",
        ]
    );

    // Test empty filename and filename with only whitespace
    panel.update_in(cx, |panel, window, cx| panel.new_file(&NewFile, window, cx));
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "    > a",
            "    v b",
            "        > 3",
            "        > 4",
            "        > new-dir",
            "          [EDITOR: '']  <== selected",
            "          a-different-filename.tar.gz",
            "    > C",
        ]
    );
    panel.update_in(cx, |panel, window, cx| {
        panel.filename_editor.update(cx, |editor, cx| {
            editor.set_text("", window, cx);
        });
        assert!(panel.confirm_edit(window, cx).is_none());
        panel.filename_editor.update(cx, |editor, cx| {
            editor.set_text("   ", window, cx);
        });
        assert!(panel.confirm_edit(window, cx).is_none());
        panel.cancel(&menu::Cancel, window, cx)
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
            "        > new-dir",
            "          a-different-filename.tar.gz  <== selected",
            "    > C",
            "      .dockerignore",
        ]
    );
}

#[gpui::test(iterations = 10)]
async fn test_adding_directories_via_file(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor().clone());
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
            editor.set_text("/bdir1/dir2/the-new-filename", window, cx)
        });
        panel.confirm_edit(window, cx).unwrap()
    });

    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "    > a",
            "    > b",
            "    > C",
            "      [PROCESSING: '/bdir1/dir2/the-new-filename']  <== selected",
            "      .dockerignore",
            "v root2",
            "    > d",
            "    > e",
        ]
    );

    confirm.await.unwrap();
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

    let fs = FakeFs::new(cx.executor().clone());
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

    select_path(&panel, "root1", cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &["v root1  <== selected", "    > .git", "      .dockerignore",]
    );

    // Add a file with the root folder selected. The filename editor is placed
    // before the first file in the root folder.
    panel.update_in(cx, |panel, window, cx| panel.new_file(&NewFile, window, cx));
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
        panel.confirm_edit(window, cx).unwrap()
    });

    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            "v root1",
            "    > .git",
            "      [PROCESSING: 'new_dir/']  <== selected",
            "      .dockerignore",
        ]
    );

    confirm.await.unwrap();
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
        panel.confirm_edit(window, cx).unwrap()
    });
    confirm.await.unwrap();
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
            panel.confirm_edit(window, cx).unwrap()
        });
        confirm.await.unwrap();
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

    let fs = FakeFs::new(cx.executor().clone());
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
            let file_name_selections = editor.selections.all::<usize>(cx);
            assert_eq!(
                file_name_selections.len(),
                1,
                "File editing should have a single selection, but got: {file_name_selections:?}"
            );
            let file_name_selection = &file_name_selections[0];
            assert_eq!(
                file_name_selection.start,
                "one".len(),
                "Should select the file name disambiguation after the original file name"
            );
            assert_eq!(
                file_name_selection.end,
                "one copy".len(),
                "Should select the file name disambiguation until the extension"
            );
        });
        assert!(panel.confirm_edit(window, cx).is_none());
    });

    panel.update_in(cx, |panel, window, cx| {
        panel.paste(&Default::default(), window, cx);
    });
    cx.executor().run_until_parked();

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
        assert!(panel.confirm_edit(window, cx).is_none())
    });
}

#[gpui::test]
async fn test_cut_paste_between_different_worktrees(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor().clone());
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

    let fs = FakeFs::new(cx.executor().clone());
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

    let fs = FakeFs::new(cx.executor().clone());
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
        panel.confirm_edit(window, cx).unwrap()
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

    let fs = FakeFs::new(cx.executor().clone());
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

    let fs = FakeFs::new(cx.executor().clone());
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

    let fs = FakeFs::new(cx.executor().clone());
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
async fn test_create_duplicate_items(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor().clone());
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

    select_path(&panel, "src/", cx);
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
        panel
            .filename_editor
            .update(cx, |editor, cx| editor.set_text("test", window, cx));
        assert!(
            panel.confirm_edit(window, cx).is_none(),
            "Should not allow to confirm on conflicting new directory name"
        );
    });
    cx.executor().run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(
            panel.edit_state.is_some(),
            "Edit state should not be None after conflicting new directory name"
        );
        panel.cancel(&menu::Cancel, window, cx);
    });
    assert_eq!(
        visible_entries_as_strings(&panel, 0..10, cx),
        &[
            //
            "v src  <== selected",
            "    > test"
        ],
        "File list should be unchanged after failed folder create confirmation"
    );

    select_path(&panel, "src/test/", cx);
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
            panel.confirm_edit(window, cx).is_none(),
            "Should not allow to confirm on conflicting new file name"
        );
    });
    cx.executor().run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(
            panel.edit_state.is_some(),
            "Edit state should not be None after conflicting new file name"
        );
        panel.cancel(&menu::Cancel, window, cx);
    });
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
            panel.confirm_edit(window, cx).is_none(),
            "Should not allow to confirm on conflicting file rename"
        )
    });
    cx.executor().run_until_parked();
    panel.update_in(cx, |panel, window, cx| {
        assert!(
            panel.edit_state.is_some(),
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

#[gpui::test]
async fn test_select_git_entry(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor().clone());
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
    fs.set_git_content_for_repo(
        path!("/root/tree1/.git").as_ref(),
        &[
            ("dir1/modified1.txt".into(), "modified".into(), None),
            ("dir1/modified2.txt".into(), "modified".into(), None),
            ("modified4.txt".into(), "modified".into(), None),
            ("dir2/modified3.txt".into(), "modified".into(), None),
        ],
    );
    fs.set_git_content_for_repo(
        path!("/root/tree2/.git").as_ref(),
        &[
            ("dir3/modified5.txt".into(), "modified".into(), None),
            ("modified6.txt".into(), "modified".into(), None),
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

    assert_eq!(
        visible_entries_as_strings(&panel, 9..11, cx),
        &["      modified4.txt  <== selected", "      unmodified3.txt",],
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.select_next_git_entry(&SelectNextGitEntry, window, cx);
    });

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

    assert_eq!(
        visible_entries_as_strings(&panel, 16..18, cx),
        &["      modified6.txt  <== selected", "      unmodified5.txt",],
    );

    // Wraps around to first modified file
    panel.update_in(cx, |panel, window, cx| {
        panel.select_next_git_entry(&SelectNextGitEntry, window, cx);
    });

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

    assert_eq!(
        visible_entries_as_strings(&panel, 16..18, cx),
        &["      modified6.txt  <== selected", "      unmodified5.txt",],
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.select_prev_git_entry(&SelectPrevGitEntry, window, cx);
    });

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

    assert_eq!(
        visible_entries_as_strings(&panel, 9..11, cx),
        &["      modified4.txt  <== selected", "      unmodified3.txt",],
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.select_prev_git_entry(&SelectPrevGitEntry, window, cx);
    });

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

    let fs = FakeFs::new(cx.executor().clone());
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

    let fs = FakeFs::new(cx.executor().clone());
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
}

#[gpui::test]
async fn test_dir_toggle_collapse(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor().clone());
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

    let fs = FakeFs::new(cx.executor().clone());
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
async fn test_new_file_move(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor().clone());
    fs.as_fake().insert_tree(path!("/root"), json!({})).await;
    let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let panel = workspace.update(cx, ProjectPanel::new).unwrap();

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

#[gpui::test]
#[cfg_attr(target_os = "windows", ignore)]
async fn test_rename_root_of_worktree(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor().clone());
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
        panel.confirm_edit(window, cx).unwrap()
    });
    confirm.await.unwrap();
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
async fn test_multiple_marked_entries(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);
    let fs = FakeFs::new(cx.executor().clone());
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
    cx.update(|window, cx| {
        panel.update(cx, |this, cx| {
            this.select_next(&Default::default(), window, cx);
            this.expand_selected_entry(&Default::default(), window, cx);
            this.expand_selected_entry(&Default::default(), window, cx);
            this.select_next(&Default::default(), window, cx);
            this.expand_selected_entry(&Default::default(), window, cx);
            this.select_next(&Default::default(), window, cx);
        })
    });
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
                active_selection: this.selection.unwrap(),
                marked_selections: Arc::new(this.marked_entries.clone()),
            };
            let target_entry = this
                .project
                .read(cx)
                .entry_for_path(&(worktree_id, "").into(), cx)
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
            // this.expand_selected_entry(&ExpandSelectedEntry, cx);
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
async fn test_autoreveal_and_gitignored_files(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);
    cx.update(|cx| {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings::<WorktreeSettings>(cx, |worktree_settings| {
                worktree_settings.file_scan_exclusions = Some(Vec::new());
            });
            store.update_user_settings::<ProjectPanelSettings>(cx, |project_panel_settings| {
                project_panel_settings.auto_reveal_entries = Some(false)
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
            store.update_user_settings::<ProjectPanelSettings>(cx, |project_panel_settings| {
                project_panel_settings.auto_reveal_entries = Some(true)
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
            store.update_user_settings::<WorktreeSettings>(cx, |worktree_settings| {
                worktree_settings.file_scan_exclusions = Some(Vec::new());
                worktree_settings.file_scan_inclusions =
                    Some(vec!["always_included_but_ignored_dir/*".to_string()]);
            });
            store.update_user_settings::<ProjectPanelSettings>(cx, |project_panel_settings| {
                project_panel_settings.auto_reveal_entries = Some(false)
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
            store.update_user_settings::<ProjectPanelSettings>(cx, |project_panel_settings| {
                project_panel_settings.auto_reveal_entries = Some(true)
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
            store.update_user_settings::<WorktreeSettings>(cx, |worktree_settings| {
                worktree_settings.file_scan_exclusions = Some(Vec::new());
            });
            store.update_user_settings::<ProjectPanelSettings>(cx, |project_panel_settings| {
                project_panel_settings.auto_reveal_entries = Some(false)
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
            store.update_user_settings::<WorktreeSettings>(cx, |project_settings| {
                project_settings.file_scan_exclusions =
                    Some(vec!["excluded_dir".to_string(), "**/.git".to_string()]);
            });
        });
    });

    cx.update(|cx| {
        register_project_item::<TestProjectItemView>(cx);
    });

    let fs = FakeFs::new(cx.executor().clone());
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
    panel.update_in(cx, |panel, window, cx| {
        assert!(panel.filename_editor.read(cx).is_focused(window));
    });
    panel
        .update_in(cx, |panel, window, cx| {
            panel.filename_editor.update(cx, |editor, cx| {
                editor.set_text(excluded_file_path, window, cx)
            });
            panel.confirm_edit(window, cx).unwrap()
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
                Path::new(excluded_file_path),
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
    panel.update_in(cx, |panel, window, cx| {
        assert!(panel.filename_editor.read(cx).is_focused(window));
    });
    panel
        .update_in(cx, |panel, window, cx| {
            panel.filename_editor.update(cx, |editor, cx| {
                editor.set_text(excluded_file_path, window, cx)
            });
            panel.confirm_edit(window, cx).unwrap()
        })
        .await
        .unwrap();

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
    panel.update_in(cx, |panel, window, cx| {
        assert!(panel.filename_editor.read(cx).is_focused(window));
    });
    panel
        .update_in(cx, |panel, window, cx| {
            panel.filename_editor.update(cx, |editor, cx| {
                editor.set_text(excluded_dir_path, window, cx)
            });
            panel.confirm_edit(window, cx).unwrap()
        })
        .await
        .unwrap();

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

    let fs = FakeFs::new(cx.executor().clone());
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

    select_path(&panel, "src/", cx);
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
        panel.cancel(&menu::Cancel, window, cx)
    });
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

    let fs = FakeFs::new(cx.executor().clone());
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

    let fs = FakeFs::new(cx.executor().clone());
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

    let fs = FakeFs::new(cx.executor().clone());
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

    let fs = FakeFs::new(cx.executor().clone());
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

    let fs = FakeFs::new(cx.executor().clone());
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

    let fs = FakeFs::new(cx.executor().clone());
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

    let fs = FakeFs::new(cx.executor().clone());
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

    let fs = FakeFs::new(cx.executor().clone());
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

    let fs = FakeFs::new(cx.executor().clone());
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

fn toggle_expand_dir(
    panel: &Entity<ProjectPanel>,
    path: impl AsRef<Path>,
    cx: &mut VisualTestContext,
) {
    let path = path.as_ref();
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
}

#[gpui::test]
async fn test_expand_all_for_entry(cx: &mut gpui::TestAppContext) {
    init_test_with_editor(cx);

    let fs = FakeFs::new(cx.executor().clone());
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

    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &["v root", "    > dir1", "      .gitignore",],
        "Initial state should show collapsed root structure"
    );

    toggle_expand_dir(&panel, "root/dir1", cx);
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            separator!("v root"),
            separator!("    v dir1  <== selected"),
            separator!("        > empty1/empty2/empty3"),
            separator!("        > ignored_dir"),
            separator!("        > subdir1"),
            separator!("      .gitignore"),
        ],
        "Should show first level with auto-folded dirs and ignored dir visible"
    );

    let entry_id = find_project_entry(&panel, "root/dir1", cx).unwrap();
    panel.update(cx, |panel, cx| {
        let project = panel.project.read(cx);
        let worktree = project.worktrees(cx).next().unwrap().read(cx);
        panel.expand_all_for_entry(worktree.id(), entry_id, cx);
        panel.update_visible_entries(None, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            separator!("v root"),
            separator!("    v dir1  <== selected"),
            separator!("        v empty1"),
            separator!("            v empty2"),
            separator!("                v empty3"),
            separator!("                      file.txt"),
            separator!("        > ignored_dir"),
            separator!("        v subdir1"),
            separator!("            > ignored_nested"),
            separator!("              file1.txt"),
            separator!("              file2.txt"),
            separator!("      .gitignore"),
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
            separator!("v root"),
            separator!("    v dir1  <== selected"),
            separator!("        > empty1"),
            separator!("        > ignored_dir"),
            separator!("        > subdir1"),
            separator!("      .gitignore"),
        ],
        "With auto-fold disabled: should show all directories separately"
    );

    let entry_id = find_project_entry(&panel, "root/dir1", cx).unwrap();
    panel.update(cx, |panel, cx| {
        let project = panel.project.read(cx);
        let worktree = project.worktrees(cx).next().unwrap().read(cx);
        panel.expand_all_for_entry(worktree.id(), entry_id, cx);
        panel.update_visible_entries(None, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            separator!("v root"),
            separator!("    v dir1  <== selected"),
            separator!("        v empty1"),
            separator!("            v empty2"),
            separator!("                v empty3"),
            separator!("                      file.txt"),
            separator!("        > ignored_dir"),
            separator!("        v subdir1"),
            separator!("            > ignored_nested"),
            separator!("              file1.txt"),
            separator!("              file2.txt"),
            separator!("      .gitignore"),
        ],
        "After expand_all without auto-fold: should expand all dirs normally, \
         expand ignored_dir itself but not its subdirs, and not expand ignored_nested"
    );

    // Test 3: When explicitly called on ignored directory
    let ignored_dir_entry = find_project_entry(&panel, "root/dir1/ignored_dir", cx).unwrap();
    panel.update(cx, |panel, cx| {
        let project = panel.project.read(cx);
        let worktree = project.worktrees(cx).next().unwrap().read(cx);
        panel.expand_all_for_entry(worktree.id(), ignored_dir_entry, cx);
        panel.update_visible_entries(None, cx);
    });
    cx.run_until_parked();

    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            separator!("v root"),
            separator!("    v dir1  <== selected"),
            separator!("        v empty1"),
            separator!("            v empty2"),
            separator!("                v empty3"),
            separator!("                      file.txt"),
            separator!("        v ignored_dir"),
            separator!("            v subdir"),
            separator!("                  deep_file.txt"),
            separator!("        v subdir1"),
            separator!("            > ignored_nested"),
            separator!("              file1.txt"),
            separator!("              file2.txt"),
            separator!("      .gitignore"),
        ],
        "After expand_all on ignored_dir: should expand all contents of the ignored directory"
    );
}

#[gpui::test]
async fn test_collapse_all_for_entry(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor().clone());
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

        toggle_expand_dir(&panel, "root/dir1", cx);
        toggle_expand_dir(&panel, "root/dir1/subdir1", cx);
        toggle_expand_dir(&panel, "root/dir1/subdir1/nested1", cx);
        toggle_expand_dir(&panel, "root/dir1/subdir2", cx);

        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                separator!("v root"),
                separator!("    v dir1"),
                separator!("        v subdir1"),
                separator!("            v nested1"),
                separator!("                  file1.txt"),
                separator!("                  file2.txt"),
                separator!("        v subdir2  <== selected"),
                separator!("              file4.txt"),
                separator!("    > dir2"),
            ],
            "Initial state with everything expanded"
        );

        let entry_id = find_project_entry(&panel, "root/dir1", cx).unwrap();
        panel.update(cx, |panel, cx| {
            let project = panel.project.read(cx);
            let worktree = project.worktrees(cx).next().unwrap().read(cx);
            panel.collapse_all_for_entry(worktree.id(), entry_id, cx);
            panel.update_visible_entries(None, cx);
        });

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

        toggle_expand_dir(&panel, "root/dir1", cx);
        toggle_expand_dir(&panel, "root/dir1/subdir1", cx);
        toggle_expand_dir(&panel, "root/dir1/subdir1/nested1", cx);

        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                separator!("v root"),
                separator!("    v dir1"),
                separator!("        v subdir1/nested1  <== selected"),
                separator!("              file1.txt"),
                separator!("              file2.txt"),
                separator!("        > subdir2"),
                separator!("    > dir2/single_file"),
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
                separator!("v root"),
                separator!("    v dir1  <== selected"),
                separator!("        > subdir1/nested1"),
                separator!("        > subdir2"),
                separator!("    > dir2/single_file"),
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

        toggle_expand_dir(&panel, "root/dir1", cx);
        toggle_expand_dir(&panel, "root/dir1/subdir1", cx);
        toggle_expand_dir(&panel, "root/dir1/subdir1/nested1", cx);

        assert_eq!(
            visible_entries_as_strings(&panel, 0..20, cx),
            &[
                separator!("v root"),
                separator!("    v dir1"),
                separator!("        v subdir1"),
                separator!("            v nested1  <== selected"),
                separator!("                  file1.txt"),
                separator!("                  file2.txt"),
                separator!("        > subdir2"),
                separator!("    > dir2"),
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
                separator!("v root"),
                separator!("    v dir1  <== selected"),
                separator!("        > subdir1"),
                separator!("        > subdir2"),
                separator!("    > dir2"),
            ],
            "Subdirs should be collapsed but not folded with auto-fold disabled"
        );
    }
}

#[gpui::test]
async fn test_create_entries_without_selection(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor().clone());
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

    #[rustfmt::skip]
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            separator!("v root"),
            separator!("    > dir1"),
        ],
        "Initial state with nothing selected"
    );

    panel.update_in(cx, |panel, window, cx| {
        panel.new_file(&NewFile, window, cx);
    });
    panel.update_in(cx, |panel, window, cx| {
        assert!(panel.filename_editor.read(cx).is_focused(window));
    });
    panel
        .update_in(cx, |panel, window, cx| {
            panel.filename_editor.update(cx, |editor, cx| {
                editor.set_text("hello_from_no_selections", window, cx)
            });
            panel.confirm_edit(window, cx).unwrap()
        })
        .await
        .unwrap();

    #[rustfmt::skip]
    assert_eq!(
        visible_entries_as_strings(&panel, 0..20, cx),
        &[
            separator!("v root"),
            separator!("    > dir1"),
            separator!("      hello_from_no_selections  <== selected  <== marked"),
        ],
        "A new file is created under the root directory"
    );
}

fn select_path(panel: &Entity<ProjectPanel>, path: impl AsRef<Path>, cx: &mut VisualTestContext) {
    let path = path.as_ref();
    panel.update(cx, |panel, cx| {
        for worktree in panel.project.read(cx).worktrees(cx).collect::<Vec<_>>() {
            let worktree = worktree.read(cx);
            if let Ok(relative_path) = path.strip_prefix(worktree.root_name()) {
                let entry_id = worktree.entry_for_path(relative_path).unwrap().id;
                panel.selection = Some(crate::SelectedEntry {
                    worktree_id: worktree.id(),
                    entry_id,
                });
                return;
            }
        }
        panic!("no worktree for path {:?}", path);
    });
}

fn select_path_with_mark(
    panel: &Entity<ProjectPanel>,
    path: impl AsRef<Path>,
    cx: &mut VisualTestContext,
) {
    let path = path.as_ref();
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
                    panel.marked_entries.insert(entry);
                }
                panel.selection = Some(entry);
                return;
            }
        }
        panic!("no worktree for path {:?}", path);
    });
}

fn find_project_entry(
    panel: &Entity<ProjectPanel>,
    path: impl AsRef<Path>,
    cx: &mut VisualTestContext,
) -> Option<ProjectEntryId> {
    let path = path.as_ref();
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
            let name = if details.is_editing {
                format!("[EDITOR: '{}']", details.filename)
            } else if details.is_processing {
                format!("[PROCESSING: '{}']", details.filename)
            } else {
                details.filename.clone()
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

fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        init_settings(cx);
        theme::init(theme::LoadThemes::JustBase, cx);
        language::init(cx);
        editor::init_settings(cx);
        crate::init(cx);
        workspace::init_settings(cx);
        client::init_settings(cx);
        Project::init_settings(cx);

        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings::<ProjectPanelSettings>(cx, |project_panel_settings| {
                project_panel_settings.auto_fold_dirs = Some(false);
            });
            store.update_user_settings::<WorktreeSettings>(cx, |worktree_settings| {
                worktree_settings.file_scan_exclusions = Some(Vec::new());
            });
        });
    });
}

fn init_test_with_editor(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let app_state = AppState::test(cx);
        theme::init(theme::LoadThemes::JustBase, cx);
        init_settings(cx);
        language::init(cx);
        editor::init(cx);
        crate::init(cx);
        workspace::init(app_state.clone(), cx);
        Project::init_settings(cx);

        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings::<ProjectPanelSettings>(cx, |project_panel_settings| {
                project_panel_settings.auto_fold_dirs = Some(false);
            });
            store.update_user_settings::<WorktreeSettings>(cx, |worktree_settings| {
                worktree_settings.file_scan_exclusions = Some(Vec::new());
            });
        });
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
                    path: Arc::from(Path::new(expected_path))
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
    ) -> Option<Task<gpui::Result<Entity<Self>>>> {
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
        _: &Pane,
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
