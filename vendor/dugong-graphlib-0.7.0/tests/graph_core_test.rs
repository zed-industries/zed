use dugong_graphlib::{EdgeKey, Graph, GraphError, GraphOptions};

fn sorted(mut values: Vec<&str>) -> Vec<&str> {
    values.sort();
    values
}

fn sorted_owned(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values
}

fn sorted_edge_tuples(edges: Vec<EdgeKey>) -> Vec<(String, String, Option<String>)> {
    let mut out: Vec<(String, String, Option<String>)> =
        edges.into_iter().map(|e| (e.v, e.w, e.name)).collect();
    out.sort();
    out
}

#[test]
fn graph_initial_state_uses_default_directed_simple_options() {
    let g: Graph<(), (), Option<String>> = Graph::new(GraphOptions::default());

    assert_eq!(g.node_count(), 0);
    assert_eq!(g.edge_count(), 0);
    assert!(g.is_directed());
    assert!(!g.is_compound());
    assert!(!g.is_multigraph());
    assert_eq!(g.graph(), &None);
}

#[test]
fn graph_options_can_enable_undirected_compound_or_multigraph_modes() {
    let undirected: Graph<(), (), ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    let compound: Graph<(), (), ()> = Graph::new(GraphOptions {
        compound: true,
        ..Default::default()
    });
    let multigraph: Graph<(), (), ()> = Graph::new(GraphOptions {
        multigraph: true,
        ..Default::default()
    });

    assert!(!undirected.is_directed());
    assert!(compound.is_compound());
    assert!(multigraph.is_multigraph());
}

#[test]
fn graph_label_can_be_set_and_read() {
    let mut g: Graph<(), (), Option<String>> = Graph::new(GraphOptions::default());

    g.set_graph(Some("graph label".to_string()));

    assert_eq!(g.graph().as_deref(), Some("graph label"));
}

#[test]
fn nodes_returns_inserted_node_ids() {
    let mut g: Graph<Option<i32>, (), ()> = Graph::new(GraphOptions::default());
    g.ensure_node("a");
    g.ensure_node("b");

    assert_eq!(sorted(g.nodes().collect()), vec!["a", "b"]);
}

#[test]
fn sources_returns_nodes_without_in_edges() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_path(&["a", "b", "c"]);
    g.ensure_node("d");

    assert_eq!(sorted(g.sources()), vec!["a", "d"]);
}

#[test]
fn sinks_returns_nodes_without_out_edges() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_path(&["a", "b", "c"]);
    g.ensure_node("d");

    assert_eq!(sorted(g.sinks()), vec!["c", "d"]);
}

#[test]
fn filter_nodes_copies_selected_graph_labels_edges_and_options() {
    let mut g: Graph<Option<i32>, Option<i32>, Option<String>> = Graph::new(GraphOptions {
        multigraph: true,
        compound: true,
        ..Default::default()
    });
    g.set_graph(Some("graph label".to_string()));
    g.set_node("a", Some(123));
    g.set_path(&["a", "b", "c"]);
    g.set_edge_named("a", "c", Some("named"), Some(Some(456)));

    let g2 = g.filter_nodes(|_| true);

    assert!(g2.is_directed());
    assert!(g2.is_multigraph());
    assert!(g2.is_compound());
    assert_eq!(g2.graph().as_deref(), Some("graph label"));
    assert_eq!(sorted(g2.nodes().collect()), vec!["a", "b", "c"]);
    assert_eq!(sorted(g2.successors("a")), vec!["b", "c"]);
    assert_eq!(sorted(g2.successors("b")), vec!["c"]);
    assert_eq!(g2.node("a"), Some(&Some(123)));
    assert_eq!(g2.edge("a", "c", Some("named")), Some(&Some(456)));

    let undirected: Graph<(), (), ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    assert!(!undirected.filter_nodes(|_| true).is_directed());

    let simple: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    assert!(!simple.filter_nodes(|_| true).is_multigraph());
    assert!(!simple.filter_nodes(|_| true).is_compound());
}

#[test]
fn filter_nodes_drops_rejected_nodes_and_incident_edges() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_edge("a", "b");

    let g2 = g.filter_nodes(|v| v == "a");
    let empty = g.filter_nodes(|_| false);

    assert_eq!(g2.nodes().collect::<Vec<_>>(), vec!["a"]);
    assert!(g2.edge_keys().is_empty());
    assert!(empty.nodes().collect::<Vec<_>>().is_empty());
    assert!(empty.edge_keys().is_empty());
}

#[test]
fn filter_nodes_preserves_compound_subgraphs_and_promotes_missing_parent() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions {
        compound: true,
        ..Default::default()
    });
    g.set_parent("a", "parent");
    g.set_parent("parent", "root");

    let full = g.filter_nodes(|_| true);
    let promoted = g.filter_nodes(|v| v != "parent");

    assert_eq!(full.parent("a"), Some("parent"));
    assert_eq!(full.parent("parent"), Some("root"));
    assert_eq!(promoted.parent("a"), Some("root"));
}

#[test]
fn ensure_node_uses_default_label_for_new_nodes() {
    let mut g: Graph<Option<i32>, (), ()> = Graph::new(GraphOptions::default());
    g.set_default_node_label(|| Some(7));

    g.ensure_node("a");

    assert_eq!(g.node("a"), Some(&Some(7)));
}

#[test]
fn default_node_label_can_read_node_id() {
    let mut g: Graph<Option<String>, (), ()> = Graph::new(GraphOptions::default());
    g.set_default_node_label_with_id(|v| Some(format!("{v}-foo")));

    g.ensure_node("a");

    assert_eq!(g.node("a").and_then(|v| v.as_deref()), Some("a-foo"));
}

#[test]
fn default_node_label_is_not_used_if_explicit_label_is_set() {
    let mut g: Graph<Option<i32>, (), ()> = Graph::new(GraphOptions::default());
    g.set_default_node_label(|| Some(7));

    g.set_node("a", Some(3));

    assert_eq!(g.node("a"), Some(&Some(3)));
}

#[test]
fn ensure_node_does_not_change_existing_node_label() {
    let mut g: Graph<Option<i32>, (), ()> = Graph::new(GraphOptions::default());
    g.set_node("a", Some(3));
    g.set_default_node_label(|| Some(7));

    g.ensure_node("a");

    assert_eq!(g.node("a"), Some(&Some(3)));
}

#[test]
fn set_nodes_uses_default_labels_without_changing_existing_nodes() {
    let mut g: Graph<Option<String>, (), ()> = Graph::new(GraphOptions::default());
    g.set_default_node_label_with_id(|v| Some(format!("{v}-default")));
    g.set_node("a", Some("existing".to_string()));

    g.set_nodes(&["a", "b", "c"]);

    assert_eq!(g.node("a").and_then(|v| v.as_deref()), Some("existing"));
    assert_eq!(g.node("b").and_then(|v| v.as_deref()), Some("b-default"));
    assert_eq!(g.node("c").and_then(|v| v.as_deref()), Some("c-default"));
}

#[test]
fn set_nodes_with_label_sets_and_updates_all_node_labels() {
    let mut g: Graph<Option<String>, (), ()> = Graph::new(GraphOptions::default());

    g.set_nodes_with_label(&["a", "b", "c"], Some("foo".to_string()));

    assert_eq!(g.node("a").and_then(|v| v.as_deref()), Some("foo"));
    assert_eq!(g.node("b").and_then(|v| v.as_deref()), Some("foo"));
    assert_eq!(g.node("c").and_then(|v| v.as_deref()), Some("foo"));

    g.set_nodes_with_label(&["a", "b", "c"], Some("bar".to_string()));

    assert_eq!(g.node("a").and_then(|v| v.as_deref()), Some("bar"));
    assert_eq!(g.node("b").and_then(|v| v.as_deref()), Some("bar"));
    assert_eq!(g.node("c").and_then(|v| v.as_deref()), Some("bar"));
}

#[test]
fn set_node_is_idempotent_for_existing_node() {
    let mut g: Graph<Option<i32>, (), ()> = Graph::new(GraphOptions::default());
    g.set_node("a", Some(1));
    g.set_node("a", Some(1));

    assert_eq!(g.node("a"), Some(&Some(1)));
    assert_eq!(g.node_count(), 1);
}

#[test]
fn set_node_with_optional_label_can_clear_label_without_removing_node() {
    let mut g: Graph<Option<&str>, (), ()> = Graph::new(GraphOptions::default());

    assert_eq!(g.node("a"), None);

    g.set_node("a", Some("foo"));
    assert!(g.has_node("a"));
    assert_eq!(g.node("a"), Some(&Some("foo")));
    assert_eq!(g.node_count(), 1);

    g.set_node("a", None);
    assert!(g.has_node("a"));
    assert_eq!(g.node("a"), Some(&None));
    assert_eq!(g.node_count(), 1);
}

#[test]
fn remove_node_is_idempotent_and_removes_incident_edges() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_edge("a", "b");
    g.set_edge("b", "c");

    assert!(g.remove_node("b"));
    assert!(!g.remove_node("b"));

    assert!(!g.has_node("b"));
    assert_eq!(g.edge_count(), 0);
}

#[test]
fn set_edge_creates_endpoint_nodes_and_uses_default_edge_label() {
    let mut g: Graph<(), Option<i32>, ()> = Graph::new(GraphOptions::default());
    g.set_default_edge_label(|| Some(9));

    g.set_edge("a", "b");

    assert!(g.has_node("a"));
    assert!(g.has_node("b"));
    assert_eq!(g.edge("a", "b", None), Some(&Some(9)));
    assert_eq!(g.edge_count(), 1);
}

#[test]
fn default_edge_label_can_read_endpoints_and_name() {
    let mut g: Graph<(), Option<String>, ()> = Graph::new(GraphOptions {
        multigraph: true,
        ..Default::default()
    });
    g.set_default_edge_label_with_endpoints(|v, w, name| {
        Some(format!("{v}-{w}-{}-foo", name.unwrap_or("none")))
    });

    g.set_edge_named("a", "b", Some("name"), None);

    assert_eq!(
        g.edge("a", "b", Some("name")).and_then(|v| v.as_deref()),
        Some("a-b-name-foo")
    );
}

#[test]
fn default_edge_label_does_not_change_existing_edge() {
    let mut g: Graph<(), Option<i32>, ()> = Graph::new(GraphOptions::default());
    g.set_edge("a", "b");
    g.set_default_edge_label(|| Some(9));

    assert_eq!(g.edge("a", "b", None), Some(&None));
}

#[test]
fn default_edge_label_is_not_used_if_explicit_label_is_set() {
    let mut g: Graph<(), Option<i32>, ()> = Graph::new(GraphOptions::default());
    g.set_default_edge_label(|| Some(9));

    g.set_edge_with_label("a", "b", Some(3));

    assert_eq!(g.edge("a", "b", None), Some(&Some(3)));
}

#[test]
fn default_edge_label_does_not_replace_existing_named_edge() {
    let mut g: Graph<(), Option<String>, ()> = Graph::new(GraphOptions {
        multigraph: true,
        ..Default::default()
    });
    g.set_edge_named("a", "b", Some("name"), Some(Some("old".to_string())));
    g.set_default_edge_label_with_endpoints(|_, _, _| Some("should not set this".to_string()));

    g.set_edge_named("a", "b", Some("name"), None);

    assert_eq!(
        g.edge("a", "b", Some("name")).and_then(|v| v.as_deref()),
        Some("old")
    );
}

#[test]
fn set_edge_with_label_updates_existing_edge_label() {
    let mut g: Graph<(), Option<i32>, ()> = Graph::new(GraphOptions::default());
    g.set_edge_with_label("a", "b", Some(1));
    g.set_edge_with_label("a", "b", Some(2));

    assert_eq!(g.edge("a", "b", None), Some(&Some(2)));
    assert_eq!(g.edge_count(), 1);
}

#[test]
fn set_edge_with_label_can_clear_optional_edge_label() {
    let mut g: Graph<(), Option<&str>, ()> = Graph::new(GraphOptions::default());
    g.set_edge_with_label("a", "b", Some("foo"));
    g.set_edge_with_label("a", "b", None);

    assert!(g.has_edge("a", "b", None));
    assert_eq!(g.edge("a", "b", None), Some(&None));
}

#[test]
fn set_edge_named_can_clear_optional_multiedge_label() {
    let mut g: Graph<(), Option<&str>, ()> = Graph::new(GraphOptions {
        multigraph: true,
        ..Default::default()
    });
    g.set_edge_named("a", "b", Some("name"), Some(Some("foo")));
    g.set_edge_named("a", "b", Some("name"), Some(None));

    assert!(g.has_edge("a", "b", Some("name")));
    assert_eq!(g.edge("a", "b", Some("name")), Some(&None));
}

#[test]
fn multigraph_preserves_named_edges() {
    let mut g: Graph<(), Option<i32>, ()> = Graph::new(GraphOptions {
        multigraph: true,
        ..Default::default()
    });

    g.set_edge_named("a", "b", Some("first"), Some(Some(1)));
    g.set_edge_named("a", "b", Some("second"), Some(Some(2)));

    assert_eq!(g.edge("a", "b", Some("first")), Some(&Some(1)));
    assert_eq!(g.edge("a", "b", Some("second")), Some(&Some(2)));
    assert_eq!(g.edge_count(), 2);
    assert!(!g.has_edge("a", "b", None));
}

#[test]
fn set_edge_named_is_noop_for_named_edge_on_non_multigraph() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());

    g.set_edge_named("a", "b", Some("name"), None);

    assert_eq!(g.node_count(), 0);
    assert_eq!(g.edge_count(), 0);
    assert!(!g.has_edge("a", "b", Some("name")));
}

#[test]
fn try_set_edge_named_reports_named_edge_on_non_multigraph() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());

    let err = match g.try_set_edge_named("a", "b", Some("name"), None) {
        Ok(_) => panic!("named edge on simple graph should report an error"),
        Err(err) => err,
    };

    assert_eq!(err, GraphError::NamedEdgeInNonMultigraph);
    assert_eq!(g.node_count(), 0);
    assert_eq!(g.edge_count(), 0);
}

#[test]
fn named_edge_queries_do_not_match_unnamed_edges_in_simple_graph() {
    let mut g: Graph<(), i32, ()> = Graph::new(GraphOptions::default());
    g.set_edge_with_label("a", "b", 5);

    assert!(g.has_edge("a", "b", None));
    assert!(!g.has_edge("a", "b", Some("name")));
    assert_eq!(g.edge("a", "b", Some("name")), None);
    assert!(!g.remove_edge("a", "b", Some("name")));
    assert!(g.has_edge("a", "b", None));
}

#[test]
fn edges_returns_inserted_edge_keys() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_edge("a", "b");
    g.set_edge("b", "c");

    assert_eq!(
        sorted_edge_tuples(g.edge_keys()),
        vec![
            ("a".to_string(), "b".to_string(), None),
            ("b".to_string(), "c".to_string(), None)
        ]
    );
}

#[test]
fn set_path_creates_path_edges() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());

    g.set_path(&["a", "b", "c"]);

    assert!(g.has_edge("a", "b", None));
    assert!(g.has_edge("b", "c", None));
}

#[test]
fn set_path_with_label_sets_and_updates_all_path_edge_labels() {
    let mut g: Graph<(), String, ()> = Graph::new(GraphOptions::default());

    g.set_path_with_label(&["a", "b", "c"], "foo".to_string());

    assert_eq!(g.edge("a", "b", None).map(String::as_str), Some("foo"));
    assert_eq!(g.edge("b", "c", None).map(String::as_str), Some("foo"));

    g.set_path_with_label(&["a", "b", "c"], "bar".to_string());

    assert_eq!(g.edge("a", "b", None).map(String::as_str), Some("bar"));
    assert_eq!(g.edge("b", "c", None).map(String::as_str), Some("bar"));
}

#[test]
fn set_edge_key_sets_simple_and_named_edge_labels() {
    let mut simple: Graph<(), String, ()> = Graph::new(GraphOptions::default());
    simple.set_edge_key(EdgeKey::new("a", "b", None::<String>), "value".to_string());

    assert_eq!(
        simple.edge("a", "b", None).map(String::as_str),
        Some("value")
    );

    let mut multi: Graph<(), String, ()> = Graph::new(GraphOptions {
        multigraph: true,
        ..Default::default()
    });
    multi.set_edge_key(EdgeKey::new("a", "b", Some("name")), "named".to_string());

    assert_eq!(
        multi.edge("a", "b", Some("name")).map(String::as_str),
        Some("named")
    );
}

#[test]
fn try_set_edge_key_reports_named_edge_on_non_multigraph() {
    let mut g: Graph<(), String, ()> = Graph::new(GraphOptions::default());

    let err = match g.try_set_edge_key(EdgeKey::new("a", "b", Some("name")), "value".to_string()) {
        Ok(_) => panic!("named edge key on simple graph should report an error"),
        Err(err) => err,
    };

    assert_eq!(err, GraphError::NamedEdgeInNonMultigraph);
    assert_eq!(g.node_count(), 0);
    assert_eq!(g.edge_count(), 0);
}

#[test]
fn edge_lookup_respects_direction_for_directed_graphs() {
    let mut g: Graph<(), i32, ()> = Graph::new(GraphOptions::default());
    g.set_edge_with_label("a", "b", 7);

    assert_eq!(g.edge("a", "b", None), Some(&7));
    assert_eq!(g.edge("b", "a", None), None);
    assert!(g.has_edge("a", "b", None));
    assert!(!g.has_edge("b", "a", None));
}

#[test]
fn edge_lookup_returns_none_for_missing_edges() {
    let g: Graph<(), i32, ()> = Graph::new(GraphOptions::default());

    assert_eq!(g.edge("a", "b", None), None);
    assert_eq!(g.edge("a", "b", Some("foo")), None);
    assert!(!g.has_edge("a", "b", None));
    assert!(!g.has_edge("a", "b", Some("foo")));
}

#[test]
fn edge_lookup_accepts_either_direction_for_undirected_graphs() {
    let mut g: Graph<(), i32, ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    g.set_edge_with_label("a", "b", 7);

    assert_eq!(g.edge("a", "b", None), Some(&7));
    assert_eq!(g.edge("b", "a", None), Some(&7));
    assert!(g.has_edge("a", "b", None));
    assert!(g.has_edge("b", "a", None));
}

#[test]
fn undirected_edges_follow_graphlib_string_order_for_stringified_ids() {
    let mut g: Graph<(), String, ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    g.set_edge_with_label("9", "10", "foo".to_string());

    assert_eq!(g.edge("9", "10", None).map(String::as_str), Some("foo"));
    assert_eq!(g.edge("10", "9", None).map(String::as_str), Some("foo"));
    assert!(g.has_edge("9", "10", None));
    assert!(g.has_edge("10", "9", None));

    let keys = g.edge_keys();
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0].v, "10");
    assert_eq!(keys[0].w, "9");
}

#[test]
fn predecessors_returns_node_predecessors() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_edge("a", "b");
    g.set_edge("b", "c");
    g.set_edge("a", "a");

    assert_eq!(sorted(g.predecessors("a")), vec!["a"]);
    assert_eq!(sorted(g.predecessors("b")), vec!["a"]);
    assert_eq!(sorted(g.predecessors("c")), vec!["b"]);
}

#[test]
fn successors_returns_node_successors() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_edge("a", "b");
    g.set_edge("b", "c");
    g.set_edge("a", "a");

    assert_eq!(sorted(g.successors("a")), vec!["a", "b"]);
    assert_eq!(sorted(g.successors("b")), vec!["c"]);
    assert!(g.successors("c").is_empty());
}

#[test]
fn neighbors_returns_unique_in_and_out_neighbors() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_edge("a", "b");
    g.set_edge("b", "c");
    g.set_edge("a", "a");

    assert_eq!(sorted(g.neighbors("a")), vec!["a", "b"]);
    assert_eq!(sorted(g.neighbors("b")), vec!["a", "c"]);
    assert_eq!(sorted(g.neighbors("c")), vec!["b"]);
}

#[test]
fn is_leaf_follows_graphlib_directed_and_undirected_rules() {
    let mut directed: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    directed.ensure_node("isolated");
    directed.set_edge("a", "b");

    assert!(directed.is_leaf("isolated"));
    assert!(!directed.is_leaf("a"));
    assert!(directed.is_leaf("b"));

    let mut undirected: Graph<(), (), ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    undirected.ensure_node("isolated");
    undirected.set_edge("a", "b");

    assert!(undirected.is_leaf("isolated"));
    assert!(!undirected.is_leaf("b"));
}

#[test]
fn in_edges_returns_edges_pointing_at_node() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_edge("a", "b");
    g.set_edge("b", "c");

    assert!(g.in_edges("a", None).is_empty());
    assert_eq!(
        sorted_edge_tuples(g.in_edges("b", None)),
        vec![("a".to_string(), "b".to_string(), None)]
    );
    assert_eq!(
        sorted_edge_tuples(g.in_edges("c", None)),
        vec![("b".to_string(), "c".to_string(), None)]
    );
}

#[test]
fn out_edges_returns_edges_pointing_from_node() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_edge("a", "b");
    g.set_edge("b", "c");

    assert_eq!(
        sorted_edge_tuples(g.out_edges("a", None)),
        vec![("a".to_string(), "b".to_string(), None)]
    );
    assert_eq!(
        sorted_edge_tuples(g.out_edges("b", None)),
        vec![("b".to_string(), "c".to_string(), None)]
    );
    assert!(g.out_edges("c", None).is_empty());
}

#[test]
fn edge_queries_work_for_multigraphs_and_endpoint_filters() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions {
        multigraph: true,
        ..Default::default()
    });
    g.set_edge("a", "b");
    g.set_edge_named("a", "b", Some("bar"), None);
    g.set_edge_named("a", "b", Some("foo"), None);
    g.set_edge("a", "c");
    g.set_edge("b", "c");
    g.set_edge("z", "a");
    g.set_edge("z", "b");

    let ab = vec![
        ("a".to_string(), "b".to_string(), None),
        ("a".to_string(), "b".to_string(), Some("bar".to_string())),
        ("a".to_string(), "b".to_string(), Some("foo".to_string())),
    ];
    assert_eq!(sorted_edge_tuples(g.out_edges("a", Some("b"))), ab);
    assert!(g.out_edges("b", Some("a")).is_empty());

    let ab = vec![
        ("a".to_string(), "b".to_string(), None),
        ("a".to_string(), "b".to_string(), Some("bar".to_string())),
        ("a".to_string(), "b".to_string(), Some("foo".to_string())),
    ];
    assert_eq!(sorted_edge_tuples(g.in_edges("b", Some("a"))), ab);
    assert!(g.in_edges("a", Some("b")).is_empty());
}

#[test]
fn node_edges_returns_all_incident_edges() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_edge("a", "b");
    g.set_edge("b", "c");

    assert_eq!(
        sorted_edge_tuples(g.node_edges("a")),
        vec![("a".to_string(), "b".to_string(), None)]
    );
    assert_eq!(
        sorted_edge_tuples(g.node_edges("b")),
        vec![
            ("a".to_string(), "b".to_string(), None),
            ("b".to_string(), "c".to_string(), None)
        ]
    );
    assert_eq!(
        sorted_edge_tuples(g.node_edges("c")),
        vec![("b".to_string(), "c".to_string(), None)]
    );
}

#[test]
fn node_edges_returns_parallel_multigraph_edges() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions {
        multigraph: true,
        ..Default::default()
    });
    g.set_edge("a", "b");
    g.set_edge_named("a", "b", Some("bar"), None);
    g.set_edge_named("a", "b", Some("foo"), None);

    let ab = vec![
        ("a".to_string(), "b".to_string(), None),
        ("a".to_string(), "b".to_string(), Some("bar".to_string())),
        ("a".to_string(), "b".to_string(), Some("foo".to_string())),
    ];
    assert_eq!(sorted_edge_tuples(g.node_edges("a")), ab);

    let ab = vec![
        ("a".to_string(), "b".to_string(), None),
        ("a".to_string(), "b".to_string(), Some("bar".to_string())),
        ("a".to_string(), "b".to_string(), Some("foo".to_string())),
    ];
    assert_eq!(sorted_edge_tuples(g.node_edges("b")), ab);
}

#[test]
fn node_edges_between_returns_edges_between_specific_nodes() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions {
        multigraph: true,
        ..Default::default()
    });
    g.set_edge("a", "b");
    g.set_edge_named("a", "b", Some("bar"), None);
    g.set_edge_named("a", "b", Some("foo"), None);
    g.set_edge("a", "c");
    g.set_edge("b", "c");
    g.set_edge("z", "a");
    g.set_edge("z", "b");

    let ab = vec![
        ("a".to_string(), "b".to_string(), None),
        ("a".to_string(), "b".to_string(), Some("bar".to_string())),
        ("a".to_string(), "b".to_string(), Some("foo".to_string())),
    ];
    assert_eq!(sorted_edge_tuples(g.node_edges_between("a", "b")), ab);

    let ab = vec![
        ("a".to_string(), "b".to_string(), None),
        ("a".to_string(), "b".to_string(), Some("bar".to_string())),
        ("a".to_string(), "b".to_string(), Some("foo".to_string())),
    ];
    assert_eq!(sorted_edge_tuples(g.node_edges_between("b", "a")), ab);
}

#[test]
fn remove_edge_missing_edge_is_noop() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());

    assert!(!g.remove_edge("a", "b", None));

    assert!(!g.has_edge("a", "b", None));
    assert_eq!(g.edge_count(), 0);
}

#[test]
fn remove_edge_key_removes_named_multigraph_edge() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions {
        multigraph: true,
        ..Default::default()
    });
    let key = EdgeKey::new("a", "b", Some("foo"));
    g.set_edge_key(key.clone(), ());

    assert!(g.remove_edge_key(&key));

    assert!(!g.has_edge("a", "b", Some("foo")));
    assert_eq!(g.edge_count(), 0);
}

#[test]
fn remove_edge_with_named_ids_removes_named_multigraph_edge() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions {
        multigraph: true,
        ..Default::default()
    });
    g.set_edge_named("a", "b", Some("foo"), None);

    assert!(g.remove_edge("a", "b", Some("foo")));

    assert!(!g.has_edge("a", "b", Some("foo")));
    assert_eq!(g.edge_count(), 0);
}

#[test]
fn remove_edge_accepts_reversed_endpoints_for_undirected_graphs() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    g.set_edge("h", "g");

    assert!(g.remove_edge("g", "h", None));

    assert!(g.neighbors("g").is_empty());
    assert!(g.neighbors("h").is_empty());
}

#[test]
fn remove_edge_updates_neighbor_queries() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_edge("a", "b");

    assert!(g.remove_edge("a", "b", None));

    assert!(g.successors("a").is_empty());
    assert!(g.neighbors("a").is_empty());
    assert!(g.predecessors("b").is_empty());
    assert!(g.neighbors("b").is_empty());
}

#[test]
fn remove_edge_keeps_named_parallel_edges() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions {
        multigraph: true,
        ..Default::default()
    });
    g.set_edge("a", "b");
    g.set_edge_named("a", "b", Some("foo"), None);

    assert!(g.remove_edge("a", "b", None));

    assert!(g.has_edge("a", "b", Some("foo")));
    assert_eq!(g.successors("a"), vec!["b"]);
    assert_eq!(g.neighbors("a"), vec!["b"]);
    assert_eq!(g.predecessors("b"), vec!["a"]);
    assert_eq!(g.neighbors("b"), vec!["a"]);
}

#[test]
fn set_parent_creates_parent_and_child_nodes() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions {
        compound: true,
        ..Default::default()
    });

    g.set_parent("a", "parent");

    assert!(g.has_node("a"));
    assert!(g.has_node("parent"));
    assert_eq!(g.parent("a"), Some("parent"));
    assert_eq!(g.children("parent"), vec!["a"]);
}

#[test]
fn children_opt_distinguishes_missing_nodes_from_empty_children() {
    let mut compound: Graph<(), (), ()> = Graph::new(GraphOptions {
        compound: true,
        ..Default::default()
    });

    assert_eq!(compound.children_opt("missing"), None);

    compound.ensure_node("a");
    assert_eq!(compound.children_opt("a"), Some(Vec::<&str>::new()));

    let mut simple: Graph<(), (), ()> = Graph::new(GraphOptions::default());

    assert_eq!(simple.children_opt("missing"), None);

    simple.ensure_node("a");
    assert_eq!(simple.children_opt("a"), Some(Vec::<&str>::new()));
}

#[test]
fn children_root_matches_graphlib_no_arg_children_semantics() {
    let mut simple: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    simple.ensure_node("a");
    simple.ensure_node("b");

    assert_eq!(sorted(simple.children_root()), vec!["a", "b"]);

    let mut compound: Graph<(), (), ()> = Graph::new(GraphOptions {
        compound: true,
        ..Default::default()
    });
    compound.ensure_node("b");
    compound.ensure_node("c");
    compound.set_parent("a", "parent");

    assert_eq!(sorted(compound.children_opt("parent").unwrap()), vec!["a"]);
    assert_eq!(sorted(compound.children_root()), vec!["b", "c", "parent"]);
}

#[test]
fn set_parent_moves_node_from_previous_parent() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions {
        compound: true,
        ..Default::default()
    });

    g.set_parent("a", "parent");
    g.set_parent("a", "parent2");

    assert_eq!(g.parent("a"), Some("parent2"));
    assert!(g.children("parent").is_empty());
    assert_eq!(g.children("parent2"), vec!["a"]);
}

#[test]
fn parent_matches_graphlib_optional_query_shape() {
    let mut simple: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    simple.ensure_node("a");
    assert_eq!(simple.parent("a"), None);
    assert_eq!(simple.parent("missing"), None);

    let mut compound: Graph<(), (), ()> = Graph::new(GraphOptions {
        compound: true,
        ..Default::default()
    });
    assert_eq!(compound.parent("missing"), None);

    compound.ensure_node("a");
    assert_eq!(compound.parent("a"), None);

    compound.set_parent("a", "parent");
    assert_eq!(compound.parent("a"), Some("parent"));
}

#[test]
fn clear_parent_returns_node_to_root_children() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions {
        compound: true,
        ..Default::default()
    });
    g.set_parent("a", "parent");

    g.clear_parent("a");
    g.clear_parent("a");

    assert_eq!(g.parent("a"), None);
    assert_eq!(sorted(g.children_root()), vec!["a", "parent"]);
}

#[test]
#[should_panic(expected = "set_parent would create a cycle")]
fn set_parent_preserves_tree_invariant() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions {
        compound: true,
        ..Default::default()
    });
    g.set_parent("c", "b");
    g.set_parent("b", "a");

    g.set_parent("a", "c");
}

#[test]
fn remove_node_clears_parent_child_relationships() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions {
        compound: true,
        ..Default::default()
    });
    g.set_parent("c", "b");
    g.set_parent("b", "a");

    assert!(g.remove_node("b"));

    assert_eq!(g.parent("b"), None);
    assert_eq!(g.children_opt("b"), None);
    assert!(g.children("b").is_empty());
    assert!(!g.children("a").contains(&"b"));
    assert_eq!(g.parent("c"), None);
}

#[test]
fn edge_key_lookup_uses_named_edges() {
    let mut g: Graph<(), i32, ()> = Graph::new(GraphOptions {
        multigraph: true,
        ..Default::default()
    });
    g.set_edge_named("a", "b", Some("name"), Some(5));
    let key = EdgeKey::new("a", "b", Some("name"));

    assert_eq!(g.edge_by_key(&key), Some(&5));
    assert!(g.remove_edge_key(&key));
    assert_eq!(g.edge_count(), 0);
}

#[test]
fn node_ids_and_edge_keys_keep_insertion_order_after_removal() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_edge("a", "b");
    g.set_edge("b", "c");
    g.remove_node("b");
    g.ensure_node("d");

    assert_eq!(sorted_owned(g.node_ids()), vec!["a", "c", "d"]);
    assert!(g.edge_keys().is_empty());
}
