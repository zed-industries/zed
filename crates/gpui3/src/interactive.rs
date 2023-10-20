use crate::{
    point, px, Action, AppContext, BorrowWindow, Bounds, DispatchContext, DispatchPhase, Element,
    ElementId, FocusHandle, KeyMatch, Keystroke, Modifiers, Overflow, Pixels, Point, SharedString,
    Size, Style, StyleRefinement, ViewContext,
};
use collections::HashMap;
use derive_more::{Deref, DerefMut};
use parking_lot::Mutex;
use refineable::Refineable;
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    fmt::Debug,
    ops::Deref,
    sync::Arc,
};

pub trait StatelessInteractive: Element {
    fn stateless_interactivity(&mut self) -> &mut StatelessInteraction<Self::ViewState>;

    fn hover(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self
    where
        Self: Sized,
    {
        self.stateless_interactivity().hover_style = f(StyleRefinement::default());
        self
    }

    fn group_hover(
        mut self,
        group_name: impl Into<SharedString>,
        f: impl FnOnce(StyleRefinement) -> StyleRefinement,
    ) -> Self
    where
        Self: Sized,
    {
        self.stateless_interactivity().group_hover_style = Some(GroupStyle {
            group: group_name.into(),
            style: f(StyleRefinement::default()),
        });
        self
    }

    fn on_mouse_down(
        mut self,
        button: MouseButton,
        handler: impl Fn(&mut Self::ViewState, &MouseDownEvent, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.stateless_interactivity()
            .mouse_down_listeners
            .push(Arc::new(move |view, event, bounds, phase, cx| {
                if phase == DispatchPhase::Bubble
                    && event.button == button
                    && bounds.contains_point(&event.position)
                {
                    handler(view, event, cx)
                }
            }));
        self
    }

    fn on_mouse_up(
        mut self,
        button: MouseButton,
        handler: impl Fn(&mut Self::ViewState, &MouseUpEvent, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.stateless_interactivity()
            .mouse_up_listeners
            .push(Arc::new(move |view, event, bounds, phase, cx| {
                if phase == DispatchPhase::Bubble
                    && event.button == button
                    && bounds.contains_point(&event.position)
                {
                    handler(view, event, cx)
                }
            }));
        self
    }

    fn on_mouse_down_out(
        mut self,
        button: MouseButton,
        handler: impl Fn(&mut Self::ViewState, &MouseDownEvent, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.stateless_interactivity()
            .mouse_down_listeners
            .push(Arc::new(move |view, event, bounds, phase, cx| {
                if phase == DispatchPhase::Capture
                    && event.button == button
                    && !bounds.contains_point(&event.position)
                {
                    handler(view, event, cx)
                }
            }));
        self
    }

    fn on_mouse_up_out(
        mut self,
        button: MouseButton,
        handler: impl Fn(&mut Self::ViewState, &MouseUpEvent, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.stateless_interactivity()
            .mouse_up_listeners
            .push(Arc::new(move |view, event, bounds, phase, cx| {
                if phase == DispatchPhase::Capture
                    && event.button == button
                    && !bounds.contains_point(&event.position)
                {
                    handler(view, event, cx);
                }
            }));
        self
    }

    fn on_mouse_move(
        mut self,
        handler: impl Fn(&mut Self::ViewState, &MouseMoveEvent, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.stateless_interactivity()
            .mouse_move_listeners
            .push(Arc::new(move |view, event, bounds, phase, cx| {
                if phase == DispatchPhase::Bubble && bounds.contains_point(&event.position) {
                    handler(view, event, cx);
                }
            }));
        self
    }

    fn on_scroll_wheel(
        mut self,
        handler: impl Fn(&mut Self::ViewState, &ScrollWheelEvent, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.stateless_interactivity()
            .scroll_wheel_listeners
            .push(Arc::new(move |view, event, bounds, phase, cx| {
                if phase == DispatchPhase::Bubble && bounds.contains_point(&event.position) {
                    handler(view, event, cx);
                }
            }));
        self
    }

    fn context<C>(mut self, context: C) -> Self
    where
        Self: Sized,
        C: TryInto<DispatchContext>,
        C::Error: Debug,
    {
        self.stateless_interactivity().dispatch_context =
            context.try_into().expect("invalid dispatch context");
        self
    }

    fn on_action<A: 'static>(
        mut self,
        listener: impl Fn(&mut Self::ViewState, &A, DispatchPhase, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.stateless_interactivity().key_listeners.push((
            TypeId::of::<A>(),
            Arc::new(move |view, event, _, phase, cx| {
                let event = event.downcast_ref().unwrap();
                listener(view, event, phase, cx);
                None
            }),
        ));
        self
    }

    fn on_key_down(
        mut self,
        listener: impl Fn(
                &mut Self::ViewState,
                &KeyDownEvent,
                DispatchPhase,
                &mut ViewContext<Self::ViewState>,
            ) + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.stateless_interactivity().key_listeners.push((
            TypeId::of::<KeyDownEvent>(),
            Arc::new(move |view, event, _, phase, cx| {
                let event = event.downcast_ref().unwrap();
                listener(view, event, phase, cx);
                None
            }),
        ));
        self
    }

    fn on_key_up(
        mut self,
        listener: impl Fn(&mut Self::ViewState, &KeyUpEvent, DispatchPhase, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.stateless_interactivity().key_listeners.push((
            TypeId::of::<KeyUpEvent>(),
            Arc::new(move |view, event, _, phase, cx| {
                let event = event.downcast_ref().unwrap();
                listener(view, event, phase, cx);
                None
            }),
        ));
        self
    }
}

pub trait StatefulInteractive: StatelessInteractive {
    fn stateful_interactivity(&mut self) -> &mut StatefulInteraction<Self::ViewState>;

    fn active(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self
    where
        Self: Sized,
    {
        self.stateful_interactivity().active_style = f(StyleRefinement::default());
        self
    }

    fn group_active(
        mut self,
        group_name: impl Into<SharedString>,
        f: impl FnOnce(StyleRefinement) -> StyleRefinement,
    ) -> Self
    where
        Self: Sized,
    {
        self.stateful_interactivity().group_active_style = Some(GroupStyle {
            group: group_name.into(),
            style: f(StyleRefinement::default()),
        });
        self
    }

    fn on_click(
        mut self,
        handler: impl Fn(&mut Self::ViewState, &MouseClickEvent, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.stateful_interactivity()
            .mouse_click_listeners
            .push(Arc::new(move |view, event, cx| handler(view, event, cx)));
        self
    }
}

pub trait ElementInteraction<V: 'static + Send + Sync>: 'static + Send + Sync {
    fn as_stateless(&self) -> &StatelessInteraction<V>;
    fn as_stateless_mut(&mut self) -> &mut StatelessInteraction<V>;
    fn as_stateful(&self) -> Option<&StatefulInteraction<V>>;
    fn as_stateful_mut(&mut self) -> Option<&mut StatefulInteraction<V>>;

    fn initialize<R>(
        &mut self,
        cx: &mut ViewContext<V>,
        f: impl FnOnce(&mut ViewContext<V>) -> R,
    ) -> R {
        if let Some(stateful) = self.as_stateful_mut() {
            cx.with_element_id(stateful.id.clone(), |global_id, cx| {
                stateful.key_listeners.push((
                    TypeId::of::<KeyDownEvent>(),
                    Arc::new(move |_, key_down, context, phase, cx| {
                        if phase == DispatchPhase::Bubble {
                            let key_down = key_down.downcast_ref::<KeyDownEvent>().unwrap();
                            if let KeyMatch::Some(action) =
                                cx.match_keystroke(&global_id, &key_down.keystroke, context)
                            {
                                return Some(action);
                            }
                        }

                        None
                    }),
                ));
                let result = stateful.stateless.initialize(cx, f);
                stateful.key_listeners.pop();
                result
            })
        } else {
            let stateless = self.as_stateless();
            cx.with_key_dispatch_context(stateless.dispatch_context.clone(), |cx| {
                cx.with_key_listeners(&stateless.key_listeners, f)
            })
        }
    }

    fn refine_style(
        &self,
        style: &mut Style,
        bounds: Bounds<Pixels>,
        element_state: &InteractiveElementState,
        cx: &mut ViewContext<V>,
    ) {
        let mouse_position = cx.mouse_position();
        let stateless = self.as_stateless();
        if let Some(group_hover) = stateless.group_hover_style.as_ref() {
            if let Some(group_bounds) = GroupBounds::get(&group_hover.group, cx) {
                if group_bounds.contains_point(&mouse_position) {
                    style.refine(&group_hover.style);
                }
            }
        }
        if bounds.contains_point(&mouse_position) {
            style.refine(&stateless.hover_style);
        }

        if let Some(stateful) = self.as_stateful() {
            let active_state = element_state.active_state.lock();
            if active_state.group {
                if let Some(group_style) = stateful.group_active_style.as_ref() {
                    style.refine(&group_style.style);
                }
            }
            if active_state.element {
                style.refine(&stateful.active_style);
            }
        }
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        content_size: Size<Pixels>,
        overflow: Point<Overflow>,
        element_state: &mut InteractiveElementState,
        cx: &mut ViewContext<V>,
    ) {
        let stateless = self.as_stateless();
        for listener in stateless.mouse_down_listeners.iter().cloned() {
            cx.on_mouse_event(move |state, event: &MouseDownEvent, phase, cx| {
                listener(state, event, &bounds, phase, cx);
            })
        }

        for listener in stateless.mouse_up_listeners.iter().cloned() {
            cx.on_mouse_event(move |state, event: &MouseUpEvent, phase, cx| {
                listener(state, event, &bounds, phase, cx);
            })
        }

        for listener in stateless.mouse_move_listeners.iter().cloned() {
            cx.on_mouse_event(move |state, event: &MouseMoveEvent, phase, cx| {
                listener(state, event, &bounds, phase, cx);
            })
        }

        for listener in stateless.scroll_wheel_listeners.iter().cloned() {
            cx.on_mouse_event(move |state, event: &ScrollWheelEvent, phase, cx| {
                listener(state, event, &bounds, phase, cx);
            })
        }

        let hover_group_bounds = stateless
            .group_hover_style
            .as_ref()
            .and_then(|group_hover| GroupBounds::get(&group_hover.group, cx));

        if let Some(group_bounds) = hover_group_bounds {
            paint_hover_listener(group_bounds, cx);
        }

        if stateless.hover_style.is_some() {
            paint_hover_listener(bounds, cx);
        }

        if let Some(stateful) = self.as_stateful() {
            let click_listeners = stateful.mouse_click_listeners.clone();

            let pending_click = element_state.pending_click.clone();
            let mouse_down = pending_click.lock().clone();
            if let Some(mouse_down) = mouse_down {
                cx.on_mouse_event(move |state, event: &MouseUpEvent, phase, cx| {
                    if phase == DispatchPhase::Bubble && bounds.contains_point(&event.position) {
                        let mouse_click = MouseClickEvent {
                            down: mouse_down.clone(),
                            up: event.clone(),
                        };
                        for listener in &click_listeners {
                            listener(state, &mouse_click, cx);
                        }
                    }

                    *pending_click.lock() = None;
                });
            } else {
                cx.on_mouse_event(move |_state, event: &MouseDownEvent, phase, _cx| {
                    if phase == DispatchPhase::Bubble && bounds.contains_point(&event.position) {
                        *pending_click.lock() = Some(event.clone());
                    }
                });
            }

            let active_state = element_state.active_state.clone();
            if active_state.lock().is_none() {
                let active_group_bounds = stateful
                    .group_active_style
                    .as_ref()
                    .and_then(|group_active| GroupBounds::get(&group_active.group, cx));
                cx.on_mouse_event(move |_view, down: &MouseDownEvent, phase, cx| {
                    if phase == DispatchPhase::Bubble {
                        let group = active_group_bounds
                            .map_or(false, |bounds| bounds.contains_point(&down.position));
                        let element = bounds.contains_point(&down.position);
                        if group || element {
                            *active_state.lock() = ActiveState { group, element };
                            cx.notify();
                        }
                    }
                });
            } else {
                cx.on_mouse_event(move |_, _: &MouseUpEvent, phase, cx| {
                    if phase == DispatchPhase::Capture {
                        *active_state.lock() = ActiveState::default();
                        cx.notify();
                    }
                });
            }

            if overflow.x == Overflow::Scroll || overflow.y == Overflow::Scroll {
                let scroll_offset = element_state
                    .scroll_offset
                    .get_or_insert_with(Arc::default)
                    .clone();
                let line_height = cx.line_height();
                let scroll_max = content_size - bounds.size;

                cx.on_mouse_event(move |_, event: &ScrollWheelEvent, _, cx| {
                    if bounds.contains_point(&event.position) {
                        let mut scroll_offset = scroll_offset.lock();
                        let delta = event.delta.pixel_delta(line_height);

                        if overflow.x == Overflow::Scroll {
                            scroll_offset.x =
                                (scroll_offset.x - delta.x).clamp(px(0.), scroll_max.width);
                        }

                        if overflow.y == Overflow::Scroll {
                            scroll_offset.y =
                                (scroll_offset.y - delta.y).clamp(px(0.), scroll_max.height);
                        }

                        cx.notify();
                    }
                });
            }
        }
    }
}

fn paint_hover_listener<V>(bounds: Bounds<Pixels>, cx: &mut ViewContext<V>)
where
    V: 'static + Send + Sync,
{
    let hovered = bounds.contains_point(&cx.mouse_position());
    cx.on_mouse_event(move |_, event: &MouseMoveEvent, phase, cx| {
        if phase == DispatchPhase::Capture {
            if bounds.contains_point(&event.position) != hovered {
                cx.notify();
            }
        }
    });
}

#[derive(Deref, DerefMut)]
pub struct StatefulInteraction<V: 'static + Send + Sync> {
    pub id: ElementId,
    #[deref]
    #[deref_mut]
    stateless: StatelessInteraction<V>,
    pub mouse_click_listeners: SmallVec<[MouseClickListener<V>; 2]>,
    pub active_style: StyleRefinement,
    pub group_active_style: Option<GroupStyle>,
}

impl<V> ElementInteraction<V> for StatefulInteraction<V>
where
    V: 'static + Send + Sync,
{
    fn as_stateful(&self) -> Option<&StatefulInteraction<V>> {
        Some(self)
    }

    fn as_stateful_mut(&mut self) -> Option<&mut StatefulInteraction<V>> {
        Some(self)
    }

    fn as_stateless(&self) -> &StatelessInteraction<V> {
        &self.stateless
    }

    fn as_stateless_mut(&mut self) -> &mut StatelessInteraction<V> {
        &mut self.stateless
    }
}

impl<V> From<ElementId> for StatefulInteraction<V>
where
    V: 'static + Send + Sync,
{
    fn from(id: ElementId) -> Self {
        Self {
            id,
            stateless: StatelessInteraction::default(),
            mouse_click_listeners: SmallVec::new(),
            active_style: StyleRefinement::default(),
            group_active_style: None,
        }
    }
}

pub struct StatelessInteraction<V> {
    pub dispatch_context: DispatchContext,
    pub mouse_down_listeners: SmallVec<[MouseDownListener<V>; 2]>,
    pub mouse_up_listeners: SmallVec<[MouseUpListener<V>; 2]>,
    pub mouse_move_listeners: SmallVec<[MouseMoveListener<V>; 2]>,
    pub scroll_wheel_listeners: SmallVec<[ScrollWheelListener<V>; 2]>,
    pub key_listeners: SmallVec<[(TypeId, KeyListener<V>); 32]>,
    pub hover_style: StyleRefinement,
    pub group_hover_style: Option<GroupStyle>,
}

impl<V> StatelessInteraction<V>
where
    V: 'static + Send + Sync,
{
    pub fn into_stateful(self, id: impl Into<ElementId>) -> StatefulInteraction<V> {
        StatefulInteraction {
            id: id.into(),
            stateless: self,
            mouse_click_listeners: SmallVec::new(),
            active_style: StyleRefinement::default(),
            group_active_style: None,
        }
    }
}

pub struct GroupStyle {
    pub group: SharedString,
    pub style: StyleRefinement,
}

#[derive(Default)]
pub struct GroupBounds(HashMap<SharedString, SmallVec<[Bounds<Pixels>; 1]>>);

impl GroupBounds {
    pub fn get(name: &SharedString, cx: &mut AppContext) -> Option<Bounds<Pixels>> {
        cx.default_global::<Self>()
            .0
            .get(name)
            .and_then(|bounds_stack| bounds_stack.last())
            .cloned()
    }

    pub fn push(name: SharedString, bounds: Bounds<Pixels>, cx: &mut AppContext) {
        cx.default_global::<Self>()
            .0
            .entry(name)
            .or_default()
            .push(bounds);
    }

    pub fn pop(name: &SharedString, cx: &mut AppContext) {
        cx.default_global::<GroupBounds>()
            .0
            .get_mut(name)
            .unwrap()
            .pop();
    }
}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
struct ActiveState {
    pub group: bool,
    pub element: bool,
}

impl ActiveState {
    pub fn is_none(&self) -> bool {
        !self.group && !self.element
    }
}

#[derive(Default)]
pub struct InteractiveElementState {
    active_state: Arc<Mutex<ActiveState>>,
    pending_click: Arc<Mutex<Option<MouseDownEvent>>>,
    scroll_offset: Option<Arc<Mutex<Point<Pixels>>>>,
}

impl InteractiveElementState {
    pub fn scroll_offset(&self) -> Option<Point<Pixels>> {
        self.scroll_offset
            .as_ref()
            .map(|offset| offset.lock().clone())
    }
}

impl<V> Default for StatelessInteraction<V> {
    fn default() -> Self {
        Self {
            dispatch_context: DispatchContext::default(),
            mouse_down_listeners: SmallVec::new(),
            mouse_up_listeners: SmallVec::new(),
            mouse_move_listeners: SmallVec::new(),
            scroll_wheel_listeners: SmallVec::new(),
            key_listeners: SmallVec::new(),
            hover_style: StyleRefinement::default(),
            group_hover_style: None,
        }
    }
}

impl<V> ElementInteraction<V> for StatelessInteraction<V>
where
    V: 'static + Send + Sync,
{
    fn as_stateful(&self) -> Option<&StatefulInteraction<V>> {
        None
    }

    fn as_stateful_mut(&mut self) -> Option<&mut StatefulInteraction<V>> {
        None
    }

    fn as_stateless(&self) -> &StatelessInteraction<V> {
        self
    }

    fn as_stateless_mut(&mut self) -> &mut StatelessInteraction<V> {
        self
    }
}

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
