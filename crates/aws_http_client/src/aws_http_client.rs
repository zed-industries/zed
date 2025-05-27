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
use futures::AsyncReadExt;
use http_client::{AsyncBody, Inner};
use http_client::{HttpClient, Request};
use tokio::runtime::Handle;

struct AwsHttpConnector {
    client: Arc<dyn HttpClient>,
    handle: Handle,
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

        let handle = self.handle.clone();

        HttpConnectorFuture::new(async move {
            let response = match response.await {
                Ok(response) => response,
                Err(err) => return Err(ConnectorError::other(err.into(), None)),
            };
            let (parts, body) = response.into_parts();
            let body = convert_to_sdk_body(body, handle).await;

            let mut response =
                HttpResponse::new(StatusCode::try_from(parts.status.as_u16()).unwrap(), body);

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
    handler: Handle,
}

impl std::fmt::Debug for AwsHttpClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AwsHttpClient").finish()
    }
}

impl AwsHttpClient {
    pub fn new(client: Arc<dyn HttpClient>, handle: Handle) -> Self {
        Self {
            client,
            handler: handle,
        }
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
            handle: self.handler.clone(),
        })
    }
}

pub async fn convert_to_sdk_body(body: AsyncBody, handle: Handle) -> SdkBody {
    match body.0 {
        Inner::Empty => SdkBody::empty(),
        Inner::Bytes(bytes) => SdkBody::from(bytes.into_inner()),
        Inner::AsyncReader(mut reader) => {
            let buffer = handle.spawn(async move {
                let mut buffer = Vec::new();
                let _ = reader.read_to_end(&mut buffer).await;
                buffer
            });

            SdkBody::from(buffer.await.unwrap_or_default())
        }
    }
}

pub fn convert_to_async_body(body: SdkBody) -> AsyncBody {
    match body.bytes() {
        Some(bytes) => AsyncBody::from((*bytes).to_vec()),
        None => AsyncBody::empty(),
    }
}
