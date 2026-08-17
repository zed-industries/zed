use dugong::order::{SortEntry, SortResult, sort};

#[test]
fn sort_sorts_nodes_by_barycenter() {
    let input = vec![
        SortEntry {
            vs: vec!["a".to_string()],
            i: 0,
            barycenter: Some(2.0),
            weight: Some(3.0),
        },
        SortEntry {
            vs: vec!["b".to_string()],
            i: 1,
            barycenter: Some(1.0),
            weight: Some(2.0),
        },
    ];
    assert_eq!(
        sort(&input, false),
        SortResult {
            vs: vec!["b".to_string(), "a".to_string()],
            barycenter: Some((2.0 * 3.0 + 1.0 * 2.0) / (3.0 + 2.0)),
            weight: Some(3.0 + 2.0)
        }
    );
}

#[test]
fn sort_can_sort_super_nodes() {
    let input = vec![
        SortEntry {
            vs: vec!["a".to_string(), "c".to_string(), "d".to_string()],
            i: 0,
            barycenter: Some(2.0),
            weight: Some(3.0),
        },
        SortEntry {
            vs: vec!["b".to_string()],
            i: 1,
            barycenter: Some(1.0),
            weight: Some(2.0),
        },
    ];
    assert_eq!(
        sort(&input, false),
        SortResult {
            vs: vec![
                "b".to_string(),
                "a".to_string(),
                "c".to_string(),
                "d".to_string()
            ],
            barycenter: Some((2.0 * 3.0 + 1.0 * 2.0) / (3.0 + 2.0)),
            weight: Some(3.0 + 2.0)
        }
    );
}

#[test]
fn sort_biases_to_the_left_by_default() {
    let input = vec![
        SortEntry {
            vs: vec!["a".to_string()],
            i: 0,
            barycenter: Some(1.0),
            weight: Some(1.0),
        },
        SortEntry {
            vs: vec!["b".to_string()],
            i: 1,
            barycenter: Some(1.0),
            weight: Some(1.0),
        },
    ];
    assert_eq!(
        sort(&input, false),
        SortResult {
            vs: vec!["a".to_string(), "b".to_string()],
            barycenter: Some(1.0),
            weight: Some(2.0)
        }
    );
}

#[test]
fn sort_biases_to_the_right_if_bias_right_is_true() {
    let input = vec![
        SortEntry {
            vs: vec!["a".to_string()],
            i: 0,
            barycenter: Some(1.0),
            weight: Some(1.0),
        },
        SortEntry {
            vs: vec!["b".to_string()],
            i: 1,
            barycenter: Some(1.0),
            weight: Some(1.0),
        },
    ];
    assert_eq!(
        sort(&input, true),
        SortResult {
            vs: vec!["b".to_string(), "a".to_string()],
            barycenter: Some(1.0),
            weight: Some(2.0)
        }
    );
}

#[test]
fn sort_can_sort_nodes_without_a_barycenter() {
    let input = vec![
        SortEntry {
            vs: vec!["a".to_string()],
            i: 0,
            barycenter: Some(2.0),
            weight: Some(1.0),
        },
        SortEntry {
            vs: vec!["b".to_string()],
            i: 1,
            barycenter: Some(6.0),
            weight: Some(1.0),
        },
        SortEntry {
            vs: vec!["c".to_string()],
            i: 2,
            barycenter: None,
            weight: None,
        },
        SortEntry {
            vs: vec!["d".to_string()],
            i: 3,
            barycenter: Some(3.0),
            weight: Some(1.0),
        },
    ];
    assert_eq!(
        sort(&input, false),
        SortResult {
            vs: vec![
                "a".to_string(),
                "d".to_string(),
                "c".to_string(),
                "b".to_string()
            ],
            barycenter: Some((2.0 + 6.0 + 3.0) / 3.0),
            weight: Some(3.0)
        }
    );
}

#[test]
fn sort_can_handle_no_barycenters_for_any_nodes() {
    let input = vec![
        SortEntry {
            vs: vec!["a".to_string()],
            i: 0,
            barycenter: None,
            weight: None,
        },
        SortEntry {
            vs: vec!["b".to_string()],
            i: 3,
            barycenter: None,
            weight: None,
        },
        SortEntry {
            vs: vec!["c".to_string()],
            i: 2,
            barycenter: None,
            weight: None,
        },
        SortEntry {
            vs: vec!["d".to_string()],
            i: 1,
            barycenter: None,
            weight: None,
        },
    ];
    assert_eq!(
        sort(&input, false),
        SortResult {
            vs: vec![
                "a".to_string(),
                "d".to_string(),
                "c".to_string(),
                "b".to_string()
            ],
            barycenter: None,
            weight: None
        }
    );
}

#[test]
fn sort_can_handle_a_barycenter_of_0() {
    let input = vec![
        SortEntry {
            vs: vec!["a".to_string()],
            i: 0,
            barycenter: Some(0.0),
            weight: Some(1.0),
        },
        SortEntry {
            vs: vec!["b".to_string()],
            i: 3,
            barycenter: None,
            weight: None,
        },
        SortEntry {
            vs: vec!["c".to_string()],
            i: 2,
            barycenter: None,
            weight: None,
        },
        SortEntry {
            vs: vec!["d".to_string()],
            i: 1,
            barycenter: None,
            weight: None,
        },
    ];
    assert_eq!(
        sort(&input, false),
        SortResult {
            vs: vec![
                "a".to_string(),
                "d".to_string(),
                "c".to_string(),
                "b".to_string()
            ],
            barycenter: Some(0.0),
            weight: Some(1.0)
        }
    );
}
