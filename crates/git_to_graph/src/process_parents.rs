//! Parent processing logic for git graph.

use super::Node;
use super::color::ColorsManager;
use super::column::ColumnManager;
use super::node::{InternalNode, NodeExt};
use super::path::new_point;
use super::point::Point;
use super::types::PointType;
use std::rc::Rc;

/// Process all parents of a node.
pub fn process_parents(
    node: &InternalNode,
    input_nodes: &[Node],
    column_man: &mut ColumnManager,
    colors_man: &mut ColorsManager,
) {
    let node_borrow = node.borrow();
    let parents: Vec<InternalNode> = node_borrow.parents.clone();
    drop(node_borrow);

    for (parent_idx, parent) in parents.iter().enumerate() {
        process_parent(
            node,
            parent,
            parent_idx,
            input_nodes,
            column_man,
            colors_man,
        );
    }
}

/// Process a single parent of a node.
pub fn process_parent(
    node: &InternalNode,
    parent: &InternalNode,
    parent_idx: usize,
    input_nodes: &[Node],
    column_man: &mut ColumnManager,
    colors_man: &mut ColorsManager,
) {
    let is_first_parent = parent_idx == 0;

    // Add initial pipe point to path
    {
        let node_column = node.borrow().column;
        let y_ref = Rc::clone(&node.borrow().idx);
        let parent_id_clone = parent.borrow().id.clone();
        let mut node_borrow = node.borrow_mut();
        let node_path_to_parent = node_borrow.path_to_id(&parent_id_clone);
        node_path_to_parent.no_dup_append(new_point(node_column, y_ref, PointType::Pipe));
    }

    let node_column = node.borrow().column;
    let node_idx = *node.borrow().idx.borrow();
    let parent_col_defined = parent.borrow().column_defined();
    let parent_column = parent.borrow().column;

    if !parent_col_defined {
        // Check if node.pathTo(node.parents[0]).isMergeTo()
        let is_merge_to_path = {
            let has_parents = !node.borrow().parents.is_empty();
            if has_parents {
                let first_parent = Rc::clone(&node.borrow().parents[0]);
                let mut node_mut = node.borrow_mut();
                let path_to_first_parent = node_mut.path_to(&first_parent);
                path_to_first_parent.is_merge_to()
            } else {
                false
            }
        };

        if is_first_parent || is_merge_to_path {
            let mut parent_borrow = parent.borrow_mut();
            parent_borrow.set_column(node_column);
            let node_color_idx = node.borrow().color_idx;
            parent_borrow.set_color(node_color_idx);
        } else {
            let new_col = column_man.next();
            let mut parent_borrow = parent.borrow_mut();
            parent_borrow.set_column(new_col);
            parent_borrow.set_color(colors_man.get_color(node_idx));

            // Add fork point
            let parent_col = parent_borrow.column;
            drop(parent_borrow);

            let y_ref = Rc::clone(&node.borrow().idx);
            let parent_id_clone = parent.borrow().id.clone();
            let mut node_borrow = node.borrow_mut();
            let node_path_to_parent = node_borrow.path_to_id(&parent_id_clone);
            node_path_to_parent.no_dup_append(new_point(parent_col, y_ref, PointType::Fork));

            node_borrow.set_first_of_branch();
        }

        // Set path color
        let parent_color_idx = parent.borrow().color_idx;
        let parent_id_clone = parent.borrow().id.clone();
        let mut node_borrow = node.borrow_mut();
        let node_path_to_parent = node_borrow.path_to_id(&parent_id_clone);
        node_path_to_parent.set_color(parent_color_idx);
    } else if node_column < parent_column {
        if is_first_parent {
            let parent_children: Vec<InternalNode> = parent.borrow().children.clone();
            let parent_id_clone = parent.borrow().id.clone();
            for child in &parent_children {
                let mut child_borrow = child.borrow_mut();
                let path_to_parent = child_borrow.path_to_id(&parent_id_clone);
                if path_to_parent.is_valid() {
                    path_to_parent.remove_last();
                    let last_x = path_to_parent.last().get_x();
                    let parent_idx_rc = Rc::clone(&parent.borrow().idx);
                    path_to_parent.no_dup_append(new_point(
                        last_x,
                        parent_idx_rc.clone(),
                        PointType::MergeBack,
                    ));
                    path_to_parent.no_dup_append(new_point(
                        node_column,
                        parent_idx_rc,
                        PointType::Pipe,
                    ));
                }
            }

            let (parent_id_clone, node_color_idx) = {
                let mut parent_borrow = parent.borrow_mut();
                parent_borrow.set_column(node_column);
                let node_color_idx = node.borrow().color_idx;
                parent_borrow.set_color(node_color_idx);
                (parent_borrow.id.clone(), node_color_idx)
            }; // Drop parent_borrow here

            let mut node_borrow = node.borrow_mut();
            let node_path_to_parent = node_borrow.path_to_id(&parent_id_clone);
            node_path_to_parent.set_color(node_color_idx);
        } else {
            let y_ref = Rc::clone(&node.borrow().idx);
            let parent_color = parent.borrow().color_idx;
            let parent_id_clone = parent.borrow().id.clone();
            let mut node_borrow = node.borrow_mut();
            let node_path_to_parent = node_borrow.path_to_id(&parent_id_clone);
            node_path_to_parent.no_dup_append(new_point(parent_column, y_ref, PointType::Fork));
            node_path_to_parent.set_color(parent_color);
        }
    } else if node_column > parent_column {
        let next_node_idx = node_idx as usize + 1;
        let next_node_id = if next_node_idx < input_nodes.len() {
            input_nodes[next_node_idx].get_id()
        } else {
            String::new() // No next node, use empty string
        };
        let parent_id_full = parent.borrow().id.clone();
        let node_first_in_branch = node.borrow().first_in_branch();

        if is_first_parent && (parent_id_full != next_node_id || node_first_in_branch) {
            let node_color = node.borrow().color_idx;
            let parent_id_clone = parent.borrow().id.clone();
            let mut node_borrow = node.borrow_mut();
            let node_path_to_parent = node_borrow.path_to_id(&parent_id_clone);
            let parent_idx_rc = Rc::clone(&parent.borrow().idx);
            node_path_to_parent.no_dup_append(new_point(
                node_column,
                parent_idx_rc,
                PointType::MergeBack,
            ));
            node_path_to_parent.set_color(node_color);
        } else {
            let y_ref = Rc::clone(&node.borrow().idx);
            let parent_color = parent.borrow().color_idx;
            let parent_id_clone = parent.borrow().id.clone();
            let mut node_borrow = node.borrow_mut();
            let node_path_to_parent = node_borrow.path_to_id(&parent_id_clone);
            node_path_to_parent.no_dup_append(new_point(parent_column, y_ref, PointType::MergeTo));
            node_path_to_parent.set_color(parent_color);
        }
    } else if node_column == parent_column {
        let mut parent_borrow = parent.borrow_mut();
        parent_borrow.set_color(node.borrow().color_idx);
    }

    // Add final point
    {
        let parent_id_clone = parent.borrow().id.clone();
        let mut node_borrow = node.borrow_mut();
        let node_path_to_parent = node_borrow.path_to_id(&parent_id_clone);
        let parent_idx_rc = Rc::clone(&parent.borrow().idx);
        let parent_col = parent.borrow().column;
        node_path_to_parent.no_dup_append(new_point(parent_col, parent_idx_rc, PointType::Pipe));
    }
}
