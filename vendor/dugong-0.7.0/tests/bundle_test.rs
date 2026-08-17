use dugong::graphlib::{Graph, GraphOptions};
use dugong::{EdgeLabel, GraphLabel, NodeLabel};

#[test]
fn bundle_exports_expected_symbols() {
    let _ = dugong::VERSION;
    let _ =
        dugong::graphlib::Graph::<NodeLabel, EdgeLabel, GraphLabel>::new(GraphOptions::default());
    let _ = dugong::util::range(0);
    let _layout: fn(&mut Graph<NodeLabel, EdgeLabel, GraphLabel>) = dugong::layout;
}

#[test]
fn bundle_can_do_trivial_layout() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions::default());
    g.set_graph(GraphLabel::default());

    g.set_node(
        "a",
        NodeLabel {
            width: 50.0,
            height: 100.0,
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            width: 50.0,
            height: 100.0,
            ..Default::default()
        },
    );
    g.set_edge_with_label(
        "a",
        "b",
        EdgeLabel {
            width: 50.0,
            height: 100.0,
            ..Default::default()
        },
    );

    dugong::layout(&mut g);

    let a = g.node("a").expect("node a must exist");
    let b = g.node("b").expect("node b must exist");
    assert!(a.x.unwrap_or(-1.0) >= 0.0);
    assert!(a.y.unwrap_or(-1.0) >= 0.0);
    assert!(b.x.unwrap_or(-1.0) >= 0.0);
    assert!(b.y.unwrap_or(-1.0) >= 0.0);

    let e = g.edge("a", "b", None).expect("edge must exist");
    assert!(e.x.unwrap_or(-1.0) >= 0.0);
    assert!(e.y.unwrap_or(-1.0) >= 0.0);
}
