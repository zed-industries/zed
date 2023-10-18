use crate::{
    Active, AnonymousElement, AnyElement, AppContext, BorrowWindow, Bounds, Click, DispatchPhase,
    Element, ElementId, ElementIdentity, Hover, IdentifiedElement, Interactive, IntoAnyElement,
    LayoutId, MouseClickEvent, MouseDownEvent, MouseEventListeners, MouseMoveEvent, MouseUpEvent,
    Overflow, ParentElement, Pixels, Point, ScrollWheelEvent, SharedString, Style, StyleRefinement,
    Styled, ViewContext,
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

#[derive(Default, Clone)]
pub struct ScrollState(Arc<Mutex<Point<Pixels>>>);

impl ScrollState {
    pub fn x(&self) -> Pixels {
        self.0.lock().x
    }

    pub fn set_x(&self, value: Pixels) {
        self.0.lock().x = value;
    }

    pub fn y(&self) -> Pixels {
        self.0.lock().y
    }

    pub fn set_y(&self, value: Pixels) {
        self.0.lock().y = value;
    }
}

pub fn div<S>() -> Div<S, AnonymousElement>
where
    S: 'static + Send + Sync,
{
    Div {
        kind: AnonymousElement,
        children: SmallVec::new(),
        group: None,
        base_style: StyleRefinement::default(),
        hover_style: StyleRefinement::default(),
        group_hover: None,
        active_style: StyleRefinement::default(),
        group_active: None,
        listeners: MouseEventListeners::default(),
    }
}

pub struct Div<V: 'static + Send + Sync, K: ElementIdentity = AnonymousElement> {
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

impl<V> Div<V, AnonymousElement>
where
    V: 'static + Send + Sync,
{
    pub fn id(self, id: impl Into<ElementId>) -> Div<V, IdentifiedElement> {
        Div {
            kind: IdentifiedElement(id.into()),
            children: self.children,
            group: self.group,
            base_style: self.base_style,
            hover_style: self.hover_style,
            group_hover: self.group_hover,
            active_style: self.active_style,
            group_active: self.group_active,
            listeners: self.listeners,
        }
    }
}

impl<V, K> Div<V, K>
where
    V: 'static + Send + Sync,
    K: ElementIdentity,
{
    pub fn group(mut self, group: impl Into<SharedString>) -> Self {
        self.group = Some(group.into());
        self
    }

    pub fn z_index(mut self, z_index: u32) -> Self {
        self.base_style.z_index = Some(z_index);
        self
    }

    pub fn overflow_hidden(mut self) -> Self {
        self.base_style.overflow.x = Some(Overflow::Hidden);
        self.base_style.overflow.y = Some(Overflow::Hidden);
        self
    }

    pub fn overflow_hidden_x(mut self) -> Self {
        self.base_style.overflow.x = Some(Overflow::Hidden);
        self
    }

    pub fn overflow_hidden_y(mut self) -> Self {
        self.base_style.overflow.y = Some(Overflow::Hidden);
        self
    }

    pub fn overflow_scroll(mut self, _scroll_state: ScrollState) -> Self {
        // todo!("impl scrolling")
        // self.scroll_state = Some(scroll_state);
        self.base_style.overflow.x = Some(Overflow::Scroll);
        self.base_style.overflow.y = Some(Overflow::Scroll);
        self
    }

    pub fn overflow_x_scroll(mut self, _scroll_state: ScrollState) -> Self {
        // todo!("impl scrolling")
        // self.scroll_state = Some(scroll_state);
        self.base_style.overflow.x = Some(Overflow::Scroll);
        self
    }

    pub fn overflow_y_scroll(mut self, _scroll_state: ScrollState) -> Self {
        // todo!("impl scrolling")
        // self.scroll_state = Some(scroll_state);
        self.base_style.overflow.y = Some(Overflow::Scroll);
        self
    }

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

    pub fn compute_style(
        &self,
        bounds: Bounds<Pixels>,
        state: &DivState,
        cx: &mut ViewContext<V>,
    ) -> Style {
        let mut computed_style = Style::default();
        computed_style.refine(&self.base_style);

        let mouse_position = cx.mouse_position();

        if let Some(group_hover) = self.group_hover.as_ref() {
            if let Some(group_bounds) = group_bounds(&group_hover.group, cx) {
                if group_bounds.contains_point(&mouse_position) {
                    computed_style.refine(&group_hover.style);
                }
            }
        }
        if bounds.contains_point(&mouse_position) {
            computed_style.refine(&self.hover_style);
        }

        let active_state = *state.active_state.lock();
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
                        group_bounds.map_or(false, |bounds| bounds.contains_point(&down.position));
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

impl<V, K> Element for Div<V, K>
where
    V: 'static + Send + Sync,
    K: ElementIdentity,
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
        let element_state = element_state.unwrap_or_default();
        let style = self.compute_style(Bounds::default(), &element_state, cx);
        style.apply_text_style(cx, |cx| {
            self.with_element_id(cx, |this, cx| {
                let layout_ids = this
                    .children
                    .iter_mut()
                    .map(|child| child.layout(view_state, cx))
                    .collect::<Vec<_>>();

                let layout_id = cx.request_layout(&style, layout_ids);
                (layout_id, element_state)
            })
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
            let style = this.compute_style(bounds, element_state, cx);
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

                cx.stack(1, |cx| {
                    style.apply_text_style(cx, |cx| {
                        style.apply_overflow(bounds, cx, |cx| {
                            for child in &mut this.children {
                                child.paint(view_state, None, cx);
                            }
                        })
                    })
                });
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
    K: ElementIdentity,
{
    fn into_any(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}

impl<V, K> ParentElement for Div<V, K>
where
    V: 'static + Send + Sync,
    K: ElementIdentity,
{
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<Self::ViewState>; 2]> {
        &mut self.children
    }
}

impl<V, K> Styled for Div<V, K>
where
    V: 'static + Send + Sync,
    K: ElementIdentity,
{
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.base_style
    }
}

impl<V, K> Interactive for Div<V, K>
where
    V: 'static + Send + Sync,
    K: ElementIdentity,
{
    fn listeners(&mut self) -> &mut MouseEventListeners<V> {
        &mut self.listeners
    }
}

impl<V, K> Hover for Div<V, K>
where
    V: 'static + Send + Sync,
    K: ElementIdentity,
{
    fn set_hover_style(&mut self, group: Option<SharedString>, style: StyleRefinement) {
        if let Some(group) = group {
            self.group_hover = Some(GroupStyle { group, style });
        } else {
            self.hover_style = style;
        }
    }
}

impl<V> Click for Div<V, IdentifiedElement> where V: 'static + Send + Sync {}

impl<V> Active for Div<V, IdentifiedElement>
where
    V: 'static + Send + Sync,
{
    fn set_active_style(&mut self, group: Option<SharedString>, style: StyleRefinement) {
        if let Some(group) = group {
            self.group_active = Some(GroupStyle { group, style });
        } else {
            self.active_style = style;
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
