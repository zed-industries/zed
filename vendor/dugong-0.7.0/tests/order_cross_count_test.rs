use dugong::NodeLabel;
use dugong::graphlib::{Graph, GraphOptions};
use dugong::order::{WeightLabel, cross_count};

fn new_graph() -> Graph<NodeLabel, WeightLabel, ()> {
    let mut g: Graph<NodeLabel, WeightLabel, ()> = Graph::new(GraphOptions::default());
    g.set_default_edge_label(|| WeightLabel { weight: 1.0 });
    g
}

fn set_path(g: &mut Graph<NodeLabel, WeightLabel, ()>, path: &[&str]) {
    for w in path.windows(2) {
        g.set_edge(w[0], w[1]);
    }
}

#[test]
fn cross_count_returns_0_for_an_empty_layering() {
    let g = new_graph();
    assert_eq!(cross_count(&g, &[]), 0.0);
}

#[test]
fn cross_count_returns_0_for_a_layering_with_no_crossings() {
    let mut g = new_graph();
    g.set_edge("a1", "b1");
    g.set_edge("a2", "b2");
    assert_eq!(
        cross_count(
            &g,
            &[
                vec!["a1".to_string(), "a2".to_string()],
                vec!["b1".to_string(), "b2".to_string()]
            ]
        ),
        0.0
    );
}

#[test]
fn cross_count_returns_1_for_a_layering_with_1_crossing() {
    let mut g = new_graph();
    g.set_edge("a1", "b1");
    g.set_edge("a2", "b2");
    assert_eq!(
        cross_count(
            &g,
            &[
                vec!["a1".to_string(), "a2".to_string()],
                vec!["b2".to_string(), "b1".to_string()]
            ]
        ),
        1.0
    );
}

#[test]
fn cross_count_returns_a_weighted_crossing_count_for_a_layering_with_1_crossing() {
    let mut g = new_graph();
    g.set_edge_with_label("a1", "b1", WeightLabel { weight: 2.0 });
    g.set_edge_with_label("a2", "b2", WeightLabel { weight: 3.0 });
    assert_eq!(
        cross_count(
            &g,
            &[
                vec!["a1".to_string(), "a2".to_string()],
                vec!["b2".to_string(), "b1".to_string()]
            ]
        ),
        6.0
    );
}

#[test]
fn cross_count_calculates_crossings_across_layers() {
    let mut g = new_graph();
    set_path(&mut g, &["a1", "b1", "c1"]);
    set_path(&mut g, &["a2", "b2", "c2"]);
    assert_eq!(
        cross_count(
            &g,
            &[
                vec!["a1".to_string(), "a2".to_string()],
                vec!["b2".to_string(), "b1".to_string()],
                vec!["c1".to_string(), "c2".to_string()]
            ]
        ),
        2.0
    );
}

#[test]
fn cross_count_works_for_graph_1() {
    let mut g = new_graph();
    set_path(&mut g, &["a", "b", "c"]);
    set_path(&mut g, &["d", "e", "c"]);
    set_path(&mut g, &["a", "f", "i"]);
    g.set_edge("a", "e");

    assert_eq!(
        cross_count(
            &g,
            &[
                vec!["a".to_string(), "d".to_string()],
                vec!["b".to_string(), "e".to_string(), "f".to_string()],
                vec!["c".to_string(), "i".to_string()]
            ]
        ),
        1.0
    );
    assert_eq!(
        cross_count(
            &g,
            &[
                vec!["d".to_string(), "a".to_string()],
                vec!["e".to_string(), "b".to_string(), "f".to_string()],
                vec!["c".to_string(), "i".to_string()]
            ]
        ),
        0.0
    );
}
