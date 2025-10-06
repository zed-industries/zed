//! KeyDispatch is where GPUI deals with binding actions to key events.
//!
//! The key pieces to making a key binding work are to define an action,
//! implement a method that takes that action as a type parameter,
//! and then to register the action during render on a focused node
//! with a keymap context:
//!
//! ```ignore
//! actions!(editor,[Undo, Redo]);
//!
//! impl Editor {
//!   fn undo(&mut self, _: &Undo, _window: &mut Window, _cx: &mut Context<Self>) { ... }
//!   fn redo(&mut self, _: &Redo, _window: &mut Window, _cx: &mut Context<Self>) { ... }
//! }
//!
//! impl Render for Editor {
//!   fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
//!     div()
//!       .track_focus(&self.focus_handle(cx))
//!       .key_context("Editor")
//!       .on_action(cx.listener(Editor::undo))
//!       .on_action(cx.listener(Editor::redo))
//!     ...
//!    }
//! }
//!```
//!
//! The keybindings themselves are managed independently by calling cx.bind_keys().
//! (Though mostly when developing Zed itself, you just need to add a new line to
//!  assets/keymaps/default-{platform}.json).
//!
//! ```ignore
//! cx.bind_keys([
//!   KeyBinding::new("cmd-z", Editor::undo, Some("Editor")),
//!   KeyBinding::new("cmd-shift-z", Editor::redo, Some("Editor")),
//! ])
//! ```
//!
//! With all of this in place, GPUI will ensure that if you have an Editor that contains
//! the focus, hitting cmd-z will Undo.
//!
//! In real apps, it is a little more complicated than this, because typically you have
//! several nested views that each register keyboard handlers. In this case action matching
//! bubbles up from the bottom. For example in Zed, the Workspace is the top-level view, which contains Pane's, which contain Editors. If there are conflicting keybindings defined
//! then the Editor's bindings take precedence over the Pane's bindings, which take precedence over the Workspace.
//!
//! In GPUI, keybindings are not limited to just single keystrokes, you can define
//! sequences by separating the keys with a space:
//!
//!  KeyBinding::new("cmd-k left", pane::SplitLeft, Some("Pane"))

use crate::{
    Action, ActionRegistry, App, DispatchPhase, EntityId, FocusId, KeyBinding, KeyContext, Keymap,
    Keystroke, ModifiersChangedEvent, Window,
};
use collections::FxHashMap;
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    mem,
    ops::Range,
    rc::Rc,
};

/// ID of a node within `DispatchTree`. Note that these are **not** stable between frames, and so a
/// `DispatchNodeId` should only be used with the `DispatchTree` that provided it.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) struct DispatchNodeId(usize);

pub(crate) struct DispatchTree {
    node_stack: Vec<DispatchNodeId>,
    pub(crate) context_stack: Vec<KeyContext>,
    view_stack: Vec<EntityId>,
    nodes: Vec<DispatchNode>,
    focusable_node_ids: FxHashMap<FocusId, DispatchNodeId>,
    view_node_ids: FxHashMap<EntityId, DispatchNodeId>,
    keymap: Rc<RefCell<Keymap>>,
    action_registry: Rc<ActionRegistry>,
}

#[derive(Default)]
pub(crate) struct DispatchNode {
    pub key_listeners: Vec<KeyListener>,
    pub action_listeners: Vec<DispatchActionListener>,
    pub modifiers_changed_listeners: Vec<ModifiersChangedListener>,
    pub context: Option<KeyContext>,
    pub focus_id: Option<FocusId>,
    view_id: Option<EntityId>,
    parent: Option<DispatchNodeId>,
}

pub(crate) struct ReusedSubtree {
    old_range: Range<usize>,
    new_range: Range<usize>,
    contains_focus: bool,
}

impl ReusedSubtree {
    pub fn refresh_node_id(&self, node_id: DispatchNodeId) -> DispatchNodeId {
        debug_assert!(
            self.old_range.contains(&node_id.0),
            "node {} was not part of the reused subtree {:?}",
            node_id.0,
            self.old_range
        );
        DispatchNodeId((node_id.0 - self.old_range.start) + self.new_range.start)
    }

    pub fn contains_focus(&self) -> bool {
        self.contains_focus
    }
}

#[derive(Default, Debug)]
pub(crate) struct Replay {
    pub(crate) keystroke: Keystroke,
    pub(crate) bindings: SmallVec<[KeyBinding; 1]>,
}

#[derive(Default, Debug)]
pub(crate) struct DispatchResult {
    pub(crate) pending: SmallVec<[Keystroke; 1]>,
    pub(crate) bindings: SmallVec<[KeyBinding; 1]>,
    pub(crate) to_replay: SmallVec<[Replay; 1]>,
    pub(crate) context_stack: Vec<KeyContext>,
}

type KeyListener = Rc<dyn Fn(&dyn Any, DispatchPhase, &mut Window, &mut App)>;
type ModifiersChangedListener = Rc<dyn Fn(&ModifiersChangedEvent, &mut Window, &mut App)>;

#[derive(Clone)]
pub(crate) struct DispatchActionListener {
    pub(crate) action_type: TypeId,
    pub(crate) listener: Rc<dyn Fn(&dyn Any, DispatchPhase, &mut Window, &mut App)>,
}

impl DispatchTree {
    pub fn new(keymap: Rc<RefCell<Keymap>>, action_registry: Rc<ActionRegistry>) -> Self {
        Self {
            node_stack: Vec::new(),
            context_stack: Vec::new(),
            view_stack: Vec::new(),
            nodes: Vec::new(),
            focusable_node_ids: FxHashMap::default(),
            view_node_ids: FxHashMap::default(),
            keymap,
            action_registry,
        }
    }

    pub fn clear(&mut self) {
        self.node_stack.clear();
        self.context_stack.clear();
        self.view_stack.clear();
        self.nodes.clear();
        self.focusable_node_ids.clear();
        self.view_node_ids.clear();
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn push_node(&mut self) -> DispatchNodeId {
        let parent = self.node_stack.last().copied();
        let node_id = DispatchNodeId(self.nodes.len());

        self.nodes.push(DispatchNode {
            parent,
            ..Default::default()
        });
        self.node_stack.push(node_id);
        node_id
    }

    pub fn set_active_node(&mut self, node_id: DispatchNodeId) {
        let next_node_parent = self.nodes[node_id.0].parent;
        while self.node_stack.last().copied() != next_node_parent && !self.node_stack.is_empty() {
            self.pop_node();
        }

        if self.node_stack.last().copied() == next_node_parent {
            self.node_stack.push(node_id);
            let active_node = &self.nodes[node_id.0];
            if let Some(view_id) = active_node.view_id {
                self.view_stack.push(view_id)
            }
            if let Some(context) = active_node.context.clone() {
                self.context_stack.push(context);
            }
        } else {
            debug_assert_eq!(self.node_stack.len(), 0);

            let mut current_node_id = Some(node_id);
            while let Some(node_id) = current_node_id {
                let node = &self.nodes[node_id.0];
                if let Some(context) = node.context.clone() {
                    self.context_stack.push(context);
                }
                if node.view_id.is_some() {
                    self.view_stack.push(node.view_id.unwrap());
                }
                self.node_stack.push(node_id);
                current_node_id = node.parent;
            }

            self.context_stack.reverse();
            self.view_stack.reverse();
            self.node_stack.reverse();
        }
    }

    pub fn set_key_context(&mut self, context: KeyContext) {
        self.active_node().context = Some(context.clone());
        self.context_stack.push(context);
    }

    pub fn set_focus_id(&mut self, focus_id: FocusId) {
        let node_id = *self.node_stack.last().unwrap();
        self.nodes[node_id.0].focus_id = Some(focus_id);
        self.focusable_node_ids.insert(focus_id, node_id);
    }

    pub fn set_view_id(&mut self, view_id: EntityId) {
        if self.view_stack.last().copied() != Some(view_id) {
            let node_id = *self.node_stack.last().unwrap();
            self.nodes[node_id.0].view_id = Some(view_id);
            self.view_node_ids.insert(view_id, node_id);
            self.view_stack.push(view_id);
        }
    }

    pub fn pop_node(&mut self) {
        let node = &self.nodes[self.active_node_id().unwrap().0];
        if node.context.is_some() {
            self.context_stack.pop();
        }
        if node.view_id.is_some() {
            self.view_stack.pop();
        }
        self.node_stack.pop();
    }

    fn move_node(&mut self, source: &mut DispatchNode) {
        self.push_node();
        if let Some(context) = source.context.clone() {
            self.set_key_context(context);
        }
        if let Some(focus_id) = source.focus_id {
            self.set_focus_id(focus_id);
        }
        if let Some(view_id) = source.view_id {
            self.set_view_id(view_id);
        }

        let target = self.active_node();
        target.key_listeners = mem::take(&mut source.key_listeners);
        target.action_listeners = mem::take(&mut source.action_listeners);
        target.modifiers_changed_listeners = mem::take(&mut source.modifiers_changed_listeners);
    }

    pub fn reuse_subtree(
        &mut self,
        old_range: Range<usize>,
        source: &mut Self,
        focus: Option<FocusId>,
    ) -> ReusedSubtree {
        let new_range = self.nodes.len()..self.nodes.len() + old_range.len();

        let mut contains_focus = false;
        let mut source_stack = vec![];
        for (source_node_id, source_node) in source
            .nodes
            .iter_mut()
            .enumerate()
            .skip(old_range.start)
            .take(old_range.len())
        {
            let source_node_id = DispatchNodeId(source_node_id);
            while let Some(source_ancestor) = source_stack.last() {
                if source_node.parent == Some(*source_ancestor) {
                    break;
                } else {
                    source_stack.pop();
                    self.pop_node();
                }
            }

            source_stack.push(source_node_id);
            if source_node.focus_id.is_some() && source_node.focus_id == focus {
                contains_focus = true;
            }
            self.move_node(source_node);
        }

        while !source_stack.is_empty() {
            source_stack.pop();
            self.pop_node();
        }

        ReusedSubtree {
            old_range,
            new_range,
            contains_focus,
        }
    }

    pub fn truncate(&mut self, index: usize) {
        for node in &self.nodes[index..] {
            if let Some(focus_id) = node.focus_id {
                self.focusable_node_ids.remove(&focus_id);
            }

            if let Some(view_id) = node.view_id {
                self.view_node_ids.remove(&view_id);
            }
        }
        self.nodes.truncate(index);
    }

    pub fn on_key_event(&mut self, listener: KeyListener) {
        self.active_node().key_listeners.push(listener);
    }

    pub fn on_modifiers_changed(&mut self, listener: ModifiersChangedListener) {
        self.active_node()
            .modifiers_changed_listeners
            .push(listener);
    }

    pub fn on_action(
        &mut self,
        action_type: TypeId,
        listener: Rc<dyn Fn(&dyn Any, DispatchPhase, &mut Window, &mut App)>,
    ) {
        self.active_node()
            .action_listeners
            .push(DispatchActionListener {
                action_type,
                listener,
            });
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

    /// Returns key bindings that invoke an action on the currently focused element. Bindings are
    /// returned in the order they were added. For display, the last binding should take precedence.
    ///
    /// Bindings are only included if they are the highest precedence match for their keystrokes, so
    /// shadowed bindings are not included.
    pub fn bindings_for_action(
        &self,
        action: &dyn Action,
        context_stack: &[KeyContext],
    ) -> Vec<KeyBinding> {
        // Ideally this would return a `DoubleEndedIterator` to avoid `highest_precedence_*`
        // methods, but this can't be done very cleanly since keymap must be borrowed.
        let keymap = self.keymap.borrow();
        keymap
            .bindings_for_action(action)
            .filter(|binding| {
                Self::binding_matches_predicate_and_not_shadowed(&keymap, binding, context_stack)
            })
            .cloned()
            .collect()
    }

    /// Returns the highest precedence binding for the given action and context stack. This is the
    /// same as the last result of `bindings_for_action`, but more efficient than getting all bindings.
    pub fn highest_precedence_binding_for_action(
        &self,
        action: &dyn Action,
        context_stack: &[KeyContext],
    ) -> Option<KeyBinding> {
        let keymap = self.keymap.borrow();
        keymap
            .bindings_for_action(action)
            .rev()
            .find(|binding| {
                Self::binding_matches_predicate_and_not_shadowed(&keymap, binding, context_stack)
            })
            .cloned()
    }

    fn binding_matches_predicate_and_not_shadowed(
        keymap: &Keymap,
        binding: &KeyBinding,
        context_stack: &[KeyContext],
    ) -> bool {
        let (bindings, _) = keymap.bindings_for_input(&binding.keystrokes, context_stack);
        if let Some(found) = bindings.iter().next() {
            found.action.partial_eq(binding.action.as_ref())
        } else {
            false
        }
    }

    fn bindings_for_input(
        &self,
        input: &[Keystroke],
        dispatch_path: &SmallVec<[DispatchNodeId; 32]>,
    ) -> (SmallVec<[KeyBinding; 1]>, bool, Vec<KeyContext>) {
        let context_stack: Vec<KeyContext> = dispatch_path
            .iter()
            .filter_map(|node_id| self.node(*node_id).context.clone())
            .collect();

        let (bindings, partial) = self
            .keymap
            .borrow()
            .bindings_for_input(input, &context_stack);
        (bindings, partial, context_stack)
    }

    /// dispatch_key processes the keystroke
    /// input should be set to the value of `pending` from the previous call to dispatch_key.
    /// This returns three instructions to the input handler:
    /// - bindings: any bindings to execute before processing this keystroke
    /// - pending: the new set of pending keystrokes to store
    /// - to_replay: any keystroke that had been pushed to pending, but are no-longer matched,
    ///   these should be replayed first.
    pub fn dispatch_key(
        &mut self,
        mut input: SmallVec<[Keystroke; 1]>,
        keystroke: Keystroke,
        dispatch_path: &SmallVec<[DispatchNodeId; 32]>,
    ) -> DispatchResult {
        input.push(keystroke.clone());
        let (bindings, pending, context_stack) = self.bindings_for_input(&input, dispatch_path);

        if pending {
            return DispatchResult {
                pending: input,
                context_stack,
                ..Default::default()
            };
        } else if !bindings.is_empty() {
            return DispatchResult {
                bindings,
                context_stack,
                ..Default::default()
            };
        } else if input.len() == 1 {
            return DispatchResult {
                context_stack,
                ..Default::default()
            };
        }
        input.pop();

        let (suffix, mut to_replay) = self.replay_prefix(input, dispatch_path);

        let mut result = self.dispatch_key(suffix, keystroke, dispatch_path);
        to_replay.extend(result.to_replay);
        result.to_replay = to_replay;
        result
    }

    /// If the user types a matching prefix of a binding and then waits for a timeout
    /// flush_dispatch() converts any previously pending input to replay events.
    pub fn flush_dispatch(
        &mut self,
        input: SmallVec<[Keystroke; 1]>,
        dispatch_path: &SmallVec<[DispatchNodeId; 32]>,
    ) -> SmallVec<[Replay; 1]> {
        let (suffix, mut to_replay) = self.replay_prefix(input, dispatch_path);

        if !suffix.is_empty() {
            to_replay.extend(self.flush_dispatch(suffix, dispatch_path))
        }

        to_replay
    }

    /// Converts the longest prefix of input to a replay event and returns the rest.
    fn replay_prefix(
        &self,
        mut input: SmallVec<[Keystroke; 1]>,
        dispatch_path: &SmallVec<[DispatchNodeId; 32]>,
    ) -> (SmallVec<[Keystroke; 1]>, SmallVec<[Replay; 1]>) {
        let mut to_replay: SmallVec<[Replay; 1]> = Default::default();
        for last in (0..input.len()).rev() {
            let (bindings, _, _) = self.bindings_for_input(&input[0..=last], dispatch_path);
            if !bindings.is_empty() {
                to_replay.push(Replay {
                    keystroke: input.drain(0..=last).next_back().unwrap(),
                    bindings,
                });
                break;
            }
        }
        if to_replay.is_empty() {
            to_replay.push(Replay {
                keystroke: input.remove(0),
                ..Default::default()
            });
        }
        (input, to_replay)
    }

    pub fn dispatch_path(&self, target: DispatchNodeId) -> SmallVec<[DispatchNodeId; 32]> {
        let mut dispatch_path: SmallVec<[DispatchNodeId; 32]> = SmallVec::new();
        let mut current_node_id = Some(target);
        while let Some(node_id) = current_node_id {
            dispatch_path.push(node_id);
            current_node_id = self.nodes.get(node_id.0).and_then(|node| node.parent);
        }
        dispatch_path.reverse(); // Reverse the path so it goes from the root to the focused node.
        dispatch_path
    }

    pub fn focus_path(&self, focus_id: FocusId) -> SmallVec<[FocusId; 8]> {
        let mut focus_path: SmallVec<[FocusId; 8]> = SmallVec::new();
        let mut current_node_id = self.focusable_node_ids.get(&focus_id).copied();
        while let Some(node_id) = current_node_id {
            let node = self.node(node_id);
            if let Some(focus_id) = node.focus_id {
                focus_path.push(focus_id);
            }
            current_node_id = node.parent;
        }
        focus_path.reverse(); // Reverse the path so it goes from the root to the focused node.
        focus_path
    }

    pub fn view_path(&self, view_id: EntityId) -> SmallVec<[EntityId; 8]> {
        let mut view_path: SmallVec<[EntityId; 8]> = SmallVec::new();
        let mut current_node_id = self.view_node_ids.get(&view_id).copied();
        while let Some(node_id) = current_node_id {
            let node = self.node(node_id);
            if let Some(view_id) = node.view_id {
                view_path.push(view_id);
            }
            current_node_id = node.parent;
        }
        view_path.reverse(); // Reverse the path so it goes from the root to the view node.
        view_path
    }

    pub fn node(&self, node_id: DispatchNodeId) -> &DispatchNode {
        &self.nodes[node_id.0]
    }

    fn active_node(&mut self) -> &mut DispatchNode {
        let active_node_id = self.active_node_id().unwrap();
        &mut self.nodes[active_node_id.0]
    }

    pub fn focusable_node_id(&self, target: FocusId) -> Option<DispatchNodeId> {
        self.focusable_node_ids.get(&target).copied()
    }

    pub fn root_node_id(&self) -> DispatchNodeId {
        debug_assert!(!self.nodes.is_empty());
        DispatchNodeId(0)
    }

    pub fn active_node_id(&self) -> Option<DispatchNodeId> {
        self.node_stack.last().copied()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        self as gpui, Element, ElementId, GlobalElementId, InspectorElementId, LayoutId, Style,
    };
    use core::panic;
    use std::{cell::RefCell, ops::Range, rc::Rc};

    use crate::{
        Action, ActionRegistry, App, Bounds, Context, DispatchTree, FocusHandle, InputHandler,
        IntoElement, KeyBinding, KeyContext, Keymap, Pixels, Point, Render, TestAppContext,
        UTF16Selection, Window,
    };

    #[derive(PartialEq, Eq)]
    struct TestAction;

    impl Action for TestAction {
        fn name(&self) -> &'static str {
            "test::TestAction"
        }

        fn name_for_type() -> &'static str
        where
            Self: ::std::marker::Sized,
        {
            "test::TestAction"
        }

        fn partial_eq(&self, action: &dyn Action) -> bool {
            action.as_any().downcast_ref::<Self>() == Some(self)
        }

        fn boxed_clone(&self) -> std::boxed::Box<dyn Action> {
            Box::new(TestAction)
        }

        fn build(_value: serde_json::Value) -> anyhow::Result<Box<dyn Action>>
        where
            Self: Sized,
        {
            Ok(Box::new(TestAction))
        }
    }

    #[test]
    fn test_keybinding_for_action_bounds() {
        let keymap = Keymap::new(vec![KeyBinding::new(
            "cmd-n",
            TestAction,
            Some("ProjectPanel"),
        )]);

        let mut registry = ActionRegistry::default();

        registry.load_action::<TestAction>();

        let keymap = Rc::new(RefCell::new(keymap));

        let tree = DispatchTree::new(keymap, Rc::new(registry));

        let contexts = vec![
            KeyContext::parse("Workspace").unwrap(),
            KeyContext::parse("ProjectPanel").unwrap(),
        ];

        let keybinding = tree.bindings_for_action(&TestAction, &contexts);

        assert!(keybinding[0].action.partial_eq(&TestAction))
    }

    #[crate::test]
    fn test_input_handler_pending(cx: &mut TestAppContext) {
        #[derive(Clone)]
        struct CustomElement {
            focus_handle: FocusHandle,
            text: Rc<RefCell<String>>,
        }
        impl CustomElement {
            fn new(cx: &mut Context<Self>) -> Self {
                Self {
                    focus_handle: cx.focus_handle(),
                    text: Rc::default(),
                }
            }
        }
        impl Element for CustomElement {
            type RequestLayoutState = ();

            type PrepaintState = ();

            fn id(&self) -> Option<ElementId> {
                Some("custom".into())
            }
            fn source_location(&self) -> Option<&'static panic::Location<'static>> {
                None
            }
            fn request_layout(
                &mut self,
                _: Option<&GlobalElementId>,
                _: Option<&InspectorElementId>,
                window: &mut Window,
                cx: &mut App,
            ) -> (LayoutId, Self::RequestLayoutState) {
                (window.request_layout(Style::default(), [], cx), ())
            }
            fn prepaint(
                &mut self,
                _: Option<&GlobalElementId>,
                _: Option<&InspectorElementId>,
                _: Bounds<Pixels>,
                _: &mut Self::RequestLayoutState,
                window: &mut Window,
                cx: &mut App,
            ) -> Self::PrepaintState {
                window.set_focus_handle(&self.focus_handle, cx);
            }
            fn paint(
                &mut self,
                _: Option<&GlobalElementId>,
                _: Option<&InspectorElementId>,
                _: Bounds<Pixels>,
                _: &mut Self::RequestLayoutState,
                _: &mut Self::PrepaintState,
                window: &mut Window,
                cx: &mut App,
            ) {
                let mut key_context = KeyContext::default();
                key_context.add("Terminal");
                window.set_key_context(key_context);
                window.handle_input(&self.focus_handle, self.clone(), cx);
                window.on_action(std::any::TypeId::of::<TestAction>(), |_, _, _, _| {});
            }
        }
        impl IntoElement for CustomElement {
            type Element = Self;

            fn into_element(self) -> Self::Element {
                self
            }
        }

        impl InputHandler for CustomElement {
            fn selected_text_range(
                &mut self,
                _: bool,
                _: &mut Window,
                _: &mut App,
            ) -> Option<UTF16Selection> {
                None
            }

            fn marked_text_range(&mut self, _: &mut Window, _: &mut App) -> Option<Range<usize>> {
                None
            }

            fn text_for_range(
                &mut self,
                _: Range<usize>,
                _: &mut Option<Range<usize>>,
                _: &mut Window,
                _: &mut App,
            ) -> Option<String> {
                None
            }

            fn replace_text_in_range(
                &mut self,
                replacement_range: Option<Range<usize>>,
                text: &str,
                _: &mut Window,
                _: &mut App,
            ) {
                if replacement_range.is_some() {
                    unimplemented!()
                }
                self.text.borrow_mut().push_str(text)
            }

            fn replace_and_mark_text_in_range(
                &mut self,
                replacement_range: Option<Range<usize>>,
                new_text: &str,
                _: Option<Range<usize>>,
                _: &mut Window,
                _: &mut App,
            ) {
                if replacement_range.is_some() {
                    unimplemented!()
                }
                self.text.borrow_mut().push_str(new_text)
            }

            fn unmark_text(&mut self, _: &mut Window, _: &mut App) {}

            fn bounds_for_range(
                &mut self,
                _: Range<usize>,
                _: &mut Window,
                _: &mut App,
            ) -> Option<Bounds<Pixels>> {
                None
            }

            fn character_index_for_point(
                &mut self,
                _: Point<Pixels>,
                _: &mut Window,
                _: &mut App,
            ) -> Option<usize> {
                None
            }
        }
        impl Render for CustomElement {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                self.clone()
            }
        }

        cx.update(|cx| {
            cx.bind_keys([KeyBinding::new("ctrl-b", TestAction, Some("Terminal"))]);
            cx.bind_keys([KeyBinding::new("ctrl-b h", TestAction, Some("Terminal"))]);
        });
        let (test, cx) = cx.add_window_view(|_, cx| CustomElement::new(cx));
        cx.update(|window, cx| {
            window.focus(&test.read(cx).focus_handle);
            window.activate_window();
        });
        cx.simulate_keystrokes("ctrl-b [");
        test.update(cx, |test, _| assert_eq!(test.text.borrow().as_str(), "["))
    }
}
