use crate::{
    point, px, Action, AnyDrag, AnyElement, AnyTooltip, AnyView, AppContext, BorrowAppContext,
    BorrowWindow, Bounds, ClickEvent, DispatchPhase, Element, ElementId, FocusEvent, FocusHandle,
    IntoElement, KeyContext, KeyDownEvent, KeyUpEvent, LayoutId, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, ParentElement, Pixels, Point, Render, ScrollWheelEvent,
    SharedString, Size, StackingOrder, Style, StyleRefinement, Styled, Task, View, Visibility,
    WindowContext,
};
use collections::HashMap;
use refineable::Refineable;
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    cmp::Ordering,
    fmt::Debug,
    mem,
    rc::Rc,
    time::Duration,
};
use taffy::style::Overflow;
use util::ResultExt;

const DRAG_THRESHOLD: f64 = 2.;
const TOOLTIP_DELAY: Duration = Duration::from_millis(500);

pub struct GroupStyle {
    pub group: SharedString,
    pub style: Box<StyleRefinement>,
}

pub trait InteractiveElement: Sized {
    fn interactivity(&mut self) -> &mut Interactivity;

    fn group(mut self, group: impl Into<SharedString>) -> Self {
        self.interactivity().group = Some(group.into());
        self
    }

    fn id(mut self, id: impl Into<ElementId>) -> Stateful<Self> {
        self.interactivity().element_id = Some(id.into());

        Stateful { element: self }
    }

    fn track_focus(mut self, focus_handle: &FocusHandle) -> Focusable<Self> {
        self.interactivity().focusable = true;
        self.interactivity().tracked_focus_handle = Some(focus_handle.clone());
        Focusable { element: self }
    }

    fn key_context<C, E>(mut self, key_context: C) -> Self
    where
        C: TryInto<KeyContext, Error = E>,
        E: Debug,
    {
        if let Some(key_context) = key_context.try_into().log_err() {
            self.interactivity().key_context = Some(key_context);
        }
        self
    }

    fn hover(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self {
        debug_assert!(
            self.interactivity().hover_style.is_none(),
            "hover style already set"
        );
        self.interactivity().hover_style = Some(Box::new(f(StyleRefinement::default())));
        self
    }

    fn group_hover(
        mut self,
        group_name: impl Into<SharedString>,
        f: impl FnOnce(StyleRefinement) -> StyleRefinement,
    ) -> Self {
        self.interactivity().group_hover_style = Some(GroupStyle {
            group: group_name.into(),
            style: Box::new(f(StyleRefinement::default())),
        });
        self
    }

    fn on_mouse_down(
        mut self,
        button: MouseButton,
        listener: impl Fn(&MouseDownEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.interactivity().mouse_down_listeners.push(Box::new(
            move |event, bounds, phase, cx| {
                if phase == DispatchPhase::Bubble
                    && event.button == button
                    && bounds.visibly_contains(&event.position, cx)
                {
                    (listener)(event, cx)
                }
            },
        ));
        self
    }

    fn on_any_mouse_down(
        mut self,
        listener: impl Fn(&MouseDownEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.interactivity().mouse_down_listeners.push(Box::new(
            move |event, bounds, phase, cx| {
                if phase == DispatchPhase::Bubble && bounds.visibly_contains(&event.position, cx) {
                    (listener)(event, cx)
                }
            },
        ));
        self
    }

    fn on_mouse_up(
        mut self,
        button: MouseButton,
        listener: impl Fn(&MouseUpEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.interactivity()
            .mouse_up_listeners
            .push(Box::new(move |event, bounds, phase, cx| {
                if phase == DispatchPhase::Bubble
                    && event.button == button
                    && bounds.visibly_contains(&event.position, cx)
                {
                    (listener)(event, cx)
                }
            }));
        self
    }

    fn on_any_mouse_up(
        mut self,
        listener: impl Fn(&MouseUpEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.interactivity()
            .mouse_up_listeners
            .push(Box::new(move |event, bounds, phase, cx| {
                if phase == DispatchPhase::Bubble && bounds.visibly_contains(&event.position, cx) {
                    (listener)(event, cx)
                }
            }));
        self
    }

    fn on_mouse_down_out(
        mut self,
        listener: impl Fn(&MouseDownEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.interactivity().mouse_down_listeners.push(Box::new(
            move |event, bounds, phase, cx| {
                if phase == DispatchPhase::Capture && !bounds.visibly_contains(&event.position, cx)
                {
                    (listener)(event, cx)
                }
            },
        ));
        self
    }

    fn on_mouse_up_out(
        mut self,
        button: MouseButton,
        listener: impl Fn(&MouseUpEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.interactivity()
            .mouse_up_listeners
            .push(Box::new(move |event, bounds, phase, cx| {
                if phase == DispatchPhase::Capture
                    && event.button == button
                    && !bounds.visibly_contains(&event.position, cx)
                {
                    (listener)(event, cx);
                }
            }));
        self
    }

    fn on_mouse_move(
        mut self,
        listener: impl Fn(&MouseMoveEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.interactivity().mouse_move_listeners.push(Box::new(
            move |event, bounds, phase, cx| {
                if phase == DispatchPhase::Bubble && bounds.visibly_contains(&event.position, cx) {
                    (listener)(event, cx);
                }
            },
        ));
        self
    }

    fn on_scroll_wheel(
        mut self,
        listener: impl Fn(&ScrollWheelEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.interactivity().scroll_wheel_listeners.push(Box::new(
            move |event, bounds, phase, cx| {
                if phase == DispatchPhase::Bubble && bounds.visibly_contains(&event.position, cx) {
                    (listener)(event, cx);
                }
            },
        ));
        self
    }

    /// Capture the given action, before normal action dispatch can fire
    fn capture_action<A: Action>(
        mut self,
        listener: impl Fn(&A, &mut WindowContext) + 'static,
    ) -> Self {
        self.interactivity().action_listeners.push((
            TypeId::of::<A>(),
            Box::new(move |action, phase, cx| {
                let action = action.downcast_ref().unwrap();
                if phase == DispatchPhase::Capture {
                    (listener)(action, cx)
                }
            }),
        ));
        self
    }

    /// Add a listener for the given action, fires during the bubble event phase
    fn on_action<A: Action>(mut self, listener: impl Fn(&A, &mut WindowContext) + 'static) -> Self {
        self.interactivity().action_listeners.push((
            TypeId::of::<A>(),
            Box::new(move |action, phase, cx| {
                let action = action.downcast_ref().unwrap();
                if phase == DispatchPhase::Bubble {
                    (listener)(action, cx)
                }
            }),
        ));
        self
    }

    fn on_boxed_action(
        mut self,
        action: &Box<dyn Action>,
        listener: impl Fn(&Box<dyn Action>, &mut WindowContext) + 'static,
    ) -> Self {
        let action = action.boxed_clone();
        self.interactivity().action_listeners.push((
            (*action).type_id(),
            Box::new(move |_, phase, cx| {
                if phase == DispatchPhase::Bubble {
                    (listener)(&action, cx)
                }
            }),
        ));
        self
    }

    fn on_key_down(
        mut self,
        listener: impl Fn(&KeyDownEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.interactivity()
            .key_down_listeners
            .push(Box::new(move |event, phase, cx| {
                if phase == DispatchPhase::Bubble {
                    (listener)(event, cx)
                }
            }));
        self
    }

    fn capture_key_down(
        mut self,
        listener: impl Fn(&KeyDownEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.interactivity()
            .key_down_listeners
            .push(Box::new(move |event, phase, cx| {
                if phase == DispatchPhase::Capture {
                    listener(event, cx)
                }
            }));
        self
    }

    fn on_key_up(mut self, listener: impl Fn(&KeyUpEvent, &mut WindowContext) + 'static) -> Self {
        self.interactivity()
            .key_up_listeners
            .push(Box::new(move |event, phase, cx| {
                if phase == DispatchPhase::Bubble {
                    listener(event, cx)
                }
            }));
        self
    }

    fn capture_key_up(
        mut self,
        listener: impl Fn(&KeyUpEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.interactivity()
            .key_up_listeners
            .push(Box::new(move |event, phase, cx| {
                if phase == DispatchPhase::Capture {
                    listener(event, cx)
                }
            }));
        self
    }

    fn drag_over<S: 'static>(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self {
        self.interactivity()
            .drag_over_styles
            .push((TypeId::of::<S>(), f(StyleRefinement::default())));
        self
    }

    fn group_drag_over<S: 'static>(
        mut self,
        group_name: impl Into<SharedString>,
        f: impl FnOnce(StyleRefinement) -> StyleRefinement,
    ) -> Self {
        self.interactivity().group_drag_over_styles.push((
            TypeId::of::<S>(),
            GroupStyle {
                group: group_name.into(),
                style: Box::new(f(StyleRefinement::default())),
            },
        ));
        self
    }

    fn on_drop<W: 'static>(
        mut self,
        listener: impl Fn(&View<W>, &mut WindowContext) + 'static,
    ) -> Self {
        self.interactivity().drop_listeners.push((
            TypeId::of::<W>(),
            Box::new(move |dragged_view, cx| {
                listener(&dragged_view.downcast().unwrap(), cx);
            }),
        ));
        self
    }
}

pub trait StatefulInteractiveElement: InteractiveElement {
    fn focusable(mut self) -> Focusable<Self> {
        self.interactivity().focusable = true;
        Focusable { element: self }
    }

    fn overflow_scroll(mut self) -> Self {
        self.interactivity().base_style.overflow.x = Some(Overflow::Scroll);
        self.interactivity().base_style.overflow.y = Some(Overflow::Scroll);
        self
    }

    fn overflow_x_scroll(mut self) -> Self {
        self.interactivity().base_style.overflow.x = Some(Overflow::Scroll);
        self
    }

    fn overflow_y_scroll(mut self) -> Self {
        self.interactivity().base_style.overflow.y = Some(Overflow::Scroll);
        self
    }

    fn track_scroll(mut self, scroll_handle: &ScrollHandle) -> Self {
        self.interactivity().scroll_handle = Some(scroll_handle.clone());
        self
    }

    fn active(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self
    where
        Self: Sized,
    {
        self.interactivity().active_style = Some(Box::new(f(StyleRefinement::default())));
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
        self.interactivity().group_active_style = Some(GroupStyle {
            group: group_name.into(),
            style: Box::new(f(StyleRefinement::default())),
        });
        self
    }

    fn on_click(mut self, listener: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self
    where
        Self: Sized,
    {
        self.interactivity()
            .click_listeners
            .push(Box::new(move |event, cx| listener(event, cx)));
        self
    }

    fn on_drag<W>(mut self, listener: impl Fn(&mut WindowContext) -> View<W> + 'static) -> Self
    where
        Self: Sized,
        W: 'static + Render,
    {
        debug_assert!(
            self.interactivity().drag_listener.is_none(),
            "calling on_drag more than once on the same element is not supported"
        );
        self.interactivity().drag_listener = Some(Box::new(move |cursor_offset, cx| AnyDrag {
            view: listener(cx).into(),
            cursor_offset,
        }));
        self
    }

    fn on_hover(mut self, listener: impl Fn(&bool, &mut WindowContext) + 'static) -> Self
    where
        Self: Sized,
    {
        debug_assert!(
            self.interactivity().hover_listener.is_none(),
            "calling on_hover more than once on the same element is not supported"
        );
        self.interactivity().hover_listener = Some(Box::new(listener));
        self
    }

    fn tooltip(mut self, build_tooltip: impl Fn(&mut WindowContext) -> AnyView + 'static) -> Self
    where
        Self: Sized,
    {
        debug_assert!(
            self.interactivity().tooltip_builder.is_none(),
            "calling tooltip more than once on the same element is not supported"
        );
        self.interactivity().tooltip_builder = Some(Rc::new(build_tooltip));
        self
    }
}

pub trait FocusableElement: InteractiveElement {
    fn focus(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self
    where
        Self: Sized,
    {
        self.interactivity().focus_style = Some(Box::new(f(StyleRefinement::default())));
        self
    }

    fn in_focus(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self
    where
        Self: Sized,
    {
        self.interactivity().in_focus_style = Some(Box::new(f(StyleRefinement::default())));
        self
    }
}

pub type FocusListeners = Vec<FocusListener>;

pub type FocusListener = Box<dyn Fn(&FocusHandle, &FocusEvent, &mut WindowContext) + 'static>;

pub type MouseDownListener =
    Box<dyn Fn(&MouseDownEvent, &InteractiveBounds, DispatchPhase, &mut WindowContext) + 'static>;
pub type MouseUpListener =
    Box<dyn Fn(&MouseUpEvent, &InteractiveBounds, DispatchPhase, &mut WindowContext) + 'static>;

pub type MouseMoveListener =
    Box<dyn Fn(&MouseMoveEvent, &InteractiveBounds, DispatchPhase, &mut WindowContext) + 'static>;

pub type ScrollWheelListener =
    Box<dyn Fn(&ScrollWheelEvent, &InteractiveBounds, DispatchPhase, &mut WindowContext) + 'static>;

pub type ClickListener = Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>;

pub type DragListener = Box<dyn Fn(Point<Pixels>, &mut WindowContext) -> AnyDrag + 'static>;

type DropListener = dyn Fn(AnyView, &mut WindowContext) + 'static;

pub type TooltipBuilder = Rc<dyn Fn(&mut WindowContext) -> AnyView + 'static>;

pub type KeyDownListener = Box<dyn Fn(&KeyDownEvent, DispatchPhase, &mut WindowContext) + 'static>;

pub type KeyUpListener = Box<dyn Fn(&KeyUpEvent, DispatchPhase, &mut WindowContext) + 'static>;

pub type DragEventListener = Box<dyn Fn(&MouseMoveEvent, &mut WindowContext) + 'static>;

pub type ActionListener = Box<dyn Fn(&dyn Any, DispatchPhase, &mut WindowContext) + 'static>;

pub fn div() -> Div {
    Div {
        interactivity: Interactivity::default(),
        children: SmallVec::default(),
    }
}

pub struct Div {
    interactivity: Interactivity,
    children: SmallVec<[AnyElement; 2]>,
}

impl Styled for Div {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity.base_style
    }
}

impl InteractiveElement for Div {
    fn interactivity(&mut self) -> &mut Interactivity {
        &mut self.interactivity
    }
}

impl ParentElement for Div {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.children
    }
}

impl Element for Div {
    type State = DivState;

    fn layout(
        &mut self,
        element_state: Option<Self::State>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::State) {
        let mut child_layout_ids = SmallVec::new();
        let (layout_id, interactive_state) = self.interactivity.layout(
            element_state.map(|s| s.interactive_state),
            cx,
            |style, cx| {
                cx.with_text_style(style.text_style().cloned(), |cx| {
                    child_layout_ids = self
                        .children
                        .iter_mut()
                        .map(|child| child.layout(cx))
                        .collect::<SmallVec<_>>();
                    cx.request_layout(&style, child_layout_ids.iter().copied())
                })
            },
        );
        (
            layout_id,
            DivState {
                interactive_state,
                child_layout_ids,
            },
        )
    }

    fn paint(
        self,
        bounds: Bounds<Pixels>,
        element_state: &mut Self::State,
        cx: &mut WindowContext,
    ) {
        let mut child_min = point(Pixels::MAX, Pixels::MAX);
        let mut child_max = Point::default();
        let content_size = if element_state.child_layout_ids.is_empty() {
            bounds.size
        } else if let Some(scroll_handle) = self.interactivity.scroll_handle.as_ref() {
            let mut state = scroll_handle.0.borrow_mut();
            state.child_bounds = Vec::with_capacity(element_state.child_layout_ids.len());
            state.bounds = bounds;
            let requested = state.requested_scroll_top.take();

            for (ix, child_layout_id) in element_state.child_layout_ids.iter().enumerate() {
                let child_bounds = cx.layout_bounds(*child_layout_id);
                child_min = child_min.min(&child_bounds.origin);
                child_max = child_max.max(&child_bounds.lower_right());
                state.child_bounds.push(child_bounds);

                if let Some(requested) = requested.as_ref() {
                    if requested.0 == ix {
                        *state.offset.borrow_mut() =
                            bounds.origin - (child_bounds.origin - point(px(0.), requested.1));
                    }
                }
            }
            (child_max - child_min).into()
        } else {
            for child_layout_id in &element_state.child_layout_ids {
                let child_bounds = cx.layout_bounds(*child_layout_id);
                child_min = child_min.min(&child_bounds.origin);
                child_max = child_max.max(&child_bounds.lower_right());
            }
            (child_max - child_min).into()
        };

        self.interactivity.paint(
            bounds,
            content_size,
            &mut element_state.interactive_state,
            cx,
            |style, scroll_offset, cx| {
                if style.visibility == Visibility::Hidden {
                    return;
                }

                let z_index = style.z_index.unwrap_or(0);

                cx.with_z_index(z_index, |cx| {
                    cx.with_z_index(0, |cx| {
                        style.paint(bounds, cx);
                    });
                    cx.with_z_index(1, |cx| {
                        cx.with_text_style(style.text_style().cloned(), |cx| {
                            cx.with_content_mask(style.overflow_mask(bounds), |cx| {
                                cx.with_element_offset(scroll_offset, |cx| {
                                    for child in self.children {
                                        child.paint(cx);
                                    }
                                })
                            })
                        })
                    })
                })
            },
        );
    }
}

impl IntoElement for Div {
    type Element = Self;

    fn element_id(&self) -> Option<ElementId> {
        self.interactivity.element_id.clone()
    }

    fn into_element(self) -> Self::Element {
        self
    }
}

pub struct DivState {
    child_layout_ids: SmallVec<[LayoutId; 2]>,
    interactive_state: InteractiveElementState,
}

impl DivState {
    pub fn is_active(&self) -> bool {
        self.interactive_state
            .pending_mouse_down
            .as_ref()
            .map_or(false, |pending| pending.borrow().is_some())
    }
}

pub struct Interactivity {
    pub element_id: Option<ElementId>,
    pub key_context: Option<KeyContext>,
    pub focusable: bool,
    pub tracked_focus_handle: Option<FocusHandle>,
    pub scroll_handle: Option<ScrollHandle>,
    pub group: Option<SharedString>,
    pub base_style: Box<StyleRefinement>,
    pub focus_style: Option<Box<StyleRefinement>>,
    pub in_focus_style: Option<Box<StyleRefinement>>,
    pub hover_style: Option<Box<StyleRefinement>>,
    pub group_hover_style: Option<GroupStyle>,
    pub active_style: Option<Box<StyleRefinement>>,
    pub group_active_style: Option<GroupStyle>,
    pub drag_over_styles: Vec<(TypeId, StyleRefinement)>,
    pub group_drag_over_styles: Vec<(TypeId, GroupStyle)>,
    pub mouse_down_listeners: Vec<MouseDownListener>,
    pub mouse_up_listeners: Vec<MouseUpListener>,
    pub mouse_move_listeners: Vec<MouseMoveListener>,
    pub scroll_wheel_listeners: Vec<ScrollWheelListener>,
    pub key_down_listeners: Vec<KeyDownListener>,
    pub key_up_listeners: Vec<KeyUpListener>,
    pub action_listeners: Vec<(TypeId, ActionListener)>,
    pub drop_listeners: Vec<(TypeId, Box<DropListener>)>,
    pub click_listeners: Vec<ClickListener>,
    pub drag_listener: Option<DragListener>,
    pub hover_listener: Option<Box<dyn Fn(&bool, &mut WindowContext)>>,
    pub tooltip_builder: Option<TooltipBuilder>,
}

#[derive(Clone, Debug)]
pub struct InteractiveBounds {
    pub bounds: Bounds<Pixels>,
    pub stacking_order: StackingOrder,
}

impl InteractiveBounds {
    pub fn visibly_contains(&self, point: &Point<Pixels>, cx: &WindowContext) -> bool {
        self.bounds.contains(point) && cx.was_top_layer(&point, &self.stacking_order)
    }

    pub fn drag_target_contains(&self, point: &Point<Pixels>, cx: &WindowContext) -> bool {
        self.bounds.contains(point)
            && cx.was_top_layer_under_active_drag(&point, &self.stacking_order)
    }
}

impl Interactivity {
    pub fn layout(
        &mut self,
        element_state: Option<InteractiveElementState>,
        cx: &mut WindowContext,
        f: impl FnOnce(Style, &mut WindowContext) -> LayoutId,
    ) -> (LayoutId, InteractiveElementState) {
        let mut element_state = element_state.unwrap_or_default();

        // Ensure we store a focus handle in our element state if we're focusable.
        // If there's an explicit focus handle we're tracking, use that. Otherwise
        // create a new handle and store it in the element state, which lives for as
        // as frames contain an element with this id.
        if self.focusable {
            element_state.focus_handle.get_or_insert_with(|| {
                self.tracked_focus_handle
                    .clone()
                    .unwrap_or_else(|| cx.focus_handle())
            });
        }

        if let Some(scroll_handle) = self.scroll_handle.as_ref() {
            element_state.scroll_offset = Some(scroll_handle.0.borrow().offset.clone());
        }

        let style = self.compute_style(None, &mut element_state, cx);
        let layout_id = f(style, cx);
        (layout_id, element_state)
    }

    pub fn paint(
        mut self,
        bounds: Bounds<Pixels>,
        content_size: Size<Pixels>,
        element_state: &mut InteractiveElementState,
        cx: &mut WindowContext,
        f: impl FnOnce(Style, Point<Pixels>, &mut WindowContext),
    ) {
        let style = self.compute_style(Some(bounds), element_state, cx);

        if style
            .background
            .as_ref()
            .is_some_and(|fill| fill.color().is_some_and(|color| !color.is_transparent()))
        {
            cx.with_z_index(style.z_index.unwrap_or(0), |cx| cx.add_opaque_layer(bounds))
        }

        let interactive_bounds = Rc::new(InteractiveBounds {
            bounds: bounds.intersect(&cx.content_mask().bounds),
            stacking_order: cx.stacking_order().clone(),
        });

        if let Some(mouse_cursor) = style.mouse_cursor {
            let mouse_position = &cx.mouse_position();
            let hovered = interactive_bounds.visibly_contains(mouse_position, cx);
            if hovered {
                cx.set_cursor_style(mouse_cursor);
            }
        }

        // If this element can be focused, register a mouse down listener
        // that will automatically transfer focus when hitting the element.
        // This behavior can be suppressed by using `cx.prevent_default()`.
        if let Some(focus_handle) = element_state.focus_handle.clone() {
            cx.on_mouse_event({
                let interactive_bounds = interactive_bounds.clone();
                move |event: &MouseDownEvent, phase, cx| {
                    if phase == DispatchPhase::Bubble
                        && !cx.default_prevented()
                        && interactive_bounds.visibly_contains(&event.position, cx)
                    {
                        cx.focus(&focus_handle);
                        // If there is a parent that is also focusable, prevent it
                        // from trasferring focus because we already did so.
                        cx.prevent_default();
                    }
                }
            });
        }

        for listener in self.mouse_down_listeners {
            let interactive_bounds = interactive_bounds.clone();
            cx.on_mouse_event(move |event: &MouseDownEvent, phase, cx| {
                listener(event, &*interactive_bounds, phase, cx);
            })
        }

        for listener in self.mouse_up_listeners {
            let interactive_bounds = interactive_bounds.clone();
            cx.on_mouse_event(move |event: &MouseUpEvent, phase, cx| {
                listener(event, &*interactive_bounds, phase, cx);
            })
        }

        for listener in self.mouse_move_listeners {
            let interactive_bounds = interactive_bounds.clone();
            cx.on_mouse_event(move |event: &MouseMoveEvent, phase, cx| {
                listener(event, &*interactive_bounds, phase, cx);
            })
        }

        for listener in self.scroll_wheel_listeners {
            let interactive_bounds = interactive_bounds.clone();
            cx.on_mouse_event(move |event: &ScrollWheelEvent, phase, cx| {
                listener(event, &*interactive_bounds, phase, cx);
            })
        }

        let hover_group_bounds = self
            .group_hover_style
            .as_ref()
            .and_then(|group_hover| GroupBounds::get(&group_hover.group, cx));

        if let Some(group_bounds) = hover_group_bounds {
            let hovered = group_bounds.contains(&cx.mouse_position());
            cx.on_mouse_event(move |event: &MouseMoveEvent, phase, cx| {
                if phase == DispatchPhase::Capture {
                    if group_bounds.contains(&event.position) != hovered {
                        cx.notify();
                    }
                }
            });
        }

        if self.hover_style.is_some()
            || self.base_style.mouse_cursor.is_some()
            || cx.active_drag.is_some() && !self.drag_over_styles.is_empty()
        {
            let bounds = bounds.intersect(&cx.content_mask().bounds);
            let hovered = bounds.contains(&cx.mouse_position());
            cx.on_mouse_event(move |event: &MouseMoveEvent, phase, cx| {
                if phase == DispatchPhase::Capture {
                    if bounds.contains(&event.position) != hovered {
                        cx.notify();
                    }
                }
            });
        }

        if cx.active_drag.is_some() {
            let drop_listeners = mem::take(&mut self.drop_listeners);
            let interactive_bounds = interactive_bounds.clone();
            if !drop_listeners.is_empty() {
                cx.on_mouse_event(move |event: &MouseUpEvent, phase, cx| {
                    if phase == DispatchPhase::Bubble
                        && interactive_bounds.drag_target_contains(&event.position, cx)
                    {
                        if let Some(drag_state_type) =
                            cx.active_drag.as_ref().map(|drag| drag.view.entity_type())
                        {
                            for (drop_state_type, listener) in &drop_listeners {
                                if *drop_state_type == drag_state_type {
                                    let drag = cx
                                        .active_drag
                                        .take()
                                        .expect("checked for type drag state type above");

                                    listener(drag.view.clone(), cx);
                                    cx.notify();
                                    cx.stop_propagation();
                                }
                            }
                        } else {
                            cx.active_drag = None;
                        }
                    }
                });
            }
        }

        let click_listeners = self.click_listeners;
        let drag_listener = self.drag_listener;

        if !click_listeners.is_empty() || drag_listener.is_some() {
            let pending_mouse_down = element_state
                .pending_mouse_down
                .get_or_insert_with(Default::default)
                .clone();
            let mouse_down = pending_mouse_down.borrow().clone();
            if let Some(mouse_down) = mouse_down {
                if let Some(drag_listener) = drag_listener {
                    let active_state = element_state
                        .clicked_state
                        .get_or_insert_with(Default::default)
                        .clone();
                    let interactive_bounds = interactive_bounds.clone();

                    cx.on_mouse_event(move |event: &MouseMoveEvent, phase, cx| {
                        if cx.active_drag.is_some() {
                            if phase == DispatchPhase::Capture {
                                cx.notify();
                            }
                        } else if phase == DispatchPhase::Bubble
                            && interactive_bounds.visibly_contains(&event.position, cx)
                            && (event.position - mouse_down.position).magnitude() > DRAG_THRESHOLD
                        {
                            *active_state.borrow_mut() = ElementClickedState::default();
                            let cursor_offset = event.position - bounds.origin;
                            let drag = drag_listener(cursor_offset, cx);
                            cx.active_drag = Some(drag);
                            cx.notify();
                            cx.stop_propagation();
                        }
                    });
                }

                let interactive_bounds = interactive_bounds.clone();
                cx.on_mouse_event(move |event: &MouseUpEvent, phase, cx| {
                    if phase == DispatchPhase::Bubble
                        && interactive_bounds.visibly_contains(&event.position, cx)
                    {
                        let mouse_click = ClickEvent {
                            down: mouse_down.clone(),
                            up: event.clone(),
                        };
                        for listener in &click_listeners {
                            listener(&mouse_click, cx);
                        }
                    }
                    *pending_mouse_down.borrow_mut() = None;
                    cx.notify();
                });
            } else {
                let interactive_bounds = interactive_bounds.clone();
                cx.on_mouse_event(move |event: &MouseDownEvent, phase, cx| {
                    if phase == DispatchPhase::Bubble
                        && event.button == MouseButton::Left
                        && interactive_bounds.visibly_contains(&event.position, cx)
                    {
                        *pending_mouse_down.borrow_mut() = Some(event.clone());
                        cx.notify();
                    }
                });
            }
        }

        if let Some(hover_listener) = self.hover_listener.take() {
            let was_hovered = element_state
                .hover_state
                .get_or_insert_with(Default::default)
                .clone();
            let has_mouse_down = element_state
                .pending_mouse_down
                .get_or_insert_with(Default::default)
                .clone();
            let interactive_bounds = interactive_bounds.clone();

            cx.on_mouse_event(move |event: &MouseMoveEvent, phase, cx| {
                if phase != DispatchPhase::Bubble {
                    return;
                }
                let is_hovered = interactive_bounds.visibly_contains(&event.position, cx)
                    && has_mouse_down.borrow().is_none();
                let mut was_hovered = was_hovered.borrow_mut();

                if is_hovered != was_hovered.clone() {
                    *was_hovered = is_hovered;
                    drop(was_hovered);

                    hover_listener(&is_hovered, cx);
                }
            });
        }

        if let Some(tooltip_builder) = self.tooltip_builder.take() {
            let active_tooltip = element_state
                .active_tooltip
                .get_or_insert_with(Default::default)
                .clone();
            let pending_mouse_down = element_state
                .pending_mouse_down
                .get_or_insert_with(Default::default)
                .clone();
            let interactive_bounds = interactive_bounds.clone();

            cx.on_mouse_event(move |event: &MouseMoveEvent, phase, cx| {
                let is_hovered = interactive_bounds.visibly_contains(&event.position, cx)
                    && pending_mouse_down.borrow().is_none();
                if !is_hovered {
                    active_tooltip.borrow_mut().take();
                    return;
                }

                if phase != DispatchPhase::Bubble {
                    return;
                }

                if active_tooltip.borrow().is_none() {
                    let task = cx.spawn({
                        let active_tooltip = active_tooltip.clone();
                        let tooltip_builder = tooltip_builder.clone();

                        move |mut cx| async move {
                            cx.background_executor().timer(TOOLTIP_DELAY).await;
                            cx.update(|_, cx| {
                                active_tooltip.borrow_mut().replace(ActiveTooltip {
                                    tooltip: Some(AnyTooltip {
                                        view: tooltip_builder(cx),
                                        cursor_offset: cx.mouse_position(),
                                    }),
                                    _task: None,
                                });
                                cx.notify();
                            })
                            .ok();
                        }
                    });
                    active_tooltip.borrow_mut().replace(ActiveTooltip {
                        tooltip: None,
                        _task: Some(task),
                    });
                }
            });

            let active_tooltip = element_state
                .active_tooltip
                .get_or_insert_with(Default::default)
                .clone();
            cx.on_mouse_event(move |_: &MouseDownEvent, _, _| {
                active_tooltip.borrow_mut().take();
            });

            if let Some(active_tooltip) = element_state
                .active_tooltip
                .get_or_insert_with(Default::default)
                .borrow()
                .as_ref()
            {
                if active_tooltip.tooltip.is_some() {
                    cx.active_tooltip = active_tooltip.tooltip.clone()
                }
            }
        }

        let active_state = element_state
            .clicked_state
            .get_or_insert_with(Default::default)
            .clone();
        if active_state.borrow().is_clicked() {
            cx.on_mouse_event(move |_: &MouseUpEvent, phase, cx| {
                if phase == DispatchPhase::Capture {
                    *active_state.borrow_mut() = ElementClickedState::default();
                    cx.notify();
                }
            });
        } else {
            let active_group_bounds = self
                .group_active_style
                .as_ref()
                .and_then(|group_active| GroupBounds::get(&group_active.group, cx));
            let interactive_bounds = interactive_bounds.clone();
            cx.on_mouse_event(move |down: &MouseDownEvent, phase, cx| {
                if phase == DispatchPhase::Bubble && !cx.default_prevented() {
                    let group =
                        active_group_bounds.map_or(false, |bounds| bounds.contains(&down.position));
                    let element = interactive_bounds.visibly_contains(&down.position, cx);
                    if group || element {
                        *active_state.borrow_mut() = ElementClickedState { group, element };
                        cx.notify();
                    }
                }
            });
        }

        let overflow = style.overflow;
        if overflow.x == Overflow::Scroll || overflow.y == Overflow::Scroll {
            if let Some(scroll_handle) = &self.scroll_handle {
                scroll_handle.0.borrow_mut().overflow = overflow;
            }

            let scroll_offset = element_state
                .scroll_offset
                .get_or_insert_with(Rc::default)
                .clone();
            let line_height = cx.line_height();
            let scroll_max = (content_size - bounds.size).max(&Size::default());
            let interactive_bounds = interactive_bounds.clone();

            cx.on_mouse_event(move |event: &ScrollWheelEvent, phase, cx| {
                if phase == DispatchPhase::Bubble
                    && interactive_bounds.visibly_contains(&event.position, cx)
                {
                    let mut scroll_offset = scroll_offset.borrow_mut();
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

        if let Some(group) = self.group.clone() {
            GroupBounds::push(group, bounds, cx);
        }

        let scroll_offset = element_state
            .scroll_offset
            .as_ref()
            .map(|scroll_offset| *scroll_offset.borrow());

        cx.with_key_dispatch(
            self.key_context.clone(),
            element_state.focus_handle.clone(),
            |_, cx| {
                for listener in self.key_down_listeners {
                    cx.on_key_event(move |event: &KeyDownEvent, phase, cx| {
                        listener(event, phase, cx);
                    })
                }

                for listener in self.key_up_listeners {
                    cx.on_key_event(move |event: &KeyUpEvent, phase, cx| {
                        listener(event, phase, cx);
                    })
                }

                for (action_type, listener) in self.action_listeners {
                    cx.on_action(action_type, listener)
                }

                f(style, scroll_offset.unwrap_or_default(), cx)
            },
        );

        if let Some(group) = self.group.as_ref() {
            GroupBounds::pop(group, cx);
        }
    }

    pub fn compute_style(
        &self,
        bounds: Option<Bounds<Pixels>>,
        element_state: &mut InteractiveElementState,
        cx: &mut WindowContext,
    ) -> Style {
        let mut style = Style::default();
        style.refine(&self.base_style);

        if let Some(focus_handle) = self.tracked_focus_handle.as_ref() {
            if let Some(in_focus_style) = self.in_focus_style.as_ref() {
                if focus_handle.within_focused(cx) {
                    style.refine(in_focus_style);
                }
            }

            if let Some(focus_style) = self.focus_style.as_ref() {
                if focus_handle.is_focused(cx) {
                    style.refine(focus_style);
                }
            }
        }

        if let Some(bounds) = bounds {
            let mouse_position = cx.mouse_position();
            if let Some(group_hover) = self.group_hover_style.as_ref() {
                if let Some(group_bounds) = GroupBounds::get(&group_hover.group, cx) {
                    if group_bounds.contains(&mouse_position)
                        && cx.was_top_layer(&mouse_position, cx.stacking_order())
                    {
                        style.refine(&group_hover.style);
                    }
                }
            }
            if let Some(hover_style) = self.hover_style.as_ref() {
                if bounds
                    .intersect(&cx.content_mask().bounds)
                    .contains(&mouse_position)
                    && cx.was_top_layer(&mouse_position, cx.stacking_order())
                {
                    style.refine(hover_style);
                }
            }

            if let Some(drag) = cx.active_drag.take() {
                for (state_type, group_drag_style) in &self.group_drag_over_styles {
                    if let Some(group_bounds) = GroupBounds::get(&group_drag_style.group, cx) {
                        if *state_type == drag.view.entity_type()
                            && group_bounds.contains(&mouse_position)
                        {
                            style.refine(&group_drag_style.style);
                        }
                    }
                }

                for (state_type, drag_over_style) in &self.drag_over_styles {
                    if *state_type == drag.view.entity_type()
                        && bounds
                            .intersect(&cx.content_mask().bounds)
                            .contains(&mouse_position)
                    {
                        style.refine(drag_over_style);
                    }
                }

                cx.active_drag = Some(drag);
            }
        }

        let clicked_state = element_state
            .clicked_state
            .get_or_insert_with(Default::default)
            .borrow();
        if clicked_state.group {
            if let Some(group) = self.group_active_style.as_ref() {
                style.refine(&group.style)
            }
        }

        if let Some(active_style) = self.active_style.as_ref() {
            if clicked_state.element {
                style.refine(active_style)
            }
        }

        style
    }
}

impl Default for Interactivity {
    fn default() -> Self {
        Self {
            element_id: None,
            key_context: None,
            focusable: false,
            tracked_focus_handle: None,
            scroll_handle: None,
            // scroll_offset: Point::default(),
            group: None,
            base_style: Box::new(StyleRefinement::default()),
            focus_style: None,
            in_focus_style: None,
            hover_style: None,
            group_hover_style: None,
            active_style: None,
            group_active_style: None,
            drag_over_styles: Vec::new(),
            group_drag_over_styles: Vec::new(),
            mouse_down_listeners: Vec::new(),
            mouse_up_listeners: Vec::new(),
            mouse_move_listeners: Vec::new(),
            scroll_wheel_listeners: Vec::new(),
            key_down_listeners: Vec::new(),
            key_up_listeners: Vec::new(),
            action_listeners: Vec::new(),
            drop_listeners: Vec::new(),
            click_listeners: Vec::new(),
            drag_listener: None,
            hover_listener: None,
            tooltip_builder: None,
        }
    }
}

#[derive(Default)]
pub struct InteractiveElementState {
    pub focus_handle: Option<FocusHandle>,
    pub clicked_state: Option<Rc<RefCell<ElementClickedState>>>,
    pub hover_state: Option<Rc<RefCell<bool>>>,
    pub pending_mouse_down: Option<Rc<RefCell<Option<MouseDownEvent>>>>,
    pub scroll_offset: Option<Rc<RefCell<Point<Pixels>>>>,
    pub active_tooltip: Option<Rc<RefCell<Option<ActiveTooltip>>>>,
}

pub struct ActiveTooltip {
    tooltip: Option<AnyTooltip>,
    _task: Option<Task<()>>,
}

/// Whether or not the element or a group that contains it is clicked by the mouse.
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct ElementClickedState {
    pub group: bool,
    pub element: bool,
}

impl ElementClickedState {
    fn is_clicked(&self) -> bool {
        self.group || self.element
    }
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

pub struct Focusable<E> {
    pub element: E,
}

impl<E: InteractiveElement> FocusableElement for Focusable<E> {}

impl<E> InteractiveElement for Focusable<E>
where
    E: InteractiveElement,
{
    fn interactivity(&mut self) -> &mut Interactivity {
        self.element.interactivity()
    }
}

impl<E: StatefulInteractiveElement> StatefulInteractiveElement for Focusable<E> {}

impl<E> Styled for Focusable<E>
where
    E: Styled,
{
    fn style(&mut self) -> &mut StyleRefinement {
        self.element.style()
    }
}

impl<E> Element for Focusable<E>
where
    E: Element,
{
    type State = E::State;

    fn layout(
        &mut self,
        state: Option<Self::State>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::State) {
        self.element.layout(state, cx)
    }

    fn paint(self, bounds: Bounds<Pixels>, state: &mut Self::State, cx: &mut WindowContext) {
        self.element.paint(bounds, state, cx)
    }
}

impl<E> IntoElement for Focusable<E>
where
    E: IntoElement,
{
    type Element = E::Element;

    fn element_id(&self) -> Option<ElementId> {
        self.element.element_id()
    }

    fn into_element(self) -> Self::Element {
        self.element.into_element()
    }
}

impl<E> ParentElement for Focusable<E>
where
    E: ParentElement,
{
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        self.element.children_mut()
    }
}

pub struct Stateful<E> {
    element: E,
}

impl<E> Styled for Stateful<E>
where
    E: Styled,
{
    fn style(&mut self) -> &mut StyleRefinement {
        self.element.style()
    }
}

impl<E> StatefulInteractiveElement for Stateful<E>
where
    E: Element,
    Self: InteractiveElement,
{
}

impl<E> InteractiveElement for Stateful<E>
where
    E: InteractiveElement,
{
    fn interactivity(&mut self) -> &mut Interactivity {
        self.element.interactivity()
    }
}

impl<E: FocusableElement> FocusableElement for Stateful<E> {}

impl<E> Element for Stateful<E>
where
    E: Element,
{
    type State = E::State;

    fn layout(
        &mut self,
        state: Option<Self::State>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::State) {
        self.element.layout(state, cx)
    }

    fn paint(self, bounds: Bounds<Pixels>, state: &mut Self::State, cx: &mut WindowContext) {
        self.element.paint(bounds, state, cx)
    }
}

impl<E> IntoElement for Stateful<E>
where
    E: Element,
{
    type Element = Self;

    fn element_id(&self) -> Option<ElementId> {
        self.element.element_id()
    }

    fn into_element(self) -> Self::Element {
        self
    }
}

impl<E> ParentElement for Stateful<E>
where
    E: ParentElement,
{
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        self.element.children_mut()
    }
}

#[derive(Default)]
struct ScrollHandleState {
    // not great to have the nested rc's...
    offset: Rc<RefCell<Point<Pixels>>>,
    bounds: Bounds<Pixels>,
    child_bounds: Vec<Bounds<Pixels>>,
    requested_scroll_top: Option<(usize, Pixels)>,
    overflow: Point<Overflow>,
}

#[derive(Clone)]
pub struct ScrollHandle(Rc<RefCell<ScrollHandleState>>);

impl ScrollHandle {
    pub fn new() -> Self {
        Self(Rc::default())
    }

    pub fn offset(&self) -> Point<Pixels> {
        self.0.borrow().offset.borrow().clone()
    }

    pub fn top_item(&self) -> usize {
        let state = self.0.borrow();
        let top = state.bounds.top() - state.offset.borrow().y;

        match state.child_bounds.binary_search_by(|bounds| {
            if top < bounds.top() {
                Ordering::Greater
            } else if top > bounds.bottom() {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }) {
            Ok(ix) => ix,
            Err(ix) => ix.min(state.child_bounds.len().saturating_sub(1)),
        }
    }

    pub fn bounds_for_item(&self, ix: usize) -> Option<Bounds<Pixels>> {
        self.0.borrow().child_bounds.get(ix).cloned()
    }

    /// scroll_to_item scrolls the minimal amount to ensure that the item is
    /// fully visible
    pub fn scroll_to_item(&self, ix: usize) {
        let state = self.0.borrow();

        let Some(bounds) = state.child_bounds.get(ix) else {
            return;
        };

        let mut scroll_offset = state.offset.borrow_mut();

        if state.overflow.y == Overflow::Scroll {
            if bounds.top() + scroll_offset.y < state.bounds.top() {
                scroll_offset.y = state.bounds.top() - bounds.top();
            } else if bounds.bottom() + scroll_offset.y > state.bounds.bottom() {
                scroll_offset.y = state.bounds.bottom() - bounds.bottom();
            }
        }

        if state.overflow.x == Overflow::Scroll {
            if bounds.left() + scroll_offset.x < state.bounds.left() {
                scroll_offset.x = state.bounds.left() - bounds.left();
            } else if bounds.right() + scroll_offset.x > state.bounds.right() {
                scroll_offset.x = state.bounds.right() - bounds.right();
            }
        }
    }

    pub fn logical_scroll_top(&self) -> (usize, Pixels) {
        let ix = self.top_item();
        let state = self.0.borrow();

        if let Some(child_bounds) = state.child_bounds.get(ix) {
            (
                ix,
                child_bounds.top() + state.offset.borrow().y - state.bounds.top(),
            )
        } else {
            (ix, px(0.))
        }
    }

    pub fn set_logical_scroll_top(&self, ix: usize, px: Pixels) {
        self.0.borrow_mut().requested_scroll_top = Some((ix, px));
    }
}
