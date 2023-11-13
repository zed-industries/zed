use crate::{
    build_action_from_type, Action, Bounds, DispatchPhase, Element, FocusEvent, FocusHandle,
    FocusId, KeyBinding, KeyContext, KeyMatch, Keymap, Keystroke, KeystrokeMatcher, MouseDownEvent,
    Pixels, Style, StyleRefinement, ViewContext, WindowContext,
};
use collections::HashMap;
use parking_lot::Mutex;
use refineable::Refineable;
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    rc::Rc,
    sync::Arc,
};
use util::ResultExt;

pub type FocusListeners<V> = SmallVec<[FocusListener<V>; 2]>;
pub type FocusListener<V> =
    Box<dyn Fn(&mut V, &FocusHandle, &FocusEvent, &mut ViewContext<V>) + 'static>;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct DispatchNodeId(usize);

pub(crate) struct DispatchTree {
    node_stack: Vec<DispatchNodeId>,
    context_stack: Vec<KeyContext>,
    nodes: Vec<DispatchNode>,
    focusable_node_ids: HashMap<FocusId, DispatchNodeId>,
    keystroke_matchers: HashMap<SmallVec<[KeyContext; 4]>, KeystrokeMatcher>,
    keymap: Arc<Mutex<Keymap>>,
}

#[derive(Default)]
pub(crate) struct DispatchNode {
    pub key_listeners: SmallVec<[KeyListener; 2]>,
    pub action_listeners: SmallVec<[DispatchActionListener; 16]>,
    pub context: KeyContext,
    parent: Option<DispatchNodeId>,
}

type KeyListener = Rc<dyn Fn(&dyn Any, DispatchPhase, &mut WindowContext)>;

#[derive(Clone)]
pub(crate) struct DispatchActionListener {
    pub(crate) action_type: TypeId,
    pub(crate) listener: Rc<dyn Fn(&dyn Any, DispatchPhase, &mut WindowContext)>,
}

impl DispatchTree {
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
        self.context_stack.clear();
        self.focusable_node_ids.clear();
        self.keystroke_matchers.clear();
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

    pub fn available_actions(&self, target: FocusId) -> Vec<Box<dyn Action>> {
        let mut actions = Vec::new();
        if let Some(node) = self.focusable_node_ids.get(&target) {
            for node_id in self.dispatch_path(*node) {
                let node = &self.nodes[node_id.0];
                for DispatchActionListener { action_type, .. } in &node.action_listeners {
                    actions.extend(build_action_from_type(action_type).log_err());
                }
            }
        }
        actions
    }

    pub fn bindings_for_action(&self, action: &dyn Action) -> Vec<KeyBinding> {
        self.keymap
            .lock()
            .bindings_for_action(action.type_id())
            .filter(|candidate| candidate.action.partial_eq(action))
            .cloned()
            .collect()
    }

    pub fn dispatch_key(
        &mut self,
        keystroke: &Keystroke,
        context: &[KeyContext],
    ) -> Option<Box<dyn Action>> {
        if !self.keystroke_matchers.contains_key(context) {
            let keystroke_contexts = context.iter().cloned().collect();
            self.keystroke_matchers.insert(
                keystroke_contexts,
                KeystrokeMatcher::new(self.keymap.clone()),
            );
        }

        let keystroke_matcher = self.keystroke_matchers.get_mut(context).unwrap();
        if let KeyMatch::Some(action) = keystroke_matcher.match_keystroke(keystroke, context) {
            // Clear all pending keystrokes when an action has been found.
            for keystroke_matcher in self.keystroke_matchers.values_mut() {
                keystroke_matcher.clear_pending();
            }

            Some(action)
        } else {
            None
        }
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

    fn active_node_id(&self) -> DispatchNodeId {
        *self.node_stack.last().unwrap()
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
        let focus_handle = if let Some(focusable) = self.as_focusable_mut() {
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
            Some(focus_handle)
        } else {
            None
        };

        cx.with_key_dispatch(self.key_context().clone(), focus_handle, f)
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
    pub non_focusable: NonFocusableKeyDispatch,
    pub focus_handle: Option<FocusHandle>,
    pub focus_listeners: FocusListeners<V>,
    pub focus_style: StyleRefinement,
    pub focus_in_style: StyleRefinement,
    pub in_focus_style: StyleRefinement,
}

impl<V> FocusableKeyDispatch<V> {
    pub fn new(non_focusable: NonFocusableKeyDispatch) -> Self {
        Self {
            non_focusable,
            focus_handle: None,
            focus_listeners: FocusListeners::default(),
            focus_style: StyleRefinement::default(),
            focus_in_style: StyleRefinement::default(),
            in_focus_style: StyleRefinement::default(),
        }
    }

    pub fn tracked(non_focusable: NonFocusableKeyDispatch, handle: &FocusHandle) -> Self {
        Self {
            non_focusable,
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
        &self.non_focusable.key_context
    }

    fn key_context_mut(&mut self) -> &mut KeyContext {
        &mut self.non_focusable.key_context
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
