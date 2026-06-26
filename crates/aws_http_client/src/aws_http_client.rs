use std::fmt;
use std::sync::Arc;
use std::time::SystemTime;

use anyhow::Context as _;
use aws_credential_types::Credentials;
use aws_sigv4::http_request::{SignableBody, SignableRequest, SigningSettings, sign};
use aws_sigv4::sign::v4;
use aws_smithy_runtime_api::client::http::{
    HttpClient as AwsClient, HttpConnector as AwsConnector,
    HttpConnectorFuture as AwsConnectorFuture, HttpConnectorFuture, HttpConnectorSettings,
    SharedHttpConnector,
};
use aws_smithy_runtime_api::client::identity::Identity;
use aws_smithy_runtime_api::client::orchestrator::{HttpRequest as AwsHttpRequest, HttpResponse};
use aws_smithy_runtime_api::client::result::ConnectorError;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_runtime_api::http::{Headers, StatusCode};
use aws_smithy_types::body::SdkBody;
use http_client::AsyncBody;
use http_client::{HttpClient, Request, http};

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

    /// Returns the underlying [`http_client::HttpClient`]. Useful for callers
    /// that need to issue requests directly (e.g. the Bedrock Mantle path)
    /// rather than through an AWS SDK client.
    pub fn client(&self) -> Arc<dyn HttpClient> {
        self.client.clone()
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

/// Signs a raw HTTP request with AWS Signature Version 4, stamping the
/// `authorization`, `x-amz-date`, and (when a session token is present)
/// `x-amz-security-token` headers onto the request in place.
///
/// This exists so callers that issue requests directly through an
/// [`http_client::HttpClient`] (rather than through an AWS SDK client) can still
/// authenticate against AWS services. `body` must be the exact bytes that will
/// be sent, since the payload hash is part of the signature.
pub fn sign_request_sigv4(
    request: &mut http::Request<AsyncBody>,
    body: &[u8],
    access_key_id: &str,
    secret_access_key: &str,
    session_token: Option<&str>,
    region: &str,
    service: &str,
) -> anyhow::Result<()> {
    // SigV4 requires the `host` header to be present and signed. Derive it from
    // the request URI so it matches what the transport will ultimately send.
    if !request.headers().contains_key(http::header::HOST)
        && let Some(authority) = request.uri().authority()
    {
        let host = http::HeaderValue::from_str(authority.as_str())
            .context("invalid host header derived from request URI")?;
        request.headers_mut().insert(http::header::HOST, host);
    }

    let identity: Identity = Credentials::new(
        access_key_id,
        secret_access_key,
        session_token.map(str::to_string),
        None,
        "zed-aws-sigv4",
    )
    .into();

    let signing_params: aws_sigv4::http_request::SigningParams = v4::SigningParams::builder()
        .identity(&identity)
        .region(region)
        .name(service)
        .time(SystemTime::now())
        .settings(SigningSettings::default())
        .build()
        .context("building SigV4 signing params")?
        .into();

    let method = request.method().as_str();
    let uri = request.uri().to_string();
    let headers = request
        .headers()
        .iter()
        .map(|(name, value)| {
            value
                .to_str()
                .map(|value| (name.as_str(), value))
                .with_context(|| format!("header {name} is not valid UTF-8 and cannot be signed"))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let signable_request =
        SignableRequest::new(method, uri, headers.into_iter(), SignableBody::Bytes(body))
            .context("constructing signable request")?;

    let (instructions, _signature) = sign(signable_request, &signing_params)
        .context("signing request with SigV4")?
        .into_parts();

    instructions.apply_to_request_http1x(request);

    Ok(())
}
