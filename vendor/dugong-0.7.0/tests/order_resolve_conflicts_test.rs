use dugong::graphlib::{Graph, GraphOptions};
use dugong::order::{BarycenterEntry, SortEntry, resolve_conflicts};

fn sort_by_first_vs(a: &SortEntry, b: &SortEntry) -> std::cmp::Ordering {
    a.vs[0].cmp(&b.vs[0])
}

#[test]
fn resolve_conflicts_returns_back_nodes_unchanged_when_no_constraints_exist() {
    let cg: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    let input = vec![
        BarycenterEntry {
            v: "a".to_string(),
            barycenter: Some(2.0),
            weight: Some(3.0),
        },
        BarycenterEntry {
            v: "b".to_string(),
            barycenter: Some(1.0),
            weight: Some(2.0),
        },
    ];

    let mut results = resolve_conflicts(&input, &cg);
    results.sort_by(sort_by_first_vs);
    assert_eq!(
        results,
        vec![
            SortEntry {
                vs: vec!["a".to_string()],
                i: 0,
                barycenter: Some(2.0),
                weight: Some(3.0)
            },
            SortEntry {
                vs: vec!["b".to_string()],
                i: 1,
                barycenter: Some(1.0),
                weight: Some(2.0)
            }
        ]
    );
}

#[test]
fn resolve_conflicts_returns_back_nodes_unchanged_when_no_conflicts_exist() {
    let mut cg: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    cg.set_edge("b", "a");
    let input = vec![
        BarycenterEntry {
            v: "a".to_string(),
            barycenter: Some(2.0),
            weight: Some(3.0),
        },
        BarycenterEntry {
            v: "b".to_string(),
            barycenter: Some(1.0),
            weight: Some(2.0),
        },
    ];

    let mut results = resolve_conflicts(&input, &cg);
    results.sort_by(sort_by_first_vs);
    assert_eq!(
        results,
        vec![
            SortEntry {
                vs: vec!["a".to_string()],
                i: 0,
                barycenter: Some(2.0),
                weight: Some(3.0)
            },
            SortEntry {
                vs: vec!["b".to_string()],
                i: 1,
                barycenter: Some(1.0),
                weight: Some(2.0)
            }
        ]
    );
}

#[test]
fn resolve_conflicts_coalesces_nodes_when_there_is_a_conflict() {
    let mut cg: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    cg.set_edge("a", "b");
    let input = vec![
        BarycenterEntry {
            v: "a".to_string(),
            barycenter: Some(2.0),
            weight: Some(3.0),
        },
        BarycenterEntry {
            v: "b".to_string(),
            barycenter: Some(1.0),
            weight: Some(2.0),
        },
    ];

    assert_eq!(
        resolve_conflicts(&input, &cg),
        vec![SortEntry {
            vs: vec!["a".to_string(), "b".to_string()],
            i: 0,
            barycenter: Some((3.0 * 2.0 + 2.0 * 1.0) / (3.0 + 2.0)),
            weight: Some(3.0 + 2.0)
        }]
    );
}

#[test]
fn resolve_conflicts_coalesces_nodes_when_there_is_a_conflict_2() {
    let mut cg: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    cg.set_edge("a", "b");
    cg.set_edge("b", "c");
    cg.set_edge("c", "d");
    let input = vec![
        BarycenterEntry {
            v: "a".to_string(),
            barycenter: Some(4.0),
            weight: Some(1.0),
        },
        BarycenterEntry {
            v: "b".to_string(),
            barycenter: Some(3.0),
            weight: Some(1.0),
        },
        BarycenterEntry {
            v: "c".to_string(),
            barycenter: Some(2.0),
            weight: Some(1.0),
        },
        BarycenterEntry {
            v: "d".to_string(),
            barycenter: Some(1.0),
            weight: Some(1.0),
        },
    ];

    assert_eq!(
        resolve_conflicts(&input, &cg),
        vec![SortEntry {
            vs: vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string()
            ],
            i: 0,
            barycenter: Some((4.0 + 3.0 + 2.0 + 1.0) / 4.0),
            weight: Some(4.0)
        }]
    );
}

#[test]
fn resolve_conflicts_works_with_multiple_constraints_for_the_same_target_1() {
    let mut cg: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    cg.set_edge("a", "c");
    cg.set_edge("b", "c");
    let input = vec![
        BarycenterEntry {
            v: "a".to_string(),
            barycenter: Some(4.0),
            weight: Some(1.0),
        },
        BarycenterEntry {
            v: "b".to_string(),
            barycenter: Some(3.0),
            weight: Some(1.0),
        },
        BarycenterEntry {
            v: "c".to_string(),
            barycenter: Some(2.0),
            weight: Some(1.0),
        },
    ];

    let results = resolve_conflicts(&input, &cg);
    assert_eq!(results.len(), 1);
    let merged = &results[0];
    let idx_c = merged.vs.iter().position(|v| v == "c").unwrap();
    let idx_a = merged.vs.iter().position(|v| v == "a").unwrap();
    let idx_b = merged.vs.iter().position(|v| v == "b").unwrap();
    assert!(idx_c > idx_a);
    assert!(idx_c > idx_b);
    assert_eq!(merged.i, 0);
    assert_eq!(merged.barycenter, Some((4.0 + 3.0 + 2.0) / 3.0));
    assert_eq!(merged.weight, Some(3.0));
}

#[test]
fn resolve_conflicts_works_with_multiple_constraints_for_the_same_target_2() {
    let mut cg: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    cg.set_edge("a", "c");
    cg.set_edge("a", "d");
    cg.set_edge("b", "c");
    cg.set_edge("c", "d");
    let input = vec![
        BarycenterEntry {
            v: "a".to_string(),
            barycenter: Some(4.0),
            weight: Some(1.0),
        },
        BarycenterEntry {
            v: "b".to_string(),
            barycenter: Some(3.0),
            weight: Some(1.0),
        },
        BarycenterEntry {
            v: "c".to_string(),
            barycenter: Some(2.0),
            weight: Some(1.0),
        },
        BarycenterEntry {
            v: "d".to_string(),
            barycenter: Some(1.0),
            weight: Some(1.0),
        },
    ];

    let results = resolve_conflicts(&input, &cg);
    assert_eq!(results.len(), 1);
    let merged = &results[0];
    let idx_a = merged.vs.iter().position(|v| v == "a").unwrap();
    let idx_b = merged.vs.iter().position(|v| v == "b").unwrap();
    let idx_c = merged.vs.iter().position(|v| v == "c").unwrap();
    let idx_d = merged.vs.iter().position(|v| v == "d").unwrap();
    assert!(idx_c > idx_a);
    assert!(idx_c > idx_b);
    assert!(idx_d > idx_c);
    assert_eq!(merged.i, 0);
    assert_eq!(merged.barycenter, Some((4.0 + 3.0 + 2.0 + 1.0) / 4.0));
    assert_eq!(merged.weight, Some(4.0));
}

#[test]
fn resolve_conflicts_does_nothing_to_a_node_lacking_both_a_barycenter_and_a_constraint() {
    let cg: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    let input = vec![
        BarycenterEntry {
            v: "a".to_string(),
            barycenter: None,
            weight: None,
        },
        BarycenterEntry {
            v: "b".to_string(),
            barycenter: Some(1.0),
            weight: Some(2.0),
        },
    ];

    let mut results = resolve_conflicts(&input, &cg);
    results.sort_by(sort_by_first_vs);
    assert_eq!(
        results,
        vec![
            SortEntry {
                vs: vec!["a".to_string()],
                i: 0,
                barycenter: None,
                weight: None
            },
            SortEntry {
                vs: vec!["b".to_string()],
                i: 1,
                barycenter: Some(1.0),
                weight: Some(2.0)
            }
        ]
    );
}

#[test]
fn resolve_conflicts_treats_a_node_without_a_barycenter_as_always_violating_constraints_1() {
    let mut cg: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    cg.set_edge("a", "b");
    let input = vec![
        BarycenterEntry {
            v: "a".to_string(),
            barycenter: None,
            weight: None,
        },
        BarycenterEntry {
            v: "b".to_string(),
            barycenter: Some(1.0),
            weight: Some(2.0),
        },
    ];

    assert_eq!(
        resolve_conflicts(&input, &cg),
        vec![SortEntry {
            vs: vec!["a".to_string(), "b".to_string()],
            i: 0,
            barycenter: Some(1.0),
            weight: Some(2.0)
        }]
    );
}

#[test]
fn resolve_conflicts_treats_a_node_without_a_barycenter_as_always_violating_constraints_2() {
    let mut cg: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    cg.set_edge("b", "a");
    let input = vec![
        BarycenterEntry {
            v: "a".to_string(),
            barycenter: None,
            weight: None,
        },
        BarycenterEntry {
            v: "b".to_string(),
            barycenter: Some(1.0),
            weight: Some(2.0),
        },
    ];

    assert_eq!(
        resolve_conflicts(&input, &cg),
        vec![SortEntry {
            vs: vec!["b".to_string(), "a".to_string()],
            i: 0,
            barycenter: Some(1.0),
            weight: Some(2.0)
        }]
    );
}

#[test]
fn resolve_conflicts_ignores_edges_not_related_to_entries() {
    let mut cg: Graph<(), (), ()> = Graph::new(GraphOptions::default());
    cg.set_edge("c", "d");
    let input = vec![
        BarycenterEntry {
            v: "a".to_string(),
            barycenter: Some(2.0),
            weight: Some(3.0),
        },
        BarycenterEntry {
            v: "b".to_string(),
            barycenter: Some(1.0),
            weight: Some(2.0),
        },
    ];

    let mut results = resolve_conflicts(&input, &cg);
    results.sort_by(sort_by_first_vs);
    assert_eq!(
        results,
        vec![
            SortEntry {
                vs: vec!["a".to_string()],
                i: 0,
                barycenter: Some(2.0),
                weight: Some(3.0)
            },
            SortEntry {
                vs: vec!["b".to_string()],
                i: 1,
                barycenter: Some(1.0),
                weight: Some(2.0)
            }
        ]
    );
}
