use crate::{
    Action, ActionRegistry, AnyTooltip, Bounds, ContentMask, CursorStyle, DispatchPhase,
    ElementContext, EntityId, FocusId, GlobalElementId, KeyBinding, KeyContext, KeyEvent, Keymap,
    KeymatchResult, Keystroke, KeystrokeMatcher, MouseEvent, Pixels, PlatformInputHandler,
    Primitive, Scene, SceneIndex, SmallVec, WindowContext,
};
use collections::FxHashMap;
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    iter,
    ops::Range,
    rc::Rc,
};

// pub(crate) struct Frame {
//     pub(crate) window_active: bool,

//     #[cfg(any(test, feature = "test-support"))]
//     pub(crate) debug_bounds: FxHashMap<String, Bounds<Pixels>>,
// }

pub struct Frame {
    elements: Vec<PaintedElement>,
    pub(crate) scene: Scene,
    focus: Option<FocusId>,
    pub(crate) window_active: bool,
    mouse_listeners: Vec<AnyMouseListener>,
    key_listeners: Vec<KeyListener>,
    action_listeners: Vec<ActionListener>,
    element_states: FxHashMap<GlobalElementId, ElementStateBox>,

    element_stack: Vec<PaintedElementId>,
    context_stack: Vec<KeyContext>,
    content_mask_stack: Vec<ContentMask<Pixels>>,
    focusable_node_ids: FxHashMap<FocusId, PaintedElementId>,
    view_node_ids: FxHashMap<EntityId, PaintedElementId>,
    keystroke_matchers: FxHashMap<SmallVec<[KeyContext; 4]>, KeystrokeMatcher>,
    keymap: Rc<RefCell<Keymap>>,
    action_registry: Rc<ActionRegistry>,
}

impl Frame {
    pub fn new(keymap: Rc<RefCell<Keymap>>, action_registry: Rc<ActionRegistry>) -> Self {
        Frame {
            keymap,
            action_registry,
            elements: Vec::new(),
            scene: Scene::default(),
            focus: None,
            window_active: false,
            mouse_listeners: Vec::new(),
            key_listeners: Vec::new(),
            action_listeners: Vec::new(),
            element_states: FxHashMap::default(),
            element_stack: Vec::new(),
            context_stack: Vec::new(),
            content_mask_stack: Vec::new(),
            focusable_node_ids: FxHashMap::default(),
            view_node_ids: FxHashMap::default(),
            keystroke_matchers: FxHashMap::default(),
        }
    }

    pub fn clear(&mut self) {
        self.elements.clear();
        self.scene.clear();
        self.focus = None;
        self.mouse_listeners.clear();
        self.key_listeners.clear();
        self.action_listeners.clear();
        self.element_states.clear();
        self.element_stack.clear();
        self.context_stack.clear();
        self.content_mask_stack.clear();
        self.focusable_node_ids.clear();
        self.view_node_ids.clear();
        self.keystroke_matchers.clear();
    }

    pub fn clear_pending_keystrokes(&mut self) {
        self.keystroke_matchers.clear();
    }

    /// Preserve keystroke matchers from previous frames to support multi-stroke
    /// bindings across multiple frames.
    pub fn preserve_pending_keystrokes(
        &mut self,
        prev_frame: &mut Self,
        focus_id: Option<FocusId>,
    ) {
        self.context_stack.clear();
        for element in self.dispatch_path(focus_id) {
            if let Some(context) = element.key_context.clone() {
                self.context_stack.push(context);
            }

            if let Some((context_stack, matcher)) = prev_frame
                .keystroke_matchers
                .remove_entry(self.context_stack.as_slice())
            {
                self.keystroke_matchers.insert(context_stack, matcher);
            }
        }
    }

    pub fn set_focus(&mut self, focus_id: Option<FocusId>) {
        self.focus = focus_id;
    }

    pub fn set_window_active(&mut self, active: bool) {
        self.window_active = active;
    }

    pub fn window_active(&self) -> bool {
        self.window_active
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
                current_node_id = self.elements[node_id.0].parent;
            }
        }
        false
    }

    pub fn focus_path(&self) -> SmallVec<[FocusId; 8]> {
        let Some(focus_id) = self.focus else {
            return SmallVec::new();
        };

        let mut focus_path = self
            .dispatch_path(Some(focus_id))
            .flat_map(|element| element.focus_id)
            .collect::<SmallVec<[FocusId; 8]>>();
        focus_path.reverse(); // Reverse the path so it goes from the root to the focused node.
        focus_path
    }

    pub fn view_path(&self, view_id: EntityId) -> SmallVec<[EntityId; 8]> {
        let Some(element_id) = self.view_node_ids.get(&view_id) else {
            return SmallVec::new();
        };

        let mut view_path = self
            .ancestors(Some(*element_id))
            .flat_map(|element| element.view_id)
            .collect::<SmallVec<[EntityId; 8]>>();
        view_path.reverse(); // Reverse the path so it goes from the root to the focused node.
        view_path
    }

    pub fn action_dispatch_path(&self, focus_id: Option<FocusId>) -> SmallVec<[ActionListener; 8]> {
        let mut action_dispatch_path = self
            .dispatch_path(focus_id)
            .flat_map(|element| {
                self.action_listeners[element.action_listeners.clone()]
                    .iter()
                    .cloned()
            })
            .collect::<SmallVec<[ActionListener; 8]>>();
        action_dispatch_path.reverse(); // Reverse the path so it goes from the root to the focused node.
        action_dispatch_path
    }

    pub fn key_dispatch_path(&self, focus_id: Option<FocusId>) -> SmallVec<[KeyListener; 8]> {
        let mut key_dispatch_path: SmallVec<[KeyListener; 8]> = self
            .dispatch_path(focus_id)
            .flat_map(|element| {
                self.key_listeners[element.key_listeners.clone()]
                    .iter()
                    .cloned()
            })
            .collect::<SmallVec<[KeyListener; 8]>>();
        key_dispatch_path.reverse(); // Reverse the path so it goes from the root to the focused node.
        key_dispatch_path
    }

    pub fn available_actions(&self, focus_id: Option<FocusId>) -> Vec<Box<dyn Action>> {
        let mut actions = Vec::<Box<dyn Action>>::new();
        for ActionListener { action_type, .. } in self.action_dispatch_path(focus_id) {
            if let Err(ix) = actions.binary_search_by_key(&action_type, |a| a.as_any().type_id()) {
                // Intentionally silence these errors without logging.
                // If an action cannot be built by default, it's not available.
                let action = self.action_registry.build_action_type(&action_type).ok();
                if let Some(action) = action {
                    actions.insert(ix, action);
                }
            }
        }
        actions
    }

    pub fn bindings_for_action(
        &self,
        action: &dyn Action,
        focus_id: Option<FocusId>,
    ) -> Vec<KeyBinding> {
        let context_stack = self
            .dispatch_path(focus_id)
            .flat_map(|element| element.key_context.clone())
            .collect::<SmallVec<[KeyContext; 8]>>();

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

    pub fn is_action_available(&self, action: &dyn Action, focus_id: Option<FocusId>) -> bool {
        for element in self.dispatch_path(focus_id) {
            if self.action_listeners[element.action_listeners.clone()]
                .iter()
                .any(|listener| listener.action_type == action.as_any().type_id())
            {
                return true;
            }
        }
        false
    }

    pub fn match_keystroke(
        &mut self,
        keystroke: &Keystroke,
        focus_id: Option<FocusId>,
    ) -> KeymatchResult {
        let mut bindings = SmallVec::<[KeyBinding; 1]>::new();
        let mut pending = false;

        let mut context_stack: SmallVec<[KeyContext; 4]> = SmallVec::new();

        for element in self.dispatch_path(focus_id) {
            if let Some(context) = element.key_context.clone() {
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

    fn dispatch_path(&self, focus_id: Option<FocusId>) -> impl Iterator<Item = &PaintedElement> {
        let mut current_node_id = focus_id
            .and_then(|focus_id| self.focusable_node_ids.get(&focus_id).copied())
            .or_else(|| self.elements.is_empty().then(|| PaintedElementId(0)));
        self.ancestors(current_node_id)
    }

    fn ancestors(
        &self,
        mut current_node_id: Option<PaintedElementId>,
    ) -> impl Iterator<Item = &PaintedElement> {
        iter::from_fn(move || {
            let node_id = current_node_id?;
            current_node_id = self.elements[node_id.0].parent;
            Some(&self.elements[node_id.0])
        })
    }

    pub fn push_element(&mut self) {
        let parent = self.element_stack.last().copied();
        let element_id = PaintedElementId(self.elements.len());
        let scene_index = self.scene.current_index();
        self.elements.push(PaintedElement {
            parent,
            scene_primitives: scene_index.clone()..scene_index,
            mouse_listeners: self.mouse_listeners.len()..self.mouse_listeners.len(),
            key_listeners: self.key_listeners.len()..self.key_listeners.len(),
            action_listeners: self.action_listeners.len()..self.action_listeners.len(),
            ..Default::default()
        });
        self.element_stack.push(element_id);
    }

    pub fn pop_element(&mut self) {
        let element = &self.elements[self.active_element_id().0];
        if element.key_context.is_some() {
            self.context_stack.pop();
        }
        self.element_stack.pop();
    }

    pub fn set_key_context(&mut self, context: KeyContext) {
        let element_id = self.active_element_id();
        self.elements[element_id.0].key_context = Some(context.clone());
        self.context_stack.push(context);
    }

    pub fn set_focus_id(&mut self, focus_id: FocusId) {
        let element_id = self.active_element_id();
        self.elements[element_id.0].focus_id = Some(focus_id);
        self.focusable_node_ids.insert(focus_id, element_id);
    }

    pub fn set_view_id(&mut self, view_id: EntityId) {
        let element_id = self.active_element_id();
        self.elements[element_id.0].view_id = Some(view_id);
        self.view_node_ids.insert(view_id, element_id);
    }

    pub fn paint_primitive<P: Into<Primitive>>(&mut self, build_primitive: impl FnOnce(u32) -> P) {
        self.scene.paint_primitive(build_primitive);
        let element_id = self.active_element_id();
        self.elements[element_id.0].scene_primitives.end = self.scene.current_index();
    }

    pub fn on_mouse_event<E: MouseEvent>(
        &mut self,
        mut listener: impl 'static + FnMut(&E, DispatchPhase, &mut WindowContext),
    ) {
        self.mouse_listeners.push(Rc::new(move |event, phase, cx| {
            if let Some(event) = event.downcast_ref::<E>() {
                listener(event, phase, cx);
            }
        }));
        self.active_element().mouse_listeners.end += 1;
    }

    pub fn on_key_event<E: KeyEvent>(
        &mut self,
        listener: impl Fn(&E, DispatchPhase, &mut WindowContext) + 'static,
    ) {
        self.key_listeners.push(Rc::new(|event, phase, cx| {
            if let Some(event) = event.downcast_ref::<E>() {
                listener(event, phase, cx);
            }
        }));
        self.active_element().key_listeners.end += 1;
    }

    pub fn on_action(
        &mut self,
        action_type: TypeId,
        listener: Rc<dyn Fn(&dyn Any, DispatchPhase, &mut WindowContext) + 'static>,
    ) {
        self.action_listeners.push(ActionListener {
            action_type,
            listener: Rc::new(|event, phase, cx| listener(event, phase, cx)),
        });

        self.active_element().action_listeners.end += 1;
    }

    pub fn set_input_handler(&mut self, handler: Option<PlatformInputHandler>) {
        self.active_element().input_handler = handler;
    }

    pub fn set_tooltip(&mut self, tooltip: Option<AnyTooltip>) {
        self.active_element().tooltip = tooltip;
    }

    pub fn set_cursor_style(&mut self, cursor_style: Option<CursorStyle>) {
        self.active_element().cursor_style = cursor_style;
    }

    fn active_element_id(&self) -> PaintedElementId {
        self.element_stack
            .last()
            .copied()
            .expect("There should be an active element")
    }

    fn active_element(&mut self) -> &mut PaintedElement {
        let element_id = self.active_element_id();
        &mut self.elements[element_id.0]
    }
}

#[derive(Default)]
struct PaintedElement {
    id: Option<GlobalElementId>,
    bounds: Bounds<Pixels>,
    content_mask: ContentMask<Pixels>,
    opaque: bool,
    scene_primitives: Range<SceneIndex>,
    mouse_listeners: Range<usize>,
    key_listeners: Range<usize>,
    action_listeners: Range<usize>,
    input_handler: Option<PlatformInputHandler>,
    tooltip: Option<AnyTooltip>,
    cursor_style: Option<CursorStyle>,
    key_context: Option<KeyContext>,
    focus_id: Option<FocusId>,
    view_id: Option<EntityId>,
    parent: Option<PaintedElementId>,
}

pub(crate) struct ElementStateBox {
    pub(crate) inner: Box<dyn Any>,
    pub(crate) parent_view_id: EntityId,
    #[cfg(debug_assertions)]
    pub(crate) type_name: &'static str,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct PaintedElementId(usize);

type AnyMouseListener = Rc<dyn Fn(&dyn Any, DispatchPhase, &mut ElementContext) + 'static>;

type KeyListener = Rc<dyn Fn(&dyn Any, DispatchPhase, &mut ElementContext)>;

#[derive(Clone)]
pub(crate) struct ActionListener {
    pub(crate) action_type: TypeId,
    pub(crate) listener: Rc<dyn Fn(&dyn Any, DispatchPhase, &mut WindowContext)>,
}
