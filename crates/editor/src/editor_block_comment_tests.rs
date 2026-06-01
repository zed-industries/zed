use crate::ToggleBlockComments;
use crate::editor_tests::init_test;
use crate::test::editor_test_context::EditorTestContext;
use gpui::TestAppContext;
use indoc::indoc;
use language::{BlockCommentConfig, Language, LanguageConfig};
use std::sync::Arc;

async fn setup_rust_context(cx: &mut TestAppContext) -> EditorTestContext {
    init_test(cx, |_| {});
    let mut cx = EditorTestContext::new(cx).await;

    let rust_language = Arc::new(Language::new(
        LanguageConfig {
            name: "Rust".into(),
            block_comment: Some(BlockCommentConfig {
                start: "/* ".into(),
                prefix: "".into(),
                end: " */".into(),
                tab_size: 0,
            }),
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    ));

    cx.language_registry().add(rust_language.clone());
    cx.update_buffer(|buffer, cx| {
        buffer.set_language(Some(rust_language), cx);
    });

    cx
}

async fn setup_python_context(cx: &mut TestAppContext) -> EditorTestContext {
    init_test(cx, |_| {});
    let mut cx = EditorTestContext::new(cx).await;

    let python_language = Arc::new(Language::new(
        LanguageConfig {
            name: "Python".into(),
            block_comment: Some(BlockCommentConfig {
                start: "\"\"\"".into(),
                prefix: "".into(),
                end: "\"\"\"".into(),
                tab_size: 0,
            }),
            ..Default::default()
        },
        None,
    ));

    cx.language_registry().add(python_language.clone());
    cx.update_buffer(|buffer, cx| {
        buffer.set_language(Some(python_language), cx);
    });

    cx
}

#[gpui::test]
async fn test_toggle_block_comments(cx: &mut TestAppContext) {
    let mut cx = setup_rust_context(cx).await;

    cx.set_state(indoc! {"
        fn main() {
            let x = «1ˇ» + 2;
        }
    "});

    cx.update_editor(|editor, window, cx| {
        editor.toggle_block_comments(&ToggleBlockComments, window, cx);
    });

    cx.assert_editor_state(indoc! {"
        fn main() {
            let x = «/* 1 */ˇ» + 2;
        }
    "});

    cx.update_editor(|editor, window, cx| {
        editor.toggle_block_comments(&ToggleBlockComments, window, cx);
    });

    cx.assert_editor_state(indoc! {"
        fn main() {
            let x = «1ˇ» + 2;
        }
    "});
}

#[gpui::test]
async fn test_toggle_block_comments_with_selection(cx: &mut TestAppContext) {
    let mut cx = setup_rust_context(cx).await;

    cx.set_state(indoc! {"
        fn main() {
            «let x = 1 + 2;ˇ»
        }
    "});

    cx.update_editor(|editor, window, cx| {
        editor.toggle_block_comments(&ToggleBlockComments, window, cx);
    });

    cx.assert_editor_state(indoc! {"
        fn main() {
            «/* let x = 1 + 2; */ˇ»
        }
    "});

    cx.update_editor(|editor, window, cx| {
        editor.toggle_block_comments(&ToggleBlockComments, window, cx);
    });

    cx.assert_editor_state(indoc! {"
        fn main() {
            «let x = 1 + 2;ˇ»
        }
    "});
}

#[gpui::test]
async fn test_toggle_block_comments_multiline(cx: &mut TestAppContext) {
    let mut cx = setup_rust_context(cx).await;

    cx.set_state(indoc! {"
        «fn main() {
            let x = 1;
        }ˇ»
    "});

    cx.update_editor(|editor, window, cx| {
        editor.toggle_block_comments(&ToggleBlockComments, window, cx);
    });

    cx.assert_editor_state(indoc! {"
        «/* fn main() {
            let x = 1;
        } */ˇ»
    "});

    cx.update_editor(|editor, window, cx| {
        editor.toggle_block_comments(&ToggleBlockComments, window, cx);
    });

    cx.assert_editor_state(indoc! {"
        «fn main() {
            let x = 1;
        }ˇ»
    "});
}

#[gpui::test]
async fn test_toggle_block_comments_cursor_inside(cx: &mut TestAppContext) {
    let mut cx = setup_rust_context(cx).await;

    cx.set_state(indoc! {"
            fn main() {
                let x = /* 1ˇ */ + 2;
            }
        "});

    cx.update_editor(|editor, window, cx| {
        editor.toggle_block_comments(&ToggleBlockComments, window, cx);
    });

    cx.assert_editor_state(indoc! {"
            fn main() {
                let x = 1ˇ + 2;
            }
        "});
}

#[gpui::test]
async fn test_toggle_block_comments_multiple_cursors(cx: &mut TestAppContext) {
    let mut cx = setup_rust_context(cx).await;

    cx.set_state(indoc! {"
            fn main() {
                let x = «1ˇ» + 2;
                let y = «3ˇ» + 4;
            }
        "});

    cx.update_editor(|editor, window, cx| {
        editor.toggle_block_comments(&ToggleBlockComments, window, cx);
    });

    cx.assert_editor_state(indoc! {"
            fn main() {
                let x = «/* 1 */ˇ» + 2;
                let y = «/* 3 */ˇ» + 4;
            }
        "});

    cx.update_editor(|editor, window, cx| {
        editor.toggle_block_comments(&ToggleBlockComments, window, cx);
    });

    cx.assert_editor_state(indoc! {"
        fn main() {
            let x = «1ˇ» + 2;
            let y = «3ˇ» + 4;
        }
    "});
}

#[gpui::test]
async fn test_toggle_block_comments_selection_ending_on_empty_line(cx: &mut TestAppContext) {
    let mut cx = setup_rust_context(cx).await;

    cx.set_state(indoc! {"
        «fn main() {
        ˇ»
            let x = 1;
        }
    "});

    cx.update_editor(|editor, window, cx| {
        editor.toggle_block_comments(&ToggleBlockComments, window, cx);
    });

    cx.assert_editor_state(indoc! {"
        «/* fn main() {
         */ˇ»
            let x = 1;
        }
    "});

    cx.update_editor(|editor, window, cx| {
        editor.toggle_block_comments(&ToggleBlockComments, window, cx);
    });

    cx.assert_editor_state(indoc! {"
        «fn main() {
        ˇ»
            let x = 1;
        }
    "});
}

#[gpui::test]
async fn test_toggle_block_comments_empty_selection_roundtrip(cx: &mut TestAppContext) {
    let mut cx = setup_rust_context(cx).await;

    cx.set_state(indoc! {"
        fn main() {
            let x = ˇ1 + 2;
        }
    "});

    cx.update_editor(|editor, window, cx| {
        editor.toggle_block_comments(&ToggleBlockComments, window, cx);
    });

    cx.update_editor(|editor, window, cx| {
        editor.toggle_block_comments(&ToggleBlockComments, window, cx);
    });

    cx.assert_editor_state(indoc! {"
        fn main() {
            let x = ˇ1 + 2;
        }
    "});
}

// Multi-byte Unicode characters (√ is 3 bytes in UTF-8) must not cause
// incorrect offset arithmetic or panics.
#[gpui::test]
async fn test_toggle_block_comments_unicode_before_selection(cx: &mut TestAppContext) {
    let mut cx = setup_rust_context(cx).await;

    cx.set_state("let √ = «42ˇ»;");

    cx.update_editor(|editor, window, cx| {
        editor.toggle_block_comments(&ToggleBlockComments, window, cx);
    });

    cx.assert_editor_state("let √ = «/* 42 */ˇ»;");

    cx.update_editor(|editor, window, cx| {
        editor.toggle_block_comments(&ToggleBlockComments, window, cx);
    });

    cx.assert_editor_state("let √ = «42ˇ»;");
}

#[gpui::test]
async fn test_toggle_block_comments_unicode_in_selection(cx: &mut TestAppContext) {
    let mut cx = setup_rust_context(cx).await;

    cx.set_state("«√√√ˇ»");

    cx.update_editor(|editor, window, cx| {
        editor.toggle_block_comments(&ToggleBlockComments, window, cx);
    });

    cx.assert_editor_state("«/* √√√ */ˇ»");

    cx.update_editor(|editor, window, cx| {
        editor.toggle_block_comments(&ToggleBlockComments, window, cx);
    });

    cx.assert_editor_state("«√√√ˇ»");
}

#[gpui::test]
async fn test_toggle_block_comments_line_mode_roundtrip_for_symmetric_markers(
    cx: &mut TestAppContext,
) {
    let mut cx = setup_python_context(cx).await;

    cx.set_state(indoc! {"
        from PIL import Image
        import sys

        «img = Image.open(sys.argv[1])
        size = int(sys.argv[2])ˇ»
    "});
    cx.update_editor(|editor, _window, _cx| {
        editor.selections.set_line_mode(true);
    });

    cx.update_editor(|editor, window, cx| {
        editor.toggle_block_comments(&ToggleBlockComments, window, cx);
    });

    cx.assert_editor_state(indoc! {"
        from PIL import Image
        import sys

        «\"\"\"
        img = Image.open(sys.argv[1])
        size = int(sys.argv[2])
        \"\"\"ˇ»
    "});

    cx.update_editor(|editor, _window, _cx| {
        editor.selections.set_line_mode(true);
    });
    cx.set_state(indoc! {"
        from PIL import Image
        import sys

        «\"\"\"
        img = Image.open(sys.argv[1])
        size = int(sys.argv[2])
        \"\"\"ˇ»
    "});
    cx.update_editor(|editor, _window, _cx| {
        editor.selections.set_line_mode(true);
    });

    cx.update_editor(|editor, window, cx| {
        editor.toggle_block_comments(&ToggleBlockComments, window, cx);
    });

    cx.assert_editor_state(indoc! {"
        from PIL import Image
        import sys

        «img = Image.open(sys.argv[1])
        size = int(sys.argv[2])ˇ»
    "});
}

#[gpui::test]
async fn test_toggle_block_comments_line_mode_multiple_cursors_roundtrip(cx: &mut TestAppContext) {
    let mut cx = setup_python_context(cx).await;

    cx.set_state(indoc! {"
        «img = Image.open(sys.argv[1])
        size = int(sys.argv[2])ˇ»

        print(size)

        «other = Image.open(sys.argv[3])
        value = int(sys.argv[4])ˇ»
    "});
    cx.update_editor(|editor, _window, _cx| {
        editor.selections.set_line_mode(true);
    });

    cx.update_editor(|editor, window, cx| {
        editor.toggle_block_comments(&ToggleBlockComments, window, cx);
    });

    cx.assert_editor_state(indoc! {"
        «\"\"\"
        img = Image.open(sys.argv[1])
        size = int(sys.argv[2])
        \"\"\"ˇ»

        print(size)

        «\"\"\"
        other = Image.open(sys.argv[3])
        value = int(sys.argv[4])
        \"\"\"ˇ»
    "});

    cx.update_editor(|editor, _window, _cx| {
        editor.selections.set_line_mode(true);
    });
    cx.update_editor(|editor, window, cx| {
        editor.toggle_block_comments(&ToggleBlockComments, window, cx);
    });

    cx.assert_editor_state(indoc! {"
        «img = Image.open(sys.argv[1])
        size = int(sys.argv[2])ˇ»

        print(size)

        «other = Image.open(sys.argv[3])
        value = int(sys.argv[4])ˇ»
    "});
}

#[gpui::test]
async fn test_toggle_block_comments_cursor_inside_unicode_comment(cx: &mut TestAppContext) {
    let mut cx = setup_rust_context(cx).await;

    cx.set_state("/* √√√ˇ */");

    cx.update_editor(|editor, window, cx| {
        editor.toggle_block_comments(&ToggleBlockComments, window, cx);
    });

    cx.assert_editor_state("√√√ˇ");
}
