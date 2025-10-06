//! Tree building and output generation.

use super::algorithm::set_columns;
use super::color::ColorGenerator;
use super::node::Node;
use super::output::{Out, PartialPath, Row, RowLine};
use super::path::expand_path;
use super::point::Point;
use super::types::{
    BOTTOM_HALF_LINE, FORK_LINE, FULL_LINE, G_KEY, MERGE_BACK_LINE, TOP_HALF_LINE,
    PARENTS_PATHS_TEST_KEY, PointType,
};
use serde_json::{Map, Value};

/// Build tree for graph rendering.
pub fn build_tree(
    input_nodes: &[Node],
    color_gen: &dyn ColorGenerator,
    from: &str,
    limit: i32,
) -> Result<Out, String> {
    let (nodes, partial_paths) = set_columns(input_nodes, from, limit);

    let mut final_struct = Vec::new();
    for node in &nodes {
        let node_borrow = node.borrow();
        let mut final_parents_paths = Vec::new();

        for parent in &node_borrow.parents {
            let parent_id = parent.borrow().id.clone();
            if let Some(n) = node_borrow.parents_paths.get(&parent_id) {
                let mut path = Vec::new();
                for point in &n.points {
                    path.push(Value::Array(vec![
                        Value::Number(point.get_x().into()),
                        Value::Number(point.get_y().into()),
                        Value::Number((point.get_type() as u8).into()),
                    ]));
                }
                final_parents_paths.push(Value::Array(vec![
                    Value::String(color_gen.get_color(n.color_idx)),
                    Value::Array(path),
                ]));
            }
        }

        if let Some(mut final_node) = node_borrow.initial_node.clone() {
            let idx = *node_borrow.idx.borrow();
            final_node.insert(
                G_KEY.to_string(),
                Value::Array(vec![
                    Value::Number(idx.into()),
                    Value::Number(node_borrow.column.into()),
                    Value::String(color_gen.get_color(node_borrow.color_idx)),
                    Value::Array(final_parents_paths),
                ]),
            );

            // Add test key for path validation
            let mut parents_paths_map = Map::new();
            for parent in &node_borrow.parents {
                let parent_id = parent.borrow().id.clone();
                if let Some(path) = node_borrow.parents_paths.get(&parent_id) {
                    let mut points_array = Vec::new();
                    for point in &path.points {
                        points_array.push(Value::Array(vec![
                            Value::Number(point.get_x().into()),
                            Value::Number(point.get_y().into()),
                            Value::Number((point.get_type() as u8).into()),
                        ]));
                    }
                    let path_obj = serde_json::json!({
                        "points": points_array,
                        "colorIdx": path.color_idx,
                    });
                    parents_paths_map.insert(parent_id, path_obj);
                }
            }
            final_node.insert(
                PARENTS_PATHS_TEST_KEY.to_string(),
                Value::Object(parents_paths_map),
            );

            final_struct.push(final_node);
        }
    }

    let mut final_pp = Vec::new();
    for p in &partial_paths {
        let mut points = Vec::new();
        for point in &p.points {
            points.push((point.get_x(), point.get_y(), point.get_type() as u8));
        }
        final_pp.push(PartialPath {
            points,
            color: color_gen.get_color(p.color_idx),
        });
    }

    Ok(Out {
        first_sha: if input_nodes.is_empty() {
            String::new()
        } else {
            input_nodes[0].get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string()
        },
        nodes: final_struct,
        partial_paths: final_pp,
    })
}

/// Build tree as rows for rendering.
pub fn build_rows(
    input_nodes: &[Node],
    color_gen: &dyn ColorGenerator,
    from: &str,
    limit: i32,
) -> Result<Vec<Row>, String> {
    let (nodes, partial_paths) = set_columns(input_nodes, from, limit);

    if nodes.is_empty() {
        return Ok(Vec::new());
    }

    let offset = *nodes[0].borrow().idx.borrow();
    let mut out = vec![Row {
        initial_node: None,
        x: 0,
        color: String::new(),
        lines: Vec::new(),
    }; nodes.len() + 1];

    // Helper function for adding lines
    let add_line = |out: &mut [Row], y_offset: usize, x1: i32, x2: i32, line_type: i32, color: String| {
        if y_offset < out.len() {
            out[y_offset].lines.push(RowLine {
                x1,
                x2,
                typ: line_type,
                color,
            });
        }
    };

    // Process paths
    let process_path = |out: &mut [Row], path: &super::path::Path, offset: i32, color: &str, is_partial_path: bool| {
        let mut i = 1;
        while i < path.points.len() {
            let p1 = &path.points[i - 1];
            let p2 = &path.points[i];
            let y_offset1 = (p1.get_y() - offset) as usize;
            let y_offset2 = (p2.get_y() - offset) as usize;

            match p2.get_type() {
                PointType::Fork => {
                    add_line(out, y_offset1, p1.get_x(), p2.get_x(), FORK_LINE, color.to_string());
                    i += 1;
                    if i < path.points.len() {
                        let p3 = &path.points[i];
                        if p3.get_x() == p2.get_x() && p3.get_type() != PointType::MergeBack {
                            let y_offset3 = (p3.get_y() - offset) as usize;
                            add_line(out, y_offset3, p3.get_x(), p3.get_x(), TOP_HALF_LINE, color.to_string());
                        }
                    }
                }
                _ if p1.get_type() == PointType::MergeBack => {
                    add_line(out, y_offset1, p1.get_x(), p2.get_x(), MERGE_BACK_LINE, color.to_string());
                    if i < path.points.len() - 1 {
                        add_line(out, y_offset2, p2.get_x(), p2.get_x(), BOTTOM_HALF_LINE, color.to_string());
                    }
                    i += 1;
                    if i == path.points.len() - 1 {
                        let p3 = &path.points[i];
                        let y_offset3 = (p3.get_y() - offset) as usize;
                        add_line(out, y_offset3, p3.get_x(), p3.get_x(), TOP_HALF_LINE, color.to_string());
                    }
                }
                PointType::MergeTo => {
                    add_line(out, y_offset1, p1.get_x(), p2.get_x(), FORK_LINE, color.to_string());
                }
                _ if i == 1 => {
                    let line_type = if is_partial_path { FULL_LINE } else { BOTTOM_HALF_LINE };
                    add_line(out, y_offset1, p1.get_x(), p1.get_x(), line_type, color.to_string());
                    if i == path.points.len() - 1 {
                        add_line(out, y_offset2, p2.get_x(), p2.get_x(), TOP_HALF_LINE, color.to_string());
                    }
                }
                _ if i == path.points.len() - 1 => {
                    add_line(out, y_offset1, p1.get_x(), p1.get_x(), FULL_LINE, color.to_string());
                    add_line(out, y_offset2, p2.get_x(), p2.get_x(), TOP_HALF_LINE, color.to_string());
                }
                _ => {
                    add_line(out, y_offset1, p1.get_x(), p1.get_x(), FULL_LINE, color.to_string());
                }
            }
            i += 1;
        }
    };

    // Process each partial path
    for path2 in &partial_paths {
        if path2.points.is_empty() {
            continue;
        }
        let path = expand_path(path2);
        let path_color = color_gen.get_color(path.color_idx);
        process_path(&mut out, &path, offset, &path_color, true);
    }

    // Process nodes and their parent paths
    for (i, node) in nodes.iter().enumerate() {
        let node_borrow = node.borrow();
        let node_color_str = color_gen.get_color(node_borrow.color_idx);
        let node_column = node_borrow.column;
        let node_idx = *node_borrow.idx.borrow();

        let t = &mut out[i];
        if let Some(initial) = &node_borrow.initial_node {
            t.initial_node = Some(initial.clone());
        }
        t.x = node_column;
        t.color = node_color_str.clone();

        // Draw path arriving at node if the node is the first node of a new page and has children
        if !node_borrow.children.is_empty() {
            let first_child_column = node_borrow.children[0].borrow().column;
            if first_child_column == node_column {
                let y_off = (node_idx - offset) as usize;
                add_line(&mut out, y_off, node_column, node_column, TOP_HALF_LINE, node_color_str.clone());
            }
        }

        for parent in &node_borrow.parents {
            let parent_id = parent.borrow().id.clone();
            if let Some(parent_path) = node_borrow.parents_paths.get(&parent_id) {
                let parent_path_expanded = expand_path(parent_path);
                let path_color = color_gen.get_color(parent_path_expanded.color_idx);
                process_path(&mut out, &parent_path_expanded, offset, &path_color, false);
            }
        }
    }

    // Sort lines in each row instance
    let is_straight = |typ: i32| typ == BOTTOM_HALF_LINE || typ == TOP_HALF_LINE || typ == FULL_LINE;
    for row in &mut out {
        row.lines.sort_by(|a, b| {
            if is_straight(a.typ) {
                std::cmp::Ordering::Less
            } else if is_straight(b.typ) {
                std::cmp::Ordering::Greater
            } else {
                a.x1.cmp(&b.x1)
            }
        });
    }

    Ok(out[..nodes.len()].to_vec())
}
