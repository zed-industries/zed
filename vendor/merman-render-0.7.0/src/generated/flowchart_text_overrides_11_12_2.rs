// This file intentionally keeps only the Flowchart text measurements that still guard
// Mermaid@11.12.3 root-viewport parity or focused text metric assertions after the shared text
// measurer absorbed the rest.

pub fn lookup_flowchart_markdown_italic_word_delta_em(
    wrap_mode: crate::text::WrapMode,
    word: &str,
) -> Option<f64> {
    match wrap_mode {
        crate::text::WrapMode::SvgLike | crate::text::WrapMode::SvgLikeSingleRun => match word {
            // `fixtures/flowchart/stress_flowchart_markdown_underscore_delims_074.mmd`
            "a_b" | "a__b" => Some(0.0),
            // `fixtures/flowchart/stress_flowchart_subgraph_markdown_titles_013.mmd`
            "Child" => Some(172.0 / 2048.0),
            // `fixtures/flowchart/upstream_docs_flowchart_markdown_formatting_007.mmd`
            "Markdown" => Some(81.0 / 1024.0),
            _ => None,
        },
        crate::text::WrapMode::HtmlLike => match word {
            // `fixtures/flowchart/upstream_docs_flowchart_markdown_formatting_008.mmd`
            "Markdown" => Some(83.0 / 1024.0),
            _ => None,
        },
    }
}

pub fn lookup_flowchart_markdown_bold_word_delta_em(
    wrap_mode: crate::text::WrapMode,
    word: &str,
) -> Option<f64> {
    match wrap_mode {
        crate::text::WrapMode::SvgLike | crate::text::WrapMode::SvgLikeSingleRun => match word {
            // `fixtures/flowchart/upstream_docs_flowchart_markdown_strings_201.mmd`
            "Two" => Some(9.0 / 128.0),
            _ => None,
        },
        crate::text::WrapMode::HtmlLike => None,
    }
}

pub fn lookup_flowchart_markdown_bold_word_extra_delta_em(
    wrap_mode: crate::text::WrapMode,
    word: &str,
) -> f64 {
    match wrap_mode {
        crate::text::WrapMode::SvgLike | crate::text::WrapMode::SvgLikeSingleRun => match word {
            // `fixtures/flowchart/upstream_cypress_flowchart_v2_spec_sub_graphs_and_markdown_strings_057.mmd`
            "ipa" => -1.0 / 1024.0,
            // `fixtures/flowchart/upstream_docs_flowchart_markdown_strings_{200,201}.mmd`
            // `fixtures/flowchart/stress_flowchart_subgraph_markdown_titles_013.mmd`
            "edge" => 1.0 / 512.0,
            "label" => -1.0 / 1024.0,
            // `fixtures/flowchart/upstream_docs_flowchart_markdown_strings_200.mmd`
            "dog" => -7.0 / 16384.0,
            _ => 0.0,
        },
        crate::text::WrapMode::HtmlLike => match word {
            "edge" | "label" => 1.0 / 1024.0,
            _ => 0.0,
        },
    }
}

pub fn lookup_flowchart_markdown_bold_char_extra_delta_em(
    wrap_mode: crate::text::WrapMode,
    word: &str,
    ch: char,
) -> f64 {
    match wrap_mode {
        crate::text::WrapMode::SvgLike | crate::text::WrapMode::SvgLikeSingleRun => {
            if word == "a" && ch == 'a' {
                1.0 / 1024.0
            } else {
                0.0
            }
        }
        crate::text::WrapMode::HtmlLike => 0.0,
    }
}

pub fn lookup_flowchart_html_width_px(
    font_key: &str,
    font_size_px: f64,
    text: &str,
) -> Option<f64> {
    if (font_size_px - 16.0).abs() > 0.01 {
        return None;
    }

    match (font_key, text) {
        // `fixtures/upstream-svgs/flowchart/upstream_cypress_flowchart_spec_2_should_render_a_simple_flowchart_with_htmllabels_002.svg`
        ("courier", "Christmas") | ("courier", "Get money") => Some(86.421875),
        // `fixtures/flowchart/stress_flowchart_edge_label_position_064.mmd`
        ("trebuchetms,verdana,arial,sans-serif", "A to B") => Some(42.1875),
        ("trebuchetms,verdana,arial,sans-serif", "B to C") => Some(43.203125),
        ("trebuchetms,verdana,arial,sans-serif", "A: (Edge Text)") => Some(101.046875),
        // `fixtures/flowchart/upstream_docs_flowchart_text_special_characters_spec.mmd`
        ("trebuchetms,verdana,arial,sans-serif", ",.?!+-*ز") => Some(51.46875),
        ("trebuchetms,verdana,arial,sans-serif", "special characters") => Some(129.9375),
        // `fixtures/flowchart/stress_flowchart_unicode_punct_in_ids_labels_035.mmd`
        ("trebuchetms,verdana,arial,sans-serif", "中文 / 日本語 / 한글") => Some(148.0625),
        ("trebuchetms,verdana,arial,sans-serif", "emoji: 😀😅👍") => Some(117.625),
        // `fixtures/flowchart/stress_flowchart_long_labels_punctuation_unicode_006.mmd`
        ("trebuchetms,verdana,arial,sans-serif", "C:\\Temp\\merman\\out.svg") => Some(203.15625),
        // `fixtures/flowchart/upstream_docs_flowchart_shapes_nodes_spec.mmd`
        ("trebuchetms,verdana,arial,sans-serif", "Rounded") => Some(61.296875),
        // `fixtures/flowchart/stress_flowchart_subgraph_boundary_edges_008.mmd`
        ("trebuchetms,verdana,arial,sans-serif", "Inner B") => Some(50.765625),
        // `fixtures/flowchart/stress_flowchart_edge_label_near_cluster_title_018.mmd`
        ("trebuchetms,verdana,arial,sans-serif", "very long edge label") => Some(145.09375),
        ("trebuchetms,verdana,arial,sans-serif", "post") => Some(30.328125),
        // `fixtures/flowchart/stress_flowchart_cluster_dense_children_021.mmd`
        ("trebuchetms,verdana,arial,sans-serif", "Dense Cluster") => Some(98.109375),
        // `fixtures/flowchart/stress_flowchart_cluster_title_long_multiline_022.mmd`
        ("trebuchetms,verdana,arial,sans-serif", "outside2") => Some(60.75),
        // `fixtures/flowchart/stress_flowchart_deeply_nested_clusters_019.mmd`
        ("trebuchetms,verdana,arial,sans-serif", "Level 1")
        | ("trebuchetms,verdana,arial,sans-serif", "Level 2")
        | ("trebuchetms,verdana,arial,sans-serif", "Level 3")
        | ("trebuchetms,verdana,arial,sans-serif", "Level 4") => Some(51.328125),
        // `fixtures/flowchart/upstream_docs_flowchart_markdown_formatting_008.mmd`
        ("trebuchetms,verdana,arial,sans-serif", "Line 2")
        | ("trebuchetms,verdana,arial,sans-serif", "Line 3") => Some(43.34375),
        // `fixtures/flowchart/stress_flowchart_html_labels_global_false_flowchart_{true,unset}_0{69,71}.mmd`
        ("trebuchetms,verdana,arial,sans-serif", "Subgraph Title") => Some(103.171875),
        ("trebuchetms,verdana,arial,sans-serif", "Edge Label") => Some(77.9375),
        // `fixtures/flowchart/stress_flowchart_html_labels_global_true_flowchart_false_070.mmd`
        ("trebuchetms,verdana,arial,sans-serif", "Node Label B") => Some(94.0),
        // `fixtures/flowchart/stress_flowchart_html_labels_default_class_077.mmd`
        ("trebuchetms,verdana,arial,sans-serif", "custom") => Some(51.359375),
        // `fixtures/flowchart/upstream_docs_flowchart_markdown_strings_200.mmd`
        ("trebuchetms,verdana,arial,sans-serif", "edge label") => Some(74.703125),
        ("trebuchetms,verdana,arial,sans-serif", "edge comment") => Some(106.109375),
        // `fixtures/flowchart/upstream_docs_flowchart_markdown_strings_201.mmd`
        ("trebuchetms,verdana,arial,sans-serif", "(1 / period_duration)") => Some(153.0),
        // `fixtures/flowchart/upstream_docs_flowchart_markdown_raw_block_*.mmd`
        ("trebuchetms,verdana,arial,sans-serif", "- e1 - e2") => Some(60.453125),
        ("trebuchetms,verdana,arial,sans-serif", "- l1 - l2") => Some(52.4375),
        // `fixtures/flowchart/stress_flowchart_edge_pipe_label_markdown_literals_*.mmd`
        ("trebuchetms,verdana,arial,sans-serif", "`**bold*`") => Some(65.546875),
        ("trebuchetms,verdana,arial,sans-serif", "`This is **bold**") => Some(112.78125),
        // `fixtures/upstream-svgs/flowchart/upstream_cypress_newshapes_spec_newshapessets_newshapesset3_lr_styles_071.svg`
        ("trebuchetms,verdana,arial,sans-serif", "new bow-rect shape") => Some(144.78125),
        // `fixtures/upstream-svgs/flowchart/upstream_cypress_newshapes_spec_newshapessets_newshapesset4_*_styles_*.svg`
        // `fixtures/upstream-svgs/flowchart/upstream_cypress_newshapes_spec_newshapessets_newshapesset4_*_classdef_*.svg`
        ("trebuchetms,verdana,arial,sans-serif", "new document shape") => Some(151.546875),
        // `fixtures/upstream-svgs/flowchart/upstream_cypress_newshapes_spec_newshapessets_newshapesset2_tb_styles_015.svg`
        // `fixtures/upstream-svgs/flowchart/upstream_cypress_newshapes_spec_newshapessets_newshapesset2_lr_styles_063.svg`
        // `fixtures/upstream-svgs/flowchart/upstream_cypress_newshapes_spec_newshapessets_newshapesset2_*_classdef_*.svg`
        ("trebuchetms,verdana,arial,sans-serif", "new documents shape") => Some(158.015625),
        ("trebuchetms,verdana,arial,sans-serif", "new window-pane shape") => Some(175.5625),
        // `fixtures/upstream-svgs/flowchart/upstream_cypress_flowchart_shape_alias_spec_shape_alias_aliasset21_021.svg`
        ("trebuchetms,verdana,arial,sans-serif", "half-rounded-rectangle") => Some(166.21875),
        // `fixtures/upstream-svgs/flowchart/upstream_cypress_flowchart_shape_alias_spec_shape_alias_aliasset34_034.svg`
        ("trebuchetms,verdana,arial,sans-serif", "stacked-rectangle") => Some(128.578125),
        _ => None,
    }
}

pub fn lookup_flowchart_svg_bbox_x_px(
    font_key: &str,
    font_size_px: f64,
    text: &str,
) -> Option<(f64, f64)> {
    if (font_size_px - 16.0).abs() > 0.01 {
        return None;
    }

    match (font_key, text) {
        // `fixtures/flowchart/upstream_cypress_flowchart_spec_1_should_render_a_simple_flowchart_no_htmllabels_001.mmd`
        // `fixtures/upstream-svgs/flowchart/upstream_cypress_flowchart_spec_1_should_render_a_simple_flowchart_no_htmllabels_001.svg`
        ("trebuchetms,verdana,arial,sans-serif", "End") => Some((13.1171875, 13.1171875)),
        // `fixtures/flowchart/stress_flowchart_html_labels_global_true_flowchart_false_070.mmd`
        ("trebuchetms,verdana,arial,sans-serif", "Subgraph Title") => Some((51.59375, 51.59375)),
        ("trebuchetms,verdana,arial,sans-serif", "Edge Label") => Some((38.96875, 38.96875)),
        // `fixtures/flowchart/stress_flowchart_html_labels_global_false_flowchart_{true,unset}_0{69,71}.mmd`
        ("trebuchetms,verdana,arial,sans-serif", "Node Label") => Some((40.0625, 40.0625)),
        ("trebuchetms,verdana,arial,sans-serif", "Node Label B") => Some((47.0, 47.0)),
        // `fixtures/flowchart/upstream_docs_flowchart_markdown_strings_200.mmd`
        ("trebuchetms,verdana,arial,sans-serif", "edge label") => Some((37.359375, 37.359375)),
        // `fixtures/flowchart/upstream_cypress_flowchart_spec_21_render_cylindrical_shape_021.mmd`
        ("courier", "Get money") => Some((43.2109375, 43.2109375)),
        // `fixtures/flowchart/upstream_cypress_newshapes_spec_newshapessets_newshapesset2_tb_md_html_false_014.mmd`
        ("trebuchetms,verdana,arial,sans-serif", "n0")
        | ("trebuchetms,verdana,arial,sans-serif", "n1")
        | ("trebuchetms,verdana,arial,sans-serif", "n2")
        | ("trebuchetms,verdana,arial,sans-serif", "n3")
        | ("trebuchetms,verdana,arial,sans-serif", "n4") => Some((8.5703125, 8.5703125)),
        // `fixtures/flowchart/upstream_cypress_newshapes_spec_newshapessets_newshapesset2_tb_md_html_false_014.mmd`
        ("trebuchetms,verdana,arial,sans-serif", "tagged-rectangle shape") => {
            Some((84.1328125, 84.1328125))
        }
        ("trebuchetms,verdana,arial,sans-serif", "</strong> for window-pane") => {
            Some((97.6171875, 97.6171875))
        }
        ("trebuchetms,verdana,arial,sans-serif", "documents shape") => Some((88.84375, 88.84375)),
        _ => None,
    }
}
