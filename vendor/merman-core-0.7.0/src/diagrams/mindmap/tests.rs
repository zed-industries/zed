use super::*;
use crate::{Engine, ParseOptions, RenderSemanticModel};
use futures::executor::block_on;
use serde_json::Value;

fn parse(text: &str) -> Value {
    let engine = Engine::new();
    block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap()
        .model
}

fn deep_mindmap_chain(depth: usize) -> String {
    let mut input = String::from("mindmap\n");
    for level in 0..depth {
        input.push_str(&" ".repeat(level));
        input.push_str(&format!("n{level}\n"));
    }
    input
}

fn root_descr(model: &Value) -> &str {
    model["rootNode"]["descr"].as_str().unwrap()
}

#[test]
fn mindmap_simple_root() {
    let model = parse("mindmap\n    root");
    assert_eq!(root_descr(&model), "root");
}

#[test]
fn mindmap_simple_root_shaped_without_id() {
    let model = parse("mindmap\n    (root)");
    assert_eq!(root_descr(&model), "root");
    assert_eq!(model["rootNode"]["nodeId"].as_str().unwrap(), "root");
}

#[test]
fn mindmap_hierarchy_two_children() {
    let model = parse("mindmap\n    root\n      child1\n      child2\n");
    assert_eq!(root_descr(&model), "root");
    assert_eq!(model["rootNode"]["children"].as_array().unwrap().len(), 2);
    assert_eq!(
        model["rootNode"]["children"][0]["descr"].as_str().unwrap(),
        "child1"
    );
    assert_eq!(
        model["rootNode"]["children"][1]["descr"].as_str().unwrap(),
        "child2"
    );
}

#[test]
fn mindmap_deeper_hierarchy() {
    let model = parse("mindmap\n    root\n      child1\n        leaf1\n      child2");
    let mm = &model["rootNode"];
    assert_eq!(mm["descr"].as_str().unwrap(), "root");
    let children = mm["children"].as_array().unwrap();
    assert_eq!(children.len(), 2);
    assert_eq!(children[0]["descr"].as_str().unwrap(), "child1");
    assert_eq!(
        children[0]["children"][0]["descr"].as_str().unwrap(),
        "leaf1"
    );
    assert_eq!(children[1]["descr"].as_str().unwrap(), "child2");
}

#[test]
fn mindmap_multiple_roots_is_error() {
    let engine = Engine::new();
    let err =
        block_on(engine.parse_diagram("mindmap\n    root\n    fakeRoot", ParseOptions::default()))
            .unwrap_err();
    assert!(
        err.to_string()
            .contains("There can be only one root. No parent could be found for (\"fakeRoot\")")
    );
}

#[test]
fn mindmap_real_root_in_wrong_place_is_error() {
    let engine = Engine::new();
    let text = "mindmap\n          root\n        fakeRoot\n    realRootWrongPlace";
    let err = block_on(engine.parse_diagram(text, ParseOptions::default())).unwrap_err();
    assert!(
        err.to_string()
            .contains("There can be only one root. No parent could be found for (\"fakeRoot\")")
    );
}

#[test]
fn mindmap_node_id_and_label_and_type_rect() {
    let model = parse("mindmap\n    root[The root]\n");
    assert_eq!(model["rootNode"]["nodeId"].as_str().unwrap(), "root");
    assert_eq!(root_descr(&model), "The root");
    assert_eq!(
        model["rootNode"]["type"].as_i64().unwrap(),
        NODE_TYPE_RECT as i64
    );
}

#[test]
fn mindmap_child_node_id_and_type_rounded_rect() {
    let model = parse("mindmap\n    root\n      theId(child1)");
    let child = &model["rootNode"]["children"][0];
    assert_eq!(child["descr"].as_str().unwrap(), "child1");
    assert_eq!(child["nodeId"].as_str().unwrap(), "theId");
    assert_eq!(
        child["type"].as_i64().unwrap(),
        NODE_TYPE_ROUNDED_RECT as i64
    );
}

#[test]
fn mindmap_node_types_circle_cloud_bang_hexagon() {
    let circle = parse("mindmap\n root((the root))");
    assert_eq!(
        circle["rootNode"]["type"].as_i64().unwrap(),
        NODE_TYPE_CIRCLE as i64
    );
    assert_eq!(circle["rootNode"]["descr"].as_str().unwrap(), "the root");

    let cloud = parse("mindmap\n root)the root(");
    assert_eq!(
        cloud["rootNode"]["type"].as_i64().unwrap(),
        NODE_TYPE_CLOUD as i64
    );
    assert_eq!(cloud["rootNode"]["descr"].as_str().unwrap(), "the root");

    let bang = parse("mindmap\n root))the root((");
    assert_eq!(
        bang["rootNode"]["type"].as_i64().unwrap(),
        NODE_TYPE_BANG as i64
    );
    assert_eq!(bang["rootNode"]["descr"].as_str().unwrap(), "the root");

    let hex = parse("mindmap\n root{{the root}}");
    assert_eq!(
        hex["rootNode"]["type"].as_i64().unwrap(),
        NODE_TYPE_HEXAGON as i64
    );
    assert_eq!(hex["rootNode"]["descr"].as_str().unwrap(), "the root");
}

#[test]
fn mindmap_icon_and_class_decorations() {
    let model = parse("mindmap\n    root[The root]\n    :::m-4 p-8\n    ::icon(bomb)\n");
    assert_eq!(model["rootNode"]["class"].as_str().unwrap(), "m-4 p-8");
    assert_eq!(model["rootNode"]["icon"].as_str().unwrap(), "bomb");
}

#[test]
fn mindmap_can_set_icon_then_class_or_class_then_icon() {
    let model = parse("mindmap\n    root[The root]\n    :::m-4 p-8\n    ::icon(bomb)\n");
    assert_eq!(model["rootNode"]["class"].as_str().unwrap(), "m-4 p-8");
    assert_eq!(model["rootNode"]["icon"].as_str().unwrap(), "bomb");

    let model = parse("mindmap\n    root[The root]\n    ::icon(bomb)\n    :::m-4 p-8\n");
    assert_eq!(model["rootNode"]["class"].as_str().unwrap(), "m-4 p-8");
    assert_eq!(model["rootNode"]["icon"].as_str().unwrap(), "bomb");
}

#[test]
fn mindmap_quoted_descriptions_can_contain_delimiters() {
    let model = parse("mindmap\n    root[\"String containing []\"]");
    assert_eq!(model["rootNode"]["nodeId"].as_str().unwrap(), "root");
    assert_eq!(
        model["rootNode"]["descr"].as_str().unwrap(),
        "String containing []"
    );

    let model = parse(
        "mindmap\n    root[\"String containing []\"]\n      child1[\"String containing ()\"]",
    );
    assert_eq!(model["rootNode"]["children"].as_array().unwrap().len(), 1);
    assert_eq!(
        model["rootNode"]["children"][0]["descr"].as_str().unwrap(),
        "String containing ()"
    );
}

#[test]
fn mindmap_child_after_class_assignment_is_attached_to_last_node() {
    let model = parse(
        "mindmap\n  root(Root)\n    Child(Child)\n    :::hot\n      a(a)\n      b[New Stuff]",
    );
    let mm = &model["rootNode"];
    assert_eq!(mm["nodeId"].as_str().unwrap(), "root");
    let child = &mm["children"][0];
    assert_eq!(child["nodeId"].as_str().unwrap(), "Child");
    assert_eq!(child["children"].as_array().unwrap().len(), 2);
    assert_eq!(child["children"][0]["nodeId"].as_str().unwrap(), "a");
    assert_eq!(child["children"][1]["nodeId"].as_str().unwrap(), "b");
}

#[test]
fn mindmap_comment_end_of_line_is_ignored() {
    let model = parse(
        "mindmap\n  root(Root)\n    Child(Child)\n      a(a) %% This is a comment\n      b[New Stuff]\n",
    );
    let child = &model["rootNode"]["children"][0];
    assert_eq!(child["nodeId"].as_str().unwrap(), "Child");
    assert_eq!(child["children"].as_array().unwrap().len(), 2);
    assert_eq!(child["children"][1]["nodeId"].as_str().unwrap(), "b");
}

#[test]
fn mindmap_rows_above_declaration_are_ignored() {
    let model = parse("\n \n\nmindmap\nroot\n A\n \n\n B");
    assert_eq!(model["rootNode"]["nodeId"].as_str().unwrap(), "root");
    assert_eq!(model["rootNode"]["children"].as_array().unwrap().len(), 2);
}

#[test]
fn mindmap_leading_comment_lines_before_declaration_are_ignored() {
    let model = parse("%% comment\n\nmindmap\nroot\n A\n B");
    assert_eq!(model["rootNode"]["nodeId"].as_str().unwrap(), "root");
    assert_eq!(model["rootNode"]["children"].as_array().unwrap().len(), 2);
}

#[test]
fn mindmap_root_without_indent_child_with_indent() {
    let model = parse("mindmap\nroot\n      theId(child1)");
    let mm = &model["rootNode"];
    assert_eq!(mm["nodeId"].as_str().unwrap(), "root");
    assert_eq!(mm["children"].as_array().unwrap().len(), 1);
    let child = &mm["children"][0];
    assert_eq!(child["descr"].as_str().unwrap(), "child1");
    assert_eq!(child["nodeId"].as_str().unwrap(), "theId");
}

#[test]
fn mindmap_rows_with_only_spaces_do_not_interfere() {
    let model = parse("mindmap\nroot\n A\n \n\n B");
    let mm = &model["rootNode"];
    assert_eq!(mm["nodeId"].as_str().unwrap(), "root");
    assert_eq!(mm["children"].as_array().unwrap().len(), 2);
    assert_eq!(mm["children"][0]["nodeId"].as_str().unwrap(), "A");
    assert_eq!(mm["children"][1]["nodeId"].as_str().unwrap(), "B");
}

#[test]
fn mindmap_meaningless_empty_rows_do_not_interfere() {
    let model = parse("mindmap\n  root(Root)\n    Child(Child)\n      a(a)\n\n      b[New Stuff]");
    let mm = &model["rootNode"];
    assert_eq!(mm["nodeId"].as_str().unwrap(), "root");
    let child = &mm["children"][0];
    assert_eq!(child["nodeId"].as_str().unwrap(), "Child");
    assert_eq!(child["children"].as_array().unwrap().len(), 2);
    assert_eq!(child["children"][1]["nodeId"].as_str().unwrap(), "b");
}

#[test]
fn mindmap_header_can_share_line_with_root_node() {
    let model = parse("mindmap root\n  child1\n");
    let mm = &model["rootNode"];
    assert_eq!(mm["descr"].as_str().unwrap(), "root");
    assert_eq!(mm["children"].as_array().unwrap().len(), 1);
    assert_eq!(mm["children"][0]["descr"].as_str().unwrap(), "child1");
}

#[test]
fn mindmap_multiline_markdown_string_node_description_is_parsed() {
    let model = parse(
        "mindmap\n    id1[\"`**Root** with\n\
a second line\n\
Unicode works too: 🤓`\"]\n      id2[\"`The dog in **the** hog... a *very long text* that wraps to a new line`\"]\n      id3[Regular labels still works]\n",
    );
    let root = &model["rootNode"];
    assert_eq!(root["nodeId"].as_str().unwrap(), "id1");
    let descr = root["descr"].as_str().unwrap();
    assert!(descr.contains("Root"));
    assert!(descr.contains("a second line"));
    assert!(descr.contains("🤓"));
}

#[test]
fn mindmap_get_data_empty_when_no_nodes() {
    let model = parse("mindmap\n");
    assert_eq!(model["nodes"].as_array().unwrap().len(), 0);
    assert_eq!(model["edges"].as_array().unwrap().len(), 0);
    assert!(model.get("rootNode").is_none());
    assert!(model.get("config").is_some());
}

#[test]
fn mindmap_get_data_basic_nodes_edges_and_layout_defaults() {
    let model = parse("mindmap\nroot(Root Node)\n child1(Child 1)\n child2(Child 2)\n");

    assert_eq!(model["nodes"].as_array().unwrap().len(), 3);
    assert_eq!(model["edges"].as_array().unwrap().len(), 2);
    assert_eq!(model["config"]["layout"].as_str().unwrap(), "cose-bilkent");
    assert!(model["diagramId"].as_str().unwrap().starts_with("mindmap-"));

    let root = model["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"].as_str() == Some("0"))
        .unwrap();
    assert_eq!(root["label"].as_str().unwrap(), "Root Node");
    assert_eq!(root["level"].as_i64().unwrap(), 0);

    let edge_0_1 = model["edges"]
        .as_array()
        .unwrap()
        .iter()
        .find(|e| e["start"].as_str() == Some("0") && e["end"].as_str() == Some("1"))
        .unwrap();
    assert_eq!(edge_0_1["depth"].as_i64().unwrap(), 0);
}

#[test]
fn mindmap_get_data_projects_look_and_theme_shape_like_mermaid_11_15() {
    let model = parse(
        r#"%%{init: {"theme": "redux", "look": "neo"}}%%
mindmap
root
 child
"#,
    );

    let nodes = model["nodes"].as_array().unwrap();
    let root = nodes
        .iter()
        .find(|n| n["id"].as_str() == Some("0"))
        .unwrap();
    let child = nodes
        .iter()
        .find(|n| n["id"].as_str() == Some("1"))
        .unwrap();
    assert_eq!(root["look"].as_str().unwrap(), "neo");
    assert_eq!(child["look"].as_str().unwrap(), "neo");
    assert_eq!(root["shape"].as_str().unwrap(), "rounded");
    assert_eq!(child["shape"].as_str().unwrap(), "rounded");
    assert_eq!(model["shapes"]["0"]["shape"].as_str().unwrap(), "rounded");

    let edge = model["edges"].as_array().unwrap()[0].as_object().unwrap();
    assert_eq!(edge["look"].as_str().unwrap(), "neo");

    let default_model = parse("mindmap\nroot\n child\n");
    assert_eq!(
        default_model["nodes"][0]["look"].as_str().unwrap(),
        "classic"
    );
    assert_eq!(
        default_model["nodes"][0]["shape"].as_str().unwrap(),
        "defaultMindmapNode"
    );
}

#[test]
fn mindmap_render_model_projects_same_look_and_theme_shape_as_json_model() {
    let engine = Engine::new();
    let input = r#"%%{init: {"theme": "redux", "look": "neo"}}%%
mindmap
root
 child
"#;

    let parsed = engine
        .parse_diagram_for_render_model_sync(input, ParseOptions::strict())
        .unwrap()
        .unwrap();

    match parsed.model {
        RenderSemanticModel::Mindmap(model) => {
            assert_eq!(model.nodes[0].look, "neo");
            assert_eq!(model.nodes[1].look, "neo");
            assert_eq!(model.nodes[0].shape, "rounded");
            assert_eq!(model.nodes[1].shape, "rounded");
            assert_eq!(model.edges[0].look, "neo");
        }
        other => panic!("mindmap render parse should return typed model, got {other:?}"),
    }
}

#[test]
fn mindmap_deep_chain_semantic_and_render_model_use_heap_traversal() {
    const DEPTH: usize = 1200;
    let input = deep_mindmap_chain(DEPTH);

    let model = parse(&input);
    let nodes = model["nodes"].as_array().expect("nodes array");
    let edges = model["edges"].as_array().expect("edges array");
    assert_eq!(nodes.len(), DEPTH);
    assert_eq!(edges.len(), DEPTH - 1);
    assert_eq!(nodes[0]["label"].as_str(), Some("n0"));
    let expected_last = format!("n{}", DEPTH - 1);
    assert_eq!(
        nodes
            .last()
            .and_then(|node| node.get("label"))
            .and_then(Value::as_str),
        Some(expected_last.as_str())
    );

    let parsed = Engine::new()
        .parse_diagram_for_render_model_sync(&input, ParseOptions::strict())
        .unwrap()
        .unwrap();
    match parsed.model {
        RenderSemanticModel::Mindmap(model) => {
            assert_eq!(model.nodes.len(), DEPTH);
            assert_eq!(model.edges.len(), DEPTH - 1);
            assert_eq!(model.nodes[0].label, "n0");
            assert_eq!(
                model.nodes.last().map(|node| node.label.as_str()),
                Some(expected_last.as_str())
            );
        }
        other => panic!("mindmap render parse should return typed model, got {other:?}"),
    }
}

#[test]
fn mindmap_get_data_assigns_section_classes_to_nodes_and_edges() {
    let model = parse("mindmap\nA\n a0\n  aa0\n a1\n  aaa\n a2\n");
    let nodes = model["nodes"].as_array().unwrap();

    let node_a = nodes
        .iter()
        .find(|n| n["label"].as_str() == Some("A"))
        .unwrap();
    let node_a0 = nodes
        .iter()
        .find(|n| n["label"].as_str() == Some("a0"))
        .unwrap();
    let node_aa0 = nodes
        .iter()
        .find(|n| n["label"].as_str() == Some("aa0"))
        .unwrap();
    let node_a1 = nodes
        .iter()
        .find(|n| n["label"].as_str() == Some("a1"))
        .unwrap();
    let node_aaa = nodes
        .iter()
        .find(|n| n["label"].as_str() == Some("aaa"))
        .unwrap();
    let node_a2 = nodes
        .iter()
        .find(|n| n["label"].as_str() == Some("a2"))
        .unwrap();

    assert!(node_a.get("section").is_none());
    assert_eq!(
        node_a["cssClasses"].as_str().unwrap(),
        "mindmap-node section-root section--1"
    );
    assert_eq!(node_a0["section"].as_i64().unwrap(), 0);
    assert_eq!(node_aa0["section"].as_i64().unwrap(), 0);
    assert_eq!(node_a1["section"].as_i64().unwrap(), 1);
    assert_eq!(node_aaa["section"].as_i64().unwrap(), 1);
    assert_eq!(node_a2["section"].as_i64().unwrap(), 2);

    let edges = model["edges"].as_array().unwrap();
    assert_eq!(edges.len(), 5);

    let edge_0_1 = edges
        .iter()
        .find(|e| e["start"].as_str() == Some("0") && e["end"].as_str() == Some("1"))
        .unwrap();
    let edge_1_2 = edges
        .iter()
        .find(|e| e["start"].as_str() == Some("1") && e["end"].as_str() == Some("2"))
        .unwrap();
    let edge_0_3 = edges
        .iter()
        .find(|e| e["start"].as_str() == Some("0") && e["end"].as_str() == Some("3"))
        .unwrap();
    let edge_3_4 = edges
        .iter()
        .find(|e| e["start"].as_str() == Some("3") && e["end"].as_str() == Some("4"))
        .unwrap();
    let edge_0_5 = edges
        .iter()
        .find(|e| e["start"].as_str() == Some("0") && e["end"].as_str() == Some("5"))
        .unwrap();

    assert_eq!(
        edge_0_1["classes"].as_str().unwrap(),
        "edge section-edge-0 edge-depth-1"
    );
    assert_eq!(
        edge_1_2["classes"].as_str().unwrap(),
        "edge section-edge-0 edge-depth-2"
    );
    assert_eq!(
        edge_0_3["classes"].as_str().unwrap(),
        "edge section-edge-1 edge-depth-1"
    );
    assert_eq!(
        edge_3_4["classes"].as_str().unwrap(),
        "edge section-edge-1 edge-depth-2"
    );
    assert_eq!(
        edge_0_5["classes"].as_str().unwrap(),
        "edge section-edge-2 edge-depth-1"
    );

    assert_eq!(edge_0_1["section"].as_i64().unwrap(), 0);
    assert_eq!(edge_1_2["section"].as_i64().unwrap(), 0);
    assert_eq!(edge_0_3["section"].as_i64().unwrap(), 1);
    assert_eq!(edge_3_4["section"].as_i64().unwrap(), 1);
    assert_eq!(edge_0_5["section"].as_i64().unwrap(), 2);
}

#[test]
fn mindmap_get_data_edge_ids_are_unique() {
    let model = parse("mindmap\nroot\n child1\n child2\n child3\n");
    let edges = model["edges"].as_array().unwrap();
    assert_eq!(edges.len(), 3);

    let ids: Vec<&str> = edges.iter().map(|e| e["id"].as_str().unwrap()).collect();
    let unique: std::collections::BTreeSet<&str> = ids.iter().copied().collect();
    assert_eq!(unique.len(), ids.len());
}

#[test]
fn mindmap_get_data_missing_optional_properties_are_absent() {
    let model = parse("mindmap\nroot\n");
    let nodes = model["nodes"].as_array().unwrap();
    assert_eq!(nodes.len(), 1);
    let node = nodes[0].as_object().unwrap();

    assert!(node.get("section").is_none());
    assert_eq!(
        node.get("cssClasses").and_then(|v| v.as_str()).unwrap(),
        "mindmap-node section-root section--1"
    );
    assert!(node.get("icon").is_none());
    assert!(node.get("x").is_none());
    assert!(node.get("y").is_none());
}

#[test]
fn mindmap_get_data_preserves_custom_classes_while_adding_section_classes() {
    let model = parse(
        "mindmap\nroot(Root Node)\n:::custom-root-class\n child(Child Node)\n :::custom-child-class\n",
    );

    let nodes = model["nodes"].as_array().unwrap();
    let root = nodes
        .iter()
        .find(|n| n["label"].as_str() == Some("Root Node"))
        .unwrap();
    let child = nodes
        .iter()
        .find(|n| n["label"].as_str() == Some("Child Node"))
        .unwrap();

    assert_eq!(
        root["cssClasses"].as_str().unwrap(),
        "mindmap-node section-root section--1 custom-root-class"
    );
    assert_eq!(
        child["cssClasses"].as_str().unwrap(),
        "mindmap-node section-0 custom-child-class"
    );
}

#[test]
fn mindmap_padding_doubles_for_rect_like_nodes() {
    let model = parse("mindmap\nroot[Root]\n");
    let node = model["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"].as_str() == Some("0"))
        .unwrap();
    assert_eq!(node["type"].as_i64().unwrap(), NODE_TYPE_RECT as i64);
    assert_eq!(node["padding"].as_i64().unwrap(), 20);
}

#[test]
fn mindmap_empty_rows_and_comments_do_not_interfere() {
    let model = parse(
        "mindmap\n  root(Root)\n    Child(Child)\n      a(a)\n\n      %% This is a comment\n      b[New Stuff]\n",
    );
    let child = &model["rootNode"]["children"][0];
    assert_eq!(child["nodeId"].as_str().unwrap(), "Child");
    assert_eq!(child["children"].as_array().unwrap().len(), 2);
    assert_eq!(child["children"][1]["nodeId"].as_str().unwrap(), "b");
}
