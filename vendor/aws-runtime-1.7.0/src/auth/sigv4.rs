/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::auth::{
    self, extract_endpoint_auth_scheme_signing_name, extract_endpoint_auth_scheme_signing_options,
    extract_endpoint_auth_scheme_signing_region, PayloadSigningOverride,
    SigV4OperationSigningConfig, SigV4SessionTokenNameOverride, SigV4SigningError,
};
use crate::content_encoding::{DeferredSignerSender, SignChunk};
use aws_credential_types::Credentials;
use aws_sigv4::http_request::{
    sign, SignableBody, SignableRequest, SigningError, SigningParams, SigningSettings,
};
use aws_sigv4::sign::v4::{self, sign_chunk, sign_trailer};
use aws_smithy_async::time::{SharedTimeSource, StaticTimeSource};
use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::auth::{
    AuthScheme, AuthSchemeEndpointConfig, AuthSchemeId, Sign,
};
use aws_smithy_runtime_api::client::identity::{Identity, SharedIdentityResolver};
use aws_smithy_runtime_api::client::orchestrator::HttpRequest;
use aws_smithy_runtime_api::client::runtime_components::{GetIdentityResolver, RuntimeComponents};
use aws_smithy_runtime_api::http::Headers;
use aws_smithy_types::config_bag::ConfigBag;
use aws_types::region::SigningRegion;
use aws_types::SigningName;
use bytes::Bytes;
use std::borrow::Cow;
use std::time::SystemTime;

const EXPIRATION_WARNING: &str = "Presigned request will expire before the given \
        `expires_in` duration because the credentials used to sign it will expire first.";

/// Auth scheme ID for SigV4.
pub const SCHEME_ID: AuthSchemeId = AuthSchemeId::new("sigv4");

/// SigV4 auth scheme.
#[derive(Debug, Default)]
pub struct SigV4AuthScheme {
    signer: SigV4Signer,
}

impl SigV4AuthScheme {
    /// Creates a new `SigV4AuthScheme`.
    pub fn new() -> Self {
        Default::default()
    }
}

impl AuthScheme for SigV4AuthScheme {
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

/// SigV4 signer.
#[derive(Debug, Default)]
pub struct SigV4Signer;

impl SigV4Signer {
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
    ) -> Result<v4::SigningParams<'a, SigningSettings>, SigV4SigningError> {
        let creds = identity
            .data::<Credentials>()
            .ok_or_else(|| SigV4SigningError::WrongIdentityType(identity.clone()))?;

        if let Some(expires_in) = settings.expires_in {
            if let Some(creds_expires_time) = creds.expiry() {
                let presigned_expires_time = request_timestamp + expires_in;
                if presigned_expires_time > creds_expires_time {
                    tracing::warn!(EXPIRATION_WARNING);
                }
            }
        }

        Ok(v4::SigningParams::builder()
            .identity(identity)
            .region(
                operation_config
                    .region
                    .as_ref()
                    .ok_or(SigV4SigningError::MissingSigningRegion)?
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

        let region = extract_endpoint_auth_scheme_signing_region(&auth_scheme_endpoint_config)?
            .or(config_bag.load::<SigningRegion>().cloned());

        let signing_options = extract_endpoint_auth_scheme_signing_options(
            &auth_scheme_endpoint_config,
            &operation_config.signing_options,
        )?;

        match (region, name, signing_options) {
            (None, None, Cow::Borrowed(_)) => Ok(Cow::Borrowed(operation_config)),
            (region, name, signing_options) => {
                let mut operation_config = operation_config.clone();
                operation_config.region = region.or(operation_config.region);
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

impl Sign for SigV4Signer {
    fn sign_http_request(
        &self,
        request: &mut HttpRequest,
        identity: &Identity,
        auth_scheme_endpoint_config: AuthSchemeEndpointConfig<'_>,
        runtime_components: &RuntimeComponents,
        config_bag: &ConfigBag,
    ) -> Result<(), BoxError> {
        if identity.data::<Credentials>().is_none() {
            return Err(SigV4SigningError::WrongIdentityType(identity.clone()).into());
        };

        let operation_config =
            Self::extract_operation_config(auth_scheme_endpoint_config, config_bag)?;
        let request_time = runtime_components.time_source().unwrap_or_default().now();

        let settings = if let Some(session_token_name_override) =
            config_bag.load::<SigV4SessionTokenNameOverride>()
        {
            let mut settings = Self::settings(&operation_config);
            let name_override = session_token_name_override.name_override(&settings, config_bag)?;
            settings.session_token_name_override = name_override;
            settings
        } else {
            Self::settings(&operation_config)
        };

        let chunk_signer_sender = config_bag.load::<DeferredSignerSender>();

        // `sender_and_settings` needs to include a cloned `settings` to satisfy Rust's borrow checker
        let (signing_params, sender_and_settings) = if let Some(signer_sender) = chunk_signer_sender
        {
            // Clone settings since we'll need it later for the message signer
            let signing_params =
                Self::signing_params(settings.clone(), identity, &operation_config, request_time)?;
            (signing_params, Some((signer_sender, settings)))
        } else {
            // Move settings since we won't need it later
            let signing_params =
                Self::signing_params(settings, identity, &operation_config, request_time)?;
            (signing_params, None)
        };

        let (signing_instructions, _signature) = {
            // A body that is already in memory can be signed directly. A body that is not in memory
            // (any sort of streaming body or presigned request) will be signed via UNSIGNED-PAYLOAD.
            let mut signable_body = operation_config
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

            // Sometimes it's necessary to override the payload signing scheme.
            // If an override exists then fetch and apply it.
            if let Some(payload_signing_override) = config_bag.load::<PayloadSigningOverride>() {
                tracing::trace!(
                    "payload signing was overridden, now set to {payload_signing_override:?}"
                );
                signable_body = payload_signing_override.clone().to_signable_body();
            }

            let signable_request = SignableRequest::new(
                request.method(),
                request.uri(),
                request.headers().iter(),
                signable_body,
            )?;
            sign(signable_request, &SigningParams::V4(signing_params))?
        }
        .into_parts();

        if let Some((signer_sender, settings)) = sender_and_settings {
            let time_source = StaticTimeSource::new(request_time).into();
            let region = operation_config
                .region
                .clone()
                .expect("`Self::signing_params` above would have errored, if region was missing");
            let name = operation_config
                .name
                .clone()
                .expect("`Self::signing_params` above would have errored, if name was missing");
            signer_sender
                .send(Box::new(SigV4MessageSigner::new(
                    _signature.clone(),
                    identity.clone(),
                    region,
                    name,
                    time_source,
                    settings,
                )) as _)
                .expect("failed to send deferred signer");
        };

        // If this is an event stream operation, set up the event stream signer
        #[cfg(feature = "event-stream")]
        {
            use crate::auth::sigv4::SigV4MessageSigner;
            use aws_smithy_eventstream::frame::DeferredSignerSender;

            if let Some(signer_sender) = config_bag.load::<DeferredSignerSender>() {
                let time_source = runtime_components.time_source().unwrap_or_default();
                let region = operation_config.region.clone().expect(
                    "`Self::signing_params` above would have errored, if region was missing",
                );
                let name = operation_config
                    .name
                    .clone()
                    .expect("`Self::signing_params` above would have errored, if name was missing");
                signer_sender
                    .send(Box::new(SigV4MessageSigner::new(
                        _signature,
                        identity.clone(),
                        region,
                        name,
                        time_source,
                        (),
                    )) as _)
                    .expect("failed to send deferred signer");
            }
        }
        auth::apply_signing_instructions(signing_instructions, request)?;
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct SigV4MessageSigner<S> {
    running_signature: String,
    identity: Identity,
    signing_region: SigningRegion,
    signing_name: SigningName,
    time: SharedTimeSource,
    signing_settings: S,
}

impl<S> SigV4MessageSigner<S>
where
    S: Clone + Default,
{
    pub(crate) fn new(
        running_signature: String,
        identity: Identity,
        signing_region: SigningRegion,
        signing_name: SigningName,
        time: SharedTimeSource,
        signing_settings: S,
    ) -> Self {
        Self {
            running_signature,
            identity,
            signing_region,
            signing_name,
            time,
            signing_settings,
        }
    }

    fn signing_params(&self) -> v4::SigningParams<'_, S> {
        let builder = v4::SigningParams::builder()
            .identity(&self.identity)
            .region(self.signing_region.as_ref())
            .name(self.signing_name.as_ref())
            .time(self.time.now())
            .settings(self.signing_settings.clone());
        builder.build().unwrap()
    }
}

impl SignChunk for SigV4MessageSigner<SigningSettings> {
    fn chunk_signature(&mut self, chunk: &Bytes) -> Result<String, SigningError> {
        let params = self.signing_params();
        let (_, signature) = sign_chunk(chunk, &self.running_signature, &params)?.into_parts();
        self.running_signature = signature.clone();
        Ok(signature)
    }

    fn trailer_signature(&mut self, trailing_headers: &Headers) -> Result<String, SigningError> {
        let params = self.signing_params();
        let (_, signature) =
            sign_trailer(trailing_headers, &self.running_signature, &params)?.into_parts();
        self.running_signature = signature.clone();
        Ok(signature)
    }
}

#[cfg(feature = "event-stream")]
mod event_stream {
    use crate::auth::sigv4::SigV4MessageSigner;
    use aws_sigv4::event_stream::{sign_empty_message, sign_message};
    use aws_smithy_eventstream::frame::{SignMessage, SignMessageError};
    use aws_smithy_types::event_stream::Message;

    impl SignMessage for SigV4MessageSigner<()> {
        fn sign(&mut self, message: Message) -> Result<Message, SignMessageError> {
            let (signed_message, signature) = {
                let params = self.signing_params();
                sign_message(&message, &self.running_signature, &params)?.into_parts()
            };
            self.running_signature = signature;
            Ok(signed_message)
        }

        fn sign_empty(&mut self) -> Option<Result<Message, SignMessageError>> {
            let (signed_message, signature) = {
                let params = self.signing_params();
                sign_empty_message(&self.running_signature, &params)
                    .ok()?
                    .into_parts()
            };
            self.running_signature = signature;
            Some(Ok(signed_message))
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::auth::sigv4::SigV4MessageSigner;
        use aws_credential_types::Credentials;
        use aws_smithy_async::time::SharedTimeSource;
        use aws_smithy_eventstream::frame::SignMessage;
        use aws_smithy_types::event_stream::{HeaderValue, Message};

        use aws_types::region::Region;
        use aws_types::region::SigningRegion;
        use aws_types::SigningName;
        use std::time::{Duration, UNIX_EPOCH};

        fn check_send_sync<T: Send + Sync>(value: T) -> T {
            value
        }

        #[test]
        fn sign_message() {
            let region = Region::new("us-east-1");
            let mut signer = check_send_sync(SigV4MessageSigner::new(
                "initial-signature".into(),
                Credentials::for_tests_with_session_token().into(),
                SigningRegion::from(region),
                SigningName::from_static("transcribe"),
                SharedTimeSource::new(UNIX_EPOCH + Duration::new(1611160427, 0)),
                (),
            ));
            let mut signatures = Vec::new();
            for _ in 0..5 {
                let signed = signer
                    .sign(Message::new(&b"identical message"[..]))
                    .unwrap();
                if let HeaderValue::ByteArray(signature) = signed
                    .headers()
                    .iter()
                    .find(|h| h.name().as_str() == ":chunk-signature")
                    .unwrap()
                    .value()
                {
                    signatures.push(signature.clone());
                } else {
                    panic!("failed to get the :chunk-signature")
                }
            }
            for i in 1..signatures.len() {
                assert_ne!(signatures[i - 1], signatures[i]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::{HttpSignatureType, SigningOptions};
    use aws_credential_types::Credentials;
    use aws_sigv4::http_request::SigningSettings;
    use aws_smithy_types::config_bag::Layer;
    use aws_smithy_types::Document;
    use aws_types::region::SigningRegion;
    use aws_types::SigningName;
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
            region: Some(SigningRegion::from_static("test")),
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
        SigV4Signer::signing_params(settings, &identity, &operation_config, now).unwrap();
        assert!(!logs_contain(EXPIRATION_WARNING));

        let mut settings = SigningSettings::default();
        settings.expires_in = Some(creds_expire_in + Duration::from_secs(10));

        SigV4Signer::signing_params(settings, &identity, &operation_config, now).unwrap();
        assert!(logs_contain(EXPIRATION_WARNING));
    }

    #[test]
    fn endpoint_config_overrides_region_and_service() {
        let mut layer = Layer::new("test");
        layer.store_put(SigV4OperationSigningConfig {
            region: Some(SigningRegion::from_static("override-this-region")),
            name: Some(SigningName::from_static("override-this-name")),
            ..Default::default()
        });
        let config = Document::Object({
            let mut out = HashMap::new();
            out.insert("name".to_string(), "sigv4".to_string().into());
            out.insert(
                "signingName".to_string(),
                "qldb-override".to_string().into(),
            );
            out.insert(
                "signingRegion".to_string(),
                "us-east-override".to_string().into(),
            );
            out
        });
        let config = AuthSchemeEndpointConfig::from(Some(&config));

        let cfg = ConfigBag::of_layers(vec![layer]);
        let result = SigV4Signer::extract_operation_config(config, &cfg).expect("success");

        assert_eq!(
            result.region,
            Some(SigningRegion::from_static("us-east-override"))
        );
        assert_eq!(result.name, Some(SigningName::from_static("qldb-override")));
        assert!(matches!(result, Cow::Owned(_)));
    }

    #[test]
    fn endpoint_config_supports_fallback_when_region_or_service_are_unset() {
        let mut layer = Layer::new("test");
        layer.store_put(SigV4OperationSigningConfig {
            region: Some(SigningRegion::from_static("us-east-1")),
            name: Some(SigningName::from_static("qldb")),
            ..Default::default()
        });
        let cfg = ConfigBag::of_layers(vec![layer]);
        let config = AuthSchemeEndpointConfig::empty();

        let result = SigV4Signer::extract_operation_config(config, &cfg).expect("success");

        assert_eq!(result.region, Some(SigningRegion::from_static("us-east-1")));
        assert_eq!(result.name, Some(SigningName::from_static("qldb")));
        assert!(matches!(result, Cow::Borrowed(_)));
    }
}
