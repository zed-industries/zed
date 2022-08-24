use super::*;
use clock::ReplicaId;
use collections::BTreeMap;
use gpui::{ModelHandle, MutableAppContext};
use rand::prelude::*;
use settings::Settings;
use std::{
    cell::RefCell,
    env,
    ops::Range,
    rc::Rc,
    time::{Duration, Instant},
};
use text::network::Network;
use unindent::Unindent as _;
use util::post_inc;

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}

#[gpui::test]
fn test_line_endings(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    cx.add_model(|cx| {
        let mut buffer =
            Buffer::new(0, "one\r\ntwo\rthree", cx).with_language(Arc::new(rust_lang()), cx);
        assert_eq!(buffer.text(), "one\ntwo\nthree");
        assert_eq!(buffer.line_ending(), LineEnding::Windows);

        buffer.check_invariants();
        buffer.edit(
            [(buffer.len()..buffer.len(), "\r\nfour")],
            Some(AutoindentMode::EachLine),
            cx,
        );
        buffer.edit([(0..0, "zero\r\n")], None, cx);
        assert_eq!(buffer.text(), "zero\none\ntwo\nthree\nfour");
        assert_eq!(buffer.line_ending(), LineEnding::Windows);
        buffer.check_invariants();

        buffer
    });
}

#[gpui::test]
fn test_select_language() {
    let registry = LanguageRegistry::test();
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
    let buffer1_ops = Rc::new(RefCell::new(Vec::new()));
    buffer1.update(cx, {
        let buffer1_ops = buffer1_ops.clone();
        |buffer, cx| {
            let buffer_1_events = buffer_1_events.clone();
            cx.subscribe(&buffer1, move |_, _, event, _| match event.clone() {
                Event::Operation(op) => buffer1_ops.borrow_mut().push(op),
                event => buffer_1_events.borrow_mut().push(event),
            })
            .detach();
            let buffer_2_events = buffer_2_events.clone();
            cx.subscribe(&buffer2, move |_, _, event, _| {
                buffer_2_events.borrow_mut().push(event.clone())
            })
            .detach();

            // An edit emits an edited event, followed by a dirty changed event,
            // since the buffer was previously in a clean state.
            buffer.edit([(2..4, "XYZ")], None, cx);

            // An empty transaction does not emit any events.
            buffer.start_transaction();
            buffer.end_transaction(cx);

            // A transaction containing two edits emits one edited event.
            now += Duration::from_secs(1);
            buffer.start_transaction_at(now);
            buffer.edit([(5..5, "u")], None, cx);
            buffer.edit([(6..6, "w")], None, cx);
            buffer.end_transaction_at(now, cx);

            // Undoing a transaction emits one edited event.
            buffer.undo(cx);
        }
    });

    // Incorporating a set of remote ops emits a single edited event,
    // followed by a dirty changed event.
    buffer2.update(cx, |buffer, cx| {
        buffer
            .apply_ops(buffer1_ops.borrow_mut().drain(..), cx)
            .unwrap();
    });
    assert_eq!(
        mem::take(&mut *buffer_1_events.borrow_mut()),
        vec![
            Event::Edited,
            Event::DirtyChanged,
            Event::Edited,
            Event::Edited,
        ]
    );
    assert_eq!(
        mem::take(&mut *buffer_2_events.borrow_mut()),
        vec![Event::Edited, Event::DirtyChanged]
    );

    buffer1.update(cx, |buffer, cx| {
        // Undoing the first transaction emits edited event, followed by a
        // dirty changed event, since the buffer is again in a clean state.
        buffer.undo(cx);
    });
    // Incorporating the remote ops again emits a single edited event,
    // followed by a dirty changed event.
    buffer2.update(cx, |buffer, cx| {
        buffer
            .apply_ops(buffer1_ops.borrow_mut().drain(..), cx)
            .unwrap();
    });
    assert_eq!(
        mem::take(&mut *buffer_1_events.borrow_mut()),
        vec![Event::Edited, Event::DirtyChanged,]
    );
    assert_eq!(
        mem::take(&mut *buffer_2_events.borrow_mut()),
        vec![Event::Edited, Event::DirtyChanged]
    );
}

#[gpui::test]
async fn test_apply_diff(cx: &mut gpui::TestAppContext) {
    let text = "a\nbb\nccc\ndddd\neeeee\nffffff\n";
    let buffer = cx.add_model(|cx| Buffer::new(0, text, cx));
    let anchor = buffer.read_with(cx, |buffer, _| buffer.anchor_before(Point::new(3, 3)));

    let text = "a\nccc\ndddd\nffffff\n";
    let diff = buffer.read_with(cx, |b, cx| b.diff(text.into(), cx)).await;
    buffer.update(cx, |buffer, cx| {
        buffer.apply_diff(diff, cx).unwrap();
        assert_eq!(buffer.text(), text);
        assert_eq!(anchor.to_point(buffer), Point::new(2, 3));
    });

    let text = "a\n1\n\nccc\ndd2dd\nffffff\n";
    let diff = buffer.read_with(cx, |b, cx| b.diff(text.into(), cx)).await;
    buffer.update(cx, |buffer, cx| {
        buffer.apply_diff(diff, cx).unwrap();
        assert_eq!(buffer.text(), text);
        assert_eq!(anchor.to_point(buffer), Point::new(4, 4));
    });
}

#[gpui::test]
async fn test_reparse(cx: &mut gpui::TestAppContext) {
    let text = "fn a() {}";
    let buffer =
        cx.add_model(|cx| Buffer::new(0, text, cx).with_language(Arc::new(rust_lang()), cx));

    // Wait for the initial text to parse
    buffer.condition(cx, |buffer, _| !buffer.is_parsing()).await;
    assert_eq!(
        get_tree_sexp(&buffer, cx),
        concat!(
            "(source_file (function_item name: (identifier) ",
            "parameters: (parameters) ",
            "body: (block)))"
        )
    );

    buffer.update(cx, |buffer, _| {
        buffer.set_sync_parse_timeout(Duration::ZERO)
    });

    // Perform some edits (add parameter and variable reference)
    // Parsing doesn't begin until the transaction is complete
    buffer.update(cx, |buf, cx| {
        buf.start_transaction();

        let offset = buf.text().find(')').unwrap();
        buf.edit([(offset..offset, "b: C")], None, cx);
        assert!(!buf.is_parsing());

        let offset = buf.text().find('}').unwrap();
        buf.edit([(offset..offset, " d; ")], None, cx);
        assert!(!buf.is_parsing());

        buf.end_transaction(cx);
        assert_eq!(buf.text(), "fn a(b: C) { d; }");
        assert!(buf.is_parsing());
    });
    buffer.condition(cx, |buffer, _| !buffer.is_parsing()).await;
    assert_eq!(
        get_tree_sexp(&buffer, cx),
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
    buffer.update(cx, |buf, cx| {
        let offset = buf.text().find(';').unwrap();
        buf.edit([(offset..offset, ".e")], None, cx);
        assert_eq!(buf.text(), "fn a(b: C) { d.e; }");
        assert!(buf.is_parsing());
    });
    buffer.update(cx, |buf, cx| {
        let offset = buf.text().find(';').unwrap();
        buf.edit([(offset..offset, "(f)")], None, cx);
        assert_eq!(buf.text(), "fn a(b: C) { d.e(f); }");
        assert!(buf.is_parsing());
    });
    buffer.update(cx, |buf, cx| {
        let offset = buf.text().find("(f)").unwrap();
        buf.edit([(offset..offset, "::<G>")], None, cx);
        assert_eq!(buf.text(), "fn a(b: C) { d.e::<G>(f); }");
        assert!(buf.is_parsing());
    });
    buffer.condition(cx, |buffer, _| !buffer.is_parsing()).await;
    assert_eq!(
        get_tree_sexp(&buffer, cx),
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

    buffer.update(cx, |buf, cx| {
        buf.undo(cx);
        assert_eq!(buf.text(), "fn a() {}");
        assert!(buf.is_parsing());
    });
    buffer.condition(cx, |buffer, _| !buffer.is_parsing()).await;
    assert_eq!(
        get_tree_sexp(&buffer, cx),
        concat!(
            "(source_file (function_item name: (identifier) ",
            "parameters: (parameters) ",
            "body: (block)))"
        )
    );

    buffer.update(cx, |buf, cx| {
        buf.redo(cx);
        assert_eq!(buf.text(), "fn a(b: C) { d.e::<G>(f); }");
        assert!(buf.is_parsing());
    });
    buffer.condition(cx, |buffer, _| !buffer.is_parsing()).await;
    assert_eq!(
        get_tree_sexp(&buffer, cx),
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
}

#[gpui::test]
async fn test_resetting_language(cx: &mut gpui::TestAppContext) {
    let buffer = cx.add_model(|cx| {
        let mut buffer = Buffer::new(0, "{}", cx).with_language(Arc::new(rust_lang()), cx);
        buffer.set_sync_parse_timeout(Duration::ZERO);
        buffer
    });

    // Wait for the initial text to parse
    buffer.condition(cx, |buffer, _| !buffer.is_parsing()).await;
    assert_eq!(
        get_tree_sexp(&buffer, cx),
        "(source_file (expression_statement (block)))"
    );

    buffer.update(cx, |buffer, cx| {
        buffer.set_language(Some(Arc::new(json_lang())), cx)
    });
    buffer.condition(cx, |buffer, _| !buffer.is_parsing()).await;
    assert_eq!(get_tree_sexp(&buffer, cx), "(document (object))");
}

#[gpui::test]
async fn test_outline(cx: &mut gpui::TestAppContext) {
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

    let buffer =
        cx.add_model(|cx| Buffer::new(0, text, cx).with_language(Arc::new(rust_lang()), cx));
    let outline = buffer
        .read_with(cx, |buffer, _| buffer.snapshot().outline(None))
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
        search(&outline, "oon", cx).await,
        &[
            ("mod module", vec![]),                    // included as the parent of a match
            ("enum LoginState", vec![]),               // included as the parent of a match
            ("LoggingOn", vec![1, 7, 8]),              // matches
            ("impl Drop for Person", vec![7, 18, 19]), // matches in two disjoint names
        ]
    );

    assert_eq!(
        search(&outline, "dp p", cx).await,
        &[
            ("impl Drop for Person", vec![5, 8, 9, 14]),
            ("fn drop", vec![]),
        ]
    );
    assert_eq!(
        search(&outline, "dpn", cx).await,
        &[("impl Drop for Person", vec![5, 14, 19])]
    );
    assert_eq!(
        search(&outline, "impl ", cx).await,
        &[
            ("impl Eq for Person", vec![0, 1, 2, 3, 4]),
            ("impl Drop for Person", vec![0, 1, 2, 3, 4]),
            ("fn drop", vec![]),
        ]
    );

    async fn search<'a>(
        outline: &'a Outline<Anchor>,
        query: &'a str,
        cx: &'a gpui::TestAppContext,
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
async fn test_symbols_containing(cx: &mut gpui::TestAppContext) {
    let text = r#"
        impl Person {
            fn one() {
                1
            }

            fn two() {
                2
            }fn three() {
                3
            }
        }
    "#
    .unindent();

    let buffer =
        cx.add_model(|cx| Buffer::new(0, text, cx).with_language(Arc::new(rust_lang()), cx));
    let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());

    // point is at the start of an item
    assert_eq!(
        symbols_containing(Point::new(1, 4), &snapshot),
        vec![
            (
                "impl Person".to_string(),
                Point::new(0, 0)..Point::new(10, 1)
            ),
            ("fn one".to_string(), Point::new(1, 4)..Point::new(3, 5))
        ]
    );

    // point is in the middle of an item
    assert_eq!(
        symbols_containing(Point::new(2, 8), &snapshot),
        vec![
            (
                "impl Person".to_string(),
                Point::new(0, 0)..Point::new(10, 1)
            ),
            ("fn one".to_string(), Point::new(1, 4)..Point::new(3, 5))
        ]
    );

    // point is at the end of an item
    assert_eq!(
        symbols_containing(Point::new(3, 5), &snapshot),
        vec![
            (
                "impl Person".to_string(),
                Point::new(0, 0)..Point::new(10, 1)
            ),
            ("fn one".to_string(), Point::new(1, 4)..Point::new(3, 5))
        ]
    );

    // point is in between two adjacent items
    assert_eq!(
        symbols_containing(Point::new(7, 5), &snapshot),
        vec![
            (
                "impl Person".to_string(),
                Point::new(0, 0)..Point::new(10, 1)
            ),
            ("fn two".to_string(), Point::new(5, 4)..Point::new(7, 5))
        ]
    );

    fn symbols_containing(
        position: Point,
        snapshot: &BufferSnapshot,
    ) -> Vec<(String, Range<Point>)> {
        snapshot
            .symbols_containing(position, None)
            .unwrap()
            .into_iter()
            .map(|item| {
                (
                    item.text,
                    item.range.start.to_point(snapshot)..item.range.end.to_point(snapshot),
                )
            })
            .collect()
    }
}

#[gpui::test]
fn test_enclosing_bracket_ranges(cx: &mut MutableAppContext) {
    cx.set_global(Settings::test(cx));
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
fn test_range_for_syntax_ancestor(cx: &mut MutableAppContext) {
    cx.add_model(|cx| {
        let text = "fn a() { b(|c| {}) }";
        let buffer = Buffer::new(0, text, cx).with_language(Arc::new(rust_lang()), cx);
        let snapshot = buffer.snapshot();

        assert_eq!(
            snapshot.range_for_syntax_ancestor(empty_range_at(text, "|")),
            Some(range_of(text, "|"))
        );
        assert_eq!(
            snapshot.range_for_syntax_ancestor(range_of(text, "|")),
            Some(range_of(text, "|c|"))
        );
        assert_eq!(
            snapshot.range_for_syntax_ancestor(range_of(text, "|c|")),
            Some(range_of(text, "|c| {}"))
        );
        assert_eq!(
            snapshot.range_for_syntax_ancestor(range_of(text, "|c| {}")),
            Some(range_of(text, "(|c| {})"))
        );

        buffer
    });

    fn empty_range_at(text: &str, part: &str) -> Range<usize> {
        let start = text.find(part).unwrap();
        start..start
    }

    fn range_of(text: &str, part: &str) -> Range<usize> {
        let start = text.find(part).unwrap();
        start..start + part.len()
    }
}

#[gpui::test]
fn test_autoindent_with_soft_tabs(cx: &mut MutableAppContext) {
    let settings = Settings::test(cx);
    cx.set_global(settings);

    cx.add_model(|cx| {
        let text = "fn a() {}";
        let mut buffer = Buffer::new(0, text, cx).with_language(Arc::new(rust_lang()), cx);

        buffer.edit([(8..8, "\n\n")], Some(AutoindentMode::EachLine), cx);
        assert_eq!(buffer.text(), "fn a() {\n    \n}");

        buffer.edit(
            [(Point::new(1, 4)..Point::new(1, 4), "b()\n")],
            Some(AutoindentMode::EachLine),
            cx,
        );
        assert_eq!(buffer.text(), "fn a() {\n    b()\n    \n}");

        // Create a field expression on a new line, causing that line
        // to be indented.
        buffer.edit(
            [(Point::new(2, 4)..Point::new(2, 4), ".c")],
            Some(AutoindentMode::EachLine),
            cx,
        );
        assert_eq!(buffer.text(), "fn a() {\n    b()\n        .c\n}");

        // Remove the dot so that the line is no longer a field expression,
        // causing the line to be outdented.
        buffer.edit(
            [(Point::new(2, 8)..Point::new(2, 9), "")],
            Some(AutoindentMode::EachLine),
            cx,
        );
        assert_eq!(buffer.text(), "fn a() {\n    b()\n    c\n}");

        buffer
    });
}

#[gpui::test]
fn test_autoindent_with_hard_tabs(cx: &mut MutableAppContext) {
    let mut settings = Settings::test(cx);
    settings.editor_overrides.hard_tabs = Some(true);
    cx.set_global(settings);

    cx.add_model(|cx| {
        let text = "fn a() {}";
        let mut buffer = Buffer::new(0, text, cx).with_language(Arc::new(rust_lang()), cx);

        buffer.edit([(8..8, "\n\n")], Some(AutoindentMode::EachLine), cx);
        assert_eq!(buffer.text(), "fn a() {\n\t\n}");

        buffer.edit(
            [(Point::new(1, 1)..Point::new(1, 1), "b()\n")],
            Some(AutoindentMode::EachLine),
            cx,
        );
        assert_eq!(buffer.text(), "fn a() {\n\tb()\n\t\n}");

        // Create a field expression on a new line, causing that line
        // to be indented.
        buffer.edit(
            [(Point::new(2, 1)..Point::new(2, 1), ".c")],
            Some(AutoindentMode::EachLine),
            cx,
        );
        assert_eq!(buffer.text(), "fn a() {\n\tb()\n\t\t.c\n}");

        // Remove the dot so that the line is no longer a field expression,
        // causing the line to be outdented.
        buffer.edit(
            [(Point::new(2, 2)..Point::new(2, 3), "")],
            Some(AutoindentMode::EachLine),
            cx,
        );
        assert_eq!(buffer.text(), "fn a() {\n\tb()\n\tc\n}");

        buffer
    });
}

#[gpui::test]
fn test_autoindent_does_not_adjust_lines_with_unchanged_suggestion(cx: &mut MutableAppContext) {
    let settings = Settings::test(cx);
    cx.set_global(settings);

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
        buffer.edit(
            [
                (empty(Point::new(1, 1)), "()"),
                (empty(Point::new(2, 1)), "()"),
            ],
            Some(AutoindentMode::EachLine),
            cx,
        );
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
        buffer.edit(
            [
                (empty(Point::new(1, 1)), "\n.f\n.g"),
                (empty(Point::new(2, 1)), "\n.f\n.g"),
            ],
            Some(AutoindentMode::EachLine),
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

    cx.add_model(|cx| {
        let text = "
            fn a() {
                {
                    b()?
                }
                Ok(())
            }
        "
        .unindent();
        let mut buffer = Buffer::new(0, text, cx).with_language(Arc::new(rust_lang()), cx);

        // Delete a closing curly brace changes the suggested indent for the line.
        buffer.edit(
            [(Point::new(3, 4)..Point::new(3, 5), "")],
            Some(AutoindentMode::EachLine),
            cx,
        );
        assert_eq!(
            buffer.text(),
            "
            fn a() {
                {
                    b()?
                        |
                Ok(())
            }
            "
            .replace('|', "") // included in the string to preserve trailing whites
            .unindent()
        );

        // Manually editing the leading whitespace
        buffer.edit(
            [(Point::new(3, 0)..Point::new(3, 12), "")],
            Some(AutoindentMode::EachLine),
            cx,
        );
        assert_eq!(
            buffer.text(),
            "
            fn a() {
                {
                    b()?

                Ok(())
            }
            "
            .unindent()
        );
        buffer
    });
}

#[gpui::test]
fn test_autoindent_adjusts_lines_when_only_text_changes(cx: &mut MutableAppContext) {
    cx.set_global(Settings::test(cx));
    cx.add_model(|cx| {
        let text = "
            fn a() {}
        "
        .unindent();

        let mut buffer = Buffer::new(0, text, cx).with_language(Arc::new(rust_lang()), cx);

        buffer.edit([(5..5, "\nb")], Some(AutoindentMode::EachLine), cx);
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
        buffer.edit(
            [(Point::new(1, 4)..Point::new(1, 5), "")],
            Some(AutoindentMode::EachLine),
            cx,
        );
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
fn test_autoindent_with_edit_at_end_of_buffer(cx: &mut MutableAppContext) {
    cx.set_global(Settings::test(cx));
    cx.add_model(|cx| {
        let text = "a\nb";
        let mut buffer = Buffer::new(0, text, cx).with_language(Arc::new(rust_lang()), cx);
        buffer.edit(
            [(0..1, "\n"), (2..3, "\n")],
            Some(AutoindentMode::EachLine),
            cx,
        );
        assert_eq!(buffer.text(), "\n\n\n");
        buffer
    });
}

#[gpui::test]
fn test_autoindent_multi_line_insertion(cx: &mut MutableAppContext) {
    cx.set_global(Settings::test(cx));
    cx.add_model(|cx| {
        let text = "
            const a: usize = 1;
            fn b() {
                if c {
                    let d = 2;
                }
            }
        "
        .unindent();

        let mut buffer = Buffer::new(0, text, cx).with_language(Arc::new(rust_lang()), cx);
        buffer.edit(
            [(Point::new(3, 0)..Point::new(3, 0), "e(\n    f()\n);\n")],
            Some(AutoindentMode::EachLine),
            cx,
        );
        assert_eq!(
            buffer.text(),
            "
                const a: usize = 1;
                fn b() {
                    if c {
                        e(
                            f()
                        );
                        let d = 2;
                    }
                }
            "
            .unindent()
        );

        buffer
    });
}

#[gpui::test]
fn test_autoindent_block_mode(cx: &mut MutableAppContext) {
    cx.set_global(Settings::test(cx));
    cx.add_model(|cx| {
        let text = r#"
            fn a() {
                b();
            }
        "#
        .unindent();
        let mut buffer = Buffer::new(0, text, cx).with_language(Arc::new(rust_lang()), cx);

        let inserted_text = r#"
            "
              c
                d
                  e
            "
        "#
        .unindent();

        // Insert the block at column zero. The entire block is indented
        // so that the first line matches the previous line's indentation.
        buffer.edit(
            [(Point::new(2, 0)..Point::new(2, 0), inserted_text.clone())],
            Some(AutoindentMode::Block {
                original_indent_columns: vec![0],
            }),
            cx,
        );
        assert_eq!(
            buffer.text(),
            r#"
            fn a() {
                b();
                "
                  c
                    d
                      e
                "
            }
            "#
            .unindent()
        );

        // Insert the block at a deeper indent level. The entire block is outdented.
        buffer.undo(cx);
        buffer.edit([(Point::new(2, 0)..Point::new(2, 0), "        ")], None, cx);
        buffer.edit(
            [(Point::new(2, 8)..Point::new(2, 8), inserted_text)],
            Some(AutoindentMode::Block {
                original_indent_columns: vec![0],
            }),
            cx,
        );
        assert_eq!(
            buffer.text(),
            r#"
            fn a() {
                b();
                "
                  c
                    d
                      e
                "
            }
            "#
            .unindent()
        );

        buffer
    });
}

#[gpui::test]
fn test_autoindent_language_without_indents_query(cx: &mut MutableAppContext) {
    cx.set_global(Settings::test(cx));
    cx.add_model(|cx| {
        let text = "
            * one
                - a
                - b
            * two
        "
        .unindent();

        let mut buffer = Buffer::new(0, text, cx).with_language(
            Arc::new(Language::new(
                LanguageConfig {
                    name: "Markdown".into(),
                    ..Default::default()
                },
                Some(tree_sitter_json::language()),
            )),
            cx,
        );
        buffer.edit(
            [(Point::new(3, 0)..Point::new(3, 0), "\n")],
            Some(AutoindentMode::EachLine),
            cx,
        );
        assert_eq!(
            buffer.text(),
            "
            * one
                - a
                - b

            * two
            "
            .unindent()
        );
        buffer
    });
}

#[gpui::test]
fn test_serialization(cx: &mut gpui::MutableAppContext) {
    let mut now = Instant::now();

    let buffer1 = cx.add_model(|cx| {
        let mut buffer = Buffer::new(0, "abc", cx);
        buffer.edit([(3..3, "D")], None, cx);

        now += Duration::from_secs(1);
        buffer.start_transaction_at(now);
        buffer.edit([(4..4, "E")], None, cx);
        buffer.end_transaction_at(now, cx);
        assert_eq!(buffer.text(), "abcDE");

        buffer.undo(cx);
        assert_eq!(buffer.text(), "abcD");

        buffer.edit([(4..4, "F")], None, cx);
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
    let network = Rc::new(RefCell::new(Network::new(rng.clone())));
    let base_buffer = cx.add_model(|cx| Buffer::new(0, base_text.as_str(), cx));

    for i in 0..rng.gen_range(min_peers..=max_peers) {
        let buffer = cx.add_model(|cx| {
            let mut buffer =
                Buffer::from_proto(i as ReplicaId, base_buffer.read(cx).to_proto(), None, cx)
                    .unwrap();
            buffer.set_group_interval(Duration::from_millis(rng.gen_range(0..=200)));
            let network = network.clone();
            cx.subscribe(&cx.handle(), move |buffer, _, event, _| {
                if let Event::Operation(op) = event {
                    network
                        .borrow_mut()
                        .broadcast(buffer.replica_id(), vec![proto::serialize_operation(op)]);
                }
            })
            .detach();
            buffer
        });
        buffers.push(buffer);
        replica_ids.push(i as ReplicaId);
        network.borrow_mut().add_peer(i as ReplicaId);
        log::info!("Adding initial peer with replica id {}", i);
    }

    log::info!("initial text: {:?}", base_text);

    let mut now = Instant::now();
    let mut mutation_count = operations;
    let mut next_diagnostic_id = 0;
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
                    buffer.set_active_selections(selections, false, cx);
                });
                mutation_count -= 1;
            }
            40..=49 if mutation_count != 0 && replica_id == 0 => {
                let entry_count = rng.gen_range(1..=5);
                buffer.update(cx, |buffer, cx| {
                    let diagnostics = DiagnosticSet::new(
                        (0..entry_count).map(|_| {
                            let range = buffer.random_byte_range(0, &mut rng);
                            let range = range.to_point_utf16(buffer);
                            DiagnosticEntry {
                                range,
                                diagnostic: Diagnostic {
                                    message: post_inc(&mut next_diagnostic_id).to_string(),
                                    ..Default::default()
                                },
                            }
                        }),
                        buffer,
                    );
                    log::info!("peer {} setting diagnostics: {:?}", replica_id, diagnostics);
                    buffer.update_diagnostics(diagnostics, cx);
                });
                mutation_count -= 1;
            }
            50..=59 if replica_ids.len() < max_peers => {
                let old_buffer = buffer.read(cx).to_proto();
                let new_replica_id = (0..=replica_ids.len() as ReplicaId)
                    .filter(|replica_id| *replica_id != buffer.read(cx).replica_id())
                    .choose(&mut rng)
                    .unwrap();
                log::info!(
                    "Adding new replica {} (replicating from {})",
                    new_replica_id,
                    replica_id
                );
                new_buffer = Some(cx.add_model(|cx| {
                    let mut new_buffer =
                        Buffer::from_proto(new_replica_id, old_buffer, None, cx).unwrap();
                    log::info!(
                        "New replica {} text: {:?}",
                        new_buffer.replica_id(),
                        new_buffer.text()
                    );
                    new_buffer.set_group_interval(Duration::from_millis(rng.gen_range(0..=200)));
                    let network = network.clone();
                    cx.subscribe(&cx.handle(), move |buffer, _, event, _| {
                        if let Event::Operation(op) = event {
                            network.borrow_mut().broadcast(
                                buffer.replica_id(),
                                vec![proto::serialize_operation(op)],
                            );
                        }
                    })
                    .detach();
                    new_buffer
                }));
                network.borrow_mut().replicate(replica_id, new_replica_id);

                if new_replica_id as usize == replica_ids.len() {
                    replica_ids.push(new_replica_id);
                } else {
                    let new_buffer = new_buffer.take().unwrap();
                    while network.borrow().has_unreceived(new_replica_id) {
                        let ops = network
                            .borrow_mut()
                            .receive(new_replica_id)
                            .into_iter()
                            .map(|op| proto::deserialize_operation(op).unwrap());
                        if ops.len() > 0 {
                            log::info!(
                                "peer {} (version: {:?}) applying {} ops from the network. {:?}",
                                new_replica_id,
                                buffer.read(cx).version(),
                                ops.len(),
                                ops
                            );
                            new_buffer.update(cx, |new_buffer, cx| {
                                new_buffer.apply_ops(ops, cx).unwrap();
                            });
                        }
                    }
                    buffers[new_replica_id as usize] = new_buffer;
                }
            }
            60..=69 if mutation_count != 0 => {
                buffer.update(cx, |buffer, cx| {
                    buffer.randomly_undo_redo(&mut rng, cx);
                    log::info!("buffer {} text: {:?}", buffer.replica_id(), buffer.text());
                });
                mutation_count -= 1;
            }
            _ if network.borrow().has_unreceived(replica_id) => {
                let ops = network
                    .borrow_mut()
                    .receive(replica_id)
                    .into_iter()
                    .map(|op| proto::deserialize_operation(op).unwrap());
                if ops.len() > 0 {
                    log::info!(
                        "peer {} (version: {:?}) applying {} ops from the network. {:?}",
                        replica_id,
                        buffer.read(cx).version(),
                        ops.len(),
                        ops
                    );
                    buffer.update(cx, |buffer, cx| buffer.apply_ops(ops, cx).unwrap());
                }
            }
            _ => {}
        }

        now += Duration::from_millis(rng.gen_range(0..=200));
        buffers.extend(new_buffer);

        for buffer in &buffers {
            buffer.read(cx).check_invariants();
        }

        if mutation_count == 0 && network.borrow().is_idle() {
            break;
        }
    }

    let first_buffer = buffers[0].read(cx).snapshot();
    for buffer in &buffers[1..] {
        let buffer = buffer.read(cx).snapshot();
        assert_eq!(
            buffer.version(),
            first_buffer.version(),
            "Replica {} version != Replica 0 version",
            buffer.replica_id()
        );
        assert_eq!(
            buffer.text(),
            first_buffer.text(),
            "Replica {} text != Replica 0 text",
            buffer.replica_id()
        );
        assert_eq!(
            buffer
                .diagnostics_in_range::<_, usize>(0..buffer.len(), false)
                .collect::<Vec<_>>(),
            first_buffer
                .diagnostics_in_range::<_, usize>(0..first_buffer.len(), false)
                .collect::<Vec<_>>(),
            "Replica {} diagnostics != Replica 0 diagnostics",
            buffer.replica_id()
        );
    }

    for buffer in &buffers {
        let buffer = buffer.read(cx).snapshot();
        let actual_remote_selections = buffer
            .remote_selections_in_range(Anchor::MIN..Anchor::MAX)
            .map(|(replica_id, _, selections)| (replica_id, selections.collect::<Vec<_>>()))
            .collect::<Vec<_>>();
        let expected_remote_selections = active_selections
            .iter()
            .filter(|(replica_id, _)| **replica_id != buffer.replica_id())
            .map(|(replica_id, selections)| (*replica_id, selections.iter().collect::<Vec<_>>()))
            .collect::<Vec<_>>();
        assert_eq!(
            actual_remote_selections,
            expected_remote_selections,
            "Replica {} remote selections != expected selections",
            buffer.replica_id()
        );
    }
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
            trait: (_)? @name
            "for"? @context
            type: (_) @name) @item
        (function_item
            "fn" @context
            name: (_) @name) @item
        (mod_item
            "mod" @context
            name: (_) @name) @item
        "#,
    )
    .unwrap()
}

fn json_lang() -> Language {
    Language::new(
        LanguageConfig {
            name: "Json".into(),
            path_suffixes: vec!["js".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_json::language()),
    )
}

fn get_tree_sexp(buffer: &ModelHandle<Buffer>, cx: &gpui::TestAppContext) -> String {
    buffer.read_with(cx, |buffer, _| {
        let snapshot = buffer.snapshot();
        let layers = snapshot.syntax.layers(buffer.as_text_snapshot());
        layers[0].2.to_sexp()
    })
}

fn empty(point: Point) -> Range<Point> {
    point..point
}
