use std::{mem, sync::Arc, time::Duration};

use futures::future::BoxFuture;
use isahc::config::{Configurable, RedirectPolicy};
use util::maybe;

pub struct IsahcHttpClient(isahc::HttpClient);

pub use http_client::*;

impl IsahcHttpClient {
    pub fn new(proxy: Option<Uri>) -> Arc<IsahcHttpClient> {
        Arc::new(IsahcHttpClient(
            isahc::HttpClient::builder()
                .connect_timeout(Duration::from_secs(5))
                .low_speed_timeout(100, Duration::from_secs(5))
                .proxy(proxy.clone())
                .build()
                .unwrap(),
        ))
    }
}

impl HttpClient for IsahcHttpClient {
    fn proxy(&self) -> Option<&Uri> {
        None
    }

    fn send_with_redirect_policy(
        &self,
        req: http_client::http::Request<http_client::AsyncBody>,
        follow_redirects: bool,
    ) -> BoxFuture<'static, Result<http_client::Response<http_client::AsyncBody>, anyhow::Error>>
    {
        let req = maybe!({
            let (mut parts, body) = req.into_parts();

            let mut builder = isahc::Request::builder()
                .method(parts.method)
                .uri(parts.uri)
                .version(parts.version);

            let headers = builder.headers_mut()?;
            mem::swap(headers, &mut parts.headers);

            let extensions = builder.extensions_mut()?;
            mem::swap(extensions, &mut parts.extensions);

            let isahc_body = isahc::AsyncBody::from_reader(body);

            builder
                .redirect_policy(if follow_redirects {
                    RedirectPolicy::Follow
                } else {
                    RedirectPolicy::None
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
