use std::{mem, sync::Arc, time::Duration};

use futures::future::BoxFuture;
use util::maybe;

pub use isahc::config::Configurable;
pub struct IsahcHttpClient(isahc::HttpClient);

pub use http_client::*;

impl IsahcHttpClient {
    pub fn new(proxy: Option<Uri>, user_agent: Option<String>) -> Arc<IsahcHttpClient> {
        let mut builder = isahc::HttpClient::builder()
            .connect_timeout(Duration::from_secs(5))
            .low_speed_timeout(100, Duration::from_secs(5))
            .proxy(proxy.clone());
        if let Some(agent) = user_agent {
            builder = builder.default_header("User-Agent", agent);
        }
        Arc::new(IsahcHttpClient(builder.build().unwrap()))
    }
    pub fn builder() -> isahc::HttpClientBuilder {
        isahc::HttpClientBuilder::new()
    }
}

impl From<isahc::HttpClient> for IsahcHttpClient {
    fn from(client: isahc::HttpClient) -> Self {
        Self(client)
    }
}

impl HttpClient for IsahcHttpClient {
    fn proxy(&self) -> Option<&Uri> {
        None
    }

    fn send(
        &self,
        req: http_client::http::Request<http_client::AsyncBody>,
    ) -> BoxFuture<'static, Result<http_client::Response<http_client::AsyncBody>, anyhow::Error>>
    {
        let redirect_policy = req
            .extensions()
            .get::<http_client::RedirectPolicy>()
            .cloned()
            .unwrap_or_default();
        let read_timeout = req
            .extensions()
            .get::<http_client::ReadTimeout>()
            .map(|t| t.0);
        let req = maybe!({
            let (mut parts, body) = req.into_parts();
            let mut builder = isahc::Request::builder()
                .method(parts.method)
                .uri(parts.uri)
                .version(parts.version);
            if let Some(read_timeout) = read_timeout {
                builder = builder.low_speed_timeout(100, read_timeout);
            }

            let headers = builder.headers_mut()?;
            mem::swap(headers, &mut parts.headers);

            let extensions = builder.extensions_mut()?;
            mem::swap(extensions, &mut parts.extensions);

            let isahc_body = match body.0 {
                http_client::Inner::Empty => isahc::AsyncBody::empty(),
                http_client::Inner::AsyncReader(reader) => isahc::AsyncBody::from_reader(reader),
                http_client::Inner::SyncReader(reader) => {
                    isahc::AsyncBody::from_bytes_static(reader.into_inner())
                }
            };

            builder
                .redirect_policy(match redirect_policy {
                    http_client::RedirectPolicy::FollowAll => isahc::config::RedirectPolicy::Follow,
                    http_client::RedirectPolicy::FollowLimit(limit) => {
                        isahc::config::RedirectPolicy::Limit(limit)
                    }
                    http_client::RedirectPolicy::NoFollow => isahc::config::RedirectPolicy::None,
                })
                .body(isahc_body)
                .ok()
        });

        let client = self.0.clone();

        Box::pin(async move {
            match req {
                Some(req) => client
                    .send_async(req)
                    .await
                    .map_err(Into::into)
                    .map(|response| {
                        let (parts, body) = response.into_parts();
                        let body = http_client::AsyncBody::from_reader(body);
                        http_client::Response::from_parts(parts, body)
                    }),
                None => Err(anyhow::anyhow!("Request was malformed")),
            }
        })
    }
}
