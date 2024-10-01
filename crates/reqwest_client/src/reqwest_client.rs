use std::{io::Read, mem, task::Poll};

use anyhow::anyhow;
use futures::{AsyncRead, TryStreamExt};
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
    fn proxy(&self) -> Option<&http_client::http::Uri> {
        None
    }

    fn send(
        &self,
        req: http_client::http::Request<http_client::AsyncBody>,
    ) -> futures::future::BoxFuture<
        'static,
        Result<http_client::Response<http_client::AsyncBody>, anyhow::Error>,
    > {
        let (parts, body) = req.into_parts();


        let mut request = self.client.request(
            parts.method,
            parts.uri.to_string(),
        );

        request.headers(parts.headers);


        if let Some(redirect_policy) = parts.extensions.get::<http_client::RedirectPolicy>() {
            request.
        }

        let body = WrappedBody(req.into_body());
        let request = request.body(reqwest::Body::wrap_stream(body));

        async move {
            let response = request.send().await.map_err(|e| anyhow!(e))?;
            let status = response.status();
            let mut builder = http_client::http::Response::builder().status(status.as_u16());
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
