use crate::{
    Action, DispatchPhase, FocusId, KeyBindingContext, KeyDownEvent, KeyMatch, Keymap,
    KeystrokeMatcher, WindowContext,
};
use collections::HashMap;
use parking_lot::Mutex;
use smallvec::SmallVec;
use std::{any::Any, sync::Arc};

// trait KeyListener -> FnMut(&E, &mut V, &mut ViewContext<V>)
type AnyKeyListener = Box<dyn Fn(&dyn Any, DispatchPhase, &mut WindowContext)>;
type AnyActionListener = Box<dyn Fn(&dyn Any, DispatchPhase, &mut WindowContext)>;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct DispatchNodeId(usize);

pub struct DispatchTree {
    node_stack: Vec<DispatchNodeId>,
    context_stack: Vec<KeyBindingContext>,
    nodes: Vec<DispatchNode>,
    focused: Option<FocusId>,
    focusable_node_ids: HashMap<FocusId, DispatchNodeId>,
    keystroke_matchers: HashMap<SmallVec<[KeyBindingContext; 4]>, KeystrokeMatcher>,
    keymap: Arc<Mutex<Keymap>>,
}

#[derive(Default)]
pub struct DispatchNode {
    key_listeners: SmallVec<[AnyKeyListener; 2]>,
    action_listeners: SmallVec<[AnyActionListener; 16]>,
    context: KeyBindingContext,
    parent: Option<DispatchNodeId>,
}

impl DispatchTree {
    pub fn clear(&mut self) {
        self.node_stack.clear();
        self.nodes.clear();
    }

    pub fn push_node(&mut self, context: Option<KeyBindingContext>, old_tree: &mut Self) {
        let parent = self.node_stack.last().copied();
        let node_id = DispatchNodeId(self.nodes.len());
        self.nodes.push(DispatchNode {
            parent,
            ..Default::default()
        });
        self.node_stack.push(node_id);
        if let Some(context) = context {
            self.context_stack.push(context);
            if let Some((context_stack, matcher)) = old_tree
                .keystroke_matchers
                .remove_entry(self.context_stack.as_slice())
            {
                self.keystroke_matchers.insert(context_stack, matcher);
            }
        }
    }

    pub fn pop_node(&mut self) -> DispatchNodeId {
        self.node_stack.pop().unwrap()
    }

    pub fn on_key_event(&mut self, listener: AnyKeyListener) {
        self.active_node().key_listeners.push(listener);
    }

    pub fn on_action(&mut self, listener: AnyActionListener) {
        self.active_node().action_listeners.push(listener);
    }

    pub fn make_focusable(&mut self, focus_id: FocusId) {
        self.focusable_node_ids
            .insert(focus_id, self.active_node_id());
    }

    pub fn set_focus(&mut self, focus_id: Option<FocusId>) {
        self.focused = focus_id;
    }

    pub fn active_node(&mut self) -> &mut DispatchNode {
        let node_id = self.active_node_id();
        &mut self.nodes[node_id.0]
    }

    fn active_node_id(&self) -> DispatchNodeId {
        *self.node_stack.last().unwrap()
    }

    /// Returns the DispatchNodeIds from the root of the tree to the given target node id.
    fn dispatch_path(&self, target: DispatchNodeId) -> SmallVec<[DispatchNodeId; 32]> {
        let mut dispatch_path: SmallVec<[DispatchNodeId; 32]> = SmallVec::new();
        let mut current_node_id = Some(target);
        while let Some(node_id) = current_node_id {
            dispatch_path.push(node_id);
            current_node_id = self.nodes[node_id.0].parent;
        }
        dispatch_path.reverse(); // Reverse the path so it goes from the root to the focused node.
        dispatch_path
    }

    pub fn dispatch_key(&mut self, event: &dyn Any, cx: &mut WindowContext) {
        if let Some(focused_node_id) = self
            .focused
            .and_then(|focus_id| self.focusable_node_ids.get(&focus_id))
            .copied()
        {
            self.dispatch_key_on_node(focused_node_id, event, cx);
        }
    }

    fn dispatch_key_on_node(
        &mut self,
        node_id: DispatchNodeId,
        event: &dyn Any,
        cx: &mut WindowContext,
    ) {
        let dispatch_path = self.dispatch_path(node_id);

        // Capture phase
        self.context_stack.clear();
        cx.propagate_event = true;
        for node_id in &dispatch_path {
            let node = &self.nodes[node_id.0];
            if !node.context.is_empty() {
                self.context_stack.push(node.context.clone());
            }

            for key_listener in &node.key_listeners {
                key_listener(event, DispatchPhase::Capture, cx);
                if !cx.propagate_event {
                    return;
                }
            }
        }

        // Bubble phase
        for node_id in dispatch_path.iter().rev() {
            let node = &self.nodes[node_id.0];

            // Handle low level key events
            for key_listener in &node.key_listeners {
                key_listener(event, DispatchPhase::Bubble, cx);
                if !cx.propagate_event {
                    return;
                }
            }

            // Match keystrokes
            if !node.context.is_empty() {
                if let Some(key_down_event) = event.downcast_ref::<KeyDownEvent>() {
                    if !self
                        .keystroke_matchers
                        .contains_key(self.context_stack.as_slice())
                    {
                        let keystroke_contexts = self.context_stack.iter().cloned().collect();
                        self.keystroke_matchers.insert(
                            keystroke_contexts,
                            KeystrokeMatcher::new(self.keymap.clone()),
                        );
                    }

                    if let Some(keystroke_matcher) = self
                        .keystroke_matchers
                        .get_mut(self.context_stack.as_slice())
                    {
                        if let KeyMatch::Some(action) = keystroke_matcher.match_keystroke(
                            &key_down_event.keystroke,
                            self.context_stack.as_slice(),
                        ) {
                            self.dispatch_action_on_node(*node_id, action, cx);
                            if !cx.propagate_event {
                                return;
                            }
                        }
                    }
                }

                self.context_stack.pop();
            }
        }
    }

    pub fn dispatch_action(&self, action: Box<dyn Action>, cx: &mut WindowContext) {
        if let Some(focused_node_id) = self
            .focused
            .and_then(|focus_id| self.focusable_node_ids.get(&focus_id))
            .copied()
        {
            self.dispatch_action_on_node(focused_node_id, action, cx);
        }
    }

    fn dispatch_action_on_node(
        &self,
        node_id: DispatchNodeId,
        action: Box<dyn Action>,
        cx: &mut WindowContext,
    ) {
        let dispatch_path = self.dispatch_path(node_id);

        // Capture phase
        for node_id in &dispatch_path {
            let node = &self.nodes[node_id.0];
            for action_listener in &node.action_listeners {
                action_listener(&action, DispatchPhase::Capture, cx);
                if !cx.propagate_event {
                    return;
                }
            }
        }

        // Bubble phase
        for node_id in dispatch_path.iter().rev() {
            let node = &self.nodes[node_id.0];
            for action_listener in &node.action_listeners {
                cx.propagate_event = false; // Actions stop propagation by default during the bubble phase
                action_listener(&action, DispatchPhase::Capture, cx);
                if !cx.propagate_event {
                    return;
                }
            }
        }
    }
}
