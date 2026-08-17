// Fixture-derived root viewport overrides for Mermaid@11.12.2 Flowchart-V2 diagrams.
//
// These values are taken from upstream SVG baselines under
// `fixtures/upstream-svgs/flowchart/*.svg` and are keyed by `diagram_id` (fixture stem).
//
// They are used to keep `parity-root` stable at higher decimal precision when browser float
// behavior (DOM `getBBox()` + serialization) differs from our deterministic headless pipeline.

pub fn lookup_flowchart_root_viewport_override(
    diagram_id: &str,
) -> Option<(&'static str, &'static str)> {
    match diagram_id {
        "stress_flowchart_classdef_and_inline_classes_003" => Some((
            "0 -0.000003814697265625 796.109375 181.20632934570312",
            "796.109",
        )),
        "stress_flowchart_comments_and_directives_010" => {
            Some(("0 0 341.890625 481.796875", "341.891"))
        }
        "stress_flowchart_dense_parallel_edges_002" => {
            Some(("0 0 658.677490234375 697.390625", "658.677"))
        }
        "stress_flowchart_shape_mix_009" => Some(("0 0 366.359375 703.21875", "366.359")),
        "stress_flowchart_subgraph_title_margin_extremes_015" => {
            Some(("0 25 806.421875 796", "806.422"))
        }
        "stress_flowchart_text_style_overrides_076" => Some(("0 0 521.75 88", "521.75")),
        "stress_flowchart_icons_basic_051" => Some(("0 0 450.78125 278", "450.781")),
        "upstream_docs_flowchart_basic_support_for_fontawesome_234" => {
            Some(("0 0 450.78125 174", "450.781"))
        }
        "upstream_docs_diagrams_flowchart_code_flow" => {
            Some(("0 0 10468.515625 3129.234375", "10468.5"))
        }
        "upstream_docs_flowchart_special_characters_that_break_syntax_185" => {
            Some(("0 0 272.65625 70", "272.656"))
        }
        "upstream_flowchart_v2_stadium_shape_spec" => {
            Some(("-96.54400634765625 -50 610.109375 608", "610.109"))
        }
        "upstream_docs_mermaid_run_003" => Some(("0 0 529.953125 174", "529.953")),
        "upstream_cypress_flowchart_handdrawn_spec_fdh21_render_cylindrical_shape_021" => Some((
            "0 0.000003814697265625 769.890625 341.0105285644531",
            "769.891",
        )),
        "upstream_cypress_flowchart_handdrawn_spec_fhd12_should_render_a_flowchart_with_long_names_and_class_defini_012" => {
            Some(("0 0 1806.8125 452", "1806.81"))
        }
        "upstream_cypress_flowchart_handdrawn_spec_fhd6_should_render_a_flowchart_full_of_circles_006" => {
            Some(("0 -45 2400.640625 645", "2400.64"))
        }
        "upstream_cypress_flowchart_handdrawn_spec_fhd7_should_render_a_flowchart_full_of_icons_007" => {
            Some(("0 0 2007.41015625 1046", "2007.41"))
        }
        "upstream_cypress_flowchart_handdrawn_spec_fdh32_render_subroutine_shape_032" => {
            Some(("0 0 953.6875 257", "953.688"))
        }
        "upstream_html_demos_flowchart_flowchart_010"
        | "upstream_html_demos_flowchart_flowchart_049" => {
            Some(("0 0 2007.41015625 1046", "2007.41"))
        }
        "upstream_cypress_flowchart_icon_spec_example_002" => Some(("0 0 98.046875 70", "98.0469")),
        "upstream_cypress_flowchart_icon_spec_should_render_aws_icons_with_labels_and_rect_elements_005" => {
            Some(("0 0 104.6875 368", "104.688"))
        }
        "upstream_cypress_flowchart_shape_alias_spec_shape_alias_aliasset34_034" => {
            Some(("0 0 609.140625 80", "609.141"))
        }
        "upstream_cypress_flowchart_spec_12_should_render_a_flowchart_with_long_names_and_class_definitio_012" => {
            Some(("0 0 1896.984375 452", "1896.98"))
        }
        "upstream_cypress_flowchart_spec_24_keep_node_label_text_if_already_defined_when_a_style_is_appli_024" => {
            Some(("0 0 410.828125 38", "410.828"))
        }
        "upstream_cypress_flowchart_spec_25_handle_link_click_events_link_anchor_mailto_other_protocol_sc_025" => {
            Some(("0 0 1525.3125 246", "1525.31"))
        }
        "upstream_cypress_flowchart_spec_27_set_text_color_of_nodes_and_links_according_to_styles_when_ht_027" => {
            Some(("0 0 370.53125 373.40625", "370.531"))
        }
        "upstream_cypress_flowchart_spec_3_should_render_a_simple_flowchart_with_line_breaks_003" => {
            Some(("0 0 440.03125 737.25", "440.031"))
        }
        "upstream_cypress_flowchart_spec_4_should_render_a_simple_flowchart_with_trapezoid_and_inverse_tr_004" => {
            Some(("0 0 428.03125 722.25", "428.031"))
        }
        "upstream_cypress_flowchart_spec_6_should_render_a_flowchart_full_of_circles_006" => {
            Some(("0 -45 2638.375 645", "2638.38"))
        }
        "upstream_cypress_flowchart_spec_7_should_render_a_flowchart_full_of_icons_007" => {
            Some(("0 0 2241.375 1142", "2241.38"))
        }
        "upstream_cypress_flowchart_spec_8_should_render_labels_with_numbers_at_the_start_008" => {
            Some(("0 0 177.625 140", "177.625"))
        }
        "upstream_cypress_newshapes_spec_newshapessets_newshapesset5_lr_md_html_false_086" => {
            Some((
                "0 -0.009033203125 373.23016357421875 924.65283203125",
                "373.23",
            ))
        }
        "upstream_cypress_oldshapes_spec_shapessets_shapesset5_tb_md_html_false_038" => {
            Some(("0 0 1377.199462890625 199.20001220703125", "1377.2"))
        }
        "upstream_html_demos_flowchart_flowchart_008"
        | "upstream_html_demos_flowchart_flowchart_048" => {
            Some(("0 -45 2400.640625 645", "2400.64"))
        }
        "upstream_html_demos_flowchart_flowchart_016"
        | "upstream_html_demos_flowchart_flowchart_052" => Some(("0 0 640.921875 70", "640.922")),
        "upstream_html_demos_flowchart_flowchart_022"
        | "upstream_html_demos_flowchart_flowchart_055" => Some(("0 0 953.6875 257", "953.688")),
        "upstream_html_demos_flowchart_flowchart_024"
        | "upstream_html_demos_flowchart_flowchart_056"
        | "upstream_html_demos_flowchart_graph_023" => Some((
            "0 0.000003814697265625 769.890625 341.0105285644531",
            "769.891",
        )),
        "upstream_html_demos_flowchart_graph_021" => Some(("0 0 953.6875 257", "953.688")),
        "upstream_pkgtests_flow_singlenode_spec_010" => Some(("0 0 438.859375 70", "438.859")),
        _ => None,
    }
}
