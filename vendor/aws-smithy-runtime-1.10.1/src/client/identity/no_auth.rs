/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_runtime_api::client::identity::{Identity, IdentityFuture, ResolveIdentity};
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_types::config_bag::ConfigBag;

/// Identity for the [`NoAuthScheme`](crate::client::auth::no_auth::NoAuthScheme) auth scheme.
#[derive(Debug, Default)]
pub struct NoAuthIdentity;

impl NoAuthIdentity {
    /// Creates a new `NoAuthIdentity`.
    pub fn new() -> Self {
        Self
    }
}

/// Identity resolver for the [`NoAuthScheme`](crate::client::auth::no_auth::NoAuthScheme) auth scheme.
#[derive(Debug, Default)]
pub struct NoAuthIdentityResolver;

impl NoAuthIdentityResolver {
    /// Creates a new `NoAuthIdentityResolver`.
    pub fn new() -> Self {
        Self
    }
}

impl ResolveIdentity for NoAuthIdentityResolver {
    fn resolve_identity<'a>(
        &'a self,
        _runtime_components: &'a RuntimeComponents,
        _: &'a ConfigBag,
    ) -> IdentityFuture<'a> {
        IdentityFuture::ready(Ok(Identity::new(NoAuthIdentity::new(), None)))
    }
}
