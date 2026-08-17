use crate::*;
use futures::executor::block_on;
use serde_json::json;

#[test]
fn parse_diagram_flowchart_basic_graph() {
    let engine = Engine::new();
    let text = "graph TD;A-->B;";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.meta.diagram_type, "flowchart-v2");
    assert_eq!(
        res.model,
        json!({
            "type": "flowchart-v2",
            "keyword": "graph",
            "direction": "TB",
            "accTitle": null,
            "accDescr": null,
            "classDefs": {},
            "tooltips": {},
            "edgeDefaults": { "style": [], "interpolate": null },
            "vertexCalls": ["A", "B"],
            "nodes": [
                { "id": "A", "label": "A", "labelType": "text", "shape": null, "layoutShape": "squareRect", "icon": null, "form": null, "pos": null, "img": null, "constraint": null, "assetWidth": null, "assetHeight": null, "styles": [], "classes": [], "link": null, "linkTarget": null, "haveCallback": false },
                { "id": "B", "label": "B", "labelType": "text", "shape": null, "layoutShape": "squareRect", "icon": null, "form": null, "pos": null, "img": null, "constraint": null, "assetWidth": null, "assetHeight": null, "styles": [], "classes": [], "link": null, "linkTarget": null, "haveCallback": false }
            ],
            "edges": [
                { "from": "A", "to": "B", "id": "L_A_B_0", "isUserDefinedId": false, "arrow": "-->", "type": "arrow_point", "stroke": "normal", "length": 1, "label": null, "labelType": "text", "style": [], "classes": [], "interpolate": null, "animate": null, "animation": null }
            ],
            "subgraphs": []
        })
    );
}

#[test]
fn parse_diagram_flowchart_tolerates_edge_labels() {
    let engine = Engine::new();
    let text = "graph TD;A--x|text including URL space|B;";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.meta.diagram_type, "flowchart-v2");
    assert_eq!(
        res.model["edges"][0],
        json!({
            "from": "A",
            "to": "B",
            "id": "L_A_B_0",
            "isUserDefinedId": false,
            "arrow": "--x",
            "type": "arrow_cross",
            "stroke": "normal",
            "length": 1,
            "label": "text including URL space",
            "labelType": "text",
            "style": [],
            "classes": [],
            "interpolate": null,
            "animate": null,
            "animation": null
        })
    );
    assert_eq!(res.model["subgraphs"], json!([]));
}

#[test]
fn parse_diagram_flowchart_supports_inline_nodes() {
    let engine = Engine::new();
    let text = "graph TD;A[Start]-->B{Is it?};";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.meta.diagram_type, "flowchart-v2");
    assert_eq!(
        res.model,
        json!({
            "type": "flowchart-v2",
            "keyword": "graph",
            "direction": "TB",
            "accTitle": null,
            "accDescr": null,
            "classDefs": {},
            "tooltips": {},
            "edgeDefaults": { "style": [], "interpolate": null },
            "vertexCalls": ["A", "B"],
            "nodes": [
                { "id": "A", "label": "Start", "labelType": "text", "shape": "square", "layoutShape": "squareRect", "icon": null, "form": null, "pos": null, "img": null, "constraint": null, "assetWidth": null, "assetHeight": null, "styles": [], "classes": [], "link": null, "linkTarget": null, "haveCallback": false },
                { "id": "B", "label": "Is it?", "labelType": "text", "shape": "diamond", "layoutShape": "diamond", "icon": null, "form": null, "pos": null, "img": null, "constraint": null, "assetWidth": null, "assetHeight": null, "styles": [], "classes": [], "link": null, "linkTarget": null, "haveCallback": false }
            ],
            "edges": [
                { "from": "A", "to": "B", "id": "L_A_B_0", "isUserDefinedId": false, "arrow": "-->", "type": "arrow_point", "stroke": "normal", "length": 1, "label": null, "labelType": "text", "style": [], "classes": [], "interpolate": null, "animate": null, "animation": null }
            ],
            "subgraphs": []
        })
    );
}

#[test]
fn parse_diagram_flowchart_allows_dashes_in_node_ids() {
    let engine = Engine::new();
    let text = r#"
flowchart
    wi-fi["a node with dashes in its name"]
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.meta.diagram_type, "flowchart-v2");
    assert_eq!(res.model["nodes"][0]["id"], json!("wi-fi"));
    assert_eq!(
        res.model["nodes"][0]["label"],
        json!("a node with dashes in its name")
    );
}

#[test]
fn parse_diagram_flowchart_supports_quoted_edge_labels_and_pipe_labels_with_whitespace() {
    let engine = Engine::new();
    let text = r#"
flowchart TD
A[Node 1] -- "Some text" --> B[Node 2]
B --> |Other text| C[Node 3]
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.meta.diagram_type, "flowchart-v2");
    let edges = res.model["edges"].as_array().unwrap();
    assert_eq!(edges.len(), 2);
    assert_eq!(edges[0]["label"], json!("Some text"));
    assert_eq!(edges[0]["labelType"], json!("string"));
    assert_eq!(edges[1]["label"], json!("Other text"));
    assert_eq!(edges[1]["labelType"], json!("text"));
}

#[test]
fn parse_diagram_flowchart_edge_stroke_and_type_normal_thick_dotted() {
    let engine = Engine::new();

    let res = block_on(engine.parse_diagram("graph TD;A-->B;", ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["edges"][0]["type"], json!("arrow_point"));
    assert_eq!(res.model["edges"][0]["stroke"], json!("normal"));
    assert_eq!(res.model["edges"][0]["length"], json!(1));

    let res = block_on(engine.parse_diagram("graph TD;A==>B;", ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["edges"][0]["type"], json!("arrow_point"));
    assert_eq!(res.model["edges"][0]["stroke"], json!("thick"));
    assert_eq!(res.model["edges"][0]["length"], json!(1));

    let res = block_on(engine.parse_diagram("graph TD;A-.->B;", ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["edges"][0]["type"], json!("arrow_point"));
    assert_eq!(res.model["edges"][0]["stroke"], json!("dotted"));
    assert_eq!(res.model["edges"][0]["length"], json!(1));
}

#[test]
fn parse_diagram_flowchart_double_ended_arrows() {
    let engine = Engine::new();

    let res = block_on(engine.parse_diagram("graph TD;A<-->B;", ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["edges"][0]["type"], json!("double_arrow_point"));
    assert_eq!(res.model["edges"][0]["stroke"], json!("normal"));
    assert_eq!(res.model["edges"][0]["length"], json!(1));
}

#[test]
fn parse_diagram_flowchart_edge_text_new_notation() {
    let engine = Engine::new();
    let text = "graph TD;A-- text including URL space and send -->B;";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["edges"][0]["type"], json!("arrow_point"));
    assert_eq!(
        res.model["edges"][0]["label"],
        json!("text including URL space and send")
    );
}

#[test]
fn parse_diagram_flowchart_edge_text_new_notation_double_ended() {
    let engine = Engine::new();
    let text = "graph TD;A<-- text -->B;";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["edges"][0]["type"], json!("double_arrow_point"));
    assert_eq!(res.model["edges"][0]["label"], json!("text"));
}

#[test]
fn parse_diagram_flowchart_invisible_edge() {
    let engine = Engine::new();
    let res = block_on(engine.parse_diagram("graph TD;A~~~B;", ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["edges"][0]["type"], json!("arrow_open"));
    assert_eq!(res.model["edges"][0]["stroke"], json!("invisible"));
    assert_eq!(res.model["edges"][0]["length"], json!(1));
}

#[test]
fn parse_diagram_flowchart_edges_spec_open_cross_circle() {
    let engine = Engine::new();

    let res = block_on(engine.parse_diagram("graph TD;A---B;", ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["edges"][0]["type"], json!("arrow_open"));
    assert_eq!(res.model["edges"][0]["stroke"], json!("normal"));
    assert_eq!(res.model["edges"][0]["length"], json!(1));

    let res = block_on(engine.parse_diagram("graph TD;A--xB;", ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["edges"][0]["type"], json!("arrow_cross"));
    assert_eq!(res.model["edges"][0]["stroke"], json!("normal"));
    assert_eq!(res.model["edges"][0]["length"], json!(1));

    let res = block_on(engine.parse_diagram("graph TD;A--oB;", ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["edges"][0]["type"], json!("arrow_circle"));
    assert_eq!(res.model["edges"][0]["stroke"], json!("normal"));
    assert_eq!(res.model["edges"][0]["length"], json!(1));
}

#[test]
fn parse_diagram_flowchart_edges_spec_edge_ids_and_node_metadata_do_not_conflict() {
    let engine = Engine::new();
    let text = "flowchart LR\nA id1@-->B\nA@{ shape: 'rect' }\n";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["edges"][0]["id"], json!("id1"));
    assert_eq!(res.model["edges"][0]["type"], json!("arrow_point"));
}

#[test]
fn parse_diagram_flowchart_edges_spec_edge_length_matrix() {
    let engine = Engine::new();
    let assert_edge = |diagram: String,
                       expected_type: &str,
                       expected_stroke: &str,
                       expected_length: usize,
                       expected_label: Option<&str>| {
        let res = block_on(engine.parse_diagram(&diagram, ParseOptions::default()))
            .unwrap()
            .unwrap();
        let e = &res.model["edges"][0];
        assert_eq!(e["type"], json!(expected_type), "diagram: {diagram}");
        assert_eq!(e["stroke"], json!(expected_stroke), "diagram: {diagram}");
        assert_eq!(e["length"], json!(expected_length), "diagram: {diagram}");
        match expected_label {
            Some(label) => assert_eq!(e["label"], json!(label), "diagram: {diagram}"),
            None => assert!(e["label"].is_null(), "diagram: {diagram}"),
        }
    };

    for length in 1..=3 {
        assert_edge(
            format!("graph TD;\nA -{}- B;", "-".repeat(length)),
            "arrow_open",
            "normal",
            length,
            None,
        );
        assert_edge(
            format!("graph TD;\nA -- Label -{}- B;", "-".repeat(length)),
            "arrow_open",
            "normal",
            length,
            Some("Label"),
        );
        assert_edge(
            format!("graph TD;\nA -{}> B;", "-".repeat(length)),
            "arrow_point",
            "normal",
            length,
            None,
        );
        assert_edge(
            format!("graph TD;\nA -- Label -{}> B;", "-".repeat(length)),
            "arrow_point",
            "normal",
            length,
            Some("Label"),
        );
        assert_edge(
            format!("graph TD;\nA <-{}> B;", "-".repeat(length)),
            "double_arrow_point",
            "normal",
            length,
            None,
        );
        assert_edge(
            format!("graph TD;\nA <-- Label -{}> B;", "-".repeat(length)),
            "double_arrow_point",
            "normal",
            length,
            Some("Label"),
        );
    }

    for length in 1..=3 {
        assert_edge(
            format!("graph TD;\nA ={}= B;", "=".repeat(length)),
            "arrow_open",
            "thick",
            length,
            None,
        );
        assert_edge(
            format!("graph TD;\nA == Label ={}= B;", "=".repeat(length)),
            "arrow_open",
            "thick",
            length,
            Some("Label"),
        );
        assert_edge(
            format!("graph TD;\nA ={}> B;", "=".repeat(length)),
            "arrow_point",
            "thick",
            length,
            None,
        );
        assert_edge(
            format!("graph TD;\nA == Label ={}> B;", "=".repeat(length)),
            "arrow_point",
            "thick",
            length,
            Some("Label"),
        );
        assert_edge(
            format!("graph TD;\nA <={}> B;", "=".repeat(length)),
            "double_arrow_point",
            "thick",
            length,
            None,
        );
        assert_edge(
            format!("graph TD;\nA <== Label ={}> B;", "=".repeat(length)),
            "double_arrow_point",
            "thick",
            length,
            Some("Label"),
        );
    }

    for length in 1..=3 {
        assert_edge(
            format!("graph TD;\nA -{}- B;", ".".repeat(length)),
            "arrow_open",
            "dotted",
            length,
            None,
        );
        assert_edge(
            format!("graph TD;\nA -. Label {}- B;", ".".repeat(length)),
            "arrow_open",
            "dotted",
            length,
            Some("Label"),
        );
        assert_edge(
            format!("graph TD;\nA -{}-> B;", ".".repeat(length)),
            "arrow_point",
            "dotted",
            length,
            None,
        );
        assert_edge(
            format!("graph TD;\nA -. Label {}-> B;", ".".repeat(length)),
            "arrow_point",
            "dotted",
            length,
            Some("Label"),
        );
        assert_edge(
            format!("graph TD;\nA <-{}-> B;", ".".repeat(length)),
            "double_arrow_point",
            "dotted",
            length,
            None,
        );
        assert_edge(
            format!("graph TD;\nA <-. Label {}-> B;", ".".repeat(length)),
            "double_arrow_point",
            "dotted",
            length,
            Some("Label"),
        );
    }
}

#[test]
fn parse_diagram_flowchart_edges_spec_keywords_as_edge_labels_in_double_ended_edges() {
    let engine = Engine::new();

    let keywords = [
        "graph",
        "flowchart",
        "flowchart-elk",
        "style",
        "default",
        "linkStyle",
        "interpolate",
        "classDef",
        "class",
        "href",
        "call",
        "click",
        "_self",
        "_blank",
        "_parent",
        "_top",
        "end",
        "subgraph",
        "kitty",
    ];

    let edges = [
        ("x--", "--x", "normal", "double_arrow_cross"),
        ("x==", "==x", "thick", "double_arrow_cross"),
        ("x-.", ".-x", "dotted", "double_arrow_cross"),
        ("o--", "--o", "normal", "double_arrow_circle"),
        ("o==", "==o", "thick", "double_arrow_circle"),
        ("o-.", ".-o", "dotted", "double_arrow_circle"),
        ("<--", "-->", "normal", "double_arrow_point"),
        ("<==", "==>", "thick", "double_arrow_point"),
        ("<-.", ".->", "dotted", "double_arrow_point"),
    ];

    for (edge_start, edge_end, stroke, edge_type) in edges {
        for keyword in keywords {
            let diagram = format!("graph TD;\nA {edge_start} {keyword} {edge_end} B;");
            let res = block_on(engine.parse_diagram(&diagram, ParseOptions::default()))
                .unwrap()
                .unwrap();
            let e = &res.model["edges"][0];
            assert_eq!(e["type"], json!(edge_type), "diagram: {diagram}");
            assert_eq!(e["stroke"], json!(stroke), "diagram: {diagram}");
            assert_eq!(e["label"], json!(keyword), "diagram: {diagram}");
            assert_eq!(e["labelType"], json!("text"), "diagram: {diagram}");
        }
    }
}

#[test]
fn parse_diagram_flowchart_node_data_basic_shape_data_statements() {
    let engine = Engine::new();

    let res = block_on(
        engine.parse_diagram("flowchart TB\nD@{ shape: rounded}", ParseOptions::default()),
    )
    .unwrap()
    .unwrap();
    let nodes = res.model["nodes"].as_array().unwrap();
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0]["id"], json!("D"));
    assert_eq!(nodes[0]["layoutShape"], json!("rounded"));
    assert_eq!(nodes[0]["label"], json!("D"));

    let res = block_on(engine.parse_diagram(
        "flowchart TB\nD@{ shape: rounded }",
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let nodes = res.model["nodes"].as_array().unwrap();
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0]["layoutShape"], json!("rounded"));
    assert_eq!(nodes[0]["label"], json!("D"));
}

#[test]
fn parse_diagram_flowchart_node_data_shape_data_with_amp_and_edges() {
    let engine = Engine::new();

    let res = block_on(engine.parse_diagram(
        "flowchart TB\nD@{ shape: rounded } & E",
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let nodes = res.model["nodes"].as_array().unwrap();
    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0]["id"], json!("D"));
    assert_eq!(nodes[0]["layoutShape"], json!("rounded"));
    assert_eq!(nodes[0]["label"], json!("D"));
    assert_eq!(nodes[1]["id"], json!("E"));
    assert_eq!(nodes[1]["label"], json!("E"));

    let res = block_on(engine.parse_diagram(
        "flowchart TB\nD@{ shape: rounded } --> E",
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let nodes = res.model["nodes"].as_array().unwrap();
    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0]["id"], json!("D"));
    assert_eq!(nodes[0]["layoutShape"], json!("rounded"));
    assert_eq!(nodes[1]["id"], json!("E"));
}

#[test]
fn parse_diagram_flowchart_node_data_shape_data_whitespace_variants() {
    let engine = Engine::new();

    for diagram in [
        "flowchart TB\nD@{shape: rounded}",
        "flowchart TB\nD@{       shape: rounded}",
        "flowchart TB\nD@{ shape: rounded         }",
    ] {
        let res = block_on(engine.parse_diagram(diagram, ParseOptions::default()))
            .unwrap()
            .unwrap();
        let nodes = res.model["nodes"].as_array().unwrap();
        assert_eq!(nodes.len(), 1, "diagram: {diagram}");
        assert_eq!(nodes[0]["id"], json!("D"), "diagram: {diagram}");
        assert_eq!(
            nodes[0]["layoutShape"],
            json!("rounded"),
            "diagram: {diagram}"
        );
        assert_eq!(nodes[0]["label"], json!("D"), "diagram: {diagram}");
    }
}

#[test]
fn parse_diagram_flowchart_node_data_shape_data_accepts_datastore() {
    let engine = Engine::new();

    for shape in ["datastore", "data-store"] {
        let diagram = format!("flowchart TB\nD@{{ shape: {shape}, label: \"Datastore\" }}");
        let res = block_on(engine.parse_diagram(&diagram, ParseOptions::default()))
            .unwrap()
            .unwrap();
        let nodes = res.model["nodes"].as_array().unwrap();
        assert_eq!(nodes.len(), 1, "diagram: {diagram}");
        assert_eq!(nodes[0]["id"], json!("D"), "diagram: {diagram}");
        assert_eq!(nodes[0]["layoutShape"], json!(shape), "diagram: {diagram}");
        assert_eq!(nodes[0]["label"], json!("Datastore"), "diagram: {diagram}");
        assert_eq!(
            nodes[0]["labelType"],
            json!("markdown"),
            "diagram: {diagram}"
        );
    }
}

#[test]
fn parse_diagram_flowchart_node_data_shape_data_amp_and_edge_matrix() {
    let engine = Engine::new();

    let cases = [
        (
            "flowchart TB\nD@{ shape: rounded } & E --> F",
            3usize,
            "D",
            "rounded",
        ),
        (
            "flowchart TB\nD@{ shape: rounded } & E@{ shape: rounded } --> F",
            3usize,
            "D",
            "rounded",
        ),
        (
            "flowchart TB\nD@{ shape: rounded } & E@{ shape: rounded } --> F & G@{ shape: rounded }",
            4usize,
            "D",
            "rounded",
        ),
        (
            "flowchart TB\nD@{ shape: rounded } & E@{ shape: rounded } --> F@{ shape: rounded } & G@{ shape: rounded }",
            4usize,
            "D",
            "rounded",
        ),
        (
            "flowchart TB\nD@{ shape: rounded } & E@{ shape: rounded } --> F{ shape: rounded } & G{ shape: rounded }    ",
            4usize,
            "D",
            "rounded",
        ),
    ];

    for (diagram, expected_nodes, first_id, first_layout) in cases {
        let res = block_on(engine.parse_diagram(diagram, ParseOptions::default()))
            .unwrap()
            .unwrap();
        let nodes = res.model["nodes"].as_array().unwrap();
        assert_eq!(nodes.len(), expected_nodes, "diagram: {diagram}");
        assert_eq!(nodes[0]["id"], json!(first_id), "diagram: {diagram}");
        assert_eq!(
            nodes[0]["layoutShape"],
            json!(first_layout),
            "diagram: {diagram}"
        );
    }
}

#[test]
fn parse_diagram_flowchart_node_data_shape_data_allows_brace_in_multiline_string() {
    let engine = Engine::new();

    let text = r#"flowchart TB
A@{
  label: "This is }"
  other: "clock"
}
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let nodes = res.model["nodes"].as_array().unwrap();
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0]["label"], json!("This is }"));
}

#[test]
fn parse_diagram_flowchart_node_data_multiple_properties_same_line() {
    let engine = Engine::new();

    let res = block_on(engine.parse_diagram(
        "flowchart TB\nD@{ shape: rounded , label: \"DD\"}",
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let nodes = res.model["nodes"].as_array().unwrap();
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0]["id"], json!("D"));
    assert_eq!(nodes[0]["layoutShape"], json!("rounded"));
    assert_eq!(nodes[0]["label"], json!("DD"));
    assert_eq!(nodes[0]["labelType"], json!("markdown"));
}

#[test]
fn parse_diagram_flowchart_node_data_label_type_defaults_to_markdown_but_can_be_overridden() {
    let engine = Engine::new();

    let res = block_on(engine.parse_diagram(
        "flowchart TB\nA@{ label: \"Default markdown\" }\nB@{ label: \"Plain text\", labelType: text }",
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let nodes = res.model["nodes"].as_array().unwrap();
    let find = |id: &str| nodes.iter().find(|n| n["id"] == json!(id)).unwrap();
    assert_eq!(find("A")["labelType"], json!("markdown"));
    assert_eq!(find("B")["labelType"], json!("text"));
}

#[test]
fn parse_diagram_flowchart_node_data_link_to_node_with_more_data_multiline_yaml() {
    let engine = Engine::new();

    let text = r#"flowchart TB
A --> D@{
  shape: circle
  other: "clock"
}
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let nodes = res.model["nodes"].as_array().unwrap();
    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0]["id"], json!("A"));
    assert_eq!(nodes[0]["layoutShape"], json!("squareRect"));
    assert_eq!(nodes[0]["label"], json!("A"));
    assert_eq!(nodes[1]["id"], json!("D"));
    assert_eq!(nodes[1]["layoutShape"], json!("circle"));
    assert_eq!(nodes[1]["label"], json!("D"));
    assert_eq!(res.model["edges"].as_array().unwrap().len(), 1);
}

#[test]
fn parse_diagram_flowchart_node_data_nodes_after_each_other() {
    let engine = Engine::new();
    let text = r#"flowchart TB
A[hello]
B@{
  shape: circle
  other: "clock"
}
C[Hello]@{
  shape: circle
  other: "clock"
}
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let nodes = res.model["nodes"].as_array().unwrap();
    assert_eq!(nodes.len(), 3);
    assert_eq!(nodes[0]["id"], json!("A"));
    assert_eq!(nodes[0]["label"], json!("hello"));
    assert_eq!(nodes[0]["layoutShape"], json!("squareRect"));
    assert_eq!(nodes[1]["id"], json!("B"));
    assert_eq!(nodes[1]["label"], json!("B"));
    assert_eq!(nodes[1]["layoutShape"], json!("circle"));
    assert_eq!(nodes[2]["id"], json!("C"));
    assert_eq!(nodes[2]["label"], json!("Hello"));
    assert_eq!(nodes[2]["layoutShape"], json!("circle"));
}

#[test]
fn parse_diagram_flowchart_node_data_shape_data_allows_brace_and_at_in_strings() {
    let engine = Engine::new();

    let res = block_on(engine.parse_diagram(
        "flowchart TB\nA@{ label: \"This is }\" }",
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let nodes = res.model["nodes"].as_array().unwrap();
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0]["layoutShape"], json!("squareRect"));
    assert_eq!(nodes[0]["label"], json!("This is }"));

    let res = block_on(engine.parse_diagram(
        "flowchart TB\nA@{ label: \"This is a string with @\" }",
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let nodes = res.model["nodes"].as_array().unwrap();
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0]["label"], json!("This is a string with @"));
}

#[test]
fn parse_diagram_flowchart_node_data_shape_validation_errors() {
    let engine = Engine::new();

    let err = block_on(engine.parse_diagram(
        "flowchart TB\nA@{ shape: this-shape-does-not-exist }",
        ParseOptions::default(),
    ))
    .unwrap_err();
    assert!(
        err.to_string()
            .contains("No such shape: this-shape-does-not-exist.")
    );

    let err = block_on(engine.parse_diagram(
        "flowchart TB\nA@{ shape: rect_left_inv_arrow }",
        ParseOptions::default(),
    ))
    .unwrap_err();
    assert!(
        err.to_string()
            .contains("No such shape: rect_left_inv_arrow. Shape names should be lowercase.")
    );
}

#[test]
fn parse_diagram_flowchart_node_data_multiline_strings_match_mermaid() {
    let engine = Engine::new();

    let res = block_on(engine.parse_diagram(
        r#"flowchart TB
A@{
  label: |
    This is a
    multiline string
  other: "clock"
}
"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let nodes = res.model["nodes"].as_array().unwrap();
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0]["label"], json!("This is a\nmultiline string\n"));

    let res = block_on(engine.parse_diagram(
        r#"flowchart TB
A@{
  label: "This is a
    multiline string"
  other: "clock"
}
"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let nodes = res.model["nodes"].as_array().unwrap();
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0]["label"], json!("This is a<br/>multiline string"));
}

#[test]
fn parse_diagram_flowchart_node_data_labels_across_multi_nodes_and_edges() {
    let engine = Engine::new();

    let text = r#"flowchart TB
n2["label for n2"] & n4@{ label: "label for n4"} & n5@{ label: "label for n5"}
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let nodes = res.model["nodes"].as_array().unwrap();
    assert_eq!(nodes.len(), 3);
    assert_eq!(nodes[0]["label"], json!("label for n2"));
    assert_eq!(nodes[1]["label"], json!("label for n4"));
    assert_eq!(nodes[2]["label"], json!("label for n5"));

    let text = r#"flowchart TD
A["A"] --> B["for B"] & C@{ label: "for c"} & E@{label : "for E"}
D@{label: "for D"}
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let nodes = res.model["nodes"].as_array().unwrap();
    assert_eq!(nodes.len(), 5);
    assert_eq!(nodes[0]["label"], json!("A"));
    assert_eq!(nodes[1]["label"], json!("for B"));
    assert_eq!(nodes[2]["label"], json!("for c"));
    assert_eq!(nodes[3]["label"], json!("for E"));
    assert_eq!(nodes[4]["label"], json!("for D"));
}

#[test]
fn parse_diagram_flowchart_node_data_allows_at_in_labels_across_shapes() {
    let engine = Engine::new();

    let text = r#"flowchart TD
A["@A@"] --> B["@for@ B@"] & C@{ label: "@for@ c@"} & E{"`@for@ E@`"} & D(("@for@ D@"))
H1{{"@for@ H@"}}
H2{{"`@for@ H@`"}}
Q1{"@for@ Q@"}
Q2{"`@for@ Q@`"}
AS1>"@for@ AS@"]
AS2>"`@for@ AS@`"]
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let nodes = res.model["nodes"].as_array().unwrap();
    assert_eq!(nodes.len(), 11);
    for (i, node) in nodes.iter().enumerate() {
        assert!(
            node["label"].as_str().unwrap().contains("@for@") || node["label"] == json!("@A@"),
            "node {i}: {:?}",
            node
        );
    }
    assert_eq!(nodes[0]["label"], json!("@A@"));
    assert_eq!(nodes[1]["label"], json!("@for@ B@"));
    assert_eq!(nodes[2]["label"], json!("@for@ c@"));
    assert_eq!(nodes[3]["label"], json!("@for@ E@"));
    assert_eq!(nodes[4]["label"], json!("@for@ D@"));
    assert_eq!(nodes[5]["label"], json!("@for@ H@"));
    assert_eq!(nodes[6]["label"], json!("@for@ H@"));
    assert_eq!(nodes[7]["label"], json!("@for@ Q@"));
    assert_eq!(nodes[8]["label"], json!("@for@ Q@"));
    assert_eq!(nodes[9]["label"], json!("@for@ AS@"));
    assert_eq!(nodes[10]["label"], json!("@for@ AS@"));
}

#[test]
fn parse_diagram_flowchart_node_data_unique_edge_ids_with_groups() {
    let engine = Engine::new();

    let text = r#"flowchart TD
A & B e1@--> C & D
A1 e2@--> C1 & D1
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["nodes"].as_array().unwrap().len(), 7);
    let edges = res.model["edges"].as_array().unwrap();
    assert_eq!(edges.len(), 6);
    assert_eq!(edges[0]["id"], json!("L_A_C_0"));
    assert_eq!(edges[1]["id"], json!("L_A_D_0"));
    assert_eq!(edges[2]["id"], json!("e1"));
    assert_eq!(edges[3]["id"], json!("L_B_D_0"));
    assert_eq!(edges[4]["id"], json!("e2"));
    assert_eq!(edges[5]["id"], json!("L_A1_D1_0"));
}

#[test]
fn parse_diagram_flowchart_node_data_redefined_edge_id_becomes_auto_id() {
    let engine = Engine::new();

    let text = r#"flowchart TD
A & B e1@--> C & D
A1 e1@--> C1 & D1
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let edges = res.model["edges"].as_array().unwrap();
    assert_eq!(edges.len(), 6);
    assert_eq!(edges[0]["id"], json!("L_A_C_0"));
    assert_eq!(edges[1]["id"], json!("L_A_D_0"));
    assert_eq!(edges[2]["id"], json!("e1"));
    assert_eq!(edges[3]["id"], json!("L_B_D_0"));
    assert_eq!(edges[4]["id"], json!("L_A1_C1_0"));
    assert_eq!(edges[5]["id"], json!("L_A1_D1_0"));
}

#[test]
fn parse_diagram_flowchart_node_data_overrides_edge_animate() {
    let engine = Engine::new();

    let text = r#"flowchart TD
A e1@--> B
C e2@--> D
E e3@--> F
e1@{ animate: true }
e2@{ animate: false }
e3@{ animate: true }
e3@{ animate: false }
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let edges = res.model["edges"].as_array().unwrap();
    assert_eq!(edges.len(), 3);
    assert_eq!(edges[0]["id"], json!("e1"));
    assert_eq!(edges[0]["animate"], json!(true));
    assert_eq!(edges[1]["id"], json!("e2"));
    assert_eq!(edges[1]["animate"], json!(false));
    assert_eq!(edges[2]["id"], json!("e3"));
    assert_eq!(edges[2]["animate"], json!(false));
}

#[test]
fn parse_diagram_flowchart_markdown_strings_in_nodes_and_edges() {
    let engine = Engine::new();
    let text = "flowchart\nA[\"`The cat in **the** hat`\"]-- \"`The *bat* in the chat`\" -->B[\"The dog in the hog\"] -- \"The rat in the mat\" -->C;";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();

    let nodes = res.model["nodes"].as_array().unwrap();
    let find_node = |id: &str| nodes.iter().find(|n| n["id"] == json!(id)).unwrap();
    let node_a = find_node("A");
    let node_b = find_node("B");

    assert_eq!(node_a["label"], json!("The cat in **the** hat"));
    assert_eq!(node_a["labelType"], json!("markdown"));
    assert_eq!(node_b["label"], json!("The dog in the hog"));
    assert_eq!(node_b["labelType"], json!("string"));

    let edges = res.model["edges"].as_array().unwrap();
    assert_eq!(edges.len(), 2);
    assert_eq!(edges[0]["from"], json!("A"));
    assert_eq!(edges[0]["to"], json!("B"));
    assert_eq!(edges[0]["type"], json!("arrow_point"));
    assert_eq!(edges[0]["label"], json!("The *bat* in the chat"));
    assert_eq!(edges[0]["labelType"], json!("markdown"));
    assert_eq!(edges[1]["from"], json!("B"));
    assert_eq!(edges[1]["to"], json!("C"));
    assert_eq!(edges[1]["type"], json!("arrow_point"));
    assert_eq!(edges[1]["label"], json!("The rat in the mat"));
    assert_eq!(edges[1]["labelType"], json!("string"));
}

#[test]
fn parse_diagram_flowchart_markdown_strings_in_subgraphs() {
    let engine = Engine::new();
    let text = r#"flowchart LR
subgraph "One"
  a("`The **cat**
  in the hat`") -- "1o" --> b{{"`The **dog** in the hog`"}}
end
subgraph "`**Two**`"
  c("`The **cat**
  in the hat`") -- "`1o **ipa**`" --> d("The dog in the hog")
end"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();

    let subgraphs = res.model["subgraphs"].as_array().unwrap();
    assert_eq!(subgraphs.len(), 2);
    assert_eq!(subgraphs[0]["nodes"].as_array().unwrap().len(), 2);
    assert_eq!(subgraphs[0]["title"], json!("One"));
    assert_eq!(subgraphs[0]["labelType"], json!("text"));
    assert_eq!(subgraphs[1]["nodes"].as_array().unwrap().len(), 2);
    assert_eq!(subgraphs[1]["title"], json!("**Two**"));
    assert_eq!(subgraphs[1]["labelType"], json!("markdown"));
}

#[test]
fn parse_diagram_flowchart_header_direction_shorthand() {
    let engine = Engine::new();

    let res = block_on(engine.parse_diagram("graph >;A-->B;", ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["direction"], json!("LR"));

    let res = block_on(engine.parse_diagram("graph <;A-->B;", ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["direction"], json!("RL"));

    let res = block_on(engine.parse_diagram("graph ^;A-->B;", ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["direction"], json!("BT"));

    let res = block_on(engine.parse_diagram("graph v;A-->B;", ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["direction"], json!("TB"));
}

#[test]
fn parse_diagram_flowchart_v_is_node_id_not_direction() {
    let engine = Engine::new();
    let res = block_on(engine.parse_diagram("graph TD;A--xv(my text);", ParseOptions::default()))
        .unwrap()
        .unwrap();

    let v = res.model["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"] == json!("v"))
        .unwrap();
    assert_eq!(v["label"], json!("my text"));
    assert_eq!(v["shape"], json!("round"));
    assert_eq!(res.model["edges"][0]["type"], json!("arrow_cross"));
}

#[test]
fn parse_diagram_flowchart_v_in_node_ids_variants_from_flow_text_spec() {
    let engine = Engine::new();
    let text = "graph TD;A--xv(my text);A--xcsv(my text);A--xava(my text);A--xva(my text);";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();

    assert_eq!(res.model["edges"].as_array().unwrap().len(), 4);
    for edge in res.model["edges"].as_array().unwrap() {
        assert_eq!(edge["type"], json!("arrow_cross"));
    }

    let nodes = res.model["nodes"].as_array().unwrap();
    let find = |id: &str| nodes.iter().find(|n| n["id"] == json!(id)).unwrap();

    assert_eq!(find("v")["label"], json!("my text"));
    assert_eq!(find("csv")["label"], json!("my text"));
    assert_eq!(find("ava")["label"], json!("my text"));
    assert_eq!(find("va")["label"], json!("my text"));
}

#[test]
fn parse_diagram_flowchart_edge_label_supports_quoted_strings() {
    let engine = Engine::new();
    let res = block_on(engine.parse_diagram(
        "graph TD;V-- \"test string()\" -->a[v]",
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    assert_eq!(res.model["edges"][0]["label"], json!("test string()"));
    assert_eq!(res.model["edges"][0]["labelType"], json!("string"));
}

#[test]
fn parse_diagram_flowchart_edge_label_old_notation_without_spaces() {
    let engine = Engine::new();
    let res = block_on(engine.parse_diagram(
        "graph TD;A--text including URL space and send-->B;",
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    assert_eq!(
        res.model["edges"][0]["label"],
        json!("text including URL space and send")
    );
    assert_eq!(res.model["edges"][0]["type"], json!("arrow_point"));
}

#[test]
fn parse_diagram_flowchart_edge_labels_can_span_multiple_lines() {
    let engine = Engine::new();
    let text = "graph TD;A--o|text space|B;\n B-->|more text with space|C;";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["edges"].as_array().unwrap().len(), 2);
    assert_eq!(res.model["edges"][0]["type"], json!("arrow_circle"));
    assert_eq!(res.model["edges"][1]["type"], json!("arrow_point"));
    assert_eq!(
        res.model["edges"][1]["label"],
        json!("more text with space")
    );
}

#[test]
fn parse_diagram_flowchart_vertex_shapes_from_flow_text_spec() {
    let engine = Engine::new();
    let text = r#"graph TD;
A_node-->B[This is square];
A_node-->C(Chimpansen hoppar);
A_node-->D{Diamond};
A_node-->E((Circle));
A_node-->F(((Double circle)));
A_node-->G{{Hex}};
A_node-->H[[Subroutine]];
A_node-->I(-Ellipse-);
A_node-->J([Stadium]);
A_node-->K[(Cylinder)];
A_node-->L>Odd];
A_node-->M[/Lean right/];
A_node-->N[\Lean left\];
A_node-->O[/Trapezoid\];
A_node-->P[\Inv trapezoid/];
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();

    let nodes = res.model["nodes"].as_array().unwrap();
    let find = |id: &str| nodes.iter().find(|n| n["id"] == json!(id)).unwrap();

    assert_eq!(find("B")["shape"], json!("square"));
    assert_eq!(find("C")["shape"], json!("round"));
    assert_eq!(find("D")["shape"], json!("diamond"));
    assert_eq!(find("E")["shape"], json!("circle"));
    assert_eq!(find("F")["shape"], json!("doublecircle"));
    assert_eq!(find("G")["shape"], json!("hexagon"));
    assert_eq!(find("H")["shape"], json!("subroutine"));
    assert_eq!(find("I")["shape"], json!("ellipse"));
    assert_eq!(find("J")["shape"], json!("stadium"));
    assert_eq!(find("K")["shape"], json!("cylinder"));
    assert_eq!(find("L")["shape"], json!("odd"));
    assert_eq!(find("M")["shape"], json!("lean_right"));
    assert_eq!(find("N")["shape"], json!("lean_left"));
    assert_eq!(find("O")["shape"], json!("trapezoid"));
    assert_eq!(find("P")["shape"], json!("inv_trapezoid"));
}

#[test]
fn parse_diagram_flowchart_rect_border_syntax_sets_rect_shape() {
    let engine = Engine::new();
    let text = "graph TD;A_node-->B[|borders:lt|This node has a graph as text];";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let b = res.model["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"] == json!("B"))
        .unwrap();
    assert_eq!(b["shape"], json!("rect"));
    assert_eq!(b["label"], json!("This node has a graph as text"));
}

#[test]
fn parse_diagram_flowchart_odd_vertex_allows_id_ending_with_minus() {
    let engine = Engine::new();
    let text = "graph TD;A_node-->odd->Vertex Text];";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();

    let odd = res.model["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"] == json!("odd-"))
        .unwrap();
    assert_eq!(odd["shape"], json!("odd"));
    assert_eq!(odd["label"], json!("Vertex Text"));
}

#[test]
fn parse_diagram_flowchart_allows_brackets_inside_quoted_square_labels() {
    let engine = Engine::new();
    let text = "graph TD;A[\"chimpansen hoppar ()[]\"] --> C;";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let a = res.model["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"] == json!("A"))
        .unwrap();
    assert_eq!(a["shape"], json!("square"));
    assert_eq!(a["label"], json!("chimpansen hoppar ()[]"));
    assert_eq!(a["labelType"], json!("string"));
}

#[test]
fn parse_diagram_flowchart_flow_text_error_cases_from_upstream_spec() {
    let engine = Engine::new();

    let err = block_on(engine.parse_diagram(
        "graph TD; A[This is a () in text];",
        ParseOptions::default(),
    ))
    .unwrap_err();
    assert!(
        err.to_string()
            .contains("Invalid text label: contains structural characters; quote it to use them")
    );

    let err = block_on(engine.parse_diagram(
        "graph TD;A(this node has \"string\" and text)-->|this link has \"string\" and text|C;",
        ParseOptions::default(),
    ))
    .unwrap_err();
    assert!(
        err.to_string()
            .contains("Invalid text label: contains structural characters; quote it to use them")
    );

    let err = block_on(engine.parse_diagram(
        "graph TD; A[This is a \\\"()\\\" in text];",
        ParseOptions::default(),
    ))
    .unwrap_err();
    assert!(
        err.to_string()
            .contains("Unterminated node label (missing `]`)")
    );

    let err = block_on(engine.parse_diagram(
        "graph TD; A[\"This is a \"()\" in text\"];",
        ParseOptions::default(),
    ))
    .unwrap_err();
    assert!(
        err.to_string()
            .contains("Invalid string label: contains nested quotes")
    );

    let err = block_on(engine.parse_diagram(
        "graph TD; node[hello ) world] --> works",
        ParseOptions::default(),
    ))
    .unwrap_err();
    assert!(
        err.to_string()
            .contains("Invalid text label: contains structural characters; quote it to use them")
    );

    let err = block_on(engine.parse_diagram("graph\nX(- My Text (", ParseOptions::default()))
        .unwrap_err();
    assert!(
        err.to_string()
            .contains("Unterminated node label (missing `-)`)")
    );
}

#[test]
fn parse_diagram_flowchart_keywords_in_vertex_text_across_shapes() {
    let engine = Engine::new();

    let keywords = [
        "graph",
        "flowchart",
        "flowchart-elk",
        "style",
        "default",
        "linkStyle",
        "interpolate",
        "classDef",
        "class",
        "href",
        "call",
        "click",
        "_self",
        "_blank",
        "_parent",
        "_top",
        "end",
        "subgraph",
        "kitty",
    ];

    let shapes: [(&str, &str, &str); 14] = [
        ("[", "]", "square"),
        ("(", ")", "round"),
        ("{", "}", "diamond"),
        ("(-", "-)", "ellipse"),
        ("([", "])", "stadium"),
        (">", "]", "odd"),
        ("[(", ")]", "cylinder"),
        ("(((", ")))", "doublecircle"),
        ("[/", "\\]", "trapezoid"),
        ("[\\", "/]", "inv_trapezoid"),
        ("[/", "/]", "lean_right"),
        ("[\\", "\\]", "lean_left"),
        ("[[", "]]", "subroutine"),
        ("{{", "}}", "hexagon"),
    ];

    for keyword in keywords {
        for (open, close, shape) in shapes {
            let text = format!(
                "graph TD;A_{keyword}_node-->B{open}This node has a {keyword} as text{close};"
            );
            let res = block_on(engine.parse_diagram(&text, ParseOptions::default()))
                .unwrap()
                .unwrap();
            let b = res
                .model
                .get("nodes")
                .and_then(|v| v.as_array())
                .unwrap()
                .iter()
                .find(|n| n["id"] == json!("B"))
                .unwrap();
            assert_eq!(b["shape"], json!(shape));
            assert_eq!(
                b["label"],
                json!(format!("This node has a {keyword} as text"))
            );
        }

        let rect_text = format!(
            "graph TD;A_{keyword}_node-->B[|borders:lt|This node has a {keyword} as text];"
        );
        let res = block_on(engine.parse_diagram(&rect_text, ParseOptions::default()))
            .unwrap()
            .unwrap();
        let b = res
            .model
            .get("nodes")
            .and_then(|v| v.as_array())
            .unwrap()
            .iter()
            .find(|n| n["id"] == json!("B"))
            .unwrap();
        assert_eq!(b["shape"], json!("rect"));
        assert_eq!(
            b["label"],
            json!(format!("This node has a {keyword} as text"))
        );
    }
}

#[test]
fn parse_diagram_flowchart_allows_slashes_in_lean_vertices() {
    let engine = Engine::new();

    let res = block_on(engine.parse_diagram(
        "graph TD;A_node-->B[/This node has a / as text/];",
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let b = res.model["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"] == json!("B"))
        .unwrap();
    assert_eq!(b["shape"], json!("lean_right"));
    assert_eq!(b["label"], json!("This node has a / as text"));

    let res = block_on(engine.parse_diagram(
        r#"graph TD;A_node-->B[\This node has a \ as text\];"#,
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let b = res.model["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"] == json!("B"))
        .unwrap();
    assert_eq!(b["shape"], json!("lean_left"));
    assert_eq!(b["label"], json!(r#"This node has a \ as text"#));
}

#[test]
fn parse_diagram_flowchart_misc_vertex_text_cases_from_flow_text_spec() {
    let engine = Engine::new();

    let res = block_on(engine.parse_diagram(
        "graph TD;A-->C{Chimpansen hoppar ???-???};",
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let c = res.model["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"] == json!("C"))
        .unwrap();
    assert_eq!(c["shape"], json!("diamond"));
    assert_eq!(c["label"], json!("Chimpansen hoppar ???-???"));

    let res = block_on(engine.parse_diagram(
        "graph TD;A-->C(Chimpansen hoppar ???  <br> -  ???);",
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let c = res.model["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"] == json!("C"))
        .unwrap();
    assert_eq!(c["shape"], json!("round"));
    assert_eq!(c["label"], json!("Chimpansen hoppar ???  <br> -  ???"));

    let res =
        block_on(engine.parse_diagram("graph TD;A-->C(妖忘折忘抖抉);", ParseOptions::default()))
            .unwrap()
            .unwrap();
    let c = res.model["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"] == json!("C"))
        .unwrap();
    assert_eq!(c["label"], json!("妖忘折忘抖抉"));

    let res =
        block_on(engine.parse_diagram(r#"graph TD;A-->C(c:\windows);"#, ParseOptions::default()))
            .unwrap()
            .unwrap();
    let c = res.model["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"] == json!("C"))
        .unwrap();
    assert_eq!(c["label"], json!(r#"c:\windows"#));
}

#[test]
fn parse_diagram_flowchart_ellipse_vertex_text_and_unterminated_ellipse_errors() {
    let engine = Engine::new();

    let ok = block_on(engine.parse_diagram(
        "graph TD\nA(-this is an ellipse-)-->B",
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let a = ok.model["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"] == json!("A"))
        .unwrap();
    assert_eq!(a["shape"], json!("ellipse"));
    assert_eq!(a["label"], json!("this is an ellipse"));

    let bad = block_on(engine.parse_diagram("graph\nX(- My Text (", ParseOptions::default()));
    assert!(bad.is_err());
}

#[test]
fn parse_diagram_flowchart_question_and_unicode_in_node_and_edge_text() {
    let engine = Engine::new();

    let res = block_on(engine.parse_diagram("graph TD;A(?)-->|?|C;", ParseOptions::default()))
        .unwrap()
        .unwrap();
    let a = res.model["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"] == json!("A"))
        .unwrap();
    assert_eq!(a["label"], json!("?"));
    assert_eq!(res.model["edges"][0]["label"], json!("?"));

    let res = block_on(engine.parse_diagram(
        "graph TD;A(谷豕那角??)-->|谷豕那角??|C;",
        ParseOptions::default(),
    ))
    .unwrap()
    .unwrap();
    let a = res.model["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"] == json!("A"))
        .unwrap();
    assert_eq!(a["label"], json!("谷豕那角??"));
    assert_eq!(res.model["edges"][0]["label"], json!("谷豕那角??"));

    let res = block_on(
        engine.parse_diagram("graph TD;A(,.?!+-*)-->|,.?!+-*|C;", ParseOptions::default()),
    )
    .unwrap()
    .unwrap();
    let a = res.model["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"] == json!("A"))
        .unwrap();
    assert_eq!(a["label"], json!(",.?!+-*"));
    assert_eq!(res.model["edges"][0]["label"], json!(",.?!+-*"));
}

#[test]
fn parse_diagram_flowchart_node_label_invalid_mixed_text_and_quotes_errors() {
    let engine = Engine::new();

    let bad = block_on(engine.parse_diagram(
        "graph TD; A[This is a () in text];",
        ParseOptions::default(),
    ));
    assert!(bad.is_err());

    let bad = block_on(engine.parse_diagram(
        "graph TD;A(this node has \"string\" and text)-->|this link has \"string\" and text|C;",
        ParseOptions::default(),
    ));
    assert!(bad.is_err());

    let bad = block_on(engine.parse_diagram(
        "graph TD; A[This is a \\\"()\\\" in text];",
        ParseOptions::default(),
    ));
    assert!(bad.is_err());

    let bad = block_on(engine.parse_diagram(
        "graph TD; A[\"This is a \"()\" in text\"];",
        ParseOptions::default(),
    ));
    assert!(bad.is_err());

    let bad = block_on(engine.parse_diagram(
        "graph TD; node[hello ) world] --> works",
        ParseOptions::default(),
    ));
    assert!(bad.is_err());
}

#[test]
fn parse_diagram_flowchart_supports_subgraph_block() {
    let engine = Engine::new();
    let text = "graph TD;subgraph S;A-->B;end;";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.meta.diagram_type, "flowchart-v2");
    assert_eq!(
        res.model["subgraphs"],
        json!([{
            "id": "S",
            "nodes": ["B", "A"],
            "title": "S",
            "classes": [],
            "styles": [],
            "dir": null,
            "labelType": "text"
        }])
    );
}

#[test]
fn parse_diagram_flowchart_supports_nested_subgraphs() {
    let engine = Engine::new();
    let text = "graph TD;subgraph Outer;subgraph Inner;A-->B;end;end;";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(
        res.model["subgraphs"],
        json!([{
            "id": "Inner",
            "nodes": ["B", "A"],
            "title": "Inner",
            "classes": [],
            "styles": [],
            "dir": null,
            "labelType": "text"
        }, {
            "id": "Outer",
            "nodes": ["Inner"],
            "title": "Outer",
            "classes": [],
            "styles": [],
            "dir": null,
            "labelType": "text"
        }])
    );
}

#[test]
fn parse_diagram_flowchart_subgraph_supports_explicit_id_and_title() {
    let engine = Engine::new();
    let text = "graph TD;subgraph ide1[one];A-->B;end;";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(
        res.model["subgraphs"],
        json!([{
            "id": "ide1",
            "nodes": ["B", "A"],
            "title": "one",
            "classes": [],
            "styles": [],
            "dir": null,
            "labelType": "text"
        }])
    );
}

#[test]
fn parse_diagram_flowchart_subgraph_title_with_spaces_uses_auto_id() {
    let engine = Engine::new();
    let text = "graph TD;subgraph number as labels;A-->B;end;";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(
        res.model["subgraphs"],
        json!([{
            "id": "subGraph0",
            "nodes": ["B", "A"],
            "title": "number as labels",
            "classes": [],
            "styles": [],
            "dir": null,
            "labelType": "text"
        }])
    );
}

#[test]
fn parse_diagram_flowchart_subgraph_direction_statement_sets_dir() {
    let engine = Engine::new();
    let text = "graph LR;subgraph TOP;direction TB;A-->B;end;";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(
        res.model["subgraphs"],
        json!([{
            "id": "TOP",
            "nodes": ["B", "A"],
            "title": "TOP",
            "classes": [],
            "styles": [],
            "dir": "TB",
            "labelType": "text"
        }])
    );
}

#[test]
fn parse_diagram_flowchart_subgraph_inherits_global_direction_when_enabled() {
    let mut site = MermaidConfig::empty_object();
    site.set_value("flowchart.inheritDir", json!(true));
    let engine = Engine::new().with_site_config(site);
    let text = "graph LR;subgraph TOP;A-->B;end;";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["subgraphs"][0]["dir"], json!("LR"));
}

#[test]
fn parse_diagram_flowchart_subgraph_tab_indentation_matches_mermaid_membership_order() {
    let engine = Engine::new();
    let text = "graph TB\nsubgraph One\n\ta1-->a2\nend";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(
        res.model["subgraphs"],
        json!([{
            "id": "One",
            "nodes": ["a2", "a1"],
            "title": "One",
            "classes": [],
            "styles": [],
            "dir": null,
            "labelType": "text"
        }])
    );
}

#[test]
fn parse_diagram_flowchart_subgraph_chain_membership_order_matches_mermaid() {
    let engine = Engine::new();
    let text = "graph TB\nsubgraph One\n\ta1-->a2-->a3\nend";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(
        res.model["subgraphs"][0]["nodes"],
        json!(["a3", "a2", "a1"])
    );
}

#[test]
fn parse_diagram_flowchart_subgraph_title_with_spaces_in_quotes_uses_auto_id() {
    let engine = Engine::new();
    let text = "graph TB\nsubgraph \"Some Title\"\n\ta1-->a2\nend";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["subgraphs"][0]["title"], json!("Some Title"));
    assert_eq!(res.model["subgraphs"][0]["id"], json!("subGraph0"));
}

#[test]
fn parse_diagram_flowchart_subgraph_id_and_title_notation() {
    let engine = Engine::new();
    let text = "graph TB\nsubgraph some-id[Some Title]\n\ta1-->a2\nend";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["subgraphs"][0]["id"], json!("some-id"));
    assert_eq!(res.model["subgraphs"][0]["title"], json!("Some Title"));
    assert_eq!(res.model["subgraphs"][0]["labelType"], json!("text"));
}

#[test]
fn parse_diagram_flowchart_subgraph_bracket_quoted_title_sets_label_type_string() {
    let engine = Engine::new();
    let text = "graph TD;subgraph uid2[\"text of doom\"];c-->d;end;";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["subgraphs"][0]["id"], json!("uid2"));
    assert_eq!(res.model["subgraphs"][0]["title"], json!("text of doom"));
    assert_eq!(res.model["subgraphs"][0]["labelType"], json!("string"));
}

#[test]
fn parse_diagram_flowchart_subgraph_markdown_title_sets_label_type_markdown() {
    let engine = Engine::new();
    let text = "graph TD\nsubgraph \"`**Two**`\"\nA-->B\nend";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["subgraphs"][0]["title"], json!("**Two**"));
    assert_eq!(res.model["subgraphs"][0]["labelType"], json!("markdown"));
}

#[test]
fn parse_diagram_flowchart_duplicate_subgraph_membership_matches_mermaid_makeuniq() {
    let engine = Engine::new();
    let text = "graph TD\nsubgraph A\nB\nend\nsubgraph X\nB\nend\nB-->C\n";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();

    assert_eq!(
        res.model["subgraphs"],
        json!([{
            "id": "A",
            "nodes": ["B"],
            "title": "A",
            "classes": [],
            "styles": [],
            "dir": null,
            "labelType": "text"
        }, {
            "id": "X",
            "nodes": [],
            "title": "X",
            "classes": [],
            "styles": [],
            "dir": null,
            "labelType": "text"
        }])
    );
}

#[test]
fn parse_diagram_flowchart_subgraph_supports_amp_group_syntax_minimally() {
    let engine = Engine::new();
    let text = "graph TD\nsubgraph myTitle\na & b --> c & e\nend";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let nodes = res.model["subgraphs"][0]["nodes"].as_array().unwrap();
    let as_set: std::collections::HashSet<String> = nodes
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();
    assert!(as_set.contains("a"));
    assert!(as_set.contains("b"));
    assert!(as_set.contains("c"));
    assert!(as_set.contains("e"));
}

#[test]
fn parse_diagram_flowchart_style_statement_applies_vertex_styles() {
    let engine = Engine::new();
    let text = "graph TD;style Q background:#fff;";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let q = res.model["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"] == json!("Q"))
        .unwrap();
    assert_eq!(q["styles"], json!(["background:#fff"]));
}

#[test]
fn parse_diagram_flowchart_classdef_and_class_assign_work() {
    let engine = Engine::new();
    let text =
        "graph TD;classDef exClass background:#bbb,border:1px solid red;a-->b;class a,b exClass;";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(
        res.model["classDefs"]["exClass"],
        json!(["background:#bbb", "border:1px solid red"])
    );
    let a = res.model["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"] == json!("a"))
        .unwrap();
    let b = res.model["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"] == json!("b"))
        .unwrap();
    assert_eq!(a["classes"][0], json!("exClass"));
    assert_eq!(b["classes"][0], json!("exClass"));
}

#[test]
fn parse_diagram_flowchart_inline_vertex_class_via_style_separator() {
    let engine = Engine::new();
    // Mermaid `encodeEntities(...)` treats `#bbb;` as an entity placeholder when semicolons
    // are used as statement separators. Use newlines to match upstream parsing behavior.
    let text = "graph TD\nclassDef exClass background:#bbb\nA-->B[test]:::exClass\n";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let b = res.model["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"] == json!("B"))
        .unwrap();
    assert_eq!(b["classes"][0], json!("exClass"));
}

#[test]
fn parse_diagram_flowchart_linkstyle_applies_edge_style_and_validates_bounds() {
    let engine = Engine::new();
    let ok = "graph TD\nA-->B\nlinkStyle 0 stroke-width:1px;";
    let res = block_on(engine.parse_diagram(ok, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["edges"][0]["style"][0], json!("stroke-width:1px"));

    let bad = "graph TD\nA-->B\nlinkStyle 1 stroke-width:1px;";
    let err = block_on(engine.parse_diagram(bad, ParseOptions::default())).unwrap_err();
    assert_eq!(
        err.to_string(),
        "Diagram parse error (flowchart-v2): The index 1 for linkStyle is out of bounds. Valid indices for linkStyle are between 0 and 0. (Help: Ensure that the index is within the range of existing edges.)"
    );
}

#[test]
fn parse_diagram_flowchart_linkstyle_default_interpolate_sets_edge_defaults() {
    let engine = Engine::new();
    let text = "graph TD\nA-->B\nlinkStyle default interpolate basis";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["edgeDefaults"]["interpolate"], json!("basis"));
}

#[test]
fn parse_diagram_flowchart_linkstyle_numbered_interpolate_sets_edges() {
    let engine = Engine::new();
    let text =
        "graph TD\nA-->B\nA-->C\nlinkStyle 0 interpolate basis\nlinkStyle 1 interpolate cardinal";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["edges"][0]["interpolate"], json!("basis"));
    assert_eq!(res.model["edges"][1]["interpolate"], json!("cardinal"));
}

#[test]
fn parse_diagram_flowchart_linkstyle_multi_numbered_interpolate_sets_edges() {
    let engine = Engine::new();
    let text = "graph TD\nA-->B\nA-->C\nlinkStyle 0,1 interpolate basis";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["edges"][0]["interpolate"], json!("basis"));
    assert_eq!(res.model["edges"][1]["interpolate"], json!("basis"));
}

#[test]
fn parse_diagram_flowchart_edge_curve_properties_using_edge_id() {
    let engine = Engine::new();
    let text =
        "graph TD\nA e1@-->B\nA uniqueName@-->C\ne1@{curve: basis}\nuniqueName@{curve: cardinal}";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["edges"][0]["id"], json!("e1"));
    assert_eq!(res.model["edges"][1]["id"], json!("uniqueName"));
    assert_eq!(res.model["edges"][0]["interpolate"], json!("basis"));
    assert_eq!(res.model["edges"][1]["interpolate"], json!("cardinal"));
}

#[test]
fn parse_diagram_flowchart_edge_curve_properties_does_not_override_default() {
    let engine = Engine::new();
    let text =
        "graph TD\nA e1@-->B\nA-->C\nlinkStyle default interpolate linear\ne1@{curve: stepAfter}";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["edgeDefaults"]["interpolate"], json!("linear"));
    assert_eq!(res.model["edges"][0]["interpolate"], json!("stepAfter"));
}

#[test]
fn parse_diagram_flowchart_edge_curve_properties_mixed_with_line_interpolation() {
    let engine = Engine::new();
    let text = "graph TD\nA e1@-->B-->D\nA-->C e4@-->D-->E\nlinkStyle default interpolate linear\nlinkStyle 1 interpolate basis\ne1@{curve: monotoneX}\ne4@{curve: stepBefore}";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.model["edgeDefaults"]["interpolate"], json!("linear"));
    assert_eq!(res.model["edges"][0]["interpolate"], json!("monotoneX"));
    assert_eq!(res.model["edges"][1]["interpolate"], json!("basis"));
    assert_eq!(res.model["edges"][3]["interpolate"], json!("stepBefore"));
}

#[test]
fn parse_diagram_flowchart_click_link_sets_link_and_tooltip_and_clickable_class() {
    let engine = Engine::new();
    let text = "graph TD\nA-->B\nclick A href \"click.html\" \"tooltip\" _blank";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let a = res.model["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"] == json!("A"))
        .unwrap();
    assert_eq!(a["link"], json!("click.html"));
    assert_eq!(a["linkTarget"], json!("_blank"));
    assert_eq!(res.model["tooltips"]["A"], json!("tooltip"));
    assert_eq!(a["classes"][0], json!("clickable"));
}

#[test]
fn parse_diagram_flowchart_click_link_sanitizes_javascript_urls_when_not_loose() {
    let engine = Engine::new();
    let text = "graph TD\nA-->B\nclick A href \"javascript:alert(1)\" \"tooltip\" _blank";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let a = res.model["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"] == json!("A"))
        .unwrap();
    assert_eq!(a["link"], json!("about:blank"));
    assert_eq!(a["linkTarget"], json!("_blank"));
}

#[test]
fn parse_diagram_flowchart_style_statement_supports_multiple_styles() {
    let engine = Engine::new();
    let text = "graph TD;style R background:#fff,border:1px solid red;";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let r = res.model["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["id"] == json!("R"))
        .unwrap();
    assert_eq!(
        r["styles"],
        json!(["background:#fff", "border:1px solid red"])
    );
}

#[test]
fn parse_diagram_flowchart_classdef_supports_multiple_classes() {
    let engine = Engine::new();
    let text = "graph TD;classDef firstClass,secondClass background:#bbb,border:1px solid red;";
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(
        res.model["classDefs"]["firstClass"],
        json!(["background:#bbb", "border:1px solid red"])
    );
    assert_eq!(
        res.model["classDefs"]["secondClass"],
        json!(["background:#bbb", "border:1px solid red"])
    );
}

#[test]
fn parse_diagram_flowchart_inline_vertex_class_in_groups_matches_mermaid_style_spec() {
    let engine = Engine::new();
    let text = r#"
graph TD
  classDef C1 stroke-dasharray:4
  classDef C2 stroke-dasharray:6
  A & B:::C1 & D:::C1 --> E:::C2
"#;
    let res = block_on(engine.parse_diagram(text, ParseOptions::default()))
        .unwrap()
        .unwrap();
    let find = |id: &str| {
        res.model["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .find(|n| n["id"] == json!(id))
            .unwrap()
            .clone()
    };
    assert!(find("A")["classes"].as_array().unwrap().is_empty());
    assert_eq!(find("B")["classes"][0], json!("C1"));
    assert_eq!(find("D")["classes"][0], json!("C1"));
    assert_eq!(find("E")["classes"][0], json!("C2"));
}

#[test]
fn parse_diagram_flowchart_keyword_flowchart() {
    let engine = Engine::new();
    let res = block_on(engine.parse_diagram("flowchart TD\nA-->B", ParseOptions::default()))
        .unwrap()
        .unwrap();
    assert_eq!(res.meta.diagram_type, "flowchart-v2");
    assert_eq!(res.model["keyword"], json!("flowchart"));
    assert_eq!(res.model["direction"], json!("TB"));
    assert_eq!(res.model["subgraphs"], json!([]));
}
