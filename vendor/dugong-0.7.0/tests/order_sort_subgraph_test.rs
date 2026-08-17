use dugong::NodeLabel;
use dugong::graphlib::{Graph, GraphOptions};
use dugong::order::{SortResult, WeightLabel, sort_subgraph};

fn new_graph_compound() -> Graph<NodeLabel, WeightLabel, ()> {
    let mut g: Graph<NodeLabel, WeightLabel, ()> = Graph::new(GraphOptions {
        compound: true,
        ..Default::default()
    });
    g.set_default_node_label(NodeLabel::default);
    g.set_default_edge_label(|| WeightLabel { weight: 1.0 });
    g
}

fn seed_order_nodes(g: &mut Graph<NodeLabel, WeightLabel, ()>) {
    for v in 0..=4usize {
        g.set_node(
            v.to_string(),
            NodeLabel {
                order: Some(v),
                ..Default::default()
            },
        );
    }
}

#[test]
fn sort_subgraph_sorts_a_flat_subgraph_based_on_barycenter() {
    let mut g = new_graph_compound();
    seed_order_nodes(&mut g);
    let cg: Graph<(), (), ()> = Graph::new(GraphOptions::default());

    g.set_edge("3", "x");
    g.set_edge_with_label("1", "y", WeightLabel { weight: 2.0 });
    g.set_edge("4", "y");
    g.set_parent("x", "movable");
    g.set_parent("y", "movable");

    assert_eq!(
        sort_subgraph(&g, "movable", &cg, false).vs,
        vec!["y".to_string(), "x".to_string()]
    );
}

#[test]
fn sort_subgraph_preserves_the_pos_of_a_node_without_neighbors_in_a_flat_subgraph() {
    let mut g = new_graph_compound();
    seed_order_nodes(&mut g);
    let cg: Graph<(), (), ()> = Graph::new(GraphOptions::default());

    g.set_edge("3", "x");
    g.ensure_node("y");
    g.set_edge_with_label("1", "z", WeightLabel { weight: 2.0 });
    g.set_edge("4", "z");
    for v in ["x", "y", "z"] {
        g.set_parent(v, "movable");
    }

    assert_eq!(
        sort_subgraph(&g, "movable", &cg, false).vs,
        vec!["z".to_string(), "y".to_string(), "x".to_string()]
    );
}

#[test]
fn sort_subgraph_biases_to_the_left_without_reverse_bias() {
    let mut g = new_graph_compound();
    seed_order_nodes(&mut g);
    let cg: Graph<(), (), ()> = Graph::new(GraphOptions::default());

    g.set_edge("1", "x");
    g.set_edge("1", "y");
    for v in ["x", "y"] {
        g.set_parent(v, "movable");
    }

    assert_eq!(
        sort_subgraph(&g, "movable", &cg, false).vs,
        vec!["x".to_string(), "y".to_string()]
    );
}

#[test]
fn sort_subgraph_biases_to_the_right_with_reverse_bias() {
    let mut g = new_graph_compound();
    seed_order_nodes(&mut g);
    let cg: Graph<(), (), ()> = Graph::new(GraphOptions::default());

    g.set_edge("1", "x");
    g.set_edge("1", "y");
    for v in ["x", "y"] {
        g.set_parent(v, "movable");
    }

    assert_eq!(
        sort_subgraph(&g, "movable", &cg, true).vs,
        vec!["y".to_string(), "x".to_string()]
    );
}

#[test]
fn sort_subgraph_aggregates_stats_about_the_subgraph() {
    let mut g = new_graph_compound();
    seed_order_nodes(&mut g);
    let cg: Graph<(), (), ()> = Graph::new(GraphOptions::default());

    g.set_edge("3", "x");
    g.set_edge_with_label("1", "y", WeightLabel { weight: 2.0 });
    g.set_edge("4", "y");
    g.set_parent("x", "movable");
    g.set_parent("y", "movable");

    let result = sort_subgraph(&g, "movable", &cg, false);
    assert_eq!(result.barycenter, Some(2.25));
    assert_eq!(result.weight, Some(4.0));
}

#[test]
fn sort_subgraph_can_sort_a_nested_subgraph_with_no_barycenter() {
    let mut g = new_graph_compound();
    seed_order_nodes(&mut g);
    let cg: Graph<(), (), ()> = Graph::new(GraphOptions::default());

    for v in ["a", "b", "c"] {
        g.ensure_node(v);
        g.set_parent(v, "y");
    }
    g.set_edge("0", "x");
    g.set_edge("1", "z");
    g.set_edge("2", "y");
    for v in ["x", "y", "z"] {
        g.set_parent(v, "movable");
    }

    assert_eq!(
        sort_subgraph(&g, "movable", &cg, false).vs,
        vec![
            "x".to_string(),
            "z".to_string(),
            "a".to_string(),
            "b".to_string(),
            "c".to_string()
        ]
    );
}

#[test]
fn sort_subgraph_can_sort_a_nested_subgraph_with_a_barycenter() {
    let mut g = new_graph_compound();
    seed_order_nodes(&mut g);
    let cg: Graph<(), (), ()> = Graph::new(GraphOptions::default());

    for v in ["a", "b", "c"] {
        g.ensure_node(v);
        g.set_parent(v, "y");
    }
    g.set_edge_with_label("0", "a", WeightLabel { weight: 3.0 });
    g.set_edge("0", "x");
    g.set_edge("1", "z");
    g.set_edge("2", "y");
    for v in ["x", "y", "z"] {
        g.set_parent(v, "movable");
    }

    assert_eq!(
        sort_subgraph(&g, "movable", &cg, false).vs,
        vec![
            "x".to_string(),
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "z".to_string()
        ]
    );
}

#[test]
fn sort_subgraph_handles_deep_compound_chains_with_small_stack() {
    const DEPTH: usize = 2048;
    let handle = std::thread::Builder::new()
        .name("dugong-sort-subgraph-deep-chain".to_string())
        .stack_size(64 * 1024)
        .spawn(|| {
            let mut g = new_graph_compound();
            g.ensure_node("root");
            for i in 0..DEPTH {
                let id = format!("n{i}");
                g.ensure_node(id.as_str());
                let parent = if i == 0 {
                    "root".to_string()
                } else {
                    format!("n{}", i - 1)
                };
                g.set_parent(id, parent);
            }
            let leaf = "leaf";
            g.ensure_node(leaf);
            g.set_parent(leaf, format!("n{}", DEPTH - 1));

            let cg: Graph<(), (), ()> = Graph::new(GraphOptions::default());
            let result = sort_subgraph(&g, "root", &cg, false);

            assert_eq!(result.vs, vec![leaf.to_string()]);
        })
        .expect("spawn dugong sort-subgraph deep chain test");
    handle
        .join()
        .expect("dugong sort-subgraph deep chain should finish without stack overflow");
}

#[test]
fn sort_subgraph_can_sort_a_nested_subgraph_with_no_in_edges() {
    let mut g = new_graph_compound();
    seed_order_nodes(&mut g);
    let cg: Graph<(), (), ()> = Graph::new(GraphOptions::default());

    for v in ["a", "b", "c"] {
        g.ensure_node(v);
        g.set_parent(v, "y");
    }
    g.set_edge("0", "a");
    g.set_edge("1", "b");
    g.set_edge("0", "x");
    g.set_edge("1", "z");
    for v in ["x", "y", "z"] {
        g.set_parent(v, "movable");
    }

    assert_eq!(
        sort_subgraph(&g, "movable", &cg, false).vs,
        vec![
            "x".to_string(),
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "z".to_string()
        ]
    );
}

#[test]
fn sort_subgraph_sorts_border_nodes_to_the_extremes_of_the_subgraph() {
    let mut g = new_graph_compound();
    seed_order_nodes(&mut g);
    let cg: Graph<(), (), ()> = Graph::new(GraphOptions::default());

    g.set_edge("0", "x");
    g.set_edge("1", "y");
    g.set_edge("2", "z");
    g.set_node(
        "sg1",
        NodeLabel {
            border_left: vec![Some("bl".to_string())],
            border_right: vec![Some("br".to_string())],
            ..Default::default()
        },
    );
    for v in ["x", "y", "z", "bl", "br"] {
        g.set_parent(v, "sg1");
    }

    assert_eq!(
        sort_subgraph(&g, "sg1", &cg, false).vs,
        vec![
            "bl".to_string(),
            "x".to_string(),
            "y".to_string(),
            "z".to_string(),
            "br".to_string()
        ]
    );
}

#[test]
fn sort_subgraph_assigns_a_barycenter_to_a_subgraph_based_on_previous_border_nodes() {
    let mut g = new_graph_compound();
    let cg: Graph<(), (), ()> = Graph::new(GraphOptions::default());

    g.set_node(
        "bl1",
        NodeLabel {
            order: Some(0),
            ..Default::default()
        },
    );
    g.set_node(
        "br1",
        NodeLabel {
            order: Some(1),
            ..Default::default()
        },
    );
    g.set_edge("bl1", "bl2");
    g.set_edge("br1", "br2");
    for v in ["bl2", "br2"] {
        g.set_parent(v, "sg");
    }
    g.set_node(
        "sg",
        NodeLabel {
            border_left: vec![Some("bl2".to_string())],
            border_right: vec![Some("br2".to_string())],
            ..Default::default()
        },
    );

    assert_eq!(
        sort_subgraph(&g, "sg", &cg, false),
        SortResult {
            barycenter: Some(0.5),
            weight: Some(2.0),
            vs: vec!["bl2".to_string(), "br2".to_string()]
        }
    );
}
