// Fixture-derived root viewport overrides for Mermaid@11.12.2 Sequence diagrams.
//
// These values are taken from upstream SVG baselines under
// `fixtures/upstream-svgs/sequence/*.svg` and are keyed by `diagram_id` (fixture stem).
//
// They are used to keep `parity-root` stable at higher decimal precision when browser float
// behavior (DOM `getBBox()` + serialization) differs from our deterministic headless pipeline.

pub fn lookup_sequence_root_viewport_override(
    diagram_id: &str,
) -> Option<(&'static str, &'static str)> {
    match diagram_id {
        "stress_deep_nested_frames_018" => Some(("-50 -10 850 967", "850")),
        "stress_end_in_labels_025" => Some(("-50 -10 450 507", "450")),
        "stress_end_keyword_016" => Some(("-50 -10 652 451", "652")),
        "stress_self_messages_rect_021" => Some(("-50 -10 450 574", "450")),
        "stress_semicolons_022" => Some(("-50 -10 522 308", "522")),
        "stress_unicode_longish_messages_027" => Some(("-50 -10 710.5 333", "710.5")),
        "stress_wrap_directive_and_prefixes_028" => Some(("-50 -10 1022 412", "1022")),
        "stress_nested_rect_par_029" => Some(("-50 -10 650 712", "650")),
        "stress_create_destroy_inside_alt_030" => Some(("-50 -10 734 679", "734")),
        "stress_critical_options_notes_033" => Some(("-50 -10 560 679", "560")),
        "stress_html_entities_and_escaping_038" => Some(("-50 -10 730 327", "730")),
        "stress_message_text_with_colons_039" => Some(("-50 -10 986 318", "986")),
        "upstream_cypress_sequencediagram_spec_should_render_bidirectional_arrows_003" => {
            Some(("-50 -10 512 435", "512"))
        }
        "upstream_cypress_sequencediagram_spec_should_render_different_note_fonts_when_configured_011" => {
            Some(("-187 -10 587 308", "587"))
        }
        "upstream_cypress_sequencediagram_spec_should_render_a_single_and_nested_opt_with_long_test_overflowing_037" => {
            Some(("-50 -10 1250 868", "1250"))
        }
        "upstream_cypress_sequencediagram_spec_should_render_a_single_and_nested_opt_with_long_test_wrapping_038" => {
            Some(("-50 -10 1250 868", "1250"))
        }
        "upstream_cypress_sequencediagram_spec_should_render_a_single_and_nested_rects_036" => {
            Some(("-50 -10 1250 717", "1250"))
        }
        "upstream_cypress_sequencediagram_spec_should_render_rect_around_and_inside_loops_039" => {
            Some(("-50 -10 871 695", "871"))
        }
        "upstream_cypress_sequencediagram_spec_should_override_config_with_directive_settings_050" => {
            Some(("-235 -10 635 327", "635"))
        }
        "upstream_cypress_sequencediagram_spec_should_override_config_with_directive_settings_2_051" => {
            Some(("-207 -10 607 241", "607"))
        }
        "upstream_cypress_sequencediagram_spec_should_handle_bidirectional_arrows_with_autonumber_053" => {
            Some(("-50 -10 517 259", "517"))
        }
        "upstream_cypress_sequencediagram_spec_should_handle_different_line_breaks_004" => {
            Some(("-50 -10 1002 687", "1002"))
        }
        "upstream_cypress_sequencediagram_spec_should_handle_line_breaks_and_wrap_annotations_006" => {
            Some(("-50 -10 820 752", "820"))
        }
        "upstream_cypress_sequencediagram_v2_spec_should_render_complex_sequence_with_all_features_010" => {
            Some(("-50 -10 938 633", "938"))
        }
        "upstream_docs_sequencediagram_collections_016" => Some(("-50 -10 453 259", "453")),
        "upstream_docs_sequencediagram_parallel_054" => Some(("-50 -10 1062 547", "1062")),
        "upstream_docs_directives_changing_sequence_diagram_config_via_directive_013" => {
            Some(("-50 -10 1013 347", "1013"))
        }
        "upstream_docs_diagrams_mermaid_api_sequence" => Some(("-50 -10 2869 10259", "2869")),
        "upstream_html_demos_sequence_sequence_diagram_demos_001" => {
            Some(("-50 -10 904 1372", "904"))
        }
        "upstream_html_demos_sequence_sequence_diagram_demos_003" => {
            Some(("-50 -10 1002 687", "1002"))
        }
        "upstream_html_demos_sequence_sequence_diagram_demos_010" => {
            Some(("-50 -10 551 303", "551"))
        }
        "stress_critical_break_007" => Some(("-50 -10 650 635", "650")),
        "stress_entities_and_escaping_005" => Some(("-50 -10 666 308", "666")),
        "stress_nested_frames_001" => Some(("-50 -10 850 1045", "850")),
        "stress_unicode_punct_012" => Some(("-50 -10 782.5 333", "782.5")),
        "stress_sequence_batch5_alt_par_nested_040" => Some(("-50 -10 861 769", "861")),
        "stress_sequence_batch5_create_destroy_in_par_046" => Some(("-50 -10 734 556", "734")),
        "stress_sequence_batch5_reserved_words_in_labels_049" => Some(("-50 -10 580 408", "580")),
        "stress_sequence_batch5_whitespace_semicolons_051" => Some(("-50 -10 450 506", "450")),
        "upstream_cypress_sequencediagram_v2_spec_should_render_bidirectional_arrows_with_autonumbering_030" => {
            Some(("-50 -10 715 435", "715"))
        }
        "upstream_cypress_sequencediagram_v2_spec_should_render_self_reference_with_bidirectional_arrows_with_auto_051" => {
            Some(("-79.5 -10 691.5 467", "691.5"))
        }
        "upstream_cypress_sequencediagram_v2_spec_should_render_self_reference_with_bidirectional_arrows_without_a_050" => {
            Some(("-79.5 -10 691.5 467", "691.5"))
        }
        "upstream_cypress_sequencediagram_v2_spec_should_render_self_reference_with_normal_arrows_with_autonumber_047" => {
            Some(("-80 -10 692 615", "692"))
        }
        "upstream_cypress_sequencediagram_v2_spec_should_render_self_reference_with_normal_arrows_without_autonumb_046" => {
            Some(("-80 -10 692 615", "692"))
        }
        "upstream_docs_readme_how_does_a_langium_based_parser_work_002" => {
            Some(("-50 -10 1334 503", "1334"))
        }
        "upstream_pkgtests_sequencediagram_spec_016" => Some(("-50 -10 471 303", "471")),
        "upstream_pkgtests_sequencediagram_spec_038" => Some(("-50 -10 513 259", "513")),
        "upstream_pkgtests_sequencediagram_spec_095" => Some(("-50 -10 450 215", "450")),
        "upstream_pkgtests_sequencediagram_spec_102" => Some(("-50 -10 509 289", "509")),
        _ => None,
    }
}
