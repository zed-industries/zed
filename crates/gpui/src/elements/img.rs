use crate::{
    px, AbsoluteLength, AnyElement, AppContext, Asset, AssetLogger, Bounds, DefiniteLength,
    Element, ElementId, GlobalElementId, Hitbox, Image, InteractiveElement, Interactivity,
    IntoElement, LayoutId, Length, ObjectFit, Pixels, RenderImage, Resource, SharedString,
    SharedUri, StyleRefinement, Styled, SvgSize, Task, WindowContext,
};
use anyhow::{anyhow, Result};

use futures::{AsyncReadExt, Future};
use image::{
    codecs::gif::GifDecoder, AnimationDecoder, Frame, ImageBuffer, ImageError, ImageFormat,
};
use smallvec::SmallVec;
use std::{
    fs,
    io::{self, Cursor},
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant},
};
use thiserror::Error;
use util::ResultExt;

use super::{FocusableElement, Stateful, StatefulInteractiveElement};

/// The delay before showing the loading state.
pub const LOADING_DELAY: Duration = Duration::from_millis(200);

/// A type alias to the resource loader that the `img()` element uses.
///
/// Note: that this is only for Resources, like URLs or file paths.
/// Custom loaders, or external images will not use this asset loader
pub type ImgResourceLoader = AssetLogger<ImageAssetLoader>;

/// A source of image content.
#[derive(Clone)]
pub enum ImageSource {
    /// The image content will be loaded from some resource location
    Resource(Resource),
    /// Cached image data
    Render(Arc<RenderImage>),
    /// Cached image data
    Image(Arc<Image>),
    /// A custom loading function to use
    Custom(Arc<dyn Fn(&mut WindowContext) -> Option<Result<Arc<RenderImage>, ImageCacheError>>>),
}

fn is_uri(uri: &str) -> bool {
    http_client::Uri::from_str(uri).is_ok()
}

impl From<SharedUri> for ImageSource {
    fn from(value: SharedUri) -> Self {
        Self::Resource(Resource::Uri(value))
    }
}

impl<'a> From<&'a str> for ImageSource {
    fn from(s: &'a str) -> Self {
        if is_uri(s) {
            Self::Resource(Resource::Uri(s.to_string().into()))
        } else {
            Self::Resource(Resource::Embedded(s.to_string().into()))
        }
    }
}

impl From<String> for ImageSource {
    fn from(s: String) -> Self {
        if is_uri(&s) {
            Self::Resource(Resource::Uri(s.into()))
        } else {
            Self::Resource(Resource::Embedded(s.into()))
        }
    }
}

impl From<SharedString> for ImageSource {
    fn from(s: SharedString) -> Self {
        s.as_ref().into()
    }
}

impl From<&Path> for ImageSource {
    fn from(value: &Path) -> Self {
        Self::Resource(value.to_path_buf().into())
    }
}

impl From<Arc<Path>> for ImageSource {
    fn from(value: Arc<Path>) -> Self {
        Self::Resource(value.into())
    }
}

impl From<PathBuf> for ImageSource {
    fn from(value: PathBuf) -> Self {
        Self::Resource(value.into())
    }
}

impl From<Arc<RenderImage>> for ImageSource {
    fn from(value: Arc<RenderImage>) -> Self {
        Self::Render(value)
    }
}

impl From<Arc<Image>> for ImageSource {
    fn from(value: Arc<Image>) -> Self {
        Self::Image(value)
    }
}

impl<F: Fn(&mut WindowContext) -> Option<Result<Arc<RenderImage>, ImageCacheError>> + 'static>
    From<F> for ImageSource
{
    fn from(value: F) -> Self {
        Self::Custom(Arc::new(value))
    }
}

/// The style of an image element.
pub struct ImageStyle {
    grayscale: bool,
    object_fit: ObjectFit,
    loading: Option<Box<dyn Fn() -> AnyElement>>,
    fallback: Option<Box<dyn Fn() -> AnyElement>>,
}

impl Default for ImageStyle {
    fn default() -> Self {
        Self {
            grayscale: false,
            object_fit: ObjectFit::Contain,
            loading: None,
            fallback: None,
        }
    }
}

/// Style an image element.
pub trait StyledImage: Sized {
    /// Get a mutable [ImageStyle] from the element.
    fn image_style(&mut self) -> &mut ImageStyle;

    /// Set the image to be displayed in grayscale.
    fn grayscale(mut self, grayscale: bool) -> Self {
        self.image_style().grayscale = grayscale;
        self
    }

    /// Set the object fit for the image.
    fn object_fit(mut self, object_fit: ObjectFit) -> Self {
        self.image_style().object_fit = object_fit;
        self
    }

    /// Set the object fit for the image.
    fn with_fallback(mut self, fallback: impl Fn() -> AnyElement + 'static) -> Self {
        self.image_style().fallback = Some(Box::new(fallback));
        self
    }

    /// Set the object fit for the image.
    fn with_loading(mut self, loading: impl Fn() -> AnyElement + 'static) -> Self {
        self.image_style().loading = Some(Box::new(loading));
        self
    }
}

impl StyledImage for Img {
    fn image_style(&mut self) -> &mut ImageStyle {
        &mut self.style
    }
}

impl StyledImage for Stateful<Img> {
    fn image_style(&mut self) -> &mut ImageStyle {
        &mut self.element.style
    }
}

/// An image element.
pub struct Img {
    interactivity: Interactivity,
    source: ImageSource,
    style: ImageStyle,
}

/// Create a new image element.
pub fn img(source: impl Into<ImageSource>) -> Img {
    Img {
        interactivity: Interactivity::default(),
        source: source.into(),
        style: ImageStyle::default(),
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
}

impl Deref for Stateful<Img> {
    type Target = Img;

    fn deref(&self) -> &Self::Target {
        &self.element
    }
}

impl DerefMut for Stateful<Img> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.element
    }
}

/// The image state between frames
struct ImgState {
    frame_index: usize,
    last_frame_time: Option<Instant>,
    started_loading: Option<(Instant, Task<()>)>,
}

/// The image layout state between frames
pub struct ImgLayoutState {
    frame_index: usize,
    replacement: Option<AnyElement>,
}

impl Element for Img {
    type RequestLayoutState = ImgLayoutState;
    type PrepaintState = Option<Hitbox>;

    fn id(&self) -> Option<ElementId> {
        self.interactivity.element_id.clone()
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut layout_state = ImgLayoutState {
            frame_index: 0,
            replacement: None,
        };

        cx.with_optional_element_state(global_id, |state, cx| {
            let mut state = state.map(|state| {
                state.unwrap_or(ImgState {
                    frame_index: 0,
                    last_frame_time: None,
                    started_loading: None,
                })
            });

            let frame_index = state.as_ref().map(|state| state.frame_index).unwrap_or(0);

            let layout_id = self
                .interactivity
                .request_layout(global_id, cx, |mut style, cx| {
                    let mut replacement_id = None;

                    match self.source.use_data(cx) {
                        Some(Ok(data)) => {
                            if let Some(state) = &mut state {
                                let frame_count = data.frame_count();
                                if frame_count > 1 {
                                    let current_time = Instant::now();
                                    if let Some(last_frame_time) = state.last_frame_time {
                                        let elapsed = current_time - last_frame_time;
                                        let frame_duration =
                                            Duration::from(data.delay(state.frame_index));

                                        if elapsed >= frame_duration {
                                            state.frame_index =
                                                (state.frame_index + 1) % frame_count;
                                            state.last_frame_time =
                                                Some(current_time - (elapsed - frame_duration));
                                        }
                                    } else {
                                        state.last_frame_time = Some(current_time);
                                    }
                                }
                                state.started_loading = None;
                            }

                            let image_size = data.size(frame_index);

                            if let Length::Auto = style.size.width {
                                style.size.width = match style.size.height {
                                    Length::Definite(DefiniteLength::Absolute(
                                        AbsoluteLength::Pixels(height),
                                    )) => Length::Definite(
                                        px(image_size.width.0 as f32 * height.0
                                            / image_size.height.0 as f32)
                                        .into(),
                                    ),
                                    _ => Length::Definite(px(image_size.width.0 as f32).into()),
                                };
                            }

                            if let Length::Auto = style.size.height {
                                style.size.height = match style.size.width {
                                    Length::Definite(DefiniteLength::Absolute(
                                        AbsoluteLength::Pixels(width),
                                    )) => Length::Definite(
                                        px(image_size.height.0 as f32 * width.0
                                            / image_size.width.0 as f32)
                                        .into(),
                                    ),
                                    _ => Length::Definite(px(image_size.height.0 as f32).into()),
                                };
                            }

                            if global_id.is_some() && data.frame_count() > 1 {
                                cx.request_animation_frame();
                            }
                        }
                        Some(_err) => {
                            if let Some(fallback) = self.style.fallback.as_ref() {
                                let mut element = fallback();
                                replacement_id = Some(element.request_layout(cx));
                                layout_state.replacement = Some(element);
                            }
                            if let Some(state) = &mut state {
                                state.started_loading = None;
                            }
                        }
                        None => {
                            if let Some(state) = &mut state {
                                if let Some((started_loading, _)) = state.started_loading {
                                    if started_loading.elapsed() > LOADING_DELAY {
                                        if let Some(loading) = self.style.loading.as_ref() {
                                            let mut element = loading();
                                            replacement_id = Some(element.request_layout(cx));
                                            layout_state.replacement = Some(element);
                                        }
                                    }
                                } else {
                                    let parent_view_id = cx.parent_view_id();
                                    let task = cx.spawn(|mut cx| async move {
                                        cx.background_executor().timer(LOADING_DELAY).await;
                                        cx.update(|cx| {
                                            cx.notify(parent_view_id);
                                        })
                                        .ok();
                                    });
                                    state.started_loading = Some((Instant::now(), task));
                                }
                            }
                        }
                    }

                    cx.request_layout(style, replacement_id)
                });

            layout_state.frame_index = frame_index;

            ((layout_id, layout_state), state)
        })
    }

    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) -> Self::PrepaintState {
        self.interactivity
            .prepaint(global_id, bounds, bounds.size, cx, |_, _, hitbox, cx| {
                if let Some(replacement) = &mut request_layout.replacement {
                    replacement.prepaint(cx);
                }

                hitbox
            })
    }

    fn paint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        layout_state: &mut Self::RequestLayoutState,
        hitbox: &mut Self::PrepaintState,
        cx: &mut WindowContext,
    ) {
        let source = self.source.clone();
        self.interactivity
            .paint(global_id, bounds, hitbox.as_ref(), cx, |style, cx| {
                let corner_radii = style.corner_radii.to_pixels(bounds.size, cx.rem_size());

                if let Some(Ok(data)) = source.use_data(cx) {
                    let new_bounds = self
                        .style
                        .object_fit
                        .get_bounds(bounds, data.size(layout_state.frame_index));
                    cx.paint_image(
                        new_bounds,
                        corner_radii,
                        data.clone(),
                        layout_state.frame_index,
                        self.style.grayscale,
                    )
                    .log_err();
                } else if let Some(replacement) = &mut layout_state.replacement {
                    replacement.paint(cx);
                }
            })
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

impl IntoElement for Img {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl FocusableElement for Img {}

impl StatefulInteractiveElement for Img {}

impl ImageSource {
    pub(crate) fn use_data(
        &self,
        cx: &mut WindowContext,
    ) -> Option<Result<Arc<RenderImage>, ImageCacheError>> {
        match self {
            ImageSource::Resource(resource) => cx.use_asset::<ImgResourceLoader>(&resource),
            ImageSource::Custom(loading_fn) => loading_fn(cx),
            ImageSource::Render(data) => Some(Ok(data.to_owned())),
            ImageSource::Image(data) => cx.use_asset::<AssetLogger<ImageDecoder>>(data),
        }
    }
}

#[derive(Clone)]
enum ImageDecoder {}

impl Asset for ImageDecoder {
    type Source = Arc<Image>;
    type Output = Result<Arc<RenderImage>, ImageCacheError>;

    fn load(
        source: Self::Source,
        cx: &mut AppContext,
    ) -> impl Future<Output = Self::Output> + Send + 'static {
        let renderer = cx.svg_renderer();
        async move { source.to_image_data(renderer).map_err(Into::into) }
    }
}

/// An image loader for the GPUI asset system
#[derive(Clone)]
pub enum ImageAssetLoader {}

impl Asset for ImageAssetLoader {
    type Source = Resource;
    type Output = Result<Arc<RenderImage>, ImageCacheError>;

    fn load(
        source: Self::Source,
        cx: &mut AppContext,
    ) -> impl Future<Output = Self::Output> + Send + 'static {
        let client = cx.http_client();
        // TODO: Can we make SVGs always rescale?
        // let scale_factor = cx.scale_factor();
        let svg_renderer = cx.svg_renderer();
        let asset_source = cx.asset_source().clone();
        async move {
            let bytes = match source.clone() {
                Resource::Path(uri) => fs::read(uri.as_ref())?,
                Resource::Uri(uri) => {
                    let mut response = client
                        .get(uri.as_ref(), ().into(), true)
                        .await
                        .map_err(|e| anyhow!(e))?;
                    let mut body = Vec::new();
                    response.body_mut().read_to_end(&mut body).await?;
                    if !response.status().is_success() {
                        let mut body = String::from_utf8_lossy(&body).into_owned();
                        let first_line = body.lines().next().unwrap_or("").trim_end();
                        body.truncate(first_line.len());
                        return Err(ImageCacheError::BadStatus {
                            uri,
                            status: response.status(),
                            body,
                        });
                    }
                    body
                }
                Resource::Embedded(path) => {
                    let data = asset_source.load(&path).ok().flatten();
                    if let Some(data) = data {
                        data.to_vec()
                    } else {
                        return Err(ImageCacheError::Asset(
                            format!("Embedded resource not found: {}", path).into(),
                        ));
                    }
                }
            };

            let data = if let Ok(format) = image::guess_format(&bytes) {
                let data = match format {
                    ImageFormat::Gif => {
                        let decoder = GifDecoder::new(Cursor::new(&bytes))?;
                        let mut frames = SmallVec::new();

                        for frame in decoder.into_frames() {
                            let mut frame = frame?;
                            // Convert from RGBA to BGRA.
                            for pixel in frame.buffer_mut().chunks_exact_mut(4) {
                                pixel.swap(0, 2);
                            }
                            frames.push(frame);
                        }

                        frames
                    }
                    _ => {
                        let mut data =
                            image::load_from_memory_with_format(&bytes, format)?.into_rgba8();

                        // Convert from RGBA to BGRA.
                        for pixel in data.chunks_exact_mut(4) {
                            pixel.swap(0, 2);
                        }

                        SmallVec::from_elem(Frame::new(data), 1)
                    }
                };

                RenderImage::new(data)
            } else {
                let pixmap =
                    // TODO: Can we make svgs always rescale?
                    svg_renderer.render_pixmap(&bytes, SvgSize::ScaleFactor(1.0))?;

                let mut buffer =
                    ImageBuffer::from_raw(pixmap.width(), pixmap.height(), pixmap.take()).unwrap();

                // Convert from RGBA to BGRA.
                for pixel in buffer.chunks_exact_mut(4) {
                    pixel.swap(0, 2);
                }

                RenderImage::new(SmallVec::from_elem(Frame::new(buffer), 1))
            };

            Ok(Arc::new(data))
        }
    }
}

/// An error that can occur when interacting with the image cache.
#[derive(Debug, Error, Clone)]
pub enum ImageCacheError {
    /// Some other kind of error occurred
    #[error("error: {0}")]
    Other(#[from] Arc<anyhow::Error>),
    /// An error that occurred while reading the image from disk.
    #[error("IO error: {0}")]
    Io(Arc<std::io::Error>),
    /// An error that occurred while processing an image.
    #[error("unexpected http status for {uri}: {status}, body: {body}")]
    BadStatus {
        /// The URI of the image.
        uri: SharedUri,
        /// The HTTP status code.
        status: http_client::StatusCode,
        /// The HTTP response body.
        body: String,
    },
    /// An error that occurred while processing an asset.
    #[error("asset error: {0}")]
    Asset(SharedString),
    /// An error that occurred while processing an image.
    #[error("image error: {0}")]
    Image(Arc<ImageError>),
    /// An error that occurred while processing an SVG.
    #[error("svg error: {0}")]
    Usvg(Arc<usvg::Error>),
}

impl From<anyhow::Error> for ImageCacheError {
    fn from(value: anyhow::Error) -> Self {
        Self::Other(Arc::new(value))
    }
}

impl From<io::Error> for ImageCacheError {
    fn from(value: io::Error) -> Self {
        Self::Io(Arc::new(value))
    }
}

impl From<usvg::Error> for ImageCacheError {
    fn from(value: usvg::Error) -> Self {
        Self::Usvg(Arc::new(value))
    }
}

impl From<image::ImageError> for ImageCacheError {
    fn from(value: image::ImageError) -> Self {
        Self::Image(Arc::new(value))
    }
}
