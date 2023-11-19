use crate::{
    BorrowWindow, Bounds, Element, InteractiveElement, InteractiveElementState, Interactivity,
    LayoutId, Pixels, RenderOnce, SharedString, StyleRefinement, Styled, ViewContext,
};
use futures::FutureExt;
use util::ResultExt;

pub struct Img<V: 'static> {
    interactivity: Interactivity<V>,
    uri: Option<SharedString>,
    grayscale: bool,
}

pub fn img<V: 'static>() -> Img<V> {
    Img {
        interactivity: Interactivity::default(),
        uri: None,
        grayscale: false,
    }
}

impl<V> Img<V>
where
    V: 'static,
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

impl<V> Element<V> for Img<V> {
    type State = InteractiveElementState;

    fn element_id(&self) -> Option<crate::ElementId> {
        self.interactivity.element_id.clone()
    }

    fn layout(
        &mut self,
        _view_state: &mut V,
        element_state: Option<Self::State>,
        cx: &mut ViewContext<V>,
    ) -> (LayoutId, Self::State) {
        self.interactivity.layout(element_state, cx, |style, cx| {
            cx.request_layout(&style, None)
        })
    }

    fn paint(
        self,
        bounds: Bounds<Pixels>,
        _view_state: &mut V,
        element_state: &mut Self::State,
        cx: &mut ViewContext<V>,
    ) {
        self.interactivity.paint(
            bounds,
            bounds.size,
            element_state,
            cx,
            |style, _scroll_offset, cx| {
                let corner_radii = style.corner_radii;

                if let Some(uri) = self.uri.clone() {
                    // eprintln!(">>> image_cache.get({uri}");
                    let image_future = cx.image_cache.get(uri.clone());
                    // eprintln!("<<< image_cache.get({uri}");
                    if let Some(data) = image_future
                        .clone()
                        .now_or_never()
                        .and_then(ResultExt::log_err)
                    {
                        let corner_radii = corner_radii.to_pixels(bounds.size, cx.rem_size());
                        cx.with_z_index(1, |cx| {
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
            },
        )
    }
}

impl<V: 'static> RenderOnce<V> for Img<V> {
    type Element = Self;

    fn render_once(self) -> Self::Element {
        self
    }
}

impl<V> Styled for Img<V> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity.base_style
    }
}

impl<V> InteractiveElement<V> for Img<V> {
    fn interactivity(&mut self) -> &mut Interactivity<V> {
        &mut self.interactivity
    }
}
