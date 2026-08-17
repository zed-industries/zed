use dugong_graphlib::{Graph, GraphOptions, alg};

fn sorted_components(mut components: Vec<Vec<String>>) -> Vec<Vec<String>> {
    for component in &mut components {
        component.sort();
    }
    components.sort_by(|a, b| a.first().cmp(&b.first()));
    components
}

#[test]
fn components_returns_empty_for_empty_graph() {
    let g: Graph<(), (), ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });

    assert!(alg::components(&g).is_empty());
}

#[test]
fn components_returns_singletons_for_unconnected_nodes() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    g.ensure_node("a");
    g.ensure_node("b");

    assert_eq!(
        sorted_components(alg::components(&g)),
        vec![vec!["a".to_string()], vec!["b".to_string()]]
    );
}

#[test]
fn components_returns_undirected_component_nodes() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });
    g.set_edge("a", "b");
    g.set_edge("b", "c");

    assert_eq!(
        sorted_components(alg::components(&g)),
        vec![vec!["a".to_string(), "b".to_string(), "c".to_string()]]
    );
}

#[test]
fn components_uses_neighbor_relationships_in_directed_graphs() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_path(&["a", "b", "c", "a"]);
    g.set_edge("d", "c");
    g.set_edge("e", "f");

    assert_eq!(
        sorted_components(alg::components(&g)),
        vec![
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string()
            ],
            vec!["e".to_string(), "f".to_string()]
        ]
    );
}

#[test]
fn find_cycles_returns_empty_for_empty_graph() {
    let g: Graph<(), (), ()> = Graph::new(GraphOptions::default());

    assert!(alg::find_cycles(&g).is_empty());
}

#[test]
fn find_cycles_returns_empty_for_acyclic_graph() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_path(&["a", "b", "c"]);

    assert!(alg::find_cycles(&g).is_empty());
}

#[test]
fn find_cycles_returns_single_node_cycle() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_path(&["a", "a"]);

    assert_eq!(sorted_components(alg::find_cycles(&g)), vec![vec!["a"]]);
}

#[test]
fn find_cycles_returns_two_node_cycle() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_path(&["a", "b", "a"]);

    assert_eq!(
        sorted_components(alg::find_cycles(&g)),
        vec![vec!["a".to_string(), "b".to_string()]]
    );
}

#[test]
fn find_cycles_returns_triangle_cycle() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_path(&["a", "b", "c", "a"]);

    assert_eq!(
        sorted_components(alg::find_cycles(&g)),
        vec![vec!["a".to_string(), "b".to_string(), "c".to_string()]]
    );
}

#[test]
fn find_cycles_returns_multiple_cycles() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_path(&["a", "b", "a"]);
    g.set_path(&["c", "d", "e", "c"]);
    g.set_path(&["f", "g", "g"]);
    g.ensure_node("h");

    assert_eq!(
        sorted_components(alg::find_cycles(&g)),
        vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "d".to_string(), "e".to_string()],
            vec!["g".to_string()]
        ]
    );
}

#[test]
fn preorder_returns_singleton_root() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.ensure_node("a");

    assert_eq!(alg::preorder(&g, &["a"]), vec!["a".to_string()]);
}

#[test]
fn preorder_visits_each_node_once() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_path(&["a", "b", "d", "e"]);
    g.set_path(&["a", "c", "d", "e"]);

    let mut nodes = alg::preorder(&g, &["a"]);
    nodes.sort();
    assert_eq!(nodes, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn preorder_preserves_parent_before_child_order() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_edge("a", "b");
    g.set_path(&["a", "c", "d"]);
    g.set_edge("c", "e");

    let nodes = alg::preorder(&g, &["a"]);
    assert!(nodes.iter().position(|v| v == "b") > nodes.iter().position(|v| v == "a"));
    assert!(nodes.iter().position(|v| v == "c") > nodes.iter().position(|v| v == "a"));
    assert!(nodes.iter().position(|v| v == "d") > nodes.iter().position(|v| v == "c"));
    assert!(nodes.iter().position(|v| v == "e") > nodes.iter().position(|v| v == "c"));
}

#[test]
fn preorder_accepts_multiple_roots() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_edge("a", "b");
    g.set_edge("c", "d");
    g.ensure_node("e");
    g.ensure_node("f");

    let mut nodes = alg::preorder(&g, &["a", "c", "e"]);
    nodes.sort();
    assert_eq!(nodes, vec!["a", "b", "c", "d", "e"]);
}

#[test]
#[should_panic(expected = "preorder root is not in the graph")]
fn preorder_panics_if_root_is_not_in_the_graph() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.ensure_node("a");

    let _ = alg::preorder(&g, &["b"]);
}

#[test]
fn postorder_returns_singleton_root() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.ensure_node("a");

    assert_eq!(alg::postorder(&g, &["a"]), vec!["a".to_string()]);
}

#[test]
fn postorder_visits_each_node_once() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_path(&["a", "b", "d", "e"]);
    g.set_path(&["a", "c", "d", "e"]);

    let mut nodes = alg::postorder(&g, &["a"]);
    nodes.sort();
    assert_eq!(nodes, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn postorder_preserves_child_before_parent_order() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_edge("a", "b");
    g.set_path(&["a", "c", "d"]);
    g.set_edge("c", "e");

    let nodes = alg::postorder(&g, &["a"]);
    assert!(nodes.iter().position(|v| v == "b") < nodes.iter().position(|v| v == "a"));
    assert!(nodes.iter().position(|v| v == "c") < nodes.iter().position(|v| v == "a"));
    assert!(nodes.iter().position(|v| v == "d") < nodes.iter().position(|v| v == "c"));
    assert!(nodes.iter().position(|v| v == "e") < nodes.iter().position(|v| v == "c"));
}

#[test]
fn postorder_accepts_multiple_roots() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_edge("a", "b");
    g.set_edge("c", "d");
    g.ensure_node("e");
    g.ensure_node("f");

    let mut nodes = alg::postorder(&g, &["a", "b", "c", "e"]);
    nodes.sort();
    assert_eq!(nodes, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn postorder_handles_multiple_connected_roots() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.set_edge("a", "b");
    g.set_edge("a", "c");
    g.set_edge("d", "c");

    let nodes = alg::postorder(&g, &["a", "d"]);
    assert!(nodes.iter().position(|v| v == "b") < nodes.iter().position(|v| v == "a"));
    assert!(nodes.iter().position(|v| v == "c") < nodes.iter().position(|v| v == "a"));
    assert!(nodes.iter().position(|v| v == "c") < nodes.iter().position(|v| v == "d"));
}

#[test]
#[should_panic(expected = "postorder root is not in the graph")]
fn postorder_panics_if_root_is_not_in_the_graph() {
    let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    g.ensure_node("a");

    let _ = alg::postorder(&g, &["b"]);
}

#[test]
fn preorder_and_postorder_handle_deep_successor_chains_with_small_stack() {
    const DEPTH: usize = 2048;
    let handle = std::thread::Builder::new()
        .name("graphlib-deep-alg-traversal".to_string())
        .stack_size(64 * 1024)
        .spawn(|| {
            let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
            for i in 0..DEPTH {
                g.set_edge(format!("n{i}"), format!("n{}", i + 1));
            }

            let leaf = format!("n{DEPTH}");
            let preorder = alg::preorder(&g, &["n0"]);
            assert_eq!(preorder.len(), DEPTH + 1);
            assert_eq!(preorder.first().map(String::as_str), Some("n0"));
            assert_eq!(preorder.last().map(String::as_str), Some(leaf.as_str()));

            let postorder = alg::postorder(&g, &["n0"]);
            assert_eq!(postorder.len(), DEPTH + 1);
            assert_eq!(postorder.first().map(String::as_str), Some(leaf.as_str()));
            assert_eq!(postorder.last().map(String::as_str), Some("n0"));
        })
        .expect("spawn graphlib deep traversal test");
    handle
        .join()
        .expect("graphlib deep traversal should finish without stack overflow");
}

#[test]
fn find_cycles_handles_deep_successor_chains_with_small_stack() {
    const DEPTH: usize = 2048;
    let handle = std::thread::Builder::new()
        .name("graphlib-deep-find-cycles".to_string())
        .stack_size(64 * 1024)
        .spawn(|| {
            let mut g: Graph<(), (), ()> = Graph::new(GraphOptions::default());
            for i in 0..DEPTH {
                g.set_edge(format!("n{i}"), format!("n{}", i + 1));
            }

            assert!(alg::find_cycles(&g).is_empty());
        })
        .expect("spawn graphlib deep find-cycles test");
    handle
        .join()
        .expect("graphlib find_cycles should finish without stack overflow");
}
