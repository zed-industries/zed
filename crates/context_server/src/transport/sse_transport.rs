use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use futures::FutureExt;
use futures::{io::BufReader, AsyncBufReadExt as _, Stream};
use gpui::http_client::HttpClient;
use gpui::{AsyncApp, BackgroundExecutor};
use smol::channel;
use smol::lock::Mutex;
use url::Url;
use util::ResultExt as _;

use crate::transport::Transport;

struct MessageUrl {
    url: Arc<Mutex<Option<String>>>,
    url_received: channel::Receiver<()>,
}

impl MessageUrl {
    fn new() -> (Self, channel::Sender<()>) {
        let (url_sender, url_received) = channel::bounded::<()>(1);
        (
            Self {
                url: Arc::new(Mutex::new(None)),
                url_received,
            },
            url_sender,
        )
    }

    async fn url(&self) -> Result<String> {
        if let Some(url) = self.url.lock().await.clone() {
            return Ok(url);
        }
        self.url_received.recv().await?;
        Ok(self.url.lock().await.clone().unwrap())
    }
}

pub struct SseTransport {
    message_url: MessageUrl,
    stdin_receiver: channel::Receiver<String>,
    stderr_receiver: channel::Receiver<String>,
    http_client: Arc<dyn HttpClient>,
}

impl SseTransport {
    pub fn new(endpoint: Url, cx: &AsyncApp) -> Result<Self> {
        let (stdin_sender, stdin_receiver) = channel::unbounded::<String>();
        let (_stderr_sender, stderr_receiver) = channel::unbounded::<String>();
        let (message_url, url_sender) = MessageUrl::new();
        let http_client = cx.update(|cx| cx.http_client().clone())?;

        let message_url_clone = message_url.url.clone();
        cx.spawn({
            let http_client = http_client.clone();
            move |cx| async move {
                Self::handle_sse_stream(
                    cx.background_executor(),
                    endpoint,
                    message_url_clone,
                    stdin_sender,
                    url_sender,
                    http_client,
                )
                .await
                .log_err()
            }
        })
        .detach();

        Ok(Self {
            message_url,
            stdin_receiver,
            stderr_receiver,
            http_client,
        })
    }

    async fn handle_sse_stream(
        executor: &BackgroundExecutor,
        endpoint: Url,
        message_url: Arc<Mutex<Option<String>>>,
        stdin_sender: channel::Sender<String>,
        url_sender: channel::Sender<()>,
        http_client: Arc<dyn HttpClient>,
    ) -> Result<()> {
        loop {
            let mut response = http_client
                .get(endpoint.as_str(), Default::default(), true)
                .await?;
            let mut reader = BufReader::new(response.body_mut());
            let mut line = String::new();

            loop {
                futures::select! {
                    result = reader.read_line(&mut line).fuse() => {
                        match result {
                            Ok(0) => break,
                            Ok(_) => {
                                if line.starts_with("data: ") {
                                    let data = line.trim_start_matches("data: ");
                                    if data.starts_with("http") {
                                        *message_url.lock().await = Some(data.trim().to_string());
                                        url_sender.send(()).await?;
                                    } else {
                                        stdin_sender.send(data.to_string()).await?;
                                    }
                                }
                                line.clear();
                            },
                            Err(_) => break,
                        }
                    },
                    _ = executor.timer(Duration::from_secs(30)).fuse() => {
                        break;
                    }
                }
            }
        }
    }
}

#[async_trait]
impl Transport for SseTransport {
    async fn send(&self, message: String) -> Result<()> {
        let url = self.message_url.url().await?;
        self.http_client.post_json(&url, message.into()).await?;
        Ok(())
    }

    fn receive(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        Box::pin(self.stdin_receiver.clone())
    }

    fn receive_err(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        Box::pin(self.stderr_receiver.clone())
    }
}
