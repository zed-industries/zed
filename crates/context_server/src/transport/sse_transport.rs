use std::pin::Pin;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::io::BufReader;
use futures::{AsyncBufReadExt as _, Stream};
use gpui::http_client::{AsyncBody, HttpClient};
use gpui::AsyncApp;
use parking_lot::Mutex;
use smol::channel;
use url::Url;
use util::ResultExt as _;

use crate::transport::Transport;

pub struct SseTransport {
    message_url: Arc<Mutex<Option<String>>>,
    stdin_receiver: channel::Receiver<String>,
    stderr_receiver: channel::Receiver<String>,
    http_client: Arc<dyn HttpClient>,
}

impl SseTransport {
    pub fn new(endpoint: Url, cx: &AsyncApp) -> Result<Self> {
        let (stdin_sender, stdin_receiver) = channel::unbounded::<String>();
        let (_stderr_sender, stderr_receiver) = channel::unbounded::<String>();
        let message_url = Arc::new(Mutex::new(None));
        let http_client = cx.update(|cx| cx.http_client().clone())?;

        let message_url_clone = message_url.clone();
        cx.spawn({
            let http_client = http_client.clone();
            move |_| async move {
                Self::handle_sse_stream(endpoint, message_url_clone, stdin_sender, http_client)
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
        endpoint: Url,
        message_url: Arc<Mutex<Option<String>>>,
        stdin_sender: channel::Sender<String>,
        http_client: Arc<dyn HttpClient>,
    ) -> Result<()> {
        loop {
            let mut response = http_client
                .get(endpoint.as_str(), Default::default(), true)
                .await?;
            let mut reader = BufReader::new(response.body_mut());
            let mut line = String::new();

            while let Ok(n) = reader.read_line(&mut line).await {
                if n == 0 {
                    break;
                }
                if line.starts_with("data: ") {
                    let data = line.trim_start_matches("data: ");
                    if data.starts_with("http") {
                        *message_url.lock() = Some(data.to_string());
                    } else {
                        stdin_sender.send(data.to_string()).await?;
                    }
                }
                line.clear();
            }
        }
    }
}

#[async_trait]
impl Transport for SseTransport {
    async fn send(&self, message: String) -> Result<()> {
        let url = self.message_url.lock().as_ref().map(String::clone);
        if let Some(url) = url {
            self.http_client
                .post_json(&url, AsyncBody::from(message))
                .await?;
            Ok(())
        } else {
            Err(anyhow!("Message URL not yet received"))
        }
    }

    fn receive(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        Box::pin(self.stdin_receiver.clone())
    }

    fn receive_err(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        Box::pin(self.stderr_receiver.clone())
    }
}
