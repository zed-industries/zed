use crate::baseline::BaselineRegistryProfile;
use crate::{DetectorRegistry, DiagramRegistry, MermaidConfig, RenderDiagramRegistry};
use std::collections::BTreeSet;

#[test]
fn detector_registries_follow_family_fact_order() {
    let full = DetectorRegistry::pinned_mermaid_baseline_full();
    let full_actual: Vec<_> = full.detector_ids().collect();
    let full_expected: Vec<_> = crate::family::detector_facts(BaselineRegistryProfile::Full)
        .iter()
        .map(|fact| fact.id)
        .collect();
    assert_eq!(full_actual, full_expected);

    let tiny = DetectorRegistry::pinned_mermaid_baseline_tiny();
    let tiny_actual: Vec<_> = tiny.detector_ids().collect();
    let tiny_expected: Vec<_> = crate::family::detector_facts(BaselineRegistryProfile::Tiny)
        .iter()
        .map(|fact| fact.id)
        .collect();
    assert_eq!(tiny_actual, tiny_expected);
}

#[test]
fn fast_detector_respects_family_feature_profile() {
    let mut config = MermaidConfig::empty_object();
    let full = DetectorRegistry::pinned_mermaid_baseline_full();
    assert_eq!(
        full.detect_type_precleaned("mindmap\n  root", &mut config)
            .unwrap(),
        "mindmap"
    );

    let tiny = DetectorRegistry::pinned_mermaid_baseline_tiny();
    let err = tiny
        .detect_type_precleaned("mindmap\n  root", &mut config)
        .unwrap_err();
    assert!(
        err.to_string()
            .contains("No diagram type detected matching given configuration")
    );
}

#[test]
fn parser_registries_follow_family_fact_projection() {
    let semantic = DiagramRegistry::for_pinned_mermaid_baseline();
    let semantic_actual = sorted_set(semantic.parser_ids());
    let semantic_expected = sorted_set(
        crate::family::semantic_parser_facts()
            .iter()
            .map(|fact| fact.id),
    );
    assert_eq!(semantic_actual, semantic_expected);

    let render = RenderDiagramRegistry::for_pinned_mermaid_baseline();
    let render_actual = sorted_set(render.parser_ids());
    let render_expected = sorted_set(
        crate::family::render_parser_facts()
            .iter()
            .map(|fact| fact.id),
    );
    assert_eq!(render_actual, render_expected);
}

#[test]
fn supported_diagram_metadata_is_backed_by_typed_render_projection() {
    assert_eq!(
        crate::supported_diagrams(),
        &[
            "architecture",
            "block",
            "c4",
            "class",
            "er",
            "flowchart",
            "gantt",
            "gitgraph",
            "info",
            "journey",
            "kanban",
            "mindmap",
            "packet",
            "pie",
            "quadrantchart",
            "radar",
            "requirement",
            "sankey",
            "sequence",
            "state",
            "timeline",
            "treemap",
            "venn",
            "xychart",
            "zenuml",
        ]
    );

    let render_ids = sorted_set(
        crate::family::render_parser_facts()
            .iter()
            .map(|fact| fact.id),
    );
    for fact in crate::family::supported_diagram_facts() {
        for parser_id in &fact.render_parser_ids {
            assert!(
                render_ids.contains(parser_id),
                "{} metadata points to missing render parser {parser_id}",
                fact.metadata_id
            );
        }
    }
}

#[test]
fn pinned_non_error_semantic_parsers_are_backed_by_typed_render_parsers() {
    let render_ids = sorted_set(
        crate::family::render_parser_facts()
            .iter()
            .map(|fact| fact.id),
    );

    for fact in crate::family::semantic_parser_facts() {
        if fact.id == "error" {
            continue;
        }

        assert!(
            render_ids.contains(fact.id),
            "built-in semantic parser {} must not rely on JSON render fallback",
            fact.id
        );
    }
}

fn sorted_set(ids: impl IntoIterator<Item = &'static str>) -> BTreeSet<&'static str> {
    ids.into_iter().collect()
}
