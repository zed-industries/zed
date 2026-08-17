use crate::*;
use futures::executor::block_on;
use serde_json::json;

#[test]
fn parse_diagram_state_v2_alias_and_colon_description() {
    let engine = Engine::new();

    let res = block_on(engine.parse_diagram(
        r#"stateDiagram-v2
state "Small State 1" as namedState1"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    assert_eq!(res.meta.diagram_type, "stateDiagram");
    assert_eq!(
        res.model["states"]["namedState1"]["descriptions"][0],
        json!("Small State 1")
    );

    let res = block_on(engine.parse_diagram(
        r#"stateDiagram-v2
namedState1 : Small State 1"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    assert_eq!(
        res.model["states"]["namedState1"]["descriptions"][0],
        json!("Small State 1")
    );

    let res = block_on(engine.parse_diagram(
        r#"stateDiagram-v2
namedState1:Small State 1"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    assert_eq!(
        res.model["states"]["namedState1"]["descriptions"][0],
        json!("Small State 1")
    );
}

#[test]
fn parse_diagram_state_v2_groups_and_unsafe_ids() {
    let engine = Engine::new();

    let res = block_on(engine.parse_diagram(
        r#"stateDiagram-v2
state "Small State 1" as namedState1
state "Big State 1" as bigState1 {
  bigState1InternalState
}
namedState1 --> bigState1: should point to \nBig State 1 container

state "Small State 2" as namedState2
state bigState2 {
  bigState2InternalState
}
namedState2 --> bigState2: should point to \nbigState2 container"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();

    assert_eq!(
        res.model["states"]["bigState1"]["doc"][0]["id"],
        json!("bigState1InternalState")
    );
    assert_eq!(
        res.model["states"]["bigState2"]["doc"][0]["id"],
        json!("bigState2InternalState")
    );
    assert_eq!(res.model["relations"][0]["id1"], json!("namedState1"));
    assert_eq!(res.model["relations"][0]["id2"], json!("bigState1"));
    assert_eq!(res.model["relations"][1]["id1"], json!("namedState2"));
    assert_eq!(res.model["relations"][1]["id2"], json!("bigState2"));

    let res = block_on(engine.parse_diagram(
        r#"stateDiagram-v2
[*] --> __proto__
__proto__ --> [*]"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    assert!(res.model["states"]["__proto__"].is_object());
    assert!(res.model["states"]["root_start"].is_object());
    assert!(res.model["states"]["root_end"].is_object());
}

#[test]
fn parse_diagram_state_v2_classdef_class_and_shorthand() {
    let engine = Engine::new();

    let res = block_on(engine.parse_diagram(
        r#"stateDiagram-v2
classDef exampleStyleClass background:#bbb,border:1.5px solid red;
a --> b:::exampleStyleClass
class a exampleStyleClass"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();

    assert_eq!(
        res.model["styleClasses"]["exampleStyleClass"]["styles"][0],
        json!("background:#bbb")
    );
    assert_eq!(
        res.model["styleClasses"]["exampleStyleClass"]["styles"][1],
        json!("border:1.5px solid red")
    );
    assert_eq!(
        res.model["states"]["a"]["classes"][0],
        json!("exampleStyleClass")
    );
    assert_eq!(
        res.model["states"]["b"]["classes"][0],
        json!("exampleStyleClass")
    );

    let res = block_on(engine.parse_diagram(
        r#"stateDiagram-v2
classDef exampleStyleClass background:#bbb,border:1px solid red;
[*]:::exampleStyleClass --> b"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    assert_eq!(
        res.model["states"]["root_start"]["classes"][0],
        json!("exampleStyleClass")
    );

    let res = block_on(engine.parse_diagram(
        r#"stateDiagram-v2
classDef exampleStyleClass background:#bbb,border:1px solid red;
a-->b
class a,b,c, d, e exampleStyleClass"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    for id in ["a", "b", "c", "d", "e"] {
        assert_eq!(
            res.model["states"][id]["classes"][0],
            json!("exampleStyleClass")
        );
    }
}

#[test]
fn parse_diagram_state_v2_style_statement_sets_node_styles_and_ignores_comments() {
    let engine = Engine::new();

    let res = block_on(engine.parse_diagram(
        r#"stateDiagram-v2
id1
id2
style id1,id2 background:#bbb, font-weight:bold, font-style:italic;"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();

    assert_eq!(
        res.model["nodes"][0]["cssStyles"],
        json!(["background:#bbb", "font-weight:bold", "font-style:italic"])
    );
    assert_eq!(
        res.model["nodes"][1]["cssStyles"],
        json!(["background:#bbb", "font-weight:bold", "font-style:italic"])
    );

    let res = block_on(engine.parse_diagram(
        r#"stateDiagram-v2
[*] --> Moving
Moving --> Still
Moving --> Crash
state Moving {
%% comment inside state
slow  --> fast
}"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();

    assert_eq!(
        res.model["states"]["Moving"]["doc"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn parse_diagram_state_v2_click_and_href_store_links() {
    let engine = Engine::new();

    let res = block_on(engine.parse_diagram(
        r#"stateDiagram-v2
S1
click S1 "https://example.com" "Go to Example""#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    assert_eq!(
        res.model["links"]["S1"]["url"],
        json!("https://example.com")
    );
    assert_eq!(res.model["links"]["S1"]["tooltip"], json!("Go to Example"));

    let res = block_on(engine.parse_diagram(
        r#"stateDiagram-v2
S2
click S2 href "https://example.com""#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    assert_eq!(
        res.model["links"]["S2"]["url"],
        json!("https://example.com")
    );
    assert_eq!(res.model["links"]["S2"]["tooltip"], json!(""));
}

#[test]
fn parse_diagram_state_v2_note_right_of_and_block_note() {
    let engine = Engine::new();

    let res = block_on(engine.parse_diagram(
        r#"stateDiagram-v2
[*] --> A
note right of A : This is a note"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    assert_eq!(
        res.model["states"]["A"]["note"]["position"],
        json!("right of")
    );
    assert_eq!(
        res.model["states"]["A"]["note"]["text"],
        json!("This is a note")
    );

    let res = block_on(engine.parse_diagram(
        r#"stateDiagram-v2
[*] --> A
note right of A
  line1
  line2
end note"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let note_text = res.model["states"]["A"]["note"]["text"].as_str().unwrap();
    assert_eq!(note_text, "line1\nline2");

    let res = block_on(engine.parse_diagram(
        r#"stateDiagram
foo: bar
note "This is a floating note" as N1"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    // Mermaid `@11.12.2` treats floating notes as a no-op in state diagrams.
    // (See upstream `stateDiagram floating notes` specs.)
    assert!(res.model["states"].get("N1").is_none());
}

#[test]
fn parse_diagram_state_v2_getdata_edges_and_note_edges() {
    let engine = Engine::new();

    let res = block_on(engine.parse_diagram(
        r#"stateDiagram-v2
A --> B: hello"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();

    assert_eq!(res.model["edges"][0]["start"], json!("A"));
    assert_eq!(res.model["edges"][0]["end"], json!("B"));
    assert_eq!(res.model["edges"][0]["label"], json!("hello"));
    assert_eq!(res.model["edges"][0]["arrowhead"], json!("normal"));

    let res = block_on(engine.parse_diagram(
        r#"stateDiagram-v2
[*] --> A
note left of A : note text"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();

    let note_edge = res.model["edges"]
        .as_array()
        .unwrap()
        .iter()
        .find(|e| e["arrowhead"] == json!("none"))
        .unwrap();
    assert_eq!(note_edge["classes"], json!("transition note-edge"));
}

#[test]
fn parse_diagram_state_v2_uses_neo_arrow_type_when_look_is_neo() {
    let engine = Engine::new();

    let res = block_on(engine.parse_diagram(
        r#"%%{init: {"look": "neo"}}%%
stateDiagram-v2
A --> B: hello"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();

    assert_eq!(
        res.model["edges"][0]["arrowTypeEnd"],
        json!("arrow_barb_neo")
    );
}

#[test]
fn parse_diagram_state_v2_sanitizes_edge_labels_like_mermaid_common() {
    let engine = Engine::new();
    let res = block_on(engine.parse_diagram(
        r#"stateDiagram-v2
A --> B: hello<script>alert(1)</script>world"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    assert_eq!(res.model["edges"][0]["label"], json!("helloworld"));
}

#[test]
fn parse_diagram_state_v2_getdata_dom_id_counter_and_note_padding_match_mermaid() {
    let engine = Engine::new();

    let res = block_on(engine.parse_diagram(
        r#"stateDiagram-v2
A --> B
note right of A : note text"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();

    let nodes = res.model["nodes"].as_array().unwrap();
    let node_a = nodes.iter().find(|n| n["id"] == json!("A")).unwrap();
    let node_b = nodes.iter().find(|n| n["id"] == json!("B")).unwrap();
    let note_group = nodes
        .iter()
        .find(|n| n["id"] == json!("A----parent"))
        .unwrap();
    let note_node = nodes
        .iter()
        .find(|n| n["id"] == json!("A----note-1"))
        .unwrap();

    assert_eq!(node_a["domId"], json!("state-A-1"));
    assert_eq!(node_b["domId"], json!("state-B-0"));
    assert_eq!(note_group["domId"], json!("state-A----parent-1"));
    assert_eq!(note_node["domId"], json!("state-A----note-1"));
    assert_eq!(note_group["padding"], json!(16));
    assert_eq!(note_node["padding"], json!(15));
    assert_eq!(note_node["parentId"], json!("A----parent"));
}

fn deep_state_composite_chain(depth: usize) -> String {
    let mut input = String::from("stateDiagram-v2\n");
    for level in 0..depth {
        input.push_str(&format!("state S{level} {{\n"));
    }
    input.push_str("Leaf\n");
    for _ in 0..depth {
        input.push_str("}\n");
    }
    input
}

#[test]
fn state_deep_composite_chain_semantic_and_render_model_use_heap_traversal() {
    const DEPTH: usize = 1200;
    let input = deep_state_composite_chain(DEPTH);
    let engine = Engine::new();

    let parsed = block_on(engine.parse_diagram(&input, ParseOptions::strict()))
        .expect("parse ok")
        .expect("diagram detected");
    assert_eq!(parsed.meta.diagram_type, "stateDiagram");
    assert!(parsed.model["states"]["S0"]["doc"].is_array());
    assert!(
        parsed.model["nodes"]
            .as_array()
            .expect("nodes array")
            .iter()
            .any(|node| node["id"] == json!("Leaf"))
    );

    let parsed = engine
        .parse_diagram_for_render_model_sync(&input, ParseOptions::strict())
        .expect("render model parse ok")
        .expect("diagram detected");
    assert_eq!(parsed.meta.diagram_type, "stateDiagram");
}
