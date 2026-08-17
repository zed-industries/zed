use dugong::add_border_segments;
use dugong::graphlib::{Graph, GraphOptions};
use dugong::{EdgeLabel, GraphLabel, NodeLabel};

fn compound_graph() -> Graph<NodeLabel, EdgeLabel, GraphLabel> {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: false,
        compound: true,
        ..Default::default()
    });
    g.set_graph(GraphLabel::default());
    g
}

#[test]
fn add_border_segments_does_not_add_border_nodes_for_a_non_compound_graph() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: false,
        compound: false,
        ..Default::default()
    });
    g.set_graph(GraphLabel::default());
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(0),
            ..Default::default()
        },
    );
    add_border_segments::add_border_segments(&mut g);
    assert_eq!(g.node_count(), 1);
    assert_eq!(
        g.node("a"),
        Some(&NodeLabel {
            rank: Some(0),
            ..Default::default()
        })
    );
}

#[test]
fn add_border_segments_does_not_add_border_nodes_for_a_graph_with_no_clusters() {
    let mut g = compound_graph();
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(0),
            ..Default::default()
        },
    );
    add_border_segments::add_border_segments(&mut g);
    assert_eq!(g.node_count(), 1);
    assert_eq!(
        g.node("a"),
        Some(&NodeLabel {
            rank: Some(0),
            ..Default::default()
        })
    );
}

#[test]
fn add_border_segments_adds_a_border_for_a_single_rank_subgraph() {
    let mut g = compound_graph();
    g.set_node(
        "sg",
        NodeLabel {
            min_rank: Some(1),
            max_rank: Some(1),
            ..Default::default()
        },
    );
    add_border_segments::add_border_segments(&mut g);

    let sg = g.node("sg").unwrap();
    let bl = sg.border_left[1].as_deref().unwrap();
    let br = sg.border_right[1].as_deref().unwrap();
    assert_eq!(
        g.node(bl),
        Some(&NodeLabel {
            dummy: Some("border".to_string()),
            border_type: Some("borderLeft".to_string()),
            rank: Some(1),
            width: 0.0,
            height: 0.0,
            ..Default::default()
        })
    );
    assert_eq!(g.parent(bl), Some("sg"));
    assert_eq!(
        g.node(br),
        Some(&NodeLabel {
            dummy: Some("border".to_string()),
            border_type: Some("borderRight".to_string()),
            rank: Some(1),
            width: 0.0,
            height: 0.0,
            ..Default::default()
        })
    );
    assert_eq!(g.parent(br), Some("sg"));
}

#[test]
fn add_border_segments_adds_a_border_for_a_multi_rank_subgraph() {
    let mut g = compound_graph();
    g.set_node(
        "sg",
        NodeLabel {
            min_rank: Some(1),
            max_rank: Some(2),
            ..Default::default()
        },
    );
    add_border_segments::add_border_segments(&mut g);

    let sg = g.node("sg").unwrap().clone();
    let bl1 = sg.border_left[1].clone().unwrap();
    let br1 = sg.border_right[1].clone().unwrap();
    assert_eq!(
        g.node(&bl1),
        Some(&NodeLabel {
            dummy: Some("border".to_string()),
            border_type: Some("borderLeft".to_string()),
            rank: Some(1),
            width: 0.0,
            height: 0.0,
            ..Default::default()
        })
    );
    assert_eq!(g.parent(&bl1), Some("sg"));
    assert_eq!(
        g.node(&br1),
        Some(&NodeLabel {
            dummy: Some("border".to_string()),
            border_type: Some("borderRight".to_string()),
            rank: Some(1),
            width: 0.0,
            height: 0.0,
            ..Default::default()
        })
    );
    assert_eq!(g.parent(&br1), Some("sg"));

    let sg2 = g.node("sg").unwrap();
    let bl2 = sg2.border_left[2].clone().unwrap();
    let br2 = sg2.border_right[2].clone().unwrap();
    assert_eq!(
        g.node(&bl2),
        Some(&NodeLabel {
            dummy: Some("border".to_string()),
            border_type: Some("borderLeft".to_string()),
            rank: Some(2),
            width: 0.0,
            height: 0.0,
            ..Default::default()
        })
    );
    assert_eq!(g.parent(&bl2), Some("sg"));
    assert_eq!(
        g.node(&br2),
        Some(&NodeLabel {
            dummy: Some("border".to_string()),
            border_type: Some("borderRight".to_string()),
            rank: Some(2),
            width: 0.0,
            height: 0.0,
            ..Default::default()
        })
    );
    assert_eq!(g.parent(&br2), Some("sg"));

    assert!(g.has_edge(&bl1, &bl2, None));
    assert!(g.has_edge(&br1, &br2, None));
}

#[test]
fn add_border_segments_adds_borders_for_nested_subgraphs() {
    let mut g = compound_graph();
    g.set_node(
        "sg1",
        NodeLabel {
            min_rank: Some(1),
            max_rank: Some(1),
            ..Default::default()
        },
    );
    g.set_node(
        "sg2",
        NodeLabel {
            min_rank: Some(1),
            max_rank: Some(1),
            ..Default::default()
        },
    );
    g.set_parent("sg2", "sg1");

    add_border_segments::add_border_segments(&mut g);

    let sg1 = g.node("sg1").unwrap();
    let bl1 = sg1.border_left[1].as_deref().unwrap();
    let br1 = sg1.border_right[1].as_deref().unwrap();
    assert_eq!(g.parent(bl1), Some("sg1"));
    assert_eq!(g.parent(br1), Some("sg1"));

    let sg2 = g.node("sg2").unwrap();
    let bl2 = sg2.border_left[1].as_deref().unwrap();
    let br2 = sg2.border_right[1].as_deref().unwrap();
    assert_eq!(g.parent(bl2), Some("sg2"));
    assert_eq!(g.parent(br2), Some("sg2"));
}
