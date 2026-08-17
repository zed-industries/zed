/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */
use aws_smithy_runtime_api::client::dns::ResolveDns;
use hyper_util::client::legacy::connect::dns::Name;
use std::error::Error;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::vec;

/// A bridge that allows our `ResolveDns` trait to work with Hyper's `Resolver` interface (based on tower)
#[derive(Clone)]
#[allow(dead_code)]
pub(crate) struct HyperUtilResolver<R> {
    pub(crate) resolver: R,
}

impl<R: ResolveDns + Clone + 'static> tower::Service<Name> for HyperUtilResolver<R> {
    type Response = vec::IntoIter<SocketAddr>;
    type Error = Box<dyn Error + Send + Sync>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Name) -> Self::Future {
        let resolver = self.resolver.clone();
        Box::pin(async move {
            let dns_entries = resolver.resolve_dns(req.as_str()).await?;
            Ok(dns_entries
                .into_iter()
                .map(|ip_addr| SocketAddr::new(ip_addr, 0))
                .collect::<Vec<_>>()
                .into_iter())
        })
    }
}
