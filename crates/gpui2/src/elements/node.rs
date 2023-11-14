use crate::{
    point, Action, AnyDrag, AnyElement, AnyView, AppContext, BorrowWindow, Bounds, ClickEvent,
    DispatchPhase, Element, FocusHandle, KeyContext, KeyDownEvent, KeyUpEvent, LayoutId,
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point, Render,
    ScrollWheelEvent, SharedString, Style, StyleRefinement, Styled, View, ViewContext, Visibility,
};
use collections::HashMap;
use refineable::Refineable;
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    marker::PhantomData,
    mem,
    sync::Arc,
};

pub struct GroupStyle {
    pub group: SharedString,
    pub style: StyleRefinement,
}

pub trait InteractiveComponent<V: 'static> {
    fn interactivity(&mut self) -> &mut Interactivity<V>;

    fn hover(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self
    where
        Self: Sized,
    {
        self.interactivity().hover_style = f(StyleRefinement::default());
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
        self.interactivity().group_hover_style = Some(GroupStyle {
            group: group_name.into(),
            style: f(StyleRefinement::default()),
        });
        self
    }

    fn on_mouse_down(
        mut self,
        button: MouseButton,
        handler: impl Fn(&mut V, &MouseDownEvent, &mut ViewContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.interactivity().mouse_down_listeners.push(Box::new(
            move |view, event, bounds, phase, cx| {
                if phase == DispatchPhase::Bubble
                    && event.button == button
                    && bounds.contains_point(&event.position)
                {
                    handler(view, event, cx)
                }
            },
        ));
        self
    }

    fn on_mouse_up(
        mut self,
        button: MouseButton,
        handler: impl Fn(&mut V, &MouseUpEvent, &mut ViewContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.interactivity().mouse_up_listeners.push(Box::new(
            move |view, event, bounds, phase, cx| {
                if phase == DispatchPhase::Bubble
                    && event.button == button
                    && bounds.contains_point(&event.position)
                {
                    handler(view, event, cx)
                }
            },
        ));
        self
    }

    fn on_mouse_down_out(
        mut self,
        handler: impl Fn(&mut V, &MouseDownEvent, &mut ViewContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.interactivity().mouse_down_listeners.push(Box::new(
            move |view, event, bounds, phase, cx| {
                if phase == DispatchPhase::Capture && !bounds.contains_point(&event.position) {
                    handler(view, event, cx)
                }
            },
        ));
        self
    }

    fn on_mouse_up_out(
        mut self,
        button: MouseButton,
        handler: impl Fn(&mut V, &MouseUpEvent, &mut ViewContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.interactivity().mouse_up_listeners.push(Box::new(
            move |view, event, bounds, phase, cx| {
                if phase == DispatchPhase::Capture
                    && event.button == button
                    && !bounds.contains_point(&event.position)
                {
                    handler(view, event, cx);
                }
            },
        ));
        self
    }

    fn on_mouse_move(
        mut self,
        handler: impl Fn(&mut V, &MouseMoveEvent, &mut ViewContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.interactivity().mouse_move_listeners.push(Box::new(
            move |view, event, bounds, phase, cx| {
                if phase == DispatchPhase::Bubble && bounds.contains_point(&event.position) {
                    handler(view, event, cx);
                }
            },
        ));
        self
    }

    fn on_scroll_wheel(
        mut self,
        handler: impl Fn(&mut V, &ScrollWheelEvent, &mut ViewContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.interactivity().scroll_wheel_listeners.push(Box::new(
            move |view, event, bounds, phase, cx| {
                if phase == DispatchPhase::Bubble && bounds.contains_point(&event.position) {
                    handler(view, event, cx);
                }
            },
        ));
        self
    }

    /// Capture the given action, fires during the capture phase
    fn capture_action<A: Action>(
        mut self,
        listener: impl Fn(&mut V, &A, &mut ViewContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.interactivity().action_listeners.push((
            TypeId::of::<A>(),
            Box::new(move |view, action, phase, cx| {
                let action = action.downcast_ref().unwrap();
                if phase == DispatchPhase::Capture {
                    listener(view, action, cx)
                }
            }),
        ));
        self
    }

    /// Add a listener for the given action, fires during the bubble event phase
    fn on_action<A: Action>(
        mut self,
        listener: impl Fn(&mut V, &A, &mut ViewContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.interactivity().action_listeners.push((
            TypeId::of::<A>(),
            Box::new(move |view, action, phase, cx| {
                let action = action.downcast_ref().unwrap();
                if phase == DispatchPhase::Bubble {
                    listener(view, action, cx)
                }
            }),
        ));
        self
    }

    fn on_key_down(
        mut self,
        listener: impl Fn(&mut V, &KeyDownEvent, DispatchPhase, &mut ViewContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.interactivity()
            .key_down_listeners
            .push(Box::new(move |view, event, phase, cx| {
                listener(view, event, phase, cx)
            }));
        self
    }

    fn on_key_up(
        mut self,
        listener: impl Fn(&mut V, &KeyUpEvent, DispatchPhase, &mut ViewContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.interactivity()
            .key_up_listeners
            .push(Box::new(move |view, event, phase, cx| {
                listener(view, event, phase, cx)
            }));
        self
    }

    fn drag_over<S: 'static>(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self
    where
        Self: Sized,
    {
        self.interactivity()
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
        self.interactivity().group_drag_over_styles.push((
            TypeId::of::<S>(),
            GroupStyle {
                group: group_name.into(),
                style: f(StyleRefinement::default()),
            },
        ));
        self
    }

    fn on_drop<W: 'static>(
        mut self,
        listener: impl Fn(&mut V, View<W>, &mut ViewContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.interactivity().drop_listeners.push((
            TypeId::of::<W>(),
            Box::new(move |view, dragged_view, cx| {
                listener(view, dragged_view.downcast().unwrap(), cx);
            }),
        ));
        self
    }
}

pub trait StatefulInteractiveComponent<V: 'static>: InteractiveComponent<V> {
    fn active(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self
    where
        Self: Sized,
    {
        self.interactivity().active_style = f(StyleRefinement::default());
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
            style: f(StyleRefinement::default()),
        });
        self
    }

    fn on_click(
        mut self,
        listener: impl Fn(&mut V, &ClickEvent, &mut ViewContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.interactivity()
            .click_listeners
            .push(Box::new(move |view, event, cx| listener(view, event, cx)));
        self
    }

    fn on_drag<W>(
        mut self,
        listener: impl Fn(&mut V, &mut ViewContext<V>) -> View<W> + 'static,
    ) -> Self
    where
        Self: Sized,
        W: 'static + Render,
    {
        debug_assert!(
            self.interactivity().drag_listener.is_none(),
            "calling on_drag more than once on the same element is not supported"
        );
        self.interactivity().drag_listener =
            Some(Box::new(move |view_state, cursor_offset, cx| AnyDrag {
                view: listener(view_state, cx).into(),
                cursor_offset,
            }));
        self
    }

    fn on_hover(mut self, listener: impl 'static + Fn(&mut V, bool, &mut ViewContext<V>)) -> Self
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

    fn tooltip<W>(
        mut self,
        build_tooltip: impl Fn(&mut V, &mut ViewContext<V>) -> View<W> + 'static,
    ) -> Self
    where
        Self: Sized,
        W: 'static + Render,
    {
        debug_assert!(
            self.interactivity().tooltip_builder.is_none(),
            "calling tooltip more than once on the same element is not supported"
        );
        self.interactivity().tooltip_builder = Some(Arc::new(move |view_state, cx| {
            build_tooltip(view_state, cx).into()
        }));

        self
    }
}

pub trait FocusableComponent<V> {
    fn focusability(&mut self) -> &mut Focusability<V>;

    fn focus(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self
    where
        Self: Sized,
    {
        self.focusability().focus_style = f(StyleRefinement::default());
        self
    }

    fn focus_in(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self
    where
        Self: Sized,
    {
        self.focusability().focus_in_style = f(StyleRefinement::default());
        self
    }

    fn in_focus(mut self, f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Self
    where
        Self: Sized,
    {
        self.focusability().in_focus_style = f(StyleRefinement::default());
        self
    }

    fn on_focus(
        mut self,
        listener: impl Fn(&mut V, &FocusEvent, &mut ViewContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.focusability()
            .focus_listeners
            .push(Box::new(move |view, focus_handle, event, cx| {
                if event.focused.as_ref() == Some(focus_handle) {
                    listener(view, event, cx)
                }
            }));
        self
    }

    fn on_blur(
        mut self,
        listener: impl Fn(&mut V, &FocusEvent, &mut ViewContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.focusability()
            .focus_listeners
            .push(Box::new(move |view, focus_handle, event, cx| {
                if event.blurred.as_ref() == Some(focus_handle) {
                    listener(view, event, cx)
                }
            }));
        self
    }

    fn on_focus_in(
        mut self,
        listener: impl Fn(&mut V, &FocusEvent, &mut ViewContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.focusability()
            .focus_listeners
            .push(Box::new(move |view, focus_handle, event, cx| {
                let descendant_blurred = event
                    .blurred
                    .as_ref()
                    .map_or(false, |blurred| focus_handle.contains(blurred, cx));
                let descendant_focused = event
                    .focused
                    .as_ref()
                    .map_or(false, |focused| focus_handle.contains(focused, cx));

                if !descendant_blurred && descendant_focused {
                    listener(view, event, cx)
                }
            }));
        self
    }

    fn on_focus_out(
        mut self,
        listener: impl Fn(&mut V, &FocusEvent, &mut ViewContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.focusability()
            .focus_listeners
            .push(Box::new(move |view, focus_handle, event, cx| {
                let descendant_blurred = event
                    .blurred
                    .as_ref()
                    .map_or(false, |blurred| focus_handle.contains(blurred, cx));
                let descendant_focused = event
                    .focused
                    .as_ref()
                    .map_or(false, |focused| focus_handle.contains(focused, cx));
                if descendant_blurred && !descendant_focused {
                    listener(view, event, cx)
                }
            }));
        self
    }
}

pub type FocusListeners<V> = SmallVec<[FocusListener<V>; 2]>;

pub type FocusListener<V> =
    Box<dyn Fn(&mut V, &FocusHandle, &FocusEvent, &mut ViewContext<V>) + 'static>;

pub type MouseDownListener<V> = Box<
    dyn Fn(&mut V, &MouseDownEvent, &Bounds<Pixels>, DispatchPhase, &mut ViewContext<V>) + 'static,
>;
pub type MouseUpListener<V> = Box<
    dyn Fn(&mut V, &MouseUpEvent, &Bounds<Pixels>, DispatchPhase, &mut ViewContext<V>) + 'static,
>;

pub type MouseMoveListener<V> = Box<
    dyn Fn(&mut V, &MouseMoveEvent, &Bounds<Pixels>, DispatchPhase, &mut ViewContext<V>) + 'static,
>;

pub type ScrollWheelListener<V> = Box<
    dyn Fn(&mut V, &ScrollWheelEvent, &Bounds<Pixels>, DispatchPhase, &mut ViewContext<V>)
        + 'static,
>;

pub type ClickListener<V> = Box<dyn Fn(&mut V, &ClickEvent, &mut ViewContext<V>) + 'static>;

pub type DragListener<V> =
    Box<dyn Fn(&mut V, Point<Pixels>, &mut ViewContext<V>) -> AnyDrag + 'static>;

type DropListener<V> = dyn Fn(&mut V, AnyView, &mut ViewContext<V>) + 'static;

pub type HoverListener<V> = Box<dyn Fn(&mut V, bool, &mut ViewContext<V>) + 'static>;

pub type TooltipBuilder<V> = Arc<dyn Fn(&mut V, &mut ViewContext<V>) -> AnyView + 'static>;

pub type KeyDownListener<V> =
    Box<dyn Fn(&mut V, &KeyDownEvent, DispatchPhase, &mut ViewContext<V>) + 'static>;

pub type KeyUpListener<V> =
    Box<dyn Fn(&mut V, &KeyUpEvent, DispatchPhase, &mut ViewContext<V>) + 'static>;

pub type ActionListener<V> =
    Box<dyn Fn(&mut V, &dyn Any, DispatchPhase, &mut ViewContext<V>) + 'static>;

pub struct FocusEvent {
    pub blurred: Option<FocusHandle>,
    pub focused: Option<FocusHandle>,
}

pub struct Node<V> {
    interactivity: Interactivity<V>,
    children: Vec<AnyElement<V>>,
}

impl<V> Styled for Node<V> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity.base_style
    }
}

impl<V: 'static> InteractiveComponent<V> for Node<V> {
    fn interactivity(&mut self) -> &mut Interactivity<V> {
        &mut self.interactivity
    }
}

pub struct NodeState {
    child_layout_ids: SmallVec<[LayoutId; 4]>,
}

impl<V: 'static> Element<V> for Node<V> {
    type ElementState = NodeState;

    fn id(&self) -> Option<crate::ElementId> {
        None
    }

    fn initialize(
        &mut self,
        view_state: &mut V,
        _: Option<Self::ElementState>,
        cx: &mut ViewContext<V>,
    ) -> Self::ElementState {
        for child in &mut self.children {
            child.initialize(view_state, cx);
        }
        NodeState {
            child_layout_ids: SmallVec::new(),
        }
    }

    fn layout(
        &mut self,
        view_state: &mut V,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    ) -> crate::LayoutId {
        let style = self.interactivity().compute_style(None, cx);
        style.with_text_style(cx, |cx| {
            element_state.child_layout_ids = self
                .children
                .iter_mut()
                .map(|child| child.layout(view_state, cx))
                .collect::<SmallVec<_>>();
            cx.request_layout(&style, element_state.child_layout_ids.iter().copied())
        })
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        view_state: &mut V,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    ) {
        let style = self.interactivity.compute_style(Some(bounds), cx);
        if style.visibility == Visibility::Hidden {
            return;
        }

        if let Some(mouse_cursor) = style.mouse_cursor {
            let hovered = bounds.contains_point(&cx.mouse_position());
            if hovered {
                cx.set_cursor_style(mouse_cursor);
            }
        }

        if let Some(group) = self.interactivity.group.clone() {
            GroupBounds::push(group, bounds, cx);
        }

        let z_index = style.z_index.unwrap_or(0);

        let mut child_min = point(Pixels::MAX, Pixels::MAX);
        let mut child_max = Point::default();

        let content_size = if element_state.child_layout_ids.is_empty() {
            bounds.size
        } else {
            for child_layout_id in &element_state.child_layout_ids {
                let child_bounds = cx.layout_bounds(*child_layout_id);
                child_min = child_min.min(&child_bounds.origin);
                child_max = child_max.max(&child_bounds.lower_right());
            }
            (child_max - child_min).into()
        };

        let mut interactivity = mem::take(&mut self.interactivity);
        interactivity.paint(bounds, cx, |cx| {
            cx.with_z_index(z_index, |cx| {
                cx.with_z_index(0, |cx| {
                    style.paint(bounds, cx);
                });
                cx.with_z_index(1, |cx| {
                    style.with_text_style(cx, |cx| {
                        style.apply_overflow(bounds, cx, |cx| {
                            let scroll_offset = self.interactivity.scroll_offset;
                            cx.with_element_offset2(scroll_offset, |cx| {
                                for child in &mut self.children {
                                    child.paint(view_state, cx);
                                }
                            });
                        })
                    })
                });
            });
        });
        self.interactivity = interactivity;

        if let Some(group) = self.interactivity.group.as_ref() {
            GroupBounds::pop(group, cx);
        }
    }
}

pub enum FocusState {
    /// The current element is not focused, and does not contain or descend from the focused element.
    None,
    /// The current element is focused.
    Focus,
    /// The current element contains the focused element
    FocusIn,
    /// The current element descends from the focused element
    InFocus,
}

pub struct Interactivity<V> {
    pub active: bool,
    pub group_active: bool,
    pub hovered: bool,
    pub group_hovered: bool,
    pub focus: FocusState,
    pub key_context: KeyContext,
    pub focus_handle: Option<FocusHandle>,
    pub scroll_offset: Point<Pixels>,
    pub base_style: StyleRefinement,
    pub focus_style: StyleRefinement,
    pub focus_in_style: StyleRefinement,
    pub in_focus_style: StyleRefinement,
    pub hover_style: StyleRefinement,
    pub group_hover_style: Option<GroupStyle>,
    pub active_style: StyleRefinement,
    pub group_active_style: Option<GroupStyle>,
    pub drag_over_styles: SmallVec<[(TypeId, StyleRefinement); 2]>,
    pub group_drag_over_styles: SmallVec<[(TypeId, GroupStyle); 2]>,
    pub group: Option<SharedString>,
    pub dispatch_context: KeyContext,
    pub mouse_down_listeners: SmallVec<[MouseDownListener<V>; 2]>,
    pub mouse_up_listeners: SmallVec<[MouseUpListener<V>; 2]>,
    pub mouse_move_listeners: SmallVec<[MouseMoveListener<V>; 2]>,
    pub scroll_wheel_listeners: SmallVec<[ScrollWheelListener<V>; 2]>,
    pub key_down_listeners: SmallVec<[KeyDownListener<V>; 2]>,
    pub key_up_listeners: SmallVec<[KeyUpListener<V>; 2]>,
    pub action_listeners: SmallVec<[(TypeId, ActionListener<V>); 8]>,
    pub drop_listeners: SmallVec<[(TypeId, Box<DropListener<V>>); 2]>,
    pub click_listeners: SmallVec<[ClickListener<V>; 2]>,
    pub drag_listener: Option<DragListener<V>>,
    pub hover_listener: Option<HoverListener<V>>,
    pub tooltip_builder: Option<TooltipBuilder<V>>,
}

impl<V: 'static> Interactivity<V> {
    fn compute_style(&self, bounds: Option<Bounds<Pixels>>, cx: &mut ViewContext<V>) -> Style {
        let mut style = Style::default();
        style.refine(&self.base_style);

        match self.focus {
            FocusState::None => {}
            FocusState::Focus => {
                style.refine(&self.focus_style);
                style.refine(&self.focus_in_style);
                style.refine(&self.in_focus_style);
            }
            FocusState::FocusIn => {
                style.refine(&self.focus_in_style);
            }
            FocusState::InFocus => {
                style.refine(&self.in_focus_style);
            }
        }

        if let Some(bounds) = bounds {
            let mouse_position = cx.mouse_position();
            if let Some(group_hover) = self.group_hover_style.as_ref() {
                if let Some(group_bounds) = GroupBounds::get(&group_hover.group, cx) {
                    if group_bounds.contains_point(&mouse_position) {
                        style.refine(&group_hover.style);
                    }
                }
            }
            if bounds.contains_point(&mouse_position) {
                style.refine(&self.hover_style);
            }

            if let Some(drag) = cx.active_drag.take() {
                for (state_type, group_drag_style) in &self.group_drag_over_styles {
                    if let Some(group_bounds) = GroupBounds::get(&group_drag_style.group, cx) {
                        if *state_type == drag.view.entity_type()
                            && group_bounds.contains_point(&mouse_position)
                        {
                            style.refine(&group_drag_style.style);
                        }
                    }
                }

                for (state_type, drag_over_style) in &self.drag_over_styles {
                    if *state_type == drag.view.entity_type()
                        && bounds.contains_point(&mouse_position)
                    {
                        style.refine(drag_over_style);
                    }
                }

                cx.active_drag = Some(drag);
            }
        }

        if self.group_active {
            if let Some(group) = self.group_active_style.as_ref() {
                style.refine(&group.style)
            }
        }

        if self.active {
            style.refine(&self.active_style)
        }

        style
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        cx: &mut ViewContext<V>,
        f: impl FnOnce(&mut ViewContext<V>),
    ) {
        for listener in self.mouse_down_listeners.drain(..) {
            cx.on_mouse_event(move |state, event: &MouseDownEvent, phase, cx| {
                listener(state, event, &bounds, phase, cx);
            })
        }

        for listener in self.mouse_up_listeners.drain(..) {
            cx.on_mouse_event(move |state, event: &MouseUpEvent, phase, cx| {
                listener(state, event, &bounds, phase, cx);
            })
        }

        for listener in self.mouse_move_listeners.drain(..) {
            cx.on_mouse_event(move |state, event: &MouseMoveEvent, phase, cx| {
                listener(state, event, &bounds, phase, cx);
            })
        }

        for listener in self.scroll_wheel_listeners.drain(..) {
            cx.on_mouse_event(move |state, event: &ScrollWheelEvent, phase, cx| {
                listener(state, event, &bounds, phase, cx);
            })
        }

        let hover_group_bounds = self
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

        if self.hover_style.is_some()
            || (cx.active_drag.is_some() && !self.drag_over_styles.is_empty())
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
            let drop_listeners = mem::take(&mut self.drop_listeners);
            cx.on_mouse_event(move |view, event: &MouseUpEvent, phase, cx| {
                if phase == DispatchPhase::Bubble && bounds.contains_point(&event.position) {
                    if let Some(drag_state_type) =
                        cx.active_drag.as_ref().map(|drag| drag.view.entity_type())
                    {
                        for (drop_state_type, listener) in &drop_listeners {
                            if *drop_state_type == drag_state_type {
                                let drag = cx
                                    .active_drag
                                    .take()
                                    .expect("checked for type drag state type above");
                                listener(view, drag.view.clone(), cx);
                                cx.notify();
                                cx.stop_propagation();
                            }
                        }
                    }
                }
            });
        }

        cx.with_key_dispatch(
            self.key_context.clone(),
            self.focus_handle.clone(),
            |_, cx| f(cx),
        );
    }
}

impl<V: 'static> Default for Interactivity<V> {
    fn default() -> Self {
        Self {
            active: false,
            group_active: false,
            hovered: false,
            group_hovered: false,
            focus: FocusState::None,
            key_context: KeyContext::default(),
            focus_handle: None,
            scroll_offset: Point::default(),
            base_style: StyleRefinement::default(),
            focus_style: StyleRefinement::default(),
            focus_in_style: StyleRefinement::default(),
            in_focus_style: StyleRefinement::default(),
            hover_style: StyleRefinement::default(),
            group_hover_style: None,
            active_style: StyleRefinement::default(),
            group_active_style: None,
            drag_over_styles: SmallVec::new(),
            group_drag_over_styles: SmallVec::new(),
            group: None,
            dispatch_context: KeyContext::default(),
            mouse_down_listeners: SmallVec::new(),
            mouse_up_listeners: SmallVec::new(),
            mouse_move_listeners: SmallVec::new(),
            scroll_wheel_listeners: SmallVec::new(),
            key_down_listeners: SmallVec::new(),
            key_up_listeners: SmallVec::new(),
            action_listeners: SmallVec::new(),
            drop_listeners: SmallVec::new(),
            click_listeners: SmallVec::new(),
            drag_listener: None,
            hover_listener: None,
            tooltip_builder: None,
        }
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

pub struct Focusable<V, E> {
    focusability: Focusability<V>,
    view_type: PhantomData<V>,
    element: E,
}

pub struct Focusability<V> {
    focus_handle: Option<FocusHandle>,
    focus_listeners: FocusListeners<V>,
    focus_style: StyleRefinement,
    focus_in_style: StyleRefinement,
    in_focus_style: StyleRefinement,
}

impl<V, E> FocusableComponent<V> for Focusable<V, E> {
    fn focusability(&mut self) -> &mut Focusability<V> {
        &mut self.focusability
    }
}

impl<V: 'static, E: InteractiveComponent<V>> InteractiveComponent<V> for Focusable<V, E> {
    fn interactivity(&mut self) -> &mut Interactivity<V> {
        self.element.interactivity()
    }
}

impl<V: 'static, E: StatefulInteractiveComponent<V>> StatefulInteractiveComponent<V>
    for Focusable<V, E>
{
}

pub struct Stateful<V, E> {
    id: SharedString,
    view_type: PhantomData<V>,
    element: E,
}

impl<V: 'static, E: InteractiveComponent<V>> StatefulInteractiveComponent<V> for Stateful<V, E> {}

impl<V: 'static, E: InteractiveComponent<V>> InteractiveComponent<V> for Stateful<V, E> {
    fn interactivity(&mut self) -> &mut Interactivity<V> {
        self.element.interactivity()
    }
}

impl<V, E: FocusableComponent<V>> FocusableComponent<V> for Stateful<V, E> {
    fn focusability(&mut self) -> &mut Focusability<V> {
        self.element.focusability()
    }
}
