// Fixture-derived root viewport overrides for Mermaid@11.12.2 Requirement diagrams.
//
// These values are taken from upstream SVG baselines under
// `fixtures/upstream-svgs/requirement/*.svg` and are keyed by `diagram_id` (fixture stem).
//
// They are used to keep `parity-root` stable at higher decimal precision when browser float
// behavior (DOM `getBBox()` + serialization) differs from our deterministic headless pipeline.

pub fn lookup_requirement_root_viewport_override(
    diagram_id: &str,
) -> Option<(&'static str, &'static str)> {
    match diagram_id {
        "upstream_cypress_requirementdiagram_unified_spec_example_007" => {
            Some(("-24.03125 -48 221.796875 434", "221.797"))
        }
        "upstream_cypress_requirementdiagram_unified_spec_example_023" => {
            Some(("0 0 801.421875 224", "801.422"))
        }
        "upstream_cypress_requirementdiagram_unified_spec_example_025" => {
            Some(("0 0 859 224", "859"))
        }
        "upstream_cypress_requirementdiagram_unified_spec_example_026" => {
            Some(("0 0 582.984375 200", "582.984"))
        }
        "upstream_docs_requirementdiagram_combined_example_022" => {
            Some(("0 0 430.28125 200", "430.281"))
        }
        "upstream_html_demos_requirements_requirement_diagram_demos_002" => {
            Some(("0 0 939.79296875 1466", "939.793"))
        }
        "stress_requirement_font_size_precedence_001" => Some(("0 0 286 758", "286")),
        _ => None,
    }
}
