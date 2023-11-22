use crate::{
    Bounds, Element, InteractiveElement, InteractiveElementState, Interactivity, IntoElement,
    LayoutId, Pixels, SharedString, StyleRefinement, Styled, WindowContext,
};
use futures::FutureExt;
use util::ResultExt;

pub struct Img {
    interactivity: Interactivity,
    uri: Option<SharedString>,
    grayscale: bool,
}

pub fn img() -> Img {
    Img {
        interactivity: Interactivity::default(),
        uri: None,
        grayscale: false,
    }
}

impl Img {
    pub fn uri(mut self, uri: impl Into<SharedString>) -> Self {
        self.uri = Some(uri.into());
        self
    }

    pub fn grayscale(mut self, grayscale: bool) -> Self {
        self.grayscale = grayscale;
        self
    }
}

impl Element for Img {
    type State = InteractiveElementState;

    fn layout(
        &mut self,
        element_state: Option<Self::State>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::State) {
        self.interactivity.layout(element_state, cx, |style, cx| {
            cx.request_layout(&style, None)
        })
    }

    fn paint(
        self,
        bounds: Bounds<Pixels>,
        element_state: &mut Self::State,
        cx: &mut WindowContext,
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
                        .and_then(|result| result.ok())
                    {
                        let corner_radii = corner_radii.to_pixels(bounds.size, cx.rem_size());
                        cx.with_z_index(1, |cx| {
                            cx.paint_image(bounds, corner_radii, data, self.grayscale)
                                .log_err()
                        });
                    } else {
                        cx.spawn(|mut cx| async move {
                            if image_future.await.ok().is_some() {
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

impl IntoElement for Img {
    type Output = Self;

    fn element_id(&self) -> Option<crate::ElementId> {
        self.interactivity.element_id.clone()
    }

    fn into_element(self) -> Self::Output {
        self
    }
}

impl Styled for Img {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity.base_style
    }
}

impl InteractiveElement for Img {
    fn interactivity(&mut self) -> &mut Interactivity {
        &mut self.interactivity
    }
}
