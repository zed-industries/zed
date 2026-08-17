use std::error::Error as StdError;
use std::future::Future;
use std::task::{Context, Poll};

use crate::body::HttpBody;
use crate::{Request, Response};

/// An asynchronous function from `Request` to `Response`.
pub trait HttpService<ReqBody>: sealed::Sealed<ReqBody> {
    /// The `HttpBody` body of the `http::Response`.
    type ResBody: HttpBody;

    /// The error type that can occur within this `Service`.
    ///
    /// Note: Returning an `Error` to a hyper server will cause the connection
    /// to be abruptly aborted. In most cases, it is better to return a `Response`
    /// with a 4xx or 5xx status code.
    type Error: Into<Box<dyn StdError + Send + Sync>>;

    /// The `Future` returned by this `Service`.
    type Future: Future<Output = Result<Response<Self::ResBody>, Self::Error>>;

    #[doc(hidden)]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;

    #[doc(hidden)]
    fn call(&mut self, req: Request<ReqBody>) -> Self::Future;
}

impl<T, B1, B2> HttpService<B1> for T
where
    T: tower_service::Service<Request<B1>, Response = Response<B2>>,
    B2: HttpBody,
    T::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    type ResBody = B2;

    type Error = T::Error;
    type Future = T::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        tower_service::Service::poll_ready(self, cx)
    }

    fn call(&mut self, req: Request<B1>) -> Self::Future {
        tower_service::Service::call(self, req)
    }
}

impl<T, B1, B2> sealed::Sealed<B1> for T
where
    T: tower_service::Service<Request<B1>, Response = Response<B2>>,
    B2: HttpBody,
{
}

mod sealed {
    pub trait Sealed<T> {}
}
