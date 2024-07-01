#![allow(unused)]

use crate::{BackgroundExecutor, LanguageModel, Message};
use anyhow::{anyhow, Result};
use futures::{channel::mpsc, future::BoxFuture, FutureExt, SinkExt, StreamExt};
use reqwest::Client;

pub struct OllamaClient {
    client: Client,
    base_url: String,
    model: String,
    executor: BackgroundExecutor,
}

impl OllamaClient {
    pub fn new(base_url: String, model: String, executor: BackgroundExecutor) -> Self {
        Self {
            client: Client::new(),
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

            let response = self
                .client
                .post(format!("{}/api/chat", self.base_url))
                .json(&request)
                .send()
                .await?;

            if !response.status().is_success() {
                return Err(anyhow!(
                    "error streaming completion: {:?}",
                    response.text().await?
                ));
            }

            self.executor
                .spawn(async move {
                    let mut stream = response.bytes_stream();
                    while let Some(chunk) = stream.next().await {
                        if let Ok(chunk) = chunk {
                            if let Ok(text) = String::from_utf8(chunk.to_vec()) {
                                if let Ok(response) =
                                    serde_json::from_str::<serde_json::Value>(&text)
                                {
                                    if let Some(content) = response["message"]["content"].as_str() {
                                        let _ = tx.send(content.to_string()).await;
                                    }
                                }
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
