use dugong_graphlib::{Graph, GraphOptions};

#[test]
fn undirected_edges_are_symmetric() {
    let mut g: Graph<(), i32, ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });

    g.set_edge_with_label("b", "a", 7);

    assert!(g.has_edge("a", "b", None));
    assert!(g.has_edge("b", "a", None));
    assert_eq!(g.edge("a", "b", None), Some(&7));
    assert_eq!(g.edge("b", "a", None), Some(&7));
}

#[test]
fn undirected_edges_are_incident_for_in_and_out_edges() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    g.set_edge("a", "b");
    g.set_edge("b", "c");

    assert_eq!(g.out_edges("b", None).len(), 2);
    assert_eq!(g.in_edges("b", None).len(), 2);

    assert_eq!(g.out_edges("b", Some("a")).len(), 1);
    assert_eq!(g.out_edges("b", Some("c")).len(), 1);
    assert_eq!(g.in_edges("b", Some("a")).len(), 1);
    assert_eq!(g.in_edges("b", Some("c")).len(), 1);
}

#[test]
fn undirected_successors_predecessors_and_neighbors_are_the_same() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    g.set_edge("a", "b");
    g.set_edge("b", "c");

    let mut succ = g.successors("b");
    let mut pred = g.predecessors("b");
    let mut neigh = g.neighbors("b");
    succ.sort();
    pred.sort();
    neigh.sort();

    assert_eq!(succ, vec!["a", "c"]);
    assert_eq!(pred, vec!["a", "c"]);
    assert_eq!(neigh, vec!["a", "c"]);
}

#[test]
fn undirected_node_edges_returns_incident_edges() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    g.set_edge("a", "b");
    g.set_edge("b", "c");

    let edges = g.node_edges("b");
    assert_eq!(edges.len(), 2);
    assert!(
        edges
            .iter()
            .any(|e| (e.v == "a" && e.w == "b") || (e.v == "b" && e.w == "a"))
    );
    assert!(
        edges
            .iter()
            .any(|e| (e.v == "b" && e.w == "c") || (e.v == "c" && e.w == "b"))
    );
}

#[test]
fn directed_successors_and_predecessors_respect_direction() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_edge("a", "b");

    assert_eq!(g.successors("a"), vec!["b"]);
    assert_eq!(g.successors("b"), Vec::<&str>::new());
    assert_eq!(g.predecessors("b"), vec!["a"]);
    assert_eq!(g.predecessors("a"), Vec::<&str>::new());
}
