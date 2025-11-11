use super::*;
use collections::{HashMap, HashSet};
use editor::{
    DisplayPoint, EditorSettings, Inlay,
    actions::{GoToDiagnostic, GoToPreviousDiagnostic, Hover, MoveToBeginning},
    display_map::DisplayRow,
    test::{
        editor_content_with_blocks, editor_lsp_test_context::EditorLspTestContext,
        editor_test_context::EditorTestContext,
    },
};
use gpui::{TestAppContext, VisualTestContext};
use indoc::indoc;
use language::{DiagnosticSourceKind, Rope};
use lsp::LanguageServerId;
use pretty_assertions::assert_eq;
use project::{
    FakeFs,
    project_settings::{GoToDiagnosticSeverity, GoToDiagnosticSeverityFilter},
};
use rand::{Rng, rngs::StdRng, seq::IteratorRandom as _};
use serde_json::json;
use settings::SettingsStore;
use std::{
    env,
    path::{Path, PathBuf},
    str::FromStr,
};
use unindent::Unindent as _;
use util::{RandomCharIter, path, post_inc, rel_path::rel_path};

#[ctor::ctor]
fn init_logger() {
    zlog::init_test();
}

#[gpui::test]
async fn test_diagnostics(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/test"),
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
    let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;
    let lsp_store = project.read_with(cx, |project, _| project.lsp_store());
    let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*window, cx);
    let workspace = window.root(cx).unwrap();
    let uri = lsp::Uri::from_file_path(path!("/test/main.rs")).unwrap();

    // Create some diagnostics
    lsp_store.update(cx, |lsp_store, cx| {
        lsp_store.update_diagnostics(language_server_id, lsp::PublishDiagnosticsParams {
            uri: uri.clone(),
            diagnostics: vec![lsp::Diagnostic{
                range: lsp::Range::new(lsp::Position::new(7, 6),lsp::Position::new(7, 7)),
                severity:Some(lsp::DiagnosticSeverity::ERROR),
                message: "use of moved value\nvalue used here after move".to_string(),
                related_information: Some(vec![lsp::DiagnosticRelatedInformation {
                    location: lsp::Location::new(uri.clone(), lsp::Range::new(lsp::Position::new(2,8),lsp::Position::new(2,9))),
                    message: "move occurs because `y` has type `Vec<char>`, which does not implement the `Copy` trait".to_string()
                },
                lsp::DiagnosticRelatedInformation {
                    location: lsp::Location::new(uri.clone(), lsp::Range::new(lsp::Position::new(4,6),lsp::Position::new(4,7))),
                    message: "value moved here".to_string()
                },
                ]),
                ..Default::default()
            },
            lsp::Diagnostic{
                range: lsp::Range::new(lsp::Position::new(8, 6),lsp::Position::new(8, 7)),
                severity:Some(lsp::DiagnosticSeverity::ERROR),
                message: "use of moved value\nvalue used here after move".to_string(),
                related_information: Some(vec![lsp::DiagnosticRelatedInformation {
                    location: lsp::Location::new(uri.clone(), lsp::Range::new(lsp::Position::new(1,8),lsp::Position::new(1,9))),
                    message: "move occurs because `x` has type `Vec<char>`, which does not implement the `Copy` trait".to_string()
                },
                lsp::DiagnosticRelatedInformation {
                    location: lsp::Location::new(uri.clone(), lsp::Range::new(lsp::Position::new(3,6),lsp::Position::new(3,7))),
                    message: "value moved here".to_string()
                },
                ]),
                ..Default::default()
            }
            ],
            version: None
        }, None, DiagnosticSourceKind::Pushed, &[], cx).unwrap();
    });

    // Open the project diagnostics view while there are already diagnostics.
    let diagnostics = window.build_entity(cx, |window, cx| {
        ProjectDiagnosticsEditor::new(true, project.clone(), workspace.downgrade(), window, cx)
    });
    let editor = diagnostics.update(cx, |diagnostics, _| diagnostics.editor.clone());

    diagnostics
        .next_notification(DIAGNOSTICS_UPDATE_DEBOUNCE + Duration::from_millis(10), cx)
        .await;

    pretty_assertions::assert_eq!(
        editor_content_with_blocks(&editor, cx),
        indoc::indoc! {
            "§ main.rs
             § -----
             fn main() {
                 let x = vec![];
             § move occurs because `x` has type `Vec<char>`, which does not implement
             § the `Copy` trait (back)
                 let y = vec![];
             § move occurs because `y` has type `Vec<char>`, which does not implement
             § the `Copy` trait (back)
                 a(x); § value moved here (back)
                 b(y); § value moved here
                 // comment 1
                 // comment 2
                 c(y);
             § use of moved value
             § value used here after move
             § hint: move occurs because `y` has type `Vec<char>`, which does not
             § implement the `Copy` trait
                 d(x);
             § use of moved value
             § value used here after move
             § hint: move occurs because `x` has type `Vec<char>`, which does not
             § implement the `Copy` trait
             § hint: value moved here
             }"
        }
    );

    // Cursor is at the first diagnostic
    editor.update(cx, |editor, cx| {
        assert_eq!(
            editor
                .selections
                .display_ranges(&editor.display_snapshot(cx)),
            [DisplayPoint::new(DisplayRow(3), 8)..DisplayPoint::new(DisplayRow(3), 8)]
        );
    });

    // Diagnostics are added for another earlier path.
    lsp_store.update(cx, |lsp_store, cx| {
        lsp_store.disk_based_diagnostics_started(language_server_id, cx);
        lsp_store
            .update_diagnostics(
                language_server_id,
                lsp::PublishDiagnosticsParams {
                    uri: lsp::Uri::from_file_path(path!("/test/consts.rs")).unwrap(),
                    diagnostics: vec![lsp::Diagnostic {
                        range: lsp::Range::new(
                            lsp::Position::new(0, 15),
                            lsp::Position::new(0, 15),
                        ),
                        severity: Some(lsp::DiagnosticSeverity::ERROR),
                        message: "mismatched types expected `usize`, found `char`".to_string(),
                        ..Default::default()
                    }],
                    version: None,
                },
                None,
                DiagnosticSourceKind::Pushed,
                &[],
                cx,
            )
            .unwrap();
        lsp_store.disk_based_diagnostics_finished(language_server_id, cx);
    });

    diagnostics
        .next_notification(DIAGNOSTICS_UPDATE_DEBOUNCE + Duration::from_millis(10), cx)
        .await;

    pretty_assertions::assert_eq!(
        editor_content_with_blocks(&editor, cx),
        indoc::indoc! {
            "§ consts.rs
             § -----
             const a: i32 = 'a'; § mismatched types expected `usize`, found `char`
             const b: i32 = c;

             § main.rs
             § -----
             fn main() {
                 let x = vec![];
             § move occurs because `x` has type `Vec<char>`, which does not implement
             § the `Copy` trait (back)
                 let y = vec![];
             § move occurs because `y` has type `Vec<char>`, which does not implement
             § the `Copy` trait (back)
                 a(x); § value moved here (back)
                 b(y); § value moved here
                 // comment 1
                 // comment 2
                 c(y);
             § use of moved value
             § value used here after move
             § hint: move occurs because `y` has type `Vec<char>`, which does not
             § implement the `Copy` trait
                 d(x);
             § use of moved value
             § value used here after move
             § hint: move occurs because `x` has type `Vec<char>`, which does not
             § implement the `Copy` trait
             § hint: value moved here
             }"
        }
    );

    // Cursor keeps its position.
    editor.update(cx, |editor, cx| {
        assert_eq!(
            editor
                .selections
                .display_ranges(&editor.display_snapshot(cx)),
            [DisplayPoint::new(DisplayRow(8), 8)..DisplayPoint::new(DisplayRow(8), 8)]
        );
    });

    // Diagnostics are added to the first path
    lsp_store.update(cx, |lsp_store, cx| {
        lsp_store.disk_based_diagnostics_started(language_server_id, cx);
        lsp_store
            .update_diagnostics(
                language_server_id,
                lsp::PublishDiagnosticsParams {
                    uri: lsp::Uri::from_file_path(path!("/test/consts.rs")).unwrap(),
                    diagnostics: vec![
                        lsp::Diagnostic {
                            range: lsp::Range::new(
                                lsp::Position::new(0, 15),
                                lsp::Position::new(0, 15),
                            ),
                            severity: Some(lsp::DiagnosticSeverity::ERROR),
                            message: "mismatched types expected `usize`, found `char`".to_string(),
                            ..Default::default()
                        },
                        lsp::Diagnostic {
                            range: lsp::Range::new(
                                lsp::Position::new(1, 15),
                                lsp::Position::new(1, 15),
                            ),
                            severity: Some(lsp::DiagnosticSeverity::ERROR),
                            message: "unresolved name `c`".to_string(),
                            ..Default::default()
                        },
                    ],
                    version: None,
                },
                None,
                DiagnosticSourceKind::Pushed,
                &[],
                cx,
            )
            .unwrap();
        lsp_store.disk_based_diagnostics_finished(language_server_id, cx);
    });

    diagnostics
        .next_notification(DIAGNOSTICS_UPDATE_DEBOUNCE + Duration::from_millis(10), cx)
        .await;

    pretty_assertions::assert_eq!(
        editor_content_with_blocks(&editor, cx),
        indoc::indoc! {
            "§ consts.rs
             § -----
             const a: i32 = 'a'; § mismatched types expected `usize`, found `char`
             const b: i32 = c; § unresolved name `c`

             § main.rs
             § -----
             fn main() {
                 let x = vec![];
             § move occurs because `x` has type `Vec<char>`, which does not implement
             § the `Copy` trait (back)
                 let y = vec![];
             § move occurs because `y` has type `Vec<char>`, which does not implement
             § the `Copy` trait (back)
                 a(x); § value moved here (back)
                 b(y); § value moved here
                 // comment 1
                 // comment 2
                 c(y);
             § use of moved value
             § value used here after move
             § hint: move occurs because `y` has type `Vec<char>`, which does not
             § implement the `Copy` trait
                 d(x);
             § use of moved value
             § value used here after move
             § hint: move occurs because `x` has type `Vec<char>`, which does not
             § implement the `Copy` trait
             § hint: value moved here
             }"
        }
    );
}

#[gpui::test]
async fn test_diagnostics_with_folds(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/test"),
        json!({
            "main.js": "
            function test() {
                return 1
            };

            tset();
            ".unindent()
        }),
    )
    .await;

    let server_id_1 = LanguageServerId(100);
    let server_id_2 = LanguageServerId(101);
    let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;
    let lsp_store = project.read_with(cx, |project, _| project.lsp_store());
    let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*window, cx);
    let workspace = window.root(cx).unwrap();

    let diagnostics = window.build_entity(cx, |window, cx| {
        ProjectDiagnosticsEditor::new(true, project.clone(), workspace.downgrade(), window, cx)
    });
    let editor = diagnostics.update(cx, |diagnostics, _| diagnostics.editor.clone());

    // Two language servers start updating diagnostics
    lsp_store.update(cx, |lsp_store, cx| {
        lsp_store.disk_based_diagnostics_started(server_id_1, cx);
        lsp_store.disk_based_diagnostics_started(server_id_2, cx);
        lsp_store
            .update_diagnostics(
                server_id_1,
                lsp::PublishDiagnosticsParams {
                    uri: lsp::Uri::from_file_path(path!("/test/main.js")).unwrap(),
                    diagnostics: vec![lsp::Diagnostic {
                        range: lsp::Range::new(lsp::Position::new(4, 0), lsp::Position::new(4, 4)),
                        severity: Some(lsp::DiagnosticSeverity::WARNING),
                        message: "no method `tset`".to_string(),
                        related_information: Some(vec![lsp::DiagnosticRelatedInformation {
                            location: lsp::Location::new(
                                lsp::Uri::from_file_path(path!("/test/main.js")).unwrap(),
                                lsp::Range::new(
                                    lsp::Position::new(0, 9),
                                    lsp::Position::new(0, 13),
                                ),
                            ),
                            message: "method `test` defined here".to_string(),
                        }]),
                        ..Default::default()
                    }],
                    version: None,
                },
                None,
                DiagnosticSourceKind::Pushed,
                &[],
                cx,
            )
            .unwrap();
    });

    // The first language server finishes
    lsp_store.update(cx, |lsp_store, cx| {
        lsp_store.disk_based_diagnostics_finished(server_id_1, cx);
    });

    // Only the first language server's diagnostics are shown.
    cx.executor()
        .advance_clock(DIAGNOSTICS_UPDATE_DEBOUNCE + Duration::from_millis(10));
    cx.executor().run_until_parked();
    editor.update_in(cx, |editor, window, cx| {
        editor.fold_ranges(vec![Point::new(0, 0)..Point::new(3, 0)], false, window, cx);
    });

    pretty_assertions::assert_eq!(
        editor_content_with_blocks(&editor, cx),
        indoc::indoc! {
            "§ main.js
             § -----
             ⋯

             tset(); § no method `tset`"
        }
    );

    editor.update(cx, |editor, cx| {
        editor.unfold_ranges(&[Point::new(0, 0)..Point::new(3, 0)], false, false, cx);
    });

    pretty_assertions::assert_eq!(
        editor_content_with_blocks(&editor, cx),
        indoc::indoc! {
            "§ main.js
             § -----
             function test() { § method `test` defined here
                 return 1
             };

             tset(); § no method `tset`"
        }
    );
}

#[gpui::test]
async fn test_diagnostics_multiple_servers(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/test"),
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
    let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;
    let lsp_store = project.read_with(cx, |project, _| project.lsp_store());
    let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*window, cx);
    let workspace = window.root(cx).unwrap();

    let diagnostics = window.build_entity(cx, |window, cx| {
        ProjectDiagnosticsEditor::new(true, project.clone(), workspace.downgrade(), window, cx)
    });
    let editor = diagnostics.update(cx, |diagnostics, _| diagnostics.editor.clone());

    // Two language servers start updating diagnostics
    lsp_store.update(cx, |lsp_store, cx| {
        lsp_store.disk_based_diagnostics_started(server_id_1, cx);
        lsp_store.disk_based_diagnostics_started(server_id_2, cx);
        lsp_store
            .update_diagnostics(
                server_id_1,
                lsp::PublishDiagnosticsParams {
                    uri: lsp::Uri::from_file_path(path!("/test/main.js")).unwrap(),
                    diagnostics: vec![lsp::Diagnostic {
                        range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 1)),
                        severity: Some(lsp::DiagnosticSeverity::WARNING),
                        message: "error 1".to_string(),
                        ..Default::default()
                    }],
                    version: None,
                },
                None,
                DiagnosticSourceKind::Pushed,
                &[],
                cx,
            )
            .unwrap();
    });

    // The first language server finishes
    lsp_store.update(cx, |lsp_store, cx| {
        lsp_store.disk_based_diagnostics_finished(server_id_1, cx);
    });

    // Only the first language server's diagnostics are shown.
    cx.executor()
        .advance_clock(DIAGNOSTICS_UPDATE_DEBOUNCE + Duration::from_millis(10));
    cx.executor().run_until_parked();

    pretty_assertions::assert_eq!(
        editor_content_with_blocks(&editor, cx),
        indoc::indoc! {
            "§ main.js
             § -----
             a(); § error 1
             b();
             c();"
        }
    );

    // The second language server finishes
    lsp_store.update(cx, |lsp_store, cx| {
        lsp_store
            .update_diagnostics(
                server_id_2,
                lsp::PublishDiagnosticsParams {
                    uri: lsp::Uri::from_file_path(path!("/test/main.js")).unwrap(),
                    diagnostics: vec![lsp::Diagnostic {
                        range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(1, 1)),
                        severity: Some(lsp::DiagnosticSeverity::ERROR),
                        message: "warning 1".to_string(),
                        ..Default::default()
                    }],
                    version: None,
                },
                None,
                DiagnosticSourceKind::Pushed,
                &[],
                cx,
            )
            .unwrap();
        lsp_store.disk_based_diagnostics_finished(server_id_2, cx);
    });

    // Both language server's diagnostics are shown.
    cx.executor()
        .advance_clock(DIAGNOSTICS_UPDATE_DEBOUNCE + Duration::from_millis(10));
    cx.executor().run_until_parked();

    pretty_assertions::assert_eq!(
        editor_content_with_blocks(&editor, cx),
        indoc::indoc! {
            "§ main.js
             § -----
             a(); § error 1
             b(); § warning 1
             c();
             d();"
        }
    );

    // Both language servers start updating diagnostics, and the first server finishes.
    lsp_store.update(cx, |lsp_store, cx| {
        lsp_store.disk_based_diagnostics_started(server_id_1, cx);
        lsp_store.disk_based_diagnostics_started(server_id_2, cx);
        lsp_store
            .update_diagnostics(
                server_id_1,
                lsp::PublishDiagnosticsParams {
                    uri: lsp::Uri::from_file_path(path!("/test/main.js")).unwrap(),
                    diagnostics: vec![lsp::Diagnostic {
                        range: lsp::Range::new(lsp::Position::new(2, 0), lsp::Position::new(2, 1)),
                        severity: Some(lsp::DiagnosticSeverity::WARNING),
                        message: "warning 2".to_string(),
                        ..Default::default()
                    }],
                    version: None,
                },
                None,
                DiagnosticSourceKind::Pushed,
                &[],
                cx,
            )
            .unwrap();
        lsp_store
            .update_diagnostics(
                server_id_2,
                lsp::PublishDiagnosticsParams {
                    uri: lsp::Uri::from_file_path(path!("/test/main.rs")).unwrap(),
                    diagnostics: vec![],
                    version: None,
                },
                None,
                DiagnosticSourceKind::Pushed,
                &[],
                cx,
            )
            .unwrap();
        lsp_store.disk_based_diagnostics_finished(server_id_1, cx);
    });

    // Only the first language server's diagnostics are updated.
    cx.executor()
        .advance_clock(DIAGNOSTICS_UPDATE_DEBOUNCE + Duration::from_millis(10));
    cx.executor().run_until_parked();

    pretty_assertions::assert_eq!(
        editor_content_with_blocks(&editor, cx),
        indoc::indoc! {
            "§ main.js
             § -----
             a();
             b(); § warning 1
             c(); § warning 2
             d();
             e();"
        }
    );

    // The second language server finishes.
    lsp_store.update(cx, |lsp_store, cx| {
        lsp_store
            .update_diagnostics(
                server_id_2,
                lsp::PublishDiagnosticsParams {
                    uri: lsp::Uri::from_file_path(path!("/test/main.js")).unwrap(),
                    diagnostics: vec![lsp::Diagnostic {
                        range: lsp::Range::new(lsp::Position::new(3, 0), lsp::Position::new(3, 1)),
                        severity: Some(lsp::DiagnosticSeverity::WARNING),
                        message: "warning 2".to_string(),
                        ..Default::default()
                    }],
                    version: None,
                },
                None,
                DiagnosticSourceKind::Pushed,
                &[],
                cx,
            )
            .unwrap();
        lsp_store.disk_based_diagnostics_finished(server_id_2, cx);
    });

    // Both language servers' diagnostics are updated.
    cx.executor()
        .advance_clock(DIAGNOSTICS_UPDATE_DEBOUNCE + Duration::from_millis(10));
    cx.executor().run_until_parked();

    pretty_assertions::assert_eq!(
        editor_content_with_blocks(&editor, cx),
        indoc::indoc! {
            "§ main.js
                 § -----
                 a();
                 b();
                 c(); § warning 2
                 d(); § warning 2
                 e();"
        }
    );
}

#[gpui::test(iterations = 20)]
async fn test_random_diagnostics_blocks(cx: &mut TestAppContext, mut rng: StdRng) {
    init_test(cx);

    let operations = env::var("OPERATIONS")
        .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
        .unwrap_or(10);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/test"), json!({})).await;

    let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;
    let lsp_store = project.read_with(cx, |project, _| project.lsp_store());
    let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*window, cx);
    let workspace = window.root(cx).unwrap();

    let mutated_diagnostics = window.build_entity(cx, |window, cx| {
        ProjectDiagnosticsEditor::new(true, project.clone(), workspace.downgrade(), window, cx)
    });

    workspace.update_in(cx, |workspace, window, cx| {
        workspace.add_item_to_center(Box::new(mutated_diagnostics.clone()), window, cx);
    });
    mutated_diagnostics.update_in(cx, |diagnostics, window, _cx| {
        assert!(diagnostics.focus_handle.is_focused(window));
    });

    let mut next_id = 0;
    let mut next_filename = 0;
    let mut language_server_ids = vec![LanguageServerId(0)];
    let mut updated_language_servers = HashSet::default();
    let mut current_diagnostics: HashMap<(PathBuf, LanguageServerId), Vec<lsp::Diagnostic>> =
        Default::default();

    for _ in 0..operations {
        match rng.random_range(0..100) {
            // language server completes its diagnostic check
            0..=20 if !updated_language_servers.is_empty() => {
                let server_id = *updated_language_servers.iter().choose(&mut rng).unwrap();
                log::info!("finishing diagnostic check for language server {server_id}");
                lsp_store.update(cx, |lsp_store, cx| {
                    lsp_store.disk_based_diagnostics_finished(server_id, cx)
                });

                if rng.random_bool(0.5) {
                    cx.run_until_parked();
                }
            }

            // language server updates diagnostics
            _ => {
                let (path, server_id, diagnostics) =
                    match current_diagnostics.iter_mut().choose(&mut rng) {
                        // update existing set of diagnostics
                        Some(((path, server_id), diagnostics)) if rng.random_bool(0.5) => {
                            (path.clone(), *server_id, diagnostics)
                        }

                        // insert a set of diagnostics for a new path
                        _ => {
                            let path: PathBuf =
                                format!(path!("/test/{}.rs"), post_inc(&mut next_filename)).into();
                            let len = rng.random_range(128..256);
                            let content =
                                RandomCharIter::new(&mut rng).take(len).collect::<String>();
                            fs.insert_file(&path, content.into_bytes()).await;

                            let server_id = match language_server_ids.iter().choose(&mut rng) {
                                Some(server_id) if rng.random_bool(0.5) => *server_id,
                                _ => {
                                    let id = LanguageServerId(language_server_ids.len());
                                    language_server_ids.push(id);
                                    id
                                }
                            };

                            (
                                path.clone(),
                                server_id,
                                current_diagnostics.entry((path, server_id)).or_default(),
                            )
                        }
                    };

                updated_language_servers.insert(server_id);

                lsp_store.update(cx, |lsp_store, cx| {
                    log::info!("updating diagnostics. language server {server_id} path {path:?}");
                    randomly_update_diagnostics_for_path(
                        &fs,
                        &path,
                        diagnostics,
                        &mut next_id,
                        &mut rng,
                    );
                    lsp_store
                        .update_diagnostics(
                            server_id,
                            lsp::PublishDiagnosticsParams {
                                uri: lsp::Uri::from_file_path(&path).unwrap_or_else(|_| {
                                    lsp::Uri::from_str("file:///test/fallback.rs").unwrap()
                                }),
                                diagnostics: diagnostics.clone(),
                                version: None,
                            },
                            None,
                            DiagnosticSourceKind::Pushed,
                            &[],
                            cx,
                        )
                        .unwrap()
                });
                cx.executor()
                    .advance_clock(DIAGNOSTICS_UPDATE_DEBOUNCE + Duration::from_millis(10));

                cx.run_until_parked();
            }
        }
    }

    log::info!("updating mutated diagnostics view");
    mutated_diagnostics.update_in(cx, |diagnostics, window, cx| {
        diagnostics.update_stale_excerpts(window, cx)
    });

    log::info!("constructing reference diagnostics view");
    let reference_diagnostics = window.build_entity(cx, |window, cx| {
        ProjectDiagnosticsEditor::new(true, project.clone(), workspace.downgrade(), window, cx)
    });
    cx.executor()
        .advance_clock(DIAGNOSTICS_UPDATE_DEBOUNCE + Duration::from_millis(10));
    cx.run_until_parked();

    let mutated_excerpts =
        editor_content_with_blocks(&mutated_diagnostics.update(cx, |d, _| d.editor.clone()), cx);
    let reference_excerpts = editor_content_with_blocks(
        &reference_diagnostics.update(cx, |d, _| d.editor.clone()),
        cx,
    );

    // The mutated view may contain more than the reference view as
    // we don't currently shrink excerpts when diagnostics were removed.
    let mut ref_iter = reference_excerpts.lines().filter(|line| {
        // ignore $ ---- and $ <file>.rs
        !line.starts_with('§')
            || line.starts_with("§ diagnostic")
            || line.starts_with("§ related info")
    });
    let mut next_ref_line = ref_iter.next();
    let mut skipped_block = false;

    for mut_line in mutated_excerpts.lines() {
        if let Some(ref_line) = next_ref_line {
            if mut_line == ref_line {
                next_ref_line = ref_iter.next();
            } else if mut_line.contains('§')
                // ignore $ ---- and $ <file>.rs
                && (!mut_line.starts_with('§')
                    || mut_line.starts_with("§ diagnostic")
                    || mut_line.starts_with("§ related info"))
            {
                skipped_block = true;
            }
        }
    }

    if next_ref_line.is_some() || skipped_block {
        pretty_assertions::assert_eq!(mutated_excerpts, reference_excerpts);
    }
}

// similar to above, but with inlays. Used to find panics when mixing diagnostics and inlays.
#[gpui::test]
async fn test_random_diagnostics_with_inlays(cx: &mut TestAppContext, mut rng: StdRng) {
    init_test(cx);

    let operations = env::var("OPERATIONS")
        .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
        .unwrap_or(10);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/test"), json!({})).await;

    let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;
    let lsp_store = project.read_with(cx, |project, _| project.lsp_store());
    let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*window, cx);
    let workspace = window.root(cx).unwrap();

    let mutated_diagnostics = window.build_entity(cx, |window, cx| {
        ProjectDiagnosticsEditor::new(true, project.clone(), workspace.downgrade(), window, cx)
    });

    workspace.update_in(cx, |workspace, window, cx| {
        workspace.add_item_to_center(Box::new(mutated_diagnostics.clone()), window, cx);
    });
    mutated_diagnostics.update_in(cx, |diagnostics, window, _cx| {
        assert!(diagnostics.focus_handle.is_focused(window));
    });

    let mut next_id = 0;
    let mut next_filename = 0;
    let mut language_server_ids = vec![LanguageServerId(0)];
    let mut updated_language_servers = HashSet::default();
    let mut current_diagnostics: HashMap<(PathBuf, LanguageServerId), Vec<lsp::Diagnostic>> =
        Default::default();
    let mut next_inlay_id = 0;

    for _ in 0..operations {
        match rng.random_range(0..100) {
            // language server completes its diagnostic check
            0..=20 if !updated_language_servers.is_empty() => {
                let server_id = *updated_language_servers.iter().choose(&mut rng).unwrap();
                log::info!("finishing diagnostic check for language server {server_id}");
                lsp_store.update(cx, |lsp_store, cx| {
                    lsp_store.disk_based_diagnostics_finished(server_id, cx)
                });

                if rng.random_bool(0.5) {
                    cx.run_until_parked();
                }
            }

            21..=50 => mutated_diagnostics.update_in(cx, |diagnostics, window, cx| {
                diagnostics.editor.update(cx, |editor, cx| {
                    let snapshot = editor.snapshot(window, cx);
                    if !snapshot.buffer_snapshot().is_empty() {
                        let position = rng.random_range(0..snapshot.buffer_snapshot().len());
                        let position = snapshot.buffer_snapshot().clip_offset(position, Bias::Left);
                        log::info!(
                            "adding inlay at {position}/{}: {:?}",
                            snapshot.buffer_snapshot().len(),
                            snapshot.buffer_snapshot().text(),
                        );

                        editor.splice_inlays(
                            &[],
                            vec![Inlay::edit_prediction(
                                post_inc(&mut next_inlay_id),
                                snapshot.buffer_snapshot().anchor_before(position),
                                Rope::from_iter(["Test inlay ", "next_inlay_id"]),
                            )],
                            cx,
                        );
                    }
                });
            }),

            // language server updates diagnostics
            _ => {
                let (path, server_id, diagnostics) =
                    match current_diagnostics.iter_mut().choose(&mut rng) {
                        // update existing set of diagnostics
                        Some(((path, server_id), diagnostics)) if rng.random_bool(0.5) => {
                            (path.clone(), *server_id, diagnostics)
                        }

                        // insert a set of diagnostics for a new path
                        _ => {
                            let path: PathBuf =
                                format!(path!("/test/{}.rs"), post_inc(&mut next_filename)).into();
                            let len = rng.random_range(128..256);
                            let content =
                                RandomCharIter::new(&mut rng).take(len).collect::<String>();
                            fs.insert_file(&path, content.into_bytes()).await;

                            let server_id = match language_server_ids.iter().choose(&mut rng) {
                                Some(server_id) if rng.random_bool(0.5) => *server_id,
                                _ => {
                                    let id = LanguageServerId(language_server_ids.len());
                                    language_server_ids.push(id);
                                    id
                                }
                            };

                            (
                                path.clone(),
                                server_id,
                                current_diagnostics.entry((path, server_id)).or_default(),
                            )
                        }
                    };

                updated_language_servers.insert(server_id);

                lsp_store.update(cx, |lsp_store, cx| {
                    log::info!("updating diagnostics. language server {server_id} path {path:?}");
                    randomly_update_diagnostics_for_path(
                        &fs,
                        &path,
                        diagnostics,
                        &mut next_id,
                        &mut rng,
                    );
                    lsp_store
                        .update_diagnostics(
                            server_id,
                            lsp::PublishDiagnosticsParams {
                                uri: lsp::Uri::from_file_path(&path).unwrap_or_else(|_| {
                                    lsp::Uri::from_str("file:///test/fallback.rs").unwrap()
                                }),
                                diagnostics: diagnostics.clone(),
                                version: None,
                            },
                            None,
                            DiagnosticSourceKind::Pushed,
                            &[],
                            cx,
                        )
                        .unwrap()
                });
                cx.executor()
                    .advance_clock(DIAGNOSTICS_UPDATE_DEBOUNCE + Duration::from_millis(10));

                cx.run_until_parked();
            }
        }
    }

    log::info!("updating mutated diagnostics view");
    mutated_diagnostics.update_in(cx, |diagnostics, window, cx| {
        diagnostics.update_stale_excerpts(window, cx)
    });

    cx.executor()
        .advance_clock(DIAGNOSTICS_UPDATE_DEBOUNCE + Duration::from_millis(10));
    cx.run_until_parked();
}

#[gpui::test]
async fn active_diagnostics_dismiss_after_invalidation(cx: &mut TestAppContext) {
    init_test(cx);

    let mut cx = EditorTestContext::new(cx).await;
    let lsp_store =
        cx.update_editor(|editor, _, cx| editor.project().unwrap().read(cx).lsp_store());

    cx.set_state(indoc! {"
        ˇfn func(abc def: i32) -> u32 {
        }
    "});

    let message = "Something's wrong!";
    cx.update(|_, cx| {
        lsp_store.update(cx, |lsp_store, cx| {
            lsp_store
                .update_diagnostics(
                    LanguageServerId(0),
                    lsp::PublishDiagnosticsParams {
                        uri: lsp::Uri::from_file_path(path!("/root/file")).unwrap(),
                        version: None,
                        diagnostics: vec![lsp::Diagnostic {
                            range: lsp::Range::new(
                                lsp::Position::new(0, 11),
                                lsp::Position::new(0, 12),
                            ),
                            severity: Some(lsp::DiagnosticSeverity::ERROR),
                            message: message.to_string(),
                            ..Default::default()
                        }],
                    },
                    None,
                    DiagnosticSourceKind::Pushed,
                    &[],
                    cx,
                )
                .unwrap()
        });
    });
    cx.run_until_parked();

    cx.update_editor(|editor, window, cx| {
        editor.go_to_diagnostic(&GoToDiagnostic::default(), window, cx);
        assert_eq!(
            editor
                .active_diagnostic_group()
                .map(|diagnostics_group| diagnostics_group.active_message.as_str()),
            Some(message),
            "Should have a diagnostics group activated"
        );
    });
    cx.assert_editor_state(indoc! {"
        fn func(abcˇ def: i32) -> u32 {
        }
    "});

    cx.update(|_, cx| {
        lsp_store.update(cx, |lsp_store, cx| {
            lsp_store
                .update_diagnostics(
                    LanguageServerId(0),
                    lsp::PublishDiagnosticsParams {
                        uri: lsp::Uri::from_file_path(path!("/root/file")).unwrap(),
                        version: None,
                        diagnostics: Vec::new(),
                    },
                    None,
                    DiagnosticSourceKind::Pushed,
                    &[],
                    cx,
                )
                .unwrap()
        });
    });
    cx.run_until_parked();
    cx.update_editor(|editor, _, _| {
        assert_eq!(editor.active_diagnostic_group(), None);
    });
    cx.assert_editor_state(indoc! {"
        fn func(abcˇ def: i32) -> u32 {
        }
    "});

    cx.update_editor(|editor, window, cx| {
        editor.go_to_diagnostic(&GoToDiagnostic::default(), window, cx);
        assert_eq!(editor.active_diagnostic_group(), None);
    });
    cx.assert_editor_state(indoc! {"
        fn func(abcˇ def: i32) -> u32 {
        }
    "});
}

#[gpui::test]
async fn cycle_through_same_place_diagnostics(cx: &mut TestAppContext) {
    init_test(cx);

    let mut cx = EditorTestContext::new(cx).await;
    let lsp_store =
        cx.update_editor(|editor, _, cx| editor.project().unwrap().read(cx).lsp_store());

    cx.set_state(indoc! {"
        ˇfn func(abc def: i32) -> u32 {
        }
    "});

    cx.update(|_, cx| {
        lsp_store.update(cx, |lsp_store, cx| {
            lsp_store
                .update_diagnostics(
                    LanguageServerId(0),
                    lsp::PublishDiagnosticsParams {
                        uri: lsp::Uri::from_file_path(path!("/root/file")).unwrap(),
                        version: None,
                        diagnostics: vec![
                            lsp::Diagnostic {
                                range: lsp::Range::new(
                                    lsp::Position::new(0, 11),
                                    lsp::Position::new(0, 12),
                                ),
                                severity: Some(lsp::DiagnosticSeverity::ERROR),
                                ..Default::default()
                            },
                            lsp::Diagnostic {
                                range: lsp::Range::new(
                                    lsp::Position::new(0, 12),
                                    lsp::Position::new(0, 15),
                                ),
                                severity: Some(lsp::DiagnosticSeverity::ERROR),
                                ..Default::default()
                            },
                            lsp::Diagnostic {
                                range: lsp::Range::new(
                                    lsp::Position::new(0, 12),
                                    lsp::Position::new(0, 15),
                                ),
                                severity: Some(lsp::DiagnosticSeverity::ERROR),
                                ..Default::default()
                            },
                            lsp::Diagnostic {
                                range: lsp::Range::new(
                                    lsp::Position::new(0, 25),
                                    lsp::Position::new(0, 28),
                                ),
                                severity: Some(lsp::DiagnosticSeverity::ERROR),
                                ..Default::default()
                            },
                        ],
                    },
                    None,
                    DiagnosticSourceKind::Pushed,
                    &[],
                    cx,
                )
                .unwrap()
        });
    });
    cx.run_until_parked();

    //// Backward

    // Fourth diagnostic
    cx.update_editor(|editor, window, cx| {
        editor.go_to_prev_diagnostic(&GoToPreviousDiagnostic::default(), window, cx);
    });
    cx.assert_editor_state(indoc! {"
        fn func(abc def: i32) -> ˇu32 {
        }
    "});

    // Third diagnostic
    cx.update_editor(|editor, window, cx| {
        editor.go_to_prev_diagnostic(&GoToPreviousDiagnostic::default(), window, cx);
    });
    cx.assert_editor_state(indoc! {"
        fn func(abc ˇdef: i32) -> u32 {
        }
    "});

    // Second diagnostic, same place
    cx.update_editor(|editor, window, cx| {
        editor.go_to_prev_diagnostic(&GoToPreviousDiagnostic::default(), window, cx);
    });
    cx.assert_editor_state(indoc! {"
        fn func(abc ˇdef: i32) -> u32 {
        }
    "});

    // First diagnostic
    cx.update_editor(|editor, window, cx| {
        editor.go_to_prev_diagnostic(&GoToPreviousDiagnostic::default(), window, cx);
    });
    cx.assert_editor_state(indoc! {"
        fn func(abcˇ def: i32) -> u32 {
        }
    "});

    // Wrapped over, fourth diagnostic
    cx.update_editor(|editor, window, cx| {
        editor.go_to_prev_diagnostic(&GoToPreviousDiagnostic::default(), window, cx);
    });
    cx.assert_editor_state(indoc! {"
        fn func(abc def: i32) -> ˇu32 {
        }
    "});

    cx.update_editor(|editor, window, cx| {
        editor.move_to_beginning(&MoveToBeginning, window, cx);
    });
    cx.assert_editor_state(indoc! {"
        ˇfn func(abc def: i32) -> u32 {
        }
    "});

    //// Forward

    // First diagnostic
    cx.update_editor(|editor, window, cx| {
        editor.go_to_diagnostic(&GoToDiagnostic::default(), window, cx);
    });
    cx.assert_editor_state(indoc! {"
        fn func(abcˇ def: i32) -> u32 {
        }
    "});

    // Second diagnostic
    cx.update_editor(|editor, window, cx| {
        editor.go_to_diagnostic(&GoToDiagnostic::default(), window, cx);
    });
    cx.assert_editor_state(indoc! {"
        fn func(abc ˇdef: i32) -> u32 {
        }
    "});

    // Third diagnostic, same place
    cx.update_editor(|editor, window, cx| {
        editor.go_to_diagnostic(&GoToDiagnostic::default(), window, cx);
    });
    cx.assert_editor_state(indoc! {"
        fn func(abc ˇdef: i32) -> u32 {
        }
    "});

    // Fourth diagnostic
    cx.update_editor(|editor, window, cx| {
        editor.go_to_diagnostic(&GoToDiagnostic::default(), window, cx);
    });
    cx.assert_editor_state(indoc! {"
        fn func(abc def: i32) -> ˇu32 {
        }
    "});

    // Wrapped around, first diagnostic
    cx.update_editor(|editor, window, cx| {
        editor.go_to_diagnostic(&GoToDiagnostic::default(), window, cx);
    });
    cx.assert_editor_state(indoc! {"
        fn func(abcˇ def: i32) -> u32 {
        }
    "});
}

#[gpui::test]
async fn test_diagnostics_with_links(cx: &mut TestAppContext) {
    init_test(cx);

    let mut cx = EditorTestContext::new(cx).await;

    cx.set_state(indoc! {"
        fn func(abˇc def: i32) -> u32 {
        }
    "});
    let lsp_store =
        cx.update_editor(|editor, _, cx| editor.project().unwrap().read(cx).lsp_store());

    cx.update(|_, cx| {
        lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.update_diagnostics(
                LanguageServerId(0),
                lsp::PublishDiagnosticsParams {
                    uri: lsp::Uri::from_file_path(path!("/root/file")).unwrap(),
                    version: None,
                    diagnostics: vec![lsp::Diagnostic {
                        range: lsp::Range::new(lsp::Position::new(0, 8), lsp::Position::new(0, 12)),
                        severity: Some(lsp::DiagnosticSeverity::ERROR),
                        message: "we've had problems with <https://link.one>, and <https://link.two> is broken".to_string(),
                        ..Default::default()
                    }],
                },
                None,
                DiagnosticSourceKind::Pushed,
                &[],
                cx,
            )
        })
    }).unwrap();
    cx.run_until_parked();
    cx.update_editor(|editor, window, cx| {
        editor::hover_popover::hover(editor, &Default::default(), window, cx)
    });
    cx.run_until_parked();
    cx.update_editor(|editor, _, _| assert!(editor.hover_state.diagnostic_popover.is_some()))
}

#[gpui::test]
async fn test_hover_diagnostic_and_info_popovers(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
            ..Default::default()
        },
        cx,
    )
    .await;

    // Hover with just diagnostic, pops DiagnosticPopover immediately and then
    // info popover once request completes
    cx.set_state(indoc! {"
        fn teˇst() { println!(); }
    "});
    // Send diagnostic to client
    let range = cx.lsp_range(indoc! {"
        fn «test»() { println!(); }
    "});
    let lsp_store =
        cx.update_editor(|editor, _, cx| editor.project().unwrap().read(cx).lsp_store());
    cx.update(|_, cx| {
        lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.update_diagnostics(
                LanguageServerId(0),
                lsp::PublishDiagnosticsParams {
                    uri: lsp::Uri::from_file_path(path!("/root/dir/file.rs")).unwrap(),
                    version: None,
                    diagnostics: vec![lsp::Diagnostic {
                        range,
                        severity: Some(lsp::DiagnosticSeverity::ERROR),
                        message: "A test diagnostic message.".to_string(),
                        ..Default::default()
                    }],
                },
                None,
                DiagnosticSourceKind::Pushed,
                &[],
                cx,
            )
        })
    })
    .unwrap();
    cx.run_until_parked();

    // Hover pops diagnostic immediately
    cx.update_editor(|editor, window, cx| editor::hover_popover::hover(editor, &Hover, window, cx));
    cx.background_executor.run_until_parked();

    cx.editor(|Editor { hover_state, .. }, _, _| {
        assert!(hover_state.diagnostic_popover.is_some());
        assert!(hover_state.info_popovers.is_empty());
    });

    // Info Popover shows after request responded to
    let range = cx.lsp_range(indoc! {"
            fn «test»() { println!(); }
        "});
    cx.set_request_handler::<lsp::request::HoverRequest, _, _>(move |_, _, _| async move {
        Ok(Some(lsp::Hover {
            contents: lsp::HoverContents::Markup(lsp::MarkupContent {
                kind: lsp::MarkupKind::Markdown,
                value: "some new docs".to_string(),
            }),
            range: Some(range),
        }))
    });
    let delay = cx.update(|_, cx| EditorSettings::get_global(cx).hover_popover_delay.0 + 1);
    cx.background_executor
        .advance_clock(Duration::from_millis(delay));

    cx.background_executor.run_until_parked();
    cx.editor(|Editor { hover_state, .. }, _, _| {
        hover_state.diagnostic_popover.is_some() && hover_state.info_task.is_some()
    });
}
#[gpui::test]
async fn test_diagnostics_with_code(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            "main.js": "
                function test() {
                    const x = 10;
                    const y = 20;
                    return 1;
                }
                test();
            "
            .unindent(),
        }),
    )
    .await;

    let language_server_id = LanguageServerId(0);
    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let lsp_store = project.read_with(cx, |project, _| project.lsp_store());
    let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*window, cx);
    let workspace = window.root(cx).unwrap();
    let uri = lsp::Uri::from_file_path(path!("/root/main.js")).unwrap();

    // Create diagnostics with code fields
    lsp_store.update(cx, |lsp_store, cx| {
        lsp_store
            .update_diagnostics(
                language_server_id,
                lsp::PublishDiagnosticsParams {
                    uri: uri.clone(),
                    diagnostics: vec![
                        lsp::Diagnostic {
                            range: lsp::Range::new(
                                lsp::Position::new(1, 4),
                                lsp::Position::new(1, 14),
                            ),
                            severity: Some(lsp::DiagnosticSeverity::WARNING),
                            code: Some(lsp::NumberOrString::String("no-unused-vars".to_string())),
                            source: Some("eslint".to_string()),
                            message: "'x' is assigned a value but never used".to_string(),
                            ..Default::default()
                        },
                        lsp::Diagnostic {
                            range: lsp::Range::new(
                                lsp::Position::new(2, 4),
                                lsp::Position::new(2, 14),
                            ),
                            severity: Some(lsp::DiagnosticSeverity::WARNING),
                            code: Some(lsp::NumberOrString::String("no-unused-vars".to_string())),
                            source: Some("eslint".to_string()),
                            message: "'y' is assigned a value but never used".to_string(),
                            ..Default::default()
                        },
                    ],
                    version: None,
                },
                None,
                DiagnosticSourceKind::Pushed,
                &[],
                cx,
            )
            .unwrap();
    });

    // Open the project diagnostics view
    let diagnostics = window.build_entity(cx, |window, cx| {
        ProjectDiagnosticsEditor::new(true, project.clone(), workspace.downgrade(), window, cx)
    });
    let editor = diagnostics.update(cx, |diagnostics, _| diagnostics.editor.clone());

    diagnostics
        .next_notification(DIAGNOSTICS_UPDATE_DEBOUNCE + Duration::from_millis(10), cx)
        .await;

    // Verify that the diagnostic codes are displayed correctly
    pretty_assertions::assert_eq!(
        editor_content_with_blocks(&editor, cx),
        indoc::indoc! {
            "§ main.js
             § -----
             function test() {
                 const x = 10; § 'x' is assigned a value but never used (eslint no-unused-vars)
                 const y = 20; § 'y' is assigned a value but never used (eslint no-unused-vars)
                 return 1;
             }"
        }
    );
}

#[gpui::test]
async fn go_to_diagnostic_with_severity(cx: &mut TestAppContext) {
    init_test(cx);

    let mut cx = EditorTestContext::new(cx).await;
    let lsp_store =
        cx.update_editor(|editor, _, cx| editor.project().unwrap().read(cx).lsp_store());

    cx.set_state(indoc! {"error warning info hiˇnt"});

    cx.update(|_, cx| {
        lsp_store.update(cx, |lsp_store, cx| {
            lsp_store
                .update_diagnostics(
                    LanguageServerId(0),
                    lsp::PublishDiagnosticsParams {
                        uri: lsp::Uri::from_file_path(path!("/root/file")).unwrap(),
                        version: None,
                        diagnostics: vec![
                            lsp::Diagnostic {
                                range: lsp::Range::new(
                                    lsp::Position::new(0, 0),
                                    lsp::Position::new(0, 5),
                                ),
                                severity: Some(lsp::DiagnosticSeverity::ERROR),
                                ..Default::default()
                            },
                            lsp::Diagnostic {
                                range: lsp::Range::new(
                                    lsp::Position::new(0, 6),
                                    lsp::Position::new(0, 13),
                                ),
                                severity: Some(lsp::DiagnosticSeverity::WARNING),
                                ..Default::default()
                            },
                            lsp::Diagnostic {
                                range: lsp::Range::new(
                                    lsp::Position::new(0, 14),
                                    lsp::Position::new(0, 18),
                                ),
                                severity: Some(lsp::DiagnosticSeverity::INFORMATION),
                                ..Default::default()
                            },
                            lsp::Diagnostic {
                                range: lsp::Range::new(
                                    lsp::Position::new(0, 19),
                                    lsp::Position::new(0, 23),
                                ),
                                severity: Some(lsp::DiagnosticSeverity::HINT),
                                ..Default::default()
                            },
                        ],
                    },
                    None,
                    DiagnosticSourceKind::Pushed,
                    &[],
                    cx,
                )
                .unwrap()
        });
    });
    cx.run_until_parked();

    macro_rules! go {
        ($severity:expr) => {
            cx.update_editor(|editor, window, cx| {
                editor.go_to_diagnostic(
                    &GoToDiagnostic {
                        severity: $severity,
                    },
                    window,
                    cx,
                );
            });
        };
    }

    // Default, should cycle through all diagnostics
    go!(GoToDiagnosticSeverityFilter::default());
    cx.assert_editor_state(indoc! {"ˇerror warning info hint"});
    go!(GoToDiagnosticSeverityFilter::default());
    cx.assert_editor_state(indoc! {"error ˇwarning info hint"});
    go!(GoToDiagnosticSeverityFilter::default());
    cx.assert_editor_state(indoc! {"error warning ˇinfo hint"});
    go!(GoToDiagnosticSeverityFilter::default());
    cx.assert_editor_state(indoc! {"error warning info ˇhint"});
    go!(GoToDiagnosticSeverityFilter::default());
    cx.assert_editor_state(indoc! {"ˇerror warning info hint"});

    let only_info = GoToDiagnosticSeverityFilter::Only(GoToDiagnosticSeverity::Information);
    go!(only_info);
    cx.assert_editor_state(indoc! {"error warning ˇinfo hint"});
    go!(only_info);
    cx.assert_editor_state(indoc! {"error warning ˇinfo hint"});

    let no_hints = GoToDiagnosticSeverityFilter::Range {
        min: GoToDiagnosticSeverity::Information,
        max: GoToDiagnosticSeverity::Error,
    };

    go!(no_hints);
    cx.assert_editor_state(indoc! {"ˇerror warning info hint"});
    go!(no_hints);
    cx.assert_editor_state(indoc! {"error ˇwarning info hint"});
    go!(no_hints);
    cx.assert_editor_state(indoc! {"error warning ˇinfo hint"});
    go!(no_hints);
    cx.assert_editor_state(indoc! {"ˇerror warning info hint"});

    let warning_info = GoToDiagnosticSeverityFilter::Range {
        min: GoToDiagnosticSeverity::Information,
        max: GoToDiagnosticSeverity::Warning,
    };

    go!(warning_info);
    cx.assert_editor_state(indoc! {"error ˇwarning info hint"});
    go!(warning_info);
    cx.assert_editor_state(indoc! {"error warning ˇinfo hint"});
    go!(warning_info);
    cx.assert_editor_state(indoc! {"error ˇwarning info hint"});
}

#[gpui::test]
async fn test_buffer_diagnostics(cx: &mut TestAppContext) {
    init_test(cx);

    // We'll be creating two different files, both with diagnostics, so we can
    // later verify that, since the `BufferDiagnosticsEditor` only shows
    // diagnostics for the provided path, the diagnostics for the other file
    // will not be shown, contrary to what happens with
    // `ProjectDiagnosticsEditor`.
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/test"),
        json!({
            "main.rs": "
                fn main() {
                    let x = vec![];
                    let y = vec![];
                    a(x);
                    b(y);
                    c(y);
                    d(x);
                }
            "
            .unindent(),
            "other.rs": "
                fn other() {
                    let unused = 42;
                    undefined_function();
                }
            "
            .unindent(),
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;
    let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*window, cx);
    let project_path = project::ProjectPath {
        worktree_id: project.read_with(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        }),
        path: rel_path("main.rs").into(),
    };
    let buffer = project
        .update(cx, |project, cx| {
            project.open_buffer(project_path.clone(), cx)
        })
        .await
        .ok();

    // Create the diagnostics for `main.rs`.
    let language_server_id = LanguageServerId(0);
    let uri = lsp::Uri::from_file_path(path!("/test/main.rs")).unwrap();
    let lsp_store = project.read_with(cx, |project, _| project.lsp_store());

    lsp_store.update(cx, |lsp_store, cx| {
        lsp_store.update_diagnostics(language_server_id, lsp::PublishDiagnosticsParams {
            uri: uri.clone(),
            diagnostics: vec![
                lsp::Diagnostic{
                    range: lsp::Range::new(lsp::Position::new(5, 6), lsp::Position::new(5, 7)),
                    severity: Some(lsp::DiagnosticSeverity::WARNING),
                    message: "use of moved value\nvalue used here after move".to_string(),
                    related_information: Some(vec![
                        lsp::DiagnosticRelatedInformation {
                            location: lsp::Location::new(uri.clone(), lsp::Range::new(lsp::Position::new(2, 8), lsp::Position::new(2, 9))),
                            message: "move occurs because `y` has type `Vec<char>`, which does not implement the `Copy` trait".to_string()
                        },
                        lsp::DiagnosticRelatedInformation {
                            location: lsp::Location::new(uri.clone(), lsp::Range::new(lsp::Position::new(4, 6), lsp::Position::new(4, 7))),
                            message: "value moved here".to_string()
                        },
                    ]),
                    ..Default::default()
                },
                lsp::Diagnostic{
                    range: lsp::Range::new(lsp::Position::new(6, 6), lsp::Position::new(6, 7)),
                    severity: Some(lsp::DiagnosticSeverity::ERROR),
                    message: "use of moved value\nvalue used here after move".to_string(),
                    related_information: Some(vec![
                        lsp::DiagnosticRelatedInformation {
                            location: lsp::Location::new(uri.clone(), lsp::Range::new(lsp::Position::new(1, 8), lsp::Position::new(1, 9))),
                            message: "move occurs because `x` has type `Vec<char>`, which does not implement the `Copy` trait".to_string()
                        },
                        lsp::DiagnosticRelatedInformation {
                            location: lsp::Location::new(uri.clone(), lsp::Range::new(lsp::Position::new(3, 6), lsp::Position::new(3, 7))),
                            message: "value moved here".to_string()
                        },
                    ]),
                    ..Default::default()
                }
            ],
            version: None
        }, None, DiagnosticSourceKind::Pushed, &[], cx).unwrap();

        // Create diagnostics for other.rs to ensure that the file and
        // diagnostics are not included in `BufferDiagnosticsEditor` when it is
        // deployed for main.rs.
        lsp_store.update_diagnostics(language_server_id, lsp::PublishDiagnosticsParams {
            uri: lsp::Uri::from_file_path(path!("/test/other.rs")).unwrap(),
            diagnostics: vec![
                lsp::Diagnostic{
                    range: lsp::Range::new(lsp::Position::new(1, 8), lsp::Position::new(1, 14)),
                    severity: Some(lsp::DiagnosticSeverity::WARNING),
                    message: "unused variable: `unused`".to_string(),
                    ..Default::default()
                },
                lsp::Diagnostic{
                    range: lsp::Range::new(lsp::Position::new(2, 4), lsp::Position::new(2, 22)),
                    severity: Some(lsp::DiagnosticSeverity::ERROR),
                    message: "cannot find function `undefined_function` in this scope".to_string(),
                    ..Default::default()
                }
            ],
            version: None
        }, None, DiagnosticSourceKind::Pushed, &[], cx).unwrap();
    });

    let buffer_diagnostics = window.build_entity(cx, |window, cx| {
        BufferDiagnosticsEditor::new(
            project_path.clone(),
            project.clone(),
            buffer,
            true,
            window,
            cx,
        )
    });
    let editor = buffer_diagnostics.update(cx, |buffer_diagnostics, _| {
        buffer_diagnostics.editor().clone()
    });

    // Since the excerpt updates is handled by a background task, we need to
    // wait a little bit to ensure that the buffer diagnostic's editor content
    // is rendered.
    cx.executor()
        .advance_clock(DIAGNOSTICS_UPDATE_DEBOUNCE + Duration::from_millis(10));

    pretty_assertions::assert_eq!(
        editor_content_with_blocks(&editor, cx),
        indoc::indoc! {
            "§ main.rs
             § -----
             fn main() {
                 let x = vec![];
             § move occurs because `x` has type `Vec<char>`, which does not implement
             § the `Copy` trait (back)
                 let y = vec![];
             § move occurs because `y` has type `Vec<char>`, which does not implement
             § the `Copy` trait
                 a(x); § value moved here
                 b(y); § value moved here
                 c(y);
             § use of moved value
             § value used here after move
                 d(x);
             § use of moved value
             § value used here after move
             § hint: move occurs because `x` has type `Vec<char>`, which does not
             § implement the `Copy` trait
             }"
        }
    );
}

#[gpui::test]
async fn test_buffer_diagnostics_without_warnings(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/test"),
        json!({
            "main.rs": "
                fn main() {
                    let x = vec![];
                    let y = vec![];
                    a(x);
                    b(y);
                    c(y);
                    d(x);
                }
            "
            .unindent(),
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;
    let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*window, cx);
    let project_path = project::ProjectPath {
        worktree_id: project.read_with(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        }),
        path: rel_path("main.rs").into(),
    };
    let buffer = project
        .update(cx, |project, cx| {
            project.open_buffer(project_path.clone(), cx)
        })
        .await
        .ok();

    let language_server_id = LanguageServerId(0);
    let uri = lsp::Uri::from_file_path(path!("/test/main.rs")).unwrap();
    let lsp_store = project.read_with(cx, |project, _| project.lsp_store());

    lsp_store.update(cx, |lsp_store, cx| {
        lsp_store.update_diagnostics(language_server_id, lsp::PublishDiagnosticsParams {
            uri: uri.clone(),
            diagnostics: vec![
                lsp::Diagnostic{
                    range: lsp::Range::new(lsp::Position::new(5, 6), lsp::Position::new(5, 7)),
                    severity: Some(lsp::DiagnosticSeverity::WARNING),
                    message: "use of moved value\nvalue used here after move".to_string(),
                    related_information: Some(vec![
                        lsp::DiagnosticRelatedInformation {
                            location: lsp::Location::new(uri.clone(), lsp::Range::new(lsp::Position::new(2, 8), lsp::Position::new(2, 9))),
                            message: "move occurs because `y` has type `Vec<char>`, which does not implement the `Copy` trait".to_string()
                        },
                        lsp::DiagnosticRelatedInformation {
                            location: lsp::Location::new(uri.clone(), lsp::Range::new(lsp::Position::new(4, 6), lsp::Position::new(4, 7))),
                            message: "value moved here".to_string()
                        },
                    ]),
                    ..Default::default()
                },
                lsp::Diagnostic{
                    range: lsp::Range::new(lsp::Position::new(6, 6), lsp::Position::new(6, 7)),
                    severity: Some(lsp::DiagnosticSeverity::ERROR),
                    message: "use of moved value\nvalue used here after move".to_string(),
                    related_information: Some(vec![
                        lsp::DiagnosticRelatedInformation {
                            location: lsp::Location::new(uri.clone(), lsp::Range::new(lsp::Position::new(1, 8), lsp::Position::new(1, 9))),
                            message: "move occurs because `x` has type `Vec<char>`, which does not implement the `Copy` trait".to_string()
                        },
                        lsp::DiagnosticRelatedInformation {
                            location: lsp::Location::new(uri.clone(), lsp::Range::new(lsp::Position::new(3, 6), lsp::Position::new(3, 7))),
                            message: "value moved here".to_string()
                        },
                    ]),
                    ..Default::default()
                }
            ],
            version: None
        }, None, DiagnosticSourceKind::Pushed, &[], cx).unwrap();
    });

    let include_warnings = false;
    let buffer_diagnostics = window.build_entity(cx, |window, cx| {
        BufferDiagnosticsEditor::new(
            project_path.clone(),
            project.clone(),
            buffer,
            include_warnings,
            window,
            cx,
        )
    });

    let editor = buffer_diagnostics.update(cx, |buffer_diagnostics, _cx| {
        buffer_diagnostics.editor().clone()
    });

    // Since the excerpt updates is handled by a background task, we need to
    // wait a little bit to ensure that the buffer diagnostic's editor content
    // is rendered.
    cx.executor()
        .advance_clock(DIAGNOSTICS_UPDATE_DEBOUNCE + Duration::from_millis(10));

    pretty_assertions::assert_eq!(
        editor_content_with_blocks(&editor, cx),
        indoc::indoc! {
            "§ main.rs
             § -----
             fn main() {
                 let x = vec![];
             § move occurs because `x` has type `Vec<char>`, which does not implement
             § the `Copy` trait (back)
                 let y = vec![];
                 a(x); § value moved here
                 b(y);
                 c(y);
                 d(x);
             § use of moved value
             § value used here after move
             § hint: move occurs because `x` has type `Vec<char>`, which does not
             § implement the `Copy` trait
             }"
        }
    );
}

#[gpui::test]
async fn test_buffer_diagnostics_multiple_servers(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/test"),
        json!({
            "main.rs": "
                fn main() {
                    let x = vec![];
                    let y = vec![];
                    a(x);
                    b(y);
                    c(y);
                    d(x);
                }
            "
            .unindent(),
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;
    let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*window, cx);
    let project_path = project::ProjectPath {
        worktree_id: project.read_with(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        }),
        path: rel_path("main.rs").into(),
    };
    let buffer = project
        .update(cx, |project, cx| {
            project.open_buffer(project_path.clone(), cx)
        })
        .await
        .ok();

    // Create the diagnostics for `main.rs`.
    // Two warnings are being created, one for each language server, in order to
    // assert that both warnings are rendered in the editor.
    let language_server_id_a = LanguageServerId(0);
    let language_server_id_b = LanguageServerId(1);
    let uri = lsp::Uri::from_file_path(path!("/test/main.rs")).unwrap();
    let lsp_store = project.read_with(cx, |project, _| project.lsp_store());

    lsp_store.update(cx, |lsp_store, cx| {
        lsp_store
            .update_diagnostics(
                language_server_id_a,
                lsp::PublishDiagnosticsParams {
                    uri: uri.clone(),
                    diagnostics: vec![lsp::Diagnostic {
                        range: lsp::Range::new(lsp::Position::new(5, 6), lsp::Position::new(5, 7)),
                        severity: Some(lsp::DiagnosticSeverity::WARNING),
                        message: "use of moved value\nvalue used here after move".to_string(),
                        related_information: None,
                        ..Default::default()
                    }],
                    version: None,
                },
                None,
                DiagnosticSourceKind::Pushed,
                &[],
                cx,
            )
            .unwrap();

        lsp_store
            .update_diagnostics(
                language_server_id_b,
                lsp::PublishDiagnosticsParams {
                    uri: uri.clone(),
                    diagnostics: vec![lsp::Diagnostic {
                        range: lsp::Range::new(lsp::Position::new(6, 6), lsp::Position::new(6, 7)),
                        severity: Some(lsp::DiagnosticSeverity::WARNING),
                        message: "use of moved value\nvalue used here after move".to_string(),
                        related_information: None,
                        ..Default::default()
                    }],
                    version: None,
                },
                None,
                DiagnosticSourceKind::Pushed,
                &[],
                cx,
            )
            .unwrap();
    });

    let buffer_diagnostics = window.build_entity(cx, |window, cx| {
        BufferDiagnosticsEditor::new(
            project_path.clone(),
            project.clone(),
            buffer,
            true,
            window,
            cx,
        )
    });
    let editor = buffer_diagnostics.update(cx, |buffer_diagnostics, _| {
        buffer_diagnostics.editor().clone()
    });

    // Since the excerpt updates is handled by a background task, we need to
    // wait a little bit to ensure that the buffer diagnostic's editor content
    // is rendered.
    cx.executor()
        .advance_clock(DIAGNOSTICS_UPDATE_DEBOUNCE + Duration::from_millis(10));

    pretty_assertions::assert_eq!(
        editor_content_with_blocks(&editor, cx),
        indoc::indoc! {
            "§ main.rs
             § -----
                 a(x);
                 b(y);
                 c(y);
             § use of moved value
             § value used here after move
                 d(x);
             § use of moved value
             § value used here after move
             }"
        }
    );

    buffer_diagnostics.update(cx, |buffer_diagnostics, _cx| {
        assert_eq!(
            *buffer_diagnostics.summary(),
            DiagnosticSummary {
                warning_count: 2,
                error_count: 0
            }
        );
    })
}

fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        zlog::init_test();
        let settings = SettingsStore::test(cx);
        cx.set_global(settings);
        theme::init(theme::LoadThemes::JustBase, cx);
        crate::init(cx);
        editor::init(cx);
    });
}

fn randomly_update_diagnostics_for_path(
    fs: &FakeFs,
    path: &Path,
    diagnostics: &mut Vec<lsp::Diagnostic>,
    next_id: &mut usize,
    rng: &mut impl Rng,
) {
    let mutation_count = rng.random_range(1..=3);
    for _ in 0..mutation_count {
        if rng.random_bool(0.3) && !diagnostics.is_empty() {
            let idx = rng.random_range(0..diagnostics.len());
            log::info!("  removing diagnostic at index {idx}");
            diagnostics.remove(idx);
        } else {
            let unique_id = *next_id;
            *next_id += 1;

            let new_diagnostic = random_lsp_diagnostic(rng, fs, path, unique_id);

            let ix = rng.random_range(0..=diagnostics.len());
            log::info!(
                "  inserting {} at index {ix}. {},{}..{},{}",
                new_diagnostic.message,
                new_diagnostic.range.start.line,
                new_diagnostic.range.start.character,
                new_diagnostic.range.end.line,
                new_diagnostic.range.end.character,
            );
            for related in new_diagnostic.related_information.iter().flatten() {
                log::info!(
                    "   {}. {},{}..{},{}",
                    related.message,
                    related.location.range.start.line,
                    related.location.range.start.character,
                    related.location.range.end.line,
                    related.location.range.end.character,
                );
            }
            diagnostics.insert(ix, new_diagnostic);
        }
    }
}

fn random_lsp_diagnostic(
    rng: &mut impl Rng,
    fs: &FakeFs,
    path: &Path,
    unique_id: usize,
) -> lsp::Diagnostic {
    // Intentionally allow erroneous ranges some of the time (that run off the end of the file),
    // because language servers can potentially give us those, and we should handle them gracefully.
    const ERROR_MARGIN: usize = 10;

    let file_content = fs.read_file_sync(path).unwrap();
    let file_text = Rope::from(String::from_utf8_lossy(&file_content).as_ref());

    let start = rng.random_range(0..file_text.len().saturating_add(ERROR_MARGIN));
    let end = rng.random_range(start..file_text.len().saturating_add(ERROR_MARGIN));

    let start_point = file_text.offset_to_point_utf16(start);
    let end_point = file_text.offset_to_point_utf16(end);

    let range = lsp::Range::new(
        lsp::Position::new(start_point.row, start_point.column),
        lsp::Position::new(end_point.row, end_point.column),
    );

    let severity = if rng.random_bool(0.5) {
        Some(lsp::DiagnosticSeverity::ERROR)
    } else {
        Some(lsp::DiagnosticSeverity::WARNING)
    };

    let message = format!("diagnostic {unique_id}");

    let related_information = if rng.random_bool(0.3) {
        let info_count = rng.random_range(1..=3);
        let mut related_info = Vec::with_capacity(info_count);

        for i in 0..info_count {
            let info_start = rng.random_range(0..file_text.len().saturating_add(ERROR_MARGIN));
            let info_end =
                rng.random_range(info_start..file_text.len().saturating_add(ERROR_MARGIN));

            let info_start_point = file_text.offset_to_point_utf16(info_start);
            let info_end_point = file_text.offset_to_point_utf16(info_end);

            let info_range = lsp::Range::new(
                lsp::Position::new(info_start_point.row, info_start_point.column),
                lsp::Position::new(info_end_point.row, info_end_point.column),
            );

            related_info.push(lsp::DiagnosticRelatedInformation {
                location: lsp::Location::new(lsp::Uri::from_file_path(path).unwrap(), info_range),
                message: format!("related info {i} for diagnostic {unique_id}"),
            });
        }

        Some(related_info)
    } else {
        None
    };

    lsp::Diagnostic {
        range,
        severity,
        message,
        related_information,
        data: None,
        ..Default::default()
    }
}
