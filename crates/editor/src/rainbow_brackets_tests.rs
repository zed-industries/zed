use crate::rainbow_brackets::RainbowBracketTracker;
use gpui::{AppContext, TestAppContext};
use language::{BracketPair, BracketPairConfig, Buffer, Language, LanguageConfig};
use multi_buffer::MultiBuffer;
use std::sync::Arc;

// Helper function to create a language with bracket pair configuration
fn test_language() -> Arc<Language> {
    Arc::new(Language::new(
        LanguageConfig {
            brackets: BracketPairConfig {
                pairs: vec![
                    BracketPair {
                        start: "{".to_string(),
                        end: "}".to_string(),
                        close: true,
                        surround: true,
                        newline: true,
                    },
                    BracketPair {
                        start: "[".to_string(),
                        end: "]".to_string(),
                        close: true,
                        surround: true,
                        newline: true,
                    },
                    BracketPair {
                        start: "(".to_string(),
                        end: ")".to_string(),
                        close: true,
                        surround: true,
                        newline: true,
                    },
                ],
                ..BracketPairConfig::default()
            },
            ..LanguageConfig::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    )
    .with_brackets_query(
        r#"
            ("{" @open "}" @close)
            ("[" @open "]" @close)
            ("(" @open ")" @close)
        "#,
    )
    .unwrap())
}

// ============================================================================
// Color Calculation Tests (from original)
// ============================================================================

#[gpui::test]
fn test_color_calculation_start_hue(_cx: &mut TestAppContext) {
    let tracker = RainbowBracketTracker::new(true, 0.0, 60.0, 100000);

    let color_0 = tracker.get_color_for_level(0);
    assert_eq!(color_0.h, 0.0, "Level 0 should start at hue 0 (red)");
    assert_eq!(color_0.s, 0.75, "Saturation should be 0.75");
    assert_eq!(color_0.l, 0.6, "Lightness should be 0.6");
    assert_eq!(color_0.a, 1.0, "Alpha should be 1.0");
}

#[gpui::test]
fn test_color_calculation_step(_cx: &mut TestAppContext) {
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

#[gpui::test]
fn test_color_wraps_at_360(_cx: &mut TestAppContext) {
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

#[gpui::test]
fn test_supports_unlimited_nesting(_cx: &mut TestAppContext) {
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

#[gpui::test]
fn test_custom_start_hue(_cx: &mut TestAppContext) {
    let tracker = RainbowBracketTracker::new(true, 180.0, 30.0, 100000);

    let color = tracker.get_color_for_level(0);
    assert!(
        (color.h - 180.0 / 360.0).abs() < 0.001,
        "Should start at cyan (180 degrees)"
    );
}

#[gpui::test]
fn test_custom_step(_cx: &mut TestAppContext) {
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

// ============================================================================
// Enable/Disable Tests (from original)
// ============================================================================

#[gpui::test]
fn test_disabled_tracker_returns_empty_highlights(_cx: &mut TestAppContext) {
    let tracker = RainbowBracketTracker::new(false, 0.0, 30.0, 100000);
    assert!(!tracker.is_enabled(), "Tracker should be disabled");

    let highlights = tracker.get_bracket_highlights();
    assert!(
        highlights.is_empty(),
        "Disabled tracker should return no highlights"
    );
}

#[gpui::test]
fn test_can_toggle_enabled(_cx: &mut TestAppContext) {
    let mut tracker = RainbowBracketTracker::new(true, 0.0, 30.0, 100000);
    assert!(tracker.is_enabled());

    tracker.set_enabled(false);
    assert!(!tracker.is_enabled());

    tracker.set_enabled(true);
    assert!(tracker.is_enabled());
}

// ============================================================================
// Active Pair Detection Tests (from original + new)
// ============================================================================

#[gpui::test]
fn test_active_pair_detection(cx: &mut TestAppContext) {
    let language = test_language();
    let buffer_entity = cx.new(|cx| {
        let mut buffer = Buffer::local("{ [ ( ) ] }", cx);
        buffer.set_language(Some(language), cx);
        buffer
    });
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer_entity, cx));

    let mut tracker = RainbowBracketTracker::new(true, 0.0, 60.0, 100000);

    let snapshot = cx.read(|cx| buffer.read(cx).snapshot(cx));

    // Update brackets first
    tracker.update_brackets(&snapshot, None);

    // Test cursor inside innermost brackets
    let cursor_at_4 = snapshot.anchor_before(4);
    tracker.update_active_pair(cursor_at_4, &snapshot);
    assert!(
        tracker.active_pair.is_some(),
        "Should detect active pair when cursor inside brackets"
    );

    // Test cursor outside all brackets
    let cursor_at_0 = snapshot.anchor_before(0);
    tracker.update_active_pair(cursor_at_0, &snapshot);

    // Test with disabled tracker
    tracker.set_enabled(false);
    tracker.update_active_pair(cursor_at_4, &snapshot);
    assert!(
        tracker.active_pair.is_none(),
        "Should not detect active pair when disabled"
    );
}

// ============================================================================
// Empty Buffer / Panic Fix Tests (NEW)
// ============================================================================

#[gpui::test]
fn test_empty_buffer_no_panic(cx: &mut TestAppContext) {
    let buffer_entity = cx.new(|cx| Buffer::local("", cx));
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer_entity, cx));

    let mut tracker = RainbowBracketTracker::new(true, 0.0, 60.0, 100000);

    let snapshot = cx.read(|cx| buffer.read(cx).snapshot(cx));

    tracker.update_brackets(&snapshot, None);
    assert_eq!(tracker.nesting_levels.len(), 0, "Empty buffer should have no brackets");

    let cursor_at_0 = snapshot.anchor_before(0);
    tracker.update_active_pair(cursor_at_0, &snapshot);
    assert!(tracker.active_pair.is_none(), "Empty buffer should have no active pair");
}

#[gpui::test]
fn test_bounds_checking_at_buffer_end(cx: &mut TestAppContext) {
    let buffer_entity = cx.new(|cx| Buffer::local("{ }", cx));
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer_entity, cx));

    let mut tracker = RainbowBracketTracker::new(true, 0.0, 60.0, 100000);

    let snapshot = cx.read(|cx| buffer.read(cx).snapshot(cx));
    tracker.update_brackets(&snapshot, None);

    let cursor_at_end = snapshot.anchor_before(snapshot.len());
    tracker.update_active_pair(cursor_at_end, &snapshot);
}

#[gpui::test]
fn test_bounds_checking_one_char_buffer(cx: &mut TestAppContext) {
    let buffer_entity = cx.new(|cx| Buffer::local("{", cx));
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer_entity, cx));

    let mut tracker = RainbowBracketTracker::new(true, 0.0, 60.0, 100000);

    let snapshot = cx.read(|cx| buffer.read(cx).snapshot(cx));
    tracker.update_brackets(&snapshot, None);

    let cursor_at_0 = snapshot.anchor_before(0);
    let cursor_at_1 = snapshot.anchor_before(1);

    tracker.update_active_pair(cursor_at_0, &snapshot);
    tracker.update_active_pair(cursor_at_1, &snapshot);
}

#[gpui::test]
fn test_adjacent_bracket_detection(cx: &mut TestAppContext) {
    let buffer_entity = cx.new(|cx| Buffer::local("{ }", cx));
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer_entity, cx));

    let mut tracker = RainbowBracketTracker::new(true, 0.0, 60.0, 100000);

    let snapshot = cx.read(|cx| buffer.read(cx).snapshot(cx));
    tracker.update_brackets(&snapshot, None);

    let cursor_before_open = snapshot.anchor_before(0);
    tracker.update_active_pair(cursor_before_open, &snapshot);

    let cursor_after_close = snapshot.anchor_before(3);
    tracker.update_active_pair(cursor_after_close, &snapshot);
}

// ============================================================================
// Caching Tests (NEW)
// ============================================================================

#[gpui::test]
fn test_cursor_position_caching(cx: &mut TestAppContext) {
    let buffer_entity = cx.new(|cx| Buffer::local("{ }", cx));
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer_entity, cx));

    let mut tracker = RainbowBracketTracker::new(true, 0.0, 60.0, 100000);

    let snapshot = cx.read(|cx| buffer.read(cx).snapshot(cx));
    tracker.update_brackets(&snapshot, None);

    let cursor_pos = snapshot.anchor_before(1);

    tracker.update_active_pair(cursor_pos.clone(), &snapshot);
    assert!(tracker.last_cursor_offset.is_some());
    let first_offset = tracker.last_cursor_offset;

    tracker.update_active_pair(cursor_pos.clone(), &snapshot);
    assert_eq!(tracker.last_cursor_offset, first_offset, "Cursor offset should be cached");
}

#[gpui::test]
fn test_cache_invalidation_on_edit(cx: &mut TestAppContext) {
    let buffer_entity = cx.new(|cx| Buffer::local("{ }", cx));
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer_entity, cx));

    let mut tracker = RainbowBracketTracker::new(true, 0.0, 60.0, 100000);

    let snapshot = cx.read(|cx| buffer.read(cx).snapshot(cx));

    tracker.update_brackets(&snapshot, None);
    assert!(tracker.cached_edit_count.is_some());
    let first_count = tracker.cached_edit_count;

    buffer.update(cx, |buffer, cx| {
        buffer.edit([(3..3, " ")], None, cx);
    });

    let new_snapshot = cx.read(|cx| buffer.read(cx).snapshot(cx));

    tracker.update_brackets(&new_snapshot, None);
    assert!(tracker.cached_edit_count.is_some());
    assert_ne!(tracker.cached_edit_count, first_count, "Cache should be updated after edit");
}

#[gpui::test]
fn test_visible_range_caching(cx: &mut TestAppContext) {
    let large_text = "{ }".repeat(5000);
    let buffer_entity = cx.new(|cx| Buffer::local(&large_text, cx));
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer_entity, cx));

    let mut tracker = RainbowBracketTracker::new(true, 0.0, 60.0, 100000);

    let snapshot = cx.read(|cx| buffer.read(cx).snapshot(cx));

    let visible_range = 5000..10000;
    tracker.update_brackets(&snapshot, Some(visible_range.clone()));

    assert!(tracker.cached_visible_range.is_some());

    let similar_range = 5100..10100;
    let cached_before = tracker.cached_edit_count;
    tracker.update_brackets(&snapshot, Some(similar_range));

    assert_eq!(tracker.cached_edit_count, cached_before,
        "Should use cached data for similar visible range");
}

// ============================================================================
// State Clearing Tests (NEW)
// ============================================================================

#[gpui::test]
fn test_state_clearing_when_disabled(cx: &mut TestAppContext) {
    let language = test_language();
    let buffer_entity = cx.new(|cx| {
        let mut buffer = Buffer::local("{ [ ( ) ] }", cx);
        buffer.set_language(Some(language), cx);
        buffer
    });
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer_entity, cx));

    let mut tracker = RainbowBracketTracker::new(true, 0.0, 60.0, 100000);

    let snapshot = cx.read(|cx| buffer.read(cx).snapshot(cx));

    tracker.update_brackets(&snapshot, None);
    let cursor_pos = snapshot.anchor_before(4);
    tracker.update_active_pair(cursor_pos, &snapshot);

    assert!(!tracker.nesting_levels.is_empty(), "Should have brackets");
    assert!(tracker.active_pair.is_some(), "Should have active pair");
    assert!(tracker.last_cursor_offset.is_some(), "Should have cached offset");

    tracker.set_enabled(false);

    assert!(tracker.nesting_levels.is_empty(), "Nesting levels should be cleared");
    assert!(tracker.active_pair.is_none(), "Active pair should be cleared");
    assert!(tracker.last_cursor_offset.is_none(), "Cursor offset should be cleared");
    assert!(tracker.cached_edit_count.is_none(), "Edit count should be cleared");
}

// ============================================================================
// Performance Limit Tests (NEW)
// ============================================================================

#[gpui::test]
fn test_large_file_limits(cx: &mut TestAppContext) {
    let large_text = "{ }".repeat(50_001);
    let buffer_entity = cx.new(|cx| Buffer::local(&large_text, cx));
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer_entity, cx));

    let mut tracker = RainbowBracketTracker::new(true, 0.0, 60.0, 100000);

    let snapshot = cx.read(|cx| buffer.read(cx).snapshot(cx));
    assert!(snapshot.len() > 100_000, "Buffer should be > 100K");

    tracker.update_brackets(&snapshot, Some(0..50_000));

    assert!(tracker.nesting_levels.len() == 0,
        "Large file should clear brackets to avoid performance issues");
}

#[gpui::test]
fn test_max_brackets_enforcement(cx: &mut TestAppContext) {
    let buffer_entity = cx.new(|cx| Buffer::local("{ [ ( { [ ( { [ ( { [ ( { [ ( ) ] } ) ] } ) ] } ) ] } ) ] }", cx));
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer_entity, cx));

    let mut tracker = RainbowBracketTracker::new(true, 0.0, 60.0, 10);

    let snapshot = cx.read(|cx| buffer.read(cx).snapshot(cx));
    tracker.update_brackets(&snapshot, None);

    assert!(tracker.nesting_levels.len() <= 20,
        "Should limit number of brackets processed");
}

// ============================================================================
// Nested Brackets Tests (NEW)
// ============================================================================

#[gpui::test]
fn test_nested_brackets_correct_levels(cx: &mut TestAppContext) {
    let language = test_language();
    let buffer_entity = cx.new(|cx| {
        let mut buffer = Buffer::local("{ [ ( ) ] }", cx);
        buffer.set_language(Some(language), cx);
        buffer
    });
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer_entity, cx));

    let mut tracker = RainbowBracketTracker::new(true, 0.0, 60.0, 100000);

    let snapshot = cx.read(|cx| buffer.read(cx).snapshot(cx));
    tracker.update_brackets(&snapshot, None);

    assert_eq!(tracker.nesting_levels.len(), 6, "Should detect all 6 brackets");

    let levels: Vec<u32> = tracker.nesting_levels.values().copied().collect();
    assert!(levels.contains(&0), "Should have level 0 brackets");
    assert!(levels.contains(&1), "Should have level 1 brackets");
    assert!(levels.contains(&2), "Should have level 2 brackets");
}

#[gpui::test]
fn test_unmatched_brackets(cx: &mut TestAppContext) {
    let language = test_language();
    let buffer_entity = cx.new(|cx| {
        let mut buffer = Buffer::local("{ ] }", cx);
        buffer.set_language(Some(language), cx);
        buffer
    });
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer_entity, cx));

    let mut tracker = RainbowBracketTracker::new(true, 0.0, 60.0, 100000);

    let snapshot = cx.read(|cx| buffer.read(cx).snapshot(cx));

    tracker.update_brackets(&snapshot, None);

    assert!(!tracker.nesting_levels.is_empty(),
        "Should process brackets even with mismatches");
}
