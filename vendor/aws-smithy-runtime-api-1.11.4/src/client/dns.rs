/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Interfaces for resolving DNS

use crate::box_error::BoxError;
use crate::impl_shared_conversions;
use std::error::Error as StdError;
use std::fmt;
use std::net::IpAddr;
use std::sync::Arc;

/// Error that occurs when failing to perform a DNS lookup.
#[derive(Debug)]
pub struct ResolveDnsError {
    source: BoxError,
}

impl ResolveDnsError {
    /// Creates a new `DnsLookupFailed` error.
    pub fn new(source: impl Into<BoxError>) -> Self {
        ResolveDnsError {
            source: source.into(),
        }
    }
}

impl fmt::Display for ResolveDnsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to perform DNS lookup")
    }
}

impl StdError for ResolveDnsError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&*self.source as _)
    }
}

new_type_future! {
    #[doc = "New-type for the future returned by the [`ResolveDns`] trait."]
    pub struct DnsFuture<'a, Vec<IpAddr>, ResolveDnsError>;
}

/// Trait for resolving domain names
pub trait ResolveDns: fmt::Debug + Send + Sync {
    /// Asynchronously resolve the given domain name
    fn resolve_dns<'a>(&'a self, name: &'a str) -> DnsFuture<'a>;
}

/// Shared instance of [`ResolveDns`].
#[derive(Clone, Debug)]
pub struct SharedDnsResolver(Arc<dyn ResolveDns>);

impl SharedDnsResolver {
    /// Create a new `SharedDnsResolver`.
    pub fn new(resolver: impl ResolveDns + 'static) -> Self {
        Self(Arc::new(resolver))
    }
}

impl ResolveDns for SharedDnsResolver {
    fn resolve_dns<'a>(&'a self, name: &'a str) -> DnsFuture<'a> {
        self.0.resolve_dns(name)
    }
}

impl_shared_conversions!(convert SharedDnsResolver from ResolveDns using SharedDnsResolver::new);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_send() {
        fn is_send<T: Send>() {}
        is_send::<DnsFuture<'_>>();
    }
}
