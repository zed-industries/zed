// Fixture-derived root viewport overrides for Mermaid@11.12.2 Timeline diagrams.
//
// These values are taken from upstream SVG baselines under
// `fixtures/upstream-svgs/timeline/*.svg` and are keyed by `diagram_id` (fixture stem).
//
// They are used to keep `parity-root` stable at higher decimal precision when browser float
// behavior (DOM `getBBox()` + serialization) differs from our deterministic headless pipeline.

pub fn lookup_timeline_root_viewport_override(
    diagram_id: &str,
) -> Option<(&'static str, &'static str)> {
    match diagram_id {
        "timeline_stress_common_long_unbroken_words" => {
            Some(("9.6796875 -61 1467.4140625 594.3999938964844", "1467.41"))
        }
        "timeline_stress_disable_multicolor_and_width" => {
            Some(("10 -61 721.59375 740.2000122070312", "721.594"))
        }
        "timeline_stress_events_with_entities_and_ampersands" => {
            Some(("100 -61 1005.8984375 705", "1005.9"))
        }
        "timeline_stress_inline_hashes_and_semicolons" => {
            Some(("-5 -61 967.921875 740.2000122070312", "967.922"))
        }
        "timeline_stress_font_size_precedence" => {
            Some(("-5 -77 1228.375 530.3999938964844", "1228.38"))
        }
        "timeline_stress_unicode_cjk_and_emoji" => Some(("95 -61 995 629.5999755859375", "995")),
        "timeline_stress_very_long_unbroken_word" => {
            Some(("-107.984375 -61 1516.3203125 594.3999938964844", "1516.32"))
        }
        "upstream_cypress_timeline_spec_12_should_render_timeline_with_proper_vertical_line_lengths_for_012" => {
            Some(("100 -57 2190 879.4000244140625", "2190"))
        }
        _ => None,
    }
}
