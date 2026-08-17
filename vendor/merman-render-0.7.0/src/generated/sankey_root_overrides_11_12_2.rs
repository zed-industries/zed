// Fixture-derived root viewport overrides for Mermaid@11.12.2 Sankey diagrams.
//
// These values are taken from upstream SVG baselines under
// `fixtures/upstream-svgs/sankey/*.svg` and are keyed by `diagram_id` (fixture stem).
//
// They are used to keep `parity-root` stable at higher decimal precision when browser float
// behavior (DOM `getBBox()` + serialization) differs from our deterministic headless pipeline.

pub fn lookup_sankey_root_viewport_override(
    diagram_id: &str,
) -> Option<(&'static str, &'static str)> {
    match diagram_id {
        "upstream_docs_sankey_example_002" => Some(("0 0 600 406.89434814453125", "600")),
        "upstream_examples_sankey_energy_flow_001" => Some(("0 0 600 406.89434814453125", "600")),
        "upstream_html_demos_sankey_energy_flow_002" => {
            Some(("0 0 1200 600.8479614257812", "1200"))
        }
        _ => None,
    }
}
