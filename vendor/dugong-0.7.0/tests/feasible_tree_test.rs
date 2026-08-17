use dugong::graphlib::{Graph, GraphOptions};
use dugong::rank;
use dugong::{EdgeLabel, GraphLabel, NodeLabel};

fn edge(minlen: usize) -> EdgeLabel {
    EdgeLabel {
        minlen,
        ..Default::default()
    }
}

#[test]
fn feasible_tree_creates_a_tree_for_a_trivial_input_graph() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions::default());
    g.set_graph(GraphLabel::default());
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
    g.set_edge_with_label("a", "b", edge(1));

    let tree = rank::feasible_tree::feasible_tree(&mut g);
    assert_eq!(
        g.node("b").unwrap().rank,
        Some(g.node("a").unwrap().rank.unwrap() + 1)
    );
    assert_eq!(tree.neighbors("a"), vec!["b"]);
}

#[test]
fn feasible_tree_correctly_shortens_slack_by_pulling_a_node_up() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions::default());
    g.set_graph(GraphLabel::default());
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
            rank: Some(2),
            ..Default::default()
        },
    );
    g.set_node(
        "d",
        NodeLabel {
            rank: Some(2),
            ..Default::default()
        },
    );
    g.set_edge_with_label("a", "b", edge(1));
    g.set_edge_with_label("b", "c", edge(1));
    g.set_edge_with_label("a", "d", edge(1));

    let tree = rank::feasible_tree::feasible_tree(&mut g);
    assert_eq!(
        g.node("b").unwrap().rank,
        Some(g.node("a").unwrap().rank.unwrap() + 1)
    );
    assert_eq!(
        g.node("c").unwrap().rank,
        Some(g.node("b").unwrap().rank.unwrap() + 1)
    );
    assert_eq!(
        g.node("d").unwrap().rank,
        Some(g.node("a").unwrap().rank.unwrap() + 1)
    );

    let mut n_a: Vec<String> = tree
        .neighbors("a")
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let mut n_b: Vec<String> = tree
        .neighbors("b")
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    n_a.sort();
    n_b.sort();
    assert_eq!(n_a, vec!["b".to_string(), "d".to_string()]);
    assert_eq!(n_b, vec!["a".to_string(), "c".to_string()]);
    assert_eq!(tree.neighbors("c"), vec!["b"]);
    assert_eq!(tree.neighbors("d"), vec!["a"]);
}

#[test]
fn feasible_tree_correctly_shortens_slack_by_pulling_a_node_down() {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions::default());
    g.set_graph(GraphLabel::default());
    g.set_node(
        "a",
        NodeLabel {
            rank: Some(2),
            ..Default::default()
        },
    );
    g.set_node(
        "b",
        NodeLabel {
            rank: Some(0),
            ..Default::default()
        },
    );
    g.set_node(
        "c",
        NodeLabel {
            rank: Some(2),
            ..Default::default()
        },
    );
    g.set_edge_with_label("b", "a", edge(1));
    g.set_edge_with_label("b", "c", edge(1));

    let tree = rank::feasible_tree::feasible_tree(&mut g);
    assert_eq!(
        g.node("a").unwrap().rank,
        Some(g.node("b").unwrap().rank.unwrap() + 1)
    );
    assert_eq!(
        g.node("c").unwrap().rank,
        Some(g.node("b").unwrap().rank.unwrap() + 1)
    );

    let mut n_a: Vec<String> = tree
        .neighbors("a")
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let mut n_b: Vec<String> = tree
        .neighbors("b")
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let mut n_c: Vec<String> = tree
        .neighbors("c")
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    n_a.sort();
    n_b.sort();
    n_c.sort();
    assert_eq!(n_a, vec!["b".to_string()]);
    assert_eq!(n_b, vec!["a".to_string(), "c".to_string()]);
    assert_eq!(n_c, vec!["b".to_string()]);
}
