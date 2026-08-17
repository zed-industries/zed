/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use std::error::Error;
use std::fmt::Formatter;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use http_1x::Uri;
use pin_project_lite::pin_project;

use aws_smithy_async::future::timeout::{TimedOutError, Timeout};
use aws_smithy_async::rt::sleep::Sleep;
use aws_smithy_async::rt::sleep::{AsyncSleep, SharedAsyncSleep};
use aws_smithy_runtime_api::box_error::BoxError;

#[derive(Debug)]
pub(crate) struct HttpTimeoutError {
    kind: &'static str,
    duration: Duration,
}

impl std::fmt::Display for HttpTimeoutError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} timeout occurred after {:?}",
            self.kind, self.duration
        )
    }
}

impl Error for HttpTimeoutError {
    // We implement the `source` function as returning a `TimedOutError` because when `downcast_error`
    // or `find_source` is called with an `HttpTimeoutError` (or another error wrapping an `HttpTimeoutError`)
    // this method will be checked to determine if it's a timeout-related error.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&TimedOutError)
    }
}

/// Timeout wrapper that will timeout on the initial TCP connection
///
/// # Stability
/// This interface is unstable.
#[derive(Clone, Debug)]
pub(crate) struct ConnectTimeout<I> {
    inner: I,
    timeout: Option<(SharedAsyncSleep, Duration)>,
}

impl<I> ConnectTimeout<I> {
    /// Create a new `ConnectTimeout` around `inner`.
    ///
    /// Typically, `I` will implement [`hyper_util::client::legacy::connect::Connect`].
    pub(crate) fn new(inner: I, sleep: SharedAsyncSleep, timeout: Duration) -> Self {
        Self {
            inner,
            timeout: Some((sleep, timeout)),
        }
    }

    pub(crate) fn no_timeout(inner: I) -> Self {
        Self {
            inner,
            timeout: None,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct HttpReadTimeout<I> {
    inner: I,
    timeout: Option<(SharedAsyncSleep, Duration)>,
}

impl<I> HttpReadTimeout<I> {
    /// Create a new `HttpReadTimeout` around `inner`.
    ///
    /// Typically, `I` will implement [`tower::Service<http::Request<SdkBody>>`].
    pub(crate) fn new(inner: I, sleep: SharedAsyncSleep, timeout: Duration) -> Self {
        Self {
            inner,
            timeout: Some((sleep, timeout)),
        }
    }

    pub(crate) fn no_timeout(inner: I) -> Self {
        Self {
            inner,
            timeout: None,
        }
    }
}

pin_project! {
    /// Timeout future for Tower services
    ///
    /// Timeout future to handle timing out, mapping errors, and the possibility of not timing out
    /// without incurring an additional allocation for each timeout layer.
    #[project = MaybeTimeoutFutureProj]
    pub enum MaybeTimeoutFuture<F> {
        Timeout {
            #[pin]
            timeout: Timeout<F, Sleep>,
            error_type: &'static str,
            duration: Duration,
        },
        NoTimeout {
            #[pin]
            future: F
        }
    }
}

impl<F, T, E> Future for MaybeTimeoutFuture<F>
where
    F: Future<Output = Result<T, E>>,
    E: Into<BoxError>,
{
    type Output = Result<T, BoxError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let (timeout_future, kind, &mut duration) = match self.project() {
            MaybeTimeoutFutureProj::NoTimeout { future } => {
                return future.poll(cx).map_err(|err| err.into());
            }
            MaybeTimeoutFutureProj::Timeout {
                timeout,
                error_type,
                duration,
            } => (timeout, error_type, duration),
        };
        match timeout_future.poll(cx) {
            Poll::Ready(Ok(response)) => Poll::Ready(response.map_err(|err| err.into())),
            Poll::Ready(Err(_timeout)) => {
                Poll::Ready(Err(HttpTimeoutError { kind, duration }.into()))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<I> tower::Service<Uri> for ConnectTimeout<I>
where
    I: tower::Service<Uri>,
    I::Error: Into<BoxError>,
{
    type Response = I::Response;
    type Error = BoxError;
    type Future = MaybeTimeoutFuture<I::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(|err| err.into())
    }

    fn call(&mut self, req: Uri) -> Self::Future {
        match &self.timeout {
            Some((sleep, duration)) => {
                let sleep = sleep.sleep(*duration);
                MaybeTimeoutFuture::Timeout {
                    timeout: Timeout::new(self.inner.call(req), sleep),
                    error_type: "HTTP connect",
                    duration: *duration,
                }
            }
            None => MaybeTimeoutFuture::NoTimeout {
                future: self.inner.call(req),
            },
        }
    }
}

impl<I, B> tower::Service<http_1x::Request<B>> for HttpReadTimeout<I>
where
    I: tower::Service<http_1x::Request<B>>,
    I::Error: Send + Sync + Error + 'static,
{
    type Response = I::Response;
    type Error = BoxError;
    type Future = MaybeTimeoutFuture<I::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(|err| err.into())
    }

    fn call(&mut self, req: http_1x::Request<B>) -> Self::Future {
        match &self.timeout {
            Some((sleep, duration)) => {
                let sleep = sleep.sleep(*duration);
                MaybeTimeoutFuture::Timeout {
                    timeout: Timeout::new(self.inner.call(req), sleep),
                    error_type: "HTTP read",
                    duration: *duration,
                }
            }
            None => MaybeTimeoutFuture::NoTimeout {
                future: self.inner.call(req),
            },
        }
    }
}

#[cfg(test)]
pub(crate) mod test {
    use hyper::rt::ReadBufCursor;
    use hyper_util::client::legacy::connect::{Connected, Connection};
    use hyper_util::rt::TokioIo;
    use tokio::net::TcpStream;

    use aws_smithy_async::future::never::Never;

    use aws_smithy_runtime_api::box_error::BoxError;
    use aws_smithy_runtime_api::client::result::ConnectorError;
    use http::Uri;
    use hyper::http;
    use hyper::rt::{Read, Write};
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    #[allow(unused)]
    fn connect_timeout_is_correct<T: Send + Sync + Clone + 'static>() {
        is_send_sync::<super::ConnectTimeout<T>>();
    }

    #[allow(unused)]
    fn is_send_sync<T: Send + Sync>() {}

    /// A service that will never return whatever it is you want
    ///
    /// Returned futures will return Pending forever
    #[non_exhaustive]
    #[derive(Clone, Default, Debug)]
    pub(crate) struct NeverConnects;
    impl tower::Service<Uri> for NeverConnects {
        type Response = TokioIo<TcpStream>;
        type Error = ConnectorError;
        type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, _uri: Uri) -> Self::Future {
            Box::pin(async move {
                Never::new().await;
                unreachable!()
            })
        }
    }

    /// A service that will connect but never send any data
    #[derive(Clone, Debug, Default)]
    pub(crate) struct NeverReplies;
    impl tower::Service<Uri> for NeverReplies {
        type Response = EmptyStream;
        type Error = BoxError;
        type Future = std::future::Ready<Result<Self::Response, Self::Error>>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, _req: Uri) -> Self::Future {
            std::future::ready(Ok(EmptyStream))
        }
    }

    /// A stream that will never return or accept any data
    #[non_exhaustive]
    #[derive(Debug, Default)]
    pub(crate) struct EmptyStream;
    impl Read for EmptyStream {
        fn poll_read(
            self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
            _buf: ReadBufCursor<'_>,
        ) -> Poll<Result<(), std::io::Error>> {
            Poll::Pending
        }
    }
    impl Write for EmptyStream {
        fn poll_write(
            self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
            _buf: &[u8],
        ) -> Poll<Result<usize, std::io::Error>> {
            Poll::Pending
        }

        fn poll_flush(
            self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), std::io::Error>> {
            Poll::Pending
        }

        fn poll_shutdown(
            self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), std::io::Error>> {
            Poll::Pending
        }
    }

    impl Connection for EmptyStream {
        fn connected(&self) -> Connected {
            Connected::new()
        }
    }
}
