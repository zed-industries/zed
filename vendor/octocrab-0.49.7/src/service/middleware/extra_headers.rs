use std::sync::Arc;

use http::{header::HeaderName, request::Request, HeaderValue};
use tower::{Layer, Service};

#[derive(Clone)]
/// Layer that adds a static set of extra headers to each request
pub struct ExtraHeadersLayer {
    pub(crate) headers: Arc<Vec<(HeaderName, HeaderValue)>>,
}

impl ExtraHeadersLayer {
    pub fn new(headers: Arc<Vec<(HeaderName, HeaderValue)>>) -> Self {
        ExtraHeadersLayer { headers }
    }
}

impl<S> Layer<S> for ExtraHeadersLayer {
    type Service = ExtraHeaders<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ExtraHeaders {
            inner,
            headers: self.headers.clone(),
        }
    }
}

#[derive(Clone)]
/// Service that adds a static set of extra headers to each request
pub struct ExtraHeaders<S> {
    inner: S,
    headers: Arc<Vec<(HeaderName, HeaderValue)>>,
}

impl<S, ReqBody> Service<Request<ReqBody>> for ExtraHeaders<S>
where
    S: Service<Request<ReqBody>>,
{
    type Error = S::Error;
    type Future = S::Future;
    type Response = S::Response;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        req.headers_mut().extend(self.headers.iter().cloned());
        self.inner.call(req)
    }
}
