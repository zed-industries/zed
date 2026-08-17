use dugong::graphlib::{Graph, GraphOptions};
use dugong::position::bk;
use dugong::util;
use dugong::{EdgeLabel, GraphLabel, LabelPos, NodeLabel};
use rustc_hash::FxHashMap as HashMap;

fn new_graph() -> Graph<NodeLabel, EdgeLabel, GraphLabel> {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions::default());
    g.set_graph(GraphLabel::default());
    g
}

fn set_node_rank_order(
    g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
    id: &str,
    rank: i32,
    order: usize,
) {
    g.set_node(
        id,
        NodeLabel {
            rank: Some(rank),
            order: Some(order),
            ..Default::default()
        },
    );
}

fn set_node_with(
    g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
    id: &str,
    rank: i32,
    order: usize,
    width: f64,
    dummy: Option<&str>,
    labelpos: Option<LabelPos>,
) {
    g.set_node(
        id,
        NodeLabel {
            rank: Some(rank),
            order: Some(order),
            width,
            dummy: dummy.map(|s| s.to_string()),
            labelpos,
            ..Default::default()
        },
    );
}

fn hm(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

fn map_from<K, V>(pairs: impl IntoIterator<Item = (K, V)>) -> HashMap<K, V>
where
    K: std::hash::Hash + Eq,
{
    pairs.into_iter().collect()
}

fn set_path(g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>, path: &[&str]) {
    for w in path.windows(2) {
        g.set_edge(w[0], w[1]);
    }
}

#[test]
fn find_type1_conflicts_does_not_mark_edges_that_have_no_conflict() {
    let mut g = new_graph();
    set_node_rank_order(&mut g, "a", 0, 0);
    set_node_rank_order(&mut g, "b", 0, 1);
    set_node_rank_order(&mut g, "c", 1, 0);
    set_node_rank_order(&mut g, "d", 1, 1);
    g.set_edge("a", "d");
    g.set_edge("b", "c");
    let layering = util::build_layer_matrix(&g);

    g.remove_edge("a", "d", None);
    g.remove_edge("b", "c", None);
    g.set_edge("a", "c");
    g.set_edge("b", "d");

    let conflicts = bk::find_type1_conflicts(&g, &layering);
    assert!(!bk::has_conflict(&conflicts, "a", "c"));
    assert!(!bk::has_conflict(&conflicts, "b", "d"));
}

#[test]
fn find_type1_conflicts_does_not_mark_type_0_conflicts_no_dummies() {
    let mut g = new_graph();
    set_node_rank_order(&mut g, "a", 0, 0);
    set_node_rank_order(&mut g, "b", 0, 1);
    set_node_rank_order(&mut g, "c", 1, 0);
    set_node_rank_order(&mut g, "d", 1, 1);
    g.set_edge("a", "d");
    g.set_edge("b", "c");
    let layering = util::build_layer_matrix(&g);

    let conflicts = bk::find_type1_conflicts(&g, &layering);
    assert!(!bk::has_conflict(&conflicts, "a", "d"));
    assert!(!bk::has_conflict(&conflicts, "b", "c"));
}

#[test]
fn find_type1_conflicts_does_not_mark_type_0_conflicts_a_is_dummy() {
    let mut g = new_graph();
    set_node_rank_order(&mut g, "a", 0, 0);
    set_node_rank_order(&mut g, "b", 0, 1);
    set_node_rank_order(&mut g, "c", 1, 0);
    set_node_rank_order(&mut g, "d", 1, 1);
    g.set_edge("a", "d");
    g.set_edge("b", "c");
    let layering = util::build_layer_matrix(&g);

    g.node_mut("a").unwrap().dummy = Some("true".to_string());
    let conflicts = bk::find_type1_conflicts(&g, &layering);
    assert!(!bk::has_conflict(&conflicts, "a", "d"));
    assert!(!bk::has_conflict(&conflicts, "b", "c"));
}

#[test]
fn find_type1_conflicts_does_not_mark_type_0_conflicts_b_is_dummy() {
    let mut g = new_graph();
    set_node_rank_order(&mut g, "a", 0, 0);
    set_node_rank_order(&mut g, "b", 0, 1);
    set_node_rank_order(&mut g, "c", 1, 0);
    set_node_rank_order(&mut g, "d", 1, 1);
    g.set_edge("a", "d");
    g.set_edge("b", "c");
    let layering = util::build_layer_matrix(&g);

    g.node_mut("b").unwrap().dummy = Some("true".to_string());
    let conflicts = bk::find_type1_conflicts(&g, &layering);
    assert!(!bk::has_conflict(&conflicts, "a", "d"));
    assert!(!bk::has_conflict(&conflicts, "b", "c"));
}

#[test]
fn find_type1_conflicts_does_not_mark_type_0_conflicts_c_is_dummy() {
    let mut g = new_graph();
    set_node_rank_order(&mut g, "a", 0, 0);
    set_node_rank_order(&mut g, "b", 0, 1);
    set_node_rank_order(&mut g, "c", 1, 0);
    set_node_rank_order(&mut g, "d", 1, 1);
    g.set_edge("a", "d");
    g.set_edge("b", "c");
    let layering = util::build_layer_matrix(&g);

    g.node_mut("c").unwrap().dummy = Some("true".to_string());
    let conflicts = bk::find_type1_conflicts(&g, &layering);
    assert!(!bk::has_conflict(&conflicts, "a", "d"));
    assert!(!bk::has_conflict(&conflicts, "b", "c"));
}

#[test]
fn find_type1_conflicts_does_not_mark_type_0_conflicts_d_is_dummy() {
    let mut g = new_graph();
    set_node_rank_order(&mut g, "a", 0, 0);
    set_node_rank_order(&mut g, "b", 0, 1);
    set_node_rank_order(&mut g, "c", 1, 0);
    set_node_rank_order(&mut g, "d", 1, 1);
    g.set_edge("a", "d");
    g.set_edge("b", "c");
    let layering = util::build_layer_matrix(&g);

    g.node_mut("d").unwrap().dummy = Some("true".to_string());
    let conflicts = bk::find_type1_conflicts(&g, &layering);
    assert!(!bk::has_conflict(&conflicts, "a", "d"));
    assert!(!bk::has_conflict(&conflicts, "b", "c"));
}

#[test]
fn find_type1_conflicts_does_mark_type_1_conflicts_a_is_non_dummy() {
    let mut g = new_graph();
    set_node_rank_order(&mut g, "a", 0, 0);
    set_node_rank_order(&mut g, "b", 0, 1);
    set_node_rank_order(&mut g, "c", 1, 0);
    set_node_rank_order(&mut g, "d", 1, 1);
    g.set_edge("a", "d");
    g.set_edge("b", "c");
    let layering = util::build_layer_matrix(&g);

    for w in ["b", "c", "d"] {
        g.node_mut(w).unwrap().dummy = Some("true".to_string());
    }
    let conflicts = bk::find_type1_conflicts(&g, &layering);
    assert!(bk::has_conflict(&conflicts, "a", "d"));
    assert!(!bk::has_conflict(&conflicts, "b", "c"));
}

#[test]
fn find_type1_conflicts_does_mark_type_1_conflicts_b_is_non_dummy() {
    let mut g = new_graph();
    set_node_rank_order(&mut g, "a", 0, 0);
    set_node_rank_order(&mut g, "b", 0, 1);
    set_node_rank_order(&mut g, "c", 1, 0);
    set_node_rank_order(&mut g, "d", 1, 1);
    g.set_edge("a", "d");
    g.set_edge("b", "c");
    let layering = util::build_layer_matrix(&g);

    for w in ["a", "c", "d"] {
        g.node_mut(w).unwrap().dummy = Some("true".to_string());
    }
    let conflicts = bk::find_type1_conflicts(&g, &layering);
    assert!(!bk::has_conflict(&conflicts, "a", "d"));
    assert!(bk::has_conflict(&conflicts, "b", "c"));
}

#[test]
fn find_type1_conflicts_does_mark_type_1_conflicts_c_is_non_dummy() {
    let mut g = new_graph();
    set_node_rank_order(&mut g, "a", 0, 0);
    set_node_rank_order(&mut g, "b", 0, 1);
    set_node_rank_order(&mut g, "c", 1, 0);
    set_node_rank_order(&mut g, "d", 1, 1);
    g.set_edge("a", "d");
    g.set_edge("b", "c");
    let layering = util::build_layer_matrix(&g);

    for w in ["a", "b", "d"] {
        g.node_mut(w).unwrap().dummy = Some("true".to_string());
    }
    let conflicts = bk::find_type1_conflicts(&g, &layering);
    assert!(!bk::has_conflict(&conflicts, "a", "d"));
    assert!(bk::has_conflict(&conflicts, "b", "c"));
}

#[test]
fn find_type1_conflicts_does_mark_type_1_conflicts_d_is_non_dummy() {
    let mut g = new_graph();
    set_node_rank_order(&mut g, "a", 0, 0);
    set_node_rank_order(&mut g, "b", 0, 1);
    set_node_rank_order(&mut g, "c", 1, 0);
    set_node_rank_order(&mut g, "d", 1, 1);
    g.set_edge("a", "d");
    g.set_edge("b", "c");
    let layering = util::build_layer_matrix(&g);

    for w in ["a", "b", "c"] {
        g.node_mut(w).unwrap().dummy = Some("true".to_string());
    }
    let conflicts = bk::find_type1_conflicts(&g, &layering);
    assert!(bk::has_conflict(&conflicts, "a", "d"));
    assert!(!bk::has_conflict(&conflicts, "b", "c"));
}

#[test]
fn find_type1_conflicts_does_not_mark_type_2_conflicts_all_dummies() {
    let mut g = new_graph();
    set_node_rank_order(&mut g, "a", 0, 0);
    set_node_rank_order(&mut g, "b", 0, 1);
    set_node_rank_order(&mut g, "c", 1, 0);
    set_node_rank_order(&mut g, "d", 1, 1);
    g.set_edge("a", "d");
    g.set_edge("b", "c");
    let layering = util::build_layer_matrix(&g);

    for v in ["a", "b", "c", "d"] {
        g.node_mut(v).unwrap().dummy = Some("true".to_string());
    }
    let conflicts = bk::find_type1_conflicts(&g, &layering);
    assert!(!bk::has_conflict(&conflicts, "a", "d"));
    assert!(!bk::has_conflict(&conflicts, "b", "c"));
}

#[test]
fn find_type2_conflicts_marks_type_2_conflicts_favoring_border_segments_1() {
    let mut g = new_graph();
    set_node_rank_order(&mut g, "a", 0, 0);
    set_node_rank_order(&mut g, "b", 0, 1);
    set_node_rank_order(&mut g, "c", 1, 0);
    set_node_rank_order(&mut g, "d", 1, 1);
    g.set_edge("a", "d");
    g.set_edge("b", "c");
    let layering = util::build_layer_matrix(&g);

    for v in ["a", "d"] {
        g.node_mut(v).unwrap().dummy = Some("true".to_string());
    }
    for v in ["b", "c"] {
        g.node_mut(v).unwrap().dummy = Some("border".to_string());
    }

    let conflicts = bk::find_type2_conflicts(&g, &layering);
    assert!(bk::has_conflict(&conflicts, "a", "d"));
    assert!(!bk::has_conflict(&conflicts, "b", "c"));
}

#[test]
fn find_type2_conflicts_marks_type_2_conflicts_favoring_border_segments_2() {
    let mut g = new_graph();
    set_node_rank_order(&mut g, "a", 0, 0);
    set_node_rank_order(&mut g, "b", 0, 1);
    set_node_rank_order(&mut g, "c", 1, 0);
    set_node_rank_order(&mut g, "d", 1, 1);
    g.set_edge("a", "d");
    g.set_edge("b", "c");
    let layering = util::build_layer_matrix(&g);

    for v in ["b", "c"] {
        g.node_mut(v).unwrap().dummy = Some("true".to_string());
    }
    for v in ["a", "d"] {
        g.node_mut(v).unwrap().dummy = Some("border".to_string());
    }

    let conflicts = bk::find_type2_conflicts(&g, &layering);
    assert!(!bk::has_conflict(&conflicts, "a", "d"));
    assert!(bk::has_conflict(&conflicts, "b", "c"));
}

#[test]
fn has_conflict_can_test_for_a_type_1_conflict_regardless_of_edge_orientation() {
    let mut conflicts: bk::Conflicts = Default::default();
    bk::add_conflict(&mut conflicts, "b", "a");
    assert!(bk::has_conflict(&conflicts, "a", "b"));
    assert!(bk::has_conflict(&conflicts, "b", "a"));
}

#[test]
fn has_conflict_works_for_multiple_conflicts_with_the_same_node() {
    let mut conflicts: bk::Conflicts = Default::default();
    bk::add_conflict(&mut conflicts, "a", "b");
    bk::add_conflict(&mut conflicts, "a", "c");
    assert!(bk::has_conflict(&conflicts, "a", "b"));
    assert!(bk::has_conflict(&conflicts, "a", "c"));
}

#[test]
fn vertical_alignment_aligns_with_itself_if_the_node_has_no_adjacencies() {
    let mut g = new_graph();
    set_node_rank_order(&mut g, "a", 0, 0);
    set_node_rank_order(&mut g, "b", 1, 0);
    let layering = util::build_layer_matrix(&g);
    let conflicts: bk::Conflicts = Default::default();

    let result = bk::vertical_alignment(&g, &layering, &conflicts, |v| {
        g.predecessors(v)
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    });
    assert_eq!(
        result,
        bk::Alignment {
            root: hm(&[("a", "a"), ("b", "b")]),
            align: hm(&[("a", "a"), ("b", "b")])
        }
    );
}

#[test]
fn vertical_alignment_aligns_with_its_sole_adjacency() {
    let mut g = new_graph();
    set_node_rank_order(&mut g, "a", 0, 0);
    set_node_rank_order(&mut g, "b", 1, 0);
    g.set_edge("a", "b");
    let layering = util::build_layer_matrix(&g);
    let conflicts: bk::Conflicts = Default::default();

    let result = bk::vertical_alignment(&g, &layering, &conflicts, |v| {
        g.predecessors(v)
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    });
    assert_eq!(
        result,
        bk::Alignment {
            root: hm(&[("a", "a"), ("b", "a")]),
            align: hm(&[("a", "b"), ("b", "a")])
        }
    );
}

#[test]
fn vertical_alignment_aligns_with_its_left_median_when_possible() {
    let mut g = new_graph();
    set_node_rank_order(&mut g, "a", 0, 0);
    set_node_rank_order(&mut g, "b", 0, 1);
    set_node_rank_order(&mut g, "c", 1, 0);
    g.set_edge("a", "c");
    g.set_edge("b", "c");
    let layering = util::build_layer_matrix(&g);
    let conflicts: bk::Conflicts = Default::default();

    let result = bk::vertical_alignment(&g, &layering, &conflicts, |v| {
        g.predecessors(v)
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    });
    assert_eq!(
        result,
        bk::Alignment {
            root: hm(&[("a", "a"), ("b", "b"), ("c", "a")]),
            align: hm(&[("a", "c"), ("b", "b"), ("c", "a")])
        }
    );
}

#[test]
fn vertical_alignment_aligns_correctly_regardless_of_node_name_or_insertion_order() {
    let mut g = new_graph();
    set_node_rank_order(&mut g, "b", 0, 1);
    set_node_rank_order(&mut g, "c", 1, 0);
    set_node_rank_order(&mut g, "z", 0, 0);
    g.set_edge("z", "c");
    g.set_edge("b", "c");
    let layering = util::build_layer_matrix(&g);
    let conflicts: bk::Conflicts = Default::default();

    let result = bk::vertical_alignment(&g, &layering, &conflicts, |v| {
        g.predecessors(v)
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    });
    assert_eq!(
        result,
        bk::Alignment {
            root: hm(&[("z", "z"), ("b", "b"), ("c", "z")]),
            align: hm(&[("z", "c"), ("b", "b"), ("c", "z")])
        }
    );
}

#[test]
fn vertical_alignment_aligns_with_its_right_median_when_left_is_unavailable() {
    let mut g = new_graph();
    set_node_rank_order(&mut g, "a", 0, 0);
    set_node_rank_order(&mut g, "b", 0, 1);
    set_node_rank_order(&mut g, "c", 1, 0);
    g.set_edge("a", "c");
    g.set_edge("b", "c");
    let layering = util::build_layer_matrix(&g);
    let mut conflicts: bk::Conflicts = Default::default();
    bk::add_conflict(&mut conflicts, "a", "c");

    let result = bk::vertical_alignment(&g, &layering, &conflicts, |v| {
        g.predecessors(v)
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    });
    assert_eq!(
        result,
        bk::Alignment {
            root: hm(&[("a", "a"), ("b", "b"), ("c", "b")]),
            align: hm(&[("a", "a"), ("b", "c"), ("c", "b")])
        }
    );
}

#[test]
fn vertical_alignment_aligns_with_neither_median_if_both_are_unavailable() {
    let mut g = new_graph();
    set_node_rank_order(&mut g, "a", 0, 0);
    set_node_rank_order(&mut g, "b", 0, 1);
    set_node_rank_order(&mut g, "c", 1, 0);
    set_node_rank_order(&mut g, "d", 1, 1);
    g.set_edge("a", "d");
    g.set_edge("b", "c");
    g.set_edge("b", "d");
    let layering = util::build_layer_matrix(&g);
    let conflicts: bk::Conflicts = Default::default();

    let result = bk::vertical_alignment(&g, &layering, &conflicts, |v| {
        g.predecessors(v)
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    });
    assert_eq!(
        result,
        bk::Alignment {
            root: hm(&[("a", "a"), ("b", "b"), ("c", "b"), ("d", "d")]),
            align: hm(&[("a", "a"), ("b", "c"), ("c", "b"), ("d", "d")])
        }
    );
}

#[test]
fn vertical_alignment_aligns_with_the_single_median_for_an_odd_number_of_adjacencies() {
    let mut g = new_graph();
    set_node_rank_order(&mut g, "a", 0, 0);
    set_node_rank_order(&mut g, "b", 0, 1);
    set_node_rank_order(&mut g, "c", 0, 2);
    set_node_rank_order(&mut g, "d", 1, 0);
    g.set_edge("a", "d");
    g.set_edge("b", "d");
    g.set_edge("c", "d");
    let layering = util::build_layer_matrix(&g);
    let conflicts: bk::Conflicts = Default::default();

    let result = bk::vertical_alignment(&g, &layering, &conflicts, |v| {
        g.predecessors(v)
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    });
    assert_eq!(
        result,
        bk::Alignment {
            root: hm(&[("a", "a"), ("b", "b"), ("c", "c"), ("d", "b")]),
            align: hm(&[("a", "a"), ("b", "d"), ("c", "c"), ("d", "b")])
        }
    );
}

#[test]
fn vertical_alignment_aligns_blocks_across_multiple_layers() {
    let mut g = new_graph();
    set_node_rank_order(&mut g, "a", 0, 0);
    set_node_rank_order(&mut g, "b", 1, 0);
    set_node_rank_order(&mut g, "c", 1, 1);
    set_node_rank_order(&mut g, "d", 2, 0);
    set_path(&mut g, &["a", "b", "d"]);
    set_path(&mut g, &["a", "c", "d"]);
    let layering = util::build_layer_matrix(&g);
    let conflicts: bk::Conflicts = Default::default();

    let result = bk::vertical_alignment(&g, &layering, &conflicts, |v| {
        g.predecessors(v)
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    });
    assert_eq!(
        result,
        bk::Alignment {
            root: hm(&[("a", "a"), ("b", "a"), ("c", "c"), ("d", "a")]),
            align: hm(&[("a", "b"), ("b", "d"), ("c", "c"), ("d", "a")])
        }
    );
}

#[test]
fn horizontal_compaction_places_the_center_of_a_single_node_graph_at_origin() {
    let mut g = new_graph();
    set_node_rank_order(&mut g, "a", 0, 0);
    let root = hm(&[("a", "a")]);
    let align = hm(&[("a", "a")]);
    let layering = util::build_layer_matrix(&g);

    let xs = bk::horizontal_compaction(&g, &layering, &root, &align, false);
    assert_eq!(xs["a"], 0.0);
}

#[test]
fn horizontal_compaction_separates_adjacent_nodes_by_specified_node_separation() {
    let mut g = new_graph();
    g.graph_mut().nodesep = 100.0;
    set_node_with(&mut g, "a", 0, 0, 100.0, None, None);
    set_node_with(&mut g, "b", 0, 1, 200.0, None, None);
    let root = hm(&[("a", "a"), ("b", "b")]);
    let align = root.clone();
    let layering = util::build_layer_matrix(&g);

    let xs = bk::horizontal_compaction(&g, &layering, &root, &align, false);
    assert_eq!(xs["a"], 0.0);
    assert_eq!(xs["b"], 100.0 / 2.0 + 100.0 + 200.0 / 2.0);
}

#[test]
fn horizontal_compaction_separates_adjacent_edges_by_specified_edge_separation() {
    let mut g = new_graph();
    g.graph_mut().edgesep = 20.0;
    set_node_with(&mut g, "a", 0, 0, 100.0, Some("edge"), None);
    set_node_with(&mut g, "b", 0, 1, 200.0, Some("edge"), None);
    let root = hm(&[("a", "a"), ("b", "b")]);
    let align = root.clone();
    let layering = util::build_layer_matrix(&g);

    let xs = bk::horizontal_compaction(&g, &layering, &root, &align, false);
    assert_eq!(xs["a"], 0.0);
    assert_eq!(xs["b"], 100.0 / 2.0 + 20.0 + 200.0 / 2.0);
}

#[test]
fn horizontal_compaction_aligns_the_centers_of_nodes_in_the_same_block() {
    let mut g = new_graph();
    set_node_with(&mut g, "a", 0, 0, 100.0, None, None);
    set_node_with(&mut g, "b", 1, 0, 200.0, None, None);
    let root = hm(&[("a", "a"), ("b", "a")]);
    let align = hm(&[("a", "b"), ("b", "a")]);
    let layering = util::build_layer_matrix(&g);

    let xs = bk::horizontal_compaction(&g, &layering, &root, &align, false);
    assert_eq!(xs["a"], 0.0);
    assert_eq!(xs["b"], 0.0);
}

#[test]
fn horizontal_compaction_separates_blocks_with_the_appropriate_separation() {
    let mut g = new_graph();
    g.graph_mut().nodesep = 75.0;
    set_node_with(&mut g, "a", 0, 0, 100.0, None, None);
    set_node_with(&mut g, "b", 1, 1, 200.0, None, None);
    set_node_with(&mut g, "c", 1, 0, 50.0, None, None);
    let root = hm(&[("a", "a"), ("b", "a"), ("c", "c")]);
    let align = hm(&[("a", "b"), ("b", "a"), ("c", "c")]);
    let layering = util::build_layer_matrix(&g);

    let xs = bk::horizontal_compaction(&g, &layering, &root, &align, false);
    assert_eq!(xs["a"], 50.0 / 2.0 + 75.0 + 200.0 / 2.0);
    assert_eq!(xs["b"], 50.0 / 2.0 + 75.0 + 200.0 / 2.0);
    assert_eq!(xs["c"], 0.0);
}

#[test]
fn horizontal_compaction_separates_classes_with_the_appropriate_separation() {
    let mut g = new_graph();
    g.graph_mut().nodesep = 75.0;
    set_node_with(&mut g, "a", 0, 0, 100.0, None, None);
    set_node_with(&mut g, "b", 0, 1, 200.0, None, None);
    set_node_with(&mut g, "c", 1, 0, 50.0, None, None);
    set_node_with(&mut g, "d", 1, 1, 80.0, None, None);
    let root = hm(&[("a", "a"), ("b", "b"), ("c", "c"), ("d", "b")]);
    let align = hm(&[("a", "a"), ("b", "d"), ("c", "c"), ("d", "b")]);
    let layering = util::build_layer_matrix(&g);

    let xs = bk::horizontal_compaction(&g, &layering, &root, &align, false);
    assert_eq!(xs["a"], 0.0);
    assert_eq!(xs["b"], 100.0 / 2.0 + 75.0 + 200.0 / 2.0);
    assert_eq!(
        xs["c"],
        100.0 / 2.0 + 75.0 + 200.0 / 2.0 - 80.0 / 2.0 - 75.0 - 50.0 / 2.0
    );
    assert_eq!(xs["d"], 100.0 / 2.0 + 75.0 + 200.0 / 2.0);
}

#[test]
fn horizontal_compaction_shifts_classes_by_max_sep_from_the_adjacent_block_1() {
    let mut g = new_graph();
    g.graph_mut().nodesep = 75.0;
    set_node_with(&mut g, "a", 0, 0, 50.0, None, None);
    set_node_with(&mut g, "b", 0, 1, 150.0, None, None);
    set_node_with(&mut g, "c", 1, 0, 60.0, None, None);
    set_node_with(&mut g, "d", 1, 1, 70.0, None, None);
    let root = hm(&[("a", "a"), ("b", "b"), ("c", "a"), ("d", "b")]);
    let align = hm(&[("a", "c"), ("b", "d"), ("c", "a"), ("d", "b")]);
    let layering = util::build_layer_matrix(&g);

    let xs = bk::horizontal_compaction(&g, &layering, &root, &align, false);
    assert_eq!(xs["a"], 0.0);
    assert_eq!(xs["b"], 50.0 / 2.0 + 75.0 + 150.0 / 2.0);
    assert_eq!(xs["c"], 0.0);
    assert_eq!(xs["d"], 50.0 / 2.0 + 75.0 + 150.0 / 2.0);
}

#[test]
fn horizontal_compaction_shifts_classes_by_max_sep_from_the_adjacent_block_2() {
    let mut g = new_graph();
    g.graph_mut().nodesep = 75.0;
    set_node_with(&mut g, "a", 0, 0, 50.0, None, None);
    set_node_with(&mut g, "b", 0, 1, 70.0, None, None);
    set_node_with(&mut g, "c", 1, 0, 60.0, None, None);
    set_node_with(&mut g, "d", 1, 1, 150.0, None, None);
    let root = hm(&[("a", "a"), ("b", "b"), ("c", "a"), ("d", "b")]);
    let align = hm(&[("a", "c"), ("b", "d"), ("c", "a"), ("d", "b")]);
    let layering = util::build_layer_matrix(&g);

    let xs = bk::horizontal_compaction(&g, &layering, &root, &align, false);
    assert_eq!(xs["a"], 0.0);
    assert_eq!(xs["b"], 60.0 / 2.0 + 75.0 + 150.0 / 2.0);
    assert_eq!(xs["c"], 0.0);
    assert_eq!(xs["d"], 60.0 / 2.0 + 75.0 + 150.0 / 2.0);
}

#[test]
fn horizontal_compaction_cascades_class_shift() {
    let mut g = new_graph();
    g.graph_mut().nodesep = 75.0;
    for (id, rank, order) in [
        ("a", 0, 0),
        ("b", 0, 1),
        ("c", 1, 0),
        ("d", 1, 1),
        ("e", 1, 2),
        ("f", 2, 0),
        ("g", 2, 1),
    ] {
        set_node_with(&mut g, id, rank, order, 50.0, None, None);
    }
    let root = hm(&[
        ("a", "a"),
        ("b", "b"),
        ("c", "c"),
        ("d", "d"),
        ("e", "b"),
        ("f", "f"),
        ("g", "d"),
    ]);
    let align = hm(&[
        ("a", "a"),
        ("b", "e"),
        ("c", "c"),
        ("d", "g"),
        ("e", "b"),
        ("f", "f"),
        ("g", "d"),
    ]);
    let layering = util::build_layer_matrix(&g);

    let xs = bk::horizontal_compaction(&g, &layering, &root, &align, false);
    assert_eq!(xs["a"], xs["b"] - 50.0 / 2.0 - 75.0 - 50.0 / 2.0);
    assert_eq!(xs["b"], xs["e"]);
    assert_eq!(xs["c"], xs["f"]);
    assert_eq!(xs["d"], xs["c"] + 50.0 / 2.0 + 75.0 + 50.0 / 2.0);
    assert_eq!(xs["e"], xs["d"] + 50.0 / 2.0 + 75.0 + 50.0 / 2.0);
    assert_eq!(xs["g"], xs["f"] + 50.0 / 2.0 + 75.0 + 50.0 / 2.0);
}

#[test]
fn horizontal_compaction_handles_labelpos_l() {
    let mut g = new_graph();
    g.graph_mut().edgesep = 50.0;
    set_node_with(&mut g, "a", 0, 0, 100.0, Some("edge"), None);
    set_node_with(
        &mut g,
        "b",
        0,
        1,
        200.0,
        Some("edge-label"),
        Some(LabelPos::L),
    );
    set_node_with(&mut g, "c", 0, 2, 300.0, Some("edge"), None);
    let root = hm(&[("a", "a"), ("b", "b"), ("c", "c")]);
    let align = root.clone();
    let layering = util::build_layer_matrix(&g);

    let xs = bk::horizontal_compaction(&g, &layering, &root, &align, false);
    assert_eq!(xs["a"], 0.0);
    assert_eq!(xs["b"], xs["a"] + 100.0 / 2.0 + 50.0 + 200.0);
    assert_eq!(xs["c"], xs["b"] + 0.0 + 50.0 + 300.0 / 2.0);
}

#[test]
fn horizontal_compaction_handles_labelpos_c() {
    let mut g = new_graph();
    g.graph_mut().edgesep = 50.0;
    set_node_with(&mut g, "a", 0, 0, 100.0, Some("edge"), None);
    set_node_with(
        &mut g,
        "b",
        0,
        1,
        200.0,
        Some("edge-label"),
        Some(LabelPos::C),
    );
    set_node_with(&mut g, "c", 0, 2, 300.0, Some("edge"), None);
    let root = hm(&[("a", "a"), ("b", "b"), ("c", "c")]);
    let align = root.clone();
    let layering = util::build_layer_matrix(&g);

    let xs = bk::horizontal_compaction(&g, &layering, &root, &align, false);
    assert_eq!(xs["a"], 0.0);
    assert_eq!(xs["b"], xs["a"] + 100.0 / 2.0 + 50.0 + 200.0 / 2.0);
    assert_eq!(xs["c"], xs["b"] + 200.0 / 2.0 + 50.0 + 300.0 / 2.0);
}

#[test]
fn horizontal_compaction_handles_labelpos_r() {
    let mut g = new_graph();
    g.graph_mut().edgesep = 50.0;
    set_node_with(&mut g, "a", 0, 0, 100.0, Some("edge"), None);
    set_node_with(
        &mut g,
        "b",
        0,
        1,
        200.0,
        Some("edge-label"),
        Some(LabelPos::R),
    );
    set_node_with(&mut g, "c", 0, 2, 300.0, Some("edge"), None);
    let root = hm(&[("a", "a"), ("b", "b"), ("c", "c")]);
    let align = root.clone();
    let layering = util::build_layer_matrix(&g);

    let xs = bk::horizontal_compaction(&g, &layering, &root, &align, false);
    assert_eq!(xs["a"], 0.0);
    assert_eq!(xs["b"], xs["a"] + 100.0 / 2.0 + 50.0 + 0.0);
    assert_eq!(xs["c"], xs["b"] + 200.0 + 50.0 + 300.0 / 2.0);
}

#[test]
fn align_coordinates_aligns_a_single_node() {
    let mut xss: HashMap<String, HashMap<String, f64>> = HashMap::default();
    xss.insert("ul".to_string(), map_from([("a".to_string(), 50.0)]));
    xss.insert("ur".to_string(), map_from([("a".to_string(), 100.0)]));
    xss.insert("dl".to_string(), map_from([("a".to_string(), 50.0)]));
    xss.insert("dr".to_string(), map_from([("a".to_string(), 200.0)]));

    let align_to = xss.get("ul").unwrap().clone();
    bk::align_coordinates(&mut xss, &align_to);

    assert_eq!(xss["ul"]["a"], 50.0);
    assert_eq!(xss["ur"]["a"], 50.0);
    assert_eq!(xss["dl"]["a"], 50.0);
    assert_eq!(xss["dr"]["a"], 50.0);
}

#[test]
fn align_coordinates_aligns_multiple_nodes() {
    let mut xss: HashMap<String, HashMap<String, f64>> = HashMap::default();
    xss.insert(
        "ul".to_string(),
        map_from([("a".to_string(), 50.0), ("b".to_string(), 1000.0)]),
    );
    xss.insert(
        "ur".to_string(),
        map_from([("a".to_string(), 100.0), ("b".to_string(), 900.0)]),
    );
    xss.insert(
        "dl".to_string(),
        map_from([("a".to_string(), 150.0), ("b".to_string(), 800.0)]),
    );
    xss.insert(
        "dr".to_string(),
        map_from([("a".to_string(), 200.0), ("b".to_string(), 700.0)]),
    );

    let align_to = xss.get("ul").unwrap().clone();
    bk::align_coordinates(&mut xss, &align_to);

    assert_eq!(xss["ul"]["a"], 50.0);
    assert_eq!(xss["ul"]["b"], 1000.0);
    assert_eq!(xss["ur"]["a"], 200.0);
    assert_eq!(xss["ur"]["b"], 1000.0);
    assert_eq!(xss["dl"]["a"], 50.0);
    assert_eq!(xss["dl"]["b"], 700.0);
    assert_eq!(xss["dr"]["a"], 500.0);
    assert_eq!(xss["dr"]["b"], 1000.0);
}

#[test]
fn find_smallest_width_alignment_finds_the_alignment_with_the_smallest_width() {
    let mut g = new_graph();
    g.set_node(
        "a",
        NodeLabel {
            width: 50.0,
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            width: 50.0,
            ..Default::default()
        },
    );

    let xss: HashMap<String, HashMap<String, f64>> = map_from([
        (
            "ul".to_string(),
            map_from([("a".to_string(), 0.0), ("b".to_string(), 1000.0)]),
        ),
        (
            "ur".to_string(),
            map_from([("a".to_string(), -5.0), ("b".to_string(), 1000.0)]),
        ),
        (
            "dl".to_string(),
            map_from([("a".to_string(), 5.0), ("b".to_string(), 2000.0)]),
        ),
        (
            "dr".to_string(),
            map_from([("a".to_string(), 0.0), ("b".to_string(), 200.0)]),
        ),
    ]);

    assert_eq!(bk::find_smallest_width_alignment(&g, &xss), xss["dr"]);
}

#[test]
fn find_smallest_width_alignment_takes_node_width_into_account() {
    let mut g = new_graph();
    g.set_node(
        "a",
        NodeLabel {
            width: 50.0,
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            width: 50.0,
            ..Default::default()
        },
    );
    g.set_node(
        "c",
        NodeLabel {
            width: 200.0,
            ..Default::default()
        },
    );

    let xss: HashMap<String, HashMap<String, f64>> = map_from([
        (
            "ul".to_string(),
            map_from([
                ("a".to_string(), 0.0),
                ("b".to_string(), 100.0),
                ("c".to_string(), 75.0),
            ]),
        ),
        (
            "ur".to_string(),
            map_from([
                ("a".to_string(), 0.0),
                ("b".to_string(), 100.0),
                ("c".to_string(), 80.0),
            ]),
        ),
        (
            "dl".to_string(),
            map_from([
                ("a".to_string(), 0.0),
                ("b".to_string(), 100.0),
                ("c".to_string(), 85.0),
            ]),
        ),
        (
            "dr".to_string(),
            map_from([
                ("a".to_string(), 0.0),
                ("b".to_string(), 100.0),
                ("c".to_string(), 90.0),
            ]),
        ),
    ]);

    assert_eq!(bk::find_smallest_width_alignment(&g, &xss), xss["ul"]);
}

#[test]
fn balance_aligns_a_single_node_to_the_shared_median_value() {
    let xss: HashMap<String, HashMap<String, f64>> = map_from([
        ("ul".to_string(), map_from([("a".to_string(), 0.0)])),
        ("ur".to_string(), map_from([("a".to_string(), 100.0)])),
        ("dl".to_string(), map_from([("a".to_string(), 100.0)])),
        ("dr".to_string(), map_from([("a".to_string(), 200.0)])),
    ]);
    assert_eq!(
        bk::balance(&xss, None),
        map_from([("a".to_string(), 100.0)])
    );
}

#[test]
fn balance_aligns_a_single_node_to_the_average_of_different_median_values() {
    let xss: HashMap<String, HashMap<String, f64>> = map_from([
        ("ul".to_string(), map_from([("a".to_string(), 0.0)])),
        ("ur".to_string(), map_from([("a".to_string(), 75.0)])),
        ("dl".to_string(), map_from([("a".to_string(), 125.0)])),
        ("dr".to_string(), map_from([("a".to_string(), 200.0)])),
    ]);
    assert_eq!(
        bk::balance(&xss, None),
        map_from([("a".to_string(), 100.0)])
    );
}

#[test]
fn balance_balances_multiple_nodes() {
    let xss: HashMap<String, HashMap<String, f64>> = map_from([
        (
            "ul".to_string(),
            map_from([("a".to_string(), 0.0), ("b".to_string(), 50.0)]),
        ),
        (
            "ur".to_string(),
            map_from([("a".to_string(), 75.0), ("b".to_string(), 0.0)]),
        ),
        (
            "dl".to_string(),
            map_from([("a".to_string(), 125.0), ("b".to_string(), 60.0)]),
        ),
        (
            "dr".to_string(),
            map_from([("a".to_string(), 200.0), ("b".to_string(), 75.0)]),
        ),
    ]);
    assert_eq!(
        bk::balance(&xss, None),
        map_from([("a".to_string(), 100.0), ("b".to_string(), 55.0)])
    );
}

#[test]
fn position_x_positions_a_single_node_at_origin() {
    let mut g = new_graph();
    set_node_with(&mut g, "a", 0, 0, 100.0, None, None);
    assert_eq!(bk::position_x(&g), map_from([("a".to_string(), 0.0)]));
}

#[test]
fn position_x_positions_a_single_node_block_at_origin() {
    let mut g = new_graph();
    set_node_with(&mut g, "a", 0, 0, 100.0, None, None);
    set_node_with(&mut g, "b", 1, 0, 100.0, None, None);
    g.set_edge("a", "b");
    assert_eq!(
        bk::position_x(&g),
        map_from([("a".to_string(), 0.0), ("b".to_string(), 0.0)])
    );
}

#[test]
fn position_x_positions_a_single_node_block_at_origin_even_when_their_sizes_differ() {
    let mut g = new_graph();
    set_node_with(&mut g, "a", 0, 0, 40.0, None, None);
    set_node_with(&mut g, "b", 1, 0, 500.0, None, None);
    set_node_with(&mut g, "c", 2, 0, 20.0, None, None);
    set_path(&mut g, &["a", "b", "c"]);
    assert_eq!(
        bk::position_x(&g),
        map_from([
            ("a".to_string(), 0.0),
            ("b".to_string(), 0.0),
            ("c".to_string(), 0.0)
        ])
    );
}

#[test]
fn position_x_centers_a_node_if_it_is_a_predecessor_of_two_same_sized_nodes() {
    let mut g = new_graph();
    g.graph_mut().nodesep = 10.0;
    set_node_with(&mut g, "a", 0, 0, 20.0, None, None);
    set_node_with(&mut g, "b", 1, 0, 50.0, None, None);
    set_node_with(&mut g, "c", 1, 1, 50.0, None, None);
    g.set_edge("a", "b");
    g.set_edge("a", "c");

    let pos = bk::position_x(&g);
    let a = pos["a"];
    assert_eq!(pos["b"], a - (25.0 + 5.0));
    assert_eq!(pos["c"], a + (25.0 + 5.0));
}

#[test]
fn position_x_shifts_blocks_on_both_sides_of_aligned_block() {
    let mut g = new_graph();
    g.graph_mut().nodesep = 10.0;
    set_node_with(&mut g, "a", 0, 0, 50.0, None, None);
    set_node_with(&mut g, "b", 0, 1, 60.0, None, None);
    set_node_with(&mut g, "c", 1, 0, 70.0, None, None);
    set_node_with(&mut g, "d", 1, 1, 80.0, None, None);
    g.set_edge("b", "c");

    let pos = bk::position_x(&g);
    let b = pos["b"];
    let c = b;
    assert_eq!(pos["a"], b - 60.0 / 2.0 - 10.0 - 50.0 / 2.0);
    assert_eq!(pos["b"], b);
    assert_eq!(pos["c"], c);
    assert_eq!(pos["d"], c + 70.0 / 2.0 + 10.0 + 80.0 / 2.0);
}

#[test]
fn position_x_aligns_inner_segments() {
    let mut g = new_graph();
    g.graph_mut().nodesep = 10.0;
    g.graph_mut().edgesep = 10.0;
    set_node_with(&mut g, "a", 0, 0, 50.0, Some("dummy"), None);
    set_node_with(&mut g, "b", 0, 1, 60.0, None, None);
    set_node_with(&mut g, "c", 1, 0, 70.0, None, None);
    set_node_with(&mut g, "d", 1, 1, 80.0, Some("dummy"), None);
    g.set_edge("b", "c");
    g.set_edge("a", "d");

    let pos = bk::position_x(&g);
    let a = pos["a"];
    let d = a;
    assert_eq!(pos["a"], a);
    assert_eq!(pos["b"], a + 50.0 / 2.0 + 10.0 + 60.0 / 2.0);
    assert_eq!(pos["c"], d - 70.0 / 2.0 - 10.0 - 80.0 / 2.0);
    assert_eq!(pos["d"], d);
}
