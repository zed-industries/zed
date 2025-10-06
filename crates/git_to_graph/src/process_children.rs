//! Child processing logic for git graph.

use super::color::ColorsManager;
use super::column::ColumnManager;
use super::node::{InternalNode, InternalNodeSet, NodeExt, ProcessedNodes};
use super::path::new_point;
use super::point::Point;
use super::types::PointType;
use super::Node;
use std::rc::Rc;

/// Process all children of a node.
pub fn process_children(
    node: &InternalNode,
    input_nodes: &[Node],
    following_nodes_with_children_before_idx: &InternalNodeSet,
    column_man: &mut ColumnManager,
    colors_man: &mut ColorsManager,
) {
    let mut processed_nodes_inst = ProcessedNodes::new();
    let children: Vec<InternalNode> = node.borrow().children.clone();
    let node_column = node.borrow().column;
    let node_is_orphan = node.borrow().is_orphan();
    let node_idx = *node.borrow().idx.borrow();

    for child in &children {
        // Get path properties
        let (second_to_last_point_x, is_sub_branch, is_merge_to) = {
            let is_first_of_branch = child.borrow().is_first_of_branch();
            let mut child_borrow = child.borrow_mut();
            let path_to_node = child_borrow.path_to(node);

            if path_to_node.len() >= 2 {
                let second_to_last_x = path_to_node.second_to_last().get_x();
                let is_sub = path_to_node.is_fork() && !is_first_of_branch;
                let is_mt = path_to_node.is_merge_to();
                (second_to_last_x, is_sub, is_mt)
            } else {
                (0, false, false)
            }
        };

        if node_column < second_to_last_point_x || node_is_orphan {
            if !is_sub_branch && !is_merge_to {
                column_man.decr();
            }

            let path_color_idx = child.borrow_mut().path_to(node).color_idx;
            colors_man.release_color(path_color_idx, node_idx);

            // Insert before the last element
            if node_column != child.borrow().column {
                let node_idx_rc = Rc::clone(&node.borrow().idx);
                let mut child_borrow = child.borrow_mut();
                let path_to_node = child_borrow.path_to(node);
                path_to_node.no_dup_insert(-1, new_point(second_to_last_point_x, node_idx_rc, PointType::MergeBack));
            }

            // Process following nodes
            for following_node in following_nodes_with_children_before_idx.nodes() {
                let following_node_id = following_node.borrow().id.clone();
                let following_node_children: Vec<InternalNode> = following_node.borrow().children.clone();

                for following_node_child in &following_node_children {
                    let following_node_child_id = following_node_child.borrow().id.clone();
                    let following_node_child_idx = *following_node_child.borrow().idx.borrow();

                    if following_node_child_idx < node_idx
                        && !processed_nodes_inst.has_child(&following_node_id, &following_node_child_id)
                    {
                        let target_column = {
                            let mut fnc_borrow = following_node_child.borrow_mut();
                            let path_to_following = fnc_borrow.path_to(following_node);
                            path_to_following.get_height_at_idx(node_idx)
                        };

                        if target_column > second_to_last_point_x {
                            // Remove duplicate points
                            {
                                let mut fnc_borrow = following_node_child.borrow_mut();
                                let path_to_following = fnc_borrow.path_to(following_node);
                                while path_to_following.len() >= 2
                                    && path_to_following.last().get_y() == path_to_following.second_to_last().get_y()
                                {
                                    path_to_following.remove_second_to_last();
                                }
                                path_to_following.remove_last();
                            }

                            // Calculate nb of merging nodes
                            let mut nb_nodes_merging_back = 0;
                            let node_for_merge = if node_is_orphan && (node_idx as usize + 1) < input_nodes.len() {
                                let next_node_id = input_nodes[node_idx as usize + 1].get_id();
                                if let Some(next_node) = following_nodes_with_children_before_idx.get(&next_node_id) {
                                    nb_nodes_merging_back += 1;
                                    next_node
                                } else {
                                    node.clone()
                                }
                            } else {
                                node.clone()
                            };

                            nb_nodes_merging_back += node_for_merge.borrow().nb_nodes_merging_back(target_column);

                            let following_node_column = following_node.borrow().column;
                            let should_move_node = following_node_column > second_to_last_point_x
                                && !processed_nodes_inst.has_node(&following_node_id);
                            let final_following_node_column = if should_move_node {
                                following_node_column - nb_nodes_merging_back
                            } else {
                                following_node_column
                            };

                            // Add points to path
                            {
                                let mut fnc_borrow = following_node_child.borrow_mut();
                                let path_to_following = fnc_borrow.path_to(following_node);
                                let path_point_x = path_to_following.last().get_x();
                                let node_for_merge_idx = Rc::clone(&node_for_merge.borrow().idx);

                                path_to_following.no_dup_append(new_point(
                                    path_point_x,
                                    node_for_merge_idx.clone(),
                                    PointType::MergeBack,
                                ));
                                path_to_following.no_dup_append2(new_point(
                                    path_point_x - nb_nodes_merging_back,
                                    node_for_merge_idx.clone(),
                                    PointType::Pipe,
                                ));

                                let following_node_idx = Rc::clone(&following_node.borrow().idx);
                                path_to_following.no_dup_append2(new_point(
                                    final_following_node_column,
                                    following_node_idx,
                                    PointType::Pipe,
                                ));
                            }

                            if should_move_node {
                                following_node.borrow_mut().move_left(nb_nodes_merging_back);
                            }

                            processed_nodes_inst.set(following_node_id.clone(), following_node_child_id.clone());
                        }
                    }
                }
            }
        }
    }
}
