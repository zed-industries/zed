use std::fmt;
use std::sync::Arc;

use aws_smithy_runtime_api::client::http::{
    HttpClient as AwsClient, HttpConnector as AwsConnector,
    HttpConnectorFuture as AwsConnectorFuture, HttpConnectorFuture, HttpConnectorSettings,
    SharedHttpConnector,
};
use aws_smithy_runtime_api::client::orchestrator::{HttpRequest as AwsHttpRequest, HttpResponse};
use aws_smithy_runtime_api::client::result::ConnectorError;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_runtime_api::http::{Headers, StatusCode};
use aws_smithy_types::body::SdkBody;
use http_client::AsyncBody;
use http_client::{HttpClient, Request};

struct AwsHttpConnector {
    client: Arc<dyn HttpClient>,
}

impl std::fmt::Debug for AwsHttpConnector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AwsHttpConnector").finish()
    }
}

impl AwsConnector for AwsHttpConnector {
    fn call(&self, request: AwsHttpRequest) -> AwsConnectorFuture {
        let req = match request.try_into_http1x() {
            Ok(req) => req,
            Err(err) => {
                return HttpConnectorFuture::ready(Err(ConnectorError::other(err.into(), None)));
            }
        };

        let (parts, body) = req.into_parts();

        let response = self
            .client
            .send(Request::from_parts(parts, convert_to_async_body(body)));

        HttpConnectorFuture::new(async move {
            let response = match response.await {
                Ok(response) => response,
                Err(err) => return Err(ConnectorError::other(err.into(), None)),
            };
            let (parts, body) = response.into_parts();

            let mut response = HttpResponse::new(
                StatusCode::try_from(parts.status.as_u16()).unwrap(),
                convert_to_sdk_body(body),
            );

            let headers = match Headers::try_from(parts.headers) {
                Ok(headers) => headers,
                Err(err) => return Err(ConnectorError::other(err.into(), None)),
            };

            *response.headers_mut() = headers;

            Ok(response)
        })
    }
}

#[derive(Clone)]
pub struct AwsHttpClient {
    client: Arc<dyn HttpClient>,
}

impl std::fmt::Debug for AwsHttpClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AwsHttpClient").finish()
    }
}

impl AwsHttpClient {
    pub fn new(client: Arc<dyn HttpClient>) -> Self {
        Self { client }
    }
}

impl AwsClient for AwsHttpClient {
    fn http_connector(
        &self,
        _settings: &HttpConnectorSettings,
        _components: &RuntimeComponents,
    ) -> SharedHttpConnector {
        SharedHttpConnector::new(AwsHttpConnector {
            client: self.client.clone(),
        })
    }
}

pub fn convert_to_sdk_body(body: AsyncBody) -> SdkBody {
    SdkBody::from_body_1_x(body)
}

pub fn convert_to_async_body(body: SdkBody) -> AsyncBody {
    match body.bytes() {
        Some(bytes) => AsyncBody::from((*bytes).to_vec()),
        None => AsyncBody::empty(),
    }
}
