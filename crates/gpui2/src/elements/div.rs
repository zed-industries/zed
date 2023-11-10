use std::fmt::Debug;

use crate::{
    point, AnyElement, BorrowWindow, Bounds, Component, Element, ElementId, ElementInteractivity,
    FocusHandle, FocusListeners, Focusable, FocusableKeyDispatch, GlobalElementId, GroupBounds,
    InteractiveElementState, KeyContext, KeyDispatch, LayoutId, NonFocusableKeyDispatch, Overflow,
    ParentElement, Pixels, Point, SharedString, StatefulInteractive, StatefulInteractivity,
    StatelessInteractive, StatelessInteractivity, Style, StyleRefinement, Styled, ViewContext,
    Visibility,
};
use refineable::Refineable;
use smallvec::SmallVec;
use util::ResultExt;

pub struct Div<
    V: 'static,
    I: ElementInteractivity<V> = StatelessInteractivity<V>,
    K: KeyDispatch<V> = NonFocusableKeyDispatch,
> {
    interactivity: I,
    key_dispatch: K,
    children: SmallVec<[AnyElement<V>; 2]>,
    group: Option<SharedString>,
    base_style: StyleRefinement,
}

pub fn div<V: 'static>() -> Div<V, StatelessInteractivity<V>, NonFocusableKeyDispatch> {
    Div {
        interactivity: StatelessInteractivity::default(),
        key_dispatch: NonFocusableKeyDispatch::default(),
        children: SmallVec::new(),
        group: None,
        base_style: StyleRefinement::default(),
    }
}

impl<V, F> Div<V, StatelessInteractivity<V>, F>
where
    V: 'static,
    F: KeyDispatch<V>,
{
    pub fn id(self, id: impl Into<ElementId>) -> Div<V, StatefulInteractivity<V>, F> {
        Div {
            interactivity: StatefulInteractivity::new(id.into(), self.interactivity),
            key_dispatch: self.key_dispatch,
            children: self.children,
            group: self.group,
            base_style: self.base_style,
        }
    }
}

impl<V, I, F> Div<V, I, F>
where
    I: ElementInteractivity<V>,
    F: KeyDispatch<V>,
{
    pub fn group(mut self, group: impl Into<SharedString>) -> Self {
        self.group = Some(group.into());
        self
    }

    pub fn z_index(mut self, z_index: u32) -> Self {
        self.base_style.z_index = Some(z_index);
        self
    }

    pub fn context<C>(mut self, context: C) -> Self
    where
        Self: Sized,
        C: TryInto<KeyContext>,
        C::Error: Debug,
    {
        if let Some(context) = context.try_into().log_err() {
            *self.key_dispatch.key_context_mut() = context;
        }
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

    fn with_element_id<R>(
        &mut self,
        cx: &mut ViewContext<V>,
        f: impl FnOnce(&mut Self, Option<GlobalElementId>, &mut ViewContext<V>) -> R,
    ) -> R {
        if let Some(id) = self.id() {
            cx.with_element_id(id, |global_id, cx| f(self, Some(global_id), cx))
        } else {
            f(self, None, cx)
        }
    }

    pub fn compute_style(
        &self,
        bounds: Bounds<Pixels>,
        element_state: &DivState,
        cx: &mut ViewContext<V>,
    ) -> Style {
        let mut computed_style = Style::default();
        computed_style.refine(&self.base_style);
        self.key_dispatch.refine_style(&mut computed_style, cx);
        self.interactivity.refine_style(
            &mut computed_style,
            bounds,
            &element_state.interactive,
            cx,
        );
        computed_style
    }
}

impl<V: 'static> Div<V, StatefulInteractivity<V>, NonFocusableKeyDispatch> {
    pub fn focusable(self) -> Div<V, StatefulInteractivity<V>, FocusableKeyDispatch<V>> {
        Div {
            interactivity: self.interactivity,
            key_dispatch: FocusableKeyDispatch::new(),
            children: self.children,
            group: self.group,
            base_style: self.base_style,
        }
    }

    pub fn track_focus(
        self,
        handle: &FocusHandle,
    ) -> Div<V, StatefulInteractivity<V>, FocusableKeyDispatch<V>> {
        Div {
            interactivity: self.interactivity,
            key_dispatch: FocusableKeyDispatch::tracked(handle),
            children: self.children,
            group: self.group,
            base_style: self.base_style,
        }
    }

    pub fn overflow_scroll(mut self) -> Self {
        self.base_style.overflow.x = Some(Overflow::Scroll);
        self.base_style.overflow.y = Some(Overflow::Scroll);
        self
    }

    pub fn overflow_x_scroll(mut self) -> Self {
        self.base_style.overflow.x = Some(Overflow::Scroll);
        self
    }

    pub fn overflow_y_scroll(mut self) -> Self {
        self.base_style.overflow.y = Some(Overflow::Scroll);
        self
    }
}

impl<V: 'static> Div<V, StatelessInteractivity<V>, NonFocusableKeyDispatch> {
    pub fn track_focus(
        self,
        handle: &FocusHandle,
    ) -> Div<V, StatefulInteractivity<V>, FocusableKeyDispatch<V>> {
        Div {
            interactivity: self.interactivity.into_stateful(handle),
            key_dispatch: handle.clone().into(),
            children: self.children,
            group: self.group,
            base_style: self.base_style,
        }
    }
}

impl<V, I> Focusable<V> for Div<V, I, FocusableKeyDispatch<V>>
where
    V: 'static,
    I: ElementInteractivity<V>,
{
    fn focus_listeners(&mut self) -> &mut FocusListeners<V> {
        &mut self.key_dispatch.focus_listeners
    }

    fn set_focus_style(&mut self, style: StyleRefinement) {
        self.key_dispatch.focus_style = style;
    }

    fn set_focus_in_style(&mut self, style: StyleRefinement) {
        self.key_dispatch.focus_in_style = style;
    }

    fn set_in_focus_style(&mut self, style: StyleRefinement) {
        self.key_dispatch.in_focus_style = style;
    }
}

#[derive(Default)]
pub struct DivState {
    interactive: InteractiveElementState,
    focus_handle: Option<FocusHandle>,
    child_layout_ids: SmallVec<[LayoutId; 4]>,
}

impl<V, I, F> Element<V> for Div<V, I, F>
where
    I: ElementInteractivity<V>,
    F: KeyDispatch<V>,
{
    type ElementState = DivState;

    fn id(&self) -> Option<ElementId> {
        self.interactivity
            .as_stateful()
            .map(|identified| identified.id.clone())
    }

    fn initialize(
        &mut self,
        view_state: &mut V,
        element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<V>,
    ) -> Self::ElementState {
        let mut element_state = element_state.unwrap_or_default();
        self.with_element_id(cx, |this, _global_id, cx| {
            this.key_dispatch.initialize(
                element_state.focus_handle.take(),
                cx,
                |focus_handle, cx| {
                    element_state.focus_handle = focus_handle;
                    for child in &mut this.children {
                        child.initialize(view_state, cx);
                    }
                },
            );
        });
        element_state
    }

    fn layout(
        &mut self,
        view_state: &mut V,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    ) -> LayoutId {
        let style = self.compute_style(Bounds::default(), element_state, cx);
        style.apply_text_style(cx, |cx| {
            self.with_element_id(cx, |this, _global_id, cx| {
                let layout_ids = this
                    .children
                    .iter_mut()
                    .map(|child| child.layout(view_state, cx))
                    .collect::<SmallVec<_>>();
                element_state.child_layout_ids = layout_ids.clone();
                cx.request_layout(&style, layout_ids)
            })
        })
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        view_state: &mut V,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    ) {
        self.with_element_id(cx, |this, _global_id, cx| {
            let style = this.compute_style(bounds, element_state, cx);
            if style.visibility == Visibility::Hidden {
                return;
            }

            if let Some(mouse_cursor) = style.mouse_cursor {
                let hovered = bounds.contains_point(&cx.mouse_position());
                if hovered {
                    cx.set_cursor_style(mouse_cursor);
                }
            }

            if let Some(group) = this.group.clone() {
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

            cx.with_z_index(z_index, |cx| {
                cx.with_z_index(0, |cx| {
                    style.paint(bounds, cx);
                    this.key_dispatch.paint(bounds, cx);
                    this.interactivity.paint(
                        bounds,
                        content_size,
                        style.overflow,
                        &mut element_state.interactive,
                        cx,
                    );
                });
                cx.with_z_index(1, |cx| {
                    style.apply_text_style(cx, |cx| {
                        style.apply_overflow(bounds, cx, |cx| {
                            let scroll_offset = element_state.interactive.scroll_offset();
                            cx.with_element_offset(scroll_offset, |cx| {
                                for child in &mut this.children {
                                    child.paint(view_state, cx);
                                }
                            });
                        })
                    })
                });
            });

            if let Some(group) = this.group.as_ref() {
                GroupBounds::pop(group, cx);
            }
        })
    }
}

impl<V, I, F> Component<V> for Div<V, I, F>
where
    I: ElementInteractivity<V>,
    F: KeyDispatch<V>,
{
    fn render(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}

impl<V, I, F> ParentElement<V> for Div<V, I, F>
where
    I: ElementInteractivity<V>,
    F: KeyDispatch<V>,
{
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<V>; 2]> {
        &mut self.children
    }
}

impl<V, I, F> Styled for Div<V, I, F>
where
    I: ElementInteractivity<V>,
    F: KeyDispatch<V>,
{
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.base_style
    }
}

impl<V, I, F> StatelessInteractive<V> for Div<V, I, F>
where
    I: ElementInteractivity<V>,
    F: KeyDispatch<V>,
{
    fn stateless_interactivity(&mut self) -> &mut StatelessInteractivity<V> {
        self.interactivity.as_stateless_mut()
    }
}

impl<V, F> StatefulInteractive<V> for Div<V, StatefulInteractivity<V>, F>
where
    F: KeyDispatch<V>,
{
    fn stateful_interactivity(&mut self) -> &mut StatefulInteractivity<V> {
        &mut self.interactivity
    }
}
