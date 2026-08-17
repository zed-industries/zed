/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_runtime_api::client::connector_metadata::ConnectorMetadata;
use aws_smithy_runtime_api::client::http::{
    HttpClient, HttpConnector, HttpConnectorFuture, HttpConnectorSettings, SharedHttpClient,
    SharedHttpConnector,
};
use aws_smithy_runtime_api::client::orchestrator::{HttpRequest, HttpResponse};
use aws_smithy_runtime_api::client::result::ConnectorError;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_runtime_api::shared::IntoShared;
use aws_smithy_types::body::SdkBody;
use std::fmt;
use std::sync::Arc;

/// Create a [`SharedHttpClient`] from `Fn(http:Request) -> http::Response`
///
/// # Examples
///
/// ```rust
/// # use http_1x as http;
/// use aws_smithy_http_client::test_util::infallible_client_fn;
/// let http_client = infallible_client_fn(|_req| http::Response::builder().status(200).body("OK!").unwrap());
/// ```
pub fn infallible_client_fn<B>(
    f: impl Fn(http_1x::Request<SdkBody>) -> http_1x::Response<B> + Send + Sync + 'static,
) -> SharedHttpClient
where
    B: Into<SdkBody>,
{
    InfallibleClientFn::new(f).into_shared()
}

#[derive(Clone)]
struct InfallibleClientFn {
    #[allow(clippy::type_complexity)]
    response: Arc<
        dyn Fn(http_1x::Request<SdkBody>) -> Result<http_1x::Response<SdkBody>, ConnectorError>
            + Send
            + Sync,
    >,
}

impl fmt::Debug for InfallibleClientFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InfallibleClientFn").finish()
    }
}

impl InfallibleClientFn {
    fn new<B: Into<SdkBody>>(
        f: impl Fn(http_1x::Request<SdkBody>) -> http_1x::Response<B> + Send + Sync + 'static,
    ) -> Self {
        Self {
            response: Arc::new(move |request| Ok(f(request).map(|b| b.into()))),
        }
    }
}

impl HttpConnector for InfallibleClientFn {
    fn call(&self, request: HttpRequest) -> HttpConnectorFuture {
        HttpConnectorFuture::ready(
            (self.response)(request.try_into_http1x().unwrap())
                .map(|res| HttpResponse::try_from(res).unwrap()),
        )
    }
}

impl HttpClient for InfallibleClientFn {
    fn http_connector(
        &self,
        _: &HttpConnectorSettings,
        _: &RuntimeComponents,
    ) -> SharedHttpConnector {
        self.clone().into_shared()
    }

    fn connector_metadata(&self) -> Option<ConnectorMetadata> {
        Some(ConnectorMetadata::new("infallible-client", None))
    }
}
