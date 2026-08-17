/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! The [`NoAuthRuntimePlugin`] and supporting code.

use crate::client::identity::no_auth::NoAuthIdentityResolver;
use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::auth::static_resolver::StaticAuthSchemeOptionResolver;
use aws_smithy_runtime_api::client::auth::{
    AuthScheme, AuthSchemeEndpointConfig, AuthSchemeId, AuthSchemeOption,
    AuthSchemeOptionResolverParams, AuthSchemeOptionsFuture, ResolveAuthSchemeOptions,
    SharedAuthScheme, SharedAuthSchemeOptionResolver, Sign,
};
use aws_smithy_runtime_api::client::identity::{Identity, SharedIdentityResolver};
use aws_smithy_runtime_api::client::orchestrator::HttpRequest;
use aws_smithy_runtime_api::client::runtime_components::{
    GetIdentityResolver, RuntimeComponents, RuntimeComponentsBuilder,
};
use aws_smithy_runtime_api::client::runtime_plugin::{Order, RuntimePlugin};
use aws_smithy_runtime_api::shared::IntoShared;
use aws_smithy_types::config_bag::ConfigBag;
use std::borrow::Cow;

/// Auth scheme ID for "no auth".
pub const NO_AUTH_SCHEME_ID: AuthSchemeId = AuthSchemeId::new("noAuth");

/// A [`RuntimePlugin`] that registers a "no auth" identity resolver and auth scheme.
///
/// This plugin can be used to disable authentication in certain cases, such as when there is
/// a Smithy `@optionalAuth` trait.
///
/// Note: This plugin does not work out of the box because it does not configure an auth scheme option resolver
/// that recognizes the `noAuth` scheme.
#[doc(hidden)]
#[non_exhaustive]
#[derive(Debug)]
#[deprecated(since = "1.9.8", note = "Use `NoAuthRuntimePluginV2` instead")]
pub struct NoAuthRuntimePlugin(RuntimeComponentsBuilder);

#[allow(deprecated)]
impl Default for NoAuthRuntimePlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(deprecated)]
impl NoAuthRuntimePlugin {
    /// Creates a new `NoAuthRuntimePlugin`.
    pub fn new() -> Self {
        Self(
            RuntimeComponentsBuilder::new("NoAuthRuntimePlugin")
                .with_identity_resolver(
                    NO_AUTH_SCHEME_ID,
                    SharedIdentityResolver::new(NoAuthIdentityResolver::new()),
                )
                .with_auth_scheme(SharedAuthScheme::new(NoAuthScheme::new())),
        )
    }
}

#[allow(deprecated)]
impl RuntimePlugin for NoAuthRuntimePlugin {
    fn runtime_components(
        &self,
        _: &RuntimeComponentsBuilder,
    ) -> Cow<'_, RuntimeComponentsBuilder> {
        Cow::Borrowed(&self.0)
    }
}

/// A [`RuntimePlugin`] that registers a "no auth" identity resolver, auth scheme, and auth scheme option resolver.
///
/// Ideally, a Smithy model should use `@optionalAuth` or `@auth([])` on operations so that:
/// - The Smithy runtime supports the no-auth scheme
/// - The code-generated default auth scheme option resolver includes the no-auth scheme for those operations
///
/// When that is not possible, this plugin can be used to achieve the same effect.
#[derive(Debug)]
pub struct NoAuthRuntimePluginV2(RuntimeComponentsBuilder);

impl Default for NoAuthRuntimePluginV2 {
    fn default() -> Self {
        Self::new()
    }
}

impl NoAuthRuntimePluginV2 {
    /// Creates a new `NoAuthRuntimePluginV2`.
    pub fn new() -> Self {
        Self(
            RuntimeComponentsBuilder::new("NoAuthRuntimePluginV2")
                .with_identity_resolver(
                    NO_AUTH_SCHEME_ID,
                    SharedIdentityResolver::new(NoAuthIdentityResolver::new()),
                )
                .with_auth_scheme(SharedAuthScheme::new(NoAuthScheme::new())),
        )
    }
}

impl RuntimePlugin for NoAuthRuntimePluginV2 {
    fn order(&self) -> Order {
        // This plugin should be applied as an escape hatch to append the no-auth scheme, hence `NestedComponents`.
        Order::NestedComponents
    }

    fn runtime_components(
        &self,
        current_components: &RuntimeComponentsBuilder,
    ) -> Cow<'_, RuntimeComponentsBuilder> {
        // No auth scheme option resolver is configured here because it needs to access
        // the existing resolver (likely the code-generated default) stored in the
        // current runtime components builder.
        let auth_scheme_option_resolver: SharedAuthSchemeOptionResolver =
            match current_components.auth_scheme_option_resolver() {
                Some(current_resolver) => {
                    NoAuthSchemeOptionResolver::new(current_resolver.clone()).into_shared()
                }
                None => StaticAuthSchemeOptionResolver::new(vec![NO_AUTH_SCHEME_ID]).into_shared(),
            };
        Cow::Owned(
            self.0
                .clone()
                .with_auth_scheme_option_resolver(Some(auth_scheme_option_resolver)),
        )
    }
}

#[derive(Debug)]
struct NoAuthSchemeOptionResolver<R> {
    inner: R,
}

impl<R> NoAuthSchemeOptionResolver<R> {
    fn new(inner: R) -> Self {
        Self { inner }
    }
}

impl<R> ResolveAuthSchemeOptions for NoAuthSchemeOptionResolver<R>
where
    R: ResolveAuthSchemeOptions,
{
    fn resolve_auth_scheme_options_v2<'a>(
        &'a self,
        params: &'a AuthSchemeOptionResolverParams,
        cfg: &'a ConfigBag,
        runtime_components: &'a RuntimeComponents,
    ) -> AuthSchemeOptionsFuture<'a> {
        let inner_future =
            self.inner
                .resolve_auth_scheme_options_v2(params, cfg, runtime_components);

        AuthSchemeOptionsFuture::new(async move {
            let mut options = inner_future.await?;
            options.push(AuthSchemeOption::from(NO_AUTH_SCHEME_ID));
            Ok(options)
        })
    }
}

/// The "no auth" auth scheme.
///
/// The orchestrator requires an auth scheme, so Smithy's `@optionalAuth` trait is implemented
/// by placing a "no auth" auth scheme at the end of the auth scheme options list so that it is
/// used if there's no identity resolver available for the other auth schemes. It's also used
/// for models that don't have auth at all.
#[derive(Debug, Default)]
pub struct NoAuthScheme {
    signer: NoAuthSigner,
}

impl NoAuthScheme {
    /// Creates a new `NoAuthScheme`.
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Default)]
struct NoAuthSigner;

impl Sign for NoAuthSigner {
    fn sign_http_request(
        &self,
        _request: &mut HttpRequest,
        _identity: &Identity,
        _auth_scheme_endpoint_config: AuthSchemeEndpointConfig<'_>,
        _runtime_components: &RuntimeComponents,
        _config_bag: &ConfigBag,
    ) -> Result<(), BoxError> {
        Ok(())
    }
}

impl AuthScheme for NoAuthScheme {
    fn scheme_id(&self) -> AuthSchemeId {
        NO_AUTH_SCHEME_ID
    }

    fn identity_resolver(
        &self,
        identity_resolvers: &dyn GetIdentityResolver,
    ) -> Option<SharedIdentityResolver> {
        identity_resolvers.identity_resolver(NO_AUTH_SCHEME_ID)
    }

    fn signer(&self) -> &dyn Sign {
        &self.signer
    }
}
