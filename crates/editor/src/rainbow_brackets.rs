use crate::{Editor, RangeToAnchorExt};
use gpui::{Context, HighlightStyle, Hsla, Window, hsla};
use indexmap::IndexMap;
use multi_buffer::{Anchor, MultiBufferSnapshot, ToOffset};
use settings::Settings;
use std::ops::Range;
use std::time::{Duration, Instant};

pub struct RainbowBracketTracker {
    enabled: bool,
    start_hue: f32,
    hue_step: f32,
    max_brackets: u32,
    pub(crate) nesting_levels: IndexMap<Range<Anchor>, u32>,
    // Cache: track buffer edit_count to avoid recalculating on scrolls
    cached_edit_count: Option<usize>,
    // Active bracket pair at cursor position
    pub(crate) active_pair: Option<(Range<Anchor>, Range<Anchor>)>,
    // Cache the last cursor position to avoid redundant active pair updates
    last_cursor_offset: Option<usize>,
    // Throttle active pair updates
    last_active_pair_update: Option<Instant>,
    // Cache for visible range to avoid recalculating all brackets
    cached_visible_range: Option<Range<usize>>,
}

impl RainbowBracketTracker {
    pub fn new(enabled: bool, start_hue: f32, hue_step: f32, max_brackets: u32) -> Self {
        Self {
            enabled,
            start_hue,
            hue_step,
            max_brackets,
            nesting_levels: IndexMap::new(),
            cached_edit_count: None,
            active_pair: None,
            last_cursor_offset: None,
            last_active_pair_update: None,
            cached_visible_range: None,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            // Clear all state when disabled
            self.nesting_levels.clear();
            self.active_pair = None;
            self.last_cursor_offset = None;
            self.cached_edit_count = None;
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_color_for_level(&self, level: u32) -> Hsla {
        let hue = (self.start_hue + (level as f32 * self.hue_step)) % 360.0;
        hsla(hue / 360.0, 0.75, 0.6, 1.0)
    }

    pub fn update_active_pair(&mut self, cursor_position: Anchor, buffer: &MultiBufferSnapshot) {
        if !self.enabled {
            self.active_pair = None;
            return;
        }

        // Early exit if cursor hasn't moved significantly
        let cursor_offset = cursor_position.to_offset(buffer);
        if let Some(last_offset) = self.last_cursor_offset {
            if cursor_offset == last_offset {
                return; // Cursor hasn't moved, no need to update
            }
        }

        // Throttle active pair updates to avoid excessive computation during rapid cursor movement
        if let Some(last_update) = self.last_active_pair_update {
            if last_update.elapsed() < Duration::from_millis(10) {
                return; // Skip update if too soon
            }
        }

        self.last_cursor_offset = Some(cursor_offset);
        self.last_active_pair_update = Some(Instant::now());

        // Try to find enclosing brackets first (most common case)
        if let Some((open_range, close_range)) =
            buffer.innermost_enclosing_bracket_ranges(cursor_position..cursor_position, None)
        {
            self.active_pair = Some((
                open_range.to_anchors(buffer),
                close_range.to_anchors(buffer),
            ));
            return;
        }

        // Check if cursor is adjacent to a bracket
        // Only check immediate neighbors, not all brackets
        let buffer_len = buffer.len();
        if buffer_len == 0 {
            self.active_pair = None;
            return;
        }

        // Check positions around cursor (before and at cursor)
        let positions_to_check = [cursor_offset.saturating_sub(1), cursor_offset];

        for &check_offset in &positions_to_check {
            if check_offset >= buffer_len {
                continue;
            }

            // Create a safe range for bracket checking
            let range_end = (check_offset + 1).min(buffer_len);
            if check_offset >= range_end {
                continue;
            }

            // Check if there's a bracket at this position
            if let Some(pairs) = buffer.bracket_ranges(check_offset..range_end) {
                if let Some((open, close)) = pairs.into_iter().next() {
                    // Found a bracket at or near cursor
                    if open.start == check_offset || close.start == check_offset {
                        self.active_pair =
                            Some((open.to_anchors(buffer), close.to_anchors(buffer)));
                        return;
                    }
                }
            }
        }

        self.active_pair = None;
    }

    pub fn update_brackets(
        &mut self,
        buffer: &MultiBufferSnapshot,
        visible_range: Option<Range<usize>>,
    ) {
        if !self.enabled {
            self.nesting_levels.clear();
            self.cached_edit_count = None;
            return;
        }

        // Only recalculate if buffer has changed (text edits), not on scrolls
        let current_edit_count = buffer.edit_count();

        // Check if we can use cached data
        if let Some(cached_count) = self.cached_edit_count {
            if cached_count == current_edit_count {
                // Buffer hasn't changed
                if let Some(ref visible) = visible_range {
                    if let Some(ref cached_visible) = self.cached_visible_range {
                        // If visible range hasn't changed significantly, skip update
                        if cached_visible.start.saturating_sub(1000) <= visible.start
                            && cached_visible.end.saturating_add(1000) >= visible.end
                        {
                            return;
                        }
                    }
                }
            }
        }

        // For large files, only process visible range plus a buffer
        let range_to_process = if let Some(ref visible) = visible_range {
            // Add padding around visible range for smoother scrolling
            let padding = 5000; // Characters of padding
            let start = visible.start.saturating_sub(padding);
            let end = (visible.end + padding).min(buffer.len());
            start..end
        } else {
            // Fall back to entire buffer for small files or when no visible range
            0..buffer.len()
        };

        // Limit processing for very large ranges
        if range_to_process.len() > 100_000 {
            // For extremely large ranges, skip rainbow brackets entirely
            self.nesting_levels.clear();
            return;
        }

        if let Some(pairs) = buffer.bracket_ranges(range_to_process.clone()) {
            self.nesting_levels.clear();

            // Limit the number of bracket pairs we process
            let pairs: Vec<_> = pairs.into_iter().take(self.max_brackets as usize).collect();

            // Collect all brackets: (position, is_opening, range_for_storage)
            let mut brackets: Vec<(usize, bool, Range<usize>)> = Vec::new();

            for (open_range, close_range) in pairs {
                brackets.push((open_range.start, true, open_range));
                brackets.push((close_range.start, false, close_range));
            }

            // Sort by position to process in document order
            brackets.sort_by_key(|(pos, _, _)| *pos);

            // Process brackets with stack to determine nesting levels
            let mut stack: Vec<Range<usize>> = Vec::new();

            for (_pos, is_opening, bracket_range) in brackets {
                if is_opening {
                    // Opening bracket: current stack depth = nesting level
                    let level = stack.len() as u32;
                    let anchor_range = bracket_range.clone().to_anchors(buffer);
                    self.nesting_levels.insert(anchor_range, level);
                    stack.push(bracket_range);
                } else {
                    // Closing bracket: level same as matching opening
                    let level = if !stack.is_empty() {
                        (stack.len() - 1) as u32
                    } else {
                        0 // Unmatched closing bracket
                    };
                    let anchor_range = bracket_range.clone().to_anchors(buffer);
                    self.nesting_levels.insert(anchor_range, level);

                    // Pop the matching opening bracket
                    if !stack.is_empty() {
                        stack.pop();
                    }
                }
            }

            // Update caches
            self.cached_edit_count = Some(current_edit_count);
            self.cached_visible_range = visible_range;
        }
    }

    #[cfg(test)]
    pub fn get_bracket_highlights(&self) -> Vec<(Range<Anchor>, HighlightStyle)> {
        if !self.enabled {
            return Vec::new();
        }

        self.nesting_levels
            .iter()
            .map(|(range, level)| {
                let color = self.get_color_for_level(*level);
                (
                    range.clone(),
                    HighlightStyle {
                        color: Some(color),
                        ..Default::default()
                    },
                )
            })
            .collect()
    }
}

// GPUI limitation: We need separate highlight types for each color level since
// highlight_text uses insert() which replaces previous highlights of the same type.
// This limits us to 12 unique colors maximum.
enum RainbowBracketHighlight0 {}
enum RainbowBracketHighlight1 {}
enum RainbowBracketHighlight2 {}
enum RainbowBracketHighlight3 {}
enum RainbowBracketHighlight4 {}
enum RainbowBracketHighlight5 {}
enum RainbowBracketHighlight6 {}
enum RainbowBracketHighlight7 {}
enum RainbowBracketHighlight8 {}
enum RainbowBracketHighlight9 {}
enum RainbowBracketHighlight10 {}
enum RainbowBracketHighlight11 {}

// Special highlight type for the active bracket pair at cursor
enum ActiveBracketHighlight {}

pub fn refresh_rainbow_brackets(
    editor: &mut Editor,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    use crate::editor_settings::EditorSettings;
    use gpui::FontWeight;

    // Early exit if disabled
    if !editor.rainbow_bracket_tracker.is_enabled() {
        return;
    }

    // Clear all highlight types
    editor.clear_highlights::<RainbowBracketHighlight0>(cx);
    editor.clear_highlights::<RainbowBracketHighlight1>(cx);
    editor.clear_highlights::<RainbowBracketHighlight2>(cx);
    editor.clear_highlights::<RainbowBracketHighlight3>(cx);
    editor.clear_highlights::<RainbowBracketHighlight4>(cx);
    editor.clear_highlights::<RainbowBracketHighlight5>(cx);
    editor.clear_highlights::<RainbowBracketHighlight6>(cx);
    editor.clear_highlights::<RainbowBracketHighlight7>(cx);
    editor.clear_highlights::<RainbowBracketHighlight8>(cx);
    editor.clear_highlights::<RainbowBracketHighlight9>(cx);
    editor.clear_highlights::<RainbowBracketHighlight10>(cx);
    editor.clear_highlights::<RainbowBracketHighlight11>(cx);
    editor.clear_highlights::<ActiveBracketHighlight>(cx);

    // Update settings
    let settings = EditorSettings::get_global(cx);
    let max_brackets = settings.rainbow_brackets.max_brackets as usize;

    editor
        .rainbow_bracket_tracker
        .set_enabled(settings.rainbow_brackets.enabled);

    if !settings.rainbow_brackets.enabled {
        return; // Exit early if disabled
    }

    editor.rainbow_bracket_tracker.start_hue = settings.rainbow_brackets.start_hue;
    editor.rainbow_bracket_tracker.hue_step = settings.rainbow_brackets.hue_step;
    editor.rainbow_bracket_tracker.max_brackets = settings.rainbow_brackets.max_brackets;

    let snapshot = editor.snapshot(window, cx);
    let buffer = snapshot.buffer_snapshot();

    // For large files, limit processing to avoid performance issues
    let buffer_len = buffer.len();
    let visible_buffer_range = if buffer_len > 100_000 {
        // For very large files (>100K chars), process only a limited range
        // This prevents performance issues while still providing some functionality
        Some(0..50_000)
    } else {
        // For normal-sized files, process everything
        None
    };

    // Update bracket tracking only for visible range (with padding)
    editor
        .rainbow_bracket_tracker
        .update_brackets(&buffer, visible_buffer_range);

    // Update active bracket pair based on cursor position
    let cursor_position = editor.selections.newest_anchor().head();
    editor
        .rainbow_bracket_tracker
        .update_active_pair(cursor_position, &buffer);

    // Performance safety: Don't highlight an excessive number of brackets
    if editor.rainbow_bracket_tracker.nesting_levels.len() > max_brackets {
        return;
    }

    // Group brackets by level (mod 12) for separate highlight types
    let mut by_level: [Vec<Range<Anchor>>; 12] = Default::default();

    for (range, &level) in &editor.rainbow_bracket_tracker.nesting_levels {
        let color_index = (level % 12) as usize;
        by_level[color_index].push(range.clone());
    }

    // Apply highlights using different type for each level
    macro_rules! apply_level {
        ($index:literal, $type:ty) => {
            if !by_level[$index].is_empty() {
                let level = $index as u32;
                let color = editor.rainbow_bracket_tracker.get_color_for_level(level);
                editor.highlight_text::<$type>(
                    by_level[$index].clone(),
                    HighlightStyle {
                        color: Some(color),
                        ..Default::default()
                    },
                    cx,
                );
            }
        };
    }

    apply_level!(0, RainbowBracketHighlight0);
    apply_level!(1, RainbowBracketHighlight1);
    apply_level!(2, RainbowBracketHighlight2);
    apply_level!(3, RainbowBracketHighlight3);
    apply_level!(4, RainbowBracketHighlight4);
    apply_level!(5, RainbowBracketHighlight5);
    apply_level!(6, RainbowBracketHighlight6);
    apply_level!(7, RainbowBracketHighlight7);
    apply_level!(8, RainbowBracketHighlight8);
    apply_level!(9, RainbowBracketHighlight9);
    apply_level!(10, RainbowBracketHighlight10);
    apply_level!(11, RainbowBracketHighlight11);

    // Highlight the active bracket pair with enhanced visibility
    if let Some((open_range, close_range)) = &editor.rainbow_bracket_tracker.active_pair {
        // Get the base color for the active brackets
        let mut base_color = hsla(0.0, 0.0, 0.9, 1.0); // Default to bright white

        // Try to use the bracket's rainbow color but make it brighter
        for (range, level) in &editor.rainbow_bracket_tracker.nesting_levels {
            if range == open_range || range == close_range {
                let original_color = editor.rainbow_bracket_tracker.get_color_for_level(*level);
                // Make the color brighter and more saturated for active brackets
                base_color = hsla(
                    original_color.h,
                    1.0, // Max saturation
                    0.8, // Brighter lightness
                    1.0,
                );
                break;
            }
        }

        let active_style = HighlightStyle {
            color: Some(base_color),
            font_weight: Some(FontWeight::BOLD),
            ..Default::default()
        };

        editor.highlight_text::<ActiveBracketHighlight>(
            vec![open_range.clone(), close_range.clone()],
            active_style,
            cx,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_calculation_start_hue() {
        let tracker = RainbowBracketTracker::new(true, 0.0, 60.0, 100000);

        let color_0 = tracker.get_color_for_level(0);
        assert_eq!(color_0.h, 0.0, "Level 0 should start at hue 0 (red)");
        assert_eq!(color_0.s, 0.75, "Saturation should be 0.75");
        assert_eq!(color_0.l, 0.6, "Lightness should be 0.6");
        assert_eq!(color_0.a, 1.0, "Alpha should be 1.0");
    }

    #[test]
    fn test_color_calculation_step() {
        let tracker = RainbowBracketTracker::new(true, 0.0, 60.0, 100000);

        let color_1 = tracker.get_color_for_level(1);
        let expected_hue_1 = 60.0 / 360.0;
        assert!(
            (color_1.h - expected_hue_1).abs() < 0.001,
            "Level 1 should be at 60 degrees (yellow)"
        );

        let color_2 = tracker.get_color_for_level(2);
        let expected_hue_2 = 120.0 / 360.0;
        assert!(
            (color_2.h - expected_hue_2).abs() < 0.001,
            "Level 2 should be at 120 degrees (green)"
        );
    }

    #[test]
    fn test_color_wraps_at_360() {
        let tracker = RainbowBracketTracker::new(true, 0.0, 60.0, 100000);

        let color_6 = tracker.get_color_for_level(6);
        assert!(
            (color_6.h - 0.0).abs() < 0.001,
            "Level 6 (360 degrees) should wrap back to red"
        );

        let color_7 = tracker.get_color_for_level(7);
        let expected_hue_7 = 60.0 / 360.0;
        assert!(
            (color_7.h - expected_hue_7).abs() < 0.001,
            "Level 7 should wrap to 60 degrees (yellow)"
        );
    }

    #[test]
    fn test_supports_unlimited_nesting() {
        let tracker = RainbowBracketTracker::new(true, 0.0, 30.0, 100000);

        let color_100 = tracker.get_color_for_level(100);
        assert!(
            color_100.h >= 0.0 && color_100.h < 1.0,
            "Should produce valid hue for deep nesting"
        );

        let color_1000 = tracker.get_color_for_level(1000);
        assert!(
            color_1000.h >= 0.0 && color_1000.h < 1.0,
            "Should produce valid hue for very deep nesting"
        );
    }

    #[test]
    fn test_custom_start_hue() {
        let tracker = RainbowBracketTracker::new(true, 180.0, 30.0, 100000);

        let color = tracker.get_color_for_level(0);
        assert!(
            (color.h - 180.0 / 360.0).abs() < 0.001,
            "Should start at cyan (180 degrees)"
        );
    }

    #[test]
    fn test_custom_step() {
        let tracker = RainbowBracketTracker::new(true, 0.0, 45.0, 100000);

        let color_1 = tracker.get_color_for_level(1);
        assert!(
            (color_1.h - 45.0 / 360.0).abs() < 0.001,
            "Should step by 45 degrees"
        );

        let color_8 = tracker.get_color_for_level(8);
        assert!(
            (color_8.h - 0.0).abs() < 0.001,
            "8 * 45 = 360, should wrap to 0"
        );
    }

    #[test]
    fn test_disabled_tracker_returns_empty_highlights() {
        let tracker = RainbowBracketTracker::new(false, 0.0, 30.0, 100000);
        assert!(!tracker.is_enabled(), "Tracker should be disabled");

        let highlights = tracker.get_bracket_highlights();
        assert!(
            highlights.is_empty(),
            "Disabled tracker should return no highlights"
        );
    }

    #[test]
    fn test_can_toggle_enabled() {
        let mut tracker = RainbowBracketTracker::new(true, 0.0, 30.0, 100000);
        assert!(tracker.is_enabled());

        tracker.set_enabled(false);
        assert!(!tracker.is_enabled());

        tracker.set_enabled(true);
        assert!(tracker.is_enabled());
    }

    #[test]
    fn test_active_pair_detection() {
        use gpui::{App, Entity};
        use language::Language;
        use multi_buffer::MultiBuffer;

        let mut cx = App::test();
        let buffer = cx.new(|cx| {
            let mut buffer = MultiBuffer::new(0, language::Capability::ReadWrite);
            buffer.push_text("{ [ ( ) ] }", cx);
            buffer
        });

        // Create a tracker
        let mut tracker = RainbowBracketTracker::new(true, 0.0, 60.0, 100000);

        // Test with cursor at different positions
        cx.update(|cx| {
            let snapshot = buffer.read(cx).snapshot();

            // Update brackets first
            tracker.update_brackets(&snapshot, None);

            // Test cursor inside innermost brackets
            let cursor_at_4 = snapshot.anchor_before(4); // Position after first (
            tracker.update_active_pair(cursor_at_4, &snapshot);
            assert!(
                tracker.active_pair.is_some(),
                "Should detect active pair when cursor inside brackets"
            );

            // Test cursor outside all brackets
            let cursor_at_0 = snapshot.anchor_before(0); // Position at start
            tracker.update_active_pair(cursor_at_0, &snapshot);
            // This might be None or might detect the adjacent bracket

            // Test with disabled tracker
            tracker.set_enabled(false);
            tracker.update_active_pair(cursor_at_4, &snapshot);
            assert!(
                tracker.active_pair.is_none(),
                "Should not detect active pair when disabled"
            );
        });
    }
}
