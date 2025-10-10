use crate::{Editor, RangeToAnchorExt};
use gpui::{Context, HighlightStyle, Hsla, Window, hsla};
use indexmap::IndexMap;
use multi_buffer::{Anchor, MultiBufferSnapshot};
use settings::Settings;
use std::ops::Range;

pub struct RainbowBracketTracker {
    enabled: bool,
    start_hue: f32,
    hue_step: f32,
    max_brackets: u32,
    pub(crate) nesting_levels: IndexMap<Range<Anchor>, u32>,
    // Cache: track buffer edit_count to avoid recalculating on scrolls
    cached_edit_count: Option<usize>,
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
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_color_for_level(&self, level: u32) -> Hsla {
        let hue = (self.start_hue + (level as f32 * self.hue_step)) % 360.0;
        hsla(hue / 360.0, 0.75, 0.6, 1.0)
    }

    pub fn update_brackets(&mut self, buffer: &MultiBufferSnapshot) {
        if !self.enabled {
            self.nesting_levels.clear();
            self.cached_edit_count = None;
            return;
        }

        // Only recalculate if buffer has changed (text edits), not on scrolls
        let current_edit_count = buffer.edit_count();
        if self.cached_edit_count == Some(current_edit_count) {
            return;
        }

        // Calculate nesting levels for the entire buffer to ensure consistent colors
        let entire_buffer_range = 0..buffer.len();

        if let Some(pairs) = buffer.bracket_ranges(entire_buffer_range.clone()) {
            self.nesting_levels.clear();

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

            // Update cache if we found brackets
            if !self.nesting_levels.is_empty() {
                self.cached_edit_count = Some(current_edit_count);
            }
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

pub fn refresh_rainbow_brackets(
    editor: &mut Editor,
    _window: &mut Window,
    cx: &mut Context<Editor>,
) {
    use crate::editor_settings::EditorSettings;

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

    if !editor.rainbow_bracket_tracker.is_enabled() {
        return;
    }

    // Update settings
    let settings = EditorSettings::get_global(cx);
    let max_brackets = settings.rainbow_brackets.max_brackets as usize;

    editor
        .rainbow_bracket_tracker
        .set_enabled(settings.rainbow_brackets.enabled);
    editor.rainbow_bracket_tracker.start_hue = settings.rainbow_brackets.start_hue;
    editor.rainbow_bracket_tracker.hue_step = settings.rainbow_brackets.hue_step;
    editor.rainbow_bracket_tracker.max_brackets = settings.rainbow_brackets.max_brackets;

    let snapshot = editor.snapshot(_window, cx);
    let buffer = snapshot.buffer_snapshot();

    // Update bracket tracking for the entire buffer
    editor.rainbow_bracket_tracker.update_brackets(&buffer);

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
}
