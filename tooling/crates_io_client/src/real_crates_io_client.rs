use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::Context as _;
use async_trait::async_trait;
use futures::AsyncReadExt as _;
use http_client::{AsyncBody, HttpClient, Method, Request, Response, http::header};
use parking_lot::Mutex;
use serde::de::DeserializeOwned;
use url::Url;

use crate::*;

const BASE_URL: &str = "https://crates.io/api/v1";
// User agent and request rate requirements: https://crates.io/data-access
const USER_AGENT: &str = "Zed GPUI releases (https://github.com/zed-industries/zed)";
pub(crate) const MIN_REQUEST_INTERVAL: Duration = Duration::from_secs(1);

/// Reserving the send time before waiting makes concurrent requests queue up
/// behind each other instead of all waking at the same instant.
pub(crate) struct RequestThrottle {
    next_request_at: Mutex<Option<Instant>>,
}

impl RequestThrottle {
    pub(crate) fn new() -> Self {
        Self {
            next_request_at: Mutex::new(None),
        }
    }

    pub(crate) fn reserve(&self, now: Instant) -> Instant {
        let mut next_request_at = self.next_request_at.lock();
        let send_at = next_request_at.map_or(now, |next_request_at| next_request_at.max(now));
        *next_request_at = Some(send_at + MIN_REQUEST_INTERVAL);
        send_at
    }
}

/// The request rate limit is enforced per client, so share a single client.
pub struct RealCratesIoClient {
    http_client: Arc<dyn HttpClient>,
    api_token: ApiToken,
    throttle: RequestThrottle,
}

impl RealCratesIoClient {
    pub fn new(http_client: Arc<dyn HttpClient>, api_token: ApiToken) -> Self {
        Self {
            http_client,
            api_token,
            throttle: RequestThrottle::new(),
        }
    }

    fn url(path: &str) -> Result<Url> {
        Ok(Url::parse(&format!("{BASE_URL}{path}")).context("Invalid crates.io URL")?)
    }

    async fn request_response(
        &self,
        method: Method,
        url: Url,
        body: Option<Vec<u8>>,
    ) -> Result<Response<AsyncBody>> {
        let mut request = Request::builder()
            .method(method)
            .uri(url.as_str())
            .header(header::USER_AGENT, USER_AGENT)
            .header(header::ACCEPT, "application/json")
            .header(header::AUTHORIZATION, self.api_token.value());
        if body.is_some() {
            request = request.header(header::CONTENT_TYPE, "application/json");
        }
        let request = request
            .body(body.map_or_else(AsyncBody::empty, AsyncBody::from))
            .context("Failed to build crates.io request")?;

        let send_at = self.throttle.reserve(Instant::now());
        smol::Timer::at(send_at).await;

        let mut response = self
            .http_client
            .send(request)
            .await
            .with_context(|| format!("Failed to send crates.io request to {url}"))?;

        if !response.status().is_success() {
            let mut body = String::new();
            response
                .body_mut()
                .read_to_string(&mut body)
                .await
                .with_context(|| format!("Failed to read crates.io response from {url}"))?;
            return Err(CratesIoError::from_response(response.status(), &body));
        }

        Ok(response)
    }

    async fn request_json<T: DeserializeOwned>(
        &self,
        method: Method,
        url: Url,
        body: Option<Vec<u8>>,
    ) -> Result<T> {
        let mut response = self.request_response(method, url.clone(), body).await?;
        let mut body = Vec::new();
        response
            .body_mut()
            .read_to_end(&mut body)
            .await
            .with_context(|| format!("Failed to read crates.io response from {url}"))?;
        Ok(serde_json::from_slice(&body)
            .with_context(|| format!("Failed to parse crates.io response from {url}"))?)
    }
}

#[async_trait]
impl CratesIoClient for RealCratesIoClient {
    async fn get_site_metadata(&self) -> Result<SiteMetadata> {
        self.request_json(Method::GET, Self::url("/site_metadata")?, None)
            .await
    }
}
