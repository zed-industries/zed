use language::{with_parser, Grammar, Tree};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{cmp, ops::Range, sync::Arc};

const CHUNK_THRESHOLD: usize = 1500;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub range: Range<usize>,
    pub digest: [u8; 32],
}

pub fn chunk_text(text: &str, grammar: Option<&Arc<Grammar>>) -> Vec<Chunk> {
    if let Some(grammar) = grammar {
        let tree = with_parser(|parser| {
            parser
                .set_language(&grammar.ts_language)
                .expect("incompatible grammar");
            parser.parse(&text, None).expect("invalid language")
        });

        chunk_parse_tree(tree, &text, CHUNK_THRESHOLD)
    } else {
        chunk_lines(&text)
    }
}

fn chunk_parse_tree(tree: Tree, text: &str, chunk_threshold: usize) -> Vec<Chunk> {
    let mut chunk_ranges = Vec::new();
    let mut cursor = tree.walk();

    let mut range = 0..0;
    loop {
        let node = cursor.node();

        // If adding the node to the current chunk exceeds the threshold
        if node.end_byte() - range.start > chunk_threshold {
            // Try to descend into its first child. If we can't, flush the current
            // range and try again.
            if cursor.goto_first_child() {
                continue;
            } else if !range.is_empty() {
                chunk_ranges.push(range.clone());
                range.start = range.end;
                continue;
            }

            // If we get here, the node itself has no children but is larger than the threshold.
            // Break its text into arbitrary chunks.
            split_text(text, range.clone(), node.end_byte(), &mut chunk_ranges);
        }
        range.end = node.end_byte();

        // If we get here, we consumed the node. Advance to the next child, ascending if there isn't one.
        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                if !range.is_empty() {
                    chunk_ranges.push(range);
                }

                return chunk_ranges
                    .into_iter()
                    .map(|range| {
                        let digest = Sha256::digest(&text[range.clone()]).into();
                        Chunk { range, digest }
                    })
                    .collect();
            }
        }
    }
}

fn chunk_lines(text: &str) -> Vec<Chunk> {
    let mut chunk_ranges = Vec::new();
    let mut range = 0..0;

    let mut newlines = text.match_indices('\n').peekable();
    while let Some((newline_ix, _)) = newlines.peek() {
        let newline_ix = newline_ix + 1;
        if newline_ix - range.start <= CHUNK_THRESHOLD {
            range.end = newline_ix;
            newlines.next();
        } else {
            if range.is_empty() {
                split_text(text, range, newline_ix, &mut chunk_ranges);
                range = newline_ix..newline_ix;
            } else {
                chunk_ranges.push(range.clone());
                range.start = range.end;
            }
        }
    }

    if !range.is_empty() {
        chunk_ranges.push(range);
    }

    chunk_ranges
        .into_iter()
        .map(|range| Chunk {
            digest: Sha256::digest(&text[range.clone()]).into(),
            range,
        })
        .collect()
}

fn split_text(
    text: &str,
    mut range: Range<usize>,
    max_end: usize,
    chunk_ranges: &mut Vec<Range<usize>>,
) {
    while range.start < max_end {
        range.end = cmp::min(range.start + CHUNK_THRESHOLD, max_end);
        while !text.is_char_boundary(range.end) {
            range.end -= 1;
        }
        chunk_ranges.push(range.clone());
        range.start = range.end;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use language::{tree_sitter_rust, Language, LanguageConfig, LanguageMatcher};

    // This example comes from crates/gpui/examples/window_positioning.rs which
    // has the property of being CHUNK_THRESHOLD < TEXT.len() < 2*CHUNK_THRESHOLD
    static TEXT: &str = r#"
    use gpui::*;

    struct WindowContent {
        text: SharedString,
    }

    impl Render for WindowContent {
        fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
            div()
                .flex()
                .bg(rgb(0x1e2025))
                .size_full()
                .justify_center()
                .items_center()
                .text_xl()
                .text_color(rgb(0xffffff))
                .child(self.text.clone())
        }
    }

    fn main() {
        App::new().run(|cx: &mut AppContext| {
            // Create several new windows, positioned in the top right corner of each screen

            for screen in cx.displays() {
                let options = {
                    let popup_margin_width = DevicePixels::from(16);
                    let popup_margin_height = DevicePixels::from(-0) - DevicePixels::from(48);

                    let window_size = Size {
                        width: px(400.),
                        height: px(72.),
                    };

                    let screen_bounds = screen.bounds();
                    let size: Size<DevicePixels> = window_size.into();

                    let bounds = gpui::Bounds::<DevicePixels> {
                        origin: screen_bounds.upper_right()
                            - point(size.width + popup_margin_width, popup_margin_height),
                        size: window_size.into(),
                    };

                    WindowOptions {
                        // Set the bounds of the window in screen coordinates
                        bounds: Some(bounds),
                        // Specify the display_id to ensure the window is created on the correct screen
                        display_id: Some(screen.id()),

                        titlebar: None,
                        window_background: WindowBackgroundAppearance::default(),
                        focus: false,
                        show: true,
                        kind: WindowKind::PopUp,
                        is_movable: false,
                        fullscreen: false,
                        app_id: None,
                    }
                };

                cx.open_window(options, |cx| {
                    cx.new_view(|_| WindowContent {
                        text: format!("{:?}", screen.id()).into(),
                    })
                });
            }
        });
    }"#;

    fn setup_rust_language() -> Language {
        Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        )
    }

    #[test]
    fn test_chunk_text() {
        let text = "a\n".repeat(1000);
        let chunks = chunk_text(&text, None);
        assert_eq!(
            chunks.len(),
            ((2000_f64) / (CHUNK_THRESHOLD as f64)).ceil() as usize
        );
    }

    #[test]
    fn test_chunk_text_grammar() {
        // Let's set up a big text with some known segments
        // We'll then chunk it and verify that the chunks are correct

        let language = setup_rust_language();

        let chunks = chunk_text(TEXT, language.grammar());
        assert_eq!(chunks.len(), 2);

        assert_eq!(chunks[0].range.start, 0);
        assert_eq!(chunks[0].range.end, 1498);
        // The break between chunks is right before the "Specify the display_id" comment

        assert_eq!(chunks[1].range.start, 1498);
        assert_eq!(chunks[1].range.end, 2434);
    }

    #[test]
    fn test_chunk_parse_tree() {
        let language = setup_rust_language();
        let grammar = language.grammar().unwrap();

        let tree = with_parser(|parser| {
            parser
                .set_language(&grammar.ts_language)
                .expect("incompatible grammar");
            parser.parse(TEXT, None).expect("invalid language")
        });

        let chunks = chunk_parse_tree(tree, TEXT, 250);
        assert_eq!(chunks.len(), 11);
    }

    #[test]
    fn test_chunk_unparsable() {
        // Even if a chunk is unparsable, we should still be able to chunk it
        let language = setup_rust_language();
        let grammar = language.grammar().unwrap();

        let text = r#"fn main() {"#;
        let tree = with_parser(|parser| {
            parser
                .set_language(&grammar.ts_language)
                .expect("incompatible grammar");
            parser.parse(text, None).expect("invalid language")
        });

        let chunks = chunk_parse_tree(tree, text, 250);
        assert_eq!(chunks.len(), 1);

        assert_eq!(chunks[0].range.start, 0);
        assert_eq!(chunks[0].range.end, 11);
    }

    #[test]
    fn test_empty_text() {
        let language = setup_rust_language();
        let grammar = language.grammar().unwrap();

        let tree = with_parser(|parser| {
            parser
                .set_language(&grammar.ts_language)
                .expect("incompatible grammar");
            parser.parse("", None).expect("invalid language")
        });

        let chunks = chunk_parse_tree(tree, "", CHUNK_THRESHOLD);
        assert!(chunks.is_empty(), "Chunks should be empty for empty text");
    }

    #[test]
    fn test_single_large_node() {
        let large_text = "static ".to_owned() + "a".repeat(CHUNK_THRESHOLD - 1).as_str() + " = 2";

        let language = setup_rust_language();
        let grammar = language.grammar().unwrap();

        let tree = with_parser(|parser| {
            parser
                .set_language(&grammar.ts_language)
                .expect("incompatible grammar");
            parser.parse(&large_text, None).expect("invalid language")
        });

        let chunks = chunk_parse_tree(tree, &large_text, CHUNK_THRESHOLD);

        assert_eq!(
            chunks.len(),
            3,
            "Large chunks are broken up according to grammar as best as possible"
        );

        // Expect chunks to be static, aaaaaa..., and = 2
        assert_eq!(chunks[0].range.start, 0);
        assert_eq!(chunks[0].range.end, "static".len());

        assert_eq!(chunks[1].range.start, "static".len());
        assert_eq!(chunks[1].range.end, "static".len() + CHUNK_THRESHOLD);

        assert_eq!(chunks[2].range.start, "static".len() + CHUNK_THRESHOLD);
        assert_eq!(chunks[2].range.end, large_text.len());
    }

    #[test]
    fn test_multiple_small_nodes() {
        let small_text = "a b c d e f g h i j k l m n o p q r s t u v w x y z";
        let language = setup_rust_language();
        let grammar = language.grammar().unwrap();

        let tree = with_parser(|parser| {
            parser
                .set_language(&grammar.ts_language)
                .expect("incompatible grammar");
            parser.parse(small_text, None).expect("invalid language")
        });

        let chunks = chunk_parse_tree(tree, small_text, 5);
        assert!(
            chunks.len() > 1,
            "Should have multiple chunks for multiple small nodes"
        );
    }

    #[test]
    fn test_node_with_children() {
        let nested_text = "fn main() { let a = 1; let b = 2; }";
        let language = setup_rust_language();
        let grammar = language.grammar().unwrap();

        let tree = with_parser(|parser| {
            parser
                .set_language(&grammar.ts_language)
                .expect("incompatible grammar");
            parser.parse(nested_text, None).expect("invalid language")
        });

        let chunks = chunk_parse_tree(tree, nested_text, 10);
        assert!(
            chunks.len() > 1,
            "Should have multiple chunks for a node with children"
        );
    }

    #[test]
    fn test_text_with_unparsable_sections() {
        // This test uses purposefully hit-or-miss sizing of 11 characters per likely chunk
        let mixed_text = "fn main() { let a = 1; let b = 2; } unparsable bits here";
        let language = setup_rust_language();
        let grammar = language.grammar().unwrap();

        let tree = with_parser(|parser| {
            parser
                .set_language(&grammar.ts_language)
                .expect("incompatible grammar");
            parser.parse(mixed_text, None).expect("invalid language")
        });

        let chunks = chunk_parse_tree(tree, mixed_text, 11);
        assert!(
            chunks.len() > 1,
            "Should handle both parsable and unparsable sections correctly"
        );

        let expected_chunks = [
            "fn main() {",
            " let a = 1;",
            " let b = 2;",
            " }",
            " unparsable",
            " bits here",
        ];

        for (i, chunk) in chunks.iter().enumerate() {
            assert_eq!(
                &mixed_text[chunk.range.clone()],
                expected_chunks[i],
                "Chunk {} should match",
                i
            );
        }
    }
}
