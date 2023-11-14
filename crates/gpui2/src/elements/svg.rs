use crate::{
    div, AnyElement, Bounds, Component, Div, DivState, Element, ElementId, ElementInteractivity,
    FocusListeners, Focusable, FocusableKeyDispatch, KeyDispatch, LayoutId,
    NonFocusableKeyDispatch, Pixels, SharedString, StatefulInteractive, StatefulInteractivity,
    StatelessInteractive, StatelessInteractivity, StyleRefinement, Styled, ViewContext,
};
use util::ResultExt;

pub struct Svg<
    V: 'static,
    I: ElementInteractivity<V> = StatelessInteractivity<V>,
    F: KeyDispatch<V> = NonFocusableKeyDispatch,
> {
    base: Div<V, I, F>,
    path: Option<SharedString>,
}

pub fn svg<V: 'static>() -> Svg<V, StatelessInteractivity<V>, NonFocusableKeyDispatch> {
    Svg {
        base: div(),
        path: None,
    }
}

impl<V, I, F> Svg<V, I, F>
where
    I: ElementInteractivity<V>,
    F: KeyDispatch<V>,
{
    pub fn path(mut self, path: impl Into<SharedString>) -> Self {
        self.path = Some(path.into());
        self
    }
}

impl<V, F> Svg<V, StatelessInteractivity<V>, F>
where
    F: KeyDispatch<V>,
{
    pub fn id(self, id: impl Into<ElementId>) -> Svg<V, StatefulInteractivity<V>, F> {
        Svg {
            base: self.base.id(id),
            path: self.path,
        }
    }
}

impl<V, I, F> Component<V> for Svg<V, I, F>
where
    I: ElementInteractivity<V>,
    F: KeyDispatch<V>,
{
    fn render(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}

impl<V, I, F> Element<V> for Svg<V, I, F>
where
    I: ElementInteractivity<V>,
    F: KeyDispatch<V>,
{
    type ElementState = DivState;

    fn id(&self) -> Option<crate::ElementId> {
        self.base.id()
    }

    fn initialize(
        &mut self,
        view_state: &mut V,
        element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<V>,
    ) -> Self::ElementState {
        self.base.initialize(view_state, element_state, cx)
    }

    fn layout(
        &mut self,
        view_state: &mut V,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    ) -> LayoutId {
        self.base.layout(view_state, element_state, cx)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        view: &mut V,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    ) where
        Self: Sized,
    {
        self.base.paint(bounds, view, element_state, cx);
        let color = self
            .base
            .compute_style(bounds, element_state, cx)
            .text
            .color;
        if let Some((path, color)) = self.path.as_ref().zip(color) {
            cx.paint_svg(bounds, path.clone(), color).log_err();
        }
    }
}

impl<V, I, F> Styled for Svg<V, I, F>
where
    I: ElementInteractivity<V>,
    F: KeyDispatch<V>,
{
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<V, I, F> StatelessInteractive<V> for Svg<V, I, F>
where
    I: ElementInteractivity<V>,
    F: KeyDispatch<V>,
{
    fn stateless_interactivity(&mut self) -> &mut StatelessInteractivity<V> {
        self.base.stateless_interactivity()
    }
}

impl<V, F> StatefulInteractive<V> for Svg<V, StatefulInteractivity<V>, F>
where
    V: 'static,
    F: KeyDispatch<V>,
{
    fn stateful_interactivity(&mut self) -> &mut StatefulInteractivity<V> {
        self.base.stateful_interactivity()
    }
}

impl<V: 'static, I> Focusable<V> for Svg<V, I, FocusableKeyDispatch<V>>
where
    I: ElementInteractivity<V>,
{
    fn focus_listeners(&mut self) -> &mut FocusListeners<V> {
        self.base.focus_listeners()
    }

    fn set_focus_style(&mut self, style: StyleRefinement) {
        self.base.set_focus_style(style)
    }

    fn set_focus_in_style(&mut self, style: StyleRefinement) {
        self.base.set_focus_in_style(style)
    }

    fn set_in_focus_style(&mut self, style: StyleRefinement) {
        self.base.set_in_focus_style(style)
    }
}
