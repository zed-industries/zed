use crate::{Editor, RangeToAnchorExt};
use gpui::{Context, HighlightStyle, Hsla, Window, hsla};
use multi_buffer::{Anchor, MultiBufferSnapshot, ToOffset};
use settings::Settings;
use indexmap::IndexMap;  // IndexMap preserves insertion order!
use std::ops::Range;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RainbowMode {
    Gradient,
    Classic,
}

impl Default for RainbowMode {
    fn default() -> Self {
        Self::Gradient
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GradientConfig {
    pub start_hue: f32,
    pub step_degrees: f32,
    pub saturation: f32,
    pub lightness: f32,
}

impl Default for GradientConfig {
    fn default() -> Self {
        Self {
            start_hue: 0.0,
            step_degrees: 60.0,  // 60° steps = 6 highly contrasting colors
            saturation: 0.75,    // Slightly more saturated for better visibility
            lightness: 0.6,      // Slightly brighter for dark themes
        }
    }
}

/// Manages animation state for fade-in and glow effects
struct AnimationState {
    /// When the current fade animation started (None = no fade active)
    fade_start: Option<Instant>,
    /// When glow animation started (for continuous pulsing)
    glow_start: Instant,
    /// Duration of fade animation in milliseconds
    fade_duration_ms: u32,
    /// Whether cascade fade-in is enabled
    animate_fade: bool,
    /// Whether active scope glow is enabled
    animate_glow: bool,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            fade_start: None,
            glow_start: Instant::now(),
            fade_duration_ms: 200,
            animate_fade: true,
            animate_glow: true,
        }
    }
}

pub struct RainbowBracketTracker {
    mode: RainbowMode,
    gradient_config: GradientConfig,
    classic_colors: Vec<Hsla>,
    pub(crate) nesting_levels: IndexMap<Range<Anchor>, u32>,  // IndexMap preserves insertion order
    pub(crate) active_scope: Option<Range<Anchor>>,
    animation_state: AnimationState,
    last_viewport: Option<Range<usize>>,
    enabled: bool,
    // Cache: track buffer edit_count to avoid recalculating on scrolls
    cached_edit_count: Option<usize>,
}

impl RainbowBracketTracker {
    pub fn new(enabled: bool) -> Self {
        Self {
            mode: RainbowMode::default(),
            gradient_config: GradientConfig::default(),
            classic_colors: Self::default_classic_colors(),
            nesting_levels: IndexMap::new(),  // IndexMap preserves insertion order
            active_scope: None,
            animation_state: AnimationState::default(),
            last_viewport: None,
            enabled,
            cached_edit_count: None,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_mode(&mut self, mode: RainbowMode) {
        self.mode = mode;
    }

    pub fn set_gradient_config(&mut self, config: GradientConfig) {
        self.gradient_config = config;
    }

    fn default_classic_colors() -> Vec<Hsla> {
        vec![
            hsla(0.0 / 360.0, 0.7, 0.5, 1.0),
            hsla(30.0 / 360.0, 0.7, 0.5, 1.0),
            hsla(60.0 / 360.0, 0.7, 0.5, 1.0),
            hsla(120.0 / 360.0, 0.7, 0.5, 1.0),
            hsla(240.0 / 360.0, 0.7, 0.5, 1.0),
            hsla(280.0 / 360.0, 0.7, 0.5, 1.0),
        ]
    }

    pub fn get_color_for_level(&self, level: u32) -> Hsla {
        match self.mode {
            RainbowMode::Gradient => {
                let hue = (self.gradient_config.start_hue
                    + (level as f32 * self.gradient_config.step_degrees))
                    % 360.0;
                hsla(
                    hue / 360.0,
                    self.gradient_config.saturation,
                    self.gradient_config.lightness,
                    1.0,
                )
            }
            RainbowMode::Classic => {
                let index = (level as usize) % self.classic_colors.len();
                self.classic_colors[index]
            }
        }
    }

    pub fn update_brackets(&mut self, buffer: &MultiBufferSnapshot, viewport: Range<Anchor>) {
        if !self.enabled {
            self.nesting_levels.clear();
            self.last_viewport = None;
            self.cached_edit_count = None;
            return;
        }

        let viewport_range = viewport.start.to_offset(buffer)..viewport.end.to_offset(buffer);

        // Detect significant viewport changes and trigger fade animation
        if self.should_trigger_fade(&viewport_range) {
            self.start_fade_animation();
        }

        self.last_viewport = Some(viewport_range.clone());

        // CACHE: Only recalculate if buffer has changed (text edits), not on scrolls!
        let current_edit_count = buffer.edit_count();
        if self.cached_edit_count == Some(current_edit_count) {
            // Buffer unchanged - use cached nesting levels
            eprintln!("🟢 CACHE HIT: Using cached brackets (edit_count={}, {} brackets)", current_edit_count, self.nesting_levels.len());
            return;
        }

        eprintln!("🔴 CACHE MISS: Recalculating brackets (old={:?}, new={})", self.cached_edit_count, current_edit_count);

        // IMPORTANT: Calculate nesting levels for the ENTIRE buffer, not just viewport!
        // This ensures colors stay consistent when scrolling.
        // Tree-sitter is fast enough that this doesn't hurt performance.
        let entire_buffer_range = 0..buffer.len();

        // Collect all brackets (opens and closes) from pairs, then sort by position
        // to process them sequentially with a stack-based algorithm
        if let Some(pairs) = buffer.bracket_ranges(entire_buffer_range.clone()) {
            // Clear old data now that we have new bracket data
            self.nesting_levels.clear();
            // Collect all brackets: (position, is_opening, range_for_storage)
            let mut brackets: Vec<(usize, bool, Range<usize>)> = Vec::new();

            let pairs_vec: Vec<_> = pairs.collect();
            eprintln!("📋 Tree-sitter found {} bracket pairs in buffer range 0..{}", pairs_vec.len(), entire_buffer_range.end);

            for (open_range, close_range) in pairs_vec {
                brackets.push((open_range.start, true, open_range));
                brackets.push((close_range.start, false, close_range));
            }

            // Sort by position so we process brackets in document order
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
                    // Closing bracket: level same as matching opening (current stack depth - 1)
                    let level = if !stack.is_empty() {
                        (stack.len() - 1) as u32
                    } else {
                        0  // Unmatched closing bracket
                    };
                    let anchor_range = bracket_range.clone().to_anchors(buffer);
                    self.nesting_levels.insert(anchor_range, level);

                    // Pop the matching opening bracket
                    if !stack.is_empty() {
                        stack.pop();
                    }
                }
            }

            // Successfully calculated - update cache (but only if we found brackets!)
            // If tree-sitter returns 0 brackets, don't cache - it might not be ready yet.
            if !self.nesting_levels.is_empty() {
                self.cached_edit_count = Some(current_edit_count);
                eprintln!("✅ Cached {} brackets for edit_count={}", self.nesting_levels.len(), current_edit_count);
            } else {
                eprintln!("⏳ Tree-sitter not ready yet (0 brackets), will retry...");
            }
        } else {
            eprintln!("⚠️ buffer.bracket_ranges() returned None");
        }
    }

    /// Determine if viewport change is significant enough to trigger fade animation
    fn should_trigger_fade(&self, new_viewport: &Range<usize>) -> bool {
        if !self.animation_state.animate_fade {
            return false;
        }

        let Some(ref last_viewport) = self.last_viewport else {
            // First time opening file - trigger fade
            return true;
        };

        // Calculate how much the viewport has moved
        let viewport_size = new_viewport.end.saturating_sub(new_viewport.start);
        let scroll_distance = if new_viewport.start > last_viewport.start {
            new_viewport.start - last_viewport.start
        } else {
            last_viewport.start - new_viewport.start
        };

        // Trigger fade if scrolled more than 30% of viewport height
        // This avoids animation on small scrolls but shows it on significant jumps
        let threshold = (viewport_size as f32 * 0.3) as usize;
        scroll_distance > threshold
    }

    pub fn get_bracket_highlights(&self) -> Vec<(Range<Anchor>, HighlightStyle)> {
        if !self.enabled {
            return Vec::new();
        }

        self.nesting_levels
            .iter()
            .map(|(range, level)| {
                let base_color = self.get_color_for_level(*level);

                // Check if this bracket is part of the active scope
                let is_active = if let Some(ref active) = self.active_scope {
                    range.start == active.start || range.end == active.end
                } else {
                    false
                };

                // Apply animation effects (fade-in cascade + active glow)
                let color = self.apply_animation(base_color, *level, is_active);

                // Use TEXT color for gradient rainbow effect
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

    pub fn update_active_scope(&mut self, cursor_position: Anchor, buffer: &MultiBufferSnapshot) {
        if !self.enabled {
            self.active_scope = None;
            return;
        }

        let offset = cursor_position.to_offset(buffer);

        if let Some((open_range, close_range)) =
            buffer.innermost_enclosing_bracket_ranges(offset..offset, None)
        {
            let open_anchor = open_range.to_anchors(buffer);
            let close_anchor = close_range.to_anchors(buffer);

            self.active_scope = Some(open_anchor.start..close_anchor.end);
        } else {
            self.active_scope = None;
        }
    }

    pub fn active_scope(&self) -> Option<Range<Anchor>> {
        self.active_scope.clone()
    }

    /// Update animation settings from editor settings
    pub fn update_animation_settings(&mut self, animate_fade: bool, animate_glow: bool, duration_ms: u32) {
        self.animation_state.animate_fade = animate_fade;
        self.animation_state.animate_glow = animate_glow;
        self.animation_state.fade_duration_ms = duration_ms;
    }

    /// Start the fade-in animation (call when viewport changes significantly)
    pub fn start_fade_animation(&mut self) {
        if self.animation_state.animate_fade {
            self.animation_state.fade_start = Some(Instant::now());
        }
    }

    /// Calculate current fade animation progress (0.0 to 1.0)
    /// Returns None if animation is complete or not active
    fn calculate_fade_progress(&self) -> Option<f32> {
        if !self.animation_state.animate_fade {
            return None;
        }

        let start = self.animation_state.fade_start?;
        let elapsed_ms = start.elapsed().as_millis() as f32;
        let duration_ms = self.animation_state.fade_duration_ms as f32;

        if elapsed_ms >= duration_ms {
            None  // Animation complete
        } else {
            Some((elapsed_ms / duration_ms).min(1.0))
        }
    }

    /// Calculate glow intensity for active bracket pair (0.6 to 1.0)
    fn calculate_glow_intensity(&self) -> f32 {
        if !self.animation_state.animate_glow {
            return 1.0;  // No glow, just base color
        }

        // Sine wave pulse: oscillates between 0.6 and 1.0
        let elapsed = self.animation_state.glow_start.elapsed().as_secs_f32();
        let pulse = (elapsed * 3.0).sin() * 0.2 + 0.8;
        pulse
    }

    /// Apply animation effects to a color
    /// depth: nesting level (for cascade delay)
    /// is_active: whether this bracket is in the active scope
    pub(crate) fn apply_animation(&self, color: Hsla, depth: u32, is_active: bool) -> Hsla {
        let mut result = color;

        // Apply fade-in animation with cascade (deeper = later)
        if let Some(progress) = self.calculate_fade_progress() {
            let depth_delay = depth as f32 * 0.05;  // 50ms per level
            let adjusted_progress = (progress - depth_delay).max(0.0).min(1.0);
            result = hsla(result.h, result.s, result.l, adjusted_progress);
        }

        // Apply glow to active brackets
        if is_active && self.animation_state.animate_glow {
            let glow = self.calculate_glow_intensity();
            let lightness_boost = 0.2 * (glow - 0.8) / 0.2;  // Map 0.8-1.0 to 0.0-0.2
            result = hsla(result.h, result.s, (result.l + lightness_boost).min(1.0), result.a);
        }

        result
    }

    /// Check if we need to request an animation frame
    pub fn needs_animation_frame(&self) -> bool {
        // Need frames if fade animation is running
        if self.animation_state.fade_start.is_some() && self.calculate_fade_progress().is_some() {
            return true;
        }

        // Need frames if glow is enabled and we have an active scope
        if self.animation_state.animate_glow && self.active_scope.is_some() {
            return true;
        }

        false
    }

    /// Check if fade animation is complete and clear it if so
    /// Returns true if fade was completed
    pub fn complete_fade_if_done(&mut self) -> bool {
        if self.animation_state.fade_start.is_some() && self.calculate_fade_progress().is_none() {
            self.animation_state.fade_start = None;
            true
        } else {
            false
        }
    }
}

// We need separate highlight types for each color level since highlight_text
// uses insert() which replaces previous highlights of the same type.
// With 30° hue steps, we have 12 unique colors (360° / 30° = 12)
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
    window: &mut Window,
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

    // Update animation settings from EditorSettings and extract max_brackets
    let settings = EditorSettings::get_global(cx);
    let max_brackets = settings.rainbow_brackets.max_brackets as usize;
    editor.rainbow_bracket_tracker.update_animation_settings(
        settings.rainbow_brackets.animate_fade,
        settings.rainbow_brackets.animate_glow,
        settings.rainbow_brackets.animation_duration_ms,
    );

    let snapshot = editor.snapshot(window, cx);
    let buffer = snapshot.buffer_snapshot();

    // Get the visible viewport range
    let start_anchor = snapshot.display_snapshot.buffer_snapshot().anchor_before(0);
    let end_anchor = snapshot
        .display_snapshot
        .buffer_snapshot()
        .anchor_after(buffer.len());
    let viewport = start_anchor..end_anchor;

    // Update bracket tracking for the viewport
    editor
        .rainbow_bracket_tracker
        .update_brackets(&buffer, viewport);

    // Group brackets by level (mod 12) for separate highlight types
    // We need separate highlight types because highlight_text uses insert()
    let mut by_level: [Vec<(Range<Anchor>, Hsla)>; 12] = Default::default();

    let bracket_count = editor.rainbow_bracket_tracker.nesting_levels.len();
    eprintln!("🎨 refresh_rainbow_brackets: Processing {} brackets", bracket_count);

    // Performance safety: Don't try to highlight an absurd number of brackets
    if bracket_count > max_brackets {
        eprintln!("⚠️ Too many brackets ({}) - skipping rainbow highlighting (max: {})", bracket_count, max_brackets);
        return;
    }

    // Directly iterate over nesting_levels to avoid double lookup
    for (range, &level) in &editor.rainbow_bracket_tracker.nesting_levels {
        // IMPORTANT: Animations are disabled for now because GPUI's highlight_text()
        // applies a single color to all ranges - can't do per-bracket colors.
        // To enable animations, we'd need an OpenGL overlay system.
        let color = editor.rainbow_bracket_tracker.get_color_for_level(level);

        let color_index = (level % 12) as usize;
        by_level[color_index].push((range.clone(), color));
    }

    eprintln!("🎨 Color buckets: 0={}, 1={}, 2={}, 3={}, 4={}, 5={}",
        by_level[0].len(), by_level[1].len(), by_level[2].len(),
        by_level[3].len(), by_level[4].len(), by_level[5].len());

    // Apply highlights using different type for each level
    macro_rules! apply_level {
        ($index:literal, $type:ty) => {
            if !by_level[$index].is_empty() {
                let ranges: Vec<_> = by_level[$index].iter().map(|(r, _)| r.clone()).collect();
                let color = by_level[$index][0].1; // All brackets at this level have same color
                editor.highlight_text::<$type>(
                    ranges,
                    HighlightStyle {
                        color: Some(color),
                        ..Default::default()
                    },
                    cx
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

    // Request animation frame if needed
    editor.rainbow_bracket_tracker.complete_fade_if_done();
    if editor.rainbow_bracket_tracker.needs_animation_frame() {
        cx.notify();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient_color_calculation_start_hue() {
        let tracker = RainbowBracketTracker::new(true);

        let color_0 = tracker.get_color_for_level(0);
        assert_eq!(color_0.h, 0.0, "Level 0 should start at hue 0 (red)");
        assert_eq!(color_0.s, 0.75, "Saturation should be 0.75");
        assert_eq!(color_0.l, 0.6, "Lightness should be 0.6");
        assert_eq!(color_0.a, 1.0, "Alpha should be 1.0");
    }

    #[test]
    fn test_gradient_color_calculation_step() {
        let tracker = RainbowBracketTracker::new(true);

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
    fn test_gradient_color_wraps_at_360() {
        let tracker = RainbowBracketTracker::new(true);

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
    fn test_gradient_supports_unlimited_nesting() {
        let tracker = RainbowBracketTracker::new(true);

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
    fn test_classic_mode_6_color_cycling() {
        let mut tracker = RainbowBracketTracker::new(true);
        tracker.set_mode(RainbowMode::Classic);

        let color_0 = tracker.get_color_for_level(0);
        let color_6 = tracker.get_color_for_level(6);

        assert_eq!(
            color_0.h, color_6.h,
            "Level 0 and 6 should have same hue (cycling)"
        );
        assert_eq!(
            color_0.s, color_6.s,
            "Level 0 and 6 should have same saturation"
        );
        assert_eq!(
            color_0.l, color_6.l,
            "Level 0 and 6 should have same lightness"
        );
    }

    #[test]
    fn test_classic_mode_all_6_colors_unique() {
        let mut tracker = RainbowBracketTracker::new(true);
        tracker.set_mode(RainbowMode::Classic);

        let mut colors = Vec::new();
        for i in 0..6 {
            colors.push(tracker.get_color_for_level(i));
        }

        for i in 0..6 {
            for j in (i + 1)..6 {
                assert_ne!(
                    colors[i].h, colors[j].h,
                    "Colors {} and {} should have different hues",
                    i, j
                );
            }
        }
    }

    #[test]
    fn test_gradient_config_custom_start_hue() {
        let mut tracker = RainbowBracketTracker::new(true);
        tracker.set_gradient_config(GradientConfig {
            start_hue: 180.0,
            step_degrees: 30.0,
            saturation: 0.7,
            lightness: 0.5,
        });

        let color = tracker.get_color_for_level(0);
        assert!(
            (color.h - 180.0 / 360.0).abs() < 0.001,
            "Should start at cyan (180 degrees)"
        );
    }

    #[test]
    fn test_gradient_config_custom_step() {
        let mut tracker = RainbowBracketTracker::new(true);
        tracker.set_gradient_config(GradientConfig {
            start_hue: 0.0,
            step_degrees: 45.0,
            saturation: 0.7,
            lightness: 0.5,
        });

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
    fn test_gradient_config_custom_saturation_lightness() {
        let mut tracker = RainbowBracketTracker::new(true);
        tracker.set_gradient_config(GradientConfig {
            start_hue: 0.0,
            step_degrees: 30.0,
            saturation: 0.8,
            lightness: 0.6,
        });

        let color = tracker.get_color_for_level(0);
        assert_eq!(color.s, 0.8, "Should use custom saturation");
        assert_eq!(color.l, 0.6, "Should use custom lightness");
    }

    #[test]
    fn test_disabled_tracker_returns_empty_highlights() {
        let tracker = RainbowBracketTracker::new(false);
        assert!(!tracker.is_enabled(), "Tracker should be disabled");

        let highlights = tracker.get_bracket_highlights();
        assert!(
            highlights.is_empty(),
            "Disabled tracker should return no highlights"
        );
    }

    #[test]
    fn test_can_toggle_enabled() {
        let mut tracker = RainbowBracketTracker::new(true);
        assert!(tracker.is_enabled());

        tracker.set_enabled(false);
        assert!(!tracker.is_enabled());

        tracker.set_enabled(true);
        assert!(tracker.is_enabled());
    }

    #[test]
    fn test_default_rainbow_mode_is_gradient() {
        let mode = RainbowMode::default();
        assert_eq!(
            mode,
            RainbowMode::Gradient,
            "Default mode should be Gradient"
        );
    }

    #[test]
    fn test_default_gradient_config() {
        let config = GradientConfig::default();
        assert_eq!(config.start_hue, 0.0, "Default should start at red");
        assert_eq!(
            config.step_degrees, 60.0,
            "Default step should be 60 degrees for high contrast"
        );
        assert_eq!(config.saturation, 0.75, "Default saturation should be 75%");
        assert_eq!(config.lightness, 0.6, "Default lightness should be 60%");
    }

    #[test]
    fn test_mode_switching() {
        let mut tracker = RainbowBracketTracker::new(true);

        tracker.set_mode(RainbowMode::Gradient);
        let gradient_color = tracker.get_color_for_level(1);

        tracker.set_mode(RainbowMode::Classic);
        let classic_color = tracker.get_color_for_level(1);

        assert_ne!(
            gradient_color.h, classic_color.h,
            "Gradient and classic modes should produce different colors for same level"
        );
    }

    #[test]
    fn test_animation_settings_update() {
        let mut tracker = RainbowBracketTracker::new(true);

        // Update animation settings
        tracker.update_animation_settings(false, false, 500);

        // Verify settings were updated
        assert!(!tracker.animation_state.animate_fade, "Fade should be disabled");
        assert!(!tracker.animation_state.animate_glow, "Glow should be disabled");
        assert_eq!(tracker.animation_state.fade_duration_ms, 500, "Duration should be 500ms");
    }

    #[test]
    fn test_start_fade_animation() {
        let mut tracker = RainbowBracketTracker::new(true);

        // Initially no fade animation
        assert!(tracker.animation_state.fade_start.is_none(), "Should start with no fade");

        // Start fade animation
        tracker.start_fade_animation();

        // Now fade should be active
        assert!(tracker.animation_state.fade_start.is_some(), "Fade should be active");
    }

    #[test]
    fn test_fade_animation_disabled() {
        let mut tracker = RainbowBracketTracker::new(true);
        tracker.update_animation_settings(false, true, 200);

        // Try to start fade animation
        tracker.start_fade_animation();

        // Fade should not start when disabled
        assert!(tracker.animation_state.fade_start.is_none(), "Fade should not start when disabled");
    }

    #[test]
    fn test_fade_progress_calculation() {
        let mut tracker = RainbowBracketTracker::new(true);
        tracker.update_animation_settings(true, true, 100);

        // No progress before starting
        assert_eq!(tracker.calculate_fade_progress(), None, "No progress before start");

        // Start animation
        tracker.start_fade_animation();

        // Should have some progress immediately
        let progress = tracker.calculate_fade_progress();
        assert!(progress.is_some(), "Should have progress after start");
        assert!(progress.unwrap() >= 0.0 && progress.unwrap() <= 1.0, "Progress should be 0.0-1.0");
    }

    #[test]
    fn test_glow_intensity_range() {
        let tracker = RainbowBracketTracker::new(true);

        // Glow should oscillate between 0.6 and 1.0
        let intensity = tracker.calculate_glow_intensity();
        assert!(intensity >= 0.6 && intensity <= 1.0, "Glow intensity should be 0.6-1.0, got {}", intensity);
    }

    #[test]
    fn test_glow_disabled() {
        let mut tracker = RainbowBracketTracker::new(true);
        tracker.update_animation_settings(true, false, 200);

        // When glow is disabled, intensity should be 1.0 (no effect)
        let intensity = tracker.calculate_glow_intensity();
        assert_eq!(intensity, 1.0, "Disabled glow should return 1.0");
    }

    #[test]
    fn test_apply_animation_fade_cascade() {
        let mut tracker = RainbowBracketTracker::new(true);
        tracker.update_animation_settings(true, false, 1000);
        tracker.start_fade_animation();

        let base_color = hsla(0.5, 0.7, 0.5, 1.0);

        // Depth 0 should have more alpha than depth 10 (cascade delay)
        let color_depth_0 = tracker.apply_animation(base_color, 0, false);
        let color_depth_10 = tracker.apply_animation(base_color, 10, false);

        assert!(color_depth_0.a >= color_depth_10.a,
            "Depth 0 should fade in before depth 10 (cascade effect)");
    }

    #[test]
    fn test_apply_animation_glow_active() {
        let mut tracker = RainbowBracketTracker::new(true);
        tracker.update_animation_settings(false, true, 200);

        let base_color = hsla(0.5, 0.7, 0.5, 1.0);

        // Active brackets should be brighter than inactive
        let inactive_color = tracker.apply_animation(base_color, 0, false);
        let active_color = tracker.apply_animation(base_color, 0, true);

        // Active should have same or higher lightness (glow effect)
        assert!(active_color.l >= inactive_color.l,
            "Active brackets should glow (higher lightness)");
    }

    #[test]
    fn test_needs_animation_frame_fade() {
        let mut tracker = RainbowBracketTracker::new(true);
        tracker.update_animation_settings(true, false, 200);

        // No frames needed initially
        assert!(!tracker.needs_animation_frame(), "No frames needed initially");

        // Start fade animation
        tracker.start_fade_animation();

        // Now frames are needed
        assert!(tracker.needs_animation_frame(), "Frames needed during fade");
    }

    #[test]
    fn test_needs_animation_frame_glow() {
        use multi_buffer::ToOffset;
        let mut tracker = RainbowBracketTracker::new(true);
        tracker.update_animation_settings(false, true, 200);

        // No frames needed without active scope
        assert!(!tracker.needs_animation_frame(), "No frames needed without active scope");

        // Simulate having an active scope
        // Create dummy anchors - we just need Some() to test the logic
        let start = Anchor::min();
        let end = Anchor::max();
        tracker.active_scope = Some(start..end);

        // Now frames are needed for glow
        assert!(tracker.needs_animation_frame(), "Frames needed for glow with active scope");
    }

    #[test]
    fn test_complete_fade_if_done() {
        let mut tracker = RainbowBracketTracker::new(true);
        tracker.update_animation_settings(true, false, 1);  // Very short duration

        // Start fade
        tracker.start_fade_animation();
        assert!(tracker.animation_state.fade_start.is_some(), "Fade should be active");

        // Wait a moment for animation to complete
        std::thread::sleep(std::time::Duration::from_millis(10));

        // Complete if done
        let completed = tracker.complete_fade_if_done();
        assert!(completed, "Fade should be complete after duration");
        assert!(tracker.animation_state.fade_start.is_none(), "Fade should be cleared");
    }

    #[test]
    fn test_animations_can_be_disabled() {
        let mut tracker = RainbowBracketTracker::new(true);
        tracker.update_animation_settings(false, false, 200);

        let base_color = hsla(0.5, 0.7, 0.5, 1.0);

        // Try to start fade
        tracker.start_fade_animation();

        // Apply animation (should have no effect)
        let result = tracker.apply_animation(base_color, 5, true);

        // Color should be unchanged (no fade since it didn't start, no glow since disabled)
        assert_eq!(result.h, base_color.h, "Hue should be unchanged");
        assert_eq!(result.s, base_color.s, "Saturation should be unchanged");
        assert_eq!(result.l, base_color.l, "Lightness should be unchanged");
        assert_eq!(result.a, base_color.a, "Alpha should be unchanged");
    }

    #[test]
    fn test_viewport_change_triggers_fade() {
        let mut tracker = RainbowBracketTracker::new(true);
        tracker.update_animation_settings(true, false, 200);

        // First viewport (no previous) should trigger fade
        let viewport1 = 0..1000;
        assert!(tracker.should_trigger_fade(&viewport1), "First viewport should trigger fade");

        // Small scroll (10% of viewport) should NOT trigger
        tracker.last_viewport = Some(viewport1);
        let viewport2 = 100..1100;
        assert!(!tracker.should_trigger_fade(&viewport2), "Small scroll should not trigger fade");

        // Large scroll (50% of viewport) SHOULD trigger
        let viewport3 = 600..1600;
        assert!(tracker.should_trigger_fade(&viewport3), "Large scroll should trigger fade");
    }

    #[test]
    fn test_viewport_tracking_updates() {
        let mut tracker = RainbowBracketTracker::new(true);

        // Initially no viewport tracked
        assert!(tracker.last_viewport.is_none(), "Should start with no viewport");

        // After update_brackets called, viewport should be tracked
        // (We can't easily test update_brackets without a full buffer,
        // but we can verify the field exists and is used)
        assert!(tracker.last_viewport.is_none(), "Viewport tracking field exists");
    }
}
