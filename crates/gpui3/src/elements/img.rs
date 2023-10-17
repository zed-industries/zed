use crate::{
    AnonymousElementKind, AnyElement, BorrowWindow, Bounds, ClickListeners, Clickable,
    ClickableElement, ClickableElementState, Element, ElementId, ElementKind, Hoverable,
    HoverableElement, IdentifiedElement, IdentifiedElementKind, IntoAnyElement, LayoutId,
    LayoutNodeElement, Pixels, SharedString, Style, StyleRefinement, Styled, ViewContext,
};
use futures::FutureExt;
use refineable::Cascade;
use util::ResultExt;

pub struct Img<V: 'static + Send + Sync, K: ElementKind = AnonymousElementKind> {
    layout_node: ClickableElement<HoverableElement<LayoutNodeElement<V, K>>>,
    uri: Option<SharedString>,
    grayscale: bool,
}

pub fn img<V>() -> Img<V, AnonymousElementKind>
where
    V: 'static + Send + Sync,
{
    Img {
        layout_node: ClickableElement::new(HoverableElement::new(LayoutNodeElement::new())),
        uri: None,
        grayscale: false,
    }
}

impl<V, K> Img<V, K>
where
    V: 'static + Send + Sync,
    K: ElementKind,
{
    pub fn uri(mut self, uri: impl Into<SharedString>) -> Self {
        self.uri = Some(uri.into());
        self
    }

    pub fn grayscale(mut self, grayscale: bool) -> Self {
        self.grayscale = grayscale;
        self
    }
}

impl<V: 'static + Send + Sync> Img<V, AnonymousElementKind> {
    pub fn id(self, id: impl Into<ElementId>) -> Img<V, IdentifiedElementKind> {
        Img {
            layout_node: self.layout_node.replace_child(|hoverable| {
                hoverable.replace_child(|layout_node| layout_node.identify(id))
            }),
            uri: self.uri,
            grayscale: self.grayscale,
        }
    }
}

impl<V, K> IntoAnyElement<V> for Img<V, K>
where
    V: 'static + Send + Sync,
    K: ElementKind,
{
    fn into_any(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}

impl<V, K> Element for Img<V, K>
where
    V: Send + Sync + 'static,
    K: ElementKind,
{
    type ViewState = V;
    type ElementState = ClickableElementState<()>;

    fn id(&self) -> Option<crate::ElementId> {
        self.layout_node.id()
    }

    fn layout(
        &mut self,
        view_state: &mut Self::ViewState,
        element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> (LayoutId, Self::ElementState)
    where
        Self: Sized,
    {
        self.layout_node.layout(view_state, element_state, cx)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        view: &mut Self::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) {
        cx.stack(1, |cx| {
            self.layout_node.paint(bounds, view, element_state, cx);
        });

        let style = self.computed_style();
        let corner_radii = style.corner_radii;

        if let Some(uri) = self.uri.clone() {
            let image_future = cx.image_cache.get(uri);
            if let Some(data) = image_future
                .clone()
                .now_or_never()
                .and_then(ResultExt::log_err)
            {
                let corner_radii = corner_radii.to_pixels(bounds.size, cx.rem_size());
                cx.stack(1, |cx| {
                    cx.paint_image(bounds, corner_radii, data, self.grayscale)
                        .log_err()
                });
            } else {
                cx.spawn(|_, mut cx| async move {
                    if image_future.await.log_err().is_some() {
                        cx.on_next_frame(|cx| cx.notify());
                    }
                })
                .detach()
            }
        }
    }
}

impl<V: 'static + Send + Sync> IdentifiedElement for Img<V, IdentifiedElementKind> {
    fn id(&self) -> ElementId {
        IdentifiedElement::id(&self.layout_node)
    }
}

impl<V, K> Styled for Img<V, K>
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

impl<V: 'static + Send + Sync, K: ElementKind> Hoverable for Img<V, K> {
    fn hover_style(&mut self) -> &mut StyleRefinement {
        self.layout_node.hover_style()
    }
}

impl<V: 'static + Send + Sync> Clickable for Img<V, IdentifiedElementKind> {
    fn active_style(&mut self) -> &mut StyleRefinement {
        self.layout_node.active_style()
    }

    fn listeners(&mut self) -> &mut ClickListeners<V> {
        self.layout_node.listeners()
    }
}
