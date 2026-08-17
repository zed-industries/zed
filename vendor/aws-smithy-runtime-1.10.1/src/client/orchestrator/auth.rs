/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::client::auth::no_auth::NO_AUTH_SCHEME_ID;
use crate::client::identity::IdentityCache;
use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::auth::{
    AuthScheme, AuthSchemeEndpointConfig, AuthSchemeId, AuthSchemeOption,
    AuthSchemeOptionResolverParams, AuthSchemePreference, ResolveAuthSchemeOptions,
};
use aws_smithy_runtime_api::client::endpoint::{EndpointResolverParams, ResolveEndpoint};
use aws_smithy_runtime_api::client::identity::{Identity, ResolveIdentity};
use aws_smithy_runtime_api::client::identity::{IdentityCacheLocation, ResolveCachedIdentity};
use aws_smithy_runtime_api::client::interceptors::context::InterceptorContext;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_types::config_bag::{ConfigBag, FrozenLayer, Storable, StoreReplace};
use aws_smithy_types::endpoint::Endpoint;
use aws_smithy_types::Document;
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt;
use tracing::trace;

#[derive(Debug)]
struct NoMatchingAuthSchemeError(ExploredList);

impl fmt::Display for NoMatchingAuthSchemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let explored = &self.0;

        // Use the information we have about the auth options that were explored to construct
        // as helpful of an error message as possible.
        if explored.items().count() == 0 {
            return f.write_str(
                "no auth options are available. This can happen if there's \
                    a problem with the service model, or if there is a codegen bug.",
            );
        }
        if explored
            .items()
            .all(|explored| matches!(explored.result, ExploreResult::NoAuthScheme))
        {
            return f.write_str(
                "no auth schemes are registered. This can happen if there's \
                    a problem with the service model, or if there is a codegen bug.",
            );
        }

        let mut try_add_identity = false;
        let mut likely_bug = false;
        f.write_str("failed to select an auth scheme to sign the request with.")?;
        for item in explored.items() {
            write!(
                f,
                " \"{}\" wasn't a valid option because ",
                item.scheme_id.inner()
            )?;
            f.write_str(match item.result {
                ExploreResult::NoAuthScheme => {
                    likely_bug = true;
                    "no auth scheme was registered for it."
                }
                ExploreResult::NoIdentityResolver => {
                    try_add_identity = true;
                    "there was no identity resolver for it."
                }
                ExploreResult::MissingEndpointConfig => {
                    likely_bug = true;
                    "there is auth config in the endpoint config, but this scheme wasn't listed in it \
                    (see https://github.com/smithy-lang/smithy-rs/discussions/3281 for more details)."
                }
                ExploreResult::NotExplored => {
                    debug_assert!(false, "this should be unreachable");
                    "<unknown>"
                }
            })?;
        }
        if try_add_identity {
            f.write_str(" Be sure to set an identity, such as credentials, auth token, or other identity type that is required for this service.")?;
        } else if likely_bug {
            f.write_str(" This is likely a bug.")?;
        }
        if !likely_bug
            && !explored
                .items()
                .any(|explored| explored.scheme_id == NO_AUTH_SCHEME_ID)
        {
            f.write_str(
                " If you intended to make an unauthenticated request, consider using `@optionalAuth` or `@auth([])` on the operation in the service model and regenerating the client SDK. \
                If modifying the model is not possible, you can disable authentication at runtime as described in https://github.com/smithy-lang/smithy-rs/discussions/4197."
            )?;
        }
        if explored.truncated {
            f.write_str(" Note: there were other auth schemes that were evaluated that weren't listed here.")?;
        }

        Ok(())
    }
}

impl StdError for NoMatchingAuthSchemeError {}

#[derive(Debug)]
enum AuthOrchestrationError {
    MissingEndpointConfig,
    BadAuthSchemeEndpointConfig(Cow<'static, str>),
    FailedToResolveEndpoint(BoxError),
}

impl fmt::Display for AuthOrchestrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // This error is never bubbled up
            Self::MissingEndpointConfig => f.write_str("missing endpoint config"),
            Self::BadAuthSchemeEndpointConfig(message) => f.write_str(message),
            Self::FailedToResolveEndpoint(source) => {
                write!(f, "failed to resolve endpoint: {source}")
            }
        }
    }
}

impl StdError for AuthOrchestrationError {}

pub(super) async fn resolve_identity(
    runtime_components: &RuntimeComponents,
    cfg: &mut ConfigBag,
) -> Result<(AuthSchemeId, Identity, Option<Endpoint>), BoxError> {
    let params = cfg
        .load::<AuthSchemeOptionResolverParams>()
        .expect("auth scheme option resolver params must be set");
    let option_resolver = runtime_components.auth_scheme_option_resolver();
    let options = option_resolver
        .resolve_auth_scheme_options_v2(params, cfg, runtime_components)
        .await?;
    let options =
        reprioritize_with_auth_scheme_preference(options, cfg.load::<AuthSchemePreference>()).await;

    trace!(
        auth_scheme_option_resolver_params = ?params,
        auth_scheme_options = ?options,
        "orchestrating auth",
    );

    let mut explored = ExploredList::default();

    // Iterate over IDs of possibly-supported auth schemes
    for auth_scheme_option in &options {
        let scheme_id = auth_scheme_option.scheme_id();
        // For each ID, try to resolve the corresponding auth scheme.
        if let Some(auth_scheme) = runtime_components.auth_scheme(scheme_id) {
            // Use the resolved auth scheme to resolve an identity
            if let Some(identity_resolver) = auth_scheme.identity_resolver(runtime_components) {
                match legacy_try_resolve_endpoint(runtime_components, cfg, scheme_id).await {
                    Ok(endpoint) => {
                        trace!(scheme_id= ?scheme_id, "resolving identity");
                        let identity_cache = if identity_resolver.cache_location()
                            == IdentityCacheLocation::RuntimeComponents
                        {
                            runtime_components.identity_cache()
                        } else {
                            IdentityCache::no_cache()
                        };
                        // Apply properties from the selected auth scheme option
                        if let Some(properties) = auth_scheme_option.properties() {
                            cfg.push_shared_layer(properties);
                        }
                        let identity = identity_cache
                            .resolve_cached_identity(identity_resolver, runtime_components, cfg)
                            .await?;
                        trace!(identity = ?identity, "resolved identity");
                        // Extract the FrozenLayer placed in the Identity property bag by the From<Credentials> impl.
                        // This layer contains feature data for the user agent and potentially other metadata.
                        if let Some(layer) = identity.property::<FrozenLayer>().cloned() {
                            cfg.push_shared_layer(layer);
                        }
                        return Ok((scheme_id.clone(), identity, endpoint));
                    }
                    Err(AuthOrchestrationError::MissingEndpointConfig) => {
                        explored.push(scheme_id.clone(), ExploreResult::MissingEndpointConfig);
                        continue;
                    }
                    Err(AuthOrchestrationError::FailedToResolveEndpoint(source)) => {
                        // Some negative endpoint tests expect an endpoint resolution error,
                        // so we need to return it to satisfy them.
                        return Err(source);
                    }
                    Err(other_err) => {
                        return Err(other_err.into());
                    }
                }
            } else {
                explored.push(scheme_id.clone(), ExploreResult::NoIdentityResolver);
            }
        } else {
            explored.push(scheme_id.clone(), ExploreResult::NoAuthScheme);
        }
    }

    Err(NoMatchingAuthSchemeError(explored).into())
}

// Re-prioritize `supported_auth_scheme_options` based on `auth_scheme_preference`
//
// Schemes in `auth_scheme_preference` that are not present in `supported_auth_scheme_options` will be ignored.
async fn reprioritize_with_auth_scheme_preference(
    supported_auth_scheme_options: Vec<AuthSchemeOption>,
    auth_scheme_preference: Option<&AuthSchemePreference>,
) -> Vec<AuthSchemeOption> {
    match auth_scheme_preference {
        Some(preference) => {
            // maps auth scheme ID to the index in the preference list
            let preference_map: HashMap<_, _> = preference
                .clone()
                .into_iter()
                .enumerate()
                .map(|(i, s)| (s, i))
                .collect();
            let (mut preferred, non_preferred): (Vec<_>, Vec<_>) = supported_auth_scheme_options
                .into_iter()
                .partition(|auth_scheme_option| {
                    preference_map.contains_key(auth_scheme_option.scheme_id())
                });

            preferred.sort_by_key(|opt| {
                preference_map
                    .get(opt.scheme_id())
                    .expect("guaranteed by `partition`")
            });
            preferred.extend(non_preferred);
            preferred
        }
        None => supported_auth_scheme_options,
    }
}

pub(super) fn sign_request(
    scheme_id: &AuthSchemeId,
    identity: &Identity,
    ctx: &mut InterceptorContext,
    runtime_components: &RuntimeComponents,
    cfg: &ConfigBag,
) -> Result<(), BoxError> {
    trace!("signing request");
    let request = ctx.request_mut().expect("set during serialization");
    let endpoint = cfg
        .load::<Endpoint>()
        .expect("endpoint added to config bag by endpoint orchestrator");
    let auth_scheme = runtime_components
        .auth_scheme(scheme_id)
        .ok_or("should be configured")?;
    let signer = auth_scheme.signer();
    let auth_scheme_endpoint_config = extract_endpoint_auth_scheme_config(endpoint, scheme_id)?;
    trace!(
        signer = ?signer,
        "signing implementation"
    );
    signer.sign_http_request(
        request,
        identity,
        auth_scheme_endpoint_config,
        runtime_components,
        cfg,
    )?;
    Ok(())
}

// Marker indicating the correct resolution order: auth scheme resolution, identity resolution,
// and then endpoint resolution, as specified in the SRA.
//
// This marker is included in the config bag to signify the intended resolution order
// by design. When the crate was released for GA, the resolution order was reversed
// (endpoint resolution first, followed by identity resolution). However, we later
// discovered the order needed correcting without breaking existing SDKs.
// This marker signals the runtime to support both resolution orders without introducing
// `aws-smithy-runtime` version 2.x.
//
// When this marker is present in the config bag (the default behavior for forward compatibility),
// `resolve_identity` skips endpoint resolution, ensuring that `try_attempt` follows
// the correct resolution order: auth scheme → identity → endpoint.
// If the marker is absent, `try_attempt` continues using
// the legacy, incorrect resolution order: endpoint → auth scheme → identity.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct AuthSchemeAndEndpointOrchestrationV2;

impl Storable for AuthSchemeAndEndpointOrchestrationV2 {
    type Storer = StoreReplace<Self>;
}

// Conditionally return an `endpoint` resolved by `SharedEndpointResolver` in `runtime_components`
// whose `authSchemes` property matches the given `scheme_id`
//
// Return the `Ok` variant in the following cases:
// - If `AuthSchemeAndEndpointOrchestrationV2` is present in the config bag, indicating that the runtime doesn't require the functionality of this function.
//   In this case, the function short-circuits and returns `Ok(None)`, as no endpoint needs resolution. Instead, a custom `AuthSchemeOptionResolver`
//   should have resolved it internally for auth scheme selection.
// - If the runtime uses the legacy auth scheme and endpoint orchestration and resolves an endpoint that matches the given condition,
//   the function returns `Ok(endpoint)`, which will then be applied to the request.
async fn legacy_try_resolve_endpoint(
    runtime_components: &RuntimeComponents,
    cfg: &ConfigBag,
    scheme_id: &AuthSchemeId,
) -> Result<Option<Endpoint>, AuthOrchestrationError> {
    if cfg.load::<AuthSchemeAndEndpointOrchestrationV2>().is_some() {
        // The orchestrator uses the correct auth scheme and endpoint resolution order,
        // and no endpoint needs to be resolved within this function.
        return Ok(None);
    }

    let params = cfg
        .load::<EndpointResolverParams>()
        .expect("endpoint resolver params must be set");

    tracing::debug!(scheme_id = ?scheme_id, endpoint_params = ?params, "using legacy auth and endpoint orchestration, resolving endpoint for auth scheme selection");

    let endpoint = runtime_components
        .endpoint_resolver()
        .resolve_endpoint(params)
        .await
        .map_err(AuthOrchestrationError::FailedToResolveEndpoint)?;

    // This line repurposes `extract_endpoint_auth_scheme_config` to check whether
    // the function returns `Ok` for the given `endpoint`.
    // Essentially, we verify if the `authSchemes` property of `endpoint` contains `scheme_id`,
    // which is done by `schemes.iter().find(...).ok_or(...)` within the function.
    // However, this execution path is only exercised for legacy auth scheme and endpoint orchestration,
    // so we don't bother refactoring the predicate out of the function.
    let _ = extract_endpoint_auth_scheme_config(&endpoint, scheme_id)?;

    Ok(Some(endpoint))
}

fn extract_endpoint_auth_scheme_config<'a>(
    endpoint: &'a Endpoint,
    scheme_id: &AuthSchemeId,
) -> Result<AuthSchemeEndpointConfig<'a>, AuthOrchestrationError> {
    // TODO(P96049742): Endpoint config doesn't currently have a concept of optional auth or "no auth", so
    // we are short-circuiting lookup of endpoint auth scheme config if that is the selected scheme.
    if scheme_id == &NO_AUTH_SCHEME_ID {
        return Ok(AuthSchemeEndpointConfig::empty());
    }
    let auth_schemes = match endpoint.properties().get("authSchemes") {
        Some(Document::Array(schemes)) => schemes,
        // no auth schemes:
        None => return Ok(AuthSchemeEndpointConfig::empty()),
        _other => {
            return Err(AuthOrchestrationError::BadAuthSchemeEndpointConfig(
                "expected an array for `authSchemes` in endpoint config".into(),
            ))
        }
    };
    let auth_scheme_config = auth_schemes
        .iter()
        .find(|doc| {
            let config_scheme_id = doc
                .as_object()
                .and_then(|object| object.get("name"))
                .and_then(Document::as_string);
            config_scheme_id == Some(scheme_id.inner())
        })
        .ok_or(AuthOrchestrationError::MissingEndpointConfig)?;
    Ok(AuthSchemeEndpointConfig::from(Some(auth_scheme_config)))
}

#[derive(Debug)]
enum ExploreResult {
    NotExplored,
    NoAuthScheme,
    NoIdentityResolver,
    MissingEndpointConfig,
}

/// Information about an evaluated auth option.
/// This should be kept small so it can fit in an array on the stack.
#[derive(Debug)]
struct ExploredAuthOption {
    scheme_id: AuthSchemeId,
    result: ExploreResult,
}
impl Default for ExploredAuthOption {
    fn default() -> Self {
        Self {
            scheme_id: AuthSchemeId::new(""),
            result: ExploreResult::NotExplored,
        }
    }
}

const MAX_EXPLORED_LIST_LEN: usize = 8;

/// Stack allocated list of explored auth options for error messaging
#[derive(Default)]
struct ExploredList {
    items: [ExploredAuthOption; MAX_EXPLORED_LIST_LEN],
    len: usize,
    truncated: bool,
}
impl ExploredList {
    fn items(&self) -> impl Iterator<Item = &ExploredAuthOption> {
        self.items.iter().take(self.len)
    }

    fn push(&mut self, scheme_id: AuthSchemeId, result: ExploreResult) {
        if self.len + 1 >= self.items.len() {
            self.truncated = true;
        } else {
            self.items[self.len] = ExploredAuthOption { scheme_id, result };
            self.len += 1;
        }
    }
}
impl fmt::Debug for ExploredList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExploredList")
            .field("items", &&self.items[0..self.len])
            .field("truncated", &self.truncated)
            .finish()
    }
}

#[cfg(all(test, any(feature = "test-util", feature = "legacy-test-util")))]
mod tests {
    use super::*;
    use crate::client::orchestrator::endpoints::{
        StaticUriEndpointResolver, StaticUriEndpointResolverParams,
    };
    use aws_smithy_runtime_api::client::auth::static_resolver::StaticAuthSchemeOptionResolver;
    use aws_smithy_runtime_api::client::auth::{
        AuthScheme, AuthSchemeId, AuthSchemeOptionResolverParams, SharedAuthScheme,
        SharedAuthSchemeOptionResolver, Sign,
    };
    use aws_smithy_runtime_api::client::endpoint::SharedEndpointResolver;
    use aws_smithy_runtime_api::client::identity::{
        Identity, IdentityFuture, ResolveIdentity, SharedIdentityResolver,
    };
    use aws_smithy_runtime_api::client::interceptors::context::{Input, InterceptorContext};
    use aws_smithy_runtime_api::client::orchestrator::HttpRequest;
    use aws_smithy_runtime_api::client::runtime_components::{
        GetIdentityResolver, RuntimeComponents, RuntimeComponentsBuilder,
    };
    use aws_smithy_types::config_bag::Layer;
    use std::collections::HashMap;

    #[tokio::test]
    async fn basic_case() {
        #[derive(Debug)]
        struct TestIdentityResolver;
        impl ResolveIdentity for TestIdentityResolver {
            fn resolve_identity<'a>(
                &'a self,
                _runtime_components: &'a RuntimeComponents,
                _config_bag: &'a ConfigBag,
            ) -> IdentityFuture<'a> {
                IdentityFuture::ready(Ok(Identity::new("doesntmatter", None)))
            }
        }

        #[derive(Debug)]
        struct TestSigner;

        impl Sign for TestSigner {
            fn sign_http_request(
                &self,
                request: &mut HttpRequest,
                _identity: &Identity,
                _auth_scheme_endpoint_config: AuthSchemeEndpointConfig<'_>,
                _runtime_components: &RuntimeComponents,
                _config_bag: &ConfigBag,
            ) -> Result<(), BoxError> {
                request
                    .headers_mut()
                    .insert(http_1x::header::AUTHORIZATION, "success!");
                Ok(())
            }
        }

        const TEST_SCHEME_ID: AuthSchemeId = AuthSchemeId::new("test-scheme");

        #[derive(Debug)]
        struct TestAuthScheme {
            signer: TestSigner,
        }
        impl AuthScheme for TestAuthScheme {
            fn scheme_id(&self) -> AuthSchemeId {
                TEST_SCHEME_ID
            }

            fn identity_resolver(
                &self,
                identity_resolvers: &dyn GetIdentityResolver,
            ) -> Option<SharedIdentityResolver> {
                identity_resolvers.identity_resolver(self.scheme_id())
            }

            fn signer(&self) -> &dyn Sign {
                &self.signer
            }
        }

        async fn run_test(add_more_to_layer: impl Fn(Layer) -> Layer) {
            let mut ctx = InterceptorContext::new(Input::doesnt_matter());
            ctx.enter_serialization_phase();
            ctx.set_request(HttpRequest::empty());
            let _ = ctx.take_input();
            ctx.enter_before_transmit_phase();

            let runtime_components = RuntimeComponentsBuilder::for_tests()
                .with_auth_scheme(SharedAuthScheme::new(TestAuthScheme { signer: TestSigner }))
                .with_auth_scheme_option_resolver(Some(SharedAuthSchemeOptionResolver::new(
                    StaticAuthSchemeOptionResolver::new(vec![TEST_SCHEME_ID]),
                )))
                .with_identity_resolver(
                    TEST_SCHEME_ID,
                    SharedIdentityResolver::new(TestIdentityResolver),
                )
                .with_endpoint_resolver(Some(SharedEndpointResolver::new(
                    StaticUriEndpointResolver::http_localhost(8080),
                )))
                .build()
                .unwrap();

            let mut layer: Layer = Layer::new("test");
            layer.store_put(AuthSchemeOptionResolverParams::new("doesntmatter"));
            layer.store_put(Endpoint::builder().url("dontcare").build());
            let layer = add_more_to_layer(layer);
            let mut cfg = ConfigBag::of_layers(vec![layer]);

            let (scheme_id, identity, _) = resolve_identity(&runtime_components, &mut cfg)
                .await
                .expect("success");

            sign_request(&scheme_id, &identity, &mut ctx, &runtime_components, &cfg)
                .expect("success");

            assert_eq!(
                "success!",
                ctx.request()
                    .expect("request is set")
                    .headers()
                    .get("Authorization")
                    .unwrap()
            );
        }

        // test for correct auth and endpoint orchestration
        run_test(|mut layer| {
            layer.store_put(AuthSchemeAndEndpointOrchestrationV2);
            layer
        })
        .await;

        // test for legacy auth and endpoint orchestration (no `AuthSchemeAndEndpointOrchestrationV2` in `layer`)
        run_test(|mut layer| {
            layer.store_put(EndpointResolverParams::from(
                StaticUriEndpointResolverParams::new(),
            ));
            layer
        })
        .await;
    }

    #[cfg(feature = "http-auth")]
    #[tokio::test]
    async fn select_best_scheme_for_available_identity_resolvers() {
        use crate::client::auth::http::{BasicAuthScheme, BearerAuthScheme};
        use aws_smithy_runtime_api::client::auth::http::{
            HTTP_BASIC_AUTH_SCHEME_ID, HTTP_BEARER_AUTH_SCHEME_ID,
        };
        use aws_smithy_runtime_api::client::identity::http::{Login, Token};

        async fn run_test(add_more_to_layer: impl Fn(Layer) -> Layer) {
            let mut ctx = InterceptorContext::new(Input::doesnt_matter());
            ctx.enter_serialization_phase();
            ctx.set_request(HttpRequest::empty());
            let _ = ctx.take_input();
            ctx.enter_before_transmit_phase();

            // First, test the presence of a basic auth login and absence of a bearer token
            let (runtime_components, layer) =
                config_with_identity(HTTP_BASIC_AUTH_SCHEME_ID, Login::new("a", "b", None));
            let layer = add_more_to_layer(layer);
            let mut cfg = ConfigBag::of_layers(vec![layer]);

            let (scheme_id, identity, _) = resolve_identity(&runtime_components, &mut cfg)
                .await
                .expect("success");
            sign_request(&scheme_id, &identity, &mut ctx, &runtime_components, &cfg)
                .expect("success");
            assert_eq!(
                // "YTpi" == "a:b" in base64
                "Basic YTpi",
                ctx.request()
                    .expect("request is set")
                    .headers()
                    .get("Authorization")
                    .unwrap()
            );

            // Next, test the presence of a bearer token and absence of basic auth
            let (runtime_components, layer) =
                config_with_identity(HTTP_BEARER_AUTH_SCHEME_ID, Token::new("t", None));
            let layer = add_more_to_layer(layer);
            let mut cfg = ConfigBag::of_layers(vec![layer]);
            let mut ctx = InterceptorContext::new(Input::erase("doesnt-matter"));
            ctx.enter_serialization_phase();
            ctx.set_request(HttpRequest::empty());
            let _ = ctx.take_input();
            ctx.enter_before_transmit_phase();
            let (scheme_id, identity, _) = resolve_identity(&runtime_components, &mut cfg)
                .await
                .expect("success");
            sign_request(&scheme_id, &identity, &mut ctx, &runtime_components, &cfg)
                .expect("success");
            assert_eq!(
                "Bearer t",
                ctx.request()
                    .expect("request is set")
                    .headers()
                    .get("Authorization")
                    .unwrap()
            );
        }

        fn config_with_identity(
            scheme_id: AuthSchemeId,
            identity: impl ResolveIdentity + 'static,
        ) -> (RuntimeComponents, Layer) {
            let runtime_components = RuntimeComponentsBuilder::for_tests()
                .with_auth_scheme(SharedAuthScheme::new(BasicAuthScheme::new()))
                .with_auth_scheme(SharedAuthScheme::new(BearerAuthScheme::new()))
                .with_auth_scheme_option_resolver(Some(SharedAuthSchemeOptionResolver::new(
                    StaticAuthSchemeOptionResolver::new(vec![
                        HTTP_BASIC_AUTH_SCHEME_ID,
                        HTTP_BEARER_AUTH_SCHEME_ID,
                    ]),
                )))
                .with_identity_resolver(scheme_id, SharedIdentityResolver::new(identity))
                .with_endpoint_resolver(Some(SharedEndpointResolver::new(
                    StaticUriEndpointResolver::http_localhost(8080),
                )))
                .build()
                .unwrap();

            let mut layer = Layer::new("test");
            layer.store_put(Endpoint::builder().url("dontcare").build());
            layer.store_put(AuthSchemeOptionResolverParams::new("doesntmatter"));

            (runtime_components, layer)
        }

        // test for correct auth and endpoint orchestration
        run_test(|mut layer| {
            layer.store_put(AuthSchemeAndEndpointOrchestrationV2);
            layer
        })
        .await;

        // test for legacy auth and endpoint orchestration (no `AuthSchemeAndEndpointOrchestrationV2` in `layer`)
        run_test(|mut layer| {
            layer.store_put(EndpointResolverParams::from(
                StaticUriEndpointResolverParams::new(),
            ));
            layer
        })
        .await;
    }

    #[test]
    fn extract_endpoint_auth_scheme_config_no_config() {
        let endpoint = Endpoint::builder()
            .url("dontcare")
            .property("something-unrelated", Document::Null)
            .build();
        let config =
            extract_endpoint_auth_scheme_config(&endpoint, &AuthSchemeId::from("test-scheme-id"))
                .expect("success");
        assert!(config.as_document().is_none());
    }

    #[test]
    fn extract_endpoint_auth_scheme_config_wrong_type() {
        let endpoint = Endpoint::builder()
            .url("dontcare")
            .property("authSchemes", Document::String("bad".into()))
            .build();
        extract_endpoint_auth_scheme_config(&endpoint, &AuthSchemeId::from("test-scheme-id"))
            .expect_err("should fail because authSchemes is the wrong type");
    }

    #[test]
    fn extract_endpoint_auth_scheme_config_no_matching_scheme() {
        let endpoint = Endpoint::builder()
            .url("dontcare")
            .property(
                "authSchemes",
                vec![
                    Document::Object({
                        let mut out = HashMap::new();
                        out.insert("name".to_string(), "wrong-scheme-id".to_string().into());
                        out
                    }),
                    Document::Object({
                        let mut out = HashMap::new();
                        out.insert(
                            "name".to_string(),
                            "another-wrong-scheme-id".to_string().into(),
                        );
                        out
                    }),
                ],
            )
            .build();
        extract_endpoint_auth_scheme_config(&endpoint, &AuthSchemeId::from("test-scheme-id"))
            .expect_err("should fail because authSchemes doesn't include the desired scheme");
    }

    #[test]
    fn extract_endpoint_auth_scheme_config_successfully() {
        let endpoint = Endpoint::builder()
            .url("dontcare")
            .property(
                "authSchemes",
                vec![
                    Document::Object({
                        let mut out = HashMap::new();
                        out.insert("name".to_string(), "wrong-scheme-id".to_string().into());
                        out
                    }),
                    Document::Object({
                        let mut out = HashMap::new();
                        out.insert("name".to_string(), "test-scheme-id".to_string().into());
                        out.insert(
                            "magicString".to_string(),
                            "magic string value".to_string().into(),
                        );
                        out
                    }),
                ],
            )
            .build();
        let config =
            extract_endpoint_auth_scheme_config(&endpoint, &AuthSchemeId::from("test-scheme-id"))
                .expect("should find test-scheme-id");
        assert_eq!(
            "magic string value",
            config
                .as_document()
                .expect("config is set")
                .as_object()
                .expect("it's an object")
                .get("magicString")
                .expect("magicString is set")
                .as_string()
                .expect("gimme the string, dammit!")
        );
    }

    #[cfg(feature = "http-auth")]
    #[tokio::test]
    async fn use_identity_cache() {
        use crate::client::auth::http::{ApiKeyAuthScheme, ApiKeyLocation};
        use aws_smithy_runtime_api::client::auth::http::HTTP_API_KEY_AUTH_SCHEME_ID;
        use aws_smithy_runtime_api::client::identity::http::Token;
        use aws_smithy_types::body::SdkBody;

        #[derive(Debug)]
        struct Cache;
        impl ResolveCachedIdentity for Cache {
            fn resolve_cached_identity<'a>(
                &'a self,
                _resolver: SharedIdentityResolver,
                _: &'a RuntimeComponents,
                _config_bag: &'a ConfigBag,
            ) -> IdentityFuture<'a> {
                IdentityFuture::ready(Ok(Identity::new(Token::new("cached (pass)", None), None)))
            }
        }

        async fn run_test(add_more_to_layer: impl Fn(Layer) -> Layer) {
            let mut ctx = InterceptorContext::new(Input::doesnt_matter());
            ctx.enter_serialization_phase();
            ctx.set_request(
                http_1x::Request::builder()
                    .body(SdkBody::empty())
                    .unwrap()
                    .try_into()
                    .unwrap(),
            );
            let _ = ctx.take_input();
            ctx.enter_before_transmit_phase();

            let runtime_components = RuntimeComponentsBuilder::for_tests()
                .with_auth_scheme(SharedAuthScheme::new(ApiKeyAuthScheme::new(
                    "result:",
                    ApiKeyLocation::Header,
                    "Authorization",
                )))
                .with_auth_scheme_option_resolver(Some(SharedAuthSchemeOptionResolver::new(
                    StaticAuthSchemeOptionResolver::new(vec![HTTP_API_KEY_AUTH_SCHEME_ID]),
                )))
                .with_identity_cache(Some(Cache))
                .with_identity_resolver(
                    HTTP_API_KEY_AUTH_SCHEME_ID,
                    SharedIdentityResolver::new(Token::new("uncached (fail)", None)),
                )
                .with_endpoint_resolver(Some(SharedEndpointResolver::new(
                    StaticUriEndpointResolver::http_localhost(8080),
                )))
                .build()
                .unwrap();
            let mut layer = Layer::new("test");
            layer.store_put(Endpoint::builder().url("dontcare").build());
            layer.store_put(AuthSchemeOptionResolverParams::new("doesntmatter"));
            let layer = add_more_to_layer(layer);
            let mut config_bag = ConfigBag::of_layers(vec![layer]);

            let (scheme_id, identity, _) = resolve_identity(&runtime_components, &mut config_bag)
                .await
                .expect("success");
            sign_request(
                &scheme_id,
                &identity,
                &mut ctx,
                &runtime_components,
                &config_bag,
            )
            .expect("success");
            assert_eq!(
                "result: cached (pass)",
                ctx.request()
                    .expect("request is set")
                    .headers()
                    .get("Authorization")
                    .unwrap()
            );
        }

        // test for correct auth and endpoint orchestration
        run_test(|mut layer| {
            layer.store_put(AuthSchemeAndEndpointOrchestrationV2);
            layer
        })
        .await;

        // test for legacy auth and endpoint orchestration (no `AuthSchemeAndEndpointOrchestrationV2` in `layer`)
        run_test(|mut layer| {
            layer.store_put(EndpointResolverParams::from(
                StaticUriEndpointResolverParams::new(),
            ));
            layer
        })
        .await;
    }

    #[test]
    fn friendly_error_messages() {
        let err = NoMatchingAuthSchemeError(ExploredList::default());
        assert_eq!(
            "no auth options are available. This can happen if there's a problem with \
            the service model, or if there is a codegen bug.",
            err.to_string()
        );

        let mut list = ExploredList::default();
        list.push(
            AuthSchemeId::new("SigV4"),
            ExploreResult::NoIdentityResolver,
        );
        list.push(
            AuthSchemeId::new("SigV4a"),
            ExploreResult::NoIdentityResolver,
        );
        let err = NoMatchingAuthSchemeError(list);
        assert_eq!(
            "failed to select an auth scheme to sign the request with. \
            \"SigV4\" wasn't a valid option because there was no identity resolver for it. \
            \"SigV4a\" wasn't a valid option because there was no identity resolver for it. \
            Be sure to set an identity, such as credentials, auth token, or other identity \
            type that is required for this service. \
            If you intended to make an unauthenticated request, consider using `@optionalAuth` or `@auth([])` \
            on the operation in the service model and regenerating the client SDK. If modifying the model is not possible, \
            you can disable authentication at runtime as described in https://github.com/smithy-lang/smithy-rs/discussions/4197.",
            err.to_string()
        );

        // It should prioritize the suggestion to try an identity before saying it's a bug
        let mut list = ExploredList::default();
        list.push(
            AuthSchemeId::new("SigV4"),
            ExploreResult::NoIdentityResolver,
        );
        list.push(
            AuthSchemeId::new("SigV4a"),
            ExploreResult::MissingEndpointConfig,
        );
        let err = NoMatchingAuthSchemeError(list);
        assert_eq!(
            "failed to select an auth scheme to sign the request with. \
            \"SigV4\" wasn't a valid option because there was no identity resolver for it. \
            \"SigV4a\" wasn't a valid option because there is auth config in the endpoint \
            config, but this scheme wasn't listed in it (see \
            https://github.com/smithy-lang/smithy-rs/discussions/3281 for more details). \
            Be sure to set an identity, such as credentials, auth token, or other identity \
            type that is required for this service.",
            err.to_string()
        );

        // Otherwise, it should suggest it's a bug
        let mut list = ExploredList::default();
        list.push(
            AuthSchemeId::new("SigV4a"),
            ExploreResult::MissingEndpointConfig,
        );
        let err = NoMatchingAuthSchemeError(list);
        assert_eq!(
            "failed to select an auth scheme to sign the request with. \
            \"SigV4a\" wasn't a valid option because there is auth config in the endpoint \
            config, but this scheme wasn't listed in it (see \
            https://github.com/smithy-lang/smithy-rs/discussions/3281 for more details). \
            This is likely a bug.",
            err.to_string()
        );

        // Truncation should be indicated
        let mut list = ExploredList::default();
        for _ in 0..=MAX_EXPLORED_LIST_LEN {
            list.push(
                AuthSchemeId::new("dontcare"),
                ExploreResult::MissingEndpointConfig,
            );
        }
        let err = NoMatchingAuthSchemeError(list).to_string();
        if !err.contains(
            "Note: there were other auth schemes that were evaluated that weren't listed here",
        ) {
            panic!("The error should indicate that the explored list was truncated.");
        }
    }

    #[cfg(feature = "http-auth")]
    #[tokio::test]
    async fn test_resolve_identity() {
        use crate::client::auth::http::{ApiKeyAuthScheme, ApiKeyLocation, BasicAuthScheme};
        use aws_smithy_runtime_api::client::auth::http::{
            HTTP_API_KEY_AUTH_SCHEME_ID, HTTP_BASIC_AUTH_SCHEME_ID,
        };
        use aws_smithy_runtime_api::client::identity::http::Token;
        use aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder;

        #[derive(Debug)]
        struct Cache;
        impl ResolveCachedIdentity for Cache {
            fn resolve_cached_identity<'a>(
                &'a self,
                identity_resolver: SharedIdentityResolver,
                rc: &'a RuntimeComponents,
                cfg: &'a ConfigBag,
            ) -> IdentityFuture<'a> {
                IdentityFuture::new(
                    async move { identity_resolver.resolve_identity(rc, cfg).await },
                )
            }
        }

        let mut layer = Layer::new("test");
        layer.store_put(AuthSchemeAndEndpointOrchestrationV2);
        layer.store_put(AuthSchemeOptionResolverParams::new("doesntmatter"));
        let mut cfg = ConfigBag::of_layers(vec![layer]);

        let runtime_components_builder = RuntimeComponentsBuilder::for_tests()
            .with_auth_scheme(SharedAuthScheme::new(BasicAuthScheme::new()))
            .with_auth_scheme(SharedAuthScheme::new(ApiKeyAuthScheme::new(
                "result:",
                ApiKeyLocation::Header,
                "Authorization",
            )))
            .with_auth_scheme_option_resolver(Some(SharedAuthSchemeOptionResolver::new(
                StaticAuthSchemeOptionResolver::new(vec![
                    HTTP_BASIC_AUTH_SCHEME_ID,
                    HTTP_API_KEY_AUTH_SCHEME_ID,
                ]),
            )))
            .with_identity_cache(Some(Cache));

        struct TestCase {
            builder_updater: Box<dyn Fn(RuntimeComponentsBuilder) -> RuntimeComponents>,
            resolved_auth_scheme: AuthSchemeId,
            should_error: bool,
        }

        for test_case in [
            TestCase {
                builder_updater: Box::new(|rcb: RuntimeComponentsBuilder| {
                    rcb.with_identity_resolver(
                        HTTP_BASIC_AUTH_SCHEME_ID,
                        SharedIdentityResolver::new(Token::new("basic", None)),
                    )
                    .with_identity_resolver(
                        HTTP_API_KEY_AUTH_SCHEME_ID,
                        SharedIdentityResolver::new(Token::new("api-key", None)),
                    )
                    .build()
                    .unwrap()
                }),
                resolved_auth_scheme: HTTP_BASIC_AUTH_SCHEME_ID,
                should_error: false,
            },
            TestCase {
                builder_updater: Box::new(|rcb: RuntimeComponentsBuilder| {
                    rcb.with_identity_resolver(
                        HTTP_BASIC_AUTH_SCHEME_ID,
                        SharedIdentityResolver::new(Token::new("basic", None)),
                    )
                    .build()
                    .unwrap()
                }),
                resolved_auth_scheme: HTTP_BASIC_AUTH_SCHEME_ID,
                should_error: false,
            },
            TestCase {
                builder_updater: Box::new(|rcb: RuntimeComponentsBuilder| {
                    rcb.with_identity_resolver(
                        HTTP_API_KEY_AUTH_SCHEME_ID,
                        SharedIdentityResolver::new(Token::new("api-key", None)),
                    )
                    .build()
                    .unwrap()
                }),
                resolved_auth_scheme: HTTP_API_KEY_AUTH_SCHEME_ID,
                should_error: false,
            },
            TestCase {
                builder_updater: Box::new(|rcb: RuntimeComponentsBuilder| rcb.build().unwrap()),
                resolved_auth_scheme: HTTP_API_KEY_AUTH_SCHEME_ID,
                should_error: true,
            },
        ]
        .into_iter()
        {
            let runtime_components =
                (test_case.builder_updater)(runtime_components_builder.clone());
            match resolve_identity(&runtime_components, &mut cfg).await {
                Ok(resolved) => assert_eq!(test_case.resolved_auth_scheme, resolved.0),
                Err(e) if test_case.should_error => {
                    assert!(e.downcast_ref::<NoMatchingAuthSchemeError>().is_some());
                }
                _ => {
                    panic!("`resolve_identity` returned an `Err` when no error was expected in the test.");
                }
            }
        }
    }

    #[cfg(feature = "http-auth")]
    #[tokio::test]
    async fn auth_scheme_preference() {
        use aws_smithy_runtime_api::client::auth::http::{
            HTTP_API_KEY_AUTH_SCHEME_ID, HTTP_BASIC_AUTH_SCHEME_ID, HTTP_BEARER_AUTH_SCHEME_ID,
        };

        struct TestCase {
            supported: Vec<AuthSchemeOption>,
            preference: Option<AuthSchemePreference>,
            expected_resolved_auths: Vec<AuthSchemeId>,
        }

        for test_case in [
            TestCase {
                supported: [HTTP_API_KEY_AUTH_SCHEME_ID, HTTP_BASIC_AUTH_SCHEME_ID]
                    .map(AuthSchemeOption::from)
                    .to_vec(),
                preference: None,
                expected_resolved_auths: vec![
                    HTTP_API_KEY_AUTH_SCHEME_ID,
                    HTTP_BASIC_AUTH_SCHEME_ID,
                ],
            },
            TestCase {
                supported: [HTTP_API_KEY_AUTH_SCHEME_ID, HTTP_BASIC_AUTH_SCHEME_ID]
                    .map(AuthSchemeOption::from)
                    .to_vec(),
                preference: Some([].into()),
                expected_resolved_auths: vec![
                    HTTP_API_KEY_AUTH_SCHEME_ID,
                    HTTP_BASIC_AUTH_SCHEME_ID,
                ],
            },
            TestCase {
                supported: [HTTP_API_KEY_AUTH_SCHEME_ID, HTTP_BASIC_AUTH_SCHEME_ID]
                    .map(AuthSchemeOption::from)
                    .to_vec(),
                preference: Some(["bogus"].map(AuthSchemeId::from).into()),
                expected_resolved_auths: vec![
                    HTTP_API_KEY_AUTH_SCHEME_ID,
                    HTTP_BASIC_AUTH_SCHEME_ID,
                ],
            },
            TestCase {
                supported: [HTTP_API_KEY_AUTH_SCHEME_ID, HTTP_BASIC_AUTH_SCHEME_ID]
                    .map(AuthSchemeOption::from)
                    .to_vec(),
                preference: Some([HTTP_BASIC_AUTH_SCHEME_ID].into()),
                expected_resolved_auths: vec![
                    HTTP_BASIC_AUTH_SCHEME_ID,
                    HTTP_API_KEY_AUTH_SCHEME_ID,
                ],
            },
            TestCase {
                supported: [HTTP_API_KEY_AUTH_SCHEME_ID, HTTP_BASIC_AUTH_SCHEME_ID]
                    .map(AuthSchemeOption::from)
                    .to_vec(),
                preference: Some([HTTP_BASIC_AUTH_SCHEME_ID, HTTP_API_KEY_AUTH_SCHEME_ID].into()),
                expected_resolved_auths: vec![
                    HTTP_BASIC_AUTH_SCHEME_ID,
                    HTTP_API_KEY_AUTH_SCHEME_ID,
                ],
            },
            TestCase {
                supported: [HTTP_API_KEY_AUTH_SCHEME_ID, HTTP_BASIC_AUTH_SCHEME_ID]
                    .map(AuthSchemeOption::from)
                    .to_vec(),
                preference: Some(
                    [
                        HTTP_BASIC_AUTH_SCHEME_ID,
                        HTTP_BEARER_AUTH_SCHEME_ID,
                        HTTP_API_KEY_AUTH_SCHEME_ID,
                    ]
                    .into(),
                ),
                expected_resolved_auths: vec![
                    HTTP_BASIC_AUTH_SCHEME_ID,
                    HTTP_API_KEY_AUTH_SCHEME_ID,
                ],
            },
        ] {
            let actual = reprioritize_with_auth_scheme_preference(
                test_case.supported,
                test_case.preference.as_ref(),
            )
            .await;
            let actual = actual
                .iter()
                .map(|opt| opt.scheme_id())
                .cloned()
                .collect::<Vec<_>>();
            assert_eq!(test_case.expected_resolved_auths, actual);
        }
    }
}
