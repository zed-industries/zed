use anyhow::{Result, anyhow};
use async_trait::async_trait;
use futures::{Stream, io::AsyncReadExt, stream};
use http_client::{AsyncBody, HttpClient, Request, http::Method};
use postage::prelude::{Sink, Stream as _};
use std::{
    pin::Pin,
    sync::{Arc, Mutex},
};

use crate::transport::Transport;

pub struct HttpTransport {
    http_client: Arc<dyn HttpClient>,
    endpoint: String,
    tx: Mutex<Option<postage::oneshot::Sender<Result<String>>>>,
    rx: Mutex<Option<postage::oneshot::Receiver<Result<String>>>>,
}

impl HttpTransport {
    pub fn new(http_client: Arc<dyn HttpClient>, endpoint: String) -> Self {
        let (tx, rx) = postage::oneshot::channel();
        Self {
            http_client,
            endpoint,
            tx: Mutex::new(Some(tx)),
            rx: Mutex::new(Some(rx)),
        }
    }
}

#[async_trait]
impl Transport for HttpTransport {
    async fn send(&self, message: String) -> Result<()> {
        let request = Request::builder()
            .method(Method::POST)
            .uri(self.endpoint.clone())
            .header("Content-Type", "application/json")
            .body(AsyncBody::from(message.into_bytes()))?;

        let http_client = self.http_client.clone();
        let mut tx = self
            .tx
            .lock()
            .unwrap()
            .take()
            .ok_or_else(|| anyhow!("transport already used"))?;
        smol::spawn(async move {
            let res = async {
                let mut response = http_client.send(request).await?;

                if response.status().is_success() {
                    let mut body_str = String::new();
                    response.body_mut().read_to_string(&mut body_str).await?;
                    Ok(body_str)
                } else {
                    let error_message =
                        format!("Request failed with status: {}", response.status());
                    Err(anyhow!(error_message))
                }
            }
            .await;
            tx.send(res).await.ok();
        })
        .detach();

        Ok(())
    }

    fn receive(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        if let Some(mut rx) = self.rx.lock().unwrap().take() {
            Box::pin(stream::once(async move {
                match rx.recv().await {
                    Some(Ok(response)) => response,
                    _ => "".to_string(),
                }
            }))
        } else {
            Box::pin(stream::empty())
        }
    }

    fn receive_err(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        Box::pin(stream::empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use http_client::FakeHttpClient;
    use http_client::http::Response;

    #[gpui::test]
    async fn test_http_transport(_cx: &mut gpui::TestAppContext) {
        let response_body = r#"{"jsonrpc":"2.0","id":1,"result":"hello"}"#;
        let http_client = FakeHttpClient::create(move |_req| async move {
            Ok(Response::builder()
                .status(200)
                .body(response_body.to_string().into())
                .unwrap())
        });
        let transport = HttpTransport::new(http_client, "http://example.com".to_string());

        let request_body = r#"{"jsonrpc":"2.0","id":1,"method":"test"}"#;
        transport.send(request_body.to_string()).await.unwrap();

        let mut receiver = transport.receive();
        let received_body = receiver.next().await.unwrap();
        assert_eq!(received_body, response_body);
    }
}
