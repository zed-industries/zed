use crate::*;
use gpui::ModelHandle;
use unindent::Unindent as _;

#[gpui::test]
async fn test_reparse(mut cx: gpui::TestAppContext) {
    let buffer = cx.add_model(|cx| {
        let text = "fn a() {}".into();
        Buffer::from_history(0, History::new(text), None, Some(rust_lang()), cx)
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
async fn test_enclosing_bracket_ranges(mut cx: gpui::TestAppContext) {
    let buffer = cx.add_model(|cx| {
        let text = "
            mod x {
                mod y {

                }
            }
        "
        .unindent()
        .into();
        Buffer::from_history(0, History::new(text), None, Some(rust_lang()), cx)
    });
    buffer
        .condition(&cx, |buffer, _| !buffer.is_parsing())
        .await;
    buffer.read_with(&cx, |buf, _| {
        assert_eq!(
            buf.enclosing_bracket_point_ranges(Point::new(1, 6)..Point::new(1, 6)),
            Some((
                Point::new(0, 6)..Point::new(0, 7),
                Point::new(4, 0)..Point::new(4, 1)
            ))
        );
        assert_eq!(
            buf.enclosing_bracket_point_ranges(Point::new(1, 10)..Point::new(1, 10)),
            Some((
                Point::new(1, 10)..Point::new(1, 11),
                Point::new(3, 4)..Point::new(3, 5)
            ))
        );
        assert_eq!(
            buf.enclosing_bracket_point_ranges(Point::new(3, 5)..Point::new(3, 5)),
            Some((
                Point::new(1, 10)..Point::new(1, 11),
                Point::new(3, 4)..Point::new(3, 5)
            ))
        );
    });
}

#[gpui::test]
async fn test_edit_with_autoindent(mut cx: gpui::TestAppContext) {
    let buffer = cx.add_model(|cx| {
        let text = "fn a() {}".into();
        Buffer::from_history(0, History::new(text), None, Some(rust_lang()), cx)
    });

    buffer.update(&mut cx, |buffer, cx| {
        buffer.edit_with_autoindent([8..8], "\n\n", cx);
        assert_eq!(buffer.text(), "fn a() {\n    \n}");

        buffer.edit_with_autoindent([Point::new(1, 4)..Point::new(1, 4)], "b()\n", cx);
        assert_eq!(buffer.text(), "fn a() {\n    b()\n    \n}");

        buffer.edit_with_autoindent([Point::new(2, 4)..Point::new(2, 4)], ".c", cx);
        assert_eq!(buffer.text(), "fn a() {\n    b()\n        .c\n}");
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
                (_ "{" "}" @end) @indent
            "#,
        )
        .unwrap()
        .with_brackets_query(r#" ("{" @open "}" @close) "#)
        .unwrap(),
    )
}
