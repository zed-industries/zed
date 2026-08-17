use crate::*;
use futures::executor::block_on;
use serde_json::{Value, json};
use std::fmt::Write;

#[cfg(feature = "large-features")]
#[test]
fn full_build_detects_mindmap() {
    let engine = Engine::new();
    let res = block_on(engine.parse_metadata("mindmap\n  root", ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.diagram_type, "mindmap");
}

#[cfg(not(feature = "large-features"))]
#[test]
fn tiny_build_does_not_detect_mindmap() {
    let engine = Engine::new();
    let err =
        block_on(engine.parse_metadata("mindmap\n  root", ParseOptions::default())).unwrap_err();
    assert!(
        err.to_string()
            .contains("No diagram type detected matching given configuration")
    );
}

#[cfg(feature = "large-features")]
#[test]
fn full_build_detects_flowchart_elk_and_sets_layout() {
    let engine = Engine::new();
    let res = block_on(engine.parse_metadata("flowchart-elk TD\nA-->B", ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.diagram_type, "flowchart-elk");
    assert_eq!(res.effective_config.get_str("layout"), Some("elk"));
}

#[cfg(not(feature = "large-features"))]
#[test]
fn tiny_build_flowchart_elk_falls_back_to_flowchart_v2() {
    let engine = Engine::new();
    let res = block_on(engine.parse_metadata("flowchart-elk TD\nA-->B", ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.diagram_type, "flowchart-v2");
    assert_eq!(res.effective_config.get_str("layout"), Some("dagre"));
}

#[test]
fn engine_with_site_config_preserves_default_renderer_for_detection() {
    let engine = Engine::new().with_site_config({
        let mut cfg = MermaidConfig::empty_object();
        cfg.set_value("securityLevel", json!("sandbox"));
        cfg
    });

    let text = r#"classDiagram
class Class1
"#;
    let res = block_on(engine.parse_metadata(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.diagram_type, "classDiagram");
}

#[test]
fn detects_tree_view_beta_as_tree_view() {
    let engine = Engine::new();
    let res = block_on(engine.parse_metadata("treeView-beta\n\"Root\"", ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.diagram_type, "treeView");
}

#[test]
fn detects_ishikawa_headers_as_ishikawa() {
    let engine = Engine::new();
    for header in ["ishikawa", "ishikawa-beta", "ISHIKAWA-BETA"] {
        let res =
            block_on(engine.parse_metadata(&format!("{header}\nProblem"), ParseOptions::default()))
                .unwrap()
                .unwrap();
        assert_eq!(res.diagram_type, "ishikawa");
    }
}

#[test]
fn detects_eventmodeling_as_eventmodeling() {
    let engine = Engine::new();
    let res =
        block_on(engine.parse_metadata("eventmodeling\ntf 01 evt Start", ParseOptions::default()))
            .unwrap()
            .unwrap();
    assert_eq!(res.diagram_type, "eventmodeling");
}

#[test]
fn detects_venn_beta_as_venn() {
    let engine = Engine::new();
    let res = block_on(engine.parse_metadata("venn-beta\nset A", ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.diagram_type, "venn");
}

#[test]
fn c4_detector_preserves_upstream_ungrouped_regex_shape() {
    let engine = Engine::new();

    let anchored = engine
        .parse_metadata_sync("  C4Context\nPerson(a, \"A\")", ParseOptions::strict())
        .unwrap()
        .unwrap();
    assert_eq!(anchored.diagram_type, "c4");

    let ungrouped_anywhere = engine
        .parse_metadata_sync("kanban\nC4Container", ParseOptions::strict())
        .unwrap()
        .unwrap();
    assert_eq!(ungrouped_anywhere.diagram_type, "c4");

    let err = engine
        .parse_metadata_sync("not a diagram C4Context", ParseOptions::strict())
        .unwrap_err();
    assert!(
        err.to_string()
            .contains("No diagram type detected matching given configuration"),
        "unexpected error: {err}"
    );
}

#[test]
fn detector_registry_strips_mermaid_comment_lines_without_regex() {
    let registry = DetectorRegistry::for_pinned_mermaid_baseline();
    let mut config = MermaidConfig::empty_object();

    for source in [
        "\n\n%% This is a comment\nflowchart TD\nA-->B\n",
        "    %% This is a comment\nflowchart TD\nA-->B\n",
        "flowchart TD\nA-->B\n%% This is a comment",
        "%%{init: {'theme': 'forest'}}%%\nflowchart TD\nA-->B\n",
    ] {
        let detected = registry
            .detect_type(source, &mut config)
            .expect("detect type");
        assert_eq!(detected, "flowchart-v2", "source: {source:?}");
    }
}

#[test]
fn preprocess_strips_mermaid_comment_at_eof_without_regex() {
    let registry = DetectorRegistry::for_pinned_mermaid_baseline();
    let result = preprocess_diagram("flowchart TD\nA-->B\n%% This is a comment", &registry)
        .expect("preprocess succeeds");

    assert_eq!(result.code, "flowchart TD\nA-->B\n");
}

#[test]
fn preprocess_normalizes_crlf_without_regex() {
    let registry = DetectorRegistry::for_pinned_mermaid_baseline();
    let result = preprocess_diagram("flowchart TD\r\nA-->B\r%% This is a comment", &registry)
        .expect("preprocess succeeds");

    assert_eq!(result.code, "flowchart TD\nA-->B\n");
}

#[test]
fn preprocess_encodes_entities_without_entity_regex() {
    let registry = DetectorRegistry::for_pinned_mermaid_baseline();
    let result = preprocess_diagram("flowchart TD\nA[#there;]\nB[#77653;]", &registry)
        .expect("preprocess succeeds");

    assert!(result.code.contains("A[ﬂ°there¶ß]"), "{:?}", result.code);
    assert!(result.code.contains("B[ﬂ°°77653¶ß]"), "{:?}", result.code);
}

#[test]
fn preprocess_rewrites_html_attributes_without_regex() {
    let registry = DetectorRegistry::for_pinned_mermaid_baseline();
    let result = preprocess_diagram(
        r#"flowchart TD
A["<span title="alpha" data-empty="">Label</span>"]
B["<é title="unchanged">Local</é>"]"#,
        &registry,
    )
    .expect("preprocess succeeds");

    assert!(
        result
            .code
            .contains(r#"A["<span title='alpha' data-empty=''>Label</span>"]"#),
        "{:?}",
        result.code
    );
    assert!(
        result
            .code
            .contains(r#"B["<é title="unchanged">Local</é>"]"#),
        "{:?}",
        result.code
    );
}

#[test]
fn detector_registry_strips_deep_frontmatter_with_small_stack() {
    const DEPTH: usize = 512;
    let mut text = String::from("---\nconfig: {\"sequence\": ");
    for idx in 0..DEPTH {
        write!(&mut text, r#"{{"k{idx}":"#).expect("write frontmatter config");
    }
    text.push_str("\"leaf\"");
    for _ in 0..DEPTH {
        text.push('}');
    }
    text.push_str("}\n---\nsequenceDiagram\nAlice->Bob: Hi\n");
    let registry = DetectorRegistry::for_pinned_mermaid_baseline();

    let handle = std::thread::Builder::new()
        .name("detector-deep-frontmatter-strip".to_string())
        .stack_size(64 * 1024)
        .spawn(move || {
            let mut config = MermaidConfig::empty_object();
            let detected = registry
                .detect_type(&text, &mut config)
                .expect("detect type");
            assert_eq!(detected, "sequence");
        })
        .expect("spawn detector deep frontmatter test");
    handle
        .join()
        .expect("detector frontmatter stripping should finish without stack overflow");
}

#[test]
fn auto_detect_common_headers_with_deep_config_small_stack() {
    const DEPTH: usize = 1_024;
    let mut value = Value::String("#778899".to_string());
    for idx in (0..DEPTH).rev() {
        let mut map = serde_json::Map::new();
        map.insert(format!("k{idx}"), value);
        value = Value::Object(map);
    }
    let mut root = serde_json::Map::new();
    root.insert("retainedConfig".to_string(), value);
    let engine = Engine::new().with_site_config(MermaidConfig::from_value(Value::Object(root)));

    let handle = std::thread::Builder::new()
        .name("detect-common-headers-deep-config".to_string())
        .stack_size(128 * 1024)
        .spawn(move || {
            for (source, expected_type) in [
                ("block\n  A\n", "block"),
                ("sankey\nA,B,1\n", "sankey"),
                ("treemap\n\"A\": 1\n", "treemap"),
                ("C4Context\nPerson(a, \"A\")\n", "c4"),
            ] {
                let meta = engine
                    .parse_metadata_sync(source, ParseOptions::strict())
                    .expect("parse succeeds")
                    .expect("diagram detected");
                assert_eq!(meta.diagram_type, expected_type);
            }
        })
        .expect("spawn common header detect test");
    handle
        .join()
        .expect("common header detection should finish without stack overflow");
}
