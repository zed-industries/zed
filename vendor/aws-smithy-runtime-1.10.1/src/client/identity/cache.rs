/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_runtime_api::client::identity::{
    IdentityFuture, ResolveCachedIdentity, ResolveIdentity, SharedIdentityCache,
    SharedIdentityResolver,
};
use aws_smithy_runtime_api::shared::IntoShared;
use aws_smithy_types::config_bag::ConfigBag;

mod lazy;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
pub use lazy::LazyCacheBuilder;

/// Identity cache configuration.
///
/// # Examples
///
/// Disabling identity caching:
/// ```no_run
/// use aws_smithy_runtime::client::identity::IdentityCache;
///
/// # /*
/// let config = some_service::Config::builder()
///     .identity_cache(
/// # */
/// # drop(
///         IdentityCache::no_cache()
/// # );
/// # /*
///     )
///     // ...
///     .build();
/// let client = some_service::Client::new(config);
/// # */
/// ```
///
/// Customizing lazy caching:
/// ```no_run
/// use aws_smithy_runtime::client::identity::IdentityCache;
/// use std::time::Duration;
///
/// # /*
/// let config = some_service::Config::builder()
///     .identity_cache(
/// # */
/// # drop(
///         IdentityCache::lazy()
///             // change the load timeout to 10 seconds
///             .load_timeout(Duration::from_secs(10))
///             .build()
/// # );
/// # /*
///     )
///     // ...
///     .build();
/// let client = some_service::Client::new(config);
/// # */
/// ```
#[non_exhaustive]
pub struct IdentityCache;

impl IdentityCache {
    /// Create an identity cache that does not cache any resolved identities.
    pub fn no_cache() -> SharedIdentityCache {
        NoCache.into_shared()
    }

    /// Configure a lazy identity cache.
    ///
    /// Identities are lazy loaded and then cached when a request is made.
    pub fn lazy() -> LazyCacheBuilder {
        LazyCacheBuilder::new()
    }
}

#[derive(Clone, Debug)]
struct NoCache;

impl ResolveCachedIdentity for NoCache {
    fn resolve_cached_identity<'a>(
        &'a self,
        resolver: SharedIdentityResolver,
        runtime_components: &'a RuntimeComponents,
        config_bag: &'a ConfigBag,
    ) -> IdentityFuture<'a> {
        IdentityFuture::new(async move {
            resolver
                .resolve_identity(runtime_components, config_bag)
                .await
        })
    }
}
