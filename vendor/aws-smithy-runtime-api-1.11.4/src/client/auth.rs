/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! APIs for request authentication.

use crate::box_error::BoxError;
use crate::client::identity::{Identity, SharedIdentityResolver};
use crate::client::orchestrator::HttpRequest;
use crate::client::runtime_components::sealed::ValidateConfig;
use crate::client::runtime_components::{GetIdentityResolver, RuntimeComponents};
use crate::impl_shared_conversions;
use aws_smithy_types::config_bag::{ConfigBag, FrozenLayer, Storable, StoreReplace};
use aws_smithy_types::type_erasure::TypeErasedBox;
use aws_smithy_types::Document;
use std::borrow::Cow;
use std::fmt;
use std::sync::Arc;

/// Auth schemes for the HTTP `Authorization` header.
#[cfg(feature = "http-auth")]
pub mod http;

/// Static auth scheme option resolver.
pub mod static_resolver;

/// The output type from the [`ResolveAuthSchemeOptions::resolve_auth_scheme_options_v2`]
///
/// The resolver returns a list of these, in the order the auth scheme resolver wishes to use them.
#[derive(Clone, Debug)]
pub struct AuthSchemeOption {
    scheme_id: AuthSchemeId,
    properties: Option<FrozenLayer>,
}

impl AuthSchemeOption {
    /// Builder struct for [`AuthSchemeOption`]
    pub fn builder() -> AuthSchemeOptionBuilder {
        AuthSchemeOptionBuilder::default()
    }

    /// Returns [`AuthSchemeId`], the ID of the scheme
    pub fn scheme_id(&self) -> &AuthSchemeId {
        &self.scheme_id
    }

    /// Returns optional properties for identity resolution or signing
    ///
    /// This config layer is applied to the [`ConfigBag`] to ensure the information is
    /// available during both the identity resolution and signature generation processes.
    pub fn properties(&self) -> Option<FrozenLayer> {
        self.properties.clone()
    }
}

impl From<AuthSchemeId> for AuthSchemeOption {
    fn from(auth_scheme_id: AuthSchemeId) -> Self {
        AuthSchemeOption::builder()
            .scheme_id(auth_scheme_id)
            .build()
            .expect("required fields set")
    }
}

/// Builder struct for [`AuthSchemeOption`]
#[derive(Debug, Default)]
pub struct AuthSchemeOptionBuilder {
    scheme_id: Option<AuthSchemeId>,
    properties: Option<FrozenLayer>,
}

impl AuthSchemeOptionBuilder {
    /// Sets [`AuthSchemeId`] for the builder
    pub fn scheme_id(mut self, auth_scheme_id: AuthSchemeId) -> Self {
        self.set_scheme_id(Some(auth_scheme_id));
        self
    }

    /// Sets [`AuthSchemeId`] for the builder
    pub fn set_scheme_id(&mut self, auth_scheme_id: Option<AuthSchemeId>) {
        self.scheme_id = auth_scheme_id;
    }

    /// Sets the properties for the builder
    pub fn properties(mut self, properties: FrozenLayer) -> Self {
        self.set_properties(Some(properties));
        self
    }

    /// Sets the properties for the builder
    pub fn set_properties(&mut self, properties: Option<FrozenLayer>) {
        self.properties = properties;
    }

    /// Builds an [`AuthSchemeOption`], otherwise returns an [`AuthSchemeOptionBuilderError`] in the case of error
    pub fn build(self) -> Result<AuthSchemeOption, AuthSchemeOptionBuilderError> {
        let scheme_id = self
            .scheme_id
            .ok_or(ErrorKind::MissingRequiredField("auth_scheme_id"))?;
        Ok(AuthSchemeOption {
            scheme_id,
            properties: self.properties,
        })
    }
}

#[derive(Debug)]
enum ErrorKind {
    MissingRequiredField(&'static str),
}

impl From<ErrorKind> for AuthSchemeOptionBuilderError {
    fn from(kind: ErrorKind) -> Self {
        Self { kind }
    }
}

/// The error type returned when failing to build [`AuthSchemeOption`] from the builder
#[derive(Debug)]
pub struct AuthSchemeOptionBuilderError {
    kind: ErrorKind,
}

impl fmt::Display for AuthSchemeOptionBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::MissingRequiredField(name) => {
                write!(f, "`{name}` is required")
            }
        }
    }
}

impl std::error::Error for AuthSchemeOptionBuilderError {}

/// New type around an auth scheme ID.
///
/// Each auth scheme must have a unique string identifier associated with it,
/// which is used to refer to auth schemes by the auth scheme option resolver, and
/// also used to select an identity resolver to use.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct AuthSchemeId {
    scheme_id: Cow<'static, str>,
}

// See: https://doc.rust-lang.org/std/convert/trait.AsRef.html#reflexivity
impl AsRef<AuthSchemeId> for AuthSchemeId {
    fn as_ref(&self) -> &AuthSchemeId {
        self
    }
}

// Normalizes auth scheme IDs for comparison and hashing by treating "no_auth" and "noAuth" as equivalent
// by converting "no_auth" to "noAuth".
// This is for backward compatibility; "no_auth" was incorrectly used in earlier GA versions of the SDK and
// could be used still in some places.
const fn normalize_auth_scheme_id(s: &'static str) -> &'static str {
    match s.as_bytes() {
        b"no_auth" => "noAuth",
        _ => s,
    }
}

impl AuthSchemeId {
    /// Creates a new auth scheme ID.
    pub const fn new(scheme_id: &'static str) -> Self {
        Self {
            scheme_id: Cow::Borrowed(normalize_auth_scheme_id(scheme_id)),
        }
    }

    /// Returns the string equivalent of this auth scheme ID.
    #[deprecated(
        note = "This function is no longer functional. Use `inner` instead",
        since = "1.8.0"
    )]
    pub const fn as_str(&self) -> &'static str {
        match self.scheme_id {
            Cow::Borrowed(val) => val,
            Cow::Owned(_) => {
                // cannot obtain `&'static str` from `String` unless we use `Box::leak`
                ""
            }
        }
    }

    /// Returns the string equivalent of this auth scheme ID.
    pub fn inner(&self) -> &str {
        &self.scheme_id
    }
}

impl From<&'static str> for AuthSchemeId {
    fn from(scheme_id: &'static str) -> Self {
        Self::new(scheme_id)
    }
}

impl From<Cow<'static, str>> for AuthSchemeId {
    fn from(scheme_id: Cow<'static, str>) -> Self {
        let normalized_scheme_id = match &scheme_id {
            Cow::Borrowed(s) => Cow::Borrowed(normalize_auth_scheme_id(s)),
            Cow::Owned(s) => {
                if s == "no_auth" {
                    Cow::Borrowed("noAuth")
                } else {
                    scheme_id
                }
            }
        };
        Self {
            scheme_id: normalized_scheme_id,
        }
    }
}

/// Parameters needed to resolve auth scheme options.
///
/// Most generated clients will use the [`StaticAuthSchemeOptionResolver`](static_resolver::StaticAuthSchemeOptionResolver),
/// which doesn't require any parameters for resolution (and has its own empty params struct).
///
/// However, more complex auth scheme resolvers may need modeled parameters in order to resolve
/// the auth scheme options. For those, this params struct holds a type erased box so that any
/// kind of parameters can be contained within, and type casted by the auth scheme option resolver
/// implementation.
#[derive(Debug)]
pub struct AuthSchemeOptionResolverParams(TypeErasedBox);

impl AuthSchemeOptionResolverParams {
    /// Creates a new [`AuthSchemeOptionResolverParams`].
    pub fn new<T: fmt::Debug + Send + Sync + 'static>(params: T) -> Self {
        Self(TypeErasedBox::new(params))
    }

    /// Returns the underlying parameters as the type `T` if they are that type.
    pub fn get<T: fmt::Debug + Send + Sync + 'static>(&self) -> Option<&T> {
        self.0.downcast_ref()
    }
}

impl Storable for AuthSchemeOptionResolverParams {
    type Storer = StoreReplace<Self>;
}

new_type_future! {
    #[doc = "Future for [`ResolveAuthSchemeOptions::resolve_auth_scheme_options_v2`]."]
    pub struct AuthSchemeOptionsFuture<'a, Vec<AuthSchemeOption>, BoxError>;
}

/// Resolver for auth scheme options.
///
/// The orchestrator needs to select an auth scheme to sign requests with, and potentially
/// from several different available auth schemes. Smithy models have a number of ways
/// to specify which operations can use which auth schemes under which conditions, as
/// documented in the [Smithy spec](https://smithy.io/2.0/spec/authentication-traits.html).
///
/// The orchestrator uses the auth scheme option resolver runtime component to resolve
/// an ordered list of options that are available to choose from for a given request.
/// This resolver can be a simple static list, such as with the
/// [`StaticAuthSchemeOptionResolver`](static_resolver::StaticAuthSchemeOptionResolver),
/// or it can be a complex code generated resolver that incorporates parameters from both
/// the model and the resolved endpoint.
pub trait ResolveAuthSchemeOptions: Send + Sync + fmt::Debug {
    #[deprecated(
        note = "This method is deprecated, use `resolve_auth_scheme_options_v2` instead.",
        since = "1.8.0"
    )]
    /// Returns a list of available auth scheme options to choose from.
    fn resolve_auth_scheme_options(
        &self,
        _params: &AuthSchemeOptionResolverParams,
    ) -> Result<Cow<'_, [AuthSchemeId]>, BoxError> {
        unimplemented!("This method is deprecated, use `resolve_auth_scheme_options_v2` instead.");
    }

    #[allow(deprecated)]
    /// Returns a list of available auth scheme options to choose from.
    fn resolve_auth_scheme_options_v2<'a>(
        &'a self,
        params: &'a AuthSchemeOptionResolverParams,
        _cfg: &'a ConfigBag,
        _runtime_components: &'a RuntimeComponents,
    ) -> AuthSchemeOptionsFuture<'a> {
        AuthSchemeOptionsFuture::ready({
            self.resolve_auth_scheme_options(params).map(|options| {
                options
                    .iter()
                    .cloned()
                    .map(|scheme_id| {
                        AuthSchemeOption::builder()
                            .scheme_id(scheme_id)
                            .build()
                            .expect("required fields set")
                    })
                    .collect::<Vec<_>>()
            })
        })
    }
}

/// A shared auth scheme option resolver.
#[derive(Clone, Debug)]
pub struct SharedAuthSchemeOptionResolver(Arc<dyn ResolveAuthSchemeOptions>);

impl SharedAuthSchemeOptionResolver {
    /// Creates a new [`SharedAuthSchemeOptionResolver`].
    pub fn new(auth_scheme_option_resolver: impl ResolveAuthSchemeOptions + 'static) -> Self {
        Self(Arc::new(auth_scheme_option_resolver))
    }
}

impl ResolveAuthSchemeOptions for SharedAuthSchemeOptionResolver {
    #[allow(deprecated)]
    fn resolve_auth_scheme_options(
        &self,
        params: &AuthSchemeOptionResolverParams,
    ) -> Result<Cow<'_, [AuthSchemeId]>, BoxError> {
        (*self.0).resolve_auth_scheme_options(params)
    }

    fn resolve_auth_scheme_options_v2<'a>(
        &'a self,
        params: &'a AuthSchemeOptionResolverParams,
        cfg: &'a ConfigBag,
        runtime_components: &'a RuntimeComponents,
    ) -> AuthSchemeOptionsFuture<'a> {
        (*self.0).resolve_auth_scheme_options_v2(params, cfg, runtime_components)
    }
}

impl_shared_conversions!(
    convert SharedAuthSchemeOptionResolver
    from ResolveAuthSchemeOptions
    using SharedAuthSchemeOptionResolver::new
);

/// An auth scheme.
///
/// Auth schemes have unique identifiers (the `scheme_id`),
/// and provide an identity resolver and a signer.
pub trait AuthScheme: Send + Sync + fmt::Debug {
    /// Returns the unique identifier associated with this auth scheme.
    ///
    /// This identifier is used to refer to this auth scheme from the
    /// [`ResolveAuthSchemeOptions`], and is also associated with
    /// identity resolvers in the config.
    fn scheme_id(&self) -> AuthSchemeId;

    /// Returns the identity resolver that can resolve an identity for this scheme, if one is available.
    ///
    /// The [`AuthScheme`] doesn't actually own an identity resolver. Rather, identity resolvers
    /// are configured as runtime components. The auth scheme merely chooses a compatible identity
    /// resolver from the runtime components via the [`GetIdentityResolver`] trait. The trait is
    /// given rather than the full set of runtime components to prevent complex resolution logic
    /// involving multiple components from taking place in this function, since that's not the
    /// intended use of this design.
    fn identity_resolver(
        &self,
        identity_resolvers: &dyn GetIdentityResolver,
    ) -> Option<SharedIdentityResolver>;

    /// Returns the signing implementation for this auth scheme.
    fn signer(&self) -> &dyn Sign;
}

/// Container for a shared auth scheme implementation.
#[derive(Clone, Debug)]
pub struct SharedAuthScheme(Arc<dyn AuthScheme>);

impl SharedAuthScheme {
    /// Creates a new [`SharedAuthScheme`] from the given auth scheme.
    pub fn new(auth_scheme: impl AuthScheme + 'static) -> Self {
        Self(Arc::new(auth_scheme))
    }
}

impl AuthScheme for SharedAuthScheme {
    fn scheme_id(&self) -> AuthSchemeId {
        self.0.scheme_id()
    }

    fn identity_resolver(
        &self,
        identity_resolvers: &dyn GetIdentityResolver,
    ) -> Option<SharedIdentityResolver> {
        self.0.identity_resolver(identity_resolvers)
    }

    fn signer(&self) -> &dyn Sign {
        self.0.signer()
    }
}

impl ValidateConfig for SharedAuthScheme {}

impl_shared_conversions!(convert SharedAuthScheme from AuthScheme using SharedAuthScheme::new);

/// Signing implementation for an auth scheme.
pub trait Sign: Send + Sync + fmt::Debug {
    /// Sign the given request with the given identity, components, and config.
    ///
    /// If the provided identity is incompatible with this signer, an error must be returned.
    fn sign_http_request(
        &self,
        request: &mut HttpRequest,
        identity: &Identity,
        auth_scheme_endpoint_config: AuthSchemeEndpointConfig<'_>,
        runtime_components: &RuntimeComponents,
        config_bag: &ConfigBag,
    ) -> Result<(), BoxError>;
}

/// Endpoint configuration for the selected auth scheme.
///
/// The configuration held by this struct originates from the endpoint rule set in the service model.
///
/// This struct gets added to the request state by the auth orchestrator.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct AuthSchemeEndpointConfig<'a>(Option<&'a Document>);

impl<'a> AuthSchemeEndpointConfig<'a> {
    /// Creates an empty [`AuthSchemeEndpointConfig`].
    pub fn empty() -> Self {
        Self(None)
    }

    /// Returns the endpoint configuration as a [`Document`].
    pub fn as_document(&self) -> Option<&'a Document> {
        self.0
    }
}

impl<'a> From<Option<&'a Document>> for AuthSchemeEndpointConfig<'a> {
    fn from(value: Option<&'a Document>) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a Document> for AuthSchemeEndpointConfig<'a> {
    fn from(value: &'a Document) -> Self {
        Self(Some(value))
    }
}

/// An ordered list of [AuthSchemeId]s
///
/// Can be used to reorder already-resolved auth schemes by an auth scheme resolver.
/// This list is intended as a hint rather than a strict override;
/// any schemes not present in the resolved auth schemes will be ignored.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AuthSchemePreference {
    preference_list: Vec<AuthSchemeId>,
}

impl Storable for AuthSchemePreference {
    type Storer = StoreReplace<Self>;
}

impl IntoIterator for AuthSchemePreference {
    type Item = AuthSchemeId;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.preference_list.into_iter()
    }
}

impl<T> From<T> for AuthSchemePreference
where
    T: AsRef<[AuthSchemeId]>,
{
    fn from(slice: T) -> Self {
        AuthSchemePreference {
            preference_list: slice.as_ref().to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_scheme_id_new_normalizes_no_auth() {
        // Test that "no_auth" gets normalized to "noAuth"
        let auth_scheme_id = AuthSchemeId::new("no_auth");
        assert_eq!(auth_scheme_id.inner(), "noAuth");
    }

    #[test]
    fn test_auth_scheme_id_new_preserves_no_auth_camel_case() {
        // Test that "noAuth" remains unchanged
        let auth_scheme_id = AuthSchemeId::new("noAuth");
        assert_eq!(auth_scheme_id.inner(), "noAuth");
    }

    #[test]
    fn test_auth_scheme_id_new_preserves_other_schemes() {
        // Test that other auth scheme IDs are not modified
        let test_cases = [
            "sigv4",
            "sigv4a",
            "httpBearerAuth",
            "httpBasicAuth",
            "custom_auth",
            "bearer",
            "basic",
        ];

        for scheme in test_cases {
            let auth_scheme_id = AuthSchemeId::new(scheme);
            assert_eq!(auth_scheme_id.inner(), scheme);
        }
    }

    #[test]
    fn test_auth_scheme_id_equality_after_normalization() {
        // Test that "no_auth" and "noAuth" are considered equal after normalization
        let no_auth_underscore = AuthSchemeId::new("no_auth");
        let no_auth_camel = AuthSchemeId::new("noAuth");

        assert_eq!(no_auth_underscore, no_auth_camel);
        assert_eq!(no_auth_underscore.inner(), no_auth_camel.inner());
    }

    #[test]
    fn test_auth_scheme_id_hash_consistency_after_normalization() {
        use std::collections::HashMap;

        // Test that normalized IDs have consistent hashing behavior
        let mut map = HashMap::new();
        let no_auth_underscore = AuthSchemeId::new("no_auth");
        let no_auth_camel = AuthSchemeId::new("noAuth");

        map.insert(no_auth_underscore.clone(), "value1");
        map.insert(no_auth_camel.clone(), "value2");

        // Should only have one entry since they normalize to the same value
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&no_auth_underscore), Some(&"value2"));
        assert_eq!(map.get(&no_auth_camel), Some(&"value2"));
    }

    #[test]
    fn test_auth_scheme_id_ordering_after_normalization() {
        // Test that ordering works correctly with normalized values
        let no_auth_underscore = AuthSchemeId::new("no_auth");
        let no_auth_camel = AuthSchemeId::new("noAuth");
        let other_scheme = AuthSchemeId::new("sigv4");

        assert_eq!(
            no_auth_underscore.cmp(&no_auth_camel),
            std::cmp::Ordering::Equal
        );
        assert_eq!(no_auth_underscore.cmp(&other_scheme), "noAuth".cmp("sigv4"));
    }

    #[test]
    fn test_normalize_auth_scheme_id_function() {
        // Test the normalize function directly
        assert_eq!(normalize_auth_scheme_id("no_auth"), "noAuth");
        assert_eq!(normalize_auth_scheme_id("noAuth"), "noAuth");
        assert_eq!(normalize_auth_scheme_id("sigv4"), "sigv4");
        assert_eq!(normalize_auth_scheme_id("custom"), "custom");
    }

    #[test]
    fn test_auth_scheme_id_from_cow_borrowed_normalizes_no_auth() {
        // Test that Cow::Borrowed("no_auth") gets normalized to "noAuth"
        let auth_scheme_id = AuthSchemeId::from(Cow::Borrowed("no_auth"));
        assert_eq!(auth_scheme_id.inner(), "noAuth");
    }

    #[test]
    fn test_auth_scheme_id_from_cow_borrowed_preserves_no_auth_camel_case() {
        // Test that Cow::Borrowed("noAuth") remains unchanged
        let auth_scheme_id = AuthSchemeId::from(Cow::Borrowed("noAuth"));
        assert_eq!(auth_scheme_id.inner(), "noAuth");
    }

    #[test]
    fn test_auth_scheme_id_from_cow_owned_normalizes_no_auth() {
        // Test that Cow::Owned(String::from("no_auth")) gets normalized to "noAuth"
        let auth_scheme_id = AuthSchemeId::from(Cow::Owned(String::from("no_auth")));
        assert_eq!(auth_scheme_id.inner(), "noAuth");
    }

    #[test]
    fn test_auth_scheme_id_from_cow_owned_preserves_no_auth_camel_case() {
        // Test that Cow::Owned(String::from("noAuth")) remains unchanged
        let auth_scheme_id = AuthSchemeId::from(Cow::Owned(String::from("noAuth")));
        assert_eq!(auth_scheme_id.inner(), "noAuth");
    }

    #[test]
    fn test_auth_scheme_id_from_cow_between_borrowed_and_owned_mixing_updated_and_legacy() {
        let borrowed_no_auth = AuthSchemeId::from(Cow::Borrowed("noAuth"));
        let owned_no_auth = AuthSchemeId::from(Cow::Owned(String::from("no_auth")));

        assert_eq!(borrowed_no_auth, owned_no_auth);
        assert_eq!(borrowed_no_auth.inner(), owned_no_auth.inner());
    }
}
