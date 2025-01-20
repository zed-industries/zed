mod utils;

use std::fmt;
use std::sync::Arc;
use aws_smithy_runtime_api::client::http::{HttpConnector as AwsConnector, HttpConnectorFuture as AwsConnectorFuture, HttpClient as AwsClient, HttpConnectorSettings, SharedHttpConnector, HttpConnectorFuture};
use aws_smithy_runtime_api::client::orchestrator::{HttpRequest as AwsHttpRequest, HttpResponse};
use aws_smithy_runtime_api::client::result::ConnectorError;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_runtime_api::shared::IntoShared;
use http_client::{HttpClient, Request};
use crate::utils::{convert_to_async_body, convert_to_sdk_body};

struct AwsHttpConnector {
    client: Arc<dyn HttpClient>
}

impl std::fmt::Debug for AwsHttpConnector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl AwsConnector for AwsHttpConnector {
    fn call(&self, request: AwsHttpRequest) -> AwsConnectorFuture {
        // convert AwsHttpRequest to Request<T>
        let mut aws_req = match request.try_into_http1x() {
            Ok(req) => req,
            Err(e) => return HttpConnectorFuture::ready(Err(ConnectorError::other(e.into(), None))),
        };

        let (mut parts, aws_body) = aws_req.into_parts();

        let coerced_body = convert_to_async_body(aws_body);

        let fut_resp = self.client.send(Request::from_parts(parts.into(), coerced_body));

        HttpConnectorFuture::new(async move {
            let response = fut_resp
                .await
                .map_err(|e| ConnectorError::other(e.into(), None))?
                .map(|b| convert_to_sdk_body(b));
            match HttpResponse::try_from(response) {
                Ok(response) => Ok(response),
                Err(err) => Err(ConnectorError::other(err.into(), None)),
            }
        })

    }
}

pub struct AwsHttpClient {
    client: Arc<dyn HttpClient>
}

impl std::fmt::Debug for AwsHttpClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl AwsHttpClient {
    pub fn new(client: Arc<dyn HttpClient>) -> Self {
        Self { client }
    }
}

impl AwsClient for AwsHttpClient {
    fn http_connector(&self, settings: &HttpConnectorSettings, components: &RuntimeComponents) -> SharedHttpConnector {
        SharedHttpConnector::new(AwsHttpConnector {
            client: self.client.clone()
        })
    }
}