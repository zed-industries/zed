use futures_util::{future, FutureExt};
use http::header::AsHeaderName;
use http::{HeaderMap, HeaderValue, Request, Response};
use hyper_util::client::legacy::Error;
use std::sync::Arc;
use std::time::Duration;
use tower::retry::Policy;

use crate::body::OctoBody;

fn header_as_u64(headers: &HeaderMap<HeaderValue>, header: impl AsHeaderName) -> Option<u64> {
    headers.get(header)?.to_str().ok()?.parse().ok()
}
fn header_as_i64(headers: &HeaderMap<HeaderValue>, header: impl AsHeaderName) -> Option<i64> {
    headers.get(header)?.to_str().ok()?.parse().ok()
}

/// Gather metrics about retry behavior when handling rate limit headers.
pub trait RateLimitMetrics: Send + Sync {
    /// An error occurred and either was not a 403/429, or did not have any rate limit headers
    fn retry_after_error(
        &self,
        req: &Request<OctoBody>,
        status_code: http::StatusCode,
        retries_remaining: usize,
    );
    /// A 403/429 error occurred, and rate limit headers were available.
    ///
    /// The handler will wait for `waiting_seconds` before retrying.
    fn rate_limited(
        &self,
        req: &Request<OctoBody>,
        status_code: http::StatusCode,
        retries_remaining: usize,
        waiting_seconds: u64,
    );
}

/// Simple No-op struct for users who do not care about collecting retry metrics
pub struct NoOpRateLimitMetrics;
impl RateLimitMetrics for NoOpRateLimitMetrics {
    fn retry_after_error(
        &self,
        _url: &Request<OctoBody>,
        _status_code: http::StatusCode,
        _retries_remaining: usize,
    ) {
    }
    fn rate_limited(
        &self,
        _url: &Request<OctoBody>,
        _status_code: http::StatusCode,
        _retries_remaining: usize,
        _waiting_seconds: u64,
    ) {
    }
}

#[derive(Clone)]
pub enum RetryConfig {
    None,
    Simple(usize),
    /// Handle GitHub's retry headers, up to [`self.0`] times.
    ///
    /// Per the rate limit documentation here: https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api?apiVersion=2022-11-28
    /// - If we get a 403/429 and can parse the headers, wait until the refresh period before retrying.
    /// - If we get a 429 and none of the headers are present, wait `min_wait_seconds` seconds.
    /// - If we get a 403, and neither of those headers are present, do not retry.
    ///   This is because it is not clear whether it's actually forbidden, or if it's a rate limit.
    /// - For server errors (5xx), retry immediately
    /// - For any other errors do not retry.
    HandleRateLimits {
        metrics: Arc<dyn RateLimitMetrics>,
        max_retries: usize,
        min_wait_seconds: u64,
    },
}

impl<B> Policy<Request<OctoBody>, Response<B>, Error> for RetryConfig {
    type Future = future::BoxFuture<'static, ()>;

    fn retry(
        &mut self,
        req: &mut Request<OctoBody>,
        result: &mut Result<Response<B>, Error>,
    ) -> Option<Self::Future> {
        match self {
            RetryConfig::None => None,
            RetryConfig::Simple(count) => match result {
                Ok(response) => {
                    if response.status().is_server_error() || response.status() == 429 {
                        if *count > 0 {
                            *count -= 1;
                            Some(future::ready(()).boxed())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                Err(_) => {
                    if *count > 0 {
                        *count -= 1;
                        Some(future::ready(()).boxed())
                    } else {
                        None
                    }
                }
            },
            RetryConfig::HandleRateLimits {
                metrics,
                max_retries,
                min_wait_seconds,
            } => {
                if *max_retries > 0 {
                    let response = result.as_ref().ok()?;

                    if matches!(
                        response.status(),
                        http::StatusCode::TOO_MANY_REQUESTS | http::StatusCode::FORBIDDEN
                    ) {
                        *max_retries -= 1;

                        let headers = response.headers();
                        let wait_secs = match (
                            header_as_u64(headers, "retry-after"),
                            header_as_u64(headers, "x-ratelimit-remaining"),
                            header_as_i64(headers, "x-ratelimit-reset"),
                        ) {
                            (Some(secs), _, _) => Some(secs),
                            (None, Some(remaining), Some(reset_ts)) if remaining == 0 => {
                                Some(std::cmp::max(5, reset_ts - chrono::Utc::now().timestamp())
                                    as u64)
                            }
                            (None, _, _)
                                if response.status() == http::StatusCode::TOO_MANY_REQUESTS =>
                            {
                                Some(*min_wait_seconds)
                            }
                            _ => {
                                metrics.retry_after_error(req, response.status(), *max_retries);
                                None
                            }
                        }?;

                        metrics.rate_limited(req, response.status(), *max_retries, wait_secs);
                        Some(
                            tokio::time::sleep(Duration::from_secs(wait_secs))
                                .then(move |_| future::ready(()))
                                .boxed(),
                        )
                    } else if response.status().is_server_error() {
                        *max_retries -= 1;
                        metrics.retry_after_error(req, response.status(), *max_retries);
                        Some(future::ready(()).boxed())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }

    fn clone_request(&mut self, req: &Request<OctoBody>) -> Option<Request<OctoBody>> {
        match self {
            RetryConfig::None => None,
            _ => {
                // This returns none if the body is empty. Just return an empty body
                // instead so that we retry GET requests.
                let body = req.body().try_clone().unwrap_or_else(OctoBody::empty);

                // `Request` can't be cloned
                let mut new_req = Request::builder()
                    .uri(req.uri())
                    .method(req.method())
                    .version(req.version());
                for (name, value) in req.headers() {
                    new_req = new_req.header(name, value);
                }

                let new_req = new_req.body(body).expect(
                    "This should never panic, as we are cloning a components from existing request",
                );
                Some(new_req)
            }
        }
    }
}
