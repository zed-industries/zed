use crate::{
    build_action_from_type, Action, Bounds, DispatchPhase, Element, FocusEvent, FocusHandle,
    FocusId, KeyContext, KeyDownEvent, KeyMatch, Keymap, KeystrokeMatcher, MouseDownEvent, Pixels,
    Style, StyleRefinement, ViewContext, WindowContext,
};
use collections::HashMap;
use parking_lot::Mutex;
use refineable::Refineable;
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    sync::Arc,
};
use util::ResultExt;

type KeyListener = Box<dyn Fn(&dyn Any, DispatchPhase, &mut WindowContext)>;
pub type FocusListeners<V> = SmallVec<[FocusListener<V>; 2]>;
pub type FocusListener<V> =
    Box<dyn Fn(&mut V, &FocusHandle, &FocusEvent, &mut ViewContext<V>) + 'static>;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct DispatchNodeId(usize);

pub struct KeyDispatcher {
    node_stack: Vec<DispatchNodeId>,
    context_stack: Vec<KeyContext>,
    nodes: Vec<DispatchNode>,
    focusable_node_ids: HashMap<FocusId, DispatchNodeId>,
    keystroke_matchers: HashMap<SmallVec<[KeyContext; 4]>, KeystrokeMatcher>,
    keymap: Arc<Mutex<Keymap>>,
}

#[derive(Default)]
pub struct DispatchNode {
    key_listeners: SmallVec<[KeyListener; 2]>,
    action_listeners: SmallVec<[ActionListener; 16]>,
    context: KeyContext,
    parent: Option<DispatchNodeId>,
}

struct ActionListener {
    action_type: TypeId,
    listener: Box<dyn Fn(&dyn Any, DispatchPhase, &mut WindowContext)>,
}

impl KeyDispatcher {
    pub fn new(keymap: Arc<Mutex<Keymap>>) -> Self {
        Self {
            node_stack: Vec::new(),
            context_stack: Vec::new(),
            nodes: Vec::new(),
            focusable_node_ids: HashMap::default(),
            keystroke_matchers: HashMap::default(),
            keymap,
        }
    }

    pub fn clear(&mut self) {
        self.node_stack.clear();
        self.nodes.clear();
    }

    pub fn push_node(&mut self, context: KeyContext, old_dispatcher: &mut Self) {
        let parent = self.node_stack.last().copied();
        let node_id = DispatchNodeId(self.nodes.len());
        self.nodes.push(DispatchNode {
            parent,
            ..Default::default()
        });
        self.node_stack.push(node_id);
        if !context.is_empty() {
            self.active_node().context = context.clone();
            self.context_stack.push(context);
            if let Some((context_stack, matcher)) = old_dispatcher
                .keystroke_matchers
                .remove_entry(self.context_stack.as_slice())
            {
                self.keystroke_matchers.insert(context_stack, matcher);
            }
        }
    }

    pub fn pop_node(&mut self) {
        let node_id = self.node_stack.pop().unwrap();
        if !self.nodes[node_id.0].context.is_empty() {
            self.context_stack.pop();
        }
    }

    pub fn on_key_event(&mut self, listener: KeyListener) {
        self.active_node().key_listeners.push(listener);
    }

    pub fn on_action(
        &mut self,
        action_type: TypeId,
        listener: Box<dyn Fn(&dyn Any, DispatchPhase, &mut WindowContext)>,
    ) {
        self.active_node().action_listeners.push(ActionListener {
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

    pub fn available_actions(&self, target: FocusId) -> Vec<Box<dyn Action>> {
        let mut actions = Vec::new();
        if let Some(node) = self.focusable_node_ids.get(&target) {
            for node_id in self.dispatch_path(*node) {
                let node = &self.nodes[node_id.0];
                for ActionListener { action_type, .. } in &node.action_listeners {
                    actions.extend(build_action_from_type(action_type).log_err());
                }
            }
        }
        actions
    }

    pub fn dispatch_key(&mut self, target: FocusId, event: &dyn Any, cx: &mut WindowContext) {
        if let Some(target_node_id) = self.focusable_node_ids.get(&target).copied() {
            self.dispatch_key_on_node(target_node_id, event, cx);
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

                    let keystroke_matcher = self
                        .keystroke_matchers
                        .get_mut(self.context_stack.as_slice())
                        .unwrap();
                    if let KeyMatch::Some(action) = keystroke_matcher
                        .match_keystroke(&key_down_event.keystroke, self.context_stack.as_slice())
                    {
                        self.dispatch_action_on_node(*node_id, action, cx);
                        if !cx.propagate_event {
                            return;
                        }
                    }
                }

                self.context_stack.pop();
            }
        }
    }

    pub fn dispatch_action(
        &self,
        target: FocusId,
        action: Box<dyn Action>,
        cx: &mut WindowContext,
    ) {
        if let Some(target_node_id) = self.focusable_node_ids.get(&target).copied() {
            self.dispatch_action_on_node(target_node_id, action, cx);
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
            for ActionListener {
                action_type,
                listener,
            } in &node.action_listeners
            {
                let any_action = action.as_any();
                if *action_type == any_action.type_id() {
                    listener(any_action, DispatchPhase::Capture, cx);
                    if !cx.propagate_event {
                        return;
                    }
                }
            }
        }

        // Bubble phase
        for node_id in dispatch_path.iter().rev() {
            let node = &self.nodes[node_id.0];
            for ActionListener {
                action_type,
                listener,
            } in &node.action_listeners
            {
                let any_action = action.as_any();
                if *action_type == any_action.type_id() {
                    cx.propagate_event = false; // Actions stop propagation by default during the bubble phase
                    listener(any_action, DispatchPhase::Bubble, cx);
                    if !cx.propagate_event {
                        return;
                    }
                }
            }
        }
    }

    fn active_node(&mut self) -> &mut DispatchNode {
        let active_node_id = self.active_node_id();
        &mut self.nodes[active_node_id.0]
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
}

pub trait KeyDispatch<V: 'static>: 'static {
    fn as_focusable(&self) -> Option<&FocusableKeyDispatch<V>>;
    fn as_focusable_mut(&mut self) -> Option<&mut FocusableKeyDispatch<V>>;
    fn key_context(&self) -> &KeyContext;
    fn key_context_mut(&mut self) -> &mut KeyContext;

    fn initialize<R>(
        &mut self,
        focus_handle: Option<FocusHandle>,
        cx: &mut ViewContext<V>,
        f: impl FnOnce(Option<FocusHandle>, &mut ViewContext<V>) -> R,
    ) -> R {
        if let Some(focusable) = self.as_focusable_mut() {
            let focus_handle = focusable
                .focus_handle
                .get_or_insert_with(|| focus_handle.unwrap_or_else(|| cx.focus_handle()))
                .clone();
            for listener in focusable.focus_listeners.drain(..) {
                let focus_handle = focus_handle.clone();
                cx.on_focus_changed(move |view, event, cx| {
                    listener(view, &focus_handle, event, cx)
                });
            }

            cx.with_key_dispatch(self.key_context().clone(), Some(focus_handle), f)
        } else {
            f(None, cx)
        }
    }

    fn refine_style(&self, style: &mut Style, cx: &WindowContext) {
        if let Some(focusable) = self.as_focusable() {
            let focus_handle = focusable
                .focus_handle
                .as_ref()
                .expect("must call initialize before refine_style");
            if focus_handle.contains_focused(cx) {
                style.refine(&focusable.focus_in_style);
            }

            if focus_handle.within_focused(cx) {
                style.refine(&focusable.in_focus_style);
            }

            if focus_handle.is_focused(cx) {
                style.refine(&focusable.focus_style);
            }
        }
    }

    fn paint(&self, bounds: Bounds<Pixels>, cx: &mut WindowContext) {
        if let Some(focusable) = self.as_focusable() {
            let focus_handle = focusable
                .focus_handle
                .clone()
                .expect("must call initialize before paint");
            cx.on_mouse_event(move |event: &MouseDownEvent, phase, cx| {
                if phase == DispatchPhase::Bubble && bounds.contains_point(&event.position) {
                    if !cx.default_prevented() {
                        cx.focus(&focus_handle);
                        cx.prevent_default();
                    }
                }
            })
        }
    }
}

pub struct FocusableKeyDispatch<V> {
    pub key_context: KeyContext,
    pub focus_handle: Option<FocusHandle>,
    pub focus_listeners: FocusListeners<V>,
    pub focus_style: StyleRefinement,
    pub focus_in_style: StyleRefinement,
    pub in_focus_style: StyleRefinement,
}

impl<V> FocusableKeyDispatch<V> {
    pub fn new() -> Self {
        Self {
            key_context: KeyContext::default(),
            focus_handle: None,
            focus_listeners: FocusListeners::default(),
            focus_style: StyleRefinement::default(),
            focus_in_style: StyleRefinement::default(),
            in_focus_style: StyleRefinement::default(),
        }
    }

    pub fn tracked(handle: &FocusHandle) -> Self {
        Self {
            key_context: KeyContext::default(),
            focus_handle: Some(handle.clone()),
            focus_listeners: FocusListeners::default(),
            focus_style: StyleRefinement::default(),
            focus_in_style: StyleRefinement::default(),
            in_focus_style: StyleRefinement::default(),
        }
    }
}

impl<V: 'static> KeyDispatch<V> for FocusableKeyDispatch<V> {
    fn as_focusable(&self) -> Option<&FocusableKeyDispatch<V>> {
        Some(self)
    }

    fn as_focusable_mut(&mut self) -> Option<&mut FocusableKeyDispatch<V>> {
        Some(self)
    }

    fn key_context(&self) -> &KeyContext {
        &self.key_context
    }

    fn key_context_mut(&mut self) -> &mut KeyContext {
        &mut self.key_context
    }
}

impl<V> From<FocusHandle> for FocusableKeyDispatch<V> {
    fn from(value: FocusHandle) -> Self {
        Self {
            key_context: KeyContext::default(),
            focus_handle: Some(value),
            focus_listeners: FocusListeners::default(),
            focus_style: StyleRefinement::default(),
            focus_in_style: StyleRefinement::default(),
            in_focus_style: StyleRefinement::default(),
        }
    }
}

#[derive(Default)]
pub struct NonFocusableKeyDispatch {
    pub(crate) key_context: KeyContext,
}

impl<V: 'static> KeyDispatch<V> for NonFocusableKeyDispatch {
    fn as_focusable(&self) -> Option<&FocusableKeyDispatch<V>> {
        None
    }

    fn as_focusable_mut(&mut self) -> Option<&mut FocusableKeyDispatch<V>> {
        None
    }

    fn key_context(&self) -> &KeyContext {
        &self.key_context
    }

    fn key_context_mut(&mut self) -> &mut KeyContext {
        &mut self.key_context
    }
}

pub trait Focusable<V: 'static>: Element<V> {
    fn focus_listeners(&mut self) -> &mut FocusListeners<V>;
    fn set_focus_style(&mut self, style: StyleRefinement);
    fn set_focus_in_style(&mut self, style: StyleRefinement);
    fn set_in_focus_style(&mut self, style: StyleRefinement);

    fn focus(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self
    where
        Self: Sized,
    {
        self.set_focus_style(f(StyleRefinement::default()));
        self
    }

    fn focus_in(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self
    where
        Self: Sized,
    {
        self.set_focus_in_style(f(StyleRefinement::default()));
        self
    }

    fn in_focus(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self
    where
        Self: Sized,
    {
        self.set_in_focus_style(f(StyleRefinement::default()));
        self
    }

    fn on_focus(
        mut self,
        listener: impl Fn(&mut V, &FocusEvent, &mut ViewContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.focus_listeners()
            .push(Box::new(move |view, focus_handle, event, cx| {
                if event.focused.as_ref() == Some(focus_handle) {
                    listener(view, event, cx)
                }
            }));
        self
    }

    fn on_blur(
        mut self,
        listener: impl Fn(&mut V, &FocusEvent, &mut ViewContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.focus_listeners()
            .push(Box::new(move |view, focus_handle, event, cx| {
                if event.blurred.as_ref() == Some(focus_handle) {
                    listener(view, event, cx)
                }
            }));
        self
    }

    fn on_focus_in(
        mut self,
        listener: impl Fn(&mut V, &FocusEvent, &mut ViewContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.focus_listeners()
            .push(Box::new(move |view, focus_handle, event, cx| {
                let descendant_blurred = event
                    .blurred
                    .as_ref()
                    .map_or(false, |blurred| focus_handle.contains(blurred, cx));
                let descendant_focused = event
                    .focused
                    .as_ref()
                    .map_or(false, |focused| focus_handle.contains(focused, cx));

                if !descendant_blurred && descendant_focused {
                    listener(view, event, cx)
                }
            }));
        self
    }

    fn on_focus_out(
        mut self,
        listener: impl Fn(&mut V, &FocusEvent, &mut ViewContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.focus_listeners()
            .push(Box::new(move |view, focus_handle, event, cx| {
                let descendant_blurred = event
                    .blurred
                    .as_ref()
                    .map_or(false, |blurred| focus_handle.contains(blurred, cx));
                let descendant_focused = event
                    .focused
                    .as_ref()
                    .map_or(false, |focused| focus_handle.contains(focused, cx));
                if descendant_blurred && !descendant_focused {
                    listener(view, event, cx)
                }
            }));
        self
    }
}
