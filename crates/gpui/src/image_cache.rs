use crate::{AppContext, ImageData, ImageId, SharedUri, Task};
use collections::HashMap;
use futures::{future::Shared, AsyncReadExt, FutureExt, TryFutureExt};
use image::ImageError;
use parking_lot::Mutex;
use std::sync::{Arc, OnceLock};
use std::{fs, path::PathBuf};
use thiserror::Error;
use util::http::{self, HttpClient};

pub use image::ImageFormat;

#[derive(PartialEq, Eq, Hash, Clone)]
pub(crate) struct RenderImageParams {
    pub(crate) image_id: ImageId,
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

pub(crate) struct ImageCache {
    client: Arc<dyn HttpClient>,
    images: Arc<Mutex<HashMap<UriOrPath, FetchImageTask>>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) enum UriOrPath {
    Uri(SharedUri),
    Path(Arc<PathBuf>),
}

impl From<SharedUri> for UriOrPath {
    fn from(value: SharedUri) -> Self {
        Self::Uri(value)
    }
}

impl From<Arc<PathBuf>> for UriOrPath {
    fn from(value: Arc<PathBuf>) -> Self {
        Self::Path(value)
    }
}

/// A task to complete fetching an image.
pub type FetchImageTask = Shared<Task<Result<Arc<ImageData>, ImageCacheError>>>;

impl ImageCache {
    pub fn new(client: Arc<dyn HttpClient>) -> Self {
        ImageCache {
            client,
            images: Default::default(),
        }
    }

    pub fn get(&self, uri_or_path: impl Into<UriOrPath>, cx: &AppContext) -> FetchImageTask {
        let uri_or_path = uri_or_path.into();
        let mut images = self.images.lock();

        match images.get(&uri_or_path) {
            Some(future) => future.clone(),
            None => {
                let client = self.client.clone();
                let future = cx
                    .background_executor()
                    .spawn(
                        {
                            let uri_or_path = uri_or_path.clone();
                            async move {
                                let body = match uri_or_path {
                                    UriOrPath::Path(uri) => fs::read(uri.as_ref())?,
                                    UriOrPath::Uri(uri) => {
                                        let mut response =
                                            client.get(uri.as_ref(), ().into(), true).await?;
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
                                Ok(Arc::new(ImageData::try_from_bytes(&body)?))
                            }
                        }
                        .map_err({
                            let uri_or_path = uri_or_path.clone();
                            move |error| {
                                log::log!(log::Level::Error, "{:?} {:?}", &uri_or_path, &error);
                                error
                            }
                        }),
                    )
                    .shared();

                images.insert(uri_or_path, future.clone());
                future
            }
        }
    }
}

/// Returns the global SVG font database.
pub fn svg_fontdb() -> &'static resvg::usvg::fontdb::Database {
    static FONTDB: OnceLock<resvg::usvg::fontdb::Database> = OnceLock::new();
    FONTDB.get_or_init(|| {
        let mut fontdb = resvg::usvg::fontdb::Database::new();
        fontdb.load_system_fonts();
        fontdb
    })
}
