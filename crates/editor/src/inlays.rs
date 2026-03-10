//! The logic, responsible for managing [`Inlay`]s in the editor.
//!
//! Inlays are "not real" text that gets mixed into the "real" buffer's text.
//! They are attached to a certain [`Anchor`], and display certain contents (usually, strings)
//! between real text around that anchor.
//!
//! Inlay examples in Zed:
//! * inlay hints, received from LSP
//! * inline values, shown in the debugger
//! * inline predictions, showing the Zeta/Copilot/etc. predictions
//! * document color values, if configured to be displayed as inlays
//! * ... anything else, potentially.
//!
//! Editor uses [`crate::DisplayMap`] and [`crate::display_map::InlayMap`] to manage what's rendered inside the editor, using
//! [`InlaySplice`] to update this state.

/// Logic, related to managing LSP inlay hint inlays.
pub mod inlay_hints;

use std::ops::Range;
use std::sync::OnceLock;

use gpui::{Context, HighlightStyle, Hsla, Rgba, Task};
use multi_buffer::Anchor;
use project::{InlayHint, InlayId};
use text::Rope;

use crate::{Editor, HighlightKey, hover_links::InlayHighlight};

/// A splice to send into the `inlay_map` for updating the visible inlays on the screen.
/// "Visible" inlays may not be displayed in the buffer right away, but those are ready to be displayed on further buffer scroll, pane item activations, etc. right away without additional LSP queries or settings changes.
/// The data in the cache is never used directly for displaying inlays on the screen, to avoid races with updates from LSP queries and sync overhead.
/// Splice is picked to help avoid extra hint flickering and "jumps" on the screen.
#[derive(Debug, Default)]
pub struct InlaySplice {
    pub to_remove: Vec<InlayId>,
    pub to_insert: Vec<Inlay>,
}

impl InlaySplice {
    pub fn is_empty(&self) -> bool {
        self.to_remove.is_empty() && self.to_insert.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct Inlay {
    pub id: InlayId,
    pub position: Anchor,
    pub content: InlayContent,
}

#[derive(Debug, Clone)]
pub enum InlayContent {
    Text(text::Rope),
    Color(Hsla),
}

/// A bracket pair region within an inlay hint that can be collapsed.
/// For example, in `WeakMap<PgClient, pg.Pool>`, the region covers `<PgClient, pg.Pool>`.
#[derive(Debug, Clone)]
pub struct CollapsibleRegion {
    /// Byte range covering the opening bracket through the closing bracket (inclusive)
    /// in the original hint text (before any padding).
    pub range: Range<usize>,
    /// The opening bracket character.
    pub open_bracket: char,
    /// The closing bracket character.
    pub close_bracket: char,
}

/// Tracks collapse state for a single inlay hint.
#[derive(Debug, Clone)]
pub struct InlayHintCollapseState {
    /// The original full text of the hint (before padding).
    pub full_text: String,
    /// Whether the original text needed left padding.
    pub needs_left_padding: bool,
    /// Whether the original text needed right padding.
    pub needs_right_padding: bool,
    /// The detected collapsible regions, sorted by start position, non-overlapping.
    pub regions: Vec<CollapsibleRegion>,
    /// Indices of regions that are currently expanded.
    pub expanded_regions: Vec<bool>,
}

const ELLIPSIS: &str = "\u{2026}";

impl InlayHintCollapseState {
    /// Detect collapsible bracket regions in the hint text.
    /// Only detects top-level bracket pairs (not nested ones).
    pub fn detect(hint: &InlayHint) -> Option<Self> {
        let full_text = hint.text().to_string();
        let regions = detect_collapsible_regions(&full_text);
        if regions.is_empty() {
            return None;
        }
        let expanded_count = regions.len();
        let needs_left_padding = hint.padding_left && !full_text.starts_with(' ');
        let needs_right_padding = hint.padding_right && !full_text.ends_with(' ');
        Some(Self {
            full_text,
            needs_left_padding,
            needs_right_padding,
            regions,
            expanded_regions: vec![false; expanded_count],
        })
    }

    /// Generate the display text with collapsed regions replaced by ellipsis.
    pub fn display_text(&self) -> String {
        let mut result = String::new();
        let mut pos = 0;
        for (index, region) in self.regions.iter().enumerate() {
            result.push_str(&self.full_text[pos..region.range.start]);
            if self.expanded_regions[index] {
                result.push_str(&self.full_text[region.range.start..region.range.end]);
            } else {
                result.push(region.open_bracket);
                result.push_str(ELLIPSIS);
                result.push(region.close_bracket);
            }
            pos = region.range.end;
        }
        result.push_str(&self.full_text[pos..]);
        result
    }

    /// Generate the display Rope with padding applied.
    pub fn display_rope(&self) -> Rope {
        let display = self.display_text();
        let mut rope = Rope::from(display.as_str());
        if self.needs_right_padding {
            rope.push(" ");
        }
        if self.needs_left_padding {
            rope.push_front(" ");
        }
        rope
    }

    /// Given a byte offset in the display text (excluding padding), find which
    /// collapsible region contains that offset. Returns the region index if found.
    pub fn region_at_offset(&self, offset: usize) -> Option<usize> {
        let mut display_pos = 0;
        let mut prev_end = 0;
        for (index, region) in self.regions.iter().enumerate() {
            display_pos += region.range.start - prev_end;

            let region_display_len = if self.expanded_regions[index] {
                region.range.end - region.range.start
            } else {
                region.open_bracket.len_utf8() + ELLIPSIS.len() + region.close_bracket.len_utf8()
            };

            if offset >= display_pos && offset < display_pos + region_display_len {
                return Some(index);
            }
            display_pos += region_display_len;
            prev_end = region.range.end;
        }
        None
    }

    /// Toggle a specific region's collapsed/expanded state.
    pub fn toggle_region(&mut self, region_index: usize) {
        if region_index < self.expanded_regions.len() {
            self.expanded_regions[region_index] = !self.expanded_regions[region_index];
        }
    }

    /// Map a byte offset in the full/original text to the corresponding offset in the display text.
    /// If the offset falls inside a collapsed region, returns None.
    pub fn full_offset_to_display_offset(&self, offset: usize) -> Option<usize> {
        let mut display_pos = 0;
        let mut full_pos = 0;
        let mut prev_end = 0;
        for (index, region) in self.regions.iter().enumerate() {
            let prefix_len = region.range.start - prev_end;
            if offset < full_pos + prefix_len {
                return Some(display_pos + (offset - full_pos));
            }
            display_pos += prefix_len;
            full_pos += prefix_len;

            let region_len = region.range.end - region.range.start;
            if self.expanded_regions[index] {
                if offset < full_pos + region_len {
                    return Some(display_pos + (offset - full_pos));
                }
                display_pos += region_len;
            } else {
                let collapsed_len = region.open_bracket.len_utf8()
                    + ELLIPSIS.len()
                    + region.close_bracket.len_utf8();
                if offset < full_pos + region_len {
                    return None;
                }
                display_pos += collapsed_len;
            }
            full_pos += region_len;
            prev_end = region.range.end;
        }
        Some(display_pos + (offset - full_pos))
    }

    /// Map a byte offset in display text to the corresponding offset in the full text.
    /// Returns None if the offset falls on an ellipsis in a collapsed region.
    pub fn display_offset_to_full_offset(&self, offset: usize) -> Option<usize> {
        let mut display_pos = 0;
        let mut full_pos = 0;
        let mut prev_end = 0;
        for (index, region) in self.regions.iter().enumerate() {
            let prefix_len = region.range.start - prev_end;
            if offset < display_pos + prefix_len {
                return Some(full_pos + (offset - display_pos));
            }
            display_pos += prefix_len;
            full_pos += prefix_len;

            if self.expanded_regions[index] {
                let region_len = region.range.end - region.range.start;
                if offset < display_pos + region_len {
                    return Some(full_pos + (offset - display_pos));
                }
                display_pos += region_len;
                full_pos += region_len;
            } else {
                let collapsed_len = region.open_bracket.len_utf8()
                    + ELLIPSIS.len()
                    + region.close_bracket.len_utf8();
                if offset < display_pos + collapsed_len {
                    return None;
                }
                display_pos += collapsed_len;
                full_pos += region.range.end - region.range.start;
            }
            prev_end = region.range.end;
        }
        Some(full_pos + (offset - display_pos))
    }
}

/// Detect top-level bracket pairs in the given text.
/// Returns collapsible regions sorted by position.
fn detect_collapsible_regions(text: &str) -> Vec<CollapsibleRegion> {
    let opening_brackets: &[char] = &['<', '(', '{', '['];
    let closing_brackets: &[char] = &['>', ')', '}', ']'];
    let mut regions = Vec::new();
    let mut depth: usize = 0;
    let mut region_start = None;
    let mut top_open: Option<char> = None;
    let mut top_close: Option<char> = None;

    for (byte_offset, ch) in text.char_indices() {
        if let Some(idx) = opening_brackets.iter().position(|&b| b == ch) {
            if depth == 0 {
                region_start = Some(byte_offset);
                top_open = Some(opening_brackets[idx]);
                top_close = Some(closing_brackets[idx]);
            }
            depth += 1;
        } else if closing_brackets.contains(&ch) && depth > 0 {
            depth -= 1;
            if depth == 0 {
                if let (Some(start), Some(open), Some(expected_close)) =
                    (region_start, top_open, top_close)
                {
                    if ch == expected_close {
                        let end = byte_offset + ch.len_utf8();
                        let content_len = end - start - open.len_utf8() - ch.len_utf8();
                        if content_len > ELLIPSIS.len() {
                            regions.push(CollapsibleRegion {
                                range: start..end,
                                open_bracket: open,
                                close_bracket: ch,
                            });
                        }
                    }
                }
                region_start = None;
                top_open = None;
                top_close = None;
            }
        }
    }
    regions
}

impl Inlay {
    pub fn hint(id: InlayId, position: Anchor, hint: &InlayHint) -> Self {
        let mut text = hint.text();
        let needs_right_padding = hint.padding_right && !text.ends_with(" ");
        let needs_left_padding = hint.padding_left && !text.starts_with(" ");
        if needs_right_padding {
            text.push(" ");
        }
        if needs_left_padding {
            text.push_front(" ");
        }
        Self {
            id,
            position,
            content: InlayContent::Text(text),
        }
    }

    /// Create an inlay hint with collapsed bracket regions.
    pub fn hint_collapsed(id: InlayId, position: Anchor, collapse_state: &InlayHintCollapseState) -> Self {
        Self {
            id,
            position,
            content: InlayContent::Text(collapse_state.display_rope()),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn mock_hint(id: usize, position: Anchor, text: impl Into<Rope>) -> Self {
        Self {
            id: InlayId::Hint(id),
            position,
            content: InlayContent::Text(text.into()),
        }
    }

    pub fn color(id: usize, position: Anchor, color: Rgba) -> Self {
        Self {
            id: InlayId::Color(id),
            position,
            content: InlayContent::Color(color.into()),
        }
    }

    pub fn edit_prediction<T: Into<Rope>>(id: usize, position: Anchor, text: T) -> Self {
        Self {
            id: InlayId::EditPrediction(id),
            position,
            content: InlayContent::Text(text.into()),
        }
    }

    pub fn debugger<T: Into<Rope>>(id: usize, position: Anchor, text: T) -> Self {
        Self {
            id: InlayId::DebuggerValue(id),
            position,
            content: InlayContent::Text(text.into()),
        }
    }

    pub fn repl_result<T: Into<Rope>>(id: usize, position: Anchor, text: T) -> Self {
        Self {
            id: InlayId::ReplResult(id),
            position,
            content: InlayContent::Text(text.into()),
        }
    }

    pub fn text(&self) -> &Rope {
        static COLOR_TEXT: OnceLock<Rope> = OnceLock::new();
        match &self.content {
            InlayContent::Text(text) => text,
            InlayContent::Color(_) => COLOR_TEXT.get_or_init(|| Rope::from("◼")),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn get_color(&self) -> Option<Hsla> {
        match self.content {
            InlayContent::Color(color) => Some(color),
            _ => None,
        }
    }
}

pub struct InlineValueCache {
    pub enabled: bool,
    pub inlays: Vec<InlayId>,
    pub refresh_task: Task<Option<()>>,
}

impl InlineValueCache {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            inlays: Vec::new(),
            refresh_task: Task::ready(None),
        }
    }
}

impl Editor {
    /// Modify which hints are displayed in the editor.
    pub fn splice_inlays(
        &mut self,
        to_remove: &[InlayId],
        to_insert: Vec<Inlay>,
        cx: &mut Context<Self>,
    ) {
        if let Some(inlay_hints) = &mut self.inlay_hints {
            for id_to_remove in to_remove {
                inlay_hints.added_hints.remove(id_to_remove);
            }
        }
        for id_to_remove in to_remove {
            self.collapsed_inlay_hints.remove(id_to_remove);
        }
        self.display_map.update(cx, |display_map, cx| {
            display_map.splice_inlays(to_remove, to_insert, cx)
        });
        cx.notify();
    }

    pub(crate) fn highlight_inlays(
        &mut self,
        key: HighlightKey,
        highlights: Vec<InlayHighlight>,
        style: HighlightStyle,
        cx: &mut Context<Self>,
    ) {
        self.display_map
            .update(cx, |map, _| map.highlight_inlays(key, highlights, style));
        cx.notify();
    }

    pub fn inline_values_enabled(&self) -> bool {
        self.inline_value_cache.enabled
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn inline_value_inlays(&self, cx: &gpui::App) -> Vec<Inlay> {
        self.display_map
            .read(cx)
            .current_inlays()
            .filter(|inlay| matches!(inlay.id, InlayId::DebuggerValue(_)))
            .cloned()
            .collect()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn all_inlays(&self, cx: &gpui::App) -> Vec<Inlay> {
        self.display_map
            .read(cx)
            .current_inlays()
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_simple_generics() {
        let regions = detect_collapsible_regions("Result<OkType, ErrType>");
        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0].range, 6..23);
        assert_eq!(regions[0].open_bracket, '<');
        assert_eq!(regions[0].close_bracket, '>');
    }

    #[test]
    fn test_detect_nested_different_bracket_types() {
        let regions = detect_collapsible_regions("Vec<(usize, usize)>");
        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0].open_bracket, '<');
        assert_eq!(regions[0].close_bracket, '>');
    }

    #[test]
    fn test_detect_multiple_regions() {
        let regions = detect_collapsible_regions("fn(a: i32) -> Result<Ok, Err>");
        assert_eq!(regions.len(), 2);
        assert_eq!(regions[0].open_bracket, '(');
        assert_eq!(regions[1].open_bracket, '<');
    }

    #[test]
    fn test_no_collapse_short_content() {
        // Content "T" is shorter than "…" (3 bytes), so should not collapse
        let regions = detect_collapsible_regions("Vec<T>");
        assert_eq!(regions.len(), 0);
    }

    #[test]
    fn test_display_text_collapsed() {
        let state = InlayHintCollapseState {
            full_text: "Map<string, number>".to_string(),
            needs_left_padding: false,
            needs_right_padding: false,
            regions: vec![CollapsibleRegion {
                range: 3..19,
                open_bracket: '<',
                close_bracket: '>',
            }],
            expanded_regions: vec![false],
        };
        assert_eq!(state.display_text(), "Map<\u{2026}>");
    }

    #[test]
    fn test_display_text_expanded() {
        let state = InlayHintCollapseState {
            full_text: "Map<string, number>".to_string(),
            needs_left_padding: false,
            needs_right_padding: false,
            regions: vec![CollapsibleRegion {
                range: 3..19,
                open_bracket: '<',
                close_bracket: '>',
            }],
            expanded_regions: vec![true],
        };
        assert_eq!(state.display_text(), "Map<string, number>");
    }

    #[test]
    fn test_region_at_offset_collapsed() {
        let state = InlayHintCollapseState {
            full_text: "Map<string, number>".to_string(),
            needs_left_padding: false,
            needs_right_padding: false,
            regions: vec![CollapsibleRegion {
                range: 3..19,
                open_bracket: '<',
                close_bracket: '>',
            }],
            expanded_regions: vec![false],
        };
        // "Map<…>" - region starts at byte 3
        assert_eq!(state.region_at_offset(0), None); // 'M'
        assert_eq!(state.region_at_offset(3), Some(0)); // '<'
        assert_eq!(state.region_at_offset(4), Some(0)); // '…' (first byte)
        assert_eq!(state.region_at_offset(7), Some(0)); // '>'
        assert_eq!(state.region_at_offset(8), None); // past '>'
    }

    #[test]
    fn test_region_at_offset_expanded() {
        let state = InlayHintCollapseState {
            full_text: "Map<string, number>".to_string(),
            needs_left_padding: false,
            needs_right_padding: false,
            regions: vec![CollapsibleRegion {
                range: 3..19,
                open_bracket: '<',
                close_bracket: '>',
            }],
            expanded_regions: vec![true],
        };
        // "Map<string, number>" - region covers bytes 3..19
        assert_eq!(state.region_at_offset(3), Some(0)); // '<'
        assert_eq!(state.region_at_offset(10), Some(0)); // inside region
        assert_eq!(state.region_at_offset(18), Some(0)); // '>'
        assert_eq!(state.region_at_offset(19), None); // past region
    }

    #[test]
    fn test_display_offset_to_full_offset() {
        let state = InlayHintCollapseState {
            full_text: ": Result<OkType, ErrType>".to_string(),
            needs_left_padding: false,
            needs_right_padding: false,
            regions: vec![CollapsibleRegion {
                range: 8..25,
                open_bracket: '<',
                close_bracket: '>',
            }],
            expanded_regions: vec![false],
        };
        // Display: ": Result<…>"
        // Byte 0-7: ": Result" → maps 1:1
        assert_eq!(state.display_offset_to_full_offset(0), Some(0));
        assert_eq!(state.display_offset_to_full_offset(7), Some(7));
        // Byte 8: '<' in collapsed region → None
        assert_eq!(state.display_offset_to_full_offset(8), None);
        assert_eq!(state.display_offset_to_full_offset(9), None); // '…'
        // After collapsed region (byte 13 in display = byte 25 in full)
        assert_eq!(state.display_offset_to_full_offset(13), Some(25));
    }

    #[test]
    fn test_full_offset_to_display_offset() {
        let state = InlayHintCollapseState {
            full_text: ": Result<OkType, ErrType>".to_string(),
            needs_left_padding: false,
            needs_right_padding: false,
            regions: vec![CollapsibleRegion {
                range: 8..25,
                open_bracket: '<',
                close_bracket: '>',
            }],
            expanded_regions: vec![false],
        };
        // Full: ": Result<OkType, ErrType>"
        // Display: ": Result<…>"
        assert_eq!(state.full_offset_to_display_offset(0), Some(0));
        assert_eq!(state.full_offset_to_display_offset(7), Some(7));
        // Inside collapsed region → None
        assert_eq!(state.full_offset_to_display_offset(8), None);
        assert_eq!(state.full_offset_to_display_offset(15), None);
        // After region: full offset 25 → display offset 13
        assert_eq!(state.full_offset_to_display_offset(25), Some(13));
    }
}
