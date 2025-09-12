use crate::{Editor, RangeToAnchorExt};
use gpui::{Context, HighlightStyle, Hsla, Window};
use itertools::Itertools;
use language::CursorShape;
use multi_buffer::ToPoint;
use text::{Bias, Point};

enum MatchingBracketHighlight {}

struct RainbowBracketHighlight;

#[derive(PartialEq, Eq)]
pub(crate) enum BracketRefreshReason {
    BufferEdited,
    ScrollPositionChanged,
    SelectionsChanged,
}

impl Editor {
    // todo! run with a debounce
    pub(crate) fn refresh_bracket_highlights(
        &mut self,
        refresh_reason: BracketRefreshReason,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        const COLORS: [Hsla; 4] = [gpui::red(), gpui::yellow(), gpui::green(), gpui::blue()];

        let snapshot = self.snapshot(window, cx);
        let multi_buffer_snapshot = &snapshot.buffer_snapshot;

        let multi_buffer_visible_start = snapshot
            .scroll_anchor
            .anchor
            .to_point(multi_buffer_snapshot);

        // todo! deduplicate?
        let multi_buffer_visible_end = multi_buffer_snapshot.clip_point(
            multi_buffer_visible_start
                + Point::new(self.visible_line_count().unwrap_or(40.).ceil() as u32, 0),
            Bias::Left,
        );

        let bracket_matches = multi_buffer_snapshot
            .range_to_buffer_ranges(multi_buffer_visible_start..multi_buffer_visible_end)
            .into_iter()
            .filter_map(|(buffer_snapshot, buffer_range, _)| {
                let buffer_brackets =
                    buffer_snapshot.bracket_ranges(buffer_range.start..buffer_range.end);

                // todo! is there a good way to use the excerpt_id instead?
                let mut excerpt = multi_buffer_snapshot.excerpt_containing(buffer_range.clone())?;

                Some(
                    buffer_brackets
                        .into_iter()
                        .filter_map(|pair| {
                            let buffer_range = pair.open_range.start..pair.close_range.end;
                            if excerpt.contains_buffer_range(buffer_range) {
                                Some((
                                    pair.depth,
                                    excerpt.map_range_from_buffer(pair.open_range),
                                    excerpt.map_range_from_buffer(pair.close_range),
                                ))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .flatten()
            .into_group_map_by(|&(depth, ..)| depth);

        for (depth, bracket_highlights) in bracket_matches {
            let style = HighlightStyle {
                color: Some({
                    // todo! these colors lack contrast for this/are not actually good for that?
                    // cx.theme().accents().color_for_index(depth as u32);
                    COLORS[depth as usize % COLORS.len()]
                }),
                ..HighlightStyle::default()
            };

            self.highlight_text_key::<RainbowBracketHighlight>(
                depth,
                bracket_highlights
                    .into_iter()
                    .flat_map(|(_, open, close)| {
                        [
                            open.to_anchors(&multi_buffer_snapshot),
                            close.to_anchors(&multi_buffer_snapshot),
                        ]
                    })
                    .collect(),
                style,
                cx,
            );
        }

        if refresh_reason == BracketRefreshReason::ScrollPositionChanged {
            return;
        }
        self.clear_highlights::<MatchingBracketHighlight>(cx);

        let newest_selection = self.selections.newest::<usize>(cx);
        // Don't highlight brackets if the selection isn't empty
        if !newest_selection.is_empty() {
            return;
        }

        let head = newest_selection.head();
        if head > snapshot.buffer_snapshot.len() {
            log::error!("bug: cursor offset is out of range while refreshing bracket highlights");
            return;
        }

        let mut tail = head;
        if (self.cursor_shape == CursorShape::Block || self.cursor_shape == CursorShape::Hollow)
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
            self.highlight_background::<MatchingBracketHighlight>(
                &[
                    opening_range.to_anchors(&snapshot.buffer_snapshot),
                    closing_range.to_anchors(&snapshot.buffer_snapshot),
                ],
                |theme| theme.colors().editor_document_highlight_bracket_background,
                cx,
            )
        }
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
