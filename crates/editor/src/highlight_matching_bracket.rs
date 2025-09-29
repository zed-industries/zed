use crate::{Editor, RangeToAnchorExt};
use gpui::{Context, HighlightStyle, Window};
use language::CursorShape;
use theme::ActiveTheme;

enum MatchingBracketHighlight {}

pub fn refresh_matching_bracket_highlights(
    editor: &mut Editor,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    editor.clear_highlights::<MatchingBracketHighlight>(cx);

    let newest_selection = editor.selections.newest::<usize>(cx);
    // Don't highlight brackets if the selection isn't empty
    if !newest_selection.is_empty() {
        return;
    }

    let snapshot = editor.snapshot(window, cx);
    let head = newest_selection.head();
    if head > snapshot.buffer_snapshot.len() {
        log::error!("bug: cursor offset is out of range while refreshing bracket highlights");
        return;
    }

    let mut tail = head;
    if (editor.cursor_shape == CursorShape::Block || editor.cursor_shape == CursorShape::Hollow)
        && head < snapshot.buffer_snapshot.len()
    {
        if let Some(tail_ch) = snapshot.buffer_snapshot.chars_at(tail).next() {
            tail += tail_ch.len_utf8();
        }
    }

    if let Some((opening_range, closing_range)) = snapshot
        .buffer_snapshot
        .innermost_enclosing_bracket_ranges(head..tail, None)
    {
        editor.highlight_text::<MatchingBracketHighlight>(
            vec![
                opening_range.to_anchors(&snapshot.buffer_snapshot),
                closing_range.to_anchors(&snapshot.buffer_snapshot),
            ],
            HighlightStyle {
                background_color: Some(
                    cx.theme()
                        .colors()
                        .editor_document_highlight_bracket_background,
                ),
                ..Default::default()
            },
            cx,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{editor_tests::init_test, test::editor_lsp_test_context::EditorLspTestContext};
    use indoc::indoc;
    use language::{BracketPair, BracketPairConfig, Language, LanguageConfig, LanguageMatcher};

    #[gpui::test]
    async fn test_matching_bracket_highlights(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});

        let mut cx = EditorLspTestContext::new(
            Language::new(
                LanguageConfig {
                    name: "Rust".into(),
                    matcher: LanguageMatcher {
                        path_suffixes: vec!["rs".to_string()],
                        ..Default::default()
                    },
                    brackets: BracketPairConfig {
                        pairs: vec![
                            BracketPair {
                                start: "{".to_string(),
                                end: "}".to_string(),
                                close: false,
                                surround: false,
                                newline: true,
                            },
                            BracketPair {
                                start: "(".to_string(),
                                end: ")".to_string(),
                                close: false,
                                surround: false,
                                newline: true,
                            },
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Some(tree_sitter_rust::LANGUAGE.into()),
            )
            .with_brackets_query(indoc! {r#"
                ("{" @open "}" @close)
                ("(" @open ")" @close)
                "#})
            .unwrap(),
            Default::default(),
            cx,
        )
        .await;

        // positioning cursor inside bracket highlights both
        cx.set_state(indoc! {r#"
            pub fn test("Test ˇargument") {
                another_test(1, 2, 3);
            }
        "#});
        cx.assert_editor_text_highlights::<MatchingBracketHighlight>(indoc! {r#"
            pub fn test«(»"Test argument"«)» {
                another_test(1, 2, 3);
            }
        "#});

        cx.set_state(indoc! {r#"
            pub fn test("Test argument") {
                another_test(1, ˇ2, 3);
            }
        "#});
        cx.assert_editor_text_highlights::<MatchingBracketHighlight>(indoc! {r#"
            pub fn test("Test argument") {
                another_test«(»1, 2, 3«)»;
            }
        "#});

        cx.set_state(indoc! {r#"
            pub fn test("Test argument") {
                anotherˇ_test(1, 2, 3);
            }
        "#});
        cx.assert_editor_text_highlights::<MatchingBracketHighlight>(indoc! {r#"
            pub fn test("Test argument") «{»
                another_test(1, 2, 3);
            «}»
        "#});

        // positioning outside of brackets removes highlight
        cx.set_state(indoc! {r#"
            pub fˇn test("Test argument") {
                another_test(1, 2, 3);
            }
        "#});
        cx.assert_editor_text_highlights::<MatchingBracketHighlight>(indoc! {r#"
            pub fn test("Test argument") {
                another_test(1, 2, 3);
            }
        "#});

        // non empty selection dismisses highlight
        cx.set_state(indoc! {r#"
            pub fn test("Te«st argˇ»ument") {
                another_test(1, 2, 3);
            }
        "#});
        cx.assert_editor_text_highlights::<MatchingBracketHighlight>(indoc! {r#"
            pub fn test«("Test argument") {
                another_test(1, 2, 3);
            }
        "#});
    }
}
