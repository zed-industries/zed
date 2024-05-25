use super::*;
use collections::HashMap;
use editor::{
    display_map::{BlockContext, DisplayRow, TransformBlock},
    DisplayPoint, GutterDimensions,
};
use gpui::{px, AvailableSpace, Stateful, TestAppContext, VisualTestContext};
use language::{
    Diagnostic, DiagnosticEntry, DiagnosticSeverity, OffsetRangeExt, PointUtf16, Rope, Unclipped,
};
use pretty_assertions::assert_eq;
use project::FakeFs;
use rand::{rngs::StdRng, seq::IteratorRandom as _, Rng};
use serde_json::json;
use settings::SettingsStore;
use std::{
    env,
    path::{Path, PathBuf},
};
use unindent::Unindent as _;
use util::{post_inc, RandomCharIter};

#[ctor::ctor]
fn init_logger() {
    if env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}

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
    let editor = view.update(cx, |view, _| view.editor.clone());

    view.next_notification(cx).await;
    assert_eq!(
        editor_blocks(&editor, cx),
        [
            (DisplayRow(0), FILE_HEADER.into()),
            (DisplayRow(2), DIAGNOSTIC_HEADER.into()),
            (DisplayRow(15), EXCERPT_HEADER.into()),
            (DisplayRow(16), DIAGNOSTIC_HEADER.into()),
            (DisplayRow(25), EXCERPT_HEADER.into()),
        ]
    );
    assert_eq!(
        editor.update(cx, |editor, cx| editor.display_text(cx)),
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
    editor.update(cx, |editor, cx| {
        assert_eq!(
            editor.selections.display_ranges(cx),
            [DisplayPoint::new(DisplayRow(12), 6)..DisplayPoint::new(DisplayRow(12), 6)]
        );
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
    assert_eq!(
        editor_blocks(&editor, cx),
        [
            (DisplayRow(0), FILE_HEADER.into()),
            (DisplayRow(2), DIAGNOSTIC_HEADER.into()),
            (DisplayRow(7), FILE_HEADER.into()),
            (DisplayRow(9), DIAGNOSTIC_HEADER.into()),
            (DisplayRow(22), EXCERPT_HEADER.into()),
            (DisplayRow(23), DIAGNOSTIC_HEADER.into()),
            (DisplayRow(32), EXCERPT_HEADER.into()),
        ]
    );

    assert_eq!(
        editor.update(cx, |editor, cx| editor.display_text(cx)),
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
    editor.update(cx, |editor, cx| {
        assert_eq!(
            editor.selections.display_ranges(cx),
            [DisplayPoint::new(DisplayRow(19), 6)..DisplayPoint::new(DisplayRow(19), 6)]
        );
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
    assert_eq!(
        editor_blocks(&editor, cx),
        [
            (DisplayRow(0), FILE_HEADER.into()),
            (DisplayRow(2), DIAGNOSTIC_HEADER.into()),
            (DisplayRow(7), EXCERPT_HEADER.into()),
            (DisplayRow(8), DIAGNOSTIC_HEADER.into()),
            (DisplayRow(13), FILE_HEADER.into()),
            (DisplayRow(15), DIAGNOSTIC_HEADER.into()),
            (DisplayRow(28), EXCERPT_HEADER.into()),
            (DisplayRow(29), DIAGNOSTIC_HEADER.into()),
            (DisplayRow(38), EXCERPT_HEADER.into()),
        ]
    );

    assert_eq!(
        editor.update(cx, |editor, cx| editor.display_text(cx)),
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
    let editor = view.update(cx, |view, _| view.editor.clone());

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
    assert_eq!(
        editor_blocks(&editor, cx),
        [
            (DisplayRow(0), FILE_HEADER.into()),
            (DisplayRow(2), DIAGNOSTIC_HEADER.into()),
        ]
    );
    assert_eq!(
        editor.update(cx, |editor, cx| editor.display_text(cx)),
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
    assert_eq!(
        editor_blocks(&editor, cx),
        [
            (DisplayRow(0), FILE_HEADER.into()),
            (DisplayRow(2), DIAGNOSTIC_HEADER.into()),
            (DisplayRow(6), EXCERPT_HEADER.into()),
            (DisplayRow(7), DIAGNOSTIC_HEADER.into()),
        ]
    );
    assert_eq!(
        editor.update(cx, |editor, cx| editor.display_text(cx)),
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
    assert_eq!(
        editor_blocks(&editor, cx),
        [
            (DisplayRow(0), FILE_HEADER.into()),
            (DisplayRow(2), DIAGNOSTIC_HEADER.into()),
            (DisplayRow(7), EXCERPT_HEADER.into()),
            (DisplayRow(8), DIAGNOSTIC_HEADER.into()),
        ]
    );
    assert_eq!(
        editor.update(cx, |editor, cx| editor.display_text(cx)),
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
    assert_eq!(
        editor_blocks(&editor, cx),
        [
            (DisplayRow(0), FILE_HEADER.into()),
            (DisplayRow(2), DIAGNOSTIC_HEADER.into()),
            (DisplayRow(7), EXCERPT_HEADER.into()),
            (DisplayRow(8), DIAGNOSTIC_HEADER.into()),
        ]
    );
    assert_eq!(
        editor.update(cx, |editor, cx| editor.display_text(cx)),
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
}

#[gpui::test(iterations = 20)]
async fn test_random_diagnostics(cx: &mut TestAppContext, mut rng: StdRng) {
    init_test(cx);

    let operations = env::var("OPERATIONS")
        .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
        .unwrap_or(10);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/test", json!({})).await;

    let project = Project::test(fs.clone(), ["/test".as_ref()], cx).await;
    let window = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
    let cx = &mut VisualTestContext::from_window(*window, cx);
    let workspace = window.root(cx).unwrap();

    let mutated_view = window.build_view(cx, |cx| {
        ProjectDiagnosticsEditor::new_with_context(1, project.clone(), workspace.downgrade(), cx)
    });

    workspace.update(cx, |workspace, cx| {
        workspace.add_item_to_center(Box::new(mutated_view.clone()), cx);
    });
    mutated_view.update(cx, |view, cx| {
        assert!(view.focus_handle.is_focused(cx));
    });

    let mut next_group_id = 0;
    let mut next_filename = 0;
    let mut language_server_ids = vec![LanguageServerId(0)];
    let mut updated_language_servers = HashSet::default();
    let mut current_diagnostics: HashMap<
        (PathBuf, LanguageServerId),
        Vec<DiagnosticEntry<Unclipped<PointUtf16>>>,
    > = Default::default();

    for _ in 0..operations {
        match rng.gen_range(0..100) {
            // language server completes its diagnostic check
            0..=20 if !updated_language_servers.is_empty() => {
                let server_id = *updated_language_servers.iter().choose(&mut rng).unwrap();
                log::info!("finishing diagnostic check for language server {server_id}");
                project.update(cx, |project, cx| {
                    project.disk_based_diagnostics_finished(server_id, cx)
                });

                if rng.gen_bool(0.5) {
                    cx.run_until_parked();
                }
            }

            // language server updates diagnostics
            _ => {
                let (path, server_id, diagnostics) =
                    match current_diagnostics.iter_mut().choose(&mut rng) {
                        // update existing set of diagnostics
                        Some(((path, server_id), diagnostics)) if rng.gen_bool(0.5) => {
                            (path.clone(), *server_id, diagnostics)
                        }

                        // insert a set of diagnostics for a new path
                        _ => {
                            let path: PathBuf =
                                format!("/test/{}.rs", post_inc(&mut next_filename)).into();
                            let len = rng.gen_range(128..256);
                            let content =
                                RandomCharIter::new(&mut rng).take(len).collect::<String>();
                            fs.insert_file(&path, content.into_bytes()).await;

                            let server_id = match language_server_ids.iter().choose(&mut rng) {
                                Some(server_id) if rng.gen_bool(0.5) => *server_id,
                                _ => {
                                    let id = LanguageServerId(language_server_ids.len());
                                    language_server_ids.push(id);
                                    id
                                }
                            };

                            (
                                path.clone(),
                                server_id,
                                current_diagnostics
                                    .entry((path, server_id))
                                    .or_insert(vec![]),
                            )
                        }
                    };

                updated_language_servers.insert(server_id);

                project.update(cx, |project, cx| {
                    log::info!("updating diagnostics. language server {server_id} path {path:?}");
                    randomly_update_diagnostics_for_path(
                        &fs,
                        &path,
                        diagnostics,
                        &mut next_group_id,
                        &mut rng,
                    );
                    project
                        .update_diagnostic_entries(server_id, path, None, diagnostics.clone(), cx)
                        .unwrap()
                });

                cx.run_until_parked();
            }
        }
    }

    log::info!("updating mutated diagnostics view");
    mutated_view.update(cx, |view, _| view.enqueue_update_stale_excerpts(None));
    cx.run_until_parked();

    log::info!("constructing reference diagnostics view");
    let reference_view = window.build_view(cx, |cx| {
        ProjectDiagnosticsEditor::new_with_context(1, project.clone(), workspace.downgrade(), cx)
    });
    cx.run_until_parked();

    let mutated_excerpts = get_diagnostics_excerpts(&mutated_view, cx);
    let reference_excerpts = get_diagnostics_excerpts(&reference_view, cx);
    assert_eq!(mutated_excerpts, reference_excerpts);
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

#[derive(Debug, PartialEq, Eq)]
struct ExcerptInfo {
    path: PathBuf,
    range: ExcerptRange<Point>,
    group_id: usize,
    primary: bool,
    language_server: LanguageServerId,
}

fn get_diagnostics_excerpts(
    view: &View<ProjectDiagnosticsEditor>,
    cx: &mut VisualTestContext,
) -> Vec<ExcerptInfo> {
    view.update(cx, |view, cx| {
        let mut result = vec![];
        let mut excerpt_indices_by_id = HashMap::default();
        view.excerpts.update(cx, |multibuffer, cx| {
            let snapshot = multibuffer.snapshot(cx);
            for (id, buffer, range) in snapshot.excerpts() {
                excerpt_indices_by_id.insert(id, result.len());
                result.push(ExcerptInfo {
                    path: buffer.file().unwrap().path().to_path_buf(),
                    range: ExcerptRange {
                        context: range.context.to_point(&buffer),
                        primary: range.primary.map(|range| range.to_point(&buffer)),
                    },
                    group_id: usize::MAX,
                    primary: false,
                    language_server: LanguageServerId(0),
                });
            }
        });

        for state in &view.path_states {
            for group in &state.diagnostic_groups {
                for (ix, excerpt_id) in group.excerpts.iter().enumerate() {
                    let excerpt_ix = excerpt_indices_by_id[excerpt_id];
                    let excerpt = &mut result[excerpt_ix];
                    excerpt.group_id = group.primary_diagnostic.diagnostic.group_id;
                    excerpt.language_server = group.language_server_id;
                    excerpt.primary = ix == group.primary_excerpt_ix;
                }
            }
        }

        result
    })
}

fn randomly_update_diagnostics_for_path(
    fs: &FakeFs,
    path: &Path,
    diagnostics: &mut Vec<DiagnosticEntry<Unclipped<PointUtf16>>>,
    next_group_id: &mut usize,
    rng: &mut impl Rng,
) {
    let file_content = fs.read_file_sync(path).unwrap();
    let file_text = Rope::from(String::from_utf8_lossy(&file_content).as_ref());

    let mut group_ids = diagnostics
        .iter()
        .map(|d| d.diagnostic.group_id)
        .collect::<HashSet<_>>();

    let mutation_count = rng.gen_range(1..=3);
    for _ in 0..mutation_count {
        if rng.gen_bool(0.5) && !group_ids.is_empty() {
            let group_id = *group_ids.iter().choose(rng).unwrap();
            log::info!("  removing diagnostic group {group_id}");
            diagnostics.retain(|d| d.diagnostic.group_id != group_id);
            group_ids.remove(&group_id);
        } else {
            let group_id = *next_group_id;
            *next_group_id += 1;

            let mut new_diagnostics = vec![random_diagnostic(rng, &file_text, group_id, true)];
            for _ in 0..rng.gen_range(0..=1) {
                new_diagnostics.push(random_diagnostic(rng, &file_text, group_id, false));
            }

            let ix = rng.gen_range(0..=diagnostics.len());
            log::info!(
                "  inserting diagnostic group {group_id} at index {ix}. ranges: {:?}",
                new_diagnostics
                    .iter()
                    .map(|d| (d.range.start.0, d.range.end.0))
                    .collect::<Vec<_>>()
            );
            diagnostics.splice(ix..ix, new_diagnostics);
        }
    }
}

fn random_diagnostic(
    rng: &mut impl Rng,
    file_text: &Rope,
    group_id: usize,
    is_primary: bool,
) -> DiagnosticEntry<Unclipped<PointUtf16>> {
    // Intentionally allow erroneous ranges some of the time (that run off the end of the file),
    // because language servers can potentially give us those, and we should handle them gracefully.
    const ERROR_MARGIN: usize = 10;

    let start = rng.gen_range(0..file_text.len().saturating_add(ERROR_MARGIN));
    let end = rng.gen_range(start..file_text.len().saturating_add(ERROR_MARGIN));
    let range = Range {
        start: Unclipped(file_text.offset_to_point_utf16(start)),
        end: Unclipped(file_text.offset_to_point_utf16(end)),
    };
    let severity = if rng.gen_bool(0.5) {
        DiagnosticSeverity::WARNING
    } else {
        DiagnosticSeverity::ERROR
    };
    let message = format!("diagnostic group {group_id}");

    DiagnosticEntry {
        range,
        diagnostic: Diagnostic {
            source: None, // (optional) service that created the diagnostic
            code: None,   // (optional) machine-readable code that identifies the diagnostic
            severity,
            message,
            group_id,
            is_primary,
            is_disk_based: false,
            is_unnecessary: false,
        },
    }
}

const FILE_HEADER: &'static str = "file header";
const EXCERPT_HEADER: &'static str = "excerpt header";
const EXCERPT_FOOTER: &'static str = "excerpt footer";

fn editor_blocks(
    editor: &View<Editor>,
    cx: &mut VisualTestContext,
) -> Vec<(DisplayRow, SharedString)> {
    let mut blocks = Vec::new();
    cx.draw(gpui::Point::default(), AvailableSpace::min_size(), |cx| {
        editor.update(cx, |editor, cx| {
            let snapshot = editor.snapshot(cx);
            blocks.extend(
                snapshot
                    .blocks_in_range(DisplayRow(0)..snapshot.max_point().row())
                    .enumerate()
                    .filter_map(|(ix, (row, block))| {
                        let name: SharedString = match block {
                            TransformBlock::Custom(block) => {
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
                                element
                                    .interactivity()
                                    .element_id
                                    .clone()?
                                    .try_into()
                                    .ok()?
                            }

                            TransformBlock::ExcerptHeader {
                                starts_new_buffer, ..
                            } => {
                                if *starts_new_buffer {
                                    FILE_HEADER.into()
                                } else {
                                    EXCERPT_HEADER.into()
                                }
                            }
                            TransformBlock::ExcerptFooter { .. } => EXCERPT_FOOTER.into(),
                        };

                        Some((row, name))
                    }),
            )
        });

        div().into_any()
    });
    blocks
}
