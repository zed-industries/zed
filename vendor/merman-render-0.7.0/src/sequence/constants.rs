pub(crate) const SEQUENCE_NOTE_WRAP_SLACK_PX: f64 = 0.0;
pub(crate) const SEQUENCE_LEFT_OF_NOTE_WIDTH_OVERFLOW_PX: f64 = 0.0;
pub(crate) const SEQUENCE_LEFT_OF_NOTE_FINAL_WRAP_SLACK_PX: f64 = 0.0;
pub(crate) const SEQUENCE_LEFT_OF_NOTE_WRAP_WIDTH_SLACK_PX: f64 = 15.0;
pub(crate) const SEQUENCE_WRAPPED_MESSAGE_WIDTH_EPS_PX: f64 = 4.0;
pub(crate) const SEQUENCE_MESSAGE_WRAP_SLACK_FACTOR: f64 = 4.5;
pub(crate) const SEQUENCE_SELF_MESSAGE_FRAME_EXTRA_Y_PX: f64 = 60.0;
pub(crate) const SEQUENCE_FRAME_SIDE_PAD_PX: f64 = 11.0;
pub(crate) const SEQUENCE_FRAME_GEOM_PAD_PX: f64 = 10.0;
pub(crate) const SEQUENCE_ACTOR_POPUP_PANEL_BASE_HEIGHT: f64 = 20.0;
pub(crate) const SEQUENCE_ACTOR_POPUP_ROW_HEIGHT: f64 = 30.0;

pub(crate) fn sequence_text_dimensions_height_px(font_size_px: f64) -> f64 {
    (font_size_px.max(1.0) * (17.0 / 16.0)).round().max(1.0)
}

pub(crate) fn sequence_text_line_step_px(font_size_px: f64) -> f64 {
    font_size_px.max(1.0) * 1.1875
}

pub(crate) fn sequence_actor_popup_panel_height(link_count: usize) -> f64 {
    SEQUENCE_ACTOR_POPUP_PANEL_BASE_HEIGHT + (link_count as f64) * SEQUENCE_ACTOR_POPUP_ROW_HEIGHT
}

pub(super) fn sequence_actor_visual_height(
    actor_type: &str,
    base_width: f64,
    base_height: f64,
    label_box_height: f64,
) -> f64 {
    match actor_type {
        // Mermaid derives these from the actor-type glyph bbox + label box height.
        // These heights are used by the footer actor rendering and affect the final SVG viewBox.
        "boundary" => (44.0 + label_box_height).max(1.0),
        // Mermaid's database actor updates the actor height from the cylinder bbox after render.
        // The cylinder uses `rect.width / 3`, then the label box height is added.
        "database" => ((base_width / 3.0) + label_box_height).max(1.0),
        "entity" => (44.0 + label_box_height).max(1.0),
        // Control uses an extra label-box height in Mermaid.
        "control" => (44.0 + 2.0 * label_box_height).max(1.0),
        _ => base_height.max(1.0),
    }
}

pub(super) fn sequence_actor_lifeline_start_y(
    actor_type: &str,
    base_height: f64,
    box_text_margin: f64,
) -> f64 {
    match actor_type {
        // Hard-coded in Mermaid's sequence svgDraw.js for these actor types.
        "actor" | "boundary" => 80.0,
        "control" | "entity" => 75.0,
        // For database, Mermaid starts the lifeline slightly below the actor box.
        "database" => base_height + 2.0 * box_text_margin,
        _ => base_height,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sequence_text_and_frame_constants_match_mermaid() {
        assert_eq!(super::SEQUENCE_NOTE_WRAP_SLACK_PX, 0.0);
        assert_eq!(super::SEQUENCE_LEFT_OF_NOTE_WIDTH_OVERFLOW_PX, 0.0);
        assert_eq!(super::SEQUENCE_LEFT_OF_NOTE_FINAL_WRAP_SLACK_PX, 0.0);
        assert_eq!(super::SEQUENCE_LEFT_OF_NOTE_WRAP_WIDTH_SLACK_PX, 15.0);
        assert_eq!(super::SEQUENCE_WRAPPED_MESSAGE_WIDTH_EPS_PX, 4.0);
        assert_eq!(super::SEQUENCE_MESSAGE_WRAP_SLACK_FACTOR, 4.5);
        assert_eq!(super::SEQUENCE_ACTOR_POPUP_PANEL_BASE_HEIGHT, 20.0);
        assert_eq!(super::SEQUENCE_ACTOR_POPUP_ROW_HEIGHT, 30.0);
        assert_eq!(super::sequence_actor_popup_panel_height(0), 20.0);
        assert_eq!(super::sequence_actor_popup_panel_height(4), 140.0);
        assert_eq!(super::sequence_text_dimensions_height_px(16.0), 17.0);
        assert_eq!(super::sequence_text_dimensions_height_px(10.0), 11.0);
        assert_eq!(super::sequence_text_line_step_px(16.0), 19.0);
        assert_eq!(
            super::sequence_actor_visual_height("database", 150.0, 65.0, 20.0),
            70.0
        );
        assert_eq!(
            super::sequence_actor_visual_height("boundary", 150.0, 65.0, 20.0),
            64.0
        );
        assert_eq!(
            super::sequence_actor_visual_height("entity", 150.0, 65.0, 20.0),
            64.0
        );
        assert_eq!(
            super::sequence_actor_visual_height("control", 150.0, 65.0, 20.0),
            84.0
        );
        assert_eq!(super::SEQUENCE_SELF_MESSAGE_FRAME_EXTRA_Y_PX, 60.0);
        assert_eq!(super::SEQUENCE_FRAME_SIDE_PAD_PX, 11.0);
        assert_eq!(super::SEQUENCE_FRAME_GEOM_PAD_PX, 10.0);
    }
}
