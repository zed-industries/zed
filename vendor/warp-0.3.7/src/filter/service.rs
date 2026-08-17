use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_util::future::TryFuture;
use hyper::service::Service;
use pin_project::pin_project;

use crate::reject::IsReject;
use crate::reply::{Reply, Response};
use crate::route::{self, Route};
use crate::{Filter, Request};

/// Convert a `Filter` into a `Service`.
///
/// Filters are normally what APIs are built on in warp. However, it can be
/// useful to convert a `Filter` into a [`Service`][Service], such as if
/// further customizing a `hyper::Service`, or if wanting to make use of
/// the greater [Tower][tower] set of middleware.
///
/// # Example
///
/// Running a `warp::Filter` on a regular `hyper::Server`:
///
/// ```
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// use std::convert::Infallible;
/// use warp::Filter;
///
/// // Our Filter...
/// let route = warp::any().map(|| "Hello From Warp!");
///
/// // Convert it into a `Service`...
/// let svc = warp::service(route);
///
/// // Typical hyper setup...
/// let make_svc = hyper::service::make_service_fn(move |_| async move {
///     Ok::<_, Infallible>(svc)
/// });
///
/// hyper::Server::bind(&([127, 0, 0, 1], 3030).into())
///     .serve(make_svc)
///     .await?;
/// # Ok(())
/// # }
/// ```
///
/// [Service]: https://docs.rs/hyper/0.13.*/hyper/service/trait.Service.html
/// [tower]: https://docs.rs/tower
pub fn service<F>(filter: F) -> FilteredService<F>
where
    F: Filter,
    <F::Future as TryFuture>::Ok: Reply,
    <F::Future as TryFuture>::Error: IsReject,
{
    FilteredService { filter }
}

#[derive(Copy, Clone, Debug)]
pub struct FilteredService<F> {
    filter: F,
}

impl<F> FilteredService<F>
where
    F: Filter,
    <F::Future as TryFuture>::Ok: Reply,
    <F::Future as TryFuture>::Error: IsReject,
{
    #[inline]
    pub(crate) fn call_with_addr(
        &self,
        req: Request,
        remote_addr: Option<SocketAddr>,
    ) -> FilteredFuture<F::Future> {
        debug_assert!(!route::is_set(), "nested route::set calls");

        let route = Route::new(req, remote_addr);
        let fut = route::set(&route, || self.filter.filter(super::Internal));
        FilteredFuture { future: fut, route }
    }
}

impl<F> Service<Request> for FilteredService<F>
where
    F: Filter,
    <F::Future as TryFuture>::Ok: Reply,
    <F::Future as TryFuture>::Error: IsReject,
{
    type Response = Response;
    type Error = Infallible;
    type Future = FilteredFuture<F::Future>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    #[inline]
    fn call(&mut self, req: Request) -> Self::Future {
        self.call_with_addr(req, None)
    }
}

#[pin_project]
#[derive(Debug)]
pub struct FilteredFuture<F> {
    #[pin]
    future: F,
    route: ::std::cell::RefCell<Route>,
}

impl<F> Future for FilteredFuture<F>
where
    F: TryFuture,
    F::Ok: Reply,
    F::Error: IsReject,
{
    type Output = Result<Response, Infallible>;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        debug_assert!(!route::is_set(), "nested route::set calls");

        let pin = self.project();
        let fut = pin.future;
        match route::set(pin.route, || fut.try_poll(cx)) {
            Poll::Ready(Ok(ok)) => Poll::Ready(Ok(ok.into_response())),
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(err)) => {
                tracing::debug!("rejected: {:?}", err);
                Poll::Ready(Ok(err.into_response()))
            }
        }
    }
}
