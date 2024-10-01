use std::{io::Read, task::Poll};

use anyhow::anyhow;
use futures::{AsyncRead, TryStreamExt};
use http_client::{http, ReadTimeout};
use reqwest::header::{HeaderMap, HeaderValue};
use smol::future::FutureExt;

pub struct ReqwestClient {
    client: reqwest::Client,
}

impl ReqwestClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub fn user_agent(agent: &str) -> anyhow::Result<Self> {
        let mut map = HeaderMap::new();
        map.insert(http::header::USER_AGENT, HeaderValue::from_str(agent)?);
        Ok(Self {
            client: reqwest::Client::builder().default_headers(map).build()?,
        })
    }
}

impl From<reqwest::Client> for ReqwestClient {
    fn from(client: reqwest::Client) -> Self {
        Self { client }
    }
}

struct WrappedBody(http_client::AsyncBody);

impl futures::stream::Stream for WrappedBody {
    type Item = Result<Vec<u8>, std::io::Error>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        match &mut self.0 .0 {
            http_client::Inner::Empty => Poll::Ready(None),
            http_client::Inner::SyncReader(cursor) => {
                let mut buf = Vec::new();
                match cursor.read_to_end(&mut buf) {
                    Ok(_) => {
                        return Poll::Ready(Some(Ok(buf)));
                    }
                    Err(e) => return Poll::Ready(Some(Err(e))),
                }
            }
            http_client::Inner::AsyncReader(async_reader) => {
                let mut buf = vec![0; 8192];
                match AsyncRead::poll_read(std::pin::Pin::new(async_reader), cx, &mut buf) {
                    Poll::Ready(Ok(n)) => {
                        buf.truncate(n);
                        return Poll::Ready(Some(Ok(buf)));
                    }
                    Poll::Ready(Err(e)) => return Poll::Ready(Some(Err(e))),
                    Poll::Pending => Poll::Pending,
                }
            }
        }
    }
}

impl http_client::HttpClient for ReqwestClient {
    fn proxy(&self) -> Option<&http::Uri> {
        None
    }

    fn send(
        &self,
        req: http::Request<http_client::AsyncBody>,
    ) -> futures::future::BoxFuture<
        'static,
        Result<http_client::Response<http_client::AsyncBody>, anyhow::Error>,
    > {
        let (parts, body) = req.into_parts();

        let mut request = self.client.request(parts.method, parts.uri.to_string());

        request = request.headers(parts.headers);

        if let Some(redirect_policy) = parts.extensions.get::<http_client::RedirectPolicy>() {
            request = request.redirect_policy(match redirect_policy {
                http_client::RedirectPolicy::NoFollow => reqwest::redirect::Policy::none(),
                http_client::RedirectPolicy::FollowLimit(limit) => {
                    reqwest::redirect::Policy::limited(*limit as usize)
                }
                http_client::RedirectPolicy::FollowAll => reqwest::redirect::Policy::limited(100),
            });
        }

        if let Some(ReadTimeout(timeout)) = parts.extensions.get::<ReadTimeout>() {
            request = request.timeout(*timeout);
        }

        let body = WrappedBody(body);
        let request = request.body(reqwest::Body::wrap_stream(body));

        async move {
            let response = request.send().await.map_err(|e| anyhow!(e))?;
            let status = response.status();
            let mut builder = http::Response::builder().status(status.as_u16());
            for (name, value) in response.headers() {
                builder = builder.header(name, value);
            }
            let bytes = response.bytes_stream();
            let bytes = bytes
                .map_err(|e| futures::io::Error::new(futures::io::ErrorKind::Other, e))
                .into_async_read();
            let body = http_client::AsyncBody::from_reader(bytes);
            builder.body(body).map_err(|e| anyhow!(e))
        }
        .boxed()
    }
}
