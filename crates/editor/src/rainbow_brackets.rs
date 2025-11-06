use crate::Editor;
use collections::HashMap;
use gpui::{Context, HighlightStyle, Hsla, Window};
use multi_buffer::{Anchor, MultiBufferSnapshot};
use std::ops::Range;
use theme::ActiveTheme;

/// Marker type for rainbow bracket highlights
enum RainbowBracketHighlight {}

/// Represents a bracket with its nesting depth
#[derive(Debug, Clone)]
struct BracketInfo {
    range: Range<usize>,
    depth: usize,
    is_opening: bool,
}

impl Editor {
    /// Refreshes rainbow bracket highlights for all visible brackets
    pub fn refresh_rainbow_bracket_highlights(
        &mut self,
        window: &Window,
        cx: &mut Context<Editor>,
    ) {
        self.clear_highlights::<RainbowBracketHighlight>(cx);

        if !self.rainbow_brackets_enabled(cx) {
            return;
        }

        let snapshot = self.snapshot(window, cx);
        let buffer_snapshot = snapshot.buffer_snapshot();

        // Get visible range - use entire buffer for now
        let start_offset = 0;
        let end_offset = buffer_snapshot.len();

        // Calculate bracket depths for visible range
        let bracket_infos =
            self.calculate_bracket_depths(&buffer_snapshot, start_offset..end_offset);

        // Get rainbow colors from theme
        let colors = self.get_rainbow_bracket_colors(cx);
        if colors.is_empty() {
            return;
        }

        // Group brackets by depth for efficient highlighting
        let mut brackets_by_depth: HashMap<usize, Vec<Range<Anchor>>> = HashMap::default();

        for bracket_info in bracket_infos {
            let depth_index = bracket_info.depth % colors.len();
            let start = buffer_snapshot.anchor_after(bracket_info.range.start);
            let end = buffer_snapshot.anchor_before(bracket_info.range.end);
            let range = start..end;
            brackets_by_depth
                .entry(depth_index)
                .or_default()
                .push(range);
        }

        // Apply highlights for each depth level
        for (depth_index, ranges) in brackets_by_depth {
            if ranges.is_empty() {
                continue;
            }

            let color = colors[depth_index];
            self.highlight_text::<RainbowBracketHighlight>(
                ranges,
                HighlightStyle {
                    color: Some(color),
                    ..Default::default()
                },
                cx,
            );
        }
    }

    /// Calculates the nesting depth for all brackets in the given range
    fn calculate_bracket_depths(
        &self,
        buffer_snapshot: &MultiBufferSnapshot,
        range: Range<usize>,
    ) -> Vec<BracketInfo> {
        let mut bracket_infos = Vec::new();
        let mut depth_stack: Vec<(char, usize)> = Vec::new();
        let mut current_offset = range.start;

        while current_offset < range.end {
            if let Some((char, _)) = buffer_snapshot.chars_at(current_offset).next() {
                let is_opening = matches!(char, '(' | '{' | '[');
                let is_closing = matches!(char, ')' | '}' | ']');

                if is_opening {
                    // Push opening bracket onto stack
                    let depth = depth_stack.len();
                    depth_stack.push((char, current_offset));

                    bracket_infos.push(BracketInfo {
                        range: current_offset..(current_offset + 1),
                        depth,
                        is_opening: true,
                    });
                } else if is_closing {
                    // Check if we have a matching opening bracket
                    let expected_opening = match char {
                        ')' => '(',
                        '}' => '{',
                        ']' => '[',
                        _ => continue,
                    };

                    // Find and pop the matching opening bracket
                    if let Some(pos) = depth_stack
                        .iter()
                        .rposition(|(c, _)| *c == expected_opening)
                    {
                        let depth = pos;
                        depth_stack.truncate(pos);

                        bracket_infos.push(BracketInfo {
                            range: current_offset..(current_offset + 1),
                            depth,
                            is_opening: false,
                        });
                    }
                }
            }

            current_offset += 1;
        }

        bracket_infos
    }

    /// Gets the rainbow bracket colors from the current theme
    fn get_rainbow_bracket_colors(&self, cx: &mut Context<Editor>) -> Vec<Hsla> {
        let theme = cx.theme();

        // Use accent colors if available, otherwise create a default palette
        let accent_colors = theme.accents();
        if !accent_colors.0.is_empty() {
            accent_colors.0.clone()
        } else {
            // Default rainbow color palette
            vec![
                hsla(0.0, 0.95, 0.5, 1.0),   // Red
                hsla(0.083, 1.0, 0.5, 1.0),  // Orange
                hsla(0.167, 0.98, 0.5, 1.0), // Yellow
                hsla(0.333, 0.69, 0.5, 1.0), // Green
                hsla(0.583, 0.85, 0.5, 1.0), // Blue
                hsla(0.75, 0.65, 0.5, 1.0),  // Purple
                hsla(0.917, 0.85, 0.5, 1.0), // Pink
                hsla(0.472, 0.8, 0.45, 1.0), // Teal
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{editor_tests::init_test, test::editor_lsp_test_context::EditorLspTestContext};
    use indoc::indoc;
    use language::{BracketPair, BracketPairConfig, Language, LanguageConfig, LanguageMatcher};

    #[gpui::test]
    async fn test_rainbow_bracket_depth_calculation(cx: &mut gpui::TestAppContext) {
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
                            BracketPair {
                                start: "[".to_string(),
                                end: "]".to_string(),
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
                ("[" @open "]" @close)
                "#})
            .unwrap(),
            Default::default(),
            cx,
        )
        .await;

        cx.set_state(indoc! {r#"
            fn main() {
                let x = vec![1, 2, 3];
                if x.len() > 0 {
                    println!("{:?}", x);
                }
            }
        "#});

        // Test that bracket depth calculation works
        cx.update_editor(|editor, window, cx| {
            let snapshot = editor.snapshot(window, cx);
            let buffer_snapshot = snapshot.buffer_snapshot();

            // Calculate depths for entire buffer
            let bracket_infos =
                editor.calculate_bracket_depths(&buffer_snapshot, 0..buffer_snapshot.len());

            // Should have found multiple bracket pairs
            assert!(!bracket_infos.is_empty(), "Should find bracket pairs");

            // Verify depth increases with nesting
            let depths: Vec<usize> = bracket_infos.iter().map(|b| b.depth).collect();
            assert!(
                depths.iter().any(|&d| d > 0),
                "Should have brackets with depth > 0"
            );
        });
    }

    #[gpui::test]
    async fn test_rainbow_bracket_colors(cx: &mut gpui::TestAppContext) {
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
                        pairs: vec![BracketPair {
                            start: "(".to_string(),
                            end: ")".to_string(),
                            close: false,
                            surround: false,
                            newline: true,
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Some(tree_sitter_rust::LANGUAGE.into()),
            )
            .with_brackets_query(indoc! {r#"
                ("(" @open ")" @close)
                "#})
            .unwrap(),
            Default::default(),
            cx,
        )
        .await;

        cx.update_editor(|editor, _window, cx| {
            let colors = editor.get_rainbow_bracket_colors(cx);

            // Should have at least some colors defined
            assert!(
                !colors.is_empty(),
                "Should have rainbow bracket colors defined"
            );

            // Should have multiple colors for variety
            assert!(
                colors.len() >= 3,
                "Should have at least 3 colors for rainbow effect"
            );
        });
    }

    #[gpui::test]
    async fn test_nested_brackets_different_depths(cx: &mut gpui::TestAppContext) {
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
                        pairs: vec![BracketPair {
                            start: "(".to_string(),
                            end: ")".to_string(),
                            close: false,
                            surround: false,
                            newline: true,
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Some(tree_sitter_rust::LANGUAGE.into()),
            )
            .with_brackets_query(indoc! {r#"
                ("(" @open ")" @close)
                "#})
            .unwrap(),
            Default::default(),
            cx,
        )
        .await;

        cx.set_state(indoc! {r#"
            ((((inner))))
        "#});

        cx.update_editor(|editor, window, cx| {
            let snapshot = editor.snapshot(window, cx);
            let buffer_snapshot = snapshot.buffer_snapshot();

            let bracket_infos =
                editor.calculate_bracket_depths(&buffer_snapshot, 0..buffer_snapshot.len());

            // Should find 4 pairs of brackets
            let opening_brackets: Vec<_> = bracket_infos.iter().filter(|b| b.is_opening).collect();

            assert!(
                opening_brackets.len() >= 2,
                "Should find multiple nested bracket pairs"
            );

            // Verify depths are different for nested brackets
            let unique_depths: std::collections::HashSet<_> =
                opening_brackets.iter().map(|b| b.depth).collect();

            assert!(
                unique_depths.len() > 1,
                "Nested brackets should have different depths"
            );
        });
    }
}
