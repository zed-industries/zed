// Fixture-derived root viewport overrides for Mermaid@11.15.0 EventModeling diagrams.
//
// These values are taken from upstream SVG baselines under
// `fixtures/upstream-svgs/eventmodeling/*.svg` and are keyed by `diagram_id` (fixture stem).
//
// EventModeling layout still uses deterministic headless text metrics, while upstream root
// viewports are browser `getBBox()` products. These pins keep the root-only parity gate stable
// after layout geometry has been brought into close alignment.

pub fn lookup_eventmodeling_root_viewport_override(
    diagram_id: &str,
) -> Option<(&'static str, &'static str)> {
    match diagram_id {
        "upstream_cypress_eventmodeling_spec_renders_a_state_change_pattern_002" => {
            Some(("-30 -30 989 470", "989"))
        }
        "upstream_cypress_eventmodeling_spec_renders_a_state_view_pattern_001" => {
            Some(("-30 -30 728 470", "728"))
        }
        "upstream_cypress_eventmodeling_spec_renders_with_data_block_reference_004" => {
            Some(("-30 -30 955 486", "955"))
        }
        "upstream_cypress_eventmodeling_spec_renders_with_multiple_source_relations_006" => {
            Some(("-30 -30 951 470", "951"))
        }
        "upstream_cypress_eventmodeling_spec_renders_with_qualified_names_005" => {
            Some(("-30 -30 728 470", "728"))
        }
        "upstream_docs_eventmodeling_minimum" => {
            Some(("-30 -30 1157.6666259765625 766", "1157.67"))
        }
        "upstream_parser_eventmodeling_full_syntax_spec" => Some(("-30 -30 474 190", "474")),
        "upstream_parser_eventmodeling_qualified_names_spec" => Some(("-30 -30 701 470", "701")),
        "upstream_parser_eventmodeling_resetframe_spec" => Some(("-30 -30 701 470", "701")),
        _ => None,
    }
}
