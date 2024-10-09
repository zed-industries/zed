use std::any::type_name;
use std::time::Duration;
use std::{pin::Pin, task::Poll};

use anyhow::Error;
use futures::channel::mpsc;
use futures::future::BoxFuture;
use futures::{AsyncRead, SinkExt, StreamExt};
use http_client::http::Response;
use http_client::{http, AsyncBody, HttpClient, RedirectPolicy, Request, Uri};
use smol::future::FutureExt;
use ureq::{Agent, Proxy, SendBody, Timeouts};
use util::ResultExt;

pub struct UreqClient {
    agent: ureq::Agent,
    background_executor: gpui::BackgroundExecutor,
}

impl UreqClient {
    pub fn new(
        proxy_url: Option<Uri>,
        user_agent: String,
        background_executor: gpui::BackgroundExecutor,
    ) -> Self {
        let agent: ureq::Agent = ureq::Config {
            http_status_as_error: false,
            proxy: proxy_url.and_then(|url| Proxy::new(&url.to_string()).log_err()),
            timeouts: Timeouts {
                connect: Some(Duration::from_secs(5)),
                ..Default::default()
            },
            user_agent: Some(user_agent),
            ..Default::default()
        }
        .into();

        Self {
            agent,
            background_executor,
        }
    }
}

impl HttpClient for UreqClient {
    fn proxy(&self) -> Option<&Uri> {
        self.agent.config().proxy.as_ref().map(|proxy| proxy.uri())
    }

    fn type_name(&self) -> &'static str {
        type_name::<Self>()
    }

    fn send(
        &self,
        request: http::Request<AsyncBody>,
    ) -> BoxFuture<'static, Result<http::Response<AsyncBody>, Error>> {
        let redirect_policy = request
            .extensions()
            .get::<RedirectPolicy>()
            .cloned()
            .unwrap_or_default();

        let timeout = request
            .extensions()
            .get::<http_client::ReadTimeout>()
            .cloned()
            .unwrap_or_default()
            .0;
        let agent = self.agent.clone();
        let executor = self.background_executor.clone();
        self.background_executor
            .spawn(async move {
                let (parts, body) = request.into_parts();
                let body = match body.0 {
                    http_client::Inner::Empty => SendBody::none(),
                    _ => SendBody::from_owned_reader(body),
                };
                let mut request = Request::from_parts(parts, body);

                let config = agent.configure_request(&mut request);
                config.max_redirects = match redirect_policy {
                    RedirectPolicy::NoFollow => 0,
                    RedirectPolicy::FollowLimit(limit) => limit,
                    RedirectPolicy::FollowAll => 100,
                };
                config.timeouts.recv_body = Some(timeout);

                let response = agent.run(request)?;

                let (parts, response_body) = response.into_parts();
                let response_body =
                    AsyncBody::from_reader(UreqResponseReader::new(executor, response_body));
                Ok(Response::from_parts(parts, response_body))
            })
            .boxed()
    }
}

struct UreqResponseReader {
    receiver: mpsc::Receiver<std::io::Result<Vec<u8>>>,
    buffer: Vec<u8>,
    idx: usize,
    _task: gpui::Task<()>,
}

impl UreqResponseReader {
    fn new(background_executor: gpui::BackgroundExecutor, response: ureq::Body) -> Self {
        let (mut sender, receiver) = mpsc::channel(1);
        let mut reader = response.into_reader();

        let task = background_executor.spawn(async move {
            let mut buffer = vec![0; 8192];
            loop {
                let n = match std::io::Read::read(&mut reader, &mut buffer) {
                    Ok(0) => break,
                    Ok(n) => n,
                    Err(e) => {
                        let _ = sender.send(Err(e)).await;
                        break;
                    }
                };
                let _ = sender.send(Ok(buffer[..n].to_vec())).await;
            }
        });

        UreqResponseReader {
            _task: task,
            receiver,
            buffer: Vec::new(),
            idx: 0,
        }
    }
}

impl AsyncRead for UreqResponseReader {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        if self.buffer.is_empty() {
            match self.receiver.poll_next_unpin(cx) {
                Poll::Ready(Some(Ok(data))) => self.buffer = data,
                Poll::Ready(Some(Err(e))) => {
                    return Poll::Ready(Err(e));
                }
                Poll::Ready(None) => {
                    return Poll::Ready(Ok(0));
                }
                Poll::Pending => {
                    return Poll::Pending;
                }
            }
        }
        let n = std::cmp::min(buf.len(), self.buffer.len() - self.idx);
        buf[..n].copy_from_slice(&self.buffer[self.idx..self.idx + n]);
        self.idx += n;
        if self.idx == self.buffer.len() {
            self.buffer.clear();
            self.idx = 0;
        }

        Poll::Ready(Ok(n))
    }
}
