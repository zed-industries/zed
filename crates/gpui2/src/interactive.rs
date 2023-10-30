use crate::{
    point, px, Action, AnyBox, AnyDrag, AppContext, BorrowWindow, Bounds, Component,
    DispatchContext, DispatchPhase, Element, ElementId, FocusHandle, KeyMatch, Keystroke,
    Modifiers, Overflow, Pixels, Point, SharedString, Size, Style, StyleRefinement, View,
    ViewContext,
};
use collections::HashMap;
use derive_more::{Deref, DerefMut};
use parking_lot::Mutex;
use refineable::Refineable;
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    fmt::Debug,
    marker::PhantomData,
    mem,
    ops::Deref,
    path::PathBuf,
    sync::Arc,
};

const DRAG_THRESHOLD: f64 = 2.;

pub trait StatelessInteractive<V: 'static>: Element<V> {
    fn stateless_interaction(&mut self) -> &mut StatelessInteraction<V>;

    fn hover(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self
    where
        Self: Sized,
    {
        self.stateless_interaction().hover_style = f(StyleRefinement::default());
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
        self.stateless_interaction().group_hover_style = Some(GroupStyle {
            group: group_name.into(),
            style: f(StyleRefinement::default()),
        });
        self
    }

    fn on_mouse_down(
        mut self,
        button: MouseButton,
        handler: impl Fn(&mut V, &MouseDownEvent, &mut ViewContext<V>) + Send + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.stateless_interaction()
            .mouse_down_listeners
            .push(Box::new(move |view, event, bounds, phase, cx| {
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
        handler: impl Fn(&mut V, &MouseUpEvent, &mut ViewContext<V>) + Send + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.stateless_interaction()
            .mouse_up_listeners
            .push(Box::new(move |view, event, bounds, phase, cx| {
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
        handler: impl Fn(&mut V, &MouseDownEvent, &mut ViewContext<V>) + Send + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.stateless_interaction()
            .mouse_down_listeners
            .push(Box::new(move |view, event, bounds, phase, cx| {
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
        handler: impl Fn(&mut V, &MouseUpEvent, &mut ViewContext<V>) + Send + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.stateless_interaction()
            .mouse_up_listeners
            .push(Box::new(move |view, event, bounds, phase, cx| {
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
        handler: impl Fn(&mut V, &MouseMoveEvent, &mut ViewContext<V>) + Send + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.stateless_interaction()
            .mouse_move_listeners
            .push(Box::new(move |view, event, bounds, phase, cx| {
                if phase == DispatchPhase::Bubble && bounds.contains_point(&event.position) {
                    handler(view, event, cx);
                }
            }));
        self
    }

    fn on_scroll_wheel(
        mut self,
        handler: impl Fn(&mut V, &ScrollWheelEvent, &mut ViewContext<V>) + Send + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.stateless_interaction()
            .scroll_wheel_listeners
            .push(Box::new(move |view, event, bounds, phase, cx| {
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
        self.stateless_interaction().dispatch_context =
            context.try_into().expect("invalid dispatch context");
        self
    }

    fn on_action<A: 'static>(
        mut self,
        listener: impl Fn(&mut V, &A, DispatchPhase, &mut ViewContext<V>) + Send + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.stateless_interaction().key_listeners.push((
            TypeId::of::<A>(),
            Box::new(move |view, event, _, phase, cx| {
                let event = event.downcast_ref().unwrap();
                listener(view, event, phase, cx);
                None
            }),
        ));
        self
    }

    fn on_key_down(
        mut self,
        listener: impl Fn(&mut V, &KeyDownEvent, DispatchPhase, &mut ViewContext<V>) + Send + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.stateless_interaction().key_listeners.push((
            TypeId::of::<KeyDownEvent>(),
            Box::new(move |view, event, _, phase, cx| {
                let event = event.downcast_ref().unwrap();
                listener(view, event, phase, cx);
                None
            }),
        ));
        self
    }

    fn on_key_up(
        mut self,
        listener: impl Fn(&mut V, &KeyUpEvent, DispatchPhase, &mut ViewContext<V>) + Send + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.stateless_interaction().key_listeners.push((
            TypeId::of::<KeyUpEvent>(),
            Box::new(move |view, event, _, phase, cx| {
                let event = event.downcast_ref().unwrap();
                listener(view, event, phase, cx);
                None
            }),
        ));
        self
    }

    fn drag_over<S: 'static>(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self
    where
        Self: Sized,
    {
        self.stateless_interaction()
            .drag_over_styles
            .push((TypeId::of::<S>(), f(StyleRefinement::default())));
        self
    }

    fn group_drag_over<S: 'static>(
        mut self,
        group_name: impl Into<SharedString>,
        f: impl FnOnce(StyleRefinement) -> StyleRefinement,
    ) -> Self
    where
        Self: Sized,
    {
        self.stateless_interaction().group_drag_over_styles.push((
            TypeId::of::<S>(),
            GroupStyle {
                group: group_name.into(),
                style: f(StyleRefinement::default()),
            },
        ));
        self
    }

    fn on_drop<S: 'static>(
        mut self,
        listener: impl Fn(&mut V, S, &mut ViewContext<V>) + Send + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.stateless_interaction().drop_listeners.push((
            TypeId::of::<S>(),
            Box::new(move |view, drag_state, cx| {
                listener(view, *drag_state.downcast().unwrap(), cx);
            }),
        ));
        self
    }
}

pub trait StatefulInteractive<V: 'static>: StatelessInteractive<V> {
    fn stateful_interaction(&mut self) -> &mut StatefulInteraction<V>;

    fn active(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self
    where
        Self: Sized,
    {
        self.stateful_interaction().active_style = f(StyleRefinement::default());
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
        self.stateful_interaction().group_active_style = Some(GroupStyle {
            group: group_name.into(),
            style: f(StyleRefinement::default()),
        });
        self
    }

    fn on_click(
        mut self,
        listener: impl Fn(&mut V, &ClickEvent, &mut ViewContext<V>) + Send + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.stateful_interaction()
            .click_listeners
            .push(Box::new(move |view, event, cx| listener(view, event, cx)));
        self
    }

    fn on_drag<S, R, E>(
        mut self,
        listener: impl Fn(&mut V, &mut ViewContext<V>) -> Drag<S, R, V, E> + Send + 'static,
    ) -> Self
    where
        Self: Sized,
        S: Any + Send,
        R: Fn(&mut V, &mut ViewContext<V>) -> E,
        R: 'static + Send,
        E: Component<V>,
    {
        debug_assert!(
            self.stateful_interaction().drag_listener.is_none(),
            "calling on_drag more than once on the same element is not supported"
        );
        self.stateful_interaction().drag_listener =
            Some(Box::new(move |view_state, cursor_offset, cx| {
                let drag = listener(view_state, cx);
                let drag_handle_view = Some(
                    cx.build_view(|cx| DragView {
                        model: cx.model().upgrade().unwrap(),
                        drag,
                    })
                    .into_any(),
                );
                AnyDrag {
                    render: {
                        let view = cx.view();
                        Box::new(move |cx| {
                            view.update(cx, |view, cx| drag.render_drag_handle(view, cx))
                        })
                    },
                    drag_handle_view,
                    cursor_offset,
                    state: Box::new(drag.state),
                    state_type: TypeId::of::<S>(),
                }
            }));
        self
    }
}

pub trait ElementInteraction<V: 'static>: 'static + Send {
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
                    Box::new(move |_, key_down, context, phase, cx| {
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
            let stateless = self.as_stateless_mut();
            cx.with_key_dispatch_context(stateless.dispatch_context.clone(), |cx| {
                cx.with_key_listeners(mem::take(&mut stateless.key_listeners), f)
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

        if let Some(drag) = cx.active_drag.take() {
            for (state_type, group_drag_style) in &self.as_stateless().group_drag_over_styles {
                if let Some(group_bounds) = GroupBounds::get(&group_drag_style.group, cx) {
                    if *state_type == drag.state_type
                        && group_bounds.contains_point(&mouse_position)
                    {
                        style.refine(&group_drag_style.style);
                    }
                }
            }

            for (state_type, drag_over_style) in &self.as_stateless().drag_over_styles {
                if *state_type == drag.state_type && bounds.contains_point(&mouse_position) {
                    style.refine(drag_over_style);
                }
            }

            cx.active_drag = Some(drag);
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
        let stateless = self.as_stateless_mut();
        for listener in stateless.mouse_down_listeners.drain(..) {
            cx.on_mouse_event(move |state, event: &MouseDownEvent, phase, cx| {
                listener(state, event, &bounds, phase, cx);
            })
        }

        for listener in stateless.mouse_up_listeners.drain(..) {
            cx.on_mouse_event(move |state, event: &MouseUpEvent, phase, cx| {
                listener(state, event, &bounds, phase, cx);
            })
        }

        for listener in stateless.mouse_move_listeners.drain(..) {
            cx.on_mouse_event(move |state, event: &MouseMoveEvent, phase, cx| {
                listener(state, event, &bounds, phase, cx);
            })
        }

        for listener in stateless.scroll_wheel_listeners.drain(..) {
            cx.on_mouse_event(move |state, event: &ScrollWheelEvent, phase, cx| {
                listener(state, event, &bounds, phase, cx);
            })
        }

        let hover_group_bounds = stateless
            .group_hover_style
            .as_ref()
            .and_then(|group_hover| GroupBounds::get(&group_hover.group, cx));

        if let Some(group_bounds) = hover_group_bounds {
            let hovered = group_bounds.contains_point(&cx.mouse_position());
            cx.on_mouse_event(move |_, event: &MouseMoveEvent, phase, cx| {
                if phase == DispatchPhase::Capture {
                    if group_bounds.contains_point(&event.position) != hovered {
                        cx.notify();
                    }
                }
            });
        }

        if stateless.hover_style.is_some()
            || (cx.active_drag.is_some() && !stateless.drag_over_styles.is_empty())
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

        if cx.active_drag.is_some() {
            let drop_listeners = mem::take(&mut stateless.drop_listeners);
            cx.on_mouse_event(move |view, event: &MouseUpEvent, phase, cx| {
                if phase == DispatchPhase::Bubble && bounds.contains_point(&event.position) {
                    if let Some(drag_state_type) =
                        cx.active_drag.as_ref().map(|drag| drag.state_type)
                    {
                        for (drop_state_type, listener) in &drop_listeners {
                            if *drop_state_type == drag_state_type {
                                let drag = cx
                                    .active_drag
                                    .take()
                                    .expect("checked for type drag state type above");
                                listener(view, drag.state, cx);
                                cx.notify();
                                cx.stop_propagation();
                            }
                        }
                    }
                }
            });
        }

        if let Some(stateful) = self.as_stateful_mut() {
            let click_listeners = mem::take(&mut stateful.click_listeners);
            let drag_listener = mem::take(&mut stateful.drag_listener);

            if !click_listeners.is_empty() || drag_listener.is_some() {
                let pending_mouse_down = element_state.pending_mouse_down.clone();
                let mouse_down = pending_mouse_down.lock().clone();
                if let Some(mouse_down) = mouse_down {
                    if let Some(drag_listener) = drag_listener {
                        let active_state = element_state.active_state.clone();

                        cx.on_mouse_event(move |view_state, event: &MouseMoveEvent, phase, cx| {
                            if cx.active_drag.is_some() {
                                if phase == DispatchPhase::Capture {
                                    cx.notify();
                                }
                            } else if phase == DispatchPhase::Bubble
                                && bounds.contains_point(&event.position)
                                && (event.position - mouse_down.position).magnitude()
                                    > DRAG_THRESHOLD
                            {
                                *active_state.lock() = ActiveState::default();
                                let cursor_offset = event.position - bounds.origin;
                                let drag = drag_listener(view_state, cursor_offset, cx);
                                cx.active_drag = Some(drag);
                                cx.notify();
                                cx.stop_propagation();
                            }
                        });
                    }

                    cx.on_mouse_event(move |view_state, event: &MouseUpEvent, phase, cx| {
                        if phase == DispatchPhase::Bubble && bounds.contains_point(&event.position)
                        {
                            let mouse_click = ClickEvent {
                                down: mouse_down.clone(),
                                up: event.clone(),
                            };
                            for listener in &click_listeners {
                                listener(view_state, &mouse_click, cx);
                            }
                        }
                        *pending_mouse_down.lock() = None;
                    });
                } else {
                    cx.on_mouse_event(move |_state, event: &MouseDownEvent, phase, _cx| {
                        if phase == DispatchPhase::Bubble && bounds.contains_point(&event.position)
                        {
                            *pending_mouse_down.lock() = Some(event.clone());
                        }
                    });
                }
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
                let scroll_max = (content_size - bounds.size).max(&Size::default());

                cx.on_mouse_event(move |_, event: &ScrollWheelEvent, phase, cx| {
                    if phase == DispatchPhase::Bubble && bounds.contains_point(&event.position) {
                        let mut scroll_offset = scroll_offset.lock();
                        let old_scroll_offset = *scroll_offset;
                        let delta = event.delta.pixel_delta(line_height);

                        if overflow.x == Overflow::Scroll {
                            scroll_offset.x =
                                (scroll_offset.x + delta.x).clamp(-scroll_max.width, px(0.));
                        }

                        if overflow.y == Overflow::Scroll {
                            scroll_offset.y =
                                (scroll_offset.y + delta.y).clamp(-scroll_max.height, px(0.));
                        }

                        if *scroll_offset != old_scroll_offset {
                            cx.notify();
                            cx.stop_propagation();
                        }
                    }
                });
            }
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct StatefulInteraction<V> {
    pub id: ElementId,
    #[deref]
    #[deref_mut]
    stateless: StatelessInteraction<V>,
    click_listeners: SmallVec<[ClickListener<V>; 2]>,
    active_style: StyleRefinement,
    group_active_style: Option<GroupStyle>,
    drag_listener: Option<DragListener<V>>,
}

impl<V: 'static> ElementInteraction<V> for StatefulInteraction<V> {
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

impl<V> From<ElementId> for StatefulInteraction<V> {
    fn from(id: ElementId) -> Self {
        Self {
            id,
            stateless: StatelessInteraction::default(),
            click_listeners: SmallVec::new(),
            drag_listener: None,
            active_style: StyleRefinement::default(),
            group_active_style: None,
        }
    }
}

type DropListener<V> = dyn Fn(&mut V, AnyBox, &mut ViewContext<V>) + 'static + Send;

pub struct StatelessInteraction<V> {
    pub dispatch_context: DispatchContext,
    pub mouse_down_listeners: SmallVec<[MouseDownListener<V>; 2]>,
    pub mouse_up_listeners: SmallVec<[MouseUpListener<V>; 2]>,
    pub mouse_move_listeners: SmallVec<[MouseMoveListener<V>; 2]>,
    pub scroll_wheel_listeners: SmallVec<[ScrollWheelListener<V>; 2]>,
    pub key_listeners: SmallVec<[(TypeId, KeyListener<V>); 32]>,
    pub hover_style: StyleRefinement,
    pub group_hover_style: Option<GroupStyle>,
    drag_over_styles: SmallVec<[(TypeId, StyleRefinement); 2]>,
    group_drag_over_styles: SmallVec<[(TypeId, GroupStyle); 2]>,
    drop_listeners: SmallVec<[(TypeId, Box<DropListener<V>>); 2]>,
}

impl<V> StatelessInteraction<V> {
    pub fn into_stateful(self, id: impl Into<ElementId>) -> StatefulInteraction<V> {
        StatefulInteraction {
            id: id.into(),
            stateless: self,
            click_listeners: SmallVec::new(),
            drag_listener: None,
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
        cx.default_global::<Self>().0.get_mut(name).unwrap().pop();
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
    pending_mouse_down: Arc<Mutex<Option<MouseDownEvent>>>,
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
            drag_over_styles: SmallVec::new(),
            group_drag_over_styles: SmallVec::new(),
            drop_listeners: SmallVec::new(),
        }
    }
}

impl<V: 'static> ElementInteraction<V> for StatelessInteraction<V> {
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
pub struct ClickEvent {
    pub down: MouseDownEvent,
    pub up: MouseUpEvent,
}

pub struct Drag<S, R, V, E>
where
    R: Fn(&mut V, &mut ViewContext<V>) -> E,
    V: 'static,
    E: Component<V>,
{
    pub state: S,
    pub render_drag_handle: R,
    view_type: PhantomData<V>,
}

impl<S, R, V, E> Drag<S, R, V, E>
where
    R: Fn(&mut V, &mut ViewContext<V>) -> E,
    V: 'static,
    E: Component<V>,
{
    pub fn new(state: S, render_drag_handle: R) -> Self {
        Drag {
            state,
            render_drag_handle,
            view_type: PhantomData,
        }
    }
}

// impl<S, R, V, E> Render for Drag<S, R, V, E> {
//     // fn render(&mut self, cx: ViewContext<Self>) ->
// }

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

#[derive(Debug, Clone, Default)]
pub struct ExternalPaths(pub(crate) SmallVec<[PathBuf; 2]>);

#[derive(Debug, Clone)]
pub enum FileDropEvent {
    Entered {
        position: Point<Pixels>,
        files: ExternalPaths,
    },
    Pending {
        position: Point<Pixels>,
    },
    Submit {
        position: Point<Pixels>,
    },
    Exited,
}

#[derive(Clone, Debug)]
pub enum InputEvent {
    KeyDown(KeyDownEvent),
    KeyUp(KeyUpEvent),
    ModifiersChanged(ModifiersChangedEvent),
    MouseDown(MouseDownEvent),
    MouseUp(MouseUpEvent),
    MouseMove(MouseMoveEvent),
    MouseExited(MouseExitEvent),
    ScrollWheel(ScrollWheelEvent),
    FileDrop(FileDropEvent),
}

impl InputEvent {
    pub fn position(&self) -> Option<Point<Pixels>> {
        match self {
            InputEvent::KeyDown { .. } => None,
            InputEvent::KeyUp { .. } => None,
            InputEvent::ModifiersChanged { .. } => None,
            InputEvent::MouseDown(event) => Some(event.position),
            InputEvent::MouseUp(event) => Some(event.position),
            InputEvent::MouseMove(event) => Some(event.position),
            InputEvent::MouseExited(event) => Some(event.position),
            InputEvent::ScrollWheel(event) => Some(event.position),
            InputEvent::FileDrop(FileDropEvent::Exited) => None,
            InputEvent::FileDrop(
                FileDropEvent::Entered { position, .. }
                | FileDropEvent::Pending { position, .. }
                | FileDropEvent::Submit { position, .. },
            ) => Some(*position),
        }
    }

    pub fn mouse_event<'a>(&'a self) -> Option<&'a dyn Any> {
        match self {
            InputEvent::KeyDown { .. } => None,
            InputEvent::KeyUp { .. } => None,
            InputEvent::ModifiersChanged { .. } => None,
            InputEvent::MouseDown(event) => Some(event),
            InputEvent::MouseUp(event) => Some(event),
            InputEvent::MouseMove(event) => Some(event),
            InputEvent::MouseExited(event) => Some(event),
            InputEvent::ScrollWheel(event) => Some(event),
            InputEvent::FileDrop(event) => Some(event),
        }
    }

    pub fn keyboard_event<'a>(&'a self) -> Option<&'a dyn Any> {
        match self {
            InputEvent::KeyDown(event) => Some(event),
            InputEvent::KeyUp(event) => Some(event),
            InputEvent::ModifiersChanged(event) => Some(event),
            InputEvent::MouseDown(_) => None,
            InputEvent::MouseUp(_) => None,
            InputEvent::MouseMove(_) => None,
            InputEvent::MouseExited(_) => None,
            InputEvent::ScrollWheel(_) => None,
            InputEvent::FileDrop(_) => None,
        }
    }
}

pub struct FocusEvent {
    pub blurred: Option<FocusHandle>,
    pub focused: Option<FocusHandle>,
}

pub type MouseDownListener<V> = Box<
    dyn Fn(&mut V, &MouseDownEvent, &Bounds<Pixels>, DispatchPhase, &mut ViewContext<V>)
        + Send
        + 'static,
>;
pub type MouseUpListener<V> = Box<
    dyn Fn(&mut V, &MouseUpEvent, &Bounds<Pixels>, DispatchPhase, &mut ViewContext<V>)
        + Send
        + 'static,
>;

pub type MouseMoveListener<V> = Box<
    dyn Fn(&mut V, &MouseMoveEvent, &Bounds<Pixels>, DispatchPhase, &mut ViewContext<V>)
        + Send
        + 'static,
>;

pub type ScrollWheelListener<V> = Box<
    dyn Fn(&mut V, &ScrollWheelEvent, &Bounds<Pixels>, DispatchPhase, &mut ViewContext<V>)
        + Send
        + 'static,
>;

pub type ClickListener<V> = Box<dyn Fn(&mut V, &ClickEvent, &mut ViewContext<V>) + Send + 'static>;

pub(crate) type DragListener<V> =
    Box<dyn Fn(&mut V, Point<Pixels>, &mut ViewContext<V>) -> AnyDrag + Send + 'static>;

pub type KeyListener<V> = Box<
    dyn Fn(
            &mut V,
            &dyn Any,
            &[&DispatchContext],
            DispatchPhase,
            &mut ViewContext<V>,
        ) -> Option<Box<dyn Action>>
        + Send
        + 'static,
>;
