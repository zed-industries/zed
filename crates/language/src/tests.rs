use super::*;
use gpui::{ModelHandle, MutableAppContext};
use std::{
    cell::RefCell,
    iter::FromIterator,
    ops::Range,
    rc::Rc,
    time::{Duration, Instant},
};
use unindent::Unindent as _;

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    // std::env::set_var("RUST_LOG", "info");
    env_logger::init();
}

#[test]
fn test_select_language() {
    let registry = LanguageRegistry {
        languages: vec![
            Arc::new(Language::new(
                LanguageConfig {
                    name: "Rust".to_string(),
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
                Some(tree_sitter_rust::language()),
            )),
            Arc::new(Language::new(
                LanguageConfig {
                    name: "Make".to_string(),
                    path_suffixes: vec!["Makefile".to_string(), "mk".to_string()],
                    ..Default::default()
                },
                Some(tree_sitter_rust::language()),
            )),
        ],
    };

    // matching file extension
    assert_eq!(
        registry.select_language("zed/lib.rs").map(|l| l.name()),
        Some("Rust")
    );
    assert_eq!(
        registry.select_language("zed/lib.mk").map(|l| l.name()),
        Some("Make")
    );

    // matching filename
    assert_eq!(
        registry.select_language("zed/Makefile").map(|l| l.name()),
        Some("Make")
    );

    // matching suffix that is not the full file extension or filename
    assert_eq!(registry.select_language("zed/cars").map(|l| l.name()), None);
    assert_eq!(
        registry.select_language("zed/a.cars").map(|l| l.name()),
        None
    );
    assert_eq!(registry.select_language("zed/sumk").map(|l| l.name()), None);
}

#[gpui::test]
fn test_edit_events(cx: &mut gpui::MutableAppContext) {
    let mut now = Instant::now();
    let buffer_1_events = Rc::new(RefCell::new(Vec::new()));
    let buffer_2_events = Rc::new(RefCell::new(Vec::new()));

    let buffer1 = cx.add_model(|cx| Buffer::new(0, "abcdef", cx));
    let buffer2 = cx.add_model(|cx| Buffer::new(1, "abcdef", cx));
    let buffer_ops = buffer1.update(cx, |buffer, cx| {
        let buffer_1_events = buffer_1_events.clone();
        cx.subscribe(&buffer1, move |_, _, event, _| {
            buffer_1_events.borrow_mut().push(event.clone())
        })
        .detach();
        let buffer_2_events = buffer_2_events.clone();
        cx.subscribe(&buffer2, move |_, _, event, _| {
            buffer_2_events.borrow_mut().push(event.clone())
        })
        .detach();

        // An edit emits an edited event, followed by a dirtied event,
        // since the buffer was previously in a clean state.
        buffer.edit(Some(2..4), "XYZ", cx);

        // An empty transaction does not emit any events.
        buffer.start_transaction();
        buffer.end_transaction(cx);

        // A transaction containing two edits emits one edited event.
        now += Duration::from_secs(1);
        buffer.start_transaction_at(now);
        buffer.edit(Some(5..5), "u", cx);
        buffer.edit(Some(6..6), "w", cx);
        buffer.end_transaction_at(now, cx);

        // Undoing a transaction emits one edited event.
        buffer.undo(cx);

        buffer.operations.clone()
    });

    // Incorporating a set of remote ops emits a single edited event,
    // followed by a dirtied event.
    buffer2.update(cx, |buffer, cx| {
        buffer.apply_ops(buffer_ops, cx).unwrap();
    });

    let buffer_1_events = buffer_1_events.borrow();
    assert_eq!(
        *buffer_1_events,
        vec![Event::Edited, Event::Dirtied, Event::Edited, Event::Edited]
    );

    let buffer_2_events = buffer_2_events.borrow();
    assert_eq!(*buffer_2_events, vec![Event::Edited, Event::Dirtied]);
}

#[gpui::test]
async fn test_apply_diff(mut cx: gpui::TestAppContext) {
    let text = "a\nbb\nccc\ndddd\neeeee\nffffff\n";
    let buffer = cx.add_model(|cx| Buffer::new(0, text, cx));

    let text = "a\nccc\ndddd\nffffff\n";
    let diff = buffer.read_with(&cx, |b, cx| b.diff(text.into(), cx)).await;
    buffer.update(&mut cx, |b, cx| b.apply_diff(diff, cx));
    cx.read(|cx| assert_eq!(buffer.read(cx).text(), text));

    let text = "a\n1\n\nccc\ndd2dd\nffffff\n";
    let diff = buffer.read_with(&cx, |b, cx| b.diff(text.into(), cx)).await;
    buffer.update(&mut cx, |b, cx| b.apply_diff(diff, cx));
    cx.read(|cx| assert_eq!(buffer.read(cx).text(), text));
}

#[gpui::test]
async fn test_reparse(mut cx: gpui::TestAppContext) {
    let text = "fn a() {}";
    let buffer = cx.add_model(|cx| {
        Buffer::new(0, text, cx).with_language(Some(Arc::new(rust_lang())), None, cx)
    });

    // Wait for the initial text to parse
    buffer
        .condition(&cx, |buffer, _| !buffer.is_parsing())
        .await;
    assert_eq!(
        get_tree_sexp(&buffer, &cx),
        concat!(
            "(source_file (function_item name: (identifier) ",
            "parameters: (parameters) ",
            "body: (block)))"
        )
    );

    buffer.update(&mut cx, |buffer, _| {
        buffer.set_sync_parse_timeout(Duration::ZERO)
    });

    // Perform some edits (add parameter and variable reference)
    // Parsing doesn't begin until the transaction is complete
    buffer.update(&mut cx, |buf, cx| {
        buf.start_transaction();

        let offset = buf.text().find(")").unwrap();
        buf.edit(vec![offset..offset], "b: C", cx);
        assert!(!buf.is_parsing());

        let offset = buf.text().find("}").unwrap();
        buf.edit(vec![offset..offset], " d; ", cx);
        assert!(!buf.is_parsing());

        buf.end_transaction(cx);
        assert_eq!(buf.text(), "fn a(b: C) { d; }");
        assert!(buf.is_parsing());
    });
    buffer
        .condition(&cx, |buffer, _| !buffer.is_parsing())
        .await;
    assert_eq!(
        get_tree_sexp(&buffer, &cx),
        concat!(
            "(source_file (function_item name: (identifier) ",
            "parameters: (parameters (parameter pattern: (identifier) type: (type_identifier))) ",
            "body: (block (identifier))))"
        )
    );

    // Perform a series of edits without waiting for the current parse to complete:
    // * turn identifier into a field expression
    // * turn field expression into a method call
    // * add a turbofish to the method call
    buffer.update(&mut cx, |buf, cx| {
        let offset = buf.text().find(";").unwrap();
        buf.edit(vec![offset..offset], ".e", cx);
        assert_eq!(buf.text(), "fn a(b: C) { d.e; }");
        assert!(buf.is_parsing());
    });
    buffer.update(&mut cx, |buf, cx| {
        let offset = buf.text().find(";").unwrap();
        buf.edit(vec![offset..offset], "(f)", cx);
        assert_eq!(buf.text(), "fn a(b: C) { d.e(f); }");
        assert!(buf.is_parsing());
    });
    buffer.update(&mut cx, |buf, cx| {
        let offset = buf.text().find("(f)").unwrap();
        buf.edit(vec![offset..offset], "::<G>", cx);
        assert_eq!(buf.text(), "fn a(b: C) { d.e::<G>(f); }");
        assert!(buf.is_parsing());
    });
    buffer
        .condition(&cx, |buffer, _| !buffer.is_parsing())
        .await;
    assert_eq!(
        get_tree_sexp(&buffer, &cx),
        concat!(
            "(source_file (function_item name: (identifier) ",
            "parameters: (parameters (parameter pattern: (identifier) type: (type_identifier))) ",
            "body: (block (call_expression ",
            "function: (generic_function ",
            "function: (field_expression value: (identifier) field: (field_identifier)) ",
            "type_arguments: (type_arguments (type_identifier))) ",
            "arguments: (arguments (identifier))))))",
        )
    );

    buffer.update(&mut cx, |buf, cx| {
        buf.undo(cx);
        assert_eq!(buf.text(), "fn a() {}");
        assert!(buf.is_parsing());
    });
    buffer
        .condition(&cx, |buffer, _| !buffer.is_parsing())
        .await;
    assert_eq!(
        get_tree_sexp(&buffer, &cx),
        concat!(
            "(source_file (function_item name: (identifier) ",
            "parameters: (parameters) ",
            "body: (block)))"
        )
    );

    buffer.update(&mut cx, |buf, cx| {
        buf.redo(cx);
        assert_eq!(buf.text(), "fn a(b: C) { d.e::<G>(f); }");
        assert!(buf.is_parsing());
    });
    buffer
        .condition(&cx, |buffer, _| !buffer.is_parsing())
        .await;
    assert_eq!(
        get_tree_sexp(&buffer, &cx),
        concat!(
            "(source_file (function_item name: (identifier) ",
            "parameters: (parameters (parameter pattern: (identifier) type: (type_identifier))) ",
            "body: (block (call_expression ",
            "function: (generic_function ",
            "function: (field_expression value: (identifier) field: (field_identifier)) ",
            "type_arguments: (type_arguments (type_identifier))) ",
            "arguments: (arguments (identifier))))))",
        )
    );

    fn get_tree_sexp(buffer: &ModelHandle<Buffer>, cx: &gpui::TestAppContext) -> String {
        buffer.read_with(cx, |buffer, _| {
            buffer.syntax_tree().unwrap().root_node().to_sexp()
        })
    }
}

#[gpui::test]
fn test_enclosing_bracket_ranges(cx: &mut MutableAppContext) {
    let buffer = cx.add_model(|cx| {
        let text = "
            mod x {
                mod y {

                }
            }
        "
        .unindent();
        Buffer::new(0, text, cx).with_language(Some(Arc::new(rust_lang())), None, cx)
    });
    let buffer = buffer.read(cx);
    assert_eq!(
        buffer.enclosing_bracket_point_ranges(Point::new(1, 6)..Point::new(1, 6)),
        Some((
            Point::new(0, 6)..Point::new(0, 7),
            Point::new(4, 0)..Point::new(4, 1)
        ))
    );
    assert_eq!(
        buffer.enclosing_bracket_point_ranges(Point::new(1, 10)..Point::new(1, 10)),
        Some((
            Point::new(1, 10)..Point::new(1, 11),
            Point::new(3, 4)..Point::new(3, 5)
        ))
    );
    assert_eq!(
        buffer.enclosing_bracket_point_ranges(Point::new(3, 5)..Point::new(3, 5)),
        Some((
            Point::new(1, 10)..Point::new(1, 11),
            Point::new(3, 4)..Point::new(3, 5)
        ))
    );
}

#[gpui::test]
fn test_edit_with_autoindent(cx: &mut MutableAppContext) {
    cx.add_model(|cx| {
        let text = "fn a() {}";
        let mut buffer =
            Buffer::new(0, text, cx).with_language(Some(Arc::new(rust_lang())), None, cx);

        buffer.edit_with_autoindent([8..8], "\n\n", cx);
        assert_eq!(buffer.text(), "fn a() {\n    \n}");

        buffer.edit_with_autoindent([Point::new(1, 4)..Point::new(1, 4)], "b()\n", cx);
        assert_eq!(buffer.text(), "fn a() {\n    b()\n    \n}");

        buffer.edit_with_autoindent([Point::new(2, 4)..Point::new(2, 4)], ".c", cx);
        assert_eq!(buffer.text(), "fn a() {\n    b()\n        .c\n}");

        buffer
    });
}

// We need another approach to managing selections with auto-indent

// #[gpui::test]
// fn test_autoindent_moves_selections(cx: &mut MutableAppContext) {
//     cx.add_model(|cx| {
//         let text = "fn a() {}";

//         let mut buffer =
//             Buffer::new(0, text, cx).with_language(Some(Arc::new(rust_lang())), None, cx);

//         let selection_set_id = buffer.add_selection_set::<usize>(&[], cx);
//         buffer.start_transaction();
//         buffer.edit_with_autoindent([5..5, 9..9], "\n\n", cx);
//         buffer
//             .update_selection_set(
//                 selection_set_id,
//                 &[
//                     Selection {
//                         id: 0,
//                         start: Point::new(1, 0),
//                         end: Point::new(1, 0),
//                         reversed: false,
//                         goal: SelectionGoal::None,
//                     },
//                     Selection {
//                         id: 1,
//                         start: Point::new(4, 0),
//                         end: Point::new(4, 0),
//                         reversed: false,
//                         goal: SelectionGoal::None,
//                     },
//                 ],
//                 cx,
//             )
//             .unwrap();
//         assert_eq!(buffer.text(), "fn a(\n\n) {}\n\n");

//         // TODO! Come up with a different approach to moving selections now that we don't manage selection sets in the buffer

//         // Ending the transaction runs the auto-indent. The selection
//         // at the start of the auto-indented row is pushed to the right.
//         buffer.end_transaction(cx);
//         assert_eq!(buffer.text(), "fn a(\n    \n) {}\n\n");
//         let selection_ranges = buffer
//             .selection_set(selection_set_id)
//             .unwrap()
//             .selections::<Point>(&buffer)
//             .map(|selection| selection.start.to_point(&buffer)..selection.end.to_point(&buffer))
//             .collect::<Vec<_>>();

//         assert_eq!(selection_ranges[0], empty(Point::new(1, 4)));
//         assert_eq!(selection_ranges[1], empty(Point::new(4, 0)));

//         buffer
//     });
// }

#[gpui::test]
fn test_autoindent_does_not_adjust_lines_with_unchanged_suggestion(cx: &mut MutableAppContext) {
    cx.add_model(|cx| {
        let text = "
            fn a() {
            c;
            d;
            }
        "
        .unindent();

        let mut buffer =
            Buffer::new(0, text, cx).with_language(Some(Arc::new(rust_lang())), None, cx);

        // Lines 2 and 3 don't match the indentation suggestion. When editing these lines,
        // their indentation is not adjusted.
        buffer.edit_with_autoindent([empty(Point::new(1, 1)), empty(Point::new(2, 1))], "()", cx);
        assert_eq!(
            buffer.text(),
            "
            fn a() {
            c();
            d();
            }
            "
            .unindent()
        );

        // When appending new content after these lines, the indentation is based on the
        // preceding lines' actual indentation.
        buffer.edit_with_autoindent(
            [empty(Point::new(1, 1)), empty(Point::new(2, 1))],
            "\n.f\n.g",
            cx,
        );
        assert_eq!(
            buffer.text(),
            "
            fn a() {
            c
                .f
                .g();
            d
                .f
                .g();
            }
            "
            .unindent()
        );
        buffer
    });
}

#[gpui::test]
fn test_autoindent_adjusts_lines_when_only_text_changes(cx: &mut MutableAppContext) {
    cx.add_model(|cx| {
        let text = "
            fn a() {}
        "
        .unindent();

        let mut buffer =
            Buffer::new(0, text, cx).with_language(Some(Arc::new(rust_lang())), None, cx);

        buffer.edit_with_autoindent([5..5], "\nb", cx);
        assert_eq!(
            buffer.text(),
            "
                fn a(
                    b) {}
            "
            .unindent()
        );

        // The indentation suggestion changed because `@end` node (a close paren)
        // is now at the beginning of the line.
        buffer.edit_with_autoindent([Point::new(1, 4)..Point::new(1, 5)], "", cx);
        assert_eq!(
            buffer.text(),
            "
                fn a(
                ) {}
            "
            .unindent()
        );

        buffer
    });
}

#[gpui::test]
async fn test_diagnostics(mut cx: gpui::TestAppContext) {
    let (language_server, mut fake) = lsp::LanguageServer::fake(cx.background()).await;
    let mut rust_lang = rust_lang();
    rust_lang.config.language_server = Some(LanguageServerConfig {
        disk_based_diagnostic_sources: HashSet::from_iter(["disk".to_string()]),
        ..Default::default()
    });

    let text = "
        fn a() { A }
        fn b() { BB }
        fn c() { CCC }
    "
    .unindent();

    let buffer = cx.add_model(|cx| {
        Buffer::new(0, text, cx).with_language(Some(Arc::new(rust_lang)), Some(language_server), cx)
    });

    let open_notification = fake
        .receive_notification::<lsp::notification::DidOpenTextDocument>()
        .await;

    // Edit the buffer, moving the content down
    buffer.update(&mut cx, |buffer, cx| buffer.edit([0..0], "\n\n", cx));
    let change_notification_1 = fake
        .receive_notification::<lsp::notification::DidChangeTextDocument>()
        .await;
    assert!(change_notification_1.text_document.version > open_notification.text_document.version);

    buffer.update(&mut cx, |buffer, cx| {
        // Receive diagnostics for an earlier version of the buffer.
        buffer
            .update_diagnostics(
                Some(open_notification.text_document.version),
                vec![
                    DiagnosticEntry {
                        range: PointUtf16::new(0, 9)..PointUtf16::new(0, 10),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::ERROR,
                            message: "undefined variable 'A'".to_string(),
                            source: Some("disk".to_string()),
                            group_id: 0,
                            is_primary: true,
                            ..Default::default()
                        },
                    },
                    DiagnosticEntry {
                        range: PointUtf16::new(1, 9)..PointUtf16::new(1, 11),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::ERROR,
                            message: "undefined variable 'BB'".to_string(),
                            source: Some("disk".to_string()),
                            group_id: 1,
                            is_primary: true,
                            ..Default::default()
                        },
                    },
                    DiagnosticEntry {
                        range: PointUtf16::new(2, 9)..PointUtf16::new(2, 12),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::ERROR,
                            source: Some("disk".to_string()),
                            message: "undefined variable 'CCC'".to_string(),
                            group_id: 2,
                            is_primary: true,
                            ..Default::default()
                        },
                    },
                ],
                cx,
            )
            .unwrap();

        // The diagnostics have moved down since they were created.
        assert_eq!(
            buffer
                .snapshot()
                .diagnostics_in_range::<_, Point>(Point::new(3, 0)..Point::new(5, 0))
                .collect::<Vec<_>>(),
            &[
                DiagnosticEntry {
                    range: Point::new(3, 9)..Point::new(3, 11),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::ERROR,
                        message: "undefined variable 'BB'".to_string(),
                        source: Some("disk".to_string()),
                        group_id: 1,
                        is_primary: true,
                        ..Default::default()
                    },
                },
                DiagnosticEntry {
                    range: Point::new(4, 9)..Point::new(4, 12),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::ERROR,
                        message: "undefined variable 'CCC'".to_string(),
                        source: Some("disk".to_string()),
                        group_id: 2,
                        is_primary: true,
                        ..Default::default()
                    }
                }
            ]
        );
        assert_eq!(
            chunks_with_diagnostics(buffer, 0..buffer.len()),
            [
                ("\n\nfn a() { ".to_string(), None),
                ("A".to_string(), Some(DiagnosticSeverity::ERROR)),
                (" }\nfn b() { ".to_string(), None),
                ("BB".to_string(), Some(DiagnosticSeverity::ERROR)),
                (" }\nfn c() { ".to_string(), None),
                ("CCC".to_string(), Some(DiagnosticSeverity::ERROR)),
                (" }\n".to_string(), None),
            ]
        );
        assert_eq!(
            chunks_with_diagnostics(buffer, Point::new(3, 10)..Point::new(4, 11)),
            [
                ("B".to_string(), Some(DiagnosticSeverity::ERROR)),
                (" }\nfn c() { ".to_string(), None),
                ("CC".to_string(), Some(DiagnosticSeverity::ERROR)),
            ]
        );

        // Ensure overlapping diagnostics are highlighted correctly.
        buffer
            .update_diagnostics(
                Some(open_notification.text_document.version),
                vec![
                    DiagnosticEntry {
                        range: PointUtf16::new(0, 9)..PointUtf16::new(0, 10),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::ERROR,
                            message: "undefined variable 'A'".to_string(),
                            source: Some("disk".to_string()),
                            group_id: 0,
                            is_primary: true,
                            ..Default::default()
                        },
                    },
                    DiagnosticEntry {
                        range: PointUtf16::new(0, 9)..PointUtf16::new(0, 12),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::WARNING,
                            message: "unreachable statement".to_string(),
                            group_id: 1,
                            is_primary: true,
                            ..Default::default()
                        },
                    },
                ],
                cx,
            )
            .unwrap();
        assert_eq!(
            buffer
                .snapshot()
                .diagnostics_in_range::<_, Point>(Point::new(2, 0)..Point::new(3, 0))
                .collect::<Vec<_>>(),
            &[
                DiagnosticEntry {
                    range: Point::new(2, 9)..Point::new(2, 12),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::WARNING,
                        message: "unreachable statement".to_string(),
                        group_id: 3,
                        is_primary: true,
                        ..Default::default()
                    }
                },
                DiagnosticEntry {
                    range: Point::new(2, 9)..Point::new(2, 10),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::ERROR,
                        message: "undefined variable 'A'".to_string(),
                        source: Some("disk".to_string()),
                        group_id: 0,
                        is_primary: true,
                        ..Default::default()
                    },
                }
            ]
        );
        assert_eq!(
            chunks_with_diagnostics(buffer, Point::new(2, 0)..Point::new(3, 0)),
            [
                ("fn a() { ".to_string(), None),
                ("A".to_string(), Some(DiagnosticSeverity::ERROR)),
                (" }".to_string(), Some(DiagnosticSeverity::WARNING)),
                ("\n".to_string(), None),
            ]
        );
        assert_eq!(
            chunks_with_diagnostics(buffer, Point::new(2, 10)..Point::new(3, 0)),
            [
                (" }".to_string(), Some(DiagnosticSeverity::WARNING)),
                ("\n".to_string(), None),
            ]
        );
    });

    // Keep editing the buffer and ensure disk-based diagnostics get translated according to the
    // changes since the last save.
    buffer.update(&mut cx, |buffer, cx| {
        buffer.edit(Some(Point::new(2, 0)..Point::new(2, 0)), "    ", cx);
        buffer.edit(Some(Point::new(2, 8)..Point::new(2, 10)), "(x: usize)", cx);
    });
    let change_notification_2 = fake
        .receive_notification::<lsp::notification::DidChangeTextDocument>()
        .await;
    assert!(
        change_notification_2.text_document.version > change_notification_1.text_document.version
    );

    buffer.update(&mut cx, |buffer, cx| {
        buffer
            .update_diagnostics(
                Some(change_notification_2.text_document.version),
                vec![
                    DiagnosticEntry {
                        range: PointUtf16::new(1, 9)..PointUtf16::new(1, 11),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::ERROR,
                            message: "undefined variable 'BB'".to_string(),
                            source: Some("disk".to_string()),
                            group_id: 1,
                            is_primary: true,
                            ..Default::default()
                        },
                    },
                    DiagnosticEntry {
                        range: PointUtf16::new(0, 9)..PointUtf16::new(0, 10),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::ERROR,
                            message: "undefined variable 'A'".to_string(),
                            source: Some("disk".to_string()),
                            group_id: 0,
                            is_primary: true,
                            ..Default::default()
                        },
                    },
                ],
                cx,
            )
            .unwrap();
        assert_eq!(
            buffer
                .snapshot()
                .diagnostics_in_range::<_, Point>(0..buffer.len())
                .collect::<Vec<_>>(),
            &[
                DiagnosticEntry {
                    range: Point::new(2, 21)..Point::new(2, 22),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::ERROR,
                        message: "undefined variable 'A'".to_string(),
                        source: Some("disk".to_string()),
                        group_id: 0,
                        is_primary: true,
                        ..Default::default()
                    }
                },
                DiagnosticEntry {
                    range: Point::new(3, 9)..Point::new(3, 11),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::ERROR,
                        message: "undefined variable 'BB'".to_string(),
                        source: Some("disk".to_string()),
                        group_id: 4,
                        is_primary: true,
                        ..Default::default()
                    },
                }
            ]
        );
    });
}

#[gpui::test]
async fn test_preserving_old_group_ids_and_disk_based_diagnostics(mut cx: gpui::TestAppContext) {
    let buffer = cx.add_model(|cx| {
        let text = "
            use a::*;
            const b: i32 = c::;
            const c: i32 = d;
            const e: i32 = f +;
        "
        .unindent();

        let mut rust_lang = rust_lang();
        rust_lang.config.language_server = Some(LanguageServerConfig {
            disk_based_diagnostic_sources: HashSet::from_iter(["disk".to_string()]),
            ..Default::default()
        });

        let mut buffer = Buffer::new(0, text, cx);
        buffer.set_language(Some(Arc::new(rust_lang)), None, cx);
        buffer
    });

    // Initially, there are three errors. The second one is disk-based.
    let diagnostics = vec![
        DiagnosticEntry {
            range: PointUtf16::new(1, 16)..PointUtf16::new(1, 18),
            diagnostic: Diagnostic {
                severity: DiagnosticSeverity::ERROR,
                message: "syntax error 1".to_string(),
                group_id: 0,
                is_primary: true,
                is_valid: true,
                ..Default::default()
            },
        },
        DiagnosticEntry {
            range: PointUtf16::new(2, 15)..PointUtf16::new(2, 16),
            diagnostic: Diagnostic {
                severity: DiagnosticSeverity::ERROR,
                message: "cannot find value `d` in this scope".to_string(),
                source: Some("disk".to_string()),
                group_id: 1,
                is_primary: true,
                is_valid: true,
                ..Default::default()
            },
        },
        DiagnosticEntry {
            range: PointUtf16::new(3, 17)..PointUtf16::new(3, 18),
            diagnostic: Diagnostic {
                severity: DiagnosticSeverity::ERROR,
                message: "syntax error 2".to_string(),
                group_id: 2,
                is_primary: true,
                is_valid: true,
                ..Default::default()
            },
        },
    ];
    buffer.update(&mut cx, |buffer, cx| {
        buffer
            .update_diagnostics(None, diagnostics.clone(), cx)
            .unwrap();
        assert_eq!(
            buffer
                .snapshot()
                .diagnostics_in_range::<_, PointUtf16>(PointUtf16::new(0, 0)..PointUtf16::new(4, 0))
                .collect::<Vec<_>>(),
            diagnostics.as_slice(),
        );
    });

    // The diagnostics are updated. The disk-based diagnostic is omitted, and one
    // other diagnostic has changed its message.
    let mut new_diagnostics = vec![diagnostics[0].clone(), diagnostics[2].clone()];
    new_diagnostics[0].diagnostic.message = "another syntax error".to_string();

    buffer.update(&mut cx, |buffer, cx| {
        buffer
            .update_diagnostics(None, new_diagnostics.clone(), cx)
            .unwrap();
        assert_eq!(
            buffer
                .snapshot()
                .diagnostics_in_range::<_, PointUtf16>(PointUtf16::new(0, 0)..PointUtf16::new(4, 0))
                .collect::<Vec<_>>(),
            &[
                // The changed diagnostic is given a new group id.
                DiagnosticEntry {
                    range: new_diagnostics[0].range.clone(),
                    diagnostic: Diagnostic {
                        group_id: 3,
                        ..new_diagnostics[0].diagnostic.clone()
                    },
                },
                // The old disk-based diagnostic is marked as invalid, but keeps
                // its original group id.
                DiagnosticEntry {
                    range: diagnostics[1].range.clone(),
                    diagnostic: Diagnostic {
                        is_valid: false,
                        ..diagnostics[1].diagnostic.clone()
                    },
                },
                // The unchanged diagnostic keeps its original group id
                new_diagnostics[1].clone(),
            ],
        );
    });
}

#[gpui::test]
async fn test_empty_diagnostic_ranges(mut cx: gpui::TestAppContext) {
    cx.add_model(|cx| {
        let text = concat!(
            "let one = ;\n", //
            "let two = \n",
            "let three = 3;\n",
        );

        let mut buffer = Buffer::new(0, text, cx);
        buffer.set_language(Some(Arc::new(rust_lang())), None, cx);
        buffer
            .update_diagnostics(
                None,
                vec![
                    DiagnosticEntry {
                        range: PointUtf16::new(0, 10)..PointUtf16::new(0, 10),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::ERROR,
                            message: "syntax error 1".to_string(),
                            ..Default::default()
                        },
                    },
                    DiagnosticEntry {
                        range: PointUtf16::new(1, 10)..PointUtf16::new(1, 10),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::ERROR,
                            message: "syntax error 2".to_string(),
                            ..Default::default()
                        },
                    },
                ],
                cx,
            )
            .unwrap();

        // An empty range is extended forward to include the following character.
        // At the end of a line, an empty range is extended backward to include
        // the preceding character.
        let chunks = chunks_with_diagnostics(&buffer, 0..buffer.len());
        assert_eq!(
            chunks
                .iter()
                .map(|(s, d)| (s.as_str(), *d))
                .collect::<Vec<_>>(),
            &[
                ("let one = ", None),
                (";", Some(DiagnosticSeverity::ERROR)),
                ("\nlet two =", None),
                (" ", Some(DiagnosticSeverity::ERROR)),
                ("\nlet three = 3;\n", None)
            ]
        );
        buffer
    });
}

fn chunks_with_diagnostics<T: ToOffset + ToPoint>(
    buffer: &Buffer,
    range: Range<T>,
) -> Vec<(String, Option<DiagnosticSeverity>)> {
    let mut chunks: Vec<(String, Option<DiagnosticSeverity>)> = Vec::new();
    for chunk in buffer.snapshot().chunks(range, Some(&Default::default())) {
        if chunks
            .last()
            .map_or(false, |prev_chunk| prev_chunk.1 == chunk.diagnostic)
        {
            chunks.last_mut().unwrap().0.push_str(chunk.text);
        } else {
            chunks.push((chunk.text.to_string(), chunk.diagnostic));
        }
    }
    chunks
}

#[test]
fn test_contiguous_ranges() {
    assert_eq!(
        contiguous_ranges([1, 2, 3, 5, 6, 9, 10, 11, 12].into_iter(), 100).collect::<Vec<_>>(),
        &[1..4, 5..7, 9..13]
    );

    // Respects the `max_len` parameter
    assert_eq!(
        contiguous_ranges(
            [2, 3, 4, 5, 6, 7, 8, 9, 23, 24, 25, 26, 30, 31].into_iter(),
            3
        )
        .collect::<Vec<_>>(),
        &[2..5, 5..8, 8..10, 23..26, 26..27, 30..32],
    );
}

impl Buffer {
    pub fn enclosing_bracket_point_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> Option<(Range<Point>, Range<Point>)> {
        self.snapshot()
            .enclosing_bracket_ranges(range)
            .map(|(start, end)| {
                let point_start = start.start.to_point(self)..start.end.to_point(self);
                let point_end = end.start.to_point(self)..end.end.to_point(self);
                (point_start, point_end)
            })
    }
}

fn rust_lang() -> Language {
    Language::new(
        LanguageConfig {
            name: "Rust".to_string(),
            path_suffixes: vec!["rs".to_string()],
            language_server: None,
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    )
    .with_indents_query(
        r#"
                (call_expression) @indent
                (field_expression) @indent
                (_ "(" ")" @end) @indent
                (_ "{" "}" @end) @indent
            "#,
    )
    .unwrap()
    .with_brackets_query(r#" ("{" @open "}" @close) "#)
    .unwrap()
}

fn empty(point: Point) -> Range<Point> {
    point..point
}
