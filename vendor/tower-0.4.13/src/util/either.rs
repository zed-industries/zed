//! Contains [`Either`] and related types and functions.
//!
//! See [`Either`] documentation for more details.

use futures_core::ready;
use pin_project::pin_project;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower_layer::Layer;
use tower_service::Service;

/// Combine two different service types into a single type.
///
/// Both services must be of the same request, response, and error types.
/// [`Either`] is useful for handling conditional branching in service middleware
/// to different inner service types.
#[pin_project(project = EitherProj)]
#[derive(Clone, Debug)]
pub enum Either<A, B> {
    /// One type of backing [`Service`].
    A(#[pin] A),
    /// The other type of backing [`Service`].
    B(#[pin] B),
}

impl<A, B, Request> Service<Request> for Either<A, B>
where
    A: Service<Request>,
    A::Error: Into<crate::BoxError>,
    B: Service<Request, Response = A::Response>,
    B::Error: Into<crate::BoxError>,
{
    type Response = A::Response;
    type Error = crate::BoxError;
    type Future = Either<A::Future, B::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        use self::Either::*;

        match self {
            A(service) => Poll::Ready(Ok(ready!(service.poll_ready(cx)).map_err(Into::into)?)),
            B(service) => Poll::Ready(Ok(ready!(service.poll_ready(cx)).map_err(Into::into)?)),
        }
    }

    fn call(&mut self, request: Request) -> Self::Future {
        use self::Either::*;

        match self {
            A(service) => A(service.call(request)),
            B(service) => B(service.call(request)),
        }
    }
}

impl<A, B, T, AE, BE> Future for Either<A, B>
where
    A: Future<Output = Result<T, AE>>,
    AE: Into<crate::BoxError>,
    B: Future<Output = Result<T, BE>>,
    BE: Into<crate::BoxError>,
{
    type Output = Result<T, crate::BoxError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.project() {
            EitherProj::A(fut) => Poll::Ready(Ok(ready!(fut.poll(cx)).map_err(Into::into)?)),
            EitherProj::B(fut) => Poll::Ready(Ok(ready!(fut.poll(cx)).map_err(Into::into)?)),
        }
    }
}

impl<S, A, B> Layer<S> for Either<A, B>
where
    A: Layer<S>,
    B: Layer<S>,
{
    type Service = Either<A::Service, B::Service>;

    fn layer(&self, inner: S) -> Self::Service {
        match self {
            Either::A(layer) => Either::A(layer.layer(inner)),
            Either::B(layer) => Either::B(layer.layer(inner)),
        }
    }
}
