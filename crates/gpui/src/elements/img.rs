use crate::{
    px, AbsoluteLength, AppContext, Asset, Bounds, DefiniteLength, Element, ElementId,
    GlobalElementId, Hitbox, Image, InteractiveElement, Interactivity, IntoElement, LayoutId,
    Length, ObjectFit, Pixels, RenderImage, SharedString, SharedUri, Size, StyleRefinement, Styled,
    SvgSize, UriOrPath, WindowContext,
};
use futures::{AsyncReadExt, Future};
use http_client;
use image::{
    codecs::gif::GifDecoder, AnimationDecoder, Frame, ImageBuffer, ImageError, ImageFormat,
};
use smallvec::SmallVec;
use std::{
    fs,
    io::Cursor,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};
use thiserror::Error;
use util::ResultExt;

/// A source of image content.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImageSource {
    /// Image content will be loaded from provided URI at render time.
    Uri(SharedUri),
    /// Image content will be loaded from the provided file at render time.
    File(Arc<PathBuf>),
    /// Cached image data
    Render(Arc<RenderImage>),
    /// Cached image data
    Image(Arc<Image>),
    /// Image content will be loaded from Asset at render time.
    Embedded(SharedString),
}

fn is_uri(uri: &str) -> bool {
    uri.contains("://")
}

impl From<SharedUri> for ImageSource {
    fn from(value: SharedUri) -> Self {
        Self::Uri(value)
    }
}

impl From<&'static str> for ImageSource {
    fn from(s: &'static str) -> Self {
        if is_uri(&s) {
            Self::Uri(s.into())
        } else {
            Self::Embedded(s.into())
        }
    }
}

impl From<String> for ImageSource {
    fn from(s: String) -> Self {
        if is_uri(&s) {
            Self::Uri(s.into())
        } else {
            Self::Embedded(s.into())
        }
    }
}

impl From<SharedString> for ImageSource {
    fn from(s: SharedString) -> Self {
        if is_uri(&s) {
            Self::Uri(s.into())
        } else {
            Self::Embedded(s)
        }
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

impl From<Arc<RenderImage>> for ImageSource {
    fn from(value: Arc<RenderImage>) -> Self {
        Self::Render(value)
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

/// The image state between frames
struct ImgState {
    frame_index: usize,
    last_frame_time: Option<Instant>,
}

impl Element for Img {
    type RequestLayoutState = usize;
    type PrepaintState = Option<Hitbox>;

    fn id(&self) -> Option<ElementId> {
        self.interactivity.element_id.clone()
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::RequestLayoutState) {
        cx.with_optional_element_state(global_id, |state, cx| {
            let mut state = state.map(|state| {
                state.unwrap_or(ImgState {
                    frame_index: 0,
                    last_frame_time: None,
                })
            });

            let frame_index = state.as_ref().map(|state| state.frame_index).unwrap_or(0);

            let layout_id = self
                .interactivity
                .request_layout(global_id, cx, |mut style, cx| {
                    if let Some(data) = self.source.use_data(cx) {
                        if let Some(state) = &mut state {
                            let frame_count = data.frame_count();
                            if frame_count > 1 {
                                let current_time = Instant::now();
                                if let Some(last_frame_time) = state.last_frame_time {
                                    let elapsed = current_time - last_frame_time;
                                    let frame_duration =
                                        Duration::from(data.delay(state.frame_index));

                                    if elapsed >= frame_duration {
                                        state.frame_index = (state.frame_index + 1) % frame_count;
                                        state.last_frame_time =
                                            Some(current_time - (elapsed - frame_duration));
                                    }
                                } else {
                                    state.last_frame_time = Some(current_time);
                                }
                            }
                        }

                        let image_size = data.size(frame_index);
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

                        if global_id.is_some() && data.frame_count() > 1 {
                            cx.request_animation_frame();
                        }
                    }

                    cx.request_layout(style, [])
                });

            ((layout_id, frame_index), state)
        })
    }

    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) -> Option<Hitbox> {
        self.interactivity
            .prepaint(global_id, bounds, bounds.size, cx, |_, _, hitbox, _| hitbox)
    }

    fn paint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        frame_index: &mut Self::RequestLayoutState,
        hitbox: &mut Self::PrepaintState,
        cx: &mut WindowContext,
    ) {
        let source = self.source.clone();
        self.interactivity
            .paint(global_id, bounds, hitbox.as_ref(), cx, |style, cx| {
                let corner_radii = style.corner_radii.to_pixels(bounds.size, cx.rem_size());

                if let Some(data) = source.use_data(cx) {
                    let new_bounds = self.object_fit.get_bounds(bounds, data.size(*frame_index));
                    cx.paint_image(
                        new_bounds,
                        corner_radii,
                        data.clone(),
                        *frame_index,
                        self.grayscale,
                    )
                    .log_err();
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
    pub(crate) fn use_data(&self, cx: &mut WindowContext) -> Option<Arc<RenderImage>> {
        match self {
            ImageSource::Uri(_) | ImageSource::Embedded(_) | ImageSource::File(_) => {
                let uri_or_path: UriOrPath = match self {
                    ImageSource::Uri(uri) => uri.clone().into(),
                    ImageSource::File(path) => path.clone().into(),
                    ImageSource::Embedded(path) => UriOrPath::Embedded(path.clone()),
                    _ => unreachable!(),
                };

                cx.use_asset::<ImageAsset>(&uri_or_path)?.log_err()
            }

            ImageSource::Render(data) => Some(data.to_owned()),
            ImageSource::Image(data) => cx.use_asset::<ImageDecoder>(data)?.log_err(),
        }
    }

    /// Fetch the data associated with this source, using GPUI's asset caching
    pub async fn data(&self, cx: &mut AppContext) -> Option<Arc<RenderImage>> {
        match self {
            ImageSource::Uri(_) | ImageSource::Embedded(_) | ImageSource::File(_) => {
                let uri_or_path: UriOrPath = match self {
                    ImageSource::Uri(uri) => uri.clone().into(),
                    ImageSource::File(path) => path.clone().into(),
                    ImageSource::Embedded(path) => UriOrPath::Embedded(path.clone()),
                    _ => unreachable!(),
                };

                cx.fetch_asset::<ImageAsset>(&uri_or_path).0.await.log_err()
            }

            ImageSource::Render(data) => Some(data.to_owned()),
            ImageSource::Image(data) => cx.fetch_asset::<ImageDecoder>(data).0.await.log_err(),
        }
    }
}

#[derive(Clone)]
enum ImageDecoder {}

impl Asset for ImageDecoder {
    type Source = Arc<Image>;
    type Output = Result<Arc<RenderImage>, Arc<anyhow::Error>>;

    fn load(
        source: Self::Source,
        cx: &mut AppContext,
    ) -> impl Future<Output = Self::Output> + Send + 'static {
        let result = source.to_image_data(cx).map_err(Arc::new);
        async { result }
    }
}

#[derive(Clone)]
enum ImageAsset {}

impl Asset for ImageAsset {
    type Source = UriOrPath;
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
                UriOrPath::Path(uri) => fs::read(uri.as_ref())?,
                UriOrPath::Uri(uri) => {
                    let mut response = client.get(uri.as_ref(), ().into(), true).await?;
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
                UriOrPath::Embedded(path) => {
                    let data = asset_source.load(&path).ok().flatten();
                    if let Some(data) = data {
                        data.to_vec()
                    } else {
                        return Err(ImageCacheError::Asset(
                            format!("not found: {}", path).into(),
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

                let buffer =
                    ImageBuffer::from_raw(pixmap.width(), pixmap.height(), pixmap.take()).unwrap();

                RenderImage::new(SmallVec::from_elem(Frame::new(buffer), 1))
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
    Client(#[from] http_client::Error),
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
