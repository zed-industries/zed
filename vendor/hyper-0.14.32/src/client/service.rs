//! Utilities used to interact with the Tower ecosystem.
//!
//! This module provides `Connect` which hook-ins into the Tower ecosystem.

use std::error::Error as StdError;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use tracing::debug;

#[cfg_attr(feature = "deprecated", allow(deprecated))]
use super::conn::{Builder, SendRequest};
use crate::{
    body::HttpBody,
    service::{MakeConnection, Service},
};

/// Creates a connection via `SendRequest`.
///
/// This accepts a `hyper::client::conn::Builder` and provides
/// a `MakeService` implementation to create connections from some
/// target `T`.
#[derive(Debug)]
pub struct Connect<C, B, T> {
    inner: C,
    #[cfg_attr(feature = "deprecated", allow(deprecated))]
    builder: Builder,
    _pd: PhantomData<fn(T, B)>,
}

impl<C, B, T> Connect<C, B, T> {
    /// Create a new `Connect` with some inner connector `C` and a connection
    /// builder.
    #[cfg_attr(feature = "deprecated", allow(deprecated))]
    pub fn new(inner: C, builder: Builder) -> Self {
        Self {
            inner,
            builder,
            _pd: PhantomData,
        }
    }
}

impl<C, B, T> Service<T> for Connect<C, B, T>
where
    C: MakeConnection<T>,
    C::Connection: Unpin + Send + 'static,
    C::Future: Send + 'static,
    C::Error: Into<Box<dyn StdError + Send + Sync>> + Send,
    B: HttpBody + Unpin + Send + 'static,
    B::Data: Send + Unpin,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    #[cfg_attr(feature = "deprecated", allow(deprecated))]
    type Response = SendRequest<B>;
    type Error = crate::Error;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner
            .poll_ready(cx)
            .map_err(|e| crate::Error::new(crate::error::Kind::Connect).with(e.into()))
    }

    fn call(&mut self, req: T) -> Self::Future {
        let builder = self.builder.clone();
        let io = self.inner.make_connection(req);

        let fut = async move {
            match io.await {
                Ok(io) => match builder.handshake(io).await {
                    Ok((sr, conn)) => {
                        #[cfg_attr(feature = "deprecated", allow(deprecated))]
                        builder.exec.execute(async move {
                            if let Err(e) = conn.await {
                                debug!("connection error: {:?}", e);
                            }
                        });
                        Ok(sr)
                    }
                    Err(e) => Err(e),
                },
                Err(e) => {
                    let err = crate::Error::new(crate::error::Kind::Connect).with(e.into());
                    Err(err)
                }
            }
        };

        Box::pin(fut)
    }
}
