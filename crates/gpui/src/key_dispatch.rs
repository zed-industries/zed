/// KeyDispatch is where GPUI deals with binding actions to key events.
///
/// The key pieces to making a key binding work are to define an action,
/// implement a method that takes that action as a type parameter,
/// and then to register the action during render on a focused node
/// with a keymap context:
///
/// ```rust
/// actions!(editor,[Undo, Redo]);;
///
/// impl Editor {
///   fn undo(&mut self, _: &Undo, _cx: &mut ViewContext<Self>) { ... }
///   fn redo(&mut self, _: &Redo, _cx: &mut ViewContext<Self>) { ... }
/// }
///
/// impl Render for Editor {
///   fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
///     div()
///       .track_focus(&self.focus_handle)
///       .keymap_context("Editor")
///       .on_action(cx.listener(Editor::undo))
///       .on_action(cx.listener(Editor::redo))
///     ...
///    }
/// }
///```
///
/// The keybindings themselves are managed independently by calling cx.bind_keys().
/// (Though mostly when developing Zed itself, you just need to add a new line to
///  assets/keymaps/default.json).
///
/// ```rust
/// cx.bind_keys([
///   KeyBinding::new("cmd-z", Editor::undo, Some("Editor")),
///   KeyBinding::new("cmd-shift-z", Editor::redo, Some("Editor")),
/// ])
/// ```
///
/// With all of this in place, GPUI will ensure that if you have an Editor that contains
/// the focus, hitting cmd-z will Undo.
///
/// In real apps, it is a little more complicated than this, because typically you have
/// several nested views that each register keyboard handlers. In this case action matching
/// bubbles up from the bottom. For example in Zed, the Workspace is the top-level view, which contains Pane's, which contain Editors. If there are conflicting keybindings defined
/// then the Editor's bindings take precedence over the Pane's bindings, which take precedence over the Workspace.
///
/// In GPUI, keybindings are not limited to just single keystrokes, you can define
/// sequences by separating the keys with a space:
///
///  KeyBinding::new("cmd-k left", pane::SplitLeft, Some("Pane"))
///
use crate::{
    Action, ActionRegistry, DispatchPhase, ElementContext, EntityId, FocusId, KeyBinding,
    KeyContext, Keymap, KeymatchResult, Keystroke, KeystrokeMatcher, WindowContext,
};
use collections::FxHashMap;
use smallvec::{smallvec, SmallVec};
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    mem,
    rc::Rc,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) struct DispatchNodeId(usize);

pub(crate) struct DispatchTree {
    node_stack: Vec<DispatchNodeId>,
    pub(crate) context_stack: Vec<KeyContext>,
    nodes: Vec<DispatchNode>,
    focusable_node_ids: FxHashMap<FocusId, DispatchNodeId>,
    view_node_ids: FxHashMap<EntityId, DispatchNodeId>,
    keystroke_matchers: FxHashMap<SmallVec<[KeyContext; 4]>, KeystrokeMatcher>,
    keymap: Rc<RefCell<Keymap>>,
    action_registry: Rc<ActionRegistry>,
}

#[derive(Default)]
pub(crate) struct DispatchNode {
    pub key_listeners: Vec<KeyListener>,
    pub action_listeners: Vec<DispatchActionListener>,
    pub context: Option<KeyContext>,
    focus_id: Option<FocusId>,
    view_id: Option<EntityId>,
    parent: Option<DispatchNodeId>,
}

type KeyListener = Rc<dyn Fn(&dyn Any, DispatchPhase, &mut ElementContext)>;

#[derive(Clone)]
pub(crate) struct DispatchActionListener {
    pub(crate) action_type: TypeId,
    pub(crate) listener: Rc<dyn Fn(&dyn Any, DispatchPhase, &mut WindowContext)>,
}

impl DispatchTree {
    pub fn new(keymap: Rc<RefCell<Keymap>>, action_registry: Rc<ActionRegistry>) -> Self {
        Self {
            node_stack: Vec::new(),
            context_stack: Vec::new(),
            nodes: Vec::new(),
            focusable_node_ids: FxHashMap::default(),
            view_node_ids: FxHashMap::default(),
            keystroke_matchers: FxHashMap::default(),
            keymap,
            action_registry,
        }
    }

    pub fn clear(&mut self) {
        self.node_stack.clear();
        self.context_stack.clear();
        self.nodes.clear();
        self.focusable_node_ids.clear();
        self.view_node_ids.clear();
        self.keystroke_matchers.clear();
    }

    pub fn push_node(
        &mut self,
        context: Option<KeyContext>,
        focus_id: Option<FocusId>,
        view_id: Option<EntityId>,
    ) {
        let parent = self.node_stack.last().copied();
        let node_id = DispatchNodeId(self.nodes.len());
        self.nodes.push(DispatchNode {
            parent,
            focus_id,
            view_id,
            ..Default::default()
        });
        self.node_stack.push(node_id);

        if let Some(context) = context {
            self.active_node().context = Some(context.clone());
            self.context_stack.push(context);
        }

        if let Some(focus_id) = focus_id {
            self.focusable_node_ids.insert(focus_id, node_id);
        }

        if let Some(view_id) = view_id {
            self.view_node_ids.insert(view_id, node_id);
        }
    }

    pub fn pop_node(&mut self) {
        let node = &self.nodes[self.active_node_id().0];
        if node.context.is_some() {
            self.context_stack.pop();
        }
        self.node_stack.pop();
    }

    fn move_node(&mut self, source: &mut DispatchNode) {
        self.push_node(source.context.take(), source.focus_id, source.view_id);
        let target = self.active_node();
        target.key_listeners = mem::take(&mut source.key_listeners);
        target.action_listeners = mem::take(&mut source.action_listeners);
    }

    pub fn reuse_view(&mut self, view_id: EntityId, source: &mut Self) -> SmallVec<[EntityId; 8]> {
        let view_source_node_id = source
            .view_node_ids
            .get(&view_id)
            .expect("view should exist in previous dispatch tree");
        let view_source_node = &mut source.nodes[view_source_node_id.0];
        self.move_node(view_source_node);

        let mut grafted_view_ids = smallvec![view_id];
        let mut source_stack = vec![*view_source_node_id];
        for (source_node_id, source_node) in source
            .nodes
            .iter_mut()
            .enumerate()
            .skip(view_source_node_id.0 + 1)
        {
            let source_node_id = DispatchNodeId(source_node_id);
            while let Some(source_ancestor) = source_stack.last() {
                if source_node.parent != Some(*source_ancestor) {
                    source_stack.pop();
                    self.pop_node();
                } else {
                    break;
                }
            }

            if source_stack.is_empty() {
                break;
            } else {
                source_stack.push(source_node_id);
                self.move_node(source_node);
                if let Some(view_id) = source_node.view_id {
                    grafted_view_ids.push(view_id);
                }
            }
        }

        while !source_stack.is_empty() {
            source_stack.pop();
            self.pop_node();
        }

        grafted_view_ids
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
        context_stack: &[KeyContext],
    ) -> Vec<KeyBinding> {
        let keymap = self.keymap.borrow();
        keymap
            .bindings_for_action(action)
            .filter(|binding| {
                for i in 0..context_stack.len() {
                    let context = &context_stack[0..=i];
                    if keymap.binding_enabled(binding, context) {
                        return true;
                    }
                }
                false
            })
            .cloned()
            .collect()
    }

    // dispatch_key pushes the next keystroke into any key binding matchers.
    // any matching bindings are returned in the order that they should be dispatched:
    // * First by length of binding (so if you have a binding for "b" and "ab", the "ab" binding fires first)
    // * Secondly by depth in the tree (so if Editor has a binding for "b" and workspace a
    // binding for "b", the Editor action fires first).
    pub fn dispatch_key(
        &mut self,
        keystroke: &Keystroke,
        dispatch_path: &SmallVec<[DispatchNodeId; 32]>,
    ) -> KeymatchResult {
        let mut bindings = SmallVec::<[KeyBinding; 1]>::new();
        let mut pending = false;

        let mut context_stack: SmallVec<[KeyContext; 4]> = SmallVec::new();
        for node_id in dispatch_path {
            let node = self.node(*node_id);

            if let Some(context) = node.context.clone() {
                context_stack.push(context);
            }
        }

        while !context_stack.is_empty() {
            let keystroke_matcher = self
                .keystroke_matchers
                .entry(context_stack.clone())
                .or_insert_with(|| KeystrokeMatcher::new(self.keymap.clone()));

            let result = keystroke_matcher.match_keystroke(keystroke, &context_stack);
            if result.pending && !pending && !bindings.is_empty() {
                context_stack.pop();
                continue;
            }

            pending = result.pending || pending;
            for new_binding in result.bindings {
                match bindings
                    .iter()
                    .position(|el| el.keystrokes.len() < new_binding.keystrokes.len())
                {
                    Some(idx) => {
                        bindings.insert(idx, new_binding);
                    }
                    None => bindings.push(new_binding),
                }
            }
            context_stack.pop();
        }

        KeymatchResult { bindings, pending }
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

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{Action, ActionRegistry, DispatchTree, KeyBinding, KeyContext, Keymap};

    #[derive(PartialEq, Eq)]
    struct TestAction;

    impl Action for TestAction {
        fn name(&self) -> &'static str {
            "test::TestAction"
        }

        fn debug_name() -> &'static str
        where
            Self: ::std::marker::Sized,
        {
            "test::TestAction"
        }

        fn partial_eq(&self, action: &dyn Action) -> bool {
            action
                .as_any()
                .downcast_ref::<Self>()
                .map_or(false, |a| self == a)
        }

        fn boxed_clone(&self) -> std::boxed::Box<dyn Action> {
            Box::new(TestAction)
        }

        fn as_any(&self) -> &dyn ::std::any::Any {
            self
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
}
