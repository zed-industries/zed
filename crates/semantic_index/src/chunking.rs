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

        chunk_parse_tree(tree, &text)
    } else {
        chunk_lines(&text)
    }
}

fn chunk_parse_tree(tree: Tree, text: &str) -> Vec<Chunk> {
    let mut chunk_ranges = Vec::new();
    let mut cursor = tree.walk();

    let mut range = 0..0;
    loop {
        let node = cursor.node();

        // If adding the node to the current chunk exceeds the threshold
        if node.end_byte() - range.start > CHUNK_THRESHOLD {
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
        .map(|range| {
            let mut hasher = Sha256::new();
            hasher.update(&text[range.clone()]);
            let mut digest = [0u8; 32];
            digest.copy_from_slice(hasher.finalize().as_slice());
            Chunk { range, digest }
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

    #[test]
    fn test_chunk_text() {
        let text = "a\n".repeat(1000);
        let chunks = chunk_text(&text, None);
        assert_eq!(
            chunks.len(),
            ((2000_f64) / (CHUNK_THRESHOLD as f64)).ceil() as usize
        );
    }

    #[gpui::test]
    async fn test_chunk_text_grammar() {
        // Let's set up a big text with some known segments
        // We'll then chunk it and verify that the chunks are correct

        let text = r#"
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

        let language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        );

        let chunks = chunk_text(text, language.grammar());
        assert_eq!(chunks.len(), 2);

        assert_eq!(chunks[0].range.start, 0);
        assert_eq!(chunks[0].range.end, 1470);
        // The break between chunks is right before the "Specify the display_id" comment

        assert_eq!(chunks[1].range.start, 1470);
        assert_eq!(chunks[1].range.end, 2168);
    }
}
