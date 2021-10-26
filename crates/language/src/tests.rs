use super::*;
use gpui::{ModelHandle, MutableAppContext};
use std::rc::Rc;
use unindent::Unindent as _;

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
        buffer.start_transaction(None).unwrap();
        buffer.end_transaction(None, cx).unwrap();

        // A transaction containing two edits emits one edited event.
        now += Duration::from_secs(1);
        buffer.start_transaction_at(None, now).unwrap();
        buffer.edit(Some(5..5), "u", cx);
        buffer.edit(Some(6..6), "w", cx);
        buffer.end_transaction_at(None, now, cx).unwrap();

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
    let buffer = cx.add_model(|cx| {
        let text = "fn a() {}".into();
        Buffer::from_history(0, History::new(text), None, Some(rust_lang()), None, cx)
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
        buf.start_transaction(None).unwrap();

        let offset = buf.text().find(")").unwrap();
        buf.edit(vec![offset..offset], "b: C", cx);
        assert!(!buf.is_parsing());

        let offset = buf.text().find("}").unwrap();
        buf.edit(vec![offset..offset], " d; ", cx);
        assert!(!buf.is_parsing());

        buf.end_transaction(None, cx).unwrap();
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
        .unindent()
        .into();
        Buffer::from_history(0, History::new(text), None, Some(rust_lang()), None, cx)
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
        let text = "fn a() {}".into();
        let mut buffer =
            Buffer::from_history(0, History::new(text), None, Some(rust_lang()), None, cx);

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
fn test_autoindent_moves_selections(cx: &mut MutableAppContext) {
    cx.add_model(|cx| {
        let text = History::new("fn a() {}".into());
        let mut buffer = Buffer::from_history(0, text, None, Some(rust_lang()), None, cx);

        let selection_set_id = buffer.add_selection_set(Vec::new(), cx);
        buffer.start_transaction(Some(selection_set_id)).unwrap();
        buffer.edit_with_autoindent([5..5, 9..9], "\n\n", cx);
        buffer
            .update_selection_set(
                selection_set_id,
                vec![
                    Selection {
                        id: 0,
                        start: buffer.anchor_before(Point::new(1, 0)),
                        end: buffer.anchor_before(Point::new(1, 0)),
                        reversed: false,
                        goal: SelectionGoal::None,
                    },
                    Selection {
                        id: 1,
                        start: buffer.anchor_before(Point::new(4, 0)),
                        end: buffer.anchor_before(Point::new(4, 0)),
                        reversed: false,
                        goal: SelectionGoal::None,
                    },
                ],
                cx,
            )
            .unwrap();
        assert_eq!(buffer.text(), "fn a(\n\n) {}\n\n");

        // Ending the transaction runs the auto-indent. The selection
        // at the start of the auto-indented row is pushed to the right.
        buffer.end_transaction(Some(selection_set_id), cx).unwrap();
        assert_eq!(buffer.text(), "fn a(\n    \n) {}\n\n");
        let selection_ranges = buffer
            .selection_set(selection_set_id)
            .unwrap()
            .selections
            .iter()
            .map(|selection| selection.point_range(&buffer))
            .collect::<Vec<_>>();

        assert_eq!(selection_ranges[0], empty(Point::new(1, 4)));
        assert_eq!(selection_ranges[1], empty(Point::new(4, 0)));

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
        .unindent()
        .into();
        let mut buffer =
            Buffer::from_history(0, History::new(text), None, Some(rust_lang()), None, cx);

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
        let text = History::new(
            "
                fn a() {}
            "
            .unindent()
            .into(),
        );
        let mut buffer = Buffer::from_history(0, text, None, Some(rust_lang()), None, cx);

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

#[test]
fn test_contiguous_ranges() {
    assert_eq!(
        contiguous_ranges([1, 2, 3, 5, 6, 9, 10, 11, 12], 100).collect::<Vec<_>>(),
        &[1..4, 5..7, 9..13]
    );

    // Respects the `max_len` parameter
    assert_eq!(
        contiguous_ranges([2, 3, 4, 5, 6, 7, 8, 9, 23, 24, 25, 26, 30, 31], 3).collect::<Vec<_>>(),
        &[2..5, 5..8, 8..10, 23..26, 26..27, 30..32],
    );
}

impl Buffer {
    pub fn enclosing_bracket_point_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> Option<(Range<Point>, Range<Point>)> {
        self.enclosing_bracket_ranges(range).map(|(start, end)| {
            let point_start = start.start.to_point(self)..start.end.to_point(self);
            let point_end = end.start.to_point(self)..end.end.to_point(self);
            (point_start, point_end)
        })
    }
}

fn rust_lang() -> Arc<Language> {
    Arc::new(
        Language::new(
            LanguageConfig {
                name: "Rust".to_string(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            tree_sitter_rust::language(),
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
        .unwrap(),
    )
}

fn empty(point: Point) -> Range<Point> {
    point..point
}
