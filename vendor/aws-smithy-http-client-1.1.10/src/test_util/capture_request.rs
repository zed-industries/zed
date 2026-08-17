/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_runtime_api::client::connector_metadata::ConnectorMetadata;
use aws_smithy_runtime_api::client::http::{
    HttpClient, HttpConnector, HttpConnectorFuture, HttpConnectorSettings, SharedHttpConnector,
};
use aws_smithy_runtime_api::client::orchestrator::{HttpRequest, HttpResponse};
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_runtime_api::http::HttpError;
use aws_smithy_runtime_api::shared::IntoShared;
use aws_smithy_types::body::SdkBody;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;

#[derive(Debug)]
struct Inner {
    response: Option<HttpResponse>,
    sender: Option<oneshot::Sender<HttpRequest>>,
}

/// Test Connection to capture a single request
#[derive(Debug, Clone)]
pub struct CaptureRequestHandler(Arc<Mutex<Inner>>);

impl HttpConnector for CaptureRequestHandler {
    fn call(&self, request: HttpRequest) -> HttpConnectorFuture {
        let mut inner = self.0.lock().unwrap();
        if let Err(_e) = inner.sender.take().expect("already sent").send(request) {
            tracing::trace!("The receiver was already dropped");
        }
        HttpConnectorFuture::ready(Ok(inner
            .response
            .take()
            .expect("could not handle second request")))
    }
}

impl HttpClient for CaptureRequestHandler {
    fn http_connector(
        &self,
        _: &HttpConnectorSettings,
        _: &RuntimeComponents,
    ) -> SharedHttpConnector {
        self.clone().into_shared()
    }

    fn connector_metadata(&self) -> Option<ConnectorMetadata> {
        Some(ConnectorMetadata::new("capture-request-handler", None))
    }
}

/// Receiver for [`CaptureRequestHandler`].
#[derive(Debug)]
pub struct CaptureRequestReceiver {
    receiver: oneshot::Receiver<HttpRequest>,
}

impl CaptureRequestReceiver {
    /// Expect that a request was sent. Returns the captured request.
    ///
    /// # Panics
    /// If no request was received
    #[track_caller]
    pub fn expect_request(mut self) -> HttpRequest {
        self.receiver.try_recv().expect("no request was received")
    }

    /// Expect that no request was captured. Panics if a request was received.
    ///
    /// # Panics
    /// If a request was received
    #[track_caller]
    pub fn expect_no_request(mut self) {
        self.receiver
            .try_recv()
            .expect_err("expected no request to be received!");
    }
}

/// Test connection used to capture a single request
///
/// If response is `None`, it will reply with a 200 response with an empty body
///
/// Example:
/// ```compile_fail
/// let (capture_client, request) = capture_request(None);
/// let conf = aws_sdk_sts::Config::builder()
///     .http_client(capture_client)
///     .build();
/// let client = aws_sdk_sts::Client::from_conf(conf);
/// let _ = client.assume_role_with_saml().send().await;
/// // web identity should be unsigned
/// assert_eq!(
///     request.expect_request().headers().get("AUTHORIZATION"),
///     None
/// );
/// ```
pub fn capture_request(
    response: Option<http_1x::Response<SdkBody>>,
) -> (CaptureRequestHandler, CaptureRequestReceiver) {
    capture_request_inner(response)
}

fn capture_request_inner(
    response: Option<impl TryInto<HttpResponse, Error = HttpError>>,
) -> (CaptureRequestHandler, CaptureRequestReceiver) {
    let (tx, rx) = oneshot::channel();
    let http_resp: HttpResponse = match response {
        Some(resp) => resp.try_into().expect("valid HttpResponse"),
        None => http_1x::Response::builder()
            .status(200)
            .body(SdkBody::empty())
            .expect("unreachable")
            .try_into()
            .expect("unreachable"),
    };
    (
        CaptureRequestHandler(Arc::new(Mutex::new(Inner {
            response: Some(http_resp),
            sender: Some(tx),
        }))),
        CaptureRequestReceiver { receiver: rx },
    )
}

#[allow(missing_docs)]
#[cfg(feature = "legacy-test-util")]
pub fn legacy_capture_request(
    response: Option<http_02x::Response<SdkBody>>,
) -> (CaptureRequestHandler, CaptureRequestReceiver) {
    capture_request_inner(response)
}

#[cfg(test)]
mod test {
    use aws_smithy_runtime_api::client::http::HttpConnector;
    use aws_smithy_runtime_api::client::orchestrator::HttpRequest;
    use aws_smithy_types::body::SdkBody;

    #[cfg(feature = "legacy-test-util")]
    #[tokio::test]
    async fn test_can_plug_in_http_02x() {
        use super::legacy_capture_request;
        let (capture_client, _request) = legacy_capture_request(Some(
            http_02x::Response::builder()
                .status(202)
                .body(SdkBody::empty())
                .expect("unreachable"),
        ));

        let resp = capture_client.call(HttpRequest::empty()).await.unwrap();
        assert_eq!(202, resp.status().as_u16());
    }

    #[tokio::test]
    async fn test_can_plug_in_http_1x() {
        use super::capture_request;
        let (capture_client, _request) = capture_request(Some(
            http_1x::Response::builder()
                .status(202)
                .body(SdkBody::empty())
                .expect("unreachable"),
        ));

        let resp = capture_client.call(HttpRequest::empty()).await.unwrap();
        assert_eq!(202, resp.status().as_u16());
    }
}
