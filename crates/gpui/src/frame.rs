use crate::{
    size, ActionRegistry, AnyTooltip, Bounds, BoxShadow, ContentMask, Corners, CursorStyle,
    DispatchPhase, Edges, ElementContext, EntityId, FocusId, FontId, GlobalElementId, GlyphId,
    Hsla, KeyContext, Keymap, KeystrokeMatcher, MonochromeSprite, Path, Pixels,
    PlatformInputHandler, Point, Primitive, Quad, Scene, SceneIndex, Shadow, SmallVec,
    StrikethroughStyle, Underline, UnderlineStyle, WindowContext,
};
use anyhow::Result;
use collections::FxHashMap;
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    ops::Range,
    rc::Rc,
};

#[derive(Default)]
pub struct Frame {
    elements: Vec<PaintedElement>,
    scene: Scene,
    mouse_listeners: Vec<AnyMouseListener>,
    key_listeners: Vec<KeyListener>,
    action_listeners: Vec<ActionListener>,

    element_stack: Vec<PaintedElementId>,
    context_stack: Vec<KeyContext>,
    content_mask_stack: Vec<ContentMask<Pixels>>,
    focusable_node_ids: FxHashMap<FocusId, PaintedElementId>,
    view_node_ids: FxHashMap<EntityId, PaintedElementId>,
    keystroke_matchers: FxHashMap<SmallVec<[KeyContext; 4]>, KeystrokeMatcher>,
    keymap: Rc<RefCell<Keymap>>,
    action_registry: Rc<ActionRegistry>,
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

#[derive(Copy, Clone)]
struct PaintedElementId(usize);

type AnyMouseListener = Rc<dyn Fn(&dyn Any, DispatchPhase, &mut ElementContext) + 'static>;

type KeyListener = Rc<dyn Fn(&dyn Any, DispatchPhase, &mut ElementContext)>;

#[derive(Clone)]
pub(crate) struct ActionListener {
    pub(crate) action_type: TypeId,
    pub(crate) listener: Rc<dyn Fn(&dyn Any, DispatchPhase, &mut WindowContext)>,
}

impl Frame {
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

    pub fn visit_element(&mut self) {}

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

    fn active_element_id(&self) -> PaintedElementId {
        self.element_stack
            .last()
            .copied()
            .expect("There should be an active element")
    }
}
