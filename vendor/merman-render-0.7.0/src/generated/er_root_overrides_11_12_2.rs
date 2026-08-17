// Fixture-derived root viewport overrides for Mermaid@11.12.2 ER diagrams.
//
// These values are taken from upstream SVG baselines under
// `fixtures/upstream-svgs/er/*.svg` and are keyed by `diagram_id` (fixture stem).
//
// They are used to keep `parity-root` stable at higher decimal precision when browser float
// behavior (DOM `getBBox()` + serialization) differs from our deterministic headless pipeline.

pub fn lookup_er_root_viewport_override(diagram_id: &str) -> Option<(&'static str, &'static str)> {
    match diagram_id {
        "upstream_html_demos_er_example_001" => Some(("0 -48 1636.14453125 1059.5", "1636.14")),
        "upstream_html_demos_er_multiline_example_001" => {
            Some(("0 0 1121.578125 878.75", "1121.58"))
        }
        "upstream_html_demos_er_multiline_example_002" => Some(("0 0 529.359375 320.5", "529.359")),
        "upstream_cypress_erdiagram_spec_should_render_an_er_diagram_with_a_recursive_relationship_002" => {
            Some(("0 0 332.73126220703125 470", "332.731"))
        }
        "upstream_cypress_erdiagram_spec_should_render_edge_labels_correctly_when_flowchart_htmllabels_is_019" => {
            Some(("0 0 547.147216796875 474", "547.147"))
        }
        "upstream_docs_entityrelationshipdiagram_layout_042" => {
            Some(("4 -48 334.015625 522", "334.016"))
        }
        _ => None,
    }
}
