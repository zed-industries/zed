use dugong::graphlib::{Graph, GraphOptions};
use dugong::normalize;
use dugong::{EdgeLabel, GraphLabel, NodeLabel, Point};
use serde_json::Value;

fn new_graph() -> Graph<NodeLabel, EdgeLabel, GraphLabel> {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: true,
        compound: true,
        ..Default::default()
    });
    g.set_graph(GraphLabel::default());
    g
}

fn edge_incident_nodes(
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
) -> Vec<(String, String, Option<String>)> {
    g.edges()
        .map(|e| (e.v.clone(), e.w.clone(), e.name.clone()))
        .collect()
}

#[test]
fn normalize_run_does_not_change_a_short_edge() {
    let mut g = new_graph();
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(0),
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            rank: Some(1),
            ..Default::default()
        },
    );
    g.set_edge_with_label("a", "b", EdgeLabel::default());

    normalize::run(&mut g);

    assert_eq!(
        edge_incident_nodes(&g),
        vec![("a".to_string(), "b".to_string(), None)]
    );
    assert_eq!(g.node("a").unwrap().rank, Some(0));
    assert_eq!(g.node("b").unwrap().rank, Some(1));
}

#[test]
fn normalize_run_splits_a_two_layer_edge_into_two_segments() {
    let mut g = new_graph();
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(0),
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            rank: Some(2),
            ..Default::default()
        },
    );
    g.set_edge_with_label("a", "b", EdgeLabel::default());

    normalize::run(&mut g);

    let sucs = g.successors("a");
    assert_eq!(sucs.len(), 1);
    let successor = sucs[0].to_string();
    assert_eq!(g.node(&successor).unwrap().dummy.as_deref(), Some("edge"));
    assert_eq!(g.node(&successor).unwrap().rank, Some(1));
    assert_eq!(g.successors(&successor), vec!["b"]);
    assert_eq!(g.node("a").unwrap().rank, Some(0));
    assert_eq!(g.node("b").unwrap().rank, Some(2));

    assert_eq!(g.graph().dummy_chains.len(), 1);
    assert_eq!(g.graph().dummy_chains[0], successor);
}

#[test]
fn normalize_run_assigns_width_and_height_0_to_dummy_nodes_by_default() {
    let mut g = new_graph();
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(0),
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            rank: Some(2),
            ..Default::default()
        },
    );
    g.set_edge_with_label(
        "a",
        "b",
        EdgeLabel {
            width: 10.0,
            height: 10.0,
            ..Default::default()
        },
    );

    normalize::run(&mut g);

    let successor = g.successors("a")[0];
    assert_eq!(g.node(successor).unwrap().width, 0.0);
    assert_eq!(g.node(successor).unwrap().height, 0.0);
}

#[test]
fn normalize_run_assigns_width_and_height_from_the_edge_for_the_node_on_label_rank() {
    let mut g = new_graph();
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(0),
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            rank: Some(4),
            ..Default::default()
        },
    );
    g.set_edge_with_label(
        "a",
        "b",
        EdgeLabel {
            width: 20.0,
            height: 10.0,
            label_rank: Some(2),
            ..Default::default()
        },
    );

    normalize::run(&mut g);

    let label_v = g.successors(g.successors("a")[0])[0];
    let label_node = g.node(label_v).unwrap();
    assert_eq!(label_node.width, 20.0);
    assert_eq!(label_node.height, 10.0);
}

#[test]
fn normalize_run_preserves_the_weight_for_the_edge() {
    let mut g = new_graph();
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(0),
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            rank: Some(2),
            ..Default::default()
        },
    );
    g.set_edge_with_label(
        "a",
        "b",
        EdgeLabel {
            weight: 2.0,
            ..Default::default()
        },
    );

    normalize::run(&mut g);

    let successor = g.successors("a")[0];
    assert_eq!(g.edge("a", successor, None).unwrap().weight, 2.0);
}

#[test]
fn normalize_undo_reverses_the_run_operation() {
    let mut g = new_graph();
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(0),
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            rank: Some(2),
            ..Default::default()
        },
    );
    g.set_edge_with_label("a", "b", EdgeLabel::default());

    normalize::run(&mut g);
    normalize::undo(&mut g);

    assert_eq!(
        edge_incident_nodes(&g),
        vec![("a".to_string(), "b".to_string(), None)]
    );
    assert_eq!(g.node("a").unwrap().rank, Some(0));
    assert_eq!(g.node("b").unwrap().rank, Some(2));
}

#[test]
fn normalize_undo_restores_previous_edge_labels() {
    let mut g = new_graph();
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(0),
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            rank: Some(2),
            ..Default::default()
        },
    );
    let mut label = EdgeLabel::default();
    label
        .extras
        .insert("foo".to_string(), Value::String("bar".to_string()));
    g.set_edge_with_label("a", "b", label);

    normalize::run(&mut g);
    normalize::undo(&mut g);

    assert_eq!(
        g.edge("a", "b", None).unwrap().extras.get("foo").cloned(),
        Some(Value::String("bar".to_string()))
    );
}

#[test]
fn normalize_undo_collects_assigned_coordinates_into_points() {
    let mut g = new_graph();
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(0),
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            rank: Some(2),
            ..Default::default()
        },
    );
    g.set_edge_with_label("a", "b", EdgeLabel::default());

    normalize::run(&mut g);

    let dummy = g.neighbors("a")[0].to_string();
    let dummy_label = g.node_mut(&dummy).unwrap();
    dummy_label.x = Some(5.0);
    dummy_label.y = Some(10.0);

    normalize::undo(&mut g);

    assert_eq!(
        g.edge("a", "b", None).unwrap().points,
        vec![Point { x: 5.0, y: 10.0 }]
    );
}

#[test]
fn normalize_undo_merges_assigned_coordinates_into_points() {
    let mut g = new_graph();
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(0),
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            rank: Some(4),
            ..Default::default()
        },
    );
    g.set_edge_with_label("a", "b", EdgeLabel::default());

    normalize::run(&mut g);

    let a_suc = g.neighbors("a")[0].to_string();
    let a_suc_label = g.node_mut(&a_suc).unwrap();
    a_suc_label.x = Some(5.0);
    a_suc_label.y = Some(10.0);

    let mid = g.successors(g.successors("a")[0])[0].to_string();
    let mid_label = g.node_mut(&mid).unwrap();
    mid_label.x = Some(20.0);
    mid_label.y = Some(25.0);

    let b_pred = g.neighbors("b")[0].to_string();
    let b_pred_label = g.node_mut(&b_pred).unwrap();
    b_pred_label.x = Some(100.0);
    b_pred_label.y = Some(200.0);

    normalize::undo(&mut g);

    assert_eq!(
        g.edge("a", "b", None).unwrap().points,
        vec![
            Point { x: 5.0, y: 10.0 },
            Point { x: 20.0, y: 25.0 },
            Point { x: 100.0, y: 200.0 },
        ]
    );
}

#[test]
fn normalize_undo_sets_coords_and_dims_for_the_label_if_the_edge_has_one() {
    let mut g = new_graph();
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(0),
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            rank: Some(2),
            ..Default::default()
        },
    );
    g.set_edge_with_label(
        "a",
        "b",
        EdgeLabel {
            width: 10.0,
            height: 20.0,
            label_rank: Some(1),
            ..Default::default()
        },
    );

    normalize::run(&mut g);

    let label_v = g.successors("a")[0].to_string();
    let label_node = g.node_mut(&label_v).unwrap();
    label_node.x = Some(50.0);
    label_node.y = Some(60.0);
    label_node.width = 20.0;
    label_node.height = 10.0;

    normalize::undo(&mut g);

    let e = g.edge("a", "b", None).unwrap();
    assert_eq!(e.x, Some(50.0));
    assert_eq!(e.y, Some(60.0));
    assert_eq!(e.width, 20.0);
    assert_eq!(e.height, 10.0);
}

#[test]
fn normalize_undo_sets_coords_and_dims_for_the_label_if_the_long_edge_has_one() {
    let mut g = new_graph();
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(0),
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            rank: Some(4),
            ..Default::default()
        },
    );
    g.set_edge_with_label(
        "a",
        "b",
        EdgeLabel {
            width: 10.0,
            height: 20.0,
            label_rank: Some(2),
            ..Default::default()
        },
    );

    normalize::run(&mut g);

    let label_v = g.successors(g.successors("a")[0])[0].to_string();
    let label_node = g.node_mut(&label_v).unwrap();
    label_node.x = Some(50.0);
    label_node.y = Some(60.0);
    label_node.width = 20.0;
    label_node.height = 10.0;

    normalize::undo(&mut g);

    let e = g.edge("a", "b", None).unwrap();
    assert_eq!(e.x, Some(50.0));
    assert_eq!(e.y, Some(60.0));
    assert_eq!(e.width, 20.0);
    assert_eq!(e.height, 10.0);
}

#[test]
fn normalize_undo_restores_multi_edges() {
    let mut g = new_graph();
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(0),
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            rank: Some(2),
            ..Default::default()
        },
    );
    g.set_edge_named("a", "b", Some("bar"), Some(EdgeLabel::default()));
    g.set_edge_named("a", "b", Some("foo"), Some(EdgeLabel::default()));

    normalize::run(&mut g);

    let mut out_edges = g.out_edges("a", None);
    out_edges.sort_by_key(|e| e.name.clone().unwrap_or_default());
    assert_eq!(out_edges.len(), 2);

    let bar_dummy = out_edges[0].w.clone();
    let bar_label = g.node_mut(&bar_dummy).unwrap();
    bar_label.x = Some(5.0);
    bar_label.y = Some(10.0);

    let foo_dummy = out_edges[1].w.clone();
    let foo_label = g.node_mut(&foo_dummy).unwrap();
    foo_label.x = Some(15.0);
    foo_label.y = Some(20.0);

    normalize::undo(&mut g);

    assert!(!g.has_edge("a", "b", None));
    assert_eq!(
        g.edge("a", "b", Some("bar")).unwrap().points,
        vec![Point { x: 5.0, y: 10.0 }]
    );
    assert_eq!(
        g.edge("a", "b", Some("foo")).unwrap().points,
        vec![Point { x: 15.0, y: 20.0 }]
    );
}
