use crate::diagrams::xychart::{XyChartAxisRenderModel, XyChartPlotType};
use crate::*;
use futures::executor::block_on;
use serde_json::{Map, Value, json};
use std::fmt::Write;

#[test]
fn parse_graph_defaults_to_flowchart_v2() {
    let engine = Engine::new();
    let res = block_on(engine.parse_metadata("graph TD;A-->B;", ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.diagram_type, "flowchart-v2");
    assert_eq!(res.config.as_value(), &json!({}));
}

#[test]
fn parse_merges_frontmatter_and_directive_config() {
    let engine = Engine::new();
    let text = r#"---
config:
  theme: forest
  flowchart:
    htmlLabels: true
---
%%{init: { 'theme': 'base' } }%%
graph TD;A-->B;"#;

    let res = block_on(engine.parse_metadata(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(
        res.config.as_value(),
        &json!({
            "theme": "base",
            "flowchart": {
                "htmlLabels": true
            }
        })
    );
}

#[test]
fn parse_maps_top_level_frontmatter_diagram_config() {
    let engine = Engine::new();
    let text = r#"---
title: Frontmatter Example
displayMode: compact
config:
  theme: forest
gantt:
  useWidth: 400
  topAxis: true
  numberSectionStyles: 2
unknownDiagram:
  ignored: true
---
gantt
    section Waffle
        Iron  : 1982, 3y
        House : 1986, 3y
"#;

    let res = block_on(engine.parse_metadata(text, ParseOptions::default()))
        .unwrap()
        .unwrap();

    assert_eq!(res.diagram_type, "gantt");
    assert_eq!(
        res.config.as_value(),
        &json!({
            "theme": "forest",
            "gantt": {
                "displayMode": "compact",
                "useWidth": 400,
                "topAxis": true,
                "numberSectionStyles": 2
            }
        })
    );
}

#[test]
fn parse_frontmatter_config_takes_priority_over_diagram_compat() {
    let engine = Engine::new();
    let text = r#"---
config:
  look: neo
  layout: elk
  gantt:
    useWidth: 640
gantt:
  useWidth: 400
  rightPadding: 10
classDiagram:
  htmlLabels: false
---
gantt
    section Waffle
        Iron  : 1982, 3y
"#;

    let res = block_on(engine.parse_metadata(text, ParseOptions::default()))
        .unwrap()
        .unwrap();

    assert_eq!(res.config.get_str("look"), Some("neo"));
    assert_eq!(res.config.get_str("layout"), Some("elk"));
    assert_eq!(res.config.as_value()["gantt"]["useWidth"], json!(640));
    assert_eq!(res.config.as_value()["gantt"]["rightPadding"], json!(10));
    assert_eq!(res.config.get_bool("class.htmlLabels"), Some(false));
}

#[test]
fn parse_diagram_with_type_sync_matches_auto_detect_for_flowchart_v2() {
    let engine = Engine::new();
    let input = "flowchart TD; A[Start]-->B[End];";

    let auto = engine
        .parse_diagram_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();
    assert_eq!(auto.meta.diagram_type, "flowchart-v2");

    let known = engine
        .parse_diagram_with_type_sync("flowchart-v2", input, ParseOptions::strict())
        .unwrap()
        .unwrap();

    assert_eq!(known.meta.diagram_type, "flowchart-v2");
    assert_eq!(known.model, auto.model);
}

#[test]
fn parse_metadata_with_type_sync_moves_init_config_without_detection() {
    let engine = Engine::new();
    let input = "%%{init: {\"config\": {\"htmlLabels\": true}}}%%\nflowchart TD; A-->B;";

    let meta = engine
        .parse_metadata_with_type_sync("flowchart-v2", input, ParseOptions::strict())
        .unwrap()
        .unwrap();

    // Mermaid special-case: `flowchart-v2` config is stored under `flowchart`.
    assert_eq!(meta.config.get_bool("flowchart.htmlLabels"), Some(true));
}

#[test]
fn parse_metadata_with_type_sync_preserves_flowchart_elk_layout_side_effect() {
    let engine = Engine::new();
    let input = "flowchart-elk TD\nA-->B;";

    let meta = engine
        .parse_metadata_with_type_sync("flowchart-elk", input, ParseOptions::strict())
        .unwrap()
        .unwrap();
    assert_eq!(meta.effective_config.get_str("layout"), Some("elk"));
}

#[test]
fn parse_sanitizes_frontmatter_title_like_mermaid_common_db() {
    let engine = Engine::new();
    let text = r#"---
title: "Flowchart v2 arrows: graph direction \"<\""
---
graph <
A-->B
"#;

    let meta = block_on(engine.parse_metadata(text, ParseOptions::default()))
        .unwrap()
        .unwrap();

    assert_eq!(
        meta.title,
        Some(r#"Flowchart v2 arrows: graph direction "&lt;""#.to_string())
    );
}

#[test]
fn parse_merges_init_directive_numeric_values_like_upstream() {
    let engine = Engine::new();
    let text = r#"%%{init: { 'logLevel': 0 } }%%
sequenceDiagram
Alice->Bob: Hi
"#;

    let res = block_on(engine.parse_metadata(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.diagram_type, "sequence");
    assert_eq!(res.config.as_value(), &json!({ "logLevel": 0 }));
}

#[test]
fn parse_init_font_family_mirrors_legacy_theme_variable_like_upstream() {
    let engine = Engine::new();
    let text = r#"%%{init: { "fontFamily": "Courier" } }%%
graph TD;A-->B;
"#;

    let res = block_on(engine.parse_metadata(text, ParseOptions::default()))
        .unwrap()
        .unwrap();

    assert_eq!(res.config.get_str("fontFamily"), Some("Courier"));
    assert_eq!(
        res.config.get_str("themeVariables.fontFamily"),
        Some("Courier")
    );
    assert_eq!(
        res.effective_config.get_str("themeVariables.fontFamily"),
        Some("Courier")
    );
}

#[test]
fn parse_init_theme_variable_font_family_overrides_legacy_root() {
    let engine = Engine::new();
    let text = r#"%%{init: { "fontFamily": "Courier", "themeVariables": { "fontFamily": "Inter" } } }%%
graph TD;A-->B;
"#;

    let res = block_on(engine.parse_metadata(text, ParseOptions::default()))
        .unwrap()
        .unwrap();

    assert_eq!(res.config.get_str("fontFamily"), Some("Courier"));
    assert_eq!(
        res.config.get_str("themeVariables.fontFamily"),
        Some("Inter")
    );
    assert_eq!(
        res.effective_config.get_str("themeVariables.fontFamily"),
        Some("Inter")
    );
}

#[test]
fn parse_architecture_exposes_11_15_fcose_config_defaults_and_overrides() {
    let engine = Engine::new();
    let default = block_on(engine.parse_metadata(
        "architecture-beta\n  service a(server)[A]\n",
        ParseOptions::strict(),
    ))
    .unwrap()
    .unwrap();

    let arch = &default.effective_config.as_value()["architecture"];
    assert_eq!(arch["randomize"], json!(false));
    assert_eq!(arch["nodeSeparation"], json!(75));
    assert_eq!(arch["idealEdgeLengthMultiplier"], json!(1.5));
    assert_eq!(arch["edgeElasticity"], json!(0.45));
    assert_eq!(arch["numIter"], json!(2500));
    assert_eq!(arch["seed"], json!(1));

    let configured = block_on(engine.parse_metadata(
        r#"%%{init: {"architecture": {"randomize": true, "nodeSeparation": 120, "idealEdgeLengthMultiplier": 2, "edgeElasticity": 0.6, "numIter": 5000, "seed": 7}}}%%
architecture-beta
  service a(server)[A]
"#,
        ParseOptions::strict(),
    ))
    .unwrap()
    .unwrap();

    let arch = &configured.effective_config.as_value()["architecture"];
    assert_eq!(arch["randomize"], json!(true));
    assert_eq!(arch["nodeSeparation"], json!(120));
    assert_eq!(arch["idealEdgeLengthMultiplier"], json!(2));
    assert_eq!(arch["edgeElasticity"], json!(0.6));
    assert_eq!(arch["numIter"], json!(5000));
    assert_eq!(arch["seed"], json!(7));
}

#[test]
fn site_config_deep_merge_handles_deep_public_config_with_small_stack() {
    const DEPTH: usize = 1_024;
    let site_config = MermaidConfig::from_value(deep_config_value(
        "sequence",
        DEPTH,
        Value::String("#112233".to_string()),
    ));
    let engine = Engine::new();

    let handle = std::thread::Builder::new()
        .name("merman-core-deep-site-config".to_string())
        .stack_size(128 * 1024)
        .spawn(move || {
            let engine = engine.with_site_config(site_config);
            let meta = engine
                .parse_metadata_sync("sequenceDiagram\nAlice->Bob: Hi", ParseOptions::strict())
                .expect("parse succeeds")
                .expect("diagram detected");

            assert_eq!(
                deep_config_leaf(meta.effective_config.as_value(), "sequence", DEPTH)
                    .and_then(Value::as_str),
                Some("#112233")
            );
        })
        .expect("spawn deep site config test");
    handle
        .join()
        .expect("deep site config merge should finish without stack overflow");
}

#[test]
fn retained_semantic_config_handles_deep_public_config_with_small_stack() {
    const DEPTH: usize = 1_024;
    let site_config = MermaidConfig::from_value(deep_config_value(
        "retainedConfig",
        DEPTH,
        Value::String("#556677".to_string()),
    ));
    let engine = Engine::new().with_site_config(site_config);

    let handle = std::thread::Builder::new()
        .name("merman-core-deep-retained-semantic-config".to_string())
        .stack_size(128 * 1024)
        .spawn(move || {
            for (label, diagram_type, source) in [
                ("block", "block", "block\n  A\n"),
                ("state", "stateDiagram", "stateDiagram-v2\n  [*] --> A\n"),
                ("treemap", "treemap", "treemap\n\"A\": 1\n"),
                ("sankey", "sankey", "sankey\nA,B,1\n"),
                ("c4", "c4", "C4Context\nPerson(a, \"A\")\n"),
                (
                    "architecture",
                    "architecture",
                    "architecture-beta\n  service a(server)[A]\n",
                ),
            ] {
                let parsed = engine
                    .parse_diagram_with_type_sync(diagram_type, source, ParseOptions::strict())
                    .expect("parse succeeds")
                    .expect("diagram detected");
                let ParsedDiagram { model, .. } = parsed;

                assert_eq!(
                    deep_config_leaf(&model["config"], "retainedConfig", DEPTH)
                        .and_then(Value::as_str),
                    Some("#556677"),
                    "retained config for {label}"
                );

                crate::config::drop_value_nonrecursive(model);
            }
        })
        .expect("spawn deep retained semantic config test");
    handle
        .join()
        .expect("retained semantic config projection should finish without stack overflow");
}

#[test]
fn remaining_retained_semantic_config_handles_deep_public_config_with_small_stack() {
    const DEPTH: usize = 1_024;
    let site_config = MermaidConfig::from_value(deep_config_value(
        "retainedConfig",
        DEPTH,
        Value::String("#778899".to_string()),
    ));
    let engine = Engine::new().with_site_config(site_config);

    let handle = std::thread::Builder::new()
        .name("merman-core-remaining-deep-retained-config".to_string())
        .stack_size(128 * 1024)
        .spawn(move || {
            for (label, diagram_type, source) in [
                ("gitGraph", "gitGraph", "gitGraph:\n commit\n"),
                ("kanban", "kanban", "kanban\n  Todo\n    item\n"),
                ("packet", "packet", "packet\n+8: \"byte\"\n"),
                (
                    "quadrantChart",
                    "quadrantChart",
                    "quadrantChart\nx-axis Low --> High\ny-axis Low --> High\nquadrant-1 A\nP: [0.5, 0.5]\n",
                ),
                ("radar", "radar", "radar-beta\naxis A, B\ncurve one{1,2}\n"),
                (
                    "requirement",
                    "requirement",
                    "requirementDiagram\nrequirement r {\n  id: R\n  text: \"T\"\n  risk: low\n  verifymethod: test\n}\n",
                ),
                ("mindmap", "mindmap", "mindmap\nroot\n child\n"),
                ("mindmap-empty", "mindmap", "mindmap\n"),
            ] {
                let parsed = engine
                    .parse_diagram_with_type_sync(diagram_type, source, ParseOptions::strict())
                    .expect("parse succeeds")
                    .expect("diagram detected");
                let ParsedDiagram { model, .. } = parsed;

                assert_eq!(
                    deep_config_leaf(&model["config"], "retainedConfig", DEPTH)
                        .and_then(Value::as_str),
                    Some("#778899"),
                    "retained config for {label}"
                );

                crate::config::drop_value_nonrecursive(model);
            }
        })
        .expect("spawn remaining deep retained semantic config test");
    handle.join().expect(
        "remaining retained semantic config projection should finish without stack overflow",
    );
}

#[test]
fn init_directive_config_sanitizes_deep_values_with_small_stack() {
    const DEPTH: usize = 32;
    let source = deep_init_directive_source("sequence", DEPTH, "<blocked>");

    let handle = std::thread::Builder::new()
        .name("merman-core-deep-init-config".to_string())
        .stack_size(256 * 1024)
        .spawn(move || {
            let meta = Engine::new()
                .parse_metadata_sync(&source, ParseOptions::strict())
                .expect("parse succeeds")
                .expect("diagram detected");

            assert_eq!(
                deep_config_leaf(meta.config.as_value(), "sequence", DEPTH).and_then(Value::as_str),
                Some("")
            );
        })
        .expect("spawn deep init config test");
    handle
        .join()
        .expect("deep init config should finish without stack overflow");
}

#[test]
fn frontmatter_config_deep_merge_handles_deep_values_with_small_stack() {
    const DEPTH: usize = 32;
    let source = deep_frontmatter_config_source("sequence", DEPTH, "#334455");

    let handle = std::thread::Builder::new()
        .name("merman-core-deep-frontmatter-config".to_string())
        .stack_size(256 * 1024)
        .spawn(move || {
            let meta = Engine::new()
                .parse_metadata_sync(&source, ParseOptions::strict())
                .expect("parse succeeds")
                .expect("diagram detected");

            assert_eq!(
                deep_config_leaf(meta.config.as_value(), "sequence", DEPTH).and_then(Value::as_str),
                Some("#334455")
            );
        })
        .expect("spawn deep frontmatter config test");
    handle
        .join()
        .expect("deep frontmatter config should finish without stack overflow");
}

#[test]
fn init_directive_rejects_excessive_config_nesting_with_small_stack() {
    const DEPTH: usize = 300;
    let source = deep_init_directive_source("sequence", DEPTH, "#112233");
    let engine = Engine::new();

    let handle = std::thread::Builder::new()
        .name("merman-core-too-deep-init-config".to_string())
        .stack_size(128 * 1024)
        .spawn(move || {
            let err = engine
                .parse_metadata_sync(&source, ParseOptions::strict())
                .expect_err("excessive init config depth should be rejected");
            assert!(
                err.to_string().contains("config nesting exceeds"),
                "unexpected error: {err}"
            );
        })
        .expect("spawn excessive init config test");
    handle
        .join()
        .expect("excessive init config should return an error without stack overflow");
}

#[test]
fn frontmatter_rejects_excessive_config_nesting_with_small_stack() {
    const DEPTH: usize = 300;
    let source = deep_frontmatter_config_source("sequence", DEPTH, "#334455");
    let engine = Engine::new();

    let handle = std::thread::Builder::new()
        .name("merman-core-too-deep-frontmatter-config".to_string())
        .stack_size(128 * 1024)
        .spawn(move || {
            let err = engine
                .parse_metadata_sync(&source, ParseOptions::strict())
                .expect_err("excessive frontmatter config depth should be rejected");
            assert!(
                err.to_string().contains("config nesting exceeds"),
                "unexpected error: {err}"
            );
        })
        .expect("spawn excessive frontmatter config test");
    handle
        .join()
        .expect("excessive frontmatter config should return an error without stack overflow");
}

#[test]
fn frontmatter_rejects_excessive_inline_yaml_sequence_nesting_with_small_stack() {
    const DEPTH: usize = 300;
    let mut source = String::from("---\nconfig:\n  ");
    source.push_str(&"- ".repeat(DEPTH));
    source.push_str("\"leaf\"\n---\nsequenceDiagram\nAlice->Bob: Hi\n");
    let engine = Engine::new();

    let handle = std::thread::Builder::new()
        .name("merman-core-too-deep-inline-yaml-sequence".to_string())
        .stack_size(128 * 1024)
        .spawn(move || {
            let err = engine
                .parse_metadata_sync(&source, ParseOptions::strict())
                .expect_err("excessive inline YAML sequence depth should be rejected");
            assert!(
                err.to_string().contains("config nesting exceeds"),
                "unexpected error: {err}"
            );
        })
        .expect("spawn excessive inline YAML sequence test");
    handle
        .join()
        .expect("excessive inline YAML sequence should return an error without stack overflow");
}

#[test]
fn frontmatter_non_string_yaml_keys_are_ignored_like_legacy_conversion() {
    let engine = Engine::new();
    let res = block_on(engine.parse_metadata(
        "---\n? [non, string, key]\n: ignored\n---\nsequenceDiagram\nAlice->Bob: Hi\n",
        ParseOptions::strict(),
    ))
    .expect("non-string YAML keys should not fail frontmatter parsing")
    .expect("diagram detected");

    assert_eq!(res.diagram_type, "sequence");
    assert_eq!(res.config.as_value(), &json!({}));
}

#[test]
fn parse_returns_malformed_frontmatter_error_for_unclosed_frontmatter() {
    let engine = Engine::new();
    let err = block_on(engine.parse_metadata(
        "---\ntitle: a malformed YAML front-matter\n",
        ParseOptions::default(),
    ))
    .unwrap_err();
    assert!(err.to_string().contains("Malformed YAML front-matter"));
}

#[test]
fn parse_can_suppress_unknown_diagram_errors() {
    let engine = Engine::new();
    let res = block_on(engine.parse_metadata(
        "this is not a mermaid diagram definition",
        ParseOptions {
            suppress_errors: true,
        },
    ))
    .unwrap();
    assert!(res.is_none());
}

#[test]
fn parse_lenient_failures_use_error_diagram_across_engine_entrypoints() {
    let engine = Engine::new();
    let input = "flowchart TD\nA -->";
    let options = ParseOptions::lenient();

    let parsed = engine.parse_diagram_sync(input, options).unwrap().unwrap();
    assert_suppressed_error_diagram(&parsed);

    let parsed = engine
        .parse_diagram_with_type_sync("flowchart-v2", input, options)
        .unwrap()
        .unwrap();
    assert_suppressed_error_diagram(&parsed);

    let parsed = engine
        .parse_diagram_for_render_model_sync(input, options)
        .unwrap()
        .unwrap();
    assert_suppressed_error_render_diagram(&parsed);

    let parsed = engine
        .parse_diagram_for_render_model_with_type_sync("flowchart-v2", input, options)
        .unwrap()
        .unwrap();
    assert_suppressed_error_render_diagram(&parsed);
}

fn assert_suppressed_error_diagram(parsed: &ParsedDiagram) {
    assert_eq!(parsed.meta.diagram_type, "error");
    assert_eq!(parsed.model["type"], json!("error"));
}

fn assert_suppressed_error_render_diagram(parsed: &ParsedDiagramRender) {
    assert_eq!(parsed.meta.diagram_type, "error");
    match &parsed.model {
        RenderSemanticModel::Json(model) => assert_eq!(model["type"], json!("error")),
        other => panic!("suppressed parse failures must render through JSON, got {other:?}"),
    }
}

#[test]
fn render_semantic_model_kind_reports_canonical_names() {
    let sequence = render_model_for("sequenceDiagram\nAlice->>Bob: Hi");
    assert_eq!(sequence.kind(), "sequence");

    let flowchart = render_model_for("flowchart TD\nA-->B");
    assert_eq!(flowchart.kind(), "flowchart");

    let er = render_model_for("erDiagram\nCUSTOMER ||--o{ ORDER : places");
    assert_eq!(er.kind(), "er");

    let json_model = RenderSemanticModel::Json(json!({ "type": "custom" }));
    assert_eq!(json_model.kind(), "json");
}

#[test]
fn render_semantic_model_supports_diagram_type_aliases() {
    let sequence = render_model_for("sequenceDiagram\nAlice->>Bob: Hi");
    assert!(sequence.supports_diagram_type("sequence"));
    assert!(sequence.supports_diagram_type("zenuml"));
    assert!(!sequence.supports_diagram_type("flowchart-v2"));

    let flowchart = render_model_for("flowchart TD\nA-->B");
    assert!(flowchart.supports_diagram_type("flowchart-v2"));
    assert!(flowchart.supports_diagram_type("flowchart"));
    assert!(flowchart.supports_diagram_type("flowchart-elk"));
    assert!(!flowchart.supports_diagram_type("sequence"));

    let er = render_model_for("erDiagram\nCUSTOMER ||--o{ ORDER : places");
    assert!(er.supports_diagram_type("er"));
    assert!(er.supports_diagram_type("erDiagram"));
    assert!(!er.supports_diagram_type("classDiagram"));

    let json_model = RenderSemanticModel::Json(json!({ "type": "custom" }));
    assert!(json_model.supports_diagram_type("unknown-plugin"));
}

#[test]
fn render_parser_registry_drives_typed_alias_parse() {
    let engine = Engine::new();
    assert!(
        engine
            .render_diagram_registry()
            .get("flowchart-elk")
            .is_some()
    );

    let parsed = engine
        .parse_diagram_for_render_model_with_type_sync(
            "flowchart-elk",
            "flowchart-elk TD\nA-->B;",
            ParseOptions::strict(),
        )
        .unwrap()
        .unwrap();

    assert_eq!(parsed.meta.diagram_type, "flowchart-elk");
    assert_eq!(parsed.model.kind(), "flowchart");
    assert!(matches!(parsed.model, RenderSemanticModel::Flowchart(_)));
}

#[test]
fn render_parser_registry_falls_back_to_json_registry_for_custom_diagrams() {
    let mut engine = Engine::new();
    engine
        .diagram_registry_mut()
        .insert("customDiagram", custom_json_parser);

    let parsed = engine
        .parse_diagram_for_render_model_with_type_sync(
            "customDiagram",
            "customDiagram\npayload",
            ParseOptions::strict(),
        )
        .unwrap()
        .unwrap();

    assert_eq!(parsed.meta.diagram_type, "customDiagram");
    match parsed.model {
        RenderSemanticModel::Json(model) => assert_eq!(model["type"], json!("customDiagram")),
        other => panic!("custom render fallback should use JSON model, got {other:?}"),
    }
}

#[test]
fn render_parser_registry_rejects_builtin_json_fallback_without_typed_parser() {
    let mut engine = Engine::new();
    assert!(
        engine
            .render_diagram_registry_mut()
            .remove("flowchart-v2")
            .is_some()
    );

    let err = engine
        .parse_diagram_for_render_model_with_type_sync(
            "flowchart-v2",
            "flowchart TD\nA-->B",
            ParseOptions::strict(),
        )
        .unwrap_err();
    let message = err.to_string();

    assert!(message.contains("missing a typed render parser"));
    assert!(message.contains("JSON render fallback is reserved"));
}

fn custom_json_parser(_code: &str, _meta: &ParseMetadata) -> Result<serde_json::Value> {
    Ok(json!({ "type": "customDiagram" }))
}

fn render_model_for(input: &str) -> RenderSemanticModel {
    Engine::new()
        .parse_diagram_for_render_model_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap()
        .model
}

fn deep_config_value(root_key: &str, depth: usize, leaf: Value) -> Value {
    let mut value = leaf;
    for idx in (0..depth).rev() {
        let mut map = Map::new();
        map.insert(format!("k{idx}"), value);
        value = Value::Object(map);
    }

    let mut root = Map::new();
    root.insert(root_key.to_string(), value);
    Value::Object(root)
}

fn deep_config_leaf<'a>(mut value: &'a Value, root_key: &str, depth: usize) -> Option<&'a Value> {
    value = value.as_object()?.get(root_key)?;
    for idx in 0..depth {
        value = value.as_object()?.get(&format!("k{idx}"))?;
    }
    Some(value)
}

fn deep_init_directive_source(root_key: &str, depth: usize, leaf: &str) -> String {
    let mut source = format!(r#"%%{{init: {{"{root_key}": "#);
    for idx in 0..depth {
        write!(&mut source, r#"{{"k{idx}":"#).expect("write init config");
    }
    write!(&mut source, "{leaf:?}").expect("write init leaf");
    for _ in 0..depth {
        source.push('}');
    }
    source.push_str("}}%%\nsequenceDiagram\nAlice->Bob: Hi\n");
    source
}

fn deep_frontmatter_config_source(root_key: &str, depth: usize, leaf: &str) -> String {
    let mut source = format!("---\nconfig: {{\"{root_key}\": ");
    for idx in 0..depth {
        write!(&mut source, r#"{{"k{idx}":"#).expect("write frontmatter config");
    }
    write!(&mut source, "{leaf:?}").expect("write frontmatter leaf");
    for _ in 0..depth {
        source.push('}');
    }
    source.push_str("}\n---\nsequenceDiagram\nAlice->Bob: Hi\n");
    source
}

#[test]
fn parse_flowchart_json_and_typed_render_model_share_semantic_source() {
    let engine = Engine::new();
    let input = r#"%%{init: {"securityLevel":"strict","flowchart":{"inheritDir":true}}}%%
flowchart LR
accTitle: Flowchart <b>access title</b>
accDescr { Flowchart <b>access description</b>
    second line
}
subgraph Cluster["Cluster <b>One</b>"]
  A["<b>Start</b>"]:::hot e1@-->|"<i>Edge</i>"| B@{ shape: rounded, label: "End" }
end
classDef hot fill:#fff,stroke:#333
class B hot
style Cluster stroke:#f66
linkStyle 0 stroke:#111,stroke-width:2px
click A href "https://example.test" "tip <b>safe</b>" _blank
"#;

    let typed = engine
        .parse_diagram_for_render_model_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();
    let model = match typed.model {
        RenderSemanticModel::Flowchart(model) => model,
        other => panic!("flowchart render parse should return typed model, got {other:?}"),
    };
    let typed_json = serde_json::to_value(&model).unwrap();

    let parsed_json = engine
        .parse_diagram_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap()
        .model;

    for key in [
        "accTitle",
        "accDescr",
        "classDefs",
        "direction",
        "edgeDefaults",
        "vertexCalls",
        "tooltips",
    ] {
        assert_eq!(typed_json[key], parsed_json[key], "shared key {key}");
    }

    let typed_nodes = typed_json["nodes"].as_array().unwrap();
    let json_nodes = parsed_json["nodes"].as_array().unwrap();
    assert_eq!(typed_nodes.len(), json_nodes.len());
    for (typed_node, json_node) in typed_nodes.iter().zip(json_nodes) {
        for key in [
            "id",
            "label",
            "labelType",
            "layoutShape",
            "icon",
            "form",
            "pos",
            "img",
            "constraint",
            "assetWidth",
            "assetHeight",
            "styles",
            "classes",
            "link",
            "linkTarget",
            "haveCallback",
        ] {
            assert_eq!(typed_node[key], json_node[key], "node key {key}");
        }
    }

    let typed_edges = typed_json["edges"].as_array().unwrap();
    let json_edges = parsed_json["edges"].as_array().unwrap();
    assert_eq!(typed_edges.len(), json_edges.len());
    for (typed_edge, json_edge) in typed_edges.iter().zip(json_edges) {
        for key in [
            "id",
            "from",
            "to",
            "label",
            "labelType",
            "type",
            "stroke",
            "length",
            "style",
            "classes",
            "interpolate",
            "animate",
            "animation",
        ] {
            assert_eq!(typed_edge[key], json_edge[key], "edge key {key}");
        }
    }

    let typed_subgraphs = typed_json["subgraphs"].as_array().unwrap();
    let json_subgraphs = parsed_json["subgraphs"].as_array().unwrap();
    assert_eq!(typed_subgraphs.len(), json_subgraphs.len());
    for (typed_subgraph, json_subgraph) in typed_subgraphs.iter().zip(json_subgraphs) {
        for key in [
            "id",
            "nodes",
            "title",
            "classes",
            "styles",
            "dir",
            "labelType",
        ] {
            assert_eq!(
                typed_subgraph[key], json_subgraph[key],
                "subgraph key {key}"
            );
        }
    }
}

#[test]
fn parse_sequence_render_model_uses_typed_variant_without_changing_json_parse() {
    let engine = Engine::new();
    let input = r#"sequenceDiagram
participant Alice as Alice A.
participant Bob
box aqua Core
participant Carol
end
Alice->>+Bob: Hi
Note right of Bob: Seen
create participant Dana
Bob-->>Dana: Spawn
destroy Dana
Dana-->>Bob: Done
"#;

    let parsed = engine
        .parse_diagram_for_render_model_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();

    assert_eq!(parsed.meta.diagram_type, "sequence");
    let typed_json = match &parsed.model {
        RenderSemanticModel::Sequence(model) => {
            assert_eq!(model.actor_order, ["Alice", "Bob", "Carol", "Dana"]);
            assert_eq!(model.messages[0].message_text(), "Hi");
            assert_eq!(model.notes[0].message, "Seen");
            assert_eq!(model.boxes[0].actor_keys, ["Carol"]);
            assert_eq!(model.created_actors["Dana"], 3);
            assert_eq!(model.destroyed_actors["Dana"], 4);
            model.to_compat_json(&parsed.meta.diagram_type)
        }
        other => panic!("sequence render parse should return typed model, got {other:?}"),
    };

    let parsed_json = engine
        .parse_diagram_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();
    assert_eq!(parsed_json.model["type"], json!("sequence"));
    assert_eq!(parsed_json.model["messages"][0]["message"], json!("Hi"));
    assert!(parsed_json.model["messages"][0].get("placement").is_none());
    assert!(
        parsed_json.model["messages"][0]
            .get("centralConnection")
            .is_none()
    );
    assert_eq!(typed_json, parsed_json.model);
}

#[test]
fn parse_kanban_render_model_uses_typed_variant_without_changing_json_parse() {
    let engine = Engine::new();
    let input = "kanban\n  Todo\n    item1\n  Doing\n    item2";

    let parsed = engine
        .parse_diagram_for_render_model_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();

    assert_eq!(parsed.meta.diagram_type, "kanban");
    match parsed.model {
        RenderSemanticModel::Kanban(model) => {
            assert_eq!(model.nodes.len(), 4);
            assert!(model.nodes[0].is_group);
            assert_eq!(model.nodes[0].label, "Todo");
            assert_eq!(model.nodes[1].label, "item1");
            assert_eq!(model.nodes[1].parent_id.as_deref(), Some("Todo"));
        }
        other => panic!("kanban render parse should return typed model, got {other:?}"),
    }

    let parsed_json = engine
        .parse_diagram_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();
    assert_eq!(parsed_json.model["type"], json!("kanban"));
    assert_eq!(parsed_json.model["nodes"][0]["label"], json!("Todo"));
    assert_eq!(parsed_json.model["nodes"][1]["label"], json!("item1"));
}

#[test]
fn parse_gantt_render_model_uses_typed_variant_without_changing_json_parse() {
    let engine = Engine::new();
    let input = r#"
gantt
dateFormat YYYY-MM-DD
title Typed Gantt
section Alpha
Task 1: id1, 2024-01-01, 2d
"#;

    let parsed = engine
        .parse_diagram_for_render_model_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();

    assert_eq!(parsed.meta.diagram_type, "gantt");
    match parsed.model {
        RenderSemanticModel::Gantt(model) => {
            assert_eq!(model.title.as_deref(), Some("Typed Gantt"));
            assert_eq!(model.date_format, "YYYY-MM-DD");
            assert_eq!(model.tasks.len(), 1);
            assert_eq!(model.tasks[0].id, "id1");
            assert_eq!(model.tasks[0].task, "Task 1");
            assert_eq!(model.tasks[0].section, "Alpha");
        }
        other => panic!("gantt render parse should return typed model, got {other:?}"),
    }

    let parsed_json = engine
        .parse_diagram_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();
    assert_eq!(parsed_json.model["type"], json!("gantt"));
    assert_eq!(parsed_json.model["title"], json!("Typed Gantt"));
    assert_eq!(parsed_json.model["tasks"][0]["id"], json!("id1"));
    assert_eq!(parsed_json.model["tasks"][0]["task"], json!("Task 1"));
}

#[test]
fn parse_pie_render_model_uses_typed_variant_without_changing_json_parse() {
    let engine = Engine::new();
    let input = r#"
pie showData title Typed Pie
  "Alpha" : 60
  "Beta" : 40
"#;

    let parsed = engine
        .parse_diagram_for_render_model_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();

    assert_eq!(parsed.meta.diagram_type, "pie");
    match parsed.model {
        RenderSemanticModel::Pie(model) => {
            assert!(model.show_data);
            assert_eq!(model.title.as_deref(), Some("Typed Pie"));
            assert_eq!(model.sections.len(), 2);
            assert_eq!(model.sections[0].label, "Alpha");
            assert_eq!(model.sections[0].value, 60.0);
        }
        other => panic!("pie render parse should return typed model, got {other:?}"),
    }

    let parsed_json = engine
        .parse_diagram_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();
    assert_eq!(parsed_json.model["type"], json!("pie"));
    assert_eq!(parsed_json.model["showData"], json!(true));
    assert_eq!(parsed_json.model["title"], json!("Typed Pie"));
    assert_eq!(parsed_json.model["sections"][0]["label"], json!("Alpha"));
    assert_eq!(parsed_json.model["sections"][0]["value"], json!(60.0));
}

#[test]
fn parse_xychart_render_model_uses_typed_variant_without_changing_json_parse() {
    let engine = Engine::new();
    let input = r#"
xychart horizontal
title "Typed XYChart"
accTitle: XY accTitle
accDescr: XY accDescription
x-axis "X Axis" [Alpha, Beta]
y-axis "Y Axis" 1 --> 5
bar "Series 1" [1, 2]
line "Series 2" [2, 3]
"#;

    let parsed = engine
        .parse_diagram_for_render_model_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();

    assert_eq!(parsed.meta.diagram_type, "xychart");
    let typed_json = match &parsed.model {
        RenderSemanticModel::XyChart(model) => {
            assert_eq!(model.orientation, "horizontal");
            assert_eq!(model.title.as_deref(), Some("Typed XYChart"));
            assert_eq!(model.acc_title.as_deref(), Some("XY accTitle"));
            assert_eq!(model.acc_descr.as_deref(), Some("XY accDescription"));
            assert_eq!(
                model.x_axis,
                XyChartAxisRenderModel::Band {
                    title: "X Axis".to_string(),
                    categories: vec!["Alpha".to_string(), "Beta".to_string()],
                }
            );
            assert_eq!(
                model.y_axis,
                XyChartAxisRenderModel::Linear {
                    title: "Y Axis".to_string(),
                    min: Some(1.0),
                    max: Some(5.0),
                }
            );
            assert_eq!(model.plots.len(), 2);
            assert_eq!(model.plots[0].plot_type, XyChartPlotType::Bar);
            assert_eq!(model.plots[0].values, vec![1.0, 2.0]);
            assert_eq!(
                model.plots[0].data,
                vec![
                    ("Alpha".to_string(), Some(1.0)),
                    ("Beta".to_string(), Some(2.0)),
                ]
            );
            assert_eq!(model.plots[1].plot_type, XyChartPlotType::Line);
            model.to_compat_json(&parsed.meta)
        }
        other => panic!("xychart render parse should return typed model, got {other:?}"),
    };

    let parsed_json = engine
        .parse_diagram_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();
    assert_eq!(parsed_json.model["type"], json!("xychart"));
    assert_eq!(parsed_json.model["title"], json!("Typed XYChart"));
    assert_eq!(parsed_json.model["xAxis"]["type"], json!("band"));
    assert_eq!(
        parsed_json.model["xAxis"]["categories"],
        json!(["Alpha", "Beta"])
    );
    assert_eq!(parsed_json.model["yAxis"]["type"], json!("linear"));
    assert_eq!(parsed_json.model["yAxis"]["min"], json!(1.0));
    assert_eq!(parsed_json.model["yAxis"]["max"], json!(5.0));
    assert_eq!(parsed_json.model["plots"][0]["type"], json!("bar"));
    assert_eq!(parsed_json.model["plots"][1]["type"], json!("line"));
    assert!(parsed_json.model.get("config").is_some());
    assert_eq!(typed_json, parsed_json.model);
}

#[test]
fn parse_xychart_exposes_11_15_data_label_outside_default_and_override() {
    let engine = Engine::new();
    let default = block_on(engine.parse_metadata("xychart\nbar [1]", ParseOptions::default()))
        .unwrap()
        .unwrap();
    let xychart = &default.effective_config.as_value()["xyChart"];
    assert_eq!(xychart["showDataLabel"], json!(false));
    assert_eq!(xychart["showDataLabelOutsideBar"], json!(false));

    let configured = block_on(engine.parse_metadata(
        r#"%%{init: {"xyChart": {"showDataLabel": true, "showDataLabelOutsideBar": true}}}%%
xychart
bar [1]"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let xychart = &configured.effective_config.as_value()["xyChart"];
    assert_eq!(xychart["showDataLabel"], json!(true));
    assert_eq!(xychart["showDataLabelOutsideBar"], json!(true));
}

#[test]
fn parse_class_exposes_11_15_hierarchical_namespaces_default_and_override() {
    let engine = Engine::new();
    let default = block_on(engine.parse_metadata("classDiagram\nclass A", ParseOptions::default()))
        .unwrap()
        .unwrap();
    let class = &default.effective_config.as_value()["class"];
    assert_eq!(class["hierarchicalNamespaces"], json!(true));

    let configured = block_on(engine.parse_metadata(
        r#"%%{init: {"class": {"hierarchicalNamespaces": false}}}%%
classDiagram
class A"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let class = &configured.effective_config.as_value()["class"];
    assert_eq!(class["hierarchicalNamespaces"], json!(false));
}

#[test]
fn parse_packet_render_model_uses_typed_variant_without_changing_json_parse() {
    let engine = Engine::new();
    let input = r#"
packet
title Typed Packet
accTitle: Packet accTitle
accDescr: Packet accDescription
+8: "byte"
+16: "word"
"#;

    let parsed = engine
        .parse_diagram_for_render_model_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();

    assert_eq!(parsed.meta.diagram_type, "packet");
    match parsed.model {
        RenderSemanticModel::Packet(model) => {
            assert_eq!(model.title.as_deref(), Some("Typed Packet"));
            assert_eq!(model.acc_title.as_deref(), Some("Packet accTitle"));
            assert_eq!(model.acc_descr.as_deref(), Some("Packet accDescription"));
            assert_eq!(model.packet.len(), 1);
            assert_eq!(model.packet[0].len(), 2);
            assert_eq!(model.packet[0][0].start, 0);
            assert_eq!(model.packet[0][0].end, 7);
            assert_eq!(model.packet[0][0].bits, 8);
            assert_eq!(model.packet[0][0].label, "byte");
        }
        other => panic!("packet render parse should return typed model, got {other:?}"),
    }

    let parsed_json = engine
        .parse_diagram_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();
    assert_eq!(parsed_json.model["type"], json!("packet"));
    assert_eq!(parsed_json.model["title"], json!("Typed Packet"));
    assert_eq!(parsed_json.model["accTitle"], json!("Packet accTitle"));
    assert_eq!(
        parsed_json.model["accDescr"],
        json!("Packet accDescription")
    );
    assert_eq!(parsed_json.model["packet"][0][0]["label"], json!("byte"));
    assert_eq!(parsed_json.model["packet"][0][0]["bits"], json!(8));
}

#[test]
fn parse_timeline_render_model_uses_typed_variant_without_changing_json_parse() {
    let engine = Engine::new();
    let input = r#"
timeline
title Typed Timeline
accTitle: Timeline accTitle
accDescr: Timeline accDescription
section Alpha
Task 1: event 1: event 2
"#;

    let parsed = engine
        .parse_diagram_for_render_model_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();

    assert_eq!(parsed.meta.diagram_type, "timeline");
    match parsed.model {
        RenderSemanticModel::Timeline(model) => {
            assert_eq!(model.title.as_deref(), Some("Typed Timeline"));
            assert_eq!(model.acc_title.as_deref(), Some("Timeline accTitle"));
            assert_eq!(model.acc_descr.as_deref(), Some("Timeline accDescription"));
            assert_eq!(model.sections.as_slice(), ["Alpha"]);
            assert_eq!(model.tasks.len(), 1);
            assert_eq!(model.tasks[0].id, 0);
            assert_eq!(model.tasks[0].section, "Alpha");
            assert_eq!(model.tasks[0].task_type, "Alpha");
            assert_eq!(model.tasks[0].task, "Task 1");
            assert_eq!(model.tasks[0].events.as_slice(), ["event 1", "event 2"]);
        }
        other => panic!("timeline render parse should return typed model, got {other:?}"),
    }

    let parsed_json = engine
        .parse_diagram_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();
    assert_eq!(parsed_json.model["type"], json!("timeline"));
    assert_eq!(parsed_json.model["title"], json!("Typed Timeline"));
    assert_eq!(parsed_json.model["sections"][0], json!("Alpha"));
    assert_eq!(parsed_json.model["tasks"][0]["id"], json!(0));
    assert_eq!(parsed_json.model["tasks"][0]["type"], json!("Alpha"));
    assert_eq!(parsed_json.model["tasks"][0]["task"], json!("Task 1"));
    assert_eq!(
        parsed_json.model["tasks"][0]["events"],
        json!(["event 1", "event 2"])
    );
}

#[test]
fn parse_journey_render_model_uses_typed_variant_without_changing_json_parse() {
    let engine = Engine::new();
    let input = r#"
journey
title Typed Journey
accTitle: Journey accTitle
accDescr: Journey accDescription
section Shopping
Get keys: 5: Dad
Drive: bad-score: Dad, Mum
"#;

    let parsed = engine
        .parse_diagram_for_render_model_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();

    assert_eq!(parsed.meta.diagram_type, "journey");
    match parsed.model {
        RenderSemanticModel::Journey(model) => {
            assert_eq!(model.title.as_deref(), Some("Typed Journey"));
            assert_eq!(model.acc_title.as_deref(), Some("Journey accTitle"));
            assert_eq!(model.acc_descr.as_deref(), Some("Journey accDescription"));
            assert_eq!(model.sections.as_slice(), ["Shopping"]);
            assert_eq!(model.actors.as_slice(), ["Dad", "Mum"]);
            assert_eq!(model.tasks.len(), 2);
            assert_eq!(model.tasks[0].score, 5);
            assert!(!model.tasks[0].score_is_nan);
            assert_eq!(model.tasks[0].people.as_slice(), ["Dad"]);
            assert_eq!(model.tasks[1].task, "Drive");
            assert!(model.tasks[1].score_is_nan);
            assert_eq!(model.tasks[1].people.as_slice(), ["Dad", "Mum"]);
        }
        other => panic!("journey render parse should return typed model, got {other:?}"),
    }

    let parsed_json = engine
        .parse_diagram_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();
    assert_eq!(parsed_json.model["type"], json!("journey"));
    assert_eq!(parsed_json.model["title"], json!("Typed Journey"));
    assert_eq!(parsed_json.model["actors"], json!(["Dad", "Mum"]));
    assert_eq!(parsed_json.model["tasks"][0]["score"], json!(5));
    assert!(parsed_json.model["tasks"][0].get("scoreIsNaN").is_none());
    assert_eq!(parsed_json.model["tasks"][1]["scoreIsNaN"], json!(true));
    assert_eq!(
        parsed_json.model["tasks"][1]["people"],
        json!(["Dad", "Mum"])
    );
}

#[test]
fn parse_requirement_render_model_uses_typed_variant_without_changing_json_parse() {
    let engine = Engine::new();
    let input = r##"
requirementDiagram
accTitle: Requirement accTitle
accDescr: Requirement accDescription
direction LR
requirement req_login:::critical {
  id: REQ-1
  text: "Login must work"
  risk: high
  verifymethod: test
}
element api {
  type: service
  docRef: docs/api.md
}
class api external
classDef critical fill:#f9f,stroke:#333,color:#111
classDef external stroke:#0f0
req_login - verifies -> api
"##;

    let parsed = engine
        .parse_diagram_for_render_model_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();

    assert_eq!(parsed.meta.diagram_type, "requirement");
    match parsed.model {
        RenderSemanticModel::Requirement(model) => {
            assert_eq!(model.acc_title.as_deref(), Some("Requirement accTitle"));
            assert_eq!(
                model.acc_descr.as_deref(),
                Some("Requirement accDescription")
            );
            assert_eq!(model.direction, "LR");
            assert_eq!(model.requirements.len(), 1);
            assert_eq!(model.requirements[0].name, "req_login");
            assert_eq!(model.requirements[0].node_type, "Requirement");
            assert_eq!(model.requirements[0].requirement_id, "REQ-1");
            assert_eq!(model.requirements[0].text, "Login must work");
            assert_eq!(model.requirements[0].risk, "High");
            assert_eq!(model.requirements[0].verify_method, "Test");
            assert!(
                model.requirements[0]
                    .classes
                    .iter()
                    .any(|c| c == "critical")
            );
            assert!(
                model.requirements[0]
                    .css_styles
                    .iter()
                    .any(|s| s == "fill:#f9f")
            );
            assert_eq!(model.elements.len(), 1);
            assert_eq!(model.elements[0].name, "api");
            assert_eq!(model.elements[0].element_type, "service");
            assert_eq!(model.elements[0].doc_ref, "docs/api.md");
            assert!(model.elements[0].classes.iter().any(|c| c == "external"));
            assert_eq!(model.relationships.len(), 1);
            assert_eq!(model.relationships[0].rel_type, "verifies");
            assert_eq!(model.relationships[0].src, "req_login");
            assert_eq!(model.relationships[0].dst, "api");
            assert!(model.classes.contains_key("critical"));
            assert!(model.classes.contains_key("external"));
        }
        other => panic!("requirement render parse should return typed model, got {other:?}"),
    }

    let parsed_json = engine
        .parse_diagram_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();
    assert_eq!(parsed_json.model["type"], json!("requirement"));
    assert_eq!(parsed_json.model["accTitle"], json!("Requirement accTitle"));
    assert_eq!(
        parsed_json.model["accDescr"],
        json!("Requirement accDescription")
    );
    assert_eq!(parsed_json.model["direction"], json!("LR"));
    assert_eq!(
        parsed_json.model["requirements"][0]["name"],
        json!("req_login")
    );
    assert_eq!(
        parsed_json.model["requirements"][0]["requirementId"],
        json!("REQ-1")
    );
    assert_eq!(
        parsed_json.model["requirements"][0]["verifyMethod"],
        json!("Test")
    );
    assert_eq!(
        parsed_json.model["elements"][0]["docRef"],
        json!("docs/api.md")
    );
    assert_eq!(
        parsed_json.model["relationships"][0]["type"],
        json!("verifies")
    );
    assert!(parsed_json.model.get("config").is_some());
}

#[test]
fn parse_sankey_render_model_uses_typed_variant_without_changing_json_parse() {
    let engine = Engine::new();
    let input = r#"
sankey-beta
Source,Target,10
Target,Done,2.5
"#;

    let parsed = engine
        .parse_diagram_for_render_model_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();

    assert_eq!(parsed.meta.diagram_type, "sankey");
    match parsed.model {
        RenderSemanticModel::Sankey(model) => {
            assert_eq!(model.graph.nodes.len(), 3);
            assert_eq!(model.graph.nodes[0].id, "Source");
            assert_eq!(model.graph.nodes[1].id, "Target");
            assert_eq!(model.graph.nodes[2].id, "Done");
            assert_eq!(model.graph.links.len(), 2);
            assert_eq!(model.graph.links[0].source, "Source");
            assert_eq!(model.graph.links[0].target, "Target");
            assert_eq!(model.graph.links[0].value, json!(10));
            assert_eq!(model.graph.links[1].source, "Target");
            assert_eq!(model.graph.links[1].target, "Done");
            assert_eq!(model.graph.links[1].value, json!(2.5));
        }
        other => panic!("sankey render parse should return typed model, got {other:?}"),
    }

    let parsed_json = engine
        .parse_diagram_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();
    assert_eq!(parsed_json.model["type"], json!("sankey"));
    assert_eq!(
        parsed_json.model["graph"]["nodes"][0]["id"],
        json!("Source")
    );
    assert_eq!(
        parsed_json.model["graph"]["links"][0],
        json!({
            "source": "Source",
            "target": "Target",
            "value": 10,
        })
    );
    assert_eq!(parsed_json.model["graph"]["links"][1]["value"], json!(2.5));
    assert!(parsed_json.model.get("config").is_some());
}

#[test]
fn parse_sankey_exposes_11_15_config_defaults_and_overrides() {
    let engine = Engine::new();
    let default = block_on(engine.parse_metadata("sankey\nA,B,1", ParseOptions::default()))
        .unwrap()
        .unwrap();
    let sankey = &default.effective_config.as_value()["sankey"];
    assert_eq!(sankey["nodeWidth"], json!(10));
    assert_eq!(sankey["nodePadding"], json!(12));
    assert_eq!(sankey["labelStyle"], json!("legacy"));
    assert_eq!(sankey["nodeColors"], json!({}));

    let configured = block_on(engine.parse_metadata(
        r##"%%{init: {"sankey": {"nodeWidth": 24, "nodePadding": 18, "labelStyle": "outlined", "nodeColors": {"A": "#112233"}}}}%%
sankey
A,B,1"##,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let sankey = &configured.effective_config.as_value()["sankey"];
    assert_eq!(sankey["nodeWidth"], json!(24));
    assert_eq!(sankey["nodePadding"], json!(18));
    assert_eq!(sankey["labelStyle"], json!("outlined"));
    assert_eq!(sankey["nodeColors"]["A"], json!("#112233"));
}

#[test]
fn parse_radar_render_model_uses_typed_variant_without_changing_json_parse() {
    let engine = Engine::new();
    let input = r#"
radar-beta
title Typed Radar
accTitle: Radar accTitle
accDescr: Radar accDescription
axis A["Axis A"], B["Axis B"], C["Axis C"]
curve first["First Curve"]{1,2,3}
curve second{ C: 9, A: 7, B: 8 }
showLegend false
ticks 4
min 1
max 10
graticule polygon
"#;

    let parsed = engine
        .parse_diagram_for_render_model_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();

    assert_eq!(parsed.meta.diagram_type, "radar");
    match parsed.model {
        RenderSemanticModel::Radar(model) => {
            assert_eq!(model.title.as_deref(), Some("Typed Radar"));
            assert_eq!(model.acc_title.as_deref(), Some("Radar accTitle"));
            assert_eq!(model.acc_descr.as_deref(), Some("Radar accDescription"));
            assert_eq!(model.axes.len(), 3);
            assert_eq!(model.axes[0].name, "A");
            assert_eq!(model.axes[0].label, "Axis A");
            assert_eq!(model.curves.len(), 2);
            assert_eq!(model.curves[0].label, "First Curve");
            assert_eq!(model.curves[0].entries, vec![json!(1), json!(2), json!(3)]);
            assert_eq!(model.curves[1].entries, vec![json!(7), json!(8), json!(9)]);
            assert!(!model.options.show_legend);
            assert_eq!(model.options.ticks, json!(4));
            assert_eq!(model.options.min, json!(1));
            assert_eq!(model.options.max, Some(json!(10)));
            assert_eq!(model.options.graticule, "polygon");
        }
        other => panic!("radar render parse should return typed model, got {other:?}"),
    }

    let parsed_json = engine
        .parse_diagram_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();
    assert_eq!(parsed_json.model["type"], json!("radar"));
    assert_eq!(parsed_json.model["title"], json!("Typed Radar"));
    assert_eq!(parsed_json.model["accTitle"], json!("Radar accTitle"));
    assert_eq!(
        parsed_json.model["axes"][0],
        json!({"name": "A", "label": "Axis A"})
    );
    assert_eq!(
        parsed_json.model["curves"][1],
        json!({"name": "second", "label": "second", "entries": [7, 8, 9]})
    );
    assert_eq!(
        parsed_json.model["options"],
        json!({
            "showLegend": false,
            "ticks": 4,
            "max": 10,
            "min": 1,
            "graticule": "polygon",
        })
    );
    assert!(parsed_json.model.get("config").is_some());
}

#[test]
fn parse_info_render_model_uses_typed_variant_without_changing_json_parse() {
    let engine = Engine::new();
    let input = r#"
info
showInfo
"#;

    let parsed = engine
        .parse_diagram_for_render_model_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();

    assert_eq!(parsed.meta.diagram_type, "info");
    match parsed.model {
        RenderSemanticModel::Info(model) => {
            assert!(model.show_info);
        }
        other => panic!("info render parse should return typed model, got {other:?}"),
    }

    let parsed_json = engine
        .parse_diagram_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();
    assert_eq!(parsed_json.model["type"], json!("info"));
    assert_eq!(parsed_json.model["showInfo"], json!(true));
}

#[test]
fn parse_er_render_model_uses_typed_variant_without_changing_json_parse() {
    let engine = Engine::new();
    let input = r#"
erDiagram
accTitle: ER accTitle
accDescr: ER accDescription
CUSTOMER ||--o{ ORDER : places
CUSTOMER {
  string id
}
"#;

    let parsed = engine
        .parse_diagram_for_render_model_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();

    assert_eq!(parsed.meta.diagram_type, "er");
    match parsed.model {
        RenderSemanticModel::Er(model) => {
            assert_eq!(model.acc_title.as_deref(), Some("ER accTitle"));
            assert_eq!(model.acc_descr.as_deref(), Some("ER accDescription"));
            assert_eq!(model.direction, "TB");
            assert!(model.entities.contains_key("CUSTOMER"));
            assert!(model.entities.contains_key("ORDER"));
            assert_eq!(model.relationships.len(), 1);
            assert_eq!(model.relationships[0].entity_a, "entity-CUSTOMER-0");
            assert_eq!(model.relationships[0].entity_b, "entity-ORDER-1");
            assert_eq!(model.relationships[0].role_a, "places");
            assert_eq!(model.relationships[0].rel_spec.card_a, "ZERO_OR_MORE");
            assert_eq!(model.relationships[0].rel_spec.card_b, "ONLY_ONE");
            assert_eq!(model.relationships[0].rel_spec.rel_type, "IDENTIFYING");
        }
        other => panic!("er render parse should return typed model, got {other:?}"),
    }

    let parsed_json = engine
        .parse_diagram_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();
    assert_eq!(parsed_json.model["type"], json!("er"));
    assert_eq!(parsed_json.model["accTitle"], json!("ER accTitle"));
    assert_eq!(
        parsed_json.model["relationships"][0]["roleA"],
        json!("places")
    );
    assert!(parsed_json.model.get("constants").is_some());
}

#[test]
fn parse_sanitizes_common_db_fields_in_strict_mode() {
    let engine = Engine::new();
    let text = r#"sequenceDiagram
title: <script>alert(1)</script><b>t</b>
accTitle: <script>alert(1)</script><b>a</b>
accDescr: <script>alert(1)</script><b>d</b>
Alice->Bob:Hello"#;

    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();

    assert_eq!(res.model["title"], json!("<b>t</b>"));
    assert_eq!(res.model["accTitle"], json!("<b>a</b>"));
    assert_eq!(res.model["accDescr"], json!("<b>d</b>"));
}

#[test]
fn parse_render_model_sanitizes_common_db_fields_for_typed_families() {
    let engine = Engine::new();
    let flowchart = r#"flowchart TD
accTitle: <script>alert(1)</script><b>a</b>
accDescr { <script>alert(1)</script>line1
    line2
}
A[Start] --> B[End]
"#;

    let parsed = engine
        .parse_diagram_for_render_model_sync(flowchart, ParseOptions::strict())
        .unwrap()
        .unwrap();

    match parsed.model {
        RenderSemanticModel::Flowchart(model) => {
            assert_eq!(model.acc_title.as_deref(), Some("<b>a</b>"));
            assert_eq!(model.acc_descr.as_deref(), Some("line1\nline2"));
        }
        other => panic!("flowchart render parse should return typed model, got {other:?}"),
    }

    let sequence = r#"sequenceDiagram
title: <script>alert(1)</script><b>t</b>
accTitle: <script>alert(1)</script><b>a</b>
accDescr { <script>alert(1)</script>line1
    line2
}
Alice->Bob:Hello"#;

    let parsed = engine
        .parse_diagram_for_render_model_sync(sequence, ParseOptions::strict())
        .unwrap()
        .unwrap();

    match parsed.model {
        RenderSemanticModel::Sequence(model) => {
            assert_eq!(model.title.as_deref(), Some("<b>t</b>"));
            assert_eq!(model.acc_title.as_deref(), Some("<b>a</b>"));
            assert_eq!(model.acc_descr.as_deref(), Some("line1\nline2"));
        }
        other => panic!("sequence render parse should return typed model, got {other:?}"),
    }

    let treemap = r#"treemap
title <script>alert(1)</script><b>t</b>
accTitle: <script>alert(1)</script><b>a</b>
accDescr: <script>alert(1)</script><b>d</b>
"Root": 1
"#;

    let parsed = engine
        .parse_diagram_for_render_model_sync(treemap, ParseOptions::strict())
        .unwrap()
        .unwrap();

    match parsed.model {
        RenderSemanticModel::Treemap(model) => {
            assert_eq!(model.title.as_deref(), Some("<b>t</b>"));
            assert_eq!(model.acc_title.as_deref(), Some("<b>a</b>"));
            assert_eq!(model.acc_descr.as_deref(), Some("<b>d</b>"));
        }
        other => panic!("treemap render parse should return typed model, got {other:?}"),
    }
}
