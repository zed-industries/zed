//! Main algorithm for computing git graph columns.

use super::Node;
use super::color::ColorsManager;
use super::column::ColumnManager;
use super::finalize::{finalize_nodes, slice_results};
use super::node::{InternalNode, InternalNodeSet, NodeExt, new_node};
use super::path::Path;
use super::point::Point;
use super::process_children::process_children;
use super::process_parents::process_parents;
use super::types::ternary;
use std::collections::HashMap;

/// Main algorithm: compute columns for nodes.
pub fn set_columns(input_nodes: &[Node], from: &str, limit: i32) -> (Vec<InternalNode>, Vec<Path>) {
    let orig_limit = limit;
    let mut colors_man = ColorsManager::new();
    let mut column_man = ColumnManager::new();
    let mut unassigned_nodes: HashMap<String, InternalNode> = HashMap::new();
    let mut tmp_row = -1;
    let mut following_nodes = InternalNodeSet::new();
    let mut from_idx = ternary(from.is_empty(), 0, -1);
    let mut nodes = Vec::new();
    let mut limit = limit;
    let mut partial_paths = Vec::new();

    for (idx, raw_node) in input_nodes.iter().enumerate() {
        if limit == 0 {
            break;
        }

        let node = init_node(
            raw_node,
            idx as i32,
            &mut tmp_row,
            &mut unassigned_nodes,
            &mut column_man,
            &mut colors_man,
        );
        nodes.push(node.clone());

        update_limit_and_index(&node, from, &mut limit, &mut from_idx, idx as i32);
        update_node_tracking(&node, &mut following_nodes);
        process_children(
            &node,
            input_nodes,
            &following_nodes,
            &mut column_man,
            &mut colors_man,
        );

        process_parents(&node, input_nodes, &mut column_man, &mut colors_man);

        let node_id_full = node.borrow().id.clone();
        if node_id_full == from {
            partial_paths = calc_partial_paths(&following_nodes);
        }
    }

    finalize_nodes(
        &following_nodes,
        &nodes,
        &partial_paths,
        from_idx,
        orig_limit,
    );

    (slice_results(&nodes, from_idx, orig_limit), partial_paths)
}

/// Initialize a node.
pub fn init_node(
    raw_node: &Node,
    idx: i32,
    tmp_row: &mut i32,
    unassigned_nodes: &mut HashMap<String, InternalNode>,
    column_man: &mut ColumnManager,
    colors_man: &mut ColorsManager,
) -> InternalNode {
    let id = raw_node.get_id();
    let node = if let Some(n) = unassigned_nodes.remove(&id) {
        n.borrow().move_down(idx);
        n
    } else {
        new_node(id.clone(), idx)
    };

    node.borrow_mut().initial_node = Some(raw_node.clone());

    // Add node parent IDs to the index cache
    for parent_id in raw_node.get_parents() {
        let parent_node = unassigned_nodes
            .entry(parent_id.clone())
            .or_insert_with(|| {
                let new_parent_node = new_node(parent_id, *tmp_row);
                *tmp_row -= 1;
                new_parent_node
            });
        parent_node.borrow_mut().children.push(node.clone());
        node.borrow_mut().parents.push(parent_node.clone());
    }

    // Set column if not defined
    if !node.borrow().column_defined() {
        let col = column_man.next();
        node.borrow_mut().set_column(col);
        let color = colors_man.get_color(*node.borrow().idx.borrow());
        node.borrow_mut().set_color(color);
    }

    node
}

/// Update limit and index based on the "from" parameter.
pub fn update_limit_and_index(
    node: &InternalNode,
    from: &str,
    limit: &mut i32,
    from_idx: &mut i32,
    idx: i32,
) {
    if node.borrow().id == from {
        *from_idx = idx + 1;
        *limit += 2;
    }
    if *from_idx != -1 {
        *limit -= 1;
    }
}

/// Cache the following node with child before the current node.
pub fn update_node_tracking(node: &InternalNode, following_nodes: &mut InternalNodeSet) {
    let parents = node.borrow().parents.clone();
    following_nodes.add(&parents);
    following_nodes.remove(node);
}

/// Return the paths that come from outside our page.
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
                            std::rc::Rc::new(std::cell::RefCell::new(point.get_y())),
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
