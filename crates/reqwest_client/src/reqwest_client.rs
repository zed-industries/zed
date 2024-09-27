use std::{
    borrow::Cow,
    io::Read,
    pin::{self, Pin},
    task::Poll,
};

use futures::AsyncRead;

struct ReqwestClient {
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

struct WrappedResponse {
    inner: reqwest::Response,
    buffer: Vec<u8>,
}

impl AsyncRead for WrappedResponse {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        let this = self.get_mut();
        let chunk = this.inner.chunk();
        match chunk {};
        let pin = std::pin::Pin::new(&mut this.inner);
        let poll_next = futures::Stream::poll_next(pin, cx);
        // TODO: WRONG
        poll_next.map(|_| Ok(0))
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
        let mut request = self.client.request(
            reqwest::Method::from_bytes(req.method().as_str().as_bytes()).unwrap(),
            req.uri().to_string(),
        );

        for (key, value) in req.headers().iter() {
            request = request.header(key, value);
        }

        let body = WrappedBody(req.into_body());
        let request = request.body(reqwest::Body::wrap_stream(body));

        async move {
            let response = request.send().await?;
            let status = response.status();
            let mut builder = http_client::http::Response::builder().status(status.as_u16());
            for (name, value) in response.headers() {
                builder = builder.header(name, value);
            }

            let body = http_client::AsyncBody::from_reader(WrappedResponse {
                inner: response,
                buffer: Vec::new(),
            });
            builder.body(body)
        }
        .boxed()
    }
}
