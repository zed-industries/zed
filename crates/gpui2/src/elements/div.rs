use crate::{
    point, AnyElement, BorrowWindow, Bounds, Element, ElementFocus, ElementId, ElementInteraction,
    FocusDisabled, FocusEnabled, FocusHandle, FocusListeners, Focusable, GlobalElementId,
    GroupBounds, InteractiveElementState, IntoAnyElement, LayoutId, Overflow, ParentElement,
    Pixels, Point, SharedString, StatefulInteraction, StatefulInteractive, StatelessInteraction,
    StatelessInteractive, Style, StyleRefinement, Styled, ViewContext,
};
use refineable::Refineable;
use smallvec::SmallVec;

pub struct Div<
    V: 'static + Send + Sync,
    I: ElementInteraction<V> = StatelessInteraction<V>,
    F: ElementFocus<V> = FocusDisabled,
> {
    interaction: I,
    focus: F,
    children: SmallVec<[AnyElement<V>; 2]>,
    group: Option<SharedString>,
    base_style: StyleRefinement,
}

pub fn div<V>() -> Div<V, StatelessInteraction<V>, FocusDisabled>
where
    V: 'static + Send + Sync,
{
    Div {
        interaction: StatelessInteraction::default(),
        focus: FocusDisabled,
        children: SmallVec::new(),
        group: None,
        base_style: StyleRefinement::default(),
    }
}

impl<V, F> Div<V, StatelessInteraction<V>, F>
where
    F: ElementFocus<V>,
    V: 'static + Send + Sync,
{
    pub fn id(self, id: impl Into<ElementId>) -> Div<V, StatefulInteraction<V>, F> {
        Div {
            interaction: id.into().into(),
            focus: self.focus,
            children: self.children,
            group: self.group,
            base_style: self.base_style,
        }
    }
}

impl<V, I, F> Div<V, I, F>
where
    I: ElementInteraction<V>,
    F: ElementFocus<V>,
    V: 'static + Send + Sync,
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
        self.focus.refine_style(&mut computed_style, cx);
        self.interaction
            .refine_style(&mut computed_style, bounds, &element_state.interactive, cx);
        computed_style
    }
}

impl<V> Div<V, StatefulInteraction<V>, FocusDisabled>
where
    V: 'static + Send + Sync,
{
    pub fn focusable(self) -> Div<V, StatefulInteraction<V>, FocusEnabled<V>> {
        Div {
            interaction: self.interaction,
            focus: FocusEnabled::new(),
            children: self.children,
            group: self.group,
            base_style: self.base_style,
        }
    }

    pub fn track_focus(
        self,
        handle: &FocusHandle,
    ) -> Div<V, StatefulInteraction<V>, FocusEnabled<V>> {
        Div {
            interaction: self.interaction,
            focus: FocusEnabled::tracked(handle),
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

impl<V> Div<V, StatelessInteraction<V>, FocusDisabled>
where
    V: 'static + Send + Sync,
{
    pub fn track_focus(
        self,
        handle: &FocusHandle,
    ) -> Div<V, StatefulInteraction<V>, FocusEnabled<V>> {
        Div {
            interaction: self.interaction.into_stateful(handle),
            focus: handle.clone().into(),
            children: self.children,
            group: self.group,
            base_style: self.base_style,
        }
    }
}

impl<V, I> Focusable for Div<V, I, FocusEnabled<V>>
where
    I: ElementInteraction<V>,
    V: 'static + Send + Sync,
{
    fn focus_listeners(&mut self) -> &mut FocusListeners<V> {
        &mut self.focus.focus_listeners
    }

    fn set_focus_style(&mut self, style: StyleRefinement) {
        self.focus.focus_style = style;
    }

    fn set_focus_in_style(&mut self, style: StyleRefinement) {
        self.focus.focus_in_style = style;
    }

    fn set_in_focus_style(&mut self, style: StyleRefinement) {
        self.focus.in_focus_style = style;
    }
}

#[derive(Default)]
pub struct DivState {
    interactive: InteractiveElementState,
    focus_handle: Option<FocusHandle>,
    child_layout_ids: SmallVec<[LayoutId; 4]>,
}

impl<V, I, F> Element for Div<V, I, F>
where
    I: ElementInteraction<V>,
    F: ElementFocus<V>,
    V: 'static + Send + Sync,
{
    type ViewState = V;
    type ElementState = DivState;

    fn id(&self) -> Option<ElementId> {
        self.interaction
            .as_stateful()
            .map(|identified| identified.id.clone())
    }

    fn initialize(
        &mut self,
        view_state: &mut Self::ViewState,
        element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> Self::ElementState {
        let mut element_state = element_state.unwrap_or_default();
        self.focus
            .initialize(element_state.focus_handle.take(), cx, |focus_handle, cx| {
                element_state.focus_handle = focus_handle;
                self.interaction.initialize(cx, |cx| {
                    for child in &mut self.children {
                        child.initialize(view_state, cx);
                    }
                })
            });
        element_state
    }

    fn layout(
        &mut self,
        view_state: &mut Self::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
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
        view_state: &mut Self::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) {
        self.with_element_id(cx, |this, _global_id, cx| {
            if let Some(group) = this.group.clone() {
                GroupBounds::push(group, bounds, cx);
            }

            let style = this.compute_style(bounds, element_state, cx);
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

            cx.stack(z_index, |cx| {
                cx.stack(0, |cx| {
                    style.paint(bounds, cx);
                    this.focus.paint(bounds, cx);
                    this.interaction.paint(
                        bounds,
                        content_size,
                        style.overflow,
                        &mut element_state.interactive,
                        cx,
                    );
                });
                cx.stack(1, |cx| {
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

impl<V, I, F> IntoAnyElement<V> for Div<V, I, F>
where
    I: ElementInteraction<V>,
    F: ElementFocus<V>,
    V: 'static + Send + Sync,
{
    fn into_any(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}

impl<V, I, F> ParentElement for Div<V, I, F>
where
    I: ElementInteraction<V>,
    F: ElementFocus<V>,
    V: 'static + Send + Sync,
{
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<Self::ViewState>; 2]> {
        &mut self.children
    }
}

impl<V, I, F> Styled for Div<V, I, F>
where
    I: ElementInteraction<V>,
    F: ElementFocus<V>,
    V: 'static + Send + Sync,
{
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.base_style
    }
}

impl<V, I, F> StatelessInteractive for Div<V, I, F>
where
    I: ElementInteraction<V>,
    F: ElementFocus<V>,
    V: 'static + Send + Sync,
{
    fn stateless_interaction(&mut self) -> &mut StatelessInteraction<V> {
        self.interaction.as_stateless_mut()
    }
}

impl<V, F> StatefulInteractive for Div<V, StatefulInteraction<V>, F>
where
    F: ElementFocus<V>,
    V: 'static + Send + Sync,
{
    fn stateful_interaction(&mut self) -> &mut StatefulInteraction<Self::ViewState> {
        &mut self.interaction
    }
}
