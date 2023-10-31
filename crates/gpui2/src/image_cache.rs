use crate::{ImageData, ImageId, SharedString};
use collections::HashMap;
use futures::{
    future::{BoxFuture, Shared},
    AsyncReadExt, FutureExt,
};
use image::ImageError;
use parking_lot::Mutex;
use std::sync::Arc;
use thiserror::Error;
use util::http::{self, HttpClient};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct RenderImageParams {
    pub(crate) image_id: ImageId,
}

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("http error: {0}")]
    Client(#[from] http::Error),
    #[error("IO error: {0}")]
    Io(Arc<std::io::Error>),
    #[error("unexpected http status: {status}, body: {body}")]
    BadStatus {
        status: http::StatusCode,
        body: String,
    },
    #[error("image error: {0}")]
    Image(Arc<ImageError>),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(Arc::new(error))
    }
}

impl From<ImageError> for Error {
    fn from(error: ImageError) -> Self {
        Error::Image(Arc::new(error))
    }
}

pub struct ImageCache {
    client: Arc<dyn HttpClient>,
    images: Arc<Mutex<HashMap<SharedString, FetchImageFuture>>>,
}

type FetchImageFuture = Shared<BoxFuture<'static, Result<Arc<ImageData>, Error>>>;

impl ImageCache {
    pub fn new(client: Arc<dyn HttpClient>) -> Self {
        ImageCache {
            client,
            images: Default::default(),
        }
    }

    pub fn get(
        &self,
        uri: impl Into<SharedString>,
    ) -> Shared<BoxFuture<'static, Result<Arc<ImageData>, Error>>> {
        let uri = uri.into();
        let mut images = self.images.lock();

        match images.get(&uri) {
            Some(future) => future.clone(),
            None => {
                let client = self.client.clone();
                let future = {
                    let uri = uri.clone();
                    async move {
                        let mut response = client.get(uri.as_ref(), ().into(), true).await?;
                        let mut body = Vec::new();
                        response.body_mut().read_to_end(&mut body).await?;

                        if !response.status().is_success() {
                            return Err(Error::BadStatus {
                                status: response.status(),
                                body: String::from_utf8_lossy(&body).into_owned(),
                            });
                        }

                        let format = image::guess_format(&body)?;
                        let image =
                            image::load_from_memory_with_format(&body, format)?.into_bgra8();
                        Ok(Arc::new(ImageData::new(image)))
                    }
                }
                .boxed()
                .shared();

                images.insert(uri, future.clone());
                future
            }
        }
    }
}
