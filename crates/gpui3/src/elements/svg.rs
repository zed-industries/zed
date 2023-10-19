use crate::{
    div, Active, AnyElement, Bounds, Div, Element, ElementFocusability, ElementId,
    ElementInteractivity, Focus, FocusListeners, Focusable, Hover, InteractiveElementState,
    IntoAnyElement, LayoutId, NonFocusable, Pixels, SharedString, StatefulInteractivity,
    StatefullyInteractive, StatelessInteractivity, StatelesslyInteractive, StyleRefinement, Styled,
    ViewContext,
};
use util::ResultExt;

pub struct Svg<
    V: 'static + Send + Sync,
    I: ElementInteractivity<V> = StatelessInteractivity<V>,
    F: ElementFocusability<V> = NonFocusable,
> {
    base: Div<V, I, F>,
    path: Option<SharedString>,
}

pub fn svg<V>() -> Svg<V, StatelessInteractivity<V>, NonFocusable>
where
    V: 'static + Send + Sync,
{
    Svg {
        base: div(),
        path: None,
    }
}

impl<V, I, F> Svg<V, I, F>
where
    V: 'static + Send + Sync,
    I: ElementInteractivity<V>,
    F: ElementFocusability<V>,
{
    pub fn path(mut self, path: impl Into<SharedString>) -> Self {
        self.path = Some(path.into());
        self
    }
}

impl<V, F> Svg<V, StatelessInteractivity<V>, F>
where
    V: 'static + Send + Sync,
    F: ElementFocusability<V>,
{
    pub fn id(self, id: impl Into<ElementId>) -> Svg<V, StatefulInteractivity<V>, F> {
        Svg {
            base: self.base.id(id),
            path: self.path,
        }
    }
}

impl<V, I, F> IntoAnyElement<V> for Svg<V, I, F>
where
    V: 'static + Send + Sync,
    I: ElementInteractivity<V>,
    F: ElementFocusability<V>,
{
    fn into_any(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}

impl<V, I, F> Element for Svg<V, I, F>
where
    V: 'static + Send + Sync,
    I: ElementInteractivity<V>,
    F: ElementFocusability<V>,
{
    type ViewState = V;
    type ElementState = InteractiveElementState;

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
        cx: &mut ViewContext<Self::ViewState>,
    ) -> LayoutId {
        self.base.layout(view_state, element_state, cx)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        view: &mut Self::ViewState,
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
    V: 'static + Send + Sync,
    I: ElementInteractivity<V>,
    F: ElementFocusability<V>,
{
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<V, I, F> StatelesslyInteractive for Svg<V, I, F>
where
    V: 'static + Send + Sync,
    I: ElementInteractivity<V>,
    F: ElementFocusability<V>,
{
    fn stateless_interactivity(&mut self) -> &mut StatelessInteractivity<V> {
        self.base.stateless_interactivity()
    }
}

impl<V, I, F> Hover for Svg<V, I, F>
where
    V: 'static + Send + Sync,
    I: ElementInteractivity<V>,
    F: ElementFocusability<V>,
{
    fn set_hover_style(&mut self, group: Option<SharedString>, style: StyleRefinement) {
        self.base.set_hover_style(group, style);
    }
}

impl<V, F> StatefullyInteractive for Svg<V, StatefulInteractivity<V>, F>
where
    V: 'static + Send + Sync,
    F: ElementFocusability<V>,
{
    fn stateful_interactivity(&mut self) -> &mut StatefulInteractivity<Self::ViewState> {
        self.base.stateful_interactivity()
    }
}

impl<V, F> Active for Svg<V, StatefulInteractivity<V>, F>
where
    V: 'static + Send + Sync,
    F: ElementFocusability<V>,
{
    fn set_active_style(&mut self, group: Option<SharedString>, style: StyleRefinement) {
        self.base.set_active_style(group, style)
    }
}

impl<V, I> Focus for Svg<V, I, Focusable<V>>
where
    V: 'static + Send + Sync,
    I: ElementInteractivity<V>,
{
    fn focus_listeners(&mut self) -> &mut FocusListeners<Self::ViewState> {
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

    fn handle(&self) -> &crate::FocusHandle {
        self.base.handle()
    }
}
