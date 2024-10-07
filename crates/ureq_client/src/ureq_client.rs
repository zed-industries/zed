use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;
use std::time::Duration;
use std::{pin::Pin, task::Poll};

use anyhow::Error;
use futures::channel::mpsc;
use futures::future::BoxFuture;
use futures::{AsyncRead, SinkExt, StreamExt};
use http_client::{http, AsyncBody, HttpClient, RedirectPolicy, Uri};
use smol::future::FutureExt;
use util::ResultExt;

pub struct UreqClient {
    // Note in ureq 2.x the options are stored on the Agent.
    // In ureq 3.x we'll be able to set these on the request.
    // In practice it's probably "fine" to have many clients, the number of distinct options
    // is low; and most requests to the same connection will have the same options so the
    // connection pool will work.
    clients: Arc<parking_lot::Mutex<HashMap<(Duration, RedirectPolicy), ureq::Agent>>>,
    proxy_url: Option<Uri>,
    proxy: Option<ureq::Proxy>,
    user_agent: String,
    background_executor: gpui::BackgroundExecutor,
}

impl UreqClient {
    pub fn new(
        proxy_url: Option<Uri>,
        user_agent: String,
        background_executor: gpui::BackgroundExecutor,
    ) -> Self {
        Self {
            clients: Arc::default(),
            proxy_url: proxy_url.clone(),
            proxy: proxy_url.and_then(|url| ureq::Proxy::new(url.to_string()).log_err()),
            user_agent,
            background_executor,
        }
    }

    fn agent_for(&self, redirect_policy: RedirectPolicy, timeout: Duration) -> ureq::Agent {
        let mut clients = self.clients.lock();
        // in case our assumption of distinct options is wrong, we'll sporadically clean it out.
        if clients.len() > 50 {
            clients.clear()
        }

        clients
            .entry((timeout, redirect_policy.clone()))
            .or_insert_with(|| {
                let mut builder = ureq::AgentBuilder::new()
                    .timeout_connect(Duration::from_secs(5))
                    .timeout_read(timeout)
                    .timeout_write(timeout)
                    .user_agent(&self.user_agent)
                    .tls_config(http_client::TLS_CONFIG.clone())
                    .redirects(match redirect_policy {
                        RedirectPolicy::NoFollow => 0,
                        RedirectPolicy::FollowLimit(limit) => limit,
                        RedirectPolicy::FollowAll => 100,
                    });
                if let Some(proxy) = &self.proxy {
                    builder = builder.proxy(proxy.clone());
                }
                builder.build()
            })
            .clone()
    }
}
impl HttpClient for UreqClient {
    fn proxy(&self) -> Option<&Uri> {
        self.proxy_url.as_ref()
    }

    fn send(
        &self,
        request: http::Request<AsyncBody>,
    ) -> BoxFuture<'static, Result<http::Response<AsyncBody>, Error>> {
        let agent = self.agent_for(
            request
                .extensions()
                .get::<RedirectPolicy>()
                .cloned()
                .unwrap_or_default(),
            request
                .extensions()
                .get::<http_client::ReadTimeout>()
                .cloned()
                .unwrap_or_default()
                .0,
        );
        let mut req = agent.request(&request.method().as_ref(), &request.uri().to_string());
        for (name, value) in request.headers().into_iter() {
            req = req.set(name.as_str(), value.to_str().unwrap());
        }
        let body = request.into_body();
        let executor = self.background_executor.clone();

        self.background_executor
            .spawn(async move {
                let response = req.send(body)?;

                let mut builder = http::Response::builder()
                    .status(response.status())
                    .version(http::Version::HTTP_11);
                for name in response.headers_names() {
                    if let Some(value) = response.header(&name) {
                        builder = builder.header(name, value);
                    }
                }

                let body = AsyncBody::from_reader(UreqResponseReader::new(executor, response));
                let http_response = builder.body(body)?;

                Ok(http_response)
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
