use crate::{
    point, Action, Bounds, DispatchContext, DispatchPhase, FocusHandle, Keystroke, Modifiers,
    Pixels, Point, ViewContext,
};
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    ops::Deref,
    sync::Arc,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyDownEvent {
    pub keystroke: Keystroke,
    pub is_held: bool,
}

#[derive(Clone, Debug)]
pub struct KeyUpEvent {
    pub keystroke: Keystroke,
}

#[derive(Clone, Debug, Default)]
pub struct ModifiersChangedEvent {
    pub modifiers: Modifiers,
}

impl Deref for ModifiersChangedEvent {
    type Target = Modifiers;

    fn deref(&self) -> &Self::Target {
        &self.modifiers
    }
}

/// The phase of a touch motion event.
/// Based on the winit enum of the same name.
#[derive(Clone, Copy, Debug)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
}

#[derive(Clone, Debug, Default)]
pub struct MouseDownEvent {
    pub button: MouseButton,
    pub position: Point<Pixels>,
    pub modifiers: Modifiers,
    pub click_count: usize,
}

#[derive(Clone, Debug, Default)]
pub struct MouseUpEvent {
    pub button: MouseButton,
    pub position: Point<Pixels>,
    pub modifiers: Modifiers,
    pub click_count: usize,
}

#[derive(Clone, Debug, Default)]
pub struct MouseClickEvent {
    pub down: MouseDownEvent,
    pub up: MouseUpEvent,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Navigate(NavigationDirection),
}

impl MouseButton {
    pub fn all() -> Vec<Self> {
        vec![
            MouseButton::Left,
            MouseButton::Right,
            MouseButton::Middle,
            MouseButton::Navigate(NavigationDirection::Back),
            MouseButton::Navigate(NavigationDirection::Forward),
        ]
    }
}

impl Default for MouseButton {
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub enum NavigationDirection {
    Back,
    Forward,
}

impl Default for NavigationDirection {
    fn default() -> Self {
        Self::Back
    }
}

#[derive(Clone, Debug, Default)]
pub struct MouseMoveEvent {
    pub position: Point<Pixels>,
    pub pressed_button: Option<MouseButton>,
    pub modifiers: Modifiers,
}

#[derive(Clone, Debug)]
pub struct ScrollWheelEvent {
    pub position: Point<Pixels>,
    pub delta: ScrollDelta,
    pub modifiers: Modifiers,
    pub touch_phase: TouchPhase,
}

impl Deref for ScrollWheelEvent {
    type Target = Modifiers;

    fn deref(&self) -> &Self::Target {
        &self.modifiers
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ScrollDelta {
    Pixels(Point<Pixels>),
    Lines(Point<f32>),
}

impl Default for ScrollDelta {
    fn default() -> Self {
        Self::Lines(Default::default())
    }
}

impl ScrollDelta {
    pub fn precise(&self) -> bool {
        match self {
            ScrollDelta::Pixels(_) => true,
            ScrollDelta::Lines(_) => false,
        }
    }

    pub fn pixel_delta(&self, line_height: Pixels) -> Point<Pixels> {
        match self {
            ScrollDelta::Pixels(delta) => *delta,
            ScrollDelta::Lines(delta) => point(line_height * delta.x, line_height * delta.y),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct MouseExitEvent {
    pub position: Point<Pixels>,
    pub pressed_button: Option<MouseButton>,
    pub modifiers: Modifiers,
}

impl Deref for MouseExitEvent {
    type Target = Modifiers;

    fn deref(&self) -> &Self::Target {
        &self.modifiers
    }
}

#[derive(Clone, Debug)]
pub enum InputEvent {
    KeyDown(KeyDownEvent),
    KeyUp(KeyUpEvent),
    ModifiersChanged(ModifiersChangedEvent),
    MouseDown(MouseDownEvent),
    MouseUp(MouseUpEvent),
    MouseMoved(MouseMoveEvent),
    MouseExited(MouseExitEvent),
    ScrollWheel(ScrollWheelEvent),
}

impl InputEvent {
    pub fn position(&self) -> Option<Point<Pixels>> {
        match self {
            InputEvent::KeyDown { .. } => None,
            InputEvent::KeyUp { .. } => None,
            InputEvent::ModifiersChanged { .. } => None,
            InputEvent::MouseDown(event) => Some(event.position),
            InputEvent::MouseUp(event) => Some(event.position),
            InputEvent::MouseMoved(event) => Some(event.position),
            InputEvent::MouseExited(event) => Some(event.position),
            InputEvent::ScrollWheel(event) => Some(event.position),
        }
    }

    pub fn mouse_event<'a>(&'a self) -> Option<&'a dyn Any> {
        match self {
            InputEvent::KeyDown { .. } => None,
            InputEvent::KeyUp { .. } => None,
            InputEvent::ModifiersChanged { .. } => None,
            InputEvent::MouseDown(event) => Some(event),
            InputEvent::MouseUp(event) => Some(event),
            InputEvent::MouseMoved(event) => Some(event),
            InputEvent::MouseExited(event) => Some(event),
            InputEvent::ScrollWheel(event) => Some(event),
        }
    }

    pub fn keyboard_event<'a>(&'a self) -> Option<&'a dyn Any> {
        match self {
            InputEvent::KeyDown(event) => Some(event),
            InputEvent::KeyUp(event) => Some(event),
            InputEvent::ModifiersChanged(event) => Some(event),
            InputEvent::MouseDown(_) => None,
            InputEvent::MouseUp(_) => None,
            InputEvent::MouseMoved(_) => None,
            InputEvent::MouseExited(_) => None,
            InputEvent::ScrollWheel(_) => None,
        }
    }
}

pub struct FocusEvent {
    pub blurred: Option<FocusHandle>,
    pub focused: Option<FocusHandle>,
}

pub type MouseDownListener<V> = Arc<
    dyn Fn(&mut V, &MouseDownEvent, &Bounds<Pixels>, DispatchPhase, &mut ViewContext<V>)
        + Send
        + Sync
        + 'static,
>;
pub type MouseUpListener<V> = Arc<
    dyn Fn(&mut V, &MouseUpEvent, &Bounds<Pixels>, DispatchPhase, &mut ViewContext<V>)
        + Send
        + Sync
        + 'static,
>;
pub type MouseClickListener<V> =
    Arc<dyn Fn(&mut V, &MouseClickEvent, &mut ViewContext<V>) + Send + Sync + 'static>;

pub type MouseMoveListener<V> = Arc<
    dyn Fn(&mut V, &MouseMoveEvent, &Bounds<Pixels>, DispatchPhase, &mut ViewContext<V>)
        + Send
        + Sync
        + 'static,
>;

pub type ScrollWheelListener<V> = Arc<
    dyn Fn(&mut V, &ScrollWheelEvent, &Bounds<Pixels>, DispatchPhase, &mut ViewContext<V>)
        + Send
        + Sync
        + 'static,
>;

pub type KeyListener<V> = Arc<
    dyn Fn(
            &mut V,
            &dyn Any,
            &[&DispatchContext],
            DispatchPhase,
            &mut ViewContext<V>,
        ) -> Option<Box<dyn Action>>
        + Send
        + Sync
        + 'static,
>;

pub type FocusListener<V> =
    Arc<dyn Fn(&mut V, &FocusEvent, &mut ViewContext<V>) + Send + Sync + 'static>;

pub struct EventListeners<V: 'static> {
    pub mouse_down: SmallVec<[MouseDownListener<V>; 2]>,
    pub mouse_up: SmallVec<[MouseUpListener<V>; 2]>,
    pub mouse_click: SmallVec<[MouseClickListener<V>; 2]>,
    pub mouse_move: SmallVec<[MouseMoveListener<V>; 2]>,
    pub scroll_wheel: SmallVec<[ScrollWheelListener<V>; 2]>,
    pub key: SmallVec<[(TypeId, KeyListener<V>); 32]>,
    pub focus: SmallVec<[FocusListener<V>; 2]>,
}

impl<V> Default for EventListeners<V> {
    fn default() -> Self {
        Self {
            mouse_down: SmallVec::new(),
            mouse_up: SmallVec::new(),
            mouse_click: SmallVec::new(),
            mouse_move: SmallVec::new(),
            scroll_wheel: SmallVec::new(),
            key: SmallVec::new(),
            focus: SmallVec::new(),
        }
    }
}
