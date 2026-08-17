pub mod mem;

use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{ready, Poll},
};

use bytes::Bytes;
use http::{header, request::Request, HeaderMap, HeaderValue, Response, StatusCode, Uri};
use http_body::{Body, Frame, SizeHint};
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use pin_project::pin_project;
use tower::{Layer, Service};

// Implementation based on the documentation at:
// https://docs.github.com/en/rest/using-the-rest-api/best-practices-for-using-the-rest-api?apiVersion=2022-11-28#use-conditional-requests-if-appropriate

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
/// Cache key identification returned by the GitHub API.
pub enum CacheKey {
    ETag(String),
    LastModified(String),
}

#[derive(Debug, Clone, Default)]
/// Cache entry containing the response data as well as response headers.
pub struct CachedResponse {
    pub body: Vec<u8>,
    pub headers: HeaderMap,
}

/// [HttpCacheLayer] is agnostic to the storage implementation (e.g., in-memory,
/// filesystem, etc.). This trait represents the required interface.
pub trait CacheStorage: Send + Sync {
    /// Returns the stored cache key for given URI if it's available in the
    /// cache.
    fn try_hit(&self, uri: &Uri) -> Option<CacheKey>;

    /// Returns the cached response for given URI if it's available in the
    /// cache.
    ///
    /// **This method is expected to return `Some` if [CacheStorage::try_hit]
    /// returned `Some` for given URI.**
    fn load(&self, uri: &Uri) -> Option<CachedResponse>;

    /// Returns a writer that writes the response body to the cache.
    fn writer(&self, uri: &Uri, key: CacheKey, headers: HeaderMap) -> Box<dyn CacheWriter>;
}

/// Writes the response body to the cache.
pub trait CacheWriter: Send + Sync {
    fn write_body(&mut self, data: &[u8]);
}

#[derive(Clone)]
/// Layer that handles response caching using given [CacheStorage].
pub struct HttpCacheLayer {
    storage: Option<Arc<dyn CacheStorage>>,
}

impl HttpCacheLayer {
    pub fn new(storage: Option<Arc<dyn CacheStorage>>) -> Self {
        HttpCacheLayer { storage }
    }
}

impl<S> Layer<S> for HttpCacheLayer {
    type Service = HttpCache<S>;

    fn layer(&self, inner: S) -> Self::Service {
        HttpCache {
            inner,
            storage: self.storage.clone(),
        }
    }
}

pub struct HttpCache<S> {
    inner: S,
    storage: Option<Arc<dyn CacheStorage>>,
}

type ResBody = BoxBody<Bytes, crate::Error>;

impl<S, ReqBody> Service<Request<ReqBody>> for HttpCache<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
{
    type Error = S::Error;
    type Response = S::Response;
    type Future = HttpCacheFuture<S::Future>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        let uri = req.uri().clone();

        if let Some(ref storage) = self.storage {
            // If there is a cache record for this URI, add the corresponding
            // header so that GitHub API might send the unmodified response.
            if let Some(key) = storage.try_hit(&uri) {
                match key {
                    CacheKey::ETag(etag) => {
                        req.headers_mut()
                            .append(header::IF_NONE_MATCH, HeaderValue::from_str(&etag).unwrap());
                    }
                    CacheKey::LastModified(last_modified) => {
                        req.headers_mut().append(
                            header::IF_MODIFIED_SINCE,
                            HeaderValue::from_str(&last_modified).unwrap(),
                        );
                    }
                }
            }
        }

        HttpCacheFuture {
            inner: self.inner.call(req),
            storage: self.storage.clone(),
            uri,
        }
    }
}

#[pin_project]
pub struct HttpCacheFuture<F> {
    #[pin]
    inner: F,
    storage: Option<Arc<dyn CacheStorage>>,
    uri: Uri,
}

impl<F, E> Future for HttpCacheFuture<F>
where
    F: Future<Output = Result<Response<ResBody>, E>>,
{
    type Output = Result<Response<ResBody>, E>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();
        let mut response = ready!(this.inner.poll(cx))?;

        if let Some(ref storage) = this.storage {
            if response.status() == StatusCode::NOT_MODIFIED {
                // If the response is indicated as not modified, reuse the body
                // from the cache.
                let cached = storage.load(this.uri).expect("no body for cache hit");

                for (name, value) in cached.headers.iter() {
                    // These headers are missing in the HTTP 304 Not Modified
                    // response from GitHub API, but are important for further
                    // processing.
                    if [header::CONTENT_TYPE, header::CONTENT_LENGTH, header::LINK].contains(name) {
                        response.headers_mut().append(name, value.clone());
                    }
                }

                // Replace the body.
                *response.body_mut() = BoxBody::new(Box::new(
                    Full::new(Bytes::from(cached.body)).map_err(|infallible| match infallible {}),
                ));
                *response.status_mut() = StatusCode::OK;
            } else {
                // Try to extract a cache header (either ETag or Last-Modified).
                let cache_key = CacheKey::extract_from_headers(response.headers());

                if let Some(key) = cache_key {
                    // If there is a cache header, write the whole response body
                    // to the cache while reading it.
                    let writer = storage.writer(this.uri, key, response.headers().clone());
                    let (parts, mut body) = response.into_parts();
                    body = BoxBody::new(Box::new(WriteToCacheBody::new(body, writer)));
                    response = Response::from_parts(parts, body);
                }
            }
        }

        Poll::Ready(Ok(response))
    }
}

#[pin_project]
struct WriteToCacheBody<B> {
    #[pin]
    inner: B,
    writer: Box<dyn CacheWriter>,
}

impl<B> WriteToCacheBody<B> {
    fn new(inner: B, writer: Box<dyn CacheWriter>) -> Self {
        Self { inner, writer }
    }
}

impl<B> Body for WriteToCacheBody<B>
where
    B: Body<Data = Bytes, Error = crate::Error>,
{
    type Data = Bytes;
    type Error = crate::Error;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let this = self.project();
        match this.inner.poll_frame(cx) {
            Poll::Ready(frame) => {
                if let Some(Ok(ref data)) = frame {
                    if let Some(data) = data.data_ref() {
                        this.writer.write_body(data);
                    }
                }

                Poll::Ready(frame)
            }
            Poll::Pending => Poll::Pending,
        }
    }

    fn is_end_stream(&self) -> bool {
        self.inner.is_end_stream()
    }

    fn size_hint(&self) -> SizeHint {
        self.inner.size_hint()
    }
}

impl CacheKey {
    fn extract_from_headers(headers: &HeaderMap) -> Option<Self> {
        // ETag takes precedence over Last-Modified, because the former is more
        // current and accurate.
        //
        // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Last-Modified
        headers
            .get(header::ETAG)
            .and_then(|etag| Some(CacheKey::ETag(etag.to_str().ok()?.to_owned())))
            .or_else(|| {
                headers
                    .get(header::LAST_MODIFIED)
                    .and_then(|last_modified| {
                        Some(CacheKey::LastModified(
                            last_modified.to_str().ok()?.to_owned(),
                        ))
                    })
            })
    }
}
