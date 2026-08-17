use dugong::graphlib::{Graph, GraphOptions, alg};
use dugong::greedy_fas;

fn check_fas(mut g: Graph<(), i64, ()>, fas: Vec<dugong::graphlib::EdgeKey>) {
    let n = g.node_count() as i64;
    let m = g.edge_count() as i64;
    for e in &fas {
        let _ = g.remove_edge_key(e);
    }
    assert_eq!(alg::find_cycles(&g), Vec::<Vec<String>>::new());
    let bound = (m / 2) - (n / 6);
    assert!(fas.len() as i64 <= bound);
}

#[test]
fn greedy_fas_returns_the_empty_set_for_empty_graphs() {
    let g: Graph<(), i64, ()> = Graph::new(GraphOptions {
        multigraph: false,
        compound: false,
        ..Default::default()
    });
    assert_eq!(greedy_fas::greedy_fas(&g), Vec::new());
}

#[test]
fn greedy_fas_returns_the_empty_set_for_single_node_graphs() {
    let mut g: Graph<(), i64, ()> = Graph::new(GraphOptions {
        multigraph: false,
        compound: false,
        ..Default::default()
    });
    g.set_node("a", ());
    assert_eq!(greedy_fas::greedy_fas(&g), Vec::new());
}

#[test]
fn greedy_fas_returns_an_empty_set_if_the_input_graph_is_acyclic() {
    let mut g: Graph<(), i64, ()> = Graph::new(GraphOptions {
        multigraph: false,
        compound: false,
        ..Default::default()
    });
    g.set_edge_with_label("a", "b", 1);
    g.set_edge_with_label("b", "c", 1);
    g.set_edge_with_label("b", "d", 1);
    g.set_edge_with_label("a", "e", 1);
    assert_eq!(greedy_fas::greedy_fas(&g), Vec::new());
}

#[test]
fn greedy_fas_returns_a_single_edge_with_a_simple_cycle() {
    let mut g: Graph<(), i64, ()> = Graph::new(GraphOptions {
        multigraph: false,
        compound: false,
        ..Default::default()
    });
    g.set_edge_with_label("a", "b", 1);
    g.set_edge_with_label("b", "a", 1);
    let fas = greedy_fas::greedy_fas(&g);
    check_fas(g, fas);
}

#[test]
fn greedy_fas_returns_a_single_edge_in_a_4_node_cycle() {
    let mut g: Graph<(), i64, ()> = Graph::new(GraphOptions {
        multigraph: false,
        compound: false,
        ..Default::default()
    });
    g.set_edge_with_label("n1", "n2", 1);
    g.set_path(&["n2", "n3", "n4", "n5", "n2"]);
    g.set_edge_with_label("n3", "n5", 1);
    g.set_edge_with_label("n4", "n2", 1);
    g.set_edge_with_label("n4", "n6", 1);
    let fas = greedy_fas::greedy_fas(&g);
    check_fas(g, fas);
}

#[test]
fn greedy_fas_returns_two_edges_for_two_4_node_cycles() {
    let mut g: Graph<(), i64, ()> = Graph::new(GraphOptions {
        multigraph: false,
        compound: false,
        ..Default::default()
    });
    g.set_edge_with_label("n1", "n2", 1);
    g.set_path(&["n2", "n3", "n4", "n5", "n2"]);
    g.set_edge_with_label("n3", "n5", 1);
    g.set_edge_with_label("n4", "n2", 1);
    g.set_edge_with_label("n4", "n6", 1);
    g.set_path(&["n6", "n7", "n8", "n9", "n6"]);
    g.set_edge_with_label("n7", "n9", 1);
    g.set_edge_with_label("n8", "n6", 1);
    g.set_edge_with_label("n8", "n10", 1);
    let fas = greedy_fas::greedy_fas(&g);
    check_fas(g, fas);
}

#[test]
fn greedy_fas_works_with_arbitrarily_weighted_edges() {
    let mut g1: Graph<(), i64, ()> = Graph::new(GraphOptions {
        multigraph: false,
        compound: false,
        ..Default::default()
    });
    g1.set_edge_with_label("n1", "n2", 2);
    g1.set_edge_with_label("n2", "n1", 1);
    let fas1 = greedy_fas::greedy_fas_with_weight(&g1, |w: &i64| *w);
    assert_eq!(
        fas1.iter()
            .map(|e| (e.v.clone(), e.w.clone(), e.name.clone()))
            .collect::<Vec<_>>(),
        vec![("n2".to_string(), "n1".to_string(), None)]
    );

    let mut g2: Graph<(), i64, ()> = Graph::new(GraphOptions {
        multigraph: false,
        compound: false,
        ..Default::default()
    });
    g2.set_edge_with_label("n1", "n2", 1);
    g2.set_edge_with_label("n2", "n1", 2);
    let fas2 = greedy_fas::greedy_fas_with_weight(&g2, |w: &i64| *w);
    assert_eq!(
        fas2.iter()
            .map(|e| (e.v.clone(), e.w.clone(), e.name.clone()))
            .collect::<Vec<_>>(),
        vec![("n1".to_string(), "n2".to_string(), None)]
    );
}

#[test]
fn greedy_fas_works_for_multigraphs() {
    let mut g: Graph<(), i64, ()> = Graph::new(GraphOptions {
        multigraph: true,
        compound: false,
        ..Default::default()
    });
    g.set_edge_named("a", "b", Some("foo"), Some(5));
    g.set_edge_named("b", "a", Some("bar"), Some(2));
    g.set_edge_named("b", "a", Some("baz"), Some(2));

    let mut fas = greedy_fas::greedy_fas_with_weight(&g, |w: &i64| *w);
    fas.sort_by_key(|e| e.name.clone().unwrap_or_default());
    assert_eq!(
        fas.iter()
            .map(|e| (e.v.clone(), e.w.clone(), e.name.clone()))
            .collect::<Vec<_>>(),
        vec![
            ("b".to_string(), "a".to_string(), Some("bar".to_string())),
            ("b".to_string(), "a".to_string(), Some("baz".to_string())),
        ]
    );
}
