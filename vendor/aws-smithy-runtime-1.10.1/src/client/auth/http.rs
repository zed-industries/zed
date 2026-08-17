/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Auth scheme implementations for HTTP API Key, Basic Auth, Bearer Token, and Digest auth.

use aws_smithy_http::query_writer::QueryWriter;
use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::auth::http::{
    HTTP_API_KEY_AUTH_SCHEME_ID, HTTP_BASIC_AUTH_SCHEME_ID, HTTP_BEARER_AUTH_SCHEME_ID,
    HTTP_DIGEST_AUTH_SCHEME_ID,
};
use aws_smithy_runtime_api::client::auth::{
    AuthScheme, AuthSchemeEndpointConfig, AuthSchemeId, Sign,
};
use aws_smithy_runtime_api::client::identity::http::{Login, Token};
use aws_smithy_runtime_api::client::identity::{Identity, SharedIdentityResolver};
use aws_smithy_runtime_api::client::orchestrator::HttpRequest;
use aws_smithy_runtime_api::client::runtime_components::{GetIdentityResolver, RuntimeComponents};
use aws_smithy_types::base64::encode;
use aws_smithy_types::config_bag::ConfigBag;

/// Destination for the API key
#[derive(Copy, Clone, Debug)]
pub enum ApiKeyLocation {
    /// Place the API key in the URL query parameters
    Query,
    /// Place the API key in the request headers
    Header,
}

/// Auth implementation for Smithy's `@httpApiKey` auth scheme
#[derive(Debug)]
pub struct ApiKeyAuthScheme {
    signer: ApiKeySigner,
}

impl ApiKeyAuthScheme {
    /// Creates a new `ApiKeyAuthScheme`.
    pub fn new(
        scheme: impl Into<String>,
        location: ApiKeyLocation,
        name: impl Into<String>,
    ) -> Self {
        Self {
            signer: ApiKeySigner {
                scheme: scheme.into(),
                location,
                name: name.into(),
            },
        }
    }
}

impl AuthScheme for ApiKeyAuthScheme {
    fn scheme_id(&self) -> AuthSchemeId {
        HTTP_API_KEY_AUTH_SCHEME_ID
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

#[derive(Debug)]
struct ApiKeySigner {
    scheme: String,
    location: ApiKeyLocation,
    name: String,
}

impl Sign for ApiKeySigner {
    fn sign_http_request(
        &self,
        request: &mut HttpRequest,
        identity: &Identity,
        _auth_scheme_endpoint_config: AuthSchemeEndpointConfig<'_>,
        _runtime_components: &RuntimeComponents,
        _config_bag: &ConfigBag,
    ) -> Result<(), BoxError> {
        let api_key = identity
            .data::<Token>()
            .ok_or("HTTP ApiKey auth requires a `Token` identity")?;
        match self.location {
            ApiKeyLocation::Header => {
                request
                    .headers_mut()
                    .try_append(
                        self.name.to_ascii_lowercase(),
                        format!("{} {}", self.scheme, api_key.token()),
                    )
                    .map_err(|_| {
                        "API key contains characters that can't be included in a HTTP header"
                    })?;
            }
            ApiKeyLocation::Query => {
                let mut query = QueryWriter::new_from_string(request.uri())?;
                query.insert(&self.name, api_key.token());
                request
                    .set_uri(query.build_uri().to_string())
                    .expect("query writer returns a valid URI")
            }
        }

        Ok(())
    }
}

/// Auth implementation for Smithy's `@httpBasicAuth` auth scheme
#[derive(Debug, Default)]
pub struct BasicAuthScheme {
    signer: BasicAuthSigner,
}

impl BasicAuthScheme {
    /// Creates a new `BasicAuthScheme`.
    pub fn new() -> Self {
        Self {
            signer: BasicAuthSigner,
        }
    }
}

impl AuthScheme for BasicAuthScheme {
    fn scheme_id(&self) -> AuthSchemeId {
        HTTP_BASIC_AUTH_SCHEME_ID
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

#[derive(Debug, Default)]
struct BasicAuthSigner;

impl Sign for BasicAuthSigner {
    fn sign_http_request(
        &self,
        request: &mut HttpRequest,
        identity: &Identity,
        _auth_scheme_endpoint_config: AuthSchemeEndpointConfig<'_>,
        _runtime_components: &RuntimeComponents,
        _config_bag: &ConfigBag,
    ) -> Result<(), BoxError> {
        let login = identity
            .data::<Login>()
            .ok_or("HTTP basic auth requires a `Login` identity")?;
        request.headers_mut().insert(
            http_1x::header::AUTHORIZATION,
            http_1x::HeaderValue::from_str(&format!(
                "Basic {}",
                encode(format!("{}:{}", login.user(), login.password()))
            ))
            .expect("valid header value"),
        );
        Ok(())
    }
}

/// Auth implementation for Smithy's `@httpBearerAuth` auth scheme
#[derive(Debug, Default)]
pub struct BearerAuthScheme {
    signer: BearerAuthSigner,
}

impl BearerAuthScheme {
    /// Creates a new `BearerAuthScheme`.
    pub fn new() -> Self {
        Self {
            signer: BearerAuthSigner,
        }
    }
}

impl AuthScheme for BearerAuthScheme {
    fn scheme_id(&self) -> AuthSchemeId {
        HTTP_BEARER_AUTH_SCHEME_ID
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

#[derive(Debug, Default)]
struct BearerAuthSigner;

impl Sign for BearerAuthSigner {
    fn sign_http_request(
        &self,
        request: &mut HttpRequest,
        identity: &Identity,
        _auth_scheme_endpoint_config: AuthSchemeEndpointConfig<'_>,
        _runtime_components: &RuntimeComponents,
        _config_bag: &ConfigBag,
    ) -> Result<(), BoxError> {
        let token = identity
            .data::<Token>()
            .ok_or("HTTP bearer auth requires a `Token` identity")?;
        request.headers_mut().insert(
            http_1x::header::AUTHORIZATION,
            http_1x::HeaderValue::from_str(&format!("Bearer {}", token.token())).map_err(|_| {
                "Bearer token contains characters that can't be included in a HTTP header"
            })?,
        );
        Ok(())
    }
}

/// Auth implementation for Smithy's `@httpDigestAuth` auth scheme
#[derive(Debug, Default)]
pub struct DigestAuthScheme {
    signer: DigestAuthSigner,
}

impl DigestAuthScheme {
    /// Creates a new `DigestAuthScheme`.
    pub fn new() -> Self {
        Self {
            signer: DigestAuthSigner,
        }
    }
}

impl AuthScheme for DigestAuthScheme {
    fn scheme_id(&self) -> AuthSchemeId {
        HTTP_DIGEST_AUTH_SCHEME_ID
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

#[derive(Debug, Default)]
struct DigestAuthSigner;

impl Sign for DigestAuthSigner {
    fn sign_http_request(
        &self,
        _request: &mut HttpRequest,
        _identity: &Identity,
        _auth_scheme_endpoint_config: AuthSchemeEndpointConfig<'_>,
        _runtime_components: &RuntimeComponents,
        _config_bag: &ConfigBag,
    ) -> Result<(), BoxError> {
        unimplemented!(
            "support for signing with Smithy's `@httpDigestAuth` auth scheme is not implemented yet"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_smithy_runtime_api::client::identity::http::Login;
    use aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder;
    use aws_smithy_types::body::SdkBody;

    #[test]
    fn test_api_key_signing_headers() {
        let signer = ApiKeySigner {
            scheme: "SomeSchemeName".into(),
            location: ApiKeyLocation::Header,
            name: "some-header-name".into(),
        };
        let runtime_components = RuntimeComponentsBuilder::for_tests().build().unwrap();
        let config_bag = ConfigBag::base();
        let identity = Identity::new(Token::new("some-token", None), None);
        let mut request: HttpRequest = http_1x::Request::builder()
            .uri("http://example.com/Foobaz")
            .body(SdkBody::empty())
            .unwrap()
            .try_into()
            .unwrap();
        signer
            .sign_http_request(
                &mut request,
                &identity,
                AuthSchemeEndpointConfig::empty(),
                &runtime_components,
                &config_bag,
            )
            .expect("success");
        assert_eq!(
            "SomeSchemeName some-token",
            request.headers().get("some-header-name").unwrap()
        );
        assert_eq!("http://example.com/Foobaz", request.uri().to_string());
    }

    #[test]
    fn test_api_key_signing_query() {
        let signer = ApiKeySigner {
            scheme: "".into(),
            location: ApiKeyLocation::Query,
            name: "some-query-name".into(),
        };
        let runtime_components = RuntimeComponentsBuilder::for_tests().build().unwrap();
        let config_bag = ConfigBag::base();
        let identity = Identity::new(Token::new("some-token", None), None);
        let mut request: HttpRequest = http_1x::Request::builder()
            .uri("http://example.com/Foobaz")
            .body(SdkBody::empty())
            .unwrap()
            .try_into()
            .unwrap();
        signer
            .sign_http_request(
                &mut request,
                &identity,
                AuthSchemeEndpointConfig::empty(),
                &runtime_components,
                &config_bag,
            )
            .expect("success");
        assert!(request.headers().get("some-query-name").is_none());
        assert_eq!(
            "http://example.com/Foobaz?some-query-name=some-token",
            request.uri().to_string()
        );
    }

    #[test]
    fn test_basic_auth() {
        let signer = BasicAuthSigner;
        let runtime_components = RuntimeComponentsBuilder::for_tests().build().unwrap();
        let config_bag = ConfigBag::base();
        let identity = Identity::new(Login::new("Aladdin", "open sesame", None), None);
        let mut request = http_1x::Request::builder()
            .body(SdkBody::empty())
            .unwrap()
            .try_into()
            .unwrap();

        signer
            .sign_http_request(
                &mut request,
                &identity,
                AuthSchemeEndpointConfig::empty(),
                &runtime_components,
                &config_bag,
            )
            .expect("success");
        assert_eq!(
            "Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==",
            request.headers().get("Authorization").unwrap()
        );
    }

    #[test]
    fn test_bearer_auth() {
        let signer = BearerAuthSigner;

        let config_bag = ConfigBag::base();
        let runtime_components = RuntimeComponentsBuilder::for_tests().build().unwrap();
        let identity = Identity::new(Token::new("some-token", None), None);
        let mut request = http_1x::Request::builder()
            .body(SdkBody::empty())
            .unwrap()
            .try_into()
            .unwrap();
        signer
            .sign_http_request(
                &mut request,
                &identity,
                AuthSchemeEndpointConfig::empty(),
                &runtime_components,
                &config_bag,
            )
            .expect("success");
        assert_eq!(
            "Bearer some-token",
            request.headers().get("Authorization").unwrap()
        );
    }

    #[test]
    fn test_bearer_auth_overwrite_existing_header() {
        let signer = BearerAuthSigner;

        let config_bag = ConfigBag::base();
        let runtime_components = RuntimeComponentsBuilder::for_tests().build().unwrap();
        let identity = Identity::new(Token::new("some-token", None), None);
        let mut request = http_1x::Request::builder()
            .header("Authorization", "wrong")
            .body(SdkBody::empty())
            .unwrap()
            .try_into()
            .unwrap();
        signer
            .sign_http_request(
                &mut request,
                &identity,
                AuthSchemeEndpointConfig::empty(),
                &runtime_components,
                &config_bag,
            )
            .expect("success");
        assert_eq!(
            "Bearer some-token",
            request.headers().get("Authorization").unwrap()
        );
    }
}
