use dugong::NodeLabel;
use dugong::graphlib::{Graph, GraphOptions};
use dugong::order::{BarycenterEntry, WeightLabel, barycenter};

fn new_graph() -> Graph<NodeLabel, WeightLabel, ()> {
    let mut g: Graph<NodeLabel, WeightLabel, ()> = Graph::new(GraphOptions::default());
    g.set_default_node_label(NodeLabel::default);
    g.set_default_edge_label(|| WeightLabel { weight: 1.0 });
    g
}

#[test]
fn barycenter_assigns_an_undefined_barycenter_for_a_node_with_no_predecessors() {
    let mut g = new_graph();
    g.set_node("x", NodeLabel::default());

    let results = barycenter(&g, &["x".to_string()]);
    assert_eq!(
        results,
        vec![BarycenterEntry {
            v: "x".to_string(),
            barycenter: None,
            weight: None
        }]
    );
}

#[test]
fn barycenter_assigns_the_position_of_the_sole_predecessors() {
    let mut g = new_graph();
    g.set_node(
        "a",
        NodeLabel {
            order: Some(2),
            ..Default::default()
        },
    );
    g.set_edge("a", "x");

    let results = barycenter(&g, &["x".to_string()]);
    assert_eq!(
        results,
        vec![BarycenterEntry {
            v: "x".to_string(),
            barycenter: Some(2.0),
            weight: Some(1.0)
        }]
    );
}

#[test]
fn barycenter_assigns_the_average_of_multiple_predecessors() {
    let mut g = new_graph();
    g.set_node(
        "a",
        NodeLabel {
            order: Some(2),
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            order: Some(4),
            ..Default::default()
        },
    );
    g.set_edge("a", "x");
    g.set_edge("b", "x");

    let results = barycenter(&g, &["x".to_string()]);
    assert_eq!(
        results,
        vec![BarycenterEntry {
            v: "x".to_string(),
            barycenter: Some(3.0),
            weight: Some(2.0)
        }]
    );
}

#[test]
fn barycenter_takes_into_account_the_weight_of_edges() {
    let mut g = new_graph();
    g.set_node(
        "a",
        NodeLabel {
            order: Some(2),
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            order: Some(4),
            ..Default::default()
        },
    );
    g.set_edge_with_label("a", "x", WeightLabel { weight: 3.0 });
    g.set_edge("b", "x");

    let results = barycenter(&g, &["x".to_string()]);
    assert_eq!(
        results,
        vec![BarycenterEntry {
            v: "x".to_string(),
            barycenter: Some(2.5),
            weight: Some(4.0)
        }]
    );
}

#[test]
fn barycenter_calculates_barycenters_for_all_nodes_in_the_movable_layer() {
    let mut g = new_graph();
    g.set_node(
        "a",
        NodeLabel {
            order: Some(1),
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            order: Some(2),
            ..Default::default()
        },
    );
    g.set_node(
        "c",
        NodeLabel {
            order: Some(4),
            ..Default::default()
        },
    );
    g.set_edge("a", "x");
    g.set_edge("b", "x");
    g.ensure_node("y");
    g.set_edge_with_label("a", "z", WeightLabel { weight: 2.0 });
    g.set_edge("c", "z");

    let results = barycenter(&g, &["x".to_string(), "y".to_string(), "z".to_string()]);
    assert_eq!(
        results,
        vec![
            BarycenterEntry {
                v: "x".to_string(),
                barycenter: Some(1.5),
                weight: Some(2.0)
            },
            BarycenterEntry {
                v: "y".to_string(),
                barycenter: None,
                weight: None
            },
            BarycenterEntry {
                v: "z".to_string(),
                barycenter: Some(2.0),
                weight: Some(3.0)
            }
        ]
    );
}
