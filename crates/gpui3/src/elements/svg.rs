use crate::{
    div, AnonymousElement, AnyElement, Bounds, Clickable, Div, DivState, Element, ElementId,
    ElementIdentity, IdentifiedElement, Interactive, IntoAnyElement, LayoutId, MouseEventListeners,
    Pixels, SharedString, StyleRefinement, Styled,
};
use util::ResultExt;

pub struct Svg<V: 'static + Send + Sync, K: ElementIdentity = AnonymousElement> {
    base: Div<V, K>,
    path: Option<SharedString>,
}

pub fn svg<V>() -> Svg<V, AnonymousElement>
where
    V: 'static + Send + Sync,
{
    Svg {
        base: div(),
        path: None,
    }
}

impl<V, K> Svg<V, K>
where
    V: 'static + Send + Sync,
    K: ElementIdentity,
{
    pub fn path(mut self, path: impl Into<SharedString>) -> Self {
        self.path = Some(path.into());
        self
    }
}

impl<V: 'static + Send + Sync> Svg<V, AnonymousElement> {
    pub fn id(self, id: impl Into<ElementId>) -> Svg<V, IdentifiedElement> {
        Svg {
            base: self.base.id(id),
            path: self.path,
        }
    }
}

impl<V, K> IntoAnyElement<V> for Svg<V, K>
where
    V: 'static + Send + Sync,
    K: ElementIdentity,
{
    fn into_any(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}

impl<V, K> Element for Svg<V, K>
where
    V: 'static + Send + Sync,
    K: ElementIdentity,
{
    type ViewState = V;
    type ElementState = DivState;

    fn id(&self) -> Option<crate::ElementId> {
        self.base.id()
    }

    fn layout(
        &mut self,
        view: &mut V,
        element_state: Option<Self::ElementState>,
        cx: &mut crate::ViewContext<V>,
    ) -> (LayoutId, Self::ElementState)
    where
        Self: Sized,
    {
        self.base.layout(view, element_state, cx)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        view: &mut Self::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut crate::ViewContext<V>,
    ) where
        Self: Sized,
    {
        self.base.paint(bounds, view, element_state, cx);
        let fill_color = self
            .base
            .compute_style(bounds, element_state, cx)
            .fill
            .as_ref()
            .and_then(|fill| fill.color());
        if let Some((path, fill_color)) = self.path.as_ref().zip(fill_color) {
            cx.paint_svg(bounds, path.clone(), fill_color).log_err();
        }
    }
}

impl<V, K> Styled for Svg<V, K>
where
    V: 'static + Send + Sync,
    K: ElementIdentity,
{
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<V, K> Interactive for Svg<V, K>
where
    V: 'static + Send + Sync,
    K: ElementIdentity,
{
    fn listeners(&mut self) -> &mut MouseEventListeners<V> {
        self.base.listeners()
    }
}

impl<V> Clickable for Svg<V, IdentifiedElement> where V: 'static + Send + Sync {}
