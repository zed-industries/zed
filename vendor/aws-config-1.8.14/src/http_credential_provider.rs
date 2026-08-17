/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Generalized HTTP credential provider. Currently, this cannot be used directly and can only
//! be used via the ECS credential provider.
//!
//! Future work will stabilize this interface and enable it to be used directly.

use crate::json_credentials::{parse_json_credentials, JsonCredentials, RefreshableCredentials};
use crate::provider_config::ProviderConfig;
use aws_credential_types::attributes::AccountId;
use aws_credential_types::credential_feature::AwsCredentialFeature;
use aws_credential_types::provider::{self, error::CredentialsError};
use aws_credential_types::Credentials;
use aws_smithy_runtime::client::metrics::MetricsRuntimePlugin;
use aws_smithy_runtime::client::orchestrator::operation::Operation;
use aws_smithy_runtime::client::retries::classifiers::{
    HttpStatusCodeClassifier, TransientErrorClassifier,
};
use aws_smithy_runtime_api::client::http::HttpConnectorSettings;
use aws_smithy_runtime_api::client::interceptors::context::{Error, InterceptorContext};
use aws_smithy_runtime_api::client::orchestrator::{
    HttpResponse, Metadata, OrchestratorError, SensitiveOutput,
};
use aws_smithy_runtime_api::client::result::SdkError;
use aws_smithy_runtime_api::client::retries::classifiers::ClassifyRetry;
use aws_smithy_runtime_api::client::retries::classifiers::RetryAction;
use aws_smithy_runtime_api::client::runtime_plugin::StaticRuntimePlugin;
use aws_smithy_types::body::SdkBody;
use aws_smithy_types::config_bag::Layer;
use aws_smithy_types::retry::RetryConfig;
use aws_smithy_types::timeout::TimeoutConfig;
use http::header::{ACCEPT, AUTHORIZATION};
use http::HeaderValue;
use std::time::Duration;

const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(5);
const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(2);

#[derive(Debug)]
struct HttpProviderAuth {
    auth: Option<HeaderValue>,
}

#[derive(Debug)]
pub(crate) struct HttpCredentialProvider {
    operation: Operation<HttpProviderAuth, Credentials, CredentialsError>,
}

impl HttpCredentialProvider {
    pub(crate) fn builder() -> Builder {
        Builder::default()
    }

    pub(crate) async fn credentials(&self, auth: Option<HeaderValue>) -> provider::Result {
        let credentials =
            self.operation
                .invoke(HttpProviderAuth { auth })
                .await
                .map(|mut creds| {
                    creds
                        .get_property_mut_or_default::<Vec<AwsCredentialFeature>>()
                        .push(AwsCredentialFeature::CredentialsHttp);
                    creds
                });
        match credentials {
            Ok(creds) => Ok(creds),
            Err(SdkError::ServiceError(context)) => Err(context.into_err()),
            Err(other) => Err(CredentialsError::unhandled(other)),
        }
    }
}

#[derive(Default)]
pub(crate) struct Builder {
    provider_config: Option<ProviderConfig>,
    http_connector_settings: Option<HttpConnectorSettings>,
}

impl Builder {
    pub(crate) fn configure(mut self, provider_config: &ProviderConfig) -> Self {
        self.provider_config = Some(provider_config.clone());
        self
    }

    pub(crate) fn http_connector_settings(
        mut self,
        http_connector_settings: HttpConnectorSettings,
    ) -> Self {
        self.http_connector_settings = Some(http_connector_settings);
        self
    }

    pub(crate) fn build(
        self,
        provider_name: &'static str,
        endpoint: &str,
        path: impl Into<String>,
    ) -> HttpCredentialProvider {
        let provider_config = self.provider_config.unwrap_or_default();
        let path = path.into();

        let mut builder = Operation::builder()
            .service_name("HttpCredentialProvider")
            .operation_name("LoadCredentials")
            .with_connection_poisoning()
            .endpoint_url(endpoint)
            .no_auth()
            .timeout_config(
                TimeoutConfig::builder()
                    .connect_timeout(DEFAULT_CONNECT_TIMEOUT)
                    .read_timeout(DEFAULT_READ_TIMEOUT)
                    .build(),
            )
            .runtime_plugin(StaticRuntimePlugin::new().with_config({
                let mut layer = Layer::new("SensitiveOutput");
                layer.store_put(SensitiveOutput);
                layer.freeze()
            }))
            .runtime_plugin(
                MetricsRuntimePlugin::builder()
                    .with_scope("aws_config::http_credential_provider")
                    .with_time_source(provider_config.time_source())
                    .with_metadata(Metadata::new(path.clone(), provider_name))
                    .build()
                    .expect("All required fields have been set"),
            );
        if let Some(http_client) = provider_config.http_client() {
            builder = builder.http_client(http_client);
        }
        if let Some(sleep_impl) = provider_config.sleep_impl() {
            builder = builder
                .standard_retry(&RetryConfig::standard())
                // The following errors are retryable:
                //   - Socket errors
                //   - Networking timeouts
                //   - 5xx errors
                //   - Non-parseable 200 responses.
                .retry_classifier(HttpCredentialRetryClassifier)
                // Socket errors and network timeouts
                .retry_classifier(TransientErrorClassifier::<Error>::new())
                // 5xx errors
                .retry_classifier(HttpStatusCodeClassifier::default())
                .sleep_impl(sleep_impl);
        } else {
            builder = builder.no_retry();
        }
        let operation = builder
            .serializer(move |input: HttpProviderAuth| {
                let mut http_req = http::Request::builder()
                    .uri(path.clone())
                    .header(ACCEPT, "application/json");
                if let Some(auth) = input.auth {
                    http_req = http_req.header(AUTHORIZATION, auth);
                }
                Ok(http_req
                    .body(SdkBody::empty())
                    .expect("valid request")
                    .try_into()
                    .unwrap())
            })
            .deserializer(move |response| parse_response(provider_name, response))
            .build();
        HttpCredentialProvider { operation }
    }
}

fn parse_response(
    provider_name: &'static str,
    response: &HttpResponse,
) -> Result<Credentials, OrchestratorError<CredentialsError>> {
    if !response.status().is_success() {
        return Err(OrchestratorError::operation(
            CredentialsError::provider_error(format!(
                "Non-success status from HTTP credential provider: {:?}",
                response.status()
            )),
        ));
    }
    let resp_bytes = response.body().bytes().expect("non-streaming deserializer");
    let str_resp = std::str::from_utf8(resp_bytes)
        .map_err(|err| OrchestratorError::operation(CredentialsError::unhandled(err)))?;
    let json_creds = parse_json_credentials(str_resp)
        .map_err(|err| OrchestratorError::operation(CredentialsError::unhandled(err)))?;
    match json_creds {
        JsonCredentials::RefreshableCredentials(RefreshableCredentials {
            access_key_id,
            secret_access_key,
            session_token,
            account_id,
            expiration,
        }) => {
            let mut builder = Credentials::builder()
                .access_key_id(access_key_id)
                .secret_access_key(secret_access_key)
                .session_token(session_token)
                .expiry(expiration)
                .provider_name(provider_name);
            builder.set_account_id(account_id.map(AccountId::from));
            Ok(builder.build())
        }
        JsonCredentials::Error { code, message } => Err(OrchestratorError::operation(
            CredentialsError::provider_error(format!(
                "failed to load credentials [{code}]: {message}",
            )),
        )),
    }
}

#[derive(Clone, Debug)]
struct HttpCredentialRetryClassifier;

impl ClassifyRetry for HttpCredentialRetryClassifier {
    fn name(&self) -> &'static str {
        "HttpCredentialRetryClassifier"
    }

    fn classify_retry(&self, ctx: &InterceptorContext) -> RetryAction {
        let output_or_error = ctx.output_or_error();
        let error = match output_or_error {
            Some(Ok(_)) | None => return RetryAction::NoActionIndicated,
            Some(Err(err)) => err,
        };

        // Retry non-parseable 200 responses
        if let Some((err, status)) = error
            .as_operation_error()
            .and_then(|err| err.downcast_ref::<CredentialsError>())
            .zip(ctx.response().map(HttpResponse::status))
        {
            if matches!(err, CredentialsError::Unhandled { .. }) && status.is_success() {
                return RetryAction::server_error();
            }
        }

        RetryAction::NoActionIndicated
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aws_credential_types::credential_feature::AwsCredentialFeature;
    use aws_credential_types::provider::error::CredentialsError;
    use aws_smithy_http_client::test_util::{ReplayEvent, StaticReplayClient};
    use aws_smithy_types::body::SdkBody;
    use http::{Request, Response, Uri};
    use std::time::SystemTime;

    async fn provide_creds(
        http_client: StaticReplayClient,
    ) -> Result<Credentials, CredentialsError> {
        let provider_config = ProviderConfig::default().with_http_client(http_client.clone());
        let provider = HttpCredentialProvider::builder()
            .configure(&provider_config)
            .build("test", "http://localhost:1234/", "/some-creds");
        provider.credentials(None).await
    }

    fn successful_req_resp() -> ReplayEvent {
        ReplayEvent::new(
            Request::builder()
                .uri(Uri::from_static("http://localhost:1234/some-creds"))
                .body(SdkBody::empty())
                .unwrap(),
            Response::builder()
                .status(200)
                .body(SdkBody::from(
                    r#"{
                        "AccessKeyId" : "MUA...",
                        "SecretAccessKey" : "/7PC5om....",
                        "Token" : "AQoDY....=",
                        "Expiration" : "2016-02-25T06:03:31Z"
                    }"#,
                ))
                .unwrap(),
        )
    }

    #[tokio::test]
    async fn successful_response() {
        let http_client = StaticReplayClient::new(vec![successful_req_resp()]);
        let creds = provide_creds(http_client.clone()).await.expect("success");
        assert_eq!("MUA...", creds.access_key_id());
        assert_eq!("/7PC5om....", creds.secret_access_key());
        assert_eq!(Some("AQoDY....="), creds.session_token());
        assert_eq!(
            Some(SystemTime::UNIX_EPOCH + Duration::from_secs(1456380211)),
            creds.expiry()
        );
        http_client.assert_requests_match(&[]);
    }

    #[tokio::test]
    async fn retry_nonparseable_response() {
        let http_client = StaticReplayClient::new(vec![
            ReplayEvent::new(
                Request::builder()
                    .uri(Uri::from_static("http://localhost:1234/some-creds"))
                    .body(SdkBody::empty())
                    .unwrap(),
                Response::builder()
                    .status(200)
                    .body(SdkBody::from(r#"not json"#))
                    .unwrap(),
            ),
            successful_req_resp(),
        ]);
        let creds = provide_creds(http_client.clone()).await.expect("success");
        assert_eq!("MUA...", creds.access_key_id());
        http_client.assert_requests_match(&[]);
    }

    #[tokio::test]
    async fn retry_error_code() {
        let http_client = StaticReplayClient::new(vec![
            ReplayEvent::new(
                Request::builder()
                    .uri(Uri::from_static("http://localhost:1234/some-creds"))
                    .body(SdkBody::empty())
                    .unwrap(),
                Response::builder()
                    .status(500)
                    .body(SdkBody::from(r#"it broke"#))
                    .unwrap(),
            ),
            successful_req_resp(),
        ]);
        let creds = provide_creds(http_client.clone()).await.expect("success");
        assert_eq!("MUA...", creds.access_key_id());
        http_client.assert_requests_match(&[]);
    }

    #[tokio::test]
    async fn explicit_error_not_retryable() {
        let http_client = StaticReplayClient::new(vec![ReplayEvent::new(
            Request::builder()
                .uri(Uri::from_static("http://localhost:1234/some-creds"))
                .body(SdkBody::empty())
                .unwrap(),
            Response::builder()
                .status(400)
                .body(SdkBody::from(
                    r#"{ "Code": "Error", "Message": "There was a problem, it was your fault" }"#,
                ))
                .unwrap(),
        )]);
        let err = provide_creds(http_client.clone())
            .await
            .expect_err("it should fail");
        assert!(
            matches!(err, CredentialsError::ProviderError { .. }),
            "should be CredentialsError::ProviderError: {err}",
        );
        http_client.assert_requests_match(&[]);
    }

    #[tokio::test]
    async fn credentials_feature() {
        let http_client = StaticReplayClient::new(vec![successful_req_resp()]);
        let creds = provide_creds(http_client.clone()).await.expect("success");
        assert_eq!(
            &vec![AwsCredentialFeature::CredentialsHttp],
            creds.get_property::<Vec<AwsCredentialFeature>>().unwrap()
        );
    }
}
