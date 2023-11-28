use std::sync::Arc;

use crate::{
    size, Bounds, Element, ImageData, InteractiveElement, InteractiveElementState, Interactivity,
    IntoElement, LayoutId, Pixels, SharedString, Size, StyleRefinement, Styled, WindowContext,
};
use futures::FutureExt;
use media::core_video::CVImageBuffer;
use util::ResultExt;

#[derive(Clone, Debug)]
pub enum ImageSource {
    /// Image content will be loaded from provided URI at render time.
    Uri(SharedString),
    Data(Arc<ImageData>),
    Surface(CVImageBuffer),
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

impl From<CVImageBuffer> for ImageSource {
    fn from(value: CVImageBuffer) -> Self {
        Self::Surface(value)
    }
}

pub struct Img {
    interactivity: Interactivity,
    source: ImageSource,
    grayscale: bool,
}

pub fn img(source: impl Into<ImageSource>) -> Img {
    Img {
        interactivity: Interactivity::default(),
        source: source.into(),
        grayscale: false,
    }
}

impl Img {
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
            let image_size = match &self.source {
                ImageSource::Uri(uri) => {
                    let image_future = cx.image_cache.get(uri.clone());
                    if let Some(data) = image_future
                        .clone()
                        .now_or_never()
                        .and_then(|result| result.ok())
                    {
                        data.size().map(|pixels| Pixels::from(u32::from(pixels)))
                    } else {
                        Size::default()
                    }
                }

                ImageSource::Data(data) => {
                    data.size().map(|pixels| Pixels::from(u32::from(pixels)))
                }

                ImageSource::Surface(surface) => {
                    size(surface.width().into(), surface.height().into())
                }
            };
            dbg!(image_size);

            cx.request_measured_layout(
                style,
                cx.rem_size(),
                move |known_dimensions, available_space| match dbg!(
                    known_dimensions.width,
                    known_dimensions.height,
                ) {
                    (None, None) => image_size,

                    (None, Some(height)) => {
                        let aspect_ratio = height / image_size.height;
                        size(image_size.width * aspect_ratio, height)
                    }

                    (Some(width), None) => {
                        let aspect_ratio = width / image_size.width;
                        size(width, image_size.height * aspect_ratio)
                    }

                    (Some(width), Some(height)) => size(width, height),
                },
            )
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
                let corner_radii = style.corner_radii.to_pixels(bounds.size, cx.rem_size());
                cx.with_z_index(1, |cx| {
                    match self.source {
                        ImageSource::Uri(uri) => {
                            let image_future = cx.image_cache.get(uri.clone());
                            if let Some(data) = image_future
                                .clone()
                                .now_or_never()
                                .and_then(|result| result.ok())
                            {
                                cx.paint_image(bounds, corner_radii, data, self.grayscale)
                                    .log_err();
                            } else {
                                cx.spawn(|mut cx| async move {
                                    if image_future.await.ok().is_some() {
                                        cx.on_next_frame(|cx| cx.notify());
                                    }
                                })
                                .detach();
                            }
                        }

                        ImageSource::Data(image) => {
                            cx.paint_image(bounds, corner_radii, image, self.grayscale)
                                .log_err();
                        }

                        ImageSource::Surface(surface) => {
                            // TODO: Add support for corner_radii and grayscale.
                            cx.paint_surface(bounds, surface);
                        }
                    };
                });
            },
        )
    }
}

impl IntoElement for Img {
    type Element = Self;

    fn element_id(&self) -> Option<crate::ElementId> {
        self.interactivity.element_id.clone()
    }

    fn into_element(self) -> Self::Element {
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
