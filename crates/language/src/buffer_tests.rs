use super::*;
use crate::Buffer;
use clock::ReplicaId;
use collections::BTreeMap;
use futures::FutureExt as _;
use gpui::{App, AppContext as _, BorrowAppContext, Entity};
use gpui::{HighlightStyle, TestAppContext};
use indoc::indoc;
use proto::deserialize_operation;
use rand::prelude::*;
use regex::RegexBuilder;
use settings::SettingsStore;
use settings::{AllLanguageSettingsContent, LanguageSettingsContent};
use std::collections::BTreeSet;
use std::{
    env,
    ops::Range,
    sync::LazyLock,
    time::{Duration, Instant},
};
use syntax_map::TreeSitterOptions;
use text::network::Network;
use text::{BufferId, LineEnding};
use text::{Point, ToPoint};
use theme::ActiveTheme;
use unindent::Unindent as _;
use util::rel_path::rel_path;
use util::test::marked_text_offsets;
use util::{RandomCharIter, assert_set_eq, post_inc, test::marked_text_ranges};

pub static TRAILING_WHITESPACE_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    RegexBuilder::new(r"[ \t]+$")
        .multi_line(true)
        .build()
        .expect("Failed to create TRAILING_WHITESPACE_REGEX")
});

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    zlog::init_test();
}

#[gpui::test]
fn test_line_endings(cx: &mut gpui::App) {
    init_settings(cx, |_| {});

    cx.new(|cx| {
        let mut buffer =
            Buffer::local("one\r\ntwo\rthree", cx).with_language(Arc::new(rust_lang()), cx);
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
fn test_set_line_ending(cx: &mut TestAppContext) {
    let base = cx.new(|cx| Buffer::local("one\ntwo\nthree\n", cx));
    let base_replica = cx.new(|cx| {
        Buffer::from_proto(
            ReplicaId::new(1),
            Capability::ReadWrite,
            base.read(cx).to_proto(cx),
            None,
        )
        .unwrap()
    });
    base.update(cx, |_buffer, cx| {
        cx.subscribe(&base_replica, |this, _, event, cx| {
            if let BufferEvent::Operation {
                operation,
                is_local: true,
            } = event
            {
                this.apply_ops([operation.clone()], cx);
            }
        })
        .detach();
    });
    base_replica.update(cx, |_buffer, cx| {
        cx.subscribe(&base, |this, _, event, cx| {
            if let BufferEvent::Operation {
                operation,
                is_local: true,
            } = event
            {
                this.apply_ops([operation.clone()], cx);
            }
        })
        .detach();
    });

    // Base
    base_replica.read_with(cx, |buffer, _| {
        assert_eq!(buffer.line_ending(), LineEnding::Unix);
    });
    base.update(cx, |buffer, cx| {
        assert_eq!(buffer.line_ending(), LineEnding::Unix);
        buffer.set_line_ending(LineEnding::Windows, cx);
        assert_eq!(buffer.line_ending(), LineEnding::Windows);
    });
    base_replica.read_with(cx, |buffer, _| {
        assert_eq!(buffer.line_ending(), LineEnding::Windows);
    });
    base.update(cx, |buffer, cx| {
        buffer.set_line_ending(LineEnding::Unix, cx);
        assert_eq!(buffer.line_ending(), LineEnding::Unix);
    });
    base_replica.read_with(cx, |buffer, _| {
        assert_eq!(buffer.line_ending(), LineEnding::Unix);
    });

    // Replica
    base.read_with(cx, |buffer, _| {
        assert_eq!(buffer.line_ending(), LineEnding::Unix);
    });
    base_replica.update(cx, |buffer, cx| {
        assert_eq!(buffer.line_ending(), LineEnding::Unix);
        buffer.set_line_ending(LineEnding::Windows, cx);
        assert_eq!(buffer.line_ending(), LineEnding::Windows);
    });
    base.read_with(cx, |buffer, _| {
        assert_eq!(buffer.line_ending(), LineEnding::Windows);
    });
    base_replica.update(cx, |buffer, cx| {
        buffer.set_line_ending(LineEnding::Unix, cx);
        assert_eq!(buffer.line_ending(), LineEnding::Unix);
    });
    base.read_with(cx, |buffer, _| {
        assert_eq!(buffer.line_ending(), LineEnding::Unix);
    });
}

#[gpui::test]
fn test_select_language(cx: &mut App) {
    init_settings(cx, |_| {});

    let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
    registry.add(Arc::new(Language::new(
        LanguageConfig {
            name: LanguageName::new("Rust"),
            matcher: LanguageMatcher {
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    )));
    registry.add(Arc::new(Language::new(
        LanguageConfig {
            name: "Rust with longer extension".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["longer.rs".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    )));
    registry.add(Arc::new(Language::new(
        LanguageConfig {
            name: LanguageName::new("Make"),
            matcher: LanguageMatcher {
                path_suffixes: vec!["Makefile".to_string(), "mk".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    )));

    // matching file extension
    assert_eq!(
        registry
            .language_for_file(&file("src/lib.rs"), None, cx)
            .map(|l| l.name()),
        Some("Rust".into())
    );
    assert_eq!(
        registry
            .language_for_file(&file("src/lib.mk"), None, cx)
            .map(|l| l.name()),
        Some("Make".into())
    );

    // matching longer, compound extension, part of which could also match another lang
    assert_eq!(
        registry
            .language_for_file(&file("src/lib.longer.rs"), None, cx)
            .map(|l| l.name()),
        Some("Rust with longer extension".into())
    );

    // matching filename
    assert_eq!(
        registry
            .language_for_file(&file("src/Makefile"), None, cx)
            .map(|l| l.name()),
        Some("Make".into())
    );

    // matching suffix that is not the full file extension or filename
    assert_eq!(
        registry
            .language_for_file(&file("zed/cars"), None, cx)
            .map(|l| l.name()),
        None
    );
    assert_eq!(
        registry
            .language_for_file(&file("zed/a.cars"), None, cx)
            .map(|l| l.name()),
        None
    );
    assert_eq!(
        registry
            .language_for_file(&file("zed/sumk"), None, cx)
            .map(|l| l.name()),
        None
    );
}

#[gpui::test(iterations = 10)]
async fn test_first_line_pattern(cx: &mut TestAppContext) {
    cx.update(|cx| init_settings(cx, |_| {}));

    let languages = LanguageRegistry::test(cx.executor());
    let languages = Arc::new(languages);

    languages.register_test_language(LanguageConfig {
        name: "JavaScript".into(),
        matcher: LanguageMatcher {
            path_suffixes: vec!["js".into()],
            first_line_pattern: Some(Regex::new(r"\bnode\b").unwrap()),
        },
        ..Default::default()
    });

    assert!(
        cx.read(|cx| languages.language_for_file(&file("the/script"), None, cx))
            .is_none()
    );
    assert!(
        cx.read(|cx| languages.language_for_file(&file("the/script"), Some(&"nothing".into()), cx))
            .is_none()
    );

    assert_eq!(
        cx.read(|cx| languages.language_for_file(
            &file("the/script"),
            Some(&"#!/bin/env node".into()),
            cx
        ))
        .unwrap()
        .name(),
        "JavaScript".into()
    );
}

#[gpui::test]
async fn test_language_for_file_with_custom_file_types(cx: &mut TestAppContext) {
    cx.update(|cx| {
        init_settings(cx, |settings| {
            settings.file_types.get_or_insert_default().extend([
                ("TypeScript".into(), vec!["js".into()].into()),
                (
                    "JavaScript".into(),
                    vec!["*longer.ts".into(), "ecmascript".into()].into(),
                ),
                ("C++".into(), vec!["c".into(), "*.dev".into()].into()),
                (
                    "Dockerfile".into(),
                    vec!["Dockerfile".into(), "Dockerfile.*".into()].into(),
                ),
            ]);
        })
    });

    let languages = Arc::new(LanguageRegistry::test(cx.executor()));

    for config in [
        LanguageConfig {
            name: "JavaScript".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["js".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        LanguageConfig {
            name: "TypeScript".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["ts".to_string(), "ts.ecmascript".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        LanguageConfig {
            name: "C++".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["cpp".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        LanguageConfig {
            name: "C".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["c".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        LanguageConfig {
            name: "Dockerfile".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["Dockerfile".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
    ] {
        languages.add(Arc::new(Language::new(config, None)));
    }

    // matches system-provided lang extension
    let language = cx
        .read(|cx| languages.language_for_file(&file("foo.ts"), None, cx))
        .unwrap();
    assert_eq!(language.name(), "TypeScript".into());
    let language = cx
        .read(|cx| languages.language_for_file(&file("foo.ts.ecmascript"), None, cx))
        .unwrap();
    assert_eq!(language.name(), "TypeScript".into());
    let language = cx
        .read(|cx| languages.language_for_file(&file("foo.cpp"), None, cx))
        .unwrap();
    assert_eq!(language.name(), "C++".into());

    // user configured lang extension, same length as system-provided
    let language = cx
        .read(|cx| languages.language_for_file(&file("foo.js"), None, cx))
        .unwrap();
    assert_eq!(language.name(), "TypeScript".into());
    let language = cx
        .read(|cx| languages.language_for_file(&file("foo.c"), None, cx))
        .unwrap();
    assert_eq!(language.name(), "C++".into());

    // user configured lang extension, longer than system-provided
    let language = cx
        .read(|cx| languages.language_for_file(&file("foo.longer.ts"), None, cx))
        .unwrap();
    assert_eq!(language.name(), "JavaScript".into());

    // user configured lang extension, shorter than system-provided
    let language = cx
        .read(|cx| languages.language_for_file(&file("foo.ecmascript"), None, cx))
        .unwrap();
    assert_eq!(language.name(), "JavaScript".into());

    // user configured glob matches
    let language = cx
        .read(|cx| languages.language_for_file(&file("c-plus-plus.dev"), None, cx))
        .unwrap();
    assert_eq!(language.name(), "C++".into());
    // should match Dockerfile.* => Dockerfile, not *.dev => C++
    let language = cx
        .read(|cx| languages.language_for_file(&file("Dockerfile.dev"), None, cx))
        .unwrap();
    assert_eq!(language.name(), "Dockerfile".into());
}

fn file(path: &str) -> Arc<dyn File> {
    Arc::new(TestFile {
        path: Arc::from(rel_path(path)),
        root_name: "zed".into(),
        local_root: None,
    })
}

#[gpui::test]
fn test_edit_events(cx: &mut gpui::App) {
    let mut now = Instant::now();
    let buffer_1_events = Arc::new(Mutex::new(Vec::new()));
    let buffer_2_events = Arc::new(Mutex::new(Vec::new()));

    let buffer1 = cx.new(|cx| Buffer::local("abcdef", cx));
    let buffer2 = cx.new(|cx| {
        Buffer::remote(
            BufferId::from(cx.entity_id().as_non_zero_u64()),
            ReplicaId::new(1),
            Capability::ReadWrite,
            "abcdef",
        )
    });
    let buffer1_ops = Arc::new(Mutex::new(Vec::new()));
    buffer1.update(cx, {
        let buffer1_ops = buffer1_ops.clone();
        |buffer, cx| {
            let buffer_1_events = buffer_1_events.clone();
            cx.subscribe(&buffer1, move |_, _, event, _| match event.clone() {
                BufferEvent::Operation {
                    operation,
                    is_local: true,
                } => buffer1_ops.lock().push(operation),
                event => buffer_1_events.lock().push(event),
            })
            .detach();
            let buffer_2_events = buffer_2_events.clone();
            cx.subscribe(&buffer2, move |_, _, event, _| match event.clone() {
                BufferEvent::Operation {
                    is_local: false, ..
                } => {}
                event => buffer_2_events.lock().push(event),
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
        buffer.apply_ops(buffer1_ops.lock().drain(..), cx);
    });
    assert_eq!(
        mem::take(&mut *buffer_1_events.lock()),
        vec![
            BufferEvent::Edited,
            BufferEvent::DirtyChanged,
            BufferEvent::Edited,
            BufferEvent::Edited,
        ]
    );
    assert_eq!(
        mem::take(&mut *buffer_2_events.lock()),
        vec![BufferEvent::Edited, BufferEvent::DirtyChanged]
    );

    buffer1.update(cx, |buffer, cx| {
        // Undoing the first transaction emits edited event, followed by a
        // dirty changed event, since the buffer is again in a clean state.
        buffer.undo(cx);
    });
    // Incorporating the remote ops again emits a single edited event,
    // followed by a dirty changed event.
    buffer2.update(cx, |buffer, cx| {
        buffer.apply_ops(buffer1_ops.lock().drain(..), cx);
    });
    assert_eq!(
        mem::take(&mut *buffer_1_events.lock()),
        vec![BufferEvent::Edited, BufferEvent::DirtyChanged,]
    );
    assert_eq!(
        mem::take(&mut *buffer_2_events.lock()),
        vec![BufferEvent::Edited, BufferEvent::DirtyChanged]
    );
}

#[gpui::test]
async fn test_apply_diff(cx: &mut TestAppContext) {
    let (text, offsets) = marked_text_offsets(
        "one two three\nfour fiˇve six\nseven eightˇ nine\nten eleven twelve\n",
    );
    let buffer = cx.new(|cx| Buffer::local(text, cx));
    let anchors = buffer.update(cx, |buffer, _| {
        offsets
            .iter()
            .map(|offset| buffer.anchor_before(offset))
            .collect::<Vec<_>>()
    });

    let (text, offsets) = marked_text_offsets(
        "one two three\n{\nfour FIVEˇ six\n}\nseven AND EIGHTˇ nine\nten eleven twelve\n",
    );

    let diff = buffer.update(cx, |b, cx| b.diff(text.clone(), cx)).await;
    buffer.update(cx, |buffer, cx| {
        buffer.apply_diff(diff, cx).unwrap();
        assert_eq!(buffer.text(), text);
        let actual_offsets = anchors
            .iter()
            .map(|anchor| anchor.to_offset(buffer))
            .collect::<Vec<_>>();
        assert_eq!(actual_offsets, offsets);
    });

    let (text, offsets) =
        marked_text_offsets("one two three\n{\nˇ}\nseven AND EIGHTEENˇ nine\nten eleven twelve\n");

    let diff = buffer.update(cx, |b, cx| b.diff(text.clone(), cx)).await;
    buffer.update(cx, |buffer, cx| {
        buffer.apply_diff(diff, cx).unwrap();
        assert_eq!(buffer.text(), text);
        let actual_offsets = anchors
            .iter()
            .map(|anchor| anchor.to_offset(buffer))
            .collect::<Vec<_>>();
        assert_eq!(actual_offsets, offsets);
    });
}

#[gpui::test(iterations = 10)]
async fn test_normalize_whitespace(cx: &mut gpui::TestAppContext) {
    let text = [
        "zero",     //
        "one  ",    // 2 trailing spaces
        "two",      //
        "three   ", // 3 trailing spaces
        "four",     //
        "five    ", // 4 trailing spaces
    ]
    .join("\n");

    let buffer = cx.new(|cx| Buffer::local(text, cx));

    // Spawn a task to format the buffer's whitespace.
    // Pause so that the formatting task starts running.
    let format = buffer.update(cx, |buffer, cx| buffer.remove_trailing_whitespace(cx));
    smol::future::yield_now().await;

    // Edit the buffer while the normalization task is running.
    let version_before_edit = buffer.update(cx, |buffer, _| buffer.version());
    buffer.update(cx, |buffer, cx| {
        buffer.edit(
            [
                (Point::new(0, 1)..Point::new(0, 1), "EE"),
                (Point::new(3, 5)..Point::new(3, 5), "EEE"),
            ],
            None,
            cx,
        );
    });

    let format_diff = format.await;
    buffer.update(cx, |buffer, cx| {
        let version_before_format = format_diff.base_version.clone();
        buffer.apply_diff(format_diff, cx);

        // The outcome depends on the order of concurrent tasks.
        //
        // If the edit occurred while searching for trailing whitespace ranges,
        // then the trailing whitespace region touched by the edit is left intact.
        if version_before_format == version_before_edit {
            assert_eq!(
                buffer.text(),
                [
                    "zEEero",      //
                    "one",         //
                    "two",         //
                    "threeEEE   ", //
                    "four",        //
                    "five",        //
                ]
                .join("\n")
            );
        }
        // Otherwise, all trailing whitespace is removed.
        else {
            assert_eq!(
                buffer.text(),
                [
                    "zEEero",   //
                    "one",      //
                    "two",      //
                    "threeEEE", //
                    "four",     //
                    "five",     //
                ]
                .join("\n")
            );
        }
    });
}

#[gpui::test]
async fn test_reparse(cx: &mut gpui::TestAppContext) {
    let text = "fn a() {}";
    let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx));

    // Wait for the initial text to parse
    cx.executor().run_until_parked();
    assert!(!buffer.update(cx, |buffer, _| buffer.is_parsing()));
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
    cx.executor().run_until_parked();
    assert!(!buffer.update(cx, |buffer, _| buffer.is_parsing()));
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
    cx.executor().run_until_parked();
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
        buf.undo(cx);
        buf.undo(cx);
        buf.undo(cx);
        assert_eq!(buf.text(), "fn a() {}");
        assert!(buf.is_parsing());
    });

    cx.executor().run_until_parked();
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
        buf.redo(cx);
        buf.redo(cx);
        buf.redo(cx);
        assert_eq!(buf.text(), "fn a(b: C) { d.e::<G>(f); }");
        assert!(buf.is_parsing());
    });
    cx.executor().run_until_parked();
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
    let buffer = cx.new(|cx| {
        let mut buffer = Buffer::local("{}", cx).with_language(Arc::new(rust_lang()), cx);
        buffer.set_sync_parse_timeout(Duration::ZERO);
        buffer
    });

    // Wait for the initial text to parse
    cx.executor().run_until_parked();
    assert_eq!(
        get_tree_sexp(&buffer, cx),
        "(source_file (expression_statement (block)))"
    );

    buffer.update(cx, |buffer, cx| {
        buffer.set_language(Some(Arc::new(json_lang())), cx)
    });
    cx.executor().run_until_parked();
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

    let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx));
    let outline = buffer.update(cx, |buffer, _| buffer.snapshot().outline(None));

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
            .update(|cx| outline.search(query, cx.background_executor().clone()))
            .await;
        matches
            .into_iter()
            .map(|mat| (outline.items[mat.candidate_id].text.as_str(), mat.positions))
            .collect::<Vec<_>>()
    }
}

#[gpui::test]
async fn test_outline_nodes_with_newlines(cx: &mut gpui::TestAppContext) {
    let text = r#"
        impl A for B<
            C
        > {
        };
    "#
    .unindent();

    let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx));
    let outline = buffer.update(cx, |buffer, _| buffer.snapshot().outline(None));

    assert_eq!(
        outline
            .items
            .iter()
            .map(|item| (item.text.as_str(), item.depth))
            .collect::<Vec<_>>(),
        &[("impl A for B<", 0)]
    );
}

#[gpui::test]
async fn test_outline_with_extra_context(cx: &mut gpui::TestAppContext) {
    let language = javascript_lang()
        .with_outline_query(
            r#"
            (function_declaration
                "function" @context
                name: (_) @name
                parameters: (formal_parameters
                    "(" @context.extra
                    ")" @context.extra)) @item
            "#,
        )
        .unwrap();

    let text = r#"
        function a() {}
        function b(c) {}
    "#
    .unindent();

    let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(Arc::new(language), cx));
    let snapshot = buffer.update(cx, |buffer, _| buffer.snapshot());

    // extra context nodes are included in the outline.
    let outline = snapshot.outline(None);
    assert_eq!(
        outline
            .items
            .iter()
            .map(|item| (item.text.as_str(), item.depth))
            .collect::<Vec<_>>(),
        &[("function a()", 0), ("function b( )", 0),]
    );

    // extra context nodes do not appear in breadcrumbs.
    let symbols = snapshot.symbols_containing(3, None);
    assert_eq!(
        symbols
            .iter()
            .map(|item| (item.text.as_str(), item.depth))
            .collect::<Vec<_>>(),
        &[("function a", 0)]
    );
}

#[gpui::test]
fn test_outline_annotations(cx: &mut App) {
    // Add this new test case
    let text = r#"
        /// This is a doc comment
        /// that spans multiple lines
        fn annotated_function() {
            // This is not an annotation
        }

        // This is a single-line annotation
        fn another_function() {}

        fn unannotated_function() {}

        // This comment is not an annotation

        fn function_after_blank_line() {}
    "#
    .unindent();

    let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx));
    let outline = buffer.update(cx, |buffer, _| buffer.snapshot().outline(None));

    assert_eq!(
        outline
            .items
            .into_iter()
            .map(|item| (
                item.text,
                item.depth,
                item.annotation_range
                    .map(|range| { buffer.read(cx).text_for_range(range).collect::<String>() })
            ))
            .collect::<Vec<_>>(),
        &[
            (
                "fn annotated_function".to_string(),
                0,
                Some("/// This is a doc comment\n/// that spans multiple lines".to_string())
            ),
            (
                "fn another_function".to_string(),
                0,
                Some("// This is a single-line annotation".to_string())
            ),
            ("fn unannotated_function".to_string(), 0, None),
            ("fn function_after_blank_line".to_string(), 0, None),
        ]
    );
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

    let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx));
    let snapshot = buffer.update(cx, |buffer, _| buffer.snapshot());

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
            .into_iter()
            .map(|item| {
                (
                    item.text,
                    item.range.start.to_point(snapshot)..item.range.end.to_point(snapshot),
                )
            })
            .collect()
    }

    let (text, offsets) = marked_text_offsets(
        &"
        // ˇ😅 //
        fn test() {
        }
    "
        .unindent(),
    );
    let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx));
    let snapshot = buffer.update(cx, |buffer, _| buffer.snapshot());

    // note, it would be nice to actually return the method test in this
    // case, but primarily asserting we don't crash because of the multibyte character.
    assert_eq!(snapshot.symbols_containing(offsets[0], None), vec![]);
}

#[gpui::test]
fn test_text_objects(cx: &mut App) {
    let (text, ranges) = marked_text_ranges(
        indoc! {r#"
            impl Hello {
                fn say() -> u8 { return /* ˇhi */ 1 }
            }"#
        },
        false,
    );

    let buffer =
        cx.new(|cx| Buffer::local(text.clone(), cx).with_language(Arc::new(rust_lang()), cx));
    let snapshot = buffer.update(cx, |buffer, _| buffer.snapshot());

    let matches = snapshot
        .text_object_ranges(ranges[0].clone(), TreeSitterOptions::default())
        .map(|(range, text_object)| (&text[range], text_object))
        .collect::<Vec<_>>();

    assert_eq!(
        matches,
        &[
            ("/* hi */", TextObject::AroundComment),
            ("return /* hi */ 1", TextObject::InsideFunction),
            (
                "fn say() -> u8 { return /* hi */ 1 }",
                TextObject::AroundFunction
            ),
        ],
    )
}

#[gpui::test]
fn test_enclosing_bracket_ranges(cx: &mut App) {
    #[track_caller]
    fn assert(selection_text: &'static str, range_markers: Vec<&'static str>, cx: &mut App) {
        assert_bracket_pairs(selection_text, range_markers, rust_lang(), cx)
    }

    assert(
        indoc! {"
            mod x {
                moˇd y {

                }
            }
            let foo = 1;"},
        vec![indoc! {"
            mod x «{»
                mod y {

                }
            «}»
            let foo = 1;"}],
        cx,
    );

    assert(
        indoc! {"
            mod x {
                mod y ˇ{

                }
            }
            let foo = 1;"},
        vec![
            indoc! {"
                mod x «{»
                    mod y {

                    }
                «}»
                let foo = 1;"},
            indoc! {"
                mod x {
                    mod y «{»

                    «}»
                }
                let foo = 1;"},
        ],
        cx,
    );

    assert(
        indoc! {"
            mod x {
                mod y {

                }ˇ
            }
            let foo = 1;"},
        vec![
            indoc! {"
                mod x «{»
                    mod y {

                    }
                «}»
                let foo = 1;"},
            indoc! {"
                mod x {
                    mod y «{»

                    «}»
                }
                let foo = 1;"},
        ],
        cx,
    );

    assert(
        indoc! {"
            mod x {
                mod y {

                }
            ˇ}
            let foo = 1;"},
        vec![indoc! {"
            mod x «{»
                mod y {

                }
            «}»
            let foo = 1;"}],
        cx,
    );

    assert(
        indoc! {"
            mod x {
                mod y {

                }
            }
            let fˇoo = 1;"},
        Vec::new(),
        cx,
    );

    // Regression test: avoid crash when querying at the end of the buffer.
    assert(
        indoc! {"
            mod x {
                mod y {

                }
            }
            let foo = 1;ˇ"},
        Vec::new(),
        cx,
    );
}

#[gpui::test]
fn test_enclosing_bracket_ranges_where_brackets_are_not_outermost_children(cx: &mut App) {
    let mut assert = |selection_text, bracket_pair_texts| {
        assert_bracket_pairs(selection_text, bracket_pair_texts, javascript_lang(), cx)
    };

    assert(
        indoc! {"
        for (const a in b)ˇ {
            // a comment that's longer than the for-loop header
        }"},
        vec![indoc! {"
        for «(»const a in b«)» {
            // a comment that's longer than the for-loop header
        }"}],
    );

    // Regression test: even though the parent node of the parentheses (the for loop) does
    // intersect the given range, the parentheses themselves do not contain the range, so
    // they should not be returned. Only the curly braces contain the range.
    assert(
        indoc! {"
        for (const a in b) {ˇ
            // a comment that's longer than the for-loop header
        }"},
        vec![indoc! {"
        for (const a in b) «{»
            // a comment that's longer than the for-loop header
        «}»"}],
    );
}

#[gpui::test]
fn test_range_for_syntax_ancestor(cx: &mut App) {
    cx.new(|cx| {
        let text = "fn a() { b(|c| {}) }";
        let buffer = Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx);
        let snapshot = buffer.snapshot();

        assert_eq!(
            snapshot
                .syntax_ancestor(empty_range_at(text, "|"))
                .unwrap()
                .byte_range(),
            range_of(text, "|")
        );
        assert_eq!(
            snapshot
                .syntax_ancestor(range_of(text, "|"))
                .unwrap()
                .byte_range(),
            range_of(text, "|c|")
        );
        assert_eq!(
            snapshot
                .syntax_ancestor(range_of(text, "|c|"))
                .unwrap()
                .byte_range(),
            range_of(text, "|c| {}")
        );
        assert_eq!(
            snapshot
                .syntax_ancestor(range_of(text, "|c| {}"))
                .unwrap()
                .byte_range(),
            range_of(text, "(|c| {})")
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
fn test_autoindent_with_soft_tabs(cx: &mut App) {
    init_settings(cx, |_| {});

    cx.new(|cx| {
        let text = "fn a() {}";
        let mut buffer = Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx);

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
fn test_autoindent_with_hard_tabs(cx: &mut App) {
    init_settings(cx, |settings| {
        settings.defaults.hard_tabs = Some(true);
    });

    cx.new(|cx| {
        let text = "fn a() {}";
        let mut buffer = Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx);

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
fn test_autoindent_does_not_adjust_lines_with_unchanged_suggestion(cx: &mut App) {
    init_settings(cx, |_| {});

    cx.new(|cx| {
        let mut buffer = Buffer::local(
            "
            fn a() {
            c;
            d;
            }
            "
            .unindent(),
            cx,
        )
        .with_language(Arc::new(rust_lang()), cx);

        // Lines 2 and 3 don't match the indentation suggestion. When editing these lines,
        // their indentation is not adjusted.
        buffer.edit_via_marked_text(
            &"
            fn a() {
            c«()»;
            d«()»;
            }
            "
            .unindent(),
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
        buffer.edit_via_marked_text(
            &"
            fn a() {
            c«
            .f
            .g()»;
            d«
            .f
            .g()»;
            }
            "
            .unindent(),
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

        // Insert a newline after the open brace. It is auto-indented
        buffer.edit_via_marked_text(
            &"
            fn a() {«
            »
            c
                .f
                .g();
            d
                .f
                .g();
            }
            "
            .unindent(),
            Some(AutoindentMode::EachLine),
            cx,
        );
        assert_eq!(
            buffer.text(),
            "
            fn a() {
                ˇ
            c
                .f
                .g();
            d
                .f
                .g();
            }
            "
            .unindent()
            .replace("ˇ", "")
        );

        // Manually outdent the line. It stays outdented.
        buffer.edit_via_marked_text(
            &"
            fn a() {
            «»
            c
                .f
                .g();
            d
                .f
                .g();
            }
            "
            .unindent(),
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

    cx.new(|cx| {
        eprintln!("second buffer: {:?}", cx.entity_id());

        let mut buffer = Buffer::local(
            "
            fn a() {
                b();
                |
            "
            .replace('|', "") // marker to preserve trailing whitespace
            .unindent(),
            cx,
        )
        .with_language(Arc::new(rust_lang()), cx);

        // Insert a closing brace. It is outdented.
        buffer.edit_via_marked_text(
            &"
            fn a() {
                b();
                «}»
            "
            .unindent(),
            Some(AutoindentMode::EachLine),
            cx,
        );
        assert_eq!(
            buffer.text(),
            "
            fn a() {
                b();
            }
            "
            .unindent()
        );

        // Manually edit the leading whitespace. The edit is preserved.
        buffer.edit_via_marked_text(
            &"
            fn a() {
                b();
            «    »}
            "
            .unindent(),
            Some(AutoindentMode::EachLine),
            cx,
        );
        assert_eq!(
            buffer.text(),
            "
            fn a() {
                b();
                }
            "
            .unindent()
        );
        buffer
    });

    eprintln!("DONE");
}

#[gpui::test]
fn test_autoindent_does_not_adjust_lines_within_newly_created_errors(cx: &mut App) {
    init_settings(cx, |_| {});

    cx.new(|cx| {
        let mut buffer = Buffer::local(
            "
            fn a() {
                i
            }
            "
            .unindent(),
            cx,
        )
        .with_language(Arc::new(rust_lang()), cx);

        // Regression test: line does not get outdented due to syntax error
        buffer.edit_via_marked_text(
            &"
            fn a() {
                i«f let Some(x) = y»
            }
            "
            .unindent(),
            Some(AutoindentMode::EachLine),
            cx,
        );
        assert_eq!(
            buffer.text(),
            "
            fn a() {
                if let Some(x) = y
            }
            "
            .unindent()
        );

        buffer.edit_via_marked_text(
            &"
            fn a() {
                if let Some(x) = y« {»
            }
            "
            .unindent(),
            Some(AutoindentMode::EachLine),
            cx,
        );
        assert_eq!(
            buffer.text(),
            "
            fn a() {
                if let Some(x) = y {
            }
            "
            .unindent()
        );

        buffer
    });
}

#[gpui::test]
fn test_autoindent_adjusts_lines_when_only_text_changes(cx: &mut App) {
    init_settings(cx, |_| {});

    cx.new(|cx| {
        let mut buffer = Buffer::local(
            "
            fn a() {}
            "
            .unindent(),
            cx,
        )
        .with_language(Arc::new(rust_lang()), cx);

        buffer.edit_via_marked_text(
            &"
            fn a(«
            b») {}
            "
            .unindent(),
            Some(AutoindentMode::EachLine),
            cx,
        );
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
        buffer.edit_via_marked_text(
            &"
            fn a(
                ˇ) {}
            "
            .unindent(),
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
fn test_autoindent_with_edit_at_end_of_buffer(cx: &mut App) {
    init_settings(cx, |_| {});

    cx.new(|cx| {
        let text = "a\nb";
        let mut buffer = Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx);
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
fn test_autoindent_multi_line_insertion(cx: &mut App) {
    init_settings(cx, |_| {});

    cx.new(|cx| {
        let text = "
            const a: usize = 1;
            fn b() {
                if c {
                    let d = 2;
                }
            }
        "
        .unindent();

        let mut buffer = Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx);
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
fn test_autoindent_block_mode(cx: &mut App) {
    init_settings(cx, |_| {});

    cx.new(|cx| {
        let text = r#"
            fn a() {
                b();
            }
        "#
        .unindent();
        let mut buffer = Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx);

        // When this text was copied, both of the quotation marks were at the same
        // indent level, but the indentation of the first line was not included in
        // the copied text. This information is retained in the
        // 'original_indent_columns' vector.
        let original_indent_columns = vec![Some(4)];
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
                original_indent_columns: original_indent_columns.clone(),
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

        // Grouping is disabled in tests, so we need 2 undos
        buffer.undo(cx); // Undo the auto-indent
        buffer.undo(cx); // Undo the original edit

        // Insert the block at a deeper indent level. The entire block is outdented.
        buffer.edit([(Point::new(2, 0)..Point::new(2, 0), "        ")], None, cx);
        buffer.edit(
            [(Point::new(2, 8)..Point::new(2, 8), inserted_text)],
            Some(AutoindentMode::Block {
                original_indent_columns,
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
fn test_autoindent_block_mode_with_newline(cx: &mut App) {
    init_settings(cx, |_| {});

    cx.new(|cx| {
        let text = r#"
            fn a() {
                b();
            }
        "#
        .unindent();
        let mut buffer = Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx);

        // First line contains just '\n', it's indentation is stored in "original_indent_columns"
        let original_indent_columns = vec![Some(4)];
        let inserted_text = r#"

                c();
                    d();
                        e();
        "#
        .unindent();
        buffer.edit(
            [(Point::new(2, 0)..Point::new(2, 0), inserted_text)],
            Some(AutoindentMode::Block {
                original_indent_columns,
            }),
            cx,
        );

        // While making edit, we ignore first line as it only contains '\n'
        // hence second line indent is used to calculate delta
        assert_eq!(
            buffer.text(),
            r#"
            fn a() {
                b();

                c();
                    d();
                        e();
            }
            "#
            .unindent()
        );

        buffer
    });
}

#[gpui::test]
fn test_autoindent_block_mode_without_original_indent_columns(cx: &mut App) {
    init_settings(cx, |_| {});

    cx.new(|cx| {
        let text = r#"
            fn a() {
                if b() {

                }
            }
        "#
        .unindent();
        let mut buffer = Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx);

        // The original indent columns are not known, so this text is
        // auto-indented in a block as if the first line was copied in
        // its entirety.
        let original_indent_columns = Vec::new();
        let inserted_text = "    c\n        .d()\n        .e();";

        // Insert the block at column zero. The entire block is indented
        // so that the first line matches the previous line's indentation.
        buffer.edit(
            [(Point::new(2, 0)..Point::new(2, 0), inserted_text)],
            Some(AutoindentMode::Block {
                original_indent_columns,
            }),
            cx,
        );
        assert_eq!(
            buffer.text(),
            r#"
            fn a() {
                if b() {
                    c
                        .d()
                        .e();
                }
            }
            "#
            .unindent()
        );

        // Grouping is disabled in tests, so we need 2 undos
        buffer.undo(cx); // Undo the auto-indent
        buffer.undo(cx); // Undo the original edit

        // Insert the block at a deeper indent level. The entire block is outdented.
        buffer.edit(
            [(Point::new(2, 0)..Point::new(2, 0), " ".repeat(12))],
            None,
            cx,
        );
        buffer.edit(
            [(Point::new(2, 12)..Point::new(2, 12), inserted_text)],
            Some(AutoindentMode::Block {
                original_indent_columns: Vec::new(),
            }),
            cx,
        );
        assert_eq!(
            buffer.text(),
            r#"
            fn a() {
                if b() {
                    c
                        .d()
                        .e();
                }
            }
            "#
            .unindent()
        );

        buffer
    });
}

#[gpui::test]
fn test_autoindent_block_mode_multiple_adjacent_ranges(cx: &mut App) {
    init_settings(cx, |_| {});

    cx.new(|cx| {
        let (text, ranges_to_replace) = marked_text_ranges(
            &"
            mod numbers {
                «fn one() {
                    1
                }
            »
                «fn two() {
                    2
                }
            »
                «fn three() {
                    3
                }
            »}
            "
            .unindent(),
            false,
        );

        let mut buffer = Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx);

        buffer.edit(
            [
                (ranges_to_replace[0].clone(), "fn one() {\n    101\n}\n"),
                (ranges_to_replace[1].clone(), "fn two() {\n    102\n}\n"),
                (ranges_to_replace[2].clone(), "fn three() {\n    103\n}\n"),
            ],
            Some(AutoindentMode::Block {
                original_indent_columns: vec![Some(0), Some(0), Some(0)],
            }),
            cx,
        );

        pretty_assertions::assert_eq!(
            buffer.text(),
            "
            mod numbers {
                fn one() {
                    101
                }

                fn two() {
                    102
                }

                fn three() {
                    103
                }
            }
            "
            .unindent()
        );

        buffer
    });
}

#[gpui::test]
fn test_autoindent_language_without_indents_query(cx: &mut App) {
    init_settings(cx, |_| {});

    cx.new(|cx| {
        let text = "
            * one
                - a
                - b
            * two
        "
        .unindent();

        let mut buffer = Buffer::local(text, cx).with_language(
            Arc::new(Language::new(
                LanguageConfig {
                    name: "Markdown".into(),
                    auto_indent_using_last_non_empty_line: false,
                    ..Default::default()
                },
                Some(tree_sitter_json::LANGUAGE.into()),
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
fn test_autoindent_with_injected_languages(cx: &mut App) {
    init_settings(cx, |settings| {
        settings.languages.0.extend([
            (
                "HTML".into(),
                LanguageSettingsContent {
                    tab_size: Some(2.try_into().unwrap()),
                    ..Default::default()
                },
            ),
            (
                "JavaScript".into(),
                LanguageSettingsContent {
                    tab_size: Some(8.try_into().unwrap()),
                    ..Default::default()
                },
            ),
        ])
    });

    let html_language = Arc::new(html_lang());

    let javascript_language = Arc::new(javascript_lang());

    let language_registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
    language_registry.add(html_language.clone());
    language_registry.add(javascript_language);

    cx.new(|cx| {
        let (text, ranges) = marked_text_ranges(
            &"
                <div>ˇ
                </div>
                <script>
                    init({ˇ
                    })
                </script>
                <span>ˇ
                </span>
            "
            .unindent(),
            false,
        );

        let mut buffer = Buffer::local(text, cx);
        buffer.set_language_registry(language_registry);
        buffer.set_language(Some(html_language), cx);
        buffer.edit(
            ranges.into_iter().map(|range| (range, "\na")),
            Some(AutoindentMode::EachLine),
            cx,
        );
        assert_eq!(
            buffer.text(),
            "
                <div>
                  a
                </div>
                <script>
                    init({
                            a
                    })
                </script>
                <span>
                  a
                </span>
            "
            .unindent()
        );
        buffer
    });
}

#[gpui::test]
fn test_autoindent_query_with_outdent_captures(cx: &mut App) {
    init_settings(cx, |settings| {
        settings.defaults.tab_size = Some(2.try_into().unwrap());
    });

    cx.new(|cx| {
        let mut buffer = Buffer::local("", cx).with_language(Arc::new(ruby_lang()), cx);

        let text = r#"
            class C
            def a(b, c)
            puts b
            puts c
            rescue
            puts "errored"
            exit 1
            end
            end
        "#
        .unindent();

        buffer.edit([(0..0, text)], Some(AutoindentMode::EachLine), cx);

        assert_eq!(
            buffer.text(),
            r#"
                class C
                  def a(b, c)
                    puts b
                    puts c
                  rescue
                    puts "errored"
                    exit 1
                  end
                end
            "#
            .unindent()
        );

        buffer
    });
}

#[gpui::test]
async fn test_async_autoindents_preserve_preview(cx: &mut TestAppContext) {
    cx.update(|cx| init_settings(cx, |_| {}));

    // First we insert some newlines to request an auto-indent (asynchronously).
    // Then we request that a preview tab be preserved for the new version, even though it's edited.
    let buffer = cx.new(|cx| {
        let text = "fn a() {}";
        let mut buffer = Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx);

        // This causes autoindent to be async.
        buffer.set_sync_parse_timeout(Duration::ZERO);

        buffer.edit([(8..8, "\n\n")], Some(AutoindentMode::EachLine), cx);
        buffer.refresh_preview();

        // Synchronously, we haven't auto-indented and we're still preserving the preview.
        assert_eq!(buffer.text(), "fn a() {\n\n}");
        assert!(buffer.preserve_preview());
        buffer
    });

    // Now let the autoindent finish
    cx.executor().run_until_parked();

    // The auto-indent applied, but didn't dismiss our preview
    buffer.update(cx, |buffer, cx| {
        assert_eq!(buffer.text(), "fn a() {\n    \n}");
        assert!(buffer.preserve_preview());

        // Edit inserting another line. It will autoindent async.
        // Then refresh the preview version.
        buffer.edit(
            [(Point::new(1, 4)..Point::new(1, 4), "\n")],
            Some(AutoindentMode::EachLine),
            cx,
        );
        buffer.refresh_preview();
        assert_eq!(buffer.text(), "fn a() {\n    \n\n}");
        assert!(buffer.preserve_preview());

        // Then perform another edit, this time without refreshing the preview version.
        buffer.edit([(Point::new(1, 4)..Point::new(1, 4), "x")], None, cx);
        // This causes the preview to not be preserved.
        assert!(!buffer.preserve_preview());
    });

    // Let the async autoindent from the first edit finish.
    cx.executor().run_until_parked();

    // The autoindent applies, but it shouldn't restore the preview status because we had an edit in the meantime.
    buffer.update(cx, |buffer, _| {
        assert_eq!(buffer.text(), "fn a() {\n    x\n    \n}");
        assert!(!buffer.preserve_preview());
    });
}

#[gpui::test]
fn test_insert_empty_line(cx: &mut App) {
    init_settings(cx, |_| {});

    // Insert empty line at the beginning, requesting an empty line above
    cx.new(|cx| {
        let mut buffer = Buffer::local("abc\ndef\nghi", cx);
        let point = buffer.insert_empty_line(Point::new(0, 0), true, false, cx);
        assert_eq!(buffer.text(), "\nabc\ndef\nghi");
        assert_eq!(point, Point::new(0, 0));
        buffer
    });

    // Insert empty line at the beginning, requesting an empty line above and below
    cx.new(|cx| {
        let mut buffer = Buffer::local("abc\ndef\nghi", cx);
        let point = buffer.insert_empty_line(Point::new(0, 0), true, true, cx);
        assert_eq!(buffer.text(), "\n\nabc\ndef\nghi");
        assert_eq!(point, Point::new(0, 0));
        buffer
    });

    // Insert empty line at the start of a line, requesting empty lines above and below
    cx.new(|cx| {
        let mut buffer = Buffer::local("abc\ndef\nghi", cx);
        let point = buffer.insert_empty_line(Point::new(2, 0), true, true, cx);
        assert_eq!(buffer.text(), "abc\ndef\n\n\n\nghi");
        assert_eq!(point, Point::new(3, 0));
        buffer
    });

    // Insert empty line in the middle of a line, requesting empty lines above and below
    cx.new(|cx| {
        let mut buffer = Buffer::local("abc\ndefghi\njkl", cx);
        let point = buffer.insert_empty_line(Point::new(1, 3), true, true, cx);
        assert_eq!(buffer.text(), "abc\ndef\n\n\n\nghi\njkl");
        assert_eq!(point, Point::new(3, 0));
        buffer
    });

    // Insert empty line in the middle of a line, requesting empty line above only
    cx.new(|cx| {
        let mut buffer = Buffer::local("abc\ndefghi\njkl", cx);
        let point = buffer.insert_empty_line(Point::new(1, 3), true, false, cx);
        assert_eq!(buffer.text(), "abc\ndef\n\n\nghi\njkl");
        assert_eq!(point, Point::new(3, 0));
        buffer
    });

    // Insert empty line in the middle of a line, requesting empty line below only
    cx.new(|cx| {
        let mut buffer = Buffer::local("abc\ndefghi\njkl", cx);
        let point = buffer.insert_empty_line(Point::new(1, 3), false, true, cx);
        assert_eq!(buffer.text(), "abc\ndef\n\n\nghi\njkl");
        assert_eq!(point, Point::new(2, 0));
        buffer
    });

    // Insert empty line at the end, requesting empty lines above and below
    cx.new(|cx| {
        let mut buffer = Buffer::local("abc\ndef\nghi", cx);
        let point = buffer.insert_empty_line(Point::new(2, 3), true, true, cx);
        assert_eq!(buffer.text(), "abc\ndef\nghi\n\n\n");
        assert_eq!(point, Point::new(4, 0));
        buffer
    });

    // Insert empty line at the end, requesting empty line above only
    cx.new(|cx| {
        let mut buffer = Buffer::local("abc\ndef\nghi", cx);
        let point = buffer.insert_empty_line(Point::new(2, 3), true, false, cx);
        assert_eq!(buffer.text(), "abc\ndef\nghi\n\n");
        assert_eq!(point, Point::new(4, 0));
        buffer
    });

    // Insert empty line at the end, requesting empty line below only
    cx.new(|cx| {
        let mut buffer = Buffer::local("abc\ndef\nghi", cx);
        let point = buffer.insert_empty_line(Point::new(2, 3), false, true, cx);
        assert_eq!(buffer.text(), "abc\ndef\nghi\n\n");
        assert_eq!(point, Point::new(3, 0));
        buffer
    });
}

#[gpui::test]
fn test_language_scope_at_with_javascript(cx: &mut App) {
    init_settings(cx, |_| {});

    cx.new(|cx| {
        let language = Language::new(
            LanguageConfig {
                name: "JavaScript".into(),
                line_comments: vec!["// ".into()],
                block_comment: Some(BlockCommentConfig {
                    start: "/*".into(),
                    end: "*/".into(),
                    prefix: "* ".into(),
                    tab_size: 1,
                }),
                brackets: BracketPairConfig {
                    pairs: vec![
                        BracketPair {
                            start: "{".into(),
                            end: "}".into(),
                            close: true,
                            surround: true,
                            newline: false,
                        },
                        BracketPair {
                            start: "'".into(),
                            end: "'".into(),
                            close: true,
                            surround: true,
                            newline: false,
                        },
                    ],
                    disabled_scopes_by_bracket_ix: vec![
                        Vec::new(),                              //
                        vec!["string".into(), "comment".into()], // single quotes disabled
                    ],
                },
                overrides: [(
                    "element".into(),
                    LanguageConfigOverride {
                        line_comments: Override::Remove { remove: true },
                        block_comment: Override::Set(BlockCommentConfig {
                            start: "{/*".into(),
                            prefix: "".into(),
                            end: "*/}".into(),
                            tab_size: 0,
                        }),
                        ..Default::default()
                    },
                )]
                .into_iter()
                .collect(),
                ..Default::default()
            },
            Some(tree_sitter_typescript::LANGUAGE_TSX.into()),
        )
        .with_override_query(
            r#"
                (jsx_element) @element
                (string) @string
                (comment) @comment.inclusive
                [
                    (jsx_opening_element)
                    (jsx_closing_element)
                    (jsx_expression)
                ] @default
            "#,
        )
        .unwrap();

        let text = r#"
            a["b"] = <C d="e">
                <F></F>
                { g() }
            </C>; // a comment
        "#
        .unindent();

        let buffer = Buffer::local(&text, cx).with_language(Arc::new(language), cx);
        let snapshot = buffer.snapshot();

        let config = snapshot.language_scope_at(0).unwrap();
        assert_eq!(config.line_comment_prefixes(), &[Arc::from("// ")]);
        assert_eq!(
            config.block_comment(),
            Some(&BlockCommentConfig {
                start: "/*".into(),
                prefix: "* ".into(),
                end: "*/".into(),
                tab_size: 1,
            })
        );

        // Both bracket pairs are enabled
        assert_eq!(
            config.brackets().map(|e| e.1).collect::<Vec<_>>(),
            &[true, true]
        );

        let comment_config = snapshot
            .language_scope_at(text.find("comment").unwrap() + "comment".len())
            .unwrap();
        assert_eq!(
            comment_config.brackets().map(|e| e.1).collect::<Vec<_>>(),
            &[true, false]
        );

        let string_config = snapshot
            .language_scope_at(text.find("b\"").unwrap())
            .unwrap();
        assert_eq!(string_config.line_comment_prefixes(), &[Arc::from("// ")]);
        assert_eq!(
            string_config.block_comment(),
            Some(&BlockCommentConfig {
                start: "/*".into(),
                prefix: "* ".into(),
                end: "*/".into(),
                tab_size: 1,
            })
        );
        // Second bracket pair is disabled
        assert_eq!(
            string_config.brackets().map(|e| e.1).collect::<Vec<_>>(),
            &[true, false]
        );

        // In between JSX tags: use the `element` override.
        let element_config = snapshot
            .language_scope_at(text.find("<F>").unwrap())
            .unwrap();
        // TODO nested blocks after newlines are captured with all whitespaces
        // https://github.com/tree-sitter/tree-sitter-typescript/issues/306
        // assert_eq!(element_config.line_comment_prefixes(), &[]);
        // assert_eq!(
        //     element_config.block_comment_delimiters(),
        //     Some((&"{/*".into(), &"*/}".into()))
        // );
        assert_eq!(
            element_config.brackets().map(|e| e.1).collect::<Vec<_>>(),
            &[true, true]
        );

        // Within a JSX tag: use the default config.
        let tag_config = snapshot
            .language_scope_at(text.find(" d=").unwrap() + 1)
            .unwrap();
        assert_eq!(tag_config.line_comment_prefixes(), &[Arc::from("// ")]);
        assert_eq!(
            tag_config.block_comment(),
            Some(&BlockCommentConfig {
                start: "/*".into(),
                prefix: "* ".into(),
                end: "*/".into(),
                tab_size: 1,
            })
        );
        assert_eq!(
            tag_config.brackets().map(|e| e.1).collect::<Vec<_>>(),
            &[true, true]
        );

        // In a JSX expression: use the default config.
        let expression_in_element_config = snapshot
            .language_scope_at(text.find('{').unwrap() + 1)
            .unwrap();
        assert_eq!(
            expression_in_element_config.line_comment_prefixes(),
            &[Arc::from("// ")]
        );
        assert_eq!(
            expression_in_element_config.block_comment(),
            Some(&BlockCommentConfig {
                start: "/*".into(),
                prefix: "* ".into(),
                end: "*/".into(),
                tab_size: 1,
            })
        );
        assert_eq!(
            expression_in_element_config
                .brackets()
                .map(|e| e.1)
                .collect::<Vec<_>>(),
            &[true, true]
        );

        buffer
    });
}

#[gpui::test]
fn test_language_scope_at_with_rust(cx: &mut App) {
    init_settings(cx, |_| {});

    cx.new(|cx| {
        let language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                brackets: BracketPairConfig {
                    pairs: vec![
                        BracketPair {
                            start: "{".into(),
                            end: "}".into(),
                            close: true,
                            surround: true,
                            newline: false,
                        },
                        BracketPair {
                            start: "'".into(),
                            end: "'".into(),
                            close: true,
                            surround: true,
                            newline: false,
                        },
                    ],
                    disabled_scopes_by_bracket_ix: vec![
                        Vec::new(), //
                        vec!["string".into()],
                    ],
                },
                ..Default::default()
            },
            Some(tree_sitter_rust::LANGUAGE.into()),
        )
        .with_override_query(
            r#"
                (string_literal) @string
            "#,
        )
        .unwrap();

        let text = r#"
            const S: &'static str = "hello";
        "#
        .unindent();

        let buffer = Buffer::local(text.clone(), cx).with_language(Arc::new(language), cx);
        let snapshot = buffer.snapshot();

        // By default, all brackets are enabled
        let config = snapshot.language_scope_at(0).unwrap();
        assert_eq!(
            config.brackets().map(|e| e.1).collect::<Vec<_>>(),
            &[true, true]
        );

        // Within a string, the quotation brackets are disabled.
        let string_config = snapshot
            .language_scope_at(text.find("ello").unwrap())
            .unwrap();
        assert_eq!(
            string_config.brackets().map(|e| e.1).collect::<Vec<_>>(),
            &[true, false]
        );

        buffer
    });
}

#[gpui::test]
fn test_language_scope_at_with_combined_injections(cx: &mut App) {
    init_settings(cx, |_| {});

    cx.new(|cx| {
        let text = r#"
            <ol>
            <% people.each do |person| %>
                <li>
                    <%= person.name %>
                </li>
            <% end %>
            </ol>
        "#
        .unindent();

        let language_registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
        language_registry.add(Arc::new(ruby_lang()));
        language_registry.add(Arc::new(html_lang()));
        language_registry.add(Arc::new(erb_lang()));

        let mut buffer = Buffer::local(text, cx);
        buffer.set_language_registry(language_registry.clone());
        buffer.set_language(
            language_registry
                .language_for_name("HTML+ERB")
                .now_or_never()
                .unwrap()
                .ok(),
            cx,
        );

        let snapshot = buffer.snapshot();
        let html_config = snapshot.language_scope_at(Point::new(2, 4)).unwrap();
        assert_eq!(html_config.line_comment_prefixes(), &[]);
        assert_eq!(
            html_config.block_comment(),
            Some(&BlockCommentConfig {
                start: "<!--".into(),
                end: "-->".into(),
                prefix: "".into(),
                tab_size: 0,
            })
        );

        let ruby_config = snapshot.language_scope_at(Point::new(3, 12)).unwrap();
        assert_eq!(ruby_config.line_comment_prefixes(), &[Arc::from("# ")]);
        assert_eq!(ruby_config.block_comment(), None);

        buffer
    });
}

#[gpui::test]
fn test_language_at_with_hidden_languages(cx: &mut App) {
    init_settings(cx, |_| {});

    cx.new(|cx| {
        let text = r#"
            this is an *emphasized* word.
        "#
        .unindent();

        let language_registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
        language_registry.add(Arc::new(markdown_lang()));
        language_registry.add(Arc::new(markdown_inline_lang()));

        let mut buffer = Buffer::local(text, cx);
        buffer.set_language_registry(language_registry.clone());
        buffer.set_language(
            language_registry
                .language_for_name("Markdown")
                .now_or_never()
                .unwrap()
                .ok(),
            cx,
        );

        let snapshot = buffer.snapshot();

        for point in [Point::new(0, 4), Point::new(0, 16)] {
            let config = snapshot.language_scope_at(point).unwrap();
            assert_eq!(config.language_name(), "Markdown".into());

            let language = snapshot.language_at(point).unwrap();
            assert_eq!(language.name().as_ref(), "Markdown");
        }

        buffer
    });
}

#[gpui::test]
fn test_language_at_for_markdown_code_block(cx: &mut App) {
    init_settings(cx, |_| {});

    cx.new(|cx| {
        let text = r#"
            ```rs
            let a = 2;
            // let b = 3;
            ```
        "#
        .unindent();

        let language_registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
        language_registry.add(Arc::new(markdown_lang()));
        language_registry.add(Arc::new(markdown_inline_lang()));
        language_registry.add(Arc::new(rust_lang()));

        let mut buffer = Buffer::local(text, cx);
        buffer.set_language_registry(language_registry.clone());
        buffer.set_language(
            language_registry
                .language_for_name("Markdown")
                .now_or_never()
                .unwrap()
                .ok(),
            cx,
        );

        let snapshot = buffer.snapshot();

        // Test points in the code line
        for point in [Point::new(1, 4), Point::new(1, 6)] {
            let config = snapshot.language_scope_at(point).unwrap();
            assert_eq!(config.language_name(), "Rust".into());

            let language = snapshot.language_at(point).unwrap();
            assert_eq!(language.name().as_ref(), "Rust");
        }

        // Test points in the comment line to verify it's still detected as Rust
        for point in [Point::new(2, 4), Point::new(2, 6)] {
            let config = snapshot.language_scope_at(point).unwrap();
            assert_eq!(config.language_name(), "Rust".into());

            let language = snapshot.language_at(point).unwrap();
            assert_eq!(language.name().as_ref(), "Rust");
        }

        buffer
    });
}

#[gpui::test]
fn test_syntax_layer_at_for_injected_languages(cx: &mut App) {
    init_settings(cx, |_| {});

    cx.new(|cx| {
        let text = r#"
            ```html+erb
            <div>Hello</div>
            <%= link_to "Some", "https://zed.dev" %>
            ```
        "#
        .unindent();

        let language_registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
        language_registry.add(Arc::new(erb_lang()));
        language_registry.add(Arc::new(html_lang()));
        language_registry.add(Arc::new(ruby_lang()));

        let mut buffer = Buffer::local(text, cx);
        buffer.set_language_registry(language_registry.clone());
        buffer.set_language(
            language_registry
                .language_for_name("HTML+ERB")
                .now_or_never()
                .unwrap()
                .ok(),
            cx,
        );

        let snapshot = buffer.snapshot();

        // Test points in the code line
        let html_point = Point::new(1, 4);
        let language = snapshot.language_at(html_point).unwrap();
        assert_eq!(language.name().as_ref(), "HTML");

        let ruby_point = Point::new(2, 6);
        let language = snapshot.language_at(ruby_point).unwrap();
        assert_eq!(language.name().as_ref(), "Ruby");

        buffer
    });
}

#[gpui::test]
fn test_serialization(cx: &mut gpui::App) {
    let mut now = Instant::now();

    let buffer1 = cx.new(|cx| {
        let mut buffer = Buffer::local("abc", cx);
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

    let state = buffer1.read(cx).to_proto(cx);
    let ops = cx
        .background_executor()
        .block(buffer1.read(cx).serialize_ops(None, cx));
    let buffer2 = cx.new(|cx| {
        let mut buffer =
            Buffer::from_proto(ReplicaId::new(1), Capability::ReadWrite, state, None).unwrap();
        buffer.apply_ops(
            ops.into_iter()
                .map(|op| proto::deserialize_operation(op).unwrap()),
            cx,
        );
        buffer
    });
    assert_eq!(buffer2.read(cx).text(), "abcDF");
}

#[gpui::test]
fn test_branch_and_merge(cx: &mut TestAppContext) {
    cx.update(|cx| init_settings(cx, |_| {}));

    let base = cx.new(|cx| Buffer::local("one\ntwo\nthree\n", cx));

    // Create a remote replica of the base buffer.
    let base_replica = cx.new(|cx| {
        Buffer::from_proto(
            ReplicaId::new(1),
            Capability::ReadWrite,
            base.read(cx).to_proto(cx),
            None,
        )
        .unwrap()
    });
    base.update(cx, |_buffer, cx| {
        cx.subscribe(&base_replica, |this, _, event, cx| {
            if let BufferEvent::Operation {
                operation,
                is_local: true,
            } = event
            {
                this.apply_ops([operation.clone()], cx);
            }
        })
        .detach();
    });

    // Create a branch, which initially has the same state as the base buffer.
    let branch = base.update(cx, |buffer, cx| buffer.branch(cx));
    branch.read_with(cx, |buffer, _| {
        assert_eq!(buffer.text(), "one\ntwo\nthree\n");
    });

    // Edits to the branch are not applied to the base.
    branch.update(cx, |buffer, cx| {
        buffer.edit(
            [
                (Point::new(1, 0)..Point::new(1, 0), "1.5\n"),
                (Point::new(2, 0)..Point::new(2, 5), "THREE"),
            ],
            None,
            cx,
        )
    });
    branch.read_with(cx, |buffer, cx| {
        assert_eq!(base.read(cx).text(), "one\ntwo\nthree\n");
        assert_eq!(buffer.text(), "one\n1.5\ntwo\nTHREE\n");
    });

    // Convert from branch buffer ranges to the corresponding ranges in the
    // base buffer.
    branch.read_with(cx, |buffer, cx| {
        assert_eq!(
            buffer.range_to_version(4..7, &base.read(cx).version()),
            4..4
        );
        assert_eq!(
            buffer.range_to_version(2..9, &base.read(cx).version()),
            2..5
        );
    });

    // Edits to the base are applied to the branch.
    base.update(cx, |buffer, cx| {
        buffer.edit([(Point::new(0, 0)..Point::new(0, 0), "ZERO\n")], None, cx)
    });
    branch.read_with(cx, |buffer, cx| {
        assert_eq!(base.read(cx).text(), "ZERO\none\ntwo\nthree\n");
        assert_eq!(buffer.text(), "ZERO\none\n1.5\ntwo\nTHREE\n");
    });

    // Edits to any replica of the base are applied to the branch.
    base_replica.update(cx, |buffer, cx| {
        buffer.edit([(Point::new(2, 0)..Point::new(2, 0), "2.5\n")], None, cx)
    });
    branch.read_with(cx, |buffer, cx| {
        assert_eq!(base.read(cx).text(), "ZERO\none\ntwo\n2.5\nthree\n");
        assert_eq!(buffer.text(), "ZERO\none\n1.5\ntwo\n2.5\nTHREE\n");
    });

    // Merging the branch applies all of its changes to the base.
    branch.update(cx, |buffer, cx| {
        buffer.merge_into_base(Vec::new(), cx);
    });

    branch.update(cx, |buffer, cx| {
        assert_eq!(base.read(cx).text(), "ZERO\none\n1.5\ntwo\n2.5\nTHREE\n");
        assert_eq!(buffer.text(), "ZERO\none\n1.5\ntwo\n2.5\nTHREE\n");
    });
}

#[gpui::test]
fn test_merge_into_base(cx: &mut TestAppContext) {
    cx.update(|cx| init_settings(cx, |_| {}));

    let base = cx.new(|cx| Buffer::local("abcdefghijk", cx));
    let branch = base.update(cx, |buffer, cx| buffer.branch(cx));

    // Make 3 edits, merge one into the base.
    branch.update(cx, |branch, cx| {
        branch.edit([(0..3, "ABC"), (7..9, "HI"), (11..11, "LMN")], None, cx);
        branch.merge_into_base(vec![5..8], cx);
    });

    branch.read_with(cx, |branch, _| assert_eq!(branch.text(), "ABCdefgHIjkLMN"));
    base.read_with(cx, |base, _| assert_eq!(base.text(), "abcdefgHIjk"));

    // Undo the one already-merged edit. Merge that into the base.
    branch.update(cx, |branch, cx| {
        branch.edit([(7..9, "hi")], None, cx);
        branch.merge_into_base(vec![5..8], cx);
    });
    base.read_with(cx, |base, _| assert_eq!(base.text(), "abcdefghijk"));

    // Merge an insertion into the base.
    branch.update(cx, |branch, cx| {
        branch.merge_into_base(vec![11..11], cx);
    });

    branch.read_with(cx, |branch, _| assert_eq!(branch.text(), "ABCdefghijkLMN"));
    base.read_with(cx, |base, _| assert_eq!(base.text(), "abcdefghijkLMN"));

    // Deleted the inserted text and merge that into the base.
    branch.update(cx, |branch, cx| {
        branch.edit([(11..14, "")], None, cx);
        branch.merge_into_base(vec![10..11], cx);
    });

    base.read_with(cx, |base, _| assert_eq!(base.text(), "abcdefghijk"));
}

#[gpui::test]
fn test_undo_after_merge_into_base(cx: &mut TestAppContext) {
    cx.update(|cx| init_settings(cx, |_| {}));

    let base = cx.new(|cx| Buffer::local("abcdefghijk", cx));
    let branch = base.update(cx, |buffer, cx| buffer.branch(cx));

    // Make 2 edits, merge one into the base.
    branch.update(cx, |branch, cx| {
        branch.edit([(0..3, "ABC"), (7..9, "HI")], None, cx);
        branch.merge_into_base(vec![7..7], cx);
    });
    base.read_with(cx, |base, _| assert_eq!(base.text(), "abcdefgHIjk"));
    branch.read_with(cx, |branch, _| assert_eq!(branch.text(), "ABCdefgHIjk"));

    // Undo the merge in the base buffer.
    base.update(cx, |base, cx| {
        base.undo(cx);
    });
    base.read_with(cx, |base, _| assert_eq!(base.text(), "abcdefghijk"));
    branch.read_with(cx, |branch, _| assert_eq!(branch.text(), "ABCdefgHIjk"));

    // Merge that operation into the base again.
    branch.update(cx, |branch, cx| {
        branch.merge_into_base(vec![7..7], cx);
    });
    base.read_with(cx, |base, _| assert_eq!(base.text(), "abcdefgHIjk"));
    branch.read_with(cx, |branch, _| assert_eq!(branch.text(), "ABCdefgHIjk"));
}

#[gpui::test]
async fn test_preview_edits(cx: &mut TestAppContext) {
    cx.update(|cx| {
        init_settings(cx, |_| {});
        theme::init(theme::LoadThemes::JustBase, cx);
    });

    let insertion_style = HighlightStyle {
        background_color: Some(cx.read(|cx| cx.theme().status().created_background)),
        ..Default::default()
    };
    let deletion_style = HighlightStyle {
        background_color: Some(cx.read(|cx| cx.theme().status().deleted_background)),
        ..Default::default()
    };

    // no edits
    assert_preview_edits(
        indoc! {"
        fn test_empty() -> bool {
            false
        }"
        },
        vec![],
        true,
        cx,
        |hl| {
            assert!(hl.text.is_empty());
            assert!(hl.highlights.is_empty());
        },
    )
    .await;

    // only insertions
    assert_preview_edits(
        indoc! {"
        fn calculate_area(: f64) -> f64 {
            std::f64::consts::PI * .powi(2)
        }"
        },
        vec![
            (Point::new(0, 18)..Point::new(0, 18), "radius"),
            (Point::new(1, 27)..Point::new(1, 27), "radius"),
        ],
        true,
        cx,
        |hl| {
            assert_eq!(
                hl.text,
                indoc! {"
                fn calculate_area(radius: f64) -> f64 {
                    std::f64::consts::PI * radius.powi(2)"
                }
            );

            assert_eq!(hl.highlights.len(), 2);
            assert_eq!(hl.highlights[0], ((18..24), insertion_style));
            assert_eq!(hl.highlights[1], ((67..73), insertion_style));
        },
    )
    .await;

    // insertions & deletions
    assert_preview_edits(
        indoc! {"
        struct Person {
            first_name: String,
        }

        impl Person {
            fn first_name(&self) -> &String {
                &self.first_name
            }
        }"
        },
        vec![
            (Point::new(1, 4)..Point::new(1, 9), "last"),
            (Point::new(5, 7)..Point::new(5, 12), "last"),
            (Point::new(6, 14)..Point::new(6, 19), "last"),
        ],
        true,
        cx,
        |hl| {
            assert_eq!(
                hl.text,
                indoc! {"
                        firstlast_name: String,
                    }

                    impl Person {
                        fn firstlast_name(&self) -> &String {
                            &self.firstlast_name"
                }
            );

            assert_eq!(hl.highlights.len(), 6);
            assert_eq!(hl.highlights[0], ((4..9), deletion_style));
            assert_eq!(hl.highlights[1], ((9..13), insertion_style));
            assert_eq!(hl.highlights[2], ((52..57), deletion_style));
            assert_eq!(hl.highlights[3], ((57..61), insertion_style));
            assert_eq!(hl.highlights[4], ((101..106), deletion_style));
            assert_eq!(hl.highlights[5], ((106..110), insertion_style));
        },
    )
    .await;

    async fn assert_preview_edits(
        text: &str,
        edits: Vec<(Range<Point>, &str)>,
        include_deletions: bool,
        cx: &mut TestAppContext,
        assert_fn: impl Fn(HighlightedText),
    ) {
        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx));
        let edits = buffer.read_with(cx, |buffer, _| {
            edits
                .into_iter()
                .map(|(range, text)| {
                    (
                        buffer.anchor_before(range.start)..buffer.anchor_after(range.end),
                        text.into(),
                    )
                })
                .collect::<Arc<[_]>>()
        });
        let edit_preview = buffer
            .read_with(cx, |buffer, cx| buffer.preview_edits(edits.clone(), cx))
            .await;
        let highlighted_edits = cx.read(|cx| {
            edit_preview.highlight_edits(&buffer.read(cx).snapshot(), &edits, include_deletions, cx)
        });
        assert_fn(highlighted_edits);
    }
}

#[gpui::test(iterations = 100)]
fn test_random_collaboration(cx: &mut App, mut rng: StdRng) {
    let min_peers = env::var("MIN_PEERS")
        .map(|i| i.parse().expect("invalid `MIN_PEERS` variable"))
        .unwrap_or(1);
    let max_peers = env::var("MAX_PEERS")
        .map(|i| i.parse().expect("invalid `MAX_PEERS` variable"))
        .unwrap_or(5);
    let operations = env::var("OPERATIONS")
        .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
        .unwrap_or(10);

    let base_text_len = rng.random_range(0..10);
    let base_text = RandomCharIter::new(&mut rng)
        .take(base_text_len)
        .collect::<String>();
    let mut replica_ids = Vec::new();
    let mut buffers = Vec::new();
    let network = Arc::new(Mutex::new(Network::new(rng.clone())));
    let base_buffer = cx.new(|cx| Buffer::local(base_text.as_str(), cx));

    for i in 0..rng.random_range(min_peers..=max_peers) {
        let buffer = cx.new(|cx| {
            let state = base_buffer.read(cx).to_proto(cx);
            let ops = cx
                .background_executor()
                .block(base_buffer.read(cx).serialize_ops(None, cx));
            let mut buffer =
                Buffer::from_proto(ReplicaId::new(i as u16), Capability::ReadWrite, state, None)
                    .unwrap();
            buffer.apply_ops(
                ops.into_iter()
                    .map(|op| proto::deserialize_operation(op).unwrap()),
                cx,
            );
            buffer.set_group_interval(Duration::from_millis(rng.random_range(0..=200)));
            let network = network.clone();
            cx.subscribe(&cx.entity(), move |buffer, _, event, _| {
                if let BufferEvent::Operation {
                    operation,
                    is_local: true,
                } = event
                {
                    network.lock().broadcast(
                        buffer.replica_id(),
                        vec![proto::serialize_operation(operation)],
                    );
                }
            })
            .detach();
            buffer
        });

        buffers.push(buffer);
        replica_ids.push(ReplicaId::new(i as u16));
        network.lock().add_peer(ReplicaId::new(i as u16));
        log::info!("Adding initial peer with replica id {:?}", replica_ids[i]);
    }

    log::info!("initial text: {:?}", base_text);

    let mut now = Instant::now();
    let mut mutation_count = operations;
    let mut next_diagnostic_id = 0;
    let mut active_selections = BTreeMap::default();
    loop {
        let replica_index = rng.random_range(0..replica_ids.len());
        let replica_id = replica_ids[replica_index];
        let buffer = &mut buffers[replica_index];
        let mut new_buffer = None;
        match rng.random_range(0..100) {
            0..=29 if mutation_count != 0 => {
                buffer.update(cx, |buffer, cx| {
                    buffer.start_transaction_at(now);
                    buffer.randomly_edit(&mut rng, 5, cx);
                    buffer.end_transaction_at(now, cx);
                    log::info!("buffer {:?} text: {:?}", buffer.replica_id(), buffer.text());
                });
                mutation_count -= 1;
            }
            30..=39 if mutation_count != 0 => {
                buffer.update(cx, |buffer, cx| {
                    if rng.random_bool(0.2) {
                        log::info!("peer {:?} clearing active selections", replica_id);
                        active_selections.remove(&replica_id);
                        buffer.remove_active_selections(cx);
                    } else {
                        let mut selections = Vec::new();
                        for id in 0..rng.random_range(1..=5) {
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
                            "peer {:?} setting active selections: {:?}",
                            replica_id,
                            selections
                        );
                        active_selections.insert(replica_id, selections.clone());
                        buffer.set_active_selections(selections, false, Default::default(), cx);
                    }
                });
                mutation_count -= 1;
            }
            40..=49 if mutation_count != 0 && replica_id == ReplicaId::REMOTE_SERVER => {
                let entry_count = rng.random_range(1..=5);
                buffer.update(cx, |buffer, cx| {
                    let diagnostics = DiagnosticSet::new(
                        (0..entry_count).map(|_| {
                            let range = buffer.random_byte_range(0, &mut rng);
                            let range = range.to_point_utf16(buffer);
                            let range = range.start..range.end;
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
                    log::info!(
                        "peer {:?} setting diagnostics: {:?}",
                        replica_id,
                        diagnostics
                    );
                    buffer.update_diagnostics(LanguageServerId(0), diagnostics, cx);
                });
                mutation_count -= 1;
            }
            50..=59 if replica_ids.len() < max_peers => {
                let old_buffer_state = buffer.read(cx).to_proto(cx);
                let old_buffer_ops = cx
                    .background_executor()
                    .block(buffer.read(cx).serialize_ops(None, cx));
                let new_replica_id = (0..=replica_ids.len() as u16)
                    .map(ReplicaId::new)
                    .filter(|replica_id| *replica_id != buffer.read(cx).replica_id())
                    .choose(&mut rng)
                    .unwrap();
                log::info!(
                    "Adding new replica {:?} (replicating from {:?})",
                    new_replica_id,
                    replica_id
                );
                new_buffer = Some(cx.new(|cx| {
                    let mut new_buffer = Buffer::from_proto(
                        new_replica_id,
                        Capability::ReadWrite,
                        old_buffer_state,
                        None,
                    )
                    .unwrap();
                    new_buffer.apply_ops(
                        old_buffer_ops
                            .into_iter()
                            .map(|op| deserialize_operation(op).unwrap()),
                        cx,
                    );
                    log::info!(
                        "New replica {:?} text: {:?}",
                        new_buffer.replica_id(),
                        new_buffer.text()
                    );
                    new_buffer.set_group_interval(Duration::from_millis(rng.random_range(0..=200)));
                    let network = network.clone();
                    cx.subscribe(&cx.entity(), move |buffer, _, event, _| {
                        if let BufferEvent::Operation {
                            operation,
                            is_local: true,
                        } = event
                        {
                            network.lock().broadcast(
                                buffer.replica_id(),
                                vec![proto::serialize_operation(operation)],
                            );
                        }
                    })
                    .detach();
                    new_buffer
                }));
                network.lock().replicate(replica_id, new_replica_id);

                if new_replica_id.as_u16() as usize == replica_ids.len() {
                    replica_ids.push(new_replica_id);
                } else {
                    let new_buffer = new_buffer.take().unwrap();
                    while network.lock().has_unreceived(new_replica_id) {
                        let ops = network
                            .lock()
                            .receive(new_replica_id)
                            .into_iter()
                            .map(|op| proto::deserialize_operation(op).unwrap());
                        if ops.len() > 0 {
                            log::info!(
                                "peer {:?} (version: {:?}) applying {} ops from the network. {:?}",
                                new_replica_id,
                                buffer.read(cx).version(),
                                ops.len(),
                                ops
                            );
                            new_buffer.update(cx, |new_buffer, cx| {
                                new_buffer.apply_ops(ops, cx);
                            });
                        }
                    }
                    buffers[new_replica_id.as_u16() as usize] = new_buffer;
                }
            }
            60..=69 if mutation_count != 0 => {
                buffer.update(cx, |buffer, cx| {
                    buffer.randomly_undo_redo(&mut rng, cx);
                    log::info!("buffer {:?} text: {:?}", buffer.replica_id(), buffer.text());
                });
                mutation_count -= 1;
            }
            _ if network.lock().has_unreceived(replica_id) => {
                let ops = network
                    .lock()
                    .receive(replica_id)
                    .into_iter()
                    .map(|op| proto::deserialize_operation(op).unwrap());
                if ops.len() > 0 {
                    log::info!(
                        "peer {:?} (version: {:?}) applying {} ops from the network. {:?}",
                        replica_id,
                        buffer.read(cx).version(),
                        ops.len(),
                        ops
                    );
                    buffer.update(cx, |buffer, cx| buffer.apply_ops(ops, cx));
                }
            }
            _ => {}
        }

        now += Duration::from_millis(rng.random_range(0..=200));
        buffers.extend(new_buffer);

        for buffer in &buffers {
            buffer.read(cx).check_invariants();
        }

        if mutation_count == 0 && network.lock().is_idle() {
            break;
        }
    }

    let first_buffer = buffers[0].read(cx).snapshot();
    for buffer in &buffers[1..] {
        let buffer = buffer.read(cx).snapshot();
        assert_eq!(
            buffer.version(),
            first_buffer.version(),
            "Replica {:?} version != Replica 0 version",
            buffer.replica_id()
        );
        assert_eq!(
            buffer.text(),
            first_buffer.text(),
            "Replica {:?} text != Replica 0 text",
            buffer.replica_id()
        );
        assert_eq!(
            buffer
                .diagnostics_in_range::<_, usize>(0..buffer.len(), false)
                .collect::<Vec<_>>(),
            first_buffer
                .diagnostics_in_range::<_, usize>(0..first_buffer.len(), false)
                .collect::<Vec<_>>(),
            "Replica {:?} diagnostics != Replica 0 diagnostics",
            buffer.replica_id()
        );
    }

    for buffer in &buffers {
        let buffer = buffer.read(cx).snapshot();
        let actual_remote_selections = buffer
            .selections_in_range(Anchor::min_max_range_for_buffer(buffer.remote_id()), false)
            .map(|(replica_id, _, _, selections)| (replica_id, selections.collect::<Vec<_>>()))
            .collect::<Vec<_>>();
        let expected_remote_selections = active_selections
            .iter()
            .filter(|(replica_id, _)| **replica_id != buffer.replica_id())
            .map(|(replica_id, selections)| (*replica_id, selections.iter().collect::<Vec<_>>()))
            .collect::<Vec<_>>();
        assert_eq!(
            actual_remote_selections,
            expected_remote_selections,
            "Replica {:?} remote selections != expected selections",
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

#[gpui::test]
fn test_insertion_after_deletion(cx: &mut gpui::App) {
    let buffer = cx.new(|cx| Buffer::local("struct Foo {\n    \n}", cx));
    buffer.update(cx, |buffer, cx| {
        let mut anchor = buffer.anchor_after(17);
        buffer.edit([(12..18, "")], None, cx);
        let snapshot = buffer.snapshot();
        assert_eq!(snapshot.text(), "struct Foo {}");
        if !anchor.is_valid(&snapshot) {
            anchor = snapshot.anchor_after(snapshot.offset_for_anchor(&anchor));
        }
        buffer.edit([(anchor..anchor, "\n")], None, cx);
        buffer.edit([(anchor..anchor, "field1:")], None, cx);
        buffer.edit([(anchor..anchor, " i32,")], None, cx);
        let snapshot = buffer.snapshot();
        assert_eq!(snapshot.text(), "struct Foo {\nfield1: i32,}");
    })
}

#[gpui::test(iterations = 500)]
fn test_trailing_whitespace_ranges(mut rng: StdRng) {
    // Generate a random multi-line string containing
    // some lines with trailing whitespace.
    let mut text = String::new();
    for _ in 0..rng.random_range(0..16) {
        for _ in 0..rng.random_range(0..36) {
            text.push(match rng.random_range(0..10) {
                0..=1 => ' ',
                3 => '\t',
                _ => rng.random_range('a'..='z'),
            });
        }
        text.push('\n');
    }

    match rng.random_range(0..10) {
        // sometimes remove the last newline
        0..=1 => drop(text.pop()), //

        // sometimes add extra newlines
        2..=3 => text.push_str(&"\n".repeat(rng.random_range(1..5))),
        _ => {}
    }

    let rope = Rope::from(text.as_str());
    let actual_ranges = trailing_whitespace_ranges(&rope);
    let expected_ranges = TRAILING_WHITESPACE_REGEX
        .find_iter(&text)
        .map(|m| m.range())
        .collect::<Vec<_>>();
    assert_eq!(
        actual_ranges,
        expected_ranges,
        "wrong ranges for text lines:\n{:?}",
        text.split('\n').collect::<Vec<_>>()
    );
}

#[gpui::test]
fn test_words_in_range(cx: &mut gpui::App) {
    init_settings(cx, |_| {});

    // The first line are words excluded from the results with heuristics, we do not expect them in the test assertions.
    let contents = r#"
0_isize 123 3.4 4  
let word=öäpple.bar你 Öäpple word2-öÄpPlE-Pizza-word ÖÄPPLE word
    "#;

    let buffer = cx.new(|cx| {
        let buffer = Buffer::local(contents, cx).with_language(Arc::new(rust_lang()), cx);
        assert_eq!(buffer.text(), contents);
        buffer.check_invariants();
        buffer
    });

    buffer.update(cx, |buffer, _| {
        let snapshot = buffer.snapshot();
        assert_eq!(
            BTreeSet::from_iter(["Pizza".to_string()]),
            snapshot
                .words_in_range(WordsQuery {
                    fuzzy_contents: Some("piz"),
                    skip_digits: true,
                    range: 0..snapshot.len(),
                })
                .into_keys()
                .collect::<BTreeSet<_>>()
        );
        assert_eq!(
            BTreeSet::from_iter([
                "öäpple".to_string(),
                "Öäpple".to_string(),
                "öÄpPlE".to_string(),
                "ÖÄPPLE".to_string(),
            ]),
            snapshot
                .words_in_range(WordsQuery {
                    fuzzy_contents: Some("öp"),
                    skip_digits: true,
                    range: 0..snapshot.len(),
                })
                .into_keys()
                .collect::<BTreeSet<_>>()
        );
        assert_eq!(
            BTreeSet::from_iter([
                "öÄpPlE".to_string(),
                "Öäpple".to_string(),
                "ÖÄPPLE".to_string(),
                "öäpple".to_string(),
            ]),
            snapshot
                .words_in_range(WordsQuery {
                    fuzzy_contents: Some("öÄ"),
                    skip_digits: true,
                    range: 0..snapshot.len(),
                })
                .into_keys()
                .collect::<BTreeSet<_>>()
        );
        assert_eq!(
            BTreeSet::default(),
            snapshot
                .words_in_range(WordsQuery {
                    fuzzy_contents: Some("öÄ好"),
                    skip_digits: true,
                    range: 0..snapshot.len(),
                })
                .into_keys()
                .collect::<BTreeSet<_>>()
        );
        assert_eq!(
            BTreeSet::from_iter(["bar你".to_string(),]),
            snapshot
                .words_in_range(WordsQuery {
                    fuzzy_contents: Some("你"),
                    skip_digits: true,
                    range: 0..snapshot.len(),
                })
                .into_keys()
                .collect::<BTreeSet<_>>()
        );
        assert_eq!(
            BTreeSet::default(),
            snapshot
                .words_in_range(WordsQuery {
                    fuzzy_contents: Some(""),
                    skip_digits: true,
                    range: 0..snapshot.len(),
                },)
                .into_keys()
                .collect::<BTreeSet<_>>()
        );
        assert_eq!(
            BTreeSet::from_iter([
                "bar你".to_string(),
                "öÄpPlE".to_string(),
                "Öäpple".to_string(),
                "ÖÄPPLE".to_string(),
                "öäpple".to_string(),
                "let".to_string(),
                "Pizza".to_string(),
                "word".to_string(),
                "word2".to_string(),
            ]),
            snapshot
                .words_in_range(WordsQuery {
                    fuzzy_contents: None,
                    skip_digits: true,
                    range: 0..snapshot.len(),
                })
                .into_keys()
                .collect::<BTreeSet<_>>()
        );
        assert_eq!(
            BTreeSet::from_iter([
                "0_isize".to_string(),
                "123".to_string(),
                "3".to_string(),
                "4".to_string(),
                "bar你".to_string(),
                "öÄpPlE".to_string(),
                "Öäpple".to_string(),
                "ÖÄPPLE".to_string(),
                "öäpple".to_string(),
                "let".to_string(),
                "Pizza".to_string(),
                "word".to_string(),
                "word2".to_string(),
            ]),
            snapshot
                .words_in_range(WordsQuery {
                    fuzzy_contents: None,
                    skip_digits: false,
                    range: 0..snapshot.len(),
                })
                .into_keys()
                .collect::<BTreeSet<_>>()
        );
    });
}

fn ruby_lang() -> Language {
    Language::new(
        LanguageConfig {
            name: "Ruby".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["rb".to_string()],
                ..Default::default()
            },
            line_comments: vec!["# ".into()],
            ..Default::default()
        },
        Some(tree_sitter_ruby::LANGUAGE.into()),
    )
    .with_indents_query(
        r#"
            (class "end" @end) @indent
            (method "end" @end) @indent
            (rescue) @outdent
            (then) @indent
        "#,
    )
    .unwrap()
}

fn html_lang() -> Language {
    Language::new(
        LanguageConfig {
            name: LanguageName::new("HTML"),
            block_comment: Some(BlockCommentConfig {
                start: "<!--".into(),
                prefix: "".into(),
                end: "-->".into(),
                tab_size: 0,
            }),
            ..Default::default()
        },
        Some(tree_sitter_html::LANGUAGE.into()),
    )
    .with_indents_query(
        "
        (element
          (start_tag) @start
          (end_tag)? @end) @indent
        ",
    )
    .unwrap()
    .with_injection_query(
        r#"
        (script_element
            (raw_text) @injection.content
            (#set! injection.language "javascript"))
        "#,
    )
    .unwrap()
}

fn erb_lang() -> Language {
    Language::new(
        LanguageConfig {
            name: "HTML+ERB".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["erb".to_string()],
                ..Default::default()
            },
            block_comment: Some(BlockCommentConfig {
                start: "<%#".into(),
                prefix: "".into(),
                end: "%>".into(),
                tab_size: 0,
            }),
            ..Default::default()
        },
        Some(tree_sitter_embedded_template::LANGUAGE.into()),
    )
    .with_injection_query(
        r#"
            (
                (code) @content
                (#set! "language" "ruby")
                (#set! "combined")
            )

            (
                (content) @content
                (#set! "language" "html")
                (#set! "combined")
            )
        "#,
    )
    .unwrap()
}

fn rust_lang() -> Language {
    Language::new(
        LanguageConfig {
            name: "Rust".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
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
    .with_text_object_query(
        r#"
        (function_item
            body: (_
                "{"
                (_)* @function.inside
                "}" )) @function.around

        (line_comment)+ @comment.around

        (block_comment) @comment.around
        "#,
    )
    .unwrap()
    .with_outline_query(
        r#"
        (line_comment) @annotation

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
            type: (_) @name
            body: (_ "{" (_)* "}")) @item
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
            matcher: LanguageMatcher {
                path_suffixes: vec!["js".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_json::LANGUAGE.into()),
    )
}

fn javascript_lang() -> Language {
    Language::new(
        LanguageConfig {
            name: "JavaScript".into(),
            ..Default::default()
        },
        Some(tree_sitter_typescript::LANGUAGE_TSX.into()),
    )
    .with_brackets_query(
        r#"
        ("{" @open "}" @close)
        ("(" @open ")" @close)
        "#,
    )
    .unwrap()
    .with_indents_query(
        r#"
        (object "}" @end) @indent
        "#,
    )
    .unwrap()
}

pub fn markdown_lang() -> Language {
    Language::new(
        LanguageConfig {
            name: "Markdown".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["md".into()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_md::LANGUAGE.into()),
    )
    .with_injection_query(
        r#"
            (fenced_code_block
                (info_string
                    (language) @injection.language)
                (code_fence_content) @injection.content)

                ((inline) @injection.content
                (#set! injection.language "markdown-inline"))
        "#,
    )
    .unwrap()
}

pub fn markdown_inline_lang() -> Language {
    Language::new(
        LanguageConfig {
            name: "Markdown-Inline".into(),
            hidden: true,
            ..LanguageConfig::default()
        },
        Some(tree_sitter_md::INLINE_LANGUAGE.into()),
    )
    .with_highlights_query("(emphasis) @emphasis")
    .unwrap()
}

fn get_tree_sexp(buffer: &Entity<Buffer>, cx: &mut gpui::TestAppContext) -> String {
    buffer.update(cx, |buffer, _| {
        let snapshot = buffer.snapshot();
        let layers = snapshot.syntax.layers(buffer.as_text_snapshot());
        layers[0].node().to_sexp()
    })
}

// Assert that the enclosing bracket ranges around the selection match the pairs indicated by the marked text in `range_markers`
#[track_caller]
fn assert_bracket_pairs(
    selection_text: &'static str,
    bracket_pair_texts: Vec<&'static str>,
    language: Language,
    cx: &mut App,
) {
    let (expected_text, selection_ranges) = marked_text_ranges(selection_text, false);
    let buffer =
        cx.new(|cx| Buffer::local(expected_text.clone(), cx).with_language(Arc::new(language), cx));
    let buffer = buffer.update(cx, |buffer, _cx| buffer.snapshot());

    let selection_range = selection_ranges[0].clone();

    let bracket_pairs = bracket_pair_texts
        .into_iter()
        .map(|pair_text| {
            let (bracket_text, ranges) = marked_text_ranges(pair_text, false);
            assert_eq!(bracket_text, expected_text);
            (ranges[0].clone(), ranges[1].clone())
        })
        .collect::<Vec<_>>();

    assert_set_eq!(
        buffer
            .bracket_ranges(selection_range)
            .map(|pair| (pair.open_range, pair.close_range))
            .collect::<Vec<_>>(),
        bracket_pairs
    );
}

fn init_settings(cx: &mut App, f: fn(&mut AllLanguageSettingsContent)) {
    let settings_store = SettingsStore::test(cx);
    cx.set_global(settings_store);
    cx.update_global::<SettingsStore, _>(|settings, cx| {
        settings.update_user_settings(cx, |content| f(&mut content.project.all_languages));
    });
}

#[gpui::test(iterations = 100)]
fn test_random_chunk_bitmaps(cx: &mut App, mut rng: StdRng) {
    use util::RandomCharIter;

    // Generate random text
    let len = rng.random_range(0..10000);
    let text = RandomCharIter::new(&mut rng).take(len).collect::<String>();

    let buffer = cx.new(|cx| Buffer::local(text, cx));
    let snapshot = buffer.read(cx).snapshot();

    // Get all chunks and verify their bitmaps
    let chunks = snapshot.chunks(0..snapshot.len(), false);

    for chunk in chunks {
        let chunk_text = chunk.text;
        let chars_bitmap = chunk.chars;
        let tabs_bitmap = chunk.tabs;

        // Check empty chunks have empty bitmaps
        if chunk_text.is_empty() {
            assert_eq!(
                chars_bitmap, 0,
                "Empty chunk should have empty chars bitmap"
            );
            assert_eq!(tabs_bitmap, 0, "Empty chunk should have empty tabs bitmap");
            continue;
        }

        // Verify that chunk text doesn't exceed 128 bytes
        assert!(
            chunk_text.len() <= 128,
            "Chunk text length {} exceeds 128 bytes",
            chunk_text.len()
        );

        // Verify chars bitmap
        let char_indices = chunk_text
            .char_indices()
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        for byte_idx in 0..chunk_text.len() {
            let should_have_bit = char_indices.contains(&byte_idx);
            let has_bit = chars_bitmap & (1 << byte_idx) != 0;

            if has_bit != should_have_bit {
                eprintln!("Chunk text bytes: {:?}", chunk_text.as_bytes());
                eprintln!("Char indices: {:?}", char_indices);
                eprintln!("Chars bitmap: {:#b}", chars_bitmap);
            }

            assert_eq!(
                has_bit, should_have_bit,
                "Chars bitmap mismatch at byte index {} in chunk {:?}. Expected bit: {}, Got bit: {}",
                byte_idx, chunk_text, should_have_bit, has_bit
            );
        }

        // Verify tabs bitmap
        for (byte_idx, byte) in chunk_text.bytes().enumerate() {
            let is_tab = byte == b'\t';
            let has_bit = tabs_bitmap & (1 << byte_idx) != 0;

            if has_bit != is_tab {
                eprintln!("Chunk text bytes: {:?}", chunk_text.as_bytes());
                eprintln!("Tabs bitmap: {:#b}", tabs_bitmap);
                assert_eq!(
                    has_bit, is_tab,
                    "Tabs bitmap mismatch at byte index {} in chunk {:?}. Byte: {:?}, Expected bit: {}, Got bit: {}",
                    byte_idx, chunk_text, byte as char, is_tab, has_bit
                );
            }
        }
    }
}
