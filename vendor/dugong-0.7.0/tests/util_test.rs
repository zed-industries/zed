use dugong::graphlib::{Graph, GraphOptions};
use dugong::util;
use dugong::{EdgeLabel, GraphLabel, NodeLabel, Point};
use serde_json::json;
use std::collections::BTreeMap;

#[test]
fn util_simplify_copies_without_change_a_graph_with_no_multi_edges() {
    let mut g: Graph<serde_json::Value, EdgeLabel, serde_json::Value> = Graph::new(GraphOptions {
        multigraph: true,
        compound: false,
        ..Default::default()
    });
    g.set_graph(serde_json::Value::Null);
    g.set_edge_with_label(
        "a",
        "b",
        EdgeLabel {
            weight: 1.0,
            minlen: 1,
            ..Default::default()
        },
    );

    let g2 = util::simplify(&g);
    assert_eq!(
        g2.edge("a", "b", None).cloned(),
        Some(EdgeLabel {
            weight: 1.0,
            minlen: 1,
            ..Default::default()
        })
    );
    assert_eq!(g2.edge_count(), 1);
}

#[test]
fn util_simplify_collapses_multi_edges() {
    let mut g: Graph<serde_json::Value, EdgeLabel, serde_json::Value> = Graph::new(GraphOptions {
        multigraph: true,
        compound: false,
        ..Default::default()
    });
    g.set_graph(serde_json::Value::Null);
    g.set_edge_with_label(
        "a",
        "b",
        EdgeLabel {
            weight: 1.0,
            minlen: 1,
            ..Default::default()
        },
    );
    g.set_edge_named(
        "a",
        "b",
        Some("multi"),
        Some(EdgeLabel {
            weight: 2.0,
            minlen: 2,
            ..Default::default()
        }),
    );

    let g2 = util::simplify(&g);
    assert!(!g2.options().multigraph);
    assert_eq!(
        g2.edge("a", "b", None).cloned(),
        Some(EdgeLabel {
            weight: 3.0,
            minlen: 2,
            ..Default::default()
        })
    );
    assert_eq!(g2.edge_count(), 1);
}

#[test]
fn util_simplify_copies_the_graph_object() {
    let mut g: Graph<serde_json::Value, EdgeLabel, serde_json::Value> = Graph::new(GraphOptions {
        multigraph: true,
        compound: false,
        ..Default::default()
    });
    g.set_graph(json!({ "foo": "bar" }));
    let g2 = util::simplify(&g);
    assert_eq!(g2.graph(), &json!({ "foo": "bar" }));
}

#[test]
fn util_as_non_compound_graph_copies_all_nodes() {
    let mut g: Graph<serde_json::Value, serde_json::Value, serde_json::Value> =
        Graph::new(GraphOptions {
            compound: true,
            multigraph: true,
            ..Default::default()
        });
    g.set_node("a", json!({ "foo": "bar" }));
    g.set_node("b", serde_json::Value::Null);

    let g2 = util::as_non_compound_graph(&g);
    assert_eq!(g2.node("a"), Some(&json!({ "foo": "bar" })));
    assert!(g2.has_node("b"));
}

#[test]
fn util_as_non_compound_graph_copies_all_edges() {
    let mut g: Graph<serde_json::Value, serde_json::Value, serde_json::Value> =
        Graph::new(GraphOptions {
            compound: true,
            multigraph: true,
            ..Default::default()
        });
    g.set_edge_named("a", "b", None::<String>, Some(json!({ "foo": "bar" })));
    g.set_edge_named("a", "b", Some("multi"), Some(json!({ "foo": "baz" })));

    let g2 = util::as_non_compound_graph(&g);
    assert_eq!(g2.edge("a", "b", None), Some(&json!({ "foo": "bar" })));
    assert_eq!(
        g2.edge("a", "b", Some("multi")),
        Some(&json!({ "foo": "baz" }))
    );
}

#[test]
fn util_as_non_compound_graph_does_not_copy_compound_nodes() {
    let mut g: Graph<serde_json::Value, serde_json::Value, serde_json::Value> =
        Graph::new(GraphOptions {
            compound: true,
            multigraph: true,
            ..Default::default()
        });
    g.set_parent("a", "sg1");
    let g2 = util::as_non_compound_graph(&g);
    assert_eq!(g2.parent("a"), None);
    assert!(!g2.options().compound);
    assert!(!g2.has_node("sg1"));
}

#[test]
fn util_as_non_compound_graph_copies_the_graph_object() {
    let mut g: Graph<serde_json::Value, serde_json::Value, serde_json::Value> =
        Graph::new(GraphOptions {
            compound: true,
            multigraph: true,
            ..Default::default()
        });
    g.set_graph(json!({ "foo": "bar" }));
    let g2 = util::as_non_compound_graph(&g);
    assert_eq!(g2.graph(), &json!({ "foo": "bar" }));
}

#[test]
fn util_successor_weights_maps_a_node_to_its_successors_with_associated_weights() {
    let mut g: Graph<NodeLabel, EdgeLabel, serde_json::Value> = Graph::new(GraphOptions {
        multigraph: true,
        compound: false,
        ..Default::default()
    });
    g.set_edge_with_label(
        "a",
        "b",
        EdgeLabel {
            weight: 2.0,
            ..Default::default()
        },
    );
    g.set_edge_with_label(
        "b",
        "c",
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.set_edge_named(
        "b",
        "c",
        Some("multi"),
        Some(EdgeLabel {
            weight: 2.0,
            ..Default::default()
        }),
    );
    g.set_edge_named(
        "b",
        "d",
        Some("multi"),
        Some(EdgeLabel {
            weight: 1.0,
            ..Default::default()
        }),
    );

    let weights = util::successor_weights(&g);
    assert_eq!(
        weights.get("a").cloned(),
        Some([("b".to_string(), 2.0)].into())
    );
    assert_eq!(
        weights.get("b").cloned(),
        Some([("c".to_string(), 3.0), ("d".to_string(), 1.0)].into())
    );
    assert_eq!(weights.get("c").cloned(), Some(BTreeMap::new()));
    assert_eq!(weights.get("d").cloned(), Some(BTreeMap::new()));
}

#[test]
fn util_predecessor_weights_maps_a_node_to_its_predecessors_with_associated_weights() {
    let mut g: Graph<NodeLabel, EdgeLabel, serde_json::Value> = Graph::new(GraphOptions {
        multigraph: true,
        compound: false,
        ..Default::default()
    });
    g.set_edge_with_label(
        "a",
        "b",
        EdgeLabel {
            weight: 2.0,
            ..Default::default()
        },
    );
    g.set_edge_with_label(
        "b",
        "c",
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.set_edge_named(
        "b",
        "c",
        Some("multi"),
        Some(EdgeLabel {
            weight: 2.0,
            ..Default::default()
        }),
    );
    g.set_edge_named(
        "b",
        "d",
        Some("multi"),
        Some(EdgeLabel {
            weight: 1.0,
            ..Default::default()
        }),
    );

    let weights = util::predecessor_weights(&g);
    assert_eq!(weights.get("a").cloned(), Some(BTreeMap::new()));
    assert_eq!(
        weights.get("b").cloned(),
        Some([("a".to_string(), 2.0)].into())
    );
    assert_eq!(
        weights.get("c").cloned(),
        Some([("b".to_string(), 3.0)].into())
    );
    assert_eq!(
        weights.get("d").cloned(),
        Some([("b".to_string(), 1.0)].into())
    );
}

fn expect_intersects(rect: util::Rect, point: Point) {
    let cross = util::intersect_rect(rect, point);
    if cross.x != point.x {
        let m = (cross.y - point.y) / (cross.x - point.x);
        let lhs = cross.y - rect.y;
        let rhs = m * (cross.x - rect.x);
        assert!((lhs - rhs).abs() < 1e-9);
    }
}

fn expect_touches_border(rect: util::Rect, point: Point) {
    let cross = util::intersect_rect(rect, point);
    if (rect.x - cross.x).abs() != rect.width / 2.0 {
        assert!(((rect.y - cross.y).abs() - rect.height / 2.0).abs() < 1e-9);
    }
}

#[test]
fn util_intersect_rect_creates_a_slope_that_will_intersect_the_rectangles_center() {
    let rect = util::Rect {
        x: 0.0,
        y: 0.0,
        width: 1.0,
        height: 1.0,
    };
    expect_intersects(rect, Point { x: 2.0, y: 6.0 });
    expect_intersects(rect, Point { x: 2.0, y: -6.0 });
    expect_intersects(rect, Point { x: 6.0, y: 2.0 });
    expect_intersects(rect, Point { x: -6.0, y: 2.0 });
    expect_intersects(rect, Point { x: 5.0, y: 0.0 });
    expect_intersects(rect, Point { x: 0.0, y: 5.0 });
}

#[test]
fn util_intersect_rect_touches_the_border_of_the_rectangle() {
    let rect = util::Rect {
        x: 0.0,
        y: 0.0,
        width: 1.0,
        height: 1.0,
    };
    expect_touches_border(rect, Point { x: 2.0, y: 6.0 });
    expect_touches_border(rect, Point { x: 2.0, y: -6.0 });
    expect_touches_border(rect, Point { x: 6.0, y: 2.0 });
    expect_touches_border(rect, Point { x: -6.0, y: 2.0 });
    expect_touches_border(rect, Point { x: 5.0, y: 0.0 });
    expect_touches_border(rect, Point { x: 0.0, y: 5.0 });
}

#[test]
fn util_intersect_rect_is_defined_if_the_point_is_at_the_center_of_the_rectangle() {
    let rect = util::Rect {
        x: 0.0,
        y: 0.0,
        width: 1.0,
        height: 1.0,
    };
    let p = util::intersect_rect(rect, Point { x: 0.0, y: 0.0 });
    assert_eq!(p, Point { x: 0.5, y: 0.0 });
}

#[test]
fn util_build_layer_matrix_creates_a_matrix_based_on_rank_and_order_of_nodes_in_the_graph() {
    let mut g: Graph<NodeLabel, EdgeLabel, serde_json::Value> = Graph::new(GraphOptions {
        multigraph: false,
        compound: false,
        ..Default::default()
    });
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(0),
            order: Some(0),
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            rank: Some(0),
            order: Some(1),
            ..Default::default()
        },
    );
    g.set_node(
        "c",
        NodeLabel {
            rank: Some(1),
            order: Some(0),
            ..Default::default()
        },
    );
    g.set_node(
        "d",
        NodeLabel {
            rank: Some(1),
            order: Some(1),
            ..Default::default()
        },
    );
    g.set_node(
        "e",
        NodeLabel {
            rank: Some(2),
            order: Some(0),
            ..Default::default()
        },
    );

    assert_eq!(
        util::build_layer_matrix(&g),
        vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "d".to_string()],
            vec!["e".to_string()],
        ]
    );
}

#[test]
fn util_time_logs_timing_information() {
    let mut buf: Vec<u8> = Vec::new();
    util::time_to_writer("foo", &mut buf, || {});
    let output = String::from_utf8(buf).unwrap();
    assert!(output.starts_with("foo time: "));
    assert!(output.trim_end().ends_with("ms"));
}

#[test]
fn util_time_returns_the_value_from_the_evaluated_function() {
    let mut buf: Vec<u8> = Vec::new();
    let v = util::time_to_writer("foo", &mut buf, || "bar");
    assert_eq!(v, "bar");
}

#[test]
fn util_normalize_ranks_adjusts_ranks_such_that_all_are_gte_0_and_at_least_one_is_0() {
    let mut g: Graph<NodeLabel, EdgeLabel, serde_json::Value> = Graph::new(GraphOptions {
        multigraph: false,
        compound: false,
        ..Default::default()
    });
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(3),
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            rank: Some(2),
            ..Default::default()
        },
    );
    g.set_node(
        "c",
        NodeLabel {
            rank: Some(4),
            ..Default::default()
        },
    );

    util::normalize_ranks(&mut g);
    assert_eq!(g.node("a").unwrap().rank, Some(1));
    assert_eq!(g.node("b").unwrap().rank, Some(0));
    assert_eq!(g.node("c").unwrap().rank, Some(2));
}

#[test]
fn util_normalize_ranks_works_for_negative_ranks() {
    let mut g: Graph<NodeLabel, EdgeLabel, serde_json::Value> = Graph::new(GraphOptions {
        multigraph: false,
        compound: false,
        ..Default::default()
    });
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(-3),
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            rank: Some(-2),
            ..Default::default()
        },
    );
    util::normalize_ranks(&mut g);
    assert_eq!(g.node("a").unwrap().rank, Some(0));
    assert_eq!(g.node("b").unwrap().rank, Some(1));
}

#[test]
fn util_normalize_ranks_does_not_assign_a_rank_to_subgraphs() {
    let mut g: Graph<NodeLabel, EdgeLabel, serde_json::Value> = Graph::new(GraphOptions {
        multigraph: false,
        compound: true,
        ..Default::default()
    });
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(0),
            ..Default::default()
        },
    );
    g.set_node("sg", NodeLabel::default());
    g.set_parent("a", "sg");

    util::normalize_ranks(&mut g);
    assert_eq!(g.node("sg").unwrap().rank, None);
    assert_eq!(g.node("a").unwrap().rank, Some(0));
}

#[test]
fn util_remove_empty_ranks_removes_border_ranks_without_any_nodes() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: false,
        compound: false,
        ..Default::default()
    });
    g.set_graph(GraphLabel {
        node_rank_factor: Some(4),
        ..Default::default()
    });
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(0),
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            rank: Some(4),
            ..Default::default()
        },
    );
    util::remove_empty_ranks(&mut g);
    assert_eq!(g.node("a").unwrap().rank, Some(0));
    assert_eq!(g.node("b").unwrap().rank, Some(1));
}

#[test]
fn util_remove_empty_ranks_does_not_remove_non_border_ranks() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: false,
        compound: false,
        ..Default::default()
    });
    g.set_graph(GraphLabel {
        node_rank_factor: Some(4),
        ..Default::default()
    });
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(0),
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            rank: Some(8),
            ..Default::default()
        },
    );
    util::remove_empty_ranks(&mut g);
    assert_eq!(g.node("a").unwrap().rank, Some(0));
    assert_eq!(g.node("b").unwrap().rank, Some(2));
}

#[test]
fn util_remove_empty_ranks_handles_parents_with_undefined_ranks() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: false,
        compound: true,
        ..Default::default()
    });
    g.set_graph(GraphLabel {
        node_rank_factor: Some(3),
        ..Default::default()
    });
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(0),
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            rank: Some(6),
            ..Default::default()
        },
    );
    g.set_node("sg", NodeLabel::default());
    g.set_parent("a", "sg");
    util::remove_empty_ranks(&mut g);
    assert_eq!(g.node("a").unwrap().rank, Some(0));
    assert_eq!(g.node("b").unwrap().rank, Some(2));
    assert_eq!(g.node("sg").unwrap().rank, None);
}

#[test]
fn util_range_builds_an_array_to_the_limit() {
    let range = util::range(4);
    assert_eq!(range.len(), 4);
    assert_eq!(range.iter().sum::<i32>(), 6);
}

#[test]
fn util_range_builds_an_array_with_a_start() {
    let range = util::range_start(2, 4);
    assert_eq!(range.len(), 2);
    assert_eq!(range.iter().sum::<i32>(), 5);
}

#[test]
fn util_range_builds_an_array_with_a_negative_step() {
    let range = util::range_with(5, -1, -1);
    assert_eq!(range[0], 5);
    assert_eq!(range[5], 0);
}

#[test]
fn util_map_values_creates_an_object_with_the_same_keys() {
    let users: BTreeMap<String, serde_json::Value> = [
        ("fred".to_string(), json!({ "user": "fred", "age": 40 })),
        (
            "pebbles".to_string(),
            json!({ "user": "pebbles", "age": 1 }),
        ),
    ]
    .into();

    let ages = util::map_values(&users, |user, _k| {
        user.get("age").unwrap().as_i64().unwrap()
    });
    assert_eq!(ages.get("fred").copied(), Some(40));
    assert_eq!(ages.get("pebbles").copied(), Some(1));
}

#[test]
fn util_map_values_can_take_a_property_name() {
    let users: BTreeMap<String, serde_json::Value> = [
        ("fred".to_string(), json!({ "user": "fred", "age": 40 })),
        (
            "pebbles".to_string(),
            json!({ "user": "pebbles", "age": 1 }),
        ),
    ]
    .into();

    let ages = util::map_values_prop(&users, "age");
    assert_eq!(ages.get("fred").cloned(), Some(json!(40)));
    assert_eq!(ages.get("pebbles").cloned(), Some(json!(1)));
}
