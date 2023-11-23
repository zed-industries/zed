use std::sync::Arc;

use crate::{
    Bounds, Element, ImageData, InteractiveElement, InteractiveElementState, Interactivity,
    LayoutId, Pixels, RenderOnce, SharedString, StyleRefinement, Styled, WindowContext,
};
use futures::FutureExt;
use util::ResultExt;

#[derive(Clone, Debug)]
pub enum ImageSource {
    /// Image content will be loaded from provided URI at render time.
    Uri(SharedString),
    Data(Arc<ImageData>),
}

impl From<SharedString> for ImageSource {
    fn from(value: SharedString) -> Self {
        Self::Uri(value)
    }
}

impl From<Arc<ImageData>> for ImageSource {
    fn from(value: Arc<ImageData>) -> Self {
        Self::Data(value)
    }
}

pub struct Img {
    interactivity: Interactivity,
    source: Option<ImageSource>,
    grayscale: bool,
}

pub fn img() -> Img {
    Img {
        interactivity: Interactivity::default(),
        source: None,
        grayscale: false,
    }
}

impl Img {
    pub fn uri(mut self, uri: impl Into<SharedString>) -> Self {
        self.source = Some(ImageSource::from(uri.into()));
        self
    }
    pub fn data(mut self, data: Arc<ImageData>) -> Self {
        self.source = Some(ImageSource::from(data));
        self
    }

    pub fn source(mut self, source: impl Into<ImageSource>) -> Self {
        self.source = Some(source.into());
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

                if let Some(source) = self.source {
                    let image = match source {
                        ImageSource::Uri(uri) => {
                            let image_future = cx.image_cache.get(uri.clone());
                            if let Some(data) = image_future
                                .clone()
                                .now_or_never()
                                .and_then(|result| result.ok())
                            {
                                data
                            } else {
                                cx.spawn(|mut cx| async move {
                                    if image_future.await.ok().is_some() {
                                        cx.on_next_frame(|cx| cx.notify());
                                    }
                                })
                                .detach();
                                return;
                            }
                        }
                        ImageSource::Data(image) => image,
                    };
                    let corner_radii = corner_radii.to_pixels(bounds.size, cx.rem_size());
                    cx.with_z_index(1, |cx| {
                        cx.paint_image(bounds, corner_radii, image, self.grayscale)
                            .log_err()
                    });
                }
            },
        )
    }
}

impl RenderOnce for Img {
    type Element = Self;

    fn element_id(&self) -> Option<crate::ElementId> {
        self.interactivity.element_id.clone()
    }

    fn render_once(self) -> Self::Element {
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
