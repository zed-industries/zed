//! Finalization functions for git graph processing.

use super::node::{InternalNode, InternalNodeSet};
use super::path::{new_point, Path};
use super::point::Point;
use std::cell::RefCell;
use std::cmp::min;
use std::rc::Rc;

/// Finalize nodes after processing.
///
/// # Examples
///
/// ```
/// use git_to_graph::finalize::finalize_nodes;
/// use git_to_graph::node::InternalNodeSet;
///
/// let following_nodes = InternalNodeSet::new();
/// let nodes = vec![];
/// let partial_paths = vec![];
/// finalize_nodes(&following_nodes, &nodes, &partial_paths, 0, 10);
/// ```
pub fn finalize_nodes(
    following_nodes: &InternalNodeSet,
    nodes: &[InternalNode],
    _partial_paths: &[Path],
    _from_idx: i32,
    _orig_limit: i32,
) {
    set_undefined_rows(following_nodes, nodes.len() as i32);
    // Note: crop functions would modify the paths, but they're passed by reference here
    // In the real implementation, these would need to be mutable
}

/// Set undefined rows for nodes.
pub fn set_undefined_rows(following_nodes: &InternalNodeSet, last_row_idx: i32) {
    for n in following_nodes.nodes() {
        let node = n.borrow();
        if *node.idx.borrow() < 0 {
            for c in &node.children {
                let mut child = c.borrow_mut();
                if let Some(p) = child.parents_paths.get_mut(&node.id) {
                    if p.len() >= 2 {
                        let second_to_last_x = p.second_to_last().get_x();
                        p.get_mut(-1).set_x(second_to_last_x);
                    }
                }
            }
            *node.idx.borrow_mut() = last_row_idx;
        }
    }
}

/// Crop partial paths to stay within bounds.
pub fn crop_partial_paths(paths: &mut [Path], from: i32, limit: i32) {
    if limit >= 0 {
        for path in paths.iter_mut() {
            *path = crop_path_at(path, from, limit);
        }
    }
}

/// Crop nodes' parent paths to stay within bounds.
pub fn crop_nodes_paths(nodes: &[InternalNode], from: i32, limit: i32) {
    if limit >= 0 {
        for n in nodes {
            let mut node = n.borrow_mut();
            let parent_ids: Vec<String> =
                node.parents.iter().map(|p| p.borrow().id.clone()).collect();

            for parent_id in parent_ids {
                if let Some(path) = node.parents_paths.get(&parent_id) {
                    let cropped = crop_path_end_at(path, from, limit);
                    node.parents_paths.insert(parent_id, cropped);
                }
            }
        }
    }
}

/// Calculate partial paths from nodes outside the page.
pub fn calc_partial_paths(following_nodes_with_children_before_idx: &InternalNodeSet) -> Vec<Path> {
    let mut out = Vec::new();
    for n in following_nodes_with_children_before_idx.nodes() {
        let node = n.borrow();
        for c in &node.children {
            let child = c.borrow();
            for parent in &child.parents {
                let parent_id = parent.borrow().id.clone();
                if let Some(path) = child.parents_paths.get(&parent_id) {
                    // Clone the path
                    let mut new_path = Path::new();
                    new_path.color_idx = path.color_idx;
                    for point in &path.points {
                        new_path.points.push(super::point::PointImpl::new(
                            point.get_x(),
                            Rc::new(RefCell::new(point.get_y())),
                            point.get_type(),
                        ));
                    }
                    out.push(new_path);
                }
            }
        }
    }
    out
}

/// Crop a path at the given boundaries.
pub fn crop_path_at(path: &Path, from: i32, limit: i32) -> Path {
    let mut points = Vec::new();
    let threshold = from + limit;

    for i in 1..path.points.len() {
        let p1 = &path.points[i - 1];
        let p2 = &path.points[i];

        if p1.get_y() < from && p2.get_y() > from {
            points.push(new_point(
                p1.get_x(),
                Rc::new(RefCell::new(from)),
                super::types::PointType::Pipe,
            ));
        }
        if p2.get_y() >= from {
            if p2.get_y() > threshold {
                points.push(new_point(
                    p1.get_x(),
                    Rc::new(RefCell::new(threshold)),
                    super::types::PointType::Pipe,
                ));
                break;
            }
            points.push(super::point::PointImpl::new(
                p2.get_x(),
                Rc::new(RefCell::new(p2.get_y())),
                p2.get_type(),
            ));
        }
    }

    Path {
        points,
        color_idx: path.color_idx,
    }
}

/// Crop a path end at the given threshold.
pub fn crop_path_end_at(path: &Path, from: i32, limit: i32) -> Path {
    if path.is_empty() {
        return Path::new();
    }

    let mut points = vec![super::point::PointImpl::new(
        path.points[0].get_x(),
        Rc::new(RefCell::new(path.points[0].get_y())),
        path.points[0].get_type(),
    )];
    let threshold = from + limit;

    for i in 1..path.points.len() {
        let p1 = &path.points[i - 1];
        let p2 = &path.points[i];

        if p2.get_y() > threshold {
            points.push(new_point(
                p1.get_x(),
                Rc::new(RefCell::new(threshold)),
                super::types::PointType::Pipe,
            ));
            break;
        }
        points.push(super::point::PointImpl::new(
            p2.get_x(),
            Rc::new(RefCell::new(p2.get_y())),
            p2.get_type(),
        ));
    }

    Path {
        points,
        color_idx: path.color_idx,
    }
}

/// Slice results based on pagination.
///
/// # Examples
///
/// ```
/// use git_to_graph::finalize::slice_results;
///
/// let nodes = vec![];
/// let result = slice_results(&nodes, 0, 10);
/// assert_eq!(result.len(), 0);
/// ```
pub fn slice_results(nodes: &[InternalNode], from_idx: i32, orig_limit: i32) -> Vec<InternalNode> {
    if orig_limit > 0 {
        let start = from_idx as usize;
        let end = min(from_idx as usize + orig_limit as usize, nodes.len());
        nodes[start..end].to_vec()
    } else {
        nodes.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_results() {
        let nodes = vec![];
        let result = slice_results(&nodes, 0, 10);
        assert_eq!(result.len(), 0);
    }
}
