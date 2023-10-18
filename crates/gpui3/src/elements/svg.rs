use crate::{
    div, Active, Anonymous, AnyElement, Bounds, Click, Div, DivState, Element, ElementId,
    ElementIdentity, EventListeners, Hover, Identified, Interactive, IntoAnyElement, LayoutId,
    NonFocusable, Pixels, SharedString, StyleRefinement, Styled,
};
use util::ResultExt;

pub struct Svg<V: 'static + Send + Sync, K: ElementIdentity = Anonymous> {
    base: Div<K, NonFocusable, V>,
    path: Option<SharedString>,
}

pub fn svg<V>() -> Svg<V, Anonymous>
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

impl<V: 'static + Send + Sync> Svg<V, Anonymous> {
    pub fn id(self, id: impl Into<ElementId>) -> Svg<V, Identified> {
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
    fn listeners(&mut self) -> &mut EventListeners<V> {
        self.base.listeners()
    }
}

impl<V, K> Hover for Svg<V, K>
where
    V: 'static + Send + Sync,
    K: ElementIdentity,
{
    fn set_hover_style(&mut self, group: Option<SharedString>, style: StyleRefinement) {
        self.base.set_hover_style(group, style);
    }
}

impl<V> Click for Svg<V, Identified> where V: 'static + Send + Sync {}

impl<V> Active for Svg<V, Identified>
where
    V: 'static + Send + Sync,
{
    fn set_active_style(&mut self, group: Option<SharedString>, style: StyleRefinement) {
        self.base.set_active_style(group, style)
    }
}
