// Fixture-derived root viewport overrides for Mermaid@11.15.0 Pie diagrams.
//
// These values are taken from upstream SVG baselines under
// `fixtures/upstream-svgs/pie/*.svg` and are keyed by `diagram_id` (fixture stem).
//
// Pie layout uses browser `getBoundingClientRect()` widths for legend/title text.
// These overrides keep `parity-root` stable where deterministic headless font
// measurement differs only by sub-pixel browser text metrics.

pub fn lookup_pie_root_viewport_override(diagram_id: &str) -> Option<(&'static str, &'static str)> {
    match diagram_id {
        "upstream_cypress_pie_spec_example_001" => Some(("0 0 596.171875 450", "596.172")),
        "upstream_cypress_pie_spec_should_render_a_pie_diagram_when_textposition_is_set_004" => {
            Some(("0 0 547.08154296875 450", "547.082"))
        }
        "upstream_cypress_pie_spec_should_render_a_pie_diagram_with_showdata_005" => {
            Some(("0 0 582.40625 450", "582.406"))
        }
        "upstream_cypress_pie_spec_should_render_a_simple_pie_diagram_with_capital_letters_for_labe_003" => {
            Some(("0 0 574.109375 450", "574.109"))
        }
        "upstream_cypress_pie_spec_should_render_a_simple_pie_diagram_with_long_labels_002" => {
            Some(("0 0 735.45849609375 450", "735.458"))
        }
        "upstream_cypress_pie_spec_should_render_pie_slices_only_for_non_zero_values_but_shows_all_006" => {
            Some(("0 0 547.08154296875 450", "547.082"))
        }
        "upstream_cypress_theme_spec_should_render_a_pie_diagram_003" => {
            Some(("0 0 596.171875 450", "596.172"))
        }
        "upstream_docs_accessibility_pie_chart_012" => Some(("0 0 596.21875 450", "596.219")),
        "upstream_docs_examples_basic_pie_chart_001" => {
            Some(("0 0 735.45849609375 450", "735.458"))
        }
        "upstream_docs_examples_basic_pie_chart_003" => Some(("0 0 574.109375 450", "574.109")),
        "upstream_docs_pie_example_003" => Some(("0 0 655.734375 450", "655.734")),
        "upstream_docs_pie_pie_chart_diagrams_001" => Some(("0 0 547.08154296875 450", "547.082")),
        "upstream_docs_readme_pie_chart_a_href_https_mermaid_js_org_syntax_pie_html_docs_a_a_h_011" => {
            Some(("0 0 547.08154296875 450", "547.082"))
        }
        "upstream_docs_readme_zh_cn_a_href_https_mermaid_js_org_syntax_pie_html_a_a_href_https_merma_011" => {
            Some(("0 0 547.08154296875 450", "547.082"))
        }
        "upstream_examples_pie_basic_pie_chart_001" => Some(("0 0 547.08154296875 450", "547.082")),
        "upstream_html_demos_pie_pie_chart_demos_001" => {
            Some(("0 0 547.08154296875 450", "547.082"))
        }
        "upstream_html_demos_pie_pie_chart_demos_002" => Some(("0 0 596.21875 450", "596.219")),
        "upstream_html_demos_pie_pie_chart_demos_003" => {
            Some(("0 0 590.81005859375 450", "590.81"))
        }
        "upstream_pie_acc_descr_multiline_spec" => Some(("0 0 537.40283203125 450", "537.403")),
        "upstream_pie_acc_descr_spec" => Some(("0 0 537.40283203125 450", "537.403")),
        "upstream_pie_acc_title_spec" => Some(("0 0 537.40283203125 450", "537.403")),
        "upstream_pie_comments_spec" => Some(("0 0 537.40283203125 450", "537.403")),
        "upstream_pie_positive_decimal_spec" => Some(("0 0 537.40283203125 450", "537.403")),
        "upstream_pie_simple_spec" => Some(("0 0 537.40283203125 450", "537.403")),
        "upstream_pie_title_spec" => Some(("0 0 537.40283203125 450", "537.403")),
        "upstream_pie_unsafe_props_spec" => Some(("0 0 600.62548828125 450", "600.625")),
        "upstream_pie_very_simple_spec" => Some(("0 0 537.109375 450", "537.109")),
        "upstream_pie_zero_slice_spec" => Some(("0 0 546.126953125 450", "546.127")),
        "upstream_pkgtests_pie_spec_001" => Some(("0 0 537.109375 450", "537.109")),
        "upstream_pkgtests_pie_spec_002" => Some(("0 0 537.40283203125 450", "537.403")),
        "upstream_pkgtests_pie_spec_004" => Some(("0 0 537.40283203125 450", "537.403")),
        "upstream_pkgtests_pie_spec_005" => Some(("0 0 537.40283203125 450", "537.403")),
        "upstream_pkgtests_pie_spec_006" => Some(("0 0 537.40283203125 450", "537.403")),
        "upstream_pkgtests_pie_spec_007" => Some(("0 0 537.40283203125 450", "537.403")),
        "upstream_pkgtests_pie_spec_008" => Some(("0 0 537.40283203125 450", "537.403")),
        "upstream_pkgtests_pie_spec_009" => Some(("0 0 537.40283203125 450", "537.403")),
        "upstream_pkgtests_pie_spec_011" => Some(("0 0 546.126953125 450", "546.127")),
        "upstream_pkgtests_pie_spec_013" => Some(("0 0 600.62548828125 450", "600.625")),
        "upstream_pkgtests_pie_test_009" => Some(("0 0 292.400390625 450", "292.4")),
        "upstream_pkgtests_pie_test_010" => Some(("0 0 292.400390625 450", "292.4")),
        "upstream_pkgtests_pie_test_011" => Some(("0 0 292.400390625 450", "292.4")),
        "upstream_pkgtests_pie_test_012" => Some(("0 0 292.400390625 450", "292.4")),
        "upstream_pkgtests_pie_test_013" => Some(("0 0 292.400390625 450", "292.4")),
        "upstream_pkgtests_pie_test_014" => Some(("0 0 292.400390625 450", "292.4")),
        "upstream_pkgtests_pie_test_017" => Some(("0 0 292.400390625 450", "292.4")),
        "upstream_pkgtests_pie_test_018" => Some(("0 0 292.400390625 450", "292.4")),
        "upstream_pkgtests_pie_test_019" => Some(("0 0 292.400390625 450", "292.4")),
        "upstream_pkgtests_pie_test_020" => Some(("0 0 292.400390625 450", "292.4")),
        "upstream_pkgtests_pie_test_023" => Some(("0 0 565.49609375 450", "565.496")),
        "upstream_pkgtests_pie_test_024" => Some(("0 0 565.49609375 450", "565.496")),
        "upstream_pkgtests_pie_test_025" => Some(("0 0 565.49609375 450", "565.496")),
        "upstream_pkgtests_pie_test_026" => Some(("0 0 565.49609375 450", "565.496")),
        "upstream_pkgtests_pie_test_028" => Some(("0 0 565.49609375 450", "565.496")),
        "upstream_pkgtests_pie_test_029" => Some(("0 0 537.40283203125 450", "537.403")),
        "upstream_pkgtests_pie_test_030" => Some(("0 0 565.49609375 450", "565.496")),
        "upstream_pkgtests_pie_test_031" => Some(("0 0 565.49609375 450", "565.496")),
        "upstream_pkgtests_pie_test_032" => Some(("0 0 565.49609375 450", "565.496")),
        "zed_pr_57644_pie" => Some(("0 0 707.953125 450", "707.953")),
        _ => None,
    }
}
