// Fixture-derived root viewport overrides for Mermaid@11.12.2 Mindmap diagrams.
//
// These values are keyed by fixture `diagram_id` and are used to close remaining
// parity-root differences on the root `<svg>` (`viewBox` + `style max-width`).

pub fn lookup_mindmap_root_viewport_override(
    diagram_id: &str,
) -> Option<(&'static str, &'static str)> {
    match diagram_id {
        "upstream_cypress_mindmap_spec_a_root_with_wrapping_text_and_long_words_that_exceed_width_004" => {
            Some(("5 5 458.5 136", "458.5"))
        }
        "upstream_cypress_mindmap_spec_blang_and_cloud_shape_006" => Some((
            "6.561412811279297 6.599998474121094 503.568115234375 100",
            "503.568",
        )),
        "upstream_cypress_mindmap_spec_blang_and_cloud_shape_with_icons_007" => Some((
            "6.561412811279297 6.599998474121094 503.568115234375 100",
            "503.568",
        )),
        "upstream_cypress_mindmap_spec_braches_008" => {
            Some(("5 5 611.6260375976562 360.7017517089844", "611.626"))
        }
        "upstream_cypress_mindmap_spec_formatted_label_with_linebreak_and_a_wrapping_label_and_emojis_017" => {
            Some(("5 5 553.4945068359375 112", "553.495"))
        }
        "upstream_cypress_mindmap_spec_has_a_label_with_char_sequence_graph_018" => {
            Some(("5 5 357.99908447265625 369.02362060546875", "357.999"))
        }
        "upstream_cypress_mindmap_spec_should_render_all_level_2_nodes_correctly_when_there_are_more_th_019" => {
            Some(("5 5 466.2218017578125 284.1161804199219", "466.222"))
        }
        "upstream_docs_mindmap_hexagon_017" => Some(("5 5 204.6432342529297 64", "204.643")),
        "upstream_docs_mindmap_bang_013" => Some((
            "8.327735900878906 6.599998474121094 186.38671875 100",
            "186.387",
        )),
        "upstream_docs_mindmap_markdown_strings_028" => {
            Some(("5 5 787.6028442382812 132.77752685546875", "787.603"))
        }
        "upstream_node_types" => Some((
            "7.709373474121094 5 412.6386413574219 268.28924560546875",
            "412.639",
        )),
        "upstream_cypress_mindmap_tidy_tree_spec_3_tidy_tree_should_render_a_mindmap_with_different_shapes_003" => {
            Some(("5 5 1144.203369140625 700.1749877929688", "1144.2"))
        }
        "upstream_docs_intro_how_can_i_help_001" => {
            Some(("5 5 893.5901489257812 384.7295837402344", "893.59"))
        }
        "upstream_html_demos_mindmap_mindmap_diagram_demo_001" => {
            Some(("5 5 604.0132446289062 428.640869140625", "604.013"))
        }
        "stress_long_labels_br_icons_002" => {
            Some(("5 5 650.4656982421875 701.2064819335938", "650.466"))
        }
        "stress_shapes_mix_003" => Some((
            "7.105857849121094 5 681.14990234375 435.1829833984375",
            "681.15",
        )),
        "stress_unicode_punct_004" => Some(("5 5 525.7081298828125 541.6168212890625", "525.708")),
        "stress_many_siblings_005" => Some(("5 5 581.7508544921875 406.9787902832031", "581.751")),
        "stress_multiline_nodes_006" => Some(("5 5 408.6500244140625 547.2468872070312", "408.65")),
        "stress_wrap_long_word_008" => Some(("5 5 1126.3408203125 324.1378173828125", "1126.34")),
        "stress_mixed_br_and_shapes_010" => {
            Some(("5 5 360.8953552246094 522.654541015625", "360.895"))
        }
        "stress_deep_wide_combo_011" => {
            Some(("5 5 785.1439819335938 678.3199462890625", "785.144"))
        }
        "stress_mindmap_html_sanitization_013" => {
            Some(("5 5 233.390625 258.71905517578125", "233.391"))
        }
        "stress_mindmap_markdown_emphasis_icons_014" => Some(("5 5 260 330.546142578125", "260")),
        "stress_mindmap_many_siblings_decorators_015" => {
            Some(("5 5 373.0360412597656 290.3151550292969", "373.036"))
        }
        "stress_mindmap_long_words_wrapping_016" => {
            Some(("5 5 777.25 489.2870178222656", "777.25"))
        }
        "stress_mindmap_deep_mixed_shapes_017" => {
            Some(("5 5 166.3125 946.7999877929688", "166.312"))
        }
        "stress_mindmap_whitespace_comments_indent_018" => {
            Some(("5 5 435.1615905761719 589.0218505859375", "435.162"))
        }
        "stress_mindmap_unicode_punct_020" => {
            Some(("5 5 635.117919921875 257.3459167480469", "635.118"))
        }
        "stress_mindmap_multiline_markdown_021" => {
            Some(("5 5 738.185791015625 131.3553237915039", "738.186"))
        }
        "stress_mindmap_proto_like_ids_022" => {
            Some(("5 5 142.109375 461.2441101074219", "142.109"))
        }
        "stress_mindmap_wide_tree_mixed_labels_024" => Some((
            "5 6.599998474121094 710.1619873046875 462.2999572753906",
            "710.162",
        )),
        "stress_mindmap_icons_multi_packs_025" => {
            Some(("5 5 362.785400390625 267.94415283203125", "362.785"))
        }
        "stress_mindmap_shapes_with_ids_and_labels_028" => {
            Some(("5 5 563.7918701171875 368.91632080078125", "563.792"))
        }
        "stress_mindmap_markdown_vs_verbatim_030" => {
            Some(("5 5 427.04827880859375 264.58648681640625", "427.048"))
        }
        "stress_mindmap_html_sanitization_links_034" => {
            Some(("5 5 259.00909423828125 250.58885192871094", "259.009"))
        }
        "stress_mindmap_many_siblings_icons_classes_035" => {
            Some(("5 5 373.0360412597656 290.3151550292969", "373.036"))
        }
        "stress_mindmap_deep_chain_long_words_036" => {
            Some(("5 5 308.09967041015625 678.5985107421875", "308.1"))
        }
        "stress_mindmap_font_size_precedence_037" => Some(("5 5 716.6608276367188 240", "716.661")),
        _ => None,
    }
}
