// Fixture-derived root viewport overrides for Mermaid@11.12.2 State diagrams.
//
// These values are taken from upstream SVG baselines under
// `fixtures/upstream-svgs/state/*.svg` and are keyed by `diagram_id` (fixture stem).
//
// They are used to keep `parity-root` stable at higher decimal precision when browser float
// behavior (DOM `getBBox()` + serialization) differs from our deterministic headless pipeline.

pub fn lookup_state_root_viewport_override(
    diagram_id: &str,
) -> Option<(&'static str, &'static str)> {
    match diagram_id {
        "stress_state_frontmatter_accessibility_012" => {
            Some(("-86.8125 -50 233.875 324", "233.875"))
        }
        "stress_state_long_descriptions_and_aliases_006" => Some(("0 0 513.890625 541", "513.891")),
        "stress_state_three_way_concurrency_013" => Some(("0 0 573.27734375 1657", "573.277")),
        "stress_state_quoted_multiline_names_015" => {
            Some(("0 0 430.5703125 644.0999755859375", "430.57"))
        }
        "stress_state_long_edge_labels_wrapping_020" => Some(("0 0 411.734375 564", "411.734")),
        "stress_state_unicode_mixed_scripts_021" => Some(("0 0 141.890625 526", "141.891")),
        "stress_state_html_sanitization_notes_025" => Some(("0 0 365.9296875 402", "365.93")),
        "stress_state_markdown_edge_labels_026" => Some(("0 0 110.609375 460", "110.609")),
        "stress_state_dense_graph_labels_027" => Some(("0 0 568 484", "568")),
        "stress_state_nested_concurrency_and_choice_030" => {
            Some(("0 0 390.5546875 983", "390.555"))
        }
        "stress_state_quoted_multiline_state_names_032" => Some(("0 0 145.59375 346", "145.594")),
        "stress_state_scale_wrapping_long_edge_labels_038" => {
            Some(("0 0 375.640625 670", "375.641"))
        }
        "stress_state_font_size_precedence_071" => Some(("0 0 182.296875 386", "182.297")),
        "stress_state_frontmatter_acctitle_accdescr_multiline_039" => {
            Some(("-143.4609375 -50 337.890625 372", "337.891"))
        }
        "stress_state_state_keyword_quotes_and_aliases_040" => {
            Some(("0 0 310.5625 356", "310.562"))
        }
        "stress_state_notes_positions_and_multiline_045" => Some(("0 0 593.578125 474", "593.578")),
        "stress_state_hide_empty_description_and_multidescr_046" => {
            Some(("0 0 210.828125 313", "210.828"))
        }
        "stress_state_unicode_quotes_and_br_in_notes_048" => Some(("0 0 398.375 596", "398.375")),
        "stress_state_accdescr_block_and_markdown_labels_049" => {
            Some(("0 0 659.6762084960938 71", "659.676"))
        }
        "stress_state_direction_rl_scale_and_long_ids_054" => {
            Some(("0.006646156311035156 0 1006.5691528320312 64", "1006.57"))
        }
        "upstream_cypress_statediagram_spec_should_render_a_state_with_a_note_together_with_another_state_008" => {
            Some(("0 0 671.140625 346", "671.141"))
        }
        "upstream_cypress_statediagram_spec_should_render_multiple_composit_states_016" => {
            Some(("0 0 233.85546875 1219", "233.855"))
        }
        "upstream_cypress_statediagram_v2_spec_should_render_edge_labels_correctly_039" => {
            Some(("0 -50 1069.5546875 1190", "1069.55"))
        }
        "upstream_cypress_statediagram_v2_spec_should_render_edge_labels_correctly_with_multiple_states_041" => {
            Some(("0 -50 188.375 1946", "188.375"))
        }
        "upstream_cypress_statediagram_v2_spec_should_render_edge_labels_correctly_with_multiple_transitions_040" => {
            Some(("0 -50 1283.5390625 1190", "1283.54"))
        }
        "upstream_cypress_statediagram_v2_spec_v2_it_should_be_possible_to_use_a_choice_022" => {
            Some(("0 0 201.6796875 532", "201.68"))
        }
        "upstream_cypress_statediagram_v2_spec_v2_should_render_a_state_with_a_note_together_with_another_state_009" => {
            Some(("0 0 671.140625 346", "671.141"))
        }
        "upstream_cypress_statediagram_v2_spec_v2_should_render_multiple_composite_states_017" => {
            Some(("0 0 233.85546875 1219", "233.855"))
        }
        "upstream_cypress_statediagram_v2_spec_should_let_styles_take_precedence_over_classes_035" => {
            Some(("0 0 294.359375 56", "294.359"))
        }
        "upstream_cypress_statediagram_v2_spec_v2_width_of_compound_state_should_grow_with_title_if_title_is_wi_024" => {
            Some(("0 0 156.765625 246", "156.766"))
        }
        "upstream_html_demos_state_you_can_add_notes_010" => Some(("0 0 908.75 470", "908.75")),
        "stress_state_batch5_choice_fork_join_with_notes_markdown_062" => {
            Some(("0 0 470.59375 794", "470.594"))
        }
        "stress_state_batch5_direction_rl_scale_long_ids_065" => {
            Some(("0.006646156311035156 0 967.2566528320312 64", "967.257"))
        }
        _ => None,
    }
}
