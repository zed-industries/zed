use crate::{
    AnyElement, BorrowWindow, Bounds, Element, IntoAnyElement, LayoutId, Pixels, SharedString,
    Style, Styled, ViewContext,
};
use futures::FutureExt;
use refineable::Cascade;
use std::marker::PhantomData;
use util::ResultExt;

pub struct Img<S> {
    style: Cascade<Style>,
    uri: Option<SharedString>,
    grayscale: bool,
    state_type: PhantomData<S>,
}

pub fn img<S>() -> Img<S> {
    Img {
        style: Cascade::default(),
        uri: None,
        grayscale: false,
        state_type: PhantomData,
    }
}

impl<S> Img<S> {
    pub fn uri(mut self, uri: impl Into<SharedString>) -> Self {
        self.uri = Some(uri.into());
        self
    }

    pub fn grayscale(mut self, grayscale: bool) -> Self {
        self.grayscale = grayscale;
        self
    }
}

impl<S> IntoAnyElement<S> for Img<S>
where
    S: 'static + Send + Sync,
{
    fn into_any(self) -> AnyElement<S> {
        AnyElement::new(self)
    }
}

impl<S: Send + Sync + 'static> Element for Img<S> {
    type ViewState = S;
    type ElementState = ();

    fn element_id(&self) -> Option<crate::ElementId> {
        None
    }

    fn layout(
        &mut self,
        _: &mut Self::ViewState,
        _: Option<Self::ElementState>,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> (LayoutId, Self::ElementState)
    where
        Self: Sized,
    {
        let style = self.computed_style();
        let layout_id = cx.request_layout(style, []);
        (layout_id, ())
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        _: &mut Self::ViewState,
        _: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) {
        let style = self.computed_style();

        style.paint(bounds, cx);

        if let Some(uri) = self.uri.clone() {
            let image_future = cx.image_cache.get(uri);
            if let Some(data) = image_future
                .clone()
                .now_or_never()
                .and_then(ResultExt::log_err)
            {
                let corner_radii = style.corner_radii.to_pixels(bounds.size, cx.rem_size());
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

impl<S> Styled for Img<S> {
    type Style = Style;

    fn style_cascade(&mut self) -> &mut Cascade<Self::Style> {
        &mut self.style
    }

    fn declared_style(&mut self) -> &mut <Self::Style as refineable::Refineable>::Refinement {
        self.style.base()
    }
}
