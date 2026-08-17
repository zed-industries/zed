use dugong::NodeLabel;
use dugong::graphlib::{Graph, GraphOptions};
use dugong::order::init_order;

fn new_graph() -> Graph<NodeLabel, (), ()> {
    Graph::new(GraphOptions {
        compound: true,
        ..Default::default()
    })
}

#[test]
fn init_order_assigns_non_overlapping_orders_for_each_rank_in_a_tree() {
    let mut g = new_graph();
    for (v, rank) in [("a", 0), ("b", 1), ("c", 2), ("d", 2), ("e", 1)] {
        g.set_node(
            v,
            NodeLabel {
                rank: Some(rank),
                ..Default::default()
            },
        );
    }
    g.set_edge("a", "b");
    g.set_edge("b", "c");
    g.set_edge("b", "d");
    g.set_edge("a", "e");

    let layering = init_order(&g);
    assert_eq!(layering[0], vec!["a".to_string()]);
    let mut l1 = layering[1].clone();
    l1.sort();
    assert_eq!(l1, vec!["b".to_string(), "e".to_string()]);
    let mut l2 = layering[2].clone();
    l2.sort();
    assert_eq!(l2, vec!["c".to_string(), "d".to_string()]);
}

#[test]
fn init_order_assigns_non_overlapping_orders_for_each_rank_in_a_dag() {
    let mut g = new_graph();
    for (v, rank) in [("a", 0), ("b", 1), ("c", 1), ("d", 2)] {
        g.set_node(
            v,
            NodeLabel {
                rank: Some(rank),
                ..Default::default()
            },
        );
    }
    g.set_edge("a", "b");
    g.set_edge("b", "d");
    g.set_edge("a", "c");
    g.set_edge("c", "d");

    let layering = init_order(&g);
    assert_eq!(layering[0], vec!["a".to_string()]);
    let mut l1 = layering[1].clone();
    l1.sort();
    assert_eq!(l1, vec!["b".to_string(), "c".to_string()]);
    assert_eq!(layering[2], vec!["d".to_string()]);
}

#[test]
fn init_order_does_not_assign_an_order_to_subgraph_nodes() {
    let mut g = new_graph();
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(0),
            ..Default::default()
        },
    );
    g.ensure_node("sg1");
    g.set_parent("a", "sg1");

    let layering = init_order(&g);
    assert_eq!(layering, vec![vec!["a".to_string()]]);
}
