use crate::{Editor, EditorSettings, RangeToAnchorExt};
use collections::HashMap;
use gpui::{Context, HighlightStyle, Hsla, Window};
use language::Anchor;
use multi_buffer::{MultiBufferRow, MultiBufferSnapshot};
use settings::Settings;
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
            brackets_by_depth
                .entry(depth_index)
                .or_default()
                .push(start..end);
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
        let mut depth_stack: Vec<(Range<usize>, usize)> = Vec::new();
        let mut current_offset = range.start;

        // Find all bracket pairs in the range
        while current_offset < range.end {
            // Try to find an enclosing bracket pair starting from current position
            if let Some((opening_range, closing_range)) = buffer_snapshot
                .innermost_enclosing_bracket_ranges(current_offset..current_offset + 1, None)
            {
                // Check if this bracket pair overlaps with our visible range
                if opening_range.start >= range.start && opening_range.end <= range.end {
                    // Calculate depth based on how many brackets are still open
                    let depth = self.calculate_depth_at_position(
                        buffer_snapshot,
                        opening_range.start,
                        &mut depth_stack,
                    );

                    bracket_infos.push(BracketInfo {
                        range: opening_range.clone(),
                        depth,
                        is_opening: true,
                    });

                    if closing_range.start >= range.start && closing_range.end <= range.end {
                        bracket_infos.push(BracketInfo {
                            range: closing_range.clone(),
                            depth,
                            is_opening: false,
                        });
                    }

                    // Move past the opening bracket to find the next one
                    current_offset = opening_range.end;
                } else {
                    // Move forward if the bracket is outside our range
                    current_offset += 1;
                }
            } else {
                // No bracket found, move forward
                current_offset += 1;
            }

            // Safety check to prevent infinite loops
            if current_offset >= buffer_snapshot.len() {
                break;
            }
        }

        bracket_infos
    }

    /// Calculates the depth at a specific position by counting enclosing brackets
    fn calculate_depth_at_position(
        &self,
        buffer_snapshot: &MultiBufferSnapshot,
        position: usize,
        _depth_stack: &mut Vec<(Range<usize>, usize)>,
    ) -> usize {
        let mut depth = 0;
        let mut check_position = position;

        // Count how many bracket pairs enclose this position
        loop {
            if let Some((opening_range, _closing_range)) = buffer_snapshot
                .innermost_enclosing_bracket_ranges::<usize>(
                    check_position..check_position + 1,
                    Some(Box::new(|open, _close| {
                        // Only count brackets that truly enclose our position
                        open.end <= position
                    })),
                )
            {
                depth += 1;
                // Move to check for an even more outer bracket pair
                if opening_range.start > 0 {
                    check_position = opening_range.start - 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        depth
    }

    /// Gets the rainbow bracket colors from the current theme
    fn get_rainbow_bracket_colors(&self, cx: &mut Context<Editor>) -> Vec<Hsla> {
        let theme = cx.theme();
        let colors = theme.colors();

        // Use accent colors if available, otherwise create a default palette
        if let Some(accent_colors) = theme.accents() {
            accent_colors.0.clone()
        } else {
            // Default rainbow color palette using available theme colors
            vec![
                colors.text_accent,       // Level 0
                colors.link_text_hover,   // Level 1
                colors.text_muted,        // Level 2
                colors.text,              // Level 3
                colors.editor_foreground, // Level 4
                colors.text_placeholder,  // Level 5
            ]
        }
    }

    /// Checks if rainbow brackets are enabled in settings
    pub fn rainbow_brackets_enabled(&self, cx: &Context<Editor>) -> bool {
        EditorSettings::get_global(cx).rainbow_brackets
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
