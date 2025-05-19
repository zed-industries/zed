// pub const NULL_NODE_ID: accesskit::NodeId = accesskit::NodeId(0);

// pub fn null_node() -> accesskit::Node {
//     let node = accesskit::Node::new(accesskit::Role::Window);
//     node
// }

use std::mem;

use collections::HashMap;

use crate::{DispatchTree, EntityId, FocusId};

pub struct AccessibilityData {
    next_node_id: u64,
    entity_ids: HashMap<EntityId, accesskit::NodeId>,
    focus_ids: HashMap<FocusId, accesskit::NodeId>,
    last_nodes: HashMap<accesskit::NodeId, accesskit::Node>,
    current_nodes: HashMap<accesskit::NodeId, accesskit::Node>,
    node_stack: Vec<accesskit::NodeId>,
}

impl AccessibilityData {
    pub fn new() -> Self {
        AccessibilityData {
            next_node_id: 0,
            entity_ids: HashMap::default(),
            last_nodes: HashMap::default(),
            current_nodes: HashMap::default(),
            focus_ids: HashMap::default(),
            node_stack: Vec::new(),
        }
    }

    // TODO: This API should work better, 
    pub fn node_id_for_entity(&mut self, entity_id: EntityId) -> accesskit::NodeId {
        *self
            .entity_ids
            .entry(entity_id)
            .or_insert_with(|| accesskit::NodeId(util::post_inc(&mut self.next_node_id)))
    }

    /// Get the nearest ancestor node ID for a given focus id
    pub fn nearest_node_id(
        &mut self,
        focus_id: FocusId,
        dispatch_tree: &DispatchTree,
    ) -> Option<accesskit::NodeId> {
        let mut focus_path = dispatch_tree.focus_path(focus_id);
        focus_path.reverse();
        for focus_id in focus_path.into_iter() {
            if let Some(node_id) = self.focus_ids.get(&focus_id) {
                return Some(*node_id);
            }
        }

        None
    }

    pub fn push_node(&mut self, node_id: accesskit::NodeId) {
        debug_assert!(self.current_nodes.get(&node_id).is_some());
        self.node_stack.push(node_id);
    }

    pub fn pop_node(&mut self) {
        if let Some(child_id) = self.node_stack.pop() {
            if let Some(parent_id) = self.node_stack.last_mut() {
                let parent_node = self
                    .current_nodes
                    .get_mut(parent_id)
                    .expect("Nodes must be inserted before using");
                parent_node.push_child(child_id);
            }
        }
    }

    pub fn insert_node(&mut self, node_id: accesskit::NodeId, node: accesskit::Node) {
        self.current_nodes.insert(node_id, node);
    }

    pub fn updated_nodes(&mut self) -> Vec<(accesskit::NodeId, accesskit::Node)> {
        debug_assert!(self.node_stack.is_empty());

        let mut changed_ids = Vec::new();

        for id in self.current_nodes.keys() {
            if let Some(node) = self.last_nodes.get(id) {
                if self.current_nodes[id] != *node {
                    changed_ids.push((*id, self.current_nodes[id].clone()));
                }
            } else {
                changed_ids.push((*id, self.current_nodes[id].clone()));
            }
        }
        mem::swap(&mut self.current_nodes, &mut self.last_nodes);
        self.current_nodes.clear();

        changed_ids
    }
}
