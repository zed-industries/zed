use dugong::NodeLabel;
use dugong::graphlib::{EdgeKey, Graph, GraphOptions};
use dugong::order::add_subgraph_constraints;

fn new_compound_graph() -> Graph<NodeLabel, (), ()> {
    Graph::new(GraphOptions {
        compound: true,
        ..Default::default()
    })
}

#[test]
fn add_subgraph_constraints_does_not_change_cg_for_a_flat_set_of_nodes() {
    let mut g = new_compound_graph();
    let mut cg: Graph<(), (), ()> = Graph::new(GraphOptions::default());

    let vs = vec!["a", "b", "c", "d"];
    for v in &vs {
        g.ensure_node(*v);
    }

    add_subgraph_constraints(
        &g,
        &mut cg,
        &vs.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
    );
    assert_eq!(cg.node_count(), 0);
    assert_eq!(cg.edge_count(), 0);
}

#[test]
fn add_subgraph_constraints_does_not_create_a_constraint_for_contiguous_subgraph_nodes() {
    let mut g = new_compound_graph();
    let mut cg: Graph<(), (), ()> = Graph::new(GraphOptions::default());

    let vs = vec!["a", "b", "c"];
    for v in &vs {
        g.set_parent(*v, "sg");
    }

    add_subgraph_constraints(
        &g,
        &mut cg,
        &vs.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
    );
    assert_eq!(cg.node_count(), 0);
    assert_eq!(cg.edge_count(), 0);
}

#[test]
fn add_subgraph_constraints_adds_a_constraint_when_parents_for_adjacent_nodes_are_different() {
    let mut g = new_compound_graph();
    let mut cg: Graph<(), (), ()> = Graph::new(GraphOptions::default());

    let vs = ["a", "b"];
    g.set_parent("a", "sg1");
    g.set_parent("b", "sg2");

    add_subgraph_constraints(
        &g,
        &mut cg,
        &vs.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
    );

    let edges: Vec<EdgeKey> = cg.edges().cloned().collect();
    assert_eq!(edges, vec![EdgeKey::new("sg1", "sg2", None::<String>)]);
}

#[test]
fn add_subgraph_constraints_works_for_multiple_levels() {
    let mut g = new_compound_graph();
    let mut cg: Graph<(), (), ()> = Graph::new(GraphOptions::default());

    let vs = vec!["a", "b", "c", "d", "e", "f", "g", "h"];
    for v in &vs {
        g.ensure_node(*v);
    }
    g.set_parent("b", "sg2");
    g.set_parent("sg2", "sg1");
    g.set_parent("c", "sg1");
    g.set_parent("d", "sg3");
    g.set_parent("sg3", "sg1");
    g.set_parent("f", "sg4");
    g.set_parent("g", "sg5");
    g.set_parent("sg5", "sg4");

    add_subgraph_constraints(
        &g,
        &mut cg,
        &vs.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
    );

    let mut edges: Vec<EdgeKey> = cg.edges().cloned().collect();
    edges.sort_by(|a, b| a.v.cmp(&b.v));
    assert_eq!(
        edges,
        vec![
            EdgeKey::new("sg1", "sg4", None::<String>),
            EdgeKey::new("sg2", "sg3", None::<String>)
        ]
    );
}
