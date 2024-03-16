use std::path::PathBuf;
use std::sync::Arc;

use crate::{
    point, px, size, AbsoluteLength, Bounds, DefiniteLength, DevicePixels, Element, ElementContext,
    Hitbox, ImageData, InteractiveElement, Interactivity, IntoElement, LayoutId, Length, Pixels,
    SharedUri, Size, StyleRefinement, Styled, UriOrPath,
};
use futures::FutureExt;
#[cfg(target_os = "macos")]
use media::core_video::CVImageBuffer;
use util::ResultExt;

/// A source of image content.
#[derive(Clone, Debug)]
pub enum ImageSource {
    /// Image content will be loaded from provided URI at render time.
    Uri(SharedUri),
    /// Image content will be loaded from the provided file at render time.
    File(Arc<PathBuf>),
    /// Cached image data
    Data(Arc<ImageData>),
    // TODO: move surface definitions into mac platform module
    /// A CoreVideo image buffer
    #[cfg(target_os = "macos")]
    Surface(CVImageBuffer),
}

impl From<SharedUri> for ImageSource {
    fn from(value: SharedUri) -> Self {
        Self::Uri(value)
    }
}

impl From<&'static str> for ImageSource {
    fn from(uri: &'static str) -> Self {
        Self::Uri(uri.into())
    }
}

impl From<String> for ImageSource {
    fn from(uri: String) -> Self {
        Self::Uri(uri.into())
    }
}

impl From<Arc<PathBuf>> for ImageSource {
    fn from(value: Arc<PathBuf>) -> Self {
        Self::File(value)
    }
}

impl From<PathBuf> for ImageSource {
    fn from(value: PathBuf) -> Self {
        Self::File(value.into())
    }
}

impl From<Arc<ImageData>> for ImageSource {
    fn from(value: Arc<ImageData>) -> Self {
        Self::Data(value)
    }
}

#[cfg(target_os = "macos")]
impl From<CVImageBuffer> for ImageSource {
    fn from(value: CVImageBuffer) -> Self {
        Self::Surface(value)
    }
}

impl ImageSource {
    fn data(&self, cx: &mut ElementContext) -> Option<Arc<ImageData>> {
        match self {
            ImageSource::Uri(_) | ImageSource::File(_) => {
                let uri_or_path: UriOrPath = match self {
                    ImageSource::Uri(uri) => uri.clone().into(),
                    ImageSource::File(path) => path.clone().into(),
                    _ => unreachable!(),
                };

                let image_future = cx.image_cache.get(uri_or_path.clone(), cx);
                if let Some(data) = image_future
                    .clone()
                    .now_or_never()
                    .and_then(|result| result.ok())
                {
                    return Some(data);
                } else {
                    cx.spawn(|mut cx| async move {
                        if image_future.await.ok().is_some() {
                            cx.on_next_frame(|cx| cx.refresh());
                        }
                    })
                    .detach();

                    return None;
                }
            }

            ImageSource::Data(data) => {
                return Some(data.clone());
            }
            #[cfg(target_os = "macos")]
            ImageSource::Surface(_) => None,
        }
    }
}

/// An image element.
pub struct Img {
    interactivity: Interactivity,
    source: ImageSource,
    grayscale: bool,
    object_fit: ObjectFit,
}

/// Create a new image element.
pub fn img(source: impl Into<ImageSource>) -> Img {
    Img {
        interactivity: Interactivity::default(),
        source: source.into(),
        grayscale: false,
        object_fit: ObjectFit::Contain,
    }
}

/// How to fit the image into the bounds of the element.
pub enum ObjectFit {
    /// The image will be stretched to fill the bounds of the element.
    Fill,
    /// The image will be scaled to fit within the bounds of the element.
    Contain,
    /// The image will be scaled to cover the bounds of the element.
    Cover,
    /// The image will maintain its original size.
    None,
}

impl ObjectFit {
    /// Get the bounds of the image within the given bounds.
    pub fn get_bounds(
        &self,
        bounds: Bounds<Pixels>,
        image_size: Size<DevicePixels>,
    ) -> Bounds<Pixels> {
        let image_size = image_size.map(|dimension| Pixels::from(u32::from(dimension)));
        let image_ratio = image_size.width / image_size.height;
        let bounds_ratio = bounds.size.width / bounds.size.height;

        match self {
            ObjectFit::Fill => bounds,
            ObjectFit::Contain => {
                let new_size = if bounds_ratio > image_ratio {
                    size(
                        image_size.width * (bounds.size.height / image_size.height),
                        bounds.size.height,
                    )
                } else {
                    size(
                        bounds.size.width,
                        image_size.height * (bounds.size.width / image_size.width),
                    )
                };

                Bounds {
                    origin: point(
                        bounds.origin.x + (bounds.size.width - new_size.width) / 2.0,
                        bounds.origin.y + (bounds.size.height - new_size.height) / 2.0,
                    ),
                    size: new_size,
                }
            }
            ObjectFit::Cover => {
                let new_size = if bounds_ratio > image_ratio {
                    size(
                        bounds.size.width,
                        image_size.height * (bounds.size.width / image_size.width),
                    )
                } else {
                    size(
                        image_size.width * (bounds.size.height / image_size.height),
                        bounds.size.height,
                    )
                };

                Bounds {
                    origin: point(
                        bounds.origin.x + (bounds.size.width - new_size.width) / 2.0,
                        bounds.origin.y + (bounds.size.height - new_size.height) / 2.0,
                    ),
                    size: new_size,
                }
            }
            ObjectFit::None => Bounds {
                origin: bounds.origin,
                size: image_size,
            },
        }
    }
}

impl Img {
    /// Set the image to be displayed in grayscale.
    pub fn grayscale(mut self, grayscale: bool) -> Self {
        self.grayscale = grayscale;
        self
    }
    /// Set the object fit for the image.
    pub fn object_fit(mut self, object_fit: ObjectFit) -> Self {
        self.object_fit = object_fit;
        self
    }
}

impl Element for Img {
    type BeforeLayout = ();
    type AfterLayout = Option<Hitbox>;

    fn before_layout(&mut self, cx: &mut ElementContext) -> (LayoutId, Self::BeforeLayout) {
        let layout_id = self.interactivity.before_layout(cx, |mut style, cx| {
            if let Some(data) = self.source.data(cx) {
                let image_size = data.size();
                match (style.size.width, style.size.height) {
                    (Length::Auto, Length::Auto) => {
                        style.size = Size {
                            width: Length::Definite(DefiniteLength::Absolute(
                                AbsoluteLength::Pixels(px(image_size.width.0 as f32)),
                            )),
                            height: Length::Definite(DefiniteLength::Absolute(
                                AbsoluteLength::Pixels(px(image_size.height.0 as f32)),
                            )),
                        }
                    }
                    _ => {}
                }
            }
            cx.request_layout(&style, [])
        });
        (layout_id, ())
    }

    fn after_layout(
        &mut self,
        bounds: Bounds<Pixels>,
        _before_layout: &mut Self::BeforeLayout,
        cx: &mut ElementContext,
    ) -> Option<Hitbox> {
        self.interactivity
            .after_layout(bounds, bounds.size, cx, |_, _, hitbox, _| hitbox)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        _: &mut Self::BeforeLayout,
        hitbox: &mut Self::AfterLayout,
        cx: &mut ElementContext,
    ) {
        let source = self.source.clone();
        self.interactivity
            .paint(bounds, hitbox.as_ref(), cx, |style, cx| {
                let corner_radii = style.corner_radii.to_pixels(bounds.size, cx.rem_size());

                match source.data(cx) {
                    Some(data) => {
                        let bounds = self.object_fit.get_bounds(bounds, data.size());
                        cx.paint_image(bounds, corner_radii, data, self.grayscale)
                            .log_err();
                    }
                    #[cfg(not(target_os = "macos"))]
                    None => {
                        // No renderable image loaded yet. Do nothing.
                    }
                    #[cfg(target_os = "macos")]
                    None => match source {
                        ImageSource::Surface(surface) => {
                            let size = size(surface.width().into(), surface.height().into());
                            let new_bounds = self.object_fit.get_bounds(bounds, size);
                            // TODO: Add support for corner_radii and grayscale.
                            cx.paint_surface(new_bounds, surface);
                        }
                        _ => {
                            // No renderable image loaded yet. Do nothing.
                        }
                    },
                }
            })
    }
}

impl IntoElement for Img {
    type Element = Self;

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
