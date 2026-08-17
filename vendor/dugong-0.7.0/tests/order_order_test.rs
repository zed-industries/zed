use dugong::NodeLabel;
use dugong::graphlib::{Graph, GraphOptions};
use dugong::order::{OrderOptions, WeightLabel, cross_count, order};
use dugong::util;

fn new_graph() -> Graph<NodeLabel, WeightLabel, ()> {
    let mut g: Graph<NodeLabel, WeightLabel, ()> = Graph::new(GraphOptions::default());
    g.set_default_edge_label(|| WeightLabel { weight: 1.0 });
    g
}

fn set_path(g: &mut Graph<NodeLabel, WeightLabel, ()>, path: &[&str]) {
    for w in path.windows(2) {
        g.set_edge(w[0], w[1]);
    }
}

#[test]
fn order_does_not_add_crossings_to_a_tree_structure() {
    let mut g = new_graph();
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(1),
            ..Default::default()
        },
    );
    for v in ["b", "e"] {
        g.set_node(
            v,
            NodeLabel {
                rank: Some(2),
                ..Default::default()
            },
        );
    }
    for v in ["c", "d", "f"] {
        g.set_node(
            v,
            NodeLabel {
                rank: Some(3),
                ..Default::default()
            },
        );
    }
    set_path(&mut g, &["a", "b", "c"]);
    g.set_edge("b", "d");
    set_path(&mut g, &["a", "e", "f"]);

    order(&mut g, OrderOptions::default());
    let layering = util::build_layer_matrix(&g);
    assert_eq!(cross_count(&g, &layering), 0.0);
}

#[test]
fn order_can_solve_a_simple_graph() {
    let mut g = new_graph();
    for v in ["a", "d"] {
        g.set_node(
            v,
            NodeLabel {
                rank: Some(1),
                ..Default::default()
            },
        );
    }
    for v in ["b", "f", "e"] {
        g.set_node(
            v,
            NodeLabel {
                rank: Some(2),
                ..Default::default()
            },
        );
    }
    for v in ["c", "g"] {
        g.set_node(
            v,
            NodeLabel {
                rank: Some(3),
                ..Default::default()
            },
        );
    }

    order(&mut g, OrderOptions::default());
    let layering = util::build_layer_matrix(&g);
    assert_eq!(cross_count(&g, &layering), 0.0);
}

#[test]
fn order_can_minimize_crossings() {
    let mut g = new_graph();
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(1),
            ..Default::default()
        },
    );
    for v in ["b", "e", "g"] {
        g.set_node(
            v,
            NodeLabel {
                rank: Some(2),
                ..Default::default()
            },
        );
    }
    for v in ["c", "f", "h"] {
        g.set_node(
            v,
            NodeLabel {
                rank: Some(3),
                ..Default::default()
            },
        );
    }
    g.set_node(
        "d",
        NodeLabel {
            rank: Some(4),
            ..Default::default()
        },
    );

    order(&mut g, OrderOptions::default());
    let layering = util::build_layer_matrix(&g);
    assert!(cross_count(&g, &layering) <= 1.0);
}

#[test]
fn order_can_skip_the_optimal_ordering() {
    let mut g = new_graph();
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(1),
            ..Default::default()
        },
    );
    for v in ["b", "d"] {
        g.set_node(
            v,
            NodeLabel {
                rank: Some(2),
                ..Default::default()
            },
        );
    }
    for v in ["c", "e"] {
        g.set_node(
            v,
            NodeLabel {
                rank: Some(3),
                ..Default::default()
            },
        );
    }
    set_path(&mut g, &["a", "b", "c"]);
    g.set_edge("a", "d");
    g.set_edge("b", "e");
    g.set_edge("d", "c");

    order(
        &mut g,
        OrderOptions {
            disable_optimal_order_heuristic: true,
        },
    );
    let layering = util::build_layer_matrix(&g);
    assert_eq!(cross_count(&g, &layering), 1.0);
}
