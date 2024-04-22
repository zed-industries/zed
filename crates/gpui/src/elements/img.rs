use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use crate::{
    point, px, size, AbsoluteLength, Asset, Bounds, DefiniteLength, DevicePixels, Element,
    ElementContext, Hitbox, ImageData, InteractiveElement, Interactivity, IntoElement, LayoutId,
    Length, Pixels, SharedUri, Size, StyleRefinement, Styled, SvgSize, UriOrPath, WindowContext,
};
use futures::{AsyncReadExt, Future};
use image::{ImageBuffer, ImageError};
#[cfg(target_os = "macos")]
use media::core_video::CVImageBuffer;

use thiserror::Error;
use util::{http, ResultExt};

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
    /// The image will be scaled down to fit within the bounds of the element.
    ScaleDown,
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

        let result_bounds = match self {
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
            ObjectFit::ScaleDown => {
                // Check if the image is larger than the bounds in either dimension.
                if image_size.width > bounds.size.width || image_size.height > bounds.size.height {
                    // If the image is larger, use the same logic as Contain to scale it down.
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
                } else {
                    // If the image is smaller than or equal to the container, display it at its original size,
                    // centered within the container.
                    let original_size = size(image_size.width, image_size.height);
                    Bounds {
                        origin: point(
                            bounds.origin.x + (bounds.size.width - original_size.width) / 2.0,
                            bounds.origin.y + (bounds.size.height - original_size.height) / 2.0,
                        ),
                        size: original_size,
                    }
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
        };

        result_bounds
    }
}

impl Img {
    /// A list of all format extensions currently supported by this img element
    pub fn extensions() -> &'static [&'static str] {
        // This is the list in [image::ImageFormat::from_extension] + `svg`
        &[
            "avif", "jpg", "jpeg", "png", "gif", "webp", "tif", "tiff", "tga", "dds", "bmp", "ico",
            "hdr", "exr", "pbm", "pam", "ppm", "pgm", "ff", "farbfeld", "qoi", "svg",
        ]
    }

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
    type BeforePaint = Option<Hitbox>;

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

    fn before_paint(
        &mut self,
        bounds: Bounds<Pixels>,
        _before_layout: &mut Self::BeforeLayout,
        cx: &mut ElementContext,
    ) -> Option<Hitbox> {
        self.interactivity
            .before_paint(bounds, bounds.size, cx, |_, _, hitbox, _| hitbox)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        _: &mut Self::BeforeLayout,
        hitbox: &mut Self::BeforePaint,
        cx: &mut ElementContext,
    ) {
        let source = self.source.clone();
        self.interactivity
            .paint(bounds, hitbox.as_ref(), cx, |style, cx| {
                let corner_radii = style.corner_radii.to_pixels(bounds.size, cx.rem_size());

                if let Some(data) = source.data(cx) {
                    let new_bounds = self.object_fit.get_bounds(bounds, data.size());
                    cx.paint_image(new_bounds, corner_radii, data.clone(), self.grayscale)
                        .log_err();
                }

                match source {
                    #[cfg(target_os = "macos")]
                    ImageSource::Surface(surface) => {
                        let size = size(surface.width().into(), surface.height().into());
                        let new_bounds = self.object_fit.get_bounds(bounds, size);
                        // TODO: Add support for corner_radii and grayscale.
                        cx.paint_surface(new_bounds, surface);
                    }
                    _ => {}
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

impl ImageSource {
    fn data(&self, cx: &mut ElementContext) -> Option<Arc<ImageData>> {
        match self {
            ImageSource::Uri(_) | ImageSource::File(_) => {
                let uri_or_path: UriOrPath = match self {
                    ImageSource::Uri(uri) => uri.clone().into(),
                    ImageSource::File(path) => path.clone().into(),
                    _ => unreachable!(),
                };

                cx.use_cached_asset::<Image>(&uri_or_path)?.log_err()
            }

            ImageSource::Data(data) => Some(data.to_owned()),
            #[cfg(target_os = "macos")]
            ImageSource::Surface(_) => None,
        }
    }
}

#[derive(Clone)]
enum Image {}

impl Asset for Image {
    type Source = UriOrPath;
    type Output = Result<Arc<ImageData>, ImageCacheError>;

    fn load(
        source: Self::Source,
        cx: &mut WindowContext,
    ) -> impl Future<Output = Self::Output> + Send + 'static {
        let client = cx.http_client();
        let scale_factor = cx.scale_factor();
        let svg_renderer = cx.svg_renderer();
        async move {
            let bytes = match source.clone() {
                UriOrPath::Path(uri) => fs::read(uri.as_ref())?,
                UriOrPath::Uri(uri) => {
                    let mut response = client.get(uri.as_ref(), ().into(), true).await?;
                    let mut body = Vec::new();
                    response.body_mut().read_to_end(&mut body).await?;
                    if !response.status().is_success() {
                        return Err(ImageCacheError::BadStatus {
                            status: response.status(),
                            body: String::from_utf8_lossy(&body).into_owned(),
                        });
                    }
                    body
                }
            };

            let data = if let Ok(format) = image::guess_format(&bytes) {
                let data = image::load_from_memory_with_format(&bytes, format)?.into_bgra8();
                ImageData::new(data)
            } else {
                let pixmap =
                    svg_renderer.render_pixmap(&bytes, SvgSize::ScaleFactor(scale_factor))?;

                let buffer =
                    ImageBuffer::from_raw(pixmap.width(), pixmap.height(), pixmap.take()).unwrap();

                ImageData::new(buffer)
            };

            Ok(Arc::new(data))
        }
    }
}

/// An error that can occur when interacting with the image cache.
#[derive(Debug, Error, Clone)]
pub enum ImageCacheError {
    /// An error that occurred while fetching an image from a remote source.
    #[error("http error: {0}")]
    Client(#[from] http::Error),
    /// An error that occurred while reading the image from disk.
    #[error("IO error: {0}")]
    Io(Arc<std::io::Error>),
    /// An error that occurred while processing an image.
    #[error("unexpected http status: {status}, body: {body}")]
    BadStatus {
        /// The HTTP status code.
        status: http::StatusCode,
        /// The HTTP response body.
        body: String,
    },
    /// An error that occurred while processing an image.
    #[error("image error: {0}")]
    Image(Arc<ImageError>),
    /// An error that occurred while processing an SVG.
    #[error("svg error: {0}")]
    Usvg(Arc<usvg::Error>),
}

impl From<std::io::Error> for ImageCacheError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(Arc::new(error))
    }
}

impl From<ImageError> for ImageCacheError {
    fn from(error: ImageError) -> Self {
        Self::Image(Arc::new(error))
    }
}

impl From<usvg::Error> for ImageCacheError {
    fn from(error: usvg::Error) -> Self {
        Self::Usvg(Arc::new(error))
    }
}
