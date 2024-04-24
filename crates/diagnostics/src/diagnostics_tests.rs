use super::*;
use editor::{
    display_map::{BlockContext, TransformBlock},
    DisplayPoint, GutterDimensions,
};
use gpui::{px, Stateful, TestAppContext, VisualTestContext, WindowContext};
use language::{Diagnostic, DiagnosticEntry, DiagnosticSeverity, PointUtf16, Unclipped};
use project::FakeFs;
use serde_json::json;
use settings::SettingsStore;
use unindent::Unindent as _;

#[gpui::test]
async fn test_diagnostics(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/test",
        json!({
            "consts.rs": "
                    const a: i32 = 'a';
                    const b: i32 = c;
                "
            .unindent(),

            "main.rs": "
                    fn main() {
                        let x = vec![];
                        let y = vec![];
                        a(x);
                        b(y);
                        // comment 1
                        // comment 2
                        c(y);
                        d(x);
                    }
                "
            .unindent(),
        }),
    )
    .await;

    let language_server_id = LanguageServerId(0);
    let project = Project::test(fs.clone(), ["/test".as_ref()], cx).await;
    let window = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
    let cx = &mut VisualTestContext::from_window(*window, cx);
    let workspace = window.root(cx).unwrap();

    // Create some diagnostics
    project.update(cx, |project, cx| {
            project
                .update_diagnostic_entries(
                    language_server_id,
                    PathBuf::from("/test/main.rs"),
                    None,
                    vec![
                        DiagnosticEntry {
                            range: Unclipped(PointUtf16::new(1, 8))..Unclipped(PointUtf16::new(1, 9)),
                            diagnostic: Diagnostic {
                                message:
                                    "move occurs because `x` has type `Vec<char>`, which does not implement the `Copy` trait"
                                        .to_string(),
                                severity: DiagnosticSeverity::INFORMATION,
                                is_primary: false,
                                is_disk_based: true,
                                group_id: 1,
                                ..Default::default()
                            },
                        },
                        DiagnosticEntry {
                            range: Unclipped(PointUtf16::new(2, 8))..Unclipped(PointUtf16::new(2, 9)),
                            diagnostic: Diagnostic {
                                message:
                                    "move occurs because `y` has type `Vec<char>`, which does not implement the `Copy` trait"
                                        .to_string(),
                                severity: DiagnosticSeverity::INFORMATION,
                                is_primary: false,
                                is_disk_based: true,
                                group_id: 0,
                                ..Default::default()
                            },
                        },
                        DiagnosticEntry {
                            range: Unclipped(PointUtf16::new(3, 6))..Unclipped(PointUtf16::new(3, 7)),
                            diagnostic: Diagnostic {
                                message: "value moved here".to_string(),
                                severity: DiagnosticSeverity::INFORMATION,
                                is_primary: false,
                                is_disk_based: true,
                                group_id: 1,
                                ..Default::default()
                            },
                        },
                        DiagnosticEntry {
                            range: Unclipped(PointUtf16::new(4, 6))..Unclipped(PointUtf16::new(4, 7)),
                            diagnostic: Diagnostic {
                                message: "value moved here".to_string(),
                                severity: DiagnosticSeverity::INFORMATION,
                                is_primary: false,
                                is_disk_based: true,
                                group_id: 0,
                                ..Default::default()
                            },
                        },
                        DiagnosticEntry {
                            range: Unclipped(PointUtf16::new(7, 6))..Unclipped(PointUtf16::new(7, 7)),
                            diagnostic: Diagnostic {
                                message: "use of moved value\nvalue used here after move".to_string(),
                                severity: DiagnosticSeverity::ERROR,
                                is_primary: true,
                                is_disk_based: true,
                                group_id: 0,
                                ..Default::default()
                            },
                        },
                        DiagnosticEntry {
                            range: Unclipped(PointUtf16::new(8, 6))..Unclipped(PointUtf16::new(8, 7)),
                            diagnostic: Diagnostic {
                                message: "use of moved value\nvalue used here after move".to_string(),
                                severity: DiagnosticSeverity::ERROR,
                                is_primary: true,
                                is_disk_based: true,
                                group_id: 1,
                                ..Default::default()
                            },
                        },
                    ],
                    cx,
                )
                .unwrap();
        });

    // Open the project diagnostics view while there are already diagnostics.
    let view = window.build_view(cx, |cx| {
        ProjectDiagnosticsEditor::new_with_context(1, project.clone(), workspace.downgrade(), cx)
    });

    view.next_notification(cx).await;
    view.update(cx, |view, cx| {
        assert_eq!(
            editor_blocks(&view.editor, cx),
            [
                (0, "path header block".into()),
                (2, "diagnostic header".into()),
                (15, "collapsed context".into()),
                (16, "diagnostic header".into()),
                (25, "collapsed context".into()),
            ]
        );
        assert_eq!(
            view.editor.update(cx, |editor, cx| editor.display_text(cx)),
            concat!(
                //
                // main.rs
                //
                "\n", // filename
                "\n", // padding
                // diagnostic group 1
                "\n", // primary message
                "\n", // padding
                "    let x = vec![];\n",
                "    let y = vec![];\n",
                "\n", // supporting diagnostic
                "    a(x);\n",
                "    b(y);\n",
                "\n", // supporting diagnostic
                "    // comment 1\n",
                "    // comment 2\n",
                "    c(y);\n",
                "\n", // supporting diagnostic
                "    d(x);\n",
                "\n", // context ellipsis
                // diagnostic group 2
                "\n", // primary message
                "\n", // padding
                "fn main() {\n",
                "    let x = vec![];\n",
                "\n", // supporting diagnostic
                "    let y = vec![];\n",
                "    a(x);\n",
                "\n", // supporting diagnostic
                "    b(y);\n",
                "\n", // context ellipsis
                "    c(y);\n",
                "    d(x);\n",
                "\n", // supporting diagnostic
                "}"
            )
        );

        // Cursor is at the first diagnostic
        view.editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.selections.display_ranges(cx),
                [DisplayPoint::new(12, 6)..DisplayPoint::new(12, 6)]
            );
        });
    });

    // Diagnostics are added for another earlier path.
    project.update(cx, |project, cx| {
        project.disk_based_diagnostics_started(language_server_id, cx);
        project
            .update_diagnostic_entries(
                language_server_id,
                PathBuf::from("/test/consts.rs"),
                None,
                vec![DiagnosticEntry {
                    range: Unclipped(PointUtf16::new(0, 15))..Unclipped(PointUtf16::new(0, 15)),
                    diagnostic: Diagnostic {
                        message: "mismatched types\nexpected `usize`, found `char`".to_string(),
                        severity: DiagnosticSeverity::ERROR,
                        is_primary: true,
                        is_disk_based: true,
                        group_id: 0,
                        ..Default::default()
                    },
                }],
                cx,
            )
            .unwrap();
        project.disk_based_diagnostics_finished(language_server_id, cx);
    });

    view.next_notification(cx).await;
    view.update(cx, |view, cx| {
        assert_eq!(
            editor_blocks(&view.editor, cx),
            [
                (0, "path header block".into()),
                (2, "diagnostic header".into()),
                (7, "path header block".into()),
                (9, "diagnostic header".into()),
                (22, "collapsed context".into()),
                (23, "diagnostic header".into()),
                (32, "collapsed context".into()),
            ]
        );
        assert_eq!(
            view.editor.update(cx, |editor, cx| editor.display_text(cx)),
            concat!(
                //
                // consts.rs
                //
                "\n", // filename
                "\n", // padding
                // diagnostic group 1
                "\n", // primary message
                "\n", // padding
                "const a: i32 = 'a';\n",
                "\n", // supporting diagnostic
                "const b: i32 = c;\n",
                //
                // main.rs
                //
                "\n", // filename
                "\n", // padding
                // diagnostic group 1
                "\n", // primary message
                "\n", // padding
                "    let x = vec![];\n",
                "    let y = vec![];\n",
                "\n", // supporting diagnostic
                "    a(x);\n",
                "    b(y);\n",
                "\n", // supporting diagnostic
                "    // comment 1\n",
                "    // comment 2\n",
                "    c(y);\n",
                "\n", // supporting diagnostic
                "    d(x);\n",
                "\n", // collapsed context
                // diagnostic group 2
                "\n", // primary message
                "\n", // filename
                "fn main() {\n",
                "    let x = vec![];\n",
                "\n", // supporting diagnostic
                "    let y = vec![];\n",
                "    a(x);\n",
                "\n", // supporting diagnostic
                "    b(y);\n",
                "\n", // context ellipsis
                "    c(y);\n",
                "    d(x);\n",
                "\n", // supporting diagnostic
                "}"
            )
        );

        // Cursor keeps its position.
        view.editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.selections.display_ranges(cx),
                [DisplayPoint::new(19, 6)..DisplayPoint::new(19, 6)]
            );
        });
    });

    // Diagnostics are added to the first path
    project.update(cx, |project, cx| {
        project.disk_based_diagnostics_started(language_server_id, cx);
        project
            .update_diagnostic_entries(
                language_server_id,
                PathBuf::from("/test/consts.rs"),
                None,
                vec![
                    DiagnosticEntry {
                        range: Unclipped(PointUtf16::new(0, 15))..Unclipped(PointUtf16::new(0, 15)),
                        diagnostic: Diagnostic {
                            message: "mismatched types\nexpected `usize`, found `char`".to_string(),
                            severity: DiagnosticSeverity::ERROR,
                            is_primary: true,
                            is_disk_based: true,
                            group_id: 0,
                            ..Default::default()
                        },
                    },
                    DiagnosticEntry {
                        range: Unclipped(PointUtf16::new(1, 15))..Unclipped(PointUtf16::new(1, 15)),
                        diagnostic: Diagnostic {
                            message: "unresolved name `c`".to_string(),
                            severity: DiagnosticSeverity::ERROR,
                            is_primary: true,
                            is_disk_based: true,
                            group_id: 1,
                            ..Default::default()
                        },
                    },
                ],
                cx,
            )
            .unwrap();
        project.disk_based_diagnostics_finished(language_server_id, cx);
    });

    view.next_notification(cx).await;
    view.update(cx, |view, cx| {
        assert_eq!(
            editor_blocks(&view.editor, cx),
            [
                (0, "path header block".into()),
                (2, "diagnostic header".into()),
                (7, "collapsed context".into()),
                (8, "diagnostic header".into()),
                (13, "path header block".into()),
                (15, "diagnostic header".into()),
                (28, "collapsed context".into()),
                (29, "diagnostic header".into()),
                (38, "collapsed context".into()),
            ]
        );
        assert_eq!(
            view.editor.update(cx, |editor, cx| editor.display_text(cx)),
            concat!(
                //
                // consts.rs
                //
                "\n", // filename
                "\n", // padding
                // diagnostic group 1
                "\n", // primary message
                "\n", // padding
                "const a: i32 = 'a';\n",
                "\n", // supporting diagnostic
                "const b: i32 = c;\n",
                "\n", // context ellipsis
                // diagnostic group 2
                "\n", // primary message
                "\n", // padding
                "const a: i32 = 'a';\n",
                "const b: i32 = c;\n",
                "\n", // supporting diagnostic
                //
                // main.rs
                //
                "\n", // filename
                "\n", // padding
                // diagnostic group 1
                "\n", // primary message
                "\n", // padding
                "    let x = vec![];\n",
                "    let y = vec![];\n",
                "\n", // supporting diagnostic
                "    a(x);\n",
                "    b(y);\n",
                "\n", // supporting diagnostic
                "    // comment 1\n",
                "    // comment 2\n",
                "    c(y);\n",
                "\n", // supporting diagnostic
                "    d(x);\n",
                "\n", // context ellipsis
                // diagnostic group 2
                "\n", // primary message
                "\n", // filename
                "fn main() {\n",
                "    let x = vec![];\n",
                "\n", // supporting diagnostic
                "    let y = vec![];\n",
                "    a(x);\n",
                "\n", // supporting diagnostic
                "    b(y);\n",
                "\n", // context ellipsis
                "    c(y);\n",
                "    d(x);\n",
                "\n", // supporting diagnostic
                "}"
            )
        );
    });
}

#[gpui::test]
async fn test_diagnostics_multiple_servers(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/test",
        json!({
            "main.js": "
                    a();
                    b();
                    c();
                    d();
                    e();
                ".unindent()
        }),
    )
    .await;

    let server_id_1 = LanguageServerId(100);
    let server_id_2 = LanguageServerId(101);
    let project = Project::test(fs.clone(), ["/test".as_ref()], cx).await;
    let window = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
    let cx = &mut VisualTestContext::from_window(*window, cx);
    let workspace = window.root(cx).unwrap();

    let view = window.build_view(cx, |cx| {
        ProjectDiagnosticsEditor::new_with_context(1, project.clone(), workspace.downgrade(), cx)
    });

    // Two language servers start updating diagnostics
    project.update(cx, |project, cx| {
        project.disk_based_diagnostics_started(server_id_1, cx);
        project.disk_based_diagnostics_started(server_id_2, cx);
        project
            .update_diagnostic_entries(
                server_id_1,
                PathBuf::from("/test/main.js"),
                None,
                vec![DiagnosticEntry {
                    range: Unclipped(PointUtf16::new(0, 0))..Unclipped(PointUtf16::new(0, 1)),
                    diagnostic: Diagnostic {
                        message: "error 1".to_string(),
                        severity: DiagnosticSeverity::WARNING,
                        is_primary: true,
                        is_disk_based: true,
                        group_id: 1,
                        ..Default::default()
                    },
                }],
                cx,
            )
            .unwrap();
    });

    // The first language server finishes
    project.update(cx, |project, cx| {
        project.disk_based_diagnostics_finished(server_id_1, cx);
    });

    // Only the first language server's diagnostics are shown.
    cx.executor().run_until_parked();
    view.update(cx, |view, cx| {
        assert_eq!(
            editor_blocks(&view.editor, cx),
            [
                (0, "path header block".into()),
                (2, "diagnostic header".into()),
            ]
        );
        assert_eq!(
            view.editor.update(cx, |editor, cx| editor.display_text(cx)),
            concat!(
                "\n", // filename
                "\n", // padding
                // diagnostic group 1
                "\n",     // primary message
                "\n",     // padding
                "a();\n", //
                "b();",
            )
        );
    });

    // The second language server finishes
    project.update(cx, |project, cx| {
        project
            .update_diagnostic_entries(
                server_id_2,
                PathBuf::from("/test/main.js"),
                None,
                vec![DiagnosticEntry {
                    range: Unclipped(PointUtf16::new(1, 0))..Unclipped(PointUtf16::new(1, 1)),
                    diagnostic: Diagnostic {
                        message: "warning 1".to_string(),
                        severity: DiagnosticSeverity::ERROR,
                        is_primary: true,
                        is_disk_based: true,
                        group_id: 2,
                        ..Default::default()
                    },
                }],
                cx,
            )
            .unwrap();
        project.disk_based_diagnostics_finished(server_id_2, cx);
    });

    // Both language server's diagnostics are shown.
    cx.executor().run_until_parked();
    view.update(cx, |view, cx| {
        assert_eq!(
            editor_blocks(&view.editor, cx),
            [
                (0, "path header block".into()),
                (2, "diagnostic header".into()),
                (6, "collapsed context".into()),
                (7, "diagnostic header".into()),
            ]
        );
        assert_eq!(
            view.editor.update(cx, |editor, cx| editor.display_text(cx)),
            concat!(
                "\n", // filename
                "\n", // padding
                // diagnostic group 1
                "\n",     // primary message
                "\n",     // padding
                "a();\n", // location
                "b();\n", //
                "\n",     // collapsed context
                // diagnostic group 2
                "\n",     // primary message
                "\n",     // padding
                "a();\n", // context
                "b();\n", //
                "c();",   // context
            )
        );
    });

    // Both language servers start updating diagnostics, and the first server finishes.
    project.update(cx, |project, cx| {
        project.disk_based_diagnostics_started(server_id_1, cx);
        project.disk_based_diagnostics_started(server_id_2, cx);
        project
            .update_diagnostic_entries(
                server_id_1,
                PathBuf::from("/test/main.js"),
                None,
                vec![DiagnosticEntry {
                    range: Unclipped(PointUtf16::new(2, 0))..Unclipped(PointUtf16::new(2, 1)),
                    diagnostic: Diagnostic {
                        message: "warning 2".to_string(),
                        severity: DiagnosticSeverity::WARNING,
                        is_primary: true,
                        is_disk_based: true,
                        group_id: 1,
                        ..Default::default()
                    },
                }],
                cx,
            )
            .unwrap();
        project
            .update_diagnostic_entries(
                server_id_2,
                PathBuf::from("/test/main.rs"),
                None,
                vec![],
                cx,
            )
            .unwrap();
        project.disk_based_diagnostics_finished(server_id_1, cx);
    });

    // Only the first language server's diagnostics are updated.
    cx.executor().run_until_parked();
    view.update(cx, |view, cx| {
        assert_eq!(
            editor_blocks(&view.editor, cx),
            [
                (0, "path header block".into()),
                (2, "diagnostic header".into()),
                (7, "collapsed context".into()),
                (8, "diagnostic header".into()),
            ]
        );
        assert_eq!(
            view.editor.update(cx, |editor, cx| editor.display_text(cx)),
            concat!(
                "\n", // filename
                "\n", // padding
                // diagnostic group 1
                "\n",     // primary message
                "\n",     // padding
                "a();\n", // location
                "b();\n", //
                "c();\n", // context
                "\n",     // collapsed context
                // diagnostic group 2
                "\n",     // primary message
                "\n",     // padding
                "b();\n", // context
                "c();\n", //
                "d();",   // context
            )
        );
    });

    // The second language server finishes.
    project.update(cx, |project, cx| {
        project
            .update_diagnostic_entries(
                server_id_2,
                PathBuf::from("/test/main.js"),
                None,
                vec![DiagnosticEntry {
                    range: Unclipped(PointUtf16::new(3, 0))..Unclipped(PointUtf16::new(3, 1)),
                    diagnostic: Diagnostic {
                        message: "warning 2".to_string(),
                        severity: DiagnosticSeverity::WARNING,
                        is_primary: true,
                        is_disk_based: true,
                        group_id: 1,
                        ..Default::default()
                    },
                }],
                cx,
            )
            .unwrap();
        project.disk_based_diagnostics_finished(server_id_2, cx);
    });

    // Both language servers' diagnostics are updated.
    cx.executor().run_until_parked();
    view.update(cx, |view, cx| {
        assert_eq!(
            editor_blocks(&view.editor, cx),
            [
                (0, "path header block".into()),
                (2, "diagnostic header".into()),
                (7, "collapsed context".into()),
                (8, "diagnostic header".into()),
            ]
        );
        assert_eq!(
            view.editor.update(cx, |editor, cx| editor.display_text(cx)),
            concat!(
                "\n", // filename
                "\n", // padding
                // diagnostic group 1
                "\n",     // primary message
                "\n",     // padding
                "b();\n", // location
                "c();\n", //
                "d();\n", // context
                "\n",     // collapsed context
                // diagnostic group 2
                "\n",     // primary message
                "\n",     // padding
                "c();\n", // context
                "d();\n", //
                "e();",   // context
            )
        );
    });
}

fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let settings = SettingsStore::test(cx);
        cx.set_global(settings);
        theme::init(theme::LoadThemes::JustBase, cx);
        language::init(cx);
        client::init_settings(cx);
        workspace::init_settings(cx);
        Project::init_settings(cx);
        crate::init(cx);
        editor::init(cx);
    });
}

fn editor_blocks(editor: &View<Editor>, cx: &mut WindowContext) -> Vec<(u32, SharedString)> {
    editor.update(cx, |editor, cx| {
        let snapshot = editor.snapshot(cx);
        snapshot
            .blocks_in_range(0..snapshot.max_point().row())
            .enumerate()
            .filter_map(|(ix, (row, block))| {
                let name: SharedString = match block {
                    TransformBlock::Custom(block) => cx.with_element_context({
                        |cx| -> Option<SharedString> {
                            let mut element = block.render(&mut BlockContext {
                                context: cx,
                                anchor_x: px(0.),
                                gutter_dimensions: &GutterDimensions::default(),
                                line_height: px(0.),
                                em_width: px(0.),
                                max_width: px(0.),
                                block_id: ix,
                                editor_style: &editor::EditorStyle::default(),
                            });
                            let element = element.downcast_mut::<Stateful<Div>>().unwrap();
                            element.interactivity().element_id.clone()?.try_into().ok()
                        }
                    })?,

                    TransformBlock::ExcerptHeader {
                        starts_new_buffer, ..
                    } => {
                        if *starts_new_buffer {
                            "path header block".into()
                        } else {
                            "collapsed context".into()
                        }
                    }
                };

                Some((row, name))
            })
            .collect()
    })
}
