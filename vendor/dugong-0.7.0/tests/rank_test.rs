use dugong::graphlib::{Graph, GraphOptions};
use dugong::rank;
use dugong::{EdgeLabel, GraphLabel, NodeLabel};

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

fn assert_respects_minlen(g: &Graph<NodeLabel, EdgeLabel, GraphLabel>) {
    for e in g.edges() {
        let v_rank = g.node(&e.v).unwrap().rank.unwrap();
        let w_rank = g.node(&e.w).unwrap().rank.unwrap();
        let minlen = g.edge_by_key(e).unwrap().minlen as i32;
        assert!(
            w_rank - v_rank >= minlen,
            "edge {} -> {} violates minlen {}: {} - {}",
            e.v,
            e.w,
            minlen,
            w_rank,
            v_rank
        );
    }
}

#[test]
fn rank_longest_path_respects_the_minlen_attribute() {
    let mut g = gansner_graph();
    g.graph_mut().ranker = Some("longest-path".to_string());
    rank::rank(&mut g);
    assert_respects_minlen(&g);
}

#[test]
fn rank_tight_tree_respects_the_minlen_attribute() {
    let mut g = gansner_graph();
    g.graph_mut().ranker = Some("tight-tree".to_string());
    rank::rank(&mut g);
    assert_respects_minlen(&g);
}

#[test]
fn rank_network_simplex_respects_the_minlen_attribute() {
    let mut g = gansner_graph();
    g.graph_mut().ranker = Some("network-simplex".to_string());
    rank::rank(&mut g);
    assert_respects_minlen(&g);
}

#[test]
fn rank_unknown_should_still_work_respects_the_minlen_attribute() {
    let mut g = gansner_graph();
    g.graph_mut().ranker = Some("unknown-should-still-work".to_string());
    rank::rank(&mut g);
    assert_respects_minlen(&g);
}

#[test]
fn rank_can_rank_a_single_node_graph_for_each_ranker() {
    for ranker in [
        "longest-path",
        "tight-tree",
        "network-simplex",
        "unknown-should-still-work",
    ] {
        let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions::default());
        g.set_graph(GraphLabel {
            ranker: Some(ranker.to_string()),
            ..Default::default()
        });
        g.set_node("a", NodeLabel::default());
        rank::rank(&mut g);
        assert_eq!(g.node("a").unwrap().rank, Some(0));
    }
}
