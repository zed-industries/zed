use crate::{
    AnonymousElementKind, AnyElement, AppContext, BorrowWindow, Bounds, DispatchPhase, Element,
    ElementId, ElementKind, IntoAnyElement, LayoutId, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    Pixels, ScrollWheelEvent, SharedString, Style, StyleRefinement, ViewContext,
};
use collections::HashMap;
use parking_lot::Mutex;
use refineable::Refineable;
use smallvec::SmallVec;
use std::sync::Arc;

#[derive(Default)]
pub struct DivState {
    active_state: Arc<Mutex<ActiveState>>,
    pending_click: Arc<Mutex<Option<MouseDownEvent>>>,
}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
struct ActiveState {
    group: bool,
    element: bool,
}

impl ActiveState {
    pub fn is_none(&self) -> bool {
        !self.group && !self.element
    }
}

#[derive(Default)]
struct GroupBounds(HashMap<SharedString, SmallVec<[Bounds<Pixels>; 1]>>);

pub fn group_bounds(name: &SharedString, cx: &mut AppContext) -> Option<Bounds<Pixels>> {
    cx.default_global::<GroupBounds>()
        .0
        .get(name)
        .and_then(|bounds_stack| bounds_stack.last().cloned())
}

pub struct Div<V: 'static + Send + Sync, K: ElementKind = AnonymousElementKind> {
    kind: K,
    children: SmallVec<[AnyElement<V>; 2]>,
    group: Option<SharedString>,
    base_style: StyleRefinement,
    hover_style: StyleRefinement,
    group_hover: Option<GroupStyle>,
    active_style: StyleRefinement,
    group_active: Option<GroupStyle>,
    listeners: MouseEventListeners<V>,
}

struct GroupStyle {
    group: SharedString,
    style: StyleRefinement,
}

impl<V, K> Div<V, K>
where
    V: 'static + Send + Sync,
    K: ElementKind,
{
    fn with_element_id<R>(
        &mut self,
        cx: &mut ViewContext<V>,
        f: impl FnOnce(&mut Self, &mut ViewContext<V>) -> R,
    ) -> R {
        if let Some(id) = self.id() {
            cx.with_element_id(id, |cx| f(self, cx))
        } else {
            f(self, cx)
        }
    }

    fn compute_style(
        &self,
        bounds: Bounds<Pixels>,
        group_bounds: Option<Bounds<Pixels>>,
        active_state: ActiveState,
        cx: &mut ViewContext<V>,
    ) -> Style {
        let mut computed_style = Style::default();
        computed_style.refine(&self.base_style);

        let mouse_position = cx.mouse_position();
        if let Some(group_bounds) = group_bounds {
            if group_bounds.contains_point(mouse_position) {
                if let Some(GroupStyle { style, .. }) = self.group_hover.as_ref() {
                    computed_style.refine(style);
                }
            }
        }
        if bounds.contains_point(mouse_position) {
            computed_style.refine(&self.hover_style);
        }

        if active_state.group {
            if let Some(GroupStyle { style, .. }) = self.group_active.as_ref() {
                computed_style.refine(style);
            }
        }

        if active_state.element {
            computed_style.refine(&self.active_style);
        }

        computed_style
    }

    fn paint_hover_listeners(
        &self,
        bounds: Bounds<Pixels>,
        group_bounds: Option<Bounds<Pixels>>,
        cx: &mut ViewContext<V>,
    ) {
        if let Some(group_bounds) = group_bounds {
            paint_hover_listener(group_bounds, cx);
        }

        if self.hover_style.is_some() {
            paint_hover_listener(bounds, cx);
        }
    }

    fn paint_active_listener(
        &self,
        bounds: Bounds<Pixels>,
        group_bounds: Option<Bounds<Pixels>>,
        active_state: Arc<Mutex<ActiveState>>,
        cx: &mut ViewContext<V>,
    ) {
        if active_state.lock().is_none() {
            cx.on_mouse_event(move |_view, down: &MouseDownEvent, phase, cx| {
                if phase == DispatchPhase::Bubble {
                    let group =
                        group_bounds.map_or(false, |bounds| bounds.contains_point(down.position));
                    let element = bounds.contains_point(down.position);
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
    }

    fn paint_event_listeners(
        &self,
        bounds: Bounds<Pixels>,
        pending_click: Arc<Mutex<Option<MouseDownEvent>>>,
        cx: &mut ViewContext<V>,
    ) {
        let click_listeners = self.listeners.mouse_click.clone();
        let mouse_down = pending_click.lock().clone();
        if let Some(mouse_down) = mouse_down {
            cx.on_mouse_event(move |state, event: &MouseUpEvent, phase, cx| {
                if phase == DispatchPhase::Bubble && bounds.contains_point(event.position) {
                    let mouse_click = MouseClickEvent {
                        down: mouse_down.clone(),
                        up: event.clone(),
                    };
                    for listener in &click_listeners {
                        listener(state, &mouse_click, &bounds, cx);
                    }
                }

                *pending_click.lock() = None;
            });
        } else {
            cx.on_mouse_event(move |_state, event: &MouseDownEvent, phase, _cx| {
                if phase == DispatchPhase::Bubble && bounds.contains_point(event.position) {
                    *pending_click.lock() = Some(event.clone());
                }
            });
        }

        for listener in self.listeners.mouse_down.iter().cloned() {
            cx.on_mouse_event(move |state, event: &MouseDownEvent, phase, cx| {
                listener(state, event, &bounds, phase, cx);
            })
        }

        for listener in self.listeners.mouse_up.iter().cloned() {
            cx.on_mouse_event(move |state, event: &MouseUpEvent, phase, cx| {
                listener(state, event, &bounds, phase, cx);
            })
        }

        for listener in self.listeners.mouse_move.iter().cloned() {
            cx.on_mouse_event(move |state, event: &MouseMoveEvent, phase, cx| {
                listener(state, event, &bounds, phase, cx);
            })
        }

        for listener in self.listeners.scroll_wheel.iter().cloned() {
            cx.on_mouse_event(move |state, event: &ScrollWheelEvent, phase, cx| {
                listener(state, event, &bounds, phase, cx);
            })
        }
    }
}

fn paint_hover_listener<V>(bounds: Bounds<Pixels>, cx: &mut ViewContext<V>)
where
    V: 'static + Send + Sync,
{
    let hovered = bounds.contains_point(cx.mouse_position());
    cx.on_mouse_event(move |_, event: &MouseMoveEvent, phase, cx| {
        if phase == DispatchPhase::Capture {
            if bounds.contains_point(event.position) != hovered {
                cx.notify();
            }
        }
    });
}

impl<V, K> Element for Div<V, K>
where
    V: 'static + Send + Sync,
    K: ElementKind,
{
    type ViewState = V;
    type ElementState = DivState;

    fn id(&self) -> Option<ElementId> {
        self.kind.id()
    }

    fn layout(
        &mut self,
        view_state: &mut Self::ViewState,
        element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> (LayoutId, Self::ElementState) {
        self.with_element_id(cx, |this, cx| {
            let layout_ids = this
                .children
                .iter_mut()
                .map(|child| child.layout(view_state, cx))
                .collect::<Vec<_>>();

            let element_state = element_state.unwrap_or_default();
            let style = this.compute_style(
                Bounds::default(),
                None,
                *element_state.active_state.lock(),
                cx,
            );
            let layout_id = cx.request_layout(&style, layout_ids);
            (layout_id, element_state)
        })
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        view_state: &mut Self::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) {
        self.with_element_id(cx, |this, cx| {
            if let Some(group) = this.group.clone() {
                cx.default_global::<GroupBounds>()
                    .0
                    .entry(group)
                    .or_default()
                    .push(bounds);
            }

            let hover_group_bounds = this
                .group_hover
                .as_ref()
                .and_then(|group_hover| group_bounds(&group_hover.group, cx));
            let active_group_bounds = this
                .group_active
                .as_ref()
                .and_then(|group_active| group_bounds(&group_active.group, cx));
            let active_state = *element_state.active_state.lock();
            let style = this.compute_style(bounds, hover_group_bounds, active_state, cx);
            let z_index = style.z_index.unwrap_or(0);

            // Paint background and event handlers.
            cx.stack(z_index, |cx| {
                cx.stack(0, |cx| {
                    style.paint(bounds, cx);
                    this.paint_hover_listeners(bounds, hover_group_bounds, cx);
                    this.paint_active_listener(
                        bounds,
                        active_group_bounds,
                        element_state.active_state.clone(),
                        cx,
                    );
                    this.paint_event_listeners(bounds, element_state.pending_click.clone(), cx);
                });
            });

            style.apply_text_style(cx, |cx| {
                style.apply_overflow(bounds, cx, |cx| {
                    cx.stack(z_index + 1, |cx| {
                        for child in &mut this.children {
                            child.paint(view_state, None, cx);
                        }
                    })
                })
            });

            if let Some(group) = this.group.as_ref() {
                cx.default_global::<GroupBounds>()
                    .0
                    .get_mut(group)
                    .unwrap()
                    .pop();
            }
        })
    }
}

impl<V, K> IntoAnyElement<V> for Div<V, K>
where
    V: 'static + Send + Sync,
    K: ElementKind,
{
    fn into_any(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}

pub struct MouseClickEvent {
    pub down: MouseDownEvent,
    pub up: MouseUpEvent,
}

type MouseDownHandler<V> = Arc<
    dyn Fn(&mut V, &MouseDownEvent, &Bounds<Pixels>, DispatchPhase, &mut ViewContext<V>)
        + Send
        + Sync
        + 'static,
>;
type MouseUpHandler<V> = Arc<
    dyn Fn(&mut V, &MouseUpEvent, &Bounds<Pixels>, DispatchPhase, &mut ViewContext<V>)
        + Send
        + Sync
        + 'static,
>;
type MouseClickHandler<V> = Arc<
    dyn Fn(&mut V, &MouseClickEvent, &Bounds<Pixels>, &mut ViewContext<V>) + Send + Sync + 'static,
>;

type MouseMoveHandler<V> = Arc<
    dyn Fn(&mut V, &MouseMoveEvent, &Bounds<Pixels>, DispatchPhase, &mut ViewContext<V>)
        + Send
        + Sync
        + 'static,
>;
type ScrollWheelHandler<V> = Arc<
    dyn Fn(&mut V, &ScrollWheelEvent, &Bounds<Pixels>, DispatchPhase, &mut ViewContext<V>)
        + Send
        + Sync
        + 'static,
>;

pub struct MouseEventListeners<V: 'static> {
    mouse_down: SmallVec<[MouseDownHandler<V>; 2]>,
    mouse_up: SmallVec<[MouseUpHandler<V>; 2]>,
    mouse_click: SmallVec<[MouseClickHandler<V>; 2]>,
    mouse_move: SmallVec<[MouseMoveHandler<V>; 2]>,
    scroll_wheel: SmallVec<[ScrollWheelHandler<V>; 2]>,
}

impl<V> Default for MouseEventListeners<V> {
    fn default() -> Self {
        Self {
            mouse_down: SmallVec::new(),
            mouse_up: SmallVec::new(),
            mouse_click: SmallVec::new(),
            mouse_move: SmallVec::new(),
            scroll_wheel: SmallVec::new(),
        }
    }
}
