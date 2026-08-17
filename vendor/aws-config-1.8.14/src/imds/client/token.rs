/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! IMDS Token Middleware
//! Requests to IMDS are two part:
//! 1. A PUT request to the token API is made
//! 2. A GET request is made to the requested API. The Token is added as a header.
//!
//! This module implements a middleware that will:
//! - Load a token via the token API
//! - Cache the token according to the TTL
//! - Retry token loading when it fails
//! - Attach the token to the request in the `x-aws-ec2-metadata-token` header

use crate::identity::IdentityCache;
use crate::imds::client::error::{ImdsError, TokenError, TokenErrorKind};
use aws_smithy_async::time::SharedTimeSource;
use aws_smithy_runtime::client::orchestrator::operation::Operation;
use aws_smithy_runtime::expiring_cache::ExpiringCache;
use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::auth::static_resolver::StaticAuthSchemeOptionResolver;
use aws_smithy_runtime_api::client::auth::{
    AuthScheme, AuthSchemeEndpointConfig, AuthSchemeId, Sign,
};
use aws_smithy_runtime_api::client::identity::{
    Identity, IdentityFuture, ResolveIdentity, SharedIdentityResolver,
};
use aws_smithy_runtime_api::client::orchestrator::{HttpRequest, HttpResponse, OrchestratorError};
use aws_smithy_runtime_api::client::runtime_components::{
    GetIdentityResolver, RuntimeComponents, RuntimeComponentsBuilder,
};
use aws_smithy_runtime_api::client::runtime_plugin::{RuntimePlugin, SharedRuntimePlugin};
use aws_smithy_types::body::SdkBody;
use aws_smithy_types::config_bag::ConfigBag;
use http::{HeaderValue, Uri};
use std::borrow::Cow;
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

/// Token Refresh Buffer
///
/// Tokens are cached to remove the need to reload the token between subsequent requests. To ensure
/// that a request never fails with a 401 (expired token), a buffer window exists during which the token
/// may not be expired, but will still be refreshed.
const TOKEN_REFRESH_BUFFER: Duration = Duration::from_secs(120);

const X_AWS_EC2_METADATA_TOKEN_TTL_SECONDS: &str = "x-aws-ec2-metadata-token-ttl-seconds";
const X_AWS_EC2_METADATA_TOKEN: &str = "x-aws-ec2-metadata-token";
const IMDS_TOKEN_AUTH_SCHEME: AuthSchemeId = AuthSchemeId::new(X_AWS_EC2_METADATA_TOKEN);

#[derive(Debug)]
struct TtlToken {
    value: HeaderValue,
    ttl: Duration,
}

/// IMDS Token
#[derive(Clone)]
struct Token {
    value: HeaderValue,
    expiry: SystemTime,
}
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Token")
            .field("value", &"** redacted **")
            .field("expiry", &self.expiry)
            .finish()
    }
}

/// Token Runtime Plugin
///
/// This runtime plugin wires up the necessary components to load/cache a token
/// when required and handle caching/expiry. This token will get attached to the
/// request to IMDS on the `x-aws-ec2-metadata-token` header.
#[derive(Debug)]
pub(super) struct TokenRuntimePlugin {
    components: RuntimeComponentsBuilder,
}

impl TokenRuntimePlugin {
    pub(super) fn new(common_plugin: SharedRuntimePlugin, token_ttl: Duration) -> Self {
        Self {
            components: RuntimeComponentsBuilder::new("TokenRuntimePlugin")
                .with_auth_scheme(TokenAuthScheme::new())
                .with_auth_scheme_option_resolver(Some(StaticAuthSchemeOptionResolver::new(vec![
                    IMDS_TOKEN_AUTH_SCHEME,
                ])))
                // The TokenResolver has a cache of its own, so don't use identity caching
                .with_identity_cache(Some(IdentityCache::no_cache()))
                .with_identity_resolver(
                    IMDS_TOKEN_AUTH_SCHEME,
                    TokenResolver::new(common_plugin, token_ttl),
                ),
        }
    }
}

impl RuntimePlugin for TokenRuntimePlugin {
    fn runtime_components(
        &self,
        _current_components: &RuntimeComponentsBuilder,
    ) -> Cow<'_, RuntimeComponentsBuilder> {
        Cow::Borrowed(&self.components)
    }
}

#[derive(Debug)]
struct TokenResolverInner {
    cache: ExpiringCache<Token, ImdsError>,
    refresh: Operation<(), TtlToken, TokenError>,
}

#[derive(Clone, Debug)]
struct TokenResolver {
    inner: Arc<TokenResolverInner>,
}

impl TokenResolver {
    fn new(common_plugin: SharedRuntimePlugin, token_ttl: Duration) -> Self {
        Self {
            inner: Arc::new(TokenResolverInner {
                cache: ExpiringCache::new(TOKEN_REFRESH_BUFFER),
                refresh: Operation::builder()
                    .service_name("imds")
                    .operation_name("get-token")
                    .runtime_plugin(common_plugin)
                    .no_auth()
                    .with_connection_poisoning()
                    .serializer(move |_| {
                        Ok(http::Request::builder()
                            .method("PUT")
                            .uri(Uri::from_static("/latest/api/token"))
                            .header(X_AWS_EC2_METADATA_TOKEN_TTL_SECONDS, token_ttl.as_secs())
                            .body(SdkBody::empty())
                            .expect("valid HTTP request")
                            .try_into()
                            .unwrap())
                    })
                    .deserializer(move |response| {
                        parse_token_response(response).map_err(OrchestratorError::operation)
                    })
                    .build(),
            }),
        }
    }

    async fn get_token(
        &self,
        time_source: SharedTimeSource,
    ) -> Result<(Token, SystemTime), ImdsError> {
        let result = self.inner.refresh.invoke(()).await;
        let now = time_source.now();
        result
            .map(|token| {
                let token = Token {
                    value: token.value,
                    expiry: now + token.ttl,
                };
                let expiry = token.expiry;
                (token, expiry)
            })
            .map_err(ImdsError::failed_to_load_token)
    }
}

fn parse_token_response(response: &HttpResponse) -> Result<TtlToken, TokenError> {
    match response.status().as_u16() {
        400 => return Err(TokenErrorKind::InvalidParameters.into()),
        403 => return Err(TokenErrorKind::Forbidden.into()),
        _ => {}
    }
    let mut value =
        HeaderValue::from_bytes(response.body().bytes().expect("non-streaming response"))
            .map_err(|_| TokenErrorKind::InvalidToken)?;
    value.set_sensitive(true);

    let ttl: u64 = response
        .headers()
        .get(X_AWS_EC2_METADATA_TOKEN_TTL_SECONDS)
        .ok_or(TokenErrorKind::NoTtl)?
        .parse()
        .map_err(|_parse_error| TokenErrorKind::InvalidTtl)?;
    Ok(TtlToken {
        value,
        ttl: Duration::from_secs(ttl),
    })
}

impl ResolveIdentity for TokenResolver {
    fn resolve_identity<'a>(
        &'a self,
        components: &'a RuntimeComponents,
        _config_bag: &'a ConfigBag,
    ) -> IdentityFuture<'a> {
        let time_source = components
            .time_source()
            .expect("time source required for IMDS token caching");
        IdentityFuture::new(async {
            let now = time_source.now();
            let preloaded_token = self.inner.cache.yield_or_clear_if_expired(now).await;
            let token = match preloaded_token {
                Some(token) => {
                    tracing::trace!(
                        buffer_time=?TOKEN_REFRESH_BUFFER,
                        expiration=?token.expiry,
                        now=?now,
                        "loaded IMDS token from cache");
                    Ok(token)
                }
                None => {
                    tracing::debug!("IMDS token cache miss");
                    self.inner
                        .cache
                        .get_or_load(|| async { self.get_token(time_source).await })
                        .await
                }
            }?;

            let expiry = token.expiry;
            Ok(Identity::new(token, Some(expiry)))
        })
    }
}

#[derive(Debug)]
struct TokenAuthScheme {
    signer: TokenSigner,
}

impl TokenAuthScheme {
    fn new() -> Self {
        Self {
            signer: TokenSigner,
        }
    }
}

impl AuthScheme for TokenAuthScheme {
    fn scheme_id(&self) -> AuthSchemeId {
        IMDS_TOKEN_AUTH_SCHEME
    }

    fn identity_resolver(
        &self,
        identity_resolvers: &dyn GetIdentityResolver,
    ) -> Option<SharedIdentityResolver> {
        identity_resolvers.identity_resolver(IMDS_TOKEN_AUTH_SCHEME)
    }

    fn signer(&self) -> &dyn Sign {
        &self.signer
    }
}

#[derive(Debug)]
struct TokenSigner;

impl Sign for TokenSigner {
    fn sign_http_request(
        &self,
        request: &mut HttpRequest,
        identity: &Identity,
        _auth_scheme_endpoint_config: AuthSchemeEndpointConfig<'_>,
        _runtime_components: &RuntimeComponents,
        _config_bag: &ConfigBag,
    ) -> Result<(), BoxError> {
        let token = identity.data::<Token>().expect("correct type");
        request
            .headers_mut()
            .append(X_AWS_EC2_METADATA_TOKEN, token.value.clone());
        Ok(())
    }
}
