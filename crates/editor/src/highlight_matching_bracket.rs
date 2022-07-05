use gpui::ViewContext;

use crate::{Editor, RangeToAnchorExt};

enum MatchingBracketHighlight {}

pub fn refresh_matching_bracket_highlights(editor: &mut Editor, cx: &mut ViewContext<Editor>) {
    editor.clear_background_highlights::<MatchingBracketHighlight>(cx);

    let newest_selection = editor.selections.newest::<usize>(cx);
    // Don't highlight brackets if the selection isn't empty
    if !newest_selection.is_empty() {
        return;
    }

    let head = newest_selection.head();
    let snapshot = editor.snapshot(cx);
    if let Some((opening_range, closing_range)) = snapshot
        .buffer_snapshot
        .enclosing_bracket_ranges(head..head)
    {
        editor.highlight_background::<MatchingBracketHighlight>(
            vec![
                opening_range.to_anchors(&snapshot.buffer_snapshot),
                closing_range.to_anchors(&snapshot.buffer_snapshot),
            ],
            |theme| theme.editor.document_highlight_read_background,
            cx,
        )
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use language::{BracketPair, Language, LanguageConfig};

    use crate::test::EditorLspTestContext;

    use super::*;

    #[gpui::test]
    async fn test_matching_bracket_highlights(cx: &mut gpui::TestAppContext) {
        let mut cx = EditorLspTestContext::new(
            Language::new(
                LanguageConfig {
                    name: "Rust".into(),
                    path_suffixes: vec!["rs".to_string()],
                    brackets: vec![
                        BracketPair {
                            start: "{".to_string(),
                            end: "}".to_string(),
                            close: false,
                            newline: true,
                        },
                        BracketPair {
                            start: "(".to_string(),
                            end: ")".to_string(),
                            close: false,
                            newline: true,
                        },
                    ],
                    ..Default::default()
                },
                Some(tree_sitter_rust::language()),
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
        cx.set_state_by(
            vec!['|'.into()],
            indoc! {r#"
                pub fn test("Test |argument") {
                    another_test(1, 2, 3);
                }"#},
        );
        cx.assert_editor_background_highlights::<MatchingBracketHighlight>(indoc! {r#"
                pub fn test[(]"Test argument"[)] {
                    another_test(1, 2, 3);
                }"#});

        cx.set_state_by(
            vec!['|'.into()],
            indoc! {r#"
                pub fn test("Test argument") {
                    another_test(1, |2, 3);
                }"#},
        );
        cx.assert_editor_background_highlights::<MatchingBracketHighlight>(indoc! {r#"
            pub fn test("Test argument") {
                another_test[(]1, 2, 3[)];
            }"#});

        cx.set_state_by(
            vec!['|'.into()],
            indoc! {r#"
                pub fn test("Test argument") {
                    another|_test(1, 2, 3);
                }"#},
        );
        cx.assert_editor_background_highlights::<MatchingBracketHighlight>(indoc! {r#"
            pub fn test("Test argument") [{]
                another_test(1, 2, 3);
            [}]"#});

        // positioning outside of brackets removes highlight
        cx.set_state_by(
            vec!['|'.into()],
            indoc! {r#"
                pub f|n test("Test argument") {
                    another_test(1, 2, 3);
                }"#},
        );
        cx.assert_editor_background_highlights::<MatchingBracketHighlight>(indoc! {r#"
            pub fn test("Test argument") {
                another_test(1, 2, 3);
            }"#});

        // non empty selection dismisses highlight
        // positioning outside of brackets removes highlight
        cx.set_state_by(
            vec![('<', '>').into()],
            indoc! {r#"
                pub fn test("Te<st arg>ument") {
                    another_test(1, 2, 3);
                }"#},
        );
        cx.assert_editor_background_highlights::<MatchingBracketHighlight>(indoc! {r#"
            pub fn test("Test argument") {
                another_test(1, 2, 3);
            }"#});
    }
}
