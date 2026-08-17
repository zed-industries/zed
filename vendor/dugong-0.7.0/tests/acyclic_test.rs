use dugong::acyclic;
use dugong::graphlib::{Graph, GraphOptions, alg};
use dugong::{EdgeLabel, GraphLabel, NodeLabel};

fn strip_edges(
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
) -> Vec<(String, String, Option<String>)> {
    let mut edges: Vec<(String, String, Option<String>)> = g
        .edges()
        .map(|e| (e.v.clone(), e.w.clone(), e.name.clone()))
        .collect();
    edges.sort_by(|a, b| {
        let name_order = match (&a.2, &b.2) {
            (Some(an), Some(bn)) => an.cmp(bn),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        };
        if name_order != std::cmp::Ordering::Equal {
            return name_order;
        }
        let o = a.0.cmp(&b.0);
        if o != std::cmp::Ordering::Equal {
            return o;
        }
        a.1.cmp(&b.1)
    });
    edges
}

fn new_graph(acyclicer: &str) -> Graph<NodeLabel, EdgeLabel, GraphLabel> {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: true,
        compound: false,
        ..Default::default()
    });
    g.set_graph(GraphLabel {
        acyclicer: Some(acyclicer.to_string()),
        ..Default::default()
    });
    g.set_default_edge_label(|| EdgeLabel {
        minlen: 1,
        weight: 1.0,
        ..Default::default()
    });
    g
}

#[test]
fn acyclic_run_does_not_change_an_already_acyclic_graph() {
    for acyclicer in ["greedy", "dfs", "unknown-should-still-work"] {
        let mut g = new_graph(acyclicer);
        g.set_path(&["a", "b", "d"]);
        g.set_path(&["a", "c", "d"]);

        acyclic::run(&mut g);
        assert_eq!(
            strip_edges(&g),
            vec![
                ("a".to_string(), "b".to_string(), None),
                ("a".to_string(), "c".to_string(), None),
                ("b".to_string(), "d".to_string(), None),
                ("c".to_string(), "d".to_string(), None),
            ]
        );
    }
}

#[test]
fn acyclic_run_breaks_cycles_in_the_input_graph() {
    for acyclicer in ["greedy", "dfs", "unknown-should-still-work"] {
        let mut g = new_graph(acyclicer);
        g.set_path(&["a", "b", "c", "d", "a"]);
        acyclic::run(&mut g);
        assert_eq!(alg::find_cycles(&g), Vec::<Vec<String>>::new());
    }
}

#[test]
fn acyclic_run_creates_a_multi_edge_where_necessary() {
    for acyclicer in ["greedy", "dfs", "unknown-should-still-work"] {
        let mut g = new_graph(acyclicer);
        g.set_path(&["a", "b", "a"]);
        acyclic::run(&mut g);
        assert_eq!(alg::find_cycles(&g), Vec::<Vec<String>>::new());

        if g.has_edge("a", "b", None) {
            assert_eq!(g.out_edges("a", Some("b")).len(), 2);
        } else {
            assert_eq!(g.out_edges("b", Some("a")).len(), 2);
        }
        assert_eq!(g.edge_count(), 2);
    }
}

#[test]
fn acyclic_undo_does_not_change_edges_where_the_original_graph_was_acyclic() {
    for acyclicer in ["greedy", "dfs", "unknown-should-still-work"] {
        let mut g = new_graph(acyclicer);
        g.set_edge_with_label(
            "a",
            "b",
            EdgeLabel {
                minlen: 2,
                weight: 3.0,
                ..Default::default()
            },
        );
        acyclic::run(&mut g);
        acyclic::undo(&mut g);
        assert_eq!(
            g.edge("a", "b", None).unwrap(),
            &EdgeLabel {
                minlen: 2,
                weight: 3.0,
                ..Default::default()
            }
        );
        assert_eq!(g.edge_count(), 1);
    }
}

#[test]
fn acyclic_undo_can_restore_previously_reversed_edges() {
    for acyclicer in ["greedy", "dfs", "unknown-should-still-work"] {
        let mut g = new_graph(acyclicer);
        g.set_edge_with_label(
            "a",
            "b",
            EdgeLabel {
                minlen: 2,
                weight: 3.0,
                ..Default::default()
            },
        );
        g.set_edge_with_label(
            "b",
            "a",
            EdgeLabel {
                minlen: 3,
                weight: 4.0,
                ..Default::default()
            },
        );
        acyclic::run(&mut g);
        acyclic::undo(&mut g);
        assert_eq!(
            g.edge("a", "b", None).unwrap(),
            &EdgeLabel {
                minlen: 2,
                weight: 3.0,
                ..Default::default()
            }
        );
        assert_eq!(
            g.edge("b", "a", None).unwrap(),
            &EdgeLabel {
                minlen: 3,
                weight: 4.0,
                ..Default::default()
            }
        );
        assert_eq!(g.edge_count(), 2);
    }
}

#[test]
fn acyclic_greedy_prefers_to_break_cycles_at_low_weight_edges() {
    let mut g = new_graph("greedy");
    g.set_default_edge_label(|| EdgeLabel {
        minlen: 1,
        weight: 2.0,
        ..Default::default()
    });
    g.set_path(&["a", "b", "c", "d", "a"]);
    g.set_edge_with_label(
        "c",
        "d",
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    acyclic::run(&mut g);
    assert_eq!(alg::find_cycles(&g), Vec::<Vec<String>>::new());
    assert!(!g.has_edge("c", "d", None));
}

#[test]
fn acyclic_run_handles_deep_dfs_chains_with_small_stack() {
    const DEPTH: usize = 2048;
    let handle = std::thread::Builder::new()
        .name("dugong-deep-acyclic-dfs".to_string())
        .stack_size(64 * 1024)
        .spawn(|| {
            let mut g = new_graph("dfs");
            for i in 0..DEPTH {
                g.set_edge(format!("n{i}"), format!("n{}", i + 1));
            }

            acyclic::run(&mut g);
            assert_eq!(g.edge_count(), DEPTH);
            assert_eq!(alg::find_cycles(&g), Vec::<Vec<String>>::new());
        })
        .expect("spawn dugong deep acyclic dfs test");
    handle
        .join()
        .expect("dugong acyclic dfs should finish without stack overflow");
}
