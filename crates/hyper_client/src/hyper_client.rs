use std::io::Read;
use std::{pin::Pin, task::Poll};

use anyhow::Error;
use futures::channel::mpsc;
use futures::future::BoxFuture;
use futures::{AsyncRead, StreamExt};
use gpui::AppContext;
use http_client::{http, AsyncBody, HttpClient, Inner};
use hyper::body::{Body, Bytes, Frame, Incoming, SizeHint};
use hyper::http::{Response, Uri};
use hyper_util::client::legacy::{connect::HttpConnector, Client};
use smol::future::FutureExt;
use std::future::Future;

pub struct UreqHttpClient {
    client: ureq::Agent,
    background_executor: gpui::BackgroundExecutor,
}

impl UreqHttpClient {
    pub fn new(background_executor: gpui::BackgroundExecutor) -> Self {
        Self {
            client: ureq::agent(),
            background_executor,
        }
    }
}

struct UreqResponseReader {
    task: gpui::Task<()>,
    receiver: mpsc::Receiver<std::io::Result<Vec<u8>>>,
    buffer: Vec<u8>,
}

impl UreqResponseReader {
    fn new(background_executor: gpui::BackgroundExecutor, response: ureq::Response) -> Self {
        let (mut sender, receiver) = mpsc::channel(1);
        let mut reader = response.into_reader();
        let task = background_executor.spawn(async move {
            let mut buffer = vec![0; 8192];
            loop {
                let n = match reader.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => n,
                    Err(e) => {
                        let _ = sender.try_send(Err(e));
                        break;
                    }
                };
                let _ = sender.try_send(Ok(buffer[..n].to_vec()));
            }
        });

        UreqResponseReader {
            task,
            receiver,
            buffer: Vec::new(),
        }
    }
}

impl AsyncRead for UreqResponseReader {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        let now = std::time::Instant::now();
        if self.buffer.is_empty() {
            match self.receiver.poll_next_unpin(cx) {
                Poll::Ready(Some(Ok(data))) => self.buffer.extend(data),
                Poll::Ready(Some(Err(e))) => return Poll::Ready(Err(e)),
                Poll::Ready(None) => return Poll::Ready(Ok(0)), // EOF
                Poll::Pending => {
                    dbg!(now.elapsed());
                    return Poll::Pending;
                }
            }
        }

        let n = std::cmp::min(buf.len(), self.buffer.len());
        dbg!(buf.len(), self.buffer.len(), now.elapsed());
        dbg!(std::thread::current().id());
        buf[..n].copy_from_slice(&self.buffer[..n]);
        self.buffer.drain(..n);

        Poll::Ready(Ok(n))
    }
}

impl HttpClient for UreqHttpClient {
    fn proxy(&self) -> Option<&Uri> {
        None
    }

    fn send_with_redirect_policy(
        &self,
        request: http::Request<AsyncBody>,
        follow_redirects: bool,
    ) -> BoxFuture<'static, Result<http::Response<AsyncBody>, Error>> {
        let method = request.method().clone();
        let url = request.uri().to_string();
        let headers = request.headers().clone();
        let mut req = self.client.request(method.as_str(), &url);
        for (name, value) in headers.iter() {
            req = req.set(name.as_str(), value.to_str().unwrap());
        }
        let executor = self.background_executor.clone();
        let req = executor.spawn(async move {
            let resp = req.send(request.into_body());
            dbg!(std::thread::current().id());
            resp
        });

        // Set follow_redirects policy
        // req = req.redirects(if follow_redirects { 10 } else { 0 });

        async move {
            // Set headers
            // Send the request
            let response = req.await?;
            dbg!(std::thread::current().id());

            // Convert ureq response to http::Response
            let mut builder = http::Response::builder()
                .status(response.status())
                .version(http::Version::HTTP_11);

            // Set response headers
            for name in response.headers_names() {
                if let Some(value) = response.header(&name) {
                    builder = builder.header(name, value);
                }
            }

            let body = AsyncBody::from_reader(UreqResponseReader::new(executor, response));
            let http_response = builder.body(body)?;

            Ok(http_response)
        }
        .boxed()
    }
}
