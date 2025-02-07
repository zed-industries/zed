mod utils;

use crate::utils::{convert_to_async_body, convert_to_sdk_body};
use aws_smithy_runtime_api::client::http::{
    HttpClient as AwsClient, HttpConnector as AwsConnector,
    HttpConnectorFuture as AwsConnectorFuture, HttpConnectorFuture, HttpConnectorSettings,
    SharedHttpConnector,
};
use aws_smithy_runtime_api::client::orchestrator::{HttpRequest as AwsHttpRequest, HttpResponse};
use aws_smithy_runtime_api::client::result::ConnectorError;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_runtime_api::http::StatusCode;
use http_client::{HttpClient, Request};
use std::fmt;
use std::sync::Arc;
use tokio::runtime::Handle;

struct AwsHttpConnector {
    client: Arc<dyn HttpClient>,
    handle: Handle,
}

impl std::fmt::Debug for AwsHttpConnector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl AwsConnector for AwsHttpConnector {
    fn call(&self, request: AwsHttpRequest) -> AwsConnectorFuture {
        // convert AwsHttpRequest to Request<T>
        let aws_req = match request.try_into_http1x() {
            Ok(req) => req,
            Err(e) => {
                return HttpConnectorFuture::ready(Err(ConnectorError::other(e.into(), None)))
            }
        };

        let (parts, aws_body) = aws_req.into_parts();

        let coerced_body = convert_to_async_body(aws_body);

        let fut_resp = self
            .client
            .send(Request::from_parts(parts.into(), coerced_body));

        let owned_handle = self.handle.clone();

        HttpConnectorFuture::new(async move {
            let cloned_resp = match fut_resp.await {
                Ok(resp) => resp,
                Err(e) => return Err(ConnectorError::other(e.into(), None)),
            };
            let (parts, aws_body) = cloned_resp.into_parts();
            let sdk_body = convert_to_sdk_body(aws_body, owned_handle).await;

            Ok(HttpResponse::new(
                StatusCode::try_from(parts.status.as_u16()).unwrap(),
                sdk_body,
            ))
        })
    }
}

pub struct AwsHttpClient {
    client: Arc<dyn HttpClient>,
    handler: Handle,
}

impl Clone for AwsHttpClient {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            handler: self.handler.clone(),
        }
    }
}

impl std::fmt::Debug for AwsHttpClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        todo!()
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
        settings: &HttpConnectorSettings,
        components: &RuntimeComponents,
    ) -> SharedHttpConnector {
        SharedHttpConnector::new(AwsHttpConnector {
            client: self.client.clone(),
            handle: self.handler.clone(),
        })
    }
}
