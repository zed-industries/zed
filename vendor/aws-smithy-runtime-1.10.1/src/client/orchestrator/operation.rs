/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::client::auth::no_auth::{NoAuthScheme, NO_AUTH_SCHEME_ID};
use crate::client::defaults::{default_plugins, DefaultPluginParams};
use crate::client::http::connection_poisoning::ConnectionPoisoningInterceptor;
use crate::client::identity::no_auth::NoAuthIdentityResolver;
use crate::client::identity::IdentityCache;
use crate::client::orchestrator::endpoints::StaticUriEndpointResolver;
use crate::client::retries::strategy::{NeverRetryStrategy, StandardRetryStrategy};
use aws_smithy_async::rt::sleep::AsyncSleep;
use aws_smithy_async::time::TimeSource;
use aws_smithy_runtime_api::client::auth::static_resolver::StaticAuthSchemeOptionResolver;
use aws_smithy_runtime_api::client::auth::{
    AuthSchemeOptionResolverParams, SharedAuthScheme, SharedAuthSchemeOptionResolver,
};
use aws_smithy_runtime_api::client::endpoint::{EndpointResolverParams, SharedEndpointResolver};
use aws_smithy_runtime_api::client::http::HttpClient;
use aws_smithy_runtime_api::client::identity::SharedIdentityResolver;
use aws_smithy_runtime_api::client::interceptors::context::{Error, Input, Output};
use aws_smithy_runtime_api::client::interceptors::Intercept;
use aws_smithy_runtime_api::client::orchestrator::{HttpRequest, OrchestratorError};
use aws_smithy_runtime_api::client::orchestrator::{HttpResponse, Metadata};
use aws_smithy_runtime_api::client::result::SdkError;
use aws_smithy_runtime_api::client::retries::classifiers::ClassifyRetry;
use aws_smithy_runtime_api::client::retries::SharedRetryStrategy;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder;
use aws_smithy_runtime_api::client::runtime_plugin::{
    RuntimePlugin, RuntimePlugins, SharedRuntimePlugin, StaticRuntimePlugin,
};
use aws_smithy_runtime_api::client::ser_de::{
    DeserializeResponse, SerializeRequest, SharedRequestSerializer, SharedResponseDeserializer,
};
use aws_smithy_runtime_api::shared::IntoShared;
use aws_smithy_runtime_api::{
    box_error::BoxError, client::stalled_stream_protection::StalledStreamProtectionConfig,
};
use aws_smithy_types::config_bag::{ConfigBag, Layer};
use aws_smithy_types::retry::RetryConfig;
use aws_smithy_types::timeout::TimeoutConfig;
use std::borrow::Cow;
use std::fmt;
use std::marker::PhantomData;
use tracing::{debug_span, Instrument};

struct FnSerializer<F, I> {
    f: F,
    _phantom: PhantomData<I>,
}
impl<F, I> FnSerializer<F, I> {
    fn new(f: F) -> Self {
        Self {
            f,
            _phantom: Default::default(),
        }
    }
}
impl<F, I> SerializeRequest for FnSerializer<F, I>
where
    F: Fn(I) -> Result<HttpRequest, BoxError> + Send + Sync,
    I: fmt::Debug + Send + Sync + 'static,
{
    fn serialize_input(&self, input: Input, _cfg: &mut ConfigBag) -> Result<HttpRequest, BoxError> {
        let input: I = input.downcast().expect("correct type");
        (self.f)(input)
    }
}
impl<F, I> fmt::Debug for FnSerializer<F, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FnSerializer")
    }
}

struct FnDeserializer<F, O, E> {
    f: F,
    _phantom: PhantomData<(O, E)>,
}
impl<F, O, E> FnDeserializer<F, O, E> {
    fn new(deserializer: F) -> Self {
        Self {
            f: deserializer,
            _phantom: Default::default(),
        }
    }
}
impl<F, O, E> DeserializeResponse for FnDeserializer<F, O, E>
where
    F: Fn(&HttpResponse) -> Result<O, OrchestratorError<E>> + Send + Sync,
    O: fmt::Debug + Send + Sync + 'static,
    E: std::error::Error + fmt::Debug + Send + Sync + 'static,
{
    fn deserialize_nonstreaming(
        &self,
        response: &HttpResponse,
    ) -> Result<Output, OrchestratorError<Error>> {
        (self.f)(response)
            .map(|output| Output::erase(output))
            .map_err(|err| err.map_operation_error(Error::erase))
    }
}
impl<F, O, E> fmt::Debug for FnDeserializer<F, O, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FnDeserializer")
    }
}

/// Orchestrates execution of a HTTP request without any modeled input or output.
#[derive(Debug)]
pub struct Operation<I, O, E> {
    service_name: Cow<'static, str>,
    operation_name: Cow<'static, str>,
    runtime_plugins: RuntimePlugins,
    _phantom: PhantomData<(I, O, E)>,
}

// Manual Clone implementation needed to get rid of Clone bounds on I, O, and E
impl<I, O, E> Clone for Operation<I, O, E> {
    fn clone(&self) -> Self {
        Self {
            service_name: self.service_name.clone(),
            operation_name: self.operation_name.clone(),
            runtime_plugins: self.runtime_plugins.clone(),
            _phantom: self._phantom,
        }
    }
}

impl Operation<(), (), ()> {
    /// Returns a new `OperationBuilder` for the `Operation`.
    pub fn builder() -> OperationBuilder {
        OperationBuilder::new()
    }
}

impl<I, O, E> Operation<I, O, E>
where
    I: fmt::Debug + Send + Sync + 'static,
    O: fmt::Debug + Send + Sync + 'static,
    E: std::error::Error + fmt::Debug + Send + Sync + 'static,
{
    /// Invokes this `Operation` with the given `input` and returns either an output for success
    /// or an [`SdkError`] for failure
    pub async fn invoke(&self, input: I) -> Result<O, SdkError<E, HttpResponse>> {
        let input = Input::erase(input);

        let output = super::invoke(
            &self.service_name,
            &self.operation_name,
            input,
            &self.runtime_plugins,
        )
        .instrument(debug_span!(
            "invoke",
            "rpc.service" = &self.service_name.as_ref(),
            "rpc.method" = &self.operation_name.as_ref()
        ))
        .await
        .map_err(|err| err.map_service_error(|e| e.downcast().expect("correct type")))?;

        Ok(output.downcast().expect("correct type"))
    }
}

/// Builder for [`Operation`].
#[derive(Debug)]
pub struct OperationBuilder<I = (), O = (), E = ()> {
    service_name: Option<Cow<'static, str>>,
    operation_name: Option<Cow<'static, str>>,
    config: Layer,
    runtime_components: RuntimeComponentsBuilder,
    runtime_plugins: Vec<SharedRuntimePlugin>,
    _phantom: PhantomData<(I, O, E)>,
}

impl Default for OperationBuilder<(), (), ()> {
    fn default() -> Self {
        Self::new()
    }
}

impl OperationBuilder<(), (), ()> {
    /// Creates a new [`OperationBuilder`].
    pub fn new() -> Self {
        Self {
            service_name: None,
            operation_name: None,
            config: Layer::new("operation"),
            runtime_components: RuntimeComponentsBuilder::new("operation"),
            runtime_plugins: Vec::new(),
            _phantom: Default::default(),
        }
    }
}

impl<I, O, E> OperationBuilder<I, O, E> {
    /// Configures the service name for the builder.
    pub fn service_name(mut self, service_name: impl Into<Cow<'static, str>>) -> Self {
        self.service_name = Some(service_name.into());
        self
    }

    /// Configures the operation name for the builder.
    pub fn operation_name(mut self, operation_name: impl Into<Cow<'static, str>>) -> Self {
        self.operation_name = Some(operation_name.into());
        self
    }

    /// Configures the http client for the builder.
    pub fn http_client(mut self, connector: impl HttpClient + 'static) -> Self {
        self.runtime_components.set_http_client(Some(connector));
        self
    }

    /// Configures the endpoint URL for the builder.
    pub fn endpoint_url(mut self, url: &str) -> Self {
        self.config.store_put(EndpointResolverParams::new(()));
        self.runtime_components
            .set_endpoint_resolver(Some(SharedEndpointResolver::new(
                StaticUriEndpointResolver::uri(url),
            )));
        self
    }

    /// Configures the retry classifier for the builder.
    pub fn retry_classifier(mut self, retry_classifier: impl ClassifyRetry + 'static) -> Self {
        self.runtime_components
            .push_retry_classifier(retry_classifier);
        self
    }

    /// Disables the retry for the operation.
    pub fn no_retry(mut self) -> Self {
        self.runtime_components
            .set_retry_strategy(Some(SharedRetryStrategy::new(NeverRetryStrategy::new())));
        self
    }

    /// Configures the standard retry for the builder.
    pub fn standard_retry(mut self, retry_config: &RetryConfig) -> Self {
        self.config.store_put(retry_config.clone());
        self.runtime_components
            .set_retry_strategy(Some(SharedRetryStrategy::new(StandardRetryStrategy::new())));
        self
    }

    /// Configures the timeout configuration for the builder.
    pub fn timeout_config(mut self, timeout_config: TimeoutConfig) -> Self {
        self.config.store_put(timeout_config);
        self
    }

    /// Disables auth for the operation.
    pub fn no_auth(mut self) -> Self {
        self.config
            .store_put(AuthSchemeOptionResolverParams::new(()));
        self.runtime_components
            .set_auth_scheme_option_resolver(Some(SharedAuthSchemeOptionResolver::new(
                StaticAuthSchemeOptionResolver::new(vec![NO_AUTH_SCHEME_ID]),
            )));
        self.runtime_components
            .push_auth_scheme(SharedAuthScheme::new(NoAuthScheme::default()));
        self.runtime_components
            .set_identity_cache(Some(IdentityCache::no_cache()));
        self.runtime_components.set_identity_resolver(
            NO_AUTH_SCHEME_ID,
            SharedIdentityResolver::new(NoAuthIdentityResolver::new()),
        );
        self
    }

    /// Configures the sleep for the builder.
    pub fn sleep_impl(mut self, async_sleep: impl AsyncSleep + 'static) -> Self {
        self.runtime_components
            .set_sleep_impl(Some(async_sleep.into_shared()));
        self
    }

    /// Configures the time source for the builder.
    pub fn time_source(mut self, time_source: impl TimeSource + 'static) -> Self {
        self.runtime_components
            .set_time_source(Some(time_source.into_shared()));
        self
    }

    /// Configures the interceptor for the builder.
    pub fn interceptor(mut self, interceptor: impl Intercept + 'static) -> Self {
        self.runtime_components.push_interceptor(interceptor);
        self
    }

    /// Registers the [`ConnectionPoisoningInterceptor`].
    pub fn with_connection_poisoning(self) -> Self {
        self.interceptor(ConnectionPoisoningInterceptor::new())
    }

    /// Configures the runtime plugin for the builder.
    pub fn runtime_plugin(mut self, runtime_plugin: impl RuntimePlugin + 'static) -> Self {
        self.runtime_plugins.push(runtime_plugin.into_shared());
        self
    }

    /// Configures stalled stream protection with the given config.
    pub fn stalled_stream_protection(
        mut self,
        stalled_stream_protection: StalledStreamProtectionConfig,
    ) -> Self {
        self.config.store_put(stalled_stream_protection);
        self
    }

    /// Configures the serializer for the builder.
    pub fn serializer<I2>(
        mut self,
        serializer: impl Fn(I2) -> Result<HttpRequest, BoxError> + Send + Sync + 'static,
    ) -> OperationBuilder<I2, O, E>
    where
        I2: fmt::Debug + Send + Sync + 'static,
    {
        self.config
            .store_put(SharedRequestSerializer::new(FnSerializer::new(serializer)));
        OperationBuilder {
            service_name: self.service_name,
            operation_name: self.operation_name,
            config: self.config,
            runtime_components: self.runtime_components,
            runtime_plugins: self.runtime_plugins,
            _phantom: Default::default(),
        }
    }

    /// Configures the deserializer for the builder.
    pub fn deserializer<O2, E2>(
        mut self,
        deserializer: impl Fn(&HttpResponse) -> Result<O2, OrchestratorError<E2>>
            + Send
            + Sync
            + 'static,
    ) -> OperationBuilder<I, O2, E2>
    where
        O2: fmt::Debug + Send + Sync + 'static,
        E2: std::error::Error + fmt::Debug + Send + Sync + 'static,
    {
        self.config
            .store_put(SharedResponseDeserializer::new(FnDeserializer::new(
                deserializer,
            )));
        OperationBuilder {
            service_name: self.service_name,
            operation_name: self.operation_name,
            config: self.config,
            runtime_components: self.runtime_components,
            runtime_plugins: self.runtime_plugins,
            _phantom: Default::default(),
        }
    }

    /// Configures the a deserializer implementation for the builder.
    #[allow(clippy::implied_bounds_in_impls)] // for `Send` and `Sync`
    pub fn deserializer_impl<O2, E2>(
        mut self,
        deserializer: impl DeserializeResponse + Send + Sync + 'static,
    ) -> OperationBuilder<I, O2, E2>
    where
        O2: fmt::Debug + Send + Sync + 'static,
        E2: std::error::Error + fmt::Debug + Send + Sync + 'static,
    {
        let deserializer: SharedResponseDeserializer = deserializer.into_shared();
        self.config.store_put(deserializer);

        OperationBuilder {
            service_name: self.service_name,
            operation_name: self.operation_name,
            config: self.config,
            runtime_components: self.runtime_components,
            runtime_plugins: self.runtime_plugins,
            _phantom: Default::default(),
        }
    }

    /// Creates an `Operation` from the builder.
    pub fn build(self) -> Operation<I, O, E> {
        let service_name = self.service_name.expect("service_name required");
        let operation_name = self.operation_name.expect("operation_name required");
        let mut config = self.config;
        config.store_put(Metadata::new(operation_name.clone(), service_name.clone()));
        let mut runtime_plugins = RuntimePlugins::new()
            .with_client_plugins(default_plugins(
                DefaultPluginParams::new().with_retry_partition_name(service_name.clone()),
            ))
            .with_client_plugin(
                StaticRuntimePlugin::new()
                    .with_config(config.freeze())
                    .with_runtime_components(self.runtime_components),
            );
        for runtime_plugin in self.runtime_plugins {
            runtime_plugins = runtime_plugins.with_client_plugin(runtime_plugin);
        }

        #[cfg(debug_assertions)]
        {
            let mut config = ConfigBag::base();
            let components = runtime_plugins
                .apply_client_configuration(&mut config)
                .expect("the runtime plugins should succeed");

            assert!(
                components.http_client().is_some(),
                "a http_client is required. Enable the `default-https-client` crate feature or configure an HTTP client to fix this."
            );
            assert!(
                components.endpoint_resolver().is_some(),
                "a endpoint_resolver is required"
            );
            assert!(
                components.retry_strategy().is_some(),
                "a retry_strategy is required"
            );
            assert!(
                config.load::<SharedRequestSerializer>().is_some(),
                "a serializer is required"
            );
            assert!(
                config.load::<SharedResponseDeserializer>().is_some(),
                "a deserializer is required"
            );
            assert!(
                config.load::<EndpointResolverParams>().is_some(),
                "endpoint resolver params are required"
            );
            assert!(
                config.load::<TimeoutConfig>().is_some(),
                "timeout config is required"
            );
        }

        Operation {
            service_name,
            operation_name,
            runtime_plugins,
            _phantom: Default::default(),
        }
    }
}

#[cfg(all(test, any(feature = "test-util", feature = "legacy-test-util")))]
mod tests {
    use super::*;
    use crate::client::retries::classifiers::HttpStatusCodeClassifier;
    use aws_smithy_async::rt::sleep::{SharedAsyncSleep, TokioSleep};
    use aws_smithy_http_client::test_util::{capture_request, ReplayEvent, StaticReplayClient};
    use aws_smithy_runtime_api::client::result::ConnectorError;
    use aws_smithy_types::body::SdkBody;
    use std::convert::Infallible;

    #[tokio::test]
    async fn operation() {
        let (connector, request_rx) = capture_request(Some(
            http_1x::Response::builder()
                .status(418)
                .body(SdkBody::from(&b"I'm a teapot!"[..]))
                .unwrap(),
        ));
        let operation = Operation::builder()
            .service_name("test")
            .operation_name("test")
            .http_client(connector)
            .endpoint_url("http://localhost:1234")
            .no_auth()
            .no_retry()
            .timeout_config(TimeoutConfig::disabled())
            .serializer(|input: String| Ok(HttpRequest::new(SdkBody::from(input.as_bytes()))))
            .deserializer::<_, Infallible>(|response| {
                assert_eq!(418, u16::from(response.status()));
                Ok(std::str::from_utf8(response.body().bytes().unwrap())
                    .unwrap()
                    .to_string())
            })
            .build();

        let output = operation
            .invoke("what are you?".to_string())
            .await
            .expect("success");
        assert_eq!("I'm a teapot!", output);

        let request = request_rx.expect_request();
        assert_eq!("http://localhost:1234/", request.uri());
        assert_eq!(b"what are you?", request.body().bytes().unwrap());
    }

    #[tokio::test]
    async fn operation_retries() {
        let connector = StaticReplayClient::new(vec![
            ReplayEvent::new(
                http_1x::Request::builder()
                    .uri("http://localhost:1234/")
                    .body(SdkBody::from(&b"what are you?"[..]))
                    .unwrap(),
                http_1x::Response::builder()
                    .status(503)
                    .body(SdkBody::from(&b""[..]))
                    .unwrap(),
            ),
            ReplayEvent::new(
                http_1x::Request::builder()
                    .uri("http://localhost:1234/")
                    .body(SdkBody::from(&b"what are you?"[..]))
                    .unwrap(),
                http_1x::Response::builder()
                    .status(418)
                    .body(SdkBody::from(&b"I'm a teapot!"[..]))
                    .unwrap(),
            ),
        ]);
        let operation = Operation::builder()
            .service_name("test")
            .operation_name("test")
            .http_client(connector.clone())
            .endpoint_url("http://localhost:1234")
            .no_auth()
            .standard_retry(&RetryConfig::standard())
            .retry_classifier(HttpStatusCodeClassifier::default())
            .timeout_config(TimeoutConfig::disabled())
            .sleep_impl(SharedAsyncSleep::new(TokioSleep::new()))
            .serializer(|input: String| Ok(HttpRequest::new(SdkBody::from(input.as_bytes()))))
            .deserializer::<_, Infallible>(|response| {
                if u16::from(response.status()) == 503 {
                    Err(OrchestratorError::connector(ConnectorError::io(
                        "test".into(),
                    )))
                } else {
                    assert_eq!(418, u16::from(response.status()));
                    Ok(std::str::from_utf8(response.body().bytes().unwrap())
                        .unwrap()
                        .to_string())
                }
            })
            .build();

        let output = operation
            .invoke("what are you?".to_string())
            .await
            .expect("success");
        assert_eq!("I'm a teapot!", output);

        connector.assert_requests_match(&[]);
    }
}
