use dugong::graphlib::{EdgeKey, Graph, GraphOptions};
use dugong::rank;
use dugong::rank::tree::{TreeEdgeLabel, TreeNodeLabel};
use dugong::{EdgeLabel, GraphLabel, NodeLabel, util};

fn new_graph() -> Graph<NodeLabel, EdgeLabel, GraphLabel> {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: true,
        ..Default::default()
    });
    g.set_graph(GraphLabel::default());
    g.set_default_node_label(NodeLabel::default);
    g.set_default_edge_label(|| EdgeLabel {
        minlen: 1,
        weight: 1.0,
        ..Default::default()
    });
    g
}

fn gansner_graph() -> Graph<NodeLabel, EdgeLabel, GraphLabel> {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions::default());
    g.set_graph(GraphLabel::default());
    g.set_default_node_label(NodeLabel::default);
    g.set_default_edge_label(|| EdgeLabel {
        minlen: 1,
        weight: 1.0,
        ..Default::default()
    });
    g.set_path(&["a", "b", "c", "d", "h"]);
    g.set_path(&["a", "e", "g", "h"]);
    g.set_path(&["a", "f", "g"]);
    g
}

fn gansner_tree() -> Graph<TreeNodeLabel, TreeEdgeLabel, ()> {
    let mut t: Graph<TreeNodeLabel, TreeEdgeLabel, ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    t.set_default_node_label(TreeNodeLabel::default);
    t.set_default_edge_label(TreeEdgeLabel::default);
    t.set_path(&["a", "b", "c", "d", "h", "g", "e"]);
    t.set_edge("g", "f");
    t
}

fn ns(g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>) {
    rank::network_simplex::network_simplex(g);
    util::normalize_ranks(g);
}

fn rank_by_ix(g: &Graph<NodeLabel, EdgeLabel, GraphLabel>) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();
    g.for_each_node_ix(|ix, _id, lbl| {
        if ix >= out.len() {
            out.resize(ix + 1, 0);
        }
        out[ix] = lbl.rank.unwrap_or(0);
    });
    out
}

fn undirected_edge(e: &EdgeKey) -> (String, String) {
    if e.v <= e.w {
        (e.v.clone(), e.w.clone())
    } else {
        (e.w.clone(), e.v.clone())
    }
}

fn ek(v: &str, w: &str) -> EdgeKey {
    EdgeKey {
        v: v.to_string(),
        w: w.to_string(),
        name: None,
    }
}

#[test]
fn network_simplex_can_assign_a_rank_to_a_single_node() {
    let mut g = new_graph();
    g.set_node("a", NodeLabel::default());
    ns(&mut g);
    assert_eq!(g.node("a").unwrap().rank, Some(0));
}

#[test]
fn network_simplex_can_assign_a_rank_to_a_2_node_connected_graph() {
    let mut g = new_graph();
    g.set_edge("a", "b");
    ns(&mut g);
    assert_eq!(g.node("a").unwrap().rank, Some(0));
    assert_eq!(g.node("b").unwrap().rank, Some(1));
}

#[test]
fn network_simplex_can_assign_ranks_for_a_diamond() {
    let mut g = new_graph();
    g.set_path(&["a", "b", "d"]);
    g.set_path(&["a", "c", "d"]);
    ns(&mut g);
    assert_eq!(g.node("a").unwrap().rank, Some(0));
    assert_eq!(g.node("b").unwrap().rank, Some(1));
    assert_eq!(g.node("c").unwrap().rank, Some(1));
    assert_eq!(g.node("d").unwrap().rank, Some(2));
}

#[test]
fn network_simplex_uses_the_minlen_attribute_on_the_edge() {
    let mut g = new_graph();
    g.set_path(&["a", "b", "d"]);
    g.set_edge("a", "c");
    g.set_edge_with_label(
        "c",
        "d",
        EdgeLabel {
            minlen: 2,
            weight: 1.0,
            ..Default::default()
        },
    );
    ns(&mut g);
    assert_eq!(g.node("a").unwrap().rank, Some(0));
    assert_eq!(g.node("b").unwrap().rank, Some(2));
    assert_eq!(g.node("c").unwrap().rank, Some(1));
    assert_eq!(g.node("d").unwrap().rank, Some(3));
}

#[test]
fn network_simplex_can_rank_the_gansner_graph() {
    let mut g = gansner_graph();
    ns(&mut g);
    assert_eq!(g.node("a").unwrap().rank, Some(0));
    assert_eq!(g.node("b").unwrap().rank, Some(1));
    assert_eq!(g.node("c").unwrap().rank, Some(2));
    assert_eq!(g.node("d").unwrap().rank, Some(3));
    assert_eq!(g.node("h").unwrap().rank, Some(4));
    assert_eq!(g.node("e").unwrap().rank, Some(1));
    assert_eq!(g.node("f").unwrap().rank, Some(1));
    assert_eq!(g.node("g").unwrap().rank, Some(2));
}

#[test]
fn network_simplex_can_handle_multi_edges() {
    let mut g = new_graph();
    g.set_path(&["a", "b", "c", "d"]);
    g.set_edge_with_label(
        "a",
        "e",
        EdgeLabel {
            weight: 2.0,
            minlen: 1,
            ..Default::default()
        },
    );
    g.set_edge("e", "d");
    g.set_edge_named(
        "b",
        "c",
        Some("multi"),
        Some(EdgeLabel {
            weight: 1.0,
            minlen: 2,
            ..Default::default()
        }),
    );
    ns(&mut g);
    assert_eq!(g.node("a").unwrap().rank, Some(0));
    assert_eq!(g.node("b").unwrap().rank, Some(1));
    assert_eq!(g.node("c").unwrap().rank, Some(3));
    assert_eq!(g.node("d").unwrap().rank, Some(4));
    assert_eq!(g.node("e").unwrap().rank, Some(1));
}

#[test]
fn leave_edge_returns_none_if_there_is_no_edge_with_a_negative_cutvalue() {
    let mut t: Graph<TreeNodeLabel, TreeEdgeLabel, ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    t.set_edge_with_label("a", "b", TreeEdgeLabel { cutvalue: 1.0 });
    t.set_edge_with_label("b", "c", TreeEdgeLabel { cutvalue: 1.0 });
    assert_eq!(rank::network_simplex::leave_edge(&t), None);
}

#[test]
fn leave_edge_returns_an_edge_if_one_is_found_with_a_negative_cutvalue() {
    let mut t: Graph<TreeNodeLabel, TreeEdgeLabel, ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    t.set_edge_with_label("a", "b", TreeEdgeLabel { cutvalue: 1.0 });
    t.set_edge_with_label("b", "c", TreeEdgeLabel { cutvalue: -1.0 });
    assert_eq!(rank::network_simplex::leave_edge(&t), Some(ek("b", "c")));
}

#[test]
fn enter_edge_finds_an_edge_from_the_head_to_tail_component() {
    let mut g = new_graph();
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
            rank: Some(2),
            ..Default::default()
        },
    );
    g.set_node(
        "c",
        NodeLabel {
            rank: Some(3),
            ..Default::default()
        },
    );
    g.set_path(&["a", "b", "c"]);
    g.set_edge("a", "c");

    let mut t: Graph<TreeNodeLabel, TreeEdgeLabel, ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    t.set_path(&["b", "c", "a"]);
    rank::network_simplex::init_low_lim_values(&mut t, Some("c"));

    let rank_by_ix = rank_by_ix(&g);
    let f = rank::network_simplex::enter_edge(&t, &g, &rank_by_ix, &ek("b", "c"));
    assert_eq!(undirected_edge(&f), undirected_edge(&ek("a", "b")));
}

#[test]
fn enter_edge_works_when_the_root_of_the_tree_is_in_the_tail_component() {
    let mut g = new_graph();
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
            rank: Some(2),
            ..Default::default()
        },
    );
    g.set_node(
        "c",
        NodeLabel {
            rank: Some(3),
            ..Default::default()
        },
    );
    g.set_path(&["a", "b", "c"]);
    g.set_edge("a", "c");

    let mut t: Graph<TreeNodeLabel, TreeEdgeLabel, ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    t.set_path(&["b", "c", "a"]);
    rank::network_simplex::init_low_lim_values(&mut t, Some("b"));

    let rank_by_ix = rank_by_ix(&g);
    let f = rank::network_simplex::enter_edge(&t, &g, &rank_by_ix, &ek("b", "c"));
    assert_eq!(undirected_edge(&f), undirected_edge(&ek("a", "b")));
}

#[test]
fn enter_edge_finds_the_edge_with_the_least_slack() {
    let mut g = new_graph();
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
            rank: Some(1),
            ..Default::default()
        },
    );
    g.set_node(
        "c",
        NodeLabel {
            rank: Some(3),
            ..Default::default()
        },
    );
    g.set_node(
        "d",
        NodeLabel {
            rank: Some(4),
            ..Default::default()
        },
    );
    g.set_edge("a", "d");
    g.set_path(&["a", "c", "d"]);
    g.set_edge("b", "c");

    let mut t: Graph<TreeNodeLabel, TreeEdgeLabel, ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    t.set_path(&["c", "d", "a", "b"]);
    rank::network_simplex::init_low_lim_values(&mut t, Some("a"));

    let rank_by_ix = rank_by_ix(&g);
    let f = rank::network_simplex::enter_edge(&t, &g, &rank_by_ix, &ek("c", "d"));
    assert_eq!(undirected_edge(&f), undirected_edge(&ek("b", "c")));
}

#[test]
fn enter_edge_finds_an_appropriate_edge_for_gansner_graph_1() {
    let mut g = gansner_graph();
    let mut t = gansner_tree();
    rank::util::longest_path(&mut g);
    rank::network_simplex::init_low_lim_values(&mut t, Some("a"));

    let rank_by_ix = rank_by_ix(&g);
    let f = rank::network_simplex::enter_edge(&t, &g, &rank_by_ix, &ek("g", "h"));
    let (u, v) = undirected_edge(&f);
    assert_eq!(u, "a");
    assert!(v == "e" || v == "f");
}

#[test]
fn enter_edge_finds_an_appropriate_edge_for_gansner_graph_2() {
    let mut g = gansner_graph();
    let mut t = gansner_tree();
    rank::util::longest_path(&mut g);
    rank::network_simplex::init_low_lim_values(&mut t, Some("e"));

    let rank_by_ix = rank_by_ix(&g);
    let f = rank::network_simplex::enter_edge(&t, &g, &rank_by_ix, &ek("g", "h"));
    let (u, v) = undirected_edge(&f);
    assert_eq!(u, "a");
    assert!(v == "e" || v == "f");
}

#[test]
fn enter_edge_finds_an_appropriate_edge_for_gansner_graph_3() {
    let mut g = gansner_graph();
    let mut t = gansner_tree();
    rank::util::longest_path(&mut g);
    rank::network_simplex::init_low_lim_values(&mut t, Some("a"));

    let rank_by_ix = rank_by_ix(&g);
    let f = rank::network_simplex::enter_edge(&t, &g, &rank_by_ix, &ek("h", "g"));
    let (u, v) = undirected_edge(&f);
    assert_eq!(u, "a");
    assert!(v == "e" || v == "f");
}

#[test]
fn enter_edge_finds_an_appropriate_edge_for_gansner_graph_4() {
    let mut g = gansner_graph();
    let mut t = gansner_tree();
    rank::util::longest_path(&mut g);
    rank::network_simplex::init_low_lim_values(&mut t, Some("e"));

    let rank_by_ix = rank_by_ix(&g);
    let f = rank::network_simplex::enter_edge(&t, &g, &rank_by_ix, &ek("h", "g"));
    let (u, v) = undirected_edge(&f);
    assert_eq!(u, "a");
    assert!(v == "e" || v == "f");
}

#[test]
fn init_low_lim_values_assigns_low_lim_and_parent_for_each_node_in_a_tree() {
    let mut g: Graph<TreeNodeLabel, TreeEdgeLabel, ()> = Graph::new(GraphOptions::default());
    g.set_default_node_label(TreeNodeLabel::default);
    g.set_default_edge_label(TreeEdgeLabel::default);
    for v in ["a", "b", "c", "d", "e"] {
        g.set_node(v, TreeNodeLabel::default());
    }
    g.set_path(&["a", "b", "a", "c", "d", "c", "e"]);

    rank::network_simplex::init_low_lim_values(&mut g, Some("a"));

    let mut lims: Vec<i32> = g
        .node_ids()
        .iter()
        .map(|v| g.node(v).unwrap().lim)
        .collect();
    lims.sort();
    assert_eq!(lims, vec![1, 2, 3, 4, 5]);

    let a = g.node("a").unwrap();
    assert_eq!(a.low, 1);
    assert_eq!(a.lim, 5);
    assert_eq!(a.parent, None);

    let b = g.node("b").unwrap();
    let c = g.node("c").unwrap();
    let d = g.node("d").unwrap();
    let e = g.node("e").unwrap();

    assert_eq!(b.parent.as_deref(), Some("a"));
    assert!(b.lim < a.lim);

    assert_eq!(c.parent.as_deref(), Some("a"));
    assert!(c.lim < a.lim);
    assert_ne!(c.lim, b.lim);

    assert_eq!(d.parent.as_deref(), Some("c"));
    assert!(d.lim < c.lim);

    assert_eq!(e.parent.as_deref(), Some("c"));
    assert!(e.lim < c.lim);
    assert_ne!(e.lim, d.lim);
}

#[test]
fn exchange_edges_exchanges_edges_and_updates_cut_values_and_low_lim_numbers() {
    let mut g = gansner_graph();
    let mut t = gansner_tree();
    rank::util::longest_path(&mut g);
    rank::network_simplex::init_low_lim_values(&mut t, None);

    let mut rank_by_ix = rank_by_ix(&g);
    rank::network_simplex::exchange_edges(
        &mut t,
        &mut g,
        &mut rank_by_ix,
        &ek("g", "h"),
        &ek("a", "e"),
    );

    assert_eq!(t.edge("a", "b", None).unwrap().cutvalue, 2.0);
    assert_eq!(t.edge("b", "c", None).unwrap().cutvalue, 2.0);
    assert_eq!(t.edge("c", "d", None).unwrap().cutvalue, 2.0);
    assert_eq!(t.edge("d", "h", None).unwrap().cutvalue, 2.0);
    assert_eq!(t.edge("a", "e", None).unwrap().cutvalue, 1.0);
    assert_eq!(t.edge("e", "g", None).unwrap().cutvalue, 1.0);
    assert_eq!(t.edge("g", "f", None).unwrap().cutvalue, 0.0);

    let mut lims: Vec<i32> = t
        .node_ids()
        .iter()
        .map(|v| t.node(v).unwrap().lim)
        .collect();
    lims.sort();
    assert_eq!(lims, vec![1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn exchange_edges_updates_ranks() {
    let mut g = gansner_graph();
    let mut t = gansner_tree();
    rank::util::longest_path(&mut g);
    rank::network_simplex::init_low_lim_values(&mut t, None);

    let mut rank_by_ix = rank_by_ix(&g);
    rank::network_simplex::exchange_edges(
        &mut t,
        &mut g,
        &mut rank_by_ix,
        &ek("g", "h"),
        &ek("a", "e"),
    );
    util::normalize_ranks(&mut g);

    assert_eq!(g.node("a").unwrap().rank, Some(0));
    assert_eq!(g.node("b").unwrap().rank, Some(1));
    assert_eq!(g.node("c").unwrap().rank, Some(2));
    assert_eq!(g.node("d").unwrap().rank, Some(3));
    assert_eq!(g.node("e").unwrap().rank, Some(1));
    assert_eq!(g.node("f").unwrap().rank, Some(1));
    assert_eq!(g.node("g").unwrap().rank, Some(2));
    assert_eq!(g.node("h").unwrap().rank, Some(4));
}

#[test]
fn calc_cut_value_works_for_a_2_node_tree_with_c_to_p() {
    let mut g = new_graph();
    let mut t: Graph<TreeNodeLabel, TreeEdgeLabel, ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    g.set_path(&["c", "p"]);
    t.set_path(&["p", "c"]);
    rank::network_simplex::init_low_lim_values(&mut t, Some("p"));
    assert_eq!(rank::network_simplex::calc_cut_value(&t, &g, "c"), 1.0);
}

#[test]
fn calc_cut_value_works_for_a_2_node_tree_with_c_from_p() {
    let mut g = new_graph();
    let mut t: Graph<TreeNodeLabel, TreeEdgeLabel, ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    g.set_path(&["p", "c"]);
    t.set_path(&["p", "c"]);
    rank::network_simplex::init_low_lim_values(&mut t, Some("p"));
    assert_eq!(rank::network_simplex::calc_cut_value(&t, &g, "c"), 1.0);
}

#[test]
fn calc_cut_value_works_for_3_node_tree_with_gc_to_c_to_p() {
    let mut g = new_graph();
    let mut t: Graph<TreeNodeLabel, TreeEdgeLabel, ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    g.set_path(&["gc", "c", "p"]);
    t.set_edge_with_label("gc", "c", TreeEdgeLabel { cutvalue: 3.0 });
    t.set_edge("p", "c");
    rank::network_simplex::init_low_lim_values(&mut t, Some("p"));
    assert_eq!(rank::network_simplex::calc_cut_value(&t, &g, "c"), 3.0);
}

#[test]
fn calc_cut_value_works_for_3_node_tree_with_gc_to_c_from_p() {
    let mut g = new_graph();
    let mut t: Graph<TreeNodeLabel, TreeEdgeLabel, ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    g.set_edge("p", "c");
    g.set_edge("gc", "c");
    t.set_edge_with_label("gc", "c", TreeEdgeLabel { cutvalue: 3.0 });
    t.set_edge("p", "c");
    rank::network_simplex::init_low_lim_values(&mut t, Some("p"));
    assert_eq!(rank::network_simplex::calc_cut_value(&t, &g, "c"), -1.0);
}

#[test]
fn calc_cut_value_works_for_3_node_tree_with_gc_from_c_to_p() {
    let mut g = new_graph();
    let mut t: Graph<TreeNodeLabel, TreeEdgeLabel, ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    g.set_edge("c", "p");
    g.set_edge("c", "gc");
    t.set_edge_with_label("gc", "c", TreeEdgeLabel { cutvalue: 3.0 });
    t.set_edge("p", "c");
    rank::network_simplex::init_low_lim_values(&mut t, Some("p"));
    assert_eq!(rank::network_simplex::calc_cut_value(&t, &g, "c"), -1.0);
}

#[test]
fn calc_cut_value_works_for_3_node_tree_with_gc_from_c_from_p() {
    let mut g = new_graph();
    let mut t: Graph<TreeNodeLabel, TreeEdgeLabel, ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    g.set_path(&["p", "c", "gc"]);
    t.set_edge_with_label("gc", "c", TreeEdgeLabel { cutvalue: 3.0 });
    t.set_edge("p", "c");
    rank::network_simplex::init_low_lim_values(&mut t, Some("p"));
    assert_eq!(rank::network_simplex::calc_cut_value(&t, &g, "c"), 3.0);
}

#[test]
fn calc_cut_value_works_for_4_node_tree_gc_to_c_to_p_to_o_with_o_to_c() {
    let mut g = new_graph();
    let mut t: Graph<TreeNodeLabel, TreeEdgeLabel, ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    g.set_edge_with_label(
        "o",
        "c",
        EdgeLabel {
            weight: 7.0,
            minlen: 1,
            ..Default::default()
        },
    );
    g.set_path(&["gc", "c", "p", "o"]);
    t.set_edge_with_label("gc", "c", TreeEdgeLabel { cutvalue: 3.0 });
    t.set_path(&["c", "p", "o"]);
    rank::network_simplex::init_low_lim_values(&mut t, Some("p"));
    assert_eq!(rank::network_simplex::calc_cut_value(&t, &g, "c"), -4.0);
}

#[test]
fn calc_cut_value_works_for_4_node_tree_gc_to_c_to_p_to_o_with_o_from_c() {
    let mut g = new_graph();
    let mut t: Graph<TreeNodeLabel, TreeEdgeLabel, ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    g.set_edge_with_label(
        "c",
        "o",
        EdgeLabel {
            weight: 7.0,
            minlen: 1,
            ..Default::default()
        },
    );
    g.set_path(&["gc", "c", "p", "o"]);
    t.set_edge_with_label("gc", "c", TreeEdgeLabel { cutvalue: 3.0 });
    t.set_path(&["c", "p", "o"]);
    rank::network_simplex::init_low_lim_values(&mut t, Some("p"));
    assert_eq!(rank::network_simplex::calc_cut_value(&t, &g, "c"), 10.0);
}

#[test]
fn calc_cut_value_works_for_4_node_tree_o_to_gc_to_c_to_p_with_o_to_c() {
    let mut g = new_graph();
    let mut t: Graph<TreeNodeLabel, TreeEdgeLabel, ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    g.set_edge_with_label(
        "o",
        "c",
        EdgeLabel {
            weight: 7.0,
            minlen: 1,
            ..Default::default()
        },
    );
    g.set_path(&["o", "gc", "c", "p"]);
    t.set_edge("o", "gc");
    t.set_edge_with_label("gc", "c", TreeEdgeLabel { cutvalue: 3.0 });
    t.set_edge("c", "p");
    rank::network_simplex::init_low_lim_values(&mut t, Some("p"));
    assert_eq!(rank::network_simplex::calc_cut_value(&t, &g, "c"), -4.0);
}

#[test]
fn calc_cut_value_works_for_4_node_tree_o_to_gc_to_c_to_p_with_o_from_c() {
    let mut g = new_graph();
    let mut t: Graph<TreeNodeLabel, TreeEdgeLabel, ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    g.set_edge_with_label(
        "c",
        "o",
        EdgeLabel {
            weight: 7.0,
            minlen: 1,
            ..Default::default()
        },
    );
    g.set_path(&["o", "gc", "c", "p"]);
    t.set_edge("o", "gc");
    t.set_edge_with_label("gc", "c", TreeEdgeLabel { cutvalue: 3.0 });
    t.set_edge("c", "p");
    rank::network_simplex::init_low_lim_values(&mut t, Some("p"));
    assert_eq!(rank::network_simplex::calc_cut_value(&t, &g, "c"), 10.0);
}

#[test]
fn calc_cut_value_works_for_4_node_tree_gc_to_c_from_p_to_o_with_o_to_c() {
    let mut g = new_graph();
    let mut t: Graph<TreeNodeLabel, TreeEdgeLabel, ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    g.set_edge("gc", "c");
    g.set_edge("p", "c");
    g.set_edge("p", "o");
    g.set_edge_with_label(
        "o",
        "c",
        EdgeLabel {
            weight: 7.0,
            minlen: 1,
            ..Default::default()
        },
    );
    t.set_edge("o", "gc");
    t.set_edge_with_label("gc", "c", TreeEdgeLabel { cutvalue: 3.0 });
    t.set_edge("c", "p");
    rank::network_simplex::init_low_lim_values(&mut t, Some("p"));
    assert_eq!(rank::network_simplex::calc_cut_value(&t, &g, "c"), 6.0);
}

#[test]
fn calc_cut_value_works_for_4_node_tree_gc_to_c_from_p_to_o_with_o_from_c() {
    let mut g = new_graph();
    let mut t: Graph<TreeNodeLabel, TreeEdgeLabel, ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    g.set_edge("gc", "c");
    g.set_edge("p", "c");
    g.set_edge("p", "o");
    g.set_edge_with_label(
        "c",
        "o",
        EdgeLabel {
            weight: 7.0,
            minlen: 1,
            ..Default::default()
        },
    );
    t.set_edge("o", "gc");
    t.set_edge_with_label("gc", "c", TreeEdgeLabel { cutvalue: 3.0 });
    t.set_edge("c", "p");
    rank::network_simplex::init_low_lim_values(&mut t, Some("p"));
    assert_eq!(rank::network_simplex::calc_cut_value(&t, &g, "c"), -8.0);
}

#[test]
fn calc_cut_value_works_for_4_node_tree_o_to_gc_to_c_from_p_with_o_to_c() {
    let mut g = new_graph();
    let mut t: Graph<TreeNodeLabel, TreeEdgeLabel, ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    g.set_edge_with_label(
        "o",
        "c",
        EdgeLabel {
            weight: 7.0,
            minlen: 1,
            ..Default::default()
        },
    );
    g.set_path(&["o", "gc", "c"]);
    g.set_edge("p", "c");
    t.set_edge("o", "gc");
    t.set_edge_with_label("gc", "c", TreeEdgeLabel { cutvalue: 3.0 });
    t.set_edge("c", "p");
    rank::network_simplex::init_low_lim_values(&mut t, Some("p"));
    assert_eq!(rank::network_simplex::calc_cut_value(&t, &g, "c"), 6.0);
}

#[test]
fn calc_cut_value_works_for_4_node_tree_o_to_gc_to_c_from_p_with_o_from_c() {
    let mut g = new_graph();
    let mut t: Graph<TreeNodeLabel, TreeEdgeLabel, ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    g.set_edge_with_label(
        "c",
        "o",
        EdgeLabel {
            weight: 7.0,
            minlen: 1,
            ..Default::default()
        },
    );
    g.set_path(&["o", "gc", "c"]);
    g.set_edge("p", "c");
    t.set_edge("o", "gc");
    t.set_edge_with_label("gc", "c", TreeEdgeLabel { cutvalue: 3.0 });
    t.set_edge("c", "p");
    rank::network_simplex::init_low_lim_values(&mut t, Some("p"));
    assert_eq!(rank::network_simplex::calc_cut_value(&t, &g, "c"), -8.0);
}

#[test]
fn init_cut_values_works_for_gansner_graph() {
    let g = gansner_graph();
    let mut t = gansner_tree();
    rank::network_simplex::init_low_lim_values(&mut t, None);
    rank::network_simplex::init_cut_values(&mut t, &g);
    assert_eq!(t.edge("a", "b", None).unwrap().cutvalue, 3.0);
    assert_eq!(t.edge("b", "c", None).unwrap().cutvalue, 3.0);
    assert_eq!(t.edge("c", "d", None).unwrap().cutvalue, 3.0);
    assert_eq!(t.edge("d", "h", None).unwrap().cutvalue, 3.0);
    assert_eq!(t.edge("g", "h", None).unwrap().cutvalue, -1.0);
    assert_eq!(t.edge("e", "g", None).unwrap().cutvalue, 0.0);
    assert_eq!(t.edge("f", "g", None).unwrap().cutvalue, 0.0);
}

#[test]
fn init_cut_values_works_for_updated_gansner_graph() {
    let g = gansner_graph();
    let mut t = gansner_tree();
    let _ = t.remove_edge("g", "h", None);
    t.set_edge("a", "e");
    rank::network_simplex::init_low_lim_values(&mut t, None);
    rank::network_simplex::init_cut_values(&mut t, &g);
    assert_eq!(t.edge("a", "b", None).unwrap().cutvalue, 2.0);
    assert_eq!(t.edge("b", "c", None).unwrap().cutvalue, 2.0);
    assert_eq!(t.edge("c", "d", None).unwrap().cutvalue, 2.0);
    assert_eq!(t.edge("d", "h", None).unwrap().cutvalue, 2.0);
    assert_eq!(t.edge("a", "e", None).unwrap().cutvalue, 1.0);
    assert_eq!(t.edge("e", "g", None).unwrap().cutvalue, 1.0);
    assert_eq!(t.edge("f", "g", None).unwrap().cutvalue, 0.0);
}
