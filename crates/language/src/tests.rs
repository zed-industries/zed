use super::*;
use clock::ReplicaId;
use collections::BTreeMap;
use gpui::{ModelHandle, MutableAppContext};
use rand::prelude::*;
use std::{
    cell::RefCell,
    env,
    iter::FromIterator,
    ops::Range,
    rc::Rc,
    time::{Duration, Instant},
};
use unindent::Unindent as _;
use util::test::Network;

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}

#[gpui::test]
fn test_select_language() {
    let registry = LanguageRegistry::new();
    registry.add(Arc::new(Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    )));
    registry.add(Arc::new(Language::new(
        LanguageConfig {
            name: "Make".into(),
            path_suffixes: vec!["Makefile".to_string(), "mk".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    )));

    // matching file extension
    assert_eq!(
        registry.select_language("zed/lib.rs").map(|l| l.name()),
        Some("Rust".into())
    );
    assert_eq!(
        registry.select_language("zed/lib.mk").map(|l| l.name()),
        Some("Make".into())
    );

    // matching filename
    assert_eq!(
        registry.select_language("zed/Makefile").map(|l| l.name()),
        Some("Make".into())
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
    let buffer =
        cx.add_model(|cx| Buffer::new(0, text, cx).with_language(Arc::new(rust_lang()), cx));

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
            "body: (block (expression_statement (identifier)))))"
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
            "body: (block (expression_statement (call_expression ",
            "function: (generic_function ",
            "function: (field_expression value: (identifier) field: (field_identifier)) ",
            "type_arguments: (type_arguments (type_identifier))) ",
            "arguments: (arguments (identifier)))))))",
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
            "body: (block (expression_statement (call_expression ",
            "function: (generic_function ",
            "function: (field_expression value: (identifier) field: (field_identifier)) ",
            "type_arguments: (type_arguments (type_identifier))) ",
            "arguments: (arguments (identifier)))))))",
        )
    );

    fn get_tree_sexp(buffer: &ModelHandle<Buffer>, cx: &gpui::TestAppContext) -> String {
        buffer.read_with(cx, |buffer, _| {
            buffer.syntax_tree().unwrap().root_node().to_sexp()
        })
    }
}

#[gpui::test]
async fn test_outline(mut cx: gpui::TestAppContext) {
    let language = Arc::new(
        rust_lang()
            .with_outline_query(
                r#"
                (struct_item
                    "struct" @context
                    name: (_) @name) @item
                (enum_item
                    "enum" @context
                    name: (_) @name) @item
                (enum_variant
                    name: (_) @name) @item
                (field_declaration
                    name: (_) @name) @item
                (impl_item
                    "impl" @context
                    trait: (_) @name
                    "for" @context
                    type: (_) @name) @item
                (function_item
                    "fn" @context
                    name: (_) @name) @item
                (mod_item
                    "mod" @context
                    name: (_) @name) @item
                "#,
            )
            .unwrap(),
    );

    let text = r#"
        struct Person {
            name: String,
            age: usize,
        }

        mod module {
            enum LoginState {
                LoggedOut,
                LoggingOn,
                LoggedIn {
                    person: Person,
                    time: Instant,
                }
            }
        }

        impl Eq for Person {}

        impl Drop for Person {
            fn drop(&mut self) {
                println!("bye");
            }
        }
    "#
    .unindent();

    let buffer = cx.add_model(|cx| Buffer::new(0, text, cx).with_language(language, cx));
    let outline = buffer
        .read_with(&cx, |buffer, _| buffer.snapshot().outline(None))
        .unwrap();

    assert_eq!(
        outline
            .items
            .iter()
            .map(|item| (item.text.as_str(), item.depth))
            .collect::<Vec<_>>(),
        &[
            ("struct Person", 0),
            ("name", 1),
            ("age", 1),
            ("mod module", 0),
            ("enum LoginState", 1),
            ("LoggedOut", 2),
            ("LoggingOn", 2),
            ("LoggedIn", 2),
            ("person", 3),
            ("time", 3),
            ("impl Eq for Person", 0),
            ("impl Drop for Person", 0),
            ("fn drop", 1),
        ]
    );

    // Without space, we only match on names
    assert_eq!(
        search(&outline, "oon", &cx).await,
        &[
            ("mod module", vec![]),                    // included as the parent of a match
            ("enum LoginState", vec![]),               // included as the parent of a match
            ("LoggingOn", vec![1, 7, 8]),              // matches
            ("impl Drop for Person", vec![7, 18, 19]), // matches in two disjoint names
        ]
    );

    assert_eq!(
        search(&outline, "dp p", &cx).await,
        &[
            ("impl Drop for Person", vec![5, 8, 9, 14]),
            ("fn drop", vec![]),
        ]
    );
    assert_eq!(
        search(&outline, "dpn", &cx).await,
        &[("impl Drop for Person", vec![5, 14, 19])]
    );
    assert_eq!(
        search(&outline, "impl ", &cx).await,
        &[
            ("impl Eq for Person", vec![0, 1, 2, 3, 4]),
            ("impl Drop for Person", vec![0, 1, 2, 3, 4]),
            ("fn drop", vec![]),
        ]
    );

    async fn search<'a>(
        outline: &'a Outline<Anchor>,
        query: &str,
        cx: &gpui::TestAppContext,
    ) -> Vec<(&'a str, Vec<usize>)> {
        let matches = cx
            .read(|cx| outline.search(query, cx.background().clone()))
            .await;
        matches
            .into_iter()
            .map(|mat| (outline.items[mat.candidate_id].text.as_str(), mat.positions))
            .collect::<Vec<_>>()
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
        Buffer::new(0, text, cx).with_language(Arc::new(rust_lang()), cx)
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
        let mut buffer = Buffer::new(0, text, cx).with_language(Arc::new(rust_lang()), cx);

        buffer.edit_with_autoindent([8..8], "\n\n", cx);
        assert_eq!(buffer.text(), "fn a() {\n    \n}");

        buffer.edit_with_autoindent([Point::new(1, 4)..Point::new(1, 4)], "b()\n", cx);
        assert_eq!(buffer.text(), "fn a() {\n    b()\n    \n}");

        buffer.edit_with_autoindent([Point::new(2, 4)..Point::new(2, 4)], ".c", cx);
        assert_eq!(buffer.text(), "fn a() {\n    b()\n        .c\n}");

        buffer
    });
}

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

        let mut buffer = Buffer::new(0, text, cx).with_language(Arc::new(rust_lang()), cx);

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

        let mut buffer = Buffer::new(0, text, cx).with_language(Arc::new(rust_lang()), cx);

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
    let (language_server, mut fake) = lsp::LanguageServer::fake(cx.background());
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
        Buffer::from_file(0, text, Box::new(FakeFile::new("/some/path")), cx)
            .with_language(Arc::new(rust_lang), cx)
            .with_language_server(language_server, cx)
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
                vec![
                    DiagnosticEntry {
                        range: PointUtf16::new(0, 9)..PointUtf16::new(0, 10),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::ERROR,
                            message: "undefined variable 'A'".to_string(),
                            is_disk_based: true,
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
                            is_disk_based: true,
                            group_id: 1,
                            is_primary: true,
                            ..Default::default()
                        },
                    },
                    DiagnosticEntry {
                        range: PointUtf16::new(2, 9)..PointUtf16::new(2, 12),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::ERROR,
                            is_disk_based: true,
                            message: "undefined variable 'CCC'".to_string(),
                            group_id: 2,
                            is_primary: true,
                            ..Default::default()
                        },
                    },
                ],
                Some(open_notification.text_document.version),
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
                        is_disk_based: true,
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
                        is_disk_based: true,
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
                vec![
                    DiagnosticEntry {
                        range: PointUtf16::new(0, 9)..PointUtf16::new(0, 10),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::ERROR,
                            message: "undefined variable 'A'".to_string(),
                            is_disk_based: true,
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
                Some(open_notification.text_document.version),
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
                        group_id: 1,
                        is_primary: true,
                        ..Default::default()
                    }
                },
                DiagnosticEntry {
                    range: Point::new(2, 9)..Point::new(2, 10),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::ERROR,
                        message: "undefined variable 'A'".to_string(),
                        is_disk_based: true,
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
                vec![
                    DiagnosticEntry {
                        range: PointUtf16::new(1, 9)..PointUtf16::new(1, 11),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::ERROR,
                            message: "undefined variable 'BB'".to_string(),
                            is_disk_based: true,
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
                            is_disk_based: true,
                            group_id: 0,
                            is_primary: true,
                            ..Default::default()
                        },
                    },
                ],
                Some(change_notification_2.text_document.version),
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
                        is_disk_based: true,
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
                        is_disk_based: true,
                        group_id: 1,
                        is_primary: true,
                        ..Default::default()
                    },
                }
            ]
        );
    });
}

#[gpui::test]
async fn test_edits_from_lsp_with_past_version(mut cx: gpui::TestAppContext) {
    let (language_server, mut fake) = lsp::LanguageServer::fake(cx.background());

    let text = "
        fn a() {
            f1();
        }
        fn b() {
            f2();
        }
        fn c() {
            f3();
        }
    "
    .unindent();

    let buffer = cx.add_model(|cx| {
        Buffer::from_file(0, text, Box::new(FakeFile::new("/some/path")), cx)
            .with_language(Arc::new(rust_lang()), cx)
            .with_language_server(language_server, cx)
    });

    let lsp_document_version = fake
        .receive_notification::<lsp::notification::DidOpenTextDocument>()
        .await
        .text_document
        .version;

    // Simulate editing the buffer after the language server computes some edits.
    buffer.update(&mut cx, |buffer, cx| {
        buffer.edit(
            [Point::new(0, 0)..Point::new(0, 0)],
            "// above first function\n",
            cx,
        );
        buffer.edit(
            [Point::new(2, 0)..Point::new(2, 0)],
            "    // inside first function\n",
            cx,
        );
        buffer.edit(
            [Point::new(6, 4)..Point::new(6, 4)],
            "// inside second function ",
            cx,
        );

        assert_eq!(
            buffer.text(),
            "
                // above first function
                fn a() {
                    // inside first function
                    f1();
                }
                fn b() {
                    // inside second function f2();
                }
                fn c() {
                    f3();
                }
            "
            .unindent()
        );
    });

    let edits = buffer
        .update(&mut cx, |buffer, cx| {
            buffer.edits_from_lsp(
                vec![
                    // replace body of first function
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(3, 0)),
                        new_text: "
                            fn a() {
                                f10();
                            }
                        "
                        .unindent(),
                    },
                    // edit inside second function
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(4, 6), lsp::Position::new(4, 6)),
                        new_text: "00".into(),
                    },
                    // edit inside third function via two distinct edits
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(7, 5), lsp::Position::new(7, 5)),
                        new_text: "4000".into(),
                    },
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(7, 5), lsp::Position::new(7, 6)),
                        new_text: "".into(),
                    },
                ],
                Some(lsp_document_version),
                cx,
            )
        })
        .await
        .unwrap();

    buffer.update(&mut cx, |buffer, cx| {
        for (range, new_text) in edits {
            buffer.edit([range], new_text, cx);
        }
        assert_eq!(
            buffer.text(),
            "
                // above first function
                fn a() {
                    // inside first function
                    f10();
                }
                fn b() {
                    // inside second function f200();
                }
                fn c() {
                    f4000();
                }
            "
            .unindent()
        );
    });
}

#[gpui::test]
async fn test_edits_from_lsp_with_edits_on_adjacent_lines(mut cx: gpui::TestAppContext) {
    let text = "
        use a::b;
        use a::c;

        fn f() {
            b();
            c();
        }
    "
    .unindent();

    let buffer = cx.add_model(|cx| Buffer::new(0, text, cx));

    // Simulate the language server sending us a small edit in the form of a very large diff.
    // Rust-analyzer does this when performing a merge-imports code action.
    let edits = buffer
        .update(&mut cx, |buffer, cx| {
            buffer.edits_from_lsp(
                [
                    // Replace the first use statement without editing the semicolon.
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(0, 4), lsp::Position::new(0, 8)),
                        new_text: "a::{b, c}".into(),
                    },
                    // Reinsert the remainder of the file between the semicolon and the final
                    // newline of the file.
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 9)),
                        new_text: "\n\n".into(),
                    },
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 9)),
                        new_text: "
                            fn f() {
                                b();
                                c();
                            }"
                        .unindent(),
                    },
                    // Delete everything after the first newline of the file.
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(7, 0)),
                        new_text: "".into(),
                    },
                ],
                None,
                cx,
            )
        })
        .await
        .unwrap();

    buffer.update(&mut cx, |buffer, cx| {
        let edits = edits
            .into_iter()
            .map(|(range, text)| {
                (
                    range.start.to_point(&buffer)..range.end.to_point(&buffer),
                    text,
                )
            })
            .collect::<Vec<_>>();

        assert_eq!(
            edits,
            [
                (Point::new(0, 4)..Point::new(0, 8), "a::{b, c}".into()),
                (Point::new(1, 0)..Point::new(2, 0), "".into())
            ]
        );

        for (range, new_text) in edits {
            buffer.edit([range], new_text, cx);
        }
        assert_eq!(
            buffer.text(),
            "
                use a::{b, c};

                fn f() {
                    b();
                    c();
                }
            "
            .unindent()
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
        buffer.set_language(Some(Arc::new(rust_lang())), cx);
        buffer
            .update_diagnostics(
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
                None,
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

#[gpui::test]
fn test_serialization(cx: &mut gpui::MutableAppContext) {
    let mut now = Instant::now();

    let buffer1 = cx.add_model(|cx| {
        let mut buffer = Buffer::new(0, "abc", cx);
        buffer.edit([3..3], "D", cx);

        now += Duration::from_secs(1);
        buffer.start_transaction_at(now);
        buffer.edit([4..4], "E", cx);
        buffer.end_transaction_at(now, cx);
        assert_eq!(buffer.text(), "abcDE");

        buffer.undo(cx);
        assert_eq!(buffer.text(), "abcD");

        buffer.edit([4..4], "F", cx);
        assert_eq!(buffer.text(), "abcDF");
        buffer
    });
    assert_eq!(buffer1.read(cx).text(), "abcDF");

    let message = buffer1.read(cx).to_proto();
    let buffer2 = cx.add_model(|cx| Buffer::from_proto(1, message, None, cx).unwrap());
    assert_eq!(buffer2.read(cx).text(), "abcDF");
}

#[gpui::test(iterations = 100)]
fn test_random_collaboration(cx: &mut MutableAppContext, mut rng: StdRng) {
    let min_peers = env::var("MIN_PEERS")
        .map(|i| i.parse().expect("invalid `MIN_PEERS` variable"))
        .unwrap_or(1);
    let max_peers = env::var("MAX_PEERS")
        .map(|i| i.parse().expect("invalid `MAX_PEERS` variable"))
        .unwrap_or(5);
    let operations = env::var("OPERATIONS")
        .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
        .unwrap_or(10);

    let base_text_len = rng.gen_range(0..10);
    let base_text = RandomCharIter::new(&mut rng)
        .take(base_text_len)
        .collect::<String>();
    let mut replica_ids = Vec::new();
    let mut buffers = Vec::new();
    let mut network = Network::new(rng.clone());

    for i in 0..rng.gen_range(min_peers..=max_peers) {
        let buffer = cx.add_model(|cx| {
            let mut buffer = Buffer::new(i as ReplicaId, base_text.as_str(), cx);
            buffer.set_group_interval(Duration::from_millis(rng.gen_range(0..=200)));
            buffer
        });
        buffers.push(buffer);
        replica_ids.push(i as ReplicaId);
        network.add_peer(i as ReplicaId);
        log::info!("Adding initial peer with replica id {}", i);
    }

    log::info!("initial text: {:?}", base_text);

    let mut now = Instant::now();
    let mut mutation_count = operations;
    let mut active_selections = BTreeMap::default();
    loop {
        let replica_index = rng.gen_range(0..replica_ids.len());
        let replica_id = replica_ids[replica_index];
        let buffer = &mut buffers[replica_index];
        let mut new_buffer = None;
        match rng.gen_range(0..100) {
            0..=29 if mutation_count != 0 => {
                buffer.update(cx, |buffer, cx| {
                    buffer.start_transaction_at(now);
                    buffer.randomly_edit(&mut rng, 5, cx);
                    buffer.end_transaction_at(now, cx);
                    log::info!("buffer {} text: {:?}", buffer.replica_id(), buffer.text());
                });
                mutation_count -= 1;
            }
            30..=39 if mutation_count != 0 => {
                buffer.update(cx, |buffer, cx| {
                    let mut selections = Vec::new();
                    for id in 0..rng.gen_range(1..=5) {
                        let range = buffer.random_byte_range(0, &mut rng);
                        selections.push(Selection {
                            id,
                            start: buffer.anchor_before(range.start),
                            end: buffer.anchor_before(range.end),
                            reversed: false,
                            goal: SelectionGoal::None,
                        });
                    }
                    let selections: Arc<[Selection<Anchor>]> = selections.into();
                    log::info!(
                        "peer {} setting active selections: {:?}",
                        replica_id,
                        selections
                    );
                    active_selections.insert(replica_id, selections.clone());
                    buffer.set_active_selections(selections, cx);
                });
                mutation_count -= 1;
            }
            40..=49 if replica_ids.len() < max_peers => {
                let old_buffer = buffer.read(cx).to_proto();
                let new_replica_id = replica_ids.len() as ReplicaId;
                log::info!(
                    "Adding new replica {} (replicating from {})",
                    new_replica_id,
                    replica_id
                );
                new_buffer = Some(cx.add_model(|cx| {
                    let mut new_buffer =
                        Buffer::from_proto(new_replica_id, old_buffer, None, cx).unwrap();
                    new_buffer.set_group_interval(Duration::from_millis(rng.gen_range(0..=200)));
                    new_buffer
                }));
                replica_ids.push(new_replica_id);
                network.replicate(replica_id, new_replica_id);
            }
            50..=69 if mutation_count != 0 => {
                buffer.update(cx, |buffer, cx| {
                    buffer.randomly_undo_redo(&mut rng, cx);
                    log::info!("buffer {} text: {:?}", buffer.replica_id(), buffer.text());
                });
                mutation_count -= 1;
            }
            70..=99 if network.has_unreceived(replica_id) => {
                let ops = network
                    .receive(replica_id)
                    .into_iter()
                    .map(|op| proto::deserialize_operation(op).unwrap());
                if ops.len() > 0 {
                    log::info!(
                        "peer {} applying {} ops from the network.",
                        replica_id,
                        ops.len()
                    );
                    buffer.update(cx, |buffer, cx| buffer.apply_ops(ops, cx).unwrap());
                }
            }
            _ => {}
        }

        buffer.update(cx, |buffer, _| {
            let ops = buffer
                .operations
                .drain(..)
                .map(|op| proto::serialize_operation(&op))
                .collect();
            network.broadcast(buffer.replica_id(), ops);
        });
        now += Duration::from_millis(rng.gen_range(0..=200));
        buffers.extend(new_buffer);

        for buffer in &buffers {
            buffer.read(cx).check_invariants();
        }

        if mutation_count == 0 && network.is_idle() {
            break;
        }
    }

    let first_buffer = buffers[0].read(cx);
    for buffer in &buffers[1..] {
        let buffer = buffer.read(cx);
        assert_eq!(
            buffer.text(),
            first_buffer.text(),
            "Replica {} text != Replica 0 text",
            buffer.replica_id()
        );
    }

    for buffer in &buffers {
        let buffer = buffer.read(cx).snapshot();
        let actual_remote_selections = buffer
            .remote_selections_in_range(Anchor::min()..Anchor::max())
            .map(|(replica_id, selections)| (replica_id, selections.collect::<Vec<_>>()))
            .collect::<Vec<_>>();
        let expected_remote_selections = active_selections
            .iter()
            .filter(|(replica_id, _)| **replica_id != buffer.replica_id())
            .map(|(replica_id, selections)| (*replica_id, selections.iter().collect::<Vec<_>>()))
            .collect::<Vec<_>>();
        assert_eq!(actual_remote_selections, expected_remote_selections);
    }
}

fn chunks_with_diagnostics<T: ToOffset + ToPoint>(
    buffer: &Buffer,
    range: Range<T>,
) -> Vec<(String, Option<DiagnosticSeverity>)> {
    let mut chunks: Vec<(String, Option<DiagnosticSeverity>)> = Vec::new();
    for chunk in buffer.snapshot().chunks(range, true) {
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
            name: "Rust".into(),
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
    .with_brackets_query(
        r#"
        ("{" @open "}" @close)
        "#,
    )
    .unwrap()
}

fn empty(point: Point) -> Range<Point> {
    point..point
}
