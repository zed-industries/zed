//! Node types and management for git graph.

use super::path::Path;
use super::point::Point;
use super::types::{ID_KEY, PARENTS_KEY};
use indexmap::IndexMap;
use serde_json::Value;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

/// Node represents raw git commit information (flexible JSON-like structure).
pub type Node = IndexMap<String, Value>;

/// Helper trait for Node operations.
pub trait NodeExt {
    /// Get the commit ID.
    fn get_id(&self) -> String;
    /// Get parent commit IDs.
    fn get_parents(&self) -> Vec<String>;
}

impl NodeExt for Node {
    fn get_id(&self) -> String {
        self.get(ID_KEY)
            .and_then(|v| v.as_str())
            .expect("id property must be a string")
            .to_string()
    }

    fn get_parents(&self) -> Vec<String> {
        self.get(PARENTS_KEY)
            .and_then(|v| v.as_array())
            .expect("parents property must be an array")
            .iter()
            .map(|v| v.as_str().expect("parent must be a string").to_string())
            .collect()
    }
}

/// Internal node representation for graph processing.
pub type InternalNode = Rc<RefCell<InternalNodeInner>>;

/// Inner struct for internal nodes.
#[derive(Debug)]
pub struct InternalNodeInner {
    pub initial_node: Option<Node>,
    pub id: String,
    pub idx: Rc<RefCell<i32>>,
    pub column: i32,
    pub color_idx: i32,
    pub first_of_branch: bool,
    pub parents: Vec<InternalNode>,
    pub children: Vec<InternalNode>,
    pub parents_paths: HashMap<String, Path>,
}

impl InternalNodeInner {
    /// Set the column for this node.
    pub fn set_column(&mut self, column: i32) {
        self.column = column;
    }

    /// Set the color index for this node.
    pub fn set_color(&mut self, color: i32) {
        self.color_idx = color;
    }

    /// Check if this node is an orphan (has no parents).
    pub fn is_orphan(&self) -> bool {
        self.parents.is_empty()
    }

    /// Check if this node is the first of a branch.
    pub fn is_first_of_branch(&self) -> bool {
        self.first_of_branch
    }

    /// Mark this node as the first of a branch.
    pub fn set_first_of_branch(&mut self) {
        self.first_of_branch = true;
    }

    /// Get the path to a parent node by parent ID.
    pub fn path_to_id(&mut self, parent_id: &str) -> &mut Path {
        self.parents_paths.entry(parent_id.to_string()).or_default()
    }

    /// Get the path to a parent node.
    pub fn path_to(&mut self, parent: &InternalNode) -> &mut Path {
        let parent_id = parent.borrow().id.clone();
        self.path_to_id(&parent_id)
    }

    /// Check if column is defined.
    pub fn column_defined(&self) -> bool {
        self.column != -1
    }

    /// Check if this is the first node in a branch.
    pub fn first_in_branch(&self) -> bool {
        for parent_node in &self.parents {
            let parent = parent_node.borrow();
            if !parent.column_defined() || parent.column == self.column {
                return false;
            }
        }
        true
    }

    /// Check if a path to a parent is a sub-branch.
    pub fn is_path_sub_branch(&mut self, parent: &InternalNode) -> bool {
        let path = self.path_to(parent);
        path.is_fork() && !self.is_first_of_branch()
    }

    /// Move this node left by the given number of columns.
    pub fn move_left(&mut self, nb: i32) {
        self.column -= nb;
        for child in &self.children {
            let mut child_borrow = child.borrow_mut();
            let parent_id = self.id.clone();
            if let Some(path) = child_borrow.parents_paths.get_mut(&parent_id) {
                if !path.is_empty() {
                    path.get_mut(-1).set_x(self.column);
                }
            }
        }
    }

    /// Move this node down to a new index.
    pub fn move_down(&self, idx: i32) {
        *self.idx.borrow_mut() = idx;
    }

    /// Count nodes merging back.
    pub fn nb_nodes_merging_back(&self, max_x: i32) -> i32 {
        if self.children.len() == 1 {
            return 0;
        }

        let mut nb_nodes_merging_back = 0;

        for child in &self.children {
            let child_borrow = child.borrow();
            let parent_id = self.id.clone();

            // Check path properties first
            let (should_count, is_sub_branch) = if let Some(path) = child_borrow.parents_paths.get(&parent_id) {
                if path.len() >= 2 {
                    let second_to_last_point = path.second_to_last();
                    let x_valid = self.column < second_to_last_point.get_x()
                        && second_to_last_point.get_x() < max_x;
                    let is_merge_to = path.is_merge_to();
                    (x_valid && !is_merge_to, path.is_fork() && !child_borrow.is_first_of_branch())
                } else {
                    (false, false)
                }
            } else {
                (false, false)
            };

            if should_count && !is_sub_branch {
                nb_nodes_merging_back += 1;
            }
        }
        nb_nodes_merging_back
    }
}

/// Create a new internal node.
///
/// # Examples
///
/// ```
/// use git_to_graph::node::new_node;
///
/// let node = new_node("abc123".to_string(), 0);
/// assert_eq!(node.borrow().id, "abc123");
/// ```
pub fn new_node(id: String, idx: i32) -> InternalNode {
    Rc::new(RefCell::new(InternalNodeInner {
        initial_node: None,
        id,
        idx: Rc::new(RefCell::new(idx)),
        column: -1,
        color_idx: 0,
        first_of_branch: false,
        parents: Vec::new(),
        children: Vec::new(),
        parents_paths: HashMap::new(),
    }))
}

/// Ordered set of internal nodes.
#[derive(Debug)]
pub struct InternalNodeSet {
    a: Vec<InternalNode>,
    m: HashSet<String>,
}

impl InternalNodeSet {
    /// Create a new internal node set.
    ///
    /// # Examples
    ///
    /// ```
    /// use git_to_graph::node::InternalNodeSet;
    ///
    /// let set = InternalNodeSet::new();
    /// ```
    pub fn new() -> Self {
        InternalNodeSet {
            a: Vec::new(),
            m: HashSet::new(),
        }
    }

    /// Get a node by ID.
    pub fn get(&self, key: &str) -> Option<InternalNode> {
        for n in &self.a {
            if n.borrow().id == key {
                return Some(n.clone());
            }
        }
        None
    }

    /// Add nodes to the set (prepend).
    pub fn add(&mut self, ins: &[InternalNode]) {
        let mut to_add = Vec::new();
        for in_node in ins {
            let id = in_node.borrow().id.clone();
            if !self.m.contains(&id) {
                to_add.push(in_node.clone());
                self.m.insert(id);
            }
        }
        // Prepend
        to_add.append(&mut self.a);
        self.a = to_add;
    }

    /// Remove a node from the set.
    pub fn remove(&mut self, in_node: &InternalNode) {
        let id = in_node.borrow().id.clone();
        self.a.retain(|n| n.borrow().id != id);
        self.m.remove(&id);
    }

    /// Get reference to the internal vector.
    pub fn nodes(&self) -> &Vec<InternalNode> {
        &self.a
    }
}

impl Default for InternalNodeSet {
    fn default() -> Self {
        Self::new()
    }
}

/// Tracks processed nodes for avoiding duplicate processing.
#[derive(Debug)]
pub struct ProcessedNodes {
    m: HashMap<String, HashMap<String, bool>>,
}

impl ProcessedNodes {
    /// Create a new processed nodes tracker.
    ///
    /// # Examples
    ///
    /// ```
    /// use git_to_graph::node::ProcessedNodes;
    ///
    /// let tracker = ProcessedNodes::new();
    /// ```
    pub fn new() -> Self {
        ProcessedNodes { m: HashMap::new() }
    }

    /// Check if a node has been processed.
    pub fn has_node(&self, node_id: &str) -> bool {
        self.m.contains_key(node_id)
    }

    /// Check if a child has been processed for a node.
    pub fn has_child(&self, node_id: &str, child_id: &str) -> bool {
        self.m.get(node_id).is_some_and(|children| children.contains_key(child_id))
    }

    /// Mark a child as processed for a node.
    pub fn set(&mut self, node_id: String, child_id: String) {
        self.m.entry(node_id).or_default().insert(child_id, true);
    }
}

impl Default for ProcessedNodes {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_node() {
        let node = new_node("test123".to_string(), 5);
        let borrowed = node.borrow();
        assert_eq!(borrowed.id, "test123");
        assert_eq!(*borrowed.idx.borrow(), 5);
        assert_eq!(borrowed.column, -1);
    }

    #[test]
    fn test_internal_node_set() {
        let mut set = InternalNodeSet::new();
        let node1 = new_node("node1".to_string(), 0);
        let node2 = new_node("node2".to_string(), 1);

        set.add(&[node1.clone(), node2]);
        assert!(set.get("node1").is_some());
        assert!(set.get("node2").is_some());
        assert!(set.get("node3").is_none());

        set.remove(&node1);
        assert!(set.get("node1").is_none());
    }

    #[test]
    fn test_processed_nodes() {
        let mut tracker = ProcessedNodes::new();
        tracker.set("node1".to_string(), "child1".to_string());

        assert!(tracker.has_node("node1"));
        assert!(tracker.has_child("node1", "child1"));
        assert!(!tracker.has_child("node1", "child2"));
        assert!(!tracker.has_node("node2"));
    }
}
