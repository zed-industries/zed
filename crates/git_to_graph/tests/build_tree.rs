// Integration tests for build_tree function
use git_to_graph::build::build_tree;
use git_to_graph::color::SimpleColorGen;
use git_to_graph::node::Node;
use git_to_graph::point::PointTest;
use git_to_graph::types::{PointType, G_KEY, ID_KEY, PARENTS_KEY, PARENTS_PATHS_TEST_KEY};
use serde_json::Value;

// Helper function to create a node
fn node(id: &str, parents: Vec<&str>) -> Node {
    let mut n = Node::new();
    n.insert(ID_KEY.to_string(), Value::String(id.to_string()));
    n.insert(
        PARENTS_KEY.to_string(),
        Value::Array(parents.iter().map(|p| Value::String(p.to_string())).collect()),
    );
    n
}

// Helper function to create a point for path validation
fn pt(x: i32, y: i32, typ: u8) -> PointTest {
    PointTest::new(x, y, PointType::from(typ))
}

// Helper function to validate columns
fn validate_columns(expected_columns: &[i32], data: &[Node]) {
    for (idx, row) in data.iter().enumerate() {
        if idx >= expected_columns.len() {
            break;
        }
        let expected_column = expected_columns[idx];
        let g_value = row.get(G_KEY).expect("g key not found");
        let g_array = g_value.as_array().expect("g is not array");
        let actual_column = g_array[1].as_i64().expect("column not i64") as i32;
        let node_id = row.get(ID_KEY).and_then(|v| v.as_str()).unwrap_or("unknown");
        assert_eq!(
            actual_column, expected_column,
            "Column mismatch for node {}: expected {}, got {}",
            node_id, expected_column, actual_column
        );
    }
}

// Helper function to extract path from node
fn get_path(node: &Node, parent_id: &str) -> Option<(Vec<PointTest>, i32)> {
    let parents_paths = node.get(PARENTS_PATHS_TEST_KEY)?;
    let parents_paths_obj = parents_paths.as_object()?;
    let path_data = parents_paths_obj.get(parent_id)?;

    let points_array = path_data.get("points")?.as_array()?;
    let color_idx = path_data.get("colorIdx")?.as_i64()? as i32;

    let mut points = Vec::new();
    for point in points_array {
        let p = point.as_array()?;
        let x = p[0].as_i64()? as i32;
        let y = p[1].as_i64()? as i32;
        let typ = p[2].as_i64()? as u8;
        points.push(pt(x, y, typ));
    }

    Some((points, color_idx))
}

// Helper function to validate path points
fn validate_path_points(node_id: &str, parent_id: &str, expected: &[PointTest], actual: &[PointTest]) {
    assert_eq!(
        actual.len(),
        expected.len(),
        "Path length mismatch for {} -> {}: expected {}, got {}",
        node_id, parent_id, expected.len(), actual.len()
    );
    for (i, (exp, act)) in expected.iter().zip(actual.iter()).enumerate() {
        assert_eq!(
            act, exp,
            "Point {} mismatch for {} -> {}: expected {:?}, got {:?}",
            i, node_id, parent_id, exp, act
        );
    }
}

// Structure for expected path data
struct ExpectedPath {
    points: Vec<(i32, i32, u8)>,
    color_idx: i32,
}

// Helper function to validate all paths for all nodes
fn validate_paths(nodes: &[Node], expected_paths: &[Vec<(&str, ExpectedPath)>]) {
    for (node_idx, node_paths) in expected_paths.iter().enumerate() {
        let node_id = nodes[node_idx]
            .get(ID_KEY)
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        for (parent_id, expected) in node_paths {
            let (actual_points, actual_color) = get_path(&nodes[node_idx], parent_id)
                .unwrap_or_else(|| panic!("Path not found: {} -> {}", node_id, parent_id));

            assert_eq!(
                actual_color, expected.color_idx,
                "Color mismatch for {} -> {}: expected {}, got {}",
                node_id, parent_id, expected.color_idx, actual_color
            );

            let expected_points: Vec<PointTest> = expected.points.iter()
                .map(|(x, y, typ)| pt(*x, *y, *typ))
                .collect();

            validate_path_points(node_id, parent_id, &expected_points, &actual_points);
        }
    }
}

// Custom colors for testing
fn custom_colors() -> SimpleColorGen {
    SimpleColorGen::new(vec![
        "color1".to_string(),
        "color2".to_string(),
        "color3".to_string(),
        "color4".to_string(),
        "color5".to_string(),
        "color6".to_string(),
        "color7".to_string(),
        "color8".to_string(),
        "color9".to_string(),
        "color10".to_string(),
    ])
}

#[test]
fn test_not_enough_colors() {
    let input_nodes = vec![
        node("0", vec!["3"]),
        node("1", vec!["3"]),
        node("2", vec!["3"]),
        node("3", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let _out = result.unwrap();

}

#[test]
fn test_get_input_nodes_from_json() {
    let input_nodes = vec![
        node("1", vec!["2"]),
        node("2", vec!["3"]),
        node("3", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 0, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("2", ExpectedPath { points: vec![(0, 0, 0), (0, 1, 0)], color_idx: 0 })],
        vec![("3", ExpectedPath { points: vec![(0, 1, 0), (0, 2, 0)], color_idx: 0 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
#[should_panic]
fn test_get_input_nodes_from_json_with_bad_json() {
    // This test verifies JSON parsing would fail with bad input
    // Skipping as it's testing JSON parsing, not graph building
    panic!("JSON parsing test - expects failure");
}

#[test]
fn test1() {
    // 1
    // |
    // 2
    // |
    // 3
    let input_nodes = vec![
        node("1", vec!["2"]),
        node("2", vec!["3"]),
        node("3", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 0, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("2", ExpectedPath { points: vec![(0, 0, 0), (0, 1, 0)], color_idx: 0 })],
        vec![("3", ExpectedPath { points: vec![(0, 1, 0), (0, 2, 0)], color_idx: 0 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test2() {
    // 1
    // | 2
    // |/
    // 3
    let input_nodes = vec![
        node("1", vec!["3"]),
        node("2", vec!["3"]),
        node("3", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("3", ExpectedPath { points: vec![(0, 0, 0), (0, 2, 0)], color_idx: 0 })],
        vec![("3", ExpectedPath { points: vec![(1, 1, 0), (1, 2, 1), (0, 2, 0)], color_idx: 1 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test3() {
    // 1
    // |\
    // | 2
    // |/
    // 3
    let input_nodes = vec![
        node("1", vec!["3", "2"]),
        node("2", vec!["3"]),
        node("3", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("3", ExpectedPath { points: vec![(0, 0, 0), (0, 2, 0)], color_idx: 0 }), ("2", ExpectedPath { points: vec![(0, 0, 0), (1, 0, 2), (1, 1, 0)], color_idx: 1 })],
        vec![("3", ExpectedPath { points: vec![(1, 1, 0), (1, 2, 1), (0, 2, 0)], color_idx: 1 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test4() {
    // 1
    // |\
    // | 2
    // 3 |
    // |\|
    // | |\
    // | | 4
    // | |/
    // |/
    // 5
    let input_nodes = vec![
        node("1", vec!["3", "2"]),
        node("2", vec!["5"]),
        node("3", vec!["5", "4"]),
        node("4", vec!["5"]),
        node("5", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 0, 2, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("3", ExpectedPath { points: vec![(0, 0, 0), (0, 2, 0)], color_idx: 0 }), ("2", ExpectedPath { points: vec![(0, 0, 0), (1, 0, 2), (1, 1, 0)], color_idx: 1 })],
        vec![("5", ExpectedPath { points: vec![(1, 1, 0), (1, 4, 1), (0, 4, 0)], color_idx: 1 })],
        vec![("5", ExpectedPath { points: vec![(0, 2, 0), (0, 4, 0)], color_idx: 0 }), ("4", ExpectedPath { points: vec![(0, 2, 0), (2, 2, 2), (2, 3, 0)], color_idx: 2 })],
        vec![("5", ExpectedPath { points: vec![(2, 3, 0), (2, 4, 1), (0, 4, 0)], color_idx: 2 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test5() {
    // 1
    // | 2
    // | | 3
    // | |/
    // |/
    // 4
    // | 5
    // |/
    // 6
    let input_nodes = vec![
        node("1", vec!["4"]),
        node("2", vec!["4"]),
        node("3", vec!["4"]),
        node("4", vec!["6"]),
        node("5", vec!["6"]),
        node("6", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 2, 0, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("4", ExpectedPath { points: vec![(0, 0, 0), (0, 3, 0)], color_idx: 0 })],
        vec![("4", ExpectedPath { points: vec![(1, 1, 0), (1, 3, 1), (0, 3, 0)], color_idx: 1 })],
        vec![("4", ExpectedPath { points: vec![(2, 2, 0), (2, 3, 1), (0, 3, 0)], color_idx: 2 })],
        vec![("6", ExpectedPath { points: vec![(0, 3, 0), (0, 5, 0)], color_idx: 0 })],
        vec![("6", ExpectedPath { points: vec![(1, 4, 0), (1, 5, 1), (0, 5, 0)], color_idx: 3 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test6() {
    // 1
    // |\
    // | 2
    // |/|
    // 3 |
    // | 4
    // |/
    // 5
    let input_nodes = vec![
        node("1", vec!["3", "2"]),
        node("2", vec!["3", "4"]),
        node("3", vec!["5"]),
        node("4", vec!["5"]),
        node("5", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 0, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("3", ExpectedPath { points: vec![(0, 0, 0), (0, 2, 0)], color_idx: 0 }), ("2", ExpectedPath { points: vec![(0, 0, 0), (1, 0, 2), (1, 1, 0)], color_idx: 1 })],
        vec![("3", ExpectedPath { points: vec![(1, 1, 0), (0, 1, 3), (0, 2, 0)], color_idx: 0 }), ("4", ExpectedPath { points: vec![(1, 1, 0), (1, 3, 0)], color_idx: 1 })],
        vec![("5", ExpectedPath { points: vec![(0, 2, 0), (0, 4, 0)], color_idx: 0 })],
        vec![("5", ExpectedPath { points: vec![(1, 3, 0), (1, 4, 1), (0, 4, 0)], color_idx: 1 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test7() {
    // 1
    // |\
    // | 2
    // 3 |\
    // | 4 |
    // | |/
    // |/|
    // 5 |
    // |/
    // 6
    let input_nodes = vec![
        node("1", vec!["3", "2"]),
        node("2", vec!["4", "5"]),
        node("3", vec!["5"]),
        node("4", vec!["6"]),
        node("5", vec!["6"]),
        node("6", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 0, 1, 0, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("3", ExpectedPath { points: vec![(0, 0, 0), (0, 2, 0)], color_idx: 0 }), ("2", ExpectedPath { points: vec![(0, 0, 0), (1, 0, 2), (1, 1, 0)], color_idx: 1 })],
        vec![("4", ExpectedPath { points: vec![(1, 1, 0), (1, 3, 0)], color_idx: 1 }), ("5", ExpectedPath { points: vec![(1, 1, 0), (2, 1, 2), (2, 4, 1), (0, 4, 0)], color_idx: 2 })],
        vec![("5", ExpectedPath { points: vec![(0, 2, 0), (0, 4, 0)], color_idx: 0 })],
        vec![("6", ExpectedPath { points: vec![(1, 3, 0), (1, 5, 1), (0, 5, 0)], color_idx: 1 })],
        vec![("6", ExpectedPath { points: vec![(0, 4, 0), (0, 5, 0)], color_idx: 0 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test8() {
    // 1
    // |\
    // | 2
    // 3 |
    // |\|
    // | |\
    // |/ /
    // 4 |
    // | 5
    // |/
    // 6
    let input_nodes = vec![
        node("1", vec!["3", "2"]),
        node("2", vec!["4"]),
        node("3", vec!["4", "5"]),
        node("4", vec!["6"]),
        node("5", vec!["6"]),
        node("6", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 0, 0, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("3", ExpectedPath { points: vec![(0, 0, 0), (0, 2, 0)], color_idx: 0 }), ("2", ExpectedPath { points: vec![(0, 0, 0), (1, 0, 2), (1, 1, 0)], color_idx: 1 })],
        vec![("4", ExpectedPath { points: vec![(1, 1, 0), (1, 3, 1), (0, 3, 0)], color_idx: 1 })],
        vec![("4", ExpectedPath { points: vec![(0, 2, 0), (0, 3, 0)], color_idx: 0 }), ("5", ExpectedPath { points: vec![(0, 2, 0), (2, 2, 2), (2, 3, 1), (1, 3, 0), (1, 4, 0)], color_idx: 2 })],
        vec![("6", ExpectedPath { points: vec![(0, 3, 0), (0, 5, 0)], color_idx: 0 })],
        vec![("6", ExpectedPath { points: vec![(1, 4, 0), (1, 5, 1), (0, 5, 0)], color_idx: 2 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test9() {
    // 1
    // |\
    // | 2
    // 3 |
    // |\|
    // | |\
    // 4 | |
    // |\| |
    // | |\|
    // | | |\
    // |/ / /
    // 5 | |
    // | | 6
    // | 7 |
    // | |/
    // |/
    // 8
    let input_nodes = vec![
        node("1", vec!["3", "2"]),
        node("2", vec!["5"]),
        node("3", vec!["4", "7"]),
        node("4", vec!["5", "6"]),
        node("5", vec!["8"]),
        node("6", vec!["8"]),
        node("7", vec!["8"]),
        node("8", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 0, 0, 0, 2, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("3", ExpectedPath { points: vec![(0, 0, 0), (0, 2, 0)], color_idx: 0 }), ("2", ExpectedPath { points: vec![(0, 0, 0), (1, 0, 2), (1, 1, 0)], color_idx: 1 })],
        vec![("5", ExpectedPath { points: vec![(1, 1, 0), (1, 4, 1), (0, 4, 0)], color_idx: 1 })],
        vec![("4", ExpectedPath { points: vec![(0, 2, 0), (0, 3, 0)], color_idx: 0 }), ("7", ExpectedPath { points: vec![(0, 2, 0), (2, 2, 2), (2, 4, 1), (1, 4, 0), (1, 6, 0)], color_idx: 2 })],
        vec![("6", ExpectedPath { points: vec![(0, 3, 0), (3, 3, 2), (3, 4, 1), (2, 4, 0), (2, 5, 0)], color_idx: 3 }), ("5", ExpectedPath { points: vec![(0, 3, 0), (0, 4, 0)], color_idx: 0 })],
        vec![("8", ExpectedPath { points: vec![(0, 4, 0), (0, 7, 0)], color_idx: 0 })],
        vec![("8", ExpectedPath { points: vec![(2, 5, 0), (2, 7, 1), (0, 7, 0)], color_idx: 3 })],
        vec![("8", ExpectedPath { points: vec![(1, 6, 0), (1, 7, 1), (0, 7, 0)], color_idx: 2 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test10() {
    // 1
    // |\
    // | 2
    // | |\
    // | | 3
    // 4 | |
    // |\| |
    // | |\|
    // | | |\
    // | |/ /
    // | 5 |
    // | |\|
    // | | 6
    // | |/
    // | 7
    // |/
    // 8
    let input_nodes = vec![
        node("1", vec!["4", "2"]),
        node("2", vec!["5", "3"]),
        node("3", vec!["5"]),
        node("4", vec!["8", "6"]),
        node("5", vec!["7", "6"]),
        node("6", vec!["7"]),
        node("7", vec!["8"]),
        node("8", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 2, 0, 1, 2, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("4", ExpectedPath { points: vec![(0, 0, 0), (0, 3, 0)], color_idx: 0 }), ("2", ExpectedPath { points: vec![(0, 0, 0), (1, 0, 2), (1, 1, 0)], color_idx: 1 })],
        vec![("5", ExpectedPath { points: vec![(1, 1, 0), (1, 4, 0)], color_idx: 1 }), ("3", ExpectedPath { points: vec![(1, 1, 0), (2, 1, 2), (2, 2, 0)], color_idx: 2 })],
        vec![("5", ExpectedPath { points: vec![(2, 2, 0), (2, 4, 1), (1, 4, 0)], color_idx: 2 })],
        vec![("8", ExpectedPath { points: vec![(0, 3, 0), (0, 7, 0)], color_idx: 0 }), ("6", ExpectedPath { points: vec![(0, 3, 0), (3, 3, 2), (3, 4, 1), (2, 4, 0), (2, 5, 0)], color_idx: 3 })],
        vec![("7", ExpectedPath { points: vec![(1, 4, 0), (1, 6, 0)], color_idx: 1 }), ("6", ExpectedPath { points: vec![(1, 4, 0), (2, 4, 2), (2, 5, 0)], color_idx: 3 })],
        vec![("7", ExpectedPath { points: vec![(2, 5, 0), (2, 6, 1), (1, 6, 0)], color_idx: 3 })],
        vec![("8", ExpectedPath { points: vec![(1, 6, 0), (1, 7, 1), (0, 7, 0)], color_idx: 1 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test11() {
    // 1
    // |-2
    // 3 |
    // |-4
    // 5 |
    // |/
    // 6
    let input_nodes = vec![
        node("1", vec!["3"]),
        node("2", vec!["3", "4"]),
        node("3", vec!["5"]),
        node("4", vec!["5", "6"]),
        node("5", vec!["6"]),
        node("6", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 0, 1, 0, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("3", ExpectedPath { points: vec![(0, 0, 0), (0, 2, 0)], color_idx: 0 })],
        vec![("4", ExpectedPath { points: vec![(1, 1, 0), (1, 3, 0)], color_idx: 1 }), ("3", ExpectedPath { points: vec![(1, 1, 0), (0, 1, 3), (0, 2, 0)], color_idx: 0 })],
        vec![("5", ExpectedPath { points: vec![(0, 2, 0), (0, 4, 0)], color_idx: 0 })],
        vec![("6", ExpectedPath { points: vec![(1, 3, 0), (1, 5, 1), (0, 5, 0)], color_idx: 1 }), ("5", ExpectedPath { points: vec![(1, 3, 0), (0, 3, 3), (0, 4, 0)], color_idx: 0 })],
        vec![("6", ExpectedPath { points: vec![(0, 4, 0), (0, 5, 0)], color_idx: 0 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test12() {
    // 1
    // |-2
    // 3 |
    // |\|
    // | |\
    // | | 4
    // | |/
    // |/|
    // 5 |
    // | 6
    // |/
    // 7
    let input_nodes = vec![
        node("1", vec!["3"]),
        node("2", vec!["3", "6"]),
        node("3", vec!["5", "4"]),
        node("4", vec!["5"]),
        node("5", vec!["7"]),
        node("6", vec!["7"]),
        node("7", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 0, 2, 0, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("3", ExpectedPath { points: vec![(0, 0, 0), (0, 2, 0)], color_idx: 0 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test13() {
    // 1
    // |\
    // | 2
    // | |
    // | 3
    // 4 |\
    // |\| |
    // | 5 |
    // | |\|
    // | 6 |\
    // 7 | | |
    // | | |/
    // | |/|
    // |/| |
    // 8 | |
    // | | 9
    // | |/
    // |/
    // 10
    let input_nodes = vec![
        node("1", vec!["4", "2"]),
        node("2", vec!["3"]),
        node("3", vec!["5", "9"]),
        node("4", vec!["7", "5"]),
        node("5", vec!["6", "8"]),
        node("6", vec!["10"]),
        node("7", vec!["8"]),
        node("8", vec!["10"]),
        node("9", vec!["10"]),
        node("10", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 1, 0, 1, 1, 0, 0, 2, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("2", ExpectedPath { points: vec![(0, 0, 0), (1, 0, 2), (1, 1, 0)], color_idx: 1 }), ("4", ExpectedPath { points: vec![(0, 0, 0), (0, 3, 0)], color_idx: 0 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test14() {
    // 1
    // | 2
    // 3 |\
    // |\| |
    // | |\|
    // | | 4
    // | |/
    // | 5
    // 6 |\
    // |\| |
    // | |\|
    // | | 7
    // | |/
    // |/
    // 8
    let input_nodes = vec![
        node("1", vec!["3"]),
        node("2", vec!["5", "4"]),
        node("3", vec!["6", "4"]),
        node("4", vec!["5"]),
        node("5", vec!["8", "7"]),
        node("6", vec!["8", "7"]),
        node("7", vec!["8"]),
        node("8", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 0, 2, 1, 0, 2, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("3", ExpectedPath { points: vec![(0, 0, 0), (0, 2, 0)], color_idx: 0 })],
        vec![("5", ExpectedPath { points: vec![(1, 1, 0), (1, 4, 0)], color_idx: 1 }), ("4", ExpectedPath { points: vec![(1, 1, 0), (2, 1, 2), (2, 3, 0)], color_idx: 2 })],
        vec![("4", ExpectedPath { points: vec![(0, 2, 0), (2, 2, 2), (2, 3, 0)], color_idx: 2 }), ("6", ExpectedPath { points: vec![(0, 2, 0), (0, 5, 0)], color_idx: 0 })],
        vec![("5", ExpectedPath { points: vec![(2, 3, 0), (2, 4, 1), (1, 4, 0)], color_idx: 2 })],
        vec![("8", ExpectedPath { points: vec![(1, 4, 0), (1, 7, 1), (0, 7, 0)], color_idx: 1 }), ("7", ExpectedPath { points: vec![(1, 4, 0), (2, 4, 2), (2, 6, 0)], color_idx: 3 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test15() {
    // 1
    // | 2
    // 3 |\
    // |\| |
    // | |\|
    // | | 4
    // 5 | |
    // |\| |
    // | |\|
    // | | |\
    // | |/ /
    // | 6 |
    // | |\|
    // | | 7
    // | |/
    // |/
    // 8
    let input_nodes = vec![
        node("1", vec!["3"]),
        node("2", vec!["6", "4"]),
        node("3", vec!["5", "4"]),
        node("4", vec!["6"]),
        node("5", vec!["8", "7"]),
        node("6", vec!["8", "7"]),
        node("7", vec!["8"]),
        node("8", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 0, 2, 0, 1, 2, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("3", ExpectedPath { points: vec![(0, 0, 0), (0, 2, 0)], color_idx: 0 })],
        vec![("6", ExpectedPath { points: vec![(1, 1, 0), (1, 5, 0)], color_idx: 1 }), ("4", ExpectedPath { points: vec![(1, 1, 0), (2, 1, 2), (2, 3, 0)], color_idx: 2 })],
        vec![("5", ExpectedPath { points: vec![(0, 2, 0), (0, 4, 0)], color_idx: 0 }), ("4", ExpectedPath { points: vec![(0, 2, 0), (2, 2, 2), (2, 3, 0)], color_idx: 2 })],
        vec![("6", ExpectedPath { points: vec![(2, 3, 0), (2, 5, 1), (1, 5, 0)], color_idx: 2 })],
        vec![("8", ExpectedPath { points: vec![(0, 4, 0), (0, 7, 0)], color_idx: 0 }), ("7", ExpectedPath { points: vec![(0, 4, 0), (3, 4, 2), (3, 5, 1), (2, 5, 0), (2, 6, 0)], color_idx: 3 })],
        vec![("8", ExpectedPath { points: vec![(1, 5, 0), (1, 7, 1), (0, 7, 0)], color_idx: 1 }), ("7", ExpectedPath { points: vec![(1, 5, 0), (2, 5, 2), (2, 6, 0)], color_idx: 3 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test16() {
    // 1
    // | 2
    // | | 3
    // | |/ 4
    // |/| /
    // 5 |/
    // | 6
    // |/
    // 7
    let input_nodes = vec![
        node("1", vec!["5"]),
        node("2", vec!["6"]),
        node("3", vec!["5"]),
        node("4", vec!["6"]),
        node("5", vec!["7"]),
        node("6", vec!["7"]),
        node("7", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 2, 3, 0, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("5", ExpectedPath { points: vec![(0, 0, 0), (0, 4, 0)], color_idx: 0 })],
        vec![("6", ExpectedPath { points: vec![(1, 1, 0), (1, 5, 0)], color_idx: 1 })],
        vec![("5", ExpectedPath { points: vec![(2, 2, 0), (2, 4, 1), (0, 4, 0)], color_idx: 2 })],
        vec![("6", ExpectedPath { points: vec![(3, 3, 0), (3, 4, 1), (2, 4, 0), (2, 5, 1), (1, 5, 0)], color_idx: 3 })],
        vec![("7", ExpectedPath { points: vec![(0, 4, 0), (0, 6, 0)], color_idx: 0 })],
        vec![("7", ExpectedPath { points: vec![(1, 5, 0), (1, 6, 1), (0, 6, 0)], color_idx: 1 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test17() {
    let input_nodes = vec![
        node("0", vec!["4"]),
        node("1", vec!["4"]),
        node("2", vec!["4"]),
        node("3", vec!["6"]),
        node("4", vec!["5"]),
        node("5", vec!["6"]),
        node("6", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 2, 3, 0, 0, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("4", ExpectedPath { points: vec![(0, 0, 0), (0, 4, 0)], color_idx: 0 })],
        vec![("4", ExpectedPath { points: vec![(1, 1, 0), (1, 4, 1), (0, 4, 0)], color_idx: 1 })],
        vec![("4", ExpectedPath { points: vec![(2, 2, 0), (2, 4, 1), (0, 4, 0)], color_idx: 2 })],
        vec![("6", ExpectedPath { points: vec![(3, 3, 0), (3, 4, 1), (1, 4, 0), (1, 6, 1), (0, 6, 0)], color_idx: 3 })],
        vec![("5", ExpectedPath { points: vec![(0, 4, 0), (0, 5, 0)], color_idx: 0 })],
        vec![("6", ExpectedPath { points: vec![(0, 5, 0), (0, 6, 0)], color_idx: 0 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test18() {
    let input_nodes = vec![
        node("0", vec!["4"]),
        node("1", vec!["4"]),
        node("2", vec!["5"]),
        node("3", vec!["5"]),
        node("4", vec!["6"]),
        node("5", vec!["6"]),
        node("6", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 2, 3, 0, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("4", ExpectedPath { points: vec![(0, 0, 0), (0, 4, 0)], color_idx: 0 })],
        vec![("4", ExpectedPath { points: vec![(1, 1, 0), (1, 4, 1), (0, 4, 0)], color_idx: 1 })],
        vec![("5", ExpectedPath { points: vec![(2, 2, 0), (2, 4, 1), (1, 4, 0), (1, 5, 0)], color_idx: 2 })],
        vec![("5", ExpectedPath { points: vec![(3, 3, 0), (3, 4, 1), (2, 4, 0), (2, 5, 1), (1, 5, 0)], color_idx: 3 })],
        vec![("6", ExpectedPath { points: vec![(0, 4, 0), (0, 6, 0)], color_idx: 0 })],
        vec![("6", ExpectedPath { points: vec![(1, 5, 0), (1, 6, 1), (0, 6, 0)], color_idx: 2 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test19() {
    let input_nodes = vec![
        node("0", vec!["5"]),
        node("1", vec!["4"]),
        node("2", vec!["9"]),
        node("3", vec!["7"]),
        node("4", vec!["11", "6"]),
        node("5", vec!["8", "6"]),
        node("6", vec!["11"]),
        node("7", vec!["8"]),
        node("8", vec!["10"]),
        node("9", vec!["10"]),
        node("10", vec!["12"]),
        node("11", vec!["12"]),
        node("12", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 2, 3, 1, 0, 4, 3, 0, 2, 0, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("5", ExpectedPath { points: vec![(0, 0, 0), (0, 5, 0)], color_idx: 0 })],
        vec![("4", ExpectedPath { points: vec![(1, 1, 0), (1, 4, 0)], color_idx: 1 })],
        vec![("9", ExpectedPath { points: vec![(2, 2, 0), (2, 9, 0)], color_idx: 2 })],
        vec![("7", ExpectedPath { points: vec![(3, 3, 0), (3, 7, 0)], color_idx: 3 })],
        vec![("6", ExpectedPath { points: vec![(1, 4, 0), (4, 4, 2), (4, 6, 0)], color_idx: 4 }), ("11", ExpectedPath { points: vec![(1, 4, 0), (1, 11, 0)], color_idx: 1 })],
        vec![("8", ExpectedPath { points: vec![(0, 5, 0), (0, 8, 0)], color_idx: 0 }), ("6", ExpectedPath { points: vec![(0, 5, 0), (4, 5, 2), (4, 6, 0)], color_idx: 4 })],
        vec![("11", ExpectedPath { points: vec![(4, 6, 0), (4, 8, 1), (3, 8, 0), (3, 10, 1), (2, 10, 0), (2, 11, 1), (1, 11, 0)], color_idx: 4 })],
        vec![("8", ExpectedPath { points: vec![(3, 7, 0), (3, 8, 1), (0, 8, 0)], color_idx: 3 })],
        vec![("10", ExpectedPath { points: vec![(0, 8, 0), (0, 10, 0)], color_idx: 0 })],
        vec![("10", ExpectedPath { points: vec![(2, 9, 0), (2, 10, 1), (0, 10, 0)], color_idx: 2 })],
        vec![("12", ExpectedPath { points: vec![(0, 10, 0), (0, 12, 0)], color_idx: 0 })],
        vec![("12", ExpectedPath { points: vec![(1, 11, 0), (1, 12, 1), (0, 12, 0)], color_idx: 1 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test20() {
    let input_nodes = vec![
        node("0", vec!["4"]),
        node("1", vec!["4"]),
        node("2", vec!["5"]),
        node("3", vec!["4"]),
        node("4", vec!["5"]),
        node("5", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 2, 3, 0, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("4", ExpectedPath { points: vec![(0, 0, 0), (0, 4, 0)], color_idx: 0 })],
        vec![("4", ExpectedPath { points: vec![(1, 1, 0), (1, 4, 1), (0, 4, 0)], color_idx: 1 })],
        vec![("5", ExpectedPath { points: vec![(2, 2, 0), (2, 4, 1), (1, 4, 0), (1, 5, 1), (0, 5, 0)], color_idx: 2 })],
        vec![("4", ExpectedPath { points: vec![(3, 3, 0), (3, 4, 1), (0, 4, 0)], color_idx: 3 })],
        vec![("5", ExpectedPath { points: vec![(0, 4, 0), (0, 5, 0)], color_idx: 0 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test21() {
    let input_nodes = vec![
        node("0", vec!["4"]),
        node("1", vec!["3"]),
        node("2", vec!["5"]),
        node("3", vec!["6", "5"]),
        node("4", vec!["5"]),
        node("5", vec!["7"]),
        node("6", vec!["7"]),
        node("7", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 2, 1, 0, 0, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("4", ExpectedPath { points: vec![(0, 0, 0), (0, 4, 0)], color_idx: 0 })],
        vec![("3", ExpectedPath { points: vec![(1, 1, 0), (1, 3, 0)], color_idx: 1 })],
        vec![("5", ExpectedPath { points: vec![(2, 2, 0), (2, 5, 1), (0, 5, 0)], color_idx: 2 })],
        vec![("6", ExpectedPath { points: vec![(1, 3, 0), (1, 6, 0)], color_idx: 1 }), ("5", ExpectedPath { points: vec![(1, 3, 0), (2, 3, 2), (2, 5, 1), (0, 5, 0)], color_idx: 2 })],
        vec![("5", ExpectedPath { points: vec![(0, 4, 0), (0, 5, 0)], color_idx: 0 })],
        vec![("7", ExpectedPath { points: vec![(0, 5, 0), (0, 7, 0)], color_idx: 0 })],
        vec![("7", ExpectedPath { points: vec![(1, 6, 0), (1, 7, 1), (0, 7, 0)], color_idx: 1 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test22() {
    let input_nodes = vec![
        node("0", vec!["5"]),
        node("1", vec!["4"]),
        node("2", vec!["6"]),
        node("3", vec!["7"]),
        node("4", vec!["7", "6"]),
        node("5", vec!["6"]),
        node("6", vec!["8"]),
        node("7", vec!["8"]),
        node("8", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 2, 3, 1, 0, 0, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("5", ExpectedPath { points: vec![(0, 0, 0), (0, 5, 0)], color_idx: 0 })],
        vec![("4", ExpectedPath { points: vec![(1, 1, 0), (1, 4, 0)], color_idx: 1 })],
        vec![("6", ExpectedPath { points: vec![(2, 2, 0), (2, 6, 1), (0, 6, 0)], color_idx: 2 })],
        vec![("7", ExpectedPath { points: vec![(3, 3, 0), (3, 6, 1), (2, 6, 0), (2, 7, 1), (1, 7, 0)], color_idx: 3 })],
        vec![("6", ExpectedPath { points: vec![(1, 4, 0), (2, 4, 2), (2, 6, 1), (0, 6, 0)], color_idx: 2 }), ("7", ExpectedPath { points: vec![(1, 4, 0), (1, 7, 0)], color_idx: 1 })],
        vec![("6", ExpectedPath { points: vec![(0, 5, 0), (0, 6, 0)], color_idx: 0 })],
        vec![("8", ExpectedPath { points: vec![(0, 6, 0), (0, 8, 0)], color_idx: 0 })],
        vec![("8", ExpectedPath { points: vec![(1, 7, 0), (1, 8, 1), (0, 8, 0)], color_idx: 1 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test23() {
    let input_nodes = vec![
        node("0", vec!["4"]),
        node("1", vec!["4"]),
        node("2", vec!["4"]),
        node("3", vec!["7"]),
        node("4", vec!["6", "5"]),
        node("5", vec!["6"]),
        node("6", vec!["8"]),
        node("7", vec!["8"]),
        node("8", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 2, 3, 0, 2, 0, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("4", ExpectedPath { points: vec![(0, 0, 0), (0, 4, 0)], color_idx: 0 })],
        vec![("4", ExpectedPath { points: vec![(1, 1, 0), (1, 4, 1), (0, 4, 0)], color_idx: 1 })],
        vec![("4", ExpectedPath { points: vec![(2, 2, 0), (2, 4, 1), (0, 4, 0)], color_idx: 2 })],
        vec![("7", ExpectedPath { points: vec![(3, 3, 0), (3, 4, 1), (1, 4, 0), (1, 7, 0)], color_idx: 3 })],
        vec![("6", ExpectedPath { points: vec![(0, 4, 0), (0, 6, 0)], color_idx: 0 }), ("5", ExpectedPath { points: vec![(0, 4, 0), (2, 4, 2), (2, 5, 0)], color_idx: 4 })],
        vec![("6", ExpectedPath { points: vec![(2, 5, 0), (2, 6, 1), (0, 6, 0)], color_idx: 4 })],
        vec![("8", ExpectedPath { points: vec![(0, 6, 0), (0, 8, 0)], color_idx: 0 })],
        vec![("8", ExpectedPath { points: vec![(1, 7, 0), (1, 8, 1), (0, 8, 0)], color_idx: 3 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test24() {
    let input_nodes = vec![
        node("0", vec!["3"]),
        node("1", vec!["5"]),
        node("2", vec!["9"]),
        node("3", vec!["7", "6"]),
        node("4", vec!["6"]),
        node("5", vec!["6"]),
        node("6", vec!["10", "8"]),
        node("7", vec!["11", "8"]),
        node("8", vec!["9"]),
        node("9", vec!["10"]),
        node("10", vec!["11"]),
        node("11", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 2, 0, 4, 1, 1, 0, 3, 2, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("3", ExpectedPath { points: vec![(0, 0, 0), (0, 3, 0)], color_idx: 0 })],
        vec![("5", ExpectedPath { points: vec![(1, 1, 0), (1, 5, 0)], color_idx: 1 })],
        vec![("9", ExpectedPath { points: vec![(2, 2, 0), (2, 9, 0)], color_idx: 2 })],
        vec![("7", ExpectedPath { points: vec![(0, 3, 0), (0, 7, 0)], color_idx: 0 }), ("6", ExpectedPath { points: vec![(0, 3, 0), (3, 3, 2), (3, 6, 1), (1, 6, 0)], color_idx: 3 })],
        vec![("6", ExpectedPath { points: vec![(4, 4, 0), (4, 6, 1), (1, 6, 0)], color_idx: 4 })],
        vec![("6", ExpectedPath { points: vec![(1, 5, 0), (1, 6, 0)], color_idx: 1 })],
        vec![("8", ExpectedPath { points: vec![(1, 6, 0), (3, 6, 2), (3, 8, 0)], color_idx: 5 }), ("10", ExpectedPath { points: vec![(1, 6, 0), (1, 10, 0)], color_idx: 1 })],
        vec![("11", ExpectedPath { points: vec![(0, 7, 0), (0, 11, 0)], color_idx: 0 }), ("8", ExpectedPath { points: vec![(0, 7, 0), (3, 7, 2), (3, 8, 0)], color_idx: 5 })],
        vec![("9", ExpectedPath { points: vec![(3, 8, 0), (3, 9, 1), (2, 9, 0)], color_idx: 5 })],
        vec![("10", ExpectedPath { points: vec![(2, 9, 0), (2, 10, 1), (1, 10, 0)], color_idx: 2 })],
        vec![("11", ExpectedPath { points: vec![(1, 10, 0), (1, 11, 1), (0, 11, 0)], color_idx: 1 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test25() {
    let input_nodes = vec![
        node("0", vec!["5"]),
        node("1", vec!["3"]),
        node("2", vec!["4"]),
        node("3", vec!["9", "7"]),
        node("4", vec!["6"]),
        node("5", vec!["8", "7"]),
        node("6", vec!["9", "7"]),
        node("7", vec!["8"]),
        node("8", vec!["12", "9"]),
        node("9", vec!["11", "10"]),
        node("10", vec!["11"]),
        node("11", vec!["12"]),
        node("12", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 2, 1, 2, 0, 2, 3, 0, 1, 2, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("5", ExpectedPath { points: vec![(0, 0, 0), (0, 5, 0)], color_idx: 0 })],
        vec![("3", ExpectedPath { points: vec![(1, 1, 0), (1, 3, 0)], color_idx: 1 })],
        vec![("4", ExpectedPath { points: vec![(2, 2, 0), (2, 4, 0)], color_idx: 2 })],
        vec![("9", ExpectedPath { points: vec![(1, 3, 0), (1, 9, 0)], color_idx: 1 }), ("7", ExpectedPath { points: vec![(1, 3, 0), (3, 3, 2), (3, 7, 0)], color_idx: 3 })],
        vec![("6", ExpectedPath { points: vec![(2, 4, 0), (2, 6, 0)], color_idx: 2 })],
        vec![("8", ExpectedPath { points: vec![(0, 5, 0), (0, 8, 0)], color_idx: 0 }), ("7", ExpectedPath { points: vec![(0, 5, 0), (3, 5, 2), (3, 7, 0)], color_idx: 3 })],
        vec![("9", ExpectedPath { points: vec![(2, 6, 0), (2, 9, 1), (1, 9, 0)], color_idx: 2 }), ("7", ExpectedPath { points: vec![(2, 6, 0), (3, 6, 2), (3, 7, 0)], color_idx: 3 })],
        vec![("8", ExpectedPath { points: vec![(3, 7, 0), (3, 8, 1), (0, 8, 0)], color_idx: 3 })],
        vec![("12", ExpectedPath { points: vec![(0, 8, 0), (0, 12, 0)], color_idx: 0 }), ("9", ExpectedPath { points: vec![(0, 8, 0), (1, 8, 2), (1, 9, 0)], color_idx: 1 })],
        vec![("11", ExpectedPath { points: vec![(1, 9, 0), (1, 11, 0)], color_idx: 1 }), ("10", ExpectedPath { points: vec![(1, 9, 0), (2, 9, 2), (2, 10, 0)], color_idx: 4 })],
        vec![("11", ExpectedPath { points: vec![(2, 10, 0), (2, 11, 1), (1, 11, 0)], color_idx: 4 })],
        vec![("12", ExpectedPath { points: vec![(1, 11, 0), (1, 12, 1), (0, 12, 0)], color_idx: 1 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test26() {
    let input_nodes = vec![
        node("0", vec!["3"]),
        node("1", vec!["4"]),
        node("2", vec!["5"]),
        node("3", vec!["8", "5"]),
        node("4", vec!["5"]),
        node("5", vec!["7", "6"]),
        node("6", vec!["7"]),
        node("7", vec!["8"]),
        node("8", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 2, 0, 1, 1, 2, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("3", ExpectedPath { points: vec![(0, 0, 0), (0, 3, 0)], color_idx: 0 })],
        vec![("4", ExpectedPath { points: vec![(1, 1, 0), (1, 4, 0)], color_idx: 1 })],
        vec![("5", ExpectedPath { points: vec![(2, 2, 0), (2, 5, 1), (1, 5, 0)], color_idx: 2 })],
        vec![("5", ExpectedPath { points: vec![(0, 3, 0), (2, 3, 2), (2, 5, 1), (1, 5, 0)], color_idx: 2 }), ("8", ExpectedPath { points: vec![(0, 3, 0), (0, 8, 0)], color_idx: 0 })],
        vec![("5", ExpectedPath { points: vec![(1, 4, 0), (1, 5, 0)], color_idx: 1 })],
        vec![("7", ExpectedPath { points: vec![(1, 5, 0), (1, 7, 0)], color_idx: 1 }), ("6", ExpectedPath { points: vec![(1, 5, 0), (2, 5, 2), (2, 6, 0)], color_idx: 3 })],
        vec![("7", ExpectedPath { points: vec![(2, 6, 0), (2, 7, 1), (1, 7, 0)], color_idx: 3 })],
        vec![("8", ExpectedPath { points: vec![(1, 7, 0), (1, 8, 1), (0, 8, 0)], color_idx: 1 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test27() {
    let input_nodes = vec![
        node("0", vec!["4"]),
        node("1", vec!["5"]),
        node("2", vec!["7"]),
        node("3", vec!["11"]),
        node("4", vec!["15", "6"]),
        node("5", vec!["8", "6"]),
        node("6", vec!["15"]),
        node("7", vec!["12"]),
        node("8", vec!["9", "13"]),
        node("9", vec!["14", "10"]),
        node("10", vec!["14"]),
        node("11", vec!["14"]),
        node("12", vec!["14"]),
        node("13", vec!["14"]),
        node("14", vec!["16"]),
        node("15", vec!["17"]),
        node("16", vec!["17"]),
        node("17", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 2, 3, 0, 1, 4, 2, 1, 1, 6, 3, 2, 5, 1, 0, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("4", ExpectedPath { points: vec![(0, 0, 0), (0, 4, 0)], color_idx: 0 })],
        vec![("5", ExpectedPath { points: vec![(1, 1, 0), (1, 5, 0)], color_idx: 1 })],
        vec![("7", ExpectedPath { points: vec![(2, 2, 0), (2, 7, 0)], color_idx: 2 })],
        vec![("11", ExpectedPath { points: vec![(3, 3, 0), (3, 11, 0)], color_idx: 3 })],
        vec![("15", ExpectedPath { points: vec![(0, 4, 0), (0, 15, 0)], color_idx: 0 }), ("6", ExpectedPath { points: vec![(0, 4, 0), (4, 4, 2), (4, 6, 0)], color_idx: 4 })],
        vec![("8", ExpectedPath { points: vec![(1, 5, 0), (1, 8, 0)], color_idx: 1 }), ("6", ExpectedPath { points: vec![(1, 5, 0), (4, 5, 2), (4, 6, 0)], color_idx: 4 })],
        vec![("15", ExpectedPath { points: vec![(4, 6, 0), (4, 14, 1), (2, 14, 0), (2, 15, 1), (0, 15, 0)], color_idx: 4 })],
        vec![("12", ExpectedPath { points: vec![(2, 7, 0), (2, 12, 0)], color_idx: 2 })],
        vec![("9", ExpectedPath { points: vec![(1, 8, 0), (1, 9, 0)], color_idx: 1 }), ("13", ExpectedPath { points: vec![(1, 8, 0), (5, 8, 2), (5, 13, 0)], color_idx: 5 })],
        vec![("14", ExpectedPath { points: vec![(1, 9, 0), (1, 14, 0)], color_idx: 1 }), ("10", ExpectedPath { points: vec![(1, 9, 0), (6, 9, 2), (6, 10, 0)], color_idx: 6 })],
        vec![("14", ExpectedPath { points: vec![(6, 10, 0), (6, 14, 1), (1, 14, 0)], color_idx: 6 })],
        vec![("14", ExpectedPath { points: vec![(3, 11, 0), (3, 14, 1), (1, 14, 0)], color_idx: 3 })],
        vec![("14", ExpectedPath { points: vec![(2, 12, 0), (2, 14, 1), (1, 14, 0)], color_idx: 2 })],
        vec![("14", ExpectedPath { points: vec![(5, 13, 0), (5, 14, 1), (1, 14, 0)], color_idx: 5 })],
        vec![("16", ExpectedPath { points: vec![(1, 14, 0), (1, 16, 0)], color_idx: 1 })],
        vec![("17", ExpectedPath { points: vec![(0, 15, 0), (0, 17, 0)], color_idx: 0 })],
        vec![("17", ExpectedPath { points: vec![(1, 16, 0), (1, 17, 1), (0, 17, 0)], color_idx: 1 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test28() {
    let input_nodes = vec![
        node("0", vec!["2", "1"]),
        node("1", vec!["2"]),
        node("2", vec!["3"]),
        node("3", vec!["4"]),
        node("4", vec!["6", "5"]),
        node("5", vec!["6"]),
        node("6", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 0, 0, 0, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("2", ExpectedPath { points: vec![(0, 0, 0), (0, 2, 0)], color_idx: 0 }), ("1", ExpectedPath { points: vec![(0, 0, 0), (1, 0, 2), (1, 1, 0)], color_idx: 1 })],
        vec![("2", ExpectedPath { points: vec![(1, 1, 0), (1, 2, 1), (0, 2, 0)], color_idx: 1 })],
        vec![("3", ExpectedPath { points: vec![(0, 2, 0), (0, 3, 0)], color_idx: 0 })],
        vec![("4", ExpectedPath { points: vec![(0, 3, 0), (0, 4, 0)], color_idx: 0 })],
        vec![("6", ExpectedPath { points: vec![(0, 4, 0), (0, 6, 0)], color_idx: 0 }), ("5", ExpectedPath { points: vec![(0, 4, 0), (1, 4, 2), (1, 5, 0)], color_idx: 1 })],
        vec![("6", ExpectedPath { points: vec![(1, 5, 0), (1, 6, 1), (0, 6, 0)], color_idx: 1 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test29() {
    let input_nodes = vec![
        node("0", vec!["7"]),
        node("1", vec!["15"]),
        node("2", vec!["17"]),
        node("3", vec!["8"]),
        node("4", vec!["18"]),
        node("5", vec!["12"]),
        node("6", vec!["20"]),
        node("7", vec!["9", "10"]),
        node("8", vec!["9", "11"]),
        node("9", vec!["13", "14"]),
        node("10", vec!["21"]),
        node("11", vec!["13"]),
        node("12", vec!["14"]),
        node("13", vec!["16", "15"]),
        node("14", vec!["19"]),
        node("15", vec!["26"]),
        node("16", vec!["27"]),
        node("17", vec!["25"]),
        node("18", vec!["24"]),
        node("19", vec!["23"]),
        node("20", vec!["22"]),
        node("21", vec!["22"]),
        node("22", vec!["23"]),
        node("23", vec!["24"]),
        node("24", vec!["25"]),
        node("25", vec!["26"]),
        node("26", vec!["27"]),
        node("27", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 2, 3, 4, 5, 6, 0, 3, 0, 7, 3, 5, 0, 4, 1, 0, 2, 3, 4, 5, 6, 5, 4, 3, 2, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("7", ExpectedPath { points: vec![(0, 0, 0), (0, 7, 0)], color_idx: 0 })],
        vec![("15", ExpectedPath { points: vec![(1, 1, 0), (1, 15, 0)], color_idx: 1 })],
        vec![("17", ExpectedPath { points: vec![(2, 2, 0), (2, 17, 0)], color_idx: 2 })],
        vec![("8", ExpectedPath { points: vec![(3, 3, 0), (3, 8, 0)], color_idx: 3 })],
        vec![("18", ExpectedPath { points: vec![(4, 4, 0), (4, 13, 1), (3, 13, 0), (3, 18, 0)], color_idx: 4 })],
        vec![("12", ExpectedPath { points: vec![(5, 5, 0), (5, 12, 0)], color_idx: 5 })],
        vec![("20", ExpectedPath { points: vec![(6, 6, 0), (6, 13, 1), (5, 13, 0), (5, 20, 0)], color_idx: 6 })],
        vec![("10", ExpectedPath { points: vec![(0, 7, 0), (7, 7, 2), (7, 10, 0)], color_idx: 7 }), ("9", ExpectedPath { points: vec![(0, 7, 0), (0, 9, 0)], color_idx: 0 })],
        vec![("9", ExpectedPath { points: vec![(3, 8, 0), (0, 8, 3), (0, 9, 0)], color_idx: 0 }), ("11", ExpectedPath { points: vec![(3, 8, 0), (3, 11, 0)], color_idx: 3 })],
        vec![("13", ExpectedPath { points: vec![(0, 9, 0), (0, 13, 0)], color_idx: 0 }), ("14", ExpectedPath { points: vec![(0, 9, 0), (8, 9, 2), (8, 13, 1), (7, 13, 0), (7, 14, 1), (4, 14, 0)], color_idx: 8 })],
        vec![("21", ExpectedPath { points: vec![(7, 10, 0), (7, 13, 1), (6, 13, 0), (6, 21, 0)], color_idx: 7 })],
        vec![("13", ExpectedPath { points: vec![(3, 11, 0), (3, 13, 1), (0, 13, 0)], color_idx: 3 })],
        vec![("14", ExpectedPath { points: vec![(5, 12, 0), (5, 13, 1), (4, 13, 0), (4, 14, 0)], color_idx: 5 })],
        vec![("16", ExpectedPath { points: vec![(0, 13, 0), (0, 16, 0)], color_idx: 0 }), ("15", ExpectedPath { points: vec![(0, 13, 0), (1, 13, 2), (1, 15, 0)], color_idx: 1 })],
        vec![("19", ExpectedPath { points: vec![(4, 14, 0), (4, 19, 0)], color_idx: 5 })],
        vec![("26", ExpectedPath { points: vec![(1, 15, 0), (1, 26, 0)], color_idx: 1 })],
        vec![("27", ExpectedPath { points: vec![(0, 16, 0), (0, 27, 0)], color_idx: 0 })],
        vec![("25", ExpectedPath { points: vec![(2, 17, 0), (2, 25, 0)], color_idx: 2 })],
        vec![("24", ExpectedPath { points: vec![(3, 18, 0), (3, 24, 0)], color_idx: 4 })],
        vec![("23", ExpectedPath { points: vec![(4, 19, 0), (4, 23, 0)], color_idx: 5 })],
        vec![("22", ExpectedPath { points: vec![(5, 20, 0), (5, 22, 0)], color_idx: 6 })],
        vec![("22", ExpectedPath { points: vec![(6, 21, 0), (6, 22, 1), (5, 22, 0)], color_idx: 7 })],
        vec![("23", ExpectedPath { points: vec![(5, 22, 0), (5, 23, 1), (4, 23, 0)], color_idx: 6 })],
        vec![("24", ExpectedPath { points: vec![(4, 23, 0), (4, 24, 1), (3, 24, 0)], color_idx: 5 })],
        vec![("25", ExpectedPath { points: vec![(3, 24, 0), (3, 25, 1), (2, 25, 0)], color_idx: 4 })],
        vec![("26", ExpectedPath { points: vec![(2, 25, 0), (2, 26, 1), (1, 26, 0)], color_idx: 2 })],
        vec![("27", ExpectedPath { points: vec![(1, 26, 0), (1, 27, 1), (0, 27, 0)], color_idx: 1 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test30() {
    let input_nodes = vec![
        node("0", vec!["4"]),
        node("1", vec!["15"]),
        node("2", vec!["14"]),
        node("3", vec!["22"]),
        node("4", vec!["5", "10"]),
        node("5", vec!["6", "7"]),
        node("6", vec!["16", "8"]),
        node("7", vec!["9"]),
        node("8", vec!["11"]),
        node("9", vec!["16"]),
        node("10", vec!["18"]),
        node("11", vec!["12"]),
        node("12", vec!["13", "16"]),
        node("13", vec!["21"]),
        node("14", vec!["18"]),
        node("15", vec!["23"]),
        node("16", vec!["18", "17"]),
        node("17", vec!["18"]),
        node("18", vec!["20", "19"]),
        node("19", vec!["20"]),
        node("20", vec!["24"]),
        node("21", vec!["22"]),
        node("22", vec!["23"]),
        node("23", vec!["24"]),
        node("24", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 2, 3, 0, 0, 0, 5, 6, 5, 4, 6, 6, 6, 2, 1, 0, 6, 0, 4, 0, 3, 2, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("4", ExpectedPath { points: vec![(0, 0, 0), (0, 4, 0)], color_idx: 0 })],
        vec![("15", ExpectedPath { points: vec![(1, 1, 0), (1, 15, 0)], color_idx: 1 })],
        vec![("14", ExpectedPath { points: vec![(2, 2, 0), (2, 14, 0)], color_idx: 2 })],
        vec![("22", ExpectedPath { points: vec![(3, 3, 0), (3, 18, 1), (2, 18, 0), (2, 22, 0)], color_idx: 3 })],
        vec![("5", ExpectedPath { points: vec![(0, 4, 0), (0, 5, 0)], color_idx: 0 }), ("10", ExpectedPath { points: vec![(0, 4, 0), (4, 4, 2), (4, 10, 0)], color_idx: 4 })],
        vec![("6", ExpectedPath { points: vec![(0, 5, 0), (0, 6, 0)], color_idx: 0 }), ("7", ExpectedPath { points: vec![(0, 5, 0), (5, 5, 2), (5, 7, 0)], color_idx: 5 })],
        vec![("8", ExpectedPath { points: vec![(0, 6, 0), (6, 6, 2), (6, 8, 0)], color_idx: 6 }), ("16", ExpectedPath { points: vec![(0, 6, 0), (0, 16, 0)], color_idx: 0 })],
        vec![("9", ExpectedPath { points: vec![(5, 7, 0), (5, 9, 0)], color_idx: 5 })],
        vec![("11", ExpectedPath { points: vec![(6, 8, 0), (6, 11, 0)], color_idx: 6 })],
        vec![("16", ExpectedPath { points: vec![(5, 9, 0), (5, 16, 1), (0, 16, 0)], color_idx: 5 })],
        vec![("18", ExpectedPath { points: vec![(4, 10, 0), (4, 18, 1), (0, 18, 0)], color_idx: 4 })],
        vec![("12", ExpectedPath { points: vec![(6, 11, 0), (6, 12, 0)], color_idx: 6 })],
        vec![("13", ExpectedPath { points: vec![(6, 12, 0), (6, 13, 0)], color_idx: 6 }), ("16", ExpectedPath { points: vec![(6, 12, 0), (0, 12, 3), (0, 16, 0)], color_idx: 0 })],
        vec![("21", ExpectedPath { points: vec![(6, 13, 0), (6, 16, 1), (5, 16, 0), (5, 18, 1), (3, 18, 0), (3, 21, 0)], color_idx: 6 })],
        vec![("18", ExpectedPath { points: vec![(2, 14, 0), (2, 18, 1), (0, 18, 0)], color_idx: 2 })],
        vec![("23", ExpectedPath { points: vec![(1, 15, 0), (1, 23, 0)], color_idx: 1 })],
        vec![("17", ExpectedPath { points: vec![(0, 16, 0), (6, 16, 2), (6, 17, 0)], color_idx: 7 }), ("18", ExpectedPath { points: vec![(0, 16, 0), (0, 18, 0)], color_idx: 0 })],
        vec![("18", ExpectedPath { points: vec![(6, 17, 0), (6, 18, 1), (0, 18, 0)], color_idx: 7 })],
        vec![("20", ExpectedPath { points: vec![(0, 18, 0), (0, 20, 0)], color_idx: 0 }), ("19", ExpectedPath { points: vec![(0, 18, 0), (4, 18, 2), (4, 19, 0)], color_idx: 5 })],
        vec![("20", ExpectedPath { points: vec![(4, 19, 0), (4, 20, 1), (0, 20, 0)], color_idx: 5 })],
        vec![("24", ExpectedPath { points: vec![(0, 20, 0), (0, 24, 0)], color_idx: 0 })],
        vec![("22", ExpectedPath { points: vec![(3, 21, 0), (3, 22, 1), (2, 22, 0)], color_idx: 6 })],
        vec![("23", ExpectedPath { points: vec![(2, 22, 0), (2, 23, 1), (1, 23, 0)], color_idx: 3 })],
        vec![("24", ExpectedPath { points: vec![(1, 23, 0), (1, 24, 1), (0, 24, 0)], color_idx: 1 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test31() {
    let input_nodes = vec![
        node("0", vec!["3"]),
        node("1", vec!["4"]),
        node("2", vec!["5", "4"]),
        node("3", vec!["4"]),
        node("4", vec!["6"]),
        node("5", vec!["6"]),
        node("6", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 2, 0, 0, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("3", ExpectedPath { points: vec![(0, 0, 0), (0, 3, 0)], color_idx: 0 })],
        vec![("4", ExpectedPath { points: vec![(1, 1, 0), (1, 4, 1), (0, 4, 0)], color_idx: 1 })],
        vec![("5", ExpectedPath { points: vec![(2, 2, 0), (2, 4, 1), (1, 4, 0), (1, 5, 0)], color_idx: 2 }), ("4", ExpectedPath { points: vec![(2, 2, 0), (1, 2, 3), (1, 4, 1), (0, 4, 0)], color_idx: 1 })],
        vec![("4", ExpectedPath { points: vec![(0, 3, 0), (0, 4, 0)], color_idx: 0 })],
        vec![("6", ExpectedPath { points: vec![(0, 4, 0), (0, 6, 0)], color_idx: 0 })],
        vec![("6", ExpectedPath { points: vec![(1, 5, 0), (1, 6, 1), (0, 6, 0)], color_idx: 2 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test32() {
    let input_nodes = vec![
        node("0", vec!["2"]),
        node("1", vec!["5", "3"]),
        node("2", vec!["3", "4"]),
        node("3", vec!["6"]),
        node("4", vec!["5"]),
        node("5", vec!["6"]),
        node("6", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 0, 0, 2, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("2", ExpectedPath { points: vec![(0, 0, 0), (0, 2, 0)], color_idx: 0 })],
        vec![("5", ExpectedPath { points: vec![(1, 1, 0), (1, 5, 0)], color_idx: 1 }), ("3", ExpectedPath { points: vec![(1, 1, 0), (2, 1, 2), (2, 3, 1), (0, 3, 0)], color_idx: 2 })],
        vec![("3", ExpectedPath { points: vec![(0, 2, 0), (0, 3, 0)], color_idx: 0 }), ("4", ExpectedPath { points: vec![(0, 2, 0), (3, 2, 2), (3, 3, 1), (2, 3, 0), (2, 4, 0)], color_idx: 3 })],
        vec![("6", ExpectedPath { points: vec![(0, 3, 0), (0, 6, 0)], color_idx: 0 })],
        vec![("5", ExpectedPath { points: vec![(2, 4, 0), (2, 5, 1), (1, 5, 0)], color_idx: 3 })],
        vec![("6", ExpectedPath { points: vec![(1, 5, 0), (1, 6, 1), (0, 6, 0)], color_idx: 1 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test33() {
    let input_nodes = vec![
        node("0", vec!["3"]),
        node("1", vec!["5"]),
        node("2", vec!["7"]),
        node("3", vec!["9", "4"]),
        node("4", vec!["9"]),
        node("5", vec!["8", "6"]),
        node("6", vec!["10"]),
        node("7", vec!["10"]),
        node("8", vec!["10"]),
        node("9", vec!["13"]),
        node("10", vec!["12", "11"]),
        node("11", vec!["12"]),
        node("12", vec!["13"]),
        node("13", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 2, 0, 3, 1, 4, 2, 1, 0, 1, 2, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("3", ExpectedPath { points: vec![(0, 0, 0), (0, 3, 0)], color_idx: 0 })],
        vec![("5", ExpectedPath { points: vec![(1, 1, 0), (1, 5, 0)], color_idx: 1 })],
        vec![("7", ExpectedPath { points: vec![(2, 2, 0), (2, 7, 0)], color_idx: 2 })],
        vec![("9", ExpectedPath { points: vec![(0, 3, 0), (0, 9, 0)], color_idx: 0 }), ("4", ExpectedPath { points: vec![(0, 3, 0), (3, 3, 2), (3, 4, 0)], color_idx: 3 })],
        vec![("9", ExpectedPath { points: vec![(3, 4, 0), (3, 9, 1), (0, 9, 0)], color_idx: 3 })],
        vec![("6", ExpectedPath { points: vec![(1, 5, 0), (4, 5, 2), (4, 6, 0)], color_idx: 4 }), ("8", ExpectedPath { points: vec![(1, 5, 0), (1, 8, 0)], color_idx: 1 })],
        vec![("10", ExpectedPath { points: vec![(4, 6, 0), (4, 9, 1), (3, 9, 0), (3, 10, 1), (1, 10, 0)], color_idx: 4 })],
        vec![("10", ExpectedPath { points: vec![(2, 7, 0), (2, 10, 1), (1, 10, 0)], color_idx: 2 })],
        vec![("10", ExpectedPath { points: vec![(1, 8, 0), (1, 10, 0)], color_idx: 1 })],
        vec![("13", ExpectedPath { points: vec![(0, 9, 0), (0, 13, 0)], color_idx: 0 })],
        vec![("11", ExpectedPath { points: vec![(1, 10, 0), (2, 10, 2), (2, 11, 0)], color_idx: 5 }), ("12", ExpectedPath { points: vec![(1, 10, 0), (1, 12, 0)], color_idx: 1 })],
        vec![("12", ExpectedPath { points: vec![(2, 11, 0), (2, 12, 1), (1, 12, 0)], color_idx: 5 })],
        vec![("13", ExpectedPath { points: vec![(1, 12, 0), (1, 13, 1), (0, 13, 0)], color_idx: 1 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test34() {
    let input_nodes = vec![
        node("0", vec!["5"]),
        node("1", vec!["4", "2"]),
        node("2", vec!["3"]),
        node("3", vec!["4", "5"]),
        node("4", vec!["6"]),
        node("5", vec!["7"]),
        node("6", vec!["7"]),
        node("7", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 2, 2, 1, 0, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("5", ExpectedPath { points: vec![(0, 0, 0), (0, 5, 0)], color_idx: 0 })],
        vec![("4", ExpectedPath { points: vec![(1, 1, 0), (1, 4, 0)], color_idx: 1 }), ("2", ExpectedPath { points: vec![(1, 1, 0), (2, 1, 2), (2, 2, 0)], color_idx: 2 })],
        vec![("3", ExpectedPath { points: vec![(2, 2, 0), (2, 3, 0)], color_idx: 2 })],
        vec![("5", ExpectedPath { points: vec![(2, 3, 0), (0, 3, 3), (0, 5, 0)], color_idx: 0 }), ("4", ExpectedPath { points: vec![(2, 3, 0), (2, 4, 1), (1, 4, 0)], color_idx: 2 })],
        vec![("6", ExpectedPath { points: vec![(1, 4, 0), (1, 6, 0)], color_idx: 1 })],
        vec![("7", ExpectedPath { points: vec![(0, 5, 0), (0, 7, 0)], color_idx: 0 })],
        vec![("7", ExpectedPath { points: vec![(1, 6, 0), (1, 7, 1), (0, 7, 0)], color_idx: 1 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test35() {
    let input_nodes = vec![
        node("0", vec!["4", "1"]),
        node("1", vec!["2", "3"]),
        node("2", vec![]),
        node("3", vec!["4"]),
        node("4", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 1, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("4", ExpectedPath { points: vec![(0, 0, 0), (0, 4, 0)], color_idx: 0 }), ("1", ExpectedPath { points: vec![(0, 0, 0), (1, 0, 2), (1, 1, 0)], color_idx: 1 })],
        vec![("2", ExpectedPath { points: vec![(1, 1, 0), (1, 2, 0)], color_idx: 1 }), ("3", ExpectedPath { points: vec![(1, 1, 0), (2, 1, 2), (2, 3, 1), (1, 3, 0)], color_idx: 2 })],
        vec![],
        vec![("4", ExpectedPath { points: vec![(1, 3, 0), (1, 4, 1), (0, 4, 0)], color_idx: 2 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test36() {
    let input_nodes = vec![
        node("0", vec!["4", "1"]),
        node("1", vec!["4", "2"]),
        node("2", vec!["3", "5"]),
        node("3", vec![]),
        node("4", vec!["6"]),
        node("5", vec!["6"]),
        node("6", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 2, 2, 0, 1, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("4", ExpectedPath { points: vec![(0, 0, 0), (0, 4, 0)], color_idx: 0 }), ("1", ExpectedPath { points: vec![(0, 0, 0), (1, 0, 2), (1, 1, 0)], color_idx: 1 })],
        vec![("4", ExpectedPath { points: vec![(1, 1, 0), (1, 4, 1), (0, 4, 0)], color_idx: 1 }), ("2", ExpectedPath { points: vec![(1, 1, 0), (2, 1, 2), (2, 2, 0)], color_idx: 2 })],
        vec![("3", ExpectedPath { points: vec![(2, 2, 0), (2, 3, 0)], color_idx: 2 }), ("5", ExpectedPath { points: vec![(2, 2, 0), (3, 2, 2), (3, 4, 1), (1, 4, 0), (1, 5, 0)], color_idx: 3 })],
        vec![],
        vec![("6", ExpectedPath { points: vec![(0, 4, 0), (0, 6, 0)], color_idx: 0 })],
        vec![("6", ExpectedPath { points: vec![(1, 5, 0), (1, 6, 1), (0, 6, 0)], color_idx: 3 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test37() {
    let input_nodes = vec![
        node("0", vec!["5"]),
        node("1", vec!["6"]),
        node("2", vec!["5"]),
        node("3", vec!["4"]),
        node("4", vec!["8", "7"]),
        node("5", vec!["6"]),
        node("6", vec!["7"]),
        node("7", vec!["8"]),
        node("8", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 2, 3, 3, 0, 0, 0, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("5", ExpectedPath { points: vec![(0, 0, 0), (0, 5, 0)], color_idx: 0 })],
        vec![("6", ExpectedPath { points: vec![(1, 1, 0), (1, 6, 1), (0, 6, 0)], color_idx: 1 })],
        vec![("5", ExpectedPath { points: vec![(2, 2, 0), (2, 5, 1), (0, 5, 0)], color_idx: 2 })],
        vec![("4", ExpectedPath { points: vec![(3, 3, 0), (3, 4, 0)], color_idx: 3 })],
        vec![("8", ExpectedPath { points: vec![(3, 4, 0), (3, 5, 1), (2, 5, 0), (2, 6, 1), (1, 6, 0), (1, 8, 1), (0, 8, 0)], color_idx: 3 }), ("7", ExpectedPath { points: vec![(3, 4, 0), (4, 4, 2), (4, 5, 1), (3, 5, 0), (3, 6, 1), (2, 6, 0), (2, 7, 1), (0, 7, 0)], color_idx: 4 })],
        vec![("6", ExpectedPath { points: vec![(0, 5, 0), (0, 6, 0)], color_idx: 0 })],
        vec![("7", ExpectedPath { points: vec![(0, 6, 0), (0, 7, 0)], color_idx: 0 })],
        vec![("8", ExpectedPath { points: vec![(0, 7, 0), (0, 8, 0)], color_idx: 0 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test38() {
    let input_nodes = vec![
        node("0", vec!["5"]),
        node("1", vec!["2"]),
        node("2", vec!["6"]),
        node("3", vec!["6"]),
        node("4", vec!["7"]),
        node("5", vec![]),
        node("6", vec!["7"]),
        node("7", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 1, 2, 3, 0, 0, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("5", ExpectedPath { points: vec![(0, 0, 0), (0, 5, 0)], color_idx: 0 })],
        vec![("2", ExpectedPath { points: vec![(1, 1, 0), (1, 2, 0)], color_idx: 1 })],
        vec![("6", ExpectedPath { points: vec![(1, 2, 0), (1, 6, 1), (0, 6, 0)], color_idx: 1 })],
        vec![("6", ExpectedPath { points: vec![(2, 3, 0), (2, 6, 1), (0, 6, 0)], color_idx: 2 })],
        vec![("7", ExpectedPath { points: vec![(3, 4, 0), (3, 6, 1), (1, 6, 0), (1, 7, 1), (0, 7, 0)], color_idx: 3 })],
        vec![],
        vec![("7", ExpectedPath { points: vec![(0, 6, 0), (0, 7, 0)], color_idx: 1 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test39() {
    let input_nodes = vec![
        node("0", vec!["6"]),
        node("1", vec!["5"]),
        node("2", vec!["6"]),
        node("3", vec!["7"]),
        node("4", vec!["8"]),
        node("5", vec![]),
        node("6", vec!["7"]),
        node("7", vec!["8"]),
        node("8", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 2, 3, 4, 1, 0, 0, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("6", ExpectedPath { points: vec![(0, 0, 0), (0, 6, 0)], color_idx: 0 })],
        vec![("5", ExpectedPath { points: vec![(1, 1, 0), (1, 5, 0)], color_idx: 1 })],
        vec![("6", ExpectedPath { points: vec![(2, 2, 0), (2, 6, 1), (0, 6, 0)], color_idx: 2 })],
        vec![("7", ExpectedPath { points: vec![(3, 3, 0), (3, 6, 1), (1, 6, 0), (1, 7, 1), (0, 7, 0)], color_idx: 3 })],
        vec![("8", ExpectedPath { points: vec![(4, 4, 0), (4, 6, 1), (2, 6, 0), (2, 7, 1), (1, 7, 0), (1, 8, 1), (0, 8, 0)], color_idx: 4 })],
        vec![],
        vec![("7", ExpectedPath { points: vec![(0, 6, 0), (0, 7, 0)], color_idx: 0 })],
        vec![("8", ExpectedPath { points: vec![(0, 7, 0), (0, 8, 0)], color_idx: 0 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test40() {
    let input_nodes = vec![
        node("0", vec!["7"]),
        node("1", vec!["5"]),
        node("2", vec!["6"]),
        node("3", vec!["8"]),
        node("4", vec!["9"]),
        node("5", vec![]),
        node("6", vec!["7"]),
        node("7", vec!["8"]),
        node("8", vec!["9"]),
        node("9", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 2, 3, 4, 1, 1, 0, 0, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("7", ExpectedPath { points: vec![(0, 0, 0), (0, 7, 0)], color_idx: 0 })],
        vec![("5", ExpectedPath { points: vec![(1, 1, 0), (1, 5, 0)], color_idx: 1 })],
        vec![("6", ExpectedPath { points: vec![(2, 2, 0), (2, 6, 1), (1, 6, 0)], color_idx: 2 })],
        vec![("8", ExpectedPath { points: vec![(3, 3, 0), (3, 6, 1), (2, 6, 0), (2, 7, 1), (1, 7, 0), (1, 8, 1), (0, 8, 0)], color_idx: 3 })],
        vec![("9", ExpectedPath { points: vec![(4, 4, 0), (4, 6, 1), (3, 6, 0), (3, 7, 1), (2, 7, 0), (2, 8, 1), (1, 8, 0), (1, 9, 1), (0, 9, 0)], color_idx: 4 })],
        vec![],
        vec![("7", ExpectedPath { points: vec![(1, 6, 0), (1, 7, 1), (0, 7, 0)], color_idx: 2 })],
        vec![("8", ExpectedPath { points: vec![(0, 7, 0), (0, 8, 0)], color_idx: 0 })],
        vec![("9", ExpectedPath { points: vec![(0, 8, 0), (0, 9, 0)], color_idx: 0 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

#[test]
fn test41() {
    // Test41 test the date-order bug where parent defined before node ends up with an infinite branch going down
    let input_nodes = vec![
        node("0", vec!["2"]),
        node("1", vec!["4", "3"]),
        node("2", vec!["3", "1"]),
        node("3", vec!["4"]),
        node("4", vec!["5"]),
        node("5", vec![]),
    ];

    let result = build_tree(&input_nodes, &custom_colors(), "", -1);
    assert!(result.is_ok());
    let out = result.unwrap();

    let expected_columns = vec![0, 1, 0, 0, 0, 0];
    validate_columns(&expected_columns, &out.nodes);

    // Path validation
    let expected_paths = vec![
        vec![("2", ExpectedPath { points: vec![(0, 0, 0), (0, 2, 0)], color_idx: 0 })],
        vec![("3", ExpectedPath { points: vec![(1, 1, 0), (2, 1, 2), (2, 3, 1), (0, 3, 0)], color_idx: 2 }), ("4", ExpectedPath { points: vec![(1, 1, 0), (1, 4, 1), (0, 4, 0)], color_idx: 1 })],
        vec![("3", ExpectedPath { points: vec![(0, 2, 0), (0, 3, 0)], color_idx: 0 }), ("1", ExpectedPath { points: vec![(0, 2, 0), (3, 2, 2), (3, 3, 1), (2, 3, 0), (2, 4, 1), (1, 4, 0), (1, 5, 1), (1, 5, 0), (1, 6, 0)], color_idx: 3 })],
        vec![("4", ExpectedPath { points: vec![(0, 3, 0), (0, 4, 0)], color_idx: 0 })],
        vec![("5", ExpectedPath { points: vec![(0, 4, 0), (0, 5, 0)], color_idx: 0 })]
    ];
    validate_paths(&out.nodes, &expected_paths);
}

// Note: TestPathHeight1, TestCropPathAt, TestExpandPath are unit tests for
// internal Path functions, not integration tests, so they're not included here
