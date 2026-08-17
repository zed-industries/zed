/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::auth::{
    apply_signing_instructions, extract_endpoint_auth_scheme_signing_name,
    extract_endpoint_auth_scheme_signing_options, SigV4OperationSigningConfig, SigV4SigningError,
};
use aws_credential_types::Credentials;
use aws_sigv4::http_request::{sign, SignableBody, SignableRequest, SigningSettings};
use aws_sigv4::sign::v4a;
use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::auth::{
    AuthScheme, AuthSchemeEndpointConfig, AuthSchemeId, Sign,
};
use aws_smithy_runtime_api::client::identity::{Identity, SharedIdentityResolver};
use aws_smithy_runtime_api::client::orchestrator::HttpRequest;
use aws_smithy_runtime_api::client::runtime_components::{GetIdentityResolver, RuntimeComponents};
use aws_smithy_types::config_bag::ConfigBag;
use aws_types::region::SigningRegionSet;
use aws_types::SigningName;
use std::borrow::Cow;
use std::time::SystemTime;

const EXPIRATION_WARNING: &str = "Presigned request will expire before the given \
        `expires_in` duration because the credentials used to sign it will expire first.";

/// Auth scheme ID for SigV4a.
pub const SCHEME_ID: AuthSchemeId = AuthSchemeId::new("sigv4a");

/// SigV4a auth scheme.
#[derive(Debug, Default)]
pub struct SigV4aAuthScheme {
    signer: SigV4aSigner,
}

impl SigV4aAuthScheme {
    /// Creates a new `SigV4aHttpAuthScheme`.
    pub fn new() -> Self {
        Default::default()
    }
}

impl AuthScheme for SigV4aAuthScheme {
    fn scheme_id(&self) -> AuthSchemeId {
        SCHEME_ID
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

/// SigV4a HTTP request signer.
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct SigV4aSigner;

impl SigV4aSigner {
    /// Creates a new signer instance.
    pub fn new() -> Self {
        Self
    }

    fn settings(operation_config: &SigV4OperationSigningConfig) -> SigningSettings {
        super::settings(operation_config)
    }

    fn signing_params<'a>(
        settings: SigningSettings,
        identity: &'a Identity,
        operation_config: &'a SigV4OperationSigningConfig,
        request_timestamp: SystemTime,
    ) -> Result<v4a::SigningParams<'a, SigningSettings>, SigV4SigningError> {
        if let Some(expires_in) = settings.expires_in {
            if let Some(identity_expiration) = identity.expiration() {
                let presigned_expires_time = request_timestamp + expires_in;
                if presigned_expires_time > identity_expiration {
                    tracing::warn!(EXPIRATION_WARNING);
                }
            }
        }

        Ok(v4a::SigningParams::builder()
            .identity(identity)
            .region_set(
                operation_config
                    .region_set
                    .as_ref()
                    .ok_or(SigV4SigningError::MissingSigningRegionSet)?
                    .as_ref(),
            )
            .name(
                operation_config
                    .name
                    .as_ref()
                    .ok_or(SigV4SigningError::MissingSigningName)?
                    .as_ref(),
            )
            .time(request_timestamp)
            .settings(settings)
            .build()
            .expect("all required fields set"))
    }

    fn extract_operation_config<'a>(
        auth_scheme_endpoint_config: AuthSchemeEndpointConfig<'a>,
        config_bag: &'a ConfigBag,
    ) -> Result<Cow<'a, SigV4OperationSigningConfig>, SigV4SigningError> {
        let operation_config = config_bag
            .load::<SigV4OperationSigningConfig>()
            .ok_or(SigV4SigningError::MissingOperationSigningConfig)?;

        let name = extract_endpoint_auth_scheme_signing_name(&auth_scheme_endpoint_config)?
            .or(config_bag.load::<SigningName>().cloned());

        let region_set =
            extract_endpoint_auth_scheme_signing_region_set(&auth_scheme_endpoint_config)?
                .or(config_bag.load::<SigningRegionSet>().cloned());

        let signing_options = extract_endpoint_auth_scheme_signing_options(
            &auth_scheme_endpoint_config,
            &operation_config.signing_options,
        )?;

        match (region_set, name, signing_options) {
            (None, None, Cow::Borrowed(_)) => Ok(Cow::Borrowed(operation_config)),
            (region_set, name, signing_options) => {
                let mut operation_config = operation_config.clone();
                operation_config.region_set = region_set.or(operation_config.region_set);
                operation_config.name = name.or(operation_config.name);
                operation_config.signing_options = match signing_options {
                    Cow::Owned(opts) => opts,
                    Cow::Borrowed(_) => operation_config.signing_options,
                };
                Ok(Cow::Owned(operation_config))
            }
        }
    }
}

fn extract_endpoint_auth_scheme_signing_region_set(
    endpoint_config: &AuthSchemeEndpointConfig<'_>,
) -> Result<Option<SigningRegionSet>, SigV4SigningError> {
    use aws_smithy_types::Document::Array;
    use SigV4SigningError::BadTypeInEndpointAuthSchemeConfig as UnexpectedType;

    match super::extract_field_from_endpoint_config("signingRegionSet", endpoint_config) {
        Some(Array(docs)) => {
            // The service defines the region set as a string array. Here, we convert it to a comma separated list.
            let region_set: SigningRegionSet =
                docs.iter().filter_map(|doc| doc.as_string()).collect();

            Ok(Some(region_set))
        }
        None => Ok(None),
        _it => Err(UnexpectedType("signingRegionSet")),
    }
}

impl Sign for SigV4aSigner {
    fn sign_http_request(
        &self,
        request: &mut HttpRequest,
        identity: &Identity,
        auth_scheme_endpoint_config: AuthSchemeEndpointConfig<'_>,
        runtime_components: &RuntimeComponents,
        config_bag: &ConfigBag,
    ) -> Result<(), BoxError> {
        let operation_config =
            Self::extract_operation_config(auth_scheme_endpoint_config, config_bag)?;
        let request_time = runtime_components.time_source().unwrap_or_default().now();

        if identity.data::<Credentials>().is_none() {
            return Err(SigV4SigningError::WrongIdentityType(identity.clone()).into());
        }

        let settings = Self::settings(&operation_config);
        let signing_params =
            Self::signing_params(settings, identity, &operation_config, request_time)?;

        let (signing_instructions, _signature) = {
            // A body that is already in memory can be signed directly. A body that is not in memory
            // (any sort of streaming body or presigned request) will be signed via UNSIGNED-PAYLOAD.
            let signable_body = operation_config
                .signing_options
                .payload_override
                .as_ref()
                // the payload_override is a cheap clone because it contains either a
                // reference or a short checksum (we're not cloning the entire body)
                .cloned()
                .unwrap_or_else(|| {
                    request
                        .body()
                        .bytes()
                        .map(SignableBody::Bytes)
                        .unwrap_or(SignableBody::UnsignedPayload)
                });

            let signable_request = SignableRequest::new(
                request.method(),
                request.uri().to_string(),
                request.headers().iter(),
                signable_body,
            )?;
            sign(signable_request, &signing_params.into())?
        }
        .into_parts();

        apply_signing_instructions(signing_instructions, request)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{SigV4OperationSigningConfig, SigV4aSigner, EXPIRATION_WARNING};
    use crate::auth::{HttpSignatureType, SigningOptions};
    use aws_credential_types::Credentials;
    use aws_sigv4::http_request::SigningSettings;
    use aws_smithy_runtime_api::client::auth::AuthSchemeEndpointConfig;
    use aws_smithy_types::config_bag::{ConfigBag, Layer};
    use aws_smithy_types::Document;
    use aws_types::SigningName;
    use std::borrow::Cow;
    use std::collections::HashMap;
    use std::time::{Duration, SystemTime};
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn expiration_warning() {
        let now = SystemTime::UNIX_EPOCH + Duration::from_secs(1000);
        let creds_expire_in = Duration::from_secs(100);

        let mut settings = SigningSettings::default();
        settings.expires_in = Some(creds_expire_in - Duration::from_secs(10));

        let identity = Credentials::new(
            "test-access-key",
            "test-secret-key",
            Some("test-session-token".into()),
            Some(now + creds_expire_in),
            "test",
        )
        .into();
        let operation_config = SigV4OperationSigningConfig {
            region_set: Some("test".into()),
            name: Some(SigningName::from_static("test")),
            signing_options: SigningOptions {
                double_uri_encode: true,
                content_sha256_header: true,
                normalize_uri_path: true,
                omit_session_token: true,
                signature_type: HttpSignatureType::HttpRequestHeaders,
                signing_optional: false,
                expires_in: None,
                payload_override: None,
            },
            ..Default::default()
        };
        SigV4aSigner::signing_params(settings, &identity, &operation_config, now).unwrap();
        assert!(!logs_contain(EXPIRATION_WARNING));

        let mut settings = SigningSettings::default();
        settings.expires_in = Some(creds_expire_in + Duration::from_secs(10));

        SigV4aSigner::signing_params(settings, &identity, &operation_config, now).unwrap();
        assert!(logs_contain(EXPIRATION_WARNING));
    }

    #[test]
    fn endpoint_config_overrides_region_and_service() {
        let mut layer = Layer::new("test");
        layer.store_put(SigV4OperationSigningConfig {
            region_set: Some("test".into()),
            name: Some(SigningName::from_static("override-this-service")),
            ..Default::default()
        });
        let config = Document::Object({
            let mut out = HashMap::new();
            out.insert("name".to_owned(), "sigv4a".to_owned().into());
            out.insert("signingName".to_owned(), "qldb-override".to_owned().into());
            out.insert(
                "signingRegionSet".to_string(),
                Document::Array(vec!["us-east-override".to_string().into()]),
            );
            out
        });
        let config = AuthSchemeEndpointConfig::from(Some(&config));

        let cfg = ConfigBag::of_layers(vec![layer]);
        let result = SigV4aSigner::extract_operation_config(config, &cfg).expect("success");

        assert_eq!(result.region_set, Some("us-east-override".into()));
        assert_eq!(result.name, Some(SigningName::from_static("qldb-override")));
        assert!(matches!(result, Cow::Owned(_)));
    }

    #[test]
    fn endpoint_config_supports_fallback_when_region_or_service_are_unset() {
        let mut layer = Layer::new("test");
        layer.store_put(SigV4OperationSigningConfig {
            region_set: Some("us-east-1".into()),
            name: Some(SigningName::from_static("qldb")),
            ..Default::default()
        });
        let cfg = ConfigBag::of_layers(vec![layer]);
        let config = AuthSchemeEndpointConfig::empty();

        let result = SigV4aSigner::extract_operation_config(config, &cfg).expect("success");

        assert_eq!(result.region_set, Some("us-east-1".into()));
        assert_eq!(result.name, Some(SigningName::from_static("qldb")));
        assert!(matches!(result, Cow::Borrowed(_)));
    }
}
