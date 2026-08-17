/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::client::connect;
use crate::proxy;
use aws_smithy_runtime_api::box_error::BoxError;
use http_1x::Uri;
use hyper::rt::{Read, ReadBufCursor, Write};
use hyper_util::client::legacy::connect::{Connected, Connection};
use pin_project_lite::pin_project;
use std::future::Future;
use std::io;
use std::io::IoSlice;
use std::pin::Pin;
use std::task::{Context, Poll};

pub(crate) trait AsyncConn:
    Read + Write + Connection + Send + Sync + Unpin + 'static
{
}

impl<T: Read + Write + Connection + Send + Sync + Unpin + 'static> AsyncConn for T {}

pub(crate) type BoxConn = Box<dyn AsyncConn>;

// Future for connecting
pub(crate) type Connecting = Pin<Box<dyn Future<Output = Result<Conn, BoxError>> + Send>>;

pin_project! {
    pub(crate) struct Conn {
        #[pin]
        pub(super)inner: BoxConn,
        pub(super) is_proxy: bool,
    }
}

impl Connection for Conn {
    fn connected(&self) -> Connected {
        self.inner.connected().proxy(self.is_proxy)
    }
}

impl Read for Conn {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: ReadBufCursor<'_>,
    ) -> Poll<io::Result<()>> {
        let this = self.project();
        Read::poll_read(this.inner, cx, buf)
    }
}

impl Write for Conn {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        let this = self.project();
        Write::poll_write(this.inner, cx, buf)
    }

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[IoSlice<'_>],
    ) -> Poll<Result<usize, io::Error>> {
        let this = self.project();
        Write::poll_write_vectored(this.inner, cx, bufs)
    }

    fn is_write_vectored(&self) -> bool {
        self.inner.is_write_vectored()
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        let this = self.project();
        Write::poll_flush(this.inner, cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        let this = self.project();
        Write::poll_shutdown(this.inner, cx)
    }
}

/// HTTP-only proxy connector for handling HTTP requests through HTTP proxies
///
/// This connector handles the HTTP proxy logic when no TLS provider is selected,
/// including request URL modification and proxy authentication.
#[derive(Debug, Clone)]
pub(crate) struct HttpProxyConnector<C> {
    inner: C,
    proxy_config: proxy::ProxyConfig,
}

impl<C> HttpProxyConnector<C> {
    pub(crate) fn new(inner: C, proxy_config: proxy::ProxyConfig) -> Self {
        Self {
            inner,
            proxy_config,
        }
    }
}

impl<C> tower::Service<Uri> for HttpProxyConnector<C>
where
    C: tower::Service<Uri> + Clone + Send + 'static,
    C::Response: hyper::rt::Read
        + hyper::rt::Write
        + hyper_util::client::legacy::connect::Connection
        + Send
        + Sync
        + Unpin
        + 'static,
    C::Future: Send + 'static,
    C::Error: Into<BoxError>,
{
    type Response = connect::Conn;
    type Error = BoxError;
    type Future = connect::Connecting;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, dst: Uri) -> Self::Future {
        // Check if this request should be proxied
        let proxy_intercept = if !self.proxy_config.is_disabled() {
            let matcher = self.proxy_config.clone().into_hyper_util_matcher();
            matcher.intercept(&dst)
        } else {
            None
        };

        if let Some(intercept) = proxy_intercept {
            // HTTP through proxy: Connect to proxy server
            let proxy_uri = intercept.uri().clone();
            let fut = self.inner.call(proxy_uri);
            Box::pin(async move {
                let conn = fut.await.map_err(Into::into)?;
                Ok(connect::Conn {
                    inner: Box::new(conn),
                    is_proxy: true,
                })
            })
        } else {
            // Direct connection
            let fut = self.inner.call(dst);
            Box::pin(async move {
                let conn = fut.await.map_err(Into::into)?;
                Ok(connect::Conn {
                    inner: Box::new(conn),
                    is_proxy: false,
                })
            })
        }
    }
}
