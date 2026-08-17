use dugong::graphlib::{Graph, GraphOptions};
use dugong::{EdgeLabel, GraphLabel, LabelPos, NodeLabel, Point, RankDir, layout};

#[cfg(feature = "dagreish")]
use dugong::layout_dagreish;

fn coords(
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
) -> std::collections::BTreeMap<String, (f64, f64)> {
    let mut out = std::collections::BTreeMap::new();
    for id in g.nodes() {
        let n = g.node(id).unwrap();
        out.insert(id.to_string(), (n.x.unwrap(), n.y.unwrap()));
    }
    out
}

#[test]
fn layout_can_layout_a_single_node() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: true,
        compound: true,
        ..Default::default()
    });
    g.set_graph(GraphLabel::default());
    g.set_default_edge_label(EdgeLabel::default);

    g.set_node(
        "a",
        NodeLabel {
            width: 50.0,
            height: 100.0,
            ..Default::default()
        },
    );

    layout(&mut g);
    assert_eq!(coords(&g), [("a".to_string(), (25.0, 50.0))].into());
    assert_eq!(g.node("a").unwrap().x, Some(25.0));
    assert_eq!(g.node("a").unwrap().y, Some(50.0));
}

#[test]
fn layout_can_layout_two_nodes_on_the_same_rank() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: true,
        compound: true,
        ..Default::default()
    });
    g.set_graph(GraphLabel::default());
    g.set_default_edge_label(EdgeLabel::default);

    g.graph_mut().nodesep = 200.0;
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
            width: 75.0,
            height: 200.0,
            ..Default::default()
        },
    );

    layout(&mut g);
    assert_eq!(
        coords(&g),
        [
            ("a".to_string(), (25.0, 100.0)),
            ("b".to_string(), (50.0 + 200.0 + 75.0 / 2.0, 100.0)),
        ]
        .into()
    );
}

#[test]
fn layout_can_layout_two_nodes_connected_by_an_edge() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: true,
        compound: true,
        ..Default::default()
    });
    g.set_graph(GraphLabel::default());
    g.set_default_edge_label(EdgeLabel::default);

    g.graph_mut().ranksep = 300.0;
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
            width: 75.0,
            height: 200.0,
            ..Default::default()
        },
    );
    g.set_edge("a", "b");

    layout(&mut g);
    assert_eq!(
        coords(&g),
        [
            ("a".to_string(), (75.0 / 2.0, 100.0 / 2.0)),
            ("b".to_string(), (75.0 / 2.0, 100.0 + 300.0 + 200.0 / 2.0)),
        ]
        .into()
    );
}

#[test]
fn layout_can_layout_an_edge_with_a_label() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: true,
        compound: true,
        ..Default::default()
    });
    g.set_graph(GraphLabel::default());
    g.set_default_edge_label(EdgeLabel::default);

    g.graph_mut().ranksep = 300.0;
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
            width: 75.0,
            height: 200.0,
            ..Default::default()
        },
    );
    g.set_edge_with_label(
        "a",
        "b",
        EdgeLabel {
            width: 60.0,
            height: 70.0,
            labelpos: LabelPos::C,
            ..Default::default()
        },
    );

    layout(&mut g);
    assert_eq!(
        coords(&g),
        [
            ("a".to_string(), (75.0 / 2.0, 100.0 / 2.0)),
            (
                "b".to_string(),
                (75.0 / 2.0, 100.0 + 150.0 + 70.0 + 150.0 + 200.0 / 2.0),
            ),
        ]
        .into()
    );

    let e = g.edge("a", "b", None).unwrap();
    assert_eq!(e.x, Some(75.0 / 2.0));
    assert_eq!(e.y, Some(100.0 + 150.0 + 70.0 / 2.0));
}

#[cfg(feature = "dagreish")]
#[test]
fn layout_dagreish_can_layout_a_long_edge_with_a_label() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: true,
        compound: true,
        ..Default::default()
    });
    g.set_graph(GraphLabel {
        ranksep: 300.0,
        ..Default::default()
    });
    g.set_default_edge_label(EdgeLabel::default);

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
            width: 75.0,
            height: 200.0,
            ..Default::default()
        },
    );
    g.set_edge_with_label(
        "a",
        "b",
        EdgeLabel {
            width: 60.0,
            height: 70.0,
            minlen: 2,
            labelpos: LabelPos::C,
            ..Default::default()
        },
    );

    layout_dagreish(&mut g);

    let edge = g.edge("a", "b", None).unwrap();
    assert_eq!(edge.x, Some(75.0 / 2.0));
    assert!(edge.y.unwrap() > g.node("a").unwrap().y.unwrap());
    assert!(edge.y.unwrap() < g.node("b").unwrap().y.unwrap());
}

#[cfg(feature = "dagreish")]
#[test]
fn layout_dagreish_can_layout_a_short_cycle() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: true,
        compound: true,
        ..Default::default()
    });
    g.set_graph(GraphLabel {
        ranksep: 200.0,
        ..Default::default()
    });
    g.set_default_edge_label(EdgeLabel::default);

    g.set_node(
        "a",
        NodeLabel {
            width: 100.0,
            height: 100.0,
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            width: 100.0,
            height: 100.0,
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
    g.set_edge("b", "a");

    layout_dagreish(&mut g);

    assert_eq!(
        coords(&g),
        [
            ("a".to_string(), (100.0 / 2.0, 100.0 / 2.0)),
            ("b".to_string(), (100.0 / 2.0, 100.0 + 200.0 + 100.0 / 2.0)),
        ]
        .into()
    );

    let ab = g.edge("a", "b", None).unwrap();
    let ba = g.edge("b", "a", None).unwrap();
    assert!(ab.points[1].y > ab.points[0].y);
    assert!(ba.points[0].y > ba.points[1].y);
}

#[test]
fn layout_adds_rectangle_intersects_for_edges() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: true,
        compound: true,
        ..Default::default()
    });
    g.set_graph(GraphLabel::default());
    g.set_default_edge_label(EdgeLabel::default);

    g.graph_mut().ranksep = 200.0;
    g.set_node(
        "a",
        NodeLabel {
            width: 100.0,
            height: 100.0,
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            width: 100.0,
            height: 100.0,
            ..Default::default()
        },
    );
    g.set_edge("a", "b");

    layout(&mut g);
    let points = &g.edge("a", "b", None).unwrap().points;
    assert_eq!(
        points.as_slice(),
        [
            Point { x: 50.0, y: 100.0 },
            Point {
                x: 50.0,
                y: 100.0 + 200.0 / 2.0,
            },
            Point {
                x: 50.0,
                y: 100.0 + 200.0,
            },
        ]
    );
}

#[test]
fn layout_adds_rectangle_intersects_for_edges_spanning_multiple_ranks() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: true,
        compound: true,
        ..Default::default()
    });
    g.set_graph(GraphLabel::default());
    g.set_default_edge_label(EdgeLabel::default);

    g.graph_mut().ranksep = 200.0;
    g.set_node(
        "a",
        NodeLabel {
            width: 100.0,
            height: 100.0,
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            width: 100.0,
            height: 100.0,
            ..Default::default()
        },
    );
    g.set_edge_with_label(
        "a",
        "b",
        EdgeLabel {
            minlen: 2,
            ..Default::default()
        },
    );

    layout(&mut g);
    let points = &g.edge("a", "b", None).unwrap().points;
    assert_eq!(
        points.as_slice(),
        [
            Point { x: 50.0, y: 100.0 },
            Point {
                x: 50.0,
                y: 100.0 + 200.0 / 2.0,
            },
            Point {
                x: 50.0,
                y: 100.0 + 400.0 / 2.0,
            },
            Point {
                x: 50.0,
                y: 100.0 + 600.0 / 2.0,
            },
            Point {
                x: 50.0,
                y: 100.0 + 800.0 / 2.0,
            },
        ]
    );
}

#[test]
fn layout_can_apply_an_offset() {
    for rankdir in [RankDir::TB, RankDir::BT, RankDir::LR, RankDir::RL] {
        let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
            multigraph: true,
            compound: true,
            ..Default::default()
        });
        g.set_graph(GraphLabel {
            rankdir,
            nodesep: 10.0,
            ranksep: 10.0,
            edgesep: 10.0,
            ..Default::default()
        });
        g.set_default_edge_label(EdgeLabel::default);

        for id in ["a", "b", "c", "d"] {
            g.set_node(
                id,
                NodeLabel {
                    width: 10.0,
                    height: 10.0,
                    ..Default::default()
                },
            );
        }
        g.set_edge_with_label(
            "a",
            "b",
            EdgeLabel {
                width: 10.0,
                height: 10.0,
                labelpos: LabelPos::L,
                labeloffset: 1000.0,
                ..Default::default()
            },
        );
        g.set_edge_with_label(
            "c",
            "d",
            EdgeLabel {
                width: 10.0,
                height: 10.0,
                labelpos: LabelPos::R,
                labeloffset: 1000.0,
                ..Default::default()
            },
        );

        layout(&mut g);

        let e1 = g.edge("a", "b", None).unwrap();
        let e2 = g.edge("c", "d", None).unwrap();
        if rankdir == RankDir::TB || rankdir == RankDir::BT {
            assert_eq!(e1.x.unwrap() - e1.points[0].x, -1000.0 - 10.0 / 2.0);
            assert_eq!(e2.x.unwrap() - e2.points[0].x, 1000.0 + 10.0 / 2.0);
        } else {
            assert_eq!(e1.y.unwrap() - e1.points[0].y, -1000.0 - 10.0 / 2.0);
            assert_eq!(e2.y.unwrap() - e2.points[0].y, 1000.0 + 10.0 / 2.0);
        }
    }
}

#[test]
fn layout_can_layout_an_edge_with_a_long_label() {
    for rankdir in [RankDir::TB, RankDir::BT, RankDir::LR, RankDir::RL] {
        let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
            multigraph: true,
            compound: true,
            ..Default::default()
        });
        g.set_graph(GraphLabel {
            rankdir,
            nodesep: 10.0,
            ranksep: 50.0,
            edgesep: 10.0,
            ..Default::default()
        });
        g.set_default_edge_label(EdgeLabel::default);

        for v in ["a", "b", "c", "d"] {
            g.set_node(
                v,
                NodeLabel {
                    width: 10.0,
                    height: 10.0,
                    ..Default::default()
                },
            );
        }
        g.set_edge_with_label(
            "a",
            "c",
            EdgeLabel {
                width: 2000.0,
                height: 10.0,
                ..Default::default()
            },
        );
        g.set_edge_with_label(
            "b",
            "d",
            EdgeLabel {
                width: 1.0,
                height: 1.0,
                ..Default::default()
            },
        );

        layout(&mut g);

        if rankdir == RankDir::TB || rankdir == RankDir::BT {
            let p1 = g.edge("a", "c", None).unwrap();
            let p2 = g.edge("b", "d", None).unwrap();
            assert!((p1.x.unwrap() - p2.x.unwrap()).abs() > 1000.0);
        } else {
            let p1 = g.node("a").unwrap();
            let p2 = g.node("c").unwrap();
            assert!((p1.x.unwrap() - p2.x.unwrap()).abs() > 1000.0);
        }
    }
}

#[test]
fn layout_can_layout_a_self_loop() {
    for rankdir in [RankDir::TB, RankDir::BT, RankDir::LR, RankDir::RL] {
        let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
            multigraph: true,
            compound: true,
            ..Default::default()
        });
        g.set_graph(GraphLabel {
            rankdir,
            nodesep: 50.0,
            ranksep: 50.0,
            edgesep: 75.0,
            ..Default::default()
        });
        g.set_default_edge_label(EdgeLabel::default);

        g.set_node(
            "a",
            NodeLabel {
                width: 100.0,
                height: 100.0,
                ..Default::default()
            },
        );
        g.set_edge_with_label(
            "a",
            "a",
            EdgeLabel {
                width: 50.0,
                height: 50.0,
                ..Default::default()
            },
        );

        layout(&mut g);
        let node_a = g.node("a").unwrap();
        let points = &g.edge("a", "a", None).unwrap().points;
        assert_eq!(points.len(), 7);
        for p in points {
            if rankdir != RankDir::LR && rankdir != RankDir::RL {
                assert!(p.x > node_a.x.unwrap());
                assert!((p.y - node_a.y.unwrap()).abs() <= node_a.height / 2.0);
            } else {
                assert!(p.y > node_a.y.unwrap());
                assert!((p.x - node_a.x.unwrap()).abs() <= node_a.width / 2.0);
            }
        }
    }
}

#[test]
fn layout_can_layout_a_graph_with_subgraphs() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: true,
        compound: true,
        ..Default::default()
    });
    g.set_graph(GraphLabel::default());
    g.set_default_edge_label(EdgeLabel::default);

    g.set_node(
        "a",
        NodeLabel {
            width: 50.0,
            height: 50.0,
            ..Default::default()
        },
    );
    g.set_parent("a", "sg1");
    layout(&mut g);

    // Cluster node should exist but should not be positioned by the layout engine.
    assert!(g.has_node("sg1"));
    let sg = g.node("sg1").unwrap();
    assert_eq!(sg.x, None);
    assert_eq!(sg.y, None);
}

#[test]
fn layout_minimizes_the_height_of_subgraphs() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: true,
        compound: true,
        ..Default::default()
    });
    g.set_graph(GraphLabel::default());
    g.set_default_edge_label(EdgeLabel::default);

    for v in ["a", "b", "c", "d", "x", "y"] {
        g.set_node(
            v,
            NodeLabel {
                width: 50.0,
                height: 50.0,
                ..Default::default()
            },
        );
    }

    // setPath(["a", "b", "c", "d"])
    g.set_edge("a", "b");
    g.set_edge("b", "c");
    g.set_edge("c", "d");

    g.set_edge_with_label(
        "a",
        "x",
        EdgeLabel {
            weight: 100.0,
            ..Default::default()
        },
    );
    g.set_edge_with_label(
        "y",
        "d",
        EdgeLabel {
            weight: 100.0,
            ..Default::default()
        },
    );
    g.set_parent("x", "sg");
    g.set_parent("y", "sg");

    layout(&mut g);
    assert_eq!(g.node("x").unwrap().y, g.node("y").unwrap().y);
}

#[cfg(feature = "dagreish")]
#[test]
fn layout_dagreish_minimizes_separation_between_nodes_not_adjacent_to_subgraphs() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: true,
        compound: true,
        ..Default::default()
    });
    g.set_graph(GraphLabel::default());
    g.set_default_edge_label(EdgeLabel::default);

    for v in ["a", "b", "c"] {
        g.set_node(
            v,
            NodeLabel {
                width: 50.0,
                height: 50.0,
                ..Default::default()
            },
        );
    }
    g.set_edge("a", "b");
    g.set_edge("b", "c");
    g.ensure_node("sg");
    g.set_parent("c", "sg");

    layout_dagreish(&mut g);

    assert_eq!(
        g.node("b").unwrap().y.unwrap() - g.node("a").unwrap().y.unwrap(),
        100.0
    );
}

#[cfg(feature = "dagreish")]
#[test]
fn layout_dagreish_can_layout_subgraphs_with_different_rankdirs() {
    for rankdir in [RankDir::TB, RankDir::BT, RankDir::LR, RankDir::RL] {
        let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
            multigraph: true,
            compound: true,
            ..Default::default()
        });
        g.set_graph(GraphLabel {
            rankdir,
            ..Default::default()
        });
        g.set_default_edge_label(EdgeLabel::default);

        g.set_node(
            "a",
            NodeLabel {
                width: 50.0,
                height: 50.0,
                ..Default::default()
            },
        );
        g.ensure_node("sg");
        g.set_parent("a", "sg");

        layout_dagreish(&mut g);

        let sg = g.node("sg").unwrap();
        assert!(sg.width > 50.0);
        assert!(sg.height > 50.0);
        assert!(sg.x.unwrap() > 50.0 / 2.0);
        assert!(sg.y.unwrap() > 50.0 / 2.0);
    }
}

#[cfg(feature = "dagreish")]
#[test]
fn layout_dagreish_adds_dimensions_to_graph() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: true,
        compound: true,
        ..Default::default()
    });
    g.set_graph(GraphLabel::default());
    g.set_default_edge_label(EdgeLabel::default);

    g.set_node(
        "a",
        NodeLabel {
            width: 100.0,
            height: 50.0,
            ..Default::default()
        },
    );

    layout_dagreish(&mut g);

    assert_eq!(g.graph().width, 100.0);
    assert_eq!(g.graph().height, 50.0);
}

#[cfg(feature = "dagreish")]
#[test]
fn layout_dagreish_graph_dimensions_include_margins() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: true,
        compound: true,
        ..Default::default()
    });
    g.set_graph(GraphLabel {
        marginx: 8.0,
        marginy: 10.0,
        ..Default::default()
    });
    g.set_default_edge_label(EdgeLabel::default);

    g.set_node(
        "a",
        NodeLabel {
            width: 100.0,
            height: 50.0,
            ..Default::default()
        },
    );

    layout_dagreish(&mut g);

    let a = g.node("a").unwrap();
    assert_eq!(a.x, Some(50.0 + 8.0));
    assert_eq!(a.y, Some(25.0 + 10.0));
    assert_eq!(g.graph().width, 100.0 + 8.0 * 2.0);
    assert_eq!(g.graph().height, 50.0 + 10.0 * 2.0);
}

#[cfg(feature = "dagreish")]
#[test]
fn layout_dagreish_keeps_node_coordinates_in_graph_bounding_box_for_rankdirs() {
    for rankdir in [RankDir::TB, RankDir::BT, RankDir::LR, RankDir::RL] {
        let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
            multigraph: true,
            compound: true,
            ..Default::default()
        });
        g.set_graph(GraphLabel {
            rankdir,
            ..Default::default()
        });
        g.set_default_edge_label(EdgeLabel::default);

        g.set_node(
            "a",
            NodeLabel {
                width: 100.0,
                height: 200.0,
                ..Default::default()
            },
        );

        layout_dagreish(&mut g);

        let a = g.node("a").unwrap();
        assert_eq!(a.x, Some(100.0 / 2.0));
        assert_eq!(a.y, Some(200.0 / 2.0));
        assert_eq!(g.graph().width, 100.0);
        assert_eq!(g.graph().height, 200.0);
        assert!(a.x.unwrap() - a.width / 2.0 >= 0.0);
        assert!(a.x.unwrap() + a.width / 2.0 <= g.graph().width);
        assert!(a.y.unwrap() - a.height / 2.0 >= 0.0);
        assert!(a.y.unwrap() + a.height / 2.0 <= g.graph().height);
    }
}

#[cfg(feature = "dagreish")]
#[test]
fn layout_dagreish_keeps_left_edge_label_coordinates_in_graph_bounding_box_for_rankdirs() {
    for rankdir in [RankDir::TB, RankDir::BT, RankDir::LR, RankDir::RL] {
        let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
            multigraph: true,
            compound: true,
            ..Default::default()
        });
        g.set_graph(GraphLabel {
            rankdir,
            ..Default::default()
        });
        g.set_default_edge_label(EdgeLabel::default);

        for v in ["a", "b"] {
            g.set_node(
                v,
                NodeLabel {
                    width: 100.0,
                    height: 100.0,
                    ..Default::default()
                },
            );
        }
        g.set_edge_with_label(
            "a",
            "b",
            EdgeLabel {
                width: 1000.0,
                height: 2000.0,
                labelpos: LabelPos::L,
                labeloffset: 0.0,
                ..Default::default()
            },
        );

        layout_dagreish(&mut g);

        let edge = g.edge("a", "b", None).unwrap();
        if matches!(rankdir, RankDir::TB | RankDir::BT) {
            assert_eq!(edge.x, Some(1000.0 / 2.0));
        } else {
            assert_eq!(edge.y, Some(2000.0 / 2.0));
        }
        assert!(edge.x.unwrap() - edge.width / 2.0 >= 0.0);
        assert!(edge.x.unwrap() + edge.width / 2.0 <= g.graph().width);
        assert!(edge.y.unwrap() - edge.height / 2.0 >= 0.0);
        assert!(edge.y.unwrap() + edge.height / 2.0 <= g.graph().height);
    }
}
