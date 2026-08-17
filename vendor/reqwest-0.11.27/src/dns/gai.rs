use futures_util::future::FutureExt;
use hyper::client::connect::dns::{GaiResolver as HyperGaiResolver, Name};
use hyper::service::Service;

use crate::dns::{Addrs, Resolve, Resolving};
use crate::error::BoxError;

#[derive(Debug)]
pub struct GaiResolver(HyperGaiResolver);

impl GaiResolver {
    pub fn new() -> Self {
        Self(HyperGaiResolver::new())
    }
}

impl Default for GaiResolver {
    fn default() -> Self {
        GaiResolver::new()
    }
}

impl Resolve for GaiResolver {
    fn resolve(&self, name: Name) -> Resolving {
        let this = &mut self.0.clone();
        Box::pin(Service::<Name>::call(this, name).map(|result| {
            result
                .map(|addrs| -> Addrs { Box::new(addrs) })
                .map_err(|err| -> BoxError { Box::new(err) })
        }))
    }
}
