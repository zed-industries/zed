//! Accessibility support, provided by AccessKit
//!
//! The rough data flow is as follows:
//! - An [`Element`] can optionally provide [`a11y_role()`] and
//!   [`write_a11y_info()`] implementations.
//! - When rendering, we maintain a stack of nodes, and use this to derive the [`NodeId`]
//!
//! in [`Drawable::prepaint`], we maintain a stack of nodes
//!
//! [`Element`]: crate::Element
//! [`a11y_role()`]: crate::Element::a11y_role
//! [`write_a11y_info()`]: crate::Element::write_a11y_info
//! [`NodeId`]: accesskit::NodeId
//! [`Drawable::prepaint`]: crate::Drawable::prepaint

use crate::{App, Bounds, FocusId, Pixels, Window};
use collections::{FxHashMap, FxHashSet};
use smallvec::SmallVec;

/// The fixed AccessKit node ID used for the root of every window's a11y tree.
pub(crate) const ROOT_NODE_ID: accesskit::NodeId = accesskit::NodeId(0);

/// An accessibility action request, stripped of internal identifiers.
///
/// This is the GPUI-facing view of an AccessKit `ActionRequest`. Element
/// handlers receive this instead of the raw request so they don't need to
/// know about `NodeId` or `TreeId`.
pub struct A11yActionRequest {
    /// The action the screen reader is asking the element to perform.
    pub action: accesskit::Action,
    /// Optional payload for the action (e.g. a numeric value for `SetValue`).
    pub data: Option<accesskit::ActionData>,
}

impl A11yActionRequest {
    pub(crate) fn from_accesskit(request: &accesskit::ActionRequest) -> Self {
        Self {
            action: request.action,
            data: request.data.clone(),
        }
    }
}

/// A listener for an accessibility action on a specific node.
pub(crate) type A11yActionListener =
    Box<dyn FnMut(&A11yActionRequest, &mut Window, &mut App) + 'static>;

/// Per-window accessibility state.
///
/// Manages the AccessKit tree that is built each frame and the mappings
/// needed to dispatch incoming action requests back to the right elements.
pub(crate) struct A11y {
    pub(crate) active: bool,
    pub(crate) nodes: A11yNodeBuilder,
    pub(crate) focus_ids: FxHashMap<accesskit::NodeId, FocusId>,
    pub(crate) node_bounds: FxHashMap<accesskit::NodeId, Bounds<Pixels>>,
    pub(crate) action_listeners:
        FxHashMap<accesskit::NodeId, Vec<(accesskit::Action, A11yActionListener)>>,
}

impl A11y {
    pub(crate) fn new() -> Self {
        Self {
            active: false,
            nodes: A11yNodeBuilder::new(),
            focus_ids: FxHashMap::default(),
            node_bounds: FxHashMap::default(),
            action_listeners: FxHashMap::default(),
        }
    }

    /// Clear per-frame state and push the root node to start a new frame.
    pub(crate) fn begin_frame(&mut self) {
        self.focus_ids.clear();
        self.node_bounds.clear();
        self.action_listeners.clear();
        self.nodes.begin_frame();
    }

    /// Finalize the tree and produce a `TreeUpdate` for the platform adapter.
    pub(crate) fn end_frame(&mut self) -> accesskit::TreeUpdate {
        self.nodes.finalize()
    }
}

pub(crate) struct A11yNodeBuilder {
    ids_stack: SmallVec<[accesskit::NodeId; 16]>,
    nodes_stack: SmallVec<[accesskit::Node; 16]>,
    /// This is the exact type required by accesskit, so we can't just make it a
    /// `HashMap<NodeId, Node>` to remove the need for `seen_ids`
    all_nodes: Vec<(accesskit::NodeId, accesskit::Node)>,
    seen_ids: FxHashSet<accesskit::NodeId>,
    focus: accesskit::NodeId,
    #[cfg(debug_assertions)]
    has_set_focus: bool,
}

impl A11yNodeBuilder {
    fn new() -> Self {
        Self {
            ids_stack: SmallVec::new(),
            nodes_stack: SmallVec::new(),
            all_nodes: Vec::new(),
            seen_ids: FxHashSet::default(),
            focus: ROOT_NODE_ID,
            #[cfg(debug_assertions)]
            has_set_focus: false,
        }
    }

    /// Push a new node onto the stack. It becomes a child of the current
    /// top-of-stack node.
    ///
    /// Returns `true` if the node was successfully pushed.
    pub(crate) fn push(&mut self, id: accesskit::NodeId, node: accesskit::Node) -> bool {
        debug_assert!(!self.ids_stack.is_empty(), "push called before push_root");

        if !self.seen_ids.insert(id) {
            debug_assert!(
                false,
                "Duplicate a11y node id: {id:?}. In a release build, this node would be silently discarded from the a11y tree."
            );
            // We need to return `false` here because inserting a duplicate
            // node will cause a panic in accesskit
            return false;
        }

        if let Some(parent) = self.nodes_stack.last_mut() {
            parent.push_child(id);
        }
        self.ids_stack.push(id);
        self.nodes_stack.push(node);
        true
    }

    /// Pop the current node off the stack and finalize it into the all_nodes
    /// list.
    pub(crate) fn pop(&mut self) {
        debug_assert!(self.ids_stack.len() > 1, "pop would remove the root node");

        if let (Some(id), Some(node)) = (self.ids_stack.pop(), self.nodes_stack.pop()) {
            self.all_nodes.push((id, node));
        }
    }

    /// Push the root node to start a new frame.
    fn begin_frame(&mut self) {
        self.all_nodes.clear();
        self.ids_stack.clear();
        self.nodes_stack.clear();
        self.seen_ids.clear();
        #[cfg(debug_assertions)]
        {
            self.has_set_focus = false;
        }
        let root_node = accesskit::Node::new(accesskit::Role::Window);

        self.ids_stack.push(ROOT_NODE_ID);
        self.nodes_stack.push(root_node);
        self.focus = ROOT_NODE_ID;
    }

    /// Set the focused node for this frame.
    pub(crate) fn set_focus(&mut self, id: accesskit::NodeId) {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                !self.has_set_focus,
                "set_focus called more than once in a single frame"
            );
            self.has_set_focus = true;
        }
        self.focus = id;
    }

    fn finalize(&mut self) -> accesskit::TreeUpdate {
        // Stack should contain only the root node
        debug_assert_eq!(self.ids_stack.len(), 1);
        debug_assert_eq!(self.ids_stack[0], ROOT_NODE_ID);

        // Pop remaining nodes (should just be the root).
        while !self.ids_stack.is_empty() {
            if let (Some(id), Some(node)) = (self.ids_stack.pop(), self.nodes_stack.pop()) {
                self.all_nodes.push((id, node));
            }
        }

        let nodes = std::mem::take(&mut self.all_nodes);
        accesskit::TreeUpdate {
            nodes,
            tree: Some(accesskit::Tree::new(ROOT_NODE_ID)),
            tree_id: accesskit::TreeId::ROOT,
            focus: self.focus,
        }
    }
}
