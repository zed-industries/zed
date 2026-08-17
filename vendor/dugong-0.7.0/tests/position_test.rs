use dugong::graphlib::{Graph, GraphOptions};
use dugong::position;
use dugong::{EdgeLabel, GraphLabel, NodeLabel};

fn graph() -> Graph<NodeLabel, EdgeLabel, GraphLabel> {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        compound: true,
        multigraph: false,
        ..Default::default()
    });
    g.set_graph(GraphLabel {
        ranksep: 50.0,
        nodesep: 50.0,
        edgesep: 10.0,
        ..Default::default()
    });
    g
}

#[test]
fn position_respects_ranksep() {
    let mut g = graph();
    g.graph_mut().ranksep = 1000.0;
    g.set_node(
        "a",
        NodeLabel {
            width: 50.0,
            height: 100.0,
            rank: Some(0),
            order: Some(0),
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            width: 50.0,
            height: 80.0,
            rank: Some(1),
            order: Some(0),
            ..Default::default()
        },
    );
    g.set_edge("a", "b");

    position::position(&mut g);
    assert_eq!(g.node("b").unwrap().y, Some(100.0 + 1000.0 + 80.0 / 2.0));
}

#[test]
fn position_uses_largest_height_in_each_rank_with_ranksep() {
    let mut g = graph();
    g.graph_mut().ranksep = 1000.0;
    g.set_node(
        "a",
        NodeLabel {
            width: 50.0,
            height: 100.0,
            rank: Some(0),
            order: Some(0),
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            width: 50.0,
            height: 80.0,
            rank: Some(0),
            order: Some(1),
            ..Default::default()
        },
    );
    g.set_node(
        "c",
        NodeLabel {
            width: 50.0,
            height: 90.0,
            rank: Some(1),
            order: Some(0),
            ..Default::default()
        },
    );
    g.set_edge("a", "c");

    position::position(&mut g);
    assert_eq!(g.node("a").unwrap().y, Some(100.0 / 2.0));
    assert_eq!(g.node("b").unwrap().y, Some(100.0 / 2.0));
    assert_eq!(g.node("c").unwrap().y, Some(100.0 + 1000.0 + 90.0 / 2.0));
}

#[test]
fn position_respects_nodesep() {
    let mut g = graph();
    g.graph_mut().nodesep = 1000.0;
    g.set_node(
        "a",
        NodeLabel {
            width: 50.0,
            height: 100.0,
            rank: Some(0),
            order: Some(0),
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            width: 70.0,
            height: 80.0,
            rank: Some(0),
            order: Some(1),
            ..Default::default()
        },
    );

    position::position(&mut g);
    assert_eq!(
        g.node("b").unwrap().x,
        Some(g.node("a").unwrap().x.unwrap() + 50.0 / 2.0 + 1000.0 + 70.0 / 2.0)
    );
}

#[test]
fn position_does_not_try_to_position_the_subgraph_node_itself() {
    let mut g = graph();
    g.set_node(
        "a",
        NodeLabel {
            width: 50.0,
            height: 50.0,
            rank: Some(0),
            order: Some(0),
            ..Default::default()
        },
    );
    g.set_node("sg1", NodeLabel::default());
    g.set_parent("a", "sg1");

    position::position(&mut g);
    assert_eq!(g.node("sg1").unwrap().x, None);
    assert_eq!(g.node("sg1").unwrap().y, None);
}
