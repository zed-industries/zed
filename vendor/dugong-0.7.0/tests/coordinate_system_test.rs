use dugong::coordinate_system;
use dugong::graphlib::{Graph, GraphOptions};
use dugong::{EdgeLabel, GraphLabel, NodeLabel, RankDir};

#[test]
fn coordinate_system_adjust_does_nothing_to_node_dimensions_with_rankdir_tb() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: false,
        compound: false,
        ..Default::default()
    });
    g.set_graph(GraphLabel {
        rankdir: RankDir::TB,
        ..Default::default()
    });
    g.set_node(
        "a",
        NodeLabel {
            width: 100.0,
            height: 200.0,
            ..Default::default()
        },
    );

    coordinate_system::adjust(&mut g);
    assert_eq!(
        g.node("a").unwrap(),
        &NodeLabel {
            width: 100.0,
            height: 200.0,
            ..Default::default()
        }
    );
}

#[test]
fn coordinate_system_adjust_does_nothing_to_node_dimensions_with_rankdir_bt() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: false,
        compound: false,
        ..Default::default()
    });
    g.set_graph(GraphLabel {
        rankdir: RankDir::BT,
        ..Default::default()
    });
    g.set_node(
        "a",
        NodeLabel {
            width: 100.0,
            height: 200.0,
            ..Default::default()
        },
    );

    coordinate_system::adjust(&mut g);
    assert_eq!(
        g.node("a").unwrap(),
        &NodeLabel {
            width: 100.0,
            height: 200.0,
            ..Default::default()
        }
    );
}

#[test]
fn coordinate_system_adjust_swaps_width_and_height_for_nodes_with_rankdir_lr() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: false,
        compound: false,
        ..Default::default()
    });
    g.set_graph(GraphLabel {
        rankdir: RankDir::LR,
        ..Default::default()
    });
    g.set_node(
        "a",
        NodeLabel {
            width: 100.0,
            height: 200.0,
            ..Default::default()
        },
    );

    coordinate_system::adjust(&mut g);
    assert_eq!(
        g.node("a").unwrap(),
        &NodeLabel {
            width: 200.0,
            height: 100.0,
            ..Default::default()
        }
    );
}

#[test]
fn coordinate_system_adjust_swaps_width_and_height_for_nodes_with_rankdir_rl() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: false,
        compound: false,
        ..Default::default()
    });
    g.set_graph(GraphLabel {
        rankdir: RankDir::RL,
        ..Default::default()
    });
    g.set_node(
        "a",
        NodeLabel {
            width: 100.0,
            height: 200.0,
            ..Default::default()
        },
    );

    coordinate_system::adjust(&mut g);
    assert_eq!(
        g.node("a").unwrap(),
        &NodeLabel {
            width: 200.0,
            height: 100.0,
            ..Default::default()
        }
    );
}

#[test]
fn coordinate_system_undo_does_nothing_to_points_with_rankdir_tb() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: false,
        compound: false,
        ..Default::default()
    });
    g.set_graph(GraphLabel {
        rankdir: RankDir::TB,
        ..Default::default()
    });
    g.set_node(
        "a",
        NodeLabel {
            width: 100.0,
            height: 200.0,
            x: Some(20.0),
            y: Some(40.0),
            ..Default::default()
        },
    );

    coordinate_system::undo(&mut g);
    assert_eq!(
        g.node("a").unwrap(),
        &NodeLabel {
            width: 100.0,
            height: 200.0,
            x: Some(20.0),
            y: Some(40.0),
            ..Default::default()
        }
    );
}

#[test]
fn coordinate_system_undo_flips_the_y_coordinate_for_points_with_rankdir_bt() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: false,
        compound: false,
        ..Default::default()
    });
    g.set_graph(GraphLabel {
        rankdir: RankDir::BT,
        ..Default::default()
    });
    g.set_node(
        "a",
        NodeLabel {
            width: 100.0,
            height: 200.0,
            x: Some(20.0),
            y: Some(40.0),
            ..Default::default()
        },
    );

    coordinate_system::undo(&mut g);
    assert_eq!(
        g.node("a").unwrap(),
        &NodeLabel {
            width: 100.0,
            height: 200.0,
            x: Some(20.0),
            y: Some(-40.0),
            ..Default::default()
        }
    );
}

#[test]
fn coordinate_system_undo_swaps_dimensions_and_coordinates_for_points_with_rankdir_lr() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: false,
        compound: false,
        ..Default::default()
    });
    g.set_graph(GraphLabel {
        rankdir: RankDir::LR,
        ..Default::default()
    });
    g.set_node(
        "a",
        NodeLabel {
            width: 100.0,
            height: 200.0,
            x: Some(20.0),
            y: Some(40.0),
            ..Default::default()
        },
    );

    coordinate_system::undo(&mut g);
    assert_eq!(
        g.node("a").unwrap(),
        &NodeLabel {
            width: 200.0,
            height: 100.0,
            x: Some(40.0),
            y: Some(20.0),
            ..Default::default()
        }
    );
}

#[test]
fn coordinate_system_undo_swaps_dims_and_coords_and_flips_x_for_points_with_rankdir_rl() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: false,
        compound: false,
        ..Default::default()
    });
    g.set_graph(GraphLabel {
        rankdir: RankDir::RL,
        ..Default::default()
    });
    g.set_node(
        "a",
        NodeLabel {
            width: 100.0,
            height: 200.0,
            x: Some(20.0),
            y: Some(40.0),
            ..Default::default()
        },
    );

    coordinate_system::undo(&mut g);
    assert_eq!(
        g.node("a").unwrap(),
        &NodeLabel {
            width: 200.0,
            height: 100.0,
            x: Some(-40.0),
            y: Some(20.0),
            ..Default::default()
        }
    );
}
