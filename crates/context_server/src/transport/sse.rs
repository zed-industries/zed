use anyhow::{Result, anyhow};
use async_trait::async_trait;
use futures::{io::BufReader, AsyncBufReadExt, Stream, StreamExt};
use http_client::{
    http::Method,
    AsyncBody, HttpClient, Request,
};
use postage::prelude::{Sink, Stream as _};
use std::{
    pin::Pin,
    sync::{Arc, Mutex},
};

use crate::transport::Transport;

type StringStream = Pin<Box<dyn Stream<Item = String> + Send>>;

pub struct SseTransport {
    http_client: Arc<dyn HttpClient>,
    endpoint: String,
    tx: Mutex<Option<postage::mpsc::Sender<StringStream>>>,
    rx: Mutex<Option<postage::mpsc::Receiver<StringStream>>>,
    err_tx: Mutex<Option<postage::mpsc::Sender<String>>>,
    err_rx: Mutex<Option<postage::mpsc::Receiver<String>>>,
}

impl SseTransport {
    pub fn new(http_client: Arc<dyn HttpClient>, endpoint: String) -> Self {
        let (tx, rx) = postage::mpsc::channel(1);
        let (err_tx, err_rx) = postage::mpsc::channel(1);
        Self {
            http_client,
            endpoint,
            tx: Mutex::new(Some(tx)),
            rx: Mutex::new(Some(rx)),
            err_tx: Mutex::new(Some(err_tx)),
            err_rx: Mutex::new(Some(err_rx)),
        }
    }
}

#[async_trait]
impl Transport for SseTransport {
    async fn send(&self, message: String) -> Result<()> {
        let request = Request::builder()
            .method(Method::POST)
            .uri(self.endpoint.clone())
            .header("Content-Type", "application/json")
            .header("Accept", "text/event-stream")
            .body(AsyncBody::from(message.into_bytes()))?;

        let http_client = self.http_client.clone();
        let mut tx = self.tx.lock().unwrap().take().ok_or_else(|| anyhow!("transport already used"))?;
        let mut err_tx = self.err_tx.lock().unwrap().take().ok_or_else(|| anyhow!("transport already used"))?;

        smol::spawn(async move {
            let response = http_client.send(request).await;
            match response {
                Ok(response) => {
                    if response.status().is_success() {
                        let body = response.into_body();
                        let reader = BufReader::new(body);
                        let lines = reader.lines();

                        let sse_stream = lines.filter_map(|line| async {
                            line.ok().and_then(|line| {
                                if line.starts_with("data:") {
                                    Some(line[5..].trim().to_string())
                                } else {
                                    None
                                }
                            })
                        });

                        tx.send(Box::pin(sse_stream)).await.ok();
                    } else {
                        let error_message =
                            format!("Request failed with status: {}", response.status());
                        err_tx.send(error_message).await.ok();
                    }
                }
                Err(e) => {
                    err_tx.send(e.to_string()).await.ok();
                }
            }
        })
        .detach();

        Ok(())
    }

    fn receive(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        if let Some(mut rx) = self.rx.lock().unwrap().take() {
            Box::pin(futures::stream::once(async move {
                rx.recv().await.unwrap()
            }).flatten())
        } else {
            Box::pin(futures::stream::empty())
        }
    }

    fn receive_err(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        if let Some(rx) = self.err_rx.lock().unwrap().take() {
            Box::pin(rx)
        } else {
            Box::pin(futures::stream::empty())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use http_client::http::Response;
    use http_client::FakeHttpClient;

    #[gpui::test]
    async fn test_sse_transport() {
        let sse_response = "data: event 1\n\ndata: event 2\n\n";
        let http_client = FakeHttpClient::create(move |_req| async move {
            Ok(Response::builder()
                .status(200)
                .body(sse_response.to_string().into())
                .unwrap())
        });
        let transport = SseTransport::new(http_client, "http://example.com/sse".to_string());

        transport.send("{}".to_string()).await.unwrap();

        let mut receiver = transport.receive();
        assert_eq!(receiver.next().await, Some("event 1".to_string()));
        assert_eq!(receiver.next().await, Some("event 2".to_string()));
        assert_eq!(receiver.next().await, None);
    }

    #[gpui::test]
    async fn test_sse_transport_error() {
        let http_client = FakeHttpClient::create(move |_req| async move {
            Ok(Response::builder()
                .status(500)
                .body("".to_string().into())
                .unwrap())
        });
        let transport = SseTransport::new(http_client, "http://example.com/sse".to_string());

        transport.send("{}".to_string()).await.unwrap();

        let mut err_receiver = transport.receive_err();
        assert_eq!(
            err_receiver.next().await,
            Some("Request failed with status: 500 INTERNAL SERVER ERROR".to_string())
        );
    }
}
