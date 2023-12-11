use crate::{
    Action, ActionRegistry, DispatchPhase, FocusId, KeyBinding, KeyContext, KeyMatch, Keymap,
    Keystroke, KeystrokeMatcher, WindowContext,
};
use collections::HashMap;
use parking_lot::Mutex;
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    rc::Rc,
    sync::Arc,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct DispatchNodeId(usize);

pub(crate) struct DispatchTree {
    node_stack: Vec<DispatchNodeId>,
    pub(crate) context_stack: Vec<KeyContext>,
    nodes: Vec<DispatchNode>,
    focusable_node_ids: HashMap<FocusId, DispatchNodeId>,
    keystroke_matchers: HashMap<SmallVec<[KeyContext; 4]>, KeystrokeMatcher>,
    keymap: Arc<Mutex<Keymap>>,
    action_registry: Rc<ActionRegistry>,
}

#[derive(Default)]
pub(crate) struct DispatchNode {
    pub key_listeners: SmallVec<[KeyListener; 2]>,
    pub action_listeners: SmallVec<[DispatchActionListener; 16]>,
    pub context: Option<KeyContext>,
    parent: Option<DispatchNodeId>,
}

type KeyListener = Rc<dyn Fn(&dyn Any, DispatchPhase, &mut WindowContext)>;

#[derive(Clone)]
pub(crate) struct DispatchActionListener {
    pub(crate) action_type: TypeId,
    pub(crate) listener: Rc<dyn Fn(&dyn Any, DispatchPhase, &mut WindowContext)>,
}

impl DispatchTree {
    pub fn new(keymap: Arc<Mutex<Keymap>>, action_registry: Rc<ActionRegistry>) -> Self {
        Self {
            node_stack: Vec::new(),
            context_stack: Vec::new(),
            nodes: Vec::new(),
            focusable_node_ids: HashMap::default(),
            keystroke_matchers: HashMap::default(),
            keymap,
            action_registry,
        }
    }

    pub fn clear(&mut self) {
        self.node_stack.clear();
        self.nodes.clear();
        self.context_stack.clear();
        self.focusable_node_ids.clear();
        self.keystroke_matchers.clear();
    }

    pub fn push_node(&mut self, context: Option<KeyContext>) {
        let parent = self.node_stack.last().copied();
        let node_id = DispatchNodeId(self.nodes.len());
        self.nodes.push(DispatchNode {
            parent,
            ..Default::default()
        });
        self.node_stack.push(node_id);
        if let Some(context) = context {
            self.active_node().context = Some(context.clone());
            self.context_stack.push(context);
        }
    }

    pub fn pop_node(&mut self) {
        let node_id = self.node_stack.pop().unwrap();
        if self.nodes[node_id.0].context.is_some() {
            self.context_stack.pop();
        }
    }

    pub fn clear_pending_keystrokes(&mut self) {
        self.keystroke_matchers.clear();
    }

    /// Preserve keystroke matchers from previous frames to support multi-stroke
    /// bindings across multiple frames.
    pub fn preserve_pending_keystrokes(&mut self, old_tree: &mut Self, focus_id: Option<FocusId>) {
        if let Some(node_id) = focus_id.and_then(|focus_id| self.focusable_node_id(focus_id)) {
            let dispatch_path = self.dispatch_path(node_id);

            self.context_stack.clear();
            for node_id in dispatch_path {
                let node = self.node(node_id);
                if let Some(context) = node.context.clone() {
                    self.context_stack.push(context);
                }

                if let Some((context_stack, matcher)) = old_tree
                    .keystroke_matchers
                    .remove_entry(self.context_stack.as_slice())
                {
                    self.keystroke_matchers.insert(context_stack, matcher);
                }
            }
        }
    }

    pub fn on_key_event(&mut self, listener: KeyListener) {
        self.active_node().key_listeners.push(listener);
    }

    pub fn on_action(
        &mut self,
        action_type: TypeId,
        listener: Rc<dyn Fn(&dyn Any, DispatchPhase, &mut WindowContext)>,
    ) {
        self.active_node()
            .action_listeners
            .push(DispatchActionListener {
                action_type,
                listener,
            });
    }

    pub fn make_focusable(&mut self, focus_id: FocusId) {
        self.focusable_node_ids
            .insert(focus_id, self.active_node_id());
    }

    pub fn focus_contains(&self, parent: FocusId, child: FocusId) -> bool {
        if parent == child {
            return true;
        }

        if let Some(parent_node_id) = self.focusable_node_ids.get(&parent) {
            let mut current_node_id = self.focusable_node_ids.get(&child).copied();
            while let Some(node_id) = current_node_id {
                if node_id == *parent_node_id {
                    return true;
                }
                current_node_id = self.nodes[node_id.0].parent;
            }
        }
        false
    }

    pub fn available_actions(&self, target: DispatchNodeId) -> Vec<Box<dyn Action>> {
        let mut actions = Vec::<Box<dyn Action>>::new();
        for node_id in self.dispatch_path(target) {
            let node = &self.nodes[node_id.0];
            for DispatchActionListener { action_type, .. } in &node.action_listeners {
                if let Err(ix) = actions.binary_search_by_key(action_type, |a| a.as_any().type_id())
                {
                    // Intentionally silence these errors without logging.
                    // If an action cannot be built by default, it's not available.
                    let action = self.action_registry.build_action_type(action_type).ok();
                    if let Some(action) = action {
                        actions.insert(ix, action);
                    }
                }
            }
        }
        actions
    }

    pub fn is_action_available(&self, action: &dyn Action, target: DispatchNodeId) -> bool {
        for node_id in self.dispatch_path(target) {
            let node = &self.nodes[node_id.0];
            if node
                .action_listeners
                .iter()
                .any(|listener| listener.action_type == action.as_any().type_id())
            {
                return true;
            }
        }
        false
    }

    pub fn bindings_for_action(
        &self,
        action: &dyn Action,
        context_stack: &Vec<KeyContext>,
    ) -> Vec<KeyBinding> {
        self.keymap
            .lock()
            .bindings_for_action(action.type_id())
            .filter(|candidate| {
                if !candidate.action.partial_eq(action) {
                    return false;
                }
                for i in 1..context_stack.len() {
                    if candidate.matches_context(&context_stack[0..=i]) {
                        return true;
                    }
                }
                return false;
            })
            .cloned()
            .collect()
    }

    pub fn dispatch_key(
        &mut self,
        keystroke: &Keystroke,
        context: &[KeyContext],
    ) -> Vec<Box<dyn Action>> {
        if !self.keystroke_matchers.contains_key(context) {
            let keystroke_contexts = context.iter().cloned().collect();
            self.keystroke_matchers.insert(
                keystroke_contexts,
                KeystrokeMatcher::new(self.keymap.clone()),
            );
        }

        let keystroke_matcher = self.keystroke_matchers.get_mut(context).unwrap();
        if let KeyMatch::Some(actions) = keystroke_matcher.match_keystroke(keystroke, context) {
            // Clear all pending keystrokes when an action has been found.
            for keystroke_matcher in self.keystroke_matchers.values_mut() {
                keystroke_matcher.clear_pending();
            }

            actions
        } else {
            vec![]
        }
    }

    pub fn has_pending_keystrokes(&self) -> bool {
        self.keystroke_matchers
            .iter()
            .any(|(_, matcher)| matcher.has_pending_keystrokes())
    }

    pub fn dispatch_path(&self, target: DispatchNodeId) -> SmallVec<[DispatchNodeId; 32]> {
        let mut dispatch_path: SmallVec<[DispatchNodeId; 32]> = SmallVec::new();
        let mut current_node_id = Some(target);
        while let Some(node_id) = current_node_id {
            dispatch_path.push(node_id);
            current_node_id = self.nodes[node_id.0].parent;
        }
        dispatch_path.reverse(); // Reverse the path so it goes from the root to the focused node.
        dispatch_path
    }

    pub fn node(&self, node_id: DispatchNodeId) -> &DispatchNode {
        &self.nodes[node_id.0]
    }

    fn active_node(&mut self) -> &mut DispatchNode {
        let active_node_id = self.active_node_id();
        &mut self.nodes[active_node_id.0]
    }

    pub fn focusable_node_id(&self, target: FocusId) -> Option<DispatchNodeId> {
        self.focusable_node_ids.get(&target).copied()
    }

    pub fn root_node_id(&self) -> DispatchNodeId {
        debug_assert!(!self.nodes.is_empty());
        DispatchNodeId(0)
    }

    fn active_node_id(&self) -> DispatchNodeId {
        *self.node_stack.last().unwrap()
    }
}
