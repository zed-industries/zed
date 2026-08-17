use dugong::graphlib::{EdgeKey, Graph, GraphOptions};
use dugong::parent_dummy_chains;
use dugong::{EdgeLabel, GraphLabel, NodeLabel};

fn graph() -> Graph<NodeLabel, EdgeLabel, GraphLabel> {
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: false,
        compound: true,
        ..Default::default()
    });
    g.set_graph(GraphLabel::default());
    g
}

#[test]
fn parent_dummy_chains_does_not_set_a_parent_if_both_tail_and_head_have_no_parent() {
    let mut g = graph();
    g.set_node("a", NodeLabel::default());
    g.set_node("b", NodeLabel::default());
    g.set_node(
        "d1",
        NodeLabel {
            edge_obj: Some(EdgeKey::new("a", "b", None::<String>)),
            ..Default::default()
        },
    );
    g.graph_mut().dummy_chains = vec!["d1".to_string()];
    g.set_path(&["a", "d1", "b"]);

    parent_dummy_chains::parent_dummy_chains(&mut g);
    assert_eq!(g.parent("d1"), None);
}

#[test]
fn parent_dummy_chains_uses_the_tails_parent_for_the_first_node_if_it_is_not_the_root() {
    let mut g = graph();
    g.set_parent("a", "sg1");
    g.set_node(
        "sg1",
        NodeLabel {
            min_rank: Some(0),
            max_rank: Some(2),
            ..Default::default()
        },
    );
    g.set_node(
        "d1",
        NodeLabel {
            edge_obj: Some(EdgeKey::new("a", "b", None::<String>)),
            rank: Some(2),
            ..Default::default()
        },
    );
    g.graph_mut().dummy_chains = vec!["d1".to_string()];
    g.set_path(&["a", "d1", "b"]);

    parent_dummy_chains::parent_dummy_chains(&mut g);
    assert_eq!(g.parent("d1"), Some("sg1"));
}

#[test]
fn parent_dummy_chains_uses_the_heads_parent_for_the_first_node_if_tails_is_root() {
    let mut g = graph();
    g.set_parent("b", "sg1");
    g.set_node(
        "sg1",
        NodeLabel {
            min_rank: Some(1),
            max_rank: Some(3),
            ..Default::default()
        },
    );
    g.set_node(
        "d1",
        NodeLabel {
            edge_obj: Some(EdgeKey::new("a", "b", None::<String>)),
            rank: Some(1),
            ..Default::default()
        },
    );
    g.graph_mut().dummy_chains = vec!["d1".to_string()];
    g.set_path(&["a", "d1", "b"]);

    parent_dummy_chains::parent_dummy_chains(&mut g);
    assert_eq!(g.parent("d1"), Some("sg1"));
}

#[test]
fn parent_dummy_chains_handles_a_long_chain_starting_in_a_subgraph() {
    let mut g = graph();
    g.set_parent("a", "sg1");
    g.set_node(
        "sg1",
        NodeLabel {
            min_rank: Some(0),
            max_rank: Some(2),
            ..Default::default()
        },
    );
    g.set_node(
        "d1",
        NodeLabel {
            edge_obj: Some(EdgeKey::new("a", "b", None::<String>)),
            rank: Some(2),
            ..Default::default()
        },
    );
    g.set_node(
        "d2",
        NodeLabel {
            rank: Some(3),
            ..Default::default()
        },
    );
    g.set_node(
        "d3",
        NodeLabel {
            rank: Some(4),
            ..Default::default()
        },
    );
    g.graph_mut().dummy_chains = vec!["d1".to_string()];
    g.set_path(&["a", "d1", "d2", "d3", "b"]);

    parent_dummy_chains::parent_dummy_chains(&mut g);
    assert_eq!(g.parent("d1"), Some("sg1"));
    assert_eq!(g.parent("d2"), None);
    assert_eq!(g.parent("d3"), None);
}

#[test]
fn parent_dummy_chains_handles_a_long_chain_ending_in_a_subgraph() {
    let mut g = graph();
    g.set_parent("b", "sg1");
    g.set_node(
        "sg1",
        NodeLabel {
            min_rank: Some(3),
            max_rank: Some(5),
            ..Default::default()
        },
    );
    g.set_node(
        "d1",
        NodeLabel {
            edge_obj: Some(EdgeKey::new("a", "b", None::<String>)),
            rank: Some(1),
            ..Default::default()
        },
    );
    g.set_node(
        "d2",
        NodeLabel {
            rank: Some(2),
            ..Default::default()
        },
    );
    g.set_node(
        "d3",
        NodeLabel {
            rank: Some(3),
            ..Default::default()
        },
    );
    g.graph_mut().dummy_chains = vec!["d1".to_string()];
    g.set_path(&["a", "d1", "d2", "d3", "b"]);

    parent_dummy_chains::parent_dummy_chains(&mut g);
    assert_eq!(g.parent("d1"), None);
    assert_eq!(g.parent("d2"), None);
    assert_eq!(g.parent("d3"), Some("sg1"));
}

#[test]
fn parent_dummy_chains_handles_nested_subgraphs() {
    let mut g = graph();
    g.set_parent("a", "sg2");
    g.set_parent("sg2", "sg1");
    g.set_node(
        "sg1",
        NodeLabel {
            min_rank: Some(0),
            max_rank: Some(4),
            ..Default::default()
        },
    );
    g.set_node(
        "sg2",
        NodeLabel {
            min_rank: Some(1),
            max_rank: Some(3),
            ..Default::default()
        },
    );
    g.set_parent("b", "sg4");
    g.set_parent("sg4", "sg3");
    g.set_node(
        "sg3",
        NodeLabel {
            min_rank: Some(6),
            max_rank: Some(10),
            ..Default::default()
        },
    );
    g.set_node(
        "sg4",
        NodeLabel {
            min_rank: Some(7),
            max_rank: Some(9),
            ..Default::default()
        },
    );
    for i in 0..5 {
        g.set_node(
            format!("d{}", i + 1),
            NodeLabel {
                rank: Some(i + 3),
                ..Default::default()
            },
        );
    }
    if let Some(n) = g.node_mut("d1") {
        n.edge_obj = Some(EdgeKey::new("a", "b", None::<String>));
    }
    g.graph_mut().dummy_chains = vec!["d1".to_string()];
    g.set_path(&["a", "d1", "d2", "d3", "d4", "d5", "b"]);

    parent_dummy_chains::parent_dummy_chains(&mut g);
    assert_eq!(g.parent("d1"), Some("sg2"));
    assert_eq!(g.parent("d2"), Some("sg1"));
    assert_eq!(g.parent("d3"), None);
    assert_eq!(g.parent("d4"), Some("sg3"));
    assert_eq!(g.parent("d5"), Some("sg4"));
}

#[test]
fn parent_dummy_chains_handles_overlapping_rank_ranges() {
    let mut g = graph();
    g.set_parent("a", "sg1");
    g.set_node(
        "sg1",
        NodeLabel {
            min_rank: Some(0),
            max_rank: Some(3),
            ..Default::default()
        },
    );
    g.set_parent("b", "sg2");
    g.set_node(
        "sg2",
        NodeLabel {
            min_rank: Some(2),
            max_rank: Some(6),
            ..Default::default()
        },
    );
    g.set_node(
        "d1",
        NodeLabel {
            edge_obj: Some(EdgeKey::new("a", "b", None::<String>)),
            rank: Some(2),
            ..Default::default()
        },
    );
    g.set_node(
        "d2",
        NodeLabel {
            rank: Some(3),
            ..Default::default()
        },
    );
    g.set_node(
        "d3",
        NodeLabel {
            rank: Some(4),
            ..Default::default()
        },
    );
    g.graph_mut().dummy_chains = vec!["d1".to_string()];
    g.set_path(&["a", "d1", "d2", "d3", "b"]);

    parent_dummy_chains::parent_dummy_chains(&mut g);
    assert_eq!(g.parent("d1"), Some("sg1"));
    assert_eq!(g.parent("d2"), Some("sg1"));
    assert_eq!(g.parent("d3"), Some("sg2"));
}

#[test]
fn parent_dummy_chains_handles_an_lca_that_is_not_the_root_1() {
    let mut g = graph();
    g.set_parent("a", "sg1");
    g.set_parent("sg2", "sg1");
    g.set_node(
        "sg1",
        NodeLabel {
            min_rank: Some(0),
            max_rank: Some(6),
            ..Default::default()
        },
    );
    g.set_parent("b", "sg2");
    g.set_node(
        "sg2",
        NodeLabel {
            min_rank: Some(3),
            max_rank: Some(5),
            ..Default::default()
        },
    );
    g.set_node(
        "d1",
        NodeLabel {
            edge_obj: Some(EdgeKey::new("a", "b", None::<String>)),
            rank: Some(2),
            ..Default::default()
        },
    );
    g.set_node(
        "d2",
        NodeLabel {
            rank: Some(3),
            ..Default::default()
        },
    );
    g.graph_mut().dummy_chains = vec!["d1".to_string()];
    g.set_path(&["a", "d1", "d2", "b"]);

    parent_dummy_chains::parent_dummy_chains(&mut g);
    assert_eq!(g.parent("d1"), Some("sg1"));
    assert_eq!(g.parent("d2"), Some("sg2"));
}

#[test]
fn parent_dummy_chains_handles_an_lca_that_is_not_the_root_2() {
    let mut g = graph();
    g.set_parent("a", "sg2");
    g.set_parent("sg2", "sg1");
    g.set_node(
        "sg1",
        NodeLabel {
            min_rank: Some(0),
            max_rank: Some(6),
            ..Default::default()
        },
    );
    g.set_parent("b", "sg1");
    g.set_node(
        "sg2",
        NodeLabel {
            min_rank: Some(1),
            max_rank: Some(3),
            ..Default::default()
        },
    );
    g.set_node(
        "d1",
        NodeLabel {
            edge_obj: Some(EdgeKey::new("a", "b", None::<String>)),
            rank: Some(3),
            ..Default::default()
        },
    );
    g.set_node(
        "d2",
        NodeLabel {
            rank: Some(4),
            ..Default::default()
        },
    );
    g.graph_mut().dummy_chains = vec!["d1".to_string()];
    g.set_path(&["a", "d1", "d2", "b"]);

    parent_dummy_chains::parent_dummy_chains(&mut g);
    assert_eq!(g.parent("d1"), Some("sg2"));
    assert_eq!(g.parent("d2"), Some("sg1"));
}
