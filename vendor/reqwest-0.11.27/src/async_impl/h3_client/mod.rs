#![cfg(feature = "http3")]

pub(crate) mod connect;
pub(crate) mod dns;
mod pool;

use crate::async_impl::h3_client::pool::{Key, Pool, PoolClient};
use crate::error::{BoxError, Error, Kind};
use crate::{error, Body};
use connect::H3Connector;
use futures_util::future;
use http::{Request, Response};
use hyper::Body as HyperBody;
use log::trace;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

#[derive(Clone)]
pub(crate) struct H3Client {
    pool: Pool,
    connector: H3Connector,
}

impl H3Client {
    pub fn new(connector: H3Connector, pool_timeout: Option<Duration>) -> Self {
        H3Client {
            pool: Pool::new(pool_timeout),
            connector,
        }
    }

    async fn get_pooled_client(&mut self, key: Key) -> Result<PoolClient, BoxError> {
        if let Some(client) = self.pool.try_pool(&key) {
            trace!("getting client from pool with key {key:?}");
            return Ok(client);
        }

        trace!("did not find connection {key:?} in pool so connecting...");

        let dest = pool::domain_as_uri(key.clone());
        self.pool.connecting(key.clone())?;
        let (driver, tx) = self.connector.connect(dest).await?;
        Ok(self.pool.new_connection(key, driver, tx))
    }

    async fn send_request(
        mut self,
        key: Key,
        req: Request<Body>,
    ) -> Result<Response<HyperBody>, Error> {
        let mut pooled = match self.get_pooled_client(key).await {
            Ok(client) => client,
            Err(e) => return Err(error::request(e)),
        };
        pooled
            .send_request(req)
            .await
            .map_err(|e| Error::new(Kind::Request, Some(e)))
    }

    pub fn request(&self, mut req: Request<Body>) -> H3ResponseFuture {
        let pool_key = match pool::extract_domain(req.uri_mut()) {
            Ok(s) => s,
            Err(e) => {
                return H3ResponseFuture {
                    inner: Box::pin(future::err(e)),
                }
            }
        };
        H3ResponseFuture {
            inner: Box::pin(self.clone().send_request(pool_key, req)),
        }
    }
}

pub(crate) struct H3ResponseFuture {
    inner: Pin<Box<dyn Future<Output = Result<Response<HyperBody>, Error>> + Send>>,
}

impl Future for H3ResponseFuture {
    type Output = Result<Response<HyperBody>, Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.inner.as_mut().poll(cx)
    }
}
