#![allow(unused)]

use crate::{BackgroundExecutor, LanguageModel, Message};
use anyhow::{anyhow, Result};
use futures::{
    channel::mpsc, future::BoxFuture, io::BufReader, AsyncBufReadExt, AsyncReadExt, FutureExt,
    SinkExt, StreamExt,
};
use http::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::Deserialize;
use std::sync::Arc;

pub struct OllamaClient {
    client: Arc<dyn HttpClient>,
    base_url: String,
    model: String,
    executor: BackgroundExecutor,
}

impl OllamaClient {
    pub fn new(base_url: String, model: String, executor: BackgroundExecutor) -> Self {
        Self {
            client: http::client(None),
            base_url,
            model,
            executor,
        }
    }
}

impl LanguageModel for OllamaClient {
    fn stream_completion(
        &self,
        messages: Vec<Message>,
    ) -> BoxFuture<Result<mpsc::Receiver<String>>> {
        async move {
            let (mut tx, rx) = mpsc::channel(100);

            let request = serde_json::json!({
                "model": &self.model,
                "messages": messages,
                "stream": true,
            });

            let uri = format!("{}/api/chat", self.base_url);
            let request = HttpRequest::builder()
                .method(Method::POST)
                .uri(uri)
                .header("Content-Type", "application/json")
                .body(AsyncBody::from(serde_json::to_vec(&request)?))?;

            let mut response = self.client.send(request).await?;

            if !response.status().is_success() {
                let mut body = Vec::new();
                response.body_mut().read_to_end(&mut body).await?;
                let body_str = std::str::from_utf8(&body)?;
                return Err(anyhow!(
                    "Failed to connect to API: {} {}",
                    response.status(),
                    body_str
                ));
            }

            let reader = BufReader::new(response.into_body());
            let stream = reader.lines().filter_map(|line| async move {
                match line {
                    Ok(line) => match serde_json::from_str::<serde_json::Value>(&line) {
                        Ok(response) => {
                            if let Some(content) = response["message"]["content"].as_str() {
                                Some(Ok(content.to_string()))
                            } else {
                                None
                            }
                        }
                        Err(error) => Some(Err(anyhow!(error))),
                    },
                    Err(error) => Some(Err(anyhow!(error))),
                }
            });

            self.executor
                .spawn(async move {
                    futures::pin_mut!(stream);
                    while let Some(result) = stream.next().await {
                        match result {
                            Ok(text) => {
                                if tx.send(text).await.is_err() {
                                    break;
                                }
                            }
                            Err(e) => {
                                eprintln!("Error in stream: {:?}", e);
                                break;
                            }
                        }
                    }
                })
                .detach();

            Ok(rx)
        }
        .boxed()
    }
}
