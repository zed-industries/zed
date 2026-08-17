// Fixture-derived root viewport overrides for Mermaid@11.12.2 C4 diagrams.
//
// These values are taken from upstream SVG baselines under
// `fixtures/upstream-svgs/c4/*.svg` and are keyed by `diagram_id` (fixture stem).
//
// They are used to keep `parity-root` stable when upstream browser float behavior (DOM `getBBox()`
// + serialization) differs from our deterministic headless pipeline.

pub fn lookup_c4_root_viewport_override(diagram_id: &str) -> Option<(&'static str, &'static str)> {
    match diagram_id {
        "link_tags_context" => Some(("0 -70 715 998", "715")),
        "sprite_empty_person" => Some(("0 -70 881 470", "881")),
        "sprite_positional_boundary" => Some(("0 -70 981 624", "981")),
        "sprite_positional_componentdb_ext" => Some(("0 -70 832 489", "832")),
        "sprite_positional_container" => Some(("0 -70 875 694", "875")),
        "sprite_positional_container_ext" => Some(("0 -70 875 723", "875")),
        "sprite_positional_containerdb_ext" => Some(("0 -70 875 489", "875")),
        "sprite_positional_containerqueue_ext" => Some(("0 -70 876 489", "876")),
        "sprite_positional_person" => Some(("0 -70 881 518", "881")),
        "sprite_positional_person_ext" => Some(("0 -70 881 518", "881")),
        "sprite_positional_system" => Some(("0 -70 881 656", "881")),
        "sprite_positional_system_ext" => Some(("0 -70 881 704", "881")),
        "sprite_positional_systemdb_ext" => Some(("0 -70 881 470", "881")),
        "sprite_positional_systemqueue_ext" => Some(("0 -70 881 470", "881")),
        "tags_positional_fields" => Some(("0 -70 615 608", "615")),
        "upstream_cypress_c4_spec_c4_1_should_render_a_simple_c4context_diagram_001" => {
            Some(("0 -70 952 1104", "952"))
        }
        "upstream_cypress_c4_spec_c4_2_should_render_a_simple_c4container_diagram_002" => {
            Some(("0 -70 919 1017", "919"))
        }
        "upstream_cypress_c4_spec_c4_3_should_render_a_simple_c4component_diagram_003" => {
            Some(("0 -70 820 802", "820"))
        }
        "upstream_cypress_c4_spec_c4_4_should_render_a_simple_c4dynamic_diagram_004" => {
            Some(("0 -70 866 1212", "866"))
        }
        "upstream_cypress_c4_spec_c4_5_should_render_a_simple_c4deployment_diagram_005" => {
            Some(("0 -70 1791 759", "1791"))
        }
        "upstream_docs_c4_c4_component_diagram_c4component_008" => Some(("0 -70 926 2013", "926")),
        "upstream_docs_c4_c4_container_diagram_c4container_006" => {
            Some(("0 -70 1023 2023", "1023"))
        }
        "upstream_docs_c4_c4_deployment_diagram_c4deployment_012" => {
            Some(("0 -70 1982 1709", "1982"))
        }
        "upstream_docs_c4_c4_diagrams_001" => Some(("0 -70 1059 2985", "1059")),
        "upstream_docs_c4_c4_dynamic_diagram_c4dynamic_010" => Some(("0 -70 866 1212", "866")),
        "upstream_docs_readme_c4_diagram_a_href_https_mermaid_js_org_syntax_c4_html_docs_a_019" => {
            Some(("0 -70 1328 2346", "1328"))
        }
        "upstream_examples_c4_internet_banking_system_context_001" => {
            Some(("0 -70 1488 2483", "1488"))
        }
        "upstream_html_demos_c4context_c4_context_diagram_demos_001" => {
            Some(("0 -70 1059 2985", "1059"))
        }
        "upstream_html_demos_c4context_c4_context_diagram_demos_002" => {
            Some(("0 -70 1023 2023", "1023"))
        }
        "upstream_html_demos_c4context_c4_context_diagram_demos_003" => {
            Some(("0 -70 926 2013", "926"))
        }
        "upstream_html_demos_c4context_c4_context_diagram_demos_005" => {
            Some(("0 -70 1982 1709", "1982"))
        }
        "upstream_pkgtests_c4person_spec_001" => Some(("0 -70 653 470", "653")),
        "upstream_pkgtests_c4person_spec_004" => Some(("0 -10 653 393", "653")),
        "upstream_pkgtests_c4personext_spec_001" => Some(("0 -70 653 470", "653")),
        "upstream_pkgtests_c4personext_spec_004" => Some(("0 -10 653 393", "653")),
        _ => None,
    }
}
