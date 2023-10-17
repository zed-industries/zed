use crate::{
    AnonymousElementKind, AnyElement, Bounds, ClickListeners, Clickable, ClickableElement,
    ClickableElementState, Element, ElementId, ElementKind, Hoverable, HoverableElement,
    IdentifiedElement, IdentifiedElementKind, IntoAnyElement, LayoutId, LayoutNodeElement, Pixels,
    SharedString, Style, StyleRefinement, Styled,
};
use refineable::Cascade;
use util::ResultExt;

pub struct Svg<V: 'static + Send + Sync, K: ElementKind = AnonymousElementKind> {
    layout_node: ClickableElement<HoverableElement<LayoutNodeElement<V, K>>>,
    path: Option<SharedString>,
}

pub fn svg<V>() -> Svg<V, AnonymousElementKind>
where
    V: 'static + Send + Sync,
{
    Svg {
        layout_node: ClickableElement::new(HoverableElement::new(LayoutNodeElement::new())),
        path: None,
    }
}

impl<V, K> Svg<V, K>
where
    V: 'static + Send + Sync,
    K: ElementKind,
{
    pub fn path(mut self, path: impl Into<SharedString>) -> Self {
        self.path = Some(path.into());
        self
    }
}

impl<V: 'static + Send + Sync> Svg<V, AnonymousElementKind> {
    pub fn id(self, id: impl Into<ElementId>) -> Svg<V, IdentifiedElementKind> {
        Svg {
            layout_node: self.layout_node.replace_child(|hoverable| {
                hoverable.replace_child(|layout_node| layout_node.identify(id))
            }),
            path: self.path,
        }
    }
}

impl<V, K> IntoAnyElement<V> for Svg<V, K>
where
    V: 'static + Send + Sync,
    K: ElementKind,
{
    fn into_any(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}

impl<V, K> Element for Svg<V, K>
where
    V: 'static + Send + Sync,
    K: ElementKind,
{
    type ViewState = V;
    type ElementState = ClickableElementState<()>;

    fn id(&self) -> Option<crate::ElementId> {
        self.layout_node.id()
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
        self.layout_node.layout(view, element_state, cx)
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
        self.layout_node.paint(bounds, view, element_state, cx);
        let fill_color = self
            .computed_style()
            .fill
            .as_ref()
            .and_then(|fill| fill.color());
        if let Some((path, fill_color)) = self.path.as_ref().zip(fill_color) {
            cx.paint_svg(bounds, path.clone(), fill_color).log_err();
        }
    }
}

impl<V: 'static + Send + Sync> IdentifiedElement for Svg<V, IdentifiedElementKind> {
    fn id(&self) -> ElementId {
        IdentifiedElement::id(&self.layout_node)
    }
}

impl<V, K> Styled for Svg<V, K>
where
    V: 'static + Send + Sync,
    K: ElementKind,
{
    fn style_cascade(&mut self) -> &mut Cascade<Style> {
        self.layout_node.style_cascade()
    }

    fn computed_style(&mut self) -> &Style {
        self.layout_node.computed_style()
    }
}

impl<V: 'static + Send + Sync, K: ElementKind> Hoverable for Svg<V, K> {
    fn hover_style(&mut self) -> &mut StyleRefinement {
        self.layout_node.hover_style()
    }
}

impl<V: 'static + Send + Sync> Clickable for Svg<V, IdentifiedElementKind> {
    fn active_style(&mut self) -> &mut StyleRefinement {
        self.layout_node.active_style()
    }

    fn listeners(&mut self) -> &mut ClickListeners<V> {
        self.layout_node.listeners()
    }
}
