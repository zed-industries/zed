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
        f.debug_struct("AwsHttpConnector").finish()
    }
}

impl AwsConnector for AwsHttpConnector {
    fn call(&self, request: AwsHttpRequest) -> AwsConnectorFuture {
        let aws_req = match request.try_into_http1x() {
            Ok(req) => req,
            Err(err) => {
                return HttpConnectorFuture::ready(Err(ConnectorError::other(err.into(), None)))
            }
        };

        let (parts, aws_body) = aws_req.into_parts();

        let body = convert_to_async_body(aws_body);

        let response = self.client.send(Request::from_parts(parts.into(), body));

        let handle = self.handle.clone();

        HttpConnectorFuture::new(async move {
            let response = match response.await {
                Ok(response) => response,
                Err(err) => return Err(ConnectorError::other(err.into(), None)),
            };
            let (parts, aws_body) = response.into_parts();
            let sdk_body = convert_to_sdk_body(aws_body, handle).await;

            Ok(HttpResponse::new(
                StatusCode::try_from(parts.status.as_u16()).unwrap(),
                sdk_body,
            ))
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
