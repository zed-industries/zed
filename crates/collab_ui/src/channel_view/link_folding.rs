//! Link folding support for channel notes.
//!
//! This module provides functionality to automatically fold markdown links in channel notes,
//! displaying only the link text with an underline decoration. When the cursor is positioned
//! inside a link, the crease is temporarily removed to show the full markdown syntax.
//!
//! Example:
//! - When rendered: [Example](https://example.com) becomes "Example" with underline
//! - When cursor is inside: Full markdown syntax is shown
//!
//! The main components are:
//! - `parse_markdown_links`: Extracts markdown links from text
//! - `create_link_creases`: Creates visual creases for links
//! - `LinkFoldingManager`: Manages dynamic showing/hiding of link creases based on cursor position

use editor::{
    Anchor, Editor, FoldPlaceholder,
    display_map::{Crease, CreaseId},
};
use gpui::{App, Entity, Window};

use std::{collections::HashMap, ops::Range, sync::Arc};
use ui::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct MarkdownLink {
    pub text: String,
    pub url: String,
    pub range: Range<usize>,
}

pub fn parse_markdown_links(text: &str) -> Vec<MarkdownLink> {
    let mut links = Vec::new();
    let mut chars = text.char_indices();

    while let Some((start, ch)) = chars.next() {
        if ch == '[' {
            // Look for the closing bracket
            let mut bracket_depth = 1;
            let text_start = start + 1;
            let mut text_end = None;

            for (i, ch) in chars.by_ref() {
                match ch {
                    '[' => bracket_depth += 1,
                    ']' => {
                        bracket_depth -= 1;
                        if bracket_depth == 0 {
                            text_end = Some(i);
                            break;
                        }
                    }
                    _ => {}
                }
            }

            if let Some(text_end) = text_end {
                // Check if the next character is '('
                if let Some((_, '(')) = chars.next() {
                    // Look for the closing parenthesis
                    let url_start = text_end + 2;
                    let mut url_end = None;

                    for (i, ch) in chars.by_ref() {
                        if ch == ')' {
                            url_end = Some(i);
                            break;
                        }
                    }

                    if let Some(url_end) = url_end {
                        links.push(MarkdownLink {
                            text: text[text_start..text_end].to_string(),
                            url: text[url_start..url_end].to_string(),
                            range: start..url_end + 1,
                        });
                    }
                }
            }
        }
    }

    links
}

pub fn create_link_creases(
    links: &[MarkdownLink],
    buffer_snapshot: &editor::MultiBufferSnapshot,
) -> Vec<Crease<Anchor>> {
    links
        .iter()
        .map(|link| {
            // Convert byte offsets to Points first
            let start_point = buffer_snapshot.offset_to_point(link.range.start);
            let end_point = buffer_snapshot.offset_to_point(link.range.end);

            // Create anchors from points
            let start = buffer_snapshot.anchor_before(start_point);
            let end = buffer_snapshot.anchor_after(end_point);

            let link_text = link.text.clone();
            Crease::simple(
                start..end,
                FoldPlaceholder {
                    render: Arc::new(move |_fold_id, _range, cx| {
                        div()
                            .child(link_text.clone())
                            .text_decoration_1()
                            .text_decoration_solid()
                            .text_color(cx.theme().colors().link_text_hover)
                            .into_any_element()
                    }),
                    constrain_width: false,
                    merge_adjacent: false,
                    type_tag: None,
                },
            )
        })
        .collect()
}

pub struct LinkFoldingManager {
    editor: Entity<Editor>,
    folded_links: Vec<MarkdownLink>,
    link_creases: HashMap<CreaseId, Range<usize>>,
}

impl LinkFoldingManager {
    pub fn new(editor: Entity<Editor>, window: &mut Window, cx: &mut App) -> Self {
        let mut manager = Self {
            editor: editor.clone(),
            folded_links: Vec::new(),
            link_creases: HashMap::default(),
        };

        manager.refresh_links(window, cx);
        manager
    }

    pub fn handle_selections_changed(&mut self, window: &mut Window, cx: &mut App) {
        self.update_link_visibility(window, cx);
    }

    pub fn handle_edited(&mut self, window: &mut Window, cx: &mut App) {
        // Re-parse links and update folds
        // Note: In a production implementation, this could be debounced
        // to avoid updating on every keystroke
        self.refresh_links(window, cx);
    }

    fn refresh_links(&mut self, window: &mut Window, cx: &mut App) {
        // Remove existing creases
        if !self.link_creases.is_empty() {
            self.editor.update(cx, |editor, cx| {
                let crease_ids: Vec<_> = self.link_creases.keys().copied().collect();
                editor.remove_creases(crease_ids, cx);
            });
            self.link_creases.clear();
        }

        // Parse new links
        let buffer_text = self.editor.read(cx).buffer().read(cx).snapshot(cx).text();
        let links = parse_markdown_links(&buffer_text);
        self.folded_links = links;

        // Insert creases for all links
        if !self.folded_links.is_empty() {
            self.editor.update(cx, |editor, cx| {
                let buffer = editor.buffer().read(cx).snapshot(cx);
                let creases = create_link_creases(&self.folded_links, &buffer);
                let crease_ids = editor.insert_creases(creases.clone(), cx);

                // Store the mapping of crease IDs to link ranges
                for (crease_id, link) in crease_ids.into_iter().zip(self.folded_links.iter()) {
                    self.link_creases.insert(crease_id, link.range.clone());
                }

                // Fold the creases to activate the custom placeholder
                editor.fold_creases(creases, true, window, cx);
            });
        }

        // Update visibility based on cursor position
        self.update_link_visibility(window, cx);
    }

    fn update_link_visibility(&mut self, window: &mut Window, cx: &mut App) {
        if self.folded_links.is_empty() {
            return;
        }

        self.editor.update(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx).snapshot(cx);
            let selections = editor.selections.all::<usize>(cx);

            // Find which links should be visible (cursor inside or adjacent)
            // Remove creases where cursor is inside or adjacent to the link
            let mut creases_to_remove = Vec::new();
            for (crease_id, link_range) in &self.link_creases {
                let cursor_near_link = selections.iter().any(|selection| {
                    let cursor_offset = selection.head();
                    // Check if cursor is inside the link
                    if link_range.contains(&cursor_offset) {
                        return true;
                    }
                    // Check if cursor is adjacent (immediately before or after)
                    cursor_offset == link_range.start || cursor_offset == link_range.end
                });

                if cursor_near_link {
                    creases_to_remove.push(*crease_id);
                }
            }

            if !creases_to_remove.is_empty() {
                editor.remove_creases(creases_to_remove.clone(), cx);
                for crease_id in creases_to_remove {
                    self.link_creases.remove(&crease_id);
                }
            }

            // Re-add creases for links where cursor is not present or adjacent
            let links_to_recreate: Vec<_> = self
                .folded_links
                .iter()
                .filter(|link| {
                    !selections.iter().any(|selection| {
                        let cursor_offset = selection.head();
                        // Check if cursor is inside or adjacent to the link
                        link.range.contains(&cursor_offset)
                            || cursor_offset == link.range.start
                            || cursor_offset == link.range.end
                    })
                })
                .filter(|link| {
                    // Only recreate if not already present
                    !self.link_creases.values().any(|range| range == &link.range)
                })
                .cloned()
                .collect();

            if !links_to_recreate.is_empty() {
                let creases = create_link_creases(&links_to_recreate, &buffer);
                let crease_ids = editor.insert_creases(creases.clone(), cx);

                for (crease_id, link) in crease_ids.into_iter().zip(links_to_recreate.iter()) {
                    self.link_creases.insert(crease_id, link.range.clone());
                }

                // Fold the creases to activate the custom placeholder
                editor.fold_creases(creases, true, window, cx);
            }
        });
    }

    /// Checks if a position (in buffer coordinates) is within a folded link
    /// and returns the link if found
    pub fn link_at_position(&self, position: usize) -> Option<&MarkdownLink> {
        // Check if the position falls within any folded link that has an active crease
        self.folded_links.iter().find(|link| {
            link.range.contains(&position)
                && self.link_creases.values().any(|range| range == &link.range)
        })
    }

    /// Opens a link URL in the default browser
    pub fn open_link(&self, url: &str, cx: &mut App) {
        cx.open_url(url);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[gpui::test]
    fn test_parse_markdown_links() {
        let text = "This is a [link](https://example.com) and another [test](https://test.com).";
        let links = parse_markdown_links(text);

        assert_eq!(links.len(), 2);

        assert_eq!(links[0].text, "link");
        assert_eq!(links[0].url, "https://example.com");
        assert_eq!(links[0].range, 10..37);

        assert_eq!(links[1].text, "test");
        assert_eq!(links[1].url, "https://test.com");
        assert_eq!(links[1].range, 50..74);
    }

    #[gpui::test]
    fn test_parse_nested_brackets() {
        let text = "A [[nested] link](https://example.com) here.";
        let links = parse_markdown_links(text);

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].text, "[nested] link");
        assert_eq!(links[0].url, "https://example.com");
    }

    #[gpui::test]
    fn test_parse_no_links() {
        let text = "This text has no links.";
        let links = parse_markdown_links(text);

        assert_eq!(links.len(), 0);
    }

    #[gpui::test]
    fn test_parse_incomplete_links() {
        let text = "This [link has no url] and [this](incomplete";
        let links = parse_markdown_links(text);

        assert_eq!(links.len(), 0);
    }

    #[gpui::test]
    fn test_link_parsing_and_folding() {
        // Test comprehensive link parsing
        let text = "Check out [Zed](https://zed.dev) and [GitHub](https://github.com)!";
        let links = parse_markdown_links(text);

        assert_eq!(links.len(), 2);
        assert_eq!(links[0].text, "Zed");
        assert_eq!(links[0].url, "https://zed.dev");
        assert_eq!(links[0].range, 10..32);

        assert_eq!(links[1].text, "GitHub");
        assert_eq!(links[1].url, "https://github.com");
        assert_eq!(links[1].range, 37..65);
    }

    #[gpui::test]
    fn test_link_detection_when_typing() {
        // Test that links are detected as they're typed
        let text1 = "Check out ";
        let links1 = parse_markdown_links(text1);
        assert_eq!(links1.len(), 0, "No links in plain text");

        let text2 = "Check out [";
        let links2 = parse_markdown_links(text2);
        assert_eq!(links2.len(), 0, "Incomplete link not detected");

        let text3 = "Check out [Zed]";
        let links3 = parse_markdown_links(text3);
        assert_eq!(links3.len(), 0, "Link without URL not detected");

        let text4 = "Check out [Zed](";
        let links4 = parse_markdown_links(text4);
        assert_eq!(links4.len(), 0, "Link with incomplete URL not detected");

        let text5 = "Check out [Zed](https://zed.dev)";
        let links5 = parse_markdown_links(text5);
        assert_eq!(links5.len(), 1, "Complete link should be detected");
        assert_eq!(links5[0].text, "Zed");
        assert_eq!(links5[0].url, "https://zed.dev");

        // Test link detection in middle of text
        let text6 = "Check out [Zed](https://zed.dev) for coding!";
        let links6 = parse_markdown_links(text6);
        assert_eq!(links6.len(), 1, "Link in middle of text should be detected");
        assert_eq!(links6[0].range, 10..32);
    }

    #[gpui::test]
    fn test_link_position_detection() {
        // Test the logic for determining if a position is within a link
        let links = vec![
            MarkdownLink {
                text: "Zed".to_string(),
                url: "https://zed.dev".to_string(),
                range: 10..32,
            },
            MarkdownLink {
                text: "GitHub".to_string(),
                url: "https://github.com".to_string(),
                range: 50..78,
            },
        ];

        // Test positions inside the first link
        assert!(links[0].range.contains(&10), "Start of first link");
        assert!(links[0].range.contains(&20), "Middle of first link");
        assert!(links[0].range.contains(&31), "Near end of first link");

        // Test positions inside the second link
        assert!(links[1].range.contains(&50), "Start of second link");
        assert!(links[1].range.contains(&65), "Middle of second link");
        assert!(links[1].range.contains(&77), "Near end of second link");

        // Test positions outside any link
        assert!(!links[0].range.contains(&9), "Before first link");
        assert!(!links[0].range.contains(&32), "After first link");
        assert!(!links[1].range.contains(&49), "Before second link");
        assert!(!links[1].range.contains(&78), "After second link");

        // Test finding a link at a specific position
        let link_at_20 = links.iter().find(|link| link.range.contains(&20));
        assert!(link_at_20.is_some());
        assert_eq!(link_at_20.unwrap().text, "Zed");
        assert_eq!(link_at_20.unwrap().url, "https://zed.dev");

        let link_at_65 = links.iter().find(|link| link.range.contains(&65));
        assert!(link_at_65.is_some());
        assert_eq!(link_at_65.unwrap().text, "GitHub");
        assert_eq!(link_at_65.unwrap().url, "https://github.com");
    }

    #[gpui::test]
    fn test_cursor_adjacent_link_expansion() {
        // Test the logic for determining if cursor is inside or adjacent to a link
        let link = MarkdownLink {
            text: "Example".to_string(),
            url: "https://example.com".to_string(),
            range: 10..37,
        };

        // Helper function to check if cursor should expand the link
        let should_expand_link = |cursor_pos: usize, link: &MarkdownLink| -> bool {
            // Link should be expanded if cursor is inside or adjacent
            link.range.contains(&cursor_pos)
                || cursor_pos == link.range.start
                || cursor_pos == link.range.end
        };

        // Test cursor positions
        assert!(
            should_expand_link(10, &link),
            "Cursor at position 10 (link start) should expand link"
        );
        assert!(
            should_expand_link(37, &link),
            "Cursor at position 37 (link end) should expand link"
        );
        assert!(
            should_expand_link(20, &link),
            "Cursor at position 20 (inside link) should expand link"
        );
        assert!(
            !should_expand_link(9, &link),
            "Cursor at position 9 (before link) should not expand link"
        );
        assert!(
            !should_expand_link(38, &link),
            "Cursor at position 38 (after link) should not expand link"
        );

        // Test the edge cases
        assert_eq!(link.range.start, 10, "Link starts at position 10");
        assert_eq!(link.range.end, 37, "Link ends at position 37");
        assert!(link.range.contains(&10), "Range includes start position");
        assert!(!link.range.contains(&37), "Range excludes end position");
    }
}
