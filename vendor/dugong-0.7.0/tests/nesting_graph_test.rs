use dugong::graphlib::alg::components;
use dugong::graphlib::{Graph, GraphOptions};
use dugong::nesting_graph;
use dugong::{EdgeLabel, GraphLabel, NodeLabel};

fn graph() -> Graph<NodeLabel, EdgeLabel, GraphLabel> {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        compound: true,
        multigraph: true,
        ..Default::default()
    });
    g.set_graph(GraphLabel::default());
    g.set_default_node_label(NodeLabel::default);
    g.set_default_edge_label(EdgeLabel::default);
    g
}

#[test]
fn nesting_graph_connects_a_disconnected_graph() {
    let mut g = graph();
    g.ensure_node("a");
    g.ensure_node("b");
    assert_eq!(components(&g).len(), 2);
    nesting_graph::run(&mut g);
    assert_eq!(components(&g).len(), 1);
    assert!(g.has_node("a"));
    assert!(g.has_node("b"));
}

#[test]
fn nesting_graph_adds_border_nodes_to_top_and_bottom_of_a_subgraph() {
    let mut g = graph();
    g.set_parent("a", "sg1");
    nesting_graph::run(&mut g);

    let sg = g.node("sg1").unwrap();
    let top = sg.border_top.clone().unwrap();
    let bottom = sg.border_bottom.clone().unwrap();

    assert_eq!(g.parent(&top), Some("sg1"));
    assert_eq!(g.parent(&bottom), Some("sg1"));

    let out = g.out_edges(&top, Some("a"));
    assert_eq!(out.len(), 1);
    assert_eq!(g.edge_by_key(&out[0]).unwrap().minlen, 1);

    let out2 = g.out_edges("a", Some(&bottom));
    assert_eq!(out2.len(), 1);
    assert_eq!(g.edge_by_key(&out2[0]).unwrap().minlen, 1);

    assert_eq!(
        g.node(&top).unwrap(),
        &NodeLabel {
            width: 0.0,
            height: 0.0,
            dummy: Some("border".to_string()),
            ..Default::default()
        }
    );
    assert_eq!(
        g.node(&bottom).unwrap(),
        &NodeLabel {
            width: 0.0,
            height: 0.0,
            dummy: Some("border".to_string()),
            ..Default::default()
        }
    );
}

#[test]
fn nesting_graph_adds_edges_between_borders_of_nested_subgraphs() {
    let mut g = graph();
    g.set_parent("sg2", "sg1");
    g.set_parent("a", "sg2");
    nesting_graph::run(&mut g);

    let sg1 = g.node("sg1").unwrap();
    let sg2 = g.node("sg2").unwrap();
    let sg1_top = sg1.border_top.clone().unwrap();
    let sg1_bottom = sg1.border_bottom.clone().unwrap();
    let sg2_top = sg2.border_top.clone().unwrap();
    let sg2_bottom = sg2.border_bottom.clone().unwrap();

    assert_eq!(g.out_edges(&sg1_top, Some(&sg2_top)).len(), 1);
    assert_eq!(
        g.edge_by_key(&g.out_edges(&sg1_top, Some(&sg2_top))[0])
            .unwrap()
            .minlen,
        1
    );
    assert_eq!(g.out_edges(&sg2_bottom, Some(&sg1_bottom)).len(), 1);
    assert_eq!(
        g.edge_by_key(&g.out_edges(&sg2_bottom, Some(&sg1_bottom))[0])
            .unwrap()
            .minlen,
        1
    );
}

#[test]
fn nesting_graph_adds_sufficient_weight_to_border_to_node_edges() {
    let mut g = graph();
    g.set_parent("x", "sg");
    g.set_edge_with_label(
        "a",
        "x",
        EdgeLabel {
            weight: 100.0,
            minlen: 1,
            ..Default::default()
        },
    );
    g.set_edge_with_label(
        "x",
        "b",
        EdgeLabel {
            weight: 200.0,
            minlen: 1,
            ..Default::default()
        },
    );
    nesting_graph::run(&mut g);

    let sg = g.node("sg").unwrap();
    let top = sg.border_top.clone().unwrap();
    let bot = sg.border_bottom.clone().unwrap();

    assert!(g.edge(&top, "x", None).unwrap().weight > 300.0);
    assert!(g.edge("x", &bot, None).unwrap().weight > 300.0);
}

#[test]
fn nesting_graph_adds_edge_from_root_to_tops_of_top_level_subgraphs() {
    let mut g = graph();
    g.set_parent("a", "sg1");
    nesting_graph::run(&mut g);

    let root = g.graph().nesting_root.clone().unwrap();
    let top = g.node("sg1").unwrap().border_top.clone().unwrap();
    assert_eq!(g.out_edges(&root, Some(&top)).len(), 1);
    assert!(g.has_edge(&root, &top, None));
}

#[test]
fn nesting_graph_adds_edge_from_root_to_each_node_minlen_1() {
    let mut g = graph();
    g.ensure_node("a");
    nesting_graph::run(&mut g);

    let root = g.graph().nesting_root.clone().unwrap();
    let out = g.out_edges(&root, Some("a"));
    assert_eq!(out.len(), 1);
    let e = g.edge_by_key(&out[0]).unwrap();
    assert_eq!(e.weight, 0.0);
    assert_eq!(e.minlen, 1);
}

#[test]
fn nesting_graph_adds_edge_from_root_to_each_node_minlen_2() {
    let mut g = graph();
    g.set_parent("a", "sg1");
    nesting_graph::run(&mut g);

    let root = g.graph().nesting_root.clone().unwrap();
    let out = g.out_edges(&root, Some("a"));
    assert_eq!(out.len(), 1);
    let e = g.edge_by_key(&out[0]).unwrap();
    assert_eq!(e.weight, 0.0);
    assert_eq!(e.minlen, 3);
}

#[test]
fn nesting_graph_adds_edge_from_root_to_each_node_minlen_3() {
    let mut g = graph();
    g.set_parent("sg2", "sg1");
    g.set_parent("a", "sg2");
    nesting_graph::run(&mut g);

    let root = g.graph().nesting_root.clone().unwrap();
    let out = g.out_edges(&root, Some("a"));
    assert_eq!(out.len(), 1);
    let e = g.edge_by_key(&out[0]).unwrap();
    assert_eq!(e.weight, 0.0);
    assert_eq!(e.minlen, 5);
}

#[test]
fn nesting_graph_does_not_add_an_edge_from_root_to_itself() {
    let mut g = graph();
    g.ensure_node("a");
    nesting_graph::run(&mut g);

    let root = g.graph().nesting_root.clone().unwrap();
    assert!(g.out_edges(&root, Some(&root)).is_empty());
}

#[test]
fn nesting_graph_expands_inter_node_edges_minlen_1() {
    let mut g = graph();
    g.set_edge_with_label(
        "a",
        "b",
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    nesting_graph::run(&mut g);
    assert_eq!(g.edge("a", "b", None).unwrap().minlen, 1);
}

#[test]
fn nesting_graph_expands_inter_node_edges_minlen_2() {
    let mut g = graph();
    g.set_parent("a", "sg1");
    g.set_edge_with_label(
        "a",
        "b",
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    nesting_graph::run(&mut g);
    assert_eq!(g.edge("a", "b", None).unwrap().minlen, 3);
}

#[test]
fn nesting_graph_expands_inter_node_edges_minlen_3() {
    let mut g = graph();
    g.set_parent("sg2", "sg1");
    g.set_parent("a", "sg2");
    g.set_edge_with_label(
        "a",
        "b",
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    nesting_graph::run(&mut g);
    assert_eq!(g.edge("a", "b", None).unwrap().minlen, 5);
}

#[test]
fn nesting_graph_sets_minlen_correctly_for_nested_border_to_children() {
    let mut g = graph();
    g.set_parent("a", "sg1");
    g.set_parent("sg2", "sg1");
    g.set_parent("b", "sg2");
    nesting_graph::run(&mut g);

    let root = g.graph().nesting_root.clone().unwrap();
    let sg1 = g.node("sg1").unwrap();
    let sg2 = g.node("sg2").unwrap();
    let sg1_top = sg1.border_top.clone().unwrap();
    let sg1_bot = sg1.border_bottom.clone().unwrap();
    let sg2_top = sg2.border_top.clone().unwrap();
    let sg2_bot = sg2.border_bottom.clone().unwrap();

    assert_eq!(g.edge(&root, &sg1_top, None).unwrap().minlen, 3);
    assert_eq!(g.edge(&sg1_top, &sg2_top, None).unwrap().minlen, 1);
    assert_eq!(g.edge(&sg1_top, "a", None).unwrap().minlen, 2);
    assert_eq!(g.edge("a", &sg1_bot, None).unwrap().minlen, 2);
    assert_eq!(g.edge(&sg2_top, "b", None).unwrap().minlen, 1);
    assert_eq!(g.edge("b", &sg2_bot, None).unwrap().minlen, 1);
    assert_eq!(g.edge(&sg2_bot, &sg1_bot, None).unwrap().minlen, 1);
}

#[test]
fn nesting_graph_cleanup_removes_nesting_graph_edges() {
    let mut g = graph();
    g.set_parent("a", "sg1");
    g.set_edge_with_label(
        "a",
        "b",
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    nesting_graph::run(&mut g);
    nesting_graph::cleanup(&mut g);
    assert_eq!(g.successors("a"), vec!["b"]);
}

#[test]
fn nesting_graph_cleanup_removes_the_root_node() {
    let mut g = graph();
    g.set_parent("a", "sg1");
    nesting_graph::run(&mut g);
    nesting_graph::cleanup(&mut g);
    assert_eq!(g.node_count(), 4);
}
