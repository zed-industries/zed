/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! AWS Access Tokens for SSO
//!
//! When authenticating with an AWS Builder ID, single sign-on (SSO) will provide
//! an access token that can then be used to authenticate with services such as
//! Code Catalyst.
//!
//! This module provides the [`ProvideToken`] trait that is used to configure
//! token providers in the SDK config.

use crate::{provider::error::TokenError, provider::future, Token};
use aws_smithy_runtime_api::client::{
    identity::{IdentityCachePartition, IdentityFuture, ResolveIdentity},
    runtime_components::RuntimeComponents,
};
use aws_smithy_runtime_api::impl_shared_conversions;
use aws_smithy_types::config_bag::ConfigBag;
use std::sync::Arc;

/// Result type for token providers
pub type Result = std::result::Result<Token, TokenError>;

/// Access Token Provider
pub trait ProvideToken: Send + Sync + std::fmt::Debug {
    /// Returns a future that provides an access token.
    fn provide_token<'a>(&'a self) -> future::ProvideToken<'a>
    where
        Self: 'a;
}

impl ProvideToken for Token {
    fn provide_token<'a>(&'a self) -> future::ProvideToken<'a>
    where
        Self: 'a,
    {
        future::ProvideToken::ready(Ok(self.clone()))
    }
}

/// Access token provider wrapper that may be shared.
///
/// Newtype wrapper around [`ProvideToken`] that implements `Clone` using an internal `Arc`.
#[derive(Clone, Debug)]
pub struct SharedTokenProvider(Arc<dyn ProvideToken>, IdentityCachePartition);

impl SharedTokenProvider {
    /// Create a new [`SharedTokenProvider`] from [`ProvideToken`].
    ///
    /// The given provider will be wrapped in an internal `Arc`. If your
    /// provider is already in an `Arc`, use `SharedTokenProvider::from(provider)` instead.
    pub fn new(provider: impl ProvideToken + 'static) -> Self {
        Self(Arc::new(provider), IdentityCachePartition::new())
    }
}

impl AsRef<dyn ProvideToken> for SharedTokenProvider {
    fn as_ref(&self) -> &(dyn ProvideToken + 'static) {
        self.0.as_ref()
    }
}

impl From<Arc<dyn ProvideToken>> for SharedTokenProvider {
    fn from(provider: Arc<dyn ProvideToken>) -> Self {
        SharedTokenProvider(provider, IdentityCachePartition::new())
    }
}

impl ProvideToken for SharedTokenProvider {
    fn provide_token<'a>(&'a self) -> future::ProvideToken<'a>
    where
        Self: 'a,
    {
        self.0.provide_token()
    }
}

impl ResolveIdentity for SharedTokenProvider {
    fn resolve_identity<'a>(
        &'a self,
        _runtime_components: &'a RuntimeComponents,
        _config_bag: &'a ConfigBag,
    ) -> IdentityFuture<'a> {
        IdentityFuture::new(async move { Ok(self.provide_token().await?.into()) })
    }

    fn cache_partition(&self) -> Option<IdentityCachePartition> {
        Some(self.1)
    }
}

impl_shared_conversions!(convert SharedTokenProvider from ProvideToken using SharedTokenProvider::new);

#[cfg(test)]
mod tests {
    use aws_smithy_runtime_api::client::identity::SharedIdentityResolver;

    use super::*;

    #[test]
    fn reuses_cache_partition() {
        let token = Token::new("token", None);
        let provider = SharedTokenProvider::new(token);
        let partition = provider.cache_partition();
        assert!(partition.is_some());

        let identity_resolver = SharedIdentityResolver::new(provider);
        let identity_partition = identity_resolver.cache_partition();

        assert!(partition.unwrap() == identity_partition);
    }
}
