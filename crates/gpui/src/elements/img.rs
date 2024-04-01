use std::hash::Hasher;
use std::path::PathBuf;
use std::sync::Arc;
use std::{fs, hash::Hash};

use crate::{
    point, px, size, AbsoluteLength, Asset, Bounds, DefiniteLength, DevicePixels, Element,
    ElementContext, GlobalElementId, Hitbox, ImageData, InteractiveElement, Interactivity,
    IntoElement, LayoutId, Length, Pixels, SharedString, SharedUri, Size, StyleRefinement, Styled,
    UriOrPath, WindowContext,
};
use collections::HashMap;
use futures::{AsyncReadExt, Future};
use image::{ImageBuffer, ImageError};
use itertools::Itertools;
#[cfg(target_os = "macos")]
use media::core_video::CVImageBuffer;

use parking_lot::Mutex;
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
    type AfterLayout = Option<Hitbox>;

    fn before_layout(&mut self, cx: &mut ElementContext) -> (LayoutId, Self::BeforeLayout) {
        let layout_id = self.interactivity.before_layout(cx, |mut style, cx| {
            if let Some(data) = self.source.data(None, cx) {
                let image_size = match data {
                    RasterOrVector::Raster(data) => data.size(),
                    RasterOrVector::Vector { data, .. } => size(
                        (data.size().width() as u32).into(),
                        (data.size().height() as u32).into(),
                    ),
                };
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

                if let Some(RasterOrVector::Raster(data)) = source.data(Some(bounds), cx) {
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
    fn data(
        &self,
        bounds: Option<Bounds<Pixels>>,
        cx: &mut ElementContext,
    ) -> Option<RasterOrVector> {
        match self {
            ImageSource::Uri(_) | ImageSource::File(_) => {
                let uri_or_path: UriOrPath = match self {
                    ImageSource::Uri(uri) => uri.clone().into(),
                    ImageSource::File(path) => path.clone().into(),
                    _ => unreachable!(),
                };

                let uri: SharedString = uri_or_path.as_ref().to_string().into();
                match cx.use_cached_asset::<RasterOrVector>(&uri_or_path)? {
                    Ok(RasterOrVector::Raster(data)) => Some(RasterOrVector::Raster(data)),
                    Ok(RasterOrVector::Vector {
                        data,
                        sizes,
                        fallback,
                    }) => {
                        if let Some(bounds) = bounds {
                            let scaled = bounds.scale(cx.scale_factor());
                            let size = size(scaled.size.width.into(), scaled.size.height.into());

                            let mut lock = sizes.lock();

                            let mut id = cx.global_element_id();

                            id.push(uri.into());
                            let mut fallback_lock = fallback.lock();
                            if let Some(cur) = lock.get_mut(&id) {
                                if !size.eq(cur) {
                                    let old = *cur;
                                    *cur = size;

                                    // Remove old cached asset if it's not used anymore
                                    if !lock.values().contains(&old) {
                                        if let Some(Some(f)) =
                                            cx.remove_cached_asset::<Vector>(&VectorKey {
                                                source: uri_or_path.clone(),
                                                size: old,
                                                tree: data.clone(),
                                            })
                                        {
                                            *fallback_lock = Some(f);
                                        };
                                    }
                                }
                            } else {
                                lock.insert(id, size);
                            };

                            let key = VectorKey {
                                source: uri_or_path.clone(),
                                size,
                                tree: data.clone(),
                            };

                            Some(
                                cx.use_cached_asset::<Vector>(&key)
                                    .flatten()
                                    .or(fallback_lock.clone())
                                    .map(|data| RasterOrVector::Raster(data))
                                    .unwrap_or(RasterOrVector::Vector {
                                        data,
                                        sizes: sizes.clone(),
                                        fallback: fallback.clone(),
                                    }),
                            )
                        } else {
                            Some(RasterOrVector::Vector {
                                data,
                                sizes,
                                fallback,
                            })
                        }
                    }
                    Err(_) => None,
                }
            }

            ImageSource::Data(data) => Some(RasterOrVector::Raster(data.to_owned())),
            #[cfg(target_os = "macos")]
            ImageSource::Surface(_) => None,
        }
    }
}

#[derive(Clone)]
struct VectorKey {
    source: UriOrPath,
    size: Size<DevicePixels>,
    tree: Arc<resvg::usvg::Tree>,
}

impl Hash for VectorKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.source.hash(state);
        self.size.hash(state);
    }
}

struct Vector {}
impl Asset for Vector {
    type Source = VectorKey;
    type Output = Option<Arc<ImageData>>;

    fn load(
        source: Self::Source,
        cx: &mut WindowContext,
    ) -> impl Future<Output = Self::Output> + Send + 'static {
        let svg_renderer = cx.svg_renderer();
        async move {
            let Ok(pixmap) = svg_renderer.render_pixmap(&source.tree, source.size) else {
                return None;
            };

            let buffer =
                ImageBuffer::from_raw(pixmap.width(), pixmap.height(), pixmap.take()).unwrap();
            Some(Arc::new(ImageData::new(buffer)))
        }
    }
}

#[derive(Clone)]
enum RasterOrVector {
    Raster(Arc<ImageData>),
    Vector {
        sizes: Arc<Mutex<HashMap<GlobalElementId, Size<DevicePixels>>>>,
        data: Arc<resvg::usvg::Tree>,
        fallback: Arc<Mutex<Option<Arc<ImageData>>>>,
    },
}

impl Asset for RasterOrVector {
    type Source = UriOrPath;
    type Output = Result<Self, ImageCacheError>;

    fn load(
        source: Self::Source,
        cx: &mut WindowContext,
    ) -> impl Future<Output = Self::Output> + Send + 'static {
        let client = cx.http_client();
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

            if let Ok(format) = image::guess_format(&bytes) {
                let data = image::load_from_memory_with_format(&bytes, format)?.into_rgba8();
                return Ok(Self::Raster(Arc::new(ImageData::new(data))));
            } else {
                let tree = svg_renderer.tree(&bytes)?;
                return Ok(Self::Vector {
                    sizes: Default::default(),
                    data: Arc::new(tree),
                    fallback: Default::default(),
                });
            };
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
    Usvg(Arc<resvg::usvg::Error>),
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

impl From<resvg::usvg::Error> for ImageCacheError {
    fn from(error: resvg::usvg::Error) -> Self {
        Self::Usvg(Arc::new(error))
    }
}
