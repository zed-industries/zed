use dugong_graphlib::json::{
    GraphJson, GraphJsonEdge, read, read_with_defaults, write, write_with_defaults,
};
use dugong_graphlib::{Graph, GraphOptions};
use serde_json::json;

#[test]
fn json_preserves_graph_options() {
    let directed: Graph<Option<()>, Option<()>, Option<()>> = Graph::new(GraphOptions {
        directed: true,
        ..Default::default()
    });
    let undirected: Graph<Option<()>, Option<()>, Option<()>> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    let multigraph: Graph<Option<()>, Option<()>, Option<()>> = Graph::new(GraphOptions {
        multigraph: true,
        ..Default::default()
    });
    let compound: Graph<Option<()>, Option<()>, Option<()>> = Graph::new(GraphOptions {
        compound: true,
        ..Default::default()
    });

    let directed =
        read::<(), (), ()>(&write(&directed).expect("serialize graph")).expect("deserialize graph");
    let undirected = read::<(), (), ()>(&write(&undirected).expect("serialize graph"))
        .expect("deserialize graph");
    let multigraph = read::<(), (), ()>(&write(&multigraph).expect("serialize graph"))
        .expect("deserialize graph");
    let compound =
        read::<(), (), ()>(&write(&compound).expect("serialize graph")).expect("deserialize graph");

    assert!(directed.is_directed());
    assert!(!undirected.is_directed());
    assert!(multigraph.is_multigraph());
    assert!(compound.is_compound());
}

#[test]
fn json_preserves_graph_value_if_any() {
    let mut number: Graph<Option<()>, Option<()>, Option<serde_json::Value>> =
        Graph::new(GraphOptions::default());
    number.set_graph(Some(json!(1)));
    let mut object: Graph<Option<()>, Option<()>, Option<serde_json::Value>> =
        Graph::new(GraphOptions::default());
    object.set_graph(Some(json!({ "foo": "bar" })));
    let empty: Graph<Option<()>, Option<()>, Option<serde_json::Value>> =
        Graph::new(GraphOptions::default());

    let number = read::<(), (), serde_json::Value>(&write(&number).expect("serialize graph"))
        .expect("deserialize graph");
    let object = read::<(), (), serde_json::Value>(&write(&object).expect("serialize graph"))
        .expect("deserialize graph");
    let empty = read::<(), (), serde_json::Value>(&write(&empty).expect("serialize graph"))
        .expect("deserialize graph");

    assert_eq!(number.graph(), &Some(json!(1)));
    assert_eq!(object.graph(), &Some(json!({ "foo": "bar" })));
    assert_eq!(empty.graph(), &None);
}

#[test]
fn json_preserves_nodes() {
    let mut missing: Graph<Option<serde_json::Value>, Option<()>, Option<()>> =
        Graph::new(GraphOptions::default());
    missing.ensure_node("a");

    let mut number: Graph<Option<serde_json::Value>, Option<()>, Option<()>> =
        Graph::new(GraphOptions::default());
    number.set_node("a", Some(json!(1)));

    let mut object: Graph<Option<serde_json::Value>, Option<()>, Option<()>> =
        Graph::new(GraphOptions::default());
    object.set_node("a", Some(json!({ "foo": "bar" })));

    let missing = read::<serde_json::Value, (), ()>(&write(&missing).expect("serialize graph"))
        .expect("deserialize graph");
    let number = read::<serde_json::Value, (), ()>(&write(&number).expect("serialize graph"))
        .expect("deserialize graph");
    let object = read::<serde_json::Value, (), ()>(&write(&object).expect("serialize graph"))
        .expect("deserialize graph");

    assert!(missing.has_node("a"));
    assert_eq!(missing.node("a"), Some(&None));
    assert_eq!(number.node("a"), Some(&Some(json!(1))));
    assert_eq!(object.node("a"), Some(&Some(json!({ "foo": "bar" }))));
}

#[test]
fn json_preserves_simple_edges() {
    let mut missing: Graph<Option<()>, Option<serde_json::Value>, Option<()>> =
        Graph::new(GraphOptions::default());
    missing.set_edge("a", "b");

    let mut number: Graph<Option<()>, Option<serde_json::Value>, Option<()>> =
        Graph::new(GraphOptions::default());
    number.set_edge_with_label("a", "b", Some(json!(1)));

    let mut object: Graph<Option<()>, Option<serde_json::Value>, Option<()>> =
        Graph::new(GraphOptions::default());
    object.set_edge_with_label("a", "b", Some(json!({ "foo": "bar" })));

    let missing = read::<(), serde_json::Value, ()>(&write(&missing).expect("serialize graph"))
        .expect("deserialize graph");
    let number = read::<(), serde_json::Value, ()>(&write(&number).expect("serialize graph"))
        .expect("deserialize graph");
    let object = read::<(), serde_json::Value, ()>(&write(&object).expect("serialize graph"))
        .expect("deserialize graph");

    assert!(missing.has_edge("a", "b", None));
    assert_eq!(missing.edge("a", "b", None), Some(&None));
    assert_eq!(number.edge("a", "b", None), Some(&Some(json!(1))));
    assert_eq!(
        object.edge("a", "b", None),
        Some(&Some(json!({ "foo": "bar" })))
    );
}

#[test]
fn json_preserves_multi_edges() {
    let mut missing: Graph<Option<()>, Option<serde_json::Value>, Option<()>> =
        Graph::new(GraphOptions {
            multigraph: true,
            ..Default::default()
        });
    missing.set_edge_named("a", "b", Some("foo"), None::<Option<serde_json::Value>>);

    let mut number: Graph<Option<()>, Option<serde_json::Value>, Option<()>> =
        Graph::new(GraphOptions {
            multigraph: true,
            ..Default::default()
        });
    number.set_edge_named("a", "b", Some("foo"), Some(Some(json!(1))));

    let mut object: Graph<Option<()>, Option<serde_json::Value>, Option<()>> =
        Graph::new(GraphOptions {
            multigraph: true,
            ..Default::default()
        });
    object.set_edge_named("a", "b", Some("foo"), Some(Some(json!({ "foo": "bar" }))));

    let missing = read::<(), serde_json::Value, ()>(&write(&missing).expect("serialize graph"))
        .expect("deserialize graph");
    let number = read::<(), serde_json::Value, ()>(&write(&number).expect("serialize graph"))
        .expect("deserialize graph");
    let object = read::<(), serde_json::Value, ()>(&write(&object).expect("serialize graph"))
        .expect("deserialize graph");

    assert!(missing.has_edge("a", "b", Some("foo")));
    assert_eq!(missing.edge("a", "b", Some("foo")), Some(&None));
    assert_eq!(number.edge("a", "b", Some("foo")), Some(&Some(json!(1))));
    assert_eq!(
        object.edge("a", "b", Some("foo")),
        Some(&Some(json!({ "foo": "bar" })))
    );
}

#[test]
fn json_rejects_named_edge_in_non_multigraph_input() {
    let json = GraphJson {
        options: GraphOptions::default(),
        value: None,
        nodes: Vec::new(),
        edges: vec![GraphJsonEdge {
            v: "a".to_string(),
            w: "b".to_string(),
            name: Some("foo".to_string()),
            value: None,
        }],
    };

    let err = match read::<(), (), ()>(&json) {
        Ok(_) => panic!("simple graph named edge should fail"),
        Err(err) => err,
    };

    assert!(err.to_string().contains("is_multigraph = false"));
}

#[test]
fn json_preserves_parent_child_relationships() {
    let mut root_only: Graph<Option<()>, Option<()>, Option<()>> = Graph::new(GraphOptions {
        compound: true,
        ..Default::default()
    });
    root_only.ensure_node("a");

    let mut parented: Graph<Option<()>, Option<()>, Option<()>> = Graph::new(GraphOptions {
        compound: true,
        ..Default::default()
    });
    parented.set_parent("a", "parent");

    let root_only = read::<(), (), ()>(&write(&root_only).expect("serialize graph"))
        .expect("deserialize graph");
    let parented =
        read::<(), (), ()>(&write(&parented).expect("serialize graph")).expect("deserialize graph");

    assert_eq!(root_only.parent("a"), None);
    assert_eq!(parented.parent("a"), Some("parent"));
}

#[test]
fn json_distinguishes_undefined_from_explicit_null_for_option_labels() {
    let mut graph: Graph<
        Option<serde_json::Value>,
        Option<serde_json::Value>,
        Option<serde_json::Value>,
    > = Graph::new(GraphOptions {
        multigraph: true,
        compound: true,
        ..Default::default()
    });
    graph.set_graph(Some(serde_json::Value::Null));
    graph.set_node("a", Some(serde_json::Value::Null));
    graph.set_node("b", None);
    graph.set_parent("a", "parent");
    graph.set_edge_named(
        "a",
        "b",
        Some("explicit-null"),
        Some(Some(serde_json::Value::Null)),
    );
    graph.set_edge_named("b", "a", Some("undefined"), Some(None));

    let written = write(&graph).expect("serialize graph");
    let roundtrip = read::<serde_json::Value, serde_json::Value, serde_json::Value>(&written)
        .expect("deserialize graph");

    assert_eq!(written.value, Some(serde_json::Value::Null));
    assert_eq!(
        written
            .nodes
            .iter()
            .find(|node| node.v == "a")
            .and_then(|node| node.value.clone()),
        Some(serde_json::Value::Null)
    );
    assert_eq!(
        written
            .nodes
            .iter()
            .find(|node| node.v == "b")
            .and_then(|node| node.value.clone()),
        None
    );
    assert_eq!(
        written
            .edges
            .iter()
            .find(|edge| edge.name.as_deref() == Some("explicit-null"))
            .and_then(|edge| edge.value.clone()),
        Some(serde_json::Value::Null)
    );
    assert_eq!(
        written
            .edges
            .iter()
            .find(|edge| edge.name.as_deref() == Some("undefined"))
            .and_then(|edge| edge.value.clone()),
        None
    );

    assert_eq!(roundtrip.graph(), &Some(serde_json::Value::Null));
    assert_eq!(roundtrip.node("a"), Some(&Some(serde_json::Value::Null)));
    assert_eq!(roundtrip.node("b"), Some(&None));
    assert_eq!(
        roundtrip.edge("a", "b", Some("explicit-null")),
        Some(&Some(serde_json::Value::Null))
    );
    assert_eq!(roundtrip.edge("b", "a", Some("undefined")), Some(&None));
}

#[test]
fn json_with_defaults_can_collapse_missing_values_to_rust_defaults() {
    let mut graph: Graph<serde_json::Value, serde_json::Value, serde_json::Value> =
        Graph::new(GraphOptions {
            multigraph: true,
            compound: true,
            ..Default::default()
        });
    graph.set_graph(serde_json::Value::Null);
    graph.set_node("a", serde_json::Value::Null);
    graph.set_edge_with_label("a", "b", serde_json::Value::Null);

    let written = write_with_defaults(&graph).expect("serialize graph");
    let roundtrip =
        read_with_defaults::<serde_json::Value, serde_json::Value, serde_json::Value>(&written)
            .expect("deserialize graph");

    assert_eq!(written.value, None);
    assert_eq!(
        written
            .nodes
            .iter()
            .find(|node| node.v == "a")
            .and_then(|node| node.value.clone()),
        None
    );
    assert_eq!(roundtrip.graph(), &serde_json::Value::Null);
    assert_eq!(roundtrip.node("a"), Some(&serde_json::Value::Null));
    assert_eq!(
        roundtrip.edge("a", "b", None),
        Some(&serde_json::Value::Null)
    );
}
